//! Security headers middleware for aerospace-grade web security.
//! 
//! Implements OWASP security headers best practices:
//! - Content-Security-Policy (CSP)
//! - X-Frame-Options
//! - X-Content-Type-Options
//! - Referrer-Policy
//! - Permissions-Policy
//! 
//! All headers are configurable and follow industry security standards.

use axum::{
    http::{header, HeaderValue, Request, Response},
    middleware::Next,
    body::Body,
};

/// Security headers configuration.
/// 
/// Aerospace Standard: All security headers must be configurable for different
/// deployment environments while maintaining secure defaults.
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Content-Security-Policy header value.
    /// 
    /// Default: Strict policy allowing only same-origin resources.
    pub csp: String,
    
    /// X-Frame-Options header value.
    /// 
    /// Default: "DENY" (prevents clickjacking)
    pub frame_options: String,
    
    /// X-Content-Type-Options header value.
    /// 
    /// Default: "nosniff" (prevents MIME sniffing)
    pub content_type_options: String,
    
    /// Referrer-Policy header value.
    /// 
    /// Default: "strict-origin-when-cross-origin"
    pub referrer_policy: String,
    
    /// Permissions-Policy header value.
    /// 
    /// Default: Restrictive policy disabling dangerous features
    pub permissions_policy: String,
    
    /// Whether to add Strict-Transport-Security header (HSTS).
    /// 
    /// Only enabled when TLS is active.
    pub hsts_enabled: bool,
    
    /// HSTS max-age in seconds.
    /// 
    /// Default: 31536000 (1 year)
    pub hsts_max_age: u64,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            // Content-Security-Policy: Strict but functional
            // - default-src 'self': Only load resources from same origin
            // - script-src 'self' 'unsafe-inline': Allow inline scripts (needed for some UI frameworks)
            // - style-src 'self' 'unsafe-inline': Allow inline styles (needed for Tailwind)
            // - img-src 'self' data: https:: Allow images from same origin, data URIs, and HTTPS
            // - font-src 'self' data:: Allow fonts from same origin and data URIs
            // - connect-src 'self': Only connect to same origin (WebSocket, fetch)
            // - frame-ancestors 'none': Prevent embedding in iframes
            csp: "default-src 'self'; \
                  script-src 'self' 'unsafe-inline'; \
                  style-src 'self' 'unsafe-inline'; \
                  img-src 'self' data: https:; \
                  font-src 'self' data:; \
                  connect-src 'self'; \
                  frame-ancestors 'none'; \
                  base-uri 'self'; \
                  form-action 'self'".to_string(),
            
            // X-Frame-Options: Prevent clickjacking
            frame_options: "DENY".to_string(),
            
            // X-Content-Type-Options: Prevent MIME sniffing
            content_type_options: "nosniff".to_string(),
            
            // Referrer-Policy: Balance privacy and functionality
            referrer_policy: "strict-origin-when-cross-origin".to_string(),
            
            // Permissions-Policy: Disable dangerous browser features
            permissions_policy: "geolocation=(), \
                                microphone=(), \
                                camera=(), \
                                payment=(), \
                                usb=(), \
                                magnetometer=(), \
                                gyroscope=(), \
                                accelerometer=()".to_string(),
            
            // HSTS: Enabled by default when TLS is active
            hsts_enabled: true,
            hsts_max_age: 31536000, // 1 year
        }
    }
}

impl SecurityHeadersConfig {
    /// Create a development-friendly configuration.
    /// 
    /// Relaxes some policies for local development while maintaining core security.
    pub fn development() -> Self {
        Self {
            // More permissive CSP for development
            csp: "default-src 'self' 'unsafe-inline' 'unsafe-eval'; \
                  img-src 'self' data: https: http:; \
                  connect-src 'self' ws: wss:".to_string(),
            
            // Allow framing in development (for debugging tools)
            frame_options: "SAMEORIGIN".to_string(),
            
            // Disable HSTS in development (no TLS)
            hsts_enabled: false,
            
            ..Default::default()
        }
    }
    
    /// Create a production configuration with maximum security.
    pub fn production() -> Self {
        Self {
            // Strict CSP for production
            csp: "default-src 'self'; \
                  script-src 'self'; \
                  style-src 'self'; \
                  img-src 'self' data: https:; \
                  font-src 'self'; \
                  connect-src 'self'; \
                  frame-ancestors 'none'; \
                  base-uri 'self'; \
                  form-action 'self'; \
                  upgrade-insecure-requests".to_string(),
            
            // Strict frame options
            frame_options: "DENY".to_string(),
            
            // Enable HSTS with includeSubDomains
            hsts_enabled: true,
            hsts_max_age: 63072000, // 2 years
            
            ..Default::default()
        }
    }
}

/// Middleware that adds security headers to all responses.
/// 
/// Aerospace Standard: Security headers must be applied consistently to all responses.
/// 
/// # Example
/// ```rust
/// use axum::Router;
/// use tower::ServiceBuilder;
/// 
/// let app = Router::new()
///     .layer(ServiceBuilder::new()
///         .layer(axum::middleware::from_fn(security_headers_middleware))
///     );
/// ```
pub async fn security_headers_middleware(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let config = SecurityHeadersConfig::default();
    security_headers_middleware_with_config(request, next, config).await
}

/// Middleware with custom configuration.
/// 
/// # Example
/// ```rust
/// let config = SecurityHeadersConfig::production();
/// let app = Router::new()
///     .layer(axum::middleware::from_fn(move |req, next| {
///         security_headers_middleware_with_config(req, next, config.clone())
///     }));
/// ```
pub async fn security_headers_middleware_with_config(
    request: Request<Body>,
    next: Next,
    config: SecurityHeadersConfig,
) -> Response<Body> {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    
    // Content-Security-Policy
    if let Ok(value) = HeaderValue::from_str(&config.csp) {
        headers.insert(header::CONTENT_SECURITY_POLICY, value);
    }
    
    // X-Frame-Options
    if let Ok(value) = HeaderValue::from_str(&config.frame_options) {
        headers.insert(header::X_FRAME_OPTIONS, value);
    }
    
    // X-Content-Type-Options
    if let Ok(value) = HeaderValue::from_str(&config.content_type_options) {
        headers.insert(header::X_CONTENT_TYPE_OPTIONS, value);
    }
    
    // Referrer-Policy
    if let Ok(value) = HeaderValue::from_str(&config.referrer_policy) {
        headers.insert(header::REFERRER_POLICY, value);
    }
    
    // Permissions-Policy
    if let Ok(value) = HeaderValue::from_str(&config.permissions_policy) {
        headers.insert(
            header::HeaderName::from_static("permissions-policy"),
            value
        );
    }
    
    // Strict-Transport-Security (HSTS) - only if enabled
    if config.hsts_enabled {
        let hsts_value = format!(
            "max-age={}; includeSubDomains; preload",
            config.hsts_max_age
        );
        if let Ok(value) = HeaderValue::from_str(&hsts_value) {
            headers.insert(header::STRICT_TRANSPORT_SECURITY, value);
        }
    }
    
    // X-XSS-Protection (legacy, but still useful for older browsers)
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block")
    );
    
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        routing::get,
        middleware,
    };
    use tower::ServiceExt;
    use http::Request;
    
    async fn test_handler() -> &'static str {
        "test"
    }
    
    #[tokio::test]
    async fn test_default_security_headers() {
        let app = Router::new()
            .route("/", get(test_handler))
            .layer(middleware::from_fn(security_headers_middleware));
        
        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        let headers = response.headers();
        
        // Verify all security headers are present
        assert!(headers.contains_key(header::CONTENT_SECURITY_POLICY));
        assert!(headers.contains_key(header::X_FRAME_OPTIONS));
        assert!(headers.contains_key(header::X_CONTENT_TYPE_OPTIONS));
        assert!(headers.contains_key(header::REFERRER_POLICY));
        assert!(headers.contains_key("permissions-policy"));
        assert!(headers.contains_key(header::STRICT_TRANSPORT_SECURITY));
    }
    
    #[tokio::test]
    async fn test_production_config() {
        let config = SecurityHeadersConfig::production();
        
        assert!(config.csp.contains("upgrade-insecure-requests"));
        assert_eq!(config.frame_options, "DENY");
        assert!(config.hsts_enabled);
        assert_eq!(config.hsts_max_age, 63072000);
    }
    
    #[tokio::test]
    async fn test_development_config() {
        let config = SecurityHeadersConfig::development();
        
        assert!(config.csp.contains("'unsafe-eval'"));
        assert_eq!(config.frame_options, "SAMEORIGIN");
        assert!(!config.hsts_enabled);
    }
    
    #[test]
    fn test_csp_prevents_inline_scripts_in_production() {
        let config = SecurityHeadersConfig::production();
        
        // Production CSP should NOT allow unsafe-inline for scripts
        assert!(!config.csp.contains("script-src 'self' 'unsafe-inline'"));
        assert!(config.csp.contains("script-src 'self'"));
    }
    
    #[test]
    fn test_frame_options_prevents_clickjacking() {
        let config = SecurityHeadersConfig::default();
        
        assert_eq!(config.frame_options, "DENY");
    }
}
