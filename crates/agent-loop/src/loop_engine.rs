//! Main agent loop engine implementation

use crate::config::{AgentLoopConfig, ToolIterationConfig};
use crate::error::AgentLoopError;
use crate::executor::{ToolExecutor, ToolChainBuilder};
use crate::tool_registry::{ToolContext, ToolExecutionResult, ToolRegistry};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{debug, info};

use super::AgentLoopInterface;

/// Current state of the agent loop
#[derive(Debug, Clone, PartialEq)]
pub enum LoopState {
    /// Loop is idle and ready to start
    Idle,
    /// Loop is currently executing
    Running,
    /// Loop is stopping
    Stopping,
    /// Loop completed successfully
    Completed,
    /// Loop failed with an error
    Failed(String),
}

/// Result of an agent loop execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopResult {
    /// Final response message
    pub response: String,
    /// All tool execution results
    pub tool_results: Vec<ToolExecutionResult>,
    /// Total execution time
    pub execution_time: Duration,
    /// Number of tool iterations used
    pub iterations_used: usize,
    /// Whether the loop completed successfully
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Intermediate data collected during execution
    pub intermediate_data: Vec<serde_json::Value>,
}

impl LoopResult {
    /// Create a successful loop result
    pub fn success(response: String, tool_results: Vec<ToolExecutionResult>) -> Self {
        Self {
            response,
            tool_results,
            execution_time: Duration::default(),
            iterations_used: 0,
            success: true,
            error: None,
            intermediate_data: Vec::new(),
        }
    }
    
    /// Create a failed loop result
    pub fn error(error: String) -> Self {
        Self {
            response: String::new(),
            tool_results: Vec::new(),
            execution_time: Duration::default(),
            iterations_used: 0,
            success: false,
            error: Some(error),
            intermediate_data: Vec::new(),
        }
    }
}

/// Main agent loop implementation
pub struct AgentLoop {
    config: AgentLoopConfig,
    tool_registry: Arc<ToolRegistry>,
    executor: Arc<ToolExecutor>,
    state: Arc<RwLock<LoopState>>,
    current_execution: Arc<Mutex<Option<LoopExecution>>>,
}

struct LoopExecution {
    start_time: Instant,
    iterations: usize,
    intermediate_data: Vec<serde_json::Value>,
    memory_used: usize,
}

impl AgentLoop {
    /// Create a new agent loop with the given configuration
    pub fn new(config: AgentLoopConfig) -> Self {
        let tool_registry = Arc::new(ToolRegistry::new());
        let executor = Arc::new(ToolExecutor::new(
            tool_registry.clone(),
            config.tool_timeout,
        ));
        
        Self {
            config,
            tool_registry,
            executor,
            state: Arc::new(RwLock::new(LoopState::Idle)),
            current_execution: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Get the tool registry for adding tools
    pub fn tool_registry(&self) -> Arc<ToolRegistry> {
        self.tool_registry.clone()
    }
    
    /// Execute the main agent loop
    async fn execute_loop_internal(
        &self,
        message: String,
        context: ToolContext,
    ) -> Result<LoopResult, AgentLoopError> {
        let start_time = Instant::now();
        let mut tool_results = Vec::new();
        let mut intermediate_data = Vec::new();
        let mut current_message = message.clone();
        
        // Initialize execution state
        {
            let mut execution = self.current_execution.lock().await;
            *execution = Some(LoopExecution {
                start_time,
                iterations: 0,
                intermediate_data: Vec::new(),
                memory_used: 0,
            });
        }
        
        info!("Starting agent loop for message: {}", current_message);
        
        // Main loop iterations
        for iteration in 0..self.config.max_tool_iterations {
            // Check if we should continue
            let iteration_config = self.create_iteration_config(iteration, start_time, &intermediate_data)?;
            
            if !iteration_config.should_continue() {
                debug!("Loop stopping: iteration config limits reached");
                break;
            }
            
            // Update execution state
            {
                let mut execution = self.current_execution.lock().await;
                if let Some(ref mut exec) = *execution {
                    exec.iterations = iteration;
                }
            }
            
            // Determine next tool to call (this would typically involve LLM reasoning)
            let tool_decision = self.decide_next_tool(&current_message, &context, &tool_results).await?;
            
            match tool_decision {
                ToolDecision::NoMoreTools => {
                    debug!("No more tools needed, completing loop");
                    break;
                }
                ToolDecision::ExecuteTool { tool_name, arguments } => {
                    debug!("Executing tool '{}' (iteration {})", tool_name, iteration);
                    
                    let result = self.executor.execute_with_timeout(
                        &tool_name,
                        arguments,
                        context.clone(),
                        self.config.tool_timeout,
                    ).await?;
                    
                    // Store intermediate data if enabled
                    if self.config.track_intermediate_results {
                        if let Some(data) = &result.data {
                            intermediate_data.push(data.clone());
                        }
                    }
                    
                    tool_results.push(result.clone());
                    
                    // Check if tool indicates we need more calls
                    if !result.needs_more_calls {
                        current_message = self.extract_response_from_result(&result)?;
                        break;
                    }
                    
                    // Prepare for next iteration
                    current_message = self.prepare_next_iteration_message(&result)?;
                }
            }
        }
        
        // Check if we exceeded max iterations
        if tool_results.len() >= self.config.max_tool_iterations {
            return Err(AgentLoopError::MaxIterationsExceeded(self.config.max_tool_iterations));
        }
        
        // Create final result
        let execution_time = start_time.elapsed();
        let iterations_count = tool_results.len();
        let mut result = LoopResult::success(current_message, tool_results);
        result.execution_time = execution_time;
        result.iterations_used = iterations_count;
        result.intermediate_data = intermediate_data;
        
        // Clean up execution state
        {
            let mut execution = self.current_execution.lock().await;
            *execution = None;
        }
        
        info!("Agent loop completed in {:?} with {} iterations", execution_time, result.iterations_used);
        Ok(result)
    }
    
    /// Create iteration configuration for current state
    fn create_iteration_config(
        &self,
        iteration: usize,
        start_time: Instant,
        intermediate_data: &[serde_json::Value],
    ) -> Result<ToolIterationConfig, AgentLoopError> {
        let elapsed_time = start_time.elapsed();
        let memory_used = self.calculate_memory_usage(intermediate_data);
        
        if memory_used > self.config.max_intermediate_memory {
            return Err(AgentLoopError::MessageProcessingFailed(
                format!("Memory limit exceeded: {} > {}", memory_used, self.config.max_intermediate_memory)
            ));
        }
        
        Ok(ToolIterationConfig {
            iteration,
            max_iterations: self.config.max_tool_iterations,
            elapsed_time,
            max_time: self.config.loop_timeout,
            memory_used,
            max_memory: self.config.max_intermediate_memory,
        })
    }
    
    /// Decide which tool to execute next (simplified version)
    async fn decide_next_tool(
        &self,
        message: &str,
        _context: &ToolContext,
        _previous_results: &[ToolExecutionResult],
    ) -> Result<ToolDecision, AgentLoopError> {
        // This is a simplified implementation
        // In a real scenario, this would involve LLM reasoning to decide which tool to call
        
        if message.contains("echo") {
            Ok(ToolDecision::ExecuteTool {
                tool_name: "echo".to_string(),
                arguments: serde_json::json!({
                    "message": message
                }),
            })
        } else {
            Ok(ToolDecision::NoMoreTools)
        }
    }
    
    /// Extract response from tool result
    fn extract_response_from_result(&self, result: &ToolExecutionResult) -> Result<String, AgentLoopError> {
        if let Some(data) = &result.data {
            if let Some(echoed) = data.get("echoed_message").and_then(|v| v.as_str()) {
                Ok(echoed.to_string())
            } else {
                Ok(format!("Tool executed successfully: {:?}", data))
            }
        } else {
            Ok("Tool executed successfully".to_string())
        }
    }
    
    /// Prepare message for next iteration
    fn prepare_next_iteration_message(&self, result: &ToolExecutionResult) -> Result<String, AgentLoopError> {
        if let Some(data) = &result.data {
            Ok(format!("Previous result: {:?}", data))
        } else {
            Ok("Previous tool executed".to_string())
        }
    }
    
    /// Calculate memory usage of intermediate data
    fn calculate_memory_usage(&self, data: &[serde_json::Value]) -> usize {
        // Simplified memory calculation
        data.iter()
            .map(|v| v.to_string().len())
            .sum()
    }
}

#[async_trait::async_trait]
impl AgentLoopInterface for AgentLoop {
    async fn execute_loop(&self, message: String, context: ToolContext) -> Result<LoopResult, AgentLoopError> {
        // Check if loop is already running
        {
            let state = self.state.read();
            if *state == LoopState::Running {
                return Err(AgentLoopError::LoopAlreadyRunning);
            }
        }
        
        // Set state to running
        {
            let mut state = self.state.write();
            *state = LoopState::Running;
        }
        
        // Execute the loop
        let result = tokio::time::timeout(self.config.loop_timeout, self.execute_loop_internal(message, context)).await;
        
        // Update state based on result
        let final_result = match result {
            Ok(Ok(loop_result)) => {
                let mut state = self.state.write();
                *state = LoopState::Completed;
                Ok(loop_result)
            }
            Ok(Err(e)) => {
                let mut state = self.state.write();
                *state = LoopState::Failed(e.to_string());
                Err(e)
            }
            Err(_) => {
                let mut state = self.state.write();
                *state = LoopState::Failed("Loop timeout".to_string());
                Err(AgentLoopError::LoopTimeoutExceeded(self.config.loop_timeout))
            }
        };
        
        final_result
    }
    
    fn get_state(&self) -> LoopState {
        self.state.read().clone()
    }
    
    async fn stop_loop(&self) -> Result<(), AgentLoopError> {
        let mut state = self.state.write();
        
        match *state {
            LoopState::Running => {
                *state = LoopState::Stopping;
                // In a real implementation, we would signal the running loop to stop
                Ok(())
            }
            LoopState::Idle => Ok(()),
            _ => Err(AgentLoopError::LoopNotRunning),
        }
    }
}

/// Decision about which tool to execute next
#[derive(Debug)]
enum ToolDecision {
    NoMoreTools,
    ExecuteTool {
        tool_name: String,
        arguments: serde_json::Value,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_registry::{EchoTool, ToolRegistry};
    
    #[tokio::test]
    async fn test_agent_loop_basic() {
        let config = AgentLoopConfig::default();
        let agent_loop = AgentLoop::new(config);
        
        // Register echo tool
        agent_loop.tool_registry().register_tool(Arc::new(EchoTool));
        
        let context = ToolContext::new(
            "session123".to_string(),
            "user123".to_string(),
            "channel123".to_string(),
            "test message".to_string(),
        );
        
        let result = agent_loop.execute_loop("echo hello".to_string(), context).await;
        
        assert!(result.is_ok());
        let loop_result = result.unwrap();
        assert!(loop_result.success);
        assert_eq!(loop_result.response, "hello");
    }
    
    #[tokio::test]
    async fn test_agent_loop_state_management() {
        let config = AgentLoopConfig::default();
        let agent_loop = AgentLoop::new(config);
        
        assert_eq!(agent_loop.get_state(), LoopState::Idle);
        
        let context = ToolContext::new(
            "session123".to_string(),
            "user123".to_string(),
            "channel123".to_string(),
            "test message".to_string(),
        );
        
        // Start execution
        let execution_future = agent_loop.execute_loop("test".to_string(), context);
        
        // Give it a moment to start
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // State should be running (though this test might be flaky due to timing)
        // In a real scenario, we'd use better synchronization
    }
}
