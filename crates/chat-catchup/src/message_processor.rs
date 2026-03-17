//! Message processing for chat catchup functionality

use {
    crate::error::{MessageProcessingError, MessageResult},
    chrono::{DateTime, Utc},
    regex::Regex,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

/// Represents a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Unique message identifier
    pub id: String,
    /// Channel identifier
    pub channel_id: String,
    /// User identifier
    pub user_id: String,
    /// Username (display name)
    pub username: String,
    /// Message content
    pub content: String,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    /// Whether this is a bot message
    pub is_bot: bool,
    /// Whether this is a system message
    pub is_system: bool,
    /// Message type (e.g., text, image, file)
    pub message_type: MessageType,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    File,
    Audio,
    Video,
    System,
    Other(String),
}

/// Summary of a message or message cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    /// Summary text
    pub summary: String,
    /// Number of messages summarized
    pub message_count: usize,
    /// Time range covered
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// Key participants
    pub participants: Vec<String>,
    /// Key topics or keywords
    pub topics: Vec<String>,
    /// Sentiment analysis (if available)
    pub sentiment: Option<SentimentAnalysis>,
}

/// Sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    /// Overall sentiment score (-1.0 to 1.0)
    pub score: f64,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Emotions detected
    pub emotions: Vec<String>,
}

/// Cluster of related messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCluster {
    /// Cluster identifier
    pub id: String,
    /// Messages in this cluster
    pub messages: Vec<ChatMessage>,
    /// Cluster topic or theme
    pub topic: String,
    /// Cluster confidence score
    pub confidence: f64,
    /// Time range of the cluster
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
    /// Key participants in this cluster
    pub participants: Vec<String>,
}

/// Message processor for filtering, clustering, and summarizing
pub struct MessageProcessor {
    filter_config: crate::config::MessageFilterConfig,
    compiled_patterns: Vec<Regex>,
}

impl MessageProcessor {
    /// Create a new message processor
    pub fn new(filter_config: crate::config::MessageFilterConfig) -> MessageResult<Self> {
        let compiled_patterns = filter_config
            .filter_patterns
            .iter()
            .map(|pattern| {
                Regex::new(pattern).map_err(|e| {
                    MessageProcessingError::InvalidFormat(format!(
                        "Invalid regex pattern '{}': {}",
                        pattern, e
                    ))
                })
            })
            .collect::<MessageResult<Vec<_>>>()?;

        Ok(Self {
            filter_config,
            compiled_patterns,
        })
    }

    /// Filter messages based on configuration
    pub fn filter_messages(&self, messages: Vec<ChatMessage>) -> MessageResult<Vec<ChatMessage>> {
        let mut filtered = Vec::new();
        let mut seen_messages = std::collections::HashSet::new();

        for message in messages {
            // Check message length
            if message.content.len() < self.filter_config.min_message_length
                || message.content.len() > self.filter_config.max_message_length
            {
                continue;
            }

            // Filter bot messages
            if self.filter_config.filter_bot_messages && message.is_bot {
                continue;
            }

            // Filter system messages
            if self.filter_config.filter_system_messages && message.is_system {
                continue;
            }

            // Filter duplicates
            if self.filter_config.filter_duplicates {
                let message_key = format!(
                    "{}:{}:{}",
                    message.user_id, message.content, message.timestamp
                );
                if seen_messages.contains(&message_key) {
                    continue;
                }
                seen_messages.insert(message_key);
            }

            // Filter by patterns
            let should_exclude = self
                .compiled_patterns
                .iter()
                .any(|pattern| pattern.is_match(&message.content));
            if should_exclude {
                continue;
            }

            // Always include priority users
            let is_priority = self.filter_config.priority_users.contains(&message.user_id);
            if is_priority || !self.should_filter_content(&message) {
                filtered.push(message);
            }
        }

        Ok(filtered)
    }

    /// Cluster messages by topic
    pub fn cluster_messages(
        &self,
        messages: Vec<ChatMessage>,
    ) -> MessageResult<Vec<MessageCluster>> {
        if messages.is_empty() {
            return Ok(Vec::new());
        }

        let mut clusters = Vec::new();
        let mut current_cluster_messages = Vec::new();
        let mut last_message_time = messages[0].timestamp;

        for message in messages {
            // Simple time-based clustering (messages within 5 minutes are in same cluster)
            let time_diff = message.timestamp.signed_duration_since(last_message_time);

            if time_diff.num_minutes() > 5 && !current_cluster_messages.is_empty() {
                // Create cluster from accumulated messages
                if let Ok(cluster) =
                    self.create_cluster(current_cluster_messages.drain(..).collect())
                {
                    clusters.push(cluster);
                }
            }

            current_cluster_messages.push(message.clone());
            last_message_time = message.timestamp;
        }

        // Create final cluster
        if !current_cluster_messages.is_empty() {
            if let Ok(cluster) = self.create_cluster(current_cluster_messages) {
                clusters.push(cluster);
            }
        }

        Ok(clusters)
    }

    /// Summarize messages
    pub fn summarize_messages(&self, messages: Vec<ChatMessage>) -> MessageResult<MessageSummary> {
        if messages.is_empty() {
            return Err(MessageProcessingError::SummarizationFailed(
                "No messages to summarize".to_string(),
            ));
        }

        let message_count = messages.len();
        let start_time = messages[0].timestamp;
        let end_time = messages.last().unwrap().timestamp;

        let participants: std::collections::HashSet<_> =
            messages.iter().map(|m| m.username.clone()).collect();

        let participants: Vec<String> = participants.into_iter().collect();

        // Simple summarization logic (in production, this would use LLM)
        let summary = if message_count == 1 {
            format!("{} said: {}", messages[0].username, messages[0].content)
        } else if message_count <= 5 {
            let message_contents: Vec<String> = messages
                .iter()
                .map(|m| format!("{}: {}", m.username, m.content))
                .collect();
            message_contents.join("; ")
        } else {
            format!(
                "{} messages discussed by {} participants",
                message_count,
                participants.len()
            )
        };

        // Extract simple topics (in production, this would be more sophisticated)
        let topics = self.extract_topics(&messages);

        Ok(MessageSummary {
            summary,
            message_count,
            time_range: (start_time, end_time),
            participants,
            topics,
            sentiment: None, // Would implement sentiment analysis in production
        })
    }

    /// Create a message cluster from messages
    fn create_cluster(&self, messages: Vec<ChatMessage>) -> MessageResult<MessageCluster> {
        if messages.is_empty() {
            return Err(MessageProcessingError::ClusteringFailed(
                "No messages in cluster".to_string(),
            ));
        }

        let start_time = messages[0].timestamp;
        let end_time = messages.last().unwrap().timestamp;

        let participants: std::collections::HashSet<_> =
            messages.iter().map(|m| m.username.clone()).collect();

        let participants: Vec<String> = participants.into_iter().collect();

        // Simple topic extraction
        let topic = if messages.len() == 1 {
            messages[0].content.chars().take(50).collect()
        } else {
            format!("Discussion with {} messages", messages.len())
        };

        Ok(MessageCluster {
            id: uuid::Uuid::new_v4().to_string(),
            messages,
            topic,
            confidence: 0.8, // Would calculate actual confidence
            time_range: (start_time, end_time),
            participants,
        })
    }

    /// Check if content should be filtered
    fn should_filter_content(&self, message: &ChatMessage) -> bool {
        // Simple content filtering - in production, this would be more sophisticated
        let content = message.content.to_lowercase();
        content.contains("spam") || content.contains("advertisement")
    }

    /// Extract topics from messages
    fn extract_topics(&self, messages: &[ChatMessage]) -> Vec<String> {
        let mut topics = Vec::new();
        let mut word_counts = HashMap::new();

        for message in messages {
            // Simple word extraction (in production, would use NLP)
            let words: Vec<String> = message
                .content
                .to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3) // Only words longer than 3 characters
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .collect();

            for word in words {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }

        // Get top 5 words as topics
        let mut sorted_words: Vec<_> = word_counts.into_iter().collect();
        sorted_words.sort_by(|a, b| b.1.cmp(&a.1));

        for (word, _count) in sorted_words.into_iter().take(5) {
            topics.push(word);
        }

        topics
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::config::MessageFilterConfig};

    #[test]
    fn test_message_filtering() {
        let config = MessageFilterConfig {
            filter_bot_messages: true,
            filter_system_messages: true,
            filter_duplicates: true,
            min_message_length: 1,
            max_message_length: 1000,
            priority_users: vec!["user1".to_string()],
            filter_patterns: vec!["spam".to_string()],
        };

        let processor = MessageProcessor::new(config).unwrap();

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
            ChatMessage {
                id: "2".to_string(),
                channel_id: "channel1".to_string(),
                user_id: "bot".to_string(),
                username: "Bot".to_string(),
                content: "This is spam".to_string(),
                timestamp: Utc::now(),
                is_bot: true,
                is_system: false,
                message_type: MessageType::Text,
                metadata: HashMap::new(),
            },
        ];

        let filtered = processor.filter_messages(messages).unwrap();
        assert_eq!(filtered.len(), 1); // Only the non-bot, non-spam message
        assert_eq!(filtered[0].username, "User1");
    }

    #[test]
    fn test_message_summarization() {
        let config = MessageFilterConfig::default();
        let processor = MessageProcessor::new(config).unwrap();

        let messages = vec![
            ChatMessage {
                id: "1".to_string(),
                channel_id: "channel1".to_string(),
                user_id: "user1".to_string(),
                username: "User1".to_string(),
                content: "Hello everyone".to_string(),
                timestamp: Utc::now(),
                is_bot: false,
                is_system: false,
                message_type: MessageType::Text,
                metadata: HashMap::new(),
            },
            ChatMessage {
                id: "2".to_string(),
                channel_id: "channel1".to_string(),
                user_id: "user2".to_string(),
                username: "User2".to_string(),
                content: "Hi there!".to_string(),
                timestamp: Utc::now(),
                is_bot: false,
                is_system: false,
                message_type: MessageType::Text,
                metadata: HashMap::new(),
            },
        ];

        let summary = processor.summarize_messages(messages).unwrap();
        assert_eq!(summary.message_count, 2);
        assert_eq!(summary.participants.len(), 2);
        assert!(summary.summary.contains("User1"));
        assert!(summary.summary.contains("User2"));
    }
}
