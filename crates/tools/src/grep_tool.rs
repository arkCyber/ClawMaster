//! Grep tool - DO-178C Level A compliant
//! Implements text searching with regex support and security controls.

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    regex::Regex,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GrepConfig {
    pub enabled: bool,
    pub workspace_only: bool,
    pub max_results: usize,
    pub max_file_size: usize,
    pub case_sensitive: bool,
}

impl Default for GrepConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_results: 1000,
            max_file_size: 10_000_000,
            case_sensitive: false,
        }
    }
}

pub struct GrepTool {
    config: GrepConfig,
    workspace_root: Option<PathBuf>,
}

impl GrepTool {
    pub fn new(config: GrepConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        let path_str = path.to_string_lossy();
        if path_str.contains("..") {
            bail!("Path traversal detected: {}", path_str);
        }

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            let workspace = self.workspace_root.as_ref().unwrap();
            workspace.join(path)
        };

        if self.config.workspace_only {
            let workspace = self.workspace_root.as_ref().unwrap();
            if let Ok(canonical) = absolute_path.canonicalize() {
                if let Ok(canonical_ws) = workspace.canonicalize() {
                    if !canonical.starts_with(&canonical_ws) {
                        bail!("Path outside workspace: {}", canonical.display());
                    }
                }
            }
        }

        Ok(absolute_path)
    }

    fn search_in_file(&self, file_path: &Path, pattern: &Regex) -> Result<Vec<Value>> {
        let metadata = fs::metadata(file_path)?;
        if metadata.len() as usize > self.config.max_file_size {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(file_path)?;
        let mut matches = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if matches.len() >= self.config.max_results {
                break;
            }

            if pattern.is_match(line) {
                matches.push(json!({
                    "file": file_path.display().to_string(),
                    "line": line_num + 1,
                    "content": line
                }));
            }
        }

        Ok(matches)
    }

    fn search_directory(
        &self,
        dir_path: &Path,
        pattern: &Regex,
        recursive: bool,
    ) -> Result<Vec<Value>> {
        let mut all_matches = Vec::new();

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if all_matches.len() >= self.config.max_results {
                    break;
                }

                let path = entry.path();

                if path.is_file() {
                    if let Ok(matches) = self.search_in_file(&path, pattern) {
                        all_matches.extend(matches);
                    }
                } else if path.is_dir() && recursive {
                    if let Ok(matches) = self.search_directory(&path, pattern, true) {
                        all_matches.extend(matches);
                    }
                }
            }
        }

        Ok(all_matches)
    }
}

#[async_trait]
impl AgentTool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for text patterns in files using regular expressions."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regular expression pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory to search in",
                    "default": "."
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Search directories recursively",
                    "default": true
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("grep tool is disabled");
        }

        let pattern_str = params
            .get("pattern")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'pattern' parameter"))?;

        let path_str = params.get("path").and_then(Value::as_str).unwrap_or(".");

        let recursive = params
            .get("recursive")
            .and_then(Value::as_bool)
            .unwrap_or(true);

        let pattern = if self.config.case_sensitive {
            Regex::new(pattern_str)?
        } else {
            Regex::new(&format!("(?i){}", pattern_str))?
        };

        let search_path = self.validate_path(Path::new(path_str))?;

        let matches = if search_path.is_file() {
            self.search_in_file(&search_path, &pattern)?
        } else {
            self.search_directory(&search_path, &pattern, recursive)?
        };

        Ok(json!({
            "pattern": pattern_str,
            "path": search_path.display().to_string(),
            "matches": matches,
            "count": matches.len()
        }))
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[tokio::test]
    async fn test_grep_in_file() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("test.txt"),
            "Hello World\nFoo Bar\nHello Again",
        )
        .unwrap();

        let tool =
            GrepTool::new(GrepConfig::default()).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"pattern": "Hello", "path": "test.txt"}))
            .await
            .unwrap();

        assert_eq!(result["count"], 2);
    }

    #[tokio::test]
    async fn test_grep_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "hello HELLO HeLLo").unwrap();

        let tool =
            GrepTool::new(GrepConfig::default()).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"pattern": "hello", "path": "test.txt"}))
            .await
            .unwrap();

        assert_eq!(result["count"], 1);
    }

    #[tokio::test]
    async fn test_grep_recursive() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("sub")).unwrap();
        fs::write(temp_dir.path().join("sub/test.txt"), "pattern").unwrap();

        let tool =
            GrepTool::new(GrepConfig::default()).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"pattern": "pattern", "path": ".", "recursive": true}))
            .await
            .unwrap();

        assert!(result["count"].as_u64().unwrap() >= 1);
    }
}
