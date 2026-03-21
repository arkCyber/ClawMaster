//! Backup Retention Policies
//!
//! DO-178C Level A Compliant Retention Management

use {
    crate::{BackupManager, BackupMetadata, BackupResult, BackupType},
    std::sync::Arc,
    time::{Duration, OffsetDateTime},
};

/// Retention policy configuration
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Maximum number of full backups to keep
    pub max_full_backups: usize,

    /// Maximum number of incremental backups per full backup
    pub max_incremental_per_full: usize,

    /// Maximum age of backups
    pub max_age: Duration,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_full_backups: 7,          // Keep 7 full backups
            max_incremental_per_full: 24, // Keep 24 incrementals per full
            max_age: Duration::days(30),  // Keep backups for 30 days
        }
    }
}

/// Retention manager
///
/// DO-178C §11.11: Backup retention management
pub struct RetentionManager {
    policy: RetentionPolicy,
    backup_manager: Arc<BackupManager>,
}

impl RetentionManager {
    /// Create new retention manager
    pub fn new(policy: RetentionPolicy, backup_manager: Arc<BackupManager>) -> Self {
        Self {
            policy,
            backup_manager,
        }
    }

    /// Apply retention policy
    ///
    /// DO-178C §11.11: Retention enforcement
    pub async fn apply_policy(&self) -> BackupResult<usize> {
        let mut deleted_count = 0;

        // Get all backups
        let backups = self.backup_manager.list_backups().await?;

        // Apply age-based retention
        deleted_count += self.apply_age_retention(&backups).await?;

        // Apply count-based retention
        deleted_count += self.apply_count_retention(&backups).await?;

        Ok(deleted_count)
    }

    /// Apply age-based retention
    async fn apply_age_retention(&self, backups: &[BackupMetadata]) -> BackupResult<usize> {
        let now = OffsetDateTime::now_utc();
        let cutoff = now - self.policy.max_age;
        let mut deleted = 0;

        for backup in backups {
            if backup.created_at < cutoff {
                self.backup_manager.delete_backup(backup.id).await?;
                deleted += 1;
                tracing::info!("Deleted old backup: {} (age-based)", backup.id);
            }
        }

        Ok(deleted)
    }

    /// Apply count-based retention
    async fn apply_count_retention(&self, backups: &[BackupMetadata]) -> BackupResult<usize> {
        let mut deleted = 0;

        // Get full backups sorted by creation time (newest first)
        let mut full_backups: Vec<_> = backups
            .iter()
            .filter(|b| b.backup_type == BackupType::Full)
            .collect();
        full_backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Delete excess full backups
        if full_backups.len() > self.policy.max_full_backups {
            for backup in &full_backups[self.policy.max_full_backups..] {
                // Delete the full backup and its incrementals
                deleted += self.delete_backup_chain(backups, backup.id).await?;
                tracing::info!("Deleted excess full backup chain: {}", backup.id);
            }
        }

        // For each remaining full backup, limit incrementals
        for full_backup in &full_backups[..full_backups.len().min(self.policy.max_full_backups)] {
            deleted += self.limit_incrementals(backups, full_backup.id).await?;
        }

        Ok(deleted)
    }

    /// Delete backup and its incremental chain
    async fn delete_backup_chain(
        &self,
        all_backups: &[BackupMetadata],
        full_backup_id: uuid::Uuid,
    ) -> BackupResult<usize> {
        let mut deleted = 0;

        // Delete the full backup
        self.backup_manager.delete_backup(full_backup_id).await?;
        deleted += 1;

        // Delete all incrementals that depend on this full backup
        for backup in all_backups {
            if backup.parent_id == Some(full_backup_id) {
                self.backup_manager.delete_backup(backup.id).await?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Limit incrementals for a full backup
    async fn limit_incrementals(
        &self,
        all_backups: &[BackupMetadata],
        full_backup_id: uuid::Uuid,
    ) -> BackupResult<usize> {
        let mut deleted = 0;

        // Get incrementals for this full backup
        let mut incrementals: Vec<_> = all_backups
            .iter()
            .filter(|b| b.parent_id == Some(full_backup_id))
            .collect();

        // Sort by creation time (newest first)
        incrementals.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Delete excess incrementals
        if incrementals.len() > self.policy.max_incremental_per_full {
            for backup in &incrementals[self.policy.max_incremental_per_full..] {
                self.backup_manager.delete_backup(backup.id).await?;
                deleted += 1;
                tracing::info!("Deleted excess incremental backup: {}", backup.id);
            }
        }

        Ok(deleted)
    }

    /// Get retention statistics
    pub async fn get_statistics(&self) -> BackupResult<RetentionStatistics> {
        let backups = self.backup_manager.list_backups().await?;

        let full_count = backups
            .iter()
            .filter(|b| b.backup_type == BackupType::Full)
            .count();
        let incremental_count = backups
            .iter()
            .filter(|b| b.backup_type == BackupType::Incremental)
            .count();

        let total_size: u64 = backups.iter().map(|b| b.compressed_size).sum();

        let oldest = backups.iter().map(|b| b.created_at).min();
        let newest = backups.iter().map(|b| b.created_at).max();

        Ok(RetentionStatistics {
            total_backups: backups.len(),
            full_backups: full_count,
            incremental_backups: incremental_count,
            total_size_bytes: total_size,
            oldest_backup: oldest,
            newest_backup: newest,
        })
    }
}

/// Retention statistics
#[derive(Debug, Clone)]
pub struct RetentionStatistics {
    pub total_backups: usize,
    pub full_backups: usize,
    pub incremental_backups: usize,
    pub total_size_bytes: u64,
    pub oldest_backup: Option<OffsetDateTime>,
    pub newest_backup: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[tokio::test]
    async fn test_retention_policy_creation() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.max_full_backups, 7);
        assert_eq!(policy.max_incremental_per_full, 24);
    }

    #[tokio::test]
    async fn test_apply_count_retention() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = Arc::new(BackupManager::new(backup_dir).unwrap());

        let policy = RetentionPolicy {
            max_full_backups: 2,
            max_incremental_per_full: 5,
            max_age: Duration::days(30),
        };
        let retention = RetentionManager::new(policy, Arc::clone(&manager));

        // Create test file
        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test").await.unwrap();

        // Create 3 full backups (should keep only 2)
        for _ in 0..3 {
            manager.create_full_backup(&source_file).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        let deleted = retention.apply_policy().await.unwrap();
        assert!(deleted > 0);

        let remaining = manager.list_backups().await.unwrap();
        assert_eq!(remaining.len(), 2);
    }

    #[tokio::test]
    async fn test_get_statistics() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = Arc::new(BackupManager::new(backup_dir).unwrap());

        let policy = RetentionPolicy::default();
        let retention = RetentionManager::new(policy, Arc::clone(&manager));

        let source_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&source_file, b"Test data").await.unwrap();

        // Create backups
        let full = manager.create_full_backup(&source_file).await.unwrap();
        manager
            .create_incremental_backup(&source_file, full.id)
            .await
            .unwrap();

        let stats = retention.get_statistics().await.unwrap();
        assert_eq!(stats.total_backups, 2);
        assert_eq!(stats.full_backups, 1);
        assert_eq!(stats.incremental_backups, 1);
        assert!(stats.total_size_bytes > 0);
    }
}
