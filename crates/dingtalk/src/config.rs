//! Configuration for DingTalk channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DingTalkConfig {
    /// App Key
    pub app_key: String,
    
    /// App Secret
    pub app_secret: String,
    
    /// Robot Webhook URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    
    /// Robot Secret for signature (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_secret: Option<String>,
    
    /// API base URL
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

fn default_api_url() -> String {
    "https://oapi.dingtalk.com".to_string()
}

impl DingTalkConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.app_key.is_empty() {
            return Err(crate::Error::InvalidConfig("app_key is required".to_string()));
        }
        if self.app_secret.is_empty() {
            return Err(crate::Error::InvalidConfig("app_secret is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = DingTalkConfig {
            app_key: "test_key".to_string(),
            app_secret: "test_secret".to_string(),
            webhook_url: None,
            robot_secret: None,
            api_url: default_api_url(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_key() {
        let config = DingTalkConfig {
            app_key: String::new(),
            app_secret: "test_secret".to_string(),
            webhook_url: None,
            robot_secret: None,
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }
}
