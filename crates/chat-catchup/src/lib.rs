//! Chat Catchup implementation inspired by MicroClaw
//! 
//! This crate provides functionality to catch up on group chat messages
//! since the last reply, enabling continuous conversation support.

pub mod config;
pub mod error;
pub mod catchup_engine;
pub mod message_processor;
pub mod context_builder;

pub use config::{CatchupConfig, MessageFilterConfig};
pub use error::{CatchupError, MessageProcessingError};
pub use catchup_engine::{ChatCatchup, CatchupResult, CatchupStrategy};
pub use message_processor::{MessageProcessor, MessageSummary, MessageCluster};
pub use context_builder::{ContextBuilder, ConversationContext};

use std::sync::Arc;
use async_trait::async_trait;

/// Main chat catchup interface
#[async_trait]
pub trait ChatCatchupInterface: Send + Sync {
    /// Catch up on messages in a channel for a specific user
    async fn catch_up(&self, channel_id: &str, user_id: &str) -> Result<CatchupResult, CatchupError>;
    
    /// Get the count of unread messages
    async fn get_unread_count(&self, channel_id: &str, user_id: &str) -> Result<usize, CatchupError>;
    
    /// Mark messages as read
    async fn mark_as_read(&self, channel_id: &str, user_id: &str, up_to_timestamp: u64) -> Result<(), CatchupError>;
}

/// Create a new chat catchup instance with default configuration
/// Note: This uses mock stores. In production, provide actual MessageStore and SessionStore implementations.
pub fn create_chat_catchup() -> Result<Arc<dyn ChatCatchupInterface>, CatchupError> {
    use catchup_engine::{MockMessageStore, MockSessionStore};
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    Ok(Arc::new(ChatCatchup::new(config, message_store, session_store)?))
}

/// Create a new chat catchup instance with custom configuration
/// Note: This uses mock stores. In production, provide actual MessageStore and SessionStore implementations.
pub fn create_chat_catchup_with_config(config: CatchupConfig) -> Result<Arc<dyn ChatCatchupInterface>, CatchupError> {
    use catchup_engine::{MockMessageStore, MockSessionStore};
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    Ok(Arc::new(ChatCatchup::new(config, message_store, session_store)?))
}
