//! User Message Validation
//!
//! DO-178C Level A Compliant Message Validation

use crate::{ValidationError, ValidationResult};
use once_cell::sync::Lazy;
use regex::Regex;

/// Maximum message length (1MB)
const MAX_MESSAGE_LENGTH: usize = 1_048_576;

/// XSS patterns to detect
static XSS_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)<script[^>]*>").unwrap(),
        Regex::new(r"(?i)</script>").unwrap(),
        Regex::new(r"(?i)javascript:").unwrap(),
        Regex::new(r"(?i)on\w+\s*=").unwrap(), // onclick, onerror, etc.
        Regex::new(r"(?i)<iframe[^>]*>").unwrap(),
        Regex::new(r"(?i)<object[^>]*>").unwrap(),
        Regex::new(r"(?i)<embed[^>]*>").unwrap(),
    ]
});

/// SQL injection patterns to detect
static SQL_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)(union\s+select)").unwrap(),
        Regex::new(r"(?i)(drop\s+table)").unwrap(),
        Regex::new(r"(?i)(delete\s+from)").unwrap(),
        Regex::new(r"(?i)(insert\s+into)").unwrap(),
        Regex::new(r"(?i)(update\s+\w+\s+set)").unwrap(),
        Regex::new(r"(?i)(exec\s*\()").unwrap(),
        Regex::new(r"(?i)(execute\s*\()").unwrap(),
        Regex::new(r"--\s*$").unwrap(), // SQL comment
        Regex::new(r";.*--").unwrap(),  // SQL injection with comment
    ]
});

/// Validate user message
///
/// DO-178C §6.3.1: Input validation
pub fn validate_message(message: &str) -> ValidationResult<()> {
    // Check length
    if message.len() > MAX_MESSAGE_LENGTH {
        return Err(ValidationError::TooLong {
            actual: message.len(),
            max: MAX_MESSAGE_LENGTH,
        });
    }

    // Check for null bytes
    if message.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for XSS attempts
    for pattern in XSS_PATTERNS.iter() {
        if pattern.is_match(message) {
            return Err(ValidationError::XssAttempt);
        }
    }

    // Check for SQL injection attempts
    for pattern in SQL_PATTERNS.iter() {
        if pattern.is_match(message) {
            return Err(ValidationError::SqlInjection);
        }
    }

    Ok(())
}

/// Sanitize user message (remove dangerous content)
///
/// DO-178C §6.3.1: Input sanitization
pub fn sanitize_message(message: &str) -> String {
    let mut sanitized = message.to_string();

    // Remove null bytes
    sanitized = sanitized.replace('\0', "");

    // Truncate if too long
    if sanitized.len() > MAX_MESSAGE_LENGTH {
        sanitized.truncate(MAX_MESSAGE_LENGTH);
    }

    sanitized
}

/// Check if message contains potentially dangerous patterns
///
/// DO-178C §6.3.1: Threat detection
pub fn is_dangerous_message(message: &str) -> bool {
    // Check for XSS
    for pattern in XSS_PATTERNS.iter() {
        if pattern.is_match(message) {
            return true;
        }
    }

    // Check for SQL injection
    for pattern in SQL_PATTERNS.iter() {
        if pattern.is_match(message) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_message_valid() {
        assert!(validate_message("Hello, world!").is_ok());
        assert!(validate_message("This is a normal message").is_ok());
        assert!(validate_message("Message with numbers 123").is_ok());
    }

    #[test]
    fn test_validate_message_too_long() {
        let long_message = "a".repeat(MAX_MESSAGE_LENGTH + 1);
        let result = validate_message(&long_message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TooLong { .. }));
    }

    #[test]
    fn test_validate_message_null_byte() {
        let message = "Hello\0World";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::NullByte));
    }

    #[test]
    fn test_validate_message_xss_script() {
        let message = "<script>alert('XSS')</script>";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::XssAttempt));
    }

    #[test]
    fn test_validate_message_xss_javascript() {
        let message = "javascript:alert('XSS')";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::XssAttempt));
    }

    #[test]
    fn test_validate_message_xss_onerror() {
        let message = "<img src=x onerror=alert('XSS')>";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::XssAttempt));
    }

    #[test]
    fn test_validate_message_sql_union() {
        let message = "1' UNION SELECT * FROM users--";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::SqlInjection));
    }

    #[test]
    fn test_validate_message_sql_drop() {
        let message = "'; DROP TABLE users;--";
        let result = validate_message(message);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::SqlInjection));
    }

    #[test]
    fn test_sanitize_message_null_bytes() {
        let message = "Hello\0World";
        let sanitized = sanitize_message(message);
        assert_eq!(sanitized, "HelloWorld");
    }

    #[test]
    fn test_sanitize_message_too_long() {
        let long_message = "a".repeat(MAX_MESSAGE_LENGTH + 100);
        let sanitized = sanitize_message(&long_message);
        assert_eq!(sanitized.len(), MAX_MESSAGE_LENGTH);
    }

    #[test]
    fn test_is_dangerous_message() {
        assert!(!is_dangerous_message("Hello, world!"));
        assert!(is_dangerous_message("<script>alert('XSS')</script>"));
        assert!(is_dangerous_message("1' UNION SELECT * FROM users--"));
    }
}
