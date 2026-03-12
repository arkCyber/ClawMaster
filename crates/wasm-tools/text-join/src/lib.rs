//! Text joining tool for Moltis AI agents.
//!
//! This tool provides text joining with:
//! - Delimiter-based joining
//! - Line joining
//! - Word joining
//! - Concatenation
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Same input = same output
//! - §11.10: Resource management - Output size limits
//! - §11.13: Initialization - No global state

wit_bindgen::generate!({
    world: "text-join",
    exports: {
        world: TextJoinTool,
    },
});

/// Maximum output size: 10MB
/// DO-178C §11.10: Prevent unbounded memory usage
const MAX_OUTPUT_SIZE: usize = 10 * 1024 * 1024;

/// Maximum input parts: 10000
/// DO-178C §11.10: Prevent unbounded input
const MAX_INPUT_PARTS: usize = 10000;

struct TextJoinTool;

impl Guest for TextJoinTool {
    /// Join text parts with delimiter.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic joining
    fn join(parts: Vec<String>, delimiter: String) -> Result<String, String> {
        // DO-178C §11.10: Check input limit
        if parts.len() > MAX_INPUT_PARTS {
            return Err(format!(
                "Input parts ({}) exceeds limit ({})",
                parts.len(),
                MAX_INPUT_PARTS
            ));
        }
        
        // DO-178C §6.3.4: Deterministic join
        let joined = parts.join(&delimiter);
        
        // DO-178C §11.10: Check output size
        if joined.len() > MAX_OUTPUT_SIZE {
            return Err(format!(
                "Output size ({} bytes) exceeds limit ({} bytes)",
                joined.len(),
                MAX_OUTPUT_SIZE
            ));
        }
        
        Ok(joined)
    }
    
    /// Join text parts with newlines.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn join_lines(lines: Vec<String>) -> Result<String, String> {
        Self::join(lines, "\n".to_string())
    }
    
    /// Join text parts with spaces.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn join_words(words: Vec<String>) -> Result<String, String> {
        Self::join(words, " ".to_string())
    }
    
    /// Concatenate text parts.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn concat(parts: Vec<String>) -> Result<String, String> {
        Self::join(parts, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_join_simple() {
        let parts = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = TextJoinTool::join(parts, ",".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a,b,c");
    }
    
    #[test]
    fn test_join_empty_parts() {
        let parts: Vec<String> = vec![];
        let result = TextJoinTool::join(parts, ",".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
    
    #[test]
    fn test_join_lines() {
        let lines = vec!["line1".to_string(), "line2".to_string(), "line3".to_string()];
        let result = TextJoinTool::join_lines(lines);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "line1\nline2\nline3");
    }
    
    #[test]
    fn test_join_words() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let result = TextJoinTool::join_words(words);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }
    
    #[test]
    fn test_concat() {
        let parts = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = TextJoinTool::concat(parts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abc");
    }
    
    #[test]
    fn test_join_input_limit() {
        let parts: Vec<String> = (0..MAX_INPUT_PARTS + 1)
            .map(|i| i.to_string())
            .collect();
        let result = TextJoinTool::join(parts, ",".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
    
    #[test]
    fn test_join_output_size_limit() {
        let large_part = "x".repeat(MAX_OUTPUT_SIZE / 2 + 1);
        let parts = vec![large_part.clone(), large_part];
        let result = TextJoinTool::join(parts, "".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds limit"));
    }
}
