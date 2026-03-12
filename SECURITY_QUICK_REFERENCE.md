# Security Quick Reference Card

**For**: Developers working on ClawMaster  
**Purpose**: Quick lookup for security best practices  
**Standard**: Aerospace-grade (DO-178C Level A)

---

## 🚫 NEVER Do This

```rust
// ❌ FORBIDDEN: Unwrap in production code
let value = some_option.unwrap();
let result = some_result.expect("error");

// ✅ CORRECT: Proper error handling
let value = some_option.ok_or_else(|| anyhow::anyhow!("value missing"))?;
let result = some_result.map_err(|e| anyhow::anyhow!("operation failed: {e}"))?;
```

```rust
// ❌ FORBIDDEN: block_on in async context
async fn my_function() {
    let result = runtime.block_on(async_operation());  // DEADLOCK RISK!
}

// ✅ CORRECT: Use await
async fn my_function() {
    let result = async_operation().await?;
}
```

```rust
// ❌ FORBIDDEN: Hardcoded secrets
const API_KEY: &str = "sk-1234567890";

// ✅ CORRECT: Environment variables or vault
let api_key = std::env::var("API_KEY")?;
let api_key = vault.decrypt_string(&encrypted_key, "api_key").await?;
```

```rust
// ❌ FORBIDDEN: String-based validation
if url.contains("localhost") { /* unsafe */ }

// ✅ CORRECT: Type-based validation
let parsed = url::Url::parse(url)?;
if parsed.host_str() == Some("localhost") { /* safe */ }
```

---

## ✅ ALWAYS Do This

### 1. Input Validation

```rust
/// Validate user input with whitelist approach.
fn validate_package_name(name: &str) -> Result<(), ValidationError> {
    // Length check (DoS prevention)
    if name.is_empty() || name.len() > 256 {
        return Err(ValidationError::InvalidLength);
    }
    
    // Whitelist characters only
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '+')) {
        return Err(ValidationError::InvalidCharacters);
    }
    
    Ok(())
}
```

### 2. Error Handling

```rust
/// Proper error propagation with context.
pub async fn fetch_data(url: &str) -> anyhow::Result<Data> {
    let response = reqwest::get(url)
        .await
        .context("Failed to fetch URL")?;
    
    let data = response.json::<Data>()
        .await
        .context("Failed to parse response")?;
    
    Ok(data)
}
```

### 3. Secret Handling

```rust
use secrecy::{Secret, ExposeSecret};

/// Store secrets in Secret<T> wrapper.
pub struct Config {
    pub api_key: Secret<String>,
}

impl Config {
    pub fn use_api_key(&self) {
        // Only expose when needed
        let key = self.api_key.expose_secret();
        make_api_call(key);
        // key is dropped here
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}
```

### 4. Logging

```rust
use tracing::{info, warn, error};

/// Log security events with structured data.
pub fn log_auth_attempt(ip: IpAddr, success: bool) {
    if success {
        info!(
            event = "auth_success",
            remote_ip = %ip,
            "Authentication successful"
        );
    } else {
        warn!(
            event = "auth_failed",
            remote_ip = %ip,
            "Authentication failed - potential attack"
        );
    }
}
```

### 5. Resource Limits

```rust
/// Always enforce bounded resource usage.
pub async fn create_session(&self, config: &AuthConfig) -> anyhow::Result<String> {
    // Check limit
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sessions WHERE expires_at > datetime('now')"
    ).fetch_one(&self.pool).await?;
    
    if count >= config.max_sessions as i64 {
        // Clean up oldest
        self.cleanup_oldest_session().await?;
    }
    
    // Create new session
    self.insert_session().await
}
```

---

## 🔒 Security Checklist

### Before Committing Code

- [ ] No `unwrap()` or `expect()` in production code
- [ ] All user input validated with whitelist approach
- [ ] All secrets use `secrecy::Secret<T>`
- [ ] All errors properly propagated with `?`
- [ ] All async operations have timeouts
- [ ] All resources have bounded limits
- [ ] Security events logged with structured data
- [ ] Tests cover security edge cases

### Before Merging PR

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes
- [ ] No new security warnings from `cargo audit`
- [ ] Security-critical changes reviewed by security team
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

### Before Deploying

- [ ] All critical security fixes applied
- [ ] Rate limiting enabled
- [ ] TLS enabled with valid certificate
- [ ] Session cookies have Secure flag
- [ ] Security headers configured
- [ ] Audit logging enabled
- [ ] Monitoring and alerting configured
- [ ] Backup and recovery tested

---

## 🛡️ Common Vulnerabilities & Fixes

### SQL Injection

```rust
// ❌ VULNERABLE: String concatenation
let query = format!("SELECT * FROM users WHERE id = {}", user_id);

// ✅ SAFE: Parameterized queries
let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(user_id)
    .fetch_one(&pool)
    .await?;
```

### XSS (Cross-Site Scripting)

```rust
// ❌ VULNERABLE: Raw HTML output
format!("<div>{}</div>", user_input)

// ✅ SAFE: HTML escaping
use html_escape::encode_text;
format!("<div>{}</div>", encode_text(user_input))
```

### CSRF (Cross-Site Request Forgery)

```rust
// ✅ SAFE: SameSite=Strict cookie
let cookie = format!(
    "session={token}; HttpOnly; SameSite=Strict; Secure; Path=/"
);
```

### SSRF (Server-Side Request Forgery)

```rust
// ✅ SAFE: Validate before fetching
use crate::ssrf::ssrf_check;

pub async fn fetch_url(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url)?;
    ssrf_check(&parsed, &allowlist).await?;
    
    let response = reqwest::get(url).await?;
    Ok(response.text().await?)
}
```

### Path Traversal

```rust
// ❌ VULNERABLE: Direct path usage
let path = format!("/data/{}", user_filename);

// ✅ SAFE: Canonicalize and validate
use std::path::{Path, PathBuf};

pub fn safe_path(base: &Path, user_input: &str) -> Result<PathBuf> {
    let requested = base.join(user_input);
    let canonical = requested.canonicalize()?;
    
    if !canonical.starts_with(base) {
        return Err(anyhow::anyhow!("Path traversal detected"));
    }
    
    Ok(canonical)
}
```

---

## 📊 Security Metrics

### What to Monitor

```rust
// Authentication metrics
metrics::counter!("auth.attempts.total", 1, "result" => "success");
metrics::counter!("auth.attempts.total", 1, "result" => "failure");
metrics::histogram!("auth.duration_ms", duration.as_millis() as f64);

// Rate limiting metrics
metrics::counter!("rate_limit.hits.total", 1, "endpoint" => "login");

// SSRF metrics
metrics::counter!("ssrf.blocks.total", 1, "reason" => "private_ip");

// Session metrics
metrics::gauge!("sessions.active", active_count as f64);
metrics::histogram!("session.duration_seconds", duration.as_secs() as f64);
```

### Alert Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Failed auth attempts | > 10/min | > 50/min |
| Rate limit hits | > 100/min | > 500/min |
| SSRF blocks | > 5/min | > 20/min |
| Active sessions | > 1000 | > 5000 |

---

## 🔧 Testing Security

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rejects_invalid_input() {
        assert!(validate_package_name("").is_err());
        assert!(validate_package_name("../etc/passwd").is_err());
        assert!(validate_package_name("curl;ls").is_err());
    }
    
    #[tokio::test]
    async fn test_ssrf_blocks_private_ip() {
        let url = Url::parse("http://192.168.1.1/admin").unwrap();
        assert!(ssrf_check(&url, &[]).await.is_err());
    }
    
    #[tokio::test]
    async fn test_rate_limit_enforced() {
        let limiter = RateLimiter::new();
        let ip = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        
        // Exhaust limit
        for _ in 0..5 {
            assert!(limiter.check(ip).await.is_ok());
        }
        
        // Should be blocked
        assert!(limiter.check(ip).await.is_err());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_session_security() {
    let app = test_app().await;
    
    // Login
    let response = app.post("/api/auth/login")
        .json(&json!({"password": "test"}))
        .send()
        .await;
    
    // Verify cookie security
    let cookie = response.headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();
    
    assert!(cookie.contains("HttpOnly"));
    assert!(cookie.contains("Secure"));
    assert!(cookie.contains("SameSite=Strict"));
}
```

---

## 📚 References

### Standards
- **DO-178C**: Software Considerations in Airborne Systems
- **OWASP Top 10**: https://owasp.org/Top10/
- **CWE Top 25**: https://cwe.mitre.org/top25/

### Rust Security
- **Rust Security Guidelines**: https://anssi-fr.github.io/rust-guide/
- **Cargo Audit**: https://github.com/rustsec/rustsec
- **Clippy Security Lints**: https://rust-lang.github.io/rust-clippy/

### Internal Docs
- `AEROSPACE_AUDIT_FIXES.md` - Full audit report
- `IMPLEMENTATION_GUIDE.md` - Implementation steps
- `AEROSPACE_AUDIT_SUMMARY.md` - Executive summary

---

## 🆘 Emergency Contacts

**Security Issue?**
1. Create GitHub issue with `[SECURITY]` prefix
2. Email: security@clawmaster.ai
3. Slack: #security-incidents

**Critical Vulnerability?**
1. Email: security-emergency@clawmaster.ai
2. Response time: < 2 hours
3. Escalation: CTO direct line

---

**Last Updated**: 2026-03-11  
**Version**: 1.0  
**Maintainer**: Security Team
