//! Configuration types for IRC channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrcConfig {
    pub server: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub nickname: String,
    pub channels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    pub use_tls: bool,
}

fn default_port() -> u16 {
    6667
}

impl IrcConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.server.is_empty() {
            return Err(crate::Error::InvalidConfig("server is required".to_string()));
        }
        if self.nickname.is_empty() {
            return Err(crate::Error::InvalidConfig("nickname is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = IrcConfig {
            server: "irc.freenode.net".to_string(),
            port: 6667,
            nickname: "clawmaster".to_string(),
            channels: vec!["#rust".to_string()],
            password: None,
            use_tls: false,
        };
        assert!(config.validate().is_ok());
    }
}
