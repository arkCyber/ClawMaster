//! Mattermost integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::MattermostConfig;
pub use error::{Error, Result};
pub use plugin::MattermostChannel;
