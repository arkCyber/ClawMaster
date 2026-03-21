use {
    crate::{ToolCall, ToolRegistry, ToolResult},
    anyhow::{Context, Result},
    std::{sync::Arc, time::Duration},
    tokio::time::timeout,
    tracing::{debug, error},
};

/// Tool executor with timeout and error handling
pub struct ToolExecutor {
    registry: Arc<ToolRegistry>,
    timeout_seconds: u64,
}

impl ToolExecutor {
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self {
            registry,
            timeout_seconds: 30,
        }
    }

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Execute a tool call with timeout protection
    pub async fn execute(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        debug!("Executing tool: {}", tool_call.tool_name);

        let tool = self
            .registry
            .get(&tool_call.tool_name)
            .context(format!("Tool not found: {}", tool_call.tool_name))?;

        let result = timeout(
            Duration::from_secs(self.timeout_seconds),
            tool.execute(tool_call.arguments.clone()),
        )
        .await;

        match result {
            Ok(Ok(output)) => {
                debug!("Tool {} executed successfully", tool_call.tool_name);
                Ok(ToolResult {
                    tool_name: tool_call.tool_name.clone(),
                    output,
                    success: true,
                    error: None,
                })
            },
            Ok(Err(e)) => {
                error!("Tool {} failed: {}", tool_call.tool_name, e);
                Ok(ToolResult {
                    tool_name: tool_call.tool_name.clone(),
                    output: String::new(),
                    success: false,
                    error: Some(e.to_string()),
                })
            },
            Err(_) => {
                error!(
                    "Tool {} timed out after {}s",
                    tool_call.tool_name, self.timeout_seconds
                );
                Ok(ToolResult {
                    tool_name: tool_call.tool_name.clone(),
                    output: String::new(),
                    success: false,
                    error: Some(format!("Timeout after {}s", self.timeout_seconds)),
                })
            },
        }
    }

    /// Execute multiple tools in sequence
    pub async fn execute_chain(&self, tool_calls: Vec<ToolCall>) -> Result<Vec<ToolResult>> {
        let mut results = Vec::new();

        for tool_call in tool_calls {
            let result = self.execute(&tool_call).await?;
            let success = result.success;
            results.push(result);

            if !success {
                debug!("Tool chain stopped due to failure");
                break;
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::Tool, async_trait::async_trait};

    struct TestTool;

    #[async_trait]
    impl Tool for TestTool {
        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        async fn execute(&self, _args: serde_json::Value) -> Result<String> {
            Ok("test output".to_string())
        }
    }

    #[tokio::test]
    async fn test_executor_success() {
        let registry = Arc::new(ToolRegistry::new());
        registry.register(Box::new(TestTool));

        let executor = ToolExecutor::new(registry);

        let tool_call = ToolCall {
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
        };

        let result = executor.execute(&tool_call).await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "test output");
    }

    #[tokio::test]
    async fn test_executor_not_found() {
        let registry = Arc::new(ToolRegistry::new());
        let executor = ToolExecutor::new(registry);

        let tool_call = ToolCall {
            tool_name: "nonexistent".to_string(),
            arguments: serde_json::json!({}),
        };

        let result = executor.execute(&tool_call).await;
        assert!(result.is_err());
    }
}
