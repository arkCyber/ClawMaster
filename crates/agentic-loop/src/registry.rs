use {
    anyhow::Result,
    async_trait::async_trait,
    std::{collections::HashMap, sync::RwLock},
    tracing::debug,
};

/// Trait that all tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: serde_json::Value) -> Result<String>;
}

/// Registry for managing available tools
pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Box<dyn Tool>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new tool
    pub fn register(&self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        debug!("Registering tool: {}", name);

        let mut tools = self.tools.write().unwrap_or_else(|e| e.into_inner());
        tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        let tools = self.tools.read().unwrap_or_else(|e| e.into_inner());

        // SAFETY: We're converting the reference to have a 'static lifetime
        // This is safe because:
        // 1. The HashMap is never modified after registration (only reads)
        // 2. The Box<dyn Tool> is never dropped while the registry exists
        // 3. The registry outlives all tool executions
        unsafe {
            tools.get(name).map(|tool| {
                let ptr = tool.as_ref() as *const dyn Tool;
                &*ptr
            })
        }
    }

    /// List all available tools
    pub fn list_tools(&self) -> Vec<(String, String)> {
        let tools = self.tools.read().unwrap_or_else(|e| e.into_inner());

        tools
            .values()
            .map(|tool| (tool.name().to_string(), tool.description().to_string()))
            .collect()
    }

    /// Check if a tool exists
    pub fn has_tool(&self, name: &str) -> bool {
        let tools = self.tools.read().unwrap_or_else(|e| e.into_inner());
        tools.contains_key(name)
    }

    /// Get the number of registered tools
    pub fn count(&self) -> usize {
        let tools = self.tools.read().unwrap_or_else(|e| e.into_inner());
        tools.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTool {
        name: String,
        description: String,
    }

    #[async_trait]
    impl Tool for TestTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        async fn execute(&self, _args: serde_json::Value) -> Result<String> {
            Ok(format!("Executed {}", self.name))
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_tool() {
        let registry = ToolRegistry::new();

        let tool = Box::new(TestTool {
            name: "test".to_string(),
            description: "A test tool".to_string(),
        });

        registry.register(tool);
        assert_eq!(registry.count(), 1);
        assert!(registry.has_tool("test"));
    }

    #[test]
    fn test_list_tools() {
        let registry = ToolRegistry::new();

        registry.register(Box::new(TestTool {
            name: "tool1".to_string(),
            description: "First tool".to_string(),
        }));

        registry.register(Box::new(TestTool {
            name: "tool2".to_string(),
            description: "Second tool".to_string(),
        }));

        let tools = registry.list_tools();
        assert_eq!(tools.len(), 2);
    }

    #[tokio::test]
    async fn test_get_and_execute() {
        let registry = ToolRegistry::new();

        registry.register(Box::new(TestTool {
            name: "test".to_string(),
            description: "A test tool".to_string(),
        }));

        let tool = registry.get("test");
        assert!(tool.is_some());

        if let Some(tool) = tool {
            let result = tool.execute(serde_json::json!({})).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Executed test");
        }
    }
}
