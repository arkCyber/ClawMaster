use {
    axum::{
        Router,
        extract::ws::{WebSocket, WebSocketUpgrade},
        response::IntoResponse,
        routing::get,
    },
    std::net::SocketAddr,
};

/// Canvas host HTTP + WebSocket server.
pub async fn start_canvas_server(port: u16) -> crate::Result<()> {
    let app = Router::new()
        .route("/", get(serve_canvas_html))
        .route("/ws", get(canvas_websocket_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| crate::Error::external(format!("Failed to bind to {}", addr), e))?;

    tracing::info!("Canvas server listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| crate::Error::external("Canvas server error", e))?;

    Ok(())
}

async fn serve_canvas_html() -> impl IntoResponse {
    axum::response::Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>ClawMaster Canvas</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 20px; font-family: system-ui; }
        #canvas { border: 1px solid #ccc; width: 100%; height: 600px; }
    </style>
</head>
<body>
    <h1>ClawMaster Canvas (A2UI)</h1>
    <div id="canvas"></div>
    <script>
        const ws = new WebSocket('ws://' + location.host + '/ws');
        ws.onmessage = (e) => {
            console.log('Received:', e.data);
            document.getElementById('canvas').innerHTML += '<p>' + e.data + '</p>';
        };
        ws.onopen = () => console.log('Canvas WebSocket connected');
        ws.onerror = (e) => console.error('WebSocket error:', e);
    </script>
</body>
</html>"#,
    )
}

async fn canvas_websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_canvas_socket)
}

async fn handle_canvas_socket(mut socket: WebSocket) {
    tracing::info!("Canvas WebSocket connection established");

    // Send welcome message
    if socket
        .send(axum::extract::ws::Message::Text(
            "Canvas connected".to_string().into(),
        ))
        .await
        .is_err()
    {
        return;
    }

    // Handle bidirectional messages
    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("Canvas WebSocket error: {}", e);
                break;
            },
        };

        match msg {
            axum::extract::ws::Message::Text(text) => {
                tracing::debug!("Canvas received: {}", text);
                // Echo back for now (can be extended with action handling)
                if socket
                    .send(axum::extract::ws::Message::Text(
                        format!("Echo: {}", text).into(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            },
            axum::extract::ws::Message::Close(_) => {
                tracing::info!("Canvas WebSocket closed");
                break;
            },
            _ => {},
        }
    }
}
