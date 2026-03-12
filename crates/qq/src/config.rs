//! Configuration types for QQ channel.

use serde::{Deserialize, Serialize};

/// QQ channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QqConfig {
    /// QQ Bot ID
    pub bot_id: String,

    /// Access token for authentication
    pub access_token: String,

    /// API base URL (e.g., http://localhost:5700)
    pub api_url: String,

    /// List of group IDs to monitor
    #[serde(default)]
    pub groups: Vec<String>,

    /// Enable private messages
    #[serde(default = "default_true")]
    pub enable_private: bool,

    /// Enable group messages
    #[serde(default = "default_true")]
    pub enable_group: bool,

    /// Webhook port for receiving messages
    #[serde(default = "default_webhook_port")]
    pub webhook_port: u16,

    /// Maximum message length
    #[serde(default = "default_max_length")]
    pub max_message_length: usize,
}

fn default_true() -> bool {
    true
}

fn default_webhook_port() -> u16 {
    8080
}

fn default_max_length() -> usize {
    4096
}

impl QqConfig {
    /// Validate the configuration.
    pub fn validate(&self) -> crate::Result<()> {
        if self.bot_id.is_empty() {
            return Err(crate::Error::InvalidConfig("bot_id is required".to_string()));
        }

        if self.access_token.is_empty() {
            return Err(crate::Error::InvalidConfig(
                "access_token is required".to_string(),
            ));
        }

        if self.api_url.is_empty() {
            return Err(crate::Error::InvalidConfig("api_url is required".to_string()));
        }

        if !self.enable_private && !self.enable_group {
            return Err(crate::Error::InvalidConfig(
                "at least one of enable_private or enable_group must be true".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = QqConfig {
            bot_id: "123456".to_string(),
            access_token: "token".to_string(),
            api_url: "http://localhost:5700".to_string(),
            groups: vec![],
            enable_private: true,
            enable_group: true,
            webhook_port: 8080,
            max_message_length: 4096,
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_bot_id() {
        let config = QqConfig {
            bot_id: String::new(),
            access_token: "token".to_string(),
            api_url: "http://localhost:5700".to_string(),
            groups: vec![],
            enable_private: true,
            enable_group: true,
            webhook_port: 8080,
            max_message_length: 4096,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_both_disabled() {
        let config = QqConfig {
            bot_id: "123456".to_string(),
            access_token: "token".to_string(),
            api_url: "http://localhost:5700".to_string(),
            groups: vec![],
            enable_private: false,
            enable_group: false,
            webhook_port: 8080,
            max_message_length: 4096,
        };

        assert!(config.validate().is_err());
    }
}
