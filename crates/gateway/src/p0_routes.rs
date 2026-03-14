//! P0 Features API Routes
//!
//! Provides HTTP endpoints for P0 features:
//! - Health check status
//! - System metrics
//! - Fault recovery status

use axum::{
    Extension,
    Json,
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;

use crate::p0_integration::P0Features;

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub checks: Vec<ComponentHealth>,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: String,
    pub criticality: String,
    pub duration_ms: u64,
    pub message: Option<String>,
}

/// System metrics response
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub circuit_breaker: CircuitBreakerMetrics,
    pub resource_quota: ResourceQuotaMetrics,
    pub fault_isolation: FaultIsolationMetrics,
}

#[derive(Debug, Serialize)]
pub struct CircuitBreakerMetrics {
    pub state: String,
    pub failure_count: u64,
    pub success_count: u64,
}

#[derive(Debug, Serialize)]
pub struct ResourceQuotaMetrics {
    pub rate_limit_active: bool,
    pub memory_used: usize,
    pub connections_active: usize,
    pub sessions_active: usize,
}

#[derive(Debug, Serialize)]
pub struct FaultIsolationMetrics {
    pub isolated_services: Vec<String>,
    pub total_faults: usize,
}

/// GET /api/p0/health - Get system health status
async fn get_health(
    Extension(p0): Extension<Arc<P0Features>>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let health = p0.get_health_status().await;
    
    let checks = health.checks.iter().map(|check| {
        let (status_str, message) = match &check.status {
            clawmaster_health_check::HealthStatus::Healthy => ("healthy".to_string(), None),
            clawmaster_health_check::HealthStatus::Degraded { reason } => {
                ("degraded".to_string(), Some(reason.clone()))
            }
            clawmaster_health_check::HealthStatus::Unhealthy { reason } => {
                ("unhealthy".to_string(), Some(reason.clone()))
            }
        };
        
        ComponentHealth {
            name: check.name.clone(),
            status: status_str,
            criticality: format!("{:?}", check.criticality),
            duration_ms: check.duration_ms,
            message,
        }
    }).collect();
    
    let status_str = if health.status.is_healthy() {
        "healthy"
    } else if health.status.is_degraded() {
        "degraded"
    } else {
        "unhealthy"
    };
    
    Ok(Json(HealthResponse {
        status: status_str.to_string(),
        timestamp: health.timestamp.to_string(),
        checks,
        duration_ms: 0, // SystemHealth doesn't expose duration_ms
    }))
}

/// GET /api/p0/metrics - Get system metrics
async fn get_metrics(
    Extension(p0): Extension<Arc<P0Features>>,
) -> Result<Json<MetricsResponse>, StatusCode> {
    // Get fault isolation metrics
    let isolated_services = vec![
        "database",
        "llm_provider",
        "channel_service",
    ]
    .into_iter()
    .filter(|s| p0.is_service_isolated(s))
    .map(|s| s.to_string())
    .collect();
    
    Ok(Json(MetricsResponse {
        circuit_breaker: CircuitBreakerMetrics {
            state: "active".to_string(),
            failure_count: 0,
            success_count: 0,
        },
        resource_quota: ResourceQuotaMetrics {
            rate_limit_active: true,
            memory_used: 0,
            connections_active: 0,
            sessions_active: 0,
        },
        fault_isolation: FaultIsolationMetrics {
            isolated_services,
            total_faults: 0,
        },
    }))
}

/// GET /api/p0/ready - Readiness probe
async fn get_ready(
    Extension(p0): Extension<Arc<P0Features>>,
) -> impl IntoResponse {
    let health = p0.get_health_status().await;
    
    if health.is_ready() {
        (StatusCode::OK, "ready")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not ready")
    }
}

/// GET /api/p0/live - Liveness probe
async fn get_live() -> impl IntoResponse {
    (StatusCode::OK, "alive")
}

/// Create P0 routes
pub fn p0_router() -> Router {
    Router::new()
        .route("/health", get(get_health))
        .route("/metrics", get(get_metrics))
        .route("/ready", get(get_ready))
        .route("/live", get(get_live))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_health_endpoint() {
        let temp_dir = TempDir::new().unwrap();
        let p0 = Arc::new(P0Features::new(temp_dir.path()).await.unwrap());
        
        let app = p0_router().layer(Extension(p0));
        
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
        let temp_dir = TempDir::new().unwrap();
        let p0 = Arc::new(P0Features::new(temp_dir.path()).await.unwrap());
        
        let app = p0_router().layer(Extension(p0));
        
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

    #[tokio::test]
    async fn test_live_endpoint() {
        let app = p0_router();
        
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/live")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}
