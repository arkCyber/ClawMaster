//! Audit Logging System
//!
//! DO-178C Level A Compliant Audit Logging
//!
//! This module provides comprehensive audit logging including:
//! - Authentication events
//! - Authorization events
//! - Configuration changes
//! - Security events
//! - System events
//! - Structured JSON logging
//! - Log signature verification
//!
//! Compliance: DO-178C §11.9 - Audit trail

pub mod events;
pub mod logger;
pub mod storage;
pub mod signature;

pub use events::*;
pub use logger::*;
pub use storage::*;
pub use signature::*;

use thiserror::Error;

/// Audit log error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Audit result
pub type AuditResult<T = ()> = Result<T, AuditError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_error_display() {
        let err = AuditError::DatabaseError("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");

        let err = AuditError::SignatureError("invalid signature".to_string());
        assert_eq!(err.to_string(), "Signature error: invalid signature");
    }
}
