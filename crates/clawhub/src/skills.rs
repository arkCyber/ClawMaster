//! Skills registry implementation.
//!
//! Manages skill metadata and provides search/discovery functionality.

use {
    crate::{
        error::{Error, Result},
        types::{SecurityStatus, SkillFormat, SkillMetadata, SkillSearchQuery, SortOrder},
    },
    sqlx::SqlitePool,
    time::OffsetDateTime,
    tracing::{debug, info},
};

#[derive(sqlx::FromRow)]
struct SkillRow {
    name: String,
    version: String,
    description: String,
    readme: String,
    author: String,
    author_email: String,
    license: String,
    repository: Option<String>,
    homepage: Option<String>,
    keywords: String,
    categories: String,
    wasm_hash: Option<String>,
    wasm_size: Option<i64>,
    wasm_url: Option<String>,
    signature: Option<String>,
    public_key: Option<String>,
    skill_format: String,
    github_repo: Option<String>,
    commit_sha: Option<String>,
    downloads: i64,
    stars: i64,
    security_status: String,
    published_at: String,
    updated_at: String,
}

/// Skills registry operations.
pub struct SkillsRegistry<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SkillsRegistry<'a> {
    /// Create a new skills registry.
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Publish a new skill version.
    ///
    /// # Arguments
    /// * `metadata` - Skill metadata
    ///
    /// # Errors
    /// Returns an error if:
    /// - Skill already exists with this version
    /// - Database operation fails
    /// - Metadata is invalid
    pub async fn publish(&self, metadata: SkillMetadata) -> Result<()> {
        // Validate version doesn't already exist
        if self.skill_exists(&metadata.name, &metadata.version).await? {
            return Err(Error::ToolAlreadyExists {
                name: metadata.name,
                version: metadata.version,
            });
        }

        // Serialize arrays to JSON
        let keywords = serde_json::to_string(&metadata.keywords)?;
        let categories = serde_json::to_string(&metadata.categories)?;

        // Convert timestamps to RFC3339 strings
        let published_at = metadata
            .published_at
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|e| Error::InvalidMetadata(e.to_string()))?;
        let updated_at = metadata
            .updated_at
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|e| Error::InvalidMetadata(e.to_string()))?;

        let skill_format = format!("{:?}", metadata.skill_format);
        let security_status = format!("{:?}", metadata.security_status);

        sqlx::query!(
            r#"
            INSERT INTO skills (
                name, version, description, readme, author, author_email,
                license, repository, homepage, keywords, categories,
                skill_format, github_repo, commit_sha, downloads, stars,
                security_status, published_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11,
                ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19
            )
            "#,
            metadata.name,
            metadata.version,
            metadata.description,
            metadata.readme,
            metadata.author,
            metadata.author_email,
            metadata.license,
            metadata.repository,
            metadata.homepage,
            keywords,
            categories,
            skill_format,
            metadata.github_repo,
            metadata.commit_sha,
            metadata.downloads as i64,
            metadata.stars as i64,
            security_status,
            published_at,
            updated_at,
        )
        .execute(self.pool)
        .await?;

        info!("Published skill: {}@{}", metadata.name, metadata.version);

        Ok(())
    }

    /// Check if a skill version exists.
    async fn skill_exists(&self, name: &str, version: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM skills WHERE name = ?1 AND version = ?2",
            name,
            version
        )
        .fetch_one(self.pool)
        .await?;

        Ok(result.count > 0)
    }

    /// Get skill metadata.
    ///
    /// # Arguments
    /// * `name` - Skill name
    /// * `version` - Skill version
    ///
    /// # Errors
    /// Returns an error if the skill is not found.
    pub async fn get_skill(&self, name: &str, version: &str) -> Result<SkillMetadata> {
        let row = sqlx::query_as::<_, SkillRow>(
            r#"
            SELECT
                name, version, description, readme, author, author_email,
                license, repository, homepage, keywords, categories,
                wasm_hash, wasm_size, wasm_url, signature, public_key,
                skill_format, github_repo, commit_sha, downloads, stars,
                security_status, published_at, updated_at
            FROM skills
            WHERE name = ? AND version = ?
            "#,
        )
        .bind(name)
        .bind(version)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::ToolNotFound {
            name: name.to_string(),
            version: version.to_string(),
        })?;

        // Deserialize JSON arrays
        let keywords: Vec<String> = serde_json::from_str(&row.keywords)?;
        let categories: Vec<String> = serde_json::from_str(&row.categories)?;

        // Parse timestamps
        let published_at = OffsetDateTime::parse(
            &row.published_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|e| Error::InvalidMetadata(e.to_string()))?;
        let updated_at = OffsetDateTime::parse(
            &row.updated_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|e| Error::InvalidMetadata(e.to_string()))?;

        // Parse skill format
        let skill_format = match row.skill_format.as_str() {
            "SkillMd" => SkillFormat::SkillMd,
            "ClaudeCode" => SkillFormat::ClaudeCode,
            _ => SkillFormat::Custom,
        };

        // Parse security status
        let security_status = match row.security_status.as_str() {
            "Pending" => SecurityStatus::Pending,
            "Scanning" => SecurityStatus::Scanning,
            "Verified" => SecurityStatus::Verified,
            "Failed" => SecurityStatus::Failed,
            "Approved" => SecurityStatus::Approved,
            _ => SecurityStatus::Pending,
        };

        Ok(SkillMetadata {
            name: row.name,
            version: row.version,
            description: row.description,
            readme: row.readme,
            author: row.author,
            author_email: row.author_email,
            license: row.license,
            repository: row.repository,
            homepage: row.homepage,
            keywords,
            categories,
            skill_format,
            github_repo: row.github_repo,
            commit_sha: row.commit_sha,
            downloads: row.downloads as u64,
            stars: row.stars as u64,
            security_status,
            published_at,
            updated_at,
        })
    }

    /// Search for skills.
    ///
    /// # Arguments
    /// * `query` - Search query
    ///
    /// # Returns
    /// List of matching skills and total count.
    pub async fn search(&self, query: SkillSearchQuery) -> Result<(Vec<SkillMetadata>, u64)> {
        debug!("Skill search query: {:?}", query);

        let offset = (query.page * query.page_size) as i64;
        let limit = query.page_size as i64;

        // Build query based on filters
        let mut sql = String::from("SELECT * FROM skills WHERE 1=1");

        // Full-text search if query provided
        if let Some(q) = &query.query {
            sql.push_str(&format!(
                " AND id IN (SELECT rowid FROM skills_fts WHERE skills_fts MATCH '{}')",
                q.replace('\'', "''")
            ));
        }

        // Category filter
        if let Some(cat) = &query.category {
            sql.push_str(&format!(
                " AND categories LIKE '%{}%'",
                cat.replace('\'', "''")
            ));
        }

        // Skill format filter
        if let Some(format) = &query.skill_format {
            sql.push_str(&format!(" AND skill_format = '{:?}'", format));
        }

        // Security status filter
        if let Some(status) = &query.security_status {
            sql.push_str(&format!(" AND security_status = '{:?}'", status));
        }

        // Sort order
        let order_by = match query.sort {
            SortOrder::Downloads => "downloads DESC",
            SortOrder::Recent => "published_at DESC",
            SortOrder::Name => "name ASC",
            SortOrder::Relevance => "downloads DESC", // Fallback to downloads
        };

        sql.push_str(&format!(
            " ORDER BY {} LIMIT {} OFFSET {}",
            order_by, limit, offset
        ));

        // Execute query
        #[derive(sqlx::FromRow)]
        struct SkillRow {
            name: String,
            version: String,
        }

        let rows: Vec<SkillRow> = sqlx::query_as(&sql).fetch_all(self.pool).await?;

        let mut skills = Vec::new();
        for row in rows {
            // Get full metadata
            if let Ok(skill) = self.get_skill(&row.name, &row.version).await {
                skills.push(skill);
            }
        }

        let total = skills.len() as u64;

        Ok((skills, total))
    }

    /// Increment download count.
    pub async fn increment_downloads(&self, name: &str, version: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE skills SET downloads = downloads + 1 WHERE name = ?1 AND version = ?2",
            name,
            version
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Increment star count.
    pub async fn increment_stars(&self, name: &str, version: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE skills SET stars = stars + 1 WHERE name = ?1 AND version = ?2",
            name,
            version
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::registry::Registry, tempfile::tempdir};

    fn create_test_metadata(name: &str, version: &str) -> SkillMetadata {
        SkillMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: format!("Test skill {}", name),
            readme: None,
            author: "Test Author".to_string(),
            author_email: Some("test@example.com".to_string()),
            license: "MIT".to_string(),
            repository: Some(format!("https://github.com/test/{}", name)),
            homepage: None,
            keywords: vec!["test".to_string()],
            categories: vec!["testing".to_string()],
            skill_format: SkillFormat::SkillMd,
            github_repo: Some(format!("test/{}", name)),
            commit_sha: Some("abc123".to_string()),
            downloads: 0,
            stars: 0,
            security_status: SecurityStatus::Pending,
            published_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }

    async fn setup_test_registry() -> Registry {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        Registry::new(&db_path).await.unwrap()
    }

    #[tokio::test]
    async fn test_publish_and_get_skill() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = create_test_metadata("test-skill", "1.0.0");
        skills.publish(metadata.clone()).await.unwrap();

        let retrieved = skills.get_skill("test-skill", "1.0.0").await.unwrap();
        assert_eq!(retrieved.name, "test-skill");
        assert_eq!(retrieved.version, "1.0.0");
        assert_eq!(retrieved.skill_format, SkillFormat::SkillMd);
        assert_eq!(retrieved.author, "Test Author");
    }

    #[tokio::test]
    async fn test_duplicate_publish_rejected() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = create_test_metadata("duplicate", "1.0.0");

        // First publish succeeds
        skills.publish(metadata.clone()).await.unwrap();

        // Second publish should fail
        let result = skills.publish(metadata).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::ToolAlreadyExists { .. }
        ));
    }

    #[tokio::test]
    async fn test_different_versions_allowed() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let v1 = create_test_metadata("versioned", "1.0.0");
        let v2 = create_test_metadata("versioned", "2.0.0");

        skills.publish(v1).await.unwrap();
        skills.publish(v2).await.unwrap();

        let retrieved_v1 = skills.get_skill("versioned", "1.0.0").await.unwrap();
        let retrieved_v2 = skills.get_skill("versioned", "2.0.0").await.unwrap();

        assert_eq!(retrieved_v1.version, "1.0.0");
        assert_eq!(retrieved_v2.version, "2.0.0");
    }

    #[tokio::test]
    async fn test_get_nonexistent_skill() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let result = skills.get_skill("nonexistent", "1.0.0").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ToolNotFound { .. }));
    }

    #[tokio::test]
    async fn test_search_skills() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = create_test_metadata("search-test", "1.0.0");
        skills.publish(metadata).await.unwrap();

        let query = SkillSearchQuery::default();
        let (results, _total) = skills.search(query).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "search-test");
    }

    #[tokio::test]
    async fn test_search_with_query() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let mut metadata = create_test_metadata("web-scraper", "1.0.0");
        metadata.description = "A web scraping tool".to_string();
        metadata.keywords = vec!["web".to_string(), "scraping".to_string()];
        skills.publish(metadata).await.unwrap();

        let query = SkillSearchQuery {
            query: Some("web".to_string()),
            ..Default::default()
        };
        let (results, _total) = skills.search(query).await.unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].name, "web-scraper");
    }

    #[tokio::test]
    async fn test_search_pagination() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        // Publish 25 skills
        for i in 0..25 {
            let metadata = create_test_metadata(&format!("skill-{:02}", i), "1.0.0");
            skills.publish(metadata).await.unwrap();
        }

        // Test page 1
        let page1 = skills
            .search(SkillSearchQuery {
                page: 0,
                page_size: 10,
                ..Default::default()
            })
            .await
            .unwrap();

        // Test page 2
        let page2 = skills
            .search(SkillSearchQuery {
                page: 1,
                page_size: 10,
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(page1.0.len(), 10);
        assert_eq!(page2.0.len(), 10);
        assert_ne!(page1.0[0].name, page2.0[0].name);
    }

    #[tokio::test]
    async fn test_search_by_category() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let mut web_skill = create_test_metadata("web-tool", "1.0.0");
        web_skill.categories = vec!["web".to_string()];
        skills.publish(web_skill).await.unwrap();

        let mut data_skill = create_test_metadata("data-tool", "1.0.0");
        data_skill.categories = vec!["data".to_string()];
        skills.publish(data_skill).await.unwrap();

        let query = SkillSearchQuery {
            category: Some("web".to_string()),
            ..Default::default()
        };
        let (results, _total) = skills.search(query).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "web-tool");
    }

    #[tokio::test]
    async fn test_search_by_format() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let mut skill_md = create_test_metadata("skill-md", "1.0.0");
        skill_md.skill_format = SkillFormat::SkillMd;
        skills.publish(skill_md).await.unwrap();

        let mut claude_code = create_test_metadata("claude-code", "1.0.0");
        claude_code.skill_format = SkillFormat::ClaudeCode;
        skills.publish(claude_code).await.unwrap();

        let query = SkillSearchQuery {
            skill_format: Some(SkillFormat::ClaudeCode),
            ..Default::default()
        };
        let (results, _total) = skills.search(query).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "claude-code");
    }

    #[tokio::test]
    async fn test_increment_downloads() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = create_test_metadata("popular", "1.0.0");
        skills.publish(metadata).await.unwrap();

        // Increment downloads
        skills
            .increment_downloads("popular", "1.0.0")
            .await
            .unwrap();
        skills
            .increment_downloads("popular", "1.0.0")
            .await
            .unwrap();

        let retrieved = skills.get_skill("popular", "1.0.0").await.unwrap();
        assert_eq!(retrieved.downloads, 2);
    }

    #[tokio::test]
    async fn test_increment_stars() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = create_test_metadata("starred", "1.0.0");
        skills.publish(metadata).await.unwrap();

        // Increment stars
        skills.increment_stars("starred", "1.0.0").await.unwrap();
        skills.increment_stars("starred", "1.0.0").await.unwrap();
        skills.increment_stars("starred", "1.0.0").await.unwrap();

        let retrieved = skills.get_skill("starred", "1.0.0").await.unwrap();
        assert_eq!(retrieved.stars, 3);
    }

    #[tokio::test]
    async fn test_sql_injection_protection() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        // Attempt SQL injection in search query
        let query = SkillSearchQuery {
            query: Some("'; DROP TABLE skills; --".to_string()),
            ..Default::default()
        };

        // Should handle safely without error
        let result = skills.search(query).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_empty_search_results() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let query = SkillSearchQuery {
            query: Some("nonexistent-keyword".to_string()),
            ..Default::default()
        };
        let (results, total) = skills.search(query).await.unwrap();

        assert_eq!(results.len(), 0);
        assert_eq!(total, 0);
    }

    #[tokio::test]
    async fn test_skill_with_all_fields() {
        let registry = setup_test_registry().await;
        let skills = SkillsRegistry::new(&registry.pool);

        let metadata = SkillMetadata {
            name: "complete-skill".to_string(),
            version: "1.0.0".to_string(),
            description: "A complete skill with all fields".to_string(),
            readme: Some("# README\n\nThis is a test skill.".to_string()),
            author: "Test Author".to_string(),
            author_email: Some("test@example.com".to_string()),
            license: "MIT".to_string(),
            repository: Some("https://github.com/test/complete".to_string()),
            homepage: Some("https://example.com".to_string()),
            keywords: vec!["complete".to_string(), "test".to_string()],
            categories: vec!["testing".to_string(), "example".to_string()],
            skill_format: SkillFormat::SkillMd,
            github_repo: Some("test/complete".to_string()),
            commit_sha: Some("abc123def456".to_string()),
            downloads: 100,
            stars: 50,
            security_status: SecurityStatus::Verified,
            published_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };

        skills.publish(metadata.clone()).await.unwrap();

        let retrieved = skills.get_skill("complete-skill", "1.0.0").await.unwrap();
        assert_eq!(retrieved.name, "complete-skill");
        assert_eq!(retrieved.readme, metadata.readme);
        assert_eq!(retrieved.homepage, metadata.homepage);
        assert_eq!(retrieved.keywords, metadata.keywords);
        assert_eq!(retrieved.categories, metadata.categories);
        assert_eq!(retrieved.downloads, 100);
        assert_eq!(retrieved.stars, 50);
    }
}
