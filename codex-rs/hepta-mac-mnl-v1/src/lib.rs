//! Pure, non-authorizing model for the Mac member of the MNL successor.
//!
//! This crate intentionally has no dependency on the effectful operator crate
//! and contains no filesystem, account, launchd, process, or socket API. Its
//! two binaries can only emit a canonical plan or reject a canonical plan.

use rand::TryRngCore;
use rand::rngs::OsRng;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

pub mod broker;
pub mod client;
pub mod install;

pub const MAX_CANONICAL_BYTES: usize = 1024 * 1024;

#[derive(Debug, Error)]
pub enum MnlError {
    #[error("BLOCKED: {0}")]
    Blocked(String),
    #[error("invalid Mac MNL v1 model: {0}")]
    Invalid(String),
    #[error("Mac MNL v1 serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub(crate) fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, MnlError> {
    Ok(serde_json::to_vec(value)?)
}

pub(crate) fn parse_canonical<T>(bytes: &[u8], label: &str) -> Result<T, MnlError>
where
    T: DeserializeOwned + Serialize,
{
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_BYTES {
        return Err(invalid(format!(
            "{label} must be non-empty and at most {MAX_CANONICAL_BYTES} bytes"
        )));
    }
    let value: T = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("{label} is malformed: {error}")))?;
    if canonical_json(&value)? != bytes {
        return Err(invalid(format!("{label} is not exact canonical JSON")));
    }
    Ok(value)
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex(&digest)
}

pub(crate) fn random_hex<const N: usize>() -> Result<String, MnlError> {
    let mut bytes = [0_u8; N];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(|error| invalid(format!("OS randomness unavailable: {error}")))?;
    Ok(hex(&bytes))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(DIGITS[usize::from(byte >> 4)]));
        encoded.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    encoded
}

pub(crate) fn blocked(message: impl Into<String>) -> MnlError {
    MnlError::Blocked(message.into())
}

pub(crate) fn invalid(message: impl Into<String>) -> MnlError {
    MnlError::Invalid(message.into())
}
