//! List available agents for subagent spawning.
//!
//! Provides information about configured agents that can be used with sessions_spawn.
//! Respects per-agent allowlists for security.

use {
    anyhow::{Result, bail},
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::sync::Arc,
};

/// Agent information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub model: Option<String>,
    pub capabilities: Vec<String>,
    pub available_for_spawn: bool,
}

/// Agent registry trait.
pub trait AgentRegistry: Send + Sync {
    fn list_agents(&self) -> Vec<AgentInfo>;
    fn get_agent(&self, id: &str) -> Option<AgentInfo>;
    fn is_agent_allowed(&self, requester_id: &str, target_id: &str) -> bool;
}

/// Simple in-memory agent registry.
#[derive(Debug, Clone)]
pub struct SimpleAgentRegistry {
    agents: Vec<AgentInfo>,
    allowlists: std::collections::HashMap<String, Vec<String>>,
}

impl SimpleAgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            allowlists: std::collections::HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, agent: AgentInfo) {
        self.agents.push(agent);
    }

    pub fn set_allowlist(&mut self, agent_id: String, allowed: Vec<String>) {
        self.allowlists.insert(agent_id, allowed);
    }
}

impl Default for SimpleAgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRegistry for SimpleAgentRegistry {
    fn list_agents(&self) -> Vec<AgentInfo> {
        self.agents.clone()
    }

    fn get_agent(&self, id: &str) -> Option<AgentInfo> {
        self.agents.iter().find(|a| a.id == id).cloned()
    }

    fn is_agent_allowed(&self, requester_id: &str, target_id: &str) -> bool {
        if let Some(allowlist) = self.allowlists.get(requester_id) {
            // Check for wildcard
            if allowlist.contains(&"*".to_string()) {
                return true;
            }
            // Check for specific agent
            allowlist.contains(&target_id.to_string())
        } else {
            // No allowlist means allow all
            true
        }
    }
}

/// Agents list tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentsListConfig {
    pub enabled: bool,
    pub include_capabilities: bool,
}

impl Default for AgentsListConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_capabilities: true,
        }
    }
}

/// Agents list tool.
pub struct AgentsListTool {
    config: AgentsListConfig,
    registry: Arc<dyn AgentRegistry>,
    current_agent_id: String,
}

impl AgentsListTool {
    pub fn new(config: AgentsListConfig, registry: Arc<dyn AgentRegistry>) -> Self {
        Self {
            config,
            registry,
            current_agent_id: "default".to_string(),
        }
    }

    pub fn with_current_agent(mut self, agent_id: String) -> Self {
        self.current_agent_id = agent_id;
        self
    }

    /// List agents available to the current agent.
    pub fn list_available_agents(&self) -> Vec<AgentInfo> {
        let all_agents = self.registry.list_agents();

        // Filter by allowlist
        all_agents
            .into_iter()
            .filter(|agent| {
                agent.available_for_spawn
                    && self
                        .registry
                        .is_agent_allowed(&self.current_agent_id, &agent.id)
            })
            .collect()
    }

    /// Get detailed information about a specific agent.
    pub fn get_agent_info(&self, agent_id: &str) -> Option<AgentInfo> {
        // Check if agent is allowed
        if !self
            .registry
            .is_agent_allowed(&self.current_agent_id, agent_id)
        {
            return None;
        }

        self.registry.get_agent(agent_id)
    }
}

#[async_trait]
impl AgentTool for AgentsListTool {
    fn name(&self) -> &str {
        "agents_list"
    }

    fn description(&self) -> &str {
        "List available agents that can be used with sessions_spawn"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "get"],
                    "description": "Action to perform: list all agents or get specific agent info"
                },
                "agent_id": {
                    "type": "string",
                    "description": "Agent ID (required for 'get' action)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        if !self.config.enabled {
            bail!("agents_list tool is disabled");
        }

        let action = params
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action' parameter"))?;

        match action {
            "list" => {
                let agents = self.list_available_agents();

                // Check if all agents are allowed (wildcard)
                let allow_any = self.registry.is_agent_allowed(&self.current_agent_id, "*");

                let agents_json: Vec<Value> = agents
                    .into_iter()
                    .map(|agent| {
                        let mut obj = json!({
                            "id": agent.id,
                            "name": agent.name,
                        });

                        if let Some(desc) = agent.description {
                            obj["description"] = json!(desc);
                        }

                        if let Some(model) = agent.model {
                            obj["model"] = json!(model);
                        }

                        if self.config.include_capabilities && !agent.capabilities.is_empty() {
                            obj["capabilities"] = json!(agent.capabilities);
                        }

                        obj
                    })
                    .collect();

                Ok(json!({
                    "agents": agents_json,
                    "count": agents_json.len(),
                    "allowAny": allow_any,
                }))
            },
            "get" => {
                let agent_id = params
                    .get("agent_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'agent_id' parameter"))?;

                if let Some(agent) = self.get_agent_info(agent_id) {
                    let mut obj = json!({
                        "id": agent.id,
                        "name": agent.name,
                        "available": agent.available_for_spawn,
                    });

                    if let Some(desc) = agent.description {
                        obj["description"] = json!(desc);
                    }

                    if let Some(model) = agent.model {
                        obj["model"] = json!(model);
                    }

                    if self.config.include_capabilities {
                        obj["capabilities"] = json!(agent.capabilities);
                    }

                    Ok(obj)
                } else {
                    bail!("Agent '{}' not found or not allowed", agent_id);
                }
            },
            _ => bail!("Invalid action: {}", action),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_registry() -> SimpleAgentRegistry {
        let mut registry = SimpleAgentRegistry::new();

        registry.add_agent(AgentInfo {
            id: "agent1".to_string(),
            name: "Agent 1".to_string(),
            description: Some("First agent".to_string()),
            model: Some("claude-3-5-sonnet".to_string()),
            capabilities: vec!["coding".to_string(), "analysis".to_string()],
            available_for_spawn: true,
        });

        registry.add_agent(AgentInfo {
            id: "agent2".to_string(),
            name: "Agent 2".to_string(),
            description: Some("Second agent".to_string()),
            model: Some("gpt-4".to_string()),
            capabilities: vec!["writing".to_string()],
            available_for_spawn: true,
        });

        registry.add_agent(AgentInfo {
            id: "agent3".to_string(),
            name: "Agent 3".to_string(),
            description: None,
            model: None,
            capabilities: vec![],
            available_for_spawn: false,
        });

        registry
    }

    #[test]
    fn test_simple_registry() {
        let registry = create_test_registry();

        let agents = registry.list_agents();
        assert_eq!(agents.len(), 3);

        let agent1 = registry.get_agent("agent1");
        assert!(agent1.is_some());
        assert_eq!(agent1.unwrap().name, "Agent 1");

        let unknown = registry.get_agent("unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_allowlist() {
        let mut registry = create_test_registry();

        // Set allowlist for default agent
        registry.set_allowlist("default".to_string(), vec!["agent1".to_string()]);

        assert!(registry.is_agent_allowed("default", "agent1"));
        assert!(!registry.is_agent_allowed("default", "agent2"));
    }

    #[test]
    fn test_wildcard_allowlist() {
        let mut registry = create_test_registry();

        // Set wildcard allowlist
        registry.set_allowlist("default".to_string(), vec!["*".to_string()]);

        assert!(registry.is_agent_allowed("default", "agent1"));
        assert!(registry.is_agent_allowed("default", "agent2"));
        assert!(registry.is_agent_allowed("default", "any_agent"));
    }

    #[tokio::test]
    async fn test_tool_list_action() {
        let registry = Arc::new(create_test_registry());
        let config = AgentsListConfig::default();
        let tool = AgentsListTool::new(config, registry);

        let params = json!({"action": "list"});
        let result = tool.execute(params).await.unwrap();

        assert!(result["agents"].is_array());
        let agents = result["agents"].as_array().unwrap();
        assert_eq!(agents.len(), 2); // Only available_for_spawn agents
        assert_eq!(result["count"], 2);
    }

    #[tokio::test]
    async fn test_tool_get_action() {
        let registry = Arc::new(create_test_registry());
        let config = AgentsListConfig::default();
        let tool = AgentsListTool::new(config, registry);

        let params = json!({"action": "get", "agent_id": "agent1"});
        let result = tool.execute(params).await.unwrap();

        assert_eq!(result["id"], "agent1");
        assert_eq!(result["name"], "Agent 1");
        assert_eq!(result["available"], true);
    }

    #[tokio::test]
    async fn test_tool_with_allowlist() {
        let mut registry = create_test_registry();
        registry.set_allowlist("default".to_string(), vec!["agent1".to_string()]);

        let config = AgentsListConfig::default();
        let tool = AgentsListTool::new(config, Arc::new(registry));

        let params = json!({"action": "list"});
        let result = tool.execute(params).await.unwrap();

        let agents = result["agents"].as_array().unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0]["id"], "agent1");
    }

    #[tokio::test]
    async fn test_tool_get_not_allowed() {
        let mut registry = create_test_registry();
        registry.set_allowlist("default".to_string(), vec!["agent1".to_string()]);

        let config = AgentsListConfig::default();
        let tool = AgentsListTool::new(config, Arc::new(registry));

        let params = json!({"action": "get", "agent_id": "agent2"});
        let result = tool.execute(params).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not found or not allowed")
        );
    }
}
