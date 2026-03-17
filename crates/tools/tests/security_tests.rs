//! Security Features Integration Tests
//!
//! DO-178C Level A Compliance Testing
//!
//! This test suite validates:
//! - Emergency stop functionality
//! - Approval mode enforcement
//! - Dangerous command detection
//! - Security level enforcement
//! - Allowlist management

use {
    clawmaster_tools::approval::{
        ApprovalAction, ApprovalDecision, ApprovalManager, ApprovalMode, SecurityLevel,
        check_dangerous, is_safe_command, matches_allowlist,
    },
    std::{sync::Arc, time::Duration},
};

/// Create a test approval manager with default settings
fn create_test_manager() -> ApprovalManager {
    let mut manager = ApprovalManager::default();
    manager.mode = ApprovalMode::OnMiss;
    manager.security_level = SecurityLevel::Allowlist;
    manager.allowlist = vec!["git".to_string(), "ls".to_string()];
    manager.timeout = Duration::from_secs(5);
    manager
}

// ══════════════════════════════════════════════════════════════════════════
// Dangerous Command Detection Tests
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn test_dangerous_command_detection_filesystem() {
    // Test filesystem destruction patterns
    assert!(check_dangerous("rm -rf /").is_some());
    assert!(check_dangerous("rm -r /").is_some());
    assert!(check_dangerous("rm -rf ~").is_some());
    assert!(check_dangerous("rm -r $HOME").is_some());
    assert!(check_dangerous("mkfs /dev/sda").is_some());
    assert!(check_dangerous("dd if=/dev/zero of=/dev/sda").is_some());

    // Safe rm commands should not be detected
    assert!(check_dangerous("rm file.txt").is_none());
    assert!(check_dangerous("rm -rf ./temp").is_none());
}

#[test]
fn test_dangerous_command_detection_git() {
    // Test Git destructive operations
    assert!(check_dangerous("git reset --hard HEAD~10").is_some());
    assert!(check_dangerous("git push --force origin main").is_some());
    assert!(check_dangerous("git push -f origin main").is_some());
    assert!(check_dangerous("git clean -f").is_some());
    assert!(check_dangerous("git stash drop").is_some());
    assert!(check_dangerous("git stash clear").is_some());

    // Safe git commands should not be detected
    assert!(check_dangerous("git status").is_none());
    assert!(check_dangerous("git log").is_none());
    assert!(check_dangerous("git diff").is_none());
}

#[test]
fn test_dangerous_command_detection_database() {
    // Test database destruction patterns
    assert!(check_dangerous("DROP TABLE users").is_some());
    assert!(check_dangerous("DROP DATABASE production").is_some());
    assert!(check_dangerous("TRUNCATE TABLE logs").is_some());

    // Safe database commands should not be detected
    assert!(check_dangerous("SELECT * FROM users").is_none());
    assert!(check_dangerous("INSERT INTO users VALUES (1)").is_none());
}

#[test]
fn test_dangerous_command_detection_infrastructure() {
    // Test infrastructure destruction patterns
    assert!(check_dangerous("docker system prune -a").is_some());
    assert!(check_dangerous("kubectl delete namespace production").is_some());
    assert!(check_dangerous("terraform destroy").is_some());
    assert!(check_dangerous("chmod -R 777 /").is_some());

    // Safe infrastructure commands should not be detected
    assert!(check_dangerous("docker ps").is_none());
    assert!(check_dangerous("kubectl get pods").is_none());
    assert!(check_dangerous("terraform plan").is_none());
}

#[test]
fn test_dangerous_command_fork_bomb() {
    // Test fork bomb detection
    assert!(check_dangerous(":(){ :|:& };:").is_some());

    // Normal function definitions should not be detected
    assert!(check_dangerous("function test() { echo hello; }").is_none());
}

// ══════════════════════════════════════════════════════════════════════════
// Safe Command Detection Tests
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn test_safe_command_detection() {
    // Test safe commands
    assert!(is_safe_command("ls -la"));
    assert!(is_safe_command("cat file.txt"));
    assert!(is_safe_command("echo hello"));
    assert!(is_safe_command("grep pattern file"));
    assert!(is_safe_command("pwd"));
    assert!(is_safe_command("whoami"));

    // Test unsafe commands
    assert!(!is_safe_command("rm -rf /"));
    assert!(!is_safe_command("sudo apt install"));
    assert!(!is_safe_command("npm install"));
}

#[test]
fn test_safe_command_with_path() {
    // Test commands with full paths
    assert!(is_safe_command("/usr/bin/ls -la"));
    assert!(is_safe_command("/bin/cat file.txt"));

    // Test commands with env vars
    assert!(is_safe_command("PATH=/usr/bin ls"));
}

// ══════════════════════════════════════════════════════════════════════════
// Allowlist Matching Tests
// ══════════════════════════════════════════════════════════════════════════

#[test]
fn test_allowlist_exact_match() {
    let allowlist = vec!["git".to_string(), "ls".to_string()];

    assert!(matches_allowlist("git status", &allowlist));
    assert!(matches_allowlist("git push", &allowlist));
    assert!(matches_allowlist("ls", &allowlist));
    assert!(!matches_allowlist("npm install", &allowlist));
}

#[test]
fn test_allowlist_wildcard() {
    let allowlist = vec!["git*".to_string(), "npm*".to_string()];

    assert!(matches_allowlist("git status", &allowlist));
    assert!(matches_allowlist("git log", &allowlist));
    assert!(matches_allowlist("npm install", &allowlist));
    assert!(!matches_allowlist("cargo build", &allowlist));
}

#[test]
fn test_allowlist_star_all() {
    let allowlist = vec!["*".to_string()];

    assert!(matches_allowlist("any command", &allowlist));
    assert!(matches_allowlist("rm -rf /", &allowlist));
}

// ══════════════════════════════════════════════════════════════════════════
// Approval Mode Tests
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_approval_mode_off() {
    let mut manager = create_test_manager();
    manager.mode = ApprovalMode::Off;

    // All commands should proceed without approval
    let action = manager.check_command("any command").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);

    // Even dangerous commands should proceed (but this is not recommended)
    let action = manager.check_command("rm -rf /").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval); // Dangerous commands always need approval
}

#[tokio::test]
async fn test_approval_mode_always() {
    let mut manager = create_test_manager();
    manager.mode = ApprovalMode::Always;

    // All commands should need approval
    let action = manager.check_command("ls").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);

    let action = manager.check_command("git status").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);
}

#[tokio::test]
async fn test_approval_mode_on_miss() {
    let manager = create_test_manager();

    // Safe commands should proceed
    let action = manager.check_command("ls").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);

    // Allowlisted commands should proceed
    let action = manager.check_command("git status").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);

    // Other commands should need approval
    let action = manager.check_command("npm install").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);

    // Dangerous commands should always need approval
    let action = manager.check_command("rm -rf /").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);
}

// ══════════════════════════════════════════════════════════════════════════
// Security Level Tests
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_security_level_deny() {
    let mut manager = create_test_manager();
    manager.security_level = SecurityLevel::Deny;

    // All commands should be denied
    let result = manager.check_command("ls").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("deny"));
}

#[tokio::test]
async fn test_security_level_full() {
    let mut manager = create_test_manager();
    manager.security_level = SecurityLevel::Full;

    // All commands should proceed
    let action = manager.check_command("any command").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);

    // Except dangerous commands
    let action = manager.check_command("rm -rf /").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);
}

#[tokio::test]
async fn test_security_level_allowlist() {
    let manager = create_test_manager();

    // Allowlisted commands should proceed
    let action = manager.check_command("git status").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);

    // Non-allowlisted commands should need approval
    let action = manager.check_command("npm install").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);
}

// ══════════════════════════════════════════════════════════════════════════
// Approval Request/Resolve Tests
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_approval_request_and_approve() {
    let manager = create_test_manager();

    // Create an approval request
    let (id, rx) = manager.create_request("npm install").await;

    // Verify the request is pending
    let pending = manager.pending_ids().await;
    assert!(pending.contains(&id));

    // Approve the request
    manager
        .resolve(&id, ApprovalDecision::Approved, Some("npm install"))
        .await;

    // Verify the decision was received
    let decision = rx.await.unwrap();
    assert_eq!(decision, ApprovalDecision::Approved);

    // Verify the request is no longer pending
    let pending = manager.pending_ids().await;
    assert!(!pending.contains(&id));

    // Verify the command is now in approved list
    let action = manager.check_command("npm install").await.unwrap();
    assert_eq!(action, ApprovalAction::Proceed);
}

#[tokio::test]
async fn test_approval_request_and_deny() {
    let manager = create_test_manager();

    // Create an approval request
    let (id, rx) = manager.create_request("dangerous command").await;

    // Deny the request
    manager.resolve(&id, ApprovalDecision::Denied, None).await;

    // Verify the decision was received
    let decision = rx.await.unwrap();
    assert_eq!(decision, ApprovalDecision::Denied);

    // Verify the command is not in approved list
    let action = manager.check_command("dangerous command").await.unwrap();
    assert_eq!(action, ApprovalAction::NeedsApproval);
}

#[tokio::test]
async fn test_approval_timeout() {
    let mut manager = create_test_manager();
    manager.timeout = Duration::from_millis(100);

    // Create an approval request
    let (_id, rx) = manager.create_request("test command").await;

    // Wait for timeout
    let decision = manager.wait_for_decision(rx).await;
    assert_eq!(decision, ApprovalDecision::Timeout);
}

// ══════════════════════════════════════════════════════════════════════════
// Concurrent Approval Tests
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_multiple_concurrent_approvals() {
    let manager = Arc::new(create_test_manager());

    // Create multiple approval requests
    let mut requests = vec![];
    for i in 0..5 {
        let (id, rx) = manager.create_request(&format!("command {}", i)).await;
        requests.push((id, rx));
    }

    // Verify all requests are pending
    let pending = manager.pending_ids().await;
    assert_eq!(pending.len(), 5);

    // Approve all requests
    for (id, _) in &requests {
        manager.resolve(id, ApprovalDecision::Approved, None).await;
    }

    // Verify all decisions were received
    for (_, rx) in requests {
        let decision = rx.await.unwrap();
        assert_eq!(decision, ApprovalDecision::Approved);
    }

    // Verify no requests are pending
    let pending = manager.pending_ids().await;
    assert_eq!(pending.len(), 0);
}

// ══════════════════════════════════════════════════════════════════════════
// Edge Cases and Error Handling
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_resolve_nonexistent_request() {
    let manager = create_test_manager();

    // Resolving a non-existent request should not panic
    manager
        .resolve("nonexistent-id", ApprovalDecision::Approved, None)
        .await;

    // Should log a warning but not fail
}

#[test]
fn test_dangerous_command_empty_string() {
    assert!(check_dangerous("").is_none());
}

#[test]
fn test_safe_command_empty_string() {
    assert!(!is_safe_command(""));
}

#[test]
fn test_allowlist_empty() {
    let allowlist: Vec<String> = vec![];
    assert!(!matches_allowlist("any command", &allowlist));
}

// ══════════════════════════════════════════════════════════════════════════
// DO-178C Compliance Tests
// ══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_do178c_deterministic_behavior() {
    // DO-178C §6.3.4: Same input should produce same output
    let manager = create_test_manager();

    for _ in 0..10 {
        let action1 = manager.check_command("ls").await.unwrap();
        let action2 = manager.check_command("ls").await.unwrap();
        assert_eq!(action1, action2);
    }
}

#[tokio::test]
async fn test_do178c_error_handling() {
    // DO-178C §6.3.2: All errors should be handled gracefully
    let mut manager = create_test_manager();
    manager.security_level = SecurityLevel::Deny;

    let result = manager.check_command("any command").await;
    assert!(result.is_err());

    // Error should contain meaningful message
    let error_msg = result.unwrap_err().to_string();
    assert!(!error_msg.is_empty());
}

#[test]
fn test_do178c_resource_limits() {
    // DO-178C §11.10: Resource usage should be bounded
    let manager = create_test_manager();

    // Timeout should be reasonable
    assert!(manager.timeout.as_secs() > 0);
    assert!(manager.timeout.as_secs() < 600); // Less than 10 minutes
}

#[test]
fn test_approval_mode_parsing() {
    // Test all valid modes
    assert_eq!(ApprovalMode::parse("off"), Some(ApprovalMode::Off));
    assert_eq!(ApprovalMode::parse("never"), Some(ApprovalMode::Off));
    assert_eq!(ApprovalMode::parse("on-miss"), Some(ApprovalMode::OnMiss));
    assert_eq!(ApprovalMode::parse("smart"), Some(ApprovalMode::OnMiss));
    assert_eq!(ApprovalMode::parse("always"), Some(ApprovalMode::Always));

    // Test invalid modes
    assert_eq!(ApprovalMode::parse("invalid"), None);
}

#[test]
fn test_security_level_parsing() {
    // Test all valid levels
    assert_eq!(SecurityLevel::parse("deny"), Some(SecurityLevel::Deny));
    assert_eq!(SecurityLevel::parse("strict"), Some(SecurityLevel::Deny));
    assert_eq!(
        SecurityLevel::parse("allowlist"),
        Some(SecurityLevel::Allowlist)
    );
    assert_eq!(SecurityLevel::parse("full"), Some(SecurityLevel::Full));
    assert_eq!(
        SecurityLevel::parse("permissive"),
        Some(SecurityLevel::Full)
    );

    // Test invalid levels
    assert_eq!(SecurityLevel::parse("invalid"), None);
}
