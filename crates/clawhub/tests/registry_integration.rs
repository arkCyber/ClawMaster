//! Integration tests for Registry.

use clawmaster_clawhub::registry::Registry;
use clawmaster_clawhub::types::{SearchQuery, SecurityStatus, SortOrder, ToolMetadata, ToolType};
use tempfile::tempdir;
use time::OffsetDateTime;

#[tokio::test]
async fn test_registry_search() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();

    // Publish multiple tools
    for i in 1..=5 {
        let metadata = ToolMetadata {
            name: format!("tool-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Test tool number {}", i),
            readme: None,
            author: "Test".to_string(),
            author_email: None,
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["test".to_string()],
            categories: vec!["testing".to_string()],
            tool_type: ToolType::Pure,
            wasm_hash: format!("hash{}", i),
            wasm_size: 100 * i as u64,
            signature: "sig".to_string(),
            public_key: "key".to_string(),
            downloads: i as u64 * 10,
            security_status: SecurityStatus::Pending,
            published_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };

        registry.publish(metadata).await.unwrap();
    }

    // Search all tools
    let query = SearchQuery {
        query: None,
        category: None,
        tool_type: None,
        security_status: None,
        sort: SortOrder::Downloads,
        page: 0,
        page_size: 10,
    };

    let (tools, _total) = registry.search(query).await.unwrap();
    assert_eq!(tools.len(), 5);

    // First tool should have highest downloads
    assert_eq!(tools[0].name, "tool-5");
}

#[tokio::test]
async fn test_registry_category_filter() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();

    // Publish tools with different categories
    let metadata1 = ToolMetadata {
        name: "calc-tool".to_string(),
        version: "1.0.0".to_string(),
        description: "Calculator".to_string(),
        readme: None,
        author: "Test".to_string(),
        author_email: None,
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        categories: vec!["math".to_string()],
        tool_type: ToolType::Pure,
        wasm_hash: "hash1".to_string(),
        wasm_size: 100,
        signature: "sig".to_string(),
        public_key: "key".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    let metadata2 = ToolMetadata {
        name: "web-tool".to_string(),
        version: "1.0.0".to_string(),
        description: "Web fetcher".to_string(),
        readme: None,
        author: "Test".to_string(),
        author_email: None,
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        categories: vec!["web".to_string()],
        tool_type: ToolType::Http,
        wasm_hash: "hash2".to_string(),
        wasm_size: 200,
        signature: "sig".to_string(),
        public_key: "key".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    registry.publish(metadata1).await.unwrap();
    registry.publish(metadata2).await.unwrap();

    // Search by category
    let query = SearchQuery {
        query: None,
        category: Some("math".to_string()),
        tool_type: None,
        security_status: None,
        sort: SortOrder::Name,
        page: 0,
        page_size: 10,
    };

    let (tools, _total) = registry.search(query).await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "calc-tool");
}

#[tokio::test]
async fn test_registry_increment_downloads() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();

    let metadata = ToolMetadata {
        name: "download-test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test".to_string(),
        readme: None,
        author: "Test".to_string(),
        author_email: None,
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        categories: vec![],
        tool_type: ToolType::Pure,
        wasm_hash: "hash".to_string(),
        wasm_size: 100,
        signature: "sig".to_string(),
        public_key: "key".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    registry.publish(metadata).await.unwrap();

    // Increment downloads
    registry.increment_downloads("download-test", "1.0.0").await.unwrap();
    registry.increment_downloads("download-test", "1.0.0").await.unwrap();

    // Check downloads
    let tool = registry.get_tool("download-test", "1.0.0").await.unwrap();
    assert_eq!(tool.downloads, 2);
}
