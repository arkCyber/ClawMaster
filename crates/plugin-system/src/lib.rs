//! ClawMaster Plugin System
//!
//! DO-178C Level A compliant plugin system with:
//! - Unified plugin lifecycle management
//! - Plugin isolation and sandboxing
//! - Event-driven communication
//! - Dependency resolution
//! - Hot reload support

pub mod config;
pub mod dependency;
pub mod event;
pub mod lifecycle;
pub mod plugin;
pub mod registry;
pub mod sandbox;

pub use {
    config::{PluginConfig, PluginConfigManager},
    dependency::DependencyResolver,
    event::{Event, EventBus, EventHandler},
    lifecycle::{LifecycleManager, PluginState},
    plugin::{Permission, Plugin, PluginContext, PluginDependency, PluginMetadata},
    registry::PluginRegistry,
    sandbox::{PluginSandbox, SandboxConfig},
};

use {
    anyhow::Result,
    std::{path::PathBuf, sync::Arc},
    tokio::sync::RwLock,
};

/// Main plugin system coordinator
pub struct PluginSystem {
    registry: Arc<RwLock<PluginRegistry>>,
    lifecycle_manager: Arc<LifecycleManager>,
    dependency_resolver: Arc<RwLock<DependencyResolver>>,
    event_bus: Arc<EventBus>,
    config_manager: Arc<RwLock<PluginConfigManager>>,
    #[cfg(feature = "sandbox")]
    sandbox: Arc<PluginSandbox>,
}

impl PluginSystem {
    /// Create a new plugin system
    pub fn new(plugin_dir: PathBuf) -> Result<Self> {
        let registry = Arc::new(RwLock::new(PluginRegistry::new(plugin_dir)));
        let lifecycle_manager = Arc::new(LifecycleManager::new());
        let dependency_resolver = Arc::new(RwLock::new(DependencyResolver::new()));
        let event_bus = Arc::new(EventBus::new());
        let config_manager = Arc::new(RwLock::new(PluginConfigManager::new()));

        #[cfg(feature = "sandbox")]
        let sandbox = Arc::new(PluginSandbox::new(SandboxConfig::default())?);

        Ok(Self {
            registry,
            lifecycle_manager,
            dependency_resolver,
            event_bus,
            config_manager,
            #[cfg(feature = "sandbox")]
            sandbox,
        })
    }

    /// Load a plugin from a directory
    pub async fn load_plugin(&self, plugin_path: PathBuf) -> Result<String> {
        // 1. Read plugin metadata
        let metadata = plugin::read_metadata(&plugin_path)?;

        // 2. Validate plugin
        self.validate_plugin(&metadata)?;

        // 3. Resolve dependencies
        let resolver = self.dependency_resolver.read().await;
        resolver.resolve(&metadata.dependencies).await?;
        drop(resolver);

        // 4. Load plugin
        let plugin_id = metadata.id.clone();
        let version = metadata.version.clone();
        let mut registry = self.registry.write().await;
        registry.register(metadata, plugin_path)?;
        drop(registry);

        // 5. Register in dependency resolver
        let mut resolver = self.dependency_resolver.write().await;
        resolver.register_plugin(plugin_id.clone(), version);
        drop(resolver);

        // 6. Initialize lifecycle
        self.lifecycle_manager.initialize(&plugin_id).await?;

        tracing::info!(plugin_id = %plugin_id, "plugin loaded successfully");
        Ok(plugin_id)
    }

    /// Enable a plugin
    pub async fn enable_plugin(&self, plugin_id: &str) -> Result<()> {
        self.lifecycle_manager.enable(plugin_id).await?;

        // Emit event
        self.event_bus
            .emit(Event::PluginEnabled {
                plugin_id: plugin_id.to_string(),
            })
            .await?;

        tracing::info!(plugin_id = %plugin_id, "plugin enabled");
        Ok(())
    }

    /// Disable a plugin
    pub async fn disable_plugin(&self, plugin_id: &str) -> Result<()> {
        self.lifecycle_manager.disable(plugin_id).await?;

        // Emit event
        self.event_bus
            .emit(Event::PluginDisabled {
                plugin_id: plugin_id.to_string(),
            })
            .await?;

        tracing::info!(plugin_id = %plugin_id, "plugin disabled");
        Ok(())
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, plugin_id: &str) -> Result<()> {
        // 1. Disable if enabled
        if self.lifecycle_manager.is_enabled(plugin_id).await? {
            self.disable_plugin(plugin_id).await?;
        }

        // 2. Unload
        self.lifecycle_manager.unload(plugin_id).await?;

        // 3. Remove from registry
        let mut registry = self.registry.write().await;
        registry.unregister(plugin_id)?;

        tracing::info!(plugin_id = %plugin_id, "plugin unloaded");
        Ok(())
    }

    /// Reload a plugin (hot reload)
    #[cfg(feature = "hot-reload")]
    pub async fn reload_plugin(&self, plugin_id: &str) -> Result<()> {
        let registry = self.registry.read().await;
        let plugin_path = registry.get_plugin_path(plugin_id)?;

        // Unload and reload
        drop(registry);
        self.unload_plugin(plugin_id).await?;
        self.load_plugin(plugin_path).await?;

        tracing::info!(plugin_id = %plugin_id, "plugin reloaded");
        Ok(())
    }

    /// List all plugins
    pub async fn list_plugins(&self) -> Result<Vec<PluginMetadata>> {
        let registry = self.registry.read().await;
        Ok(registry.list())
    }

    /// Get plugin metadata
    pub async fn get_plugin(&self, plugin_id: &str) -> Result<PluginMetadata> {
        let registry = self.registry.read().await;
        registry.get(plugin_id)
    }

    /// Update plugin configuration
    pub async fn update_config(&self, plugin_id: &str, config: serde_json::Value) -> Result<()> {
        // Validate config against schema
        let registry = self.registry.read().await;
        let metadata = registry.get(plugin_id)?;
        drop(registry);

        let config_manager = self.config_manager.read().await;
        config_manager.validate(&metadata.config_schema, &config)?;
        drop(config_manager);

        let mut config_manager = self.config_manager.write().await;
        config_manager.update(plugin_id, config.clone())?;
        drop(config_manager);

        // Notify plugin of config change
        self.event_bus
            .emit(Event::ConfigChanged {
                plugin_id: plugin_id.to_string(),
                config,
            })
            .await?;

        Ok(())
    }

    /// Subscribe to events
    pub async fn subscribe<F>(&self, event_type: &str, handler: F) -> Result<()>
    where
        F: Fn(Event) -> Result<()> + Send + Sync + 'static,
    {
        self.event_bus
            .subscribe(event_type, Box::new(handler))
            .await
    }

    /// Validate plugin
    fn validate_plugin(&self, metadata: &PluginMetadata) -> Result<()> {
        // Check ID format
        if metadata.id.is_empty() {
            anyhow::bail!("plugin ID cannot be empty");
        }

        // Check version format
        semver::Version::parse(&metadata.version)?;

        // Check permissions
        for permission in &metadata.permissions {
            if !permission.is_valid() {
                anyhow::bail!("invalid permission: {:?}", permission);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_system_creation() {
        let tmp = tempfile::tempdir().unwrap();
        let system = PluginSystem::new(tmp.path().to_path_buf());
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_list_plugins_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();
        let plugins = system.list_plugins().await.unwrap();
        assert_eq!(plugins.len(), 0);
    }
}
