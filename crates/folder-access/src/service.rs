//! Folder Access Control Service
//!
//! DO-178C Level A: Complete access control with audit logging

use std::sync::Arc;
use anyhow::{anyhow, Result};
use sqlx::{SqlitePool, Row};
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::models::{
    FolderPermission, PermissionFlags, AccessLog, AccessOperation,
    ValidationRule, RuleType,
};
use crate::validation::{PathValidator, calculate_path_hash};

/// Folder access control service
///
/// # Compliance
/// DO-178C §11.10: Thread-safe access control with audit logging
#[derive(Clone)]
pub struct FolderAccessService {
    pool: SqlitePool,
    validator: Arc<RwLock<PathValidator>>,
}

impl FolderAccessService {
    /// Create new folder access service
    ///
    /// # Compliance
    /// DO-178C §11.13: Proper initialization
    pub async fn new(pool: SqlitePool) -> Result<Self> {
        let rules = Self::load_validation_rules_static(&pool).await?;
        let validator = Arc::new(RwLock::new(PathValidator::new(rules)));
        
        Ok(Self { pool, validator })
    }

    // ── Permission Management ────────────────────────────────────

    /// Add folder with permissions
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Comprehensive validation and error handling
    pub async fn add_folder(
        &self,
        folder_path: &str,
        permissions: PermissionFlags,
        description: Option<String>,
        created_by: &str,
    ) -> Result<i64> {
        // Validate path
        let validation = self.validator.read().await.validate(folder_path);
        if !validation.is_valid {
            return Err(anyhow!(validation.error_message.unwrap_or_default()));
        }

        let canonical_path = validation.canonical_path.unwrap();
        let canonical_str = canonical_path.to_string_lossy().to_string();

        // Check if path is a directory
        if !canonical_path.is_dir() {
            return Err(anyhow!("Path is not a directory: {}", folder_path));
        }

        // Calculate hash for integrity
        let folder_hash = calculate_path_hash(&canonical_str);

        // Check if folder already exists
        let existing = sqlx::query(
            "SELECT id FROM folder_permissions WHERE folder_path = ?"
        )
        .bind(&canonical_str)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(anyhow!("Folder already exists in permissions"));
        }

        // Validate permissions
        if !permissions.has_any_permission() {
            return Err(anyhow!("At least one permission must be granted"));
        }

        let now = chrono::Utc::now().timestamp();

        // Insert folder permission
        let result = sqlx::query(
            r#"
            INSERT INTO folder_permissions (
                folder_path, folder_hash, can_read, can_write, can_execute, can_delete,
                description, created_at, updated_at, created_by, is_active, access_count
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0)
            "#
        )
        .bind(&canonical_str)
        .bind(&folder_hash)
        .bind(permissions.can_read)
        .bind(permissions.can_write)
        .bind(permissions.can_execute)
        .bind(permissions.can_delete)
        .bind(description)
        .bind(now)
        .bind(now)
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        let folder_id = result.last_insert_rowid();

        info!(
            folder_path = %canonical_str,
            folder_id = folder_id,
            created_by = %created_by,
            "Folder added to access control"
        );

        Ok(folder_id)
    }

    /// Remove folder from access control
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Safe deletion with audit trail
    pub async fn remove_folder(&self, folder_id: i64) -> Result<()> {
        // Soft delete - mark as inactive
        let result = sqlx::query(
            "UPDATE folder_permissions SET is_active = 0, updated_at = ? WHERE id = ?"
        )
        .bind(chrono::Utc::now().timestamp())
        .bind(folder_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow!("Folder not found"));
        }

        info!(folder_id = folder_id, "Folder removed from access control");

        Ok(())
    }

    /// Update folder permissions
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Atomic permission updates
    pub async fn update_permissions(
        &self,
        folder_id: i64,
        permissions: PermissionFlags,
    ) -> Result<()> {
        if !permissions.has_any_permission() {
            return Err(anyhow!("At least one permission must be granted"));
        }

        let result = sqlx::query(
            r#"
            UPDATE folder_permissions 
            SET can_read = ?, can_write = ?, can_execute = ?, can_delete = ?, updated_at = ?
            WHERE id = ? AND is_active = 1
            "#
        )
        .bind(permissions.can_read)
        .bind(permissions.can_write)
        .bind(permissions.can_execute)
        .bind(permissions.can_delete)
        .bind(chrono::Utc::now().timestamp())
        .bind(folder_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow!("Folder not found or inactive"));
        }

        info!(folder_id = folder_id, "Folder permissions updated");

        Ok(())
    }

    /// List all folders with permissions
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic listing
    pub async fn list_folders(&self, include_inactive: bool) -> Result<Vec<FolderPermission>> {
        let query = if include_inactive {
            "SELECT * FROM folder_permissions ORDER BY folder_path"
        } else {
            "SELECT * FROM folder_permissions WHERE is_active = 1 ORDER BY folder_path"
        };

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        let mut folders = Vec::new();
        for row in rows {
            folders.push(FolderPermission {
                id: row.get("id"),
                folder_path: row.get("folder_path"),
                folder_hash: row.get("folder_hash"),
                permissions: PermissionFlags {
                    can_read: row.get("can_read"),
                    can_write: row.get("can_write"),
                    can_execute: row.get("can_execute"),
                    can_delete: row.get("can_delete"),
                },
                description: row.get("description"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                created_by: row.get("created_by"),
                is_active: row.get("is_active"),
                last_accessed_at: row.get("last_accessed_at"),
                access_count: row.get("access_count"),
            });
        }

        Ok(folders)
    }

    // ── Access Control ────────────────────────────────────────────

    /// Check if operation is allowed on file path
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic permission checking
    /// DO-178C §11.10: Complete audit logging
    pub async fn check_access(
        &self,
        file_path: &str,
        operation: AccessOperation,
        session_key: Option<String>,
    ) -> Result<bool> {
        // Validate path
        let validation = self.validator.read().await.validate(file_path);
        if !validation.is_valid {
            let error = validation.error_message.unwrap_or_default();
            warn!(
                file_path = %file_path,
                operation = ?operation,
                error = %error,
                "Path validation failed"
            );
            return Ok(false);
        }

        let canonical_path = validation.canonical_path.unwrap();
        let canonical_str = canonical_path.to_string_lossy().to_string();

        // Find matching folder permission
        let folder = self.find_folder_for_path(&canonical_str).await?;

        let (allowed, folder_id) = match folder {
            Some(f) => {
                let allowed = match operation {
                    AccessOperation::Read => f.permissions.can_read,
                    AccessOperation::Write => f.permissions.can_write,
                    AccessOperation::Execute => f.permissions.can_execute,
                    AccessOperation::Delete => f.permissions.can_delete,
                };
                (allowed, Some(f.id))
            }
            None => {
                warn!(
                    file_path = %canonical_str,
                    operation = ?operation,
                    "No folder permission found"
                );
                (false, None)
            }
        };

        // Log access attempt
        if let Some(fid) = folder_id {
            self.log_access(
                fid,
                operation,
                Some(canonical_str.clone()),
                allowed,
                session_key,
                if !allowed {
                    Some("Permission denied".to_string())
                } else {
                    None
                },
            )
            .await?;

            // Update access statistics
            if allowed {
                self.update_access_stats(fid).await?;
            }
        }

        Ok(allowed)
    }

    /// Find folder permission for given path
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Longest prefix match
    async fn find_folder_for_path(&self, path: &str) -> Result<Option<FolderPermission>> {
        let rows = sqlx::query(
            "SELECT * FROM folder_permissions WHERE is_active = 1 ORDER BY LENGTH(folder_path) DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        for row in rows {
            let folder_path: String = row.get("folder_path");
            if path.starts_with(&folder_path) {
                return Ok(Some(FolderPermission {
                    id: row.get("id"),
                    folder_path: row.get("folder_path"),
                    folder_hash: row.get("folder_hash"),
                    permissions: PermissionFlags {
                        can_read: row.get("can_read"),
                        can_write: row.get("can_write"),
                        can_execute: row.get("can_execute"),
                        can_delete: row.get("can_delete"),
                    },
                    description: row.get("description"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    created_by: row.get("created_by"),
                    is_active: row.get("is_active"),
                    last_accessed_at: row.get("last_accessed_at"),
                    access_count: row.get("access_count"),
                }));
            }
        }

        Ok(None)
    }

    // ── Audit Logging ─────────────────────────────────────────────

    /// Log access attempt
    ///
    /// # Compliance
    /// DO-178C §11.10: Complete audit trail
    async fn log_access(
        &self,
        folder_id: i64,
        operation: AccessOperation,
        file_path: Option<String>,
        success: bool,
        session_key: Option<String>,
        error_message: Option<String>,
    ) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp();
        let operation_str = operation.as_str();

        sqlx::query(
            r#"
            INSERT INTO folder_access_log (
                folder_id, operation, file_path, success, session_key, error_message, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(folder_id)
        .bind(operation_str)
        .bind(file_path)
        .bind(success)
        .bind(session_key)
        .bind(error_message)
        .bind(timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update access statistics
    ///
    /// # Compliance
    /// DO-178C §11.10: Track usage patterns
    async fn update_access_stats(&self, folder_id: i64) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE folder_permissions 
            SET access_count = access_count + 1, last_accessed_at = ?
            WHERE id = ?
            "#
        )
        .bind(now)
        .bind(folder_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get access logs for folder
    ///
    /// # Compliance
    /// DO-178C §11.10: Audit trail retrieval
    pub async fn get_access_logs(
        &self,
        folder_id: i64,
        limit: i64,
    ) -> Result<Vec<AccessLog>> {
        let rows = sqlx::query(
            "SELECT * FROM folder_access_log WHERE folder_id = ? ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(folder_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            let operation_str: String = row.get("operation");
            let operation = AccessOperation::from_str(&operation_str)
                .ok_or_else(|| anyhow!("Invalid operation type"))?;

            logs.push(AccessLog {
                id: row.get("id"),
                folder_id: row.get("folder_id"),
                operation,
                file_path: row.get("file_path"),
                success: row.get("success"),
                session_key: row.get("session_key"),
                user_agent: row.get("user_agent"),
                error_message: row.get("error_message"),
                timestamp: row.get("timestamp"),
            });
        }

        Ok(logs)
    }

    // ── Validation Rules ──────────────────────────────────────────

    /// Load validation rules from database
    ///
    /// # Compliance
    /// DO-178C §11.13: Dynamic rule loading
    async fn load_validation_rules_static(pool: &SqlitePool) -> Result<Vec<ValidationRule>> {
        let rows = sqlx::query(
            "SELECT * FROM folder_validation_rules WHERE is_active = 1 ORDER BY priority DESC"
        )
        .fetch_all(pool)
        .await?;

        let mut rules = Vec::new();
        for row in rows {
            let rule_type_str: String = row.get("rule_type");
            let rule_type = RuleType::from_str(&rule_type_str)
                .ok_or_else(|| anyhow!("Invalid rule type"))?;

            rules.push(ValidationRule {
                id: row.get("id"),
                rule_type,
                pattern: row.get("pattern"),
                description: row.get("description"),
                is_active: row.get("is_active"),
                priority: row.get("priority"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
            });
        }

        Ok(rules)
    }

    /// Reload validation rules
    ///
    /// # Compliance
    /// DO-178C §11.13: Hot reload capability
    pub async fn reload_validation_rules(&self) -> Result<()> {
        let rules = Self::load_validation_rules_static(&self.pool).await?;
        *self.validator.write().await = PathValidator::new(rules);
        info!("Validation rules reloaded");
        Ok(())
    }

    /// Add validation rule
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Safe rule addition
    pub async fn add_validation_rule(
        &self,
        rule_type: RuleType,
        pattern: String,
        description: Option<String>,
        priority: i32,
        created_by: &str,
    ) -> Result<i64> {
        let now = chrono::Utc::now().timestamp();
        let rule_type_str = rule_type.as_str();
        let pattern_clone = pattern.clone();

        let result = sqlx::query(
            r#"
            INSERT INTO folder_validation_rules (
                rule_type, pattern, description, is_active, priority, created_at, created_by
            ) VALUES (?, ?, ?, 1, ?, ?, ?)
            "#
        )
        .bind(rule_type_str)
        .bind(pattern)
        .bind(description)
        .bind(priority)
        .bind(now)
        .bind(created_by)
        .execute(&self.pool)
        .await?;

        let rule_id = result.last_insert_rowid();

        // Reload rules
        self.reload_validation_rules().await?;

        info!(
            rule_id = rule_id,
            pattern = %pattern_clone,
            "Validation rule added"
        );

        Ok(rule_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    async fn create_test_service() -> FolderAccessService {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::run_migrations(&pool).await.unwrap();
        
        // Disable default blacklist rules for testing
        sqlx::query("UPDATE folder_validation_rules SET is_active = 0")
            .execute(&pool)
            .await
            .unwrap();
        
        FolderAccessService::new(pool).await.unwrap()
    }

    #[tokio::test]
    async fn test_add_folder() {
        let service = create_test_service().await;
        let dir = tempdir().unwrap();
        
        let result = service.add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            Some("Test folder".to_string()),
            "test_user",
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_access_allowed() {
        let service = create_test_service().await;
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "test").unwrap();
        
        service.add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "test_user",
        ).await.unwrap();

        let allowed = service.check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Read,
            None,
        ).await.unwrap();

        assert!(allowed);
    }

    #[tokio::test]
    async fn test_check_access_denied() {
        let service = create_test_service().await;
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "test").unwrap();
        
        service.add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "test_user",
        ).await.unwrap();

        let allowed = service.check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Write,
            None,
        ).await.unwrap();

        assert!(!allowed);
    }

    #[tokio::test]
    async fn test_update_permissions() {
        let service = create_test_service().await;
        let dir = tempdir().unwrap();
        
        let folder_id = service.add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "test_user",
        ).await.unwrap();

        let result = service.update_permissions(
            folder_id,
            PermissionFlags::read_write(),
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_folders() {
        let service = create_test_service().await;
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        
        service.add_folder(
            dir1.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "test_user",
        ).await.unwrap();

        service.add_folder(
            dir2.path().to_str().unwrap(),
            PermissionFlags::read_write(),
            None,
            "test_user",
        ).await.unwrap();

        let folders = service.list_folders(false).await.unwrap();
        assert_eq!(folders.len(), 2);
    }

    #[tokio::test]
    async fn test_access_logging() {
        let service = create_test_service().await;
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "test").unwrap();
        
        let folder_id = service.add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "test_user",
        ).await.unwrap();

        service.check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Read,
            Some("session123".to_string()),
        ).await.unwrap();

        let logs = service.get_access_logs(folder_id, 10).await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].operation, AccessOperation::Read);
        assert!(logs[0].success);
    }
}
