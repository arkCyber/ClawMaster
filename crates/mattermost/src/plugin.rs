//! Mattermost channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, instrument};
use clawmaster_channels::{ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin, ChannelStreamOutbound, ChannelType, StreamReceiver};
use clawmaster_common::types::ReplyPayload;
use crate::{config::MattermostConfig, Error, Result};

pub struct MattermostChannel {
    clients: Arc<RwLock<HashMap<String, Arc<MattermostConfig>>>>,
}

impl MattermostChannel {
    pub fn new() -> Self {
        Self { clients: Arc::new(RwLock::new(HashMap::new())) }
    }
}

impl Default for MattermostChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for MattermostChannel {
    fn id(&self) -> &str { "mattermost" }
    
    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "mattermost".to_string(),
            name: "Mattermost".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: true,
            supports_threads: true,
        }
    }
    
    #[instrument(skip(self, config))]
    async fn start_account(&self, account_id: &str, config: serde_json::Value) -> clawmaster_channels::Result<()> {
        info!("Starting Mattermost account: {}", account_id);
        let mm_config: MattermostConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;
        mm_config.validate().map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;
        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(mm_config));
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
impl ChannelOutbound for MattermostChannel {
    async fn send_reply(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for MattermostChannel {
    async fn send_stream_start(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported("Mattermost does not support streaming"))
    }
    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Mattermost does not support streaming"))
    }
    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Mattermost does not support streaming"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_creation() {
        let channel = MattermostChannel::new();
        assert_eq!(channel.id(), "mattermost");
    }

    #[tokio::test]
    async fn test_channel_descriptor() {
        let channel = MattermostChannel::new();
        let descriptor = channel.descriptor();
        assert_eq!(descriptor.id, "mattermost");
        assert_eq!(descriptor.name, "Mattermost");
        assert!(descriptor.supports_interactive);
        assert!(descriptor.supports_threads);
    }

    #[tokio::test]
    async fn test_channel_health() {
        let channel = MattermostChannel::new();
        let health = channel.health().await;
        assert!(health.healthy);
        assert_eq!(health.account_count, 0);
    }
}
