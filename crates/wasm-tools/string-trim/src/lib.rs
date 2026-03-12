//! String trimming tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "string-trim",
    exports: {
        world: StringTrimTool,
    },
});

struct StringTrimTool;

impl Guest for StringTrimTool {
    fn trim(input: String) -> String {
        input.trim().to_string()
    }
    
    fn trim_start(input: String) -> String {
        input.trim_start().to_string()
    }
    
    fn trim_end(input: String) -> String {
        input.trim_end().to_string()
    }
    
    fn trim_chars(input: String, chars: String) -> String {
        let char_set: Vec<char> = chars.chars().collect();
        input.trim_matches(|c| char_set.contains(&c)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trim() {
        assert_eq!(StringTrimTool::trim("  hello  ".to_string()), "hello");
        assert_eq!(StringTrimTool::trim("\n\thello\t\n".to_string()), "hello");
    }
    
    #[test]
    fn test_trim_start() {
        assert_eq!(StringTrimTool::trim_start("  hello  ".to_string()), "hello  ");
    }
    
    #[test]
    fn test_trim_end() {
        assert_eq!(StringTrimTool::trim_end("  hello  ".to_string()), "  hello");
    }
    
    #[test]
    fn test_trim_chars() {
        assert_eq!(StringTrimTool::trim_chars("***hello***".to_string(), "*".to_string()), "hello");
        assert_eq!(StringTrimTool::trim_chars("abchelloabc".to_string(), "abc".to_string()), "hello");
    }
}
