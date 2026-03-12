//! Context builder for creating conversation context from catchup results

use crate::error::{CatchupError, MessageProcessingError};
use crate::message_processor::{MessageCluster, MessageSummary, ChatMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Built conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Context identifier
    pub id: String,
    /// Channel identifier
    pub channel_id: String,
    /// User identifier
    pub user_id: String,
    /// Context type
    pub context_type: ContextType,
    /// Formatted context string
    pub context_string: String,
    /// Context metadata
    pub metadata: ContextMetadata,
    /// Time range covered
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// Total messages processed
    pub message_count: usize,
    /// Key participants
    pub participants: Vec<String>,
    /// Key topics discussed
    pub topics: Vec<String>,
}

/// Type of context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextType {
    /// Full context with all messages
    Full {
        messages: Vec<ChatMessage>,
    },
    /// Clustered context by topic
    Clustered {
        clusters: Vec<MessageCluster>,
    },
    /// Summarized context
    Summarized {
        summary: MessageSummary,
    },
    /// Hybrid context
    Hybrid {
        recent_messages: Vec<ChatMessage>,
        older_summary: MessageSummary,
    },
}

/// Context metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    /// Context creation time
    pub created_at: DateTime<Utc>,
    /// Context length in characters
    pub length: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Strategy used
    pub strategy: String,
    /// Additional metadata
    pub additional: HashMap<String, String>,
}

/// Builder for creating conversation context
pub struct ContextBuilder {
    max_context_length: usize,
    include_timestamps: bool,
    include_usernames: bool,
    context_separator: String,
    message_format: MessageFormat,
}

/// Message formatting options
#[derive(Debug, Clone)]
pub enum MessageFormat {
    /// Simple format: "username: message"
    Simple,
    /// Detailed format: "[timestamp] username: message"
    Detailed,
    /// Compact format: "message" (no usernames)
    Compact,
    /// Custom format with template
    Custom(String),
}

impl ContextBuilder {
    /// Create a new context builder with default settings
    pub fn new() -> Self {
        Self {
            max_context_length: 10000,
            include_timestamps: true,
            include_usernames: true,
            context_separator: "\n".to_string(),
            message_format: MessageFormat::Detailed,
        }
    }

    /// Set maximum context length
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_context_length = max_length;
        self
    }

    /// Set whether to include timestamps
    pub fn with_timestamps(mut self, include: bool) -> Self {
        self.include_timestamps = include;
        self
    }

    /// Set whether to include usernames
    pub fn with_usernames(mut self, include: bool) -> Self {
        self.include_usernames = include;
        self
    }

    /// Set context separator
    pub fn with_separator(mut self, separator: String) -> Self {
        self.context_separator = separator;
        self
    }

    /// Set message format
    pub fn with_format(mut self, format: MessageFormat) -> Self {
        self.message_format = format;
        self
    }

    /// Build context from full messages
    pub fn build_full_context(
        &self,
        messages: Vec<ChatMessage>,
        channel_id: String,
        user_id: String,
    ) -> Result<ConversationContext, CatchupError> {
        let start_time = std::time::Instant::now();
        
        if messages.is_empty() {
            return Err(CatchupError::MessageProcessingFailed(
                MessageProcessingError::ContextBuildingFailed("No messages to build context".to_string())
            ));
        }

        let context_string = self.format_messages(&messages)?;
        let time_range = (messages[0].timestamp, messages.last().unwrap().timestamp);
        
        let participants: std::collections::HashSet<_> = messages
            .iter()
            .map(|m| m.username.clone())
            .collect();
        let participants: Vec<String> = participants.into_iter().collect();

        let topics = self.extract_topics(&messages);

        let metadata = ContextMetadata {
            created_at: Utc::now(),
            length: context_string.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy: "full".to_string(),
            additional: HashMap::new(),
        };

        let message_count = messages.len();
        
        Ok(ConversationContext {
            id: uuid::Uuid::new_v4().to_string(),
            channel_id,
            user_id,
            context_type: ContextType::Full { messages },
            context_string,
            metadata,
            time_range,
            message_count,
            participants,
            topics,
        })
    }

    /// Build context from clustered messages
    pub fn build_clustered_context(
        &self,
        clusters: Vec<MessageCluster>,
        channel_id: String,
        user_id: String,
    ) -> Result<ConversationContext, CatchupError> {
        let start_time = std::time::Instant::now();
        
        if clusters.is_empty() {
            return Err(CatchupError::MessageProcessingFailed(
                MessageProcessingError::ContextBuildingFailed("No clusters to build context".to_string())
            ));
        }

        let mut context_parts = Vec::new();
        let mut all_messages = Vec::new();
        let mut all_participants = std::collections::HashSet::new();

        for cluster in &clusters {
            let cluster_header = format!("📌 Topic: {}", cluster.topic);
            context_parts.push(cluster_header);
            
            let cluster_messages = self.format_messages(&cluster.messages)?;
            context_parts.push(cluster_messages);
            
            all_messages.extend(cluster.messages.clone());
            all_participants.extend(cluster.participants.iter().cloned());
        }

        let context_string = context_parts.join(&format!("{}{}", self.context_separator, self.context_separator));
        let time_range = (
            all_messages[0].timestamp,
            all_messages.last().unwrap().timestamp,
        );

        let topics: Vec<String> = clusters.iter().map(|c| c.topic.clone()).collect();
        let participants: Vec<String> = all_participants.into_iter().collect();

        let metadata = ContextMetadata {
            created_at: Utc::now(),
            length: context_string.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy: "clustered".to_string(),
            additional: HashMap::new(),
        };

        Ok(ConversationContext {
            id: uuid::Uuid::new_v4().to_string(),
            channel_id,
            user_id,
            context_type: ContextType::Clustered { clusters },
            context_string,
            metadata,
            time_range,
            message_count: all_messages.len(),
            participants,
            topics,
        })
    }

    /// Build context from summary
    pub fn build_summarized_context(
        &self,
        summary: MessageSummary,
        channel_id: String,
        user_id: String,
    ) -> Result<ConversationContext, CatchupError> {
        let start_time = std::time::Instant::now();

        let context_string = format!(
            "📝 Conversation Summary ({} messages):\n{}\n\n👥 Participants: {}\n🏷️ Topics: {}",
            summary.message_count,
            summary.summary,
            summary.participants.join(", "),
            summary.topics.join(", ")
        );

        let metadata = ContextMetadata {
            created_at: Utc::now(),
            length: context_string.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy: "summarized".to_string(),
            additional: HashMap::new(),
        };

        let time_range = summary.time_range;
        let message_count = summary.message_count;
        let participants = summary.participants.clone();
        let topics = summary.topics.clone();
        
        Ok(ConversationContext {
            id: uuid::Uuid::new_v4().to_string(),
            channel_id,
            user_id,
            context_type: ContextType::Summarized { summary },
            context_string,
            metadata,
            time_range,
            message_count,
            participants,
            topics,
        })
    }

    /// Build hybrid context (recent messages + older summary)
    pub fn build_hybrid_context(
        &self,
        recent_messages: Vec<ChatMessage>,
        older_summary: MessageSummary,
        channel_id: String,
        user_id: String,
    ) -> Result<ConversationContext, CatchupError> {
        let start_time = std::time::Instant::now();

        let recent_context = self.format_messages(&recent_messages)?;
        
        let context_string = format!(
            "📝 Older Conversation Summary:\n{}\n\n💬 Recent Messages:\n{}",
            older_summary.summary,
            recent_context
        );

        let time_range = (older_summary.time_range.0, recent_messages.last().unwrap().timestamp);
        
        let mut participants = older_summary.participants.clone();
        for message in &recent_messages {
            if !participants.contains(&message.username) {
                participants.push(message.username.clone());
            }
        }

        let metadata = ContextMetadata {
            created_at: Utc::now(),
            length: context_string.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy: "hybrid".to_string(),
            additional: HashMap::new(),
        };

        let message_count = older_summary.message_count + recent_messages.len();
        let topics = older_summary.topics.clone();
        
        Ok(ConversationContext {
            id: uuid::Uuid::new_v4().to_string(),
            channel_id,
            user_id,
            context_type: ContextType::Hybrid {
                recent_messages,
                older_summary,
            },
            context_string,
            metadata,
            time_range,
            message_count,
            participants,
            topics,
        })
    }

    /// Format messages according to the configured format
    fn format_messages(&self, messages: &[ChatMessage]) -> Result<String, CatchupError> {
        let mut formatted_parts = Vec::new();
        let mut total_length = 0;

        for message in messages {
            let formatted = match &self.message_format {
                MessageFormat::Simple => {
                    if self.include_usernames {
                        format!("{}: {}", message.username, message.content)
                    } else {
                        message.content.clone()
                    }
                }
                MessageFormat::Detailed => {
                    let mut parts = Vec::new();
                    if self.include_timestamps {
                        parts.push(format!("[{}]", message.timestamp.format("%H:%M")));
                    }
                    if self.include_usernames {
                        parts.push(format!("{}:", message.username));
                    }
                    parts.push(message.content.clone());
                    parts.join(" ")
                }
                MessageFormat::Compact => message.content.clone(),
                MessageFormat::Custom(template) => {
                    template
                        .replace("{username}", &message.username)
                        .replace("{timestamp}", &message.timestamp.to_rfc3339())
                        .replace("{content}", &message.content)
                }
            };

            // Check if adding this message would exceed the limit
            if total_length + formatted.len() > self.max_context_length {
                break;
            }

            total_length += formatted.len();
            formatted_parts.push(formatted);
        }

        Ok(formatted_parts.join(&self.context_separator))
    }

    /// Extract topics from messages
    fn extract_topics(&self, messages: &[ChatMessage]) -> Vec<String> {
        let mut word_counts = HashMap::new();

        for message in messages {
            let words: Vec<String> = message
                .content
                .to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .collect();

            for word in words {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }

        let mut sorted_words: Vec<_> = word_counts.into_iter().collect();
        sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
        
        sorted_words
            .into_iter()
            .take(5)
            .map(|(word, _)| word)
            .collect()
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message_processor::{ChatMessage, MessageType};

    #[test]
    fn test_full_context_building() {
        let builder = ContextBuilder::new().with_max_length(1000);
        
        let messages = vec![
            ChatMessage {
                id: "1".to_string(),
                channel_id: "channel1".to_string(),
                user_id: "user1".to_string(),
                username: "User1".to_string(),
                content: "Hello world".to_string(),
                timestamp: Utc::now(),
                is_bot: false,
                is_system: false,
                message_type: MessageType::Text,
                metadata: HashMap::new(),
            },
        ];

        let context = builder.build_full_context(
            messages,
            "channel1".to_string(),
            "user1".to_string(),
        ).unwrap();

        assert_eq!(context.message_count, 1);
        assert_eq!(context.participants.len(), 1);
        assert!(context.context_string.contains("User1"));
        assert!(context.context_string.contains("Hello world"));
    }

    #[test]
    fn test_context_length_limit() {
        let builder = ContextBuilder::new().with_max_length(50);
        
        let messages = vec![
            ChatMessage {
                id: "1".to_string(),
                channel_id: "channel1".to_string(),
                user_id: "user1".to_string(),
                username: "User1".to_string(),
                content: "This is a very long message that should exceed the limit".to_string(),
                timestamp: Utc::now(),
                is_bot: false,
                is_system: false,
                message_type: MessageType::Text,
                metadata: HashMap::new(),
            },
        ];

        let context = builder.build_full_context(
            messages,
            "channel1".to_string(),
            "user1".to_string(),
        ).unwrap();

        assert!(context.context_string.len() <= 50);
    }
}
