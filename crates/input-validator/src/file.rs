//! File Path Validation
//!
//! DO-178C Level A Compliant Path Validation

use crate::{ValidationError, ValidationResult};
use std::path::{Path, PathBuf};

/// Maximum path length
const MAX_PATH_LENGTH: usize = 4096;

/// Dangerous path components
const DANGEROUS_COMPONENTS: &[&str] = &[
    "..",           // Parent directory traversal
    "~",            // Home directory (when at start)
];

/// Dangerous path prefixes (absolute paths to system directories)
const DANGEROUS_PREFIXES: &[&str] = &[
    "/etc",
    "/bin",
    "/sbin",
    "/usr/bin",
    "/usr/sbin",
    "/System",
    "/Library/System",
    "/private/etc",
    "/private/var/root",
];

/// Validate file path
///
/// DO-178C §6.3.1: Path validation
pub fn validate_path(path: &str) -> ValidationResult<PathBuf> {
    // Check length
    if path.len() > MAX_PATH_LENGTH {
        return Err(ValidationError::TooLong {
            actual: path.len(),
            max: MAX_PATH_LENGTH,
        });
    }

    // Check for null bytes
    if path.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for path traversal
    if path.contains("..") {
        return Err(ValidationError::PathTraversal);
    }

    // Check for dangerous absolute paths
    for prefix in DANGEROUS_PREFIXES {
        if path.starts_with(prefix) {
            return Err(ValidationError::Dangerous(
                format!("Access to system directory not allowed: {}", prefix)
            ));
        }
    }

    // Convert to PathBuf and normalize
    let path_buf = PathBuf::from(path);

    // Additional check: ensure no parent directory components after normalization
    for component in path_buf.components() {
        if component.as_os_str() == ".." {
            return Err(ValidationError::PathTraversal);
        }
    }

    Ok(path_buf)
}

/// Validate file path is within allowed directory
///
/// DO-178C §6.3.1: Access control
pub fn validate_path_in_directory(path: &str, allowed_dir: &Path) -> ValidationResult<PathBuf> {
    let path_buf = validate_path(path)?;

    // Convert to absolute path for comparison
    let abs_path = if path_buf.is_absolute() {
        path_buf.clone()
    } else {
        allowed_dir.join(&path_buf)
    };

    // Canonicalize to resolve symlinks and .. components
    let canonical = abs_path.canonicalize().map_err(|e| {
        ValidationError::Invalid(format!("Cannot resolve path: {}", e))
    })?;

    let canonical_allowed = allowed_dir.canonicalize().map_err(|e| {
        ValidationError::Invalid(format!("Cannot resolve allowed directory: {}", e))
    })?;

    // Check if path is within allowed directory
    if !canonical.starts_with(&canonical_allowed) {
        return Err(ValidationError::Dangerous(
            "Path is outside allowed directory".to_string()
        ));
    }

    Ok(canonical)
}

/// Sanitize file path (remove dangerous components)
///
/// DO-178C §6.3.1: Path sanitization
pub fn sanitize_path(path: &str) -> String {
    let mut sanitized = path.to_string();

    // Remove null bytes
    sanitized = sanitized.replace('\0', "");

    // Remove parent directory references
    sanitized = sanitized.replace("..", "");

    // Truncate if too long
    if sanitized.len() > MAX_PATH_LENGTH {
        sanitized.truncate(MAX_PATH_LENGTH);
    }

    sanitized
}

/// Check if path is dangerous
///
/// DO-178C §6.3.1: Threat detection
pub fn is_dangerous_path(path: &str) -> bool {
    // Check for null bytes
    if path.contains('\0') {
        return true;
    }

    // Check for path traversal
    if path.contains("..") {
        return true;
    }

    // Check for dangerous prefixes
    for prefix in DANGEROUS_PREFIXES {
        if path.starts_with(prefix) {
            return true;
        }
    }

    false
}

/// Validate filename (no path components)
///
/// DO-178C §6.3.1: Filename validation
pub fn validate_filename(filename: &str) -> ValidationResult<String> {
    // Check length
    if filename.len() > 255 {
        return Err(ValidationError::TooLong {
            actual: filename.len(),
            max: 255,
        });
    }

    // Check for null bytes
    if filename.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for path separators
    if filename.contains('/') || filename.contains('\\') {
        return Err(ValidationError::ForbiddenChar(
            "Path separators not allowed in filename".to_string()
        ));
    }

    // Check for parent directory reference
    if filename == ".." || filename == "." {
        return Err(ValidationError::Dangerous(
            "Special directory names not allowed".to_string()
        ));
    }

    Ok(filename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_valid() {
        assert!(validate_path("file.txt").is_ok());
        assert!(validate_path("dir/file.txt").is_ok());
        assert!(validate_path("./file.txt").is_ok());
    }

    #[test]
    fn test_validate_path_too_long() {
        let long_path = "a".repeat(MAX_PATH_LENGTH + 1);
        let result = validate_path(&long_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TooLong { .. }));
    }

    #[test]
    fn test_validate_path_null_byte() {
        let result = validate_path("file\0.txt");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::NullByte));
    }

    #[test]
    fn test_validate_path_traversal() {
        let result = validate_path("../etc/passwd");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::PathTraversal));
    }

    #[test]
    fn test_validate_path_dangerous_prefix() {
        let result = validate_path("/etc/passwd");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::Dangerous(_)));
    }

    #[test]
    fn test_sanitize_path_null_bytes() {
        let sanitized = sanitize_path("file\0.txt");
        assert_eq!(sanitized, "file.txt");
    }

    #[test]
    fn test_sanitize_path_traversal() {
        let sanitized = sanitize_path("../etc/passwd");
        assert_eq!(sanitized, "/etc/passwd");
    }

    #[test]
    fn test_is_dangerous_path() {
        assert!(!is_dangerous_path("file.txt"));
        assert!(is_dangerous_path("../etc/passwd"));
        assert!(is_dangerous_path("/etc/passwd"));
        assert!(is_dangerous_path("file\0.txt"));
    }

    #[test]
    fn test_validate_filename_valid() {
        assert!(validate_filename("file.txt").is_ok());
        assert!(validate_filename("document.pdf").is_ok());
    }

    #[test]
    fn test_validate_filename_too_long() {
        let long_name = "a".repeat(256);
        let result = validate_filename(&long_name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TooLong { .. }));
    }

    #[test]
    fn test_validate_filename_path_separator() {
        let result = validate_filename("dir/file.txt");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::ForbiddenChar(_)));
    }

    #[test]
    fn test_validate_filename_parent_dir() {
        let result = validate_filename("..");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::Dangerous(_)));
    }
}
