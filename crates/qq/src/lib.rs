//! QQ channel integration for ClawMaster.
//!
//! Supports QQ Bot API and go-cqhttp protocol for sending and receiving messages
//! in QQ groups and private chats.
//!
//! # Features
//! - Message sending and receiving
//! - Group chat support
//! - Private chat support
//! - Image and file support
//! - Markdown conversion
//! - Event handling
//!
//! # Example
//! ```no_run
//! use clawmaster_qq::QqChannel;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = serde_json::json!({
//!         "bot_id": "your_bot_id",
//!         "access_token": "your_token",
//!         "api_url": "http://localhost:5700"
//!     });
//!     
//!     let channel = QqChannel::new(config)?;
//!     // Use channel...
//!     Ok(())
//! }
//! ```

mod client;
mod config;
mod error;
mod markdown;
mod plugin;
mod types;

pub use client::QqClient;
pub use config::QqConfig;
pub use error::{Error, Result};
pub use plugin::QqChannel;
pub use types::{QqMessage, QqMessageType, QqEvent};
