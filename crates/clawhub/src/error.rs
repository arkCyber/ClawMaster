//! Error types for ClawHub.

use thiserror::Error;

/// ClawHub error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Tool not found
    #[error("Tool not found: {name}@{version}")]
    ToolNotFound { name: String, version: String },

    /// Invalid tool metadata
    #[error("Invalid tool metadata: {0}")]
    InvalidMetadata(String),

    /// Security verification failed
    #[error("Security verification failed: {0}")]
    SecurityVerificationFailed(String),

    /// Signature verification failed
    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid version
    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    /// Tool already exists
    #[error("Tool already exists: {name}@{version}")]
    ToolAlreadyExists { name: String, version: String },

    /// Unauthorized
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

/// Result type for ClawHub operations.
pub type Result<T> = std::result::Result<T, Error>;
