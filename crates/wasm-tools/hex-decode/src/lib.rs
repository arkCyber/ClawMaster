//! Hex decoding tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "hex-decode",
    exports: {
        world: HexDecodeTool,
    },
});

const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct HexDecodeTool;

impl Guest for HexDecodeTool {
    fn decode(input: String) -> Result<Vec<u8>, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!("Input size exceeds limit"));
        }
        hex::decode(&input).map_err(|e| format!("Invalid hex: {}", e))
    }
    
    fn decode_string(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!("Input size exceeds limit"));
        }
        let bytes = hex::decode(&input).map_err(|e| format!("Invalid hex: {}", e))?;
        String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))
    }
    
    fn is_valid(input: String) -> bool {
        if input.len() > MAX_INPUT_SIZE {
            return false;
        }
        hex::decode(&input).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode() {
        let result = HexDecodeTool::decode("48656c6c6f".to_string());
        assert_eq!(result.unwrap(), vec![72, 101, 108, 108, 111]);
    }
    
    #[test]
    fn test_decode_string() {
        let result = HexDecodeTool::decode_string("48656c6c6f".to_string());
        assert_eq!(result.unwrap(), "Hello");
    }
    
    #[test]
    fn test_is_valid() {
        assert!(HexDecodeTool::is_valid("48656c6c6f".to_string()));
        assert!(!HexDecodeTool::is_valid("invalid".to_string()));
    }
}
