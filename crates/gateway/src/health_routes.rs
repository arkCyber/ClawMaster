//! Health Check Routes
//!
//! DO-178C Level A Compliant Health Check Endpoints

use {
    axum::{
        Router,
        extract::State,
        http::StatusCode,
        response::{IntoResponse, Json, Response},
        routing::get,
    },
    clawmaster_health_check::{HealthCheckService, HealthStatus},
    serde_json::json,
    std::sync::Arc,
};

/// Health check state
#[derive(Clone)]
pub struct HealthState {
    pub service: Arc<HealthCheckService>,
}

/// Create health check routes
///
/// DO-178C §11.10: Runtime monitoring endpoints
pub fn health_routes(service: Arc<HealthCheckService>) -> Router {
    let state = HealthState { service };

    Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .with_state(state)
}

/// Health check endpoint
///
/// Returns detailed health status of all components
///
/// DO-178C §11.10: Comprehensive health reporting
async fn health_handler(State(state): State<HealthState>) -> Response {
    let health = state.service.check_health().await;

    let status_code = match &health.status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded { .. } => StatusCode::OK, // Still serving requests
        HealthStatus::Unhealthy { .. } => StatusCode::SERVICE_UNAVAILABLE,
    };

    let response = json!({
        "status": format!("{:?}", health.status),
        "timestamp": health.timestamp.to_rfc3339(),
        "duration_ms": health.total_duration_ms,
        "checks": health.checks.iter().map(|check| {
            json!({
                "name": check.name,
                "status": format!("{:?}", check.status),
                "criticality": format!("{:?}", check.criticality),
                "duration_ms": check.duration_ms,
                "metadata": check.metadata,
            })
        }).collect::<Vec<_>>(),
    });

    (status_code, Json(response)).into_response()
}

/// Readiness check endpoint
///
/// Returns simple ready/not-ready status for load balancers
///
/// DO-178C §11.10: Simplified readiness check
async fn ready_handler(State(state): State<HealthState>) -> Response {
    let is_ready = state.service.is_ready().await;

    if is_ready {
        (
            StatusCode::OK,
            Json(json!({
                "ready": true,
                "status": "ok"
            })),
        )
            .into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "ready": false,
                "status": "unavailable"
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        axum::{body::Body, http::Request},
        clawmaster_health_check::MemoryHealthCheck,
        tower::ServiceExt,
    };

    #[tokio::test]
    async fn test_health_endpoint() {
        let mut service = HealthCheckService::new();
        service.register(Arc::new(MemoryHealthCheck::new()));

        let app = health_routes(Arc::new(service));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ready_endpoint() {
        let mut service = HealthCheckService::new();
        service.register(Arc::new(MemoryHealthCheck::new()));

        let app = health_routes(Arc::new(service));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ready")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
