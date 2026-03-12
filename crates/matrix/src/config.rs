//! Configuration for Matrix channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    pub homeserver_url: String,
    pub user_id: String,
    pub access_token: String,
    #[serde(default)]
    pub rooms: Vec<String>,
}

impl MatrixConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.homeserver_url.is_empty() {
            return Err(crate::Error::InvalidConfig("homeserver_url is required".to_string()));
        }
        if self.user_id.is_empty() {
            return Err(crate::Error::InvalidConfig("user_id is required".to_string()));
        }
        if self.access_token.is_empty() {
            return Err(crate::Error::InvalidConfig("access_token is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = MatrixConfig {
            homeserver_url: "https://matrix.org".to_string(),
            user_id: "@bot:matrix.org".to_string(),
            access_token: "token123".to_string(),
            rooms: vec![],
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_homeserver() {
        let config = MatrixConfig {
            homeserver_url: String::new(),
            user_id: "@bot:matrix.org".to_string(),
            access_token: "token123".to_string(),
            rooms: vec![],
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_user_id() {
        let config = MatrixConfig {
            homeserver_url: "https://matrix.org".to_string(),
            user_id: String::new(),
            access_token: "token123".to_string(),
            rooms: vec![],
        };
        assert!(config.validate().is_err());
    }
}
