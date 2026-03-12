//! Tool executor for running tools with timeout and error handling

use crate::error::ToolExecutionError;
use crate::tool_registry::{ToolContext, ToolExecutionResult, ToolRegistry};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, warn};

/// Tool executor with timeout and error handling
pub struct ToolExecutor {
    registry: Arc<ToolRegistry>,
    default_timeout: Duration,
}

impl ToolExecutor {
    /// Create a new tool executor
    pub fn new(registry: Arc<ToolRegistry>, default_timeout: Duration) -> Self {
        Self {
            registry,
            default_timeout,
        }
    }
    
    /// Execute a tool with default timeout
    pub async fn execute(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
        context: ToolContext,
    ) -> Result<ToolExecutionResult, ToolExecutionError> {
        self.execute_with_timeout(tool_name, arguments, context, self.default_timeout).await
    }
    
    /// Execute a tool with custom timeout
    pub async fn execute_with_timeout(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
        context: ToolContext,
        timeout_duration: Duration,
    ) -> Result<ToolExecutionResult, ToolExecutionError> {
        debug!("Executing tool '{}' with timeout {:?}", tool_name, timeout_duration);
        
        let execution_future = self.registry.execute_tool(tool_name, arguments, context);
        
        match timeout(timeout_duration, execution_future).await {
            Ok(result) => result,
            Err(_) => {
                warn!("Tool '{}' timed out after {:?}", tool_name, timeout_duration);
                Err(ToolExecutionError::Timeout(tool_name.to_string(), timeout_duration))
            }
        }
    }
    
    /// Execute multiple tools in sequence (chaining)
    pub async fn execute_chain(
        &self,
        tool_calls: Vec<ToolCall>,
        context: ToolContext,
    ) -> Result<Vec<ToolExecutionResult>, ToolExecutionError> {
        let mut results = Vec::new();
        let mut current_context = context;
        
        for tool_call in tool_calls {
            let result = self.execute_with_timeout(
                &tool_call.tool_name,
                tool_call.arguments,
                current_context.clone(),
                tool_call.timeout,
            ).await?;
            
            // Update context with result data for next tool
            if let Some(data) = &result.data {
                current_context.metadata.insert(
                    format!("{}_result", tool_call.tool_name),
                    data.to_string(),
                );
            }
            
            results.push(result);
            
            // If a tool failed, stop the chain
            if !results.last().unwrap().success {
                break;
            }
        }
        
        Ok(results)
    }
}

/// A single tool call in a chain
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// Name of the tool to call
    pub tool_name: String,
    /// Arguments to pass to the tool
    pub arguments: serde_json::Value,
    /// Timeout for this specific call
    pub timeout: Duration,
}

impl ToolCall {
    /// Create a new tool call
    pub fn new(tool_name: String, arguments: serde_json::Value) -> Self {
        Self {
            tool_name,
            arguments,
            timeout: Duration::from_secs(30),
        }
    }
    
    /// Set a custom timeout for this call
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Builder for creating tool chains
pub struct ToolChainBuilder {
    calls: Vec<ToolCall>,
}

impl ToolChainBuilder {
    /// Create a new tool chain builder
    pub fn new() -> Self {
        Self {
            calls: Vec::new(),
        }
    }
    
    /// Add a tool call to the chain
    pub fn add_tool(mut self, tool_name: String, arguments: serde_json::Value) -> Self {
        self.calls.push(ToolCall::new(tool_name, arguments));
        self
    }
    
    /// Add a tool call with custom timeout
    pub fn add_tool_with_timeout(
        mut self,
        tool_name: String,
        arguments: serde_json::Value,
        timeout: Duration,
    ) -> Self {
        self.calls.push(ToolCall {
            tool_name,
            arguments,
            timeout,
        });
        self
    }
    
    /// Build the tool chain
    pub fn build(self) -> Vec<ToolCall> {
        self.calls
    }
}

impl Default for ToolChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_registry::{ToolRegistry, EchoTool};
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_tool_executor() {
        let registry = Arc::new(ToolRegistry::new());
        registry.register_tool(Arc::new(EchoTool));
        
        let executor = ToolExecutor::new(registry.clone(), Duration::from_secs(5));
        
        let context = ToolContext::new(
            "session123".to_string(),
            "user123".to_string(),
            "channel123".to_string(),
            "test message".to_string(),
        );
        
        let result = executor.execute(
            "echo",
            serde_json::json!({"message": "Hello, World!"}),
            context,
        ).await;
        
        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert!(tool_result.success);
    }
    
    #[tokio::test]
    async fn test_tool_chain() {
        let registry = Arc::new(ToolRegistry::new());
        registry.register_tool(Arc::new(EchoTool));
        
        let executor = ToolExecutor::new(registry, Duration::from_secs(5));
        
        let context = ToolContext::new(
            "session123".to_string(),
            "user123".to_string(),
            "channel123".to_string(),
            "test message".to_string(),
        );
        
        let chain = ToolChainBuilder::new()
            .add_tool("echo".to_string(), serde_json::json!({"message": "First"}))
            .add_tool("echo".to_string(), serde_json::json!({"message": "Second"}))
            .build();
        
        let results = executor.execute_chain(chain, context).await;
        
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].success);
        assert!(results[1].success);
    }
}
