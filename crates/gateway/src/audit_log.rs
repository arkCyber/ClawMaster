//! Security audit logging for compliance and forensics.
//! 
//! Aerospace Standard: All security-critical events must be logged with
//! sufficient detail for forensic analysis and compliance auditing.
//! 
//! Log Format: Structured JSON logs with consistent fields:
//! - timestamp (ISO 8601)
//! - event (event type identifier)
//! - severity (INFO, WARN, ERROR, CRITICAL)
//! - actor (IP address, user ID, API key ID)
//! - action (what was attempted)
//! - result (success, failure, blocked)
//! - metadata (additional context)

use std::net::IpAddr;
use tracing::{info, warn, error};

/// Authentication method used.
#[derive(Debug, Clone, Copy)]
pub enum AuthMethod {
    Password,
    Passkey,
    ApiKey,
    Session,
    Local,
}

impl std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthMethod::Password => write!(f, "password"),
            AuthMethod::Passkey => write!(f, "passkey"),
            AuthMethod::ApiKey => write!(f, "api_key"),
            AuthMethod::Session => write!(f, "session"),
            AuthMethod::Local => write!(f, "local"),
        }
    }
}

/// Log a successful authentication event.
/// 
/// Aerospace Standard: All successful authentications must be logged for audit trail.
/// 
/// # Example
/// ```rust
/// audit_log::log_auth_success(
///     AuthMethod::Password,
///     client_ip,
///     "/api/chat/send",
///     Some("user@example.com")
/// );
/// ```
pub fn log_auth_success(
    method: AuthMethod,
    ip: IpAddr,
    path: &str,
    identity: Option<&str>,
) {
    info!(
        event = "auth_success",
        method = %method,
        remote_ip = %ip,
        path = path,
        identity = identity,
        "Authentication successful"
    );
}

/// Log a failed authentication attempt.
/// 
/// Aerospace Standard: Failed auth attempts indicate potential attacks and must be logged.
/// 
/// # Example
/// ```rust
/// audit_log::log_auth_failure(
///     client_ip,
///     "/api/auth/login",
///     "invalid_password",
///     Some("user@example.com")
/// );
/// ```
pub fn log_auth_failure(
    ip: IpAddr,
    path: &str,
    reason: &str,
    attempted_identity: Option<&str>,
) {
    warn!(
        event = "auth_failed",
        remote_ip = %ip,
        path = path,
        reason = reason,
        attempted_identity = attempted_identity,
        "Authentication failed - potential attack"
    );
}

/// Log a session creation event.
/// 
/// # Example
/// ```rust
/// audit_log::log_session_created(client_ip, "30d", AuthMethod::Password);
/// ```
pub fn log_session_created(ip: IpAddr, expiry: &str, method: AuthMethod) {
    info!(
        event = "session_created",
        remote_ip = %ip,
        expiry = expiry,
        auth_method = %method,
        "New session created"
    );
}

/// Log a session invalidation event.
/// 
/// # Example
/// ```rust
/// audit_log::log_session_invalidated("password_change", 5);
/// ```
pub fn log_session_invalidated(reason: &str, count: usize) {
    info!(
        event = "session_invalidated",
        reason = reason,
        session_count = count,
        "Sessions invalidated"
    );
}

/// Log an SSRF block event.
/// 
/// Aerospace Standard: SSRF attempts indicate potential attacks and must be logged.
/// 
/// # Example
/// ```rust
/// audit_log::log_ssrf_block(
///     "http://192.168.1.1/admin",
///     "192.168.1.1",
///     "private_ip",
///     client_ip
/// );
/// ```
pub fn log_ssrf_block(url: &str, resolved_ip: &str, reason: &str, client_ip: IpAddr) {
    warn!(
        event = "ssrf_blocked",
        url = url,
        resolved_ip = resolved_ip,
        reason = reason,
        remote_ip = %client_ip,
        "SSRF attack blocked"
    );
}

/// Log a rate limit event.
/// 
/// Aerospace Standard: Rate limit hits indicate potential brute force attacks.
/// 
/// # Example
/// ```rust
/// audit_log::log_rate_limit(client_ip, "login", 5, 60);
/// ```
pub fn log_rate_limit(ip: IpAddr, endpoint: &str, limit: u32, window_secs: u64) {
    warn!(
        event = "rate_limit_exceeded",
        remote_ip = %ip,
        endpoint = endpoint,
        limit = limit,
        window_seconds = window_secs,
        "Rate limit exceeded - potential brute force"
    );
}

/// Log a WebSocket connection event.
/// 
/// # Example
/// ```rust
/// audit_log::log_websocket_connect(client_ip, true, Some("session_token"));
/// ```
pub fn log_websocket_connect(ip: IpAddr, authenticated: bool, auth_method: Option<&str>) {
    info!(
        event = "websocket_connected",
        remote_ip = %ip,
        authenticated = authenticated,
        auth_method = auth_method,
        "WebSocket connection established"
    );
}

/// Log a WebSocket disconnection event.
/// 
/// # Example
/// ```rust
/// audit_log::log_websocket_disconnect(client_ip, "client_closed", 3600);
/// ```
pub fn log_websocket_disconnect(ip: IpAddr, reason: &str, duration_secs: u64) {
    info!(
        event = "websocket_disconnected",
        remote_ip = %ip,
        reason = reason,
        duration_seconds = duration_secs,
        "WebSocket connection closed"
    );
}

/// Log a password change event.
/// 
/// Aerospace Standard: Password changes are security-critical events.
/// 
/// # Example
/// ```rust
/// audit_log::log_password_changed(client_ip, true);
/// ```
pub fn log_password_changed(ip: IpAddr, sessions_invalidated: bool) {
    info!(
        event = "password_changed",
        remote_ip = %ip,
        sessions_invalidated = sessions_invalidated,
        "Password changed successfully"
    );
}

/// Log a passkey registration event.
/// 
/// # Example
/// ```rust
/// audit_log::log_passkey_registered(client_ip, "YubiKey 5");
/// ```
pub fn log_passkey_registered(ip: IpAddr, device_name: &str) {
    info!(
        event = "passkey_registered",
        remote_ip = %ip,
        device_name = device_name,
        "Passkey registered successfully"
    );
}

/// Log an API key creation event.
/// 
/// # Example
/// ```rust
/// audit_log::log_api_key_created(client_ip, "operator.admin", Some("Production API"));
/// ```
pub fn log_api_key_created(ip: IpAddr, scope: &str, label: Option<&str>) {
    info!(
        event = "api_key_created",
        remote_ip = %ip,
        scope = scope,
        label = label,
        "API key created"
    );
}

/// Log an API key revocation event.
/// 
/// # Example
/// ```rust
/// audit_log::log_api_key_revoked(client_ip, "key_id_123", "compromised");
/// ```
pub fn log_api_key_revoked(ip: IpAddr, key_id: &str, reason: &str) {
    warn!(
        event = "api_key_revoked",
        remote_ip = %ip,
        key_id = key_id,
        reason = reason,
        "API key revoked"
    );
}

/// Log a vault seal/unseal event.
/// 
/// Aerospace Standard: Vault operations are security-critical.
/// 
/// # Example
/// ```rust
/// audit_log::log_vault_unsealed(client_ip, AuthMethod::Password);
/// ```
pub fn log_vault_unsealed(ip: IpAddr, method: AuthMethod) {
    info!(
        event = "vault_unsealed",
        remote_ip = %ip,
        auth_method = %method,
        "Vault unsealed successfully"
    );
}

/// Log a vault seal event.
/// 
/// # Example
/// ```rust
/// audit_log::log_vault_sealed("manual");
/// ```
pub fn log_vault_sealed(reason: &str) {
    info!(
        event = "vault_sealed",
        reason = reason,
        "Vault sealed"
    );
}

/// Log a critical security event that requires immediate attention.
/// 
/// Aerospace Standard: Critical events trigger alerts.
/// 
/// # Example
/// ```rust
/// audit_log::log_security_critical(
///     "multiple_failed_auth",
///     "10 failed login attempts from same IP in 1 minute",
///     client_ip
/// );
/// ```
pub fn log_security_critical(event_type: &str, description: &str, ip: IpAddr) {
    error!(
        event = "security_critical",
        event_type = event_type,
        description = description,
        remote_ip = %ip,
        "CRITICAL SECURITY EVENT - immediate attention required"
    );
}

/// Log a configuration change event.
/// 
/// # Example
/// ```rust
/// audit_log::log_config_changed(client_ip, "auth.session_expiry_seconds", "2592000", "3600");
/// ```
pub fn log_config_changed(ip: IpAddr, key: &str, old_value: &str, new_value: &str) {
    info!(
        event = "config_changed",
        remote_ip = %ip,
        config_key = key,
        old_value = old_value,
        new_value = new_value,
        "Configuration changed"
    );
}

/// Log a database migration event.
/// 
/// # Example
/// ```rust
/// audit_log::log_migration_started("sessions", &["projects"]);
/// ```
pub fn log_migration_started(name: &str, dependencies: &[&str]) {
    info!(
        event = "migration_started",
        migration_name = name,
        dependencies = ?dependencies,
        "Database migration started"
    );
}

/// Log a migration completion event.
/// 
/// # Example
/// ```rust
/// audit_log::log_migration_completed("sessions", 1234);
/// ```
pub fn log_migration_completed(name: &str, duration_ms: u128) {
    info!(
        event = "migration_completed",
        migration_name = name,
        duration_ms = duration_ms,
        "Database migration completed"
    );
}

/// Log a migration failure event.
/// 
/// # Example
/// ```rust
/// audit_log::log_migration_failed("sessions", "foreign key constraint failed");
/// ```
pub fn log_migration_failed(name: &str, error: &str) {
    error!(
        event = "migration_failed",
        migration_name = name,
        error = error,
        "Database migration failed - CRITICAL"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[test]
    fn test_auth_method_display() {
        assert_eq!(AuthMethod::Password.to_string(), "password");
        assert_eq!(AuthMethod::Passkey.to_string(), "passkey");
        assert_eq!(AuthMethod::ApiKey.to_string(), "api_key");
        assert_eq!(AuthMethod::Session.to_string(), "session");
        assert_eq!(AuthMethod::Local.to_string(), "local");
    }
    
    #[test]
    fn test_log_functions_compile() {
        // These tests just verify the functions compile and don't panic
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        
        log_auth_success(AuthMethod::Password, ip, "/api/test", Some("user"));
        log_auth_failure(ip, "/api/test", "invalid", None);
        log_session_created(ip, "30d", AuthMethod::Password);
        log_session_invalidated("test", 1);
        log_ssrf_block("http://test.com", "1.2.3.4", "test", ip);
        log_rate_limit(ip, "test", 5, 60);
        log_websocket_connect(ip, true, Some("session"));
        log_websocket_disconnect(ip, "test", 100);
        log_password_changed(ip, true);
        log_passkey_registered(ip, "test");
        log_api_key_created(ip, "test", None);
        log_api_key_revoked(ip, "test", "test");
        log_vault_unsealed(ip, AuthMethod::Password);
        log_vault_sealed("test");
        log_security_critical("test", "test", ip);
        log_config_changed(ip, "test", "old", "new");
        log_migration_started("test", &[]);
        log_migration_completed("test", 100);
        log_migration_failed("test", "test");
    }
}
