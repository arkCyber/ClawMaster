//! Demonstration of chat catchup functionality

use clawmaster_chat_catchup::{CatchupConfig, CatchupStrategy, ChatCatchupInterface};
use clawmaster_chat_catchup::catchup_engine::{MockMessageStore, MockSessionStore, ChatCatchup};
use clawmaster_chat_catchup::message_processor::{ChatMessage, MessageType};
use std::sync::Arc;
use std::collections::HashMap;
use chrono::Utc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Chat Catchup Demo ===\n");

    // Create configuration
    let config = CatchupConfig {
        max_messages_per_batch: 50,
        max_lookback_period: Duration::from_secs(3600), // 1 hour
        catchup_timeout: Duration::from_secs(10),
        enable_clustering: true,
        enable_summarization: true,
        max_context_length: 2000,
        message_filter: Default::default(),
        strategy: CatchupStrategy::Full,
    };

    // Create mock stores
    let message_store = Arc::new(MockMessageStore::new());
    let session_store = Arc::new(MockSessionStore::new());

    // Add sample messages
    println!("Adding sample messages...");
    add_sample_messages(&message_store).await;

    // Create catchup engine
    let catchup = ChatCatchup::new(config, message_store.clone(), session_store.clone())?;

    // Perform catchup
    println!("\nPerforming catchup for user1...");
    let start_time = std::time::Instant::now();
    let result = catchup.catch_up("general", "user1").await?;
    let catchup_duration = start_time.elapsed();

    // Display results
    println!("\n=== Catchup Results ===");
    println!("Catchup ID: {}", result.id);
    println!("Channel: {}", result.channel_id);
    println!("User: {}", result.user_id);
    println!("Messages processed: {}", result.messages_processed);
    println!("Messages filtered: {}", result.messages_filtered);
    println!("Had unread: {}", result.had_unread);
    println!("Mode used: {:?}", result.mode);
    println!("Processing time: {:?}", result.processing_time);
    println!("Total time: {:?}", catchup_duration);

    println!("\n=== Context Information ===");
    println!("Context length: {} characters", result.context.context_string.len());
    println!("Participants: {}", result.context.participants.join(", "));
    println!("Topics: {}", result.context.topics.join(", "));
    println!("Time range: {} to {}", 
             result.context.time_range.0.format("%H:%M:%S"),
             result.context.time_range.1.format("%H:%M:%S"));

    println!("\n=== Generated Context ===");
    println!("{}", result.context.context_string);

    // Test unread count
    println!("\n=== Unread Count ===");
    let unread_count = catchup.get_unread_count("general", "user2").await?;
    println!("Unread messages for user2: {}", unread_count);

    // Test marking as read
    println!("\n=== Mark as Read ===");
    catchup.mark_as_read("general", "user2", Utc::now().timestamp() as u64).await?;
    println!("Marked messages as read for user2");

    // Demonstrate different strategies
    println!("\n=== Testing Different Strategies ===");
    
    // Test with many messages for summarization
    println!("Adding many messages to test summarization...");
    add_many_messages(&message_store, 20).await;
    
    let result2 = catchup.catch_up("general", "user3").await?;
    println!("Strategy used for 20 messages: {:?}", result2.mode);
    println!("Context type: {:?}", 
             match result2.context.context_type {
                 clawmaster_chat_catchup::context_builder::ContextType::Summarized { .. } => "Summarized",
                 clawmaster_chat_catchup::context_builder::ContextType::Clustered { .. } => "Clustered",
                 clawmaster_chat_catchup::context_builder::ContextType::Full { .. } => "Full",
                 _ => "Other",
             });

    println!("\n=== Demo Complete ===");

    Ok(())
}

async fn add_sample_messages(message_store: &MockMessageStore) {
    let messages = vec![
        ChatMessage {
            id: "1".to_string(),
            channel_id: "general".to_string(),
            user_id: "alice".to_string(),
            username: "Alice".to_string(),
            content: "Hey everyone! How's the project going?".to_string(),
            timestamp: Utc::now() - Duration::from_secs(1800),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
        ChatMessage {
            id: "2".to_string(),
            channel_id: "general".to_string(),
            user_id: "bob".to_string(),
            username: "Bob".to_string(),
            content: "Going great! Just finished the API integration.".to_string(),
            timestamp: Utc::now() - Duration::from_secs(1500),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
        ChatMessage {
            id: "3".to_string(),
            channel_id: "general".to_string(),
            user_id: "charlie".to_string(),
            username: "Charlie".to_string(),
            content: "Nice work Bob! I'm still working on the frontend.".to_string(),
            timestamp: Utc::now() - Duration::from_secs(1200),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
        ChatMessage {
            id: "4".to_string(),
            channel_id: "general".to_string(),
            user_id: "alice".to_string(),
            username: "Alice".to_string(),
            content: "Let's have a quick sync meeting tomorrow at 2 PM.".to_string(),
            timestamp: Utc::now() - Duration::from_secs(900),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
        ChatMessage {
            id: "5".to_string(),
            channel_id: "general".to_string(),
            user_id: "bot".to_string(),
            username: "Bot".to_string(),
            content: "Meeting scheduled for tomorrow at 2 PM.".to_string(),
            timestamp: Utc::now() - Duration::from_secs(600),
            is_bot: true,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
        ChatMessage {
            id: "6".to_string(),
            channel_id: "general".to_string(),
            user_id: "bob".to_string(),
            username: "Bob".to_string(),
            content: "Perfect! I'll prepare the demo.".to_string(),
            timestamp: Utc::now() - Duration::from_secs(300),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        },
    ];

    for message in messages {
        message_store.add_message(message);
    }

    println!("Added 6 sample messages");
}

async fn add_many_messages(message_store: &MockMessageStore, count: usize) {
    let users = vec!["user1", "user2", "user3", "user4"];
    let topics = vec![
        "Discussing the new feature implementation",
        "Reviewing the code changes",
        "Planning the next sprint",
        "Analyzing the performance metrics",
        "Optimizing the database queries",
    ];

    for i in 0..count {
        let user = users[i % users.len()];
        let topic = topics[i % topics.len()];
        
        let message = ChatMessage {
            id: format!("many_{}", i),
            channel_id: "general".to_string(),
            user_id: user.to_string(),
            username: user.to_string(),
            content: format!("{} - Message {}", topic, i),
            timestamp: Utc::now() - Duration::from_secs(((count - i) * 60) as u64),
            is_bot: false,
            is_system: false,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
        };
        
        message_store.add_message(message);
    }

    println!("Added {} additional messages", count);
}
