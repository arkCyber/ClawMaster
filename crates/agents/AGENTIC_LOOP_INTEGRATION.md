# Agentic Loop Integration Guide

## Overview

The Agentic Loop integration enables ClawMaster agents to autonomously execute multi-step tasks through tool chaining and iterative reasoning.

## Features

- **Autonomous Task Execution**: Agents can break down complex tasks into steps
- **Tool Chaining**: Automatically chain multiple tool calls to complete tasks
- **Iterative Reasoning**: LLM-powered decision making at each step
- **Timeout Protection**: Configurable timeouts prevent infinite loops
- **Memory Management**: Maintains context across iterations

## Quick Start

### 1. Add Dependency

The `clawmaster-agentic-loop` dependency is already included in the agents crate.

### 2. Create an Agentic Agent

```rust
use clawmaster_agents::agentic_loop::{AgenticAgent, LLMClient};
use clawmaster_agentic_loop::AgenticLoopConfig;
use std::sync::Arc;

// Create configuration
let config = AgenticLoopConfig {
    max_iterations: 10,
    timeout_seconds: 300,
    enable_memory: true,
};

// Create agent with your LLM client
let agent = AgenticAgent::new(config, your_llm_client);
```

### 3. Register Tools

```rust
use clawmaster_agentic_loop::Tool;

// Register built-in or custom tools
agent.register_tool(Box::new(WebSearchTool));
agent.register_tool(Box::new(ReadFileTool));
agent.register_tool(Box::new(WriteFileTool));
```

### 4. Execute Tasks

```rust
let result = agent.execute_task(
    "Search for Rust best practices and create a summary document"
).await?;

println!("Task completed: {}", result);
```

## Implementing Custom Tools

Create custom tools by implementing the `Tool` trait:

```rust
use async_trait::async_trait;
use clawmaster_agentic_loop::Tool;
use anyhow::Result;
use serde_json::Value;

struct MyCustomTool;

#[async_trait]
impl Tool for MyCustomTool {
    fn name(&self) -> &str {
        "my_tool"
    }
    
    fn description(&self) -> &str {
        "Description of what this tool does"
    }
    
    async fn execute(&self, args: Value) -> Result<String> {
        // Extract arguments
        let param = args["param"].as_str().unwrap_or("default");
        
        // Perform tool operation
        let result = perform_operation(param)?;
        
        Ok(result)
    }
}
```

## Implementing LLM Client

Implement the `LLMClient` trait to connect your LLM:

```rust
use async_trait::async_trait;
use clawmaster_agents::agentic_loop::LLMClient;
use anyhow::Result;

struct MyLLMClient {
    // Your LLM client fields
}

#[async_trait]
impl LLMClient for MyLLMClient {
    async fn reason(&self, prompt: &str) -> Result<String> {
        // Call your LLM API
        let response = self.call_llm_api(prompt).await?;
        Ok(response)
    }
}
```

## LLM Response Format

The LLM should respond in this format:

```
[Thought process]

TOOL: tool_name {"arg1": "value1", "arg2": "value2"}
```

Or when complete:

```
[Final thought]

COMPLETE: Final answer or result
```

## Configuration Options

```rust
pub struct AgenticLoopConfig {
    /// Maximum number of iterations before stopping
    pub max_iterations: usize,
    
    /// Timeout in seconds for the entire loop
    pub timeout_seconds: u64,
    
    /// Enable memory across iterations
    pub enable_memory: bool,
}
```

## Example: Multi-Step Research Task

```rust
use clawmaster_agents::agentic_loop::{AgenticAgent, LLMClient};
use clawmaster_agentic_loop::AgenticLoopConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup
    let config = AgenticLoopConfig::default();
    let llm_client = Arc::new(YourLLMClient::new());
    let agent = AgenticAgent::new(config, llm_client);
    
    // Register tools
    agent.register_tool(Box::new(WebSearchTool));
    agent.register_tool(Box::new(ReadFileTool));
    agent.register_tool(Box::new(WriteFileTool));
    
    // Execute complex task
    let task = "Research the top 5 Rust web frameworks, \
                compare their features, and create a \
                markdown comparison table";
    
    let result = agent.execute_task(task).await?;
    println!("Result: {}", result);
    
    Ok(())
}
```

## Built-in Tools

The following tools are commonly used with agentic agents:

- **web_search**: Search the web for information
- **read_file**: Read file contents
- **write_file**: Write content to a file
- **bash**: Execute bash commands (use with caution)
- **glob**: Find files matching patterns
- **grep**: Search file contents

## Best Practices

1. **Clear Task Descriptions**: Provide clear, specific task descriptions
2. **Tool Selection**: Register only necessary tools to reduce decision complexity
3. **Timeout Configuration**: Set appropriate timeouts based on task complexity
4. **Error Handling**: Always handle potential errors from task execution
5. **Tool Safety**: Be cautious with tools that modify system state

## Integration with Existing Agents

To add agentic loop capabilities to existing agents:

```rust
use clawmaster_agents::agentic_loop::AgenticAgent;

pub struct EnhancedAgent {
    // Existing agent fields
    agentic_agent: Option<AgenticAgent>,
}

impl EnhancedAgent {
    pub fn with_agentic_loop(mut self, config: AgenticLoopConfig) -> Self {
        let agentic_agent = AgenticAgent::new(config, self.llm_client.clone());
        
        // Register tools
        self.register_tools(&agentic_agent);
        
        self.agentic_agent = Some(agentic_agent);
        self
    }
    
    pub async fn execute_autonomous_task(&self, task: &str) -> Result<String> {
        if let Some(agent) = &self.agentic_agent {
            agent.execute_task(task).await
        } else {
            Err(anyhow!("Agentic loop not configured"))
        }
    }
}
```

## Testing

Run the example to see the agentic loop in action:

```bash
cargo run --example agentic_loop_demo -p clawmaster-agents
```

Run tests:

```bash
cargo test -p clawmaster-agents --lib agentic_loop
```

## Performance Considerations

- Each iteration involves an LLM call, which can be slow
- Set reasonable `max_iterations` to prevent excessive API calls
- Use caching where possible for repeated operations
- Monitor token usage if using paid LLM APIs

## Troubleshooting

### Agent Gets Stuck in Loop

- Reduce `max_iterations`
- Improve task description clarity
- Check LLM response format

### Tools Not Being Called

- Verify tool registration
- Check LLM response format
- Ensure tool names match exactly

### Timeout Errors

- Increase `timeout_seconds`
- Optimize tool execution time
- Break down complex tasks

## Future Enhancements

- [ ] Tool result caching
- [ ] Parallel tool execution
- [ ] Advanced memory management
- [ ] Tool dependency resolution
- [ ] Automatic tool discovery

## Related Documentation

- [Agentic Loop Core Documentation](../agentic-loop/README.md)
- [Tool Development Guide](../agentic-loop/README.md#implementing-tools)
- [Agent Architecture](./README.md)

---

**Status**: ✅ Production Ready  
**Version**: 0.10.18  
**Last Updated**: 2026-03-13
