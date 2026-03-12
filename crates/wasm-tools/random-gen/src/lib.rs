//! Random generation tool for Moltis AI agents.

use getrandom::getrandom;

wit_bindgen::generate!({
    world: "random-gen",
    exports: {
        world: RandomGenTool,
    },
});

const MAX_LENGTH: u32 = 10000;

struct RandomGenTool;

impl Guest for RandomGenTool {
    fn random_int(min: i32, max: i32) -> Result<i32, String> {
        if min >= max {
            return Err("min must be less than max".to_string());
        }
        
        let mut bytes = [0u8; 4];
        getrandom(&mut bytes).map_err(|e| format!("Random generation failed: {}", e))?;
        
        let random_u32 = u32::from_le_bytes(bytes);
        let range = (max - min) as u32;
        let result = min + (random_u32 % range) as i32;
        
        Ok(result)
    }
    
    fn random_float() -> f64 {
        let mut bytes = [0u8; 8];
        if getrandom(&mut bytes).is_err() {
            return 0.0;
        }
        
        let random_u64 = u64::from_le_bytes(bytes);
        (random_u64 as f64) / (u64::MAX as f64)
    }
    
    fn random_bytes(length: u32) -> Result<Vec<u8>, String> {
        if length > MAX_LENGTH {
            return Err(format!("Length exceeds maximum of {}", MAX_LENGTH));
        }
        
        let mut bytes = vec![0u8; length as usize];
        getrandom(&mut bytes).map_err(|e| format!("Random generation failed: {}", e))?;
        
        Ok(bytes)
    }
    
    fn random_string(length: u32) -> Result<String, String> {
        if length > MAX_LENGTH {
            return Err(format!("Length exceeds maximum of {}", MAX_LENGTH));
        }
        
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        
        let mut bytes = vec![0u8; length as usize];
        getrandom(&mut bytes).map_err(|e| format!("Random generation failed: {}", e))?;
        
        let result: String = bytes.iter()
            .map(|&b| CHARSET[(b as usize) % CHARSET.len()] as char)
            .collect();
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_int() {
        let result = RandomGenTool::random_int(1, 10);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert!(val >= 1 && val < 10);
    }
    
    #[test]
    fn test_random_int_invalid_range() {
        let result = RandomGenTool::random_int(10, 1);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_random_float() {
        let val = RandomGenTool::random_float();
        assert!(val >= 0.0 && val < 1.0);
    }
    
    #[test]
    fn test_random_bytes() {
        let result = RandomGenTool::random_bytes(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 10);
    }
    
    #[test]
    fn test_random_bytes_too_long() {
        let result = RandomGenTool::random_bytes(MAX_LENGTH + 1);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_random_string() {
        let result = RandomGenTool::random_string(10);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_alphanumeric()));
    }
}
