//! Input Validation and Sanitization
//!
//! DO-178C Level A Compliant Input Validation System
//!
//! This module provides comprehensive input validation and sanitization including:
//! - User message validation (XSS, SQL injection)
//! - File path validation (traversal, null bytes)
//! - Command validation (shell injection)
//! - API parameter validation
//! - File upload validation
//! - Output encoding utilities
//!
//! Compliance: DO-178C §6.3.1 - Input validation and security

pub mod command;
pub mod file;
pub mod message;
pub mod parameter;
pub mod sanitize;

pub use {command::*, file::*, message::*, parameter::*, sanitize::*};

use thiserror::Error;

/// Input validation error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid input: {0}")]
    Invalid(String),

    #[error("Potentially dangerous input detected: {0}")]
    Dangerous(String),

    #[error("Input too long: {actual} bytes (max: {max})")]
    TooLong { actual: usize, max: usize },

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Forbidden character detected: {0}")]
    ForbiddenChar(String),

    #[error("Path traversal attempt detected")]
    PathTraversal,

    #[error("Null byte detected")]
    NullByte,

    #[error("Shell injection attempt detected")]
    ShellInjection,

    #[error("XSS attempt detected")]
    XssAttempt,

    #[error("SQL injection attempt detected")]
    SqlInjection,
}

/// Validation result
pub type ValidationResult<T = ()> = Result<T, ValidationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::Invalid("test".to_string());
        assert_eq!(err.to_string(), "Invalid input: test");

        let err = ValidationError::TooLong {
            actual: 100,
            max: 50,
        };
        assert_eq!(err.to_string(), "Input too long: 100 bytes (max: 50)");
    }
}
