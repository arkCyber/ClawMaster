//! Matrix protocol integration for ClawMaster.

mod config;
mod error;
mod plugin;

pub use config::MatrixConfig;
pub use error::{Error, Result};
pub use plugin::MatrixChannel;
