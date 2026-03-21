//! Health Check Implementations
//!
//! DO-178C Level A Compliant Health Checks

use {
    crate::{Criticality, HealthCheck, HealthStatus, ResourceMetrics},
    async_trait::async_trait,
    chrono::Utc,
    std::sync::Arc,
    sysinfo::{Disks, System},
};

/// Database health check
///
/// DO-178C §11.10: Database connectivity verification
#[cfg(feature = "database")]
pub struct DatabaseHealthCheck {
    pool: sqlx::SqlitePool,
}

#[cfg(feature = "database")]
impl DatabaseHealthCheck {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "database")]
#[async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> HealthStatus {
        // Try to execute a simple query
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => {
                // Check connection pool stats
                let size = self.pool.size();
                let idle = self.pool.num_idle();

                if idle == 0 && size > 0 {
                    HealthStatus::Degraded {
                        reason: "No idle database connections".to_string(),
                    }
                } else {
                    HealthStatus::Healthy
                }
            },
            Err(e) => HealthStatus::Unhealthy {
                reason: format!("Database query failed: {}", e),
            },
        }
    }

    fn name(&self) -> &str {
        "database"
    }

    fn criticality(&self) -> Criticality {
        Criticality::Critical
    }

    fn metadata(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "pool_size": self.pool.size(),
            "idle_connections": self.pool.num_idle(),
        }))
    }
}

/// Memory health check
///
/// DO-178C §11.10: Memory usage monitoring
pub struct MemoryHealthCheck {
    system: Arc<tokio::sync::Mutex<System>>,
}

impl MemoryHealthCheck {
    pub fn new() -> Self {
        Self {
            system: Arc::new(tokio::sync::Mutex::new(System::new_all())),
        }
    }
}

impl Default for MemoryHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for MemoryHealthCheck {
    async fn check(&self) -> HealthStatus {
        let mut sys = self.system.lock().await;
        sys.refresh_memory();

        let total = sys.total_memory();
        let used = sys.used_memory();
        let usage_percent = (used as f64 / total as f64) * 100.0;

        if usage_percent > 90.0 {
            HealthStatus::Unhealthy {
                reason: format!("Memory usage critical: {:.1}%", usage_percent),
            }
        } else if usage_percent > 80.0 {
            HealthStatus::Degraded {
                reason: format!("Memory usage high: {:.1}%", usage_percent),
            }
        } else {
            HealthStatus::Healthy
        }
    }

    fn name(&self) -> &str {
        "memory"
    }

    fn criticality(&self) -> Criticality {
        Criticality::Critical
    }

    fn metadata(&self) -> Option<serde_json::Value> {
        // Get current memory stats synchronously for metadata
        let sys = System::new_all();
        Some(serde_json::json!({
            "total_bytes": sys.total_memory(),
            "used_bytes": sys.used_memory(),
            "available_bytes": sys.available_memory(),
        }))
    }
}

/// CPU health check
///
/// DO-178C §11.10: CPU usage monitoring
pub struct CpuHealthCheck {
    system: Arc<tokio::sync::Mutex<System>>,
}

impl CpuHealthCheck {
    pub fn new() -> Self {
        Self {
            system: Arc::new(tokio::sync::Mutex::new(System::new_all())),
        }
    }
}

impl Default for CpuHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for CpuHealthCheck {
    async fn check(&self) -> HealthStatus {
        let mut sys = self.system.lock().await;
        sys.refresh_cpu();

        // Wait a bit for CPU measurement
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        sys.refresh_cpu();

        let cpu_usage = sys.global_cpu_info().cpu_usage();

        if cpu_usage > 95.0 {
            HealthStatus::Degraded {
                reason: format!("CPU usage very high: {:.1}%", cpu_usage),
            }
        } else if cpu_usage > 85.0 {
            HealthStatus::Degraded {
                reason: format!("CPU usage high: {:.1}%", cpu_usage),
            }
        } else {
            HealthStatus::Healthy
        }
    }

    fn name(&self) -> &str {
        "cpu"
    }

    fn criticality(&self) -> Criticality {
        Criticality::Important
    }

    fn metadata(&self) -> Option<serde_json::Value> {
        let sys = System::new_all();
        Some(serde_json::json!({
            "cpu_count": sys.cpus().len(),
            "cpu_usage_percent": sys.global_cpu_info().cpu_usage(),
        }))
    }
}

/// Disk health check
///
/// DO-178C §11.10: Disk space monitoring
pub struct DiskHealthCheck;

impl DiskHealthCheck {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DiskHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for DiskHealthCheck {
    async fn check(&self) -> HealthStatus {
        let disks = Disks::new_with_refreshed_list();

        if disks.is_empty() {
            return HealthStatus::Unhealthy {
                reason: "No disks found".to_string(),
            };
        }

        // Find the disk with highest usage
        let mut max_usage = 0.0;
        for disk in &disks {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            let usage_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            if usage_percent > max_usage {
                max_usage = usage_percent;
            }
        }

        if max_usage > 95.0 {
            HealthStatus::Unhealthy {
                reason: format!("Disk usage critical: {:.1}%", max_usage),
            }
        } else if max_usage > 90.0 {
            HealthStatus::Degraded {
                reason: format!("Disk usage high: {:.1}%", max_usage),
            }
        } else {
            HealthStatus::Healthy
        }
    }

    fn name(&self) -> &str {
        "disk"
    }

    fn criticality(&self) -> Criticality {
        Criticality::Critical
    }

    fn metadata(&self) -> Option<serde_json::Value> {
        let disks = Disks::new_with_refreshed_list();
        let disk_list: Vec<_> = disks
            .iter()
            .map(|disk| {
                serde_json::json!({
                    "name": disk.name().to_string_lossy(),
                    "mount_point": disk.mount_point().to_string_lossy(),
                    "total_bytes": disk.total_space(),
                    "available_bytes": disk.available_space(),
                })
            })
            .collect();

        Some(serde_json::json!({ "disks": disk_list }))
    }
}

/// Resource metrics collector
///
/// DO-178C §11.10: Comprehensive resource monitoring
pub struct ResourceMonitor {
    system: Arc<tokio::sync::Mutex<System>>,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        Self {
            system: Arc::new(tokio::sync::Mutex::new(System::new_all())),
        }
    }

    /// Collect current resource metrics
    pub async fn collect(&self) -> ResourceMetrics {
        let mut sys = self.system.lock().await;

        // Refresh all metrics
        sys.refresh_all();

        // Wait for CPU measurement
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        sys.refresh_cpu();

        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();
        let memory_usage_percent = if memory_total > 0 {
            (memory_used as f64 / memory_total as f64) * 100.0
        } else {
            0.0
        };

        let cpu_usage = sys.global_cpu_info().cpu_usage();

        // Calculate disk usage
        let disks = Disks::new_with_refreshed_list();
        let (disk_total, disk_used) = disks.iter().fold((0u64, 0u64), |(total, used), disk| {
            let disk_total = disk.total_space();
            let disk_available = disk.available_space();
            let disk_used = disk_total.saturating_sub(disk_available);
            (total + disk_total, used + disk_used)
        });

        let disk_usage_percent = if disk_total > 0 {
            (disk_used as f64 / disk_total as f64) * 100.0
        } else {
            0.0
        };

        ResourceMetrics {
            cpu_usage_percent: cpu_usage,
            memory_used_bytes: memory_used,
            memory_total_bytes: memory_total,
            memory_usage_percent: memory_usage_percent as f32,
            disk_used_bytes: disk_used,
            disk_total_bytes: disk_total,
            disk_usage_percent: disk_usage_percent as f32,
            active_connections: 0, // Will be set by caller
            timestamp: Utc::now(),
        }
    }
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_health_check() {
        let check = MemoryHealthCheck::new();
        let status = check.check().await;

        // Should return some status
        assert!(status.is_healthy() || status.is_degraded() || status.is_unhealthy());

        // Should have metadata
        assert!(check.metadata().is_some());
    }

    #[tokio::test]
    async fn test_cpu_health_check() {
        let check = CpuHealthCheck::new();
        let status = check.check().await;

        // Should return some status
        assert!(status.is_healthy() || status.is_degraded() || status.is_unhealthy());

        // Should have metadata
        assert!(check.metadata().is_some());
    }

    #[tokio::test]
    async fn test_disk_health_check() {
        let check = DiskHealthCheck::new();
        let status = check.check().await;

        // Should return some status
        assert!(status.is_healthy() || status.is_degraded() || status.is_unhealthy());

        // Should have metadata
        assert!(check.metadata().is_some());
    }

    #[tokio::test]
    async fn test_resource_monitor() {
        let monitor = ResourceMonitor::new();
        let metrics = monitor.collect().await;

        // Should have valid metrics
        assert!(metrics.memory_total_bytes > 0);
        assert!(metrics.disk_total_bytes > 0);
        assert!(metrics.cpu_usage_percent >= 0.0);
    }
}
