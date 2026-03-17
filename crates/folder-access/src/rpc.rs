//! RPC methods for folder access control
//!
//! DO-178C Level A: RPC interface with comprehensive validation

use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
};

use crate::{
    models::{AccessOperation, PermissionFlags, RuleType},
    service::FolderAccessService,
};

/// RPC request to add folder permission
#[derive(Debug, Deserialize)]
pub struct AddFolderRequest {
    pub folder_path: String,
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delete: bool,
    pub description: Option<String>,
}

/// RPC request to update folder permissions
#[derive(Debug, Deserialize)]
pub struct UpdatePermissionsRequest {
    pub folder_id: i64,
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delete: bool,
}

/// RPC request to check access
#[derive(Debug, Deserialize)]
pub struct CheckAccessRequest {
    pub file_path: String,
    pub operation: String, // "read", "write", "execute", "delete"
    pub session_key: Option<String>,
}

/// RPC request to add validation rule
#[derive(Debug, Deserialize)]
pub struct AddValidationRuleRequest {
    pub rule_type: String, // "blacklist", "whitelist", "pattern"
    pub pattern: String,
    pub description: Option<String>,
    pub priority: i32,
}

/// RPC response for folder operations
#[derive(Debug, Serialize)]
pub struct FolderResponse {
    pub id: i64,
    pub folder_path: String,
    pub permissions: PermissionFlagsResponse,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: String,
    pub is_active: bool,
    pub access_count: i64,
}

/// RPC response for permissions
#[derive(Debug, Serialize)]
pub struct PermissionFlagsResponse {
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delete: bool,
}

/// RPC response for access logs
#[derive(Debug, Serialize)]
pub struct AccessLogResponse {
    pub id: i64,
    pub folder_id: i64,
    pub operation: String,
    pub file_path: Option<String>,
    pub success: bool,
    pub session_key: Option<String>,
    pub error_message: Option<String>,
    pub timestamp: i64,
}

/// RPC handler for folder access control
pub struct FolderAccessRpc {
    service: FolderAccessService,
}

impl FolderAccessRpc {
    /// Create new RPC handler
    pub fn new(service: FolderAccessService) -> Self {
        Self { service }
    }

    /// Add folder permission
    ///
    /// # RPC Method
    /// `folder_access.add`
    pub async fn add_folder(&self, params: Value, created_by: &str) -> Result<Value> {
        let req: AddFolderRequest = serde_json::from_value(params)?;

        let permissions = PermissionFlags {
            can_read: req.can_read,
            can_write: req.can_write,
            can_execute: req.can_execute,
            can_delete: req.can_delete,
        };

        let folder_id = self
            .service
            .add_folder(&req.folder_path, permissions, req.description, created_by)
            .await?;

        Ok(json!({
            "ok": true,
            "folder_id": folder_id,
            "message": "Folder added successfully"
        }))
    }

    /// Remove folder permission
    ///
    /// # RPC Method
    /// `folder_access.remove`
    pub async fn remove_folder(&self, params: Value) -> Result<Value> {
        let folder_id: i64 = serde_json::from_value(
            params
                .get("folder_id")
                .ok_or_else(|| anyhow::anyhow!("Missing folder_id"))?
                .clone(),
        )?;

        self.service.remove_folder(folder_id).await?;

        Ok(json!({
            "ok": true,
            "message": "Folder removed successfully"
        }))
    }

    /// Update folder permissions
    ///
    /// # RPC Method
    /// `folder_access.update_permissions`
    pub async fn update_permissions(&self, params: Value) -> Result<Value> {
        let req: UpdatePermissionsRequest = serde_json::from_value(params)?;

        let permissions = PermissionFlags {
            can_read: req.can_read,
            can_write: req.can_write,
            can_execute: req.can_execute,
            can_delete: req.can_delete,
        };

        self.service
            .update_permissions(req.folder_id, permissions)
            .await?;

        Ok(json!({
            "ok": true,
            "message": "Permissions updated successfully"
        }))
    }

    /// List all folders
    ///
    /// # RPC Method
    /// `folder_access.list`
    pub async fn list_folders(&self, params: Value) -> Result<Value> {
        let include_inactive = params
            .get("include_inactive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let folders = self.service.list_folders(include_inactive).await?;

        let response: Vec<FolderResponse> = folders
            .into_iter()
            .map(|f| FolderResponse {
                id: f.id,
                folder_path: f.folder_path,
                permissions: PermissionFlagsResponse {
                    can_read: f.permissions.can_read,
                    can_write: f.permissions.can_write,
                    can_execute: f.permissions.can_execute,
                    can_delete: f.permissions.can_delete,
                },
                description: f.description,
                created_at: f.created_at,
                updated_at: f.updated_at,
                created_by: f.created_by,
                is_active: f.is_active,
                access_count: f.access_count,
            })
            .collect();

        Ok(json!({
            "ok": true,
            "folders": response
        }))
    }

    /// Check access permission
    ///
    /// # RPC Method
    /// `folder_access.check`
    pub async fn check_access(&self, params: Value) -> Result<Value> {
        let req: CheckAccessRequest = serde_json::from_value(params)?;

        let operation = AccessOperation::from_str(&req.operation)
            .ok_or_else(|| anyhow::anyhow!("Invalid operation type"))?;

        let allowed = self
            .service
            .check_access(&req.file_path, operation, req.session_key)
            .await?;

        Ok(json!({
            "ok": true,
            "allowed": allowed
        }))
    }

    /// Get access logs
    ///
    /// # RPC Method
    /// `folder_access.logs`
    pub async fn get_logs(&self, params: Value) -> Result<Value> {
        let folder_id: i64 = serde_json::from_value(
            params
                .get("folder_id")
                .ok_or_else(|| anyhow::anyhow!("Missing folder_id"))?
                .clone(),
        )?;

        let limit: i64 = params.get("limit").and_then(|v| v.as_i64()).unwrap_or(100);

        let logs = self.service.get_access_logs(folder_id, limit).await?;

        let response: Vec<AccessLogResponse> = logs
            .into_iter()
            .map(|l| AccessLogResponse {
                id: l.id,
                folder_id: l.folder_id,
                operation: l.operation.as_str().to_string(),
                file_path: l.file_path,
                success: l.success,
                session_key: l.session_key,
                error_message: l.error_message,
                timestamp: l.timestamp,
            })
            .collect();

        Ok(json!({
            "ok": true,
            "logs": response
        }))
    }

    /// Add validation rule
    ///
    /// # RPC Method
    /// `folder_access.add_rule`
    pub async fn add_validation_rule(&self, params: Value, created_by: &str) -> Result<Value> {
        let req: AddValidationRuleRequest = serde_json::from_value(params)?;

        let rule_type = RuleType::from_str(&req.rule_type)
            .ok_or_else(|| anyhow::anyhow!("Invalid rule type"))?;

        let rule_id = self
            .service
            .add_validation_rule(
                rule_type,
                req.pattern,
                req.description,
                req.priority,
                created_by,
            )
            .await?;

        Ok(json!({
            "ok": true,
            "rule_id": rule_id,
            "message": "Validation rule added successfully"
        }))
    }

    /// Reload validation rules
    ///
    /// # RPC Method
    /// `folder_access.reload_rules`
    pub async fn reload_rules(&self) -> Result<Value> {
        self.service.reload_validation_rules().await?;

        Ok(json!({
            "ok": true,
            "message": "Validation rules reloaded successfully"
        }))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, sqlx::SqlitePool};

    async fn create_test_rpc() -> FolderAccessRpc {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::run_migrations(&pool).await.unwrap();

        // Disable default blacklist rules for testing
        sqlx::query("UPDATE folder_validation_rules SET is_active = 0")
            .execute(&pool)
            .await
            .unwrap();

        let service = FolderAccessService::new(pool).await.unwrap();
        FolderAccessRpc::new(service)
    }

    #[tokio::test]
    async fn test_add_folder_rpc() {
        let rpc = create_test_rpc().await;
        let dir = tempfile::tempdir().unwrap();

        let params = json!({
            "folder_path": dir.path().to_str().unwrap(),
            "can_read": true,
            "can_write": true,
            "can_execute": false,
            "can_delete": false,
            "description": "Test folder"
        });

        let result = rpc.add_folder(params, "test_user").await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["ok"], true);
        assert!(response["folder_id"].is_number());
    }

    #[tokio::test]
    async fn test_list_folders_rpc() {
        let rpc = create_test_rpc().await;
        let dir = tempfile::tempdir().unwrap();

        let params = json!({
            "folder_path": dir.path().to_str().unwrap(),
            "can_read": true,
            "can_write": false,
            "can_execute": false,
            "can_delete": false,
        });

        rpc.add_folder(params, "test_user").await.unwrap();

        let result = rpc.list_folders(json!({})).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["ok"], true);
        assert!(response["folders"].is_array());
    }

    #[tokio::test]
    async fn test_check_access_rpc() {
        let rpc = create_test_rpc().await;
        let dir = tempfile::tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "test").unwrap();

        let params = json!({
            "folder_path": dir.path().to_str().unwrap(),
            "can_read": true,
            "can_write": false,
            "can_execute": false,
            "can_delete": false,
        });

        rpc.add_folder(params, "test_user").await.unwrap();

        let check_params = json!({
            "file_path": test_file.to_str().unwrap(),
            "operation": "read",
            "session_key": "test_session"
        });

        let result = rpc.check_access(check_params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["ok"], true);
        assert_eq!(response["allowed"], true);
    }
}
