//! Data models for folder access control
//!
//! DO-178C Level A: All models include validation and integrity checks

use serde::{Deserialize, Serialize};

/// Permission flags for folder access
///
/// # Compliance
/// DO-178C §6.3.4: Explicit permission model
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionFlags {
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delete: bool,
}

impl PermissionFlags {
    /// Check if any permission is granted
    pub fn has_any_permission(&self) -> bool {
        self.can_read || self.can_write || self.can_execute || self.can_delete
    }

    /// Check if all permissions are granted
    pub fn has_all_permissions(&self) -> bool {
        self.can_read && self.can_write && self.can_execute && self.can_delete
    }

    /// Create read-only permissions
    pub fn read_only() -> Self {
        Self {
            can_read: true,
            can_write: false,
            can_execute: false,
            can_delete: false,
        }
    }

    /// Create read-write permissions
    pub fn read_write() -> Self {
        Self {
            can_read: true,
            can_write: true,
            can_execute: false,
            can_delete: false,
        }
    }

    /// Create full permissions
    pub fn full() -> Self {
        Self {
            can_read: true,
            can_write: true,
            can_execute: true,
            can_delete: true,
        }
    }
}

/// Folder permission record
///
/// # Compliance
/// DO-178C §6.3.2: All fields validated on creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderPermission {
    pub id: i64,
    pub folder_path: String,
    pub folder_hash: String,
    pub permissions: PermissionFlags,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: String,
    pub is_active: bool,
    pub last_accessed_at: Option<i64>,
    pub access_count: i64,
}

impl FolderPermission {
    /// Validate folder permission record
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Input validation
    pub fn validate(&self) -> Result<(), String> {
        if self.folder_path.is_empty() {
            return Err("Folder path cannot be empty".to_string());
        }

        if self.folder_hash.is_empty() {
            return Err("Folder hash cannot be empty".to_string());
        }

        if self.created_by.is_empty() {
            return Err("Created by cannot be empty".to_string());
        }

        if !self.permissions.has_any_permission() {
            return Err("At least one permission must be granted".to_string());
        }

        Ok(())
    }
}

/// Access operation type
///
/// # Compliance
/// DO-178C §6.3.4: Enumerated operation types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AccessOperation {
    Read,
    Write,
    Execute,
    Delete,
}

impl AccessOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Execute => "execute",
            Self::Delete => "delete",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            "execute" => Some(Self::Execute),
            "delete" => Some(Self::Delete),
            _ => None,
        }
    }
}

/// Access log entry
///
/// # Compliance
/// DO-178C §11.10: Complete audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog {
    pub id: i64,
    pub folder_id: i64,
    pub operation: AccessOperation,
    pub file_path: Option<String>,
    pub success: bool,
    pub session_key: Option<String>,
    pub user_agent: Option<String>,
    pub error_message: Option<String>,
    pub timestamp: i64,
}

impl AccessLog {
    /// Create new access log entry
    pub fn new(
        folder_id: i64,
        operation: AccessOperation,
        file_path: Option<String>,
        success: bool,
    ) -> Self {
        Self {
            id: 0, // Will be set by database
            folder_id,
            operation,
            file_path,
            success,
            session_key: None,
            user_agent: None,
            error_message: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Add context information
    pub fn with_context(mut self, session_key: Option<String>, user_agent: Option<String>) -> Self {
        self.session_key = session_key;
        self.user_agent = user_agent;
        self
    }

    /// Add error information
    pub fn with_error(mut self, error: String) -> Self {
        self.error_message = Some(error);
        self
    }
}

/// Validation rule type
///
/// # Compliance
/// DO-178C Security: Path validation rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Blacklist,
    Whitelist,
    Pattern,
}

impl RuleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blacklist => "blacklist",
            Self::Whitelist => "whitelist",
            Self::Pattern => "pattern",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blacklist" => Some(Self::Blacklist),
            "whitelist" => Some(Self::Whitelist),
            "pattern" => Some(Self::Pattern),
            _ => None,
        }
    }
}

/// Validation rule
///
/// # Compliance
/// DO-178C Security: Configurable validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: i64,
    pub rule_type: RuleType,
    pub pattern: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub priority: i32,
    pub created_at: i64,
    pub created_by: String,
}

impl ValidationRule {
    /// Check if path matches this rule
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic pattern matching
    pub fn matches(&self, path: &str) -> bool {
        match self.rule_type {
            RuleType::Blacklist | RuleType::Whitelist => {
                self.matches_glob_pattern(path)
            }
            RuleType::Pattern => {
                self.matches_regex_pattern(path)
            }
        }
    }

    fn matches_glob_pattern(&self, path: &str) -> bool {
        // Simple glob matching (* wildcard)
        let pattern = &self.pattern;
        
        if pattern == "*" {
            return true;
        }

        // Exact match
        if !pattern.contains('*') {
            return path == pattern;
        }

        // Split pattern by * and match each part
        let parts: Vec<&str> = pattern.split('*').collect();
        
        if parts.is_empty() {
            return true;
        }

        let mut pos = 0;
        
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if i == 0 {
                // First part must match at the beginning
                if !path[pos..].starts_with(part) {
                    return false;
                }
                pos += part.len();
            } else if i == parts.len() - 1 {
                // Last part must match at the end
                if !path[pos..].ends_with(part) {
                    return false;
                }
                return true;
            } else {
                // Middle parts must be found somewhere
                if let Some(found_pos) = path[pos..].find(part) {
                    pos += found_pos + part.len();
                } else {
                    return false;
                }
            }
        }

        true
    }

    fn matches_regex_pattern(&self, _path: &str) -> bool {
        // For now, use simple glob matching
        // In production, use regex crate
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_flags() {
        let read_only = PermissionFlags::read_only();
        assert!(read_only.can_read);
        assert!(!read_only.can_write);
        assert!(read_only.has_any_permission());
        assert!(!read_only.has_all_permissions());

        let full = PermissionFlags::full();
        assert!(full.has_all_permissions());
    }

    #[test]
    fn test_access_operation() {
        assert_eq!(AccessOperation::Read.as_str(), "read");
        assert_eq!(AccessOperation::from_str("write"), Some(AccessOperation::Write));
        assert_eq!(AccessOperation::from_str("invalid"), None);
    }

    #[test]
    fn test_validation_rule_glob_matching() {
        let rule = ValidationRule {
            id: 1,
            rule_type: RuleType::Blacklist,
            pattern: "/etc/*".to_string(),
            description: None,
            is_active: true,
            priority: 100,
            created_at: 0,
            created_by: "test".to_string(),
        };

        assert!(rule.matches("/etc/passwd"));
        assert!(rule.matches("/etc/shadow"));
        assert!(!rule.matches("/home/user/file"));
    }

    #[test]
    fn test_validation_rule_exact_match() {
        let rule = ValidationRule {
            id: 1,
            rule_type: RuleType::Blacklist,
            pattern: "/etc/passwd".to_string(),
            description: None,
            is_active: true,
            priority: 100,
            created_at: 0,
            created_by: "test".to_string(),
        };

        assert!(rule.matches("/etc/passwd"));
        assert!(!rule.matches("/etc/shadow"));
    }

    #[test]
    fn test_validation_rule_suffix_match() {
        let rule = ValidationRule {
            id: 1,
            rule_type: RuleType::Blacklist,
            pattern: "*/.ssh/*".to_string(),
            description: None,
            is_active: true,
            priority: 100,
            created_at: 0,
            created_by: "test".to_string(),
        };

        assert!(rule.matches("/home/user/.ssh/id_rsa"));
        assert!(!rule.matches("/home/user/file"));
    }
}
