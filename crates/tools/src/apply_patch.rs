//! Apply code patches to files.
//!
//! Implements unified diff patch application with safety checks and validation.
//! Supports both workspace-only mode and full filesystem access.

use {
    anyhow::{Result, bail, Context},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

/// Apply patch tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplyPatchConfig {
    pub enabled: bool,
    pub workspace_only: bool,
    pub max_patch_size: usize,
    pub backup_before_patch: bool,
}

impl Default for ApplyPatchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workspace_only: true,
            max_patch_size: 1_000_000, // 1MB
            backup_before_patch: true,
        }
    }
}

/// Apply patch tool.
pub struct ApplyPatchTool {
    config: ApplyPatchConfig,
    workspace_root: Option<PathBuf>,
}

impl ApplyPatchTool {
    pub fn new(config: ApplyPatchConfig) -> Self {
        Self {
            config,
            workspace_root: std::env::current_dir().ok(),
        }
    }

    pub fn with_workspace_root(mut self, root: PathBuf) -> Self {
        self.workspace_root = Some(root);
        self
    }

    /// Validate that the file path is within workspace if workspace_only is enabled.
    fn validate_path(&self, file_path: &Path) -> Result<PathBuf> {
        let absolute_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            let workspace = self.workspace_root.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;
            workspace.join(file_path)
        };

        if self.config.workspace_only {
            let workspace = self.workspace_root.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Workspace root not set"))?;
            
            let canonical_path = absolute_path.canonicalize()
                .with_context(|| format!("Failed to canonicalize path: {}", absolute_path.display()))?;
            let canonical_workspace = workspace.canonicalize()
                .with_context(|| format!("Failed to canonicalize workspace: {}", workspace.display()))?;

            if !canonical_path.starts_with(&canonical_workspace) {
                bail!("Path '{}' is outside workspace '{}'", 
                    canonical_path.display(), canonical_workspace.display());
            }
        }

        Ok(absolute_path)
    }

    /// Parse a unified diff patch.
    fn parse_patch(&self, patch_content: &str) -> Result<Vec<PatchHunk>> {
        if patch_content.len() > self.config.max_patch_size {
            bail!("Patch size {} exceeds maximum {}", 
                patch_content.len(), self.config.max_patch_size);
        }

        let mut hunks = Vec::new();
        let mut current_hunk: Option<PatchHunk> = None;
        let mut in_hunk = false;

        for line in patch_content.lines() {
            if line.starts_with("@@") {
                // Save previous hunk if exists
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 3 {
                    bail!("Invalid hunk header: {}", line);
                }

                let old_range = parts[1].trim_start_matches('-');
                let new_range = parts[2].trim_start_matches('+');

                let (old_start, old_count) = parse_range(old_range)?;
                let (new_start, _new_count) = parse_range(new_range)?;

                current_hunk = Some(PatchHunk {
                    old_start,
                    old_count,
                    new_start,
                    lines: Vec::new(),
                });
                in_hunk = true;
            } else if in_hunk {
                if let Some(ref mut hunk) = current_hunk {
                    if line.starts_with('+') || line.starts_with('-') || line.starts_with(' ') {
                        hunk.lines.push(line.to_string());
                    }
                }
            }
        }

        // Save last hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        if hunks.is_empty() {
            bail!("No valid hunks found in patch");
        }

        Ok(hunks)
    }

    /// Apply hunks to file content.
    fn apply_hunks(&self, original: &str, hunks: &[PatchHunk]) -> Result<String> {
        let mut lines: Vec<String> = original.lines().map(|s| s.to_string()).collect();

        // Apply hunks in reverse order to maintain line numbers
        for hunk in hunks.iter().rev() {
            let start_idx = (hunk.old_start - 1) as usize;
            
            if start_idx > lines.len() {
                bail!("Hunk start line {} is beyond file length {}", 
                    hunk.old_start, lines.len());
            }

            // Verify context lines match
            let mut original_idx = start_idx;
            let mut new_lines = Vec::new();

            for patch_line in &hunk.lines {
                if patch_line.starts_with(' ') {
                    // Context line - verify it matches
                    let expected = &patch_line[1..];
                    if original_idx >= lines.len() {
                        bail!("Context line beyond file end");
                    }
                    if lines[original_idx] != expected {
                        bail!("Context mismatch at line {}: expected '{}', found '{}'",
                            original_idx + 1, expected, lines[original_idx]);
                    }
                    new_lines.push(expected.to_string());
                    original_idx += 1;
                } else if patch_line.starts_with('-') {
                    // Deletion - verify line matches
                    let expected = &patch_line[1..];
                    if original_idx >= lines.len() {
                        bail!("Deletion line beyond file end");
                    }
                    if lines[original_idx] != expected {
                        bail!("Deletion mismatch at line {}: expected '{}', found '{}'",
                            original_idx + 1, expected, lines[original_idx]);
                    }
                    original_idx += 1;
                } else if patch_line.starts_with('+') {
                    // Addition
                    new_lines.push(patch_line[1..].to_string());
                }
            }

            // Replace lines
            let end_idx = original_idx;
            lines.splice(start_idx..end_idx, new_lines);
        }

        Ok(lines.join("\n"))
    }

    /// Apply patch to a file.
    pub fn apply_patch_to_file(&self, file_path: &Path, patch_content: &str) -> Result<ApplyPatchResult> {
        let validated_path = self.validate_path(file_path)?;

        // Read original file
        let original_content = fs::read_to_string(&validated_path)
            .with_context(|| format!("Failed to read file: {}", validated_path.display()))?;

        // Backup if enabled
        let backup_path = if self.config.backup_before_patch {
            let backup = validated_path.with_extension("bak");
            fs::write(&backup, &original_content)
                .with_context(|| format!("Failed to create backup: {}", backup.display()))?;
            Some(backup)
        } else {
            None
        };

        // Parse and apply patch
        let hunks = self.parse_patch(patch_content)?;
        let patched_content = self.apply_hunks(&original_content, &hunks)?;

        // Write patched content
        fs::write(&validated_path, &patched_content)
            .with_context(|| format!("Failed to write patched file: {}", validated_path.display()))?;

        Ok(ApplyPatchResult {
            success: true,
            file_path: validated_path.display().to_string(),
            backup_path: backup_path.map(|p| p.display().to_string()),
            hunks_applied: hunks.len(),
            lines_added: hunks.iter().flat_map(|h| &h.lines).filter(|l| l.starts_with('+')).count(),
            lines_removed: hunks.iter().flat_map(|h| &h.lines).filter(|l| l.starts_with('-')).count(),
        })
    }
}

/// Parse a range string like "10,5" into (start, count).
fn parse_range(range: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = range.split(',').collect();
    let start = parts[0].parse::<u32>()
        .with_context(|| format!("Invalid range start: {}", parts[0]))?;
    let count = if parts.len() > 1 {
        parts[1].parse::<u32>()
            .with_context(|| format!("Invalid range count: {}", parts[1]))?
    } else {
        1
    };
    Ok((start, count))
}

/// A single hunk in a patch.
#[derive(Debug, Clone)]
struct PatchHunk {
    old_start: u32,
    old_count: u32,
    new_start: u32,
    lines: Vec<String>,
}

/// Result of applying a patch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyPatchResult {
    pub success: bool,
    pub file_path: String,
    pub backup_path: Option<String>,
    pub hunks_applied: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
}

#[async_trait]
impl AgentTool for ApplyPatchTool {
    fn name(&self) -> &str {
        "apply_patch"
    }

    fn description(&self) -> &str {
        "Apply a unified diff patch to a file with safety checks and automatic backup"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Path to the file to patch (relative to workspace or absolute)"
                },
                "patch": {
                    "type": "string",
                    "description": "Unified diff patch content"
                }
            },
            "required": ["file_path", "patch"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("apply_patch tool is disabled");
        }

        let file_path = params.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'file_path' parameter"))?;

        let patch = params.get("patch")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'patch' parameter"))?;

        let result = self.apply_patch_to_file(Path::new(file_path), patch)?;

        Ok(serde_json::to_value(result)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("10,5").unwrap(), (10, 5));
        assert_eq!(parse_range("1").unwrap(), (1, 1));
        assert_eq!(parse_range("100,20").unwrap(), (100, 20));
    }

    #[test]
    fn test_parse_patch() {
        let config = ApplyPatchConfig::default();
        let tool = ApplyPatchTool::new(config);

        let patch = r#"@@ -1,3 +1,3 @@
 line1
-line2
+line2_modified
 line3
"#;

        let hunks = tool.parse_patch(patch).unwrap();
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[0].old_count, 3);
        assert_eq!(hunks[0].new_start, 1);
        assert_eq!(hunks[0].lines.len(), 4); // 包含空格前缀的上下文行
    }

    #[test]
    fn test_apply_simple_patch() {
        let config = ApplyPatchConfig::default();
        let tool = ApplyPatchTool::new(config);

        let original = "line1\nline2\nline3";
        let patch = r#"@@ -1,3 +1,3 @@
 line1
-line2
+line2_modified
 line3
"#;

        let hunks = tool.parse_patch(patch).unwrap();
        let result = tool.apply_hunks(original, &hunks).unwrap();

        assert_eq!(result, "line1\nline2_modified\nline3");
    }

    #[tokio::test]
    async fn test_apply_patch_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "line1\nline2\nline3").unwrap();

        let mut config = ApplyPatchConfig::default();
        config.workspace_only = false;
        let tool = ApplyPatchTool::new(config);

        let patch = r#"@@ -1,3 +1,3 @@
 line1
-line2
+line2_modified
 line3
"#;

        let result = tool.apply_patch_to_file(&test_file, patch).unwrap();

        assert!(result.success);
        assert_eq!(result.hunks_applied, 1);
        assert_eq!(result.lines_added, 1);
        assert_eq!(result.lines_removed, 1);

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "line1\nline2_modified\nline3");
    }

    #[tokio::test]
    async fn test_tool_execute() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "line1\nline2\nline3").unwrap();

        let mut config = ApplyPatchConfig::default();
        config.workspace_only = false;
        let tool = ApplyPatchTool::new(config);

        let patch = r#"@@ -1,3 +1,3 @@
 line1
-line2
+line2_modified
 line3
"#;

        let params = json!({
            "file_path": test_file.display().to_string(),
            "patch": patch
        });

        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["success"], true);
        assert_eq!(result["hunks_applied"], 1);
    }
}
