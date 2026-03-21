//! Recovery Operations
//!
//! DO-178C Level A Compliant Recovery System

use {
    crate::{BackupError, BackupMetadata, BackupResult, BackupType},
    flate2::read::GzDecoder,
    std::{io::Read, path::Path},
};

/// Recovery manager
///
/// DO-178C §11.11: Recovery operations
pub struct RecoveryManager;

impl RecoveryManager {
    /// Create new recovery manager
    pub fn new() -> Self {
        Self
    }

    /// Recover data from backup
    ///
    /// DO-178C §11.11: Data recovery
    pub async fn recover(&self, metadata: &BackupMetadata, target_path: &Path) -> BackupResult<()> {
        // Read compressed backup
        let compressed_data = tokio::fs::read(&metadata.backup_path)
            .await
            .map_err(|e| BackupError::IoError(e.to_string()))?;

        // Decompress data
        let decompressed_data = self.decompress_data(&compressed_data)?;

        // Verify checksum
        self.verify_checksum(&decompressed_data, &metadata.checksum)?;

        // Write to target
        tokio::fs::write(target_path, &decompressed_data)
            .await
            .map_err(|e| BackupError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Recover with chain (for incremental backups)
    ///
    /// DO-178C §11.11: Chain recovery
    pub async fn recover_chain(
        &self,
        backups: &[BackupMetadata],
        target_path: &Path,
    ) -> BackupResult<()> {
        if backups.is_empty() {
            return Err(BackupError::InvalidBackup(
                "No backups provided".to_string(),
            ));
        }

        // Find full backup
        let full_backup = backups
            .iter()
            .find(|b| b.backup_type == BackupType::Full)
            .ok_or_else(|| BackupError::InvalidBackup("No full backup found".to_string()))?;

        // Recover full backup first
        self.recover(full_backup, target_path).await?;

        // Apply incremental backups in order
        let mut incrementals: Vec<_> = backups
            .iter()
            .filter(|b| b.backup_type == BackupType::Incremental)
            .collect();

        incrementals.sort_by_key(|b| b.created_at);

        for incremental in incrementals {
            self.recover(incremental, target_path).await?;
        }

        Ok(())
    }

    /// Decompress data using gzip
    fn decompress_data(&self, data: &[u8]) -> BackupResult<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| BackupError::CompressionError(e.to_string()))?;
        Ok(decompressed)
    }

    /// Verify checksum
    fn verify_checksum(&self, data: &[u8], expected: &str) -> BackupResult<()> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(data);
        let actual = format!("{:x}", hasher.finalize());

        if actual != expected {
            return Err(BackupError::VerificationError(format!(
                "Checksum mismatch: expected {}, got {}",
                expected, actual
            )));
        }

        Ok(())
    }
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::BackupManager, tempfile::TempDir};

    #[tokio::test]
    async fn test_recover_full_backup() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let backup_manager = BackupManager::new(backup_dir).unwrap();
        let recovery_manager = RecoveryManager::new();

        // Create source file
        let source_file = temp_dir.path().join("source.txt");
        let test_data = b"Hello, World!";
        tokio::fs::write(&source_file, test_data).await.unwrap();

        // Create backup
        let metadata = backup_manager
            .create_full_backup(&source_file)
            .await
            .unwrap();

        // Recover to new location
        let target_file = temp_dir.path().join("recovered.txt");
        recovery_manager
            .recover(&metadata, &target_file)
            .await
            .unwrap();

        // Verify recovered data
        let recovered_data = tokio::fs::read(&target_file).await.unwrap();
        assert_eq!(recovered_data, test_data);
    }

    #[tokio::test]
    async fn test_recover_chain() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let backup_manager = BackupManager::new(backup_dir).unwrap();
        let recovery_manager = RecoveryManager::new();

        // Create source file
        let source_file = temp_dir.path().join("source.txt");
        tokio::fs::write(&source_file, b"Version 1").await.unwrap();

        // Create full backup
        let full_metadata = backup_manager
            .create_full_backup(&source_file)
            .await
            .unwrap();

        // Update file and create incremental
        tokio::fs::write(&source_file, b"Version 2").await.unwrap();
        let inc_metadata = backup_manager
            .create_incremental_backup(&source_file, full_metadata.id)
            .await
            .unwrap();

        // Recover chain
        let target_file = temp_dir.path().join("recovered.txt");
        recovery_manager
            .recover_chain(&[full_metadata, inc_metadata], &target_file)
            .await
            .unwrap();

        // Verify final version
        let recovered_data = tokio::fs::read(&target_file).await.unwrap();
        assert_eq!(recovered_data, b"Version 2");
    }

    #[tokio::test]
    async fn test_verify_checksum_success() {
        let recovery_manager = RecoveryManager::new();
        let data = b"Test data";

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let checksum = format!("{:x}", hasher.finalize());

        let result = recovery_manager.verify_checksum(data, &checksum);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_checksum_failure() {
        let recovery_manager = RecoveryManager::new();
        let data = b"Test data";
        let wrong_checksum = "0".repeat(64);

        let result = recovery_manager.verify_checksum(data, &wrong_checksum);
        assert!(result.is_err());
    }
}
