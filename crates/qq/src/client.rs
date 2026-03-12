//! QQ API client implementation.

use reqwest::Client;
use tracing::{debug, instrument};

use crate::{
    config::QqConfig,
    types::{QqApiResponse, SendMessageRequest, SendMessageResponse},
    Error, Result,
};

/// QQ API client.
pub struct QqClient {
    config: QqConfig,
    client: Client,
}

impl QqClient {
    /// Create a new QQ client.
    pub fn new(config: QqConfig) -> Result<Self> {
        config.validate()?;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Send a message.
    #[instrument(skip(self, request))]
    pub async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse> {
        let url = format!("{}/send_msg", self.config.api_url);

        debug!("Sending message to QQ API: {:?}", request);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .json(&request)
            .send()
            .await?;

        let api_response: QqApiResponse<SendMessageResponse> = response.json().await?;

        if api_response.is_success() {
            api_response
                .data
                .ok_or_else(|| Error::SendFailed("No data in response".to_string()))
        } else {
            Err(Error::ApiError {
                code: api_response.retcode,
                message: api_response
                    .error_message()
                    .unwrap_or_else(|| "Unknown error".to_string()),
            })
        }
    }

    /// Get group list.
    #[instrument(skip(self))]
    pub async fn get_group_list(&self) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}/get_group_list", self.config.api_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .send()
            .await?;

        let api_response: QqApiResponse<Vec<serde_json::Value>> = response.json().await?;

        if api_response.is_success() {
            Ok(api_response.data.unwrap_or_default())
        } else {
            Err(Error::ApiError {
                code: api_response.retcode,
                message: api_response
                    .error_message()
                    .unwrap_or_else(|| "Unknown error".to_string()),
            })
        }
    }

    /// Get login info.
    #[instrument(skip(self))]
    pub async fn get_login_info(&self) -> Result<serde_json::Value> {
        let url = format!("{}/get_login_info", self.config.api_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .send()
            .await?;

        let api_response: QqApiResponse<serde_json::Value> = response.json().await?;

        if api_response.is_success() {
            api_response
                .data
                .ok_or_else(|| Error::Other("No login info".to_string()))
        } else {
            Err(Error::ApiError {
                code: api_response.retcode,
                message: api_response
                    .error_message()
                    .unwrap_or_else(|| "Unknown error".to_string()),
            })
        }
    }

    /// Get the configuration.
    pub fn config(&self) -> &QqConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> QqConfig {
        QqConfig {
            bot_id: "123456".to_string(),
            access_token: "test_token".to_string(),
            api_url: "http://localhost:5700".to_string(),
            groups: vec![],
            enable_private: true,
            enable_group: true,
            webhook_port: 8080,
            max_message_length: 4096,
        }
    }

    #[test]
    fn test_client_creation() {
        let config = create_test_config();
        let client = QqClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_invalid_config() {
        let config = QqConfig {
            bot_id: String::new(),
            access_token: "test_token".to_string(),
            api_url: "http://localhost:5700".to_string(),
            groups: vec![],
            enable_private: true,
            enable_group: true,
            webhook_port: 8080,
            max_message_length: 4096,
        };

        let client = QqClient::new(config);
        assert!(client.is_err());
    }
}
