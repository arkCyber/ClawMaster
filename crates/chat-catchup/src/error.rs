//! Error types for chat catchup implementation

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatchupError {
    #[error("Message processing failed: {0}")]
    MessageProcessingFailed(#[from] MessageProcessingError),

    #[error("Channel not found: {0}")]
    ChannelNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Timeout exceeded: {0:?}")]
    TimeoutExceeded(std::time::Duration),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

#[derive(Debug, Error)]
pub enum MessageProcessingError {
    #[error("Failed to parse message: {0}")]
    ParseFailed(String),

    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),

    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    #[error("Content filtering failed: {0}")]
    ContentFilteringFailed(String),

    #[error("Clustering failed: {0}")]
    ClusteringFailed(String),

    #[error("Summarization failed: {0}")]
    SummarizationFailed(String),

    #[error("Context building failed: {0}")]
    ContextBuildingFailed(String),

    #[error("Memory limit exceeded: {0} bytes")]
    MemoryLimitExceeded(usize),
}

pub type Result<T> = std::result::Result<T, CatchupError>;
pub type MessageResult<T> = std::result::Result<T, MessageProcessingError>;
