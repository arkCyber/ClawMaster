//! Integration tests for chat catchup functionality

use clawmaster_chat_catchup::{create_chat_catchup_with_config, CatchupConfig, CatchupStrategy, ChatCatchupInterface};
use clawmaster_chat_catchup::catchup_engine::{MockMessageStore, MockSessionStore, ChatCatchup};
use clawmaster_chat_catchup::message_processor::{ChatMessage, MessageType};
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::test]
async fn test_catchup_adaptive_strategy() {
    let config = CatchupConfig {
        max_messages_per_batch: 100,
        max_lookback_period: std::time::Duration::from_secs(86400),
        catchup_timeout: std::time::Duration::from_secs(10),
        enable_clustering: true,
        enable_summarization: true,
        max_context_length: 10000,
        message_filter: Default::default(),
        strategy: CatchupStrategy::Adaptive {
            summary_threshold: 10,
            cluster_threshold: 5,
            old_message_threshold: std::time::Duration::from_secs(3600),
        },
    };

    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    // Add many messages to trigger summary mode
    for i in 0..15 {
        let message = ChatMessage {
            id: format!("msg_{}", i),
            channel_id: "channel1".to_string(),
            user_id: "user2".to_string(),
            username: "User2".to_string(),
            content: format!("Message {}", i),
            timestamp: Utc::now() - std::time::Duration::from_secs((15 - i) * 60),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        };
        message_store.add_message(message);
    }

    let catchup = create_chat_catchup_with_config(config);
    let catchup_engine = clawmaster_chat_catchup::catchup_engine::ChatCatchup::new(
        catchup.get_config(),
        message_store,
        session_store,
    ).unwrap();

    let result = catchup_engine.catch_up("channel1", "user1").await.unwrap();

    assert_eq!(result.messages_processed, 15);
    assert!(result.had_unread);
    
    // Should use summarized mode due to high message count
    match result.context.context_type {
        clawmaster_chat_catchup::context_builder::ContextType::Summarized { .. } => {
            // Expected
        }
        _ => panic!("Expected summarized context type"),
    }
}

#[tokio::test]
async fn test_catchup_message_filtering() {
    let mut filter_config = clawmaster_chat_catchup::config::MessageFilterConfig::default();
    filter_config.filter_bot_messages = true;
    filter_config.filter_patterns = vec!["spam".to_string()];

    let config = CatchupConfig {
        max_messages_per_batch: 100,
        max_lookback_period: std::time::Duration::from_secs(86400),
        catchup_timeout: std::time::Duration::from_secs(10),
        enable_clustering: false,
        enable_summarization: false,
        max_context_length: 10000,
        message_filter: filter_config,
        strategy: CatchupStrategy::Full,
    };

    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    // Add mixed messages
    let normal_message = ChatMessage {
        id: "1".to_string(),
        channel_id: "channel1".to_string(),
        user_id: "user2".to_string(),
        username: "User2".to_string(),
        content: "Normal message".to_string(),
        timestamp: Utc::now(),
        is_bot: false,
        is_system: false,
        message_type: MessageType::Text,
        metadata: HashMap::new(),
    };

    let bot_message = ChatMessage {
        id: "2".to_string(),
        channel_id: "channel1".to_string(),
        user_id: "bot".to_string(),
        username: "Bot".to_string(),
        content: "Bot message".to_string(),
        timestamp: Utc::now(),
        is_bot: true,
        is_system: false,
        message_type: MessageType::Text,
        metadata: HashMap::new(),
    };

    let spam_message = ChatMessage {
        id: "3".to_string(),
        channel_id: "channel1".to_string(),
        user_id: "user3".to_string(),
        username: "User3".to_string(),
        content: "This is spam content".to_string(),
        timestamp: Utc::now(),
        is_bot: false,
        is_system: false,
        message_type: MessageType::Text,
        metadata: HashMap::new(),
    };

    message_store.add_message(normal_message);
    message_store.add_message(bot_message);
    message_store.add_message(spam_message);

    let catchup = create_chat_catchup_with_config(config);
    let catchup_engine = clawmaster_chat_catchup::catchup_engine::ChatCatchup::new(
        catchup.get_config(),
        message_store,
        session_store,
    ).unwrap();

    let result = catchup_engine.catch_up("channel1", "user1").await.unwrap();

    // Should only process the normal message (bot and spam filtered out)
    assert_eq!(result.messages_processed, 1);
    assert_eq!(result.messages_filtered, 2);
    assert!(result.context.context_string.contains("Normal message"));
    assert!(!result.context.context_string.contains("Bot message"));
    assert!(!result.context.context_string.contains("spam"));
}

#[tokio::test]
async fn test_catchup_context_length_limit() {
    let config = CatchupConfig {
        max_messages_per_batch: 100,
        max_lookback_period: std::time::Duration::from_secs(86400),
        catchup_timeout: std::time::Duration::from_secs(10),
        enable_clustering: false,
        enable_summarization: false,
        max_context_length: 100, // Very small limit
        message_filter: Default::default(),
        strategy: CatchupStrategy::Full,
    };

    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    // Add a long message
    let long_message = ChatMessage {
        id: "1".to_string(),
        channel_id: "channel1".to_string(),
        user_id: "user2".to_string(),
        username: "User2".to_string(),
        content: "This is a very long message that should exceed the context length limit and be truncated".to_string(),
        timestamp: Utc::now(),
        is_bot: false,
        is_system: false,
        message_type: MessageType::Text,
        metadata: HashMap::new(),
    };

    message_store.add_message(long_message);

    let catchup = create_chat_catchup_with_config(config);
    let catchup_engine = clawmaster_chat_catchup::catchup_engine::ChatCatchup::new(
        catchup.get_config(),
        message_store,
        session_store,
    ).unwrap();

    let result = catchup_engine.catch_up("channel1", "user1").await.unwrap();

    // Context should be truncated to fit the limit
    assert!(result.context.context_string.len() <= 100);
    assert_eq!(result.context.metadata.length, result.context.context_string.len());
}

#[tokio::test]
async fn test_catchup_concurrent_requests() {
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    // Add test messages
    for i in 0..5 {
        let message = ChatMessage {
            id: format!("msg_{}", i),
            channel_id: "channel1".to_string(),
            user_id: "user2".to_string(),
            username: "User2".to_string(),
            content: format!("Message {}", i),
            timestamp: Utc::now(),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        };
        message_store.add_message(message);
    }

    let catchup = create_chat_catchup_with_config(config);
    let catchup_engine = clawmaster_chat_catchup::catchup_engine::ChatCatchup::new(
        catchup.get_config(),
        message_store.clone(),
        session_store.clone(),
    ).unwrap();

    // Run concurrent catchup requests
    let future1 = catchup_engine.catch_up("channel1", "user1");
    let future2 = catchup_engine.catch_up("channel1", "user2");
    let future3 = catchup_engine.catch_up("channel1", "user3");

    let (result1, result2, result3) = tokio::join!(future1, future2, future3);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    let result1 = result1.unwrap();
    let result2 = result2.unwrap();
    let result3 = result3.unwrap();

    // All should have processed the same messages
    assert_eq!(result1.messages_processed, 5);
    assert_eq!(result2.messages_processed, 5);
    assert_eq!(result3.messages_processed, 5);
}

#[tokio::test]
async fn test_catchup_timeout_handling() {
    let config = CatchupConfig {
        max_messages_per_batch: 100,
        max_lookback_period: std::time::Duration::from_secs(86400),
        catchup_timeout: std::time::Duration::from_millis(1), // Very short timeout
        enable_clustering: false,
        enable_summarization: false,
        max_context_length: 10000,
        message_filter: Default::default(),
        strategy: CatchupStrategy::Full,
    };

    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    let catchup = create_chat_catchup_with_config(config);
    let catchup_engine = clawmaster_chat_catchup::catchup_engine::ChatCatchup::new(
        catchup.get_config(),
        message_store,
        session_store,
    ).unwrap();

    // This should timeout
    let result = catchup_engine.catch_up("channel1", "user1").await;

    assert!(result.is_err());
    match result.unwrap_err() {
        clawmaster_chat_catchup::error::CatchupError::TimeoutExceeded(_) => {
            // Expected
        }
        _ => panic!("Expected timeout error"),
    }
}
