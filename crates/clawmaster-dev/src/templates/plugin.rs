//! Plugin templates

pub fn generate_manifest(name: &str) -> String {
    format!(
        r#"id = "{}"
name = "{}"
version = "0.1.0"
author = "Your Name <your.email@example.com>"
description = "A ClawMaster plugin"
homepage = "https://github.com/yourusername/{}"
license = "MIT"
tags = ["plugin"]

[[dependencies]]
# Add plugin dependencies here
# plugin_id = "other-plugin"
# version = "^1.0.0"
# optional = false

[config_schema]
type = "object"
properties = {{}}
required = []
"#,
        name, name, name
    )
}

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
        r#"//! {} - A ClawMaster Plugin
//!
//! This plugin provides...

use anyhow::Result;
use async_trait::async_trait;
use serde::{{Deserialize, Serialize}};

/// Plugin implementation
pub struct {}Plugin {{
    // Plugin state
}}

impl {}Plugin {{
    /// Create a new plugin instance
    pub fn new() -> Self {{
        Self {{}}
    }}
}}

impl Default for {}Plugin {{
    fn default() -> Self {{
        Self::new()
    }}
}}

// TODO: Implement Plugin trait

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_plugin_creation() {{
        let plugin = {}Plugin::new();
        // Add tests
    }}
}}
"#,
        name,
        to_pascal_case(name),
        to_pascal_case(name),
        to_pascal_case(name),
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
