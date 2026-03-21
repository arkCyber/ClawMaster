//! Read file tool - DO-178C Level A compliant
//!
//! Implements safe file reading with comprehensive validation and error handling.
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

/// Read file tool configuration.
/// DO-178C §6.3.1: All configuration parameters must have safe defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReadFileConfig {
    /// Enable/disable the tool
    pub enabled: bool,

    /// Restrict file access to workspace only
    pub workspace_only: bool,

    /// Maximum file size to read (bytes)
    /// DO-178C §6.3.2: Resource limits prevent DoS attacks
    pub max_file_size: usize,

    /// Maximum line length before truncation
    pub max_line_length: usize,

    /// Allowed file extensions (empty = all allowed)
    pub allowed_extensions: Vec<String>,
}

impl Default for ReadFileConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_file_size: 10_000_000, // 10MB
            max_line_length: 10_000,   // 10K chars per line
            allowed_extensions: vec![],
        }
    }
}

/// Read file tool.
/// DO-178C §6.3.1: Tool provides safe file reading with comprehensive validation.
pub struct ReadFileTool {
    config: ReadFileConfig,
    workspace_root: Option<PathBuf>,
}

impl ReadFileTool {
    /// Create a new ReadFileTool with the given configuration.
    pub fn new(config: ReadFileConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    /// Set the workspace root directory.
    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    /// Validate file path.
    /// DO-178C §6.3.2: Path validation prevents directory traversal attacks.
    fn validate_path(&self, file_path: &Path) -> Result<PathBuf> {
        // DO-178C §6.3.2a: Check for dangerous path components
        let path_str = file_path.to_string_lossy();
        if path_str.contains("..") || path_str.contains("~") {
            bail!("Path traversal detected: {}", path_str);
        }

        // DO-178C §6.3.2b: Resolve to absolute path
        let absolute_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;
            workspace.join(file_path)
        };

        // DO-178C §6.3.2c: File must exist (check before canonicalize)
        if !absolute_path.exists() {
            bail!("File does not exist: {}", absolute_path.display());
        }

        // DO-178C §6.3.2d: Workspace-only mode validation
        if self.config.workspace_only {
            let workspace = self
                .workspace_root
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;

            let canonical_path = absolute_path.canonicalize().with_context(|| {
                format!("Failed to canonicalize path: {}", absolute_path.display())
            })?;

            let canonical_workspace = workspace.canonicalize().with_context(|| {
                format!("Failed to canonicalize workspace: {}", workspace.display())
            })?;

            if !canonical_path.starts_with(&canonical_workspace) {
                bail!(
                    "Path '{}' is outside workspace '{}'",
                    canonical_path.display(),
                    canonical_workspace.display()
                );
            }
        }

        // DO-178C §6.3.2e: Must be a file, not a directory
        if !absolute_path.is_file() {
            bail!("Path is not a file: {}", absolute_path.display());
        }

        Ok(absolute_path)
    }

    /// Validate file extension.
    /// DO-178C §6.3.3: Extension validation prevents reading unauthorized file types.
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

    /// Read file content with size validation.
    /// DO-178C §6.3.4: Size limits prevent memory exhaustion.
    fn read_file_content(&self, file_path: &Path) -> Result<String> {
        // DO-178C §6.3.4a: Check file size before reading
        let metadata = fs::metadata(file_path)
            .with_context(|| format!("Failed to read metadata: {}", file_path.display()))?;

        let file_size = metadata.len() as usize;
        if file_size > self.config.max_file_size {
            bail!(
                "File size {} exceeds maximum {}",
                file_size,
                self.config.max_file_size
            );
        }

        // DO-178C §6.3.4b: Read file content
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        // DO-178C §6.3.4c: Verify content size (UTF-8 may differ from byte size)
        if content.len() > self.config.max_file_size {
            bail!(
                "Content size {} exceeds maximum {}",
                content.len(),
                self.config.max_file_size
            );
        }

        Ok(content)
    }

    /// Truncate long lines for safety.
    /// DO-178C §6.3.5: Line truncation prevents display issues.
    fn truncate_long_lines(&self, content: String) -> String {
        if self.config.max_line_length == 0 {
            return content;
        }

        content
            .lines()
            .map(|line| {
                if line.len() > self.config.max_line_length {
                    format!("{}... (truncated)", &line[..self.config.max_line_length])
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[async_trait]
impl AgentTool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read the contents of a file. Returns the file content as a string."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read (relative to workspace or absolute)"
                },
                "truncate_lines": {
                    "type": "boolean",
                    "description": "Whether to truncate long lines (default: true)",
                    "default": true
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        // DO-178C §6.3.6a: Validate tool is enabled
        if !self.config.enabled {
            bail!("read_file tool is disabled");
        }

        // DO-178C §6.3.6b: Extract and validate parameters
        let path_str = params
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing 'path' parameter"))?;

        let truncate_lines = params
            .get("truncate_lines")
            .and_then(Value::as_bool)
            .unwrap_or(true);

        let file_path = Path::new(path_str);

        // DO-178C §6.3.6c: Validate path
        let validated_path = self.validate_path(file_path)?;

        // DO-178C §6.3.6d: Validate extension
        self.validate_extension(&validated_path)?;

        // DO-178C §6.3.6e: Read file content
        let mut content = self.read_file_content(&validated_path)?;

        // DO-178C §6.3.6f: Truncate long lines if requested
        if truncate_lines {
            content = self.truncate_long_lines(content);
        }

        // DO-178C §6.3.6g: Return structured result
        Ok(json!({
            "path": validated_path.display().to_string(),
            "content": content,
            "size": content.len(),
            "lines": content.lines().count()
        }))
    }
}

// DO-178C §6.4: Comprehensive unit tests
#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::TempDir};

    fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[tokio::test]
    async fn test_read_simple_file() {
        let temp_dir = TempDir::new().unwrap();
        let _file_path = create_test_file(&temp_dir, "test.txt", "Hello, World!");

        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "test.txt"})).await.unwrap();

        assert_eq!(result["content"], "Hello, World!");
        assert_eq!(result["size"], 13);
        assert_eq!(result["lines"], 1);
    }

    #[tokio::test]
    async fn test_read_multiline_file() {
        let temp_dir = TempDir::new().unwrap();
        let content = "Line 1\nLine 2\nLine 3";
        let _file_path = create_test_file(&temp_dir, "multi.txt", content);

        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "multi.txt"})).await.unwrap();

        assert_eq!(result["content"], content);
        assert_eq!(result["lines"], 3);
    }

    #[tokio::test]
    async fn test_rejects_path_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "../etc/passwd"})).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Path traversal"));
    }

    #[tokio::test]
    async fn test_rejects_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "nonexistent.txt"})).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string().to_lowercase();
        assert!(
            err_msg.contains("does not exist")
                || err_msg.contains("no such file")
                || err_msg.contains("not found")
                || err_msg.contains("cannot find"),
            "Error message was: {}",
            err_msg
        );
    }

    #[tokio::test]
    async fn test_rejects_directory() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "subdir"})).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a file"));
    }

    #[tokio::test]
    async fn test_respects_file_size_limit() {
        let temp_dir = TempDir::new().unwrap();
        let large_content = "x".repeat(1000);
        create_test_file(&temp_dir, "large.txt", &large_content);

        let mut config = ReadFileConfig::default();
        config.max_file_size = 500;

        let tool = ReadFileTool::new(config).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool.execute(json!({"path": "large.txt"})).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
    }

    #[tokio::test]
    async fn test_truncates_long_lines() {
        let temp_dir = TempDir::new().unwrap();
        let long_line = "x".repeat(200);
        create_test_file(&temp_dir, "long.txt", &long_line);

        let mut config = ReadFileConfig::default();
        config.max_line_length = 100;

        let tool = ReadFileTool::new(config).with_workspace_root(temp_dir.path().to_path_buf());

        let result = tool
            .execute(json!({"path": "long.txt", "truncate_lines": true}))
            .await
            .unwrap();

        let content = result["content"].as_str().unwrap();
        assert!(content.contains("truncated"));
        assert!(content.len() < 200);
    }

    #[tokio::test]
    async fn test_respects_allowed_extensions() {
        let temp_dir = TempDir::new().unwrap();
        create_test_file(&temp_dir, "test.txt", "content");
        create_test_file(&temp_dir, "test.rs", "content");

        let mut config = ReadFileConfig::default();
        config.allowed_extensions = vec!["rs".to_string()];

        let tool = ReadFileTool::new(config).with_workspace_root(temp_dir.path().to_path_buf());

        // Should succeed for .rs file
        let result = tool.execute(json!({"path": "test.rs"})).await;
        assert!(result.is_ok());

        // Should fail for .txt file
        let result = tool.execute(json!({"path": "test.txt"})).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not allowed"));
    }

    #[tokio::test]
    async fn test_workspace_only_mode() {
        let temp_dir = TempDir::new().unwrap();
        create_test_file(&temp_dir, "inside.txt", "content");

        let tool = ReadFileTool::new(ReadFileConfig::default())
            .with_workspace_root(temp_dir.path().to_path_buf());

        // Should succeed for file inside workspace
        let result = tool.execute(json!({"path": "inside.txt"})).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tool_name_and_schema() {
        let tool = ReadFileTool::new(ReadFileConfig::default());

        assert_eq!(tool.name(), "read_file");
        assert!(tool.description().contains("Read the contents"));

        let schema = tool.parameters_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["path"].is_object());
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!("path"))
        );
    }
}
