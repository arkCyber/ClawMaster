//! Regex operations tool for Moltis AI agents.

use regex::Regex;

wit_bindgen::generate!({
    world: "regex-ops",
    exports: {
        world: RegexOpsTool,
    },
});

const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct RegexOpsTool;

impl Guest for RegexOpsTool {
    fn test(pattern: String, input: String) -> Result<bool, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        Ok(re.is_match(&input))
    }
    
    fn find(pattern: String, input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        match re.find(&input) {
            Some(m) => Ok(m.as_str().to_string()),
            None => Ok(String::new()),
        }
    }
    
    fn find_all(pattern: String, input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        let matches: Vec<String> = re.find_iter(&input)
            .map(|m| m.as_str().to_string())
            .collect();
        
        serde_json::to_string(&matches)
            .map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn replace(pattern: String, input: String, replacement: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        Ok(re.replace(&input, replacement.as_str()).to_string())
    }
    
    fn replace_all(pattern: String, input: String, replacement: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        Ok(re.replace_all(&input, replacement.as_str()).to_string())
    }
    
    fn split(pattern: String, input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let re = Regex::new(&pattern)
            .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        
        let parts: Vec<String> = re.split(&input)
            .map(|s| s.to_string())
            .collect();
        
        serde_json::to_string(&parts)
            .map_err(|e| format!("Serialization error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_test() {
        assert!(RegexOpsTool::test(r"\d+".to_string(), "hello123".to_string()).unwrap());
        assert!(!RegexOpsTool::test(r"\d+".to_string(), "hello".to_string()).unwrap());
    }
    
    #[test]
    fn test_find() {
        let result = RegexOpsTool::find(r"\d+".to_string(), "hello123world".to_string());
        assert_eq!(result.unwrap(), "123");
    }
    
    #[test]
    fn test_find_all() {
        let result = RegexOpsTool::find_all(r"\d+".to_string(), "a1b22c333".to_string());
        assert_eq!(result.unwrap(), r#"["1","22","333"]"#);
    }
    
    #[test]
    fn test_replace() {
        let result = RegexOpsTool::replace(r"\d+".to_string(), "hello123world456".to_string(), "X".to_string());
        assert_eq!(result.unwrap(), "helloXworld456");
    }
    
    #[test]
    fn test_replace_all() {
        let result = RegexOpsTool::replace_all(r"\d+".to_string(), "hello123world456".to_string(), "X".to_string());
        assert_eq!(result.unwrap(), "helloXworldX");
    }
    
    #[test]
    fn test_split() {
        let result = RegexOpsTool::split(r"\s+".to_string(), "a  b   c".to_string());
        assert_eq!(result.unwrap(), r#"["a","b","c"]"#);
    }
}
