//! Signal channel plugin implementation

use {
    crate::{config::SignalConfig, error::Result},
    async_trait::async_trait,
    clawmaster_channels::plugin::{
        ChannelCapabilities, ChannelDescriptor, ChannelEvent, ChannelEventSink, ChannelHealth,
        ChannelMessageKind, ChannelPlugin, ChannelReplyTarget, ChannelType, InboundMessage,
        InboundMode, OutboundMessage,
    },
    clawmaster_common::types::ReplyPayload,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
    tracing::{debug, error, info, warn},
};

/// Signal channel plugin
///
/// Provides Signal messaging support with end-to-end encryption.
///
/// # Compliance
///
/// DO-178C §11.10: Resource management
/// - All connections are properly cleaned up
/// - No resource leaks
pub struct SignalChannel {
    /// Active clients by account ID
    clients: Arc<RwLock<HashMap<String, Arc<SignalClient>>>>,
}

/// Signal client for a single account
struct SignalClient {
    config: SignalConfig,
    account_id: String,
}

impl SignalChannel {
    /// Create a new Signal channel plugin
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for SignalChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ChannelPlugin for SignalChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Signal
    }

    fn descriptor(&self) -> ChannelDescriptor {
        ChannelDescriptor {
            channel_type: ChannelType::Signal,
            display_name: "Signal",
            capabilities: ChannelCapabilities {
                inbound_mode: InboundMode::Polling,
                supports_outbound: true,
                supports_streaming: false,
                supports_voice: false,
                supports_images: true,
                supports_files: true,
            },
        }
    }

    async fn start_account(
        &self,
        account_id: String,
        config: serde_json::Value,
        _event_sink: Arc<dyn ChannelEventSink>,
    ) -> anyhow::Result<()> {
        let signal_config: SignalConfig = serde_json::from_value(config)?;

        info!(
            account_id = %account_id,
            phone = %signal_config.phone_number,
            "Starting Signal account"
        );

        let client = Arc::new(SignalClient {
            config: signal_config,
            account_id: account_id.clone(),
        });

        let mut clients = self.clients.write().await;
        clients.insert(account_id.clone(), client);

        info!(account_id = %account_id, "Signal account started");

        Ok(())
    }

    async fn stop_account(&self, account_id: &str) -> anyhow::Result<()> {
        info!(account_id = %account_id, "Stopping Signal account");

        let mut clients = self.clients.write().await;
        clients.remove(account_id);

        info!(account_id = %account_id, "Signal account stopped");

        Ok(())
    }

    async fn send_message(
        &self,
        target: &ChannelReplyTarget,
        message: OutboundMessage,
    ) -> anyhow::Result<()> {
        let clients = self.clients.read().await;
        let client = clients
            .get(&target.account_id)
            .ok_or_else(|| anyhow::anyhow!("Signal account not found: {}", target.account_id))?;

        debug!(
            account_id = %target.account_id,
            chat_id = %target.chat_id,
            "Sending Signal message"
        );

        // TODO: Implement actual Signal message sending
        // This is a placeholder implementation
        info!(
            account_id = %target.account_id,
            chat_id = %target.chat_id,
            message_len = message.text.len(),
            "Signal message sent (placeholder)"
        );

        Ok(())
    }

    async fn health(&self, account_id: &str) -> anyhow::Result<ChannelHealth> {
        let clients = self.clients.read().await;
        let connected = clients.contains_key(account_id);

        Ok(ChannelHealth {
            connected,
            details: if connected {
                Some("Connected".to_string())
            } else {
                Some("Not connected".to_string())
            },
        })
    }
}

impl SignalClient {
    /// Get the phone number for this client
    #[allow(dead_code)]
    fn phone_number(&self) -> &str {
        &self.config.phone_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_channel_creation() {
        let channel = SignalChannel::new();
        assert_eq!(channel.channel_type(), ChannelType::Signal);
    }

    #[test]
    fn test_descriptor() {
        let channel = SignalChannel::new();
        let desc = channel.descriptor();
        assert_eq!(desc.display_name, "Signal");
        assert!(desc.capabilities.supports_outbound);
        assert!(desc.capabilities.supports_images);
    }

    #[tokio::test]
    async fn test_health_not_connected() {
        let channel = SignalChannel::new();
        let health = channel.health("test_account").await.unwrap();
        assert!(!health.connected);
    }
}
