//! Resource Quota Management
//!
//! DO-178C Level A Compliant Resource Quota and Rate Limiting System
//!
//! This module provides comprehensive resource management including:
//! - Rate limiting (API requests)
//! - Memory quota management
//! - Connection pool limits
//! - Concurrent sessions limit
//! - File upload size limits
//!
//! Compliance: DO-178C §11.10 - Resource management

pub mod connection_limit;
pub mod memory_quota;
pub mod rate_limiter;
pub mod session_limit;
pub mod upload_limit;

pub use {
    connection_limit::*, memory_quota::*, rate_limiter::*, session_limit::*, upload_limit::*,
};

use thiserror::Error;

/// Resource quota error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum QuotaError {
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Memory quota exceeded: {used} bytes used, {limit} bytes limit")]
    MemoryQuotaExceeded { used: usize, limit: usize },

    #[error("Connection limit exceeded: {current} connections, {limit} max")]
    ConnectionLimitExceeded { current: usize, limit: usize },

    #[error("Session limit exceeded: {current} sessions, {limit} max")]
    SessionLimitExceeded { current: usize, limit: usize },

    #[error("Upload size limit exceeded: {size} bytes, {limit} bytes max")]
    UploadSizeExceeded { size: usize, limit: usize },

    #[error("Quota not available")]
    QuotaNotAvailable,
}

/// Quota result
pub type QuotaResult<T = ()> = Result<T, QuotaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_error_display() {
        let err = QuotaError::RateLimitExceeded("too many requests".to_string());
        assert_eq!(err.to_string(), "Rate limit exceeded: too many requests");

        let err = QuotaError::MemoryQuotaExceeded {
            used: 1000,
            limit: 500,
        };
        assert_eq!(
            err.to_string(),
            "Memory quota exceeded: 1000 bytes used, 500 bytes limit"
        );
    }
}
