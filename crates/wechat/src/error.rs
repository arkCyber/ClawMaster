//! Error types for WeChat channel integration.

use thiserror::Error;

/// Result type for WeChat operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for WeChat channel operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("XML parsing failed: {0}")]
    Xml(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("API error: {code} - {message}")]
    ApiError { code: i32, message: String },

    #[error("Access token expired")]
    TokenExpired,

    #[error("Message send failed: {0}")]
    SendFailed(String),

    #[error("Message receive failed: {0}")]
    ReceiveFailed(String),

    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Channel error: {0}")]
    Channel(#[from] clawmaster_channels::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
