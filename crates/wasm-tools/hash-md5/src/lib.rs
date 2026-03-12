//! MD5 hash tool for Moltis AI agents.

use md5::{Md5, Digest};

wit_bindgen::generate!({
    world: "hash-md5",
    exports: {
        world: HashMd5Tool,
    },
});

struct HashMd5Tool;

impl Guest for HashMd5Tool {
    fn hash(input: String) -> String {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    fn hash_bytes(input: Vec<u8>) -> String {
        let mut hasher = Md5::new();
        hasher.update(&input);
        format!("{:x}", hasher.finalize())
    }
    
    fn verify(input: String, expected: String) -> bool {
        let hash = Self::hash(input);
        hash.eq_ignore_ascii_case(&expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let result = HashMd5Tool::hash("hello".to_string());
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }
    
    #[test]
    fn test_hash_bytes() {
        let result = HashMd5Tool::hash_bytes(vec![104, 101, 108, 108, 111]);
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }
    
    #[test]
    fn test_verify() {
        assert!(HashMd5Tool::verify("hello".to_string(), "5d41402abc4b2a76b9719d911017c592".to_string()));
        assert!(!HashMd5Tool::verify("hello".to_string(), "wrong".to_string()));
    }
}
