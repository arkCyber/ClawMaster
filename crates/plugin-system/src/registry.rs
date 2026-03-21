//! Plugin registry for managing installed plugins

use {
    crate::plugin::PluginMetadata,
    anyhow::Result,
    std::{collections::HashMap, path::PathBuf},
};

/// Plugin registry
pub struct PluginRegistry {
    plugins: HashMap<String, PluginEntry>,
    plugin_dir: PathBuf,
}

/// Plugin registry entry
#[derive(Debug, Clone)]
struct PluginEntry {
    metadata: PluginMetadata,
    path: PathBuf,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_dir,
        }
    }

    /// Register a plugin
    pub fn register(&mut self, metadata: PluginMetadata, path: PathBuf) -> Result<()> {
        let plugin_id = metadata.id.clone();

        if self.plugins.contains_key(&plugin_id) {
            anyhow::bail!("plugin already registered: {}", plugin_id);
        }

        self.plugins
            .insert(plugin_id.clone(), PluginEntry { metadata, path });

        tracing::info!(plugin_id = %plugin_id, "plugin registered");
        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister(&mut self, plugin_id: &str) -> Result<()> {
        if !self.plugins.contains_key(plugin_id) {
            anyhow::bail!("plugin not found: {}", plugin_id);
        }

        self.plugins.remove(plugin_id);
        tracing::info!(plugin_id = %plugin_id, "plugin unregistered");
        Ok(())
    }

    /// Get plugin metadata
    pub fn get(&self, plugin_id: &str) -> Result<PluginMetadata> {
        self.plugins
            .get(plugin_id)
            .map(|entry| entry.metadata.clone())
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_id))
    }

    /// Get plugin path
    pub fn get_plugin_path(&self, plugin_id: &str) -> Result<PathBuf> {
        self.plugins
            .get(plugin_id)
            .map(|entry| entry.path.clone())
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_id))
    }

    /// List all plugins
    pub fn list(&self) -> Vec<PluginMetadata> {
        self.plugins
            .values()
            .map(|entry| entry.metadata.clone())
            .collect()
    }

    /// Check if a plugin is registered
    pub fn contains(&self, plugin_id: &str) -> bool {
        self.plugins.contains_key(plugin_id)
    }

    /// Get plugin count
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Find plugins by tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<PluginMetadata> {
        self.plugins
            .values()
            .filter(|entry| entry.metadata.tags.contains(&tag.to_string()))
            .map(|entry| entry.metadata.clone())
            .collect()
    }

    /// Find plugins by author
    pub fn find_by_author(&self, author: &str) -> Vec<PluginMetadata> {
        self.plugins
            .values()
            .filter(|entry| entry.metadata.author == author)
            .map(|entry| entry.metadata.clone())
            .collect()
    }

    /// Search plugins by name or description
    pub fn search(&self, query: &str) -> Vec<PluginMetadata> {
        let query_lower = query.to_lowercase();

        self.plugins
            .values()
            .filter(|entry| {
                entry.metadata.name.to_lowercase().contains(&query_lower)
                    || entry
                        .metadata
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .map(|entry| entry.metadata.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::plugin::Permission};

    fn create_test_metadata(id: &str) -> PluginMetadata {
        PluginMetadata {
            id: id.to_string(),
            name: format!("Test Plugin {}", id),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test description".to_string(),
            homepage: None,
            license: None,
            dependencies: vec![],
            permissions: vec![Permission::FileRead],
            config_schema: serde_json::json!({}),
            tags: vec!["test".to_string()],
        }
    }

    #[test]
    fn test_registry_register_and_get() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        let metadata = create_test_metadata("test-plugin");
        let path = tmp.path().join("test-plugin");

        registry.register(metadata.clone(), path.clone()).unwrap();

        let retrieved = registry.get("test-plugin").unwrap();
        assert_eq!(retrieved.id, "test-plugin");
    }

    #[test]
    fn test_registry_duplicate_registration() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        let metadata = create_test_metadata("test-plugin");
        let path = tmp.path().join("test-plugin");

        registry.register(metadata.clone(), path.clone()).unwrap();
        assert!(registry.register(metadata, path).is_err());
    }

    #[test]
    fn test_registry_unregister() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        let metadata = create_test_metadata("test-plugin");
        let path = tmp.path().join("test-plugin");

        registry.register(metadata, path).unwrap();
        assert!(registry.contains("test-plugin"));

        registry.unregister("test-plugin").unwrap();
        assert!(!registry.contains("test-plugin"));
    }

    #[test]
    fn test_registry_list() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        registry
            .register(
                create_test_metadata("plugin-a"),
                tmp.path().join("plugin-a"),
            )
            .unwrap();

        registry
            .register(
                create_test_metadata("plugin-b"),
                tmp.path().join("plugin-b"),
            )
            .unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_registry_find_by_tag() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        let mut metadata = create_test_metadata("plugin-a");
        metadata.tags = vec!["tag1".to_string(), "tag2".to_string()];
        registry
            .register(metadata, tmp.path().join("plugin-a"))
            .unwrap();

        let results = registry.find_by_tag("tag1");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "plugin-a");
    }

    #[test]
    fn test_registry_search() {
        let tmp = tempfile::tempdir().unwrap();
        let mut registry = PluginRegistry::new(tmp.path().to_path_buf());

        let mut metadata = create_test_metadata("plugin-a");
        metadata.description = "A plugin for testing search functionality".to_string();
        registry
            .register(metadata, tmp.path().join("plugin-a"))
            .unwrap();

        let results = registry.search("search");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "plugin-a");
    }
}
