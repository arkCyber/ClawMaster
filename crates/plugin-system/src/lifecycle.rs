//! Plugin lifecycle management

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    /// Plugin is not loaded
    Unloaded,
    /// Plugin is loaded but not enabled
    Loaded,
    /// Plugin is enabled and running
    Enabled,
    /// Plugin is in error state
    Error,
}

/// Plugin lifecycle manager
pub struct LifecycleManager {
    states: Arc<RwLock<HashMap<String, PluginState>>>,
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize a plugin (set to Loaded state)
    pub async fn initialize(&self, plugin_id: &str) -> Result<()> {
        let mut states = self.states.write().await;
        
        if states.contains_key(plugin_id) {
            anyhow::bail!("plugin already initialized: {}", plugin_id);
        }

        states.insert(plugin_id.to_string(), PluginState::Loaded);
        tracing::debug!(plugin_id = %plugin_id, "plugin initialized");
        
        Ok(())
    }

    /// Enable a plugin
    pub async fn enable(&self, plugin_id: &str) -> Result<()> {
        let mut states = self.states.write().await;
        
        let state = states.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_id))?;

        match state {
            PluginState::Loaded => {
                states.insert(plugin_id.to_string(), PluginState::Enabled);
                tracing::info!(plugin_id = %plugin_id, "plugin enabled");
                Ok(())
            }
            PluginState::Enabled => {
                tracing::warn!(plugin_id = %plugin_id, "plugin already enabled");
                Ok(())
            }
            PluginState::Unloaded => {
                anyhow::bail!("cannot enable unloaded plugin: {}", plugin_id)
            }
            PluginState::Error => {
                anyhow::bail!("cannot enable plugin in error state: {}", plugin_id)
            }
        }
    }

    /// Disable a plugin
    pub async fn disable(&self, plugin_id: &str) -> Result<()> {
        let mut states = self.states.write().await;
        
        let state = states.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_id))?;

        match state {
            PluginState::Enabled => {
                states.insert(plugin_id.to_string(), PluginState::Loaded);
                tracing::info!(plugin_id = %plugin_id, "plugin disabled");
                Ok(())
            }
            PluginState::Loaded => {
                tracing::warn!(plugin_id = %plugin_id, "plugin already disabled");
                Ok(())
            }
            PluginState::Unloaded => {
                anyhow::bail!("cannot disable unloaded plugin: {}", plugin_id)
            }
            PluginState::Error => {
                states.insert(plugin_id.to_string(), PluginState::Loaded);
                tracing::info!(plugin_id = %plugin_id, "plugin disabled from error state");
                Ok(())
            }
        }
    }

    /// Unload a plugin
    pub async fn unload(&self, plugin_id: &str) -> Result<()> {
        let mut states = self.states.write().await;
        
        if !states.contains_key(plugin_id) {
            anyhow::bail!("plugin not found: {}", plugin_id);
        }

        states.remove(plugin_id);
        tracing::info!(plugin_id = %plugin_id, "plugin unloaded");
        
        Ok(())
    }

    /// Set plugin to error state
    pub async fn set_error(&self, plugin_id: &str) -> Result<()> {
        let mut states = self.states.write().await;
        
        if !states.contains_key(plugin_id) {
            anyhow::bail!("plugin not found: {}", plugin_id);
        }

        states.insert(plugin_id.to_string(), PluginState::Error);
        tracing::error!(plugin_id = %plugin_id, "plugin entered error state");
        
        Ok(())
    }

    /// Get plugin state
    pub async fn get_state(&self, plugin_id: &str) -> Result<PluginState> {
        let states = self.states.read().await;
        
        states.get(plugin_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_id))
    }

    /// Check if plugin is enabled
    pub async fn is_enabled(&self, plugin_id: &str) -> Result<bool> {
        let state = self.get_state(plugin_id).await?;
        Ok(state == PluginState::Enabled)
    }

    /// List all plugins with their states
    pub async fn list_all(&self) -> HashMap<String, PluginState> {
        let states = self.states.read().await;
        states.clone()
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lifecycle_initialize() {
        let manager = LifecycleManager::new();
        assert!(manager.initialize("test-plugin").await.is_ok());
        
        let state = manager.get_state("test-plugin").await.unwrap();
        assert_eq!(state, PluginState::Loaded);
    }

    #[tokio::test]
    async fn test_lifecycle_enable() {
        let manager = LifecycleManager::new();
        manager.initialize("test-plugin").await.unwrap();
        
        assert!(manager.enable("test-plugin").await.is_ok());
        assert!(manager.is_enabled("test-plugin").await.unwrap());
    }

    #[tokio::test]
    async fn test_lifecycle_disable() {
        let manager = LifecycleManager::new();
        manager.initialize("test-plugin").await.unwrap();
        manager.enable("test-plugin").await.unwrap();
        
        assert!(manager.disable("test-plugin").await.is_ok());
        assert!(!manager.is_enabled("test-plugin").await.unwrap());
    }

    #[tokio::test]
    async fn test_lifecycle_unload() {
        let manager = LifecycleManager::new();
        manager.initialize("test-plugin").await.unwrap();
        
        assert!(manager.unload("test-plugin").await.is_ok());
        assert!(manager.get_state("test-plugin").await.is_err());
    }

    #[tokio::test]
    async fn test_lifecycle_error_state() {
        let manager = LifecycleManager::new();
        manager.initialize("test-plugin").await.unwrap();
        
        assert!(manager.set_error("test-plugin").await.is_ok());
        
        let state = manager.get_state("test-plugin").await.unwrap();
        assert_eq!(state, PluginState::Error);
    }

    #[tokio::test]
    async fn test_lifecycle_double_initialize() {
        let manager = LifecycleManager::new();
        manager.initialize("test-plugin").await.unwrap();
        
        assert!(manager.initialize("test-plugin").await.is_err());
    }
}
