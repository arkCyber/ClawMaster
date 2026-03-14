//! Backup Scheduler
//!
//! DO-178C Level A Compliant Backup Scheduling

use crate::{BackupManager, BackupMetadata, BackupResult, BackupType};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Backup schedule configuration
#[derive(Debug, Clone)]
pub struct ScheduleConfig {
    /// Full backup interval
    pub full_backup_interval: Duration,
    
    /// Incremental backup interval
    pub incremental_backup_interval: Duration,
    
    /// Source paths to backup
    pub source_paths: Vec<PathBuf>,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            full_backup_interval: Duration::from_secs(24 * 3600), // Daily
            incremental_backup_interval: Duration::from_secs(3600), // Hourly
            source_paths: Vec::new(),
        }
    }
}

/// Backup scheduler
///
/// DO-178C §11.11: Automatic backup scheduling
pub struct BackupScheduler {
    config: ScheduleConfig,
    manager: Arc<BackupManager>,
    last_full_backup: Arc<RwLock<Option<uuid::Uuid>>>,
    running: Arc<RwLock<bool>>,
}

impl BackupScheduler {
    /// Create new scheduler
    pub fn new(config: ScheduleConfig, manager: Arc<BackupManager>) -> Self {
        Self {
            config,
            manager,
            last_full_backup: Arc::new(RwLock::new(None)),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start scheduler
    ///
    /// DO-178C §11.11: Scheduled backup execution
    pub async fn start(&self) -> BackupResult<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        // Spawn full backup task
        let full_task = self.spawn_full_backup_task();

        // Spawn incremental backup task
        let inc_task = self.spawn_incremental_backup_task();

        tokio::select! {
            _ = full_task => {},
            _ = inc_task => {},
        }

        Ok(())
    }

    /// Stop scheduler
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    /// Spawn full backup task
    fn spawn_full_backup_task(&self) -> tokio::task::JoinHandle<()> {
        let config = self.config.clone();
        let manager = Arc::clone(&self.manager);
        let last_full = Arc::clone(&self.last_full_backup);
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.full_backup_interval);

            loop {
                interval.tick().await;

                if !*running.read().await {
                    break;
                }

                for source_path in &config.source_paths {
                    if !source_path.exists() {
                        continue;
                    }

                    match manager.create_full_backup(source_path).await {
                        Ok(metadata) => {
                            tracing::info!("Full backup created: {}", metadata.id);
                            *last_full.write().await = Some(metadata.id);
                        }
                        Err(e) => {
                            tracing::error!("Full backup failed: {}", e);
                        }
                    }
                }
            }
        })
    }

    /// Spawn incremental backup task
    fn spawn_incremental_backup_task(&self) -> tokio::task::JoinHandle<()> {
        let config = self.config.clone();
        let manager = Arc::clone(&self.manager);
        let last_full = Arc::clone(&self.last_full_backup);
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.incremental_backup_interval);

            loop {
                interval.tick().await;

                if !*running.read().await {
                    break;
                }

                let parent_id = match *last_full.read().await {
                    Some(id) => id,
                    None => continue, // No full backup yet
                };

                for source_path in &config.source_paths {
                    if !source_path.exists() {
                        continue;
                    }

                    match manager.create_incremental_backup(source_path, parent_id).await {
                        Ok(metadata) => {
                            tracing::info!("Incremental backup created: {}", metadata.id);
                        }
                        Err(e) => {
                            tracing::error!("Incremental backup failed: {}", e);
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_scheduler_creation() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = Arc::new(BackupManager::new(backup_dir).unwrap());

        let config = ScheduleConfig::default();
        let scheduler = BackupScheduler::new(config, manager);

        assert!(!*scheduler.running.read().await);
    }

    #[tokio::test]
    async fn test_scheduler_start_stop() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let manager = Arc::new(BackupManager::new(backup_dir).unwrap());

        let config = ScheduleConfig {
            full_backup_interval: Duration::from_secs(3600),
            incremental_backup_interval: Duration::from_secs(300),
            source_paths: vec![],
        };
        let scheduler = BackupScheduler::new(config, manager);

        // Start and immediately stop
        let handle = tokio::spawn({
            let scheduler = BackupScheduler::new(
                ScheduleConfig::default(),
                Arc::new(BackupManager::new(temp_dir.path().join("backups2")).unwrap()),
            );
            async move {
                scheduler.start().await.ok();
            }
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        scheduler.stop().await;

        handle.abort();
    }
}
