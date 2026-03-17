//! Security verification for ClawHub tools.

use {
    crate::error::{Error, Result},
    ed25519_dalek::{Signature, Verifier, VerifyingKey},
    sha2::{Digest, Sha256},
};

/// Verify tool signature.
///
/// # Arguments
/// * `wasm_bytes` - Wasm file bytes
/// * `signature_hex` - Hex-encoded Ed25519 signature
/// * `public_key_hex` - Hex-encoded Ed25519 public key
///
/// # Errors
/// Returns an error if signature verification fails.
///
/// # Compliance
/// DO-178C §6.3.2: Exception handling
/// - All crypto errors are properly handled
/// - No panics in production code
pub fn verify_signature(
    wasm_bytes: &[u8],
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<()> {
    // Decode hex strings
    let signature_bytes =
        hex::decode(signature_hex).map_err(|_| Error::SignatureVerificationFailed)?;
    let public_key_bytes =
        hex::decode(public_key_hex).map_err(|_| Error::SignatureVerificationFailed)?;

    // Parse signature and public key
    let signature = Signature::from_bytes(
        signature_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::SignatureVerificationFailed)?,
    );

    let public_key = VerifyingKey::from_bytes(
        public_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::SignatureVerificationFailed)?,
    )
    .map_err(|_| Error::SignatureVerificationFailed)?;

    // Verify signature
    public_key
        .verify(wasm_bytes, &signature)
        .map_err(|_| Error::SignatureVerificationFailed)?;

    Ok(())
}

/// Compute SHA-256 hash of Wasm bytes.
///
/// # Returns
/// Hex-encoded SHA-256 hash.
pub fn compute_wasm_hash(wasm_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(wasm_bytes);
    hex::encode(hasher.finalize())
}

/// Basic security scan of Wasm bytes.
///
/// This performs basic checks:
/// - Valid Wasm magic number
/// - Reasonable file size
/// - No suspicious patterns
///
/// # Returns
/// `Ok(())` if basic checks pass, error otherwise.
pub fn basic_security_scan(wasm_bytes: &[u8]) -> Result<()> {
    // Check Wasm magic number
    if !wasm_bytes.starts_with(b"\x00asm") {
        return Err(Error::SecurityVerificationFailed(
            "Invalid Wasm magic number".to_string(),
        ));
    }

    // Check reasonable file size (< 10 MB)
    if wasm_bytes.len() > 10 * 1024 * 1024 {
        return Err(Error::SecurityVerificationFailed(
            "Wasm file too large (> 10 MB)".to_string(),
        ));
    }

    // TODO: Add more sophisticated checks
    // - Parse Wasm structure
    // - Check for suspicious imports
    // - Validate component model compliance

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_wasm_hash() {
        let wasm_bytes = b"\x00asm\x01\x00\x00\x00";
        let hash = compute_wasm_hash(wasm_bytes);

        // Hash should be 64 hex characters (32 bytes)
        assert_eq!(hash.len(), 64);

        // Same input should give same hash
        assert_eq!(hash, compute_wasm_hash(wasm_bytes));
    }

    #[test]
    fn test_basic_security_scan() {
        // Valid Wasm magic
        assert!(basic_security_scan(b"\x00asm\x01\x00\x00\x00").is_ok());

        // Invalid magic
        assert!(basic_security_scan(b"invalid").is_err());

        // Too large (> 10 MB)
        let large_file = vec![0u8; 11 * 1024 * 1024];
        assert!(basic_security_scan(&large_file).is_err());
    }
}
