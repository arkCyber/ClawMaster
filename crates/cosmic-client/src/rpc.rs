//! RPC client implementation for communicating with ClawMaster backend

use {
    anyhow::{Context, Result},
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, sync::Arc},
    tokio::sync::{RwLock, oneshot},
    tracing::debug,
};

/// RPC client for communicating with the ClawMaster backend
#[derive(Debug, Clone)]
pub struct RpcClient {
    base_url: String,
    client: reqwest::Client,
    next_request_id: Arc<std::sync::atomic::AtomicU64>,
    #[allow(dead_code)]
    pending_requests: Arc<RwLock<HashMap<u64, oneshot::Sender<RpcResponse>>>>,
}

/// RPC request
#[derive(Debug, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: serde_json::Value,
}

/// RPC response
#[derive(Debug, Deserialize)]
struct RpcResponse {
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    id: Option<u64>,
    result: Option<serde_json::Value>,
    error: Option<RpcErrorResponse>,
}

/// RPC error from server
#[derive(Debug, Deserialize)]
struct RpcErrorResponse {
    code: i32,
    message: String,
    #[allow(dead_code)]
    data: Option<serde_json::Value>,
}

/// RPC event from WebSocket
#[derive(Debug, Clone)]
pub enum RpcEvent {
    Message(crate::models::Message),
    SessionUpdate(crate::models::Session),
    SystemStatus(crate::models::SystemStatus),
    Error(String),
}

/// RPC error type
#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON-RPC error: {code} - {message}")]
    JsonRpc { code: i32, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Request timeout")]
    Timeout,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl RpcClient {
    /// Create a new RPC client
    pub async fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(tokio::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
            next_request_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Make an RPC call
    pub async fn call<T>(&self, method: &str, params: impl Serialize) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request_id = self
            .next_request_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let params_json = serde_json::to_value(params)?;

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: method.to_string(),
            params: params_json,
        };

        debug!("Making RPC call: {} (id: {})", method, request_id);

        let url = format!("{}/api/rpc", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send RPC request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        let rpc_response: RpcResponse = response
            .json()
            .await
            .context("Failed to parse RPC response")?;

        if let Some(error) = rpc_response.error {
            return Err(RpcError::JsonRpc {
                code: error.code,
                message: error.message,
            }
            .into());
        }

        let result = rpc_response
            .result
            .ok_or_else(|| RpcError::InvalidResponse("No result in response".to_string()))?;

        let typed_result: T =
            serde_json::from_value(result).context("Failed to deserialize RPC result")?;

        Ok(typed_result)
    }

    /// Make a simple GET request
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        debug!("Making GET request: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send GET request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GET request failed with status: {}",
                response.status()
            ));
        }

        let result: T = response
            .json()
            .await
            .context("Failed to parse GET response")?;

        Ok(result)
    }

    /// Make a simple POST request
    pub async fn post<T>(&self, endpoint: &str, data: impl Serialize) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        debug!("Making POST request: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&data)
            .send()
            .await
            .context("Failed to send POST request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "POST request failed with status: {}",
                response.status()
            ));
        }

        let result: T = response
            .json()
            .await
            .context("Failed to parse POST response")?;

        Ok(result)
    }

    /// Connect to WebSocket for real-time events
    pub async fn connect_websocket(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    > {
        use tokio_tungstenite::{connect_async, tungstenite::Message};

        let ws_url = format!("{}/ws", self.base_url.replace("http", "ws"));
        debug!("Connecting to WebSocket: {}", ws_url);

        let (ws_stream, _) = connect_async(&ws_url)
            .await
            .context("Failed to connect to WebSocket")?;

        Ok(ws_stream)
    }

    /// Get the next WebSocket event
    pub async fn next_event(&mut self) -> Result<RpcEvent> {
        // This is a simplified implementation
        // In a real implementation, we'd maintain a WebSocket connection
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // For now, return a placeholder
        // In a real implementation, we'd parse WebSocket messages
        Ok(RpcEvent::Error("Not implemented".to_string()))
    }

    /// Check if the server is reachable
    pub async fn ping(&self) -> Result<bool> {
        match self.get::<serde_json::Value>("ping").await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get server version
    pub async fn version(&self) -> Result<String> {
        let response: serde_json::Value = self.get("version").await?;
        Ok(response["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string())
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:59233".to_string(),
            client: reqwest::Client::new(),
            next_request_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Helper trait for converting between different error types
pub trait IntoRpcResult<T> {
    fn into_rpc_result(self) -> Result<T, RpcError>;
}

impl<T, E> IntoRpcResult<T> for Result<T, E>
where
    E: Into<RpcError>,
{
    fn into_rpc_result(self) -> Result<T, RpcError> {
        self.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_request_serialization() {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "test.method".to_string(),
            params: serde_json::json!({"param1": "value1"}),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test.method"));
        assert!(json.contains("value1"));
    }

    #[test]
    fn test_rpc_response_deserialization() {
        let json = r#"
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {"status": "ok"}
        }
        "#;

        let response: RpcResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, Some(1));
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = RpcClient::new("http://localhost:59233").await;
        assert!(client.is_ok());
    }
}
