# clawmaster-agent-loop

Agent Loop implementation inspired by MicroClaw, providing tool chaining and agentic execution capabilities.

## Features
- ✅ Agentic loop with tool execution
- ✅ Tool chaining and iteration limits
- ✅ Timeout and memory management
- ✅ Tool registry with dynamic registration
- ✅ Context-aware tool execution
- ✅ Error handling and recovery

## Usage
```rust
use clawmaster_agent_loop::{create_agent_loop, ToolContext};

let agent_loop = create_agent_loop();

let context = ToolContext::new(
    "session123".to_string(),
    "user123".to_string(),
    "channel123".to_string(),
    "Hello, echo this!".to_string(),
);

let result = agent_loop.execute_loop("echo hello".to_string(), context).await?;
println!("Response: {}", result.response);
```

## Architecture
```
Message → Agent Loop → Tool Decision → Tool Execution → Result → Next Iteration
    ↑                                                                 ↓
    └────────────────────── Response Generation ←─────────────────┘
```

## Configuration
```rust
use clawmaster_agent_loop::AgentLoopConfig;
use std::time::Duration;

let config = AgentLoopConfig {
    max_tool_iterations: 10,
    loop_timeout: Duration::from_secs(300),
    tool_timeout: Duration::from_secs(30),
    enable_tool_chaining: true,
    track_intermediate_results: true,
    max_intermediate_memory: 10 * 1024 * 1024, // 10MB
};
```
