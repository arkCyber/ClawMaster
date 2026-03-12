//! Line Messaging API integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::LineConfig;
pub use error::{Error, Result};
pub use plugin::LineChannel;
