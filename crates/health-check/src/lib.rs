//! System Health Check and Monitoring
//!
//! DO-178C Level A Compliant Health Check System
//!
//! This module provides comprehensive system health monitoring including:
//! - Component health checks (database, WebSocket, etc.)
//! - Resource usage monitoring (CPU, memory, disk)
//! - System readiness verification
//! - Health status aggregation
//!
//! Compliance: DO-178C §11.10 - Runtime monitoring and diagnostics

pub mod checks;
pub mod models;
pub mod service;

pub use {checks::*, models::*, service::*};

use {anyhow::Result, async_trait::async_trait};

/// Health check trait for components
///
/// DO-178C §11.10: All critical components must implement health checks
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Perform the health check
    ///
    /// Returns HealthStatus indicating component health
    async fn check(&self) -> HealthStatus;

    /// Get the name of this health check
    fn name(&self) -> &str;

    /// Get the criticality level of this component
    fn criticality(&self) -> Criticality;

    /// Get optional metadata about this check
    fn metadata(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Run database migrations
///
/// DO-178C §11.13: Proper initialization and migration
#[cfg(feature = "database")]
pub async fn run_migrations(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_ordering() {
        // Unhealthy is worse than Degraded
        assert!(
            HealthStatus::Unhealthy {
                reason: "test".to_string()
            } < HealthStatus::Degraded {
                reason: "test".to_string()
            }
        );

        // Degraded is worse than Healthy
        assert!(
            HealthStatus::Degraded {
                reason: "test".to_string()
            } < HealthStatus::Healthy
        );
    }

    #[test]
    fn test_criticality_ordering() {
        assert!(Criticality::Critical > Criticality::Important);
        assert!(Criticality::Important > Criticality::Optional);
    }
}
