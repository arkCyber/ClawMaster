//! API integration tests for Skills endpoints.

use {
    axum::http::StatusCode,
    axum_test::TestServer,
    clawmaster_clawhub::{
        api::{ApiState, routes},
        registry::Registry,
        types::{PublishSkillRequest, SecurityStatus, SkillFormat, SkillMetadata},
    },
    tempfile::tempdir,
    time::OffsetDateTime,
};

async fn setup_test_server() -> TestServer {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let registry = Registry::new(&db_path).await.unwrap();
    let state = ApiState::new(registry);
    let app = routes(state);
    TestServer::new(app).unwrap()
}

fn create_test_skill_metadata(name: &str, version: &str) -> SkillMetadata {
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

#[tokio::test]
async fn test_publish_and_get_skill() {
    let server = setup_test_server().await;

    let metadata = create_test_skill_metadata("test-skill", "1.0.0");
    let request = PublishSkillRequest { metadata };

    // Publish skill
    let response = server.post("/skills").json(&request).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["name"], "test-skill");
    assert_eq!(body["version"], "1.0.0");

    // Get skill
    let response = server.get("/skills/test-skill/1.0.0").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let skill: serde_json::Value = response.json();
    assert_eq!(skill["name"], "test-skill");
    assert_eq!(skill["version"], "1.0.0");
}

#[tokio::test]
async fn test_get_nonexistent_skill() {
    let server = setup_test_server().await;

    let response = server.get("/skills/nonexistent/1.0.0").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_skills() {
    let server = setup_test_server().await;

    // Publish some skills
    for i in 0..3 {
        let metadata = create_test_skill_metadata(&format!("skill-{}", i), "1.0.0");
        let request = PublishSkillRequest { metadata };
        server.post("/skills").json(&request).await;
    }

    // List skills
    let response = server.get("/skills").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let skills: Vec<serde_json::Value> = response.json();
    assert_eq!(skills.len(), 3);
}

#[tokio::test]
async fn test_search_skills() {
    let server = setup_test_server().await;

    // Publish a skill
    let mut metadata = create_test_skill_metadata("web-scraper", "1.0.0");
    metadata.description = "A web scraping tool".to_string();
    metadata.keywords = vec!["web".to_string(), "scraping".to_string()];
    let request = PublishSkillRequest { metadata };
    server.post("/skills").json(&request).await;

    // Search for it
    let response = server.get("/skills/search?query=web").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let skills: Vec<serde_json::Value> = response.json();
    assert!(!skills.is_empty());
    assert_eq!(skills[0]["name"], "web-scraper");
}

#[tokio::test]
async fn test_search_skills_with_category() {
    let server = setup_test_server().await;

    // Publish skills with different categories
    let mut web_skill = create_test_skill_metadata("web-tool", "1.0.0");
    web_skill.categories = vec!["web".to_string()];
    server
        .post("/skills")
        .json(&PublishSkillRequest {
            metadata: web_skill,
        })
        .await;

    let mut data_skill = create_test_skill_metadata("data-tool", "1.0.0");
    data_skill.categories = vec!["data".to_string()];
    server
        .post("/skills")
        .json(&PublishSkillRequest {
            metadata: data_skill,
        })
        .await;

    // Search by category
    let response = server.get("/skills/search?category=web").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let skills: Vec<serde_json::Value> = response.json();
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0]["name"], "web-tool");
}

#[tokio::test]
async fn test_get_skill_install_info() {
    let server = setup_test_server().await;

    // Publish a skill
    let metadata = create_test_skill_metadata("installable", "1.0.0");
    let request = PublishSkillRequest { metadata };
    server.post("/skills").json(&request).await;

    // Get install info
    let response = server.get("/skills/installable/1.0.0/install").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let info: serde_json::Value = response.json();
    assert_eq!(info["name"], "installable");
    assert_eq!(info["version"], "1.0.0");
    assert!(
        info["install_command"]
            .as_str()
            .unwrap()
            .contains("clawmaster skills install")
    );
}

#[tokio::test]
async fn test_duplicate_publish_rejected() {
    let server = setup_test_server().await;

    let metadata = create_test_skill_metadata("duplicate", "1.0.0");
    let request = PublishSkillRequest {
        metadata: metadata.clone(),
    };

    // First publish succeeds
    let response = server.post("/skills").json(&request).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    // Second publish fails
    let request2 = PublishSkillRequest { metadata };
    let response = server.post("/skills").json(&request2).await;
    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_search_pagination() {
    let server = setup_test_server().await;

    // Publish 25 skills
    for i in 0..25 {
        let metadata = create_test_skill_metadata(&format!("skill-{:02}", i), "1.0.0");
        let request = PublishSkillRequest { metadata };
        server.post("/skills").json(&request).await;
    }

    // Get page 1
    let response = server.get("/skills?page=0&page_size=10").await;
    let page1: Vec<serde_json::Value> = response.json();
    assert_eq!(page1.len(), 10);

    // Get page 2
    let response = server.get("/skills?page=1&page_size=10").await;
    let page2: Vec<serde_json::Value> = response.json();
    assert_eq!(page2.len(), 10);

    // Pages should be different
    assert_ne!(page1[0]["name"], page2[0]["name"]);
}

#[tokio::test]
async fn test_empty_search_results() {
    let server = setup_test_server().await;

    let response = server.get("/skills/search?query=nonexistent").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let skills: Vec<serde_json::Value> = response.json();
    assert_eq!(skills.len(), 0);
}
