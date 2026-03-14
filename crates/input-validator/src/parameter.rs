//! API Parameter Validation
//!
//! DO-178C Level A Compliant Parameter Validation

use crate::{ValidationError, ValidationResult};

/// Maximum parameter value length
const MAX_PARAM_LENGTH: usize = 10_000;

/// Validate string parameter
///
/// DO-178C §6.3.1: Parameter validation
pub fn validate_string_param(value: &str, max_length: usize) -> ValidationResult<String> {
    // Check length
    if value.len() > max_length {
        return Err(ValidationError::TooLong {
            actual: value.len(),
            max: max_length,
        });
    }

    // Check for null bytes
    if value.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    Ok(value.to_string())
}

/// Validate integer parameter
///
/// DO-178C §6.3.1: Integer validation
pub fn validate_int_param(value: &str, min: i64, max: i64) -> ValidationResult<i64> {
    let parsed = value.parse::<i64>().map_err(|_| {
        ValidationError::InvalidFormat(format!("Invalid integer: {}", value))
    })?;

    if parsed < min || parsed > max {
        return Err(ValidationError::Invalid(
            format!("Value {} out of range [{}, {}]", parsed, min, max)
        ));
    }

    Ok(parsed)
}

/// Validate boolean parameter
///
/// DO-178C §6.3.1: Boolean validation
pub fn validate_bool_param(value: &str) -> ValidationResult<bool> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(ValidationError::InvalidFormat(
            format!("Invalid boolean: {}", value)
        )),
    }
}

/// Validate enum parameter
///
/// DO-178C §6.3.1: Enum validation
pub fn validate_enum_param(value: &str, allowed_values: &[&str]) -> ValidationResult<String> {
    if allowed_values.contains(&value) {
        Ok(value.to_string())
    } else {
        Err(ValidationError::Invalid(
            format!("Invalid value: {}. Allowed: {:?}", value, allowed_values)
        ))
    }
}

/// Validate email parameter
///
/// DO-178C §6.3.1: Email validation
pub fn validate_email_param(value: &str) -> ValidationResult<String> {
    // Basic email validation
    if !value.contains('@') {
        return Err(ValidationError::InvalidFormat(
            "Email must contain @".to_string()
        ));
    }

    let parts: Vec<&str> = value.split('@').collect();
    if parts.len() != 2 {
        return Err(ValidationError::InvalidFormat(
            "Email must have exactly one @".to_string()
        ));
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(ValidationError::InvalidFormat(
            "Email parts cannot be empty".to_string()
        ));
    }

    if !parts[1].contains('.') {
        return Err(ValidationError::InvalidFormat(
            "Email domain must contain a dot".to_string()
        ));
    }

    validate_string_param(value, 254) // RFC 5321 max email length
}

/// Validate URL parameter
///
/// DO-178C §6.3.1: URL validation
pub fn validate_url_param(value: &str) -> ValidationResult<String> {
    // Basic URL validation
    if !value.starts_with("http://") && !value.starts_with("https://") {
        return Err(ValidationError::InvalidFormat(
            "URL must start with http:// or https://".to_string()
        ));
    }

    validate_string_param(value, 2048) // Common max URL length
}

/// Validate UUID parameter
///
/// DO-178C §6.3.1: UUID validation
pub fn validate_uuid_param(value: &str) -> ValidationResult<String> {
    // Basic UUID format: 8-4-4-4-12 hex digits
    let parts: Vec<&str> = value.split('-').collect();
    
    if parts.len() != 5 {
        return Err(ValidationError::InvalidFormat(
            "UUID must have 5 parts separated by hyphens".to_string()
        ));
    }

    if parts[0].len() != 8 || parts[1].len() != 4 || parts[2].len() != 4 
        || parts[3].len() != 4 || parts[4].len() != 12 {
        return Err(ValidationError::InvalidFormat(
            "UUID parts have incorrect lengths".to_string()
        ));
    }

    // Check all characters are hex
    for part in &parts {
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ValidationError::InvalidFormat(
                "UUID must contain only hex digits".to_string()
            ));
        }
    }

    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_string_param_valid() {
        assert!(validate_string_param("hello", 100).is_ok());
    }

    #[test]
    fn test_validate_string_param_too_long() {
        let result = validate_string_param("hello", 3);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TooLong { .. }));
    }

    #[test]
    fn test_validate_string_param_null_byte() {
        let result = validate_string_param("hel\0lo", 100);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::NullByte));
    }

    #[test]
    fn test_validate_int_param_valid() {
        assert_eq!(validate_int_param("42", 0, 100).unwrap(), 42);
    }

    #[test]
    fn test_validate_int_param_invalid_format() {
        let result = validate_int_param("abc", 0, 100);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }

    #[test]
    fn test_validate_int_param_out_of_range() {
        let result = validate_int_param("150", 0, 100);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::Invalid(_)));
    }

    #[test]
    fn test_validate_bool_param_valid() {
        assert_eq!(validate_bool_param("true").unwrap(), true);
        assert_eq!(validate_bool_param("false").unwrap(), false);
        assert_eq!(validate_bool_param("1").unwrap(), true);
        assert_eq!(validate_bool_param("0").unwrap(), false);
    }

    #[test]
    fn test_validate_bool_param_invalid() {
        let result = validate_bool_param("maybe");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }

    #[test]
    fn test_validate_enum_param_valid() {
        let allowed = &["red", "green", "blue"];
        assert!(validate_enum_param("red", allowed).is_ok());
    }

    #[test]
    fn test_validate_enum_param_invalid() {
        let allowed = &["red", "green", "blue"];
        let result = validate_enum_param("yellow", allowed);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::Invalid(_)));
    }

    #[test]
    fn test_validate_email_param_valid() {
        assert!(validate_email_param("user@example.com").is_ok());
    }

    #[test]
    fn test_validate_email_param_no_at() {
        let result = validate_email_param("userexample.com");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }

    #[test]
    fn test_validate_email_param_no_domain() {
        let result = validate_email_param("user@");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }

    #[test]
    fn test_validate_url_param_valid() {
        assert!(validate_url_param("https://example.com").is_ok());
        assert!(validate_url_param("http://example.com").is_ok());
    }

    #[test]
    fn test_validate_url_param_invalid_scheme() {
        let result = validate_url_param("ftp://example.com");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }

    #[test]
    fn test_validate_uuid_param_valid() {
        assert!(validate_uuid_param("550e8400-e29b-41d4-a716-446655440000").is_ok());
    }

    #[test]
    fn test_validate_uuid_param_invalid_format() {
        let result = validate_uuid_param("not-a-uuid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::InvalidFormat(_)));
    }
}
