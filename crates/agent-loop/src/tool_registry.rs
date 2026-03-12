//! Tool registry for managing available tools in the agent loop

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info, warn};

use crate::error::{ToolExecutionError, ToolResult};

/// Context provided to tools during execution
#[derive(Debug, Clone)]
pub struct ToolContext {
    /// Session identifier
    pub session_id: String,
    /// User identifier
    pub user_id: String,
    /// Channel identifier
    pub channel_id: String,
    /// Message that triggered the tool
    pub message: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ToolContext {
    /// Create a new tool context
    pub fn new(session_id: String, user_id: String, channel_id: String, message: String) -> Self {
        Self {
            session_id,
            user_id,
            channel_id,
            message,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the context
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    /// Whether the tool execution was successful
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Whether more tool calls are needed
    pub needs_more_calls: bool,
    /// Next tool to call (if any)
    pub next_tool: Option<String>,
    /// Arguments for the next tool call
    pub next_arguments: Option<serde_json::Value>,
}

impl ToolExecutionResult {
    /// Create a successful result
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms: 0,
            needs_more_calls: false,
            next_tool: None,
            next_arguments: None,
        }
    }
    
    /// Create a successful result that needs more tool calls
    pub fn success_with_next_call(
        data: serde_json::Value,
        next_tool: String,
        next_arguments: serde_json::Value,
    ) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms: 0,
            needs_more_calls: true,
            next_tool: Some(next_tool),
            next_arguments: Some(next_arguments),
        }
    }
    
    /// Create a failed result
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            execution_time_ms: 0,
            needs_more_calls: false,
            next_tool: None,
            next_arguments: None,
        }
    }
    
    /// Set execution time
    pub fn with_execution_time(mut self, time_ms: u64) -> Self {
        self.execution_time_ms = time_ms;
        self
    }
}

/// Trait for tools that can be executed by the agent loop
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the unique name of this tool
    fn name(&self) -> &'static str;
    
    /// Get a description of what this tool does
    fn description(&self) -> &'static str;
    
    /// Get the JSON schema for this tool's arguments
    fn argument_schema(&self) -> serde_json::Value;
    
    /// Execute the tool with given arguments
    async fn execute(&self, arguments: serde_json::Value, context: ToolContext) -> ToolResult<ToolExecutionResult>;
    
    /// Check if this tool is available for the given context
    fn is_available(&self, context: &ToolContext) -> bool {
        true // Default to always available
    }
}

/// Registry for managing available tools
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a new tool
    pub fn register_tool(&self, tool: Arc<dyn Tool>) {
        let name = tool.name().to_string();
        info!("Registering tool: {}", name);
        
        let mut tools = self.tools.write();
        tools.insert(name, tool);
    }
    
    /// Unregister a tool by name
    pub fn unregister_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        info!("Unregistering tool: {}", name);
        
        let mut tools = self.tools.write();
        tools.remove(name)
    }
    
    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        let tools = self.tools.read();
        tools.get(name).cloned()
    }
    
    /// List all registered tool names
    pub fn list_tools(&self) -> Vec<String> {
        let tools = self.tools.read();
        tools.keys().cloned().collect()
    }
    
    /// Get tools available for a given context
    pub fn get_available_tools(&self, context: &ToolContext) -> Vec<Arc<dyn Tool>> {
        let tools = self.tools.read();
        tools
            .values()
            .filter(|tool| tool.is_available(context))
            .cloned()
            .collect()
    }
    
    /// Get tool schemas for all available tools
    pub fn get_tool_schemas(&self, context: &ToolContext) -> Vec<serde_json::Value> {
        let tools = self.get_available_tools(context);
        
        tools
            .into_iter()
            .map(|tool| {
                serde_json::json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "input_schema": tool.argument_schema()
                })
            })
            .collect()
    }
    
    /// Execute a tool by name
    pub async fn execute_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
        context: ToolContext,
    ) -> ToolResult<ToolExecutionResult> {
        debug!("Executing tool: {} with arguments: {}", name, arguments);
        
        let tool = self.get_tool(name)
            .ok_or_else(|| ToolExecutionError::ToolNotFound(name.to_string()))?;
        
        if !tool.is_available(&context) {
            return Err(ToolExecutionError::ToolNotAvailable(name.to_string()));
        }
        
        let start_time = std::time::Instant::now();
        let result = tool.execute(arguments, context).await;
        let execution_time = start_time.elapsed();
        
        match result {
            Ok(mut tool_result) => {
                tool_result.execution_time_ms = execution_time.as_millis() as u64;
                debug!("Tool {} executed successfully in {}ms", name, tool_result.execution_time_ms);
                Ok(tool_result)
            }
            Err(e) => {
                warn!("Tool {} execution failed: {}", name, e);
                Err(e)
            }
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple echo tool for testing
pub struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn name(&self) -> &'static str {
        "echo"
    }
    
    fn description(&self) -> &'static str {
        "Echoes back the input message"
    }
    
    fn argument_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Message to echo back"
                }
            },
            "required": ["message"]
        })
    }
    
    async fn execute(&self, arguments: serde_json::Value, _context: ToolContext) -> ToolResult<ToolExecutionResult> {
        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolExecutionError::InvalidArguments("Missing 'message' argument".to_string()))?;
        
        Ok(ToolExecutionResult::success(serde_json::json!({
            "echoed_message": message
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tool_registry() {
        let registry = ToolRegistry::new();
        let echo_tool = Arc::new(EchoTool);
        
        // Register tool
        registry.register_tool(echo_tool.clone());
        
        // List tools
        let tools = registry.list_tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0], "echo");
        
        // Get tool
        let retrieved_tool = registry.get_tool("echo");
        assert!(retrieved_tool.is_some());
        
        // Execute tool
        let context = ToolContext::new(
            "session123".to_string(),
            "user123".to_string(),
            "channel123".to_string(),
            "test message".to_string(),
        );
        
        let result = registry.execute_tool(
            "echo",
            serde_json::json!({"message": "Hello, World!"}),
            context,
        ).await;
        
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert!(tool_result.data.is_some());
    }
}
