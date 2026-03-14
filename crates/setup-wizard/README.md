# ClawMaster Setup Wizard

Interactive TUI setup wizard for ClawMaster first-time configuration.

## Features

- 🎨 Beautiful terminal UI using `ratatui`
- 🚀 Quick setup in 2-3 minutes
- 🔑 Secure API key configuration
- 📡 Channel selection (Telegram, Discord, Slack)
- ✅ Configuration validation
- 💾 Automatic config file generation

## Usage

```bash
# Run the setup wizard
clawmaster setup

# Or programmatically
use clawmaster_setup_wizard::run_setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_setup().await
}
```

## Wizard Flow

1. **Welcome Screen** - Introduction and overview
2. **Provider Selection** - Choose LLM providers (OpenAI, Anthropic, etc.)
3. **Provider Configuration** - Enter API keys
4. **Channel Selection** - Choose communication channels
5. **Channel Configuration** - Enter bot tokens
6. **Summary** - Review configuration
7. **Complete** - Save and start

## Supported Providers

- OpenAI (GPT-4, GPT-3.5-turbo)
- Anthropic (Claude 3 Opus, Sonnet, Haiku)
- OpenRouter (Multiple models)
- Ollama (Local models)
- GitHub Copilot

## Supported Channels

- Web UI (always enabled)
- Telegram
- Discord
- Slack

## Configuration Output

The wizard generates two files:

### `~/.config/clawmaster/clawmaster.toml`
```toml
# ClawMaster Configuration
[providers]
openai = true
anthropic = true

[channels]
web = true
telegram = true
```

### `~/.config/clawmaster/.env`
```bash
# Environment Variables
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
TELEGRAM_BOT_TOKEN=123456:ABC...
```

## Keyboard Controls

- **Arrow Keys** (↑/↓) - Navigate options
- **Space** - Toggle selection
- **Enter** - Confirm and continue
- **Esc** - Go back
- **q** - Quit wizard

## Architecture

```
setup-wizard/
├── src/
│   ├── lib.rs       # Public API
│   ├── state.rs     # State machine and config
│   ├── ui.rs        # UI rendering
│   └── wizard.rs    # Main wizard logic
└── Cargo.toml
```

## Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `tokio` - Async runtime
- `serde` - Serialization
- `clawmaster-config` - Configuration management

## Development

```bash
# Build
cargo build -p clawmaster-setup-wizard

# Test
cargo test -p clawmaster-setup-wizard

# Run standalone
cargo run -p clawmaster-setup-wizard --example wizard
```

## License

MIT OR Apache-2.0
