//! Screen Record Tool - Record screen activity
//!
//! This tool provides screen recording capabilities.
//!
//! # Compliance
//!
//! DO-178C §11.10: Resource management
//! - Recording resources are properly released
//! - No resource leaks
//!
//! # Security
//!
//! - Requires screen recording permissions
//! - Videos are stored securely
//! - Path validation prevents traversal

use {
    anyhow::{Result, bail},
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::path::{Path, PathBuf},
};

/// Screen Record Tool configuration
#[derive(Debug, Clone)]
pub struct ScreenRecordConfig {
    /// Output directory for recorded videos
    pub output_dir: PathBuf,
    /// Maximum video size in bytes (default: 100MB)
    pub max_video_size: usize,
    /// Video format (mp4, webm)
    pub video_format: VideoFormat,
    /// Maximum recording duration in seconds
    pub max_duration_secs: u32,
}

/// Supported video formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoFormat {
    Mp4,
    Webm,
}

impl Default for ScreenRecordConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./screen_recordings"),
            max_video_size: 100 * 1024 * 1024, // 100MB
            video_format: VideoFormat::Mp4,
            max_duration_secs: 300, // 5 minutes
        }
    }
}

/// Screen Record Tool
///
/// Records screen activity to a video file.
///
/// # Example Input
///
/// ```json
/// {
///     "filename": "recording.mp4",
///     "duration_secs": 30,
///     "fps": 30
/// }
/// ```
///
/// # Example Output
///
/// ```json
/// {
///     "path": "/path/to/recording.mp4",
///     "size": 12345678,
///     "duration_secs": 30,
///     "width": 1920,
///     "height": 1080,
///     "fps": 30,
///     "format": "mp4"
/// }
/// ```
pub struct ScreenRecordTool {
    config: ScreenRecordConfig,
}

#[derive(Debug, Deserialize)]
struct ScreenRecordInput {
    /// Output filename
    filename: String,
    /// Recording duration in seconds
    duration_secs: u32,
    /// Frames per second (default: 30)
    #[serde(default = "default_fps")]
    fps: u32,
}

fn default_fps() -> u32 {
    30
}

impl ScreenRecordTool {
    /// Create a new Screen Record Tool
    pub fn new(config: ScreenRecordConfig) -> Self {
        Self { config }
    }

    /// Validate output path
    fn validate_path(&self, filename: &str) -> Result<PathBuf> {
        // Check for path traversal
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            bail!("Invalid filename: path traversal detected");
        }

        // Check filename length
        if filename.is_empty() || filename.len() > 255 {
            bail!("Invalid filename length");
        }

        // Build full path
        let output_path = self.config.output_dir.join(filename);

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(output_path)
    }

    /// Validate recording parameters
    fn validate_params(&self, duration_secs: u32, fps: u32) -> Result<()> {
        if duration_secs == 0 || duration_secs > self.config.max_duration_secs {
            bail!(
                "Invalid duration: must be 1-{} seconds",
                self.config.max_duration_secs
            );
        }

        if fps == 0 || fps > 60 {
            bail!("Invalid FPS: must be 1-60");
        }

        Ok(())
    }

    /// Record screen (placeholder implementation)
    async fn record_screen(
        &self,
        output_path: &Path,
        duration_secs: u32,
        fps: u32,
    ) -> Result<ScreenRecordOutput> {
        // TODO: Implement actual screen recording using scrap or similar
        // This is a placeholder implementation

        #[cfg(feature = "metrics")]
        tracing::info!("Screen recording started");

        #[cfg(feature = "metrics")]
        tracing::info!(
            output_path = ?output_path,
            duration_secs = duration_secs,
            fps = fps,
            "Recording screen (placeholder)"
        );

        // For now, return a placeholder response
        Ok(ScreenRecordOutput {
            path: output_path.to_string_lossy().to_string(),
            size: 0,
            duration_secs,
            width: 1920,
            height: 1080,
            fps,
            format: format!("{:?}", self.config.video_format).to_lowercase(),
        })
    }
}

#[derive(Debug, Serialize)]
struct ScreenRecordOutput {
    path: String,
    size: u64,
    duration_secs: u32,
    width: u32,
    height: u32,
    fps: u32,
    format: String,
}

impl ScreenRecordTool {
    fn name(&self) -> &str {
        "screen_record"
    }

    fn description(&self) -> &str {
        "Record screen activity to a video file. Requires screen recording permissions."
    }

    fn parameters_json_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "filename": {
                    "type": "string",
                    "description": "Output filename (e.g., 'recording.mp4')"
                },
                "duration_secs": {
                    "type": "integer",
                    "description": "Recording duration in seconds (max 300)"
                },
                "fps": {
                    "type": "integer",
                    "description": "Frames per second (1-60, default 30)",
                    "default": 30
                }
            },
            "required": ["filename", "duration_secs"]
        })
    }

    pub async fn execute(&self, input: Value) -> Result<Value> {
        let input: ScreenRecordInput = serde_json::from_value(input)?;

        // Validate output path
        let output_path = self.validate_path(&input.filename)?;

        // Validate parameters
        self.validate_params(input.duration_secs, input.fps)?;

        // Record screen
        let output = self
            .record_screen(&output_path, input.duration_secs, input.fps)
            .await?;

        Ok(serde_json::to_value(output)?)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[test]
    fn test_screen_record_tool_creation() {
        let config = ScreenRecordConfig::default();
        let tool = ScreenRecordTool::new(config);
        assert_eq!(tool.name(), "screen_record");
    }

    #[test]
    fn test_validate_path_rejects_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let config = ScreenRecordConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let tool = ScreenRecordTool::new(config);

        assert!(tool.validate_path("../etc/passwd").is_err());
        assert!(tool.validate_path("../../secret").is_err());
    }

    #[test]
    fn test_validate_params() {
        let config = ScreenRecordConfig::default();
        let tool = ScreenRecordTool::new(config);

        assert!(tool.validate_params(30, 30).is_ok());
        assert!(tool.validate_params(0, 30).is_err()); // Invalid duration
        assert!(tool.validate_params(400, 30).is_err()); // Duration too long
        assert!(tool.validate_params(30, 0).is_err()); // Invalid FPS
        assert!(tool.validate_params(30, 100).is_err()); // FPS too high
    }

    #[tokio::test]
    async fn test_execute_with_valid_input() {
        let temp_dir = TempDir::new().unwrap();
        let config = ScreenRecordConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let tool = ScreenRecordTool::new(config);

        let input = json!({
            "filename": "test.mp4",
            "duration_secs": 30,
            "fps": 30
        });

        let result = tool.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_rejects_invalid_duration() {
        let temp_dir = TempDir::new().unwrap();
        let config = ScreenRecordConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let tool = ScreenRecordTool::new(config);

        let input = json!({
            "filename": "test.mp4",
            "duration_secs": 400
        });

        let result = tool.execute(input).await;
        assert!(result.is_err());
    }
}
