//! Configuration for Zulip channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipConfig {
    pub site_url: String,
    pub email: String,
    pub api_key: String,
    #[serde(default)]
    pub streams: Vec<String>,
}

impl ZulipConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.site_url.is_empty() {
            return Err(crate::Error::InvalidConfig("site_url is required".to_string()));
        }
        if self.email.is_empty() {
            return Err(crate::Error::InvalidConfig("email is required".to_string()));
        }
        if self.api_key.is_empty() {
            return Err(crate::Error::InvalidConfig("api_key is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = ZulipConfig {
            site_url: "https://zulip.example.com".to_string(),
            email: "bot@example.com".to_string(),
            api_key: "key123".to_string(),
            streams: vec![],
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_site_url() {
        let config = ZulipConfig {
            site_url: String::new(),
            email: "bot@example.com".to_string(),
            api_key: "key123".to_string(),
            streams: vec![],
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_email() {
        let config = ZulipConfig {
            site_url: "https://zulip.example.com".to_string(),
            email: String::new(),
            api_key: "key123".to_string(),
            streams: vec![],
        };
        assert!(config.validate().is_err());
    }
}
