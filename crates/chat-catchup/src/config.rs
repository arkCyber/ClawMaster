//! Configuration for chat catchup behavior

use {
    serde::{Deserialize, Serialize},
    std::time::Duration,
};

/// Configuration for chat catchup functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchupConfig {
    /// Maximum number of messages to fetch in one batch
    pub max_messages_per_batch: usize,
    /// Maximum time period to look back for messages
    pub max_lookback_period: Duration,
    /// Timeout for catchup operations
    pub catchup_timeout: Duration,
    /// Whether to enable message clustering
    pub enable_clustering: bool,
    /// Whether to enable message summarization
    pub enable_summarization: bool,
    /// Maximum context length in characters
    pub max_context_length: usize,
    /// Message filter configuration
    pub message_filter: MessageFilterConfig,
    /// Strategy for handling different catchup scenarios
    pub strategy: CatchupStrategy,
}

impl Default for CatchupConfig {
    fn default() -> Self {
        Self {
            max_messages_per_batch: 100,
            max_lookback_period: Duration::from_secs(86400), // 24 hours
            catchup_timeout: Duration::from_secs(30),
            enable_clustering: true,
            enable_summarization: true,
            max_context_length: 10000, // 10k characters
            message_filter: MessageFilterConfig::default(),
            strategy: CatchupStrategy::default(),
        }
    }
}

/// Configuration for message filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFilterConfig {
    /// Whether to filter out bot messages
    pub filter_bot_messages: bool,
    /// Whether to filter out system messages
    pub filter_system_messages: bool,
    /// Whether to filter out duplicate messages
    pub filter_duplicates: bool,
    /// Minimum message length to consider
    pub min_message_length: usize,
    /// Maximum message length to consider
    pub max_message_length: usize,
    /// List of users to always include their messages
    pub priority_users: Vec<String>,
    /// List of patterns to filter out (regex)
    pub filter_patterns: Vec<String>,
}

impl Default for MessageFilterConfig {
    fn default() -> Self {
        Self {
            filter_bot_messages: true,
            filter_system_messages: true,
            filter_duplicates: true,
            min_message_length: 1,
            max_message_length: 4000,
            priority_users: Vec::new(),
            filter_patterns: Vec::new(),
        }
    }
}

/// Strategy for handling different catchup scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatchupStrategy {
    /// Always fetch all messages since last interaction
    Full,
    /// Fetch messages and cluster them by topic
    Clustered,
    /// Fetch messages and summarize them
    Summarized,
    /// Adaptive strategy based on message count and time
    Adaptive {
        /// Threshold for switching to summarized mode
        summary_threshold: usize,
        /// Threshold for switching to clustered mode
        cluster_threshold: usize,
        /// Time threshold for considering messages "old"
        old_message_threshold: Duration,
    },
    /// Custom strategy with specific rules
    Custom {
        /// Custom strategy name
        name: String,
        /// Custom parameters
        parameters: std::collections::HashMap<String, serde_json::Value>,
    },
}

impl Default for CatchupStrategy {
    fn default() -> Self {
        Self::Adaptive {
            summary_threshold: 50,
            cluster_threshold: 20,
            old_message_threshold: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl CatchupStrategy {
    /// Determine the appropriate strategy based on message count and time elapsed
    pub fn determine_strategy(&self, message_count: usize, time_elapsed: Duration) -> CatchupMode {
        match self {
            CatchupStrategy::Full => CatchupMode::Full,
            CatchupStrategy::Clustered => CatchupMode::Clustered,
            CatchupStrategy::Summarized => CatchupMode::Summarized,
            CatchupStrategy::Adaptive {
                summary_threshold,
                cluster_threshold,
                old_message_threshold,
            } => {
                if message_count > *summary_threshold {
                    CatchupMode::Summarized
                } else if message_count > *cluster_threshold
                    || time_elapsed > *old_message_threshold
                {
                    CatchupMode::Clustered
                } else {
                    CatchupMode::Full
                }
            },
            CatchupStrategy::Custom { .. } => CatchupMode::Custom,
        }
    }
}

/// Mode for processing messages during catchup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatchupMode {
    /// Process all messages individually
    Full,
    /// Cluster messages by topic
    Clustered,
    /// Summarize messages
    Summarized,
    /// Custom processing mode
    Custom,
}
