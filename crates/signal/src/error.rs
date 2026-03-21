//! Signal channel error types

use thiserror::Error;

/// Signal channel error type
#[derive(Debug, Error)]
pub enum Error {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Message send error
    #[error("Failed to send message: {0}")]
    SendFailed(String),

    /// Message receive error
    #[error("Failed to receive message: {0}")]
    ReceiveFailed(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type for Signal operations
pub type Result<T> = std::result::Result<T, Error>;
