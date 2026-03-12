//! Agent Loop implementation inspired by MicroClaw
//! 
//! This crate provides an agentic loop that can chain tool executions
//! until a task is completed, similar to MicroClaw's approach.

pub mod config;
pub mod error;
pub mod executor;
pub mod loop_engine;
pub mod tool_registry;

pub use config::{AgentLoopConfig, ToolIterationConfig};
pub use error::{AgentLoopError, ToolExecutionError};
pub use executor::ToolExecutor;
pub use loop_engine::{AgentLoop, LoopState, LoopResult};
pub use tool_registry::{ToolRegistry, Tool, ToolContext};

use std::sync::Arc;

/// Main agent loop interface
#[async_trait::async_trait]
pub trait AgentLoopInterface: Send + Sync {
    /// Execute the agent loop with a given message
    async fn execute_loop(&self, message: String, context: ToolContext) -> Result<LoopResult, AgentLoopError>;
    
    /// Get current loop state
    fn get_state(&self) -> LoopState;
    
    /// Stop the current loop execution
    async fn stop_loop(&self) -> Result<(), AgentLoopError>;
}

/// Create a new agent loop with default configuration
pub fn create_agent_loop() -> Arc<AgentLoop> {
    let config = AgentLoopConfig::default();
    Arc::new(AgentLoop::new(config))
}

/// Create a new agent loop with custom configuration
pub fn create_agent_loop_with_config(config: AgentLoopConfig) -> Arc<AgentLoop> {
    Arc::new(AgentLoop::new(config))
}
