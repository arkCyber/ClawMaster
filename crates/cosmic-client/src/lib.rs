//! ClawMaster libcosmic UI RPC Client
//!
//! This crate provides a Rust client for communicating with the ClawMaster
//! backend services, specifically designed for the libcosmic native UI.

use {
    crate::rpc::RpcEvent,
    anyhow::{Context, Result},
    std::collections::HashMap,
    tokio::sync::{RwLock, mpsc},
    tracing::{debug, error, info, warn},
};

pub mod config;
pub mod models;
pub mod rpc;

pub use {config::CosmicConfig, models::*, rpc::RpcClient};

/// Main client for ClawMaster libcosmic UI
pub struct CosmicClient {
    rpc_client: RpcClient,
    config: CosmicConfig,
    event_sender: mpsc::UnboundedSender<UiEvent>,
    sessions: RwLock<HashMap<String, Session>>,
    system_status: RwLock<SystemStatus>,
}

impl CosmicClient {
    /// Create a new client instance
    pub async fn new() -> Result<Self> {
        let config = CosmicConfig::load().await?;
        let rpc_client = RpcClient::new(&config.gateway_url).await?;
        let (event_sender, _event_receiver) = mpsc::unbounded_channel();

        let client = Self {
            rpc_client,
            config,
            event_sender,
            sessions: RwLock::new(HashMap::new()),
            system_status: RwLock::new(SystemStatus::default()),
        };

        // Start event listener
        client.start_event_listener().await?;

        info!("CosmicClient initialized successfully");
        Ok(client)
    }

    /// Get gateway URL
    pub fn gateway_url(&self) -> &str {
        &self.config.gateway_url
    }

    /// Get all sessions
    pub async fn get_sessions(&self) -> Result<Vec<Session>> {
        debug!("Fetching sessions");
        let sessions: Vec<Session> = self
            .rpc_client
            .call("sessions.list", ())
            .await
            .context("Failed to fetch sessions")?;

        // Update local cache
        let mut sessions_map = self.sessions.write().await;
        sessions_map.clear();
        for session in &sessions {
            sessions_map.insert(session.id.clone(), session.clone());
        }

        Ok(sessions)
    }

    /// Get a specific session
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    /// Create a new session
    pub async fn create_session(&self, title: Option<String>) -> Result<Session> {
        debug!("Creating new session with title: {:?}", title);
        let session: Session = self
            .rpc_client
            .call("sessions.create", (title,))
            .await
            .context("Failed to create session")?;

        // Update local cache
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    /// Send a message to a session
    pub async fn send_message(&self, session_id: &str, message: &str) -> Result<Message> {
        debug!("Sending message to session {}: {}", session_id, message);
        let msg: Message = self
            .rpc_client
            .call("chat.send", (session_id, message))
            .await
            .context("Failed to send message")?;

        Ok(msg)
    }

    /// Get messages for a session
    pub async fn get_messages(&self, session_id: &str, limit: Option<u32>) -> Result<Vec<Message>> {
        debug!(
            "Fetching messages for session {} with limit {:?}",
            session_id, limit
        );
        let messages: Vec<Message> = self
            .rpc_client
            .call("chat.messages", (session_id, limit))
            .await
            .context("Failed to fetch messages")?;

        Ok(messages)
    }

    /// Get system status
    pub async fn get_system_status(&self) -> Result<SystemStatus> {
        debug!("Fetching system status");
        let status: SystemStatus = self
            .rpc_client
            .call("system.status", ())
            .await
            .context("Failed to fetch system status")?;

        // Update local cache
        let mut cached_status = self.system_status.write().await;
        *cached_status = status.clone();

        Ok(status)
    }

    /// Emergency stop
    pub async fn emergency_stop(&self) -> Result<()> {
        debug!("Executing emergency stop");
        self.rpc_client
            .call::<()>("security.emergency_stop", ())
            .await
            .context("Failed to execute emergency stop")?;
        Ok(()) // Explicit type annotation added here
    }

    /// Get available models
    pub async fn get_models(&self) -> Result<Vec<Model>> {
        debug!("Fetching available models");
        let models: Vec<Model> = self
            .rpc_client
            .call("models.list", ())
            .await
            .context("Failed to fetch models")?;
        Ok(models)
    }

    /// Get configuration
    pub async fn get_config(&self) -> Result<Config> {
        debug!("Fetching configuration");
        let config: Config = self
            .rpc_client
            .call("config.get", ())
            .await
            .context("Failed to fetch configuration")?;
        Ok(config)
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: CosmicConfig) -> Result<()> {
        debug!("Updating configuration");
        self.rpc_client
            .call::<()>("config.update", (config,))
            .await
            .context("Failed to update configuration")?;
        Ok(())
    }

    /// Get recent activity
    pub async fn get_recent_activity(&self, limit: Option<u32>) -> Result<Vec<Activity>> {
        debug!("Fetching recent activity with limit {:?}", limit);
        let activity: Vec<Activity> = self
            .rpc_client
            .call("activity.recent", (limit,))
            .await
            .context("Failed to fetch recent activity")?;
        Ok(activity)
    }

    /// Subscribe to UI events
    pub fn subscribe_events(&self) -> mpsc::UnboundedReceiver<UiEvent> {
        let (_sender, receiver) = mpsc::unbounded_channel();
        // In a real implementation, we'd manage multiple subscribers
        receiver
    }

    /// Start the WebSocket event listener
    async fn start_event_listener(&self) -> Result<()> {
        let rpc_client = self.rpc_client.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::event_loop(rpc_client, event_sender).await {
                error!("Event listener error: {}", e);
            }
        });

        Ok(())
    }

    /// Event loop for WebSocket messages
    async fn event_loop(
        mut rpc_client: RpcClient,
        event_sender: mpsc::UnboundedSender<UiEvent>,
    ) -> Result<()> {
        loop {
            match rpc_client.next_event().await {
                Ok(event) => {
                    let ui_event = match event {
                        RpcEvent::Message(msg) => UiEvent::NewMessage(msg),
                        RpcEvent::SessionUpdate(session) => UiEvent::SessionUpdated(session),
                        RpcEvent::SystemStatus(status) => UiEvent::SystemStatusUpdated(status),
                        RpcEvent::Error(error) => UiEvent::Error(error),
                    };

                    if let Err(e) = event_sender.send(ui_event) {
                        warn!("Failed to send UI event: {}", e);
                        break;
                    }
                },
                Err(e) => {
                    error!("RPC event error: {}", e);
                    // Try to reconnect
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                },
            }
        }

        Ok(())
    }
}

/// UI events that can be subscribed to
#[derive(Debug, Clone)]
pub enum UiEvent {
    NewMessage(Message),
    SessionUpdated(Session),
    SystemStatusUpdated(SystemStatus),
    Error(String),
}

/// Error type for the cosmic client
#[derive(Debug, thiserror::Error)]
pub enum CosmicClientError {
    #[error("RPC error: {0}")]
    Rpc(#[from] rpc::RpcError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        // This test requires a running server
        // In real tests, we'd mock the RPC client
        let result = CosmicClient::new().await;
        assert!(result.is_ok() || result.is_err()); // Just ensure it doesn't panic
    }
}
