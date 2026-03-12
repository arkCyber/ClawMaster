//! Configuration for Feishu channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuConfig {
    /// App ID
    pub app_id: String,
    
    /// App Secret
    pub app_secret: String,
    
    /// Verification Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
    
    /// Encrypt Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_key: Option<String>,
    
    /// API base URL
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

fn default_api_url() -> String {
    "https://open.feishu.cn".to_string()
}

impl FeishuConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.app_id.is_empty() {
            return Err(crate::Error::InvalidConfig("app_id is required".to_string()));
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
        let config = FeishuConfig {
            app_id: "cli_test123".to_string(),
            app_secret: "secret123".to_string(),
            verification_token: None,
            encrypt_key: None,
            api_url: default_api_url(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_app_id() {
        let config = FeishuConfig {
            app_id: String::new(),
            app_secret: "secret123".to_string(),
            verification_token: None,
            encrypt_key: None,
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }
}
