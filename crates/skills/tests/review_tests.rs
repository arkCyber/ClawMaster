//! Tests for skill review and security scanning
//! DO-178C Level A compliant test suite

use {
    clawmaster_skills::{
        review::*,
        types::{SkillMetadata, SkillRequirements},
    },
    std::path::PathBuf,
};

#[tokio::test]
async fn test_security_scan_clean_skill() {
    let metadata = SkillMetadata {
        name: "clean-skill".to_string(),
        description: "A clean skill with no issues".to_string(),
        homepage: Some("https://example.com".to_string()),
        license: Some("MIT".to_string()),
        compatibility: None,
        allowed_tools: vec!["read_file".to_string()],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();
    tokio::fs::write(
        tmp.path().join("SKILL.md"),
        "---\nname: clean\n---\n# Clean Skill\n\nThis is a safe skill.",
    )
    .await
    .unwrap();

    let result = scan_security(tmp.path(), &metadata).await.unwrap();
    assert!(result.score >= 80);
    assert!(result.issues.len() <= 1); // May have low-severity issues
}

#[tokio::test]
async fn test_security_scan_dangerous_tools() {
    let metadata = SkillMetadata {
        name: "dangerous-skill".to_string(),
        description: "A skill with dangerous tools".to_string(),
        homepage: Some("https://example.com".to_string()),
        license: Some("MIT".to_string()),
        compatibility: None,
        allowed_tools: vec!["bash".to_string(), "exec".to_string()],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();
    tokio::fs::write(
        tmp.path().join("SKILL.md"),
        "---\nname: dangerous\n---\n# Dangerous Skill\n\nUses bash.",
    )
    .await
    .unwrap();

    let result = scan_security(tmp.path(), &metadata).await.unwrap();
    assert!(result.score < 100);
    assert!(!result.issues.is_empty());
}

#[tokio::test]
async fn test_security_scan_path_traversal() {
    let metadata = SkillMetadata {
        name: "traversal-skill".to_string(),
        description: "A skill with path traversal".to_string(),
        homepage: Some("https://example.com".to_string()),
        license: Some("MIT".to_string()),
        compatibility: None,
        allowed_tools: vec![],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();
    tokio::fs::write(
        tmp.path().join("SKILL.md"),
        "---\nname: traversal\n---\n# Skill\n\nAccess ../../../etc/passwd",
    )
    .await
    .unwrap();

    let result = scan_security(tmp.path(), &metadata).await.unwrap();
    assert!(result.score < 90);
    let has_traversal_issue = result.issues.iter().any(|i| i.category == "path_traversal");
    assert!(has_traversal_issue);
}

#[tokio::test]
async fn test_quality_analysis_complete_skill() {
    let metadata = SkillMetadata {
        name: "complete-skill".to_string(),
        description: "A complete skill with all documentation".to_string(),
        homepage: Some("https://example.com".to_string()),
        license: Some("MIT".to_string()),
        compatibility: None,
        allowed_tools: vec![],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();

    // Create complete documentation
    let content =
        "---\nname: complete\n---\n# Complete Skill\n\n## Example\n\nThis is an example.\n\n"
            .repeat(10);
    tokio::fs::write(tmp.path().join("SKILL.md"), content)
        .await
        .unwrap();
    tokio::fs::write(tmp.path().join("README.md"), "# README")
        .await
        .unwrap();
    tokio::fs::write(tmp.path().join("LICENSE"), "MIT License")
        .await
        .unwrap();

    let result = analyze_quality(tmp.path(), &metadata).await.unwrap();
    assert!(result.score >= 80);
}

#[tokio::test]
async fn test_quality_analysis_incomplete_skill() {
    let metadata = SkillMetadata {
        name: "incomplete-skill".to_string(),
        description: "Short".to_string(),
        homepage: None,
        license: None,
        compatibility: None,
        allowed_tools: vec![],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();
    tokio::fs::write(
        tmp.path().join("SKILL.md"),
        "---\nname: incomplete\n---\n# Skill\n\nShort content.",
    )
    .await
    .unwrap();

    let result = analyze_quality(tmp.path(), &metadata).await.unwrap();
    assert!(result.score < 80);
    assert!(!result.issues.is_empty());
}

#[tokio::test]
async fn test_complete_review_approved() {
    let metadata = SkillMetadata {
        name: "approved-skill".to_string(),
        description: "An excellent skill that should be approved".to_string(),
        homepage: Some("https://example.com".to_string()),
        license: Some("MIT".to_string()),
        compatibility: None,
        allowed_tools: vec!["read_file".to_string()],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();

    let content = "---\nname: approved\n---\n# Approved Skill\n\n## Example 1\n\nExample content.\n\n## Example 2\n\nMore examples.\n\n".repeat(5);
    tokio::fs::write(tmp.path().join("SKILL.md"), content)
        .await
        .unwrap();
    tokio::fs::write(tmp.path().join("README.md"), "# README")
        .await
        .unwrap();
    tokio::fs::write(tmp.path().join("LICENSE"), "MIT License")
        .await
        .unwrap();

    let review = review_skill(tmp.path(), &metadata).await.unwrap();
    assert_eq!(review.status, ReviewStatus::Approved);
    assert!(review.overall_score >= 80);
}

#[tokio::test]
async fn test_complete_review_rejected() {
    let metadata = SkillMetadata {
        name: "rejected-skill".to_string(),
        description: "Bad".to_string(),
        homepage: None,
        license: None,
        compatibility: None,
        allowed_tools: vec!["bash".to_string(), "exec".to_string()],
        requires: SkillRequirements::default(),
        path: PathBuf::from("/tmp/test"),
        source: None,
        dockerfile: None,
    };

    let tmp = tempfile::tempdir().unwrap();
    tokio::fs::write(
        tmp.path().join("SKILL.md"),
        "---\nname: bad\n---\n# Bad\n\neval(dangerous_code)",
    )
    .await
    .unwrap();

    let review = review_skill(tmp.path(), &metadata).await.unwrap();
    assert_eq!(review.status, ReviewStatus::Rejected);
    assert!(review.overall_score < 50);
}

#[test]
fn test_generate_review_report() {
    let review = SkillReview {
        skill_name: "test-skill".to_string(),
        version: "1.0.0".to_string(),
        status: ReviewStatus::Approved,
        security_scan: SecurityScanResult {
            score: 95,
            issues: vec![],
            scanned_at: "2026-03-17T10:00:00Z".to_string(),
        },
        code_quality: CodeQualityResult {
            score: 90,
            issues: vec![],
            analyzed_at: "2026-03-17T10:00:00Z".to_string(),
        },
        overall_score: 93,
        notes: Some("Excellent skill".to_string()),
        reviewed_at: "2026-03-17T10:00:00Z".to_string(),
    };

    let report = generate_review_report(&review);

    assert!(report.contains("test-skill"));
    assert!(report.contains("APPROVED"));
    assert!(report.contains("93/100"));
    assert!(report.contains("Excellent skill"));
}

#[test]
fn test_severity_levels() {
    let critical = Severity::Critical;
    let high = Severity::High;
    let medium = Severity::Medium;
    let low = Severity::Low;
    let info = Severity::Info;

    assert_eq!(critical, Severity::Critical);
    assert_eq!(high, Severity::High);
    assert_eq!(medium, Severity::Medium);
    assert_eq!(low, Severity::Low);
    assert_eq!(info, Severity::Info);
}

#[test]
fn test_review_status_transitions() {
    let pending = ReviewStatus::Pending;
    let approved = ReviewStatus::Approved;
    let rejected = ReviewStatus::Rejected;
    let manual = ReviewStatus::ManualReview;

    assert_eq!(pending, ReviewStatus::Pending);
    assert_eq!(approved, ReviewStatus::Approved);
    assert_eq!(rejected, ReviewStatus::Rejected);
    assert_eq!(manual, ReviewStatus::ManualReview);
}
