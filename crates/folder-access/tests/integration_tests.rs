//! Integration tests for folder access control
//!
//! DO-178C Level A: Comprehensive integration testing

use clawmaster_folder_access::{
    FolderAccessService, PermissionFlags, AccessOperation, RuleType, run_migrations,
};
use sqlx::SqlitePool;
use tempfile::tempdir;

async fn create_test_service() -> FolderAccessService {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    run_migrations(&pool).await.unwrap();
    
    // Disable default blacklist rules for testing
    sqlx::query("UPDATE folder_validation_rules SET is_active = 0")
        .execute(&pool)
        .await
        .unwrap();
    
    FolderAccessService::new(pool).await.unwrap()
}

#[tokio::test]
async fn test_complete_workflow() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test.txt");
    std::fs::write(&test_file, "test content").unwrap();

    // 1. Add folder with read-only permissions
    let folder_id = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            Some("Test workspace".to_string()),
            "admin",
        )
        .await
        .unwrap();

    assert!(folder_id > 0);

    // 2. Check read access (should be allowed)
    let can_read = service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Read,
            Some("session_123".to_string()),
        )
        .await
        .unwrap();

    assert!(can_read);

    // 3. Check write access (should be denied)
    let can_write = service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Write,
            Some("session_123".to_string()),
        )
        .await
        .unwrap();

    assert!(!can_write);

    // 4. Update permissions to read-write
    service
        .update_permissions(folder_id, PermissionFlags::read_write())
        .await
        .unwrap();

    // 5. Check write access again (should now be allowed)
    let can_write_now = service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Write,
            Some("session_123".to_string()),
        )
        .await
        .unwrap();

    assert!(can_write_now);

    // 6. Get access logs
    let logs = service.get_access_logs(folder_id, 10).await.unwrap();

    // Should have 3 access attempts (2 reads, 1 write denied, 1 write allowed)
    assert!(logs.len() >= 3);

    // 7. List all folders
    let folders = service.list_folders(false).await.unwrap();
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0].id, folder_id);

    // 8. Remove folder
    service.remove_folder(folder_id).await.unwrap();

    // 9. Verify folder is inactive
    let folders_after = service.list_folders(false).await.unwrap();
    assert_eq!(folders_after.len(), 0);

    // 10. Verify folder still exists but is inactive
    let all_folders = service.list_folders(true).await.unwrap();
    assert_eq!(all_folders.len(), 1);
    assert!(!all_folders[0].is_active);
}

#[tokio::test]
async fn test_nested_folder_permissions() {
    let service = create_test_service().await;
    let parent_dir = tempdir().unwrap();
    let child_dir = parent_dir.path().join("child");
    std::fs::create_dir(&child_dir).unwrap();
    let test_file = child_dir.join("test.txt");
    std::fs::write(&test_file, "test").unwrap();

    // Add parent folder with read-only
    service
        .add_folder(
            parent_dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            Some("Parent folder".to_string()),
            "admin",
        )
        .await
        .unwrap();

    // Child file should inherit parent permissions
    let can_read = service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Read,
            None,
        )
        .await
        .unwrap();

    assert!(can_read);

    let can_write = service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Write,
            None,
        )
        .await
        .unwrap();

    assert!(!can_write);
}

#[tokio::test]
async fn test_validation_rules() {
    let service = create_test_service().await;

    // Create a directory first
    let dir = tempdir().unwrap();
    let sensitive_dir = dir.path().join("sensitive");
    std::fs::create_dir(&sensitive_dir).unwrap();

    // Add a custom blacklist rule that matches the directory
    let rule_id = service
        .add_validation_rule(
            RuleType::Blacklist,
            "*/sensitive".to_string(),
            Some("Sensitive data".to_string()),
            100,
            "admin",
        )
        .await
        .unwrap();

    assert!(rule_id > 0);

    // Try to add a folder that matches the blacklist
    // Note: The rule is already reloaded by add_validation_rule
    let result = service
        .add_folder(
            sensitive_dir.to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("blacklist"));
}

#[tokio::test]
async fn test_concurrent_access_checks() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test.txt");
    std::fs::write(&test_file, "test").unwrap();

    service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::full(),
            None,
            "admin",
        )
        .await
        .unwrap();

    // Perform multiple concurrent access checks
    let mut handles = vec![];

    for i in 0..10 {
        let service_clone = service.clone();
        let file_path = test_file.to_str().unwrap().to_string();
        let session = format!("session_{}", i);

        let handle = tokio::spawn(async move {
            service_clone
                .check_access(&file_path, AccessOperation::Read, Some(session))
                .await
        });

        handles.push(handle);
    }

    // Wait for all checks to complete
    for handle in handles {
        let result: Result<bool, anyhow::Error> = handle.await.unwrap();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}

#[tokio::test]
async fn test_permission_validation() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();

    // Try to add folder with no permissions (should fail)
    let result = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags {
                can_read: false,
                can_write: false,
                can_execute: false,
                can_delete: false,
            },
            None,
            "admin",
        )
        .await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("At least one permission"));
}

#[tokio::test]
async fn test_duplicate_folder_prevention() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();

    // Add folder first time (should succeed)
    let result1 = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await;

    assert!(result1.is_ok());

    // Try to add same folder again (should fail)
    let result2 = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_write(),
            None,
            "admin",
        )
        .await;

    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("already exists"));
}

#[tokio::test]
async fn test_access_statistics() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test.txt");
    std::fs::write(&test_file, "test").unwrap();

    let _folder_id = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await
        .unwrap();

    // Perform multiple access checks
    for _ in 0..5 {
        service
            .check_access(test_file.to_str().unwrap(), AccessOperation::Read, None)
            .await
            .unwrap();
    }

    // Get folder info and verify access count
    let folders = service.list_folders(false).await.unwrap();
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0].access_count, 5);
    assert!(folders[0].last_accessed_at.is_some());
}

#[tokio::test]
async fn test_invalid_path_handling() {
    let service = create_test_service().await;

    // Try to add non-existent directory
    let result = service
        .add_folder(
            "/nonexistent/directory/path",
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await;

    assert!(result.is_err());

    // Try to add a file instead of directory
    let dir = tempdir().unwrap();
    let file = dir.path().join("file.txt");
    std::fs::write(&file, "test").unwrap();

    let result2 = service
        .add_folder(
            file.to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await;

    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("not a directory"));
}

#[tokio::test]
async fn test_audit_log_completeness() {
    let service = create_test_service().await;
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test.txt");
    std::fs::write(&test_file, "test").unwrap();

    let folder_id = service
        .add_folder(
            dir.path().to_str().unwrap(),
            PermissionFlags::read_only(),
            None,
            "admin",
        )
        .await
        .unwrap();

    // Perform various operations
    service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Read,
            Some("session_1".to_string()),
        )
        .await
        .unwrap();

    service
        .check_access(
            test_file.to_str().unwrap(),
            AccessOperation::Write,
            Some("session_2".to_string()),
        )
        .await
        .unwrap();

    // Get logs and verify completeness
    let logs = service.get_access_logs(folder_id, 10).await.unwrap();

    assert_eq!(logs.len(), 2);

    // Verify both operations are logged
    let read_log = logs.iter().find(|l| l.operation == AccessOperation::Read).unwrap();
    let write_log = logs.iter().find(|l| l.operation == AccessOperation::Write).unwrap();

    // Verify read log
    assert!(read_log.success);
    assert_eq!(read_log.session_key, Some("session_1".to_string()));

    // Verify write log (should be denied)
    assert!(!write_log.success);
    assert_eq!(write_log.session_key, Some("session_2".to_string()));
}
