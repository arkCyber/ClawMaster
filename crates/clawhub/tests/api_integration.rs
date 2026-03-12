//! Integration tests for ClawHub API.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use clawmaster_clawhub::api::{routes, ApiState};
use clawmaster_clawhub::registry::Registry;
use clawmaster_clawhub::types::{PublishRequest, SecurityStatus, ToolMetadata, ToolType};
use tempfile::tempdir;
use time::OffsetDateTime;
use tower::ServiceExt;

#[tokio::test]
async fn test_list_tools_empty() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    let state = ApiState::new(registry);
    let app = routes(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tools")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_publish_and_get_tool() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    let state = ApiState::new(registry);
    let app = routes(state);

    // Create test metadata
    let metadata = ToolMetadata {
        name: "test-api-tool".to_string(),
        version: "1.0.0".to_string(),
        description: "Test tool for API".to_string(),
        readme: Some("# Test Tool\n\nA test tool.".to_string()),
        author: "Test Author".to_string(),
        author_email: Some("test@example.com".to_string()),
        license: "MIT".to_string(),
        repository: Some("https://github.com/test/tool".to_string()),
        homepage: None,
        keywords: vec!["test".to_string(), "api".to_string()],
        categories: vec!["testing".to_string()],
        tool_type: ToolType::Pure,
        wasm_hash: "abc123def456".to_string(),
        wasm_size: 1024,
        signature: "sig123".to_string(),
        public_key: "key123".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    let publish_request = PublishRequest {
        metadata: metadata.clone(),
        wasm_bytes: base64::encode(b"fake wasm bytes"),
    };

    // Publish tool
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tools")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&publish_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Get tool
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tools/test-api-tool/1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_nonexistent_tool() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    let state = ApiState::new(registry);
    let app = routes(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tools/nonexistent/1.0.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_duplicate_publish() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    let state = ApiState::new(registry);
    let app = routes(state);

    let metadata = ToolMetadata {
        name: "duplicate-tool".to_string(),
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
        wasm_hash: "abc".to_string(),
        wasm_size: 100,
        signature: "sig".to_string(),
        public_key: "key".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    let publish_request = PublishRequest {
        metadata,
        wasm_bytes: base64::encode(b"test"),
    };

    // First publish should succeed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tools")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&publish_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Second publish should fail
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tools")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&publish_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}
