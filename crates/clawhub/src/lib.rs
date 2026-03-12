//! ClawHub - Wasm Tool Plugin Marketplace
//!
//! ClawHub is a secure, decentralized marketplace for Wasm-based AI agent tools.
//! It provides discovery, installation, and management of community-contributed tools.
//!
//! # Architecture
//!
//! - **Registry**: Central metadata store (SQLite)
//! - **Storage**: S3-compatible object storage for `.wasm` files
//! - **Security**: Ed25519 signing, automated scanning
//! - **CLI**: `claw` command-line tool for publishing/installing
//!
//! # Compliance
//!
//! DO-178C §11.13: Deterministic initialization
//! - All tools are sandboxed via Wasmtime
//! - Resource limits enforced
//! - Capability-based security model
//!
//! # Example
//!
//! ```no_run
//! use clawmaster_clawhub::registry::Registry;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let registry = Registry::new("clawhub.db").await?;
//!
//! // Search for tools
//! let tools = registry.search("calculator", None).await?;
//!
//! // Get tool metadata
//! let tool = registry.get_tool("calc", "1.0.0").await?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod error;
pub mod metadata;
pub mod registry;
pub mod security;
pub mod skills;
pub mod storage;
pub mod types;

pub use error::{Error, Result};
