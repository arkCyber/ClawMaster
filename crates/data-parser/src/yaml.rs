//! YAML parsing functionality.

use crate::{ParserError, Result};
use serde_json::Value;

/// YAML parser with JSON output.
pub struct YamlParser;

impl YamlParser {
    /// Create a new YAML parser.
    pub fn new() -> Self {
        Self
    }
    
    /// Parse YAML string to JSON.
    pub fn parse_to_json(&self, input: &str) -> Result<String> {
        let value: Value = serde_yaml::from_str(input)?;
        let json = serde_json::to_string(&value)?;
        Ok(json)
    }
    
    /// Parse YAML string to JSON (pretty printed).
    pub fn parse_to_json_pretty(&self, input: &str) -> Result<String> {
        let value: Value = serde_yaml::from_str(input)?;
        let json = serde_json::to_string_pretty(&value)?;
        Ok(json)
    }
    
    /// Validate YAML format.
    pub fn validate(&self, input: &str) -> Result<bool> {
        let _: Value = serde_yaml::from_str(input)?;
        Ok(true)
    }
    
    /// Get value by key path (dot notation).
    pub fn get_value(&self, input: &str, path: &str) -> Result<String> {
        let value: Value = serde_yaml::from_str(input)?;
        
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &value;
        
        for part in parts {
            current = current.get(part)
                .ok_or_else(|| ParserError::InvalidInput(format!("Key not found: {}", part)))?;
        }
        
        let result = serde_json::to_string(current)?;
        Ok(result)
    }
}

impl Default for YamlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_to_json() {
        let yaml = r#"
name: Alice
age: 30
city: NYC
"#;
        let parser = YamlParser::new();
        let result = parser.parse_to_json(yaml).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("30"));
    }
    
    #[test]
    fn test_validate() {
        let yaml = r#"
key: value
list:
  - item1
  - item2
"#;
        let parser = YamlParser::new();
        assert!(parser.validate(yaml).unwrap());
    }
    
    #[test]
    fn test_get_value() {
        let yaml = r#"
user:
  name: Alice
  age: 30
"#;
        let parser = YamlParser::new();
        let result = parser.get_value(yaml, "user.name").unwrap();
        assert!(result.contains("Alice"));
    }
}
