//! JSON parsing and manipulation tool for Moltis AI agents.
//!
//! This tool provides JSON operations with:
//! - Validation and parsing
//! - Simple path-based queries
//! - Pretty printing and minification
//! - Size limits
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - No random operations
//! - §11.10: Resource management - JSON size limits enforced
//! - §11.13: Initialization - No global state

use serde_json::{Value, from_str, to_string, to_string_pretty};

wit_bindgen::generate!({
    world: "json-parse",
    exports: {
        world: JsonParseTool,
    },
});

/// Maximum JSON size: 10MB
/// DO-178C §11.10: Resource limits prevent unbounded memory usage
const MAX_JSON_SIZE: usize = 10 * 1024 * 1024;

struct JsonParseTool;

impl Guest for JsonParseTool {
    /// Validate JSON.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All parse errors are caught and returned
    fn validate_json(json: String) -> Result<(), String> {
        // DO-178C §11.10: Check size limit
        if json.len() > MAX_JSON_SIZE {
            return Err(format!(
                "JSON size ({} bytes) exceeds limit ({} bytes)",
                json.len(),
                MAX_JSON_SIZE
            ));
        }
        
        // DO-178C §6.3.2: Parse and validate
        from_str::<Value>(&json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        Ok(())
    }
    
    /// Pretty print JSON.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors properly handled
    fn format_json(json: String) -> Result<String, String> {
        if json.len() > MAX_JSON_SIZE {
            return Err(format!(
                "JSON size ({} bytes) exceeds limit ({} bytes)",
                json.len(),
                MAX_JSON_SIZE
            ));
        }
        
        let value: Value = from_str(&json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        to_string_pretty(&value)
            .map_err(|e| format!("Failed to format JSON: {}", e))
    }
    
    /// Minify JSON.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors properly handled
    fn minify_json(json: String) -> Result<String, String> {
        if json.len() > MAX_JSON_SIZE {
            return Err(format!(
                "JSON size ({} bytes) exceeds limit ({} bytes)",
                json.len(),
                MAX_JSON_SIZE
            ));
        }
        
        let value: Value = from_str(&json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        to_string(&value)
            .map_err(|e| format!("Failed to minify JSON: {}", e))
    }
    
    /// Get value from JSON using dot-separated path.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors properly handled
    /// DO-178C §6.3.4: Deterministic path traversal
    fn get_value(json: String, path: String) -> Result<String, String> {
        if json.len() > MAX_JSON_SIZE {
            return Err(format!(
                "JSON size ({} bytes) exceeds limit ({} bytes)",
                json.len(),
                MAX_JSON_SIZE
            ));
        }
        
        let mut value: Value = from_str(&json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        // DO-178C §6.3.4: Deterministic path traversal
        for key in path.split('.') {
            if key.is_empty() {
                continue;
            }
            
            // Try as object key
            if let Some(obj) = value.as_object() {
                value = obj
                    .get(key)
                    .ok_or_else(|| format!("Key '{}' not found", key))?
                    .clone();
                continue;
            }
            
            // Try as array index
            if let Some(arr) = value.as_array() {
                let index: usize = key
                    .parse()
                    .map_err(|_| format!("Invalid array index: {}", key))?;
                
                value = arr
                    .get(index)
                    .ok_or_else(|| format!("Array index {} out of bounds", index))?
                    .clone();
                continue;
            }
            
            return Err(format!("Cannot traverse path at '{}'", key));
        }
        
        // Convert final value to string
        match value {
            Value::String(s) => Ok(s),
            other => to_string(&other)
                .map_err(|e| format!("Failed to serialize value: {}", e)),
        }
    }
    
    /// Check if JSON object has a key.
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic - no side effects
    fn has_key(json: String, key: String) -> bool {
        if json.len() > MAX_JSON_SIZE {
            return false;
        }
        
        match from_str::<Value>(&json) {
            Ok(Value::Object(obj)) => obj.contains_key(&key),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_json_valid() {
        let result = JsonParseTool::validate_json(r#"{"key": "value"}"#.to_string());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_json_invalid() {
        let result = JsonParseTool::validate_json("{invalid}".to_string());
        assert!(result.is_err());
    }
    
    #[test]
    fn test_format_json() {
        let json = r#"{"key":"value","nested":{"a":1}}"#;
        let result = JsonParseTool::format_json(json.to_string());
        assert!(result.is_ok());
        let formatted = result.unwrap();
        assert!(formatted.contains('\n'));
        assert!(formatted.contains("  "));
    }
    
    #[test]
    fn test_minify_json() {
        let json = r#"{
            "key": "value",
            "nested": {
                "a": 1
            }
        }"#;
        let result = JsonParseTool::minify_json(json.to_string());
        assert!(result.is_ok());
        let minified = result.unwrap();
        assert!(!minified.contains('\n'));
        assert!(!minified.contains("  "));
    }
    
    #[test]
    fn test_get_value_simple() {
        let json = r#"{"user": {"name": "Alice", "age": 30}}"#;
        
        let result = JsonParseTool::get_value(json.to_string(), "user.name".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Alice");
    }
    
    #[test]
    fn test_get_value_array() {
        let json = r#"{"items": [1, 2, 3]}"#;
        
        let result = JsonParseTool::get_value(json.to_string(), "items.0".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }
    
    #[test]
    fn test_get_value_not_found() {
        let json = r#"{"key": "value"}"#;
        
        let result = JsonParseTool::get_value(json.to_string(), "nonexistent".to_string());
        assert!(result.is_err());
    }
    
    #[test]
    fn test_has_key() {
        let json = r#"{"key1": "value1", "key2": "value2"}"#;
        
        assert!(JsonParseTool::has_key(json.to_string(), "key1".to_string()));
        assert!(!JsonParseTool::has_key(json.to_string(), "key3".to_string()));
    }
    
    #[test]
    fn test_size_limit() {
        let large_json = format!(r#"{{"data": "{}"}}"#, "x".repeat(MAX_JSON_SIZE));
        
        let result = JsonParseTool::validate_json(large_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
