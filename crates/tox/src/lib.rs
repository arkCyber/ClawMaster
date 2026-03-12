//! Tox protocol integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::ToxConfig;
pub use error::{Error, Result};
pub use plugin::ToxChannel;
