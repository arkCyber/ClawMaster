//! Text splitting tool for Moltis AI agents.
//!
//! This tool provides text splitting with:
//! - Delimiter-based splitting
//! - Line splitting
//! - Whitespace splitting
//! - Chunk splitting
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Same input = same output
//! - §11.10: Resource management - Output count limits
//! - §11.13: Initialization - No global state

wit_bindgen::generate!({
    world: "text-split",
    exports: {
        world: TextSplitTool,
    },
});

/// Maximum input size: 10MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_INPUT_SIZE: usize = 10 * 1024 * 1024;

/// Maximum output parts: 10000
/// DO-178C §11.10: Prevent unbounded output
const MAX_OUTPUT_PARTS: usize = 10000;

struct TextSplitTool;

impl Guest for TextSplitTool {
    /// Split text by delimiter.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic splitting
    fn split(text: String, delimiter: String) -> Result<Vec<String>, String> {
        // DO-178C §11.10: Check size limit
        if text.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Text size ({} bytes) exceeds limit ({} bytes)",
                text.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        if delimiter.is_empty() {
            return Err("Delimiter cannot be empty".to_string());
        }
        
        // DO-178C §6.3.4: Deterministic split
        let parts: Vec<String> = text
            .split(&delimiter)
            .map(|s| s.to_string())
            .collect();
        
        // DO-178C §11.10: Check output limit
        if parts.len() > MAX_OUTPUT_PARTS {
            return Err(format!(
                "Output parts ({}) exceeds limit ({})",
                parts.len(),
                MAX_OUTPUT_PARTS
            ));
        }
        
        Ok(parts)
    }
    
    /// Split text by lines.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn split_lines(text: String) -> Result<Vec<String>, String> {
        if text.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Text size ({} bytes) exceeds limit ({} bytes)",
                text.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let lines: Vec<String> = text
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        if lines.len() > MAX_OUTPUT_PARTS {
            return Err(format!(
                "Output lines ({}) exceeds limit ({})",
                lines.len(),
                MAX_OUTPUT_PARTS
            ));
        }
        
        Ok(lines)
    }
    
    /// Split text by whitespace.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn split_whitespace(text: String) -> Result<Vec<String>, String> {
        if text.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Text size ({} bytes) exceeds limit ({} bytes)",
                text.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        let words: Vec<String> = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if words.len() > MAX_OUTPUT_PARTS {
            return Err(format!(
                "Output words ({}) exceeds limit ({})",
                words.len(),
                MAX_OUTPUT_PARTS
            ));
        }
        
        Ok(words)
    }
    
    /// Split text into chunks.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn split_chunks(text: String, chunk_size: u32) -> Result<Vec<String>, String> {
        if text.len() > MAX_INPUT_SIZE {
            return Err(format!(
                "Text size ({} bytes) exceeds limit ({} bytes)",
                text.len(),
                MAX_INPUT_SIZE
            ));
        }
        
        if chunk_size == 0 {
            return Err("Chunk size must be greater than 0".to_string());
        }
        
        let chunk_size = chunk_size as usize;
        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        
        for chunk in chars.chunks(chunk_size) {
            chunks.push(chunk.iter().collect());
        }
        
        if chunks.len() > MAX_OUTPUT_PARTS {
            return Err(format!(
                "Output chunks ({}) exceeds limit ({})",
                chunks.len(),
                MAX_OUTPUT_PARTS
            ));
        }
        
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_simple() {
        let result = TextSplitTool::split("a,b,c".to_string(), ",".to_string());
        assert!(result.is_ok());
        let parts = result.unwrap();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
    
    #[test]
    fn test_split_empty_delimiter() {
        let result = TextSplitTool::split("abc".to_string(), String::new());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }
    
    #[test]
    fn test_split_lines() {
        let text = "line1\nline2\nline3".to_string();
        let result = TextSplitTool::split_lines(text);
        assert!(result.is_ok());
        let lines = result.unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }
    
    #[test]
    fn test_split_whitespace() {
        let text = "hello   world  test".to_string();
        let result = TextSplitTool::split_whitespace(text);
        assert!(result.is_ok());
        let words = result.unwrap();
        assert_eq!(words, vec!["hello", "world", "test"]);
    }
    
    #[test]
    fn test_split_chunks() {
        let text = "abcdefgh".to_string();
        let result = TextSplitTool::split_chunks(text, 3);
        assert!(result.is_ok());
        let chunks = result.unwrap();
        assert_eq!(chunks, vec!["abc", "def", "gh"]);
    }
    
    #[test]
    fn test_split_chunks_zero_size() {
        let result = TextSplitTool::split_chunks("abc".to_string(), 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("greater than 0"));
    }
    
    #[test]
    fn test_split_size_limit() {
        let large_text = "x".repeat(MAX_INPUT_SIZE + 1);
        let result = TextSplitTool::split(large_text, ",".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
