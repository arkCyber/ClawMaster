//! Configuration Validation and Security Checks
//!
//! DO-178C Level A Compliant Configuration Validation System
//!
//! This module provides comprehensive configuration validation including:
//! - Schema validation
//! - Security baseline checks
//! - Conflict detection
//! - Resource limit validation
//! - Path permission validation
//!
//! Compliance: DO-178C §11.13 - Configuration management and validation

pub mod rules;
pub mod validator;

pub use {rules::*, validator::*};

use thiserror::Error;

/// Configuration validation error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Configuration conflict: {0}")]
    Conflict(String),

    #[error("Invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Path permission error: {0}")]
    PathPermission(String),
}

/// Validation result
pub type ValidationResult<T = ()> = Result<T, ValidationError>;

/// Validation severity level
///
/// DO-178C §11.13: Severity classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Critical - prevents system startup
    Critical,

    /// Error - should be fixed but allows startup
    Error,

    /// Warning - should be reviewed
    Warning,

    /// Info - informational only
    Info,
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Severity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Severity::Critical, Severity::Critical) => Ordering::Equal,
            (Severity::Critical, _) => Ordering::Greater,
            (_, Severity::Critical) => Ordering::Less,
            (Severity::Error, Severity::Error) => Ordering::Equal,
            (Severity::Error, Severity::Warning) => Ordering::Greater,
            (Severity::Error, Severity::Info) => Ordering::Greater,
            (Severity::Warning, Severity::Error) => Ordering::Less,
            (Severity::Warning, Severity::Warning) => Ordering::Equal,
            (Severity::Warning, Severity::Info) => Ordering::Greater,
            (Severity::Info, Severity::Info) => Ordering::Equal,
            (Severity::Info, _) => Ordering::Less,
        }
    }
}

/// Validation issue
///
/// DO-178C §11.13: Detailed issue reporting
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationIssue {
    pub fn critical(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Critical,
            field: field.into(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            field: field.into(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn warning(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            field: field.into(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn info(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Info,
            field: field.into(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::Error);
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Warning > Severity::Info);
    }

    #[test]
    fn test_validation_issue_creation() {
        let issue = ValidationIssue::critical("test", "test message");
        assert_eq!(issue.severity, Severity::Critical);
        assert_eq!(issue.field, "test");
        assert_eq!(issue.message, "test message");
        assert!(issue.suggestion.is_none());

        let issue_with_suggestion =
            ValidationIssue::error("test", "test").with_suggestion("fix this");
        assert!(issue_with_suggestion.suggestion.is_some());
    }
}
