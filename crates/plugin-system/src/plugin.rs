//! Plugin trait and metadata definitions

use {
    anyhow::Result,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::path::{Path, PathBuf},
};

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Unique plugin identifier
    pub id: String,
    /// Plugin name
    pub name: String,
    /// Plugin version (semver)
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Plugin homepage
    pub homepage: Option<String>,
    /// Plugin license
    pub license: Option<String>,
    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Required permissions
    pub permissions: Vec<Permission>,
    /// Configuration schema (JSON Schema)
    pub config_schema: serde_json::Value,
    /// Plugin tags
    pub tags: Vec<String>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Dependency plugin ID
    pub plugin_id: String,
    /// Required version (semver range)
    pub version: String,
    /// Whether this dependency is optional
    pub optional: bool,
}

/// Plugin permission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    /// Read file system
    FileRead,
    /// Write file system
    FileWrite,
    /// Execute commands
    Execute,
    /// Network access
    Network,
    /// Environment variables
    Environment,
    /// Database access
    Database,
    /// Custom permission
    Custom(String),
}

impl Permission {
    /// Check if permission is valid
    pub fn is_valid(&self) -> bool {
        match self {
            Permission::Custom(s) => !s.is_empty(),
            _ => true,
        }
    }
}

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Plugin ID
    pub plugin_id: String,
    /// Plugin configuration
    pub config: serde_json::Value,
    /// Plugin data directory
    pub data_dir: PathBuf,
    /// Granted permissions
    pub permissions: Vec<Permission>,
}

/// Plugin trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Called when plugin is loaded
    async fn on_load(&mut self, context: &PluginContext) -> Result<()> {
        let _ = context;
        Ok(())
    }

    /// Called when plugin is enabled
    async fn on_enable(&mut self, context: &PluginContext) -> Result<()> {
        let _ = context;
        Ok(())
    }

    /// Called when plugin is disabled
    async fn on_disable(&mut self, context: &PluginContext) -> Result<()> {
        let _ = context;
        Ok(())
    }

    /// Called when plugin is unloaded
    async fn on_unload(&mut self, context: &PluginContext) -> Result<()> {
        let _ = context;
        Ok(())
    }

    /// Called when configuration changes
    async fn on_config_change(
        &mut self,
        context: &PluginContext,
        new_config: serde_json::Value,
    ) -> Result<()> {
        let _ = (context, new_config);
        Ok(())
    }

    /// Execute plugin action
    async fn execute(
        &self,
        context: &PluginContext,
        action: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let _ = (context, action, params);
        anyhow::bail!("action not implemented: {}", action)
    }
}

/// Read plugin metadata from plugin.toml
pub fn read_metadata(plugin_path: &Path) -> Result<PluginMetadata> {
    let manifest_path = plugin_path.join("plugin.toml");

    if !manifest_path.exists() {
        anyhow::bail!("plugin.toml not found in {:?}", plugin_path);
    }

    let content = std::fs::read_to_string(&manifest_path)?;
    let metadata: PluginMetadata = toml::from_str(&content)?;

    Ok(metadata)
}

/// Validate plugin metadata
pub fn validate_metadata(metadata: &PluginMetadata) -> Result<()> {
    // Validate ID
    if metadata.id.is_empty() {
        anyhow::bail!("plugin ID cannot be empty");
    }

    if !metadata
        .id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!(
            "plugin ID can only contain alphanumeric characters, hyphens, and underscores"
        );
    }

    // Validate version
    semver::Version::parse(&metadata.version)?;

    // Validate name
    if metadata.name.is_empty() {
        anyhow::bail!("plugin name cannot be empty");
    }

    // Validate author
    if metadata.author.is_empty() {
        anyhow::bail!("plugin author cannot be empty");
    }

    // Validate permissions
    for permission in &metadata.permissions {
        if !permission.is_valid() {
            anyhow::bail!("invalid permission: {:?}", permission);
        }
    }

    // Validate dependencies
    for dep in &metadata.dependencies {
        if dep.plugin_id.is_empty() {
            anyhow::bail!("dependency plugin_id cannot be empty");
        }
        semver::VersionReq::parse(&dep.version)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_validation() {
        assert!(Permission::FileRead.is_valid());
        assert!(Permission::FileWrite.is_valid());
        assert!(Permission::Execute.is_valid());
        assert!(Permission::Network.is_valid());
        assert!(Permission::Custom("custom".to_string()).is_valid());
        assert!(!Permission::Custom("".to_string()).is_valid());
    }

    #[test]
    fn test_metadata_validation() {
        let valid_metadata = PluginMetadata {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test description".to_string(),
            homepage: None,
            license: None,
            dependencies: vec![],
            permissions: vec![Permission::FileRead],
            config_schema: serde_json::json!({}),
            tags: vec![],
        };

        assert!(validate_metadata(&valid_metadata).is_ok());
    }

    #[test]
    fn test_metadata_validation_empty_id() {
        let invalid_metadata = PluginMetadata {
            id: "".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test description".to_string(),
            homepage: None,
            license: None,
            dependencies: vec![],
            permissions: vec![],
            config_schema: serde_json::json!({}),
            tags: vec![],
        };

        assert!(validate_metadata(&invalid_metadata).is_err());
    }

    #[test]
    fn test_metadata_validation_invalid_version() {
        let invalid_metadata = PluginMetadata {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "invalid".to_string(),
            author: "Test Author".to_string(),
            description: "Test description".to_string(),
            homepage: None,
            license: None,
            dependencies: vec![],
            permissions: vec![],
            config_schema: serde_json::json!({}),
            tags: vec![],
        };

        assert!(validate_metadata(&invalid_metadata).is_err());
    }
}
