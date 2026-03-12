//! Zulip REST API integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::ZulipConfig;
pub use error::{Error, Result};
pub use plugin::ZulipChannel;
