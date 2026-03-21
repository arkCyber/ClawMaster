//! Image analysis and processing tool.
//!
//! Provides image analysis capabilities independent of the main model.
//! Handles sensitive image data, so should be implemented with privacy protection.

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    base64::{Engine as _, engine::general_purpose},
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::sync::Arc,
};

/// Image analysis provider trait.
#[async_trait]
pub trait ImageAnalysisProvider: Send + Sync {
    async fn analyze_image(
        &self,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<ImageAnalysisResult>;
    async fn analyze_image_url(
        &self,
        url: &str,
        prompt: Option<&str>,
    ) -> Result<ImageAnalysisResult>;
    fn supported_formats(&self) -> Vec<String>;
}

/// Image analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysisResult {
    pub description: String,
    pub objects: Vec<DetectedObject>,
    pub text: Option<String>,
    pub metadata: ImageMetadata,
}

/// Detected object in image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: Option<BoundingBox>,
}

/// Bounding box coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Image metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: usize,
}

/// Image analysis tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageToolConfig {
    pub enabled: bool,
    pub max_image_size: usize,
    pub allow_url_fetch: bool,
}

impl Default for ImageToolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_image_size: 10_000_000, // 10MB
            allow_url_fetch: true,
        }
    }
}

/// Image analysis tool.
pub struct ImageTool {
    config: ImageToolConfig,
    provider: Arc<dyn ImageAnalysisProvider>,
}

impl ImageTool {
    pub fn new(config: ImageToolConfig, provider: Arc<dyn ImageAnalysisProvider>) -> Self {
        Self { config, provider }
    }
}

#[async_trait]
impl AgentTool for ImageTool {
    fn name(&self) -> &str {
        "image"
    }

    fn description(&self) -> &str {
        "Analyze images using vision models, independent of the main conversation model"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["analyze", "formats"],
                    "description": "Action to perform: analyze image or list supported formats"
                },
                "image_url": {
                    "type": "string",
                    "description": "URL of the image to analyze (for analyze action)"
                },
                "image_base64": {
                    "type": "string",
                    "description": "Base64-encoded image data (for analyze action)"
                },
                "prompt": {
                    "type": "string",
                    "description": "Optional prompt to guide the analysis"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("Image tool is disabled");
        }

        let action = params
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' parameter"))?;

        match action {
            "analyze" => {
                let prompt = params.get("prompt").and_then(|v| v.as_str());

                if let Some(url) = params.get("image_url").and_then(|v| v.as_str()) {
                    if !self.config.allow_url_fetch {
                        bail!("URL fetching is disabled");
                    }

                    let result = self.provider.analyze_image_url(url, prompt).await?;
                    Ok(serde_json::to_value(result)?)
                } else if let Some(base64_str) = params.get("image_base64").and_then(|v| v.as_str())
                {
                    let image_data = general_purpose::STANDARD
                        .decode(base64_str)
                        .map_err(|e| anyhow::anyhow!("Invalid base64 data: {}", e))?;

                    if image_data.len() > self.config.max_image_size {
                        bail!(
                            "Image size {} exceeds maximum {}",
                            image_data.len(),
                            self.config.max_image_size
                        );
                    }

                    let result = self.provider.analyze_image(&image_data, prompt).await?;
                    Ok(serde_json::to_value(result)?)
                } else {
                    bail!("Either 'image_url' or 'image_base64' must be provided");
                }
            },
            "formats" => {
                let formats = self.provider.supported_formats();
                Ok(json!({
                    "formats": formats,
                    "count": formats.len()
                }))
            },
            _ => bail!("Invalid action: {}", action),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockImageProvider;

    #[async_trait]
    impl ImageAnalysisProvider for MockImageProvider {
        async fn analyze_image(
            &self,
            _image_data: &[u8],
            prompt: Option<&str>,
        ) -> Result<ImageAnalysisResult> {
            Ok(ImageAnalysisResult {
                description: format!(
                    "Mock analysis{}",
                    prompt
                        .map(|p| format!(" with prompt: {}", p))
                        .unwrap_or_default()
                ),
                objects: vec![DetectedObject {
                    label: "cat".to_string(),
                    confidence: 0.95,
                    bounding_box: Some(BoundingBox {
                        x: 10.0,
                        y: 20.0,
                        width: 100.0,
                        height: 150.0,
                    }),
                }],
                text: Some("Hello World".to_string()),
                metadata: ImageMetadata {
                    width: 800,
                    height: 600,
                    format: "JPEG".to_string(),
                    size_bytes: 50000,
                },
            })
        }

        async fn analyze_image_url(
            &self,
            _url: &str,
            prompt: Option<&str>,
        ) -> Result<ImageAnalysisResult> {
            self.analyze_image(&[], prompt).await
        }

        fn supported_formats(&self) -> Vec<String> {
            vec!["JPEG".to_string(), "PNG".to_string(), "GIF".to_string()]
        }
    }

    #[tokio::test]
    async fn test_analyze_base64() {
        let config = ImageToolConfig::default();
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let image_data = b"fake image data";
        let base64_data = general_purpose::STANDARD.encode(image_data);

        let params = json!({
            "action": "analyze",
            "image_base64": base64_data
        });

        let result = tool.execute(params).await.unwrap();
        assert!(
            result["description"]
                .as_str()
                .unwrap()
                .contains("Mock analysis")
        );
        assert_eq!(result["objects"][0]["label"], "cat");
    }

    #[tokio::test]
    async fn test_analyze_with_prompt() {
        let config = ImageToolConfig::default();
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let image_data = b"fake image data";
        let base64_data = general_purpose::STANDARD.encode(image_data);

        let params = json!({
            "action": "analyze",
            "image_base64": base64_data,
            "prompt": "What animals are in this image?"
        });

        let result = tool.execute(params).await.unwrap();
        assert!(
            result["description"]
                .as_str()
                .unwrap()
                .contains("What animals")
        );
    }

    #[tokio::test]
    async fn test_analyze_url() {
        let config = ImageToolConfig::default();
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let params = json!({
            "action": "analyze",
            "image_url": "https://example.com/image.jpg"
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result["description"].is_string());
    }

    #[tokio::test]
    async fn test_formats() {
        let config = ImageToolConfig::default();
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let params = json!({"action": "formats"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["count"], 3);
        assert!(
            result["formats"]
                .as_array()
                .unwrap()
                .contains(&json!("JPEG"))
        );
    }

    #[tokio::test]
    async fn test_disabled() {
        let mut config = ImageToolConfig::default();
        config.enabled = false;
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let params = json!({
            "action": "analyze",
            "image_url": "https://example.com/image.jpg"
        });

        let result = tool.execute(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disabled"));
    }

    #[tokio::test]
    async fn test_url_fetch_disabled() {
        let mut config = ImageToolConfig::default();
        config.allow_url_fetch = false;
        let provider = Arc::new(MockImageProvider);
        let tool = ImageTool::new(config, provider);

        let params = json!({
            "action": "analyze",
            "image_url": "https://example.com/image.jpg"
        });

        let result = tool.execute(params).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("URL fetching is disabled")
        );
    }
}
