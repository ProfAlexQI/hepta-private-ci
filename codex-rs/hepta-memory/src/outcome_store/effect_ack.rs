use std::error::Error;
use std::fmt;

use hepta_contracts::ContentHash;
use sha2::Digest;
use sha2::Sha256;

const EFFECT_ACK_DOMAIN: &str = "hepta.memory.execution-effect-ack.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEffectAckParts {
    pub attempt_id: String,
    pub idempotency_key: String,
    pub effect_plan_hash: ContentHash,
    pub canonical_provider_ack: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEffectAck {
    attempt_id: String,
    idempotency_key: String,
    effect_plan_hash: ContentHash,
    canonical_provider_ack: String,
    ack_hash: ContentHash,
}

impl ExecutionEffectAck {
    pub fn try_new(parts: ExecutionEffectAckParts) -> Result<Self, ExecutionEffectAckError> {
        for (field, value) in [
            ("attempt_id", parts.attempt_id.as_str()),
            ("idempotency_key", parts.idempotency_key.as_str()),
            ("effect_plan_hash", parts.effect_plan_hash.as_str()),
            (
                "canonical_provider_ack",
                parts.canonical_provider_ack.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                return Err(ExecutionEffectAckError::EmptyField { field });
            }
        }
        let expected_prefix = format!("hepta-execution:{}:sha256:", parts.attempt_id);
        let digest = parts
            .idempotency_key
            .strip_prefix(&expected_prefix)
            .ok_or(ExecutionEffectAckError::IdempotencyBindingMismatch)?;
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ExecutionEffectAckError::IdempotencyBindingMismatch);
        }
        let ack_hash = framed_hash(
            EFFECT_ACK_DOMAIN,
            &[
                ("attempt_id", parts.attempt_id.as_str()),
                ("idempotency_key", parts.idempotency_key.as_str()),
                ("effect_plan_hash", parts.effect_plan_hash.as_str()),
                (
                    "canonical_provider_ack",
                    parts.canonical_provider_ack.as_str(),
                ),
            ],
        );
        Ok(Self {
            attempt_id: parts.attempt_id,
            idempotency_key: parts.idempotency_key,
            effect_plan_hash: parts.effect_plan_hash,
            canonical_provider_ack: parts.canonical_provider_ack,
            ack_hash,
        })
    }

    pub fn attempt_id(&self) -> &str {
        &self.attempt_id
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    pub fn effect_plan_hash(&self) -> &ContentHash {
        &self.effect_plan_hash
    }

    pub fn canonical_provider_ack(&self) -> &str {
        &self.canonical_provider_ack
    }

    pub fn ack_hash(&self) -> &ContentHash {
        &self.ack_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionEffectAckError {
    EmptyField { field: &'static str },
    IdempotencyBindingMismatch,
}

impl fmt::Display for ExecutionEffectAckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "execution effect ACK field {field} must not be empty"
                )
            }
            Self::IdempotencyBindingMismatch => formatter
                .write_str("execution effect ACK idempotency key is not bound to its attempt"),
        }
    }
}

impl Error for ExecutionEffectAckError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionEffectAckRecordResult {
    Recorded,
    AlreadyRecorded,
}

fn framed_hash(domain: &str, fields: &[(&str, &str)]) -> ContentHash {
    let mut hasher = Sha256::new();
    update_frame(&mut hasher, domain.as_bytes());
    for (name, value) in fields {
        update_frame(&mut hasher, name.as_bytes());
        update_frame(&mut hasher, value.as_bytes());
    }
    ContentHash::new(format!("sha256:{}", encode_hex(&hasher.finalize())))
}

fn update_frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
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
