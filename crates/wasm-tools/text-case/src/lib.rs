//! Text case conversion tool for Moltis AI agents.

use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};

wit_bindgen::generate!({
    world: "text-case",
    exports: {
        world: TextCaseTool,
    },
});

struct TextCaseTool;

impl Guest for TextCaseTool {
    fn to_upper(input: String) -> String {
        input.to_uppercase()
    }
    
    fn to_lower(input: String) -> String {
        input.to_lowercase()
    }
    
    fn to_title(input: String) -> String {
        input.to_title_case()
    }
    
    fn to_camel(input: String) -> String {
        input.to_lower_camel_case()
    }
    
    fn to_snake(input: String) -> String {
        input.to_snake_case()
    }
    
    fn to_kebab(input: String) -> String {
        input.to_kebab_case()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_upper() {
        assert_eq!(TextCaseTool::to_upper("hello world".to_string()), "HELLO WORLD");
    }
    
    #[test]
    fn test_to_lower() {
        assert_eq!(TextCaseTool::to_lower("HELLO WORLD".to_string()), "hello world");
    }
    
    #[test]
    fn test_to_title() {
        assert_eq!(TextCaseTool::to_title("hello world".to_string()), "Hello World");
    }
    
    #[test]
    fn test_to_camel() {
        assert_eq!(TextCaseTool::to_camel("hello world".to_string()), "helloWorld");
    }
    
    #[test]
    fn test_to_snake() {
        assert_eq!(TextCaseTool::to_snake("hello world".to_string()), "hello_world");
    }
    
    #[test]
    fn test_to_kebab() {
        assert_eq!(TextCaseTool::to_kebab("hello world".to_string()), "hello-world");
    }
}
