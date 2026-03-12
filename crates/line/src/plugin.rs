//! Line channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, instrument};
use clawmaster_channels::{ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin, ChannelStreamOutbound, ChannelType, StreamReceiver};
use clawmaster_common::types::ReplyPayload;
use crate::{config::LineConfig, Error, Result};

pub struct LineChannel {
    clients: Arc<RwLock<HashMap<String, Arc<LineConfig>>>>,
}

impl LineChannel {
    pub fn new() -> Self {
        Self { clients: Arc::new(RwLock::new(HashMap::new())) }
    }
}

impl Default for LineChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for LineChannel {
    fn id(&self) -> &str { "line" }
    
    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "line".to_string(),
            name: "Line".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: true,
            supports_threads: false,
        }
    }
    
    #[instrument(skip(self, config))]
    async fn start_account(&self, account_id: &str, config: serde_json::Value) -> clawmaster_channels::Result<()> {
        info!("Starting Line account: {}", account_id);
        let line_config: LineConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;
        line_config.validate().map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;
        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(line_config));
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
impl ChannelOutbound for LineChannel {
    async fn send_reply(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for LineChannel {
    async fn send_stream_start(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported("Line does not support streaming"))
    }
    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Line does not support streaming"))
    }
    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("Line does not support streaming"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_creation() {
        let channel = LineChannel::new();
        assert_eq!(channel.id(), "line");
    }

    #[tokio::test]
    async fn test_channel_descriptor() {
        let channel = LineChannel::new();
        let descriptor = channel.descriptor();
        assert_eq!(descriptor.id, "line");
        assert_eq!(descriptor.name, "Line");
        assert!(descriptor.supports_interactive);
    }

    #[tokio::test]
    async fn test_channel_health() {
        let channel = LineChannel::new();
        let health = channel.health().await;
        assert!(health.healthy);
        assert_eq!(health.account_count, 0);
    }
}
