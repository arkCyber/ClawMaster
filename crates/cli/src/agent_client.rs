//! CLI client for connecting to the ClawMaster gateway and sending agent requests.

use {
    anyhow::{Result, bail},
    futures::{SinkExt, StreamExt},
    native_tls,
    serde_json::{Value, json},
    tokio_tungstenite::{Connector, connect_async, tungstenite::protocol::Message},
    uuid::Uuid,
};

/// Send a message to the ClawMaster gateway via WebSocket and wait for the response.
pub async fn send_agent_message(gateway_url: &str, message: &str) -> Result<String> {
    // Connect to WebSocket
    let ws_url = if gateway_url.starts_with("http://") {
        gateway_url.replace("http://", "ws://")
    } else if gateway_url.starts_with("https://") {
        gateway_url.replace("https://", "wss://")
    } else {
        format!("ws://{}", gateway_url)
    };

    let ws_url = if ws_url.ends_with("/ws") {
        ws_url
    } else {
        format!("{}/ws", ws_url.trim_end_matches('/'))
    };

    // Create TLS connector that accepts invalid certificates (for self-signed certs)
    let connector = if ws_url.starts_with("wss://") {
        let mut tls_connector = native_tls::TlsConnector::builder();
        tls_connector.danger_accept_invalid_certs(true);
        tls_connector.danger_accept_invalid_hostnames(true);
        Some(Connector::NativeTls(tls_connector.build()?))
    } else {
        None
    };

    let (ws_stream, _) = if let Some(connector) = connector {
        tokio_tungstenite::connect_async_tls_with_config(&ws_url, None, false, Some(connector))
            .await?
    } else {
        connect_async(&ws_url).await?
    };
    let (mut write, mut read) = ws_stream.split();

    // Send connect frame (v4 protocol)
    let connect_frame = json!({
        "type": "req",
        "id": Uuid::new_v4().to_string(),
        "method": "connect",
        "params": {
            "protocol": {
                "min": 4,
                "max": 4
            },
            "client": {
                "id": "clawmaster-cli",
                "version": "0.10.18",
                "platform": "cli",
                "mode": "operator"
            }
        }
    });

    write
        .send(Message::Text(connect_frame.to_string().into()))
        .await?;

    // Wait for hello response
    let hello_msg = read
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Connection closed before hello"))??;

    let hello_text = hello_msg.to_text()?;
    let hello: Value = serde_json::from_str(hello_text)?;

    if hello.get("error").is_some() {
        bail!("Connection failed: {}", hello);
    }

    // Subscribe to all events (v4 protocol requires explicit subscription)
    let subscribe_frame = json!({
        "type": "req",
        "id": Uuid::new_v4().to_string(),
        "method": "subscribe",
        "params": {
            "events": ["*"]  // Wildcard: subscribe to all events
        }
    });

    write
        .send(Message::Text(subscribe_frame.to_string().into()))
        .await?;

    // Send chat.send request (without specifying model to use first available)
    let chat_request = json!({
        "type": "req",
        "id": Uuid::new_v4().to_string(),
        "method": "chat.send",
        "params": {
            "text": message,
            "session": "main"
        }
    });

    write
        .send(Message::Text(chat_request.to_string().into()))
        .await?;

    // Collect response
    let mut response_text = String::new();
    let mut done = false;

    while !done {
        match read.next().await {
            Some(Ok(msg)) => {
                let text = msg.to_text()?;
                let frame: Value = serde_json::from_str(text)?;

                // Check message type
                let msg_type = frame.get("type").and_then(|v| v.as_str()).unwrap_or("");

                match msg_type {
                    "res" => {
                        // Response to chat.send - contains runId
                        if !frame.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                            if let Some(error) = frame.get("error") {
                                bail!("Agent error: {}", error);
                            }
                        }
                        // Continue waiting for events
                    },
                    "event" => {
                        // Server-push event
                        if let Some(event) = frame.get("event").and_then(|v| v.as_str()) {
                            if event == "chat" {
                                // Chat event from backend
                                if let Some(payload) = frame.get("payload") {
                                    // Check state field to determine event type
                                    let state =
                                        payload.get("state").and_then(|v| v.as_str()).unwrap_or("");

                                    match state {
                                        "final" => {
                                            // Final response with complete text
                                            if let Some(text) =
                                                payload.get("text").and_then(|v| v.as_str())
                                            {
                                                response_text = text.to_string();
                                            }
                                            done = true;
                                        },
                                        "error" => {
                                            // Error occurred
                                            let error_msg = payload
                                                .get("error")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Unknown error");
                                            bail!("Chat error: {}", error_msg);
                                        },
                                        _ => {
                                            // Other states (streaming, thinking, etc.) - ignore for now
                                        },
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        // Unknown message type - ignore
                    },
                }
            },
            Some(Err(e)) => bail!("WebSocket error: {}", e),
            None => break,
        }
    }

    // Close connection
    write.send(Message::Close(None)).await.ok();

    if response_text.is_empty() {
        bail!("No response received from agent");
    }

    Ok(response_text)
}

/// Send a message using HTTP POST (fallback method)
/// DO-178C §6.3.1: HTTP fallback communication
#[allow(dead_code)] // Reserved for future HTTP fallback implementation
pub async fn send_agent_message_http(gateway_url: &str, message: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let url = if gateway_url.ends_with("/") {
        format!("{}api/chat/send", gateway_url)
    } else {
        format!("{}/api/chat/send", gateway_url)
    };

    let response = client
        .post(&url)
        .json(&json!({
            "message": message,
            "session_key": "cli_test_session"
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("HTTP error: {}", response.status());
    }

    let result: Value = response.json().await?;

    if let Some(text) = result.get("text").and_then(|v| v.as_str()) {
        Ok(text.to_string())
    } else if let Some(error) = result.get("error") {
        bail!("Agent error: {}", error);
    } else {
        bail!("Unexpected response format: {}", result);
    }
}
