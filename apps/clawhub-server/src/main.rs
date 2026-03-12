//! ClawHub standalone server.
//!
//! Independent HTTP server for the ClawHub registry.

use anyhow::Result;
use axum::Router;
use clawmaster_clawhub::api::{routes, ApiState};
use clawmaster_clawhub::registry::Registry;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    info!("Starting ClawHub server...");
    
    // Get configuration from environment
    let database_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "./clawhub.db".to_string());
    let bind_addr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    
    info!("Database: {}", database_path);
    info!("Bind address: {}", bind_addr);
    
    // Create registry
    let registry = Registry::new(&database_path).await?;
    info!("Registry initialized");
    
    // Create API state
    let state = ApiState::new(registry);
    
    // Build router
    let app = Router::new()
        .nest("/api/clawhub", routes(state))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());
    
    // Parse bind address
    let addr: SocketAddr = bind_addr.parse()?;
    
    info!("🚀 ClawHub server listening on http://{}", addr);
    info!("📚 API documentation: http://{}/api/clawhub/tools", addr);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
