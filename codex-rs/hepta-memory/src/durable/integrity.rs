use std::fmt;
use std::sync::Arc;

use hmac::Hmac;
use hmac::Mac;
use sha2::Digest;
use sha2::Sha256;
use zeroize::Zeroizing;

use super::DurableStorageError;

const KEY_ID_DOMAIN: &[u8] = b"hepta.memory.durable-integrity.key-id.v1";
const ROW_MAC_DOMAIN: &[u8] = b"hepta.memory.durable-integrity.row-mac.v1";
const KEYED_SCHEMA_VERSION: i64 = 5;
const UNKEYED_SCHEMA_VERSION: i64 = 4;

type HmacSha256 = Hmac<Sha256>;

/// Non-cloneable caller-supplied authority for keyed durable-row integrity.
///
/// Hepta never persists this key. Production composition must recover the same
/// 32-byte value from an external secret boundary before opening the database.
pub struct DurableIntegrityKey(Zeroizing<[u8; 32]>);

impl DurableIntegrityKey {
    /// Constructs an exact 256-bit durable-integrity key.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    pub(crate) fn into_context(self) -> DurableIntegrityContext {
        DurableIntegrityContext::keyed(self.0)
    }
}

impl fmt::Debug for DurableIntegrityKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DurableIntegrityKey")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct DurableIntegrityContext {
    mode: DurableIntegrityMode,
}

#[derive(Clone)]
enum DurableIntegrityMode {
    Unkeyed,
    Keyed {
        key: Arc<Zeroizing<[u8; 32]>>,
        key_id: Arc<str>,
    },
}

impl DurableIntegrityContext {
    pub(crate) fn unkeyed() -> Self {
        Self {
            mode: DurableIntegrityMode::Unkeyed,
        }
    }

    fn keyed(key: Zeroizing<[u8; 32]>) -> Self {
        let key_id = keyed_digest(KEY_ID_DOMAIN, key.as_ref());
        Self {
            mode: DurableIntegrityMode::Keyed {
                key: Arc::new(key),
                key_id: Arc::from(key_id),
            },
        }
    }

    pub(crate) const fn schema_version(&self) -> i64 {
        match &self.mode {
            DurableIntegrityMode::Unkeyed => UNKEYED_SCHEMA_VERSION,
            DurableIntegrityMode::Keyed { .. } => KEYED_SCHEMA_VERSION,
        }
    }

    pub(crate) const fn algorithm(&self) -> &'static str {
        match &self.mode {
            DurableIntegrityMode::Unkeyed => "sha256",
            DurableIntegrityMode::Keyed { .. } => "hmac-sha256-v1",
        }
    }

    pub(crate) fn key_id(&self) -> Option<&str> {
        match &self.mode {
            DurableIntegrityMode::Unkeyed => None,
            DurableIntegrityMode::Keyed { key_id, .. } => Some(key_id),
        }
    }

    pub(crate) fn protect(&self, payload_json: &str) -> Result<String, DurableStorageError> {
        Ok(match &self.mode {
            DurableIntegrityMode::Unkeyed => unkeyed_digest(payload_json.as_bytes()),
            DurableIntegrityMode::Keyed { key, .. } => {
                let mut mac = new_mac(key.as_ref().as_ref())?;
                update_frame(&mut mac, ROW_MAC_DOMAIN);
                update_frame(&mut mac, payload_json.as_bytes());
                format!("hmac-sha256:{}", encode_hex(&mac.finalize().into_bytes()))
            }
        })
    }

    pub(crate) fn verify(
        &self,
        payload_json: &str,
        expected: &str,
        row_kind: &str,
    ) -> Result<(), DurableStorageError> {
        match &self.mode {
            DurableIntegrityMode::Unkeyed => {
                let actual = unkeyed_digest(payload_json.as_bytes());
                if actual != expected {
                    return Err(DurableStorageError::corrupt(format!(
                        "{row_kind} storage hash mismatch: expected {expected}, calculated {actual}"
                    )));
                }
            }
            DurableIntegrityMode::Keyed { key, .. } => {
                let encoded = expected.strip_prefix("hmac-sha256:").ok_or_else(|| {
                    DurableStorageError::corrupt(format!(
                        "{row_kind} keyed integrity tag has an invalid algorithm prefix"
                    ))
                })?;
                let expected_bytes = decode_hex(encoded).ok_or_else(|| {
                    DurableStorageError::corrupt(format!(
                        "{row_kind} keyed integrity tag is not canonical hex"
                    ))
                })?;
                let mut mac = new_mac(key.as_ref().as_ref())?;
                update_frame(&mut mac, ROW_MAC_DOMAIN);
                update_frame(&mut mac, payload_json.as_bytes());
                mac.verify_slice(&expected_bytes).map_err(|_| {
                    DurableStorageError::corrupt(format!(
                        "{row_kind} keyed integrity verification failed"
                    ))
                })?;
            }
        }
        Ok(())
    }
}

fn new_mac(key: &[u8]) -> Result<HmacSha256, DurableStorageError> {
    HmacSha256::new_from_slice(key).map_err(|_| {
        DurableStorageError::corrupt("durable integrity key has an unsupported length")
    })
}

fn keyed_digest(domain: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    update_digest_frame(&mut digest, domain);
    update_digest_frame(&mut digest, value);
    format!("sha256:{}", encode_hex(&digest.finalize()))
}

fn unkeyed_digest(value: &[u8]) -> String {
    format!("sha256:{}", encode_hex(&Sha256::digest(value)))
}

fn update_frame(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn update_digest_frame(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn decode_hex(encoded: &str) -> Option<Vec<u8>> {
    if encoded.len() != 64 {
        return None;
    }
    encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = decode_nibble(pair[0])?;
            let low = decode_nibble(pair[1])?;
            Some((high << 4) | low)
        })
        .collect()
}

fn decode_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}
