//! HTTP API routes for ClawHub.
//!
//! This module provides REST API endpoints for the ClawHub registry.

use {
    crate::{
        error::{Error, Result},
        registry::Registry,
        skills::SkillsRegistry,
        types::{
            PublishRequest, PublishResponse, PublishSkillRequest, PublishSkillResponse,
            SearchQuery, SkillMetadata, SkillSearchQuery, ToolMetadata,
        },
    },
    axum::{
        Json, Router,
        extract::{Path, Query, State},
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
    },
    std::sync::Arc,
    tracing::{debug, info},
};

/// API state shared across handlers.
#[derive(Clone)]
pub struct ApiState {
    registry: Arc<Registry>,
}

impl ApiState {
    /// Create new API state.
    pub fn new(registry: Registry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }
}

/// Create API router.
///
/// # Example
/// ```no_run
/// use clawmaster_clawhub::api::{routes, ApiState};
/// use clawmaster_clawhub::registry::Registry;
///
/// # async fn example() -> anyhow::Result<()> {
/// let registry = Registry::new("clawhub.db").await?;
/// let state = ApiState::new(registry);
/// let app = routes(state);
/// # Ok(())
/// # }
/// ```
pub fn routes(state: ApiState) -> Router {
    Router::new()
        // Tools routes
        .route("/tools", get(list_tools).post(publish_tool))
        .route("/tools/:name", get(get_tool_latest))
        .route("/tools/:name/:version", get(get_tool_version))
        .route("/tools/:name/:version/download", get(download_tool))
        .route("/search", get(search_tools))
        // Skills routes
        .route("/skills", get(list_skills).post(publish_skill))
        .route("/skills/search", get(search_skills))
        .route("/skills/:name/:version", get(get_skill_version))
        .route("/skills/:name/:version/install", get(get_skill_install_info))
        .with_state(state)
}

/// List all tools.
///
/// GET /tools?page=0&page_size=20
async fn list_tools(
    State(state): State<ApiState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<ToolMetadata>>> {
    debug!("Listing tools: {:?}", query);

    let (tools, _total) = state.registry.search(query).await?;

    Ok(Json(tools))
}

/// Search for tools.
///
/// GET /search?q=calculator&category=utilities
async fn search_tools(
    State(state): State<ApiState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<ToolMetadata>>> {
    debug!("Searching tools: {:?}", query);

    let (tools, _total) = state.registry.search(query).await?;

    Ok(Json(tools))
}

/// Get latest version of a tool.
///
/// GET /tools/:name
async fn get_tool_latest(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> Result<Json<ToolMetadata>> {
    debug!("Getting latest version of tool: {}", name);

    // TODO: Implement get_latest_version in registry
    // For now, return error
    Err(Error::ToolNotFound {
        name,
        version: "latest".to_string(),
    })
}

/// Get specific version of a tool.
///
/// GET /tools/:name/:version
async fn get_tool_version(
    State(state): State<ApiState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<ToolMetadata>> {
    debug!("Getting tool: {}@{}", name, version);

    let tool = state.registry.get_tool(&name, &version).await?;

    Ok(Json(tool))
}

/// Download a tool.
///
/// GET /tools/:name/:version/download
async fn download_tool(
    State(state): State<ApiState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    info!("Downloading tool: {}@{}", name, version);

    // Increment download count
    state.registry.increment_downloads(&name, &version).await?;

    // TODO: Implement actual file download
    // For now, return placeholder
    Ok((
        StatusCode::OK,
        [("Content-Type", "application/wasm")],
        "Wasm file placeholder",
    ))
}

/// Publish a tool.
///
/// POST /tools
async fn publish_tool(
    State(state): State<ApiState>,
    Json(request): Json<PublishRequest>,
) -> Result<Json<PublishResponse>> {
    info!(
        "Publishing tool: {}@{}",
        request.metadata.name, request.metadata.version
    );

    // Publish to registry
    state.registry.publish(request.metadata.clone()).await?;

    let response = PublishResponse {
        name: request.metadata.name.clone(),
        version: request.metadata.version.clone(),
        download_url: format!(
            "/tools/{}/{}/download",
            request.metadata.name, request.metadata.version
        ),
        message: "Tool published successfully".to_string(),
    };

    Ok(Json(response))
}

// ── Skills API Handlers ─────────────────────────────────────────────────────

/// List all skills.
///
/// GET /skills?page=0&page_size=20
async fn list_skills(
    State(state): State<ApiState>,
    Query(query): Query<SkillSearchQuery>,
) -> Result<Json<Vec<SkillMetadata>>> {
    debug!("Listing skills: {:?}", query);

    let skills_registry = SkillsRegistry::new(&state.registry.pool);
    let (skills, _total) = skills_registry.search(query).await?;

    Ok(Json(skills))
}

/// Search for skills.
///
/// GET /skills/search?q=web&category=utilities
async fn search_skills(
    State(state): State<ApiState>,
    Query(query): Query<SkillSearchQuery>,
) -> Result<Json<Vec<SkillMetadata>>> {
    debug!("Searching skills: {:?}", query);

    let skills_registry = SkillsRegistry::new(&state.registry.pool);
    let (skills, _total) = skills_registry.search(query).await?;

    Ok(Json(skills))
}

/// Get specific version of a skill.
///
/// GET /skills/:name/:version
async fn get_skill_version(
    State(state): State<ApiState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<SkillMetadata>> {
    debug!("Getting skill: {}@{}", name, version);

    let skills_registry = SkillsRegistry::new(&state.registry.pool);
    let skill = skills_registry.get_skill(&name, &version).await?;

    Ok(Json(skill))
}

/// Get skill install information.
///
/// GET /skills/:name/:version/install
async fn get_skill_install_info(
    State(state): State<ApiState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>> {
    info!("Getting install info for skill: {}@{}", name, version);

    let skills_registry = SkillsRegistry::new(&state.registry.pool);
    let skill = skills_registry.get_skill(&name, &version).await?;

    let install_command = if let Some(ref repo) = skill.github_repo {
        format!("clawmaster skills install {}", repo)
    } else {
        format!("# Skill {}@{} has no GitHub repository", name, version)
    };

    Ok(Json(serde_json::json!({
        "name": skill.name,
        "version": skill.version,
        "github_repo": skill.github_repo,
        "install_command": install_command,
        "format": skill.skill_format,
    })))
}

/// Publish a skill.
///
/// POST /skills
async fn publish_skill(
    State(state): State<ApiState>,
    Json(request): Json<PublishSkillRequest>,
) -> Result<Json<PublishSkillResponse>> {
    info!(
        "Publishing skill: {}@{}",
        request.metadata.name, request.metadata.version
    );

    // Publish to registry
    let skills_registry = SkillsRegistry::new(&state.registry.pool);
    skills_registry.publish(request.metadata.clone()).await?;

    let install_command = if let Some(ref repo) = request.metadata.github_repo {
        format!("clawmaster skills install {}", repo)
    } else {
        format!("claw skills install {}", request.metadata.name)
    };

    let response = PublishSkillResponse {
        name: request.metadata.name.clone(),
        version: request.metadata.version.clone(),
        install_command,
        message: "Skill published successfully".to_string(),
    };

    Ok(Json(response))
}

/// Convert our errors to HTTP responses.
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::ToolNotFound { name, version } => (
                StatusCode::NOT_FOUND,
                format!("Tool/Skill not found: {}@{}", name, version),
            ),
            Error::ToolAlreadyExists { name, version } => (
                StatusCode::CONFLICT,
                format!("Tool/Skill already exists: {}@{}", name, version),
            ),
            Error::InvalidMetadata(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::SecurityVerificationFailed(msg) => (StatusCode::FORBIDDEN, msg),
            Error::SignatureVerificationFailed => (
                StatusCode::FORBIDDEN,
                "Signature verification failed".to_string(),
            ),
            Error::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (status, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::types::{SecurityStatus, ToolType},
        axum::{
            body::Body,
            http::{Request, StatusCode},
        },
        tempfile::tempdir,
        time::OffsetDateTime,
        tower::ServiceExt,
    };

    #[tokio::test]
    async fn test_list_tools() {
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
    async fn test_get_tool_version() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let registry = Registry::new(&db_path).await.unwrap();

        // Publish a test tool
        let metadata = ToolMetadata {
            name: "test-tool".to_string(),
            version: "1.0.0".to_string(),
            description: "Test tool".to_string(),
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
        registry.publish(metadata).await.unwrap();

        let state = ApiState::new(registry);
        let app = routes(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/tools/test-tool/1.0.0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
