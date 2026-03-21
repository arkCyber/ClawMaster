//! Signal channel configuration

use serde::{Deserialize, Serialize};

/// Signal channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    /// Phone number (E.164 format)
    pub phone_number: String,

    /// Signal server URL (optional, defaults to official server)
    #[serde(default = "default_server_url")]
    pub server_url: String,

    /// Device name
    #[serde(default = "default_device_name")]
    pub device_name: String,

    /// Auto-accept group invites
    #[serde(default)]
    pub auto_accept_groups: bool,

    /// Enable typing indicators
    #[serde(default = "default_true")]
    pub typing_indicators: bool,

    /// Enable read receipts
    #[serde(default = "default_true")]
    pub read_receipts: bool,
}

fn default_server_url() -> String {
    "https://textsecure-service.whispersystems.org".to_string()
}

fn default_device_name() -> String {
    "ClawMaster".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            phone_number: String::new(),
            server_url: default_server_url(),
            device_name: default_device_name(),
            auto_accept_groups: false,
            typing_indicators: true,
            read_receipts: true,
        }
    }
}
