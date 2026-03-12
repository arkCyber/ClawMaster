//! DingTalk (钉钉) integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::DingTalkConfig;
pub use error::{Error, Result};
pub use plugin::DingTalkChannel;
