//! Event stream separation for tool execution, LLM output, and system messages.
//!
//! This module provides a clean separation of different event types, allowing
//! clients to selectively subscribe to specific event streams.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

/// Maximum number of events to buffer per stream
const EVENT_BUFFER_SIZE: usize = 1000;

/// Event stream types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "stream", rename_all = "lowercase")]
pub enum EventStream {
    /// Tool execution events
    Tool(ToolEvent),
    /// LLM output events
    Llm(LlmEvent),
    /// System messages and notifications
    System(SystemEvent),
}

/// Tool execution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    #[serde(rename = "sessionKey", skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(rename = "runId", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "toolCallId", skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool name
    pub tool_name: String,
    /// Execution status
    pub status: ToolStatus,
    /// Tool arguments (optional, for started events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    /// Tool result (optional, for completed events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error message (optional, for failed events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Execution duration in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// Timestamp
    pub timestamp: i64,
}

/// Tool execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolStatus {
    /// Tool execution started
    Started,
    /// Tool execution completed successfully
    Completed,
    /// Tool execution failed
    Failed,
}

/// LLM output event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmEvent {
    #[serde(rename = "sessionKey", skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(rename = "runId", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "messageIndex", skip_serializing_if = "Option::is_none")]
    pub message_index: Option<u64>,
    /// Content chunk
    pub content: String,
    /// Whether this is the final chunk
    #[serde(default)]
    pub is_final: bool,
    /// Finish reason (optional, for final chunks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    /// Token usage (optional, for final chunks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
    /// Timestamp
    pub timestamp: i64,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// System event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    #[serde(rename = "sessionKey", skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(rename = "runId", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Log level
    pub level: LogLevel,
    /// Message
    pub message: String,
    /// Additional context (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
    /// Timestamp
    pub timestamp: i64,
}

/// Log level for system events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Event router for managing multiple event streams
#[derive(Clone)]
pub struct EventRouter {
    tool_tx: broadcast::Sender<ToolEvent>,
    llm_tx: broadcast::Sender<LlmEvent>,
    system_tx: broadcast::Sender<SystemEvent>,
}

impl EventRouter {
    /// Create a new event router
    pub fn new() -> Self {
        let (tool_tx, _) = broadcast::channel(EVENT_BUFFER_SIZE);
        let (llm_tx, _) = broadcast::channel(EVENT_BUFFER_SIZE);
        let (system_tx, _) = broadcast::channel(EVENT_BUFFER_SIZE);

        Self {
            tool_tx,
            llm_tx,
            system_tx,
        }
    }

    /// Subscribe to tool events
    pub fn subscribe_tool(&self) -> broadcast::Receiver<ToolEvent> {
        self.tool_tx.subscribe()
    }

    /// Subscribe to LLM events
    pub fn subscribe_llm(&self) -> broadcast::Receiver<LlmEvent> {
        self.llm_tx.subscribe()
    }

    /// Subscribe to system events
    pub fn subscribe_system(&self) -> broadcast::Receiver<SystemEvent> {
        self.system_tx.subscribe()
    }

    /// Emit a tool event
    pub fn emit_tool(&self, event: ToolEvent) {
        let _ = self.tool_tx.send(event);
    }

    /// Emit an LLM event
    pub fn emit_llm(&self, event: LlmEvent) {
        let _ = self.llm_tx.send(event);
    }

    /// Emit a system event
    pub fn emit_system(&self, event: SystemEvent) {
        let _ = self.system_tx.send(event);
    }

    /// Emit a tool started event
    pub fn emit_tool_started(&self, tool_name: String, arguments: Option<Value>) {
        self.emit_tool(ToolEvent {
            session_key: None,
            run_id: None,
            tool_call_id: None,
            tool_name,
            status: ToolStatus::Started,
            arguments,
            result: None,
            error: None,
            duration_ms: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a tool completed event
    pub fn emit_tool_completed(
        &self,
        tool_name: String,
        result: Value,
        duration_ms: Option<u64>,
    ) {
        self.emit_tool(ToolEvent {
            session_key: None,
            run_id: None,
            tool_call_id: None,
            tool_name,
            status: ToolStatus::Completed,
            arguments: None,
            result: Some(result),
            error: None,
            duration_ms,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a tool failed event
    pub fn emit_tool_failed(&self, tool_name: String, error: String, duration_ms: Option<u64>) {
        self.emit_tool(ToolEvent {
            session_key: None,
            run_id: None,
            tool_call_id: None,
            tool_name,
            status: ToolStatus::Failed,
            arguments: None,
            result: None,
            error: Some(error),
            duration_ms,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit an LLM content chunk
    pub fn emit_llm_chunk(&self, content: String) {
        self.emit_llm(LlmEvent {
            session_key: None,
            run_id: None,
            message_index: None,
            content,
            is_final: false,
            finish_reason: None,
            token_usage: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a final LLM event
    pub fn emit_llm_final(
        &self,
        content: String,
        finish_reason: Option<String>,
        token_usage: Option<TokenUsage>,
    ) {
        self.emit_llm(LlmEvent {
            session_key: None,
            run_id: None,
            message_index: None,
            content,
            is_final: true,
            finish_reason,
            token_usage,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a system debug message
    pub fn emit_debug(&self, message: String, context: Option<Value>) {
        self.emit_system(SystemEvent {
            session_key: None,
            run_id: None,
            level: LogLevel::Debug,
            message,
            context,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a system info message
    pub fn emit_info(&self, message: String, context: Option<Value>) {
        self.emit_system(SystemEvent {
            session_key: None,
            run_id: None,
            level: LogLevel::Info,
            message,
            context,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a system warning message
    pub fn emit_warning(&self, message: String, context: Option<Value>) {
        self.emit_system(SystemEvent {
            session_key: None,
            run_id: None,
            level: LogLevel::Warning,
            message,
            context,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }

    /// Emit a system error message
    pub fn emit_error(&self, message: String, context: Option<Value>) {
        self.emit_system(SystemEvent {
            session_key: None,
            run_id: None,
            level: LogLevel::Error,
            message,
            context,
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Stream filter for selective event subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamFilter {
    /// Subscribe to tool events
    #[serde(default)]
    pub tool: bool,
    /// Subscribe to LLM events
    #[serde(default)]
    pub llm: bool,
    /// Subscribe to system events
    #[serde(default)]
    pub system: bool,
}

impl StreamFilter {
    /// Create a filter that subscribes to all streams
    pub fn all() -> Self {
        Self {
            tool: true,
            llm: true,
            system: true,
        }
    }

    /// Create a filter that subscribes to tool and LLM streams only
    pub fn tool_and_llm() -> Self {
        Self {
            tool: true,
            llm: true,
            system: false,
        }
    }

    /// Create a filter that subscribes to LLM stream only
    pub fn llm_only() -> Self {
        Self {
            tool: false,
            llm: true,
            system: false,
        }
    }

    /// Check if any stream is enabled
    pub fn is_any_enabled(&self) -> bool {
        self.tool || self.llm || self.system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_router_tool_events() {
        let router = EventRouter::new();
        let mut rx = router.subscribe_tool();

        router.emit_tool_started("test_tool".to_string(), None);

        let event = rx.recv().await.unwrap();
        assert_eq!(event.tool_name, "test_tool");
        assert_eq!(event.status, ToolStatus::Started);
    }

    #[tokio::test]
    async fn test_event_router_llm_events() {
        let router = EventRouter::new();
        let mut rx = router.subscribe_llm();

        router.emit_llm_chunk("Hello".to_string());

        let event = rx.recv().await.unwrap();
        assert_eq!(event.content, "Hello");
        assert!(!event.is_final);
    }

    #[tokio::test]
    async fn test_event_router_system_events() {
        let router = EventRouter::new();
        let mut rx = router.subscribe_system();

        router.emit_info("Test message".to_string(), None);

        let event = rx.recv().await.unwrap();
        assert_eq!(event.level, LogLevel::Info);
        assert_eq!(event.message, "Test message");
    }

    #[tokio::test]
    async fn test_stream_filter() {
        let filter = StreamFilter::all();
        assert!(filter.tool);
        assert!(filter.llm);
        assert!(filter.system);
        assert!(filter.is_any_enabled());

        let filter = StreamFilter::llm_only();
        assert!(!filter.tool);
        assert!(filter.llm);
        assert!(!filter.system);
        assert!(filter.is_any_enabled());
    }

    #[test]
    fn test_event_serialization() {
        let tool_event = ToolEvent {
            session_key: Some("main".to_string()),
            run_id: Some("run-1".to_string()),
            tool_call_id: Some("tool-1".to_string()),
            tool_name: "test".to_string(),
            status: ToolStatus::Completed,
            arguments: None,
            result: Some(serde_json::json!({"success": true})),
            error: None,
            duration_ms: Some(100),
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&tool_event).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("completed"));
    }
}
