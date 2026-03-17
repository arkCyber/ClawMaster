//! Error types for authentication and credential management.
//!
//! DO-178C Level A compliant error handling - all errors must be explicitly handled.

use thiserror::Error;

/// Authentication and credential management errors.
#[derive(Error, Debug)]
pub enum AuthError {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(String),

    /// Failed to read or write credential file
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to serialize or deserialize credentials
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Password hashing failed
    #[error("Password hashing failed: {0}")]
    PasswordHash(String),

    /// Password verification failed
    #[error("Password verification failed: {0}")]
    PasswordVerify(String),

    /// Invalid password format or strength
    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    /// Setup already completed
    #[error("Setup already completed")]
    SetupAlreadyComplete,

    /// Setup not completed
    #[error("Setup not completed")]
    SetupNotComplete,

    /// Session not found or invalid
    #[error("Invalid session: {0}")]
    InvalidSession(String),

    /// API key not found or invalid
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    /// Passkey operation failed
    #[error("Passkey error: {0}")]
    Passkey(String),

    /// WebAuthn operation failed
    #[error("WebAuthn error: {0}")]
    WebAuthn(String),

    /// Vault operation failed (encryption/decryption)
    #[error("Vault error: {0}")]
    Vault(String),

    /// Environment variable operation failed
    #[error("Environment variable error: {0}")]
    EnvVar(String),

    /// Invalid scope specified
    #[error("Invalid scope: {0}")]
    InvalidScope(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Internal error (should not happen in production)
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for authentication operations.
pub type Result<T> = std::result::Result<T, AuthError>;

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::Database(err.to_string())
    }
}

impl From<password_hash::Error> for AuthError {
    fn from(err: password_hash::Error) -> Self {
        AuthError::PasswordHash(err.to_string())
    }
}

#[cfg(feature = "vault")]
impl From<clawmaster_vault::VaultError> for AuthError {
    fn from(err: clawmaster_vault::VaultError) -> Self {
        AuthError::Vault(err.to_string())
    }
}

impl From<webauthn_rs::prelude::WebauthnError> for AuthError {
    fn from(err: webauthn_rs::prelude::WebauthnError) -> Self {
        AuthError::WebAuthn(err.to_string())
    }
}

// Implement conversion from anyhow::Error for compatibility
impl From<anyhow::Error> for AuthError {
    fn from(err: anyhow::Error) -> Self {
        AuthError::Internal(err.to_string())
    }
}
