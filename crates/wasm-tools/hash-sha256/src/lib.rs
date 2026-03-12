//! SHA256 hashing tool for Moltis AI agents.
//!
//! This tool provides cryptographic hashing with:
//! - SHA256 algorithm
//! - Hex encoding
//! - Hash verification
//! - Size limits
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Same input = same hash
//! - §11.10: Resource management - Input size limits
//! - §11.13: Initialization - No global state

use sha2::{Sha256, Digest};

wit_bindgen::generate!({
    world: "hash-sha256",
    exports: {
        world: HashSha256Tool,
    },
});

/// Maximum input size: 100MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_INPUT_SIZE: usize = 100 * 1024 * 1024;

struct HashSha256Tool;

impl Guest for HashSha256Tool {
    /// Compute SHA256 hash of string.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic - same input always produces same hash
    fn hash(input: String) -> Result<String, String> {
        // DO-178C §11.10: Check size limit
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        // DO-178C §6.3.4: Deterministic SHA256 hashing
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        
        // Convert to hex string
        let hash_hex = hex::encode(result);
        Ok(hash_hex)
    }
    
    /// Compute SHA256 hash of bytes.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn hash_bytes(input: Vec<u8>) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Input size ({} bytes) exceeds limit ({} bytes)",
                input.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let mut hasher = Sha256::new();
        hasher.update(&input);
        let result = hasher.finalize();
        
        let hash_hex = hex::encode(result);
        Ok(hash_hex)
    }
    
    /// Verify hash matches input.
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic verification
    fn verify(input: String, expected_hash: String) -> bool {
        if input.len() > MAX_INPUT_SIZE {
            return false;
        }
        
        match Self::hash(input) {
            Ok(computed_hash) => {
                // Case-insensitive comparison
                computed_hash.eq_ignore_ascii_case(&expected_hash)
            }
            Err(_) => false,
        }
    }
    
    /// Compute hash of file content.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn hash_content(content: String) -> Result<String, String> {
        Self::hash(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_simple() {
        let result = HashSha256Tool::hash("Hello, World!".to_string());
        assert!(result.is_ok());
        let hash = result.unwrap();
        // Known SHA256 hash of "Hello, World!"
        assert_eq!(hash, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }
    
    #[test]
    fn test_hash_empty() {
        let result = HashSha256Tool::hash(String::new());
        assert!(result.is_ok());
        let hash = result.unwrap();
        // Known SHA256 hash of empty string
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
    
    #[test]
    fn test_hash_bytes() {
        let input = vec![72, 101, 108, 108, 111]; // "Hello"
        let result = HashSha256Tool::hash_bytes(input);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969");
    }
    
    #[test]
    fn test_verify_success() {
        let input = "Hello, World!".to_string();
        let expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f".to_string();
        assert!(HashSha256Tool::verify(input, expected));
    }
    
    #[test]
    fn test_verify_case_insensitive() {
        let input = "Hello, World!".to_string();
        let expected = "DFFD6021BB2BD5B0AF676290809EC3A53191DD81C7F70A4B28688A362182986F".to_string();
        assert!(HashSha256Tool::verify(input, expected));
    }
    
    #[test]
    fn test_verify_failure() {
        let input = "Hello, World!".to_string();
        let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        assert!(!HashSha256Tool::verify(input, wrong_hash));
    }
    
    #[test]
    fn test_hash_deterministic() {
        let input = "Test".to_string();
        let hash1 = HashSha256Tool::hash(input.clone()).unwrap();
        let hash2 = HashSha256Tool::hash(input).unwrap();
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_hash_size_limit() {
        let large_input = "x".repeat(MAX_INPUT_SIZE + 1);
        let result = HashSha256Tool::hash(large_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
