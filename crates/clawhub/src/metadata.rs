//! Tool metadata validation and parsing.

use crate::error::{Error, Result};

/// Validate tool name.
///
/// Tool names must:
/// - Be 3-64 characters long
/// - Contain only lowercase letters, numbers, and hyphens
/// - Start with a letter
/// - Not end with a hyphen
///
/// # Compliance
/// DO-178C §6.3.4: Deterministic behavior
/// - Validation is deterministic
/// - No regex (simpler, faster)
pub fn validate_tool_name(name: &str) -> Result<()> {
    if name.len() < 3 || name.len() > 64 {
        return Err(Error::InvalidMetadata(
            "Tool name must be 3-64 characters".to_string(),
        ));
    }

    if !name.chars().next().unwrap().is_ascii_lowercase() {
        return Err(Error::InvalidMetadata(
            "Tool name must start with a lowercase letter".to_string(),
        ));
    }

    if name.ends_with('-') {
        return Err(Error::InvalidMetadata(
            "Tool name cannot end with a hyphen".to_string(),
        ));
    }

    for ch in name.chars() {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '-' {
            return Err(Error::InvalidMetadata(
                "Tool name can only contain lowercase letters, numbers, and hyphens".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate semantic version.
///
/// Versions must follow semver: MAJOR.MINOR.PATCH
///
/// # Example
/// ```
/// use clawmaster_clawhub::metadata::validate_version;
///
/// assert!(validate_version("1.0.0").is_ok());
/// assert!(validate_version("0.1.2").is_ok());
/// assert!(validate_version("invalid").is_err());
/// ```
pub fn validate_version(version: &str) -> Result<()> {
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 3 {
        return Err(Error::InvalidVersion(
            "Version must be in format MAJOR.MINOR.PATCH".to_string(),
        ));
    }

    for part in parts {
        if part.parse::<u32>().is_err() {
            return Err(Error::InvalidVersion(
                "Version components must be numbers".to_string(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tool_name() {
        // Valid names
        assert!(validate_tool_name("calc").is_ok());
        assert!(validate_tool_name("web-fetch").is_ok());
        assert!(validate_tool_name("tool123").is_ok());

        // Invalid names
        assert!(validate_tool_name("ab").is_err()); // Too short
        assert!(validate_tool_name("Tool").is_err()); // Uppercase
        assert!(validate_tool_name("tool-").is_err()); // Ends with hyphen
        assert!(validate_tool_name("tool_name").is_err()); // Underscore
    }

    #[test]
    fn test_validate_version() {
        // Valid versions
        assert!(validate_version("1.0.0").is_ok());
        assert!(validate_version("0.1.2").is_ok());
        assert!(validate_version("10.20.30").is_ok());

        // Invalid versions
        assert!(validate_version("1.0").is_err());
        assert!(validate_version("1.0.0.0").is_err());
        assert!(validate_version("v1.0.0").is_err());
        assert!(validate_version("1.0.x").is_err());
    }
}
