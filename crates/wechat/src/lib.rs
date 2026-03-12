//! WeChat Work (企业微信) channel integration for ClawMaster.
//!
//! Supports WeChat Work API for sending and receiving messages in enterprise environments.
//!
//! # Features
//! - Message sending and receiving
//! - Text, image, and file messages
//! - Interactive cards
//! - Webhook callbacks
//! - Access token management
//!
//! # Example
//! ```no_run
//! use clawmaster_wechat::WeChatChannel;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = serde_json::json!({
//!         "corp_id": "your_corp_id",
//!         "agent_id": "1000002",
//!         "secret": "your_secret"
//!     });
//!     
//!     let channel = WeChatChannel::new();
//!     // Use channel...
//!     Ok(())
//! }
//! ```

mod client;
mod config;
mod error;
mod plugin;
mod types;

pub use client::WeChatClient;
pub use config::WeChatConfig;
pub use error::{Error, Result};
pub use plugin::WeChatChannel;
pub use types::{WeChatMessage, WeChatMessageType, WeChatTextMessage};
