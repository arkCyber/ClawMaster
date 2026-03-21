//! List directory tool - DO-178C Level A compliant

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListDirectoryConfig {
    pub enabled: bool,
    pub workspace_only: bool,
    pub max_depth: usize,
    pub max_entries: usize,
    pub show_hidden: bool,
}

impl Default for ListDirectoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_depth: 10,
            max_entries: 1000,
            show_hidden: false,
        }
    }
}

pub struct ListDirectoryTool {
    config: ListDirectoryConfig,
    workspace_root: Option<PathBuf>,
}

impl ListDirectoryTool {
    pub fn new(config: ListDirectoryConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    fn validate_path(&self, dir_path: &Path) -> Result<PathBuf> {
        let path_str = dir_path.to_string_lossy();
        if path_str.contains("..") || path_str.contains("~") {
            bail!("Path traversal detected: {}", path_str);
        }

        let absolute_path = if dir_path.is_absolute() {
            dir_path.to_path_buf()
        } else {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;
            workspace.join(dir_path)
        };

        if self.config.workspace_only {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;

            let canonical_path = absolute_path.canonicalize()?;
            let canonical_workspace = workspace.canonicalize()?;

            if !canonical_path.starts_with(&canonical_workspace) {
                bail!(
                    "Path '{}' is outside workspace '{}'",
                    canonical_path.display(),
                    canonical_workspace.display()
                );
            }
        }

        if !absolute_path.exists() {
            bail!("Directory does not exist: {}", absolute_path.display());
        }

        if !absolute_path.is_dir() {
            bail!("Path is not a directory: {}", absolute_path.display());
        }

        Ok(absolute_path)
    }

    fn list_entries(&self, dir_path: &Path, recursive: bool, current_depth: usize) -> Result<Vec<Value>> {
        if current_depth > self.config.max_depth {
            return Ok(vec![]);
        }

        let mut entries = Vec::new();
        let read_dir = fs::read_dir(dir_path)
            .with_context(|| format!("Failed to read directory: {}", dir_path.display()))?;

        for entry in read_dir {
            if entries.len() >= self.config.max_entries {
                break;
            }

            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if !self.config.show_hidden && file_name.starts_with('.') {
                continue;
            }

            let metadata = entry.metadata()?;
            let is_dir = metadata.is_dir();
            let size = if is_dir { None } else { Some(metadata.len()) };

            entries.push(json!({
                "name": file_name,
                "path": path.display().to_string(),
                "type": if is_dir { "directory" } else { "file" },
                "size": size
            }));

            if recursive && is_dir {
                let sub_entries = self.list_entries(&path, true, current_depth + 1)?;
                entries.extend(sub_entries);
            }
        }

        Ok(entries)
    }
}

#[async_trait]
impl AgentTool for ListDirectoryTool {
    fn name(&self) -> &str {
        "list_directory"
    }

    fn description(&self) -> &str {
        "List the contents of a directory."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the directory to list"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Whether to list subdirectories recursively",
                    "default": false
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("list_directory tool is disabled");
        }

        let path_str = params
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let recursive = params
            .get("recursive")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let dir_path = Path::new(path_str);
        let validated_path = self.validate_path(dir_path)?;

        let entries = self.list_entries(&validated_path, recursive, 0)?;

        Ok(json!({
            "path": validated_path.display().to_string(),
            "entries": entries,
            "count": entries.len()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_list_simple_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content").unwrap();

        let tool = ListDirectoryTool::new(ListDirectoryConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "."}))
            .await
            .unwrap();

        assert_eq!(result["count"], 2);
        let entries = result["entries"].as_array().unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_list_recursive() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("subdir/file.txt"), "content").unwrap();

        let tool = ListDirectoryTool::new(ListDirectoryConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": ".", "recursive": true}))
            .await
            .unwrap();

        assert!(result["count"].as_u64().unwrap() >= 2);
    }

    #[tokio::test]
    async fn test_hides_hidden_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join(".hidden"), "content").unwrap();
        fs::write(temp_dir.path().join("visible.txt"), "content").unwrap();

        let tool = ListDirectoryTool::new(ListDirectoryConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "."}))
            .await
            .unwrap();

        assert_eq!(result["count"], 1);
    }
}
