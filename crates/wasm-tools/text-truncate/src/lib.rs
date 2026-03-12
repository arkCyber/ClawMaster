//! Text truncation tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "text-truncate",
    exports: {
        world: TextTruncateTool,
    },
});

struct TextTruncateTool;

impl Guest for TextTruncateTool {
    fn truncate(text: String, max_length: u32) -> String {
        let max_len = max_length as usize;
        if text.len() <= max_len {
            text
        } else {
            text.chars().take(max_len).collect()
        }
    }
    
    fn truncate_ellipsis(text: String, max_length: u32) -> String {
        let max_len = max_length as usize;
        if text.len() <= max_len {
            text
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            let truncated: String = text.chars().take(max_len - 3).collect();
            format!("{}...", truncated)
        }
    }
    
    fn truncate_words(text: String, max_words: u32) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() <= max_words as usize {
            text
        } else {
            words.iter()
                .take(max_words as usize)
                .cloned()
                .collect::<Vec<&str>>()
                .join(" ")
        }
    }
    
    fn truncate_lines(text: String, max_lines: u32) -> String {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() <= max_lines as usize {
            text
        } else {
            lines.iter()
                .take(max_lines as usize)
                .cloned()
                .collect::<Vec<&str>>()
                .join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truncate() {
        assert_eq!(TextTruncateTool::truncate("Hello World".to_string(), 5), "Hello");
        assert_eq!(TextTruncateTool::truncate("Hi".to_string(), 5), "Hi");
    }
    
    #[test]
    fn test_truncate_ellipsis() {
        assert_eq!(TextTruncateTool::truncate_ellipsis("Hello World".to_string(), 8), "Hello...");
        assert_eq!(TextTruncateTool::truncate_ellipsis("Hi".to_string(), 5), "Hi");
    }
    
    #[test]
    fn test_truncate_words() {
        assert_eq!(TextTruncateTool::truncate_words("one two three four".to_string(), 2), "one two");
        assert_eq!(TextTruncateTool::truncate_words("one two".to_string(), 5), "one two");
    }
    
    #[test]
    fn test_truncate_lines() {
        assert_eq!(TextTruncateTool::truncate_lines("line1\nline2\nline3".to_string(), 2), "line1\nline2");
        assert_eq!(TextTruncateTool::truncate_lines("line1".to_string(), 5), "line1");
    }
}
