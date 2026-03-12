//! Feishu/Lark (飞书) integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::FeishuConfig;
pub use error::{Error, Result};
pub use plugin::FeishuChannel;
