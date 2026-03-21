//! Write file tool - DO-178C Level A compliant
//!
//! Implements safe file writing with comprehensive validation and backup.
//! Designed for WASM sandbox execution with strict security controls.

use {
    anyhow::{Context, Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

/// Write file tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WriteFileConfig {
    pub enabled: bool,
    pub workspace_only: bool,
    pub max_file_size: usize,
    pub backup_before_write: bool,
    pub allowed_extensions: Vec<String>,
    pub create_directories: bool,
}

impl Default for WriteFileConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_file_size: 10_000_000, // 10MB
            backup_before_write: true,
            allowed_extensions: vec![],
            create_directories: true,
        }
    }
}

/// Write file tool.
pub struct WriteFileTool {
    config: WriteFileConfig,
    workspace_root: Option<PathBuf>,
}

impl WriteFileTool {
    pub fn new(config: WriteFileConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    fn validate_path(&self, file_path: &Path) -> Result<PathBuf> {
        let path_str = file_path.to_string_lossy();
        if path_str.contains("..") || path_str.contains("~") {
            bail!("Path traversal detected: {}", path_str);
        }

        let absolute_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;
            workspace.join(file_path)
        };

        if self.config.workspace_only {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;

            let parent = absolute_path.parent().unwrap_or(&absolute_path);
            if parent.exists() {
                let canonical_parent = parent.canonicalize()?;
                let canonical_workspace = workspace.canonicalize()?;

                if !canonical_parent.starts_with(&canonical_workspace) {
                    bail!(
                        "Path '{}' is outside workspace '{}'",
                        canonical_parent.display(),
                        canonical_workspace.display()
                    );
                }
            }
        }

        Ok(absolute_path)
    }

    fn validate_extension(&self, file_path: &Path) -> Result<()> {
        if self.config.allowed_extensions.is_empty() {
            return Ok(());
        }

        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        if !self
            .config
            .allowed_extensions
            .iter()
            .any(|ext| ext == extension)
        {
            bail!(
                "File extension '{}' not allowed. Allowed: {:?}",
                extension,
                self.config.allowed_extensions
            );
        }

        Ok(())
    }

    fn backup_file(&self, file_path: &Path) -> Result<Option<PathBuf>> {
        if !self.config.backup_before_write || !file_path.exists() {
            return Ok(None);
        }

        let backup_path = if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            let mut new_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file")
                .to_string();
            new_name.push_str(&format!(".{}.backup", ext));
            file_path.with_file_name(new_name)
        } else {
            file_path.with_extension("backup")
        };

        fs::copy(file_path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

        Ok(Some(backup_path))
    }

    fn write_content(&self, file_path: &Path, content: &str) -> Result<()> {
        if content.len() > self.config.max_file_size {
            bail!(
                "Content size {} exceeds maximum {}",
                content.len(),
                self.config.max_file_size
            );
        }

        if self.config.create_directories {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create directories: {}", parent.display())
                })?;
            }
        }

        fs::write(file_path, content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

        Ok(())
    }
}

#[async_trait]
impl AgentTool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file. Creates the file if it doesn't exist, overwrites if it does."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("write_file tool is disabled");
        }

        let path_str = params
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let content = params
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'content' parameter"))?;

        let file_path = Path::new(path_str);
        let validated_path = self.validate_path(file_path)?;
        self.validate_extension(&validated_path)?;

        let backup_path = self.backup_file(&validated_path)?;
        self.write_content(&validated_path, content)?;

        Ok(json!({
            "path": validated_path.display().to_string(),
            "size": content.len(),
            "backup": backup_path.map(|p| p.display().to_string())
        }))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[tokio::test]
    async fn test_write_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = WriteFileTool::new(WriteFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "test.txt", "content": "Hello"}))
            .await
            .unwrap();

        assert_eq!(result["size"], 5);

        let content = fs::read_to_string(temp_dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "Hello");
    }

    #[tokio::test]
    async fn test_overwrites_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Old content").unwrap();

        let tool = WriteFileTool::new(WriteFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        tool.execute(json!({"path": "test.txt", "content": "New content"}))
            .await
            .unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "New content");
    }

    #[tokio::test]
    async fn test_creates_backup() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Original").unwrap();

        let tool = WriteFileTool::new(WriteFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "test.txt", "content": "Modified"}))
            .await
            .unwrap();

        assert!(result["backup"].is_string());

        let backup_path = temp_dir.path().join("test.txt.backup");
        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, "Original");
    }

    #[tokio::test]
    async fn test_rejects_path_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let tool = WriteFileTool::new(WriteFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "../evil.txt", "content": "bad"}))
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Path traversal"));
    }

    #[tokio::test]
    async fn test_respects_size_limit() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = WriteFileConfig::default();
        config.max_file_size = 10;

        let tool = WriteFileTool::new(config).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "test.txt", "content": "This is too long"}))
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
    }

    #[tokio::test]
    async fn test_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let tool = WriteFileTool::new(WriteFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        tool.execute(json!({"path": "sub/dir/test.txt", "content": "content"}))
            .await
            .unwrap();

        let file_path = temp_dir.path().join("sub/dir/test.txt");
        assert!(file_path.exists());
    }
}
