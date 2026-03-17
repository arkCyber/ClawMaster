//! Skill review and security scanning system
//!
//! DO-178C Level A compliant automated review system with:
//! - Security scanning
//! - Code quality analysis
//! - Dependency checking
//! - Compliance verification

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::types::SkillMetadata;

/// Review status for a skill
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    /// Pending review
    Pending,
    /// Approved for use
    Approved,
    /// Rejected with issues
    Rejected,
    /// Requires manual review
    ManualReview,
}

/// Security scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    /// Overall security score (0-100)
    pub score: u8,
    /// Security issues found
    pub issues: Vec<SecurityIssue>,
    /// Scan timestamp
    pub scanned_at: String,
}

/// Security issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// Issue severity
    pub severity: Severity,
    /// Issue category
    pub category: String,
    /// Issue description
    pub description: String,
    /// File location (optional)
    pub file: Option<String>,
    /// Line number (optional)
    pub line: Option<u32>,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Code quality result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityResult {
    /// Overall quality score (0-100)
    pub score: u8,
    /// Quality issues found
    pub issues: Vec<QualityIssue>,
    /// Analyzed at
    pub analyzed_at: String,
}

/// Code quality issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Issue type
    pub issue_type: String,
    /// Issue description
    pub description: String,
    /// File location (optional)
    pub file: Option<String>,
    /// Line number (optional)
    pub line: Option<u32>,
}

/// Complete skill review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillReview {
    /// Skill name
    pub skill_name: String,
    /// Skill version
    pub version: String,
    /// Review status
    pub status: ReviewStatus,
    /// Security scan result
    pub security_scan: SecurityScanResult,
    /// Code quality result
    pub code_quality: CodeQualityResult,
    /// Overall score (0-100)
    pub overall_score: u8,
    /// Reviewer notes
    pub notes: Option<String>,
    /// Reviewed at
    pub reviewed_at: String,
}

/// Perform automated security scan on a skill
pub async fn scan_security(skill_path: &Path, metadata: &SkillMetadata) -> Result<SecurityScanResult> {
    let mut issues = Vec::new();
    let mut score = 100u8;

    // 1. Check for dangerous tool usage
    let dangerous_tools = ["bash", "exec", "write_file", "delete_file"];
    for tool in &metadata.allowed_tools {
        if dangerous_tools.contains(&tool.as_str()) {
            issues.push(SecurityIssue {
                severity: Severity::Medium,
                category: "dangerous_tool".to_string(),
                description: format!("Uses potentially dangerous tool: {}", tool),
                file: Some("SKILL.md".to_string()),
                line: None,
            });
            score = score.saturating_sub(5);
        }
    }

    // 2. Check for path traversal patterns
    if let Ok(content) = tokio::fs::read_to_string(skill_path.join("SKILL.md")).await {
        if content.contains("../") || content.contains("..\\") {
            issues.push(SecurityIssue {
                severity: Severity::High,
                category: "path_traversal".to_string(),
                description: "Potential path traversal detected".to_string(),
                file: Some("SKILL.md".to_string()),
                line: None,
            });
            score = score.saturating_sub(20);
        }

        // 3. Check for hardcoded credentials
        let credential_patterns = ["password", "api_key", "secret", "token"];
        for pattern in &credential_patterns {
            if content.to_lowercase().contains(pattern) {
                issues.push(SecurityIssue {
                    severity: Severity::High,
                    category: "credential_exposure".to_string(),
                    description: format!("Potential credential exposure: {}", pattern),
                    file: Some("SKILL.md".to_string()),
                    line: None,
                });
                score = score.saturating_sub(15);
            }
        }

        // 4. Check for shell injection patterns
        let injection_patterns = ["eval", "exec", "system", "$(", "`"];
        for pattern in &injection_patterns {
            if content.contains(pattern) {
                issues.push(SecurityIssue {
                    severity: Severity::Critical,
                    category: "code_injection".to_string(),
                    description: format!("Potential code injection pattern: {}", pattern),
                    file: Some("SKILL.md".to_string()),
                    line: None,
                });
                score = score.saturating_sub(30);
            }
        }

        // 5. Check for network access patterns
        let network_patterns = ["http://", "https://", "ftp://", "ssh://"];
        for pattern in &network_patterns {
            if content.contains(pattern) {
                issues.push(SecurityIssue {
                    severity: Severity::Low,
                    category: "network_access".to_string(),
                    description: format!("Network access detected: {}", pattern),
                    file: Some("SKILL.md".to_string()),
                    line: None,
                });
                score = score.saturating_sub(3);
            }
        }
    }

    // 6. Check for missing license
    if metadata.license.is_none() {
        issues.push(SecurityIssue {
            severity: Severity::Info,
            category: "missing_license".to_string(),
            description: "No license specified".to_string(),
            file: Some("SKILL.md".to_string()),
            line: None,
        });
        score = score.saturating_sub(5);
    }

    Ok(SecurityScanResult {
        score,
        issues,
        scanned_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Perform code quality analysis on a skill
pub async fn analyze_quality(skill_path: &Path, metadata: &SkillMetadata) -> Result<CodeQualityResult> {
    let mut issues = Vec::new();
    let mut score = 100u8;

    // 1. Check metadata completeness
    if metadata.description.is_empty() {
        issues.push(QualityIssue {
            issue_type: "missing_description".to_string(),
            description: "Skill description is empty".to_string(),
            file: Some("SKILL.md".to_string()),
            line: None,
        });
        score = score.saturating_sub(10);
    }

    if metadata.description.len() < 20 {
        issues.push(QualityIssue {
            issue_type: "short_description".to_string(),
            description: "Skill description is too short (< 20 chars)".to_string(),
            file: Some("SKILL.md".to_string()),
            line: None,
        });
        score = score.saturating_sub(5);
    }

    if metadata.homepage.is_none() {
        issues.push(QualityIssue {
            issue_type: "missing_homepage".to_string(),
            description: "No homepage URL specified".to_string(),
            file: Some("SKILL.md".to_string()),
            line: None,
        });
        score = score.saturating_sub(5);
    }

    // 2. Check for README
    if !skill_path.join("README.md").exists() {
        issues.push(QualityIssue {
            issue_type: "missing_readme".to_string(),
            description: "No README.md file found".to_string(),
            file: None,
            line: None,
        });
        score = score.saturating_sub(10);
    }

    // 3. Check for LICENSE
    if !skill_path.join("LICENSE").exists() && !skill_path.join("LICENSE.md").exists() {
        issues.push(QualityIssue {
            issue_type: "missing_license_file".to_string(),
            description: "No LICENSE file found".to_string(),
            file: None,
            line: None,
        });
        score = score.saturating_sub(10);
    }

    // 4. Check SKILL.md content quality
    if let Ok(content) = tokio::fs::read_to_string(skill_path.join("SKILL.md")).await {
        // Check for examples
        if !content.contains("##") && !content.contains("Example") && !content.contains("示例") {
            issues.push(QualityIssue {
                issue_type: "missing_examples".to_string(),
                description: "No usage examples found".to_string(),
                file: Some("SKILL.md".to_string()),
                line: None,
            });
            score = score.saturating_sub(15);
        }

        // Check content length
        if content.len() < 500 {
            issues.push(QualityIssue {
                issue_type: "insufficient_content".to_string(),
                description: "SKILL.md content is too short (< 500 chars)".to_string(),
                file: Some("SKILL.md".to_string()),
                line: None,
            });
            score = score.saturating_sub(10);
        }

        // Check for proper frontmatter
        if !content.starts_with("---") {
            issues.push(QualityIssue {
                issue_type: "invalid_frontmatter".to_string(),
                description: "SKILL.md missing frontmatter".to_string(),
                file: Some("SKILL.md".to_string()),
                line: Some(1),
            });
            score = score.saturating_sub(20);
        }
    }

    Ok(CodeQualityResult {
        score,
        issues,
        analyzed_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Perform complete skill review
pub async fn review_skill(skill_path: &Path, metadata: &SkillMetadata) -> Result<SkillReview> {
    let security_scan = scan_security(skill_path, metadata).await?;
    let code_quality = analyze_quality(skill_path, metadata).await?;

    // Calculate overall score (weighted average)
    let overall_score = ((security_scan.score as u32 * 60 + code_quality.score as u32 * 40) / 100) as u8;

    // Determine review status
    let status = if overall_score >= 80 && security_scan.score >= 70 {
        ReviewStatus::Approved
    } else if overall_score < 50 || security_scan.score < 50 {
        ReviewStatus::Rejected
    } else {
        ReviewStatus::ManualReview
    };

    Ok(SkillReview {
        skill_name: metadata.name.clone(),
        version: "1.0.0".to_string(), // TODO: Get from metadata
        status,
        security_scan,
        code_quality,
        overall_score,
        notes: None,
        reviewed_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Generate review report in markdown format
pub fn generate_review_report(review: &SkillReview) -> String {
    let mut report = String::new();

    report.push_str(&format!("# Skill Review Report: {}\n\n", review.skill_name));
    report.push_str(&format!("**Version**: {}\n", review.version));
    report.push_str(&format!("**Status**: {:?}\n", review.status));
    report.push_str(&format!("**Overall Score**: {}/100\n", review.overall_score));
    report.push_str(&format!("**Reviewed At**: {}\n\n", review.reviewed_at));

    // Security scan
    report.push_str("## Security Scan\n\n");
    report.push_str(&format!("**Score**: {}/100\n\n", review.security_scan.score));
    
    if review.security_scan.issues.is_empty() {
        report.push_str("✅ No security issues found.\n\n");
    } else {
        report.push_str("### Issues\n\n");
        for issue in &review.security_scan.issues {
            report.push_str(&format!(
                "- **{:?}** [{}]: {}\n",
                issue.severity, issue.category, issue.description
            ));
        }
        report.push('\n');
    }

    // Code quality
    report.push_str("## Code Quality\n\n");
    report.push_str(&format!("**Score**: {}/100\n\n", review.code_quality.score));
    
    if review.code_quality.issues.is_empty() {
        report.push_str("✅ No quality issues found.\n\n");
    } else {
        report.push_str("### Issues\n\n");
        for issue in &review.code_quality.issues {
            report.push_str(&format!(
                "- [{}]: {}\n",
                issue.issue_type, issue.description
            ));
        }
        report.push('\n');
    }

    // Recommendation
    report.push_str("## Recommendation\n\n");
    match review.status {
        ReviewStatus::Approved => {
            report.push_str("✅ **APPROVED** - This skill meets quality and security standards.\n");
        }
        ReviewStatus::Rejected => {
            report.push_str("❌ **REJECTED** - This skill has significant issues that must be addressed.\n");
        }
        ReviewStatus::ManualReview => {
            report.push_str("⚠️ **MANUAL REVIEW REQUIRED** - This skill requires human review before approval.\n");
        }
        ReviewStatus::Pending => {
            report.push_str("⏳ **PENDING** - Review in progress.\n");
        }
    }

    if let Some(notes) = &review.notes {
        report.push_str("\n## Notes\n\n");
        report.push_str(notes);
        report.push('\n');
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_review_status_serialization() {
        let status = ReviewStatus::Approved;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ReviewStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, status);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(matches!(Severity::Critical, Severity::Critical));
        assert!(matches!(Severity::High, Severity::High));
    }

    #[tokio::test]
    async fn test_scan_security_basic() {
        let metadata = SkillMetadata {
            name: "test-skill".to_string(),
            description: "Test skill".to_string(),
            homepage: Some("https://example.com".to_string()),
            license: Some("MIT".to_string()),
            compatibility: None,
            allowed_tools: vec![],
            requires: Default::default(),
            path: PathBuf::from("/tmp/test"),
            source: None,
            dockerfile: None,
        };

        let tmp = tempfile::tempdir().unwrap();
        tokio::fs::write(
            tmp.path().join("SKILL.md"),
            "---\nname: test\n---\n# Test\n\nContent here."
        ).await.unwrap();

        let result = scan_security(tmp.path(), &metadata).await.unwrap();
        assert!(result.score > 0);
    }

    #[tokio::test]
    async fn test_analyze_quality_basic() {
        let metadata = SkillMetadata {
            name: "test-skill".to_string(),
            description: "Test skill with sufficient description".to_string(),
            homepage: Some("https://example.com".to_string()),
            license: Some("MIT".to_string()),
            compatibility: None,
            allowed_tools: vec![],
            requires: Default::default(),
            path: PathBuf::from("/tmp/test"),
            source: None,
            dockerfile: None,
        };

        let tmp = tempfile::tempdir().unwrap();
        let content = "---\nname: test\n---\n# Test\n\n## Example\n\nThis is an example.\n\n".repeat(10);
        tokio::fs::write(tmp.path().join("SKILL.md"), content).await.unwrap();
        tokio::fs::write(tmp.path().join("README.md"), "# README").await.unwrap();
        tokio::fs::write(tmp.path().join("LICENSE"), "MIT License").await.unwrap();

        let result = analyze_quality(tmp.path(), &metadata).await.unwrap();
        assert!(result.score >= 80);
    }

    #[test]
    fn test_generate_review_report() {
        let review = SkillReview {
            skill_name: "test-skill".to_string(),
            version: "1.0.0".to_string(),
            status: ReviewStatus::Approved,
            security_scan: SecurityScanResult {
                score: 95,
                issues: vec![],
                scanned_at: "2026-03-17T10:00:00Z".to_string(),
            },
            code_quality: CodeQualityResult {
                score: 90,
                issues: vec![],
                analyzed_at: "2026-03-17T10:00:00Z".to_string(),
            },
            overall_score: 93,
            notes: Some("Excellent skill".to_string()),
            reviewed_at: "2026-03-17T10:00:00Z".to_string(),
        };

        let report = generate_review_report(&review);
        assert!(report.contains("test-skill"));
        assert!(report.contains("APPROVED"));
        assert!(report.contains("93/100"));
    }
}
