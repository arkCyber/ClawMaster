//! Configuration for Line channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineConfig {
    pub channel_access_token: String,
    pub channel_secret: String,
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

fn default_api_url() -> String {
    "https://api.line.me".to_string()
}

impl LineConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.channel_access_token.is_empty() {
            return Err(crate::Error::InvalidConfig("channel_access_token is required".to_string()));
        }
        if self.channel_secret.is_empty() {
            return Err(crate::Error::InvalidConfig("channel_secret is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = LineConfig {
            channel_access_token: "token123".to_string(),
            channel_secret: "secret123".to_string(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_token() {
        let config = LineConfig {
            channel_access_token: String::new(),
            channel_secret: "secret123".to_string(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_secret() {
        let config = LineConfig {
            channel_access_token: "token123".to_string(),
            channel_secret: String::new(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }
}
