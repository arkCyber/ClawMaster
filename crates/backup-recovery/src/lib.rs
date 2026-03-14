//! Backup and Recovery System
//!
//! DO-178C Level A Compliant Backup and Recovery
//!
//! This module provides comprehensive backup and recovery including:
//! - Automatic backup scheduling
//! - Incremental backups
//! - Full backups
//! - Backup verification
//! - Recovery operations
//! - Retention policies
//!
//! Compliance: DO-178C §11.11 - Data backup and recovery

pub mod backup;
pub mod recovery;
pub mod scheduler;
pub mod verification;
pub mod retention;

pub use backup::*;
pub use recovery::*;
pub use scheduler::*;
pub use verification::*;
pub use retention::*;

use thiserror::Error;

/// Backup error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum BackupError {
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Compression error: {0}")]
    CompressionError(String),
    
    #[error("Verification error: {0}")]
    VerificationError(String),
    
    #[error("Recovery error: {0}")]
    RecoveryError(String),
    
    #[error("Backup not found: {0}")]
    BackupNotFound(String),
    
    #[error("Invalid backup: {0}")]
    InvalidBackup(String),
}

/// Backup result
pub type BackupResult<T = ()> = Result<T, BackupError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_error_display() {
        let err = BackupError::IoError("file not found".to_string());
        assert_eq!(err.to_string(), "IO error: file not found");

        let err = BackupError::VerificationError("checksum mismatch".to_string());
        assert_eq!(err.to_string(), "Verification error: checksum mismatch");
    }
}
