//! QQ channel plugin implementation.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use clawmaster_channels::{
    ChannelDescriptor, ChannelHealthSnapshot, ChannelOutbound, ChannelPlugin,
    ChannelStreamOutbound, ChannelType, InteractiveMessage, StreamReceiver, ThreadMessage,
};
use clawmaster_common::types::ReplyPayload;

use crate::{
    client::QqClient,
    config::QqConfig,
    markdown::markdown_to_qq,
    types::{QqMessageType, SendMessageRequest},
    Error, Result,
};

/// QQ channel plugin.
pub struct QqChannel {
    /// Active clients by account ID
    clients: Arc<RwLock<HashMap<String, Arc<QqClient>>>>,
}

impl QqChannel {
    /// Create a new QQ channel plugin.
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a client for an account.
    async fn get_client(&self, account_id: &str) -> Result<Arc<QqClient>> {
        let clients = self.clients.read().await;
        clients
            .get(account_id)
            .cloned()
            .ok_or_else(|| Error::AccountNotFound(account_id.to_string()))
    }
}

impl Default for QqChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for QqChannel {
    fn id(&self) -> &str {
        "qq"
    }

    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            id: "qq".to_string(),
            name: "QQ".to_string(),
            channel_type: ChannelType::Messaging,
            supports_streaming: false,
            supports_interactive: false,
            supports_threads: false,
        }
    }

    #[instrument(skip(self, config))]
    async fn start_account(
        &self,
        account_id: &str,
        config: serde_json::Value,
    ) -> clawmaster_channels::Result<()> {
        info!("Starting QQ account: {}", account_id);

        let qq_config: QqConfig = serde_json::from_value(config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Invalid config: {}", e)))?;

        let client = QqClient::new(qq_config)
            .map_err(|e| clawmaster_channels::Error::invalid_input(format!("Client creation failed: {}", e)))?;

        // Test connection
        match client.get_login_info().await {
            Ok(info) => {
                debug!("QQ login info: {:?}", info);
            }
            Err(e) => {
                warn!("Failed to get login info: {}", e);
            }
        }

        let mut clients = self.clients.write().await;
        clients.insert(account_id.to_string(), Arc::new(client));

        info!("QQ account started: {}", account_id);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn stop_account(&self, account_id: &str) -> clawmaster_channels::Result<()> {
        info!("Stopping QQ account: {}", account_id);

        let mut clients = self.clients.write().await;
        if clients.remove(account_id).is_some() {
            info!("QQ account stopped: {}", account_id);
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
impl ChannelOutbound for QqChannel {
    #[instrument(skip(self, payload))]
    async fn send_reply(&self, payload: ReplyPayload) -> clawmaster_channels::Result<()> {
        debug!("Sending QQ reply: {:?}", payload);

        let client = self
            .get_client(&payload.channel_account_id)
            .await
            .map_err(|e| clawmaster_channels::Error::invalid_input(e.to_string()))?;

        // Convert markdown to QQ format
        let message = markdown_to_qq(&payload.text);

        // Truncate if too long
        let max_length = client.config().max_message_length;
        let message = if message.len() > max_length {
            format!("{}...", &message[..max_length - 3])
        } else {
            message
        };

        // Parse thread context to determine message type and target
        let (message_type, user_id, group_id) = parse_thread_context(&payload.thread_context)?;

        let request = SendMessageRequest {
            message_type,
            user_id,
            group_id,
            message,
            auto_escape: false,
        };

        client
            .send_message(request)
            .await
            .map_err(|e| clawmaster_channels::Error::send_failed(e.to_string()))?;

        info!("QQ reply sent successfully");
        Ok(())
    }
}

#[async_trait]
impl ChannelStreamOutbound for QqChannel {
    async fn send_stream_start(
        &self,
        _payload: ReplyPayload,
    ) -> clawmaster_channels::Result<StreamReceiver> {
        Err(clawmaster_channels::Error::unsupported(
            "QQ does not support streaming",
        ))
    }

    async fn send_stream_chunk(&self, _chunk: String) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported(
            "QQ does not support streaming",
        ))
    }

    async fn send_stream_end(&self) -> clawmaster_channels::Result<()> {
        Err(clawmaster_channels::Error::unsupported(
            "QQ does not support streaming",
        ))
    }
}

/// Parse thread context to extract message type and target IDs.
fn parse_thread_context(
    context: &serde_json::Value,
) -> Result<(QqMessageType, Option<i64>, Option<i64>)> {
    let message_type = context
        .get("message_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Other("Missing message_type in thread context".to_string()))?;

    match message_type {
        "private" => {
            let user_id = context
                .get("user_id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| Error::Other("Missing user_id for private message".to_string()))?;

            Ok((QqMessageType::Private, Some(user_id), None))
        }
        "group" => {
            let group_id = context
                .get("group_id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| Error::Other("Missing group_id for group message".to_string()))?;

            Ok((QqMessageType::Group, None, Some(group_id)))
        }
        _ => Err(Error::Other(format!(
            "Unknown message type: {}",
            message_type
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_thread_context_private() {
        let context = serde_json::json!({
            "message_type": "private",
            "user_id": 123456
        });

        let result = parse_thread_context(&context);
        assert!(result.is_ok());

        let (msg_type, user_id, group_id) = result.unwrap();
        assert_eq!(msg_type, QqMessageType::Private);
        assert_eq!(user_id, Some(123456));
        assert_eq!(group_id, None);
    }

    #[test]
    fn test_parse_thread_context_group() {
        let context = serde_json::json!({
            "message_type": "group",
            "group_id": 789012
        });

        let result = parse_thread_context(&context);
        assert!(result.is_ok());

        let (msg_type, user_id, group_id) = result.unwrap();
        assert_eq!(msg_type, QqMessageType::Group);
        assert_eq!(user_id, None);
        assert_eq!(group_id, Some(789012));
    }

    #[tokio::test]
    async fn test_channel_creation() {
        let channel = QqChannel::new();
        assert_eq!(channel.id(), "qq");
    }

    #[tokio::test]
    async fn test_channel_health() {
        let channel = QqChannel::new();
        let health = channel.health().await;
        assert!(health.healthy);
        assert_eq!(health.account_count, 0);
    }
}
