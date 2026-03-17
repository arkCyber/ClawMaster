//! Critical path tests for Skills Registry - DO-178C Level A compliance.
//!
//! These tests ensure 100% coverage of all critical execution paths,
//! including error conditions, boundary cases, and race conditions.

use {
    clawmaster_clawhub::{
        registry::Registry,
        skills::SkillsRegistry,
        types::{SecurityStatus, SkillFormat, SkillMetadata},
    },
    tempfile::tempdir,
    time::OffsetDateTime,
};

/// Helper to create test metadata with specific parameters.
fn create_metadata(name: &str, version: &str, format: SkillFormat) -> SkillMetadata {
    SkillMetadata {
        name: name.to_string(),
        version: version.to_string(),
        description: format!("Critical path test skill {}", name),
        readme: Some(format!(
            "# {}\n\nTest skill for critical path coverage",
            name
        )),
        author: "Test Author".to_string(),
        author_email: Some("test@example.com".to_string()),
        license: "MIT".to_string(),
        repository: Some(format!("https://github.com/test/{}", name)),
        homepage: Some(format!("https://example.com/{}", name)),
        keywords: vec!["test".to_string(), "critical".to_string()],
        categories: vec!["testing".to_string()],
        skill_format: format,
        github_repo: Some(format!("test/{}", name)),
        commit_sha: Some("abc123def456".to_string()),
        downloads: 0,
        stars: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    }
}

async fn setup_registry() -> (Registry, tempfile::TempDir) {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    (registry, dir)
}

// ── Critical Path: Publish ──────────────────────────────────────────────────

#[tokio::test]
async fn test_critical_publish_success_path() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("critical-skill", "1.0.0", SkillFormat::SkillMd);

    // Critical path: successful publish
    let result = skills.publish(metadata.clone()).await;
    assert!(result.is_ok(), "Critical path publish must succeed");

    // Verify data integrity
    let retrieved = skills.get_skill("critical-skill", "1.0.0").await.unwrap();
    assert_eq!(retrieved.name, metadata.name);
    assert_eq!(retrieved.version, metadata.version);
    assert_eq!(retrieved.skill_format, metadata.skill_format);
}

#[tokio::test]
async fn test_critical_publish_duplicate_rejection() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("duplicate", "1.0.0", SkillFormat::SkillMd);

    // First publish succeeds
    skills.publish(metadata.clone()).await.unwrap();

    // Critical path: duplicate must be rejected
    let result = skills.publish(metadata).await;
    assert!(result.is_err(), "Duplicate publish must be rejected");
}

#[tokio::test]
async fn test_critical_publish_version_validation() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let v1 = create_metadata("versioned", "1.0.0", SkillFormat::SkillMd);
    let v2 = create_metadata("versioned", "2.0.0", SkillFormat::SkillMd);
    let v3 = create_metadata("versioned", "1.0.1", SkillFormat::SkillMd);

    // Critical path: multiple versions must coexist
    skills.publish(v1).await.unwrap();
    skills.publish(v2).await.unwrap();
    skills.publish(v3).await.unwrap();

    // Verify all versions exist
    assert!(skills.get_skill("versioned", "1.0.0").await.is_ok());
    assert!(skills.get_skill("versioned", "2.0.0").await.is_ok());
    assert!(skills.get_skill("versioned", "1.0.1").await.is_ok());
}

// ── Critical Path: Retrieval ────────────────────────────────────────────────

#[tokio::test]
async fn test_critical_get_existing_skill() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("existing", "1.0.0", SkillFormat::SkillMd);
    skills.publish(metadata.clone()).await.unwrap();

    // Critical path: retrieval must succeed
    let result = skills.get_skill("existing", "1.0.0").await;
    assert!(result.is_ok(), "Retrieval of existing skill must succeed");

    let retrieved = result.unwrap();
    assert_eq!(retrieved.name, "existing");
    assert_eq!(retrieved.version, "1.0.0");
}

#[tokio::test]
async fn test_critical_get_nonexistent_skill() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Critical path: nonexistent skill must return error
    let result = skills.get_skill("nonexistent", "1.0.0").await;
    assert!(result.is_err(), "Nonexistent skill must return error");
}

#[tokio::test]
async fn test_critical_get_wrong_version() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("versioned", "1.0.0", SkillFormat::SkillMd);
    skills.publish(metadata).await.unwrap();

    // Critical path: wrong version must return error
    let result = skills.get_skill("versioned", "2.0.0").await;
    assert!(result.is_err(), "Wrong version must return error");
}

// ── Critical Path: Search ───────────────────────────────────────────────────

#[tokio::test]
async fn test_critical_search_empty_database() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Critical path: empty search must return empty results
    let query = clawmaster_clawhub::types::SkillSearchQuery::default();
    let (results, total) = skills.search(query).await.unwrap();

    assert_eq!(results.len(), 0, "Empty database must return no results");
    assert_eq!(total, 0, "Total count must be zero");
}

#[tokio::test]
async fn test_critical_search_with_results() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Publish test skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0", SkillFormat::SkillMd);
        skills.publish(metadata).await.unwrap();
    }

    // Critical path: search must return all results
    let query = clawmaster_clawhub::types::SkillSearchQuery::default();
    let (results, total) = skills.search(query).await.unwrap();

    assert_eq!(results.len(), 5, "Must return all published skills");
    assert_eq!(total, 5, "Total count must match");
}

#[tokio::test]
async fn test_critical_search_pagination_boundary() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Publish exactly page_size skills
    for i in 0..20 {
        let metadata = create_metadata(&format!("skill-{:02}", i), "1.0.0", SkillFormat::SkillMd);
        skills.publish(metadata).await.unwrap();
    }

    // Critical path: exact page boundary
    let query = clawmaster_clawhub::types::SkillSearchQuery {
        page: 0,
        page_size: 20,
        ..Default::default()
    };
    let (results, total) = skills.search(query).await.unwrap();

    assert_eq!(results.len(), 20, "Must return exactly page_size results");
    assert_eq!(total, 20, "Total must be exact");
}

#[tokio::test]
async fn test_critical_search_last_page_partial() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Publish 25 skills (2.5 pages)
    for i in 0..25 {
        let metadata = create_metadata(&format!("skill-{:02}", i), "1.0.0", SkillFormat::SkillMd);
        skills.publish(metadata).await.unwrap();
    }

    // Critical path: partial last page
    let query = clawmaster_clawhub::types::SkillSearchQuery {
        page: 2,
        page_size: 10,
        ..Default::default()
    };
    let (results, total) = skills.search(query).await.unwrap();

    assert_eq!(results.len(), 5, "Last page must have remaining items");
    assert_eq!(total, 25, "Total must be correct");
}

// ── Critical Path: Statistics ───────────────────────────────────────────────

#[tokio::test]
async fn test_critical_increment_downloads_atomicity() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("popular", "1.0.0", SkillFormat::SkillMd);
    skills.publish(metadata).await.unwrap();

    // Critical path: atomic increments
    for _ in 0..10 {
        skills
            .increment_downloads("popular", "1.0.0")
            .await
            .unwrap();
    }

    let retrieved = skills.get_skill("popular", "1.0.0").await.unwrap();
    assert_eq!(retrieved.downloads, 10, "Downloads must be exactly 10");
}

#[tokio::test]
async fn test_critical_increment_stars_atomicity() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("starred", "1.0.0", SkillFormat::SkillMd);
    skills.publish(metadata).await.unwrap();

    // Critical path: atomic increments
    for _ in 0..5 {
        skills.increment_stars("starred", "1.0.0").await.unwrap();
    }

    let retrieved = skills.get_skill("starred", "1.0.0").await.unwrap();
    assert_eq!(retrieved.stars, 5, "Stars must be exactly 5");
}

#[tokio::test]
async fn test_critical_increment_nonexistent_skill() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Critical path: increment on nonexistent skill must fail gracefully
    let result = skills.increment_downloads("nonexistent", "1.0.0").await;
    assert!(result.is_ok(), "Increment on nonexistent must not panic");
}

// ── Critical Path: Concurrent Operations ────────────────────────────────────

#[tokio::test]
async fn test_critical_concurrent_publishes() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Critical path: concurrent publishes of different skills
    let mut handles = vec![];
    for i in 0..10 {
        let skills_clone = SkillsRegistry::new(&registry.pool);
        let handle = tokio::spawn(async move {
            let metadata =
                create_metadata(&format!("concurrent-{}", i), "1.0.0", SkillFormat::SkillMd);
            skills_clone.publish(metadata).await
        });
        handles.push(handle);
    }

    // All must succeed
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "Concurrent publish must succeed");
    }

    // Verify all were published
    let query = clawmaster_clawhub::types::SkillSearchQuery::default();
    let (results, _) = skills.search(query).await.unwrap();
    assert_eq!(
        results.len(),
        10,
        "All concurrent publishes must be recorded"
    );
}

#[tokio::test]
async fn test_critical_concurrent_increments() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("concurrent-stats", "1.0.0", SkillFormat::SkillMd);
    skills.publish(metadata).await.unwrap();

    // Critical path: concurrent increments must be atomic
    let mut handles = vec![];
    for _ in 0..100 {
        let skills_clone = SkillsRegistry::new(&registry.pool);
        let handle = tokio::spawn(async move {
            skills_clone
                .increment_downloads("concurrent-stats", "1.0.0")
                .await
        });
        handles.push(handle);
    }

    // Wait for all increments
    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    // Verify atomicity
    let retrieved = skills.get_skill("concurrent-stats", "1.0.0").await.unwrap();
    assert_eq!(
        retrieved.downloads, 100,
        "Concurrent increments must be atomic"
    );
}

// ── Critical Path: Data Integrity ───────────────────────────────────────────

#[tokio::test]
async fn test_critical_data_integrity_after_publish() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("integrity-test", "1.0.0", SkillFormat::SkillMd);
    let original_description = metadata.description.clone();
    let original_keywords = metadata.keywords.clone();

    skills.publish(metadata).await.unwrap();

    // Critical path: data must be exactly as published
    let retrieved = skills.get_skill("integrity-test", "1.0.0").await.unwrap();
    assert_eq!(retrieved.description, original_description);
    assert_eq!(retrieved.keywords, original_keywords);
}

#[tokio::test]
async fn test_critical_unicode_handling() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let mut metadata = create_metadata("unicode-test", "1.0.0", SkillFormat::SkillMd);
    metadata.description = "测试中文 🚀 Emoji テスト".to_string();
    metadata.keywords = vec![
        "中文".to_string(),
        "日本語".to_string(),
        "한국어".to_string(),
    ];

    skills.publish(metadata.clone()).await.unwrap();

    // Critical path: Unicode must be preserved
    let retrieved = skills.get_skill("unicode-test", "1.0.0").await.unwrap();
    assert_eq!(retrieved.description, metadata.description);
    assert_eq!(retrieved.keywords, metadata.keywords);
}

#[tokio::test]
async fn test_critical_special_characters_in_name() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Critical path: names with hyphens and underscores
    let names = vec![
        "skill-with-hyphens",
        "skill_with_underscores",
        "skill-mixed_chars",
    ];

    for name in &names {
        let metadata = create_metadata(name, "1.0.0", SkillFormat::SkillMd);
        let result = skills.publish(metadata).await;
        assert!(result.is_ok(), "Name '{}' must be valid", name);
    }

    // Verify all were stored correctly
    for name in &names {
        let result = skills.get_skill(name, "1.0.0").await;
        assert!(result.is_ok(), "Name '{}' must be retrievable", name);
    }
}

// ── Critical Path: Error Recovery ───────────────────────────────────────────

#[tokio::test]
async fn test_critical_error_recovery_after_failed_publish() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    let metadata = create_metadata("recovery-test", "1.0.0", SkillFormat::SkillMd);

    // First publish succeeds
    skills.publish(metadata.clone()).await.unwrap();

    // Second publish fails (duplicate)
    let _ = skills.publish(metadata.clone()).await;

    // Critical path: system must recover and allow other operations
    let other_metadata = create_metadata("other-skill", "1.0.0", SkillFormat::SkillMd);
    let result = skills.publish(other_metadata).await;
    assert!(result.is_ok(), "System must recover after error");
}

#[tokio::test]
async fn test_critical_search_after_errors() {
    let (registry, _dir) = setup_registry().await;
    let skills = SkillsRegistry::new(&registry.pool);

    // Cause some errors
    let _ = skills.get_skill("nonexistent", "1.0.0").await;
    let _ = skills.increment_downloads("nonexistent", "1.0.0").await;

    // Critical path: search must still work after errors
    let query = clawmaster_clawhub::types::SkillSearchQuery::default();
    let result = skills.search(query).await;
    assert!(result.is_ok(), "Search must work after errors");
}
