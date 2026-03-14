# clawmaster-user-errors

User-friendly error messages for ClawMaster, inspired by Rust's excellent error reporting.

## Overview

This crate transforms technical error messages into helpful, actionable messages that guide users to solutions. Instead of cryptic stack traces, users get clear explanations with suggestions and next steps.

## Features

- **Colored Output**: Uses colors to highlight important information
- **Actionable Suggestions**: Every error includes specific steps to fix it
- **Context-Aware**: Detects common error patterns and provides relevant help
- **Links to Documentation**: Includes URLs to relevant documentation
- **Command Examples**: Shows exact commands to run

## Usage

### Basic Usage

```rust
use clawmaster_user_errors::{UserError, format_error};

// Create a user-friendly error
let err = UserError::ApiKeyMissing {
    provider: "OpenAI".to_string(),
    help_url: "https://platform.openai.com/api-keys".to_string(),
};

println!("{}", err);
```

Output:
```
❌ API key for OpenAI is missing
   💡 Set your API key in one of these ways:
      1. Environment variable: OPENAI_API_KEY=sk-your-key
      2. Run: clawmaster provider add openai --key YOUR_KEY
   🔗 Get your API key at: https://platform.openai.com/api-keys
```

### Wrapping Existing Errors

```rust
use clawmaster_user_errors::{format_error, UserErrorExt};
use anyhow::Result;

fn load_config() -> Result<Config> {
    // ... some operation that might fail
}

// Method 1: Format manually
match load_config() {
    Ok(config) => println!("Config loaded"),
    Err(e) => eprintln!("{}", format_error(&e)),
}

// Method 2: Use extension trait
if let Err(e) = load_config() {
    eprintln!("{}", e.user_friendly());
}
```

### Error Types

#### Config Not Found
```rust
UserError::ConfigNotFound {
    expected_path: PathBuf::from("/home/user/.config/clawmaster/config.toml"),
    suggestion: "Run 'clawmaster setup' to create it".to_string(),
}
```

#### API Key Missing
```rust
UserError::ApiKeyMissing {
    provider: "Anthropic".to_string(),
    help_url: "https://console.anthropic.com".to_string(),
}
```

#### Port In Use
```rust
UserError::PortInUse {
    port: 13131,
    suggestion: "Change the port in config or stop the conflicting service".to_string(),
}
```

#### Database Error
```rust
UserError::DatabaseError {
    operation: "migration".to_string(),
    suggestion: "Try running 'clawmaster db reset'".to_string(),
}
```

#### Provider Unavailable
```rust
UserError::ProviderUnavailable {
    provider: "OpenAI".to_string(),
    reason: "API rate limit exceeded".to_string(),
    alternatives: vec![
        "Anthropic Claude".to_string(),
        "Ollama (local)".to_string(),
    ],
}
```

#### Channel Error
```rust
UserError::ChannelError {
    channel: "Telegram".to_string(),
    error: "Invalid bot token".to_string(),
    fix_steps: vec![
        "Check your bot token in .env file".to_string(),
        "Verify the token with: curl https://api.telegram.org/bot<TOKEN>/getMe".to_string(),
        "Get a new token from @BotFather if needed".to_string(),
    ],
}
```

#### Invalid Configuration
```rust
UserError::InvalidConfiguration {
    field: "server.port".to_string(),
    expected: "1024-65535".to_string(),
    got: "99999".to_string(),
}
```

#### Setup Required
```rust
UserError::SetupRequired {
    reason: "No providers configured".to_string(),
}
```

#### Dependency Missing
```rust
UserError::DependencyMissing {
    dependency: "Docker".to_string(),
    install_command: "curl -fsSL https://get.docker.com | sh".to_string(),
}
```

## Integration Examples

### In CLI Commands

```rust
use clawmaster_user_errors::UserError;

pub async fn start_server(config: Config) -> Result<()> {
    if config.providers.is_empty() {
        return Err(UserError::SetupRequired {
            reason: "No LLM providers configured".to_string(),
        }.into());
    }
    
    // ... start server
}
```

### In Gateway

```rust
use clawmaster_user_errors::format_error;

async fn handle_request(req: Request) -> Response {
    match process_request(req).await {
        Ok(response) => response,
        Err(e) => {
            let user_msg = format_error(&e);
            Response::error(user_msg)
        }
    }
}
```

### In Web UI

```rust
use clawmaster_user_errors::UserError;

async fn setup_provider(provider: &str, api_key: &str) -> Result<()> {
    if api_key.is_empty() {
        return Err(UserError::ApiKeyMissing {
            provider: provider.to_string(),
            help_url: get_help_url(provider),
        }.into());
    }
    
    // ... setup provider
}
```

## Automatic Error Detection

The `format_error` function automatically detects common error patterns:

```rust
use clawmaster_user_errors::format_error;

// Detects file not found errors
let err = anyhow::anyhow!("No such file or directory: '/path/to/config.toml'");
println!("{}", format_error(&err));
// Output: ❌ Configuration file not found...

// Detects port conflicts
let err = anyhow::anyhow!("Address already in use: 0.0.0.0:13131");
println!("{}", format_error(&err));
// Output: ❌ Port 13131 is already in use...

// Detects API key issues
let err = anyhow::anyhow!("API key authentication failed");
println!("{}", format_error(&err));
// Output: ❌ API key for OpenAI is missing...
```

## Best Practices

### 1. Always Provide Context

❌ Bad:
```rust
Err(anyhow::anyhow!("Failed"))
```

✅ Good:
```rust
Err(UserError::DatabaseError {
    operation: "loading user preferences".to_string(),
    suggestion: "Check database permissions".to_string(),
}.into())
```

### 2. Include Actionable Steps

❌ Bad:
```rust
UserError::ChannelError {
    channel: "Discord".to_string(),
    error: "Connection failed".to_string(),
    fix_steps: vec![],  // Empty!
}
```

✅ Good:
```rust
UserError::ChannelError {
    channel: "Discord".to_string(),
    error: "Connection failed".to_string(),
    fix_steps: vec![
        "Check your internet connection".to_string(),
        "Verify bot token is correct".to_string(),
        "Check Discord API status: https://discordstatus.com".to_string(),
    ],
}
```

### 3. Provide Alternatives

❌ Bad:
```rust
UserError::ProviderUnavailable {
    provider: "OpenAI".to_string(),
    reason: "Service down".to_string(),
    alternatives: vec![],  // No alternatives!
}
```

✅ Good:
```rust
UserError::ProviderUnavailable {
    provider: "OpenAI".to_string(),
    reason: "Service temporarily unavailable".to_string(),
    alternatives: vec![
        "Anthropic Claude".to_string(),
        "Ollama (local)".to_string(),
        "OpenRouter".to_string(),
    ],
}
```

## Comparison with Standard Errors

### Before (Standard Error)
```
Error: No such file or directory (os error 2)
   0: std::fs::read_to_string
   1: clawmaster_config::load
   2: clawmaster::main
```

### After (User-Friendly Error)
```
❌ Configuration file not found
   Expected: /home/user/.config/clawmaster/config.toml
   💡 Run 'clawmaster setup' to create the configuration file
```

## Testing

```bash
cargo test -p clawmaster-user-errors
```

## License

MIT OR Apache-2.0
