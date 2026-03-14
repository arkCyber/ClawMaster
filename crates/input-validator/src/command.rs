//! Command Validation
//!
//! DO-178C Level A Compliant Command Validation

use crate::{ValidationError, ValidationResult};
use once_cell::sync::Lazy;
use regex::Regex;

/// Maximum command length
const MAX_COMMAND_LENGTH: usize = 10_000;

/// Shell injection patterns to detect
static SHELL_INJECTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"[;&|`$]").unwrap(),           // Command separators and substitution
        Regex::new(r"\$\(").unwrap(),              // Command substitution
        Regex::new(r"\$\{").unwrap(),              // Variable expansion
        Regex::new(r">\s*/dev/").unwrap(),         // Redirect to device
        Regex::new(r"\|\s*sh").unwrap(),           // Pipe to shell
        Regex::new(r"\|\s*bash").unwrap(),         // Pipe to bash
        Regex::new(r"&&").unwrap(),                // Command chaining
        Regex::new(r"\|\|").unwrap(),              // Command chaining
        Regex::new(r">\s*&").unwrap(),             // Redirect stderr
        Regex::new(r"<\s*&").unwrap(),             // Redirect stdin
    ]
});

/// Dangerous commands that should never be allowed
const DANGEROUS_COMMANDS: &[&str] = &[
    "rm -rf /",
    "mkfs",
    "dd if=/dev/zero",
    ":(){ :|:& };:",  // Fork bomb
    "chmod -R 777 /",
    "chown -R",
];

/// Validate command string
///
/// DO-178C §6.3.1: Command validation
pub fn validate_command(command: &str) -> ValidationResult<()> {
    // Check length
    if command.len() > MAX_COMMAND_LENGTH {
        return Err(ValidationError::TooLong {
            actual: command.len(),
            max: MAX_COMMAND_LENGTH,
        });
    }

    // Check for null bytes
    if command.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for shell injection patterns
    for pattern in SHELL_INJECTION_PATTERNS.iter() {
        if pattern.is_match(command) {
            return Err(ValidationError::ShellInjection);
        }
    }

    // Check for dangerous commands
    for dangerous in DANGEROUS_COMMANDS {
        if command.contains(dangerous) {
            return Err(ValidationError::Dangerous(
                format!("Dangerous command detected: {}", dangerous)
            ));
        }
    }

    Ok(())
}

/// Validate command arguments (safer than full command)
///
/// DO-178C §6.3.1: Argument validation
pub fn validate_command_args(args: &[String]) -> ValidationResult<()> {
    for arg in args {
        // Check length
        if arg.len() > 1000 {
            return Err(ValidationError::TooLong {
                actual: arg.len(),
                max: 1000,
            });
        }

        // Check for null bytes
        if arg.contains('\0') {
            return Err(ValidationError::NullByte);
        }

        // Check for shell metacharacters in arguments
        if arg.contains('$') || arg.contains('`') || arg.contains('|') {
            return Err(ValidationError::ShellInjection);
        }
    }

    Ok(())
}

/// Sanitize command (remove dangerous characters)
///
/// DO-178C §6.3.1: Command sanitization
pub fn sanitize_command(command: &str) -> String {
    let mut sanitized = command.to_string();

    // Remove null bytes
    sanitized = sanitized.replace('\0', "");

    // Remove dangerous characters
    sanitized = sanitized.replace('$', "");
    sanitized = sanitized.replace('`', "");
    sanitized = sanitized.replace('|', "");
    sanitized = sanitized.replace(';', "");
    sanitized = sanitized.replace('&', "");

    // Truncate if too long
    if sanitized.len() > MAX_COMMAND_LENGTH {
        sanitized.truncate(MAX_COMMAND_LENGTH);
    }

    sanitized
}

/// Check if command is dangerous
///
/// DO-178C §6.3.1: Threat detection
pub fn is_dangerous_command(command: &str) -> bool {
    // Check for shell injection patterns
    for pattern in SHELL_INJECTION_PATTERNS.iter() {
        if pattern.is_match(command) {
            return true;
        }
    }

    // Check for dangerous commands
    for dangerous in DANGEROUS_COMMANDS {
        if command.contains(dangerous) {
            return true;
        }
    }

    false
}

/// Extract command binary name (first word)
///
/// DO-178C §6.3.1: Command parsing
pub fn extract_command_binary(command: &str) -> Option<String> {
    command.split_whitespace().next().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command_valid() {
        assert!(validate_command("ls").is_ok());
        assert!(validate_command("git status").is_ok());
        assert!(validate_command("npm install").is_ok());
    }

    #[test]
    fn test_validate_command_too_long() {
        let long_command = "a".repeat(MAX_COMMAND_LENGTH + 1);
        let result = validate_command(&long_command);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TooLong { .. }));
    }

    #[test]
    fn test_validate_command_null_byte() {
        let result = validate_command("ls\0-la");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::NullByte));
    }

    #[test]
    fn test_validate_command_shell_injection_semicolon() {
        let result = validate_command("ls; rm -rf /");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::ShellInjection));
    }

    #[test]
    fn test_validate_command_shell_injection_pipe() {
        let result = validate_command("cat file | sh");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::ShellInjection));
    }

    #[test]
    fn test_validate_command_shell_injection_substitution() {
        let result = validate_command("echo $(whoami)");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::ShellInjection));
    }

    #[test]
    fn test_validate_command_dangerous() {
        let result = validate_command("rm -rf /");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::Dangerous(_)));
    }

    #[test]
    fn test_validate_command_args_valid() {
        let args = vec!["status".to_string(), "--short".to_string()];
        assert!(validate_command_args(&args).is_ok());
    }

    #[test]
    fn test_validate_command_args_shell_injection() {
        let args = vec!["status".to_string(), "$(whoami)".to_string()];
        let result = validate_command_args(&args);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::ShellInjection));
    }

    #[test]
    fn test_sanitize_command() {
        let sanitized = sanitize_command("ls; rm -rf /");
        assert!(!sanitized.contains(';'));
    }

    #[test]
    fn test_is_dangerous_command() {
        assert!(!is_dangerous_command("ls"));
        assert!(is_dangerous_command("ls; rm -rf /"));
        assert!(is_dangerous_command("rm -rf /"));
    }

    #[test]
    fn test_extract_command_binary() {
        assert_eq!(extract_command_binary("ls -la"), Some("ls".to_string()));
        assert_eq!(extract_command_binary("git status"), Some("git".to_string()));
        assert_eq!(extract_command_binary(""), None);
    }
}
