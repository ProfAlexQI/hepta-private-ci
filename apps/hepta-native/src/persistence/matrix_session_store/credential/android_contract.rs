use std::{error::Error, fmt};

const IDENTITY_DOMAIN: &str = "ai.hepta.native.android-keystore-credential.v1";
const BLOB_VERSION: u8 = 1;
const AES_GCM_IV_LEN: usize = 12;
const AES_GCM_TAG_LEN: usize = 16;
const MAX_ATTRIBUTE_LEN: usize = 1024;
const MAX_CIPHERTEXT_LEN: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AndroidCredentialIdentity {
    pub(super) key_alias: String,
    pub(super) preference_key: String,
    pub(super) aad: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProtectedCredentialBlob {
    pub(super) iv: Vec<u8>,
    pub(super) ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AndroidContractError(&'static str);

impl fmt::Display for AndroidContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Error for AndroidContractError {}

fn push_bound_value(output: &mut Vec<u8>, value: &str) {
    output.extend_from_slice(&(value.len() as u64).to_le_bytes());
    output.extend_from_slice(value.as_bytes());
}

pub(super) fn credential_identity(
    service: &str,
    account: &str,
) -> Result<AndroidCredentialIdentity, AndroidContractError> {
    if service.is_empty() || service.len() > MAX_ATTRIBUTE_LEN {
        return Err(AndroidContractError(
            "invalid Android credential service length",
        ));
    }
    if account.is_empty() || account.len() > MAX_ATTRIBUTE_LEN {
        return Err(AndroidContractError(
            "invalid Android credential account length",
        ));
    }

    let mut bound_identity = Vec::with_capacity(service.len() + account.len() + 16);
    push_bound_value(&mut bound_identity, service);
    push_bound_value(&mut bound_identity, account);
    let digest = blake3::derive_key(IDENTITY_DOMAIN, &bound_identity);
    let digest_hex = encode_hex(&digest);

    let mut aad = Vec::with_capacity(IDENTITY_DOMAIN.len() + bound_identity.len() + 8);
    push_bound_value(&mut aad, IDENTITY_DOMAIN);
    aad.extend_from_slice(&bound_identity);
    Ok(AndroidCredentialIdentity {
        key_alias: format!("ai.hepta.native.matrix.{digest_hex}"),
        preference_key: format!("credential.{digest_hex}"),
        aad,
    })
}

pub(super) fn encode_protected_blob(
    iv: &[u8],
    ciphertext: &[u8],
) -> Result<String, AndroidContractError> {
    validate_blob_parts(iv, ciphertext)?;
    let mut bytes = Vec::with_capacity(2 + iv.len() + ciphertext.len());
    bytes.push(BLOB_VERSION);
    bytes.push(iv.len() as u8);
    bytes.extend_from_slice(iv);
    bytes.extend_from_slice(ciphertext);
    Ok(encode_hex(&bytes))
}

pub(super) fn decode_protected_blob(
    encoded: &str,
) -> Result<ProtectedCredentialBlob, AndroidContractError> {
    let bytes = decode_hex(encoded)?;
    if bytes.len() < 2 || bytes[0] != BLOB_VERSION {
        return Err(AndroidContractError(
            "unsupported Android credential blob version",
        ));
    }
    let iv_len = usize::from(bytes[1]);
    if bytes.len() < 2 + iv_len {
        return Err(AndroidContractError("truncated Android credential blob"));
    }
    let blob = ProtectedCredentialBlob {
        iv: bytes[2..2 + iv_len].to_vec(),
        ciphertext: bytes[2 + iv_len..].to_vec(),
    };
    validate_blob_parts(&blob.iv, &blob.ciphertext)?;
    Ok(blob)
}

fn validate_blob_parts(iv: &[u8], ciphertext: &[u8]) -> Result<(), AndroidContractError> {
    if iv.len() != AES_GCM_IV_LEN {
        return Err(AndroidContractError("invalid Android AES-GCM IV length"));
    }
    if !(AES_GCM_TAG_LEN..=MAX_CIPHERTEXT_LEN).contains(&ciphertext.len()) {
        return Err(AndroidContractError(
            "invalid Android encrypted credential length",
        ));
    }
    Ok(())
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn decode_hex(encoded: &str) -> Result<Vec<u8>, AndroidContractError> {
    if encoded.len() % 2 != 0 || encoded.len() > (2 + AES_GCM_IV_LEN + MAX_CIPHERTEXT_LEN) * 2 {
        return Err(AndroidContractError(
            "invalid Android credential hex length",
        ));
    }
    let mut output = Vec::with_capacity(encoded.len() / 2);
    for pair in encoded.as_bytes().chunks_exact(2) {
        let high = decode_nibble(pair[0])?;
        let low = decode_nibble(pair[1])?;
        output.push((high << 4) | low);
    }
    Ok(output)
}

fn decode_nibble(value: u8) -> Result<u8, AndroidContractError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(AndroidContractError("invalid Android credential hex")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_deterministic_domain_bound_and_opaque() {
        let first =
            credential_identity("ai.hepta.native.matrix", "matrix-session-v2|@a:b|nonce").unwrap();
        let second =
            credential_identity("ai.hepta.native.matrix", "matrix-session-v2|@a:b|nonce").unwrap();
        let other =
            credential_identity("ai.hepta.native.matrix", "matrix-session-v2|@c:d|nonce").unwrap();
        assert_eq!(first, second);
        assert_ne!(first.key_alias, other.key_alias);
        assert!(!first.key_alias.contains("@a:b"));
        assert!(!first.preference_key.contains("@a:b"));
        assert_ne!(first.aad, other.aad);
    }

    #[test]
    fn protected_blob_roundtrips_with_fixed_gcm_iv_contract() {
        let iv = [7_u8; AES_GCM_IV_LEN];
        let ciphertext = [9_u8; 32];
        let encoded = encode_protected_blob(&iv, &ciphertext).unwrap();
        assert_eq!(
            decode_protected_blob(&encoded).unwrap(),
            ProtectedCredentialBlob {
                iv: iv.to_vec(),
                ciphertext: ciphertext.to_vec(),
            }
        );
    }

    #[test]
    fn malformed_or_oversized_blobs_fail_closed() {
        assert!(decode_protected_blob("").is_err());
        assert!(decode_protected_blob("zz").is_err());
        assert!(encode_protected_blob(&[0_u8; 11], &[0_u8; 16]).is_err());
        assert!(encode_protected_blob(&[0_u8; 12], &[0_u8; 15]).is_err());
        assert!(encode_protected_blob(&[0_u8; 12], &[0_u8; MAX_CIPHERTEXT_LEN + 1]).is_err());
    }

    #[test]
    fn invalid_identity_attributes_fail_closed() {
        assert!(credential_identity("", "account").is_err());
        assert!(credential_identity("service", "").is_err());
        assert!(credential_identity(&"s".repeat(MAX_ATTRIBUTE_LEN + 1), "account").is_err());
    }
}
