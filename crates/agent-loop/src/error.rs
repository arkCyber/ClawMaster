//! Error types for agent loop implementation

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentLoopError {
    #[error("Tool execution failed: {0}")]
    ToolExecutionFailed(#[from] ToolExecutionError),
    
    #[error("Maximum tool iterations exceeded: {0}")]
    MaxIterationsExceeded(usize),
    
    #[error("Loop timeout exceeded: {0:?}")]
    LoopTimeoutExceeded(std::time::Duration),
    
    #[error("Invalid loop state: {0}")]
    InvalidLoopState(String),
    
    #[error("Context initialization failed: {0}")]
    ContextInitializationFailed(String),
    
    #[error("Message processing failed: {0}")]
    MessageProcessingFailed(String),
    
    #[error("Loop already running")]
    LoopAlreadyRunning,
    
    #[error("Loop not running")]
    LoopNotRunning,
}

#[derive(Debug, Error)]
pub enum ToolExecutionError {
    #[error("Tool '{0}' execution failed: {1}")]
    ExecutionFailed(String, String),
    
    #[error("Tool '{0}' timed out after {1:?}")]
    Timeout(String, std::time::Duration),
    
    #[error("Tool '{0}' not found")]
    ToolNotFound(String),
    
    #[error("Tool '{0}' is not available in current context")]
    ToolNotAvailable(String),
    
    #[error("Invalid tool arguments: {0}")]
    InvalidArguments(String),
    
    #[error("Tool '{0}' returned invalid result: {1}")]
    InvalidResult(String, String),
    
    #[error("Tool '{0}' panicked: {1}")]
    ToolPanicked(String, String),
}

pub type Result<T> = std::result::Result<T, AgentLoopError>;
pub type ToolResult<T> = std::result::Result<T, ToolExecutionError>;
