//! Backup Verification
//!
//! DO-178C Level A Compliant Verification System

use crate::{BackupError, BackupMetadata, BackupResult};
use sha2::{Digest, Sha256};
use std::io::Read;

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Whether verification passed
    pub passed: bool,
    
    /// Checksum matches
    pub checksum_valid: bool,
    
    /// File exists
    pub file_exists: bool,
    
    /// Size matches
    pub size_matches: bool,
    
    /// Error message if failed
    pub error: Option<String>,
}

/// Backup verifier
///
/// DO-178C §11.11: Backup verification
pub struct BackupVerifier;

impl BackupVerifier {
    /// Create new verifier
    pub fn new() -> Self {
        Self
    }

    /// Verify backup integrity
    ///
    /// DO-178C §11.11: Integrity verification
    pub async fn verify(&self, metadata: &BackupMetadata) -> BackupResult<VerificationResult> {
        let mut result = VerificationResult {
            passed: true,
            checksum_valid: false,
            file_exists: false,
            size_matches: false,
            error: None,
        };

        // Check if backup file exists
        if !metadata.backup_path.exists() {
            result.passed = false;
            result.error = Some("Backup file not found".to_string());
            return Ok(result);
        }
        result.file_exists = true;

        // Read backup file
        let backup_data = match tokio::fs::read(&metadata.backup_path).await {
            Ok(data) => data,
            Err(e) => {
                result.passed = false;
                result.error = Some(format!("Failed to read backup: {}", e));
                return Ok(result);
            }
        };

        // Verify size
        if backup_data.len() as u64 != metadata.compressed_size {
            result.passed = false;
            result.error = Some(format!(
                "Size mismatch: expected {}, got {}",
                metadata.compressed_size,
                backup_data.len()
            ));
            return Ok(result);
        }
        result.size_matches = true;

        // Decompress and verify checksum
        let decompressed = match self.decompress_data(&backup_data) {
            Ok(data) => data,
            Err(e) => {
                result.passed = false;
                result.error = Some(format!("Decompression failed: {}", e));
                return Ok(result);
            }
        };

        let actual_checksum = self.calculate_checksum(&decompressed);
        if actual_checksum != metadata.checksum {
            result.passed = false;
            result.error = Some(format!(
                "Checksum mismatch: expected {}, got {}",
                metadata.checksum, actual_checksum
            ));
            return Ok(result);
        }
        result.checksum_valid = true;

        Ok(result)
    }

    /// Verify multiple backups
    pub async fn verify_all(
        &self,
        backups: &[BackupMetadata],
    ) -> BackupResult<Vec<(uuid::Uuid, VerificationResult)>> {
        let mut results = Vec::new();

        for backup in backups {
            let result = self.verify(backup).await?;
            results.push((backup.id, result));
        }

        Ok(results)
    }

    /// Decompress data
    fn decompress_data(&self, data: &[u8]) -> BackupResult<Vec<u8>> {
        use flate2::read::GzDecoder;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| BackupError::CompressionError(e.to_string()))?;
        Ok(decompressed)
    }

    /// Calculate checksum
    fn calculate_checksum(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

impl Default for BackupVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BackupManager;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_verify_valid_backup() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = BackupManager::new(backup_dir).unwrap();
        let verifier = BackupVerifier::new();

        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test data").await.unwrap();

        let metadata = manager.create_full_backup(&source_file).await.unwrap();

        let result = verifier.verify(&metadata).await.unwrap();
        assert!(result.passed);
        assert!(result.checksum_valid);
        assert!(result.file_exists);
        assert!(result.size_matches);
        assert!(result.error.is_none());
    }

    #[tokio::test]
    async fn test_verify_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = BackupManager::new(backup_dir).unwrap();
        let verifier = BackupVerifier::new();

        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test data").await.unwrap();

        let mut metadata = manager.create_full_backup(&source_file).await.unwrap();

        // Delete backup file
        tokio::fs::remove_file(&metadata.backup_path).await.unwrap();

        let result = verifier.verify(&metadata).await.unwrap();
        assert!(!result.passed);
        assert!(!result.file_exists);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_verify_corrupted_checksum() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = BackupManager::new(backup_dir).unwrap();
        let verifier = BackupVerifier::new();

        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test data").await.unwrap();

        let mut metadata = manager.create_full_backup(&source_file).await.unwrap();

        // Corrupt checksum
        metadata.checksum = "0".repeat(64);

        let result = verifier.verify(&metadata).await.unwrap();
        assert!(!result.passed);
        assert!(!result.checksum_valid);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_verify_all() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = BackupManager::new(backup_dir).unwrap();
        let verifier = BackupVerifier::new();

        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test").await.unwrap();

        let metadata1 = manager.create_full_backup(&source_file).await.unwrap();
        let metadata2 = manager.create_full_backup(&source_file).await.unwrap();

        let results = verifier.verify_all(&[metadata1, metadata2]).await.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].1.passed);
        assert!(results[1].1.passed);
    }
}
