//! Hex encoding tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "hex-encode",
    exports: {
        world: HexEncodeTool,
    },
});

const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

struct HexEncodeTool;

impl Guest for HexEncodeTool {
    fn encode(input: Vec<u8>) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!("Input size exceeds limit"));
        }
        Ok(hex::encode(input))
    }
    
    fn encode_string(input: String) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!("Input size exceeds limit"));
        }
        Ok(hex::encode(input.as_bytes()))
    }
    
    fn encode_upper(input: Vec<u8>) -> Result<String, String> {
        if input.len() > MAX_INPUT_SIZE {
            return Err(format!("Input size exceeds limit"));
        }
        Ok(hex::encode_upper(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode() {
        let result = HexEncodeTool::encode(vec![72, 101, 108, 108, 111]);
        assert_eq!(result.unwrap(), "48656c6c6f");
    }
    
    #[test]
    fn test_encode_string() {
        let result = HexEncodeTool::encode_string("Hello".to_string());
        assert_eq!(result.unwrap(), "48656c6c6f");
    }
    
    #[test]
    fn test_encode_upper() {
        let result = HexEncodeTool::encode_upper(vec![72, 101, 108, 108, 111]);
        assert_eq!(result.unwrap(), "48656C6C6F");
    }
}
