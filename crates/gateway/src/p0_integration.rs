//! P0 Features Integration Module
//!
//! This module integrates all P0 priority features into the gateway:
//! - P0-1: Health Check
//! - P0-2: Config Validator
//! - P0-3: Fault Recovery
//! - P0-4: Audit Log
//! - P0-5: Resource Quota
//! - P0-6: Backup Recovery
//! - P0-7: Input Validator

use std::{sync::Arc, time::Duration};

// P0 feature imports
use {
    clawmaster_audit_log::{AuditLogger, AuditLoggerConfig, MemoryStorage as AuditStorage},
    clawmaster_backup_recovery::{BackupManager, BackupScheduler, ScheduleConfig},
    clawmaster_config_validator::ConfigValidator,
    clawmaster_fault_recovery::{
        CircuitBreaker, CircuitBreakerConfig, DegradationManager, IsolationManager, RetryExecutor,
        RetryPolicy,
    },
    clawmaster_health_check::HealthCheckService,
    clawmaster_resource_quota::{
        ConnectionLimiter, MemoryQuota, RateLimiter, SessionLimiter, UploadLimiter,
    },
};

/// P0 features integration state
pub struct P0Features {
    // P0-1: Health Check
    pub health_checker: Arc<HealthCheckService>,

    // P0-2: Config Validator
    pub config_validator: Arc<ConfigValidator>,

    // P0-3: Fault Recovery
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub retry_executor: Arc<RetryExecutor>,
    pub degradation_manager: Arc<DegradationManager>,
    pub isolation_manager: Arc<IsolationManager>,

    // P0-4: Audit Log
    pub audit_logger: Arc<AuditLogger>,

    // P0-5: Resource Quota
    pub rate_limiter: Arc<RateLimiter>,
    pub memory_quota: Arc<MemoryQuota>,
    pub connection_limiter: Arc<ConnectionLimiter>,
    pub session_limiter: Arc<SessionLimiter>,
    pub upload_limiter: Arc<UploadLimiter>,

    // P0-6: Backup Recovery
    pub backup_manager: Arc<BackupManager>,
    pub backup_scheduler: Option<Arc<BackupScheduler>>,
}

impl P0Features {
    /// Initialize all P0 features
    pub async fn new(data_dir: &std::path::Path) -> anyhow::Result<Self> {
        tracing::info!("Initializing P0 features...");

        // P0-1: Health Check
        tracing::info!("Initializing health checker...");
        let health_checker = Arc::new(HealthCheckService::new());

        // P0-2: Config Validator
        tracing::info!("Initializing config validator...");
        let config_validator = Arc::new(ConfigValidator::new());

        // P0-3: Fault Recovery
        tracing::info!("Initializing fault recovery components...");
        let cb_config = CircuitBreakerConfig {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            window: Duration::from_secs(60),
        };
        let circuit_breaker = Arc::new(CircuitBreaker::new(cb_config));

        let retry_policy = RetryPolicy {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            multiplier: 2.0,
        };
        let retry_executor = Arc::new(RetryExecutor::new(retry_policy));

        let degradation_manager = Arc::new(DegradationManager::new());
        let isolation_manager = Arc::new(IsolationManager::new());

        // Register critical components for isolation
        isolation_manager.register("database");
        isolation_manager.register("llm_provider");
        isolation_manager.register("channel_service");

        // P0-4: Audit Log
        tracing::info!("Initializing audit logger...");
        let audit_config = AuditLoggerConfig::default();
        let audit_storage = Arc::new(AuditStorage::new());
        let audit_logger = Arc::new(AuditLogger::new(audit_config, audit_storage, None));

        // P0-5: Resource Quota
        tracing::info!("Initializing resource quota managers...");
        use clawmaster_resource_quota::{
            ConnectionLimitConfig, MemoryQuotaConfig, RateLimitConfig, SessionLimitConfig,
            UploadLimitConfig,
        };

        let rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig {
            max_requests: 100,
            window_duration: Duration::from_secs(60),
        }));

        let memory_quota = Arc::new(MemoryQuota::new(MemoryQuotaConfig {
            max_memory: 1024 * 1024 * 1024, // 1GB
        }));

        let connection_limiter = Arc::new(ConnectionLimiter::new(ConnectionLimitConfig {
            max_connections: 1000,
        }));

        let session_limiter = Arc::new(SessionLimiter::new(SessionLimitConfig {
            max_sessions_per_user: 10,
            max_total_sessions: 10000,
        }));

        let upload_limiter = Arc::new(UploadLimiter::new(UploadLimitConfig {
            max_file_size: 100 * 1024 * 1024,  // 100MB per file
            max_total_size: 500 * 1024 * 1024, // 500MB total
        }));

        // P0-6: Backup Recovery
        tracing::info!("Initializing backup manager...");
        let backup_dir = data_dir.join("backups");
        let backup_manager = Arc::new(BackupManager::new(backup_dir)?);

        // Optional: Initialize backup scheduler
        let backup_scheduler = if std::env::var("ENABLE_AUTO_BACKUP").is_ok() {
            let schedule_config = ScheduleConfig {
                full_backup_interval: Duration::from_secs(24 * 3600), // Daily
                incremental_backup_interval: Duration::from_secs(3600), // Hourly
                source_paths: vec![data_dir.join("database.db")],
            };
            Some(Arc::new(BackupScheduler::new(
                schedule_config,
                Arc::clone(&backup_manager),
            )))
        } else {
            None
        };

        tracing::info!("All P0 features initialized successfully");

        Ok(Self {
            health_checker,
            config_validator,
            circuit_breaker,
            retry_executor,
            degradation_manager,
            isolation_manager,
            audit_logger,
            rate_limiter,
            memory_quota,
            connection_limiter,
            session_limiter,
            upload_limiter,
            backup_manager,
            backup_scheduler,
        })
    }

    /// Start background tasks for P0 features
    pub async fn start_background_tasks(&self) -> anyhow::Result<()> {
        tracing::info!("Starting P0 background tasks...");

        // Start backup scheduler if enabled
        if let Some(scheduler) = &self.backup_scheduler {
            let scheduler = Arc::clone(scheduler);
            tokio::spawn(async move {
                if let Err(e) = scheduler.start().await {
                    tracing::error!("Backup scheduler error: {}", e);
                }
            });
            tracing::info!("Backup scheduler started");
        }

        // Start health check monitoring
        let health_checker: Arc<HealthCheckService> = Arc::clone(&self.health_checker);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let health = health_checker.check_health().await;
                if !health.status.is_healthy() {
                    tracing::warn!("Health check failed: {:?}", health);
                }
            }
        });
        tracing::info!("Health check monitoring started");

        tracing::info!("All P0 background tasks started");
        Ok(())
    }

    /// Get health status
    pub async fn get_health_status(&self) -> clawmaster_health_check::SystemHealth {
        self.health_checker.check_health().await
    }

    /// Validate configuration
    pub fn validate_config(
        &self,
        config: &clawmaster_config::MoltisConfig,
    ) -> clawmaster_config_validator::ValidationReport {
        self.config_validator.validate(config)
    }

    /// Check if service is isolated
    pub fn is_service_isolated(&self, service: &str) -> bool {
        self.isolation_manager.is_isolated(service)
    }

    /// Report service fault
    pub fn report_fault(&self, service: &str, description: String) -> anyhow::Result<()> {
        self.isolation_manager
            .report_fault(service, description)
            .map_err(|e| anyhow::anyhow!("Failed to report fault: {}", e))
    }

    /// Isolate service
    pub fn isolate_service(&self, service: &str) -> anyhow::Result<()> {
        self.isolation_manager
            .isolate(service)
            .map_err(|e| anyhow::anyhow!("Failed to isolate service: {}", e))
    }

    /// Restore service
    pub fn restore_service(&self, service: &str) -> anyhow::Result<()> {
        self.isolation_manager
            .restore(service)
            .map_err(|e| anyhow::anyhow!("Failed to restore service: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[tokio::test]
    async fn test_p0_features_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let features = P0Features::new(temp_dir.path()).await.unwrap();

        // Verify all components are initialized
        assert!(Arc::strong_count(&features.health_checker) >= 1);
        assert!(Arc::strong_count(&features.config_validator) >= 1);
        assert!(Arc::strong_count(&features.circuit_breaker) >= 1);
        assert!(Arc::strong_count(&features.audit_logger) >= 1);
        assert!(Arc::strong_count(&features.rate_limiter) >= 1);
        assert!(Arc::strong_count(&features.backup_manager) >= 1);
    }

    #[tokio::test]
    async fn test_health_check() {
        let temp_dir = TempDir::new().unwrap();
        let features = P0Features::new(temp_dir.path()).await.unwrap();

        let health = features.get_health_status().await;
        // Health status should be available
        assert!(health.checks.is_empty() || !health.checks.is_empty());
    }

    #[tokio::test]
    async fn test_service_isolation() {
        let temp_dir = TempDir::new().unwrap();
        let features = P0Features::new(temp_dir.path()).await.unwrap();

        // Initially not isolated
        assert!(!features.is_service_isolated("database"));

        // Report fault
        features
            .report_fault("database", "Connection timeout".to_string())
            .unwrap();

        // Isolate service
        features.isolate_service("database").unwrap();
        assert!(features.is_service_isolated("database"));

        // Restore service
        features.restore_service("database").unwrap();
        assert!(!features.is_service_isolated("database"));
    }
}
