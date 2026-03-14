//! Basic tests for chat catchup functionality

use clawmaster_chat_catchup::{CatchupConfig, CatchupStrategy, ChatCatchupInterface};
use clawmaster_chat_catchup::catchup_engine::{MockMessageStore, MockSessionStore, ChatCatchup};
use clawmaster_chat_catchup::message_processor::{ChatMessage, MessageType};
use std::sync::Arc;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::test]
async fn test_create_catchup_engine() {
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    
    let catchup = ChatCatchup::new(config, message_store, session_store);
    assert!(catchup.is_ok());
}

#[tokio::test]
async fn test_catchup_with_no_messages() {
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    
    let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
    let result = catchup.catch_up("channel1", "user1").await;
    
    if let Err(e) = &result {
        eprintln!("Error in test_catchup_with_no_messages: {:?}", e);
    }
    assert!(result.is_ok());
    let catchup_result = result.unwrap();
    assert_eq!(catchup_result.messages_processed, 0);
    assert!(!catchup_result.had_unread);
}

#[tokio::test]
async fn test_catchup_with_messages() {
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
    
    let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
    let result = catchup.catch_up("channel1", "user1").await;
    
    assert!(result.is_ok());
    let catchup_result = result.unwrap();
    assert_eq!(catchup_result.messages_processed, 5);
    assert!(catchup_result.had_unread);
}

#[tokio::test]
async fn test_get_unread_count() {
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    
    // Set up the message store reference in session store
    session_store.set_message_store(message_store.clone());
    
    // Add test messages
    for i in 0..3 {
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
    
    let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
    let count = catchup.get_unread_count("channel1", "user1").await;
    
    assert!(count.is_ok());
    assert_eq!(count.unwrap(), 3);
}

#[tokio::test]
async fn test_mark_as_read() {
    let config = CatchupConfig::default();
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());
    
    let catchup = ChatCatchup::new(config, message_store, session_store).unwrap();
    let timestamp = Utc::now().timestamp() as u64;
    let result = catchup.mark_as_read("channel1", "user1", timestamp).await;
    
    assert!(result.is_ok());
}

#[test]
fn test_catchup_config_default() {
    let config = CatchupConfig::default();
    assert_eq!(config.max_messages_per_batch, 100);
    assert!(config.enable_clustering);
}

#[test]
fn test_catchup_strategy_variants() {
    let full = CatchupStrategy::Full;
    
    // Just verify it can be created
    assert!(matches!(full, CatchupStrategy::Full));
}
