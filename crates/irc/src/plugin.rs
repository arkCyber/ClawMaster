//! IRC channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, instrument};

use clawmaster_channels::{
    ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin,
    ChannelStreamOutbound, ChannelType, StreamReceiver,
};
use clawmaster_common::types::ReplyPayload;

use crate::{config::IrcConfig, Error, Result};

pub struct IrcChannel {
    clients: Arc<RwLock<HashMap<String, Arc<IrcConfig>>>>,
}

impl IrcChannel {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for IrcChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for IrcChannel {
    fn id(&self) -> &str {
        "irc"
    }

    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "irc".to_string(),
            name: "IRC".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: false,
            supports_threads: false,
        }
    }

    #[instrument(skip(self, config))]
    async fn start_account(&self, account_id: &str, config: serde_json::Value) -> clawmaster_channels::Result<()> {
        info!("Starting IRC account: {}", account_id);
        let irc_config: IrcConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;
        irc_config.validate().map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;
        
        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(irc_config));
        Ok(())
    }

    async fn stop_account(&self, account_id: &str) -> clawmaster_channels::Result<()> {
        let mut clients = self.clients.write().await;
        clients.remove(account_id);
        Ok(())
    }

    async fn health(&self) -> ChannelHealthSnapshot {
        let clients = self.clients.read().await;
        ChannelHealthSnapshot {
            healthy: true,
            account_count: clients.len(),
            error: None,
        }
    }
}

#[async_trait]
impl ChannelOutbound for IrcChannel {
    async fn send_reply(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for IrcChannel {
    async fn send_stream_start(&self, _payload: ReplyPayload) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported("IRC does not support streaming"))
    }

    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("IRC does not support streaming"))
    }

    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported("IRC does not support streaming"))
    }
}
