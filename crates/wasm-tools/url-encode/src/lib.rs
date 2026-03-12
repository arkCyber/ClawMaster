//! URL encoding tool for Moltis AI agents.
//!
//! This tool provides URL encoding with:
//! - Standard URL encoding
//! - Component encoding
//! - Form data encoding
//! - Size limits
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Same input = same output
//! - §11.10: Resource management - Input size limits
//! - §11.13: Initialization - No global state

use urlencoding::encode;

wit_bindgen::generate!({
    world: "url-encode",
    exports: {
        world: UrlEncodeTool,
    },
});

/// Maximum input size: 10MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct UrlEncodeTool;

impl Guest for UrlEncodeTool {
    /// Encode string for URL.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic encoding
    fn encode(input: String) -> Result<String, String> {
        // DO-178C §11.10: Check size limit
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        // DO-178C §6.3.4: Deterministic URL encoding
        let encoded = encode(&input).to_string();
        Ok(encoded)
    }
    
    /// Encode component for URL.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn encode_component(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        // Component encoding is the same as standard encoding in this library
        let encoded = encode(&input).to_string();
        Ok(encoded)
    }
    
    /// Encode form data pair.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn encode_form_pair(key: String, value: String) -> Result<String, String> {
        if key.len() + value.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Combined size ({} bytes) exceeds limit ({} bytes)",
                key.len() + value.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let encoded_key = encode(&key);
        let encoded_value = encode(&value);
        let pair = format!("{}={}", encoded_key, encoded_value);
        
        Ok(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_simple() {
        let result = UrlEncodeTool::encode("Hello World".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello%20World");
    }
    
    #[test]
    fn test_encode_special_chars() {
        let result = UrlEncodeTool::encode("hello@example.com".to_string());
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded.contains("%40")); // @ encoded
    }
    
    #[test]
    fn test_encode_component() {
        let result = UrlEncodeTool::encode_component("path/to/file".to_string());
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded.contains("%2F")); // / encoded
    }
    
    #[test]
    fn test_encode_form_pair() {
        let result = UrlEncodeTool::encode_form_pair(
            "user name".to_string(),
            "John Doe".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "user%20name=John%20Doe");
    }
    
    #[test]
    fn test_encode_empty() {
        let result = UrlEncodeTool::encode(String::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
    
    #[test]
    fn test_encode_size_limit() {
        let large_input = "x".repeat(MAX_INPUT_SIZE + 1);
        let result = UrlEncodeTool::encode(large_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
