//! Core types for ClawHub.

use {
    serde::{Deserialize, Serialize},
    time::OffsetDateTime,
};

/// Tool metadata.
///
/// This represents all the information about a published tool.
///
/// # Compliance
/// DO-178C §6.3.4: Deterministic behavior
/// - All fields are explicitly typed
/// - No dynamic dispatch
/// - Serialization is deterministic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// Tool name (unique identifier)
    pub name: String,

    /// Semantic version (e.g., "1.0.0")
    pub version: String,

    /// Short description
    pub description: String,

    /// Long description (Markdown)
    pub readme: Option<String>,

    /// Author name
    pub author: String,

    /// Author email
    pub author_email: Option<String>,

    /// License (SPDX identifier)
    pub license: String,

    /// Repository URL
    pub repository: Option<String>,

    /// Homepage URL
    pub homepage: Option<String>,

    /// Keywords for search
    pub keywords: Vec<String>,

    /// Categories
    pub categories: Vec<String>,

    /// Tool type
    pub tool_type: ToolType,

    /// Wasm file SHA-256 hash
    pub wasm_hash: String,

    /// Wasm file size in bytes
    pub wasm_size: u64,

    /// Ed25519 signature (hex-encoded)
    pub signature: String,

    /// Public key (hex-encoded)
    pub public_key: String,

    /// Download count
    pub downloads: u64,

    /// Security scan status
    pub security_status: SecurityStatus,

    /// Publication timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub published_at: OffsetDateTime,

    /// Last update timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Tool type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Pure computation tool (no I/O)
    Pure,

    /// HTTP-capable tool
    Http,
}

/// Security scan status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityStatus {
    /// Not yet scanned
    Pending,

    /// Scan in progress
    Scanning,

    /// Passed all security checks
    Verified,

    /// Failed security checks
    Failed,

    /// Manually reviewed and approved
    Approved,
}

/// Tool search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search term (matches name, description, keywords)
    pub query: Option<String>,

    /// Filter by category
    pub category: Option<String>,

    /// Filter by tool type
    pub tool_type: Option<ToolType>,

    /// Filter by security status
    pub security_status: Option<SecurityStatus>,

    /// Sort order
    pub sort: SortOrder,

    /// Page number (0-indexed)
    pub page: u32,

    /// Page size
    pub page_size: u32,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: None,
            category: None,
            tool_type: None,
            security_status: None,
            sort: SortOrder::Downloads,
            page: 0,
            page_size: 20,
        }
    }
}

/// Sort order for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Most downloads first
    Downloads,

    /// Most recent first
    Recent,

    /// Alphabetical
    Name,

    /// Relevance (when query is provided)
    Relevance,
}

/// Tool version info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVersion {
    /// Version string
    pub version: String,

    /// Publication timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub published_at: OffsetDateTime,

    /// Download count for this version
    pub downloads: u64,

    /// Security status
    pub security_status: SecurityStatus,
}

/// Publishing request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    /// Tool metadata
    pub metadata: ToolMetadata,

    /// Wasm file bytes (base64-encoded)
    pub wasm_bytes: String,
}

/// Publishing response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponse {
    /// Tool name
    pub name: String,

    /// Version
    pub version: String,

    /// Download URL
    pub download_url: String,

    /// Success message
    pub message: String,
}

// ── Skills Types ─────────────────────────────────────────────────────────────

/// Skill metadata.
///
/// Represents a skill (SKILL.md, Claude Code plugin, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Skill name (unique identifier)
    pub name: String,

    /// Semantic version (e.g., "1.0.0")
    pub version: String,

    /// Short description
    pub description: String,

    /// Long description (Markdown)
    pub readme: Option<String>,

    /// Author name
    pub author: String,

    /// Author email
    pub author_email: Option<String>,

    /// License (SPDX identifier)
    pub license: String,

    /// Repository URL
    pub repository: Option<String>,

    /// Homepage URL
    pub homepage: Option<String>,

    /// Keywords for search
    pub keywords: Vec<String>,

    /// Categories
    pub categories: Vec<String>,

    /// Skill format
    pub skill_format: SkillFormat,

    /// GitHub repository (owner/repo)
    pub github_repo: Option<String>,

    /// Commit SHA
    pub commit_sha: Option<String>,

    /// Download count
    pub downloads: u64,

    /// Star count
    pub stars: u64,

    /// Security scan status
    pub security_status: SecurityStatus,

    /// Publication timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub published_at: OffsetDateTime,

    /// Last update timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// Skill format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillFormat {
    /// SKILL.md format (standard)
    SkillMd,

    /// Claude Code plugin format
    ClaudeCode,

    /// Custom format
    Custom,
}

/// Skill search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSearchQuery {
    /// Search term (matches name, description, keywords)
    pub query: Option<String>,

    /// Filter by category
    pub category: Option<String>,

    /// Filter by skill format
    pub skill_format: Option<SkillFormat>,

    /// Filter by security status
    pub security_status: Option<SecurityStatus>,

    /// Sort order
    pub sort: SortOrder,

    /// Page number (0-indexed)
    pub page: u32,

    /// Page size
    pub page_size: u32,
}

impl Default for SkillSearchQuery {
    fn default() -> Self {
        Self {
            query: None,
            category: None,
            skill_format: None,
            security_status: None,
            sort: SortOrder::Downloads,
            page: 0,
            page_size: 20,
        }
    }
}

/// Skill publishing request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishSkillRequest {
    /// Skill metadata
    pub metadata: SkillMetadata,
}

/// Skill publishing response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishSkillResponse {
    /// Skill name
    pub name: String,

    /// Version
    pub version: String,

    /// Install command
    pub install_command: String,

    /// Success message
    pub message: String,
}

/// Item type for unified operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    /// Wasm tool
    Tool,

    /// Skill
    Skill,
}
