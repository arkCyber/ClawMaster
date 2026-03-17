//! Tool templates

pub fn generate_cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
async-trait = "0.1"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tokio = {{ version = "1.0", features = ["full"] }}

[dev-dependencies]
tempfile = "3.0"
"#,
        name
    )
}

pub fn generate_lib_rs(name: &str) -> String {
    format!(
        r#"//! {} - A ClawMaster Tool

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// {} tool implementation
pub struct {}Tool;

impl {}Tool {{
    pub fn new() -> Self {{
        Self
    }}
}}

// TODO: Implement Tool trait

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_tool_creation() {{
        let tool = {}Tool::new();
        // Add tests
    }}
}}
"#,
        name,
        name,
        to_pascal_case(name),
        to_pascal_case(name),
        to_pascal_case(name)
    )
}

pub fn generate_tool_code(name: &str) -> String {
    format!(
        r#"//! {} tool

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct {}Tool;

impl {}Tool {{
    pub fn new() -> Self {{
        Self
    }}

    pub async fn execute(&self, params: Value) -> Result<Value> {{
        // Implement tool logic here
        Ok(serde_json::json!({{
            "result": "success"
        }}))
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[tokio::test]
    async fn test_{}_tool() {{
        let tool = {}Tool::new();
        let result = tool.execute(serde_json::json!({{}})).await;
        assert!(result.is_ok());
    }}
}}
"#,
        name,
        to_pascal_case(name),
        to_pascal_case(name),
        name.replace("-", "_"),
        to_pascal_case(name)
    )
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
