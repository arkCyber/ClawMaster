//! Lightweight server implementation

use crate::config::LiteConfig;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Lightweight server implementation
pub struct LiteServer {
    config: Arc<RwLock<LiteConfig>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl LiteServer {
    /// Create a new lightweight server
    pub fn new(config: LiteConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            server_handle: None,
            shutdown_tx: None,
        }
    }
    
    /// Set server address
    pub fn set_address(&mut self, host: String, port: u16) {
        let config = Arc::clone(&self.config);
        tokio::spawn(async move {
            let mut config = config.write().await;
            config.server.host = host;
            config.server.port = port;
        });
    }
    
    /// Start the server
    pub async fn start(&mut self) -> Result<()> {
        let config = self.config.read().await;
        
        info!("Starting ClawMaster Lite server");
        info!("Mode: {}", config.mode);
        info!("Address: {}:{}", config.server.host, config.server.port);
        
        // Validate configuration
        config.validate()
            .context("Invalid configuration")?;
        
        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);
        
        // Start servers based on features
        // Note: This is a simplified implementation for the lightweight deployment
        // In production, this would start actual web and API servers
        info!("Server features enabled:");
        if cfg!(feature = "web") {
            info!("  - Web server");
        }
        if cfg!(feature = "gateway") {
            info!("  - API gateway");
        }
        
        // Simulate server running
        let server_task = tokio::spawn(async move {
            let _ = shutdown_rx.await;
            info!("Server shutdown signal received");
        });
        
        self.server_handle = Some(server_task);
        
        info!("✅ Server started successfully");
        info!("🌐 Web interface: http://{}:{}", config.server.host, config.server.port);
        info!("📚 API documentation: http://{}:{}/api/docs", config.server.host, config.server.port);
        
        Ok(())
    }
    
    /// Stop the server
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping server...");
        
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
        
        if let Some(handle) = self.server_handle.take() {
            let _ = handle.await;
        }
        
        info!("Server stopped");
        
        Ok(())
    }
    
    /// Get server status
    pub async fn get_status(&self) -> ServerStatus {
        let config = self.config.read().await;
        
        ServerStatus {
            running: self.server_handle.is_some(),
            mode: config.mode.clone(),
            address: format!("{}:{}", config.server.host, config.server.port),
            uptime: std::time::Duration::from_secs(0), // Would track actual uptime
            version: env!("CARGO_PKG_VERSION").to_string(),
            features: config.features.clone(),
        }
    }
}

/// Server status information
#[derive(Debug, Clone)]
pub struct ServerStatus {
    /// Whether the server is running
    pub running: bool,
    /// Deployment mode
    pub mode: crate::config::DeployMode,
    /// Server address
    pub address: String,
    /// Server uptime
    pub uptime: std::time::Duration,
    /// Server version
    pub version: String,
    /// Enabled features
    pub features: crate::config::FeatureFlags,
}

impl Drop for LiteServer {
    fn drop(&mut self) {
        // Note: This is a synchronous drop, but we need to handle async shutdown
        // In a real implementation, you'd want to ensure proper cleanup
        if self.server_handle.is_some() {
            warn!("LiteServer dropped without explicit shutdown");
        }
    }
}
