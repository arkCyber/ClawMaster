//! Error types for IRC channel integration.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IRC error: {0}")]
    Irc(#[from] irc::error::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Channel error: {0}")]
    Channel(#[from] clawmaster_channels::Error),

    #[error("Other error: {0}")]
    Other(String),
}
