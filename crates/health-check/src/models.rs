//! Health Check Models
//!
//! DO-178C Level A Compliant Data Models

use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::cmp::Ordering,
};

/// Health status of a component
///
/// DO-178C §11.10: Clear status indication for monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum HealthStatus {
    /// Component is fully operational
    Healthy,

    /// Component is operational but with reduced functionality
    Degraded { reason: String },

    /// Component is not operational
    Unhealthy { reason: String },
}

impl HealthStatus {
    /// Check if status is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Check if status is degraded
    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded { .. })
    }

    /// Check if status is unhealthy
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy { .. })
    }

    /// Get the reason if status is not healthy
    pub fn reason(&self) -> Option<&str> {
        match self {
            HealthStatus::Healthy => None,
            HealthStatus::Degraded { reason } => Some(reason),
            HealthStatus::Unhealthy { reason } => Some(reason),
        }
    }

    /// Convert to HTTP status code
    pub fn to_http_status(&self) -> u16 {
        match self {
            HealthStatus::Healthy => 200,
            HealthStatus::Degraded { .. } => 200, // Still accepting traffic
            HealthStatus::Unhealthy { .. } => 503,
        }
    }
}

impl PartialOrd for HealthStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HealthStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HealthStatus::Healthy, HealthStatus::Healthy) => Ordering::Equal,
            (HealthStatus::Healthy, _) => Ordering::Greater,
            (_, HealthStatus::Healthy) => Ordering::Less,
            (HealthStatus::Degraded { .. }, HealthStatus::Degraded { .. }) => Ordering::Equal,
            (HealthStatus::Degraded { .. }, HealthStatus::Unhealthy { .. }) => Ordering::Greater,
            (HealthStatus::Unhealthy { .. }, HealthStatus::Degraded { .. }) => Ordering::Less,
            (HealthStatus::Unhealthy { .. }, HealthStatus::Unhealthy { .. }) => Ordering::Equal,
        }
    }
}

/// Criticality level of a component
///
/// DO-178C §6.3.3: Component criticality classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Criticality {
    /// Component failure causes system failure
    Critical = 3,

    /// Component failure causes degraded functionality
    Important = 2,

    /// Component failure has minimal impact
    Optional = 1,
}

impl Criticality {
    /// Check if this is a critical component
    pub fn is_critical(&self) -> bool {
        matches!(self, Criticality::Critical)
    }
}

/// Result of a single health check
///
/// DO-178C §11.10: Detailed health check results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Name of the component
    pub name: String,

    /// Health status
    pub status: HealthStatus,

    /// Criticality level
    pub criticality: Criticality,

    /// Timestamp of the check
    pub timestamp: DateTime<Utc>,

    /// Duration of the check in milliseconds
    pub duration_ms: u64,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl HealthCheckResult {
    /// Create a new health check result
    pub fn new(
        name: String,
        status: HealthStatus,
        criticality: Criticality,
        duration_ms: u64,
    ) -> Self {
        Self {
            name,
            status,
            criticality,
            timestamp: Utc::now(),
            duration_ms,
            metadata: None,
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Overall system health
///
/// DO-178C §11.10: Aggregated system health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system status
    pub status: HealthStatus,

    /// Individual component results
    pub checks: Vec<HealthCheckResult>,

    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,

    /// Total duration in milliseconds
    pub total_duration_ms: u64,
}

impl SystemHealth {
    /// Create a new system health from check results
    pub fn from_checks(checks: Vec<HealthCheckResult>, total_duration_ms: u64) -> Self {
        let status = Self::aggregate_status(&checks);

        Self {
            status,
            checks,
            timestamp: Utc::now(),
            total_duration_ms,
        }
    }

    /// Aggregate status from individual checks
    ///
    /// DO-178C §11.10: Critical components determine overall status
    fn aggregate_status(checks: &[HealthCheckResult]) -> HealthStatus {
        // If any critical component is unhealthy, system is unhealthy
        if checks
            .iter()
            .any(|c| c.criticality.is_critical() && c.status.is_unhealthy())
        {
            return HealthStatus::Unhealthy {
                reason: "Critical component failure".to_string(),
            };
        }

        // If any critical component is degraded, system is degraded
        if checks
            .iter()
            .any(|c| c.criticality.is_critical() && c.status.is_degraded())
        {
            return HealthStatus::Degraded {
                reason: "Critical component degraded".to_string(),
            };
        }

        // If any important component is unhealthy, system is degraded
        if checks
            .iter()
            .any(|c| c.criticality == Criticality::Important && c.status.is_unhealthy())
        {
            return HealthStatus::Degraded {
                reason: "Important component failure".to_string(),
            };
        }

        // Otherwise, system is healthy
        HealthStatus::Healthy
    }

    /// Check if system is ready to accept traffic
    ///
    /// DO-178C §11.10: Readiness verification
    pub fn is_ready(&self) -> bool {
        // System is ready if all critical components are healthy
        self.checks
            .iter()
            .all(|c| !c.criticality.is_critical() || c.status.is_healthy())
    }

    /// Get HTTP status code for this health
    pub fn to_http_status(&self) -> u16 {
        self.status.to_http_status()
    }
}

/// Resource usage metrics
///
/// DO-178C §11.10: Resource monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_usage_percent: f32,

    /// Memory usage in bytes
    pub memory_used_bytes: u64,

    /// Total memory in bytes
    pub memory_total_bytes: u64,

    /// Memory usage percentage (0-100)
    pub memory_usage_percent: f32,

    /// Disk usage in bytes
    pub disk_used_bytes: u64,

    /// Total disk space in bytes
    pub disk_total_bytes: u64,

    /// Disk usage percentage (0-100)
    pub disk_usage_percent: f32,

    /// Number of active connections
    pub active_connections: usize,

    /// Timestamp of metrics collection
    pub timestamp: DateTime<Utc>,
}

impl ResourceMetrics {
    /// Check if resources are within acceptable limits
    ///
    /// DO-178C §11.10: Resource limit enforcement
    pub fn is_healthy(&self) -> HealthStatus {
        // Memory threshold: 90%
        if self.memory_usage_percent > 90.0 {
            return HealthStatus::Unhealthy {
                reason: format!("Memory usage critical: {:.1}%", self.memory_usage_percent),
            };
        }

        // Disk threshold: 95%
        if self.disk_usage_percent > 95.0 {
            return HealthStatus::Unhealthy {
                reason: format!("Disk usage critical: {:.1}%", self.disk_usage_percent),
            };
        }

        // CPU threshold: 95%
        if self.cpu_usage_percent > 95.0 {
            return HealthStatus::Degraded {
                reason: format!("CPU usage high: {:.1}%", self.cpu_usage_percent),
            };
        }

        // Memory warning: 80%
        if self.memory_usage_percent > 80.0 {
            return HealthStatus::Degraded {
                reason: format!("Memory usage high: {:.1}%", self.memory_usage_percent),
            };
        }

        // Disk warning: 90%
        if self.disk_usage_percent > 90.0 {
            return HealthStatus::Degraded {
                reason: format!("Disk usage high: {:.1}%", self.disk_usage_percent),
            };
        }

        HealthStatus::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_is_methods() {
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(!HealthStatus::Healthy.is_degraded());
        assert!(!HealthStatus::Healthy.is_unhealthy());

        let degraded = HealthStatus::Degraded {
            reason: "test".to_string(),
        };
        assert!(!degraded.is_healthy());
        assert!(degraded.is_degraded());
        assert!(!degraded.is_unhealthy());

        let unhealthy = HealthStatus::Unhealthy {
            reason: "test".to_string(),
        };
        assert!(!unhealthy.is_healthy());
        assert!(!unhealthy.is_degraded());
        assert!(unhealthy.is_unhealthy());
    }

    #[test]
    fn test_health_status_http_codes() {
        assert_eq!(HealthStatus::Healthy.to_http_status(), 200);
        assert_eq!(
            HealthStatus::Degraded {
                reason: "test".to_string()
            }
            .to_http_status(),
            200
        );
        assert_eq!(
            HealthStatus::Unhealthy {
                reason: "test".to_string()
            }
            .to_http_status(),
            503
        );
    }

    #[test]
    fn test_system_health_aggregation() {
        let checks = vec![
            HealthCheckResult::new(
                "db".to_string(),
                HealthStatus::Healthy,
                Criticality::Critical,
                10,
            ),
            HealthCheckResult::new(
                "cache".to_string(),
                HealthStatus::Healthy,
                Criticality::Important,
                5,
            ),
        ];

        let health = SystemHealth::from_checks(checks, 15);
        assert!(health.status.is_healthy());
        assert!(health.is_ready());
    }

    #[test]
    fn test_system_health_critical_failure() {
        let checks = vec![HealthCheckResult::new(
            "db".to_string(),
            HealthStatus::Unhealthy {
                reason: "connection failed".to_string(),
            },
            Criticality::Critical,
            10,
        )];

        let health = SystemHealth::from_checks(checks, 10);
        assert!(health.status.is_unhealthy());
        assert!(!health.is_ready());
    }

    #[test]
    fn test_resource_metrics_healthy() {
        let metrics = ResourceMetrics {
            cpu_usage_percent: 50.0,
            memory_used_bytes: 1_000_000_000,
            memory_total_bytes: 4_000_000_000,
            memory_usage_percent: 25.0,
            disk_used_bytes: 10_000_000_000,
            disk_total_bytes: 100_000_000_000,
            disk_usage_percent: 10.0,
            active_connections: 10,
            timestamp: Utc::now(),
        };

        assert!(metrics.is_healthy().is_healthy());
    }

    #[test]
    fn test_resource_metrics_memory_critical() {
        let metrics = ResourceMetrics {
            cpu_usage_percent: 50.0,
            memory_used_bytes: 3_600_000_000,
            memory_total_bytes: 4_000_000_000,
            memory_usage_percent: 91.0,
            disk_used_bytes: 10_000_000_000,
            disk_total_bytes: 100_000_000_000,
            disk_usage_percent: 10.0,
            active_connections: 10,
            timestamp: Utc::now(),
        };

        assert!(metrics.is_healthy().is_unhealthy());
    }
}
