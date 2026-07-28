use std::error::Error;
use std::fmt;

use hmac::Hmac;
use hmac::Mac;
use sha2::Digest;
use sha2::Sha256;

use crate::AuthorityJournalPolicy;

type HmacSha256 = Hmac<Sha256>;

/// Domain framing used by an authenticated journal's persisted schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationFraming {
    /// The domain is written directly and each field is length-prefixed.
    RawDomain,
    /// The domain and every field are length-prefixed.
    FramedDomain,
}

/// Shared failure type for authenticated journal primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticatedJournalError {
    /// The HMAC key could not initialize the selected algorithm.
    InvalidKey,
    /// A hexadecimal value was not canonical lowercase bytes.
    InvalidHex,
    /// An authenticated value did not match its expected MAC.
    AuthenticationFailed,
    /// Active journal entries exceeded their policy limit.
    ActiveRecordLimit {
        /// Observed active record count.
        actual: usize,
        /// Policy limit.
        maximum: usize,
    },
    /// Compacted replay authorities exceeded their policy limit.
    CheckpointLimit {
        /// Observed compacted authority count.
        actual: usize,
        /// Policy limit.
        maximum: usize,
    },
    /// Serialized state exceeded its policy limit.
    JournalByteLimit {
        /// Observed serialized byte count.
        actual: u64,
        /// Policy limit.
        maximum: u64,
    },
    /// One serialized record exceeded its policy limit.
    RecordByteLimit {
        /// Observed serialized byte count.
        actual: usize,
        /// Policy limit.
        maximum: usize,
    },
}

impl fmt::Display for AuthenticatedJournalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey => formatter.write_str("authenticated journal HMAC key is invalid"),
            Self::InvalidHex => formatter.write_str("hex value is not canonical lowercase bytes"),
            Self::AuthenticationFailed => {
                formatter.write_str("authenticated journal MAC is invalid")
            }
            Self::ActiveRecordLimit { actual, maximum } => {
                write!(formatter, "active record count {actual} exceeds {maximum}")
            }
            Self::CheckpointLimit { actual, maximum } => {
                write!(
                    formatter,
                    "checkpointed authority count {actual} exceeds {maximum}"
                )
            }
            Self::JournalByteLimit { actual, maximum } => {
                write!(formatter, "journal size {actual} exceeds {maximum} bytes")
            }
            Self::RecordByteLimit { actual, maximum } => {
                write!(formatter, "record size {actual} exceeds {maximum} bytes")
            }
        }
    }
}

impl Error for AuthenticatedJournalError {}

/// Cryptographic and bounded-state primitives shared by authority journals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatedJournalEngine {
    policy: AuthorityJournalPolicy,
}

impl AuthenticatedJournalEngine {
    /// Creates an engine bound to one immutable journal policy.
    pub const fn new(policy: AuthorityJournalPolicy) -> Self {
        Self { policy }
    }

    /// Returns the immutable policy enforced by this engine.
    pub const fn policy(self) -> AuthorityJournalPolicy {
        self.policy
    }

    /// Validates active and compacted replay-authority counts.
    pub fn validate_counts(
        self,
        active_records: usize,
        checkpointed_authorities: usize,
    ) -> Result<(), AuthenticatedJournalError> {
        if active_records > self.policy.max_active_records {
            return Err(AuthenticatedJournalError::ActiveRecordLimit {
                actual: active_records,
                maximum: self.policy.max_active_records,
            });
        }
        if checkpointed_authorities > self.policy.max_checkpointed_authorities {
            return Err(AuthenticatedJournalError::CheckpointLimit {
                actual: checkpointed_authorities,
                maximum: self.policy.max_checkpointed_authorities,
            });
        }
        Ok(())
    }

    /// Validates the serialized journal byte count.
    pub fn validate_journal_bytes(self, bytes: u64) -> Result<(), AuthenticatedJournalError> {
        if bytes > self.policy.max_journal_bytes {
            return Err(AuthenticatedJournalError::JournalByteLimit {
                actual: bytes,
                maximum: self.policy.max_journal_bytes,
            });
        }
        Ok(())
    }

    /// Validates one serialized record, response, or error payload.
    pub fn validate_record_bytes(self, bytes: usize) -> Result<(), AuthenticatedJournalError> {
        if bytes > self.policy.max_record_bytes {
            return Err(AuthenticatedJournalError::RecordByteLimit {
                actual: bytes,
                maximum: self.policy.max_record_bytes,
            });
        }
        Ok(())
    }

    /// Computes a lowercase SHA-256 digest with the selected persisted framing.
    pub fn digest_hex(
        self,
        framing: AuthenticationFraming,
        domain: &[u8],
        fields: &[&[u8]],
    ) -> String {
        let mut hasher = Sha256::new();
        update_domain(&mut hasher, framing, domain);
        for field in fields {
            update_frame(&mut hasher, field);
        }
        hex_encode(&hasher.finalize())
    }

    /// Computes a `sha256:` content hash with the selected persisted framing.
    pub fn content_hash(
        self,
        framing: AuthenticationFraming,
        domain: &[u8],
        fields: &[&[u8]],
    ) -> String {
        format!("sha256:{}", self.digest_hex(framing, domain, fields))
    }

    /// Computes a lowercase HMAC-SHA256 with the selected persisted framing.
    pub fn mac_hex(
        self,
        key: &[u8],
        framing: AuthenticationFraming,
        domain: &[u8],
        fields: &[&[u8]],
    ) -> Result<String, AuthenticatedJournalError> {
        let mut mac =
            HmacSha256::new_from_slice(key).map_err(|_| AuthenticatedJournalError::InvalidKey)?;
        update_domain(&mut mac, framing, domain);
        for field in fields {
            update_frame(&mut mac, field);
        }
        Ok(hex_encode(&mac.finalize().into_bytes()))
    }

    /// Verifies a canonical lowercase HMAC-SHA256 in constant time.
    pub fn verify_mac_hex(
        self,
        key: &[u8],
        framing: AuthenticationFraming,
        domain: &[u8],
        fields: &[&[u8]],
        proof: &str,
    ) -> Result<(), AuthenticatedJournalError> {
        let proof = decode_sha256_hex(proof)?;
        let mut mac =
            HmacSha256::new_from_slice(key).map_err(|_| AuthenticatedJournalError::InvalidKey)?;
        update_domain(&mut mac, framing, domain);
        for field in fields {
            update_frame(&mut mac, field);
        }
        mac.verify_slice(&proof)
            .map_err(|_| AuthenticatedJournalError::AuthenticationFailed)
    }

    /// Compares two authenticated text values without data-dependent early exit.
    pub fn constant_time_equal(self, left: &str, right: &str) -> bool {
        left.len() == right.len()
            && left
                .bytes()
                .zip(right.bytes())
                .fold(0_u8, |difference, (left, right)| {
                    difference | (left ^ right)
                })
                == 0
    }
}

/// Encodes bytes as canonical lowercase hexadecimal.
pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

/// Decodes canonical lowercase hexadecimal bytes.
pub fn hex_decode(value: &str) -> Result<Vec<u8>, AuthenticatedJournalError> {
    if !value.len().is_multiple_of(2)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(AuthenticatedJournalError::InvalidHex);
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| Ok((hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?))
        .collect()
}

/// Decodes one canonical lowercase SHA-256 hexadecimal digest.
pub fn decode_sha256_hex(value: &str) -> Result<[u8; 32], AuthenticatedJournalError> {
    if value.len() != 64 {
        return Err(AuthenticatedJournalError::InvalidHex);
    }
    hex_decode(value)?
        .try_into()
        .map_err(|_| AuthenticatedJournalError::InvalidHex)
}

fn hex_nibble(value: u8) -> Result<u8, AuthenticatedJournalError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(AuthenticatedJournalError::InvalidHex),
    }
}

fn update_domain(
    target: &mut impl DigestFrameTarget,
    framing: AuthenticationFraming,
    domain: &[u8],
) {
    match framing {
        AuthenticationFraming::RawDomain => target.update_bytes(domain),
        AuthenticationFraming::FramedDomain => update_frame(target, domain),
    }
}

fn update_frame(target: &mut impl DigestFrameTarget, value: &[u8]) {
    target.update_bytes(&(value.len() as u64).to_be_bytes());
    target.update_bytes(value);
}

trait DigestFrameTarget {
    fn update_bytes(&mut self, bytes: &[u8]);
}

impl DigestFrameTarget for Sha256 {
    fn update_bytes(&mut self, bytes: &[u8]) {
        Digest::update(self, bytes);
    }
}

impl DigestFrameTarget for HmacSha256 {
    fn update_bytes(&mut self, bytes: &[u8]) {
        Mac::update(self, bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PLUGIN_MUTATION_JOURNAL_POLICY;

    const ENGINE: AuthenticatedJournalEngine =
        AuthenticatedJournalEngine::new(PLUGIN_MUTATION_JOURNAL_POLICY);

    #[test]
    fn frozen_framing_vectors_preserve_all_three_journal_schemas() {
        let key = [0x11; 32];
        let fields = [b"alpha".as_slice(), b"beta".as_slice()];
        assert_eq!(
            ENGINE
                .mac_hex(&key, AuthenticationFraming::RawDomain, b"domain", &fields)
                .expect("raw-domain MAC"),
            "2919966d63084b90efb38ab890148a2fa436db8a2c45750fd4b8b72088cb32d0"
        );
        assert_eq!(
            ENGINE
                .mac_hex(
                    &key,
                    AuthenticationFraming::FramedDomain,
                    b"domain",
                    &fields
                )
                .expect("framed-domain MAC"),
            "767e4a18170d01eb4cf6427394c91788e591e5bf08a713555576d97eec177bd0"
        );
        assert_eq!(
            ENGINE.digest_hex(AuthenticationFraming::RawDomain, b"domain", &fields),
            "d93beb5aacb7c43cbc587e068f3f73a7d87c3e6207c285c0505c3e1df2cbf4e2"
        );
        assert_eq!(
            ENGINE.digest_hex(AuthenticationFraming::FramedDomain, b"domain", &fields),
            "75fe625e75d36d371aeeb2a084eaa5ab88a849f87fcefcd9dd3c9780a0e170da"
        );
    }

    #[test]
    fn verification_hex_and_bounds_fail_closed() {
        let key = [7_u8; 32];
        let proof = ENGINE
            .mac_hex(
                &key,
                AuthenticationFraming::FramedDomain,
                b"proof",
                &[b"field"],
            )
            .expect("proof");
        ENGINE
            .verify_mac_hex(
                &key,
                AuthenticationFraming::FramedDomain,
                b"proof",
                &[b"field"],
                &proof,
            )
            .expect("valid proof");
        assert_eq!(
            ENGINE.verify_mac_hex(
                &key,
                AuthenticationFraming::FramedDomain,
                b"proof",
                &[b"substituted"],
                &proof,
            ),
            Err(AuthenticatedJournalError::AuthenticationFailed)
        );
        assert!(decode_sha256_hex(&"A".repeat(64)).is_err());
        assert!(
            ENGINE
                .validate_counts(ENGINE.policy().max_active_records + 1, 0)
                .is_err()
        );
        assert!(
            ENGINE
                .validate_journal_bytes(ENGINE.policy().max_journal_bytes + 1)
                .is_err()
        );
        assert!(
            ENGINE
                .validate_record_bytes(ENGINE.policy().max_record_bytes + 1)
                .is_err()
        );
    }
}
