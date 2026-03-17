/// HTTP server for serving media files to agents.
///
/// This module provides static file serving capabilities for media files
/// generated or used by ClawMaster agents.
use axum::{Router, http::StatusCode, response::IntoResponse};
use {crate::Result, std::path::Path, tower_http::services::ServeDir};

/// Start a simple HTTP server to serve media files.
///
/// This creates an axum server that serves static files from the media directory.
pub async fn start_media_server(media_dir: &Path, port: u16) -> Result<()> {
    use std::net::SocketAddr;

    // Ensure media directory exists
    tokio::fs::create_dir_all(media_dir)
        .await
        .map_err(|e| crate::Error::external("Failed to create media directory", e))?;

    let media_dir = media_dir.to_path_buf();

    // Create router with static file serving
    let app = Router::new()
        .nest_service("/", ServeDir::new(&media_dir))
        .fallback(handler_404);

    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| crate::Error::external(format!("Failed to bind to {}", addr), e))?;

    tracing::info!("Media server listening on http://{}", addr);

    // Start server
    axum::serve(listener, app)
        .await
        .map_err(|e| crate::Error::external("Media server error", e))?;

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Media file not found")
}
