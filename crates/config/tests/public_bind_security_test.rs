//! Tests for public bind address security validation

use clawmaster_config::{MoltisConfig, Severity, validate_toml_str};

#[test]
fn test_default_bind_is_localhost() {
    let config = MoltisConfig::default();
    assert_eq!(config.server.bind, "127.0.0.1");
    assert_eq!(config.server.allow_public_bind, false);
}

#[test]
fn test_localhost_bind_allowed() {
    let toml = r#"
[server]
bind = "127.0.0.1"
port = 8080
"#;
    let result = validate_toml_str(toml);
    assert!(
        result.diagnostics.is_empty(),
        "Localhost bind should not produce errors"
    );
}

#[test]
fn test_public_bind_without_permission_rejected() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080
"#;
    let result = validate_toml_str(toml);

    // Should have an error
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();

    assert!(
        !errors.is_empty(),
        "Should have error for public bind without permission"
    );

    let error = errors
        .iter()
        .find(|d| d.path == "server.bind")
        .expect("Should have error on server.bind");

    assert!(error.message.contains("not allowed"));
    assert!(error.message.contains("allow_public_bind"));
}

#[test]
fn test_public_bind_with_explicit_permission_allowed() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080
allow_public_bind = true
"#;
    let result = validate_toml_str(toml);

    // Should not have errors, but should have warning
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();

    if !errors.is_empty() {
        eprintln!("Unexpected errors:");
        for err in &errors {
            eprintln!("  - {}: {}", err.path, err.message);
        }
    }

    assert!(
        errors.is_empty(),
        "Should not have errors with explicit permission"
    );

    let warnings: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();

    assert!(!warnings.is_empty(), "Should have warning for public bind");

    let warning = warnings
        .iter()
        .find(|d| d.message.contains("public access explicitly allowed"))
        .expect("Should have warning about public access");

    assert!(warning.message.contains("authentication"));
    assert!(warning.message.contains("TLS"));
}

#[test]
fn test_public_bind_with_tailscale_serve_allowed() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080

[tailscale]
mode = "serve"
"#;
    let result = validate_toml_str(toml);

    // Should not have errors when tunnel is configured
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error && d.path == "server.bind")
        .collect();

    assert!(
        errors.is_empty(),
        "Should not have errors with Tailscale serve mode"
    );

    // Should have info message about tunnel
    let infos: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .collect();

    let tunnel_info = infos.iter().find(|d| d.message.contains("Tailscale"));

    assert!(tunnel_info.is_some(), "Should have info about Tailscale");
}

#[test]
fn test_public_bind_with_tailscale_funnel_allowed() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080

[tailscale]
mode = "funnel"
"#;
    let result = validate_toml_str(toml);

    // Should not have errors when funnel is configured
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error && d.path == "server.bind")
        .collect();

    assert!(
        errors.is_empty(),
        "Should not have errors with Tailscale funnel mode"
    );
}

#[test]
fn test_ipv6_public_bind_rejected() {
    let toml = r#"
[server]
bind = "::"
port = 8080
"#;
    let result = validate_toml_str(toml);

    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error && d.path == "server.bind")
        .collect();

    assert!(
        !errors.is_empty(),
        "Should reject IPv6 public bind without permission"
    );
}

#[test]
fn test_ipv6_localhost_allowed() {
    let toml = r#"
[server]
bind = "::1"
port = 8080
"#;
    let result = validate_toml_str(toml);

    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();

    assert!(errors.is_empty(), "IPv6 localhost should be allowed");
}

#[test]
fn test_localhost_string_allowed() {
    let toml = r#"
[server]
bind = "localhost"
port = 8080
"#;
    let result = validate_toml_str(toml);

    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();

    assert!(errors.is_empty(), "localhost string should be allowed");
}

#[test]
fn test_error_message_provides_solutions() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080
"#;
    let result = validate_toml_str(toml);

    let error = result
        .diagnostics
        .iter()
        .find(|d| d.severity == Severity::Error && d.path == "server.bind")
        .expect("Should have error");

    // Check that error message provides helpful solutions
    assert!(
        error.message.contains("127.0.0.1"),
        "Should mention localhost option"
    );
    assert!(
        error.message.contains("Tailscale"),
        "Should mention Tailscale option"
    );
    assert!(
        error.message.contains("allow_public_bind"),
        "Should mention explicit permission option"
    );
}

#[test]
fn test_public_bind_with_disabled_auth_double_warning() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080
allow_public_bind = true

[auth]
disabled = true
"#;
    let result = validate_toml_str(toml);

    let warnings: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();

    // Should have warning about public bind AND warning about disabled auth
    assert!(
        warnings.len() >= 2,
        "Should have multiple security warnings"
    );

    let has_public_bind_warning = warnings
        .iter()
        .any(|w| w.message.contains("public access explicitly allowed"));
    let has_auth_warning = warnings
        .iter()
        .any(|w| w.message.contains("authentication is disabled"));

    assert!(has_public_bind_warning, "Should warn about public bind");
    assert!(has_auth_warning, "Should warn about disabled auth");
}

#[test]
fn test_public_bind_with_disabled_tls_double_warning() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080
allow_public_bind = true

[tls]
enabled = false
"#;
    let result = validate_toml_str(toml);

    let warnings: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .collect();

    // Should have warning about public bind AND warning about disabled TLS
    assert!(
        warnings.len() >= 2,
        "Should have multiple security warnings"
    );

    let has_tls_warning = warnings
        .iter()
        .any(|w| w.message.contains("TLS is disabled"));

    assert!(has_tls_warning, "Should warn about disabled TLS");
}

#[test]
fn test_tailscale_off_mode_not_considered_tunnel() {
    let toml = r#"
[server]
bind = "0.0.0.0"
port = 8080

[tailscale]
mode = "off"
"#;
    let result = validate_toml_str(toml);

    // Should still have error because Tailscale is off
    let errors: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error && d.path == "server.bind")
        .collect();

    assert!(
        !errors.is_empty(),
        "Should reject when Tailscale mode is off"
    );
}
