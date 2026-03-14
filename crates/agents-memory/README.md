# clawmaster-agents-memory

AGENTS.md long-term memory system for ClawMaster, inspired by OpenClaw's memory architecture.

## Overview

This crate provides a persistent long-term memory system using a simple Markdown file (`AGENTS.md`) that stores important information across all conversations. Unlike session-based memory, this memory persists indefinitely and helps the AI maintain context about user preferences, project details, and important decisions.

## Features

- **Persistent Storage**: All memory stored in a human-readable Markdown file
- **Categorized Entries**: Organize memories by type (preferences, context, decisions, etc.)
- **Section Management**: Update specific sections without rewriting the entire file
- **Search Capability**: Find relevant memories quickly
- **Automatic Creation**: Creates default structure on first use

## Usage

### Basic Usage

```rust
use clawmaster_agents_memory::{AgentsMemory, MemoryEntry, MemoryCategory};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load or create AGENTS.md
    let mut memory = AgentsMemory::load().await?;
    
    // Add a user preference
    let entry = MemoryEntry::user_preference("Preferred language: Rust");
    memory.append_entry(entry).await?;
    
    // Add a learning record
    let entry = MemoryEntry::learning_record("Learned about DO-178C Level A standards");
    memory.append_entry(entry).await?;
    
    // Search for relevant memories
    let results = memory.search("Rust");
    for result in results {
        println!("Found: {}", result);
    }
    
    Ok(())
}
```

### Memory Categories

```rust
// User preferences
MemoryEntry::user_preference("Timezone: UTC+8");

// Project context
MemoryEntry::project_context("Working on ClawMaster v0.10.18");

// Learning records
MemoryEntry::learning_record("Implemented P0 enterprise features");

// Important decisions
MemoryEntry::important_decision("Chose Rust over TypeScript for performance");

// Conversation summaries
MemoryEntry::conversation_summary("Discussed Web UI improvements");

// Custom categories
MemoryEntry::custom("API Keys", "OpenAI API key configured");
```

### Section Management

```rust
// Update a specific section
memory.update_section(
    "User Preferences",
    "- Language: Chinese\n- Timezone: UTC+8\n- Code style: Concise with comments"
).await?;

// Extract a section
if let Some(prefs) = memory.extract_section("User Preferences") {
    println!("User preferences: {}", prefs);
}
```

### Raw Content

```rust
// Append raw markdown
memory.append_raw("## Custom Section\n\nSome important notes...").await?;

// Get full content
let content = memory.content();
println!("{}", content);
```

## File Structure

The AGENTS.md file is organized into sections:

```markdown
# AGENTS.md - Long-term Memory

## User Preferences
- Language: English
- Timezone: UTC

## Project Context
- Project: ClawMaster
- Tech Stack: Rust, Tokio, Axum

## Learning Records
### 2026-03-13 10:30:00 UTC - Learning Records
Learned about DO-178C Level A compliance

## Important Decisions
### 2026-03-13 11:00:00 UTC - Important Decisions
Decided to implement P0 enterprise features

## Conversation Summaries

## Custom Notes
```

## File Location

The AGENTS.md file is stored in the ClawMaster config directory:
- Linux: `~/.config/clawmaster/AGENTS.md`
- macOS: `~/.config/clawmaster/AGENTS.md`
- Windows: `%APPDATA%\clawmaster\AGENTS.md`

## Integration with Chat

```rust
use clawmaster_agents_memory::AgentsMemory;

// In your chat handler
async fn handle_chat(message: &str) -> Result<String> {
    let mut memory = AgentsMemory::load().await?;
    
    // Include relevant memories in context
    let context = memory.search(message);
    
    // After generating response, save important information
    if message.contains("remember") {
        let entry = MemoryEntry::user_preference(message);
        memory.append_entry(entry).await?;
    }
    
    Ok(response)
}
```

## API Reference

### `AgentsMemory`

- `load() -> Result<Self>` - Load or create AGENTS.md
- `reload() -> Result<()>` - Reload from disk
- `content() -> &str` - Get full content
- `path() -> &Path` - Get file path
- `append_entry(entry: MemoryEntry) -> Result<()>` - Add categorized entry
- `append_raw(content: &str) -> Result<()>` - Add raw markdown
- `update_section(name: &str, content: &str) -> Result<()>` - Update section
- `search(query: &str) -> Vec<String>` - Search for text
- `extract_section(name: &str) -> Option<String>` - Get section content

### `MemoryEntry`

- `new(category: MemoryCategory, content: String) -> Self`
- `user_preference(content: String) -> Self`
- `project_context(content: String) -> Self`
- `learning_record(content: String) -> Self`
- `important_decision(content: String) -> Self`
- `conversation_summary(content: String) -> Self`
- `custom(category: String, content: String) -> Self`

### `MemoryCategory`

```rust
pub enum MemoryCategory {
    UserPreference,
    ProjectContext,
    LearningRecord,
    ImportantDecision,
    ConversationSummary,
    Custom(String),
}
```

## Testing

```bash
cargo test -p clawmaster-agents-memory
```

## Comparison with OpenClaw

This implementation is inspired by OpenClaw's AGENTS.md system but enhanced with:
- ✅ Structured API for memory management
- ✅ Categorized entries with timestamps
- ✅ Section-based updates
- ✅ Search functionality
- ✅ Full test coverage

## License

MIT OR Apache-2.0
