use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::Digest as _;

use crate::LinuxMnlError;
use crate::invalid;

pub const MAX_CANONICAL_DOCUMENT_BYTES_V1: usize = 256 * 1024;

/// Serialize a typed document as compact, deterministic JSON.
///
/// The model intentionally contains no unordered maps. Field declaration
/// order and enum representation are frozen by the Rust types.
pub fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, LinuxMnlError> {
    serde_json::to_vec(value)
        .map_err(|error| invalid(format!("canonical JSON serialization failed: {error}")))
}

pub fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, LinuxMnlError> {
    Ok(sha256(&canonical_json(value)?))
}

/// Decode only an exact compact canonical representation.
pub fn decode_canonical_json<T>(bytes: &[u8]) -> Result<T, LinuxMnlError>
where
    T: DeserializeOwned + Serialize,
{
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_DOCUMENT_BYTES_V1 {
        return Err(invalid(
            "canonical document length is outside its exact bound",
        ));
    }
    let value: T = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("strict canonical JSON decode failed: {error}")))?;
    if canonical_json(&value)? != bytes {
        return Err(invalid("document is not exact compact canonical JSON"));
    }
    Ok(value)
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    let mut digest = sha2::Sha256::new();
    digest.update(bytes);
    format!("{:x}", digest.finalize())
}

pub(crate) fn validate_digest(label: &str, digest: &str) -> Result<(), LinuxMnlError> {
    if digest.len() != 64
        || !digest
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid(format!(
            "{label} must be one lowercase hexadecimal SHA-256"
        )));
    }
    Ok(())
}
