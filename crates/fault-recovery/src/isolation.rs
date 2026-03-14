//! Fault Isolation
//!
//! DO-178C Level A Compliant Fault Isolation

use crate::{FaultError, FaultResult};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use time::OffsetDateTime;

/// Isolation status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsolationStatus {
    /// Component is active
    Active,
    
    /// Component is isolated due to faults
    Isolated,
}

/// Fault record
#[derive(Debug, Clone)]
pub struct FaultRecord {
    pub timestamp: OffsetDateTime,
    pub description: String,
}

/// Isolation manager
///
/// DO-178C §6.3.3: Fault isolation
pub struct IsolationManager {
    components: Arc<RwLock<HashMap<String, ComponentState>>>,
}

#[derive(Debug, Clone)]
struct ComponentState {
    status: IsolationStatus,
    fault_count: usize,
    faults: Vec<FaultRecord>,
}

impl IsolationManager {
    /// Create new isolation manager
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register component
    pub fn register(&self, component_id: &str) {
        let mut components = self.components.write();
        components.insert(
            component_id.to_string(),
            ComponentState {
                status: IsolationStatus::Active,
                fault_count: 0,
                faults: Vec::new(),
            },
        );
    }

    /// Report fault for component
    ///
    /// DO-178C §6.3.3: Fault reporting
    pub fn report_fault(&self, component_id: &str, description: String) -> FaultResult<()> {
        let mut components = self.components.write();
        
        let state = components
            .get_mut(component_id)
            .ok_or_else(|| FaultError::OperationFailed("Component not registered".to_string()))?;

        state.fault_count += 1;
        state.faults.push(FaultRecord {
            timestamp: OffsetDateTime::now_utc(),
            description: description.clone(),
        });

        tracing::warn!(
            "Fault reported for {}: {} (total: {})",
            component_id,
            description,
            state.fault_count
        );

        Ok(())
    }

    /// Isolate component
    ///
    /// DO-178C §6.3.3: Component isolation
    pub fn isolate(&self, component_id: &str) -> FaultResult<()> {
        let mut components = self.components.write();
        
        let state = components
            .get_mut(component_id)
            .ok_or_else(|| FaultError::OperationFailed("Component not registered".to_string()))?;

        if state.status == IsolationStatus::Active {
            state.status = IsolationStatus::Isolated;
            tracing::error!("Component isolated: {}", component_id);
        }

        Ok(())
    }

    /// Restore component
    pub fn restore(&self, component_id: &str) -> FaultResult<()> {
        let mut components = self.components.write();
        
        let state = components
            .get_mut(component_id)
            .ok_or_else(|| FaultError::OperationFailed("Component not registered".to_string()))?;

        if state.status == IsolationStatus::Isolated {
            state.status = IsolationStatus::Active;
            state.fault_count = 0;
            state.faults.clear();
            tracing::info!("Component restored: {}", component_id);
        }

        Ok(())
    }

    /// Check if component is isolated
    pub fn is_isolated(&self, component_id: &str) -> bool {
        let components = self.components.read();
        components
            .get(component_id)
            .map(|s| s.status == IsolationStatus::Isolated)
            .unwrap_or(false)
    }

    /// Get component status
    pub fn get_status(&self, component_id: &str) -> Option<IsolationStatus> {
        let components = self.components.read();
        components.get(component_id).map(|s| s.status.clone())
    }

    /// Get fault count
    pub fn get_fault_count(&self, component_id: &str) -> usize {
        let components = self.components.read();
        components.get(component_id).map(|s| s.fault_count).unwrap_or(0)
    }

    /// Execute with isolation check
    pub async fn execute_isolated<F, Fut, T>(
        &self,
        component_id: &str,
        operation: F,
    ) -> FaultResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = FaultResult<T>>,
    {
        if self.is_isolated(component_id) {
            return Err(FaultError::FaultIsolated(format!(
                "Component {} is isolated",
                component_id
            )));
        }

        operation().await
    }
}

impl Default for IsolationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolation_manager_creation() {
        let manager = IsolationManager::new();
        manager.register("component1");
        
        assert!(!manager.is_isolated("component1"));
        assert_eq!(manager.get_fault_count("component1"), 0);
    }

    #[test]
    fn test_report_fault() {
        let manager = IsolationManager::new();
        manager.register("component1");

        manager.report_fault("component1", "Error 1".to_string()).unwrap();
        manager.report_fault("component1", "Error 2".to_string()).unwrap();

        assert_eq!(manager.get_fault_count("component1"), 2);
    }

    #[test]
    fn test_isolate_component() {
        let manager = IsolationManager::new();
        manager.register("component1");

        assert!(!manager.is_isolated("component1"));

        manager.isolate("component1").unwrap();

        assert!(manager.is_isolated("component1"));
        assert_eq!(manager.get_status("component1"), Some(IsolationStatus::Isolated));
    }

    #[test]
    fn test_restore_component() {
        let manager = IsolationManager::new();
        manager.register("component1");

        manager.report_fault("component1", "Error".to_string()).unwrap();
        manager.isolate("component1").unwrap();

        assert!(manager.is_isolated("component1"));
        assert_eq!(manager.get_fault_count("component1"), 1);

        manager.restore("component1").unwrap();

        assert!(!manager.is_isolated("component1"));
        assert_eq!(manager.get_fault_count("component1"), 0);
    }

    #[tokio::test]
    async fn test_execute_isolated_allowed() {
        let manager = IsolationManager::new();
        manager.register("component1");

        let result = manager
            .execute_isolated("component1", || async { Ok::<_, FaultError>("success") })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_isolated_blocked() {
        let manager = IsolationManager::new();
        manager.register("component1");
        manager.isolate("component1").unwrap();

        let result = manager
            .execute_isolated("component1", || async { Ok::<_, FaultError>("success") })
            .await;

        assert!(result.is_err());
    }
}
