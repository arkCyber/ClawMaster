# Security Policy

**Project**: ClawMaster (Rust OpenClaw)  
**Security Standard**: Aerospace-grade (DO-178C Level A)  
**Last Updated**: 2026-03-11

---

## 🔒 Security Overview

ClawMaster implements aerospace-grade security controls following DO-178C Level A standards and OWASP best practices. This document outlines our security architecture, threat model, and vulnerability reporting process.

---

## 🛡️ Security Architecture

### Authentication & Authorization

**Multi-Factor Authentication**:
- Password authentication (Argon2 hashing)
- WebAuthn passkeys (FIDO2 compliant)
- API keys with scoped permissions
- Session tokens (30-day expiry, configurable)

**Session Management**:
- Cryptographically random tokens (32 bytes)
- SHA-256 hashed storage
- Automatic expiry enforcement
- Session limit per user (default: 10)
- Session regeneration on login (prevents fixation)
- All sessions invalidated on password change

**Rate Limiting**:
- Login: 5 attempts per IP per minute
- Password reset: 3 attempts per IP per hour
- Setup: 10 attempts per IP per hour
- Token bucket algorithm with automatic cleanup

### Network Security

**SSRF Protection**:
- Blocks all private IP ranges (RFC 1918, RFC 4193, RFC 3927)
- Blocks loopback addresses (127.0.0.0/8, ::1)
- Blocks link-local addresses (169.254.0.0/16, fe80::/10)
- Blocks CGNAT range (100.64.0.0/10)
- Configurable CIDR allowlist for trusted networks
- DNS resolution before validation (prevents TOCTOU)

**WebSocket Security**:
- Same-origin validation (prevents CSWSH)
- Handles localhost variants (127.0.0.1, ::1, .localhost)
- Supports proxy mode (X-Forwarded-Host)
- Authentication required for all connections

**TLS/HTTPS**:
- Session cookies have Secure flag when TLS enabled
- HSTS header with 1-year max-age
- Certificate validation enforced
- No insecure fallback

### Data Protection

**Encryption at Rest**:
- XChaCha20-Poly1305 AEAD for vault
- Argon2 KDF for key derivation
- DEK (Data Encryption Key) wrapping
- Recovery key support
- Automatic encryption for environment variables

**Encryption in Transit**:
- TLS 1.2+ required for production
- Strong cipher suites only
- Certificate pinning supported

**Secret Management**:
- All secrets use `secrecy::Secret<T>` wrapper
- Zeroizing memory on drop
- No secrets in logs (manual Debug impl)
- Vault integration for encrypted storage

### Input Validation

**Whitelist Approach**:
- Package names: `[a-zA-Z0-9][a-zA-Z0-9.-+]*` (max 256 chars)
- Image refs: Alphanumeric + `-._:/` (max 512 chars)
- No path traversal (`..`, absolute paths blocked)
- Length limits on all inputs

**SQL Injection Prevention**:
- Parameterized queries only (sqlx)
- No string concatenation in SQL
- Type-safe query builders

**XSS Prevention**:
- HTML escaping for all user input
- Content-Security-Policy header
- X-XSS-Protection header

### Security Headers

All responses include:
- `Content-Security-Policy`: Strict CSP with frame-ancestors 'none'
- `X-Frame-Options`: DENY (prevents clickjacking)
- `X-Content-Type-Options`: nosniff (prevents MIME sniffing)
- `Referrer-Policy`: strict-origin-when-cross-origin
- `Permissions-Policy`: Restrictive (disables dangerous features)
- `Strict-Transport-Security`: max-age=31536000 (when TLS enabled)

---

## 🎯 Threat Model

### In Scope

1. **Authentication Bypass**: Unauthorized access to protected resources
2. **Session Hijacking**: Stealing or fixating user sessions
3. **SSRF Attacks**: Server-side request forgery to internal resources
4. **SQL Injection**: Malicious SQL queries via user input
5. **XSS Attacks**: Cross-site scripting via user content
6. **CSRF Attacks**: Cross-site request forgery
7. **Brute Force**: Password guessing attacks
8. **DoS Attacks**: Resource exhaustion
9. **Path Traversal**: Accessing files outside allowed directories
10. **Clickjacking**: UI redressing attacks

### Out of Scope

1. **Physical Access**: Physical access to server hardware
2. **Social Engineering**: Phishing, pretexting, etc.
3. **Insider Threats**: Malicious administrators with valid credentials
4. **Supply Chain**: Compromised dependencies (mitigated by cargo-audit)
5. **Zero-Day Exploits**: Unknown vulnerabilities in Rust/dependencies

### Assumptions

1. **TLS Termination**: TLS is properly configured at load balancer or reverse proxy
2. **Database Security**: SQLite database file has proper filesystem permissions
3. **Environment Security**: Server environment is hardened per industry standards
4. **Dependency Trust**: Crates from crates.io are trustworthy (verified by cargo-audit)

---

## 🚨 Vulnerability Reporting

### Reporting Process

**For Security Vulnerabilities**:
1. **DO NOT** create a public GitHub issue
2. Email: security@clawmaster.ai (PGP key available on request)
3. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

**Response Timeline**:
- **Acknowledgment**: Within 24 hours
- **Initial Assessment**: Within 48 hours
- **Fix Timeline**: Based on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: 90 days

**Disclosure Policy**:
- Coordinated disclosure after fix is released
- Security advisory published on GitHub
- CVE assigned for critical/high severity issues
- Credit given to reporter (unless anonymous requested)

### Severity Levels

**Critical** (CVSS 9.0-10.0):
- Remote code execution
- Authentication bypass
- Data breach affecting all users

**High** (CVSS 7.0-8.9):
- Privilege escalation
- SQL injection
- SSRF with data exfiltration

**Medium** (CVSS 4.0-6.9):
- XSS (stored)
- CSRF on sensitive operations
- Information disclosure

**Low** (CVSS 0.1-3.9):
- XSS (reflected)
- Information disclosure (non-sensitive)
- Minor configuration issues

---

## 🔍 Security Testing

### Continuous Security

**Automated Checks** (every PR):
- `cargo audit`: Known vulnerability scanning
- `cargo clippy`: Security lints enabled
- No `unwrap()`/`expect()` in production code
- No hardcoded secrets
- No SQL injection patterns
- Semgrep SAST scanning

**Manual Reviews**:
- Security-critical PRs reviewed by security team
- Quarterly security audits
- Annual penetration testing

### Testing Requirements

**Unit Tests**:
- All security functions have test coverage
- Edge cases and attack vectors tested
- Regression tests for known vulnerabilities

**Integration Tests**:
- Authentication flows
- Rate limiting behavior
- SSRF protection
- Session management
- WebSocket security

**Load Tests**:
- No deadlocks under concurrent load
- Rate limiting effectiveness
- Resource exhaustion resistance

---

## 📋 Security Checklist

### For Developers

Before committing code:
- [ ] No `unwrap()` or `expect()` in production code
- [ ] All user input validated with whitelist approach
- [ ] All secrets use `secrecy::Secret<T>`
- [ ] All errors properly propagated with `?`
- [ ] All async operations have timeouts
- [ ] All resources have bounded limits
- [ ] Security events logged with structured data
- [ ] Tests cover security edge cases

Before merging PR:
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes
- [ ] `cargo audit` shows no vulnerabilities
- [ ] Security-critical changes reviewed by security team
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

Before deploying:
- [ ] All critical security fixes applied
- [ ] Rate limiting enabled
- [ ] TLS enabled with valid certificate
- [ ] Session cookies have Secure flag
- [ ] Security headers configured
- [ ] Audit logging enabled
- [ ] Monitoring and alerting configured
- [ ] Backup and recovery tested

---

## 🔐 Cryptography

### Algorithms Used

**Password Hashing**:
- Algorithm: Argon2id
- Memory: 19 MiB
- Iterations: 2
- Parallelism: 1
- Salt: 16 bytes (random)

**Session Tokens**:
- Algorithm: Cryptographically random
- Length: 32 bytes
- Encoding: Base64
- Storage: SHA-256 hashed

**API Keys**:
- Algorithm: Cryptographically random
- Length: 32 bytes
- Encoding: Base64
- Storage: SHA-256 hashed

**Vault Encryption**:
- Algorithm: XChaCha20-Poly1305 (AEAD)
- Key Derivation: Argon2id
- Nonce: 24 bytes (random)
- AAD: Context-specific

### Key Management

**DEK (Data Encryption Key)**:
- Generated on vault initialization
- Wrapped with password-derived key
- Stored encrypted in database
- Zeroized on drop

**Recovery Key**:
- Generated on vault initialization
- Displayed once to user
- Can unwrap DEK if password lost
- Should be stored securely offline

---

## 📊 Compliance

### Standards

**DO-178C Level A**:
- ✅ Deterministic behavior
- ✅ Bounded resource usage
- ✅ Comprehensive error handling
- ✅ Extensive testing
- ✅ Traceability

**OWASP Top 10 2021**:
- ✅ A01: Broken Access Control
- ✅ A02: Cryptographic Failures
- ✅ A03: Injection
- ✅ A04: Insecure Design
- ✅ A05: Security Misconfiguration
- ✅ A06: Vulnerable and Outdated Components
- ✅ A07: Identification and Authentication Failures
- ✅ A08: Software and Data Integrity Failures
- ✅ A09: Security Logging and Monitoring Failures
- ✅ A10: Server-Side Request Forgery

**CWE Top 25**:
- Comprehensive coverage of most dangerous software weaknesses
- Regular scanning with automated tools
- Manual review of high-risk areas

---

## 🆘 Incident Response

### Security Incident Procedure

1. **Detection**: Automated alerts or manual report
2. **Containment**: Isolate affected systems
3. **Analysis**: Determine scope and impact
4. **Eradication**: Remove vulnerability
5. **Recovery**: Restore normal operations
6. **Lessons Learned**: Post-mortem and improvements

### Emergency Contacts

**Critical Security Issues**:
- Email: security-emergency@clawmaster.ai
- Response time: < 2 hours
- Escalation: CTO direct line

**Non-Critical Issues**:
- Email: security@clawmaster.ai
- GitHub: Issue with `[SECURITY]` prefix
- Response time: < 24 hours

---

## 📚 References

### Standards & Guidelines
- [DO-178C](https://en.wikipedia.org/wiki/DO-178C): Software Considerations in Airborne Systems
- [OWASP Top 10 2021](https://owasp.org/Top10/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)

### Rust Security
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [RustSec Advisory Database](https://rustsec.org/)
- [Cargo Audit](https://github.com/rustsec/rustsec)

### Internal Documentation
- `AEROSPACE_AUDIT_FIXES.md`: Detailed audit report
- `IMPLEMENTATION_GUIDE.md`: Security implementation guide
- `SECURITY_QUICK_REFERENCE.md`: Developer quick reference

---

## 📝 Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-11 | Initial security policy |

---

**Maintained by**: Security Team  
**Last Review**: 2026-03-11  
**Next Review**: 2026-06-11 (Quarterly)
