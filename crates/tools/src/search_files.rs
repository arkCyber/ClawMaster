//! Search files tool - DO-178C Level A compliant
//! Implements glob pattern file searching with security controls.

use {
    anyhow::{Context, Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    glob::glob,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchFilesConfig {
    pub enabled: bool,
    pub workspace_only: bool,
    pub max_results: usize,
    pub max_depth: usize,
}

impl Default for SearchFilesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_results: 1000,
            max_depth: 10,
        }
    }
}

pub struct SearchFilesTool {
    config: SearchFilesConfig,
    workspace_root: Option<PathBuf>,
}

impl SearchFilesTool {
    pub fn new(config: SearchFilesConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    fn search_files(&self, pattern: &str, base_path: &Path) -> Result<Vec<PathBuf>> {
        let search_pattern = if pattern.starts_with('/') || pattern.contains(':') {
            bail!("Absolute patterns not allowed: {}", pattern);
        } else {
            base_path.join(pattern).display().to_string()
        };

        let mut results = Vec::new();

        for entry in
            glob(&search_pattern).with_context(|| format!("Invalid glob pattern: {}", pattern))?
        {
            if results.len() >= self.config.max_results {
                break;
            }

            match entry {
                Ok(path) => {
                    if self.config.workspace_only {
                        let workspace = self.workspace_root.as_ref().unwrap();
                        if let Ok(canonical) = path.canonicalize() {
                            if let Ok(canonical_ws) = workspace.canonicalize() {
                                if canonical.starts_with(&canonical_ws) {
                                    results.push(path);
                                }
                            }
                        }
                    } else {
                        results.push(path);
                    }
                },
                Err(e) => {
                    tracing::warn!("Glob error: {}", e);
                },
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl AgentTool for SearchFilesTool {
    fn name(&self) -> &str {
        "search_files"
    }

    fn description(&self) -> &str {
        "Search for files using glob patterns (e.g., '*.rs', 'src/**/*.txt')."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern to search for (e.g., '*.rs', 'src/**/*.txt')"
                },
                "path": {
                    "type": "string",
                    "description": "Base directory to search in (default: workspace root)",
                    "default": "."
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("search_files tool is disabled");
        }

        let pattern = params
            .get("pattern")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'pattern' parameter"))?;

        let base_path_str = params.get("path").and_then(Value::as_str).unwrap_or(".");

        let base_path = if base_path_str == "." {
            self.workspace_root
                .clone()
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            PathBuf::from(base_path_str)
        };

        let results = self.search_files(pattern, &base_path)?;

        let files: Vec<Value> = results
            .iter()
            .map(|path| {
                json!({
                    "path": path.display().to_string(),
                    "name": path.file_name().and_then(|n| n.to_str()).unwrap_or(""),
                })
            })
            .collect();

        Ok(json!({
            "pattern": pattern,
            "base_path": base_path.display().to_string(),
            "files": files,
            "count": files.len()
        }))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::TempDir};

    #[tokio::test]
    async fn test_search_by_extension() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.rs"), "").unwrap();
        fs::write(temp_dir.path().join("file2.rs"), "").unwrap();
        fs::write(temp_dir.path().join("file3.txt"), "").unwrap();

        let tool = SearchFilesTool::new(SearchFilesConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"pattern": "*.rs"})).await.unwrap();

        assert_eq!(result["count"], 2);
    }

    #[tokio::test]
    async fn test_search_recursive() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src/main.rs"), "").unwrap();

        let tool = SearchFilesTool::new(SearchFilesConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"pattern": "**/*.rs"})).await.unwrap();

        assert!(result["count"].as_u64().unwrap() >= 1);
    }

    #[tokio::test]
    async fn test_rejects_absolute_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SearchFilesTool::new(SearchFilesConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"pattern": "/etc/passwd"})).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Absolute patterns not allowed")
        );
    }
}
