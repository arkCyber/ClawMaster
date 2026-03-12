//! Viber Bot API integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::ViberConfig;
pub use error::{Error, Result};
pub use plugin::ViberChannel;
