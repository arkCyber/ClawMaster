//! Base64 encoding tool for Moltis AI agents.
//!
//! This tool provides Base64 encoding with:
//! - Standard Base64
//! - URL-safe Base64
//! - Size limits
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Same input = same output
//! - §11.10: Resource management - Input size limits
//! - §11.13: Initialization - No global state

use base64::{Engine as _, engine::general_purpose};

wit_bindgen::generate!({
    world: "base64-encode",
    exports: {
        world: Base64EncodeTool,
    },
});

/// Maximum input size: 10MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct Base64EncodeTool;

impl Guest for Base64EncodeTool {
    /// Encode string to Base64.
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
        
        // DO-178C §6.3.4: Deterministic Base64 encoding
        let encoded = general_purpose::STANDARD.encode(input.as_bytes());
        Ok(encoded)
    }
    
    /// Encode bytes to Base64.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn encode_bytes(input: Vec<u8>) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let encoded = general_purpose::STANDARD.encode(&input);
        Ok(encoded)
    }
    
    /// Encode with URL-safe alphabet.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    /// DO-178C §6.3.4: Deterministic URL-safe encoding
    fn encode_url_safe(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let encoded = general_purpose::URL_SAFE.encode(input.as_bytes());
        Ok(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_simple() {
        let result = Base64EncodeTool::encode("Hello, World!".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "SGVsbG8sIFdvcmxkIQ==");
    }
    
    #[test]
    fn test_encode_empty() {
        let result = Base64EncodeTool::encode(String::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
    
    #[test]
    fn test_encode_bytes() {
        let input = vec![72, 101, 108, 108, 111]; // "Hello"
        let result = Base64EncodeTool::encode_bytes(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "SGVsbG8=");
    }
    
    #[test]
    fn test_encode_url_safe() {
        let input = "Hello+World/Test=".to_string();
        let result = Base64EncodeTool::encode_url_safe(input);
        assert!(result.is_ok());
        // URL-safe uses - and _ instead of + and /
        let encoded = result.unwrap();
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
    }
    
    #[test]
    fn test_encode_size_limit() {
        let large_input = "x".repeat(MAX_INPUT_SIZE + 1);
        let result = Base64EncodeTool::encode(large_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
