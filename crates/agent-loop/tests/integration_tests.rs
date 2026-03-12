//! Integration tests for agent loop

use clawmaster_agent_loop::{create_agent_loop_with_config, AgentLoopConfig, ToolContext};
use std::time::Duration;
use std::sync::Arc;
use clawmaster_agent_loop::tool_registry::{Tool, ToolRegistry, ToolExecutionResult};
use async_trait::async_trait;
use serde_json::Value;

#[tokio::test]
async fn test_agent_loop_with_custom_tool() {
    #[derive(Debug)]
    struct CounterTool {
        count: Arc<std::sync::atomic::AtomicUsize>,
    }
    
    #[async_trait]
    impl Tool for CounterTool {
        fn name(&self) -> &'static str {
            "counter"
        }
        
        fn description(&self) -> &'static str {
            "Counts how many times it's been called"
        }
        
        fn argument_schema(&self) -> Value {
            serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            })
        }
        
        async fn execute(&self, _arguments: Value, _context: ToolContext) -> clawmaster_agent_loop::tool_registry::ToolResult<ToolExecutionResult> {
            let count = self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(ToolExecutionResult::success(serde_json::json!({
                "count": count + 1
            })))
        }
    }
    
    let config = AgentLoopConfig {
        max_tool_iterations: 5,
        loop_timeout: Duration::from_secs(10),
        tool_timeout: Duration::from_secs(1),
        enable_tool_chaining: true,
        track_intermediate_results: true,
        max_intermediate_memory: 1024 * 1024,
    };
    
    let agent_loop = create_agent_loop_with_config(config);
    
    // Register custom tool
    let counter_tool = Arc::new(CounterTool {
        count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });
    agent_loop.tool_registry().register_tool(counter_tool.clone());
    
    let context = ToolContext::new(
        "session123".to_string(),
        "user123".to_string(),
        "channel123".to_string(),
        "counter please".to_string(),
    );
    
    let result = agent_loop.execute_loop("counter".to_string(), context).await;
    
    assert!(result.is_ok());
    let loop_result = result.unwrap();
    assert!(loop_result.success);
    assert_eq!(loop_result.iterations_used, 1);
}

#[tokio::test]
async fn test_agent_loop_timeout() {
    let config = AgentLoopConfig {
        max_tool_iterations: 100,
        loop_timeout: Duration::from_millis(100),
        tool_timeout: Duration::from_secs(30),
        enable_tool_chaining: true,
        track_intermediate_results: true,
        max_intermediate_memory: 1024 * 1024,
    };
    
    let agent_loop = create_agent_loop_with_config(config);
    
    let context = ToolContext::new(
        "session123".to_string(),
        "user123".to_string(),
        "channel123".to_string(),
        "test message".to_string(),
    );
    
    let result = agent_loop.execute_loop("test".to_string(), context).await;
    
    // Should complete quickly since no tools are called
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agent_loop_memory_limits() {
    let config = AgentLoopConfig {
        max_tool_iterations: 10,
        loop_timeout: Duration::from_secs(10),
        tool_timeout: Duration::from_secs(1),
        enable_tool_chaining: true,
        track_intermediate_results: true,
        max_intermediate_memory: 100, // Very small limit
    };
    
    let agent_loop = create_agent_loop_with_config(config);
    
    let context = ToolContext::new(
        "session123".to_string(),
        "user123".to_string(),
        "channel123".to_string(),
        "test message".to_string(),
    );
    
    let result = agent_loop.execute_loop("test".to_string(), context).await;
    
    // Should complete without memory issues since no tools are called
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_concurrent_agent_loops() {
    let config = AgentLoopConfig::default();
    let agent_loop1 = create_agent_loop_with_config(config.clone());
    let agent_loop2 = create_agent_loop_with_config(config);
    
    let context1 = ToolContext::new(
        "session1".to_string(),
        "user1".to_string(),
        "channel1".to_string(),
        "message1".to_string(),
    );
    
    let context2 = ToolContext::new(
        "session2".to_string(),
        "user2".to_string(),
        "channel2".to_string(),
        "message2".to_string(),
    );
    
    let future1 = agent_loop1.execute_loop("test".to_string(), context1);
    let future2 = agent_loop2.execute_loop("test".to_string(), context2);
    
    let (result1, result2) = tokio::join!(future1, future2);
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
