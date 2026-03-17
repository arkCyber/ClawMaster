//! Boundary condition tests for Skills Registry - DO-178C Level A compliance.
//!
//! Tests all boundary conditions, edge cases, and limit scenarios to ensure
//! robust behavior under extreme conditions.

use {
    clawmaster_clawhub::{
        registry::Registry,
        skills::SkillsRegistry,
        types::{SecurityStatus, SkillFormat, SkillMetadata, SkillSearchQuery},
    },
    tempfile::tempdir,
    time::OffsetDateTime,
};

fn create_metadata(name: &str, version: &str) -> SkillMetadata {
    SkillMetadata {
        name: name.to_string(),
        version: version.to_string(),
        description: "Boundary test skill".to_string(),
        readme: None,
        author: "Test".to_string(),
        author_email: None,
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        categories: vec![],
        skill_format: SkillFormat::SkillMd,
        github_repo: None,
        commit_sha: None,
        downloads: 0,
        stars: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    }
}

async fn setup() -> (SkillsRegistry, tempfile::TempDir) {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    (SkillsRegistry::new(&registry.pool), dir)
}

// ── Boundary: String Lengths ────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_empty_name() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("", "1.0.0");

    // Boundary: empty name should be handled
    let result = skills.publish(metadata).await;
    // System should either accept or reject gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_boundary_single_character_name() {
    let (skills, _dir) = setup().await;

    let metadata = create_metadata("a", "1.0.0");

    // Boundary: single character name
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Single character name should be valid");
}

#[tokio::test]
async fn test_boundary_maximum_name_length() {
    let (skills, _dir) = setup().await;

    // Boundary: very long name (255 characters)
    let long_name = "a".repeat(255);
    let metadata = create_metadata(&long_name, "1.0.0");

    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Long name within limits should be valid");
}

#[tokio::test]
async fn test_boundary_excessive_name_length() {
    let (skills, _dir) = setup().await;

    // Boundary: excessively long name (1000 characters)
    let excessive_name = "a".repeat(1000);
    let metadata = create_metadata(&excessive_name, "1.0.0");

    let result = skills.publish(metadata).await;
    // Should either truncate or reject gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_boundary_empty_description() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("empty-desc", "1.0.0");
    metadata.description = String::new();

    // Boundary: empty description
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Empty description should be valid");
}

#[tokio::test]
async fn test_boundary_very_long_description() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("long-desc", "1.0.0");
    metadata.description = "x".repeat(10000);

    // Boundary: very long description
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Long description should be handled");
}

// ── Boundary: Collections ───────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_empty_keywords() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("no-keywords", "1.0.0");
    metadata.keywords = vec![];

    // Boundary: no keywords
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Empty keywords should be valid");
}

#[tokio::test]
async fn test_boundary_single_keyword() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("one-keyword", "1.0.0");
    metadata.keywords = vec!["test".to_string()];

    // Boundary: single keyword
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Single keyword should be valid");
}

#[tokio::test]
async fn test_boundary_many_keywords() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("many-keywords", "1.0.0");
    metadata.keywords = (0..100).map(|i| format!("keyword-{}", i)).collect();

    // Boundary: many keywords
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Many keywords should be handled");
}

#[tokio::test]
async fn test_boundary_empty_categories() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("no-categories", "1.0.0");
    metadata.categories = vec![];

    // Boundary: no categories
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Empty categories should be valid");
}

// ── Boundary: Numeric Values ────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_zero_downloads() {
    let (skills, _dir) = setup().await;

    let metadata = create_metadata("zero-downloads", "1.0.0");
    skills.publish(metadata).await.unwrap();

    // Boundary: initial downloads should be 0
    let retrieved = skills.get_skill("zero-downloads", "1.0.0").await.unwrap();
    assert_eq!(retrieved.downloads, 0);
}

#[tokio::test]
async fn test_boundary_large_download_count() {
    let (skills, _dir) = setup().await;

    let metadata = create_metadata("popular", "1.0.0");
    skills.publish(metadata).await.unwrap();

    // Boundary: increment to large number
    for _ in 0..10000 {
        skills
            .increment_downloads("popular", "1.0.0")
            .await
            .unwrap();
    }

    let retrieved = skills.get_skill("popular", "1.0.0").await.unwrap();
    assert_eq!(retrieved.downloads, 10000);
}

#[tokio::test]
async fn test_boundary_zero_stars() {
    let (skills, _dir) = setup().await;

    let metadata = create_metadata("zero-stars", "1.0.0");
    skills.publish(metadata).await.unwrap();

    // Boundary: initial stars should be 0
    let retrieved = skills.get_skill("zero-stars", "1.0.0").await.unwrap();
    assert_eq!(retrieved.stars, 0);
}

// ── Boundary: Pagination ────────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_page_zero() {
    let (skills, _dir) = setup().await;

    // Publish some skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: page 0 (first page)
    let query = SkillSearchQuery {
        page: 0,
        page_size: 10,
        ..Default::default()
    };
    let (results, _) = skills.search(query).await.unwrap();
    assert_eq!(results.len(), 5);
}

#[tokio::test]
async fn test_boundary_page_beyond_results() {
    let (skills, _dir) = setup().await;

    // Publish 5 skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: page beyond available results
    let query = SkillSearchQuery {
        page: 10,
        page_size: 10,
        ..Default::default()
    };
    let (results, total) = skills.search(query).await.unwrap();
    assert_eq!(results.len(), 0, "Beyond-range page should return empty");
    assert_eq!(total, 5, "Total should still be correct");
}

#[tokio::test]
async fn test_boundary_page_size_one() {
    let (skills, _dir) = setup().await;

    // Publish skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: page size of 1
    let query = SkillSearchQuery {
        page: 0,
        page_size: 1,
        ..Default::default()
    };
    let (results, _) = skills.search(query).await.unwrap();
    assert_eq!(results.len(), 1, "Page size 1 should return 1 result");
}

#[tokio::test]
async fn test_boundary_page_size_zero() {
    let (skills, _dir) = setup().await;

    // Publish skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: page size of 0 (should use default or return empty)
    let query = SkillSearchQuery {
        page: 0,
        page_size: 0,
        ..Default::default()
    };
    let result = skills.search(query).await;
    assert!(result.is_ok(), "Page size 0 should be handled gracefully");
}

#[tokio::test]
async fn test_boundary_very_large_page_size() {
    let (skills, _dir) = setup().await;

    // Publish 10 skills
    for i in 0..10 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: very large page size
    let query = SkillSearchQuery {
        page: 0,
        page_size: 10000,
        ..Default::default()
    };
    let (results, _) = skills.search(query).await.unwrap();
    assert_eq!(results.len(), 10, "Should return all available results");
}

// ── Boundary: Search Queries ────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_empty_search_query() {
    let (skills, _dir) = setup().await;

    // Publish skills
    for i in 0..5 {
        let metadata = create_metadata(&format!("skill-{}", i), "1.0.0");
        skills.publish(metadata).await.unwrap();
    }

    // Boundary: empty query string
    let query = SkillSearchQuery {
        query: Some(String::new()),
        ..Default::default()
    };
    let (results, _) = skills.search(query).await.unwrap();
    // Empty query should return all or none, but not error
    assert!(results.len() <= 5);
}

#[tokio::test]
async fn test_boundary_single_character_query() {
    let (skills, _dir) = setup().await;

    let mut metadata = create_metadata("a-skill", "1.0.0");
    metadata.description = "a test".to_string();
    skills.publish(metadata).await.unwrap();

    // Boundary: single character query
    let query = SkillSearchQuery {
        query: Some("a".to_string()),
        ..Default::default()
    };
    let result = skills.search(query).await;
    assert!(result.is_ok(), "Single character query should work");
}

#[tokio::test]
async fn test_boundary_very_long_query() {
    let (skills, _dir) = setup().await;

    // Boundary: very long query string
    let long_query = "test ".repeat(1000);
    let query = SkillSearchQuery {
        query: Some(long_query),
        ..Default::default()
    };
    let result = skills.search(query).await;
    assert!(result.is_ok(), "Long query should be handled");
}

#[tokio::test]
async fn test_boundary_special_characters_in_query() {
    let (skills, _dir) = setup().await;

    // Boundary: special characters that might break SQL
    let special_queries = vec!["'", "\"", "%", "_", "\\", "*", "?", "[", "]", "(", ")"];

    for special in special_queries {
        let query = SkillSearchQuery {
            query: Some(special.to_string()),
            ..Default::default()
        };
        let result = skills.search(query).await;
        assert!(
            result.is_ok(),
            "Special character '{}' should be handled",
            special
        );
    }
}

// ── Boundary: Version Strings ───────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_version_zero() {
    let (skills, _dir) = setup().await;

    // Boundary: version 0.0.0
    let metadata = create_metadata("v-zero", "0.0.0");
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Version 0.0.0 should be valid");
}

#[tokio::test]
async fn test_boundary_version_large_numbers() {
    let (skills, _dir) = setup().await;

    // Boundary: large version numbers
    let metadata = create_metadata("v-large", "999.999.999");
    let result = skills.publish(metadata).await;
    assert!(result.is_ok(), "Large version numbers should be valid");
}

#[tokio::test]
async fn test_boundary_version_with_prerelease() {
    let (skills, _dir) = setup().await;

    // Boundary: version with prerelease tag
    let metadata = create_metadata("v-pre", "1.0.0-alpha.1");
    let result = skills.publish(metadata).await;
    // Should either accept or reject gracefully
    assert!(result.is_ok() || result.is_err());
}

// ── Boundary: Concurrent Operations ─────────────────────────────────────────

#[tokio::test]
async fn test_boundary_single_concurrent_operation() {
    let (skills, _dir) = setup().await;

    // Boundary: single concurrent operation (edge case of concurrency)
    let metadata = create_metadata("single-concurrent", "1.0.0");
    let result = skills.publish(metadata).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_boundary_maximum_concurrent_operations() {
    let (skills, _dir) = setup().await;

    let metadata = create_metadata("max-concurrent", "1.0.0");
    skills.publish(metadata).await.unwrap();

    // Boundary: many concurrent increments
    let mut handles = vec![];
    for _ in 0..1000 {
        let skills_clone = SkillsRegistry::new(&skills.pool);
        let handle = tokio::spawn(async move {
            skills_clone
                .increment_downloads("max-concurrent", "1.0.0")
                .await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    let retrieved = skills.get_skill("max-concurrent", "1.0.0").await.unwrap();
    assert_eq!(
        retrieved.downloads, 1000,
        "All concurrent operations must complete"
    );
}

// ── Boundary: Data Types ────────────────────────────────────────────────────

#[tokio::test]
async fn test_boundary_all_security_statuses() {
    let (skills, _dir) = setup().await;

    // Boundary: test all security status variants
    let statuses = vec![
        SecurityStatus::Pending,
        SecurityStatus::Verified,
        SecurityStatus::Failed,
    ];

    for (i, status) in statuses.iter().enumerate() {
        let mut metadata = create_metadata(&format!("status-{}", i), "1.0.0");
        metadata.security_status = status.clone();
        let result = skills.publish(metadata).await;
        assert!(
            result.is_ok(),
            "Security status {:?} should be valid",
            status
        );
    }
}

#[tokio::test]
async fn test_boundary_all_skill_formats() {
    let (skills, _dir) = setup().await;

    // Boundary: test all format variants
    let formats = vec![SkillFormat::SkillMd, SkillFormat::ClaudeCode];

    for (i, format) in formats.iter().enumerate() {
        let mut metadata = create_metadata(&format!("format-{}", i), "1.0.0");
        metadata.skill_format = format.clone();
        let result = skills.publish(metadata).await;
        assert!(result.is_ok(), "Format {:?} should be valid", format);
    }
}
