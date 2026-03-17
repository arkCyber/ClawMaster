//! Path validation and security checks
//!
//! DO-178C Level A: Comprehensive path validation to prevent security vulnerabilities

use {
    anyhow::{Result, anyhow},
    sha2::{Digest, Sha256},
    std::path::{Path, PathBuf},
};

use crate::models::ValidationRule;

/// Validation result
///
/// # Compliance
/// DO-178C §6.3.2: Explicit validation results
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub canonical_path: Option<PathBuf>,
    pub error_message: Option<String>,
    pub matched_rules: Vec<String>,
}

impl ValidationResult {
    pub fn valid(canonical_path: PathBuf) -> Self {
        Self {
            is_valid: true,
            canonical_path: Some(canonical_path),
            error_message: None,
            matched_rules: Vec::new(),
        }
    }

    pub fn invalid(error: String) -> Self {
        Self {
            is_valid: false,
            canonical_path: None,
            error_message: Some(error),
            matched_rules: Vec::new(),
        }
    }

    pub fn with_matched_rules(mut self, rules: Vec<String>) -> Self {
        self.matched_rules = rules;
        self
    }
}

/// Path validator with security checks
///
/// # Compliance
/// DO-178C Security: Multi-layer path validation
pub struct PathValidator {
    validation_rules: Vec<ValidationRule>,
}

impl PathValidator {
    /// Create new path validator
    pub fn new(validation_rules: Vec<ValidationRule>) -> Self {
        Self { validation_rules }
    }

    /// Validate path with comprehensive security checks
    ///
    /// # Security Checks
    /// 1. Null byte injection
    /// 2. Path traversal (..)
    /// 3. Symbolic link resolution
    /// 4. Blacklist/whitelist rules
    /// 5. Path canonicalization
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All validation errors properly reported
    /// DO-178C Security: Defense in depth
    pub fn validate(&self, path: &str) -> ValidationResult {
        // Check 1: Null byte injection (DO-178C Security)
        if path.contains('\0') {
            return ValidationResult::invalid("Path contains null bytes".to_string());
        }

        // Check 2: Empty path
        if path.is_empty() {
            return ValidationResult::invalid("Path cannot be empty".to_string());
        }

        // Check 3: Path length limit (prevent DoS)
        if path.len() > 4096 {
            return ValidationResult::invalid("Path exceeds maximum length".to_string());
        }

        // Check 4: Invalid characters
        if path.contains('\r') || path.contains('\n') {
            return ValidationResult::invalid("Path contains invalid characters".to_string());
        }

        let path_obj = Path::new(path);

        // Check 5: Canonicalize path (resolve symlinks and ..)
        let canonical_path = match path_obj.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                // If path doesn't exist, try to canonicalize parent
                if let Some(parent) = path_obj.parent() {
                    if let Ok(parent_canonical) = parent.canonicalize() {
                        parent_canonical.join(path_obj.file_name().unwrap_or_default())
                    } else {
                        return ValidationResult::invalid(format!(
                            "Failed to canonicalize path: {}",
                            e
                        ));
                    }
                } else {
                    return ValidationResult::invalid(format!(
                        "Failed to canonicalize path: {}",
                        e
                    ));
                }
            },
        };

        // Check 6: Apply validation rules
        let canonical_str = canonical_path.to_string_lossy();
        let mut matched_rules = Vec::new();

        for rule in &self.validation_rules {
            if !rule.is_active {
                continue;
            }

            if rule.matches(&canonical_str) {
                matched_rules.push(rule.pattern.clone());

                match rule.rule_type {
                    crate::models::RuleType::Blacklist => {
                        return ValidationResult::invalid(format!(
                            "Path matches blacklist rule: {}",
                            rule.pattern
                        ))
                        .with_matched_rules(matched_rules);
                    },
                    crate::models::RuleType::Whitelist => {
                        // Whitelist match is good, continue checking
                    },
                    crate::models::RuleType::Pattern => {
                        // Custom pattern, check description for action
                    },
                }
            }
        }

        ValidationResult::valid(canonical_path).with_matched_rules(matched_rules)
    }

    /// Validate that path is within allowed base directory
    ///
    /// # Compliance
    /// DO-178C Security: Prevent directory escape
    pub fn validate_within_base(&self, path: &str, base: &Path) -> ValidationResult {
        let validation = self.validate(path);

        if !validation.is_valid {
            return validation;
        }

        let canonical_path = validation.canonical_path.as_ref().unwrap();

        // Canonicalize base path
        let canonical_base = match base.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return ValidationResult::invalid(format!(
                    "Failed to canonicalize base path: {}",
                    e
                ));
            },
        };

        // Check if path is within base
        if !canonical_path.starts_with(&canonical_base) {
            return ValidationResult::invalid(format!(
                "Path escapes base directory: {}",
                base.display()
            ));
        }

        validation
    }

    /// Check if path is a directory
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic checks
    pub fn is_directory(&self, path: &str) -> Result<bool> {
        let validation = self.validate(path);

        if !validation.is_valid {
            return Err(anyhow!(validation.error_message.unwrap_or_default()));
        }

        let canonical_path = validation.canonical_path.unwrap();
        Ok(canonical_path.is_dir())
    }

    /// Check if path exists
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic checks
    pub fn exists(&self, path: &str) -> Result<bool> {
        let validation = self.validate(path);

        if !validation.is_valid {
            return Err(anyhow!(validation.error_message.unwrap_or_default()));
        }

        let canonical_path = validation.canonical_path.unwrap();
        Ok(canonical_path.exists())
    }
}

/// Calculate SHA-256 hash of path for integrity verification
///
/// # Compliance
/// DO-178C §6.3.4: Deterministic hash generation
pub fn calculate_path_hash(path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Normalize path for consistent comparison
///
/// # Compliance
/// DO-178C §6.3.4: Consistent path normalization
pub fn normalize_path(path: &str) -> Result<String> {
    let path_obj = Path::new(path);
    let canonical = path_obj
        .canonicalize()
        .map_err(|e| anyhow!("Failed to canonicalize path: {}", e))?;

    Ok(canonical.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::models::{RuleType, ValidationRule},
        tempfile::tempdir,
    };

    fn create_test_validator() -> PathValidator {
        let rules = vec![
            ValidationRule {
                id: 1,
                rule_type: RuleType::Blacklist,
                pattern: "/etc/*".to_string(),
                description: Some("System files".to_string()),
                is_active: true,
                priority: 100,
                created_at: 0,
                created_by: "test".to_string(),
            },
            ValidationRule {
                id: 2,
                rule_type: RuleType::Blacklist,
                pattern: "*/.ssh/*".to_string(),
                description: Some("SSH keys".to_string()),
                is_active: true,
                priority: 100,
                created_at: 0,
                created_by: "test".to_string(),
            },
        ];

        PathValidator::new(rules)
    }

    #[test]
    fn test_validate_null_bytes() {
        let validator = create_test_validator();
        let result = validator.validate("test\0file.txt");
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("null bytes"));
    }

    #[test]
    fn test_validate_empty_path() {
        let validator = create_test_validator();
        let result = validator.validate("");
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("empty"));
    }

    #[test]
    fn test_validate_blacklist_match() {
        // Test that blacklist rules work correctly
        // We create a validator with a specific blacklist rule
        let rules = vec![ValidationRule {
            id: 1,
            rule_type: RuleType::Blacklist,
            pattern: "*/blocked/*".to_string(),
            description: Some("Blocked paths".to_string()),
            is_active: true,
            priority: 100,
            created_at: 0,
            created_by: "test".to_string(),
        }];

        let validator = PathValidator::new(rules);

        // Create a path that matches the blacklist pattern
        let dir = tempdir().unwrap();
        let blocked_dir = dir.path().join("blocked");
        std::fs::create_dir(&blocked_dir).unwrap();
        let test_file = blocked_dir.join("test.txt");
        std::fs::write(&test_file, "test").unwrap();

        let result = validator.validate(test_file.to_str().unwrap());
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("blacklist"));
    }

    #[test]
    fn test_validate_valid_path() {
        let validator = create_test_validator();
        let dir = tempdir().unwrap();
        let test_path = dir.path().join("test.txt");
        std::fs::write(&test_path, "test").unwrap();

        let result = validator.validate(test_path.to_str().unwrap());
        assert!(result.is_valid);
        assert!(result.canonical_path.is_some());
    }

    #[test]
    fn test_validate_within_base() {
        let validator = create_test_validator();
        let dir = tempdir().unwrap();
        let base = dir.path();
        let test_path = base.join("subdir/test.txt");
        std::fs::create_dir_all(test_path.parent().unwrap()).unwrap();
        std::fs::write(&test_path, "test").unwrap();

        let result = validator.validate_within_base(test_path.to_str().unwrap(), base);
        assert!(result.is_valid);
    }

    #[test]
    fn test_calculate_path_hash() {
        let hash1 = calculate_path_hash("/test/path");
        let hash2 = calculate_path_hash("/test/path");
        let hash3 = calculate_path_hash("/different/path");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_is_directory() {
        let validator = create_test_validator();
        let dir = tempdir().unwrap();

        let result = validator.is_directory(dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_path_length_limit() {
        let validator = create_test_validator();
        let long_path = "a".repeat(5000);
        let result = validator.validate(&long_path);
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("maximum length"));
    }

    #[test]
    fn test_invalid_characters() {
        let validator = create_test_validator();
        let result = validator.validate("test\nfile.txt");
        assert!(!result.is_valid);
        assert!(result.error_message.unwrap().contains("invalid characters"));
    }
}
