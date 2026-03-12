//! Data validation tool for Moltis AI agents.

use regex::Regex;

wit_bindgen::generate!({
    world: "validate-data",
    exports: {
        world: ValidateDataTool,
    },
});

struct ValidateDataTool;

impl Guest for ValidateDataTool {
    fn is_email(input: String) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(&input)
    }
    
    fn is_url(input: String) -> bool {
        let re = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        re.is_match(&input)
    }
    
    fn is_json(input: String) -> bool {
        serde_json::from_str::<serde_json::Value>(&input).is_ok()
    }
    
    fn is_uuid(input: String) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
        re.is_match(&input)
    }
    
    fn is_ipv4(input: String) -> bool {
        let re = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        re.is_match(&input)
    }
    
    fn is_number(input: String) -> bool {
        input.parse::<f64>().is_ok()
    }
    
    fn is_integer(input: String) -> bool {
        input.parse::<i64>().is_ok()
    }
    
    fn is_hex(input: String) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        !input.is_empty() && re.is_match(&input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_email() {
        assert!(ValidateDataTool::is_email("test@example.com".to_string()));
        assert!(ValidateDataTool::is_email("user.name+tag@example.co.uk".to_string()));
        assert!(!ValidateDataTool::is_email("invalid".to_string()));
        assert!(!ValidateDataTool::is_email("@example.com".to_string()));
    }
    
    #[test]
    fn test_is_url() {
        assert!(ValidateDataTool::is_url("https://example.com".to_string()));
        assert!(ValidateDataTool::is_url("http://example.com/path".to_string()));
        assert!(!ValidateDataTool::is_url("not a url".to_string()));
        assert!(!ValidateDataTool::is_url("ftp://example.com".to_string()));
    }
    
    #[test]
    fn test_is_json() {
        assert!(ValidateDataTool::is_json(r#"{"key":"value"}"#.to_string()));
        assert!(ValidateDataTool::is_json(r#"[1,2,3]"#.to_string()));
        assert!(ValidateDataTool::is_json(r#"null"#.to_string()));
        assert!(!ValidateDataTool::is_json("not json".to_string()));
    }
    
    #[test]
    fn test_is_uuid() {
        assert!(ValidateDataTool::is_uuid("550e8400-e29b-41d4-a716-446655440000".to_string()));
        assert!(!ValidateDataTool::is_uuid("not-a-uuid".to_string()));
        assert!(!ValidateDataTool::is_uuid("550e8400e29b41d4a716446655440000".to_string()));
    }
    
    #[test]
    fn test_is_ipv4() {
        assert!(ValidateDataTool::is_ipv4("192.168.1.1".to_string()));
        assert!(ValidateDataTool::is_ipv4("0.0.0.0".to_string()));
        assert!(ValidateDataTool::is_ipv4("255.255.255.255".to_string()));
        assert!(!ValidateDataTool::is_ipv4("256.1.1.1".to_string()));
        assert!(!ValidateDataTool::is_ipv4("192.168.1".to_string()));
    }
    
    #[test]
    fn test_is_number() {
        assert!(ValidateDataTool::is_number("123".to_string()));
        assert!(ValidateDataTool::is_number("123.45".to_string()));
        assert!(ValidateDataTool::is_number("-123.45".to_string()));
        assert!(!ValidateDataTool::is_number("abc".to_string()));
    }
    
    #[test]
    fn test_is_integer() {
        assert!(ValidateDataTool::is_integer("123".to_string()));
        assert!(ValidateDataTool::is_integer("-123".to_string()));
        assert!(!ValidateDataTool::is_integer("123.45".to_string()));
        assert!(!ValidateDataTool::is_integer("abc".to_string()));
    }
    
    #[test]
    fn test_is_hex() {
        assert!(ValidateDataTool::is_hex("48656c6c6f".to_string()));
        assert!(ValidateDataTool::is_hex("ABCDEF".to_string()));
        assert!(!ValidateDataTool::is_hex("xyz".to_string()));
        assert!(!ValidateDataTool::is_hex("".to_string()));
    }
}
