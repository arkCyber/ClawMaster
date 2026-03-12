//! WeChat Work channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument};

use clawmaster_channels::{
    ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin,
    ChannelStreamOutbound, ChannelType, StreamReceiver,
};
use clawmaster_common::types::ReplyPayload;

use crate::{
    client::WeChatClient,
    config::WeChatConfig,
    types::{WeChatMessage, WeChatMessageType, WeChatTextMessage},
    Error, Result,
};

/// WeChat Work channel plugin.
pub struct WeChatChannel {
    /// Active clients by account ID
    clients: Arc<RwLock<HashMap<String, Arc<WeChatClient>>>>,
}

impl WeChatChannel {
    /// Create a new WeChat channel plugin.
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a client for an account.
    async fn get_client(&self, account_id: &str) -> Result<Arc<WeChatClient>> {
        let clients = self.clients.read().await;
        clients
            .get(account_id)
            .cloned()
            .ok_or_else(|| Error::AccountNotFound(account_id.to_string()))
    }
}

impl Default for WeChatChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for WeChatChannel {
    fn id(&self) -> &str {
        "wechat"
    }

    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "wechat".to_string(),
            name: "WeChat Work".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: true,
            supports_threads: false,
        }
    }

    #[instrument(skip(self, config))]
    async fn start_account(
        &self,
        account_id: &str,
        config: serde_json::Value,
    ) -> clawmaster_channels::Result<()> {
        info!("Starting WeChat account: {}", account_id);

        let wechat_config: WeChatConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;

        let client = WeChatClient::new(wechat_config).map_err(|e| {
            clawmaster_channels::Error::invalid_input(format!("Client creation failed: {}", e))
        })?;

        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(client));

        info!("WeChat account started: {}", account_id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn stop_account(&self, account_id: &str) -> clawmaster_channels::Result<()> {
        info!("Stopping WeChat account: {}", account_id);

        let mut clients = self.clients.write().await;
        if clients.remove(account_id).is_some() {
            info!("WeChat account stopped: {}", account_id);
            Ok(())
        } else {
            Err(clawmaster_channels::Error::invalid_input(format!(
                "Account not found: {}",
                account_id
            )))
        }
    }

    async fn health(&self) -> ChannelHealthSnapshot {
        let clients = self.clients.read().await;
        let account_count = clients.len();

        ChannelHealthSnapshot {
            healthy: true,
            account_count,
            error: None,
        }
    }
}

#[async_trait]
impl ChannelOutbound for WeChatChannel {
    #[instrument(skip(self, payload))]
    async fn send_reply(&self, payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        debug!("Sending WeChat reply: {:?}", payload);

        let client = self
            .get_client(&payload.channel_account_id)
            .await
            .map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;

        // Truncate if too long
        let max_length = client.config().max_message_length;
        let text = if payload.text.len() > max_length {
            format!("{}...", &payload.text[..max_length - 3])
        } else {
            payload.text.clone()
        };

        // Parse thread context to get recipient
        let touser = payload
            .thread_context
            .get("user_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let message = WeChatMessage {
            touser,
            toparty: None,
            totag: None,
            msgtype: WeChatMessageType::Text,
            agentid: client.config().agent_id.clone(),
            text: Some(WeChatTextMessage { content: text }),
            safe: 0,
            enable_id_trans: 0,
            enable_duplicate_check: 0,
            duplicate_check_interval: 0,
        };

        client
            .send_message(message)
            .await
            .map_err(|e| clawmaster_channels::Error::send_failed(e.to_string()))?;

        info!("WeChat reply sent successfully");
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for WeChatChannel {
    async fn send_stream_start(
        &self,
        _payload: ReplyPayload,
    ) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported(
            "WeChat does not support streaming",
        ))
    }

    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported(
            "WeChat does not support streaming",
        ))
    }

    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported(
            "WeChat does not support streaming",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_creation() {
        let channel = WeChatChannel::new();
        assert_eq!(channel.id(), "wechat");
    }

    #[tokio::test]
    async fn test_channel_descriptor() {
        let channel = WeChatChannel::new();
        let descriptor = channel.descriptor();
        assert_eq!(descriptor.id, "wechat");
        assert_eq!(descriptor.name, "WeChat Work");
        assert!(!descriptor.supports_streaming);
        assert!(descriptor.supports_interactive);
    }

    #[tokio::test]
    async fn test_channel_health() {
        let channel = WeChatChannel::new();
        let health = channel.health().await;
        assert!(health.healthy);
        assert_eq!(health.account_count, 0);
    }
}
