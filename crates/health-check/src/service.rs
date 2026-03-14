//! Health Check Service
//!
//! DO-178C Level A Compliant Health Check Service

use crate::{Criticality, HealthCheck, HealthCheckResult, HealthStatus, SystemHealth};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

/// Health check service
///
/// DO-178C §11.10: Centralized health monitoring
pub struct HealthCheckService {
    checks: Vec<Arc<dyn HealthCheck>>,
    last_result: Arc<RwLock<Option<SystemHealth>>>,
}

impl HealthCheckService {
    /// Create a new health check service
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            last_result: Arc::new(RwLock::new(None)),
        }
    }

    /// Register a health check
    ///
    /// DO-178C §11.13: Component registration
    pub fn register(&mut self, check: Arc<dyn HealthCheck>) {
        debug!("Registering health check: {}", check.name());
        self.checks.push(check);
    }

    /// Run all health checks
    ///
    /// DO-178C §11.10: Comprehensive health verification
    pub async fn check_health(&self) -> SystemHealth {
        let start = Instant::now();
        let mut results = Vec::new();

        debug!("Running {} health checks", self.checks.len());

        for check in &self.checks {
            let check_start = Instant::now();
            
            match tokio::time::timeout(
                tokio::time::Duration::from_secs(5),
                check.check()
            ).await {
                Ok(status) => {
                    let duration_ms = check_start.elapsed().as_millis() as u64;
                    
                    let mut result = HealthCheckResult::new(
                        check.name().to_string(),
                        status.clone(),
                        check.criticality(),
                        duration_ms,
                    );

                    if let Some(metadata) = check.metadata() {
                        result = result.with_metadata(metadata);
                    }

                    // Log unhealthy or degraded status
                    match &status {
                        HealthStatus::Unhealthy { reason } => {
                            error!(
                                check = check.name(),
                                criticality = ?check.criticality(),
                                reason = reason,
                                "Health check failed"
                            );
                        }
                        HealthStatus::Degraded { reason } => {
                            warn!(
                                check = check.name(),
                                criticality = ?check.criticality(),
                                reason = reason,
                                "Health check degraded"
                            );
                        }
                        HealthStatus::Healthy => {
                            debug!(
                                check = check.name(),
                                duration_ms = duration_ms,
                                "Health check passed"
                            );
                        }
                    }

                    results.push(result);
                }
                Err(_) => {
                    // Timeout
                    error!(
                        check = check.name(),
                        "Health check timed out after 5 seconds"
                    );
                    
                    results.push(HealthCheckResult::new(
                        check.name().to_string(),
                        HealthStatus::Unhealthy {
                            reason: "Health check timed out".to_string(),
                        },
                        check.criticality(),
                        5000,
                    ));
                }
            }
        }

        let total_duration_ms = start.elapsed().as_millis() as u64;
        let health = SystemHealth::from_checks(results, total_duration_ms);

        // Update last result
        *self.last_result.write().await = Some(health.clone());

        debug!(
            status = ?health.status,
            duration_ms = total_duration_ms,
            "Health check completed"
        );

        health
    }

    /// Get the last health check result
    pub async fn last_health(&self) -> Option<SystemHealth> {
        self.last_result.read().await.clone()
    }

    /// Check if system is ready
    ///
    /// DO-178C §11.10: Readiness verification
    pub async fn is_ready(&self) -> bool {
        let health = self.check_health().await;
        health.is_ready()
    }

    /// Get number of registered checks
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Get critical checks
    pub fn critical_checks(&self) -> Vec<&str> {
        self.checks
            .iter()
            .filter(|c| c.criticality() == Criticality::Critical)
            .map(|c| c.name())
            .collect()
    }
}

impl Default for HealthCheckService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockHealthCheck {
        name: String,
        status: HealthStatus,
        criticality: Criticality,
    }

    impl MockHealthCheck {
        fn new(name: &str, status: HealthStatus, criticality: Criticality) -> Self {
            Self {
                name: name.to_string(),
                status,
                criticality,
            }
        }
    }

    #[async_trait]
    impl HealthCheck for MockHealthCheck {
        async fn check(&self) -> HealthStatus {
            self.status.clone()
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn criticality(&self) -> Criticality {
            self.criticality
        }
    }

    #[tokio::test]
    async fn test_health_check_service_all_healthy() {
        let mut service = HealthCheckService::new();
        
        service.register(Arc::new(MockHealthCheck::new(
            "db",
            HealthStatus::Healthy,
            Criticality::Critical,
        )));
        
        service.register(Arc::new(MockHealthCheck::new(
            "cache",
            HealthStatus::Healthy,
            Criticality::Important,
        )));

        let health = service.check_health().await;
        
        assert!(health.status.is_healthy());
        assert!(health.is_ready());
        assert_eq!(health.checks.len(), 2);
    }

    #[tokio::test]
    async fn test_health_check_service_critical_failure() {
        let mut service = HealthCheckService::new();
        
        service.register(Arc::new(MockHealthCheck::new(
            "db",
            HealthStatus::Unhealthy {
                reason: "connection failed".to_string(),
            },
            Criticality::Critical,
        )));

        let health = service.check_health().await;
        
        assert!(health.status.is_unhealthy());
        assert!(!health.is_ready());
    }

    #[tokio::test]
    async fn test_health_check_service_degraded() {
        let mut service = HealthCheckService::new();
        
        service.register(Arc::new(MockHealthCheck::new(
            "db",
            HealthStatus::Degraded {
                reason: "slow response".to_string(),
            },
            Criticality::Critical,
        )));

        let health = service.check_health().await;
        
        assert!(health.status.is_degraded());
        assert!(!health.is_ready());
    }

    #[tokio::test]
    async fn test_health_check_service_last_health() {
        let mut service = HealthCheckService::new();
        
        service.register(Arc::new(MockHealthCheck::new(
            "test",
            HealthStatus::Healthy,
            Criticality::Optional,
        )));

        // Initially no last health
        assert!(service.last_health().await.is_none());

        // After check, should have last health
        service.check_health().await;
        assert!(service.last_health().await.is_some());
    }

    #[tokio::test]
    async fn test_health_check_service_critical_checks() {
        let mut service = HealthCheckService::new();
        
        service.register(Arc::new(MockHealthCheck::new(
            "db",
            HealthStatus::Healthy,
            Criticality::Critical,
        )));
        
        service.register(Arc::new(MockHealthCheck::new(
            "cache",
            HealthStatus::Healthy,
            Criticality::Important,
        )));

        let critical = service.critical_checks();
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0], "db");
    }
}
