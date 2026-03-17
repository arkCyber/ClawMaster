//! Tool call loop detection and prevention.
//!
//! Implements guardrails to prevent agents from getting stuck in infinite loops
//! when calling tools. Detects three types of loop patterns:
//! - Generic repeat: same tool + same params repeatedly
//! - Known poll no progress: polling tools with identical outputs
//! - Ping-pong: alternating A/B/A/B patterns with no progress

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::{
        collections::{HashMap, VecDeque},
        sync::{Arc, RwLock},
        time::{SystemTime, UNIX_EPOCH},
    },
};

/// Loop detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoopDetectionConfig {
    pub enabled: bool,
    pub warning_threshold: usize,
    pub critical_threshold: usize,
    pub global_circuit_breaker_threshold: usize,
    pub history_size: usize,
    pub detectors: DetectorConfig,
}

impl Default for LoopDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            warning_threshold: 10,
            critical_threshold: 20,
            global_circuit_breaker_threshold: 30,
            history_size: 30,
            detectors: DetectorConfig::default(),
        }
    }
}

/// Individual detector configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DetectorConfig {
    pub generic_repeat: bool,
    pub known_poll_no_progress: bool,
    pub ping_pong: bool,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            generic_repeat: true,
            known_poll_no_progress: true,
            ping_pong: true,
        }
    }
}

/// A single tool call record.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCallRecord {
    tool_name: String,
    params_hash: u64,
    result_hash: u64,
    timestamp: u64,
}

impl ToolCallRecord {
    fn new(tool_name: String, params: &Value, result: &Value) -> Self {
        Self {
            tool_name,
            params_hash: hash_value(params),
            result_hash: hash_value(result),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Hash a JSON value for comparison.
fn hash_value(value: &Value) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    value.to_string().hash(&mut hasher);
    hasher.finish()
}

/// Loop detection state for a single session.
#[derive(Debug)]
struct SessionState {
    history: VecDeque<ToolCallRecord>,
    warning_count: usize,
    critical_count: usize,
}

impl SessionState {
    fn new(history_size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(history_size),
            warning_count: 0,
            critical_count: 0,
        }
    }

    fn add_record(&mut self, record: ToolCallRecord, max_size: usize) {
        if self.history.len() >= max_size {
            self.history.pop_front();
        }
        self.history.push_back(record);
    }
}

/// Loop detection tool.
pub struct LoopDetectionTool {
    config: LoopDetectionConfig,
    sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    global_call_count: Arc<RwLock<usize>>,
}

impl LoopDetectionTool {
    pub fn new(config: LoopDetectionConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            global_call_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Check for generic repeat pattern.
    fn detect_generic_repeat(&self, history: &VecDeque<ToolCallRecord>) -> Option<String> {
        if history.len() < 3 {
            return None;
        }

        let recent = history.iter().rev().take(5).collect::<Vec<_>>();
        if recent.len() < 3 {
            return None;
        }

        // Check if last 3+ calls are identical
        let first = &recent[0];
        let identical_count = recent.iter()
            .take_while(|r| {
                r.tool_name == first.tool_name && r.params_hash == first.params_hash
            })
            .count();

        if identical_count >= 3 {
            return Some(format!(
                "Generic repeat detected: tool '{}' called {} times with identical parameters",
                first.tool_name, identical_count
            ));
        }

        None
    }

    /// Check for known poll no progress pattern.
    fn detect_poll_no_progress(&self, history: &VecDeque<ToolCallRecord>) -> Option<String> {
        if history.len() < 3 {
            return None;
        }

        let poll_tools = ["process", "session_status", "cron"];
        let recent = history.iter().rev().take(5).collect::<Vec<_>>();
        
        for tool in &poll_tools {
            let poll_calls: Vec<_> = recent.iter()
                .filter(|r| r.tool_name == *tool)
                .collect();

            if poll_calls.len() >= 3 {
                // Check if results are identical
                let first_result = poll_calls[0].result_hash;
                let identical_results = poll_calls.iter()
                    .take_while(|r| r.result_hash == first_result)
                    .count();

                if identical_results >= 3 {
                    return Some(format!(
                        "Poll no progress detected: tool '{}' called {} times with identical results",
                        tool, identical_results
                    ));
                }
            }
        }

        None
    }

    /// Check for ping-pong pattern (A/B/A/B).
    fn detect_ping_pong(&self, history: &VecDeque<ToolCallRecord>) -> Option<String> {
        if history.len() < 4 {
            return None;
        }

        let recent = history.iter().rev().take(6).collect::<Vec<_>>();
        if recent.len() < 4 {
            return None;
        }

        // Check for A/B/A/B pattern
        for i in 0..recent.len() - 3 {
            let a1 = &recent[i];
            let b1 = &recent[i + 1];
            let a2 = &recent[i + 2];
            let b2 = &recent[i + 3];

            if a1.tool_name == a2.tool_name 
                && b1.tool_name == b2.tool_name
                && a1.tool_name != b1.tool_name
                && a1.params_hash == a2.params_hash
                && b1.params_hash == b2.params_hash {
                return Some(format!(
                    "Ping-pong detected: alternating between '{}' and '{}'",
                    a1.tool_name, b1.tool_name
                ));
            }
        }

        None
    }

    /// Record a tool call and check for loops.
    pub fn record_and_check(
        &self,
        session_id: &str,
        tool_name: &str,
        params: &Value,
        result: &Value,
    ) -> Result<LoopDetectionResult> {
        if !self.config.enabled {
            return Ok(LoopDetectionResult::Ok);
        }

        // Increment global counter
        {
            let mut count = self.global_call_count.write()
                .map_err(|e| anyhow::anyhow!("Failed to acquire global count lock: {}", e))?;
            *count += 1;

            if *count >= self.config.global_circuit_breaker_threshold {
                return Ok(LoopDetectionResult::CircuitBreaker {
                    message: format!(
                        "Global circuit breaker triggered: {} total tool calls",
                        *count
                    ),
                });
            }
        }

        // Get or create session state
        let mut sessions = self.sessions.write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire sessions lock: {}", e))?;
        
        let state = sessions.entry(session_id.to_string())
            .or_insert_with(|| SessionState::new(self.config.history_size));

        // Add record
        let record = ToolCallRecord::new(tool_name.to_string(), params, result);
        state.add_record(record, self.config.history_size);

        // Run detectors
        let mut warnings = Vec::new();

        if self.config.detectors.generic_repeat {
            if let Some(msg) = self.detect_generic_repeat(&state.history) {
                warnings.push(msg);
            }
        }

        if self.config.detectors.known_poll_no_progress {
            if let Some(msg) = self.detect_poll_no_progress(&state.history) {
                warnings.push(msg);
            }
        }

        if self.config.detectors.ping_pong {
            if let Some(msg) = self.detect_ping_pong(&state.history) {
                warnings.push(msg);
            }
        }

        if warnings.is_empty() {
            return Ok(LoopDetectionResult::Ok);
        }

        // Increment warning/critical counters
        state.warning_count += 1;
        
        if state.warning_count >= self.config.critical_threshold {
            state.critical_count += 1;
            return Ok(LoopDetectionResult::Critical {
                warnings,
                count: state.critical_count,
            });
        }

        if state.warning_count >= self.config.warning_threshold {
            return Ok(LoopDetectionResult::Warning {
                warnings,
                count: state.warning_count,
            });
        }

        Ok(LoopDetectionResult::Ok)
    }

    /// Reset session state.
    pub fn reset_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire sessions lock: {}", e))?;
        sessions.remove(session_id);
        Ok(())
    }

    /// Get session statistics.
    pub fn get_stats(&self, session_id: &str) -> Result<LoopDetectionStats> {
        let sessions = self.sessions.read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire sessions lock: {}", e))?;
        
        let global_count = *self.global_call_count.read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire global count lock: {}", e))?;

        if let Some(state) = sessions.get(session_id) {
            Ok(LoopDetectionStats {
                history_size: state.history.len(),
                warning_count: state.warning_count,
                critical_count: state.critical_count,
                global_call_count: global_count,
            })
        } else {
            Ok(LoopDetectionStats {
                history_size: 0,
                warning_count: 0,
                critical_count: 0,
                global_call_count: global_count,
            })
        }
    }
}

/// Loop detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum LoopDetectionResult {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "warning")]
    Warning {
        warnings: Vec<String>,
        count: usize,
    },
    #[serde(rename = "critical")]
    Critical {
        warnings: Vec<String>,
        count: usize,
    },
    #[serde(rename = "circuit_breaker")]
    CircuitBreaker {
        message: String,
    },
}

/// Loop detection statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopDetectionStats {
    pub history_size: usize,
    pub warning_count: usize,
    pub critical_count: usize,
    pub global_call_count: usize,
}

#[async_trait]
impl AgentTool for LoopDetectionTool {
    fn name(&self) -> &str {
        "loop_detection"
    }

    fn description(&self) -> &str {
        "Monitor and detect tool call loops to prevent infinite execution patterns"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["check", "reset", "stats"],
                    "description": "Action to perform: check status, reset session, or get statistics"
                },
                "session_id": {
                    "type": "string",
                    "description": "Session ID to check/reset (optional, uses current session if omitted)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' parameter"))?;

        let session_id = params.get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        match action {
            "check" => {
                let stats = self.get_stats(session_id)?;
                Ok(json!({
                    "status": "ok",
                    "enabled": self.config.enabled,
                    "stats": stats,
                    "config": {
                        "warning_threshold": self.config.warning_threshold,
                        "critical_threshold": self.config.critical_threshold,
                        "global_circuit_breaker_threshold": self.config.global_circuit_breaker_threshold,
                    }
                }))
            }
            "reset" => {
                self.reset_session(session_id)?;
                Ok(json!({
                    "status": "ok",
                    "message": format!("Session '{}' reset successfully", session_id)
                }))
            }
            "stats" => {
                let stats = self.get_stats(session_id)?;
                Ok(json!({
                    "status": "ok",
                    "session_id": session_id,
                    "stats": stats
                }))
            }
            _ => bail!("Invalid action: {}", action),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_detection_config_default() {
        let config = LoopDetectionConfig::default();
        assert!(config.enabled);
        assert_eq!(config.warning_threshold, 10);
        assert_eq!(config.critical_threshold, 20);
        assert_eq!(config.global_circuit_breaker_threshold, 30);
        assert_eq!(config.history_size, 30);
    }

    #[test]
    fn test_generic_repeat_detection() {
        let config = LoopDetectionConfig::default();
        let tool = LoopDetectionTool::new(config);

        let params = json!({"command": "echo test"});
        let result = json!({"output": "test"});

        // Call same tool 3 times
        for _ in 0..3 {
            let _ = tool.record_and_check("test_session", "exec", &params, &result);
        }

        let stats = tool.get_stats("test_session").unwrap();
        assert_eq!(stats.history_size, 3);
    }

    #[test]
    fn test_session_reset() {
        let config = LoopDetectionConfig::default();
        let tool = LoopDetectionTool::new(config);

        let params = json!({"command": "echo test"});
        let result = json!({"output": "test"});

        tool.record_and_check("test_session", "exec", &params, &result).unwrap();
        
        let stats_before = tool.get_stats("test_session").unwrap();
        assert_eq!(stats_before.history_size, 1);

        tool.reset_session("test_session").unwrap();

        let stats_after = tool.get_stats("test_session").unwrap();
        assert_eq!(stats_after.history_size, 0);
    }

    #[tokio::test]
    async fn test_tool_execute_check() {
        let config = LoopDetectionConfig::default();
        let tool = LoopDetectionTool::new(config);

        let params = json!({"action": "check", "session_id": "test"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["status"], "ok");
        assert!(result["enabled"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_tool_execute_reset() {
        let config = LoopDetectionConfig::default();
        let tool = LoopDetectionTool::new(config);

        let params = json!({"action": "reset", "session_id": "test"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["status"], "ok");
        assert!(result["message"].as_str().unwrap().contains("reset successfully"));
    }

    #[tokio::test]
    async fn test_tool_execute_stats() {
        let config = LoopDetectionConfig::default();
        let tool = LoopDetectionTool::new(config);

        let params = json!({"action": "stats", "session_id": "test"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["status"], "ok");
        assert_eq!(result["session_id"], "test");
        assert!(result["stats"].is_object());
    }
}
