//! Tox channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, instrument};
use clawmaster_channels::{ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin, ChannelStreamOutbound, ChannelType, StreamReceiver};
use clawmaster_common::types::ReplyPayload;
use crate::{config::ToxConfig, Error, Result};

pub struct ToxChannel {
    clients: Arc<RwLock<HashMap<String, Arc<ToxConfig>>>>,
}

impl ToxChannel {
    pub fn new() -> Self {
        Self { clients: Arc::new(RwLock::new(HashMap::new())) }
    }
}

impl Default for ToxChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for ToxChannel {
    fn id(&self) -> &str { "tox" }
    
    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "tox".to_string(),
            name: "Tox".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: false,
            supports_threads: false,
        }
    }
    
    #[instrument(skip(self, config))]
    async fn start_account(&self, account_id: &str, config: serde_json::Value) -> clawmaster_channels::Result<()> {
        info!("Starting Tox account: {}", account_id);
        let tox_config: ToxConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;
        tox_config.validate().map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;
        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(tox_config));
        Ok(())
    }
    
    async fn stop_account(&self, account_id: &str) -> clawmaster_channels::Result<()> {
        let mut clients = self.clients.write().await;
        clients.remove(account_id);
        Ok(())
    }
    
    async fn health(&self) -> ChannelHealthSnapshot {
        let clients = self.clients.read().await;
        ChannelHealthSnapshot { healthy: true, account_count: clients.len(), error: None }
    }
}

#[async_trait]
impl ChannelOutbound for ToxChannel {
    async fn send_reply(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for ToxChannel {
    async fn send_stream_start(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported("Tox does not support streaming"))
    }
    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Tox does not support streaming"))
    }
    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Tox does not support streaming"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_creation() {
        let channel = ToxChannel::new();
        assert_eq!(channel.id(), "tox");
    }

    #[tokio::test]
    async fn test_channel_descriptor() {
        let channel = ToxChannel::new();
        let descriptor = channel.descriptor();
        assert_eq!(descriptor.id, "tox");
        assert_eq!(descriptor.name, "Tox");
    }

    #[tokio::test]
    async fn test_channel_health() {
        let channel = ToxChannel::new();
        let health = channel.health().await;
        assert!(health.healthy);
        assert_eq!(health.account_count, 0);
    }
}
