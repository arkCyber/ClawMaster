//! Base64 decoding tool for Moltis AI agents.
//!
//! This tool provides Base64 decoding with:
//! - Standard Base64
//! - URL-safe Base64
//! - Validation
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
    world: "base64-decode",
    exports: {
        world: Base64DecodeTool,
    },
});

/// Maximum input size: 10MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct Base64DecodeTool;

impl Guest for Base64DecodeTool {
    /// Decode Base64 to string.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic decoding
    fn decode(input: String) -> Result<String, String> {
        // DO-178C §11.10: Check size limit
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        // DO-178C §6.3.2: Decode with error handling
        let decoded_bytes = general_purpose::STANDARD
            .decode(input.as_bytes())
            .map_err(|e| format!("Invalid Base64: {}", e))?;
        
        // DO-178C §6.3.2: Validate UTF-8
        let decoded_string = String::from_utf8(decoded_bytes)
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        
        Ok(decoded_string)
    }
    
    /// Decode Base64 to bytes.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn decode_bytes(input: String) -> Result<Vec<u8>, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let decoded = general_purpose::STANDARD
            .decode(input.as_bytes())
            .map_err(|e| format!("Invalid Base64: {}", e))?;
        
        Ok(decoded)
    }
    
    /// Decode URL-safe Base64.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    /// DO-178C §6.3.4: Deterministic URL-safe decoding
    fn decode_url_safe(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let decoded_bytes = general_purpose::URL_SAFE
            .decode(input.as_bytes())
            .map_err(|e| format!("Invalid Base64: {}", e))?;
        
        let decoded_string = String::from_utf8(decoded_bytes)
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        
        Ok(decoded_string)
    }
    
    /// Validate Base64 string.
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic - no side effects
    fn is_valid(input: String) -> bool {
        if input.len() > MAX_INPUT_SIZE {
            return false;
        }
        
        general_purpose::STANDARD.decode(input.as_bytes()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_simple() {
        let result = Base64DecodeTool::decode("SGVsbG8sIFdvcmxkIQ==".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }
    
    #[test]
    fn test_decode_empty() {
        let result = Base64DecodeTool::decode(String::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
    
    #[test]
    fn test_decode_invalid() {
        let result = Base64DecodeTool::decode("Invalid!!!".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Base64"));
    }
    
    #[test]
    fn test_decode_bytes() {
        let result = Base64DecodeTool::decode_bytes("SGVsbG8=".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![72, 101, 108, 108, 111]); // "Hello"
    }
    
    #[test]
    fn test_decode_url_safe() {
        // First encode with URL-safe
        let encoded = general_purpose::URL_SAFE.encode("Hello+World/Test=");
        let result = Base64DecodeTool::decode_url_safe(encoded);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello+World/Test=");
    }
    
    #[test]
    fn test_is_valid() {
        assert!(Base64DecodeTool::is_valid("SGVsbG8=".to_string()));
        assert!(!Base64DecodeTool::is_valid("Invalid!!!".to_string()));
        assert!(Base64DecodeTool::is_valid(String::new()));
    }
    
    #[test]
    fn test_decode_size_limit() {
        let large_input = "A".repeat(MAX_INPUT_SIZE + 1);
        let result = Base64DecodeTool::decode(large_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
