//! Configuration Validator
//!
//! DO-178C Level A Compliant Configuration Validator

use {
    crate::{
        ConflictDetectionRule, NetworkSecurityRule, PathPermissionRule, ResourceLimitsRule,
        SecurityBaselineRule, Severity, ValidationIssue, ValidationRule,
    },
    clawmaster_config::MoltisConfig,
    std::sync::Arc,
    tracing::{debug, error, info, warn},
};

/// Configuration validator
///
/// DO-178C §11.13: Systematic configuration validation
pub struct ConfigValidator {
    rules: Vec<Arc<dyn ValidationRule>>,
    fail_on_critical: bool,
    fail_on_error: bool,
}

impl ConfigValidator {
    /// Create a new validator with default rules
    pub fn new() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
            fail_on_critical: true,
            fail_on_error: false,
        };

        // Register default rules
        validator.register_default_rules();
        validator
    }

    /// Create a strict validator (fails on errors)
    pub fn strict() -> Self {
        let mut validator = Self::new();
        validator.fail_on_error = true;
        validator
    }

    /// Register default validation rules
    fn register_default_rules(&mut self) {
        self.register(Arc::new(SecurityBaselineRule));
        self.register(Arc::new(ResourceLimitsRule));
        self.register(Arc::new(PathPermissionRule));
        self.register(Arc::new(ConflictDetectionRule));
        self.register(Arc::new(NetworkSecurityRule));
    }

    /// Register a validation rule
    pub fn register(&mut self, rule: Arc<dyn ValidationRule>) {
        debug!("Registering validation rule: {}", rule.name());
        self.rules.push(rule);
    }

    /// Validate configuration
    ///
    /// DO-178C §11.13: Comprehensive validation
    pub fn validate(&self, config: &MoltisConfig) -> ValidationReport {
        info!(
            "Starting configuration validation with {} rules",
            self.rules.len()
        );

        let mut all_issues = Vec::new();

        for rule in &self.rules {
            debug!("Running validation rule: {}", rule.name());
            let issues = rule.validate(config);

            for issue in &issues {
                match issue.severity {
                    Severity::Critical => {
                        error!(
                            rule = rule.name(),
                            field = issue.field,
                            message = issue.message,
                            "Critical validation issue"
                        );
                    },
                    Severity::Error => {
                        error!(
                            rule = rule.name(),
                            field = issue.field,
                            message = issue.message,
                            "Validation error"
                        );
                    },
                    Severity::Warning => {
                        warn!(
                            rule = rule.name(),
                            field = issue.field,
                            message = issue.message,
                            "Validation warning"
                        );
                    },
                    Severity::Info => {
                        info!(
                            rule = rule.name(),
                            field = issue.field,
                            message = issue.message,
                            "Validation info"
                        );
                    },
                }
            }

            all_issues.extend(issues);
        }

        let report = ValidationReport::new(all_issues);

        info!(
            "Validation complete: {} critical, {} errors, {} warnings, {} info",
            report.critical_count(),
            report.error_count(),
            report.warning_count(),
            report.info_count()
        );

        report
    }

    /// Validate and fail if issues found
    ///
    /// DO-178C §11.13: Fail-fast validation
    pub fn validate_or_fail(&self, config: &MoltisConfig) -> anyhow::Result<()> {
        let report = self.validate(config);

        if self.fail_on_critical && report.has_critical() {
            anyhow::bail!(
                "Configuration validation failed with {} critical issue(s):\n{}",
                report.critical_count(),
                report.format_issues(Severity::Critical)
            );
        }

        if self.fail_on_error && report.has_errors() {
            anyhow::bail!(
                "Configuration validation failed with {} error(s):\n{}",
                report.error_count(),
                report.format_issues(Severity::Error)
            );
        }

        if report.has_warnings() {
            warn!(
                "Configuration has {} warning(s):\n{}",
                report.warning_count(),
                report.format_issues(Severity::Warning)
            );
        }

        Ok(())
    }

    /// Get number of registered rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation report
///
/// DO-178C §11.13: Detailed validation reporting
#[derive(Debug, Clone)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Create a new validation report
    pub fn new(issues: Vec<ValidationIssue>) -> Self {
        Self { issues }
    }

    /// Get all issues
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Check if validation passed (no critical or error issues)
    pub fn is_valid(&self) -> bool {
        !self.has_critical() && !self.has_errors()
    }

    /// Check if there are critical issues
    pub fn has_critical(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Critical)
    }

    /// Check if there are error issues
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    /// Check if there are warning issues
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    /// Get count of critical issues
    pub fn critical_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Critical)
            .count()
    }

    /// Get count of error issues
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    /// Get count of warning issues
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }

    /// Get count of info issues
    pub fn info_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Info)
            .count()
    }

    /// Get issues by severity
    pub fn issues_by_severity(&self, severity: Severity) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == severity)
            .collect()
    }

    /// Format issues for display
    pub fn format_issues(&self, min_severity: Severity) -> String {
        let mut output = String::new();

        for issue in &self.issues {
            if issue.severity >= min_severity {
                output.push_str(&format!(
                    "[{:?}] {}: {}\n",
                    issue.severity, issue.field, issue.message
                ));

                if let Some(suggestion) = &issue.suggestion {
                    output.push_str(&format!("  Suggestion: {}\n", suggestion));
                }
            }
        }

        output
    }

    /// Format all issues
    pub fn format_all(&self) -> String {
        self.format_issues(Severity::Info)
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Validation Summary: {} critical, {} errors, {} warnings, {} info",
            self.critical_count(),
            self.error_count(),
            self.warning_count(),
            self.info_count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> MoltisConfig {
        MoltisConfig::default()
    }

    #[test]
    fn test_validator_creation() {
        let validator = ConfigValidator::new();
        assert_eq!(validator.rule_count(), 5); // 5 default rules
    }

    #[test]
    fn test_validator_strict() {
        let validator = ConfigValidator::strict();
        assert!(validator.fail_on_error);
        assert!(validator.fail_on_critical);
    }

    #[test]
    fn test_validate_default_config() {
        let validator = ConfigValidator::new();
        let config = create_test_config();
        let report = validator.validate(&config);

        // Default config should be valid
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_default_config_passes() {
        let validator = ConfigValidator::new();
        let config = create_test_config();

        let report = validator.validate(&config);

        // Default config should pass (simplified rules return no issues)
        assert!(report.is_valid());
        assert!(!report.has_critical());
        assert_eq!(report.critical_count(), 0);
    }

    #[test]
    fn test_validate_or_fail_passes_on_valid_config() {
        let validator = ConfigValidator::new();
        let config = create_test_config();

        let result = validator.validate_or_fail(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_report_summary() {
        let issues = vec![
            ValidationIssue::critical("test1", "critical issue"),
            ValidationIssue::error("test2", "error issue"),
            ValidationIssue::warning("test3", "warning issue"),
            ValidationIssue::info("test4", "info issue"),
        ];

        let report = ValidationReport::new(issues);

        assert_eq!(report.critical_count(), 1);
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.info_count(), 1);
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validation_report_format() {
        let issues =
            vec![ValidationIssue::critical("test", "test message").with_suggestion("fix this")];

        let report = ValidationReport::new(issues);
        let formatted = report.format_all();

        assert!(formatted.contains("Critical"));
        assert!(formatted.contains("test message"));
        assert!(formatted.contains("Suggestion"));
    }

    #[test]
    fn test_validation_report_is_valid() {
        let report = ValidationReport::new(vec![]);
        assert!(report.is_valid());

        let report_with_warning =
            ValidationReport::new(vec![ValidationIssue::warning("test", "warning")]);
        assert!(report_with_warning.is_valid()); // Warnings don't fail validation

        let report_with_error =
            ValidationReport::new(vec![ValidationIssue::error("test", "error")]);
        assert!(!report_with_error.is_valid());
    }
}
