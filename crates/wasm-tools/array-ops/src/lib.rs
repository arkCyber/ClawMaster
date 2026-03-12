//! Array operations tool for Moltis AI agents.

use serde_json::Value;

wit_bindgen::generate!({
    world: "array-ops",
    exports: {
        world: ArrayOpsTool,
    },
});

const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct ArrayOpsTool;

impl Guest for ArrayOpsTool {
    fn sort(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let mut arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        arr.sort_by(|a, b| {
            match (a, b) {
                (Value::Number(n1), Value::Number(n2)) => {
                    n1.as_f64().partial_cmp(&n2.as_f64()).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        serde_json::to_string(&arr).map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn reverse(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let mut arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        arr.reverse();
        
        serde_json::to_string(&arr).map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn unique(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        let mut unique_arr = Vec::new();
        for item in arr {
            if !unique_arr.contains(&item) {
                unique_arr.push(item);
            }
        }
        
        serde_json::to_string(&unique_arr).map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn flatten(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        let mut flat = Vec::new();
        for item in arr {
            if let Value::Array(nested) = item {
                flat.extend(nested);
            } else {
                flat.push(item);
            }
        }
        
        serde_json::to_string(&flat).map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn length(input: String) -> Result<u32, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        Ok(arr.len() as u32)
    }
    
    fn includes(input: String, value: String) -> Result<bool, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        let search_value: Value = serde_json::from_str(&value)
            .map_err(|e| format!("Invalid search value: {}", e))?;
        
        Ok(arr.contains(&search_value))
    }
    
    fn take(input: String, n: u32) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        let result: Vec<Value> = arr.into_iter().take(n as usize).collect();
        
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
    
    fn skip(input: String, n: u32) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err("Input size exceeds limit".to_string());
        }
        
        let arr: Vec<Value> = serde_json::from_str(&input)
            .map_err(|e| format!("Invalid JSON array: {}", e))?;
        
        let result: Vec<Value> = arr.into_iter().skip(n as usize).collect();
        
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sort() {
        let result = ArrayOpsTool::sort("[3,1,2]".to_string());
        assert_eq!(result.unwrap(), "[1,2,3]");
    }
    
    #[test]
    fn test_reverse() {
        let result = ArrayOpsTool::reverse("[1,2,3]".to_string());
        assert_eq!(result.unwrap(), "[3,2,1]");
    }
    
    #[test]
    fn test_unique() {
        let result = ArrayOpsTool::unique("[1,2,2,3,1]".to_string());
        assert_eq!(result.unwrap(), "[1,2,3]");
    }
    
    #[test]
    fn test_flatten() {
        let result = ArrayOpsTool::flatten("[[1,2],[3,4]]".to_string());
        assert_eq!(result.unwrap(), "[1,2,3,4]");
    }
    
    #[test]
    fn test_length() {
        let result = ArrayOpsTool::length("[1,2,3]".to_string());
        assert_eq!(result.unwrap(), 3);
    }
    
    #[test]
    fn test_includes() {
        let result = ArrayOpsTool::includes("[1,2,3]".to_string(), "2".to_string());
        assert_eq!(result.unwrap(), true);
        
        let result = ArrayOpsTool::includes("[1,2,3]".to_string(), "4".to_string());
        assert_eq!(result.unwrap(), false);
    }
    
    #[test]
    fn test_take() {
        let result = ArrayOpsTool::take("[1,2,3,4,5]".to_string(), 3);
        assert_eq!(result.unwrap(), "[1,2,3]");
    }
    
    #[test]
    fn test_skip() {
        let result = ArrayOpsTool::skip("[1,2,3,4,5]".to_string(), 2);
        assert_eq!(result.unwrap(), "[3,4,5]");
    }
}
