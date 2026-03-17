//! Gateway configuration and management tool.
//!
//! Provides runtime configuration query and modification capabilities.
//! This is a sensitive tool that accesses system configuration, so it should
//! be implemented as a WASM tool for enhanced security isolation.

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::sync::Arc,
};

/// Gateway configuration provider trait.
pub trait GatewayConfigProvider: Send + Sync {
    fn get_config(&self, key: &str) -> Result<Value>;
    fn set_config(&self, key: &str, value: Value) -> Result<()>;
    fn list_config_keys(&self) -> Vec<String>;
    fn restart_gateway(&self) -> Result<()>;
    fn get_version(&self) -> String;
    fn get_status(&self) -> GatewayStatus;
}

/// Gateway status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub running: bool,
    pub uptime_seconds: u64,
    pub active_sessions: usize,
    pub total_requests: u64,
    pub version: String,
}

/// Gateway configuration tool.
pub struct GatewayConfigTool {
    provider: Arc<dyn GatewayConfigProvider>,
    allow_restart: bool,
    allow_config_write: bool,
}

impl GatewayConfigTool {
    pub fn new(provider: Arc<dyn GatewayConfigProvider>) -> Self {
        Self {
            provider,
            allow_restart: false,
            allow_config_write: false,
        }
    }

    pub fn with_restart_enabled(mut self, enabled: bool) -> Self {
        self.allow_restart = enabled;
        self
    }

    pub fn with_config_write_enabled(mut self, enabled: bool) -> Self {
        self.allow_config_write = enabled;
        self
    }
}

#[async_trait]
impl AgentTool for GatewayConfigTool {
    fn name(&self) -> &str {
        "gateway"
    }

    fn description(&self) -> &str {
        "Query and manage gateway configuration and status"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["get", "set", "list", "status", "restart", "version"],
                    "description": "Action to perform"
                },
                "key": {
                    "type": "string",
                    "description": "Configuration key (for get/set actions)"
                },
                "value": {
                    "description": "Configuration value (for set action)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' parameter"))?;

        match action {
            "get" => {
                let key = params.get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'key' parameter"))?;

                let value = self.provider.get_config(key)?;
                Ok(json!({
                    "key": key,
                    "value": value
                }))
            }
            "set" => {
                if !self.allow_config_write {
                    bail!("Configuration write is not enabled for this tool");
                }

                let key = params.get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'key' parameter"))?;

                let value = params.get("value")
                    .ok_or_else(|| anyhow::anyhow!("Missing 'value' parameter"))?
                    .clone();

                self.provider.set_config(key, value.clone())?;
                Ok(json!({
                    "key": key,
                    "value": value,
                    "status": "updated"
                }))
            }
            "list" => {
                let keys = self.provider.list_config_keys();
                Ok(json!({
                    "keys": keys,
                    "count": keys.len()
                }))
            }
            "status" => {
                let status = self.provider.get_status();
                Ok(serde_json::to_value(status)?)
            }
            "restart" => {
                if !self.allow_restart {
                    bail!("Gateway restart is not enabled for this tool");
                }

                self.provider.restart_gateway()?;
                Ok(json!({
                    "status": "restarting",
                    "message": "Gateway restart initiated"
                }))
            }
            "version" => {
                let version = self.provider.get_version();
                Ok(json!({
                    "version": version
                }))
            }
            _ => bail!("Invalid action: {}", action),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::RwLock;

    struct MockConfigProvider {
        config: RwLock<HashMap<String, Value>>,
        restart_count: RwLock<usize>,
    }

    impl MockConfigProvider {
        fn new() -> Self {
            let mut config = HashMap::new();
            config.insert("server.port".to_string(), json!(3000));
            config.insert("server.host".to_string(), json!("localhost"));
            config.insert("log.level".to_string(), json!("info"));

            Self {
                config: RwLock::new(config),
                restart_count: RwLock::new(0),
            }
        }
    }

    impl GatewayConfigProvider for MockConfigProvider {
        fn get_config(&self, key: &str) -> Result<Value> {
            let config = self.config.read().unwrap();
            config.get(key)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key))
        }

        fn set_config(&self, key: &str, value: Value) -> Result<()> {
            let mut config = self.config.write().unwrap();
            config.insert(key.to_string(), value);
            Ok(())
        }

        fn list_config_keys(&self) -> Vec<String> {
            let config = self.config.read().unwrap();
            config.keys().cloned().collect()
        }

        fn restart_gateway(&self) -> Result<()> {
            let mut count = self.restart_count.write().unwrap();
            *count += 1;
            Ok(())
        }

        fn get_version(&self) -> String {
            "0.10.18".to_string()
        }

        fn get_status(&self) -> GatewayStatus {
            GatewayStatus {
                running: true,
                uptime_seconds: 3600,
                active_sessions: 5,
                total_requests: 1000,
                version: "0.10.18".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn test_get_config() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({"action": "get", "key": "server.port"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["key"], "server.port");
        assert_eq!(result["value"], 3000);
    }

    #[tokio::test]
    async fn test_set_config() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider)
            .with_config_write_enabled(true);

        let params = json!({
            "action": "set",
            "key": "server.port",
            "value": 8080
        });
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["status"], "updated");
        assert_eq!(result["value"], 8080);
    }

    #[tokio::test]
    async fn test_set_config_disabled() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({
            "action": "set",
            "key": "server.port",
            "value": 8080
        });
        let result = tool.execute(params).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not enabled"));
    }

    #[tokio::test]
    async fn test_list_config() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({"action": "list"});
        let result = tool.execute(params).await.unwrap();

        assert!(result["keys"].is_array());
        assert_eq!(result["count"], 3);
    }

    #[tokio::test]
    async fn test_status() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({"action": "status"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["running"], true);
        assert_eq!(result["version"], "0.10.18");
        assert_eq!(result["active_sessions"], 5);
    }

    #[tokio::test]
    async fn test_version() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({"action": "version"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["version"], "0.10.18");
    }

    #[tokio::test]
    async fn test_restart() {
        let mock_provider = Arc::new(MockConfigProvider::new());
        let provider: Arc<dyn GatewayConfigProvider> = Arc::clone(&mock_provider) as Arc<dyn GatewayConfigProvider>;
        let tool = GatewayConfigTool::new(provider)
            .with_restart_enabled(true);

        let params = json!({"action": "restart"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["status"], "restarting");

        // Verify restart was called
        let count = mock_provider.restart_count.read().unwrap();
        assert_eq!(*count, 1);
    }

    #[tokio::test]
    async fn test_restart_disabled() {
        let provider = Arc::new(MockConfigProvider::new());
        let tool = GatewayConfigTool::new(provider);

        let params = json!({"action": "restart"});
        let result = tool.execute(params).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not enabled"));
    }
}
