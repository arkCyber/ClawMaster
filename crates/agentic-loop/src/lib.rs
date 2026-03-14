use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, warn};

mod executor;
mod registry;
mod context;

pub use executor::ToolExecutor;
pub use registry::{Tool, ToolRegistry};
pub use context::ExecutionContext;

/// Configuration for the agentic loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgenticLoopConfig {
    pub max_iterations: usize,
    pub timeout_seconds: u64,
    pub enable_memory: bool,
}

impl Default for AgenticLoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            timeout_seconds: 300,
            enable_memory: true,
        }
    }
}

/// Result of a reasoning step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub thought: String,
    pub tool_call: Option<ToolCall>,
    pub is_complete: bool,
    pub final_answer: Option<String>,
}

/// A tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

/// Result of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_name: String,
    pub output: String,
    pub success: bool,
    pub error: Option<String>,
}

/// The main agentic loop
pub struct AgenticLoop {
    config: AgenticLoopConfig,
    registry: Arc<ToolRegistry>,
    executor: Arc<ToolExecutor>,
}

impl AgenticLoop {
    pub fn new(config: AgenticLoopConfig) -> Self {
        let registry = Arc::new(ToolRegistry::new());
        let executor = Arc::new(ToolExecutor::new(registry.clone()));
        
        Self {
            config,
            registry,
            executor,
        }
    }

    pub fn registry(&self) -> &Arc<ToolRegistry> {
        &self.registry
    }

    /// Run the agentic loop until task completion or max iterations
    pub async fn run_until_complete(
        &self,
        task: &str,
        reasoning_fn: impl Fn(&ExecutionContext) -> Result<ReasoningResult>,
    ) -> Result<String> {
        let mut context = ExecutionContext::new(task.to_string());
        let mut iteration = 0;

        info!("Starting agentic loop for task: {}", task);

        let result = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.run_loop(&mut context, &reasoning_fn, &mut iteration),
        )
        .await??;

        info!(
            "Agentic loop completed after {} iterations",
            iteration
        );

        Ok(result)
    }

    async fn run_loop(
        &self,
        context: &mut ExecutionContext,
        reasoning_fn: &impl Fn(&ExecutionContext) -> Result<ReasoningResult>,
        iteration: &mut usize,
    ) -> Result<String> {
        while *iteration < self.config.max_iterations {
            debug!("Iteration {}/{}", *iteration + 1, self.config.max_iterations);

            // Step 1: Reasoning
            let reasoning = reasoning_fn(context)?;
            context.add_thought(reasoning.thought.clone());

            debug!("Thought: {}", reasoning.thought);

            // Step 2: Check if complete
            if reasoning.is_complete {
                if let Some(answer) = reasoning.final_answer {
                    info!("Task completed with answer");
                    return Ok(answer);
                } else {
                    warn!("Marked complete but no final answer provided");
                    return Ok(context.get_summary());
                }
            }

            // Step 3: Execute tool if requested
            if let Some(tool_call) = reasoning.tool_call {
                debug!("Executing tool: {}", tool_call.tool_name);
                
                let result = self.executor.execute(&tool_call).await?;
                context.add_tool_result(result.clone());

                if !result.success {
                    warn!("Tool execution failed: {:?}", result.error);
                }
            }

            *iteration += 1;
        }

        warn!("Max iterations ({}) reached without completion", self.config.max_iterations);
        Ok(context.get_summary())
    }

    /// Run a single iteration (for testing or manual control)
    pub async fn run_iteration(
        &self,
        context: &mut ExecutionContext,
        reasoning_fn: impl Fn(&ExecutionContext) -> Result<ReasoningResult>,
    ) -> Result<Option<String>> {
        let reasoning = reasoning_fn(context)?;
        context.add_thought(reasoning.thought);

        if reasoning.is_complete {
            return Ok(reasoning.final_answer);
        }

        if let Some(tool_call) = reasoning.tool_call {
            let result = self.executor.execute(&tool_call).await?;
            context.add_tool_result(result);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agentic_loop_creation() {
        let config = AgenticLoopConfig::default();
        let loop_instance = AgenticLoop::new(config);
        
        assert_eq!(loop_instance.config.max_iterations, 10);
        assert_eq!(loop_instance.config.timeout_seconds, 300);
    }

    #[tokio::test]
    async fn test_single_iteration() {
        let config = AgenticLoopConfig::default();
        let loop_instance = AgenticLoop::new(config);
        let mut context = ExecutionContext::new("test task".to_string());

        let reasoning_fn = |_ctx: &ExecutionContext| {
            Ok(ReasoningResult {
                thought: "Testing".to_string(),
                tool_call: None,
                is_complete: true,
                final_answer: Some("Done".to_string()),
            })
        };

        let result = loop_instance.run_iteration(&mut context, reasoning_fn).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("Done".to_string()));
    }

    #[tokio::test]
    async fn test_max_iterations() {
        let config = AgenticLoopConfig {
            max_iterations: 3,
            timeout_seconds: 10,
            enable_memory: true,
        };
        let loop_instance = AgenticLoop::new(config);

        let reasoning_fn = |_ctx: &ExecutionContext| {
            Ok(ReasoningResult {
                thought: "Still thinking".to_string(),
                tool_call: None,
                is_complete: false,
                final_answer: None,
            })
        };

        let result = loop_instance.run_until_complete("test", reasoning_fn).await;
        assert!(result.is_ok());
    }
}
