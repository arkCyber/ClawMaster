//! Demonstration of the Agentic Loop integration
//!
//! This example shows how to use the agentic loop functionality
//! to create an autonomous agent that can execute multi-step tasks.

use {
    anyhow::Result,
    async_trait::async_trait,
    clawmaster_agentic_loop::{AgenticLoopConfig, Tool},
    clawmaster_agents::agentic_loop::{AgenticAgent, LLMClient},
    serde_json::Value,
    std::sync::Arc,
};

/// Example LLM client that simulates reasoning
struct ExampleLLMClient;

#[async_trait]
impl LLMClient for ExampleLLMClient {
    async fn reason(&self, prompt: &str) -> Result<String> {
        println!("=== LLM Reasoning ===");
        println!("Prompt: {}", prompt);

        // Simulate LLM response
        let response = if prompt.contains("search") {
            "I need to search for information about Rust.\nTOOL: web_search {\"query\": \"Rust programming language\"}"
        } else if prompt.contains("Last tool result") {
            "I found the information. Now I'll complete the task.\nCOMPLETE: Rust is a systems programming language focused on safety and performance."
        } else {
            "I should search for information first.\nTOOL: web_search {\"query\": \"Rust\"}"
        };

        println!("Response: {}", response);
        println!();

        Ok(response.to_string())
    }
}

/// Example web search tool
struct WebSearchTool;

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web for information"
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let query = args["query"].as_str().unwrap_or("unknown");
        println!("🔍 Executing web search: {}", query);

        // Simulate search results
        let result = format!(
            "Search results for '{}': Found information about Rust programming language",
            query
        );

        println!("✅ Search completed: {}", result);
        println!();

        Ok(result)
    }
}

/// Example file read tool
struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read contents of a file"
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"].as_str().unwrap_or("unknown");
        println!("📖 Reading file: {}", path);

        // Simulate file read
        let content = format!("Contents of {}", path);

        println!("✅ File read completed");
        println!();

        Ok(content)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🤖 Agentic Loop Demo\n");
    println!("This demo shows how an agent can autonomously execute multi-step tasks.\n");

    // Create configuration
    let config = AgenticLoopConfig {
        max_iterations: 5,
        timeout_seconds: 60,
        enable_memory: true,
    };

    // Create LLM client
    let llm_client = Arc::new(ExampleLLMClient);

    // Create agentic agent
    let agent = AgenticAgent::new(config, llm_client);

    // Register tools
    println!("📋 Registering tools...");
    agent.register_tool(Box::new(WebSearchTool));
    agent.register_tool(Box::new(ReadFileTool));
    println!("✅ Tools registered\n");

    // Execute a task
    let task = "Find information about Rust programming language";
    println!("🎯 Task: {}\n", task);

    println!("🔄 Starting autonomous execution...\n");

    match agent.execute_task(task).await {
        Ok(result) => {
            println!("✅ Task completed successfully!");
            println!("📝 Final result: {}", result);
        },
        Err(e) => {
            println!("❌ Task failed: {}", e);
        },
    }

    Ok(())
}
