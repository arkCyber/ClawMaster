//! Basic example of using the agent loop

use clawmaster_agent_loop::{create_agent_loop_with_config, AgentLoopConfig, ToolContext};
use std::time::Duration;
use std::sync::Arc;
use clawmaster_agent_loop::tool_registry::{Tool, ToolExecutionResult};
use async_trait::async_trait;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Agent Loop Example ===\n");
    
    // Create agent loop with custom configuration
    let config = AgentLoopConfig {
        max_tool_iterations: 5,
        loop_timeout: Duration::from_secs(30),
        tool_timeout: Duration::from_secs(5),
        enable_tool_chaining: true,
        track_intermediate_results: true,
        max_intermediate_memory: 1024 * 1024, // 1MB
    };
    
    let agent_loop = create_agent_loop_with_config(config);
    
    // Register a custom tool
    let calculator_tool = Arc::new(CalculatorTool);
    agent_loop.tool_registry().register_tool(calculator_tool);
    
    // Create context
    let context = ToolContext::new(
        "session123".to_string(),
        "user123".to_string(),
        "channel123".to_string(),
        "What is 5 + 3?".to_string(),
    );
    
    // Execute the agent loop
    println!("Executing agent loop with message: 'What is 5 + 3?'");
    let result = agent_loop.execute_loop("calculate 5 + 3".to_string(), context).await?;
    
    println!("\n=== Results ===");
    println!("Success: {}", result.success);
    println!("Response: {}", result.response);
    println!("Iterations used: {}", result.iterations_used);
    println!("Execution time: {:?}", result.execution_time);
    println!("Tool results: {}", result.tool_results.len());
    
    for (i, tool_result) in result.tool_results.iter().enumerate() {
        println!("  Tool {}: {} - Success: {}", 
                 i + 1, 
                 tool_result.data.as_ref().unwrap_or(&Value::Null), 
                 tool_result.success);
    }
    
    Ok(())
}

/// A simple calculator tool for demonstration
#[derive(Debug)]
struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &'static str {
        "calculator"
    }
    
    fn description(&self) -> &'static str {
        "Performs simple mathematical operations"
    }
    
    fn argument_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate (e.g., '5 + 3')"
                }
            },
            "required": ["expression"]
        })
    }
    
    async fn execute(&self, arguments: Value, _context: ToolContext) -> clawmaster_agent_loop::tool_registry::ToolResult<ToolExecutionResult> {
        let expression = arguments
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| clawmaster_agent_loop::error::ToolExecutionError::InvalidArguments(
                "Missing 'expression' argument".to_string()
            ))?;
        
        // Simple expression parsing (just support basic addition for demo)
        if let Some((left_str, right_str)) = expression.split_once('+') {
            let left: f64 = left_str.trim().parse()
                .map_err(|_| clawmaster_agent_loop::error::ToolExecutionError::InvalidArguments(
                    "Invalid left operand".to_string()
                ))?;
            let right: f64 = right_str.trim().parse()
                .map_err(|_| clawmaster_agent_loop::error::ToolExecutionError::InvalidArguments(
                    "Invalid right operand".to_string()
                ))?;
            
            let result = left + right;
            
            Ok(ToolExecutionResult::success(serde_json::json!({
                "expression": expression,
                "result": result
            })))
        } else {
            Err(clawmaster_agent_loop::error::ToolExecutionError::InvalidArguments(
                "Only addition is supported in this demo".to_string()
            ))
        }
    }
}
