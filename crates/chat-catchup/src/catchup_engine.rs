//! Main catchup engine for chat catchup functionality

use crate::config::{CatchupConfig, CatchupMode};
use crate::error::CatchupError;
use crate::message_processor::{MessageProcessor, ChatMessage};
use crate::context_builder::{ContextBuilder, ConversationContext};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::timeout;
use tracing::{debug, info, warn};

use super::ChatCatchupInterface;

/// Result of a catchup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchupResult {
    /// Catchup operation identifier
    pub id: String,
    /// Channel identifier
    pub channel_id: String,
    /// User identifier
    pub user_id: String,
    /// Number of messages processed
    pub messages_processed: usize,
    /// Number of messages filtered out
    pub messages_filtered: usize,
    /// Catchup mode used
    pub mode: CatchupMode,
    /// Built conversation context
    pub context: ConversationContext,
    /// Time taken for catchup operation
    pub processing_time: Duration,
    /// Whether there were unread messages
    pub had_unread: bool,
    /// Additional metadata
    pub metadata: CatchupMetadata,
}

/// Metadata for catchup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchupMetadata {
    /// Last read message timestamp (if known)
    pub last_read_at: Option<DateTime<Utc>>,
    /// Newest message timestamp
    pub newest_message_at: DateTime<Utc>,
    /// Oldest message timestamp processed
    pub oldest_message_at: DateTime<Utc>,
    /// Strategy used for catchup
    pub strategy: String,
    /// Additional metadata
    pub additional: std::collections::HashMap<String, String>,
}

/// Catchup strategy enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatchupStrategy {
    /// Full catchup - process all messages
    Full,
    /// Smart catchup - adapt based on message count and time
    Smart,
    /// Quick catchup - only recent messages
    Quick,
    /// Custom strategy
    Custom(String),
}

/// Main chat catchup implementation
pub struct ChatCatchup {
    config: CatchupConfig,
    message_processor: Arc<MessageProcessor>,
    context_builder: Arc<ContextBuilder>,
    // In a real implementation, these would be actual database/session clients
    message_store: Arc<dyn MessageStore>,
    session_store: Arc<dyn SessionStore>,
}

/// Trait for message storage operations
#[async_trait::async_trait]
pub trait MessageStore: Send + Sync {
    /// Get messages since a specific timestamp
    async fn get_messages_since(
        &self,
        channel_id: &str,
        since: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<ChatMessage>, CatchupError>;
    
    /// Get latest message timestamp
    async fn get_latest_message_timestamp(&self, channel_id: &str) -> Result<Option<DateTime<Utc>>, CatchupError>;
    
    /// Get message count in time range
    async fn get_message_count(
        &self,
        channel_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<usize, CatchupError>;
}

/// Trait for session storage operations
#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    /// Get last read timestamp for user in channel
    async fn get_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<Option<DateTime<Utc>>, CatchupError>;
    
    /// Update last read timestamp for user in channel
    async fn update_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<(), CatchupError>;
    
    /// Get unread message count
    async fn get_unread_count(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<usize, CatchupError>;
}

impl ChatCatchup {
    /// Create a new chat catchup instance
    pub fn new(
        config: CatchupConfig,
        message_store: Arc<dyn MessageStore>,
        session_store: Arc<dyn SessionStore>,
    ) -> Result<Self, CatchupError> {
        let message_processor = Arc::new(MessageProcessor::new(config.message_filter.clone())
            .map_err(|e| CatchupError::ConfigurationError(e.to_string()))?);
        
        let context_builder = Arc::new(ContextBuilder::new()
            .with_max_length(config.max_context_length));

        Ok(Self {
            config,
            message_processor,
            context_builder,
            message_store,
            session_store,
        })
    }

    /// Determine catchup strategy based on message count and time
    async fn determine_strategy(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<(CatchupMode, DateTime<Utc>), CatchupError> {
        let last_read = self.session_store.get_last_read_timestamp(channel_id, user_id).await?;
        let now = Utc::now();
        
        let since = last_read.unwrap_or_else(|| now - self.config.max_lookback_period);
        
        // Get message count to determine strategy
        let message_count = self.message_store.get_message_count(channel_id, since, now).await?;
        let time_elapsed = now.signed_duration_since(since);
        
        let mode = self.config.strategy.determine_strategy(message_count, time_elapsed.to_std().unwrap_or_default());
        
        debug!("Determined catchup strategy: {:?} for {} messages over {:?}", 
               mode, message_count, time_elapsed);
        
        Ok((mode, since))
    }

    /// Process messages based on catchup mode
    async fn process_messages(
        &self,
        messages: Vec<ChatMessage>,
        mode: CatchupMode,
        channel_id: String,
        user_id: String,
    ) -> Result<ConversationContext, CatchupError> {
        match mode {
            CatchupMode::Full => {
                self.context_builder
                    .build_full_context(messages, channel_id, user_id)
            }
            CatchupMode::Clustered => {
                let clusters = self.message_processor.cluster_messages(messages)?;
                self.context_builder
                    .build_clustered_context(clusters, channel_id, user_id)
            }
            CatchupMode::Summarized => {
                let summary = self.message_processor.summarize_messages(messages)?;
                self.context_builder
                    .build_summarized_context(summary, channel_id, user_id)
            }
            CatchupMode::Custom => {
                // For custom mode, default to full processing
                self.context_builder
                    .build_full_context(messages, channel_id, user_id)
            }
        }
    }

    /// Create catchup metadata
    fn create_metadata(
        &self,
        channel_id: &str,
        user_id: &str,
        last_read: Option<DateTime<Utc>>,
        newest_message: DateTime<Utc>,
        oldest_message: DateTime<Utc>,
        strategy: &str,
    ) -> CatchupMetadata {
        let mut additional = std::collections::HashMap::new();
        additional.insert("config_version".to_string(), "1.0".to_string());
        additional.insert("processor_version".to_string(), "1.0".to_string());

        CatchupMetadata {
            last_read_at: last_read,
            newest_message_at: newest_message,
            oldest_message_at: oldest_message,
            strategy: strategy.to_string(),
            additional,
        }
    }
}

#[async_trait::async_trait]
impl ChatCatchupInterface for ChatCatchup {
    async fn catch_up(&self, channel_id: &str, user_id: &str) -> Result<CatchupResult, CatchupError> {
        let start_time = std::time::Instant::now();
        let catchup_id = uuid::Uuid::new_v4().to_string();
        
        info!("Starting catchup for user {} in channel {}", user_id, channel_id);

        // Determine strategy and get since timestamp
        let (mode, since) = timeout(self.config.catchup_timeout, 
            self.determine_strategy(channel_id, user_id)
        ).await.map_err(|_| CatchupError::TimeoutExceeded(self.config.catchup_timeout))??;

        // Get messages since last read
        let raw_messages = timeout(self.config.catchup_timeout,
            self.message_store.get_messages_since(channel_id, since, self.config.max_messages_per_batch)
        ).await.map_err(|_| CatchupError::TimeoutExceeded(self.config.catchup_timeout))??;

        let raw_count = raw_messages.len();
        debug!("Retrieved {} raw messages", raw_count);

        // Filter messages
        let filtered_messages = self.message_processor.filter_messages(raw_messages)
            .map_err(|e| CatchupError::MessageProcessingFailed(e))?;
        let filtered_count = filtered_messages.len();
        debug!("Filtered to {} messages", filtered_count);

        // Get latest message timestamp
        let latest_timestamp = self.message_store.get_latest_message_timestamp(channel_id).await?
            .unwrap_or_else(|| Utc::now());

        let oldest_timestamp = filtered_messages.first()
            .map(|m| m.timestamp)
            .unwrap_or_else(|| Utc::now());

        // Process messages and build context
        let context = if filtered_messages.is_empty() {
            // Create empty context
            self.context_builder.build_full_context(
                Vec::new(),
                channel_id.to_string(),
                user_id.to_string(),
            )?
        } else {
            self.process_messages(filtered_messages.clone(), mode.clone(), 
                                 channel_id.to_string(), user_id.to_string()).await?
        };

        // Get last read timestamp for metadata
        let last_read = self.session_store.get_last_read_timestamp(channel_id, user_id).await?;

        // Create metadata
        let metadata = self.create_metadata(
            channel_id,
            user_id,
            last_read,
            latest_timestamp,
            oldest_timestamp,
            &format!("{:?}", mode),
        );

        let processing_time = start_time.elapsed();
        let had_unread = filtered_count > 0;

        // Update last read timestamp to now
        if had_unread {
            if let Err(e) = self.session_store.update_last_read_timestamp(channel_id, user_id, Utc::now()).await {
                warn!("Failed to update last read timestamp: {}", e);
            }
        }

        let result = CatchupResult {
            id: catchup_id,
            channel_id: channel_id.to_string(),
            user_id: user_id.to_string(),
            messages_processed: filtered_count,
            messages_filtered: raw_count - filtered_count,
            mode,
            context,
            processing_time: Duration::from_std(processing_time).unwrap_or_default(),
            had_unread,
            metadata,
        };

        info!("Catchup completed for user {} in channel {}: {} messages processed in {:?}", 
              user_id, channel_id, result.messages_processed, processing_time);

        Ok(result)
    }

    async fn get_unread_count(&self, channel_id: &str, user_id: &str) -> Result<usize, CatchupError> {
        self.session_store.get_unread_count(channel_id, user_id).await
    }

    async fn mark_as_read(&self, channel_id: &str, user_id: &str, up_to_timestamp: u64) -> Result<(), CatchupError> {
        let up_to = DateTime::from_timestamp(up_to_timestamp as i64, 0)
            .ok_or_else(|| CatchupError::InvalidTimestamp(up_to_timestamp))?;
        self.session_store.update_last_read_timestamp(channel_id, user_id, up_to).await
    }
}

// Mock implementations for testing
pub struct MockMessageStore {
    messages: std::sync::RwLock<Vec<ChatMessage>>,
}

impl MockMessageStore {
    pub fn new() -> Self {
        Self {
            messages: std::sync::RwLock::new(Vec::new()),
        }
    }

    pub fn add_message(&self, message: ChatMessage) {
        self.messages.write().unwrap().push(message);
    }
}

#[async_trait::async_trait]
impl MessageStore for MockMessageStore {
    async fn get_messages_since(
        &self,
        _channel_id: &str,
        since: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<ChatMessage>, CatchupError> {
        let messages = self.messages.read().unwrap();
        Ok(messages
            .iter()
            .filter(|m| m.timestamp > since)
            .take(limit)
            .cloned()
            .collect())
    }

    async fn get_latest_message_timestamp(&self, _channel_id: &str) -> Result<Option<DateTime<Utc>>, CatchupError> {
        let messages = self.messages.read().unwrap();
        Ok(messages.last().map(|m| m.timestamp))
    }

    async fn get_message_count(
        &self,
        _channel_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<usize, CatchupError> {
        let messages = self.messages.read().unwrap();
        Ok(messages
            .iter()
            .filter(|m| m.timestamp > start && m.timestamp <= end)
            .count())
    }
}

pub struct MockSessionStore {
    last_read: std::sync::RwLock<std::collections::HashMap<(String, String), DateTime<Utc>>>,
}

impl MockSessionStore {
    pub fn new() -> Self {
        Self {
            last_read: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl SessionStore for MockSessionStore {
    async fn get_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<Option<DateTime<Utc>>, CatchupError> {
        let last_read = self.last_read.read().unwrap();
        Ok(last_read.get(&(channel_id.to_string(), user_id.to_string())).copied())
    }

    async fn update_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<(), CatchupError> {
        let mut last_read = self.last_read.write().unwrap();
        last_read.insert((channel_id.to_string(), user_id.to_string()), timestamp);
        Ok(())
    }

    async fn get_unread_count(&self, _channel_id: &str, _user_id: &str) -> Result<usize, CatchupError> {
        // Mock implementation
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CatchupConfig, MessageFilterConfig};

    #[tokio::test]
    async fn test_catchup_no_messages() {
        let config = CatchupConfig::default();
        let message_store = Arc::new(MockMessageStore::new());
        let session_store = Arc::new(MockSessionStore::new());
        
        let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
        
        let result = catchup.catch_up("channel1", "user1").await.unwrap();
        
        assert_eq!(result.messages_processed, 0);
        assert!(!result.had_unread);
    }

    #[tokio::test]
    async fn test_catchup_with_messages() {
        let config = CatchupConfig::default();
        let message_store = Arc::new(MockMessageStore::new());
        let session_store = Arc::new(MockSessionStore::new());
        
        // Add some test messages
        let message = ChatMessage {
            id: "1".to_string(),
            channel_id: "channel1".to_string(),
            user_id: "user2".to_string(),
            username: "User2".to_string(),
            content: "Hello world".to_string(),
            timestamp: Utc::now(),
            is_bot: false,
            is_system: false,
            message_type: crate::message_processor::MessageType::Text,
            metadata: std::collections::HashMap::new(),
        };
        
        message_store.add_message(message);
        
        let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
        
        let result = catchup.catch_up("channel1", "user1").await.unwrap();
        
        assert_eq!(result.messages_processed, 1);
        assert!(result.had_unread);
        assert_eq!(result.context.participants.len(), 1);
    }
}
