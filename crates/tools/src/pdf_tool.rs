//! PDF document processing and analysis tool.
//!
//! Provides PDF text extraction, metadata reading, and document analysis.
//! Handles sensitive document data with privacy protection.

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    base64::{Engine as _, engine::general_purpose},
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::sync::Arc,
};

/// PDF processing provider trait.
#[async_trait]
pub trait PdfProcessingProvider: Send + Sync {
    async fn extract_text(&self, pdf_data: &[u8]) -> Result<String>;
    async fn extract_text_from_url(&self, url: &str) -> Result<String>;
    async fn get_metadata(&self, pdf_data: &[u8]) -> Result<PdfMetadata>;
    async fn get_page_count(&self, pdf_data: &[u8]) -> Result<usize>;
    async fn extract_page(&self, pdf_data: &[u8], page: usize) -> Result<String>;
}

/// PDF document metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub page_count: usize,
    pub file_size: usize,
}

/// PDF tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PdfToolConfig {
    pub enabled: bool,
    pub max_file_size: usize,
    pub allow_url_fetch: bool,
    pub max_pages_extract: usize,
}

impl Default for PdfToolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_file_size: 50_000_000, // 50MB
            allow_url_fetch: true,
            max_pages_extract: 100,
        }
    }
}

/// PDF processing tool.
pub struct PdfTool {
    config: PdfToolConfig,
    provider: Arc<dyn PdfProcessingProvider>,
}

impl PdfTool {
    pub fn new(config: PdfToolConfig, provider: Arc<dyn PdfProcessingProvider>) -> Self {
        Self { config, provider }
    }
}

#[async_trait]
impl AgentTool for PdfTool {
    fn name(&self) -> &str {
        "pdf"
    }

    fn description(&self) -> &str {
        "Extract text and metadata from PDF documents"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["extract_text", "metadata", "page_count", "extract_page"],
                    "description": "Action to perform on the PDF"
                },
                "pdf_url": {
                    "type": "string",
                    "description": "URL of the PDF document"
                },
                "pdf_base64": {
                    "type": "string",
                    "description": "Base64-encoded PDF data"
                },
                "page": {
                    "type": "integer",
                    "description": "Page number (1-indexed, for extract_page action)",
                    "minimum": 1
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("PDF tool is disabled");
        }

        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' parameter"))?;

        // Get PDF data
        let pdf_data = if let Some(url) = params.get("pdf_url").and_then(|v| v.as_str()) {
            if !self.config.allow_url_fetch {
                bail!("URL fetching is disabled");
            }
            // For URL, we'll use the provider's URL method
            return self.execute_url_action(action, url, &params).await;
        } else if let Some(base64_str) = params.get("pdf_base64").and_then(|v| v.as_str()) {
            let data = general_purpose::STANDARD.decode(base64_str)
                .map_err(|e| anyhow::anyhow!("Invalid base64 data: {}", e))?;

            if data.len() > self.config.max_file_size {
                bail!("PDF size {} exceeds maximum {}", 
                    data.len(), self.config.max_file_size);
            }
            data
        } else {
            bail!("Either 'pdf_url' or 'pdf_base64' must be provided");
        };

        match action {
            "extract_text" => {
                let text = self.provider.extract_text(&pdf_data).await?;
                Ok(json!({
                    "text": text,
                    "length": text.len()
                }))
            }
            "metadata" => {
                let metadata = self.provider.get_metadata(&pdf_data).await?;
                Ok(serde_json::to_value(metadata)?)
            }
            "page_count" => {
                let count = self.provider.get_page_count(&pdf_data).await?;
                Ok(json!({
                    "page_count": count
                }))
            }
            "extract_page" => {
                let page = params.get("page")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'page' parameter"))? as usize;

                if page == 0 {
                    bail!("Page number must be >= 1");
                }

                let text = self.provider.extract_page(&pdf_data, page).await?;
                Ok(json!({
                    "page": page,
                    "text": text,
                    "length": text.len()
                }))
            }
            _ => bail!("Invalid action: {}", action),
        }
    }
}

impl PdfTool {
    async fn execute_url_action(&self, action: &str, url: &str, _params: &Value) -> Result<Value> {
        match action {
            "extract_text" => {
                let text = self.provider.extract_text_from_url(url).await?;
                Ok(json!({
                    "text": text,
                    "length": text.len(),
                    "source": "url"
                }))
            }
            _ => {
                // For other actions, we need to fetch the PDF first
                bail!("Action '{}' with URL requires pdf_base64 instead", action);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPdfProvider;

    #[async_trait]
    impl PdfProcessingProvider for MockPdfProvider {
        async fn extract_text(&self, _pdf_data: &[u8]) -> Result<String> {
            Ok("This is extracted text from PDF.\nPage 1 content.\nPage 2 content.".to_string())
        }

        async fn extract_text_from_url(&self, _url: &str) -> Result<String> {
            self.extract_text(&[]).await
        }

        async fn get_metadata(&self, _pdf_data: &[u8]) -> Result<PdfMetadata> {
            Ok(PdfMetadata {
                title: Some("Test Document".to_string()),
                author: Some("Test Author".to_string()),
                subject: Some("Testing".to_string()),
                keywords: Some("test, pdf".to_string()),
                creator: Some("Test Creator".to_string()),
                producer: Some("Test Producer".to_string()),
                creation_date: Some("2024-01-01".to_string()),
                modification_date: Some("2024-01-02".to_string()),
                page_count: 10,
                file_size: 50000,
            })
        }

        async fn get_page_count(&self, _pdf_data: &[u8]) -> Result<usize> {
            Ok(10)
        }

        async fn extract_page(&self, _pdf_data: &[u8], page: usize) -> Result<String> {
            Ok(format!("Content of page {}", page))
        }
    }

    #[tokio::test]
    async fn test_extract_text() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let pdf_data = b"fake pdf data";
        let base64_data = general_purpose::STANDARD.encode(pdf_data);

        let params = json!({
            "action": "extract_text",
            "pdf_base64": base64_data
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result["text"].as_str().unwrap().contains("extracted text"));
        assert!(result["length"].as_u64().is_some());
    }

    #[tokio::test]
    async fn test_metadata() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let pdf_data = b"fake pdf data";
        let base64_data = general_purpose::STANDARD.encode(pdf_data);

        let params = json!({
            "action": "metadata",
            "pdf_base64": base64_data
        });

        let result = tool.execute(params).await.unwrap();
        assert_eq!(result["title"], "Test Document");
        assert_eq!(result["author"], "Test Author");
        assert_eq!(result["page_count"], 10);
    }

    #[tokio::test]
    async fn test_page_count() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let pdf_data = b"fake pdf data";
        let base64_data = general_purpose::STANDARD.encode(pdf_data);

        let params = json!({
            "action": "page_count",
            "pdf_base64": base64_data
        });

        let result = tool.execute(params).await.unwrap();
        assert_eq!(result["page_count"], 10);
    }

    #[tokio::test]
    async fn test_extract_page() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let pdf_data = b"fake pdf data";
        let base64_data = general_purpose::STANDARD.encode(pdf_data);

        let params = json!({
            "action": "extract_page",
            "pdf_base64": base64_data,
            "page": 5
        });

        let result = tool.execute(params).await.unwrap();
        assert_eq!(result["page"], 5);
        assert!(result["text"].as_str().unwrap().contains("page 5"));
    }

    #[tokio::test]
    async fn test_extract_text_from_url() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let params = json!({
            "action": "extract_text",
            "pdf_url": "https://example.com/document.pdf"
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result["text"].is_string());
        assert_eq!(result["source"], "url");
    }

    #[tokio::test]
    async fn test_disabled() {
        let mut config = PdfToolConfig::default();
        config.enabled = false;
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let params = json!({
            "action": "extract_text",
            "pdf_url": "https://example.com/document.pdf"
        });

        let result = tool.execute(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disabled"));
    }

    #[tokio::test]
    async fn test_url_fetch_disabled() {
        let mut config = PdfToolConfig::default();
        config.allow_url_fetch = false;
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let params = json!({
            "action": "extract_text",
            "pdf_url": "https://example.com/document.pdf"
        });

        let result = tool.execute(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("URL fetching is disabled"));
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let config = PdfToolConfig::default();
        let provider = Arc::new(MockPdfProvider);
        let tool = PdfTool::new(config, provider);

        let pdf_data = b"fake pdf data";
        let base64_data = general_purpose::STANDARD.encode(pdf_data);

        let params = json!({
            "action": "extract_page",
            "pdf_base64": base64_data,
            "page": 0
        });

        let result = tool.execute(params).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be >= 1"));
    }
}
