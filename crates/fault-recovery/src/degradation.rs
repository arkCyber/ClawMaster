//! Graceful Degradation
//!
//! DO-178C Level A Compliant Graceful Degradation

use {
    crate::{FaultError, FaultResult},
    parking_lot::RwLock,
    std::sync::Arc,
};

/// Service level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceLevel {
    /// Emergency mode
    Emergency,

    /// Minimal functionality
    Minimal,

    /// Reduced functionality
    Reduced,

    /// Full functionality
    Full,
}

/// Degradation manager
///
/// DO-178C §6.3.3: Graceful degradation
pub struct DegradationManager {
    current_level: Arc<RwLock<ServiceLevel>>,
}

impl DegradationManager {
    /// Create new degradation manager
    pub fn new() -> Self {
        Self {
            current_level: Arc::new(RwLock::new(ServiceLevel::Full)),
        }
    }

    /// Degrade service level
    ///
    /// DO-178C §6.3.3: Service degradation
    pub fn degrade(&self, level: ServiceLevel) {
        let mut current = self.current_level.write();
        if level < *current {
            *current = level;
            tracing::warn!("Service degraded to {:?}", level);
        }
    }

    /// Restore service level
    pub fn restore(&self, level: ServiceLevel) {
        let mut current = self.current_level.write();
        if level > *current {
            *current = level;
            tracing::info!("Service restored to {:?}", level);
        }
    }

    /// Get current service level
    pub fn get_level(&self) -> ServiceLevel {
        *self.current_level.read()
    }

    /// Check if operation is allowed at current level
    pub fn check_allowed(&self, required_level: ServiceLevel) -> FaultResult<()> {
        let current = *self.current_level.read();
        if current >= required_level {
            Ok(())
        } else {
            Err(FaultError::ServiceDegraded(format!(
                "Operation requires {:?}, current level is {:?}",
                required_level, current
            )))
        }
    }

    /// Execute with degradation fallback
    pub async fn execute_with_fallback<F, Fut, T>(
        &self,
        primary: F,
        fallback: Option<T>,
    ) -> FaultResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = FaultResult<T>>,
    {
        match primary().await {
            Ok(result) => Ok(result),
            Err(e) => {
                if let Some(fallback_value) = fallback {
                    tracing::warn!("Using fallback due to: {}", e);
                    self.degrade(ServiceLevel::Reduced);
                    Ok(fallback_value)
                } else {
                    Err(e)
                }
            },
        }
    }
}

impl Default for DegradationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degradation_manager_creation() {
        let manager = DegradationManager::new();
        assert_eq!(manager.get_level(), ServiceLevel::Full);
    }

    #[test]
    fn test_degrade_service() {
        let manager = DegradationManager::new();
        assert_eq!(manager.get_level(), ServiceLevel::Full);

        manager.degrade(ServiceLevel::Reduced);
        assert_eq!(manager.get_level(), ServiceLevel::Reduced);

        manager.degrade(ServiceLevel::Minimal);
        assert_eq!(manager.get_level(), ServiceLevel::Minimal);

        // Trying to degrade to higher level should not work
        manager.degrade(ServiceLevel::Reduced);
        assert_eq!(manager.get_level(), ServiceLevel::Minimal);
    }

    #[test]
    fn test_restore_service() {
        let manager = DegradationManager::new();

        manager.degrade(ServiceLevel::Minimal);
        assert_eq!(manager.get_level(), ServiceLevel::Minimal);

        manager.restore(ServiceLevel::Reduced);
        assert_eq!(manager.get_level(), ServiceLevel::Reduced);

        manager.restore(ServiceLevel::Full);
        assert_eq!(manager.get_level(), ServiceLevel::Full);
    }

    #[test]
    fn test_check_allowed() {
        let manager = DegradationManager::new();

        // Full level allows all operations
        assert!(manager.check_allowed(ServiceLevel::Full).is_ok());
        assert!(manager.check_allowed(ServiceLevel::Reduced).is_ok());
        assert!(manager.check_allowed(ServiceLevel::Minimal).is_ok());

        // Degrade to Reduced
        manager.degrade(ServiceLevel::Reduced);

        // Reduced level blocks Full operations but allows Reduced and lower
        assert!(manager.check_allowed(ServiceLevel::Full).is_err());
        assert!(manager.check_allowed(ServiceLevel::Reduced).is_ok());
        assert!(manager.check_allowed(ServiceLevel::Minimal).is_ok());
        assert!(manager.check_allowed(ServiceLevel::Emergency).is_ok());
    }

    #[tokio::test]
    async fn test_execute_with_fallback_success() {
        let manager = DegradationManager::new();

        let result = manager
            .execute_with_fallback(
                || async { Ok::<_, FaultError>("primary") },
                Some("fallback"),
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "primary");
        assert_eq!(manager.get_level(), ServiceLevel::Full);
    }

    #[tokio::test]
    async fn test_execute_with_fallback_degraded() {
        let manager = DegradationManager::new();

        let result = manager
            .execute_with_fallback(
                || async { Err::<String, _>(FaultError::OperationFailed("error".to_string())) },
                Some("fallback".to_string()),
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "fallback");
        assert_eq!(manager.get_level(), ServiceLevel::Reduced);
    }

    #[tokio::test]
    async fn test_execute_with_fallback_no_fallback() {
        let manager = DegradationManager::new();

        let result = manager
            .execute_with_fallback::<_, _, String>(
                || async { Err(FaultError::OperationFailed("error".to_string())) },
                None,
            )
            .await;

        assert!(result.is_err());
    }
}
