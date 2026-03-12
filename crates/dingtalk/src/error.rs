//! Error types for DingTalk channel integration.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    
    #[error("API error: {code} - {message}")]
    ApiError { code: i32, message: String },
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Channel error: {0}")]
    Channel(#[from] clawmaster_channels::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}
