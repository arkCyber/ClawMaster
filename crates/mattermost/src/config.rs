//! Configuration for Mattermost channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MattermostConfig {
    pub server_url: String,
    pub access_token: String,
    pub team_id: String,
    #[serde(default)]
    pub channels: Vec<String>,
}

impl MattermostConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.server_url.is_empty() {
            return Err(crate::Error::InvalidConfig("server_url is required".to_string()));
        }
        if self.access_token.is_empty() {
            return Err(crate::Error::InvalidConfig("access_token is required".to_string()));
        }
        if self.team_id.is_empty() {
            return Err(crate::Error::InvalidConfig("team_id is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = MattermostConfig {
            server_url: "https://mattermost.example.com".to_string(),
            access_token: "token".to_string(),
            team_id: "team123".to_string(),
            channels: vec![],
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_server_url() {
        let config = MattermostConfig {
            server_url: String::new(),
            access_token: "token".to_string(),
            team_id: "team123".to_string(),
            channels: vec![],
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_token() {
        let config = MattermostConfig {
            server_url: "https://mattermost.example.com".to_string(),
            access_token: String::new(),
            team_id: "team123".to_string(),
            channels: vec![],
        };
        assert!(config.validate().is_err());
    }
}
