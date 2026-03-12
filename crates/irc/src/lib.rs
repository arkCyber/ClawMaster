//! IRC (Internet Relay Chat) channel integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::IrcConfig;
pub use error::{Error, Result};
pub use plugin::IrcChannel;
