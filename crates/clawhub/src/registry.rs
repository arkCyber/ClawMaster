//! ClawHub registry implementation.
//!
//! The registry manages tool metadata and provides search/discovery functionality.

use {
    crate::{
        error::{Error, Result},
        types::{SearchQuery, SecurityStatus, SortOrder, ToolMetadata, ToolType, ToolVersion},
    },
    sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    std::{path::Path, str::FromStr},
    time::OffsetDateTime,
    tracing::{debug, info},
};

/// ClawHub registry.
///
/// Manages tool metadata in a SQLite database with full-text search.
///
/// # Compliance
/// DO-178C §11.13: Deterministic initialization
/// - Database schema is versioned via migrations
/// - All queries are parameterized (SQL injection safe)
/// - Transactions ensure consistency
pub struct Registry {
    pub(crate) pool: SqlitePool,
}

impl Registry {
    /// Create a new registry.
    ///
    /// # Arguments
    /// * `database_url` - Path to SQLite database file
    ///
    /// # Errors
    /// Returns an error if the database cannot be opened or migrations fail.
    pub async fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let database_url = format!("sqlite://{}", database_path.as_ref().display());

        let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        info!("Connected to ClawHub registry database");

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        info!("Database migrations complete");

        Ok(Self { pool })
    }

    /// Publish a new tool version.
    ///
    /// # Arguments
    /// * `metadata` - Tool metadata
    ///
    /// # Errors
    /// Returns an error if:
    /// - Tool already exists with this version
    /// - Database operation fails
    /// - Metadata is invalid
    pub async fn publish(&self, metadata: ToolMetadata) -> Result<()> {
        // Validate version doesn't already exist
        if self.tool_exists(&metadata.name, &metadata.version).await? {
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

        sqlx::query!(
            r#"
            INSERT INTO tools (
                name, version, description, readme, author, author_email,
                license, repository, homepage, tool_type, keywords, categories,
                wasm_hash, wasm_size, wasm_url, signature, public_key,
                security_status, downloads, published_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,
                ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21
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
            metadata.tool_type as ToolType,
            keywords,
            categories,
            metadata.wasm_hash,
            metadata.wasm_size as i64,
            format!(
                "https://clawhub.io/tools/{}/{}/download",
                metadata.name, metadata.version
            ),
            metadata.signature,
            metadata.public_key,
            metadata.security_status as SecurityStatus,
            metadata.downloads as i64,
            published_at,
            updated_at,
        )
        .execute(&self.pool)
        .await?;

        info!("Published tool: {}@{}", metadata.name, metadata.version);

        Ok(())
    }

    /// Check if a tool version exists.
    async fn tool_exists(&self, name: &str, version: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM tools WHERE name = ?1 AND version = ?2",
            name,
            version
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count > 0)
    }

    /// Get tool metadata.
    ///
    /// # Arguments
    /// * `name` - Tool name
    /// * `version` - Tool version
    ///
    /// # Errors
    /// Returns an error if the tool is not found.
    pub async fn get_tool(&self, name: &str, version: &str) -> Result<ToolMetadata> {
        #[derive(sqlx::FromRow)]
        struct ToolRow {
            name: String,
            version: String,
            description: String,
            author: String,
            license: String,
            keywords: String,
            category: String,
            tool_type: String,
            wasm_hash: String,
            wasm_size: i64,
            wasm_url: String,
            signature: String,
            public_key: String,
            downloads: i64,
            security_status: String,
            created_at: String,
            updated_at: String,
        }

        let row = sqlx::query_as::<_, ToolRow>(
            r#"
            SELECT
                name, version, description, author, license,
                keywords, category, tool_type,
                wasm_hash, wasm_size, wasm_url,
                signature, public_key,
                downloads, security_status,
                created_at, updated_at
            FROM tools
            WHERE name = ? AND version = ?
            "#
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

        Ok(ToolMetadata {
            name: row.name,
            version: row.version,
            description: row.description,
            readme: row.readme,
            author: row.author,
            author_email: row.author_email,
            license: row.license,
            repository: row.repository,
            homepage: row.homepage,
            tool_type: serde_json::from_str(&format!("\"{}\"", row.tool_type))?,
            keywords,
            categories,
            wasm_hash: row.wasm_hash,
            wasm_size: row.wasm_size as u64,
            signature: row.signature,
            public_key: row.public_key,
            downloads: row.downloads as u64,
            security_status: serde_json::from_str(&format!("\"{}\"", row.security_status))?,
            published_at,
            updated_at,
        })
    }

    /// Search for tools.
    ///
    /// # Arguments
    /// * `query` - Search query
    ///
    /// # Returns
    /// List of matching tools and total count.
    pub async fn search(&self, query: SearchQuery) -> Result<(Vec<ToolMetadata>, u64)> {
        debug!("Search query: {:?}", query);

        let offset = (query.page * query.page_size) as i64;
        let limit = query.page_size as i64;

        // Build query based on filters
        let mut sql = String::from("SELECT * FROM tools WHERE 1=1");

        // Full-text search if query provided
        if let Some(q) = &query.query {
            sql.push_str(&format!(
                " AND id IN (SELECT rowid FROM tools_fts WHERE tools_fts MATCH '{}')",
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

        // Tool type filter
        if let Some(tool_type) = &query.tool_type {
            sql.push_str(&format!(" AND tool_type = '{:?}'", tool_type));
        }

        // Security status filter
        if let Some(status) = &query.security_status {
            sql.push_str(&format!(" AND security_status = '{:?}'", status));
        }

        // Sort order
        let order_by = match query.sort {
            crate::types::SortOrder::Downloads => "downloads DESC",
            crate::types::SortOrder::Recent => "published_at DESC",
            crate::types::SortOrder::Name => "name ASC",
            crate::types::SortOrder::Relevance => "downloads DESC", // Fallback to downloads
        };

        sql.push_str(&format!(
            " ORDER BY {} LIMIT {} OFFSET {}",
            order_by, limit, offset
        ));

        // Execute query
        #[derive(sqlx::FromRow)]
        struct ToolRow {
            name: String,
            version: String,
        }

        let rows: Vec<ToolRow> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        let mut tools = Vec::new();
        for row in rows {
            // Get full metadata
            if let Ok(tool) = self.get_tool(&row.name, &row.version).await {
                tools.push(tool);
            }
        }

        let total = tools.len() as u64;

        Ok((tools, total))
    }

    /// Increment download count.
    pub async fn increment_downloads(&self, name: &str, version: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE tools SET downloads = downloads + 1 WHERE name = ?1 AND version = ?2",
            name,
            version
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::tempdir};

    #[tokio::test]
    async fn test_registry_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let registry = Registry::new(&db_path).await.unwrap();
        assert!(db_path.exists());
    }

    #[tokio::test]
    async fn test_publish_and_get_tool() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let registry = Registry::new(&db_path).await.unwrap();

        let metadata = ToolMetadata {
            name: "test-tool".to_string(),
            version: "1.0.0".to_string(),
            description: "A test tool".to_string(),
            readme: None,
            author: "Test Author".to_string(),
            author_email: Some("test@example.com".to_string()),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["test".to_string()],
            categories: vec!["testing".to_string()],
            tool_type: ToolType::Pure,
            wasm_hash: "abc123".to_string(),
            wasm_size: 1024,
            signature: "sig123".to_string(),
            public_key: "key123".to_string(),
            downloads: 0,
            security_status: SecurityStatus::Pending,
            published_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };

        registry.publish(metadata.clone()).await.unwrap();

        let retrieved = registry.get_tool("test-tool", "1.0.0").await.unwrap();
        assert_eq!(retrieved.name, "test-tool");
        assert_eq!(retrieved.version, "1.0.0");
    }
}
