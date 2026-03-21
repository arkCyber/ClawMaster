//! Audit Log Signature
//!
//! DO-178C Level A Compliant Log Signing and Verification

use {
    crate::{AuditError, AuditEvent, AuditResult},
    async_trait::async_trait,
    hmac::{Hmac, Mac},
    sha2::Sha256,
};

type HmacSha256 = Hmac<Sha256>;

/// HMAC-based log signer
pub struct HmacSigner {
    key: Vec<u8>,
}

impl HmacSigner {
    /// Create new HMAC signer
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    /// Create from hex-encoded key
    pub fn from_hex(hex_key: &str) -> AuditResult<Self> {
        let key = hex::decode(hex_key)
            .map_err(|e| AuditError::SignatureError(format!("Invalid hex key: {}", e)))?;
        Ok(Self::new(key))
    }

    /// Generate signing key
    pub fn generate_key() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::rng();
        (0..32).map(|_| rng.random::<u8>()).collect()
    }

    /// Compute signature for event
    fn compute_signature(&self, event: &AuditEvent) -> AuditResult<String> {
        // Create canonical representation
        let canonical = self.canonicalize(event)?;

        // Compute HMAC
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|e| AuditError::SignatureError(e.to_string()))?;
        mac.update(canonical.as_bytes());
        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        Ok(signature)
    }

    /// Create canonical representation of event
    fn canonicalize(&self, event: &AuditEvent) -> AuditResult<String> {
        // Create deterministic JSON without signature
        let mut event_copy = event.clone();
        if let Some(obj) = event_copy.metadata.as_object_mut() {
            obj.remove("signature");
        }

        serde_json::to_string(&event_copy)
            .map_err(|e| AuditError::SerializationError(e.to_string()))
    }
}

#[async_trait]
impl crate::LogSigner for HmacSigner {
    async fn sign(&self, event: &AuditEvent) -> AuditResult<String> {
        self.compute_signature(event)
    }

    async fn verify(&self, event: &AuditEvent) -> AuditResult<bool> {
        // Extract signature from metadata
        let stored_signature = event
            .metadata
            .get("signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AuditError::SignatureError("No signature found".to_string()))?;

        // Compute expected signature
        let expected_signature = self.compute_signature(event)?;

        // Constant-time comparison
        Ok(stored_signature == expected_signature)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{AuthEvent, EventSeverity, LogSigner},
    };

    #[tokio::test]
    async fn test_hmac_signer_sign() {
        let key = HmacSigner::generate_key();
        let signer = HmacSigner::new(key);

        let event = AuditEvent::auth(EventSeverity::High, AuthEvent::LoginAttempt {
            username: "user1".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
        });

        let signature = signer.sign(&event).await.unwrap();
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64); // SHA256 hex = 64 chars
    }

    #[tokio::test]
    async fn test_hmac_signer_verify() {
        let key = HmacSigner::generate_key();
        let signer = HmacSigner::new(key);

        let mut event = AuditEvent::auth(EventSeverity::High, AuthEvent::LoginAttempt {
            username: "user1".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
        });

        // Sign event
        let signature = signer.sign(&event).await.unwrap();
        event
            .metadata
            .as_object_mut()
            .unwrap()
            .insert("signature".to_string(), serde_json::json!(signature));

        // Verify
        let valid = signer.verify(&event).await.unwrap();
        assert!(valid);
    }

    #[tokio::test]
    async fn test_hmac_signer_invalid_signature() {
        let key = HmacSigner::generate_key();
        let signer = HmacSigner::new(key);

        let mut event = AuditEvent::auth(EventSeverity::High, AuthEvent::LoginAttempt {
            username: "user1".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
        });

        // Add invalid signature
        event
            .metadata
            .as_object_mut()
            .unwrap()
            .insert("signature".to_string(), serde_json::json!("invalid"));

        // Verify should fail
        let valid = signer.verify(&event).await.unwrap();
        assert!(!valid);
    }

    #[tokio::test]
    async fn test_hmac_signer_tamper_detection() {
        let key = HmacSigner::generate_key();
        let signer = HmacSigner::new(key);

        let mut event = AuditEvent::auth(EventSeverity::High, AuthEvent::LoginAttempt {
            username: "user1".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
        });

        // Sign event
        let signature = signer.sign(&event).await.unwrap();
        event
            .metadata
            .as_object_mut()
            .unwrap()
            .insert("signature".to_string(), serde_json::json!(signature));

        // Tamper with event
        if let crate::EventDetails::Auth(crate::AuthEvent::LoginAttempt {
            ref mut username, ..
        }) = event.details
        {
            *username = "hacker".to_string();
        }

        // Verify should fail
        let valid = signer.verify(&event).await.unwrap();
        assert!(!valid);
    }

    #[tokio::test]
    async fn test_hmac_signer_deterministic() {
        let key = HmacSigner::generate_key();
        let signer = HmacSigner::new(key);

        let event = AuditEvent::auth(EventSeverity::High, AuthEvent::LoginAttempt {
            username: "user1".to_string(),
            success: true,
            ip_address: None,
            user_agent: None,
        });

        let sig1 = signer.sign(&event).await.unwrap();
        let sig2 = signer.sign(&event).await.unwrap();

        assert_eq!(sig1, sig2);
    }
}
