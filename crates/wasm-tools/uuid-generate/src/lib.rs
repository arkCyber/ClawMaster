//! UUID generation tool for Moltis AI agents.

use uuid::Uuid;

wit_bindgen::generate!({
    world: "uuid-generate",
    exports: {
        world: UuidGenerateTool,
    },
});

struct UuidGenerateTool;

impl Guest for UuidGenerateTool {
    fn v4() -> Result<String, String> {
        let uuid = Uuid::new_v4();
        Ok(uuid.to_string())
    }
    
    fn v4_hyphenated() -> Result<String, String> {
        let uuid = Uuid::new_v4();
        Ok(uuid.hyphenated().to_string())
    }
    
    fn v4_simple() -> Result<String, String> {
        let uuid = Uuid::new_v4();
        Ok(uuid.simple().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_v4() {
        let result = UuidGenerateTool::v4();
        assert!(result.is_ok());
        let uuid = result.unwrap();
        assert_eq!(uuid.len(), 36); // Standard UUID length with hyphens
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }
    
    #[test]
    fn test_v4_hyphenated() {
        let result = UuidGenerateTool::v4_hyphenated();
        assert!(result.is_ok());
        let uuid = result.unwrap();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }
    
    #[test]
    fn test_v4_simple() {
        let result = UuidGenerateTool::v4_simple();
        assert!(result.is_ok());
        let uuid = result.unwrap();
        assert_eq!(uuid.len(), 32); // No hyphens
        assert!(!uuid.contains('-'));
    }
    
    #[test]
    fn test_uniqueness() {
        let uuid1 = UuidGenerateTool::v4().unwrap();
        let uuid2 = UuidGenerateTool::v4().unwrap();
        assert_ne!(uuid1, uuid2);
    }
}
