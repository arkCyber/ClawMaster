//! String length tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "string-length",
    exports: {
        world: StringLengthTool,
    },
});

struct StringLengthTool;

impl Guest for StringLengthTool {
    fn byte_length(input: String) -> u32 {
        input.len() as u32
    }
    
    fn char_count(input: String) -> u32 {
        input.chars().count() as u32
    }
    
    fn word_count(input: String) -> u32 {
        input.split_whitespace().count() as u32
    }
    
    fn line_count(input: String) -> u32 {
        if input.is_empty() {
            0
        } else {
            input.lines().count() as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_byte_length() {
        assert_eq!(StringLengthTool::byte_length("Hello".to_string()), 5);
        assert_eq!(StringLengthTool::byte_length("你好".to_string()), 6); // 3 bytes per char
    }
    
    #[test]
    fn test_char_count() {
        assert_eq!(StringLengthTool::char_count("Hello".to_string()), 5);
        assert_eq!(StringLengthTool::char_count("你好".to_string()), 2);
    }
    
    #[test]
    fn test_word_count() {
        assert_eq!(StringLengthTool::word_count("Hello World".to_string()), 2);
        assert_eq!(StringLengthTool::word_count("  a  b  c  ".to_string()), 3);
    }
    
    #[test]
    fn test_line_count() {
        assert_eq!(StringLengthTool::line_count("line1\nline2\nline3".to_string()), 3);
        assert_eq!(StringLengthTool::line_count("".to_string()), 0);
    }
}
