//! Configuration for Viber channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViberConfig {
    pub auth_token: String,
    pub bot_name: String,
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

fn default_api_url() -> String {
    "https://chatapi.viber.com/pa".to_string()
}

impl ViberConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.auth_token.is_empty() {
            return Err(crate::Error::InvalidConfig("auth_token is required".to_string()));
        }
        if self.bot_name.is_empty() {
            return Err(crate::Error::InvalidConfig("bot_name is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = ViberConfig {
            auth_token: "token123".to_string(),
            bot_name: "TestBot".to_string(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_token() {
        let config = ViberConfig {
            auth_token: String::new(),
            bot_name: "TestBot".to_string(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_bot_name() {
        let config = ViberConfig {
            auth_token: "token123".to_string(),
            bot_name: String::new(),
            api_url: default_api_url(),
        };
        assert!(config.validate().is_err());
    }
}
