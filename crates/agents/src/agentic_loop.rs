//! Agentic Loop integration for ClawMaster agents
//!
//! This module integrates the agentic loop functionality into the agent system,
//! enabling autonomous multi-step task execution with tool chaining.

use anyhow::Result;
use async_trait::async_trait;
use clawmaster_agentic_loop::{
    AgenticLoop, AgenticLoopConfig, ExecutionContext, ReasoningResult, Tool, ToolCall,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, info};

/// Agent with agentic loop capabilities
pub struct AgenticAgent {
    /// The agentic loop instance
    agentic_loop: AgenticLoop,
    /// LLM client for reasoning
    llm_client: Arc<dyn LLMClient>,
}

/// LLM client trait for reasoning
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Generate a reasoning response
    async fn reason(&self, prompt: &str) -> Result<String>;
}

impl AgenticAgent {
    /// Create a new agentic agent
    pub fn new(config: AgenticLoopConfig, llm_client: Arc<dyn LLMClient>) -> Self {
        let agentic_loop = AgenticLoop::new(config);
        Self {
            agentic_loop,
            llm_client,
        }
    }

    /// Register a tool with the agentic loop
    pub fn register_tool(&self, tool: Box<dyn Tool>) {
        self.agentic_loop.registry().register(tool);
    }

    /// Execute a task using the agentic loop
    pub async fn execute_task(&self, task: &str) -> Result<String> {
        info!("Starting agentic task execution: {}", task);

        let result = self
            .agentic_loop
            .run_until_complete(task, |ctx| self.reason_with_llm(ctx))
            .await?;

        info!("Agentic task completed successfully");
        Ok(result)
    }

    /// Perform reasoning using the LLM
    fn reason_with_llm(&self, ctx: &ExecutionContext) -> Result<ReasoningResult> {
        // Build prompt from context
        let prompt = self.build_reasoning_prompt(ctx);

        // Call LLM (blocking for now, will be async in full implementation)
        let response = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.llm_client.reason(&prompt))
        })?;

        // Parse LLM response into reasoning result
        self.parse_llm_response(&response, ctx)
    }

    /// Build a reasoning prompt from execution context
    fn build_reasoning_prompt(&self, ctx: &ExecutionContext) -> String {
        let mut prompt = format!("Task: {}\n\n", ctx.task);

        if !ctx.thoughts.is_empty() {
            prompt.push_str("Previous thoughts:\n");
            for thought in &ctx.thoughts {
                prompt.push_str(&format!("- {}\n", thought));
            }
            prompt.push('\n');
        }

        if let Some(last_result) = ctx.get_last_tool_result() {
            prompt.push_str(&format!("Last tool result: {:?}\n\n", last_result));
        }

        prompt.push_str("Available tools:\n");
        for (tool_name, _description) in self.agentic_loop.registry().list_tools() {
            prompt.push_str(&format!("- {}\n", tool_name));
        }

        prompt.push_str("\nWhat should I do next? Respond with:\n");
        prompt.push_str("1. Your thought process\n");
        prompt.push_str("2. Tool to use (if any) in format: TOOL: tool_name {\"arg\": \"value\"}\n");
        prompt.push_str("3. Or COMPLETE: final_answer if task is done\n");

        prompt
    }

    /// Parse LLM response into reasoning result
    fn parse_llm_response(&self, response: &str, _ctx: &ExecutionContext) -> Result<ReasoningResult> {
        debug!("Parsing LLM response: {}", response);

        // Extract thought
        let thought = self.extract_thought(response);

        // Check if task is complete
        if let Some(answer) = self.extract_completion(response) {
            return Ok(ReasoningResult {
                thought,
                tool_call: None,
                is_complete: true,
                final_answer: Some(answer),
            });
        }

        // Extract tool call
        let tool_call = self.extract_tool_call(response)?;

        Ok(ReasoningResult {
            thought,
            tool_call,
            is_complete: false,
            final_answer: None,
        })
    }

    /// Extract thought from response
    fn extract_thought(&self, response: &str) -> String {
        // Simple extraction - take first line or paragraph
        response
            .lines()
            .next()
            .unwrap_or("Thinking...")
            .to_string()
    }

    /// Extract completion marker and final answer
    fn extract_completion(&self, response: &str) -> Option<String> {
        if let Some(idx) = response.find("COMPLETE:") {
            let answer = response[idx + 9..].trim().to_string();
            return Some(answer);
        }
        None
    }

    /// Extract tool call from response
    fn extract_tool_call(&self, response: &str) -> Result<Option<ToolCall>> {
        if let Some(idx) = response.find("TOOL:") {
            let tool_line = &response[idx + 5..].trim();
            
            // Find the first space to separate tool name from arguments
            if let Some(space_idx) = tool_line.find(' ') {
                let tool_name = tool_line[..space_idx].trim().to_string();
                let args_str = tool_line[space_idx + 1..].trim();

                // Parse JSON arguments
                let args: Value = serde_json::from_str(args_str)
                    .unwrap_or_else(|_| serde_json::json!({}));

                return Ok(Some(ToolCall {
                    tool_name,
                    arguments: args,
                }));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLLMClient;

    #[async_trait]
    impl LLMClient for MockLLMClient {
        async fn reason(&self, _prompt: &str) -> Result<String> {
            Ok("I need to search for information.\nTOOL: web_search {\"query\": \"test\"}".to_string())
        }
    }

    #[tokio::test]
    async fn test_agentic_agent_creation() {
        let config = AgenticLoopConfig::default();
        let llm_client = Arc::new(MockLLMClient);
        let _agent = AgenticAgent::new(config, llm_client);
    }

    #[test]
    fn test_extract_thought() {
        let agent = AgenticAgent::new(
            AgenticLoopConfig::default(),
            Arc::new(MockLLMClient),
        );
        let response = "I need to search.\nTOOL: search {}";
        let thought = agent.extract_thought(response);
        assert_eq!(thought, "I need to search.");
    }

    #[test]
    fn test_extract_completion() {
        let agent = AgenticAgent::new(
            AgenticLoopConfig::default(),
            Arc::new(MockLLMClient),
        );
        let response = "Task is done.\nCOMPLETE: The answer is 42";
        let completion = agent.extract_completion(response);
        assert_eq!(completion, Some("The answer is 42".to_string()));
    }

    #[test]
    fn test_extract_tool_call() {
        let agent = AgenticAgent::new(
            AgenticLoopConfig::default(),
            Arc::new(MockLLMClient),
        );
        let response = "TOOL: web_search {\"query\": \"test\"}";
        let tool_call = agent.extract_tool_call(response).unwrap();
        assert!(tool_call.is_some());
        let call = tool_call.unwrap();
        assert_eq!(call.tool_name, "web_search");
    }
}
