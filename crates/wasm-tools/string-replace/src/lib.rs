//! String replacement tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "string-replace",
    exports: {
        world: StringReplaceTool,
    },
});

struct StringReplaceTool;

impl Guest for StringReplaceTool {
    fn replace_all(input: String, from: String, to: String) -> String {
        input.replace(&from, &to)
    }
    
    fn replace_first(input: String, from: String, to: String) -> String {
        if let Some(pos) = input.find(&from) {
            let mut result = input.clone();
            result.replace_range(pos..pos + from.len(), &to);
            result
        } else {
            input
        }
    }
    
    fn replace_last(input: String, from: String, to: String) -> String {
        if let Some(pos) = input.rfind(&from) {
            let mut result = input.clone();
            result.replace_range(pos..pos + from.len(), &to);
            result
        } else {
            input
        }
    }
    
    fn replace_n(input: String, from: String, to: String, count: u32) -> String {
        let mut result = input.clone();
        let mut replaced = 0u32;
        let mut start = 0;
        
        while replaced < count {
            if let Some(pos) = result[start..].find(&from) {
                let absolute_pos = start + pos;
                result.replace_range(absolute_pos..absolute_pos + from.len(), &to);
                start = absolute_pos + to.len();
                replaced += 1;
            } else {
                break;
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_replace_all() {
        assert_eq!(
            StringReplaceTool::replace_all("hello world hello".to_string(), "hello".to_string(), "hi".to_string()),
            "hi world hi"
        );
    }
    
    #[test]
    fn test_replace_first() {
        assert_eq!(
            StringReplaceTool::replace_first("hello world hello".to_string(), "hello".to_string(), "hi".to_string()),
            "hi world hello"
        );
    }
    
    #[test]
    fn test_replace_last() {
        assert_eq!(
            StringReplaceTool::replace_last("hello world hello".to_string(), "hello".to_string(), "hi".to_string()),
            "hello world hi"
        );
    }
    
    #[test]
    fn test_replace_n() {
        assert_eq!(
            StringReplaceTool::replace_n("a a a a".to_string(), "a".to_string(), "b".to_string(), 2),
            "b b a a"
        );
    }
}
