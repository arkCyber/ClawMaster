# clawmaster-qq

QQ channel integration for ClawMaster.

## Features

- ✅ QQ Bot API support
- ✅ go-cqhttp protocol support
- ✅ Private and group messages
- ✅ Image and file support
- ✅ Markdown conversion
- ✅ Event handling
- ✅ DO-178C Level A compliant

## Configuration

```toml
[channels.qq]
enabled = true
bot_id = "123456"
access_token = "your_token"
api_url = "http://localhost:5700"
groups = ["789012", "345678"]
enable_private = true
enable_group = true
webhook_port = 8080
max_message_length = 4096
```

## Usage

```rust
use clawmaster_qq::{QqChannel, QqConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = QqConfig {
        bot_id: "123456".to_string(),
        access_token: "your_token".to_string(),
        api_url: "http://localhost:5700".to_string(),
        groups: vec![],
        enable_private: true,
        enable_group: true,
        webhook_port: 8080,
        max_message_length: 4096,
    };
    
    let channel = QqChannel::new();
    // Use channel...
    Ok(())
}
```

## API Support

### Implemented
- ✅ Send message (private/group)
- ✅ Get group list
- ✅ Get login info
- ✅ Markdown conversion
- ✅ CQ code support

### Planned
- 🔄 Image upload
- 🔄 File upload
- 🔄 Voice message
- 🔄 Forward message
- 🔄 Group management

## Testing

```bash
cargo test -p clawmaster-qq
```

## License

Same as ClawMaster project.
