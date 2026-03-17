//! Plugin configuration management

use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use serde_json::Value;

/// Plugin configuration manager
pub struct PluginConfigManager {
    configs: HashMap<String, PluginConfig>,
    config_dir: Option<PathBuf>,
}

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Plugin ID
    pub plugin_id: String,
    /// Configuration data
    pub config: Value,
    /// Configuration file path
    pub config_path: Option<PathBuf>,
}

impl PluginConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            config_dir: None,
        }
    }

    /// Create a new configuration manager with a config directory
    pub fn with_config_dir(config_dir: PathBuf) -> Self {
        Self {
            configs: HashMap::new(),
            config_dir: Some(config_dir),
        }
    }

    /// Load configuration for a plugin
    pub fn load(&mut self, plugin_id: &str) -> Result<Value> {
        // Check if already loaded
        if let Some(config) = self.configs.get(plugin_id) {
            return Ok(config.config.clone());
        }

        // Try to load from file
        if let Some(config_dir) = &self.config_dir {
            let config_path = config_dir.join(format!("{}.json", plugin_id));
            
            if config_path.exists() {
                let content = std::fs::read_to_string(&config_path)?;
                let config: Value = serde_json::from_str(&content)?;
                
                self.configs.insert(
                    plugin_id.to_string(),
                    PluginConfig {
                        plugin_id: plugin_id.to_string(),
                        config: config.clone(),
                        config_path: Some(config_path),
                    },
                );
                
                return Ok(config);
            }
        }

        // Return empty config if not found
        let empty_config = serde_json::json!({});
        self.configs.insert(
            plugin_id.to_string(),
            PluginConfig {
                plugin_id: plugin_id.to_string(),
                config: empty_config.clone(),
                config_path: None,
            },
        );
        
        Ok(empty_config)
    }

    /// Update configuration for a plugin
    pub fn update(&mut self, plugin_id: &str, config: Value) -> Result<()> {
        let config_path = if let Some(config_dir) = &self.config_dir {
            Some(config_dir.join(format!("{}.json", plugin_id)))
        } else {
            None
        };

        self.configs.insert(
            plugin_id.to_string(),
            PluginConfig {
                plugin_id: plugin_id.to_string(),
                config: config.clone(),
                config_path: config_path.clone(),
            },
        );

        // Save to file if config directory is set
        if let Some(path) = config_path {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(&config)?;
            std::fs::write(&path, content)?;
        }

        tracing::debug!(plugin_id = %plugin_id, "configuration updated");
        Ok(())
    }

    /// Get configuration for a plugin
    pub fn get(&self, plugin_id: &str) -> Option<&Value> {
        self.configs.get(plugin_id).map(|c| &c.config)
    }

    /// Validate configuration against schema
    pub fn validate(&self, schema: &Value, config: &Value) -> Result<()> {
        // Basic validation - check required fields
        if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if !config.get(field_name).is_some() {
                        anyhow::bail!("required field missing: {}", field_name);
                    }
                }
            }
        }

        // Type validation
        if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
            for (key, prop_schema) in properties {
                if let Some(value) = config.get(key) {
                    if let Some(expected_type) = prop_schema.get("type").and_then(|t| t.as_str()) {
                        let actual_type = match value {
                            Value::Null => "null",
                            Value::Bool(_) => "boolean",
                            Value::Number(_) => "number",
                            Value::String(_) => "string",
                            Value::Array(_) => "array",
                            Value::Object(_) => "object",
                        };

                        if expected_type != actual_type {
                            anyhow::bail!(
                                "type mismatch for field {}: expected {}, got {}",
                                key,
                                expected_type,
                                actual_type
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Remove configuration for a plugin
    pub fn remove(&mut self, plugin_id: &str) -> Result<()> {
        if let Some(config) = self.configs.remove(plugin_id) {
            // Delete config file if it exists
            if let Some(path) = config.config_path {
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
            }
        }
        Ok(())
    }

    /// List all plugin IDs with configurations
    pub fn list(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }
}

impl Default for PluginConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager_update_and_get() {
        let mut manager = PluginConfigManager::new();
        let config = serde_json::json!({
            "key": "value"
        });

        manager.update("test-plugin", config.clone()).unwrap();
        
        let retrieved = manager.get("test-plugin").unwrap();
        assert_eq!(retrieved, &config);
    }

    #[test]
    fn test_config_manager_load_default() {
        let mut manager = PluginConfigManager::new();
        let config = manager.load("non-existent").unwrap();
        
        assert_eq!(config, serde_json::json!({}));
    }

    #[test]
    fn test_config_validation_required_fields() {
        let manager = PluginConfigManager::new();
        
        let schema = serde_json::json!({
            "required": ["name", "version"]
        });

        let valid_config = serde_json::json!({
            "name": "test",
            "version": "1.0.0"
        });

        let invalid_config = serde_json::json!({
            "name": "test"
        });

        assert!(manager.validate(&schema, &valid_config).is_ok());
        assert!(manager.validate(&schema, &invalid_config).is_err());
    }

    #[test]
    fn test_config_validation_types() {
        let manager = PluginConfigManager::new();
        
        let schema = serde_json::json!({
            "properties": {
                "name": { "type": "string" },
                "count": { "type": "number" }
            }
        });

        let valid_config = serde_json::json!({
            "name": "test",
            "count": 42
        });

        let invalid_config = serde_json::json!({
            "name": 123,
            "count": "not a number"
        });

        assert!(manager.validate(&schema, &valid_config).is_ok());
        assert!(manager.validate(&schema, &invalid_config).is_err());
    }

    #[test]
    fn test_config_manager_remove() {
        let mut manager = PluginConfigManager::new();
        let config = serde_json::json!({"key": "value"});

        manager.update("test-plugin", config).unwrap();
        assert!(manager.get("test-plugin").is_some());

        manager.remove("test-plugin").unwrap();
        assert!(manager.get("test-plugin").is_none());
    }

    #[test]
    fn test_config_manager_list() {
        let mut manager = PluginConfigManager::new();
        
        manager.update("plugin-a", serde_json::json!({})).unwrap();
        manager.update("plugin-b", serde_json::json!({})).unwrap();

        let list = manager.list();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"plugin-a".to_string()));
        assert!(list.contains(&"plugin-b".to_string()));
    }
}
