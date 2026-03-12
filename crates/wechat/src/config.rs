//! Configuration types for WeChat Work channel.

use serde::{Deserialize, Serialize};

/// WeChat Work channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatConfig {
    /// Corporation ID
    pub corp_id: String,

    /// Agent ID
    pub agent_id: String,

    /// Agent secret
    pub secret: String,

    /// API base URL
    #[serde(default = "default_api_url")]
    pub api_url: String,

    /// Token for webhook verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Encoding AES key for message encryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_aes_key: Option<String>,

    /// Access token cache duration (seconds)
    #[serde(default = "default_token_cache_duration")]
    pub token_cache_duration: u64,

    /// Maximum message length
    #[serde(default = "default_max_length")]
    pub max_message_length: usize,
}

fn default_api_url() -> String {
    "https://qyapi.weixin.qq.com".to_string()
}

fn default_token_cache_duration() -> u64 {
    7200 // 2 hours, WeChat tokens expire in 2 hours
}

fn default_max_length() -> usize {
    2048
}

impl WeChatConfig {
    /// Validate the configuration.
    pub fn validate(&self) -> crate::Result<()> {
        if self.corp_id.is_empty() {
            return Err(crate::Error::InvalidConfig("corp_id is required".to_string()));
        }

        if self.agent_id.is_empty() {
            return Err(crate::Error::InvalidConfig("agent_id is required".to_string()));
        }

        if self.secret.is_empty() {
            return Err(crate::Error::InvalidConfig("secret is required".to_string()));
        }

        if self.api_url.is_empty() {
            return Err(crate::Error::InvalidConfig("api_url is required".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = WeChatConfig {
            corp_id: "test_corp".to_string(),
            agent_id: "1000002".to_string(),
            secret: "test_secret".to_string(),
            api_url: default_api_url(),
            token: None,
            encoding_aes_key: None,
            token_cache_duration: default_token_cache_duration(),
            max_message_length: default_max_length(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_corp_id() {
        let config = WeChatConfig {
            corp_id: String::new(),
            agent_id: "1000002".to_string(),
            secret: "test_secret".to_string(),
            api_url: default_api_url(),
            token: None,
            encoding_aes_key: None,
            token_cache_duration: default_token_cache_duration(),
            max_message_length: default_max_length(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_values() {
        let config = WeChatConfig {
            corp_id: "test".to_string(),
            agent_id: "1000002".to_string(),
            secret: "secret".to_string(),
            api_url: default_api_url(),
            token: None,
            encoding_aes_key: None,
            token_cache_duration: default_token_cache_duration(),
            max_message_length: default_max_length(),
        };

        assert_eq!(config.api_url, "https://qyapi.weixin.qq.com");
        assert_eq!(config.token_cache_duration, 7200);
        assert_eq!(config.max_message_length, 2048);
    }
}
