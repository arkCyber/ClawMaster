# clawmaster-chat-catchup

Chat Catchup implementation inspired by MicroClaw, enabling continuous conversation support through intelligent message catchup.

## Features
- ✅ Unread message detection
- ✅ Intelligent context recovery
- ✅ Message clustering by topic
- ✅ Message summarization
- ✅ Adaptive catchup strategies
- ✅ Configurable filtering
- ✅ Context length management

## Usage
```rust
use clawmaster_chat_catchup::{create_chat_catchup, CatchupConfig};

let catchup = create_chat_catchup();

let result = catchup.catch_up("channel123", "user123").await?;
println!("Caught up on {} messages", result.messages_processed);
println!("Context: {}", result.context.context_string);
```

## Architecture
```
Last Read Timestamp → Get Messages → Filter → Process (Cluster/Summarize) → Build Context
                                    ↓
                              Adaptive Strategy Selection
```

## Configuration
```rust
use clawmaster_chat_catchup::CatchupConfig;
use std::time::Duration;

let config = CatchupConfig {
    max_messages_per_batch: 100,
    max_lookback_period: Duration::from_secs(86400), // 24 hours
    catchup_timeout: Duration::from_secs(30),
    enable_clustering: true,
    enable_summarization: true,
    max_context_length: 10000,
    message_filter: MessageFilterConfig::default(),
    strategy: CatchupStrategy::Adaptive { /* ... */ },
};
```

## Catchup Strategies
- **Full**: Process all messages individually
- **Clustered**: Group messages by topic
- **Summarized**: Create summary of messages
- **Adaptive**: Choose strategy based on message count and time

## Context Types
- **Full**: Complete message list
- **Clustered**: Topic-based clusters
- **Summarized**: Conversation summary
- **Hybrid**: Recent messages + older summary
