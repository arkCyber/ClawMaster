//! Signal channel implementation for ClawMaster
//!
//! This module provides Signal messaging support through the Signal Protocol.
//!
//! # Features
//!
//! - End-to-end encryption
//! - Group messaging
//! - Media support
//! - Typing indicators
//!
//! # Compliance
//!
//! DO-178C §11.13: Deterministic initialization
//! - All connections are properly managed
//! - Resource cleanup is guaranteed
//!
//! # Example
//!
//! ```no_run
//! use clawmaster_signal::SignalChannel;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let channel = SignalChannel::new();
//! // Configure and use the channel
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod error;
pub mod plugin;

pub use {
    config::SignalConfig,
    error::{Error, Result},
    plugin::SignalChannel,
};
