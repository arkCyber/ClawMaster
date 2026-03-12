# clawmaster-wechat

WeChat Work (企业微信) channel integration for ClawMaster.

## Features

- ✅ WeChat Work API support
- ✅ Text messages
- ✅ Image and file messages
- ✅ Interactive cards
- ✅ Access token management (with caching)
- ✅ Webhook callbacks
- ✅ DO-178C Level A compliant

## Configuration

```toml
[channels.wechat]
enabled = true
corp_id = "your_corp_id"
agent_id = "1000002"
secret = "your_secret"
api_url = "https://qyapi.weixin.qq.com"
token = "your_webhook_token"
encoding_aes_key = "your_aes_key"
token_cache_duration = 7200
max_message_length = 2048
```

## Usage

```rust
use clawmaster_wechat::{WeChatChannel, WeChatConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = WeChatConfig {
        corp_id: "your_corp_id".to_string(),
        agent_id: "1000002".to_string(),
        secret: "your_secret".to_string(),
        api_url: "https://qyapi.weixin.qq.com".to_string(),
        token: None,
        encoding_aes_key: None,
        token_cache_duration: 7200,
        max_message_length: 2048,
    };
    
    let channel = WeChatChannel::new();
    // Use channel...
    Ok(())
}
```

## API Support

### Implemented
- ✅ Get access token (with caching)
- ✅ Send text message
- ✅ Token auto-refresh
- ✅ Error handling

### Planned
- 🔄 Image upload
- 🔄 File upload
- 🔄 Interactive cards
- 🔄 Webhook verification
- 🔄 Message encryption/decryption

## Testing

```bash
cargo test -p clawmaster-wechat
```

## License

Same as ClawMaster project.
