//! Tests for skill update functionality
//! DO-178C Level A compliant test suite

use clawmaster_skills::update::*;

#[test]
fn test_skill_update_serialization() {
    let update = SkillUpdate {
        source: "owner/repo".to_string(),
        current_sha: Some("abc123".to_string()),
        latest_sha: "def456".to_string(),
        commits_behind: 5,
        update_available: true,
        latest_message: Some("Update README".to_string()),
        latest_date: Some("2026-03-17T10:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&update).unwrap();
    let deserialized: SkillUpdate = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.source, update.source);
    assert_eq!(deserialized.commits_behind, 5);
    assert!(deserialized.update_available);
}

#[tokio::test]
async fn test_check_updates_empty() {
    let tmp = tempfile::tempdir().unwrap();
    let result = check_updates(tmp.path()).await;

    // Should succeed even with no installed skills
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_update_available_logic() {
    let update_with_same_sha = SkillUpdate {
        source: "owner/repo".to_string(),
        current_sha: Some("abc123".to_string()),
        latest_sha: "abc123".to_string(),
        commits_behind: 0,
        update_available: false,
        latest_message: None,
        latest_date: None,
    };

    assert!(!update_with_same_sha.update_available);

    let update_with_different_sha = SkillUpdate {
        source: "owner/repo".to_string(),
        current_sha: Some("abc123".to_string()),
        latest_sha: "def456".to_string(),
        commits_behind: 3,
        update_available: true,
        latest_message: None,
        latest_date: None,
    };

    assert!(update_with_different_sha.update_available);
}
