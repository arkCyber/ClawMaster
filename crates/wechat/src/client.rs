//! WeChat Work API client implementation.

use reqwest::Client;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, instrument, warn};

use crate::{
    config::WeChatConfig,
    types::{AccessTokenResponse, SendMessageResponse, WeChatApiResponse, WeChatMessage},
    Error, Result,
};

/// Cached access token.
#[derive(Debug, Clone)]
struct CachedToken {
    token: String,
    expires_at: SystemTime,
}

/// WeChat Work API client.
pub struct WeChatClient {
    config: WeChatConfig,
    client: Client,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl WeChatClient {
    /// Create a new WeChat client.
    pub fn new(config: WeChatConfig) -> Result<Self> {
        config.validate()?;

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            config,
            client,
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Get access token (with caching).
    #[instrument(skip(self))]
    async fn get_access_token(&self) -> Result<String> {
        // Check cache first
        {
            let cached = self.cached_token.read().await;
            if let Some(token) = &*cached {
                if token.expires_at > SystemTime::now() {
                    debug!("Using cached access token");
                    return Ok(token.token.clone());
                }
            }
        }

        // Fetch new token
        debug!("Fetching new access token");
        let url = format!(
            "{}/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.config.api_url, self.config.corp_id, self.config.secret
        );

        let response = self.client.get(&url).send().await?;
        let api_response: WeChatApiResponse<AccessTokenResponse> = response.json().await?;

        if !api_response.is_success() {
            return Err(Error::ApiError {
                code: api_response.errcode,
                message: api_response.errmsg,
            });
        }

        let token_data = api_response
            .data
            .ok_or_else(|| Error::Other("No token data in response".to_string()))?;

        // Cache the token
        let expires_at = SystemTime::now() + Duration::from_secs(token_data.expires_in - 60); // 1 minute buffer
        let cached = CachedToken {
            token: token_data.access_token.clone(),
            expires_at,
        };

        {
            let mut cache = self.cached_token.write().await;
            *cache = Some(cached);
        }

        Ok(token_data.access_token)
    }

    /// Send a message.
    #[instrument(skip(self, message))]
    pub async fn send_message(&self, message: WeChatMessage) -> Result<SendMessageResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "{}/cgi-bin/message/send?access_token={}",
            self.config.api_url, access_token
        );

        debug!("Sending message to WeChat API");

        let response = self.client.post(&url).json(&message).send().await?;

        let api_response: WeChatApiResponse<SendMessageResponse> = response.json().await?;

        if !api_response.is_success() {
            // Check if token expired
            if api_response.errcode == 40014 || api_response.errcode == 42001 {
                warn!("Access token expired, clearing cache");
                let mut cache = self.cached_token.write().await;
                *cache = None;
                return Err(Error::TokenExpired);
            }

            return Err(Error::ApiError {
                code: api_response.errcode,
                message: api_response.errmsg,
            });
        }

        api_response
            .data
            .ok_or_else(|| Error::SendFailed("No data in response".to_string()))
    }

    /// Get the configuration.
    pub fn config(&self) -> &WeChatConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> WeChatConfig {
        WeChatConfig {
            corp_id: "test_corp".to_string(),
            agent_id: "1000002".to_string(),
            secret: "test_secret".to_string(),
            api_url: "https://qyapi.weixin.qq.com".to_string(),
            token: None,
            encoding_aes_key: None,
            token_cache_duration: 7200,
            max_message_length: 2048,
        }
    }

    #[test]
    fn test_client_creation() {
        let config = create_test_config();
        let client = WeChatClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_invalid_config() {
        let config = WeChatConfig {
            corp_id: String::new(),
            agent_id: "1000002".to_string(),
            secret: "test_secret".to_string(),
            api_url: "https://qyapi.weixin.qq.com".to_string(),
            token: None,
            encoding_aes_key: None,
            token_cache_duration: 7200,
            max_message_length: 2048,
        };

        let client = WeChatClient::new(config);
        assert!(client.is_err());
    }
}
