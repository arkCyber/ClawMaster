//! Configuration for Tox channel.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxConfig {
    /// Tox ID
    pub tox_id: String,
    
    /// Bootstrap nodes
    #[serde(default)]
    pub bootstrap_nodes: Vec<String>,
    
    /// Save file path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_file: Option<String>,
}

impl ToxConfig {
    pub fn validate(&self) -> crate::Result<()> {
        if self.tox_id.is_empty() {
            return Err(crate::Error::InvalidConfig("tox_id is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = ToxConfig {
            tox_id: "76518406F6A9F2217E8DC487CC783C25CC16A15EB36FF32E335364EC37B13020".to_string(),
            bootstrap_nodes: vec![],
            save_file: None,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_id() {
        let config = ToxConfig {
            tox_id: String::new(),
            bootstrap_nodes: vec![],
            save_file: None,
        };
        assert!(config.validate().is_err());
    }
}
