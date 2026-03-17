//! Folder Access Control System
//!
//! DO-178C Level A compliant folder access control for AI agents.
//!
//! # Compliance
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Consistent permission checks
//! - §11.10: Resource management - Database connection pooling
//! - §11.13: Initialization - Proper database migration
//!
//! # Security Features
//! - Path canonicalization and validation
//! - Permission-based access control (read, write, execute, delete)
//! - Complete audit logging
//! - Blacklist/whitelist validation rules
//! - Path traversal attack prevention
//! - Symbolic link resolution

pub mod models;
pub mod rpc;
pub mod service;
pub mod validation;

pub use {
    models::{
        AccessLog, AccessOperation, FolderPermission, PermissionFlags, RuleType, ValidationRule,
    },
    rpc::FolderAccessRpc,
    service::FolderAccessService,
    validation::{PathValidator, ValidationResult},
};

use anyhow::Result;

/// Initialize folder access control database
///
/// # Compliance
/// DO-178C §11.13: Database initialization with migrations
pub async fn run_migrations(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run folder access migrations: {}", e))?;

    tracing::info!("Folder access control migrations completed");
    Ok(())
}
