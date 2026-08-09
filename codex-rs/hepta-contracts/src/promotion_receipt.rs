use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
#[cfg(test)]
use std::sync::Mutex;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use ed25519_dalek::Signature;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use serde::Deserialize;
use serde::Serialize;

use crate::Sha256Digest;

pub const PROMOTION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const PROMOTION_TRUST_ROOT_SCHEMA_VERSION: u32 = 1;
pub const PROMOTION_TRUST_ROOT_ROTATION_SCHEMA_VERSION: u32 = 1;
pub const PROMOTION_TRUST_HISTORY_SCHEMA_VERSION: u32 = 1;
pub const PROMOTION_REVOCATIONS_SCHEMA_VERSION: u32 = 1;
pub const SIGNED_PROMOTION_REVOCATIONS_SCHEMA_VERSION: u32 = 1;
pub const PROMOTION_HEAD_CHECKPOINT_SCHEMA_VERSION: u32 = 1;

/// Domain separation prefix for the V1 receipt digest preimage.
///
/// This is not a complete signature message. Valid signatures additionally
/// bind the signer key id and canonical algorithm code with
/// PROMOTION_RECEIPT_V1_SIGNER_BINDING_DOMAIN.
pub const PROMOTION_RECEIPT_V1_PREIMAGE_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-receipt/evidence-only/v1\0";
pub const PROMOTION_RECEIPT_V1_SIGNER_BINDING_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-receipt/signer-key-id/v1\0";
pub const PROMOTION_TRUST_ROOT_V1_DIGEST_DOMAIN: &[u8] = b"hepta.vnext.promotion-trust-root/v1\0";
pub const PROMOTION_TRUST_ROOT_ROTATION_V1_SIGNING_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-trust-root-rotation/v1\0";
pub const PROMOTION_REVOCATIONS_V1_DIGEST_DOMAIN: &[u8] = b"hepta.vnext.promotion-revocations/v1\0";
pub const PROMOTION_REVOCATIONS_V1_SIGNING_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-revocations-signature/v1\0";
pub const PROMOTION_HISTORY_CHAIN_GENESIS_V1_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-history-chain/genesis/v1\0";
pub const PROMOTION_HISTORY_CHAIN_UPDATE_V1_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-history-chain/update/v1\0";
pub const PROMOTION_HEAD_CHECKPOINT_V1_DIGEST_DOMAIN: &[u8] =
    b"hepta.vnext.promotion-head-checkpoint/v1\0";
pub const MAX_PROMOTION_PACKET_JSON_BYTES: usize = 256 * 1024;
pub const MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_PROMOTION_HEAD_CHECKPOINT_JSON_BYTES: usize = 64 * 1024;
pub const MAX_PROMOTION_SIGNATURES: usize = 16;
pub const MAX_QUALIFICATION_PLATFORM_RECEIPTS: usize = 16;
pub const MAX_TRUSTED_PROMOTION_KEYS: usize = 32;
pub const MAX_PROMOTION_TRUST_ROOT_ROTATIONS: usize = 128;
pub const MAX_PROMOTION_TRUST_HISTORY_UPDATES: usize = 4_096;
pub const MAX_PROMOTION_REVOCATIONS_PER_KIND: usize = 4_096;
pub const MAX_PROMOTION_DURABLE_INTEGER: u64 = i64::MAX as u64;

const CANONICAL_PAYLOAD_MAGIC: &[u8] = b"HPR1";
const CANONICAL_TRUST_ROOT_MAGIC: &[u8] = b"HTR1";
const CANONICAL_TRUST_ROOT_ROTATION_MAGIC: &[u8] = b"HRO1";
const CANONICAL_REVOCATIONS_MAGIC: &[u8] = b"HRV1";
const CANONICAL_SIGNED_REVOCATIONS_MAGIC: &[u8] = b"HSR1";
const CANONICAL_HISTORY_CHAIN_GENESIS_MAGIC: &[u8] = b"HCG1";
const CANONICAL_HISTORY_CHAIN_UPDATE_MAGIC: &[u8] = b"HCU1";
#[cfg(test)]
const CANONICAL_HEAD_CHECKPOINT_MAGIC: &[u8] = b"HCP1";
const MAX_IDENTIFIER_BYTES: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromotionReceiptError {
    Json(String),
    UnsupportedSchemaVersion {
        component: &'static str,
        found: u32,
    },
    InvalidField {
        field: &'static str,
        detail: &'static str,
    },
    NonCanonicalOrder(&'static str),
    DuplicateEntry(&'static str),
    AuthorityClaimed,
    InvalidTimeWindow,
    ReceiptNotYetValid,
    ReceiptExpired,
    ReceiptLifetimeTooLong,
    ReceiptPredatesTrustRoot,
    ReceiptPredatesRevocations,
    TrustedClockBeforeUnixEpoch,
    GenesisTrustRootMismatch,
    GenesisTrustRootInFuture,
    CheckpointJsonDigestMismatch,
    CheckpointTrustRootMismatch,
    CheckpointHeadMismatch,
    TrustRootRotationPredecessorMismatch,
    TrustRootRotationRevisionGap,
    TrustRootEffectiveTimeMismatch,
    TrustUpdateTimeRollback,
    TrustRootRotationInFuture,
    TrustRootKeyIdentityConflict(String),
    TrustRootKeyReintroduced(String),
    TrustRootKeyRevocationRemoved(String),
    RevocationSnapshotTrustRootMismatch,
    RevocationSnapshotInFuture,
    RevocationEffectiveTimeMismatch,
    TrustRootMismatch,
    TrustRootRevisionTooOld {
        minimum: u64,
        found: u64,
    },
    RevocationRevisionTooOld {
        minimum: u64,
        found: u64,
    },
    TrustRootRevisionRollback {
        highest_seen: u64,
        found: u64,
    },
    RevocationRevisionRollback {
        highest_seen: u64,
        found: u64,
    },
    TrustRootRevisionConflict {
        revision: u64,
    },
    RevocationRevisionConflict {
        revision: u64,
    },
    RevocationRevisionGap {
        expected: u64,
        found: u64,
    },
    RevocationTombstoneRemoved(&'static str),
    RevokedConfigurationSigner(String),
    TrustedConfigurationNotInitialized,
    TrustedConfigurationNotCurrent,
    GenesisAnchorConflict,
    TrustRootHistoryMismatch,
    ClockRollback {
        highest_seen: u64,
        found: u64,
    },
    BindingMismatch,
    UnknownSigner(String),
    InactiveSigner(String),
    RevokedSigner(String),
    MalformedPublicKey(String),
    WeakPublicKey(String),
    UnreachableTrustRootThreshold,
    MalformedSignature(String),
    SignatureVerificationFailed(String),
    SignatureThresholdNotMet {
        required: u16,
        verified: u16,
    },
    ReceiptRevoked,
    NonceRevoked,
    NonceReplay,
    ReceiptReplay,
    ReplayStoreFailure(String),
}

impl fmt::Display for PromotionReceiptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(detail) => write!(formatter, "invalid promotion receipt JSON: {detail}"),
            Self::UnsupportedSchemaVersion { component, found } => {
                write!(formatter, "unsupported {component} schema version {found}")
            }
            Self::InvalidField { field, detail } => {
                write!(formatter, "invalid {field}: {detail}")
            }
            Self::NonCanonicalOrder(collection) => {
                write!(formatter, "{collection} must be in strict canonical order")
            }
            Self::DuplicateEntry(collection) => {
                write!(formatter, "{collection} contains a duplicate entry")
            }
            Self::AuthorityClaimed => write!(
                formatter,
                "promotion evidence cannot claim operator acceptance or promotion authority"
            ),
            Self::InvalidTimeWindow => write!(formatter, "invalid receipt time window"),
            Self::ReceiptNotYetValid => write!(formatter, "receipt was issued in the future"),
            Self::ReceiptExpired => write!(formatter, "receipt has expired"),
            Self::ReceiptLifetimeTooLong => {
                write!(formatter, "receipt lifetime exceeds trust-root policy")
            }
            Self::ReceiptPredatesTrustRoot => {
                write!(formatter, "receipt predates the active trust-root revision")
            }
            Self::ReceiptPredatesRevocations => {
                write!(formatter, "receipt predates the active revocation revision")
            }
            Self::TrustedClockBeforeUnixEpoch => {
                write!(formatter, "trusted system clock is before the Unix epoch")
            }
            Self::GenesisTrustRootMismatch => {
                write!(
                    formatter,
                    "genesis trust root does not match the fixed digest anchor"
                )
            }
            Self::GenesisTrustRootInFuture => {
                write!(
                    formatter,
                    "genesis trust root becomes effective in the future"
                )
            }
            Self::CheckpointJsonDigestMismatch => {
                write!(
                    formatter,
                    "checkpoint JSON does not match its independent digest pin"
                )
            }
            Self::CheckpointTrustRootMismatch => {
                write!(
                    formatter,
                    "checkpoint trust-root identity does not match history"
                )
            }
            Self::CheckpointHeadMismatch => {
                write!(
                    formatter,
                    "verified history head does not match the pinned checkpoint"
                )
            }
            Self::TrustRootRotationPredecessorMismatch => {
                write!(
                    formatter,
                    "trust-root rotation predecessor digest does not match"
                )
            }
            Self::TrustRootRotationRevisionGap => {
                write!(
                    formatter,
                    "trust-root rotations must advance by exactly one revision"
                )
            }
            Self::TrustRootEffectiveTimeMismatch => {
                write!(
                    formatter,
                    "trust-root successor effective time does not match its signed update time"
                )
            }
            Self::TrustUpdateTimeRollback => {
                write!(
                    formatter,
                    "trusted-configuration update time moved backwards"
                )
            }
            Self::TrustRootRotationInFuture => {
                write!(formatter, "trust-root rotation was issued in the future")
            }
            Self::TrustRootKeyIdentityConflict(key_id) => {
                write!(
                    formatter,
                    "trust-root key id {key_id} changed public-key identity"
                )
            }
            Self::TrustRootKeyReintroduced(key_id) => {
                write!(
                    formatter,
                    "retired trust-root key id {key_id} was reintroduced"
                )
            }
            Self::TrustRootKeyRevocationRemoved(key_id) => {
                write!(
                    formatter,
                    "trust-root key revocation tombstone for {key_id} was removed"
                )
            }
            Self::RevocationSnapshotTrustRootMismatch => {
                write!(
                    formatter,
                    "revocation snapshot is not bound to the anchored trust root"
                )
            }
            Self::RevocationSnapshotInFuture => {
                write!(formatter, "revocation snapshot was issued in the future")
            }
            Self::RevocationEffectiveTimeMismatch => {
                write!(
                    formatter,
                    "revocation effective time does not match its signed update time"
                )
            }
            Self::TrustRootMismatch => write!(formatter, "receipt trust root does not match"),
            Self::TrustRootRevisionTooOld { minimum, found } => write!(
                formatter,
                "trust-root revision {found} is below signed minimum {minimum}"
            ),
            Self::RevocationRevisionTooOld { minimum, found } => write!(
                formatter,
                "revocation revision {found} is below signed minimum {minimum}"
            ),
            Self::TrustRootRevisionRollback {
                highest_seen,
                found,
            } => write!(
                formatter,
                "trust-root revision rollback from {highest_seen} to {found}"
            ),
            Self::RevocationRevisionRollback {
                highest_seen,
                found,
            } => write!(
                formatter,
                "revocation revision rollback from {highest_seen} to {found}"
            ),
            Self::TrustRootRevisionConflict { revision } => write!(
                formatter,
                "trust-root revision {revision} has conflicting canonical content"
            ),
            Self::RevocationRevisionConflict { revision } => write!(
                formatter,
                "revocation revision {revision} has conflicting canonical content"
            ),
            Self::RevocationRevisionGap { expected, found } => write!(
                formatter,
                "revocation revision must advance to {expected}, found {found}"
            ),
            Self::RevocationTombstoneRemoved(kind) => {
                write!(formatter, "revocation update removed a {kind} tombstone")
            }
            Self::RevokedConfigurationSigner(key_id) => write!(
                formatter,
                "previously revoked key {key_id} signed a trusted-configuration update"
            ),
            Self::TrustedConfigurationNotInitialized => {
                write!(
                    formatter,
                    "trusted configuration is not initialized in durable state"
                )
            }
            Self::TrustedConfigurationNotCurrent => write!(
                formatter,
                "anchored trusted configuration is no longer current; re-anchor and retry"
            ),
            Self::GenesisAnchorConflict => {
                write!(
                    formatter,
                    "fixed genesis digest conflicts with durable state"
                )
            }
            Self::TrustRootHistoryMismatch => {
                write!(
                    formatter,
                    "trust-root chain does not extend durable history"
                )
            }
            Self::ClockRollback {
                highest_seen,
                found,
            } => write!(
                formatter,
                "verification clock rollback from {highest_seen} to {found}"
            ),
            Self::BindingMismatch => write!(formatter, "exact promotion bindings do not match"),
            Self::UnknownSigner(key_id) => write!(formatter, "unknown signer key id {key_id}"),
            Self::InactiveSigner(key_id) => write!(formatter, "inactive signer key id {key_id}"),
            Self::RevokedSigner(key_id) => write!(formatter, "revoked signer key id {key_id}"),
            Self::MalformedPublicKey(key_id) => {
                write!(formatter, "malformed Ed25519 public key for {key_id}")
            }
            Self::WeakPublicKey(key_id) => {
                write!(formatter, "weak Ed25519 public key for {key_id}")
            }
            Self::UnreachableTrustRootThreshold => {
                write!(
                    formatter,
                    "trust-root threshold is unreachable at every effective time"
                )
            }
            Self::MalformedSignature(key_id) => {
                write!(formatter, "malformed Ed25519 signature for {key_id}")
            }
            Self::SignatureVerificationFailed(key_id) => {
                write!(formatter, "signature verification failed for {key_id}")
            }
            Self::SignatureThresholdNotMet { required, verified } => write!(
                formatter,
                "signature threshold not met: required {required}, verified {verified}"
            ),
            Self::ReceiptRevoked => write!(formatter, "receipt digest is revoked"),
            Self::NonceRevoked => write!(formatter, "receipt nonce is revoked"),
            Self::NonceReplay => write!(formatter, "receipt nonce was already consumed"),
            Self::ReceiptReplay => write!(formatter, "receipt digest was already consumed"),
            Self::ReplayStoreFailure(detail) => {
                write!(formatter, "replay store failed closed: {detail}")
            }
        }
    }
}

impl StdError for PromotionReceiptError {}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct GitObjectId(String);

impl GitObjectId {
    pub fn parse(value: impl Into<String>) -> Result<Self, PromotionReceiptError> {
        let value = value.into();
        validate_git_object_id(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_git_object_id(&self.0)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionSignatureAlgorithm {
    Ed25519,
}

impl PromotionSignatureAlgorithm {
    const fn canonical_code(self) -> u8 {
        match self {
            Self::Ed25519 => 1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateBindingV1 {
    pub commit: GitObjectId,
    pub tree: GitObjectId,
}

impl CandidateBindingV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        self.commit.validate()?;
        self.tree.validate()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationPlatformReceiptV1 {
    pub platform: String,
    pub receipt_sha256: Sha256Digest,
}

impl QualificationPlatformReceiptV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_identifier("qualification platform", &self.platform)?;
        validate_sha256(
            "qualification platform receipt SHA-256",
            &self.receipt_sha256,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationBindingV1 {
    pub manifest_sha256: Sha256Digest,
    /// Entries must be sorted by platform and contain no duplicates.
    pub platform_receipts: Vec<QualificationPlatformReceiptV1>,
}

impl QualificationBindingV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_sha256("qualification manifest SHA-256", &self.manifest_sha256)?;
        if self.platform_receipts.is_empty() {
            return Err(PromotionReceiptError::InvalidField {
                field: "qualification platform receipts",
                detail: "at least one exact platform receipt is required",
            });
        }
        validate_collection_bound(
            "qualification platform receipts",
            self.platform_receipts.len(),
            MAX_QUALIFICATION_PLATFORM_RECEIPTS,
        )?;
        validate_strict_order(
            &self.platform_receipts,
            |entry| entry.platform.as_str(),
            "qualification platform receipts",
        )?;
        for entry in &self.platform_receipts {
            entry.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenOracleBindingV1 {
    pub commit: GitObjectId,
    pub tree: GitObjectId,
    pub manifest_sha256: Sha256Digest,
}

impl FrozenOracleBindingV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        self.commit.validate()?;
        self.tree.validate()?;
        validate_sha256("frozen oracle manifest SHA-256", &self.manifest_sha256)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalOracleConformanceBindingV1 {
    pub manifest_sha256: Sha256Digest,
}

impl CanonicalOracleConformanceBindingV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_sha256(
            "canonical-oracle conformance manifest SHA-256",
            &self.manifest_sha256,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductShadowSoakBindingV1 {
    pub manifest_sha256: Sha256Digest,
}

impl ProductShadowSoakBindingV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_sha256(
            "product Shadow soak manifest SHA-256",
            &self.manifest_sha256,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionBindingsV1 {
    pub candidate: CandidateBindingV1,
    pub qualification: QualificationBindingV1,
    pub frozen_oracle: FrozenOracleBindingV1,
    pub canonical_oracle_conformance: CanonicalOracleConformanceBindingV1,
    pub product_shadow_soak: ProductShadowSoakBindingV1,
}

impl PromotionBindingsV1 {
    /// Validates shape and canonical ordering only. The required platform set
    /// is external release policy, not schema policy.
    pub fn validate(&self) -> Result<(), PromotionReceiptError> {
        self.candidate.validate()?;
        self.qualification.validate()?;
        self.frozen_oracle.validate()?;
        self.canonical_oracle_conformance.validate()?;
        self.product_shadow_soak.validate()
    }
}

/// Signed evidence template. V1 deliberately carries no promotion authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionReceiptV1 {
    pub schema_version: u32,
    pub trust_root_id: String,
    pub minimum_trust_root_revision: u64,
    pub minimum_revocation_revision: u64,
    pub bindings: PromotionBindingsV1,
    pub issued_at_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub nonce: String,
    pub operator_accepted: bool,
    pub promotion_authorized: bool,
}

impl PromotionReceiptV1 {
    pub fn evidence_only_template(
        trust_root_id: impl Into<String>,
        minimum_trust_root_revision: u64,
        minimum_revocation_revision: u64,
        bindings: PromotionBindingsV1,
        issued_at_unix_seconds: u64,
        expires_at_unix_seconds: u64,
        nonce: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: PROMOTION_RECEIPT_SCHEMA_VERSION,
            trust_root_id: trust_root_id.into(),
            minimum_trust_root_revision,
            minimum_revocation_revision,
            bindings,
            issued_at_unix_seconds,
            expires_at_unix_seconds,
            nonce: nonce.into(),
            operator_accepted: false,
            promotion_authorized: false,
        }
    }

    pub fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion receipt",
            self.schema_version,
            PROMOTION_RECEIPT_SCHEMA_VERSION,
        )?;
        validate_identifier("trust root id", &self.trust_root_id)?;
        for (field, value) in [
            (
                "minimum trust-root revision",
                self.minimum_trust_root_revision,
            ),
            (
                "minimum revocation revision",
                self.minimum_revocation_revision,
            ),
            ("issued-at Unix seconds", self.issued_at_unix_seconds),
            ("expires-at Unix seconds", self.expires_at_unix_seconds),
        ] {
            validate_durable_integer(field, value)?;
        }
        if self.minimum_trust_root_revision == 0 || self.minimum_revocation_revision == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "minimum trust revisions",
                detail: "signed minimum revisions must be nonzero",
            });
        }
        self.bindings.validate()?;
        if self.issued_at_unix_seconds == 0
            || self.expires_at_unix_seconds <= self.issued_at_unix_seconds
        {
            return Err(PromotionReceiptError::InvalidTimeWindow);
        }
        validate_fixed_hex("receipt nonce", &self.nonce, 64)?;
        if self.operator_accepted || self.promotion_authorized {
            return Err(PromotionReceiptError::AuthorityClaimed);
        }
        Ok(())
    }

    /// Returns the common, unique, domain-separated V1 receipt digest preimage.
    ///
    /// This value must not be signed directly. Only
    /// canonical_signing_bytes_for_signer, or the safer
    /// sign_promotion_receipt_v1 helper, produces a complete signature input.
    pub fn canonical_receipt_preimage(&self) -> Result<Vec<u8>, PromotionReceiptError> {
        self.validate()?;
        Ok(self.canonical_receipt_preimage_unchecked())
    }

    pub fn canonical_signing_bytes_for_signer(
        &self,
        key_id: &str,
        algorithm: PromotionSignatureAlgorithm,
    ) -> Result<Vec<u8>, PromotionReceiptError> {
        self.validate()?;
        validate_identifier("signer key id", key_id)?;
        Ok(self.canonical_signing_bytes_for_signer_unchecked(key_id, algorithm))
    }

    pub fn receipt_sha256(&self) -> Result<Sha256Digest, PromotionReceiptError> {
        Ok(Sha256Digest::for_bytes(&self.canonical_receipt_preimage()?))
    }

    fn canonical_receipt_preimage_unchecked(&self) -> Vec<u8> {
        let payload = self.canonical_payload_bytes_unchecked();
        let mut message =
            Vec::with_capacity(PROMOTION_RECEIPT_V1_PREIMAGE_DOMAIN.len() + payload.len());
        message.extend_from_slice(PROMOTION_RECEIPT_V1_PREIMAGE_DOMAIN);
        message.extend_from_slice(&payload);
        message
    }

    fn canonical_signing_bytes_for_signer_unchecked(
        &self,
        key_id: &str,
        algorithm: PromotionSignatureAlgorithm,
    ) -> Vec<u8> {
        let mut message = self.canonical_receipt_preimage_unchecked();
        message.extend_from_slice(PROMOTION_RECEIPT_V1_SIGNER_BINDING_DOMAIN);
        put_string(&mut message, 1, key_id);
        put_u8(&mut message, 2, algorithm.canonical_code());
        message
    }

    fn canonical_payload_bytes_unchecked(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(CANONICAL_PAYLOAD_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, &self.trust_root_id);
        put_u64(&mut bytes, 3, self.minimum_trust_root_revision);
        put_u64(&mut bytes, 4, self.minimum_revocation_revision);
        put_string(&mut bytes, 5, self.bindings.candidate.commit.as_str());
        put_string(&mut bytes, 6, self.bindings.candidate.tree.as_str());
        put_string(
            &mut bytes,
            7,
            self.bindings.qualification.manifest_sha256.as_str(),
        );
        let mut platform_receipts = Vec::new();
        put_u32(
            &mut platform_receipts,
            1,
            self.bindings.qualification.platform_receipts.len() as u32,
        );
        for platform_receipt in &self.bindings.qualification.platform_receipts {
            let mut frame = Vec::new();
            put_string(&mut frame, 1, &platform_receipt.platform);
            put_string(&mut frame, 2, platform_receipt.receipt_sha256.as_str());
            put_bytes(&mut platform_receipts, 2, &frame);
        }
        put_bytes(&mut bytes, 8, &platform_receipts);
        put_string(&mut bytes, 9, self.bindings.frozen_oracle.commit.as_str());
        put_string(&mut bytes, 10, self.bindings.frozen_oracle.tree.as_str());
        put_string(
            &mut bytes,
            11,
            self.bindings.frozen_oracle.manifest_sha256.as_str(),
        );
        put_string(
            &mut bytes,
            12,
            self.bindings
                .canonical_oracle_conformance
                .manifest_sha256
                .as_str(),
        );
        put_string(
            &mut bytes,
            13,
            self.bindings.product_shadow_soak.manifest_sha256.as_str(),
        );
        put_u64(&mut bytes, 14, self.issued_at_unix_seconds);
        put_u64(&mut bytes, 15, self.expires_at_unix_seconds);
        put_string(&mut bytes, 16, &self.nonce);
        put_bool(&mut bytes, 17, self.operator_accepted);
        put_bool(&mut bytes, 18, self.promotion_authorized);
        bytes
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionSignatureV1 {
    pub key_id: String,
    pub algorithm: PromotionSignatureAlgorithm,
    pub signature_hex: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedPromotionReceiptV1 {
    pub receipt: PromotionReceiptV1,
    /// Entries must be sorted by key id and contain no duplicates.
    pub signatures: Vec<PromotionSignatureV1>,
}

impl SignedPromotionReceiptV1 {
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self, PromotionReceiptError> {
        validate_byte_bound(
            "signed promotion receipt JSON",
            bytes.len(),
            MAX_PROMOTION_PACKET_JSON_BYTES,
        )?;
        serde_json::from_slice(bytes)
            .map_err(|error| PromotionReceiptError::Json(error.to_string()))
    }

    pub fn validate_structure(&self) -> Result<(), PromotionReceiptError> {
        self.receipt.validate()?;
        if self.signatures.is_empty() {
            return Err(PromotionReceiptError::InvalidField {
                field: "promotion signatures",
                detail: "at least one signature is required",
            });
        }
        validate_collection_bound(
            "promotion signatures",
            self.signatures.len(),
            MAX_PROMOTION_SIGNATURES,
        )?;
        validate_strict_order(
            &self.signatures,
            |signature| signature.key_id.as_str(),
            "promotion signatures",
        )?;
        for signature in &self.signatures {
            validate_identifier("signer key id", &signature.key_id)?;
            validate_fixed_hex("Ed25519 signature", &signature.signature_hex, 128)?;
        }
        Ok(())
    }
}

pub struct PromotionSignerV1<'a> {
    pub key_id: &'a str,
    pub signing_key: &'a SigningKey,
}

pub fn sign_promotion_receipt_v1(
    receipt: PromotionReceiptV1,
    signers: &[PromotionSignerV1<'_>],
) -> Result<SignedPromotionReceiptV1, PromotionReceiptError> {
    receipt.validate()?;
    if signers.is_empty() {
        return Err(PromotionReceiptError::InvalidField {
            field: "promotion signers",
            detail: "at least one signer is required",
        });
    }
    validate_collection_bound("promotion signers", signers.len(), MAX_PROMOTION_SIGNATURES)?;
    validate_strict_order(signers, |signer| signer.key_id, "promotion signers")?;
    let mut signatures = Vec::with_capacity(signers.len());
    for signer in signers {
        validate_identifier("signer key id", signer.key_id)?;
        let algorithm = PromotionSignatureAlgorithm::Ed25519;
        let message =
            receipt.canonical_signing_bytes_for_signer_unchecked(signer.key_id, algorithm);
        signatures.push(PromotionSignatureV1 {
            key_id: signer.key_id.to_string(),
            algorithm,
            signature_hex: encode_hex(&signer.signing_key.sign(&message).to_bytes()),
        });
    }
    Ok(SignedPromotionReceiptV1 {
        receipt,
        signatures,
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedPromotionKeyV1 {
    pub key_id: String,
    pub algorithm: PromotionSignatureAlgorithm,
    pub public_key_hex: String,
    pub valid_from_unix_seconds: u64,
    pub valid_until_unix_seconds: u64,
    pub revoked_at_unix_seconds: Option<u64>,
}

impl TrustedPromotionKeyV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_identifier("trusted signer key id", &self.key_id)?;
        validate_fixed_hex("Ed25519 public key", &self.public_key_hex, 64)?;
        let public_key_bytes = decode_hex::<32>(&self.public_key_hex)
            .map_err(|_| PromotionReceiptError::MalformedPublicKey(self.key_id.clone()))?;
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|_| PromotionReceiptError::MalformedPublicKey(self.key_id.clone()))?;
        if verifying_key.is_weak() {
            return Err(PromotionReceiptError::WeakPublicKey(self.key_id.clone()));
        }
        for (field, value) in [
            ("trusted key valid-from", self.valid_from_unix_seconds),
            ("trusted key valid-until", self.valid_until_unix_seconds),
        ] {
            validate_durable_integer(field, value)?;
        }
        if let Some(revoked_at) = self.revoked_at_unix_seconds {
            validate_durable_integer("trusted key revoked-at", revoked_at)?;
        }
        if self.valid_from_unix_seconds == 0
            || self.valid_until_unix_seconds <= self.valid_from_unix_seconds
        {
            return Err(PromotionReceiptError::InvalidField {
                field: "trusted signer validity",
                detail: "valid_until must be greater than a nonzero valid_from",
            });
        }
        Ok(())
    }

    fn active_at(&self, issued_at: u64, now: u64) -> bool {
        self.valid_from_unix_seconds <= issued_at
            && issued_at < self.valid_until_unix_seconds
            && self.valid_from_unix_seconds <= now
            && now < self.valid_until_unix_seconds
    }

    fn active_for_configuration_signature(&self, issued_at: u64) -> bool {
        self.valid_from_unix_seconds <= issued_at && issued_at < self.valid_until_unix_seconds
    }

    fn revoked_at(&self, now: u64) -> bool {
        self.revoked_at_unix_seconds
            .is_some_and(|revoked_at| now >= revoked_at)
    }
}

/// One versioned trust-root configuration. A bare instance is never sufficient
/// trust: genesis is selected by a fixed out-of-band digest and every higher
/// revision must be proven by a complete predecessor-signed history.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionTrustRootV1 {
    pub schema_version: u32,
    pub trust_root_id: String,
    pub revision: u64,
    /// The first Unix second at which this exact root revision may authorize
    /// receipts. For successors this must equal the signed rotation time.
    pub effective_at_unix_seconds: u64,
    pub signature_threshold: u16,
    pub max_receipt_lifetime_seconds: u64,
    /// Entries must be sorted by key id and public keys must also be unique.
    pub keys: Vec<TrustedPromotionKeyV1>,
}

impl PromotionTrustRootV1 {
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self, PromotionReceiptError> {
        validate_byte_bound(
            "promotion trust-root JSON",
            bytes.len(),
            MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES,
        )?;
        serde_json::from_slice(bytes)
            .map_err(|error| PromotionReceiptError::Json(error.to_string()))
    }

    pub fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion trust root",
            self.schema_version,
            PROMOTION_TRUST_ROOT_SCHEMA_VERSION,
        )?;
        validate_identifier("trust root id", &self.trust_root_id)?;
        validate_durable_integer("trust-root revision", self.revision)?;
        validate_durable_integer("trust-root effective-at", self.effective_at_unix_seconds)?;
        validate_durable_integer(
            "maximum receipt lifetime",
            self.max_receipt_lifetime_seconds,
        )?;
        if self.revision == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "trust-root revision",
                detail: "revision must be nonzero",
            });
        }
        if self.effective_at_unix_seconds == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "trust-root effective-at",
                detail: "timestamp must be nonzero",
            });
        }
        if self.keys.is_empty()
            || self.signature_threshold == 0
            || usize::from(self.signature_threshold) > self.keys.len()
            || usize::from(self.signature_threshold) > MAX_PROMOTION_SIGNATURES
        {
            return Err(PromotionReceiptError::InvalidField {
                field: "signature threshold",
                detail: "threshold must fit both the trusted key count and signature hard limit",
            });
        }
        validate_collection_bound(
            "trusted promotion keys",
            self.keys.len(),
            MAX_TRUSTED_PROMOTION_KEYS,
        )?;
        if self.max_receipt_lifetime_seconds == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "maximum receipt lifetime",
                detail: "lifetime must be nonzero",
            });
        }
        validate_strict_order(
            &self.keys,
            |key| key.key_id.as_str(),
            "trusted promotion keys",
        )?;
        let mut public_keys = BTreeSet::new();
        for key in &self.keys {
            key.validate()?;
            if !public_keys.insert(key.public_key_hex.as_str()) {
                return Err(PromotionReceiptError::DuplicateEntry(
                    "trusted promotion public keys",
                ));
            }
        }
        validate_root_threshold_reachable(self)?;
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, PromotionReceiptError> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PROMOTION_TRUST_ROOT_V1_DIGEST_DOMAIN);
        bytes.extend_from_slice(CANONICAL_TRUST_ROOT_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, &self.trust_root_id);
        put_u64(&mut bytes, 3, self.revision);
        put_u64(&mut bytes, 4, self.effective_at_unix_seconds);
        put_u16(&mut bytes, 5, self.signature_threshold);
        put_u64(&mut bytes, 6, self.max_receipt_lifetime_seconds);
        let mut keys = Vec::new();
        put_u32(&mut keys, 1, self.keys.len() as u32);
        for key in &self.keys {
            let mut frame = Vec::new();
            put_string(&mut frame, 1, &key.key_id);
            put_u8(&mut frame, 2, key.algorithm.canonical_code());
            put_string(&mut frame, 3, &key.public_key_hex);
            put_u64(&mut frame, 4, key.valid_from_unix_seconds);
            put_u64(&mut frame, 5, key.valid_until_unix_seconds);
            put_bool(&mut frame, 6, key.revoked_at_unix_seconds.is_some());
            if let Some(revoked_at) = key.revoked_at_unix_seconds {
                put_u64(&mut frame, 7, revoked_at);
            }
            put_bytes(&mut keys, 2, &frame);
        }
        put_bytes(&mut bytes, 7, &keys);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

/// A threshold-signed transition from one complete trusted state to the next
/// trust-root revision. The predecessor revocation state is part of the signed
/// message so a revoked predecessor key cannot authorize a rotation by
/// presenting an older revocation snapshot.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionTrustRootRotationV1 {
    pub schema_version: u32,
    pub predecessor_trust_root_sha256: Sha256Digest,
    pub predecessor_revocation_revision: u64,
    pub predecessor_revocations_sha256: Sha256Digest,
    pub successor: PromotionTrustRootV1,
    pub issued_at_unix_seconds: u64,
    /// Entries must be sorted by key id and contain no duplicates.
    pub signatures: Vec<PromotionSignatureV1>,
}

impl PromotionTrustRootRotationV1 {
    pub fn validate_structure(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion trust-root rotation",
            self.schema_version,
            PROMOTION_TRUST_ROOT_ROTATION_SCHEMA_VERSION,
        )?;
        validate_sha256(
            "rotation predecessor trust-root SHA-256",
            &self.predecessor_trust_root_sha256,
        )?;
        validate_durable_integer(
            "rotation predecessor revocation revision",
            self.predecessor_revocation_revision,
        )?;
        if self.predecessor_revocation_revision == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "rotation predecessor revocation revision",
                detail: "a root rotation requires initialized revocation state",
            });
        }
        validate_sha256(
            "rotation predecessor revocations SHA-256",
            &self.predecessor_revocations_sha256,
        )?;
        self.successor.validate()?;
        validate_durable_integer("root rotation issued-at", self.issued_at_unix_seconds)?;
        if self.issued_at_unix_seconds == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "root rotation issued-at",
                detail: "timestamp must be nonzero",
            });
        }
        if self.successor.effective_at_unix_seconds != self.issued_at_unix_seconds {
            return Err(PromotionReceiptError::TrustRootEffectiveTimeMismatch);
        }
        validate_signatures_structure(&self.signatures, "trust-root rotation signatures")
    }

    fn canonical_statement(&self) -> Result<Vec<u8>, PromotionReceiptError> {
        let successor_sha256 = self.successor.canonical_sha256()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PROMOTION_TRUST_ROOT_ROTATION_V1_SIGNING_DOMAIN);
        bytes.extend_from_slice(CANONICAL_TRUST_ROOT_ROTATION_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, self.predecessor_trust_root_sha256.as_str());
        put_u64(&mut bytes, 3, self.predecessor_revocation_revision);
        put_string(&mut bytes, 4, self.predecessor_revocations_sha256.as_str());
        put_string(&mut bytes, 5, successor_sha256.as_str());
        put_u64(&mut bytes, 6, self.issued_at_unix_seconds);
        Ok(bytes)
    }

    pub fn canonical_signing_bytes_for_signer(
        &self,
        key_id: &str,
        algorithm: PromotionSignatureAlgorithm,
    ) -> Result<Vec<u8>, PromotionReceiptError> {
        self.validate_structure()?;
        validate_identifier("rotation signer key id", key_id)?;
        Ok(key_specific_signing_message(
            self.canonical_statement()?,
            key_id,
            algorithm,
        ))
    }
}

pub fn sign_promotion_trust_root_rotation_v1(
    predecessor: &PromotionTrustRootV1,
    predecessor_revocations: &PromotionRevocationsV1,
    successor: PromotionTrustRootV1,
    issued_at_unix_seconds: u64,
    signers: &[PromotionSignerV1<'_>],
) -> Result<PromotionTrustRootRotationV1, PromotionReceiptError> {
    predecessor.validate()?;
    predecessor_revocations.validate()?;
    validate_root_successor(predecessor, &successor)?;
    if successor.effective_at_unix_seconds != issued_at_unix_seconds {
        return Err(PromotionReceiptError::TrustRootEffectiveTimeMismatch);
    }
    if issued_at_unix_seconds < predecessor_revocations.effective_at_unix_seconds {
        return Err(PromotionReceiptError::TrustUpdateTimeRollback);
    }
    if predecessor.trust_root_id != predecessor_revocations.trust_root_id {
        return Err(PromotionReceiptError::TrustRootMismatch);
    }
    let mut rotation = PromotionTrustRootRotationV1 {
        schema_version: PROMOTION_TRUST_ROOT_ROTATION_SCHEMA_VERSION,
        predecessor_trust_root_sha256: predecessor.canonical_sha256()?,
        predecessor_revocation_revision: predecessor_revocations.revision,
        predecessor_revocations_sha256: predecessor_revocations.canonical_sha256()?,
        successor,
        issued_at_unix_seconds,
        signatures: Vec::new(),
    };
    validate_signers(signers, "trust-root rotation signers")?;
    let statement = rotation.canonical_statement()?;
    rotation.signatures = sign_statement(statement, signers);
    rotation.validate_structure()?;
    Ok(rotation)
}

/// Cumulative revocation state. It is accepted only inside the complete signed
/// history and independently ratcheted with the trust root; it is never read
/// from the untrusted receipt packet.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionRevocationsV1 {
    pub schema_version: u32,
    pub trust_root_id: String,
    pub revision: u64,
    /// The first Unix second at which this exact cumulative snapshot applies.
    /// It must equal the signed revocation-update time.
    pub effective_at_unix_seconds: u64,
    /// Every list must be strictly sorted and duplicate-free.
    pub revoked_key_ids: Vec<String>,
    pub revoked_receipt_sha256: Vec<Sha256Digest>,
    pub revoked_nonces: Vec<String>,
}

impl PromotionRevocationsV1 {
    pub fn empty(
        trust_root_id: impl Into<String>,
        revision: u64,
        effective_at_unix_seconds: u64,
    ) -> Self {
        Self {
            schema_version: PROMOTION_REVOCATIONS_SCHEMA_VERSION,
            trust_root_id: trust_root_id.into(),
            revision,
            effective_at_unix_seconds,
            revoked_key_ids: Vec::new(),
            revoked_receipt_sha256: Vec::new(),
            revoked_nonces: Vec::new(),
        }
    }

    pub fn from_json_slice(bytes: &[u8]) -> Result<Self, PromotionReceiptError> {
        validate_byte_bound(
            "promotion revocations JSON",
            bytes.len(),
            MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES,
        )?;
        serde_json::from_slice(bytes)
            .map_err(|error| PromotionReceiptError::Json(error.to_string()))
    }

    pub fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion revocations",
            self.schema_version,
            PROMOTION_REVOCATIONS_SCHEMA_VERSION,
        )?;
        validate_identifier("revocation trust root id", &self.trust_root_id)?;
        validate_durable_integer("revocation revision", self.revision)?;
        validate_durable_integer("revocation effective-at", self.effective_at_unix_seconds)?;
        if self.revision == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "revocation revision",
                detail: "revision must be nonzero",
            });
        }
        if self.effective_at_unix_seconds == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "revocation effective-at",
                detail: "timestamp must be nonzero",
            });
        }
        for (field, length) in [
            ("revoked signer key ids", self.revoked_key_ids.len()),
            ("revoked receipt digests", self.revoked_receipt_sha256.len()),
            ("revoked receipt nonces", self.revoked_nonces.len()),
        ] {
            validate_collection_bound(field, length, MAX_PROMOTION_REVOCATIONS_PER_KIND)?;
        }
        validate_strict_order(
            &self.revoked_key_ids,
            String::as_str,
            "revoked signer key ids",
        )?;
        validate_strict_order(
            &self.revoked_receipt_sha256,
            Sha256Digest::as_str,
            "revoked receipt digests",
        )?;
        validate_strict_order(
            &self.revoked_nonces,
            String::as_str,
            "revoked receipt nonces",
        )?;
        for key_id in &self.revoked_key_ids {
            validate_identifier("revoked signer key id", key_id)?;
        }
        for digest in &self.revoked_receipt_sha256 {
            validate_sha256("revoked receipt SHA-256", digest)?;
        }
        for nonce in &self.revoked_nonces {
            validate_fixed_hex("revoked receipt nonce", nonce, 64)?;
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, PromotionReceiptError> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PROMOTION_REVOCATIONS_V1_DIGEST_DOMAIN);
        bytes.extend_from_slice(CANONICAL_REVOCATIONS_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, &self.trust_root_id);
        put_u64(&mut bytes, 3, self.revision);
        put_u64(&mut bytes, 4, self.effective_at_unix_seconds);
        put_string_collection(&mut bytes, 5, &self.revoked_key_ids);
        put_digest_collection(&mut bytes, 6, &self.revoked_receipt_sha256);
        put_string_collection(&mut bytes, 7, &self.revoked_nonces);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

/// A threshold-signed cumulative revocation update. Revision one has no
/// predecessor revocation digest; every later revision binds the immediately
/// preceding full trusted state and must preserve all tombstones.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedPromotionRevocationsV1 {
    pub schema_version: u32,
    pub predecessor_trust_root_sha256: Sha256Digest,
    pub predecessor_revocation_revision: u64,
    pub predecessor_revocations_sha256: Option<Sha256Digest>,
    pub revocations: PromotionRevocationsV1,
    pub issued_at_unix_seconds: u64,
    /// Entries must be sorted by key id and contain no duplicates.
    pub signatures: Vec<PromotionSignatureV1>,
}

impl SignedPromotionRevocationsV1 {
    pub fn validate_structure(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "signed promotion revocations",
            self.schema_version,
            SIGNED_PROMOTION_REVOCATIONS_SCHEMA_VERSION,
        )?;
        validate_sha256(
            "revocation predecessor trust-root SHA-256",
            &self.predecessor_trust_root_sha256,
        )?;
        validate_durable_integer(
            "revocation predecessor revision",
            self.predecessor_revocation_revision,
        )?;
        match (
            self.predecessor_revocation_revision,
            &self.predecessor_revocations_sha256,
        ) {
            (0, None) => {}
            (0, Some(_)) => {
                return Err(PromotionReceiptError::InvalidField {
                    field: "revocation predecessor digest",
                    detail: "revision zero must not carry a predecessor digest",
                });
            }
            (_, None) => {
                return Err(PromotionReceiptError::InvalidField {
                    field: "revocation predecessor digest",
                    detail: "nonzero predecessor revision requires a digest",
                });
            }
            (_, Some(digest)) => validate_sha256("revocation predecessor SHA-256", digest)?,
        }
        self.revocations.validate()?;
        validate_durable_integer("revocation snapshot issued-at", self.issued_at_unix_seconds)?;
        if self.issued_at_unix_seconds == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "revocation snapshot issued-at",
                detail: "timestamp must be nonzero",
            });
        }
        if self.revocations.effective_at_unix_seconds != self.issued_at_unix_seconds {
            return Err(PromotionReceiptError::RevocationEffectiveTimeMismatch);
        }
        validate_signatures_structure(&self.signatures, "revocation snapshot signatures")
    }

    fn canonical_statement(&self) -> Result<Vec<u8>, PromotionReceiptError> {
        let revocations_sha256 = self.revocations.canonical_sha256()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PROMOTION_REVOCATIONS_V1_SIGNING_DOMAIN);
        bytes.extend_from_slice(CANONICAL_SIGNED_REVOCATIONS_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, self.predecessor_trust_root_sha256.as_str());
        put_u64(&mut bytes, 3, self.predecessor_revocation_revision);
        put_bool(&mut bytes, 4, self.predecessor_revocations_sha256.is_some());
        if let Some(predecessor) = &self.predecessor_revocations_sha256 {
            put_string(&mut bytes, 5, predecessor.as_str());
        }
        put_string(&mut bytes, 6, revocations_sha256.as_str());
        put_u64(&mut bytes, 7, self.issued_at_unix_seconds);
        Ok(bytes)
    }

    pub fn canonical_signing_bytes_for_signer(
        &self,
        key_id: &str,
        algorithm: PromotionSignatureAlgorithm,
    ) -> Result<Vec<u8>, PromotionReceiptError> {
        self.validate_structure()?;
        validate_identifier("revocation signer key id", key_id)?;
        Ok(key_specific_signing_message(
            self.canonical_statement()?,
            key_id,
            algorithm,
        ))
    }
}

pub fn sign_promotion_revocations_v1(
    trust_root: &PromotionTrustRootV1,
    predecessor: Option<&PromotionRevocationsV1>,
    revocations: PromotionRevocationsV1,
    issued_at_unix_seconds: u64,
    signers: &[PromotionSignerV1<'_>],
) -> Result<SignedPromotionRevocationsV1, PromotionReceiptError> {
    trust_root.validate()?;
    revocations.validate()?;
    if revocations.effective_at_unix_seconds != issued_at_unix_seconds {
        return Err(PromotionReceiptError::RevocationEffectiveTimeMismatch);
    }
    if revocations.trust_root_id != trust_root.trust_root_id {
        return Err(PromotionReceiptError::TrustRootMismatch);
    }
    if revocations.effective_at_unix_seconds < trust_root.effective_at_unix_seconds {
        return Err(PromotionReceiptError::TrustUpdateTimeRollback);
    }
    let (predecessor_revocation_revision, predecessor_revocations_sha256) =
        if let Some(predecessor) = predecessor {
            predecessor.validate()?;
            if predecessor.trust_root_id != trust_root.trust_root_id {
                return Err(PromotionReceiptError::TrustRootMismatch);
            }
            let expected =
                predecessor
                    .revision
                    .checked_add(1)
                    .ok_or(PromotionReceiptError::InvalidField {
                        field: "revocation revision",
                        detail: "revision overflow",
                    })?;
            if revocations.revision != expected {
                return Err(PromotionReceiptError::RevocationRevisionGap {
                    expected,
                    found: revocations.revision,
                });
            }
            ensure_revocation_superset(predecessor, &revocations)?;
            if revocations.effective_at_unix_seconds < predecessor.effective_at_unix_seconds {
                return Err(PromotionReceiptError::TrustUpdateTimeRollback);
            }
            (predecessor.revision, Some(predecessor.canonical_sha256()?))
        } else {
            if revocations.revision != 1 {
                return Err(PromotionReceiptError::RevocationRevisionGap {
                    expected: 1,
                    found: revocations.revision,
                });
            }
            (0, None)
        };
    let mut signed = SignedPromotionRevocationsV1 {
        schema_version: SIGNED_PROMOTION_REVOCATIONS_SCHEMA_VERSION,
        predecessor_trust_root_sha256: trust_root.canonical_sha256()?,
        predecessor_revocation_revision,
        predecessor_revocations_sha256,
        revocations,
        issued_at_unix_seconds,
        signatures: Vec::new(),
    };
    validate_signers(signers, "revocation snapshot signers")?;
    let statement = signed.canonical_statement()?;
    signed.signatures = sign_statement(statement, signers);
    signed.validate_structure()?;
    Ok(signed)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    content = "update",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum PromotionTrustUpdateV1 {
    TrustRootRotation(PromotionTrustRootRotationV1),
    Revocations(SignedPromotionRevocationsV1),
}

impl PromotionTrustUpdateV1 {
    fn history_chain_kind_code(&self) -> u8 {
        match self {
            Self::TrustRootRotation(_) => 1,
            Self::Revocations(_) => 2,
        }
    }

    fn canonical_statement_sha256(&self) -> Result<Sha256Digest, PromotionReceiptError> {
        let statement = match self {
            Self::TrustRootRotation(rotation) => rotation.canonical_statement()?,
            Self::Revocations(revocations) => revocations.canonical_statement()?,
        };
        Ok(Sha256Digest::for_bytes(&statement))
    }
}

/// Complete ordered trust history from the fixed genesis root. It is replayed
/// for every anchor operation; no bare higher root or revocation snapshot can
/// mint an anchored capability.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionTrustHistoryV1 {
    pub schema_version: u32,
    pub genesis: PromotionTrustRootV1,
    pub updates: Vec<PromotionTrustUpdateV1>,
}

impl PromotionTrustHistoryV1 {
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self, PromotionReceiptError> {
        validate_byte_bound(
            "promotion trust history JSON",
            bytes.len(),
            MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES,
        )?;
        serde_json::from_slice(bytes)
            .map_err(|error| PromotionReceiptError::Json(error.to_string()))
    }

    pub fn validate_structure(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion trust history",
            self.schema_version,
            PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
        )?;
        self.genesis.validate()?;
        if self.genesis.revision != 1 {
            return Err(PromotionReceiptError::InvalidField {
                field: "genesis trust-root revision",
                detail: "genesis revision must be exactly one",
            });
        }
        if self.updates.is_empty() {
            return Err(PromotionReceiptError::InvalidField {
                field: "promotion trust history",
                detail: "revision-one revocation state is required",
            });
        }
        validate_collection_bound(
            "promotion trust history",
            self.updates.len(),
            MAX_PROMOTION_TRUST_HISTORY_UPDATES,
        )?;
        let rotations = self
            .updates
            .iter()
            .filter(|update| matches!(update, PromotionTrustUpdateV1::TrustRootRotation(_)))
            .count();
        validate_collection_bound(
            "promotion trust-root rotations",
            rotations,
            MAX_PROMOTION_TRUST_ROOT_ROTATIONS,
        )?;
        for update in &self.updates {
            match update {
                PromotionTrustUpdateV1::TrustRootRotation(rotation) => {
                    rotation.validate_structure()?;
                }
                PromotionTrustUpdateV1::Revocations(revocations) => {
                    revocations.validate_structure()?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PromotionHeadCheckpointDocumentV1 {
    schema_version: u32,
    trust_root_id: String,
    genesis_trust_root_sha256: Sha256Digest,
    terminal_history_chain_sha256: Sha256Digest,
    terminal_trust_root_revision: u64,
    terminal_trust_root_sha256: Sha256Digest,
    terminal_revocation_revision: u64,
    terminal_revocations_sha256: Sha256Digest,
}

#[cfg(test)]
impl PromotionHeadCheckpointDocumentV1 {
    fn validate(&self) -> Result<(), PromotionReceiptError> {
        validate_version(
            "promotion head checkpoint",
            self.schema_version,
            PROMOTION_HEAD_CHECKPOINT_SCHEMA_VERSION,
        )?;
        validate_identifier("checkpoint trust root id", &self.trust_root_id)?;
        validate_sha256(
            "checkpoint genesis trust-root SHA-256",
            &self.genesis_trust_root_sha256,
        )?;
        validate_sha256(
            "checkpoint terminal history-chain SHA-256",
            &self.terminal_history_chain_sha256,
        )?;
        validate_durable_integer(
            "checkpoint terminal trust-root revision",
            self.terminal_trust_root_revision,
        )?;
        validate_sha256(
            "checkpoint terminal trust-root SHA-256",
            &self.terminal_trust_root_sha256,
        )?;
        validate_durable_integer(
            "checkpoint terminal revocation revision",
            self.terminal_revocation_revision,
        )?;
        validate_sha256(
            "checkpoint terminal revocations SHA-256",
            &self.terminal_revocations_sha256,
        )?;
        if self.terminal_trust_root_revision == 0 || self.terminal_revocation_revision == 0 {
            return Err(PromotionReceiptError::InvalidField {
                field: "checkpoint terminal revisions",
                detail: "terminal revisions must be nonzero",
            });
        }
        Ok(())
    }

    fn canonical_sha256(&self) -> Result<Sha256Digest, PromotionReceiptError> {
        self.validate()?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PROMOTION_HEAD_CHECKPOINT_V1_DIGEST_DOMAIN);
        bytes.extend_from_slice(CANONICAL_HEAD_CHECKPOINT_MAGIC);
        put_u32(&mut bytes, 1, self.schema_version);
        put_string(&mut bytes, 2, &self.trust_root_id);
        put_string(&mut bytes, 3, self.genesis_trust_root_sha256.as_str());
        put_string(&mut bytes, 4, self.terminal_history_chain_sha256.as_str());
        put_u64(&mut bytes, 5, self.terminal_trust_root_revision);
        put_string(&mut bytes, 6, self.terminal_trust_root_sha256.as_str());
        put_u64(&mut bytes, 7, self.terminal_revocation_revision);
        put_string(&mut bytes, 8, self.terminal_revocations_sha256.as_str());
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

/// Opaque independently pinned checkpoint capability.
///
/// Production minting is deliberately disabled in this crate until an exact
/// compiled pin or authenticated anti-rollback pin integration is added. This
/// type has no public fields, `Clone`, Serde implementation, or public safe
/// constructor. In particular, hashing caller-provided checkpoint bytes cannot
/// mint this capability because that would be self-attestation rather than an
/// independent pin.
#[derive(Debug)]
pub struct PinnedPromotionHeadCheckpointV1 {
    source_json_sha256: Sha256Digest,
    checkpoint_sha256: Sha256Digest,
    trust_root_id: String,
    genesis_trust_root_sha256: Sha256Digest,
    terminal_history_chain_sha256: Sha256Digest,
    terminal_trust_root_revision: u64,
    terminal_trust_root_sha256: Sha256Digest,
    terminal_revocation_revision: u64,
    terminal_revocations_sha256: Sha256Digest,
}

impl PinnedPromotionHeadCheckpointV1 {
    pub fn source_json_sha256(&self) -> &Sha256Digest {
        &self.source_json_sha256
    }

    pub fn checkpoint_sha256(&self) -> &Sha256Digest {
        &self.checkpoint_sha256
    }

    pub fn trust_root_id(&self) -> &str {
        &self.trust_root_id
    }

    pub fn genesis_trust_root_sha256(&self) -> &Sha256Digest {
        &self.genesis_trust_root_sha256
    }

    pub fn terminal_history_chain_sha256(&self) -> &Sha256Digest {
        &self.terminal_history_chain_sha256
    }

    pub fn terminal_trust_root_revision(&self) -> u64 {
        self.terminal_trust_root_revision
    }

    pub fn terminal_trust_root_sha256(&self) -> &Sha256Digest {
        &self.terminal_trust_root_sha256
    }

    pub fn terminal_revocation_revision(&self) -> u64 {
        self.terminal_revocation_revision
    }

    pub fn terminal_revocations_sha256(&self) -> &Sha256Digest {
        &self.terminal_revocations_sha256
    }
}

#[cfg(test)]
fn load_test_pinned_promotion_head_checkpoint_json_v1(
    independently_expected_json_sha256: &Sha256Digest,
    checkpoint_json: &[u8],
) -> Result<PinnedPromotionHeadCheckpointV1, PromotionReceiptError> {
    validate_sha256(
        "independently pinned checkpoint JSON SHA-256",
        independently_expected_json_sha256,
    )?;
    validate_byte_bound(
        "promotion head checkpoint JSON",
        checkpoint_json.len(),
        MAX_PROMOTION_HEAD_CHECKPOINT_JSON_BYTES,
    )?;
    let source_json_sha256 = Sha256Digest::for_bytes(checkpoint_json);
    if &source_json_sha256 != independently_expected_json_sha256 {
        return Err(PromotionReceiptError::CheckpointJsonDigestMismatch);
    }
    let document: PromotionHeadCheckpointDocumentV1 = serde_json::from_slice(checkpoint_json)
        .map_err(|error| PromotionReceiptError::Json(error.to_string()))?;
    document.validate()?;
    let checkpoint_sha256 = document.canonical_sha256()?;
    Ok(PinnedPromotionHeadCheckpointV1 {
        source_json_sha256,
        checkpoint_sha256,
        trust_root_id: document.trust_root_id,
        genesis_trust_root_sha256: document.genesis_trust_root_sha256,
        terminal_history_chain_sha256: document.terminal_history_chain_sha256,
        terminal_trust_root_revision: document.terminal_trust_root_revision,
        terminal_trust_root_sha256: document.terminal_trust_root_sha256,
        terminal_revocation_revision: document.terminal_revocation_revision,
        terminal_revocations_sha256: document.terminal_revocations_sha256,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromotionReplayStoreError {
    NonceReplay,
    ReceiptReplay,
    TrustRootRevisionRollback { highest_seen: u64, found: u64 },
    RevocationRevisionRollback { highest_seen: u64, found: u64 },
    TrustRootRevisionConflict { revision: u64 },
    RevocationRevisionConflict { revision: u64 },
    RevocationTombstoneRemoved(&'static str),
    TrustedConfigurationNotInitialized,
    TrustedConfigurationNotCurrent,
    GenesisAnchorConflict,
    TrustRootHistoryMismatch,
    ClockRollback { highest_seen: u64, found: u64 },
    Storage(String),
}

pub type PromotionReplayFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), PromotionReplayStoreError>> + Send + 'a>>;

/// One verified state in a complete genesis-anchored history. Fields are
/// private so external code cannot invent ancestry accepted by a durable store.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionTrustStateHistoryEntry {
    trust_root_revision: u64,
    trust_root_sha256: Sha256Digest,
    revocation_revision: u64,
    revocations_sha256: Sha256Digest,
    history_chain_sha256: Sha256Digest,
}

impl PromotionTrustStateHistoryEntry {
    pub fn trust_root_revision(&self) -> u64 {
        self.trust_root_revision
    }

    pub fn trust_root_sha256(&self) -> &Sha256Digest {
        &self.trust_root_sha256
    }

    pub fn revocation_revision(&self) -> u64 {
        self.revocation_revision
    }

    pub fn revocations_sha256(&self) -> &Sha256Digest {
        &self.revocations_sha256
    }

    pub fn history_chain_sha256(&self) -> &Sha256Digest {
        &self.history_chain_sha256
    }
}

/// Unforgeable request for the independent trusted-configuration/time ratchet.
/// It is minted only after replaying the complete signed history from the
/// independently pinned genesis through the checkpoint's exact terminal head.
#[derive(Clone, Copy, Debug)]
pub struct PromotionTrustedConfigRatchet<'a> {
    checkpoint: &'a PinnedPromotionHeadCheckpointV1,
    history: &'a [PromotionTrustStateHistoryEntry],
    trust_root: &'a PromotionTrustRootV1,
    trust_root_sha256: &'a Sha256Digest,
    revocations: &'a PromotionRevocationsV1,
    revocations_sha256: &'a Sha256Digest,
    history_chain_sha256: &'a Sha256Digest,
    observed_at_unix_seconds: u64,
}

impl<'a> PromotionTrustedConfigRatchet<'a> {
    pub fn checkpoint(self) -> &'a PinnedPromotionHeadCheckpointV1 {
        self.checkpoint
    }

    pub fn genesis_trust_root_sha256(self) -> &'a Sha256Digest {
        self.checkpoint.genesis_trust_root_sha256()
    }

    pub fn history(self) -> &'a [PromotionTrustStateHistoryEntry] {
        self.history
    }

    pub fn trust_root(self) -> &'a PromotionTrustRootV1 {
        self.trust_root
    }

    pub fn trust_root_sha256(self) -> &'a Sha256Digest {
        self.trust_root_sha256
    }

    pub fn revocations(self) -> &'a PromotionRevocationsV1 {
        self.revocations
    }

    pub fn revocations_sha256(self) -> &'a Sha256Digest {
        self.revocations_sha256
    }

    pub fn history_chain_sha256(self) -> &'a Sha256Digest {
        self.history_chain_sha256
    }

    pub fn observed_at_unix_seconds(self) -> u64 {
        self.observed_at_unix_seconds
    }
}

/// Trusted configuration capability returned only after the independent
/// durable ratchet succeeds. It carries evidence-validation trust, never
/// operator acceptance or promotion authority.
#[derive(Debug)]
struct AnchoredPromotionTrust {
    checkpoint_sha256: Sha256Digest,
    trust_root: PromotionTrustRootV1,
    trust_root_sha256: Sha256Digest,
    revocations: PromotionRevocationsV1,
    revocations_sha256: Sha256Digest,
    history_chain_sha256: Sha256Digest,
    observed_at_unix_seconds: u64,
}

#[cfg(test)]
impl AnchoredPromotionTrust {
    fn trust_root(&self) -> &PromotionTrustRootV1 {
        &self.trust_root
    }

    fn revocations(&self) -> &PromotionRevocationsV1 {
        &self.revocations
    }
}

/// Unforgeable replay-consumption request minted only after structure,
/// binding, time, anchored trust, revocation, and receipt-signature checks.
#[derive(Clone, Copy, Debug)]
pub struct PromotionReplayConsumption<'a> {
    trust_root_id: &'a str,
    checkpoint_sha256: &'a Sha256Digest,
    trust_root_revision: u64,
    trust_root_sha256: &'a Sha256Digest,
    revocation_revision: u64,
    revocations_sha256: &'a Sha256Digest,
    history_chain_sha256: &'a Sha256Digest,
    observed_at_unix_seconds: u64,
    nonce: &'a str,
    receipt_sha256: &'a Sha256Digest,
    expires_at_unix_seconds: u64,
}

impl<'a> PromotionReplayConsumption<'a> {
    pub fn trust_root_id(self) -> &'a str {
        self.trust_root_id
    }

    pub fn checkpoint_sha256(self) -> &'a Sha256Digest {
        self.checkpoint_sha256
    }

    pub fn trust_root_revision(self) -> u64 {
        self.trust_root_revision
    }

    pub fn trust_root_sha256(self) -> &'a Sha256Digest {
        self.trust_root_sha256
    }

    pub fn revocation_revision(self) -> u64 {
        self.revocation_revision
    }

    pub fn revocations_sha256(self) -> &'a Sha256Digest {
        self.revocations_sha256
    }

    pub fn history_chain_sha256(self) -> &'a Sha256Digest {
        self.history_chain_sha256
    }

    pub fn observed_at_unix_seconds(self) -> u64 {
        self.observed_at_unix_seconds
    }

    pub fn nonce(self) -> &'a str {
        self.nonce
    }

    pub fn receipt_sha256(self) -> &'a Sha256Digest {
        self.receipt_sha256
    }

    pub fn expires_at_unix_seconds(self) -> u64 {
        self.expires_at_unix_seconds
    }
}

/// Durable implementations must make each operation atomic and leave all
/// values unchanged on error. Fresh initialization atomically binds the exact
/// independent checkpoint, terminal cumulative history chain, terminal root
/// and revocation digests, and trusted-clock watermark. Later ratchets may only
/// extend that exact chain.
///
/// `ratchet_trusted_config` is deliberately separate from packet verification,
/// so trusted history and time advance even when no packet exists or a packet
/// later fails. `check_and_consume` must never advance trusted configuration and
/// must require the exact ratcheted checkpoint, chain, state, and time to remain
/// current. A deployment recovering from an erroneous large forward clock jump
/// must use an authenticated store-specific procedure; receipt fields are never
/// a recovery clock source.
pub trait PromotionReceiptReplayStore: Send + Sync {
    fn ratchet_trusted_config<'a>(
        &'a self,
        ratchet: PromotionTrustedConfigRatchet<'a>,
    ) -> PromotionReplayFuture<'a>;

    fn check_and_consume<'a>(
        &'a self,
        consumption: PromotionReplayConsumption<'a>,
    ) -> PromotionReplayFuture<'a>;
}

/// Test and single-process development implementation only. It is volatile,
/// cannot provide restart durability, and must never back promotion decisions.
#[cfg(test)]
#[derive(Default)]
struct InMemoryPromotionReceiptReplayStore {
    state: Mutex<InMemoryReplayState>,
}

#[cfg(test)]
#[derive(Default)]
struct InMemoryReplayState {
    trusted: BTreeMap<String, TrustedConfigurationWatermarks>,
    nonces: BTreeMap<(String, String), (Sha256Digest, u64)>,
    receipts: BTreeSet<(String, Sha256Digest)>,
}

#[cfg(test)]
struct TrustedConfigurationWatermarks {
    checkpoint_sha256: Sha256Digest,
    genesis_trust_root_sha256: Sha256Digest,
    trust_root_revision: u64,
    trust_root_sha256: Sha256Digest,
    revocation_revision: u64,
    revocations_sha256: Sha256Digest,
    history_chain_sha256: Sha256Digest,
    revoked_key_ids: Vec<String>,
    revoked_receipt_sha256: Vec<Sha256Digest>,
    revoked_nonces: Vec<String>,
    max_observed_time: u64,
}

#[cfg(test)]
impl PromotionReceiptReplayStore for InMemoryPromotionReceiptReplayStore {
    fn ratchet_trusted_config<'a>(
        &'a self,
        ratchet: PromotionTrustedConfigRatchet<'a>,
    ) -> PromotionReplayFuture<'a> {
        Box::pin(async move {
            let mut state = self.state.lock().map_err(|_| {
                PromotionReplayStoreError::Storage("replay mutex is poisoned".into())
            })?;
            let root_id = ratchet.trust_root.trust_root_id.clone();
            if let Some(current) = state.trusted.get(&root_id) {
                validate_time_ratchet(current.max_observed_time, ratchet.observed_at_unix_seconds)?;
                if &current.genesis_trust_root_sha256
                    != ratchet.checkpoint.genesis_trust_root_sha256()
                {
                    return Err(PromotionReplayStoreError::GenesisAnchorConflict);
                }
                validate_revision_ratchet(
                    current.trust_root_revision,
                    &current.trust_root_sha256,
                    ratchet.trust_root.revision,
                    ratchet.trust_root_sha256,
                    true,
                )?;
                validate_revision_ratchet(
                    current.revocation_revision,
                    &current.revocations_sha256,
                    ratchet.revocations.revision,
                    ratchet.revocations_sha256,
                    false,
                )?;
                let extends_current = ratchet.history.iter().any(|entry| {
                    entry.trust_root_revision == current.trust_root_revision
                        && entry.trust_root_sha256 == current.trust_root_sha256
                        && entry.revocation_revision == current.revocation_revision
                        && entry.revocations_sha256 == current.revocations_sha256
                        && entry.history_chain_sha256 == current.history_chain_sha256
                });
                if !extends_current {
                    return Err(PromotionReplayStoreError::TrustRootHistoryMismatch);
                }
                ensure_string_superset_store(
                    &current.revoked_key_ids,
                    &ratchet.revocations.revoked_key_ids,
                    "key-id",
                )?;
                ensure_digest_superset_store(
                    &current.revoked_receipt_sha256,
                    &ratchet.revocations.revoked_receipt_sha256,
                    "receipt",
                )?;
                ensure_string_superset_store(
                    &current.revoked_nonces,
                    &ratchet.revocations.revoked_nonces,
                    "nonce",
                )?;
            } else if ratchet.history.is_empty() {
                return Err(PromotionReplayStoreError::TrustRootHistoryMismatch);
            }
            state.trusted.insert(
                root_id,
                TrustedConfigurationWatermarks {
                    checkpoint_sha256: ratchet.checkpoint.checkpoint_sha256().clone(),
                    genesis_trust_root_sha256: ratchet
                        .checkpoint
                        .genesis_trust_root_sha256()
                        .clone(),
                    trust_root_revision: ratchet.trust_root.revision,
                    trust_root_sha256: ratchet.trust_root_sha256.clone(),
                    revocation_revision: ratchet.revocations.revision,
                    revocations_sha256: ratchet.revocations_sha256.clone(),
                    history_chain_sha256: ratchet.history_chain_sha256.clone(),
                    revoked_key_ids: ratchet.revocations.revoked_key_ids.clone(),
                    revoked_receipt_sha256: ratchet.revocations.revoked_receipt_sha256.clone(),
                    revoked_nonces: ratchet.revocations.revoked_nonces.clone(),
                    max_observed_time: ratchet.observed_at_unix_seconds,
                },
            );
            Ok(())
        })
    }

    fn check_and_consume<'a>(
        &'a self,
        consumption: PromotionReplayConsumption<'a>,
    ) -> PromotionReplayFuture<'a> {
        Box::pin(async move {
            let mut state = self.state.lock().map_err(|_| {
                PromotionReplayStoreError::Storage("replay mutex is poisoned".into())
            })?;
            let current = state
                .trusted
                .get(consumption.trust_root_id)
                .ok_or(PromotionReplayStoreError::TrustedConfigurationNotInitialized)?;
            validate_time_ratchet(
                current.max_observed_time,
                consumption.observed_at_unix_seconds,
            )?;
            if current.max_observed_time != consumption.observed_at_unix_seconds
                || &current.checkpoint_sha256 != consumption.checkpoint_sha256
                || current.trust_root_revision != consumption.trust_root_revision
                || &current.trust_root_sha256 != consumption.trust_root_sha256
                || current.revocation_revision != consumption.revocation_revision
                || &current.revocations_sha256 != consumption.revocations_sha256
                || &current.history_chain_sha256 != consumption.history_chain_sha256
            {
                return Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent);
            }
            let root = consumption.trust_root_id.to_string();
            if state
                .receipts
                .contains(&(root.clone(), consumption.receipt_sha256.clone()))
            {
                return Err(PromotionReplayStoreError::ReceiptReplay);
            }
            let nonce_key = (root.clone(), consumption.nonce.to_string());
            if state.nonces.contains_key(&nonce_key) {
                return Err(PromotionReplayStoreError::NonceReplay);
            }
            state.nonces.insert(
                nonce_key,
                (
                    consumption.receipt_sha256.clone(),
                    consumption.expires_at_unix_seconds,
                ),
            );
            state
                .receipts
                .insert((root, consumption.receipt_sha256.clone()));
            Ok(())
        })
    }
}

struct VerifiedPromotionTrustHistory {
    genesis_trust_root_sha256: Sha256Digest,
    trust_root: PromotionTrustRootV1,
    trust_root_sha256: Sha256Digest,
    revocations: PromotionRevocationsV1,
    revocations_sha256: Sha256Digest,
    history_chain_sha256: Sha256Digest,
    states: Vec<PromotionTrustStateHistoryEntry>,
}

async fn anchor_and_ratchet_promotion_trust_at(
    checkpoint: &PinnedPromotionHeadCheckpointV1,
    history: &PromotionTrustHistoryV1,
    observed_at_unix_seconds: u64,
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<AnchoredPromotionTrust, PromotionReceiptError> {
    validate_durable_integer(
        "trusted-config observation Unix seconds",
        observed_at_unix_seconds,
    )?;
    let verified = verify_promotion_trust_history(
        checkpoint.genesis_trust_root_sha256(),
        history,
        observed_at_unix_seconds,
    )?;
    validate_verified_history_checkpoint(checkpoint, &verified)?;
    replay_store
        .ratchet_trusted_config(PromotionTrustedConfigRatchet {
            checkpoint,
            history: &verified.states,
            trust_root: &verified.trust_root,
            trust_root_sha256: &verified.trust_root_sha256,
            revocations: &verified.revocations,
            revocations_sha256: &verified.revocations_sha256,
            history_chain_sha256: &verified.history_chain_sha256,
            observed_at_unix_seconds,
        })
        .await
        .map_err(map_replay_store_error)?;
    Ok(AnchoredPromotionTrust {
        checkpoint_sha256: checkpoint.checkpoint_sha256().clone(),
        trust_root: verified.trust_root,
        trust_root_sha256: verified.trust_root_sha256,
        revocations: verified.revocations,
        revocations_sha256: verified.revocations_sha256,
        history_chain_sha256: verified.history_chain_sha256,
        observed_at_unix_seconds,
    })
}

fn validate_verified_history_checkpoint(
    checkpoint: &PinnedPromotionHeadCheckpointV1,
    verified: &VerifiedPromotionTrustHistory,
) -> Result<(), PromotionReceiptError> {
    if checkpoint.trust_root_id != verified.trust_root.trust_root_id
        || checkpoint.genesis_trust_root_sha256 != verified.genesis_trust_root_sha256
    {
        return Err(PromotionReceiptError::CheckpointTrustRootMismatch);
    }
    if checkpoint.terminal_history_chain_sha256 != verified.history_chain_sha256
        || checkpoint.terminal_trust_root_revision != verified.trust_root.revision
        || checkpoint.terminal_trust_root_sha256 != verified.trust_root_sha256
        || checkpoint.terminal_revocation_revision != verified.revocations.revision
        || checkpoint.terminal_revocations_sha256 != verified.revocations_sha256
    {
        return Err(PromotionReceiptError::CheckpointHeadMismatch);
    }
    Ok(())
}

fn verify_promotion_trust_history(
    fixed_genesis_trust_root_sha256: &Sha256Digest,
    history: &PromotionTrustHistoryV1,
    observed_at_unix_seconds: u64,
) -> Result<VerifiedPromotionTrustHistory, PromotionReceiptError> {
    history.validate_structure()?;
    let genesis_sha256 = history.genesis.canonical_sha256()?;
    if &genesis_sha256 != fixed_genesis_trust_root_sha256 {
        return Err(PromotionReceiptError::GenesisTrustRootMismatch);
    }
    if history.genesis.effective_at_unix_seconds > observed_at_unix_seconds {
        return Err(PromotionReceiptError::GenesisTrustRootInFuture);
    }

    let mut trust_root = history.genesis.clone();
    let mut trust_root_sha256 = genesis_sha256.clone();
    let mut history_chain_sha256 = genesis_history_chain_sha256(&trust_root_sha256);
    let mut revocations: Option<(PromotionRevocationsV1, Sha256Digest)> = None;
    let mut states = Vec::with_capacity(history.updates.len());
    let mut last_update_time = history.genesis.effective_at_unix_seconds;
    let mut known_key_identities = BTreeMap::<String, String>::new();
    let mut known_public_key_ids = BTreeMap::<String, String>::new();
    let mut retired_key_ids = BTreeSet::<String>::new();
    let mut historically_revoked_key_ids = BTreeSet::<String>::new();
    let mut historically_revoked_public_keys = BTreeSet::<String>::new();
    for key in &trust_root.keys {
        known_key_identities.insert(key.key_id.clone(), key.public_key_hex.clone());
        known_public_key_ids.insert(key.public_key_hex.clone(), key.key_id.clone());
    }

    for (index, update) in history.updates.iter().enumerate() {
        let issued_at = match update {
            PromotionTrustUpdateV1::TrustRootRotation(rotation) => rotation.issued_at_unix_seconds,
            PromotionTrustUpdateV1::Revocations(update) => update.issued_at_unix_seconds,
        };
        if issued_at < last_update_time {
            return Err(PromotionReceiptError::TrustUpdateTimeRollback);
        }
        if issued_at > observed_at_unix_seconds {
            return Err(match update {
                PromotionTrustUpdateV1::TrustRootRotation(_) => {
                    PromotionReceiptError::TrustRootRotationInFuture
                }
                PromotionTrustUpdateV1::Revocations(_) => {
                    PromotionReceiptError::RevocationSnapshotInFuture
                }
            });
        }
        last_update_time = issued_at;

        match update {
            PromotionTrustUpdateV1::Revocations(signed) => {
                if signed.predecessor_trust_root_sha256 != trust_root_sha256
                    || signed.revocations.trust_root_id != trust_root.trust_root_id
                {
                    return Err(PromotionReceiptError::RevocationSnapshotTrustRootMismatch);
                }
                match &revocations {
                    None => {
                        if signed.predecessor_revocation_revision != 0
                            || signed.predecessor_revocations_sha256.is_some()
                            || signed.revocations.revision != 1
                        {
                            return Err(PromotionReceiptError::RevocationRevisionGap {
                                expected: 1,
                                found: signed.revocations.revision,
                            });
                        }
                    }
                    Some((predecessor, predecessor_sha256)) => {
                        if signed.predecessor_revocation_revision != predecessor.revision
                            || signed.predecessor_revocations_sha256.as_ref()
                                != Some(predecessor_sha256)
                        {
                            return Err(PromotionReceiptError::RevocationRevisionConflict {
                                revision: signed.predecessor_revocation_revision,
                            });
                        }
                        let expected = predecessor.revision.checked_add(1).ok_or(
                            PromotionReceiptError::InvalidField {
                                field: "revocation revision",
                                detail: "revision overflow",
                            },
                        )?;
                        if signed.revocations.revision != expected {
                            return Err(PromotionReceiptError::RevocationRevisionGap {
                                expected,
                                found: signed.revocations.revision,
                            });
                        }
                        ensure_revocation_superset(predecessor, &signed.revocations)?;
                    }
                }
                verify_configuration_signatures(
                    signed.canonical_statement()?,
                    &signed.signatures,
                    &trust_root,
                    issued_at,
                    &historically_revoked_key_ids,
                )?;
                let revocations_sha256 = signed.revocations.canonical_sha256()?;
                revocations = Some((signed.revocations.clone(), revocations_sha256));
                for key_id in &signed.revocations.revoked_key_ids {
                    historically_revoked_key_ids.insert(key_id.clone());
                    if let Some(public_key) = known_key_identities.get(key_id) {
                        historically_revoked_public_keys.insert(public_key.clone());
                    }
                }
            }
            PromotionTrustUpdateV1::TrustRootRotation(rotation) => {
                let (current_revocations, current_revocations_sha256) = revocations
                    .as_ref()
                    .ok_or(PromotionReceiptError::InvalidField {
                        field: "promotion trust history",
                        detail: "revocation revision one must precede root rotation",
                    })?;
                if rotation.predecessor_trust_root_sha256 != trust_root_sha256
                    || rotation.predecessor_revocation_revision != current_revocations.revision
                    || &rotation.predecessor_revocations_sha256 != current_revocations_sha256
                {
                    return Err(PromotionReceiptError::TrustRootRotationPredecessorMismatch);
                }
                validate_root_successor(&trust_root, &rotation.successor)?;
                verify_configuration_signatures(
                    rotation.canonical_statement()?,
                    &rotation.signatures,
                    &trust_root,
                    issued_at,
                    &historically_revoked_key_ids,
                )?;
                validate_root_key_history(
                    &trust_root,
                    &rotation.successor,
                    &mut known_key_identities,
                    &mut known_public_key_ids,
                    &mut retired_key_ids,
                    &historically_revoked_key_ids,
                    &historically_revoked_public_keys,
                )?;
                trust_root = rotation.successor.clone();
                trust_root_sha256 = trust_root.canonical_sha256()?;
            }
        }
        history_chain_sha256 =
            extend_history_chain_sha256(&history_chain_sha256, (index + 1) as u64, update)?;
        if let Some((current_revocations, current_revocations_sha256)) = &revocations {
            states.push(PromotionTrustStateHistoryEntry {
                trust_root_revision: trust_root.revision,
                trust_root_sha256: trust_root_sha256.clone(),
                revocation_revision: current_revocations.revision,
                revocations_sha256: current_revocations_sha256.clone(),
                history_chain_sha256: history_chain_sha256.clone(),
            });
        }
    }
    let (revocations, revocations_sha256) =
        revocations.ok_or(PromotionReceiptError::InvalidField {
            field: "promotion trust history",
            detail: "revision-one revocation state is required",
        })?;
    Ok(VerifiedPromotionTrustHistory {
        genesis_trust_root_sha256: genesis_sha256,
        trust_root,
        trust_root_sha256,
        revocations,
        revocations_sha256,
        history_chain_sha256,
        states,
    })
}

fn verify_configuration_signatures(
    statement: Vec<u8>,
    signatures: &[PromotionSignatureV1],
    trust_root: &PromotionTrustRootV1,
    issued_at_unix_seconds: u64,
    historically_revoked_key_ids: &BTreeSet<String>,
) -> Result<(), PromotionReceiptError> {
    let mut verified = 0u16;
    for signature in signatures {
        let key = trust_root
            .keys
            .binary_search_by(|key| key.key_id.as_str().cmp(&signature.key_id))
            .ok()
            .map(|index| &trust_root.keys[index])
            .ok_or_else(|| PromotionReceiptError::UnknownSigner(signature.key_id.clone()))?;
        if historically_revoked_key_ids.contains(&signature.key_id)
            || key.revoked_at(issued_at_unix_seconds)
        {
            return Err(PromotionReceiptError::RevokedConfigurationSigner(
                signature.key_id.clone(),
            ));
        }
        if !key.active_for_configuration_signature(issued_at_unix_seconds) {
            return Err(PromotionReceiptError::InactiveSigner(
                signature.key_id.clone(),
            ));
        }
        verify_signature(
            &key_specific_signing_message(
                statement.clone(),
                &signature.key_id,
                signature.algorithm,
            ),
            signature,
            key,
        )?;
        verified = verified.saturating_add(1);
    }
    if verified < trust_root.signature_threshold {
        return Err(PromotionReceiptError::SignatureThresholdNotMet {
            required: trust_root.signature_threshold,
            verified,
        });
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_root_key_history(
    predecessor: &PromotionTrustRootV1,
    successor: &PromotionTrustRootV1,
    known_key_identities: &mut BTreeMap<String, String>,
    known_public_key_ids: &mut BTreeMap<String, String>,
    retired_key_ids: &mut BTreeSet<String>,
    historically_revoked_key_ids: &BTreeSet<String>,
    historically_revoked_public_keys: &BTreeSet<String>,
) -> Result<(), PromotionReceiptError> {
    for predecessor_key in &predecessor.keys {
        if successor
            .keys
            .binary_search_by(|key| key.key_id.cmp(&predecessor_key.key_id))
            .is_err()
        {
            retired_key_ids.insert(predecessor_key.key_id.clone());
        }
    }
    for successor_key in &successor.keys {
        let predecessor_key = predecessor
            .keys
            .binary_search_by(|key| key.key_id.cmp(&successor_key.key_id))
            .ok()
            .map(|index| &predecessor.keys[index]);
        if let Some(known_public_key) = known_key_identities.get(&successor_key.key_id)
            && known_public_key != &successor_key.public_key_hex
        {
            return Err(PromotionReceiptError::TrustRootKeyIdentityConflict(
                successor_key.key_id.clone(),
            ));
        }
        if let Some(known_key_id) = known_public_key_ids.get(&successor_key.public_key_hex)
            && known_key_id != &successor_key.key_id
        {
            return Err(PromotionReceiptError::TrustRootKeyIdentityConflict(
                successor_key.key_id.clone(),
            ));
        }
        if retired_key_ids.contains(&successor_key.key_id)
            || (predecessor_key.is_none()
                && (historically_revoked_key_ids.contains(&successor_key.key_id)
                    || historically_revoked_public_keys.contains(&successor_key.public_key_hex)))
        {
            return Err(PromotionReceiptError::TrustRootKeyReintroduced(
                successor_key.key_id.clone(),
            ));
        }
        if let Some(predecessor_key) = predecessor_key
            && let Some(predecessor_revoked_at) = predecessor_key.revoked_at_unix_seconds
        {
            let Some(successor_revoked_at) = successor_key.revoked_at_unix_seconds else {
                return Err(PromotionReceiptError::TrustRootKeyRevocationRemoved(
                    successor_key.key_id.clone(),
                ));
            };
            if successor_revoked_at > predecessor_revoked_at {
                return Err(PromotionReceiptError::TrustRootKeyRevocationRemoved(
                    successor_key.key_id.clone(),
                ));
            }
        }
        known_key_identities
            .entry(successor_key.key_id.clone())
            .or_insert_with(|| successor_key.public_key_hex.clone());
        known_public_key_ids
            .entry(successor_key.public_key_hex.clone())
            .or_insert_with(|| successor_key.key_id.clone());
    }
    Ok(())
}

fn verify_signature(
    message: &[u8],
    signature: &PromotionSignatureV1,
    key: &TrustedPromotionKeyV1,
) -> Result<(), PromotionReceiptError> {
    if signature.algorithm != key.algorithm {
        return Err(PromotionReceiptError::SignatureVerificationFailed(
            signature.key_id.clone(),
        ));
    }
    let public_key_bytes = decode_hex::<32>(&key.public_key_hex)
        .map_err(|_| PromotionReceiptError::MalformedPublicKey(signature.key_id.clone()))?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|_| PromotionReceiptError::MalformedPublicKey(signature.key_id.clone()))?;
    let signature_bytes = decode_hex::<64>(&signature.signature_hex)
        .map_err(|_| PromotionReceiptError::MalformedSignature(signature.key_id.clone()))?;
    verifying_key
        .verify_strict(message, &Signature::from_bytes(&signature_bytes))
        .map_err(|_| PromotionReceiptError::SignatureVerificationFailed(signature.key_id.clone()))
}

/// Verified evidence only. This type intentionally has no authority or acceptance field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPromotionEvidence {
    receipt_sha256: Sha256Digest,
    trust_root_id: String,
    trust_root_revision: u64,
    revocation_revision: u64,
    bindings: PromotionBindingsV1,
    issued_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    nonce: String,
}

impl VerifiedPromotionEvidence {
    pub fn receipt_sha256(&self) -> &Sha256Digest {
        &self.receipt_sha256
    }

    pub fn trust_root_id(&self) -> &str {
        &self.trust_root_id
    }

    pub fn trust_root_revision(&self) -> u64 {
        self.trust_root_revision
    }

    pub fn revocation_revision(&self) -> u64 {
        self.revocation_revision
    }

    pub fn bindings(&self) -> &PromotionBindingsV1 {
        &self.bindings
    }

    pub fn issued_at_unix_seconds(&self) -> u64 {
        self.issued_at_unix_seconds
    }

    pub fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }
}

fn verify_promotion_receipt_at(
    packet: &SignedPromotionReceiptV1,
    expected_bindings: &PromotionBindingsV1,
    anchored_trust: &AnchoredPromotionTrust,
) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
    // The independent trusted-config/time ratchet already succeeded before any
    // untrusted packet work begins.
    packet.validate_structure()?;
    expected_bindings.validate()?;
    let trust_root = &anchored_trust.trust_root;
    let revocations = &anchored_trust.revocations;
    let now_unix_seconds = anchored_trust.observed_at_unix_seconds;

    let receipt = &packet.receipt;
    if receipt.trust_root_id != trust_root.trust_root_id
        || receipt.trust_root_id != revocations.trust_root_id
    {
        return Err(PromotionReceiptError::TrustRootMismatch);
    }
    if trust_root.revision < receipt.minimum_trust_root_revision {
        return Err(PromotionReceiptError::TrustRootRevisionTooOld {
            minimum: receipt.minimum_trust_root_revision,
            found: trust_root.revision,
        });
    }
    if revocations.revision < receipt.minimum_revocation_revision {
        return Err(PromotionReceiptError::RevocationRevisionTooOld {
            minimum: receipt.minimum_revocation_revision,
            found: revocations.revision,
        });
    }
    if &receipt.bindings != expected_bindings {
        return Err(PromotionReceiptError::BindingMismatch);
    }
    if receipt.issued_at_unix_seconds < trust_root.effective_at_unix_seconds {
        return Err(PromotionReceiptError::ReceiptPredatesTrustRoot);
    }
    if receipt.issued_at_unix_seconds < revocations.effective_at_unix_seconds {
        return Err(PromotionReceiptError::ReceiptPredatesRevocations);
    }
    if now_unix_seconds < receipt.issued_at_unix_seconds {
        return Err(PromotionReceiptError::ReceiptNotYetValid);
    }
    if now_unix_seconds >= receipt.expires_at_unix_seconds {
        return Err(PromotionReceiptError::ReceiptExpired);
    }
    if receipt.expires_at_unix_seconds - receipt.issued_at_unix_seconds
        > trust_root.max_receipt_lifetime_seconds
    {
        return Err(PromotionReceiptError::ReceiptLifetimeTooLong);
    }

    let receipt_preimage = receipt.canonical_receipt_preimage_unchecked();
    let receipt_sha256 = Sha256Digest::for_bytes(&receipt_preimage);
    if revocations
        .revoked_receipt_sha256
        .binary_search(&receipt_sha256)
        .is_ok()
    {
        return Err(PromotionReceiptError::ReceiptRevoked);
    }
    if revocations
        .revoked_nonces
        .binary_search(&receipt.nonce)
        .is_ok()
    {
        return Err(PromotionReceiptError::NonceRevoked);
    }

    let mut verified = 0u16;
    for signature in &packet.signatures {
        let signing_message = receipt
            .canonical_signing_bytes_for_signer_unchecked(&signature.key_id, signature.algorithm);
        let key = trust_root
            .keys
            .binary_search_by(|key| key.key_id.as_str().cmp(&signature.key_id))
            .ok()
            .map(|index| &trust_root.keys[index])
            .ok_or_else(|| PromotionReceiptError::UnknownSigner(signature.key_id.clone()))?;
        if revocations
            .revoked_key_ids
            .binary_search(&signature.key_id)
            .is_ok()
            || key.revoked_at(now_unix_seconds)
        {
            return Err(PromotionReceiptError::RevokedSigner(
                signature.key_id.clone(),
            ));
        }
        if !key.active_at(receipt.issued_at_unix_seconds, now_unix_seconds) {
            return Err(PromotionReceiptError::InactiveSigner(
                signature.key_id.clone(),
            ));
        }
        verify_signature(&signing_message, signature, key)?;
        verified = verified.saturating_add(1);
    }
    if verified < trust_root.signature_threshold {
        return Err(PromotionReceiptError::SignatureThresholdNotMet {
            required: trust_root.signature_threshold,
            verified,
        });
    }

    Ok(VerifiedPromotionEvidence {
        receipt_sha256,
        trust_root_id: receipt.trust_root_id.clone(),
        trust_root_revision: trust_root.revision,
        revocation_revision: revocations.revision,
        bindings: receipt.bindings.clone(),
        issued_at_unix_seconds: receipt.issued_at_unix_seconds,
        expires_at_unix_seconds: receipt.expires_at_unix_seconds,
        nonce: receipt.nonce.clone(),
    })
}

async fn consume_verified_promotion_evidence(
    evidence: &VerifiedPromotionEvidence,
    anchored_trust: &AnchoredPromotionTrust,
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<(), PromotionReceiptError> {
    replay_store
        .check_and_consume(PromotionReplayConsumption {
            trust_root_id: &evidence.trust_root_id,
            checkpoint_sha256: &anchored_trust.checkpoint_sha256,
            trust_root_revision: anchored_trust.trust_root.revision,
            trust_root_sha256: &anchored_trust.trust_root_sha256,
            revocation_revision: anchored_trust.revocations.revision,
            revocations_sha256: &anchored_trust.revocations_sha256,
            history_chain_sha256: &anchored_trust.history_chain_sha256,
            observed_at_unix_seconds: anchored_trust.observed_at_unix_seconds,
            nonce: &evidence.nonce,
            receipt_sha256: &evidence.receipt_sha256,
            expires_at_unix_seconds: evidence.expires_at_unix_seconds,
        })
        .await
        .map_err(map_replay_store_error)
}

#[cfg(test)]
async fn verify_and_consume_promotion_receipt_at(
    packet: &SignedPromotionReceiptV1,
    expected_bindings: &PromotionBindingsV1,
    anchored_trust: &AnchoredPromotionTrust,
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
    let evidence = verify_promotion_receipt_at(packet, expected_bindings, anchored_trust)?;
    consume_verified_promotion_evidence(&evidence, anchored_trust, replay_store).await?;
    Ok(evidence)
}

fn trusted_system_time_unix_seconds() -> Result<u64, PromotionReceiptError> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| PromotionReceiptError::TrustedClockBeforeUnixEpoch)?
        .as_secs();
    validate_durable_integer("trusted system clock Unix seconds", seconds)?;
    Ok(seconds)
}

/// Advances independently pinned promotion trust and the durable trusted-clock
/// watermark without accepting a receipt packet. The history is parsed only
/// through the capped strict-JSON path and no reusable verification capability
/// is returned.
///
/// Safe downstream production use is disabled until a product integration
/// supplies an exact compiled checkpoint or an authenticated anti-rollback
/// checkpoint source. This crate intentionally exposes no checkpoint loader or
/// constructor and does not mint a dummy or caller-self-attested pin.
///
/// A large erroneous system-clock jump is intentionally fail-closed and may
/// require an authenticated deployment- and store-specific recovery procedure
/// governed by that store's explicit clock-jump policy. This schema does not
/// define a universal maximum forward step; packet fields are never a clock
/// source.
pub async fn ratchet_promotion_trust_json_v1(
    checkpoint: &PinnedPromotionHeadCheckpointV1,
    history_json: &[u8],
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<(), PromotionReceiptError> {
    let observed_at_unix_seconds = trusted_system_time_unix_seconds()?;
    let history = PromotionTrustHistoryV1::from_json_slice(history_json)?;
    anchor_and_ratchet_promotion_trust_at(
        checkpoint,
        &history,
        observed_at_unix_seconds,
        replay_store,
    )
    .await?;
    Ok(())
}

/// Production verification facade. It accepts only capped strict JSON, starts
/// from an independently pinned exact checkpoint, and never exposes a reusable
/// anchored capability or a typed verifier.
///
/// Safe downstream production use is disabled until a product integration
/// supplies an exact compiled checkpoint or an authenticated anti-rollback
/// checkpoint source. This crate intentionally exposes no checkpoint loader or
/// constructor.
///
/// System time is sampled before any packet parsing and sampled again after
/// cryptographic verification. The complete checkpoint-bound history and
/// durable time are ratcheted at both samples; the packet is rechecked against
/// the second sample immediately before an exact-state atomic consume. If the
/// packet crosses expiry or signer validity/revocation at the second sample,
/// the second time watermark has already been persisted before the error is
/// returned.
pub async fn verify_and_consume_promotion_receipt_json_v1(
    checkpoint: &PinnedPromotionHeadCheckpointV1,
    history_json: &[u8],
    packet_json: &[u8],
    expected_bindings: &PromotionBindingsV1,
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
    let first_observed_at = trusted_system_time_unix_seconds()?;
    let history = PromotionTrustHistoryV1::from_json_slice(history_json)?;
    let first_anchor = anchor_and_ratchet_promotion_trust_at(
        checkpoint,
        &history,
        first_observed_at,
        replay_store,
    )
    .await?;

    // Packet parsing deliberately follows the independent trust/time ratchet,
    // so missing, oversized, malformed, or invalid packets cannot suppress it.
    let packet = SignedPromotionReceiptV1::from_json_slice(packet_json)?;
    verify_promotion_receipt_at(&packet, expected_bindings, &first_anchor)?;

    let second_observed_at = trusted_system_time_unix_seconds()?;
    let second_anchor = anchor_and_ratchet_promotion_trust_at(
        checkpoint,
        &history,
        second_observed_at,
        replay_store,
    )
    .await?;
    let evidence = verify_promotion_receipt_at(&packet, expected_bindings, &second_anchor)?;
    consume_verified_promotion_evidence(&evidence, &second_anchor, replay_store).await?;
    Ok(evidence)
}

#[cfg(test)]
async fn verify_and_consume_promotion_receipt_json_at_times(
    checkpoint: &PinnedPromotionHeadCheckpointV1,
    history_json: &[u8],
    packet_json: &[u8],
    expected_bindings: &PromotionBindingsV1,
    first_observed_at: u64,
    second_observed_at: u64,
    replay_store: &dyn PromotionReceiptReplayStore,
) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
    validate_durable_integer("first test observation Unix seconds", first_observed_at)?;
    validate_durable_integer("second test observation Unix seconds", second_observed_at)?;
    let history = PromotionTrustHistoryV1::from_json_slice(history_json)?;
    let first_anchor = anchor_and_ratchet_promotion_trust_at(
        checkpoint,
        &history,
        first_observed_at,
        replay_store,
    )
    .await?;
    let packet = SignedPromotionReceiptV1::from_json_slice(packet_json)?;
    verify_promotion_receipt_at(&packet, expected_bindings, &first_anchor)?;
    let second_anchor = anchor_and_ratchet_promotion_trust_at(
        checkpoint,
        &history,
        second_observed_at,
        replay_store,
    )
    .await?;
    let evidence = verify_promotion_receipt_at(&packet, expected_bindings, &second_anchor)?;
    consume_verified_promotion_evidence(&evidence, &second_anchor, replay_store).await?;
    Ok(evidence)
}

pub fn ed25519_public_key_hex(signing_key: &SigningKey) -> String {
    encode_hex(&signing_key.verifying_key().to_bytes())
}

fn validate_signatures_structure(
    signatures: &[PromotionSignatureV1],
    collection: &'static str,
) -> Result<(), PromotionReceiptError> {
    if signatures.is_empty() {
        return Err(PromotionReceiptError::InvalidField {
            field: collection,
            detail: "at least one signature is required",
        });
    }
    validate_collection_bound(collection, signatures.len(), MAX_PROMOTION_SIGNATURES)?;
    validate_strict_order(
        signatures,
        |signature| signature.key_id.as_str(),
        collection,
    )?;
    for signature in signatures {
        validate_identifier("signer key id", &signature.key_id)?;
        validate_fixed_hex("Ed25519 signature", &signature.signature_hex, 128)?;
    }
    Ok(())
}

fn validate_signers(
    signers: &[PromotionSignerV1<'_>],
    collection: &'static str,
) -> Result<(), PromotionReceiptError> {
    if signers.is_empty() {
        return Err(PromotionReceiptError::InvalidField {
            field: collection,
            detail: "at least one signer is required",
        });
    }
    validate_collection_bound(collection, signers.len(), MAX_PROMOTION_SIGNATURES)?;
    validate_strict_order(signers, |signer| signer.key_id, collection)?;
    for signer in signers {
        validate_identifier("signer key id", signer.key_id)?;
    }
    Ok(())
}

fn sign_statement(
    statement: Vec<u8>,
    signers: &[PromotionSignerV1<'_>],
) -> Vec<PromotionSignatureV1> {
    signers
        .iter()
        .map(|signer| {
            let algorithm = PromotionSignatureAlgorithm::Ed25519;
            let message = key_specific_signing_message(statement.clone(), signer.key_id, algorithm);
            PromotionSignatureV1 {
                key_id: signer.key_id.to_string(),
                algorithm,
                signature_hex: encode_hex(&signer.signing_key.sign(&message).to_bytes()),
            }
        })
        .collect()
}

fn key_specific_signing_message(
    mut statement: Vec<u8>,
    key_id: &str,
    algorithm: PromotionSignatureAlgorithm,
) -> Vec<u8> {
    put_string(&mut statement, 250, key_id);
    put_u8(&mut statement, 251, algorithm.canonical_code());
    statement
}

fn genesis_history_chain_sha256(genesis_sha256: &Sha256Digest) -> Sha256Digest {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(PROMOTION_HISTORY_CHAIN_GENESIS_V1_DOMAIN);
    bytes.extend_from_slice(CANONICAL_HISTORY_CHAIN_GENESIS_MAGIC);
    put_string(&mut bytes, 1, genesis_sha256.as_str());
    Sha256Digest::for_bytes(&bytes)
}

fn extend_history_chain_sha256(
    predecessor_chain_sha256: &Sha256Digest,
    ordinal: u64,
    update: &PromotionTrustUpdateV1,
) -> Result<Sha256Digest, PromotionReceiptError> {
    let statement_sha256 = update.canonical_statement_sha256()?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(PROMOTION_HISTORY_CHAIN_UPDATE_V1_DOMAIN);
    bytes.extend_from_slice(CANONICAL_HISTORY_CHAIN_UPDATE_MAGIC);
    put_string(&mut bytes, 1, predecessor_chain_sha256.as_str());
    put_u64(&mut bytes, 2, ordinal);
    put_u8(&mut bytes, 3, update.history_chain_kind_code());
    put_string(&mut bytes, 4, statement_sha256.as_str());
    Ok(Sha256Digest::for_bytes(&bytes))
}

fn validate_root_successor(
    predecessor: &PromotionTrustRootV1,
    successor: &PromotionTrustRootV1,
) -> Result<(), PromotionReceiptError> {
    successor.validate()?;
    if predecessor.trust_root_id != successor.trust_root_id {
        return Err(PromotionReceiptError::TrustRootMismatch);
    }
    let expected =
        predecessor
            .revision
            .checked_add(1)
            .ok_or(PromotionReceiptError::InvalidField {
                field: "trust-root revision",
                detail: "revision overflow",
            })?;
    if successor.revision != expected {
        return Err(PromotionReceiptError::TrustRootRotationRevisionGap);
    }
    if successor.effective_at_unix_seconds < predecessor.effective_at_unix_seconds {
        return Err(PromotionReceiptError::TrustUpdateTimeRollback);
    }
    Ok(())
}

fn validate_root_threshold_reachable(
    trust_root: &PromotionTrustRootV1,
) -> Result<(), PromotionReceiptError> {
    let intervals = trust_root
        .keys
        .iter()
        .filter_map(|key| {
            let start = key
                .valid_from_unix_seconds
                .max(trust_root.effective_at_unix_seconds);
            let end = key
                .revoked_at_unix_seconds
                .unwrap_or(key.valid_until_unix_seconds)
                .min(key.valid_until_unix_seconds);
            (start < end).then_some((start, end))
        })
        .collect::<Vec<_>>();
    let required = usize::from(trust_root.signature_threshold);
    if intervals.iter().any(|(candidate, _)| {
        intervals
            .iter()
            .filter(|(start, end)| start <= candidate && candidate < end)
            .count()
            >= required
    }) {
        Ok(())
    } else {
        Err(PromotionReceiptError::UnreachableTrustRootThreshold)
    }
}

fn ensure_revocation_superset(
    predecessor: &PromotionRevocationsV1,
    successor: &PromotionRevocationsV1,
) -> Result<(), PromotionReceiptError> {
    if !is_sorted_superset(&predecessor.revoked_key_ids, &successor.revoked_key_ids) {
        return Err(PromotionReceiptError::RevocationTombstoneRemoved("key-id"));
    }
    if !is_sorted_superset(
        &predecessor.revoked_receipt_sha256,
        &successor.revoked_receipt_sha256,
    ) {
        return Err(PromotionReceiptError::RevocationTombstoneRemoved("receipt"));
    }
    if !is_sorted_superset(&predecessor.revoked_nonces, &successor.revoked_nonces) {
        return Err(PromotionReceiptError::RevocationTombstoneRemoved("nonce"));
    }
    Ok(())
}

fn is_sorted_superset<T: Ord>(predecessor: &[T], successor: &[T]) -> bool {
    predecessor
        .iter()
        .all(|value| successor.binary_search(value).is_ok())
}

#[cfg(test)]
fn validate_time_ratchet(highest_seen: u64, found: u64) -> Result<(), PromotionReplayStoreError> {
    if found < highest_seen {
        return Err(PromotionReplayStoreError::ClockRollback {
            highest_seen,
            found,
        });
    }
    Ok(())
}

#[cfg(test)]
fn validate_revision_ratchet(
    highest_revision: u64,
    highest_digest: &Sha256Digest,
    found_revision: u64,
    found_digest: &Sha256Digest,
    trust_root: bool,
) -> Result<(), PromotionReplayStoreError> {
    if found_revision < highest_revision {
        return if trust_root {
            Err(PromotionReplayStoreError::TrustRootRevisionRollback {
                highest_seen: highest_revision,
                found: found_revision,
            })
        } else {
            Err(PromotionReplayStoreError::RevocationRevisionRollback {
                highest_seen: highest_revision,
                found: found_revision,
            })
        };
    }
    if found_revision == highest_revision && found_digest != highest_digest {
        return if trust_root {
            Err(PromotionReplayStoreError::TrustRootRevisionConflict {
                revision: found_revision,
            })
        } else {
            Err(PromotionReplayStoreError::RevocationRevisionConflict {
                revision: found_revision,
            })
        };
    }
    Ok(())
}

#[cfg(test)]
fn ensure_string_superset_store(
    predecessor: &[String],
    successor: &[String],
    kind: &'static str,
) -> Result<(), PromotionReplayStoreError> {
    if is_sorted_superset(predecessor, successor) {
        Ok(())
    } else {
        Err(PromotionReplayStoreError::RevocationTombstoneRemoved(kind))
    }
}

#[cfg(test)]
fn ensure_digest_superset_store(
    predecessor: &[Sha256Digest],
    successor: &[Sha256Digest],
    kind: &'static str,
) -> Result<(), PromotionReplayStoreError> {
    if is_sorted_superset(predecessor, successor) {
        Ok(())
    } else {
        Err(PromotionReplayStoreError::RevocationTombstoneRemoved(kind))
    }
}

fn map_replay_store_error(error: PromotionReplayStoreError) -> PromotionReceiptError {
    match error {
        PromotionReplayStoreError::NonceReplay => PromotionReceiptError::NonceReplay,
        PromotionReplayStoreError::ReceiptReplay => PromotionReceiptError::ReceiptReplay,
        PromotionReplayStoreError::TrustRootRevisionRollback {
            highest_seen,
            found,
        } => PromotionReceiptError::TrustRootRevisionRollback {
            highest_seen,
            found,
        },
        PromotionReplayStoreError::RevocationRevisionRollback {
            highest_seen,
            found,
        } => PromotionReceiptError::RevocationRevisionRollback {
            highest_seen,
            found,
        },
        PromotionReplayStoreError::TrustRootRevisionConflict { revision } => {
            PromotionReceiptError::TrustRootRevisionConflict { revision }
        }
        PromotionReplayStoreError::RevocationRevisionConflict { revision } => {
            PromotionReceiptError::RevocationRevisionConflict { revision }
        }
        PromotionReplayStoreError::RevocationTombstoneRemoved(kind) => {
            PromotionReceiptError::RevocationTombstoneRemoved(kind)
        }
        PromotionReplayStoreError::TrustedConfigurationNotInitialized => {
            PromotionReceiptError::TrustedConfigurationNotInitialized
        }
        PromotionReplayStoreError::TrustedConfigurationNotCurrent => {
            PromotionReceiptError::TrustedConfigurationNotCurrent
        }
        PromotionReplayStoreError::GenesisAnchorConflict => {
            PromotionReceiptError::GenesisAnchorConflict
        }
        PromotionReplayStoreError::TrustRootHistoryMismatch => {
            PromotionReceiptError::TrustRootHistoryMismatch
        }
        PromotionReplayStoreError::ClockRollback {
            highest_seen,
            found,
        } => PromotionReceiptError::ClockRollback {
            highest_seen,
            found,
        },
        PromotionReplayStoreError::Storage(detail) => {
            PromotionReceiptError::ReplayStoreFailure(detail)
        }
    }
}

fn validate_version(
    component: &'static str,
    found: u32,
    expected: u32,
) -> Result<(), PromotionReceiptError> {
    if found != expected {
        return Err(PromotionReceiptError::UnsupportedSchemaVersion { component, found });
    }
    Ok(())
}

fn validate_git_object_id(value: &str) -> Result<(), PromotionReceiptError> {
    if !matches!(value.len(), 40 | 64) || !is_lower_hex(value) {
        return Err(PromotionReceiptError::InvalidField {
            field: "Git object id",
            detail: "expected 40 or 64 lowercase hexadecimal characters",
        });
    }
    Ok(())
}

fn validate_sha256(
    field: &'static str,
    digest: &Sha256Digest,
) -> Result<(), PromotionReceiptError> {
    if digest.as_str().len() != 64 || !is_lower_hex(digest.as_str()) {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "expected 64 lowercase hexadecimal characters",
        });
    }
    Ok(())
}

fn validate_identifier(field: &'static str, value: &str) -> Result<(), PromotionReceiptError> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
    {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "expected 1-128 portable ASCII identifier characters",
        });
    }
    Ok(())
}

fn validate_fixed_hex(
    field: &'static str,
    value: &str,
    expected_length: usize,
) -> Result<(), PromotionReceiptError> {
    if value.len() != expected_length || !is_lower_hex(value) {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "value is not canonical lowercase hexadecimal",
        });
    }
    Ok(())
}

fn validate_byte_bound(
    field: &'static str,
    length: usize,
    maximum: usize,
) -> Result<(), PromotionReceiptError> {
    if length > maximum {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "input exceeds the hard byte limit",
        });
    }
    Ok(())
}

fn validate_durable_integer(field: &'static str, value: u64) -> Result<(), PromotionReceiptError> {
    if value > MAX_PROMOTION_DURABLE_INTEGER {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "value exceeds the signed 64-bit durable-store limit",
        });
    }
    Ok(())
}

fn validate_collection_bound(
    field: &'static str,
    length: usize,
    maximum: usize,
) -> Result<(), PromotionReceiptError> {
    if length > maximum {
        return Err(PromotionReceiptError::InvalidField {
            field,
            detail: "collection exceeds the hard entry limit",
        });
    }
    Ok(())
}

fn validate_strict_order<T>(
    values: &[T],
    key: impl Fn(&T) -> &str,
    collection: &'static str,
) -> Result<(), PromotionReceiptError> {
    for pair in values.windows(2) {
        let left = key(&pair[0]);
        let right = key(&pair[1]);
        if left == right {
            return Err(PromotionReceiptError::DuplicateEntry(collection));
        }
        if left > right {
            return Err(PromotionReceiptError::NonCanonicalOrder(collection));
        }
    }
    Ok(())
}

fn is_lower_hex(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn put_u8(bytes: &mut Vec<u8>, tag: u8, value: u8) {
    bytes.push(tag);
    bytes.push(value);
}

fn put_u16(bytes: &mut Vec<u8>, tag: u8, value: u16) {
    bytes.push(tag);
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn put_u32(bytes: &mut Vec<u8>, tag: u8, value: u32) {
    bytes.push(tag);
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn put_u64(bytes: &mut Vec<u8>, tag: u8, value: u64) {
    bytes.push(tag);
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn put_bool(bytes: &mut Vec<u8>, tag: u8, value: bool) {
    bytes.push(tag);
    bytes.push(u8::from(value));
}

fn put_bytes(bytes: &mut Vec<u8>, tag: u8, value: &[u8]) {
    bytes.push(tag);
    bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn put_string(bytes: &mut Vec<u8>, tag: u8, value: &str) {
    put_bytes(bytes, tag, value.as_bytes());
}

fn put_string_collection(bytes: &mut Vec<u8>, tag: u8, values: &[String]) {
    let mut collection = Vec::new();
    put_u32(&mut collection, 1, values.len() as u32);
    for value in values {
        put_string(&mut collection, 2, value);
    }
    put_bytes(bytes, tag, &collection);
}

fn put_digest_collection(bytes: &mut Vec<u8>, tag: u8, values: &[Sha256Digest]) {
    let mut collection = Vec::new();
    put_u32(&mut collection, 1, values.len() as u32);
    for value in values {
        put_string(&mut collection, 2, value.as_str());
    }
    put_bytes(bytes, tag, &collection);
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

fn decode_hex<const N: usize>(value: &str) -> Result<[u8; N], ()> {
    if value.len() != N * 2 || !is_lower_hex(value) {
        return Err(());
    }
    let mut decoded = [0u8; N];
    for (index, output) in decoded.iter_mut().enumerate() {
        let high = decode_nibble(value.as_bytes()[index * 2])?;
        let low = decode_nibble(value.as_bytes()[index * 2 + 1])?;
        *output = (high << 4) | low;
    }
    Ok(decoded)
}

fn decode_nibble(byte: u8) -> Result<u8, ()> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use ed25519_dalek::Signer as _;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    fn verify_and_consume_promotion_receipt_v1(
        packet: &SignedPromotionReceiptV1,
        expected_bindings: &PromotionBindingsV1,
        trust_root: &PromotionTrustRootV1,
        revocations: &PromotionRevocationsV1,
        now_unix_seconds: u64,
        replay_store: &dyn PromotionReceiptReplayStore,
    ) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
        validate_durable_integer("verification Unix seconds", now_unix_seconds)?;
        let trust_root_sha256 = trust_root.canonical_sha256()?;
        let revocations_sha256 = revocations.canonical_sha256()?;
        let fixed_test_genesis = digest(0xee);
        let test_chain_sha256 = digest(0xed);
        let test_checkpoint = PinnedPromotionHeadCheckpointV1 {
            source_json_sha256: digest(0xeb),
            checkpoint_sha256: digest(0xec),
            trust_root_id: trust_root.trust_root_id.clone(),
            genesis_trust_root_sha256: fixed_test_genesis,
            terminal_history_chain_sha256: test_chain_sha256.clone(),
            terminal_trust_root_revision: trust_root.revision,
            terminal_trust_root_sha256: trust_root_sha256.clone(),
            terminal_revocation_revision: revocations.revision,
            terminal_revocations_sha256: revocations_sha256.clone(),
        };
        let history = [PromotionTrustStateHistoryEntry {
            trust_root_revision: trust_root.revision,
            trust_root_sha256: trust_root_sha256.clone(),
            revocation_revision: revocations.revision,
            revocations_sha256: revocations_sha256.clone(),
            history_chain_sha256: test_chain_sha256.clone(),
        }];
        futures::executor::block_on(replay_store.ratchet_trusted_config(
            PromotionTrustedConfigRatchet {
                checkpoint: &test_checkpoint,
                history: &history,
                trust_root,
                trust_root_sha256: &trust_root_sha256,
                revocations,
                revocations_sha256: &revocations_sha256,
                history_chain_sha256: &test_chain_sha256,
                observed_at_unix_seconds: now_unix_seconds,
            },
        ))
        .map_err(map_replay_store_error)?;
        let anchored = AnchoredPromotionTrust {
            checkpoint_sha256: test_checkpoint.checkpoint_sha256,
            trust_root: trust_root.clone(),
            trust_root_sha256,
            revocations: revocations.clone(),
            revocations_sha256,
            history_chain_sha256: test_chain_sha256,
            observed_at_unix_seconds: now_unix_seconds,
        };
        futures::executor::block_on(super::verify_and_consume_promotion_receipt_at(
            packet,
            expected_bindings,
            &anchored,
            replay_store,
        ))
    }

    #[derive(Default)]
    struct GetterCheckingReplayStore {
        inner: InMemoryPromotionReceiptReplayStore,
    }

    #[derive(Default)]
    struct GetterCheckingRatchetStore {
        inner: InMemoryPromotionReceiptReplayStore,
    }

    impl PromotionReceiptReplayStore for GetterCheckingRatchetStore {
        fn ratchet_trusted_config<'a>(
            &'a self,
            ratchet: PromotionTrustedConfigRatchet<'a>,
        ) -> PromotionReplayFuture<'a> {
            assert_eq!(
                ratchet.genesis_trust_root_sha256().as_str(),
                "78536584da6f70b144e1790cfe24f161c46d1b5c6b0f42c30fd4ccf427c89458"
            );
            assert_eq!(ratchet.trust_root().revision, 1);
            assert_eq!(ratchet.revocations().revision, 1);
            assert_eq!(ratchet.history().len(), 1);
            assert_eq!(
                ratchet.history()[0].trust_root_sha256(),
                ratchet.trust_root_sha256()
            );
            assert_eq!(
                ratchet.history()[0].revocations_sha256(),
                ratchet.revocations_sha256()
            );
            assert_eq!(
                ratchet.history()[0].history_chain_sha256(),
                ratchet.history_chain_sha256()
            );
            assert_eq!(
                ratchet.checkpoint().terminal_history_chain_sha256(),
                ratchet.history_chain_sha256()
            );
            assert_eq!(ratchet.observed_at_unix_seconds(), NOW);
            self.inner.ratchet_trusted_config(ratchet)
        }

        fn check_and_consume<'a>(
            &'a self,
            consumption: PromotionReplayConsumption<'a>,
        ) -> PromotionReplayFuture<'a> {
            self.inner.check_and_consume(consumption)
        }
    }

    impl PromotionReceiptReplayStore for GetterCheckingReplayStore {
        fn ratchet_trusted_config<'a>(
            &'a self,
            ratchet: PromotionTrustedConfigRatchet<'a>,
        ) -> PromotionReplayFuture<'a> {
            self.inner.ratchet_trusted_config(ratchet)
        }

        fn check_and_consume<'a>(
            &'a self,
            consumption: PromotionReplayConsumption<'a>,
        ) -> PromotionReplayFuture<'a> {
            assert_eq!(consumption.trust_root_id(), "hepta-promotion-root-2026q3");
            assert_eq!(consumption.checkpoint_sha256(), &digest(0xec));
            assert_eq!(consumption.trust_root_revision(), 1);
            assert_eq!(consumption.revocation_revision(), 1);
            assert_eq!(consumption.history_chain_sha256(), &digest(0xed));
            assert_eq!(
                consumption.trust_root_sha256().as_str(),
                "78536584da6f70b144e1790cfe24f161c46d1b5c6b0f42c30fd4ccf427c89458"
            );
            assert_eq!(
                consumption.revocations_sha256().as_str(),
                "af3f507dcd8be745f3aa9e66d9c6cf0ed1e932fdfaed10cf31cd633cca0b826c"
            );
            assert_eq!(
                consumption.receipt_sha256().as_str(),
                "c832e655138e9c0e3e0241288b4b354a3427ddbdc442626824896866b9c78f2c"
            );
            assert_eq!(consumption.nonce(), "55".repeat(32));
            assert_eq!(consumption.expires_at_unix_seconds(), EXPIRES_AT);
            assert_eq!(consumption.observed_at_unix_seconds(), NOW);
            self.inner.check_and_consume(consumption)
        }
    }

    const NOW: u64 = 1_800_000_100;
    const ISSUED_AT: u64 = 1_800_000_000;
    const EXPIRES_AT: u64 = 1_800_003_600;
    const GENESIS_EFFECTIVE_AT: u64 = ISSUED_AT - 20;
    const REVOCATIONS_V1_EFFECTIVE_AT: u64 = ISSUED_AT - 8;
    const ROOT_V2_EFFECTIVE_AT: u64 = ISSUED_AT - 6;
    const REVOCATIONS_V2_EFFECTIVE_AT: u64 = ISSUED_AT - 4;
    const ROOT_V3_EFFECTIVE_AT: u64 = ISSUED_AT - 2;
    const REVOCATIONS_V3_EFFECTIVE_AT: u64 = ISSUED_AT - 1;

    fn digest(byte: u8) -> Sha256Digest {
        Sha256Digest::parse(format!("{byte:02x}").repeat(32)).expect("test digest")
    }

    fn git(byte: u8) -> GitObjectId {
        GitObjectId::parse(format!("{byte:02x}").repeat(20)).expect("test git object id")
    }

    fn bindings() -> PromotionBindingsV1 {
        PromotionBindingsV1 {
            candidate: CandidateBindingV1 {
                commit: git(0x11),
                tree: git(0x12),
            },
            qualification: QualificationBindingV1 {
                manifest_sha256: digest(0x21),
                platform_receipts: vec![
                    QualificationPlatformReceiptV1 {
                        platform: "linux-x86_64".to_string(),
                        receipt_sha256: digest(0x22),
                    },
                    QualificationPlatformReceiptV1 {
                        platform: "macos-aarch64".to_string(),
                        receipt_sha256: digest(0x23),
                    },
                    QualificationPlatformReceiptV1 {
                        platform: "nix-linux-x86_64".to_string(),
                        receipt_sha256: digest(0x24),
                    },
                ],
            },
            frozen_oracle: FrozenOracleBindingV1 {
                commit: git(0x31),
                tree: git(0x32),
                manifest_sha256: digest(0x33),
            },
            canonical_oracle_conformance: CanonicalOracleConformanceBindingV1 {
                manifest_sha256: digest(0x41),
            },
            product_shadow_soak: ProductShadowSoakBindingV1 {
                manifest_sha256: digest(0x42),
            },
        }
    }

    fn receipt() -> PromotionReceiptV1 {
        PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            1,
            1,
            bindings(),
            ISSUED_AT,
            EXPIRES_AT,
            "55".repeat(32),
        )
    }

    fn signing_key(seed: u8) -> SigningKey {
        SigningKey::from_bytes(&[seed; 32])
    }

    fn trusted_key(
        key_id: &str,
        key: &SigningKey,
        valid_from: u64,
        valid_until: u64,
    ) -> TrustedPromotionKeyV1 {
        TrustedPromotionKeyV1 {
            key_id: key_id.to_string(),
            algorithm: PromotionSignatureAlgorithm::Ed25519,
            public_key_hex: ed25519_public_key_hex(key),
            valid_from_unix_seconds: valid_from,
            valid_until_unix_seconds: valid_until,
            revoked_at_unix_seconds: None,
        }
    }

    fn root(keys: Vec<TrustedPromotionKeyV1>, threshold: u16) -> PromotionTrustRootV1 {
        PromotionTrustRootV1 {
            schema_version: PROMOTION_TRUST_ROOT_SCHEMA_VERSION,
            trust_root_id: "hepta-promotion-root-2026q3".to_string(),
            revision: 1,
            effective_at_unix_seconds: GENESIS_EFFECTIVE_AT,
            signature_threshold: threshold,
            max_receipt_lifetime_seconds: 7_200,
            keys,
        }
    }

    fn revocations() -> PromotionRevocationsV1 {
        PromotionRevocationsV1::empty(
            "hepta-promotion-root-2026q3",
            1,
            REVOCATIONS_V1_EFFECTIVE_AT,
        )
    }

    fn packet_one(key: &SigningKey) -> SignedPromotionReceiptV1 {
        sign_promotion_receipt_v1(
            receipt(),
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: key,
            }],
        )
        .expect("sign test receipt")
    }

    fn complete_history_v1(key: &SigningKey) -> (Sha256Digest, PromotionTrustHistoryV1) {
        let genesis = root(
            vec![trusted_key("key-a", key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let revocations_v1 = revocations();
        let signed_v1 = sign_promotion_revocations_v1(
            &genesis,
            None,
            revocations_v1,
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: key,
            }],
        )
        .expect("signed genesis revocations");
        let genesis_sha256 = genesis.canonical_sha256().expect("genesis digest");
        (
            genesis_sha256,
            PromotionTrustHistoryV1 {
                schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
                genesis,
                updates: vec![PromotionTrustUpdateV1::Revocations(signed_v1)],
            },
        )
    }

    fn complete_history_v2(key: &SigningKey) -> (Sha256Digest, PromotionTrustHistoryV1) {
        let (genesis_sha256, mut history) = complete_history_v1(key);
        let predecessor_revocations = match &history.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => unreachable!("fixture order"),
        };
        let mut successor = history.genesis.clone();
        successor.revision = 2;
        successor.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        successor.max_receipt_lifetime_seconds += 1;
        let rotation = sign_promotion_trust_root_rotation_v1(
            &history.genesis,
            &predecessor_revocations,
            successor,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: key,
            }],
        )
        .expect("signed root rotation");
        history
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation));
        (genesis_sha256, history)
    }

    fn complete_history_v2_rev2(
        key: &SigningKey,
        revoked_key_ids: Vec<String>,
        revoked_receipts: Vec<Sha256Digest>,
        revoked_nonces: Vec<String>,
    ) -> (Sha256Digest, PromotionTrustHistoryV1) {
        let (genesis_sha256, mut history) = complete_history_v2(key);
        let current_root = match history.updates.last().expect("root rotation") {
            PromotionTrustUpdateV1::TrustRootRotation(rotation) => rotation.successor.clone(),
            PromotionTrustUpdateV1::Revocations(_) => unreachable!("fixture order"),
        };
        let predecessor = match &history.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => unreachable!("fixture order"),
        };
        let revocations_v2 = PromotionRevocationsV1 {
            schema_version: PROMOTION_REVOCATIONS_SCHEMA_VERSION,
            trust_root_id: current_root.trust_root_id.clone(),
            revision: 2,
            effective_at_unix_seconds: REVOCATIONS_V2_EFFECTIVE_AT,
            revoked_key_ids,
            revoked_receipt_sha256: revoked_receipts,
            revoked_nonces,
        };
        let signed_v2 = sign_promotion_revocations_v1(
            &current_root,
            Some(&predecessor),
            revocations_v2,
            REVOCATIONS_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: key,
            }],
        )
        .expect("signed revocation revision two");
        history
            .updates
            .push(PromotionTrustUpdateV1::Revocations(signed_v2));
        (genesis_sha256, history)
    }

    fn checkpoint_json_from_verified(verified: &VerifiedPromotionTrustHistory) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "schema_version": PROMOTION_HEAD_CHECKPOINT_SCHEMA_VERSION,
            "trust_root_id": verified.trust_root.trust_root_id,
            "genesis_trust_root_sha256": verified.genesis_trust_root_sha256,
            "terminal_history_chain_sha256": verified.history_chain_sha256,
            "terminal_trust_root_revision": verified.trust_root.revision,
            "terminal_trust_root_sha256": verified.trust_root_sha256,
            "terminal_revocation_revision": verified.revocations.revision,
            "terminal_revocations_sha256": verified.revocations_sha256,
        }))
        .expect("test checkpoint JSON")
    }

    fn checkpoint_from_verified(
        verified: &VerifiedPromotionTrustHistory,
    ) -> PinnedPromotionHeadCheckpointV1 {
        let checkpoint_json = checkpoint_json_from_verified(verified);
        let source_sha256 = Sha256Digest::for_bytes(&checkpoint_json);
        load_test_pinned_promotion_head_checkpoint_json_v1(&source_sha256, &checkpoint_json)
            .expect("strict test-only checkpoint")
    }

    fn checkpoint_for_history(
        history: &PromotionTrustHistoryV1,
        now: u64,
    ) -> Result<PinnedPromotionHeadCheckpointV1, PromotionReceiptError> {
        let genesis_sha256 = history.genesis.canonical_sha256()?;
        let verified = verify_promotion_trust_history(&genesis_sha256, history, now)?;
        Ok(checkpoint_from_verified(&verified))
    }

    fn anchor_with_checkpoint(
        checkpoint: &PinnedPromotionHeadCheckpointV1,
        history: &PromotionTrustHistoryV1,
        now: u64,
        store: &dyn PromotionReceiptReplayStore,
    ) -> Result<AnchoredPromotionTrust, PromotionReceiptError> {
        futures::executor::block_on(anchor_and_ratchet_promotion_trust_at(
            checkpoint, history, now, store,
        ))
    }

    fn anchor_public(
        genesis_sha256: &Sha256Digest,
        history: &PromotionTrustHistoryV1,
        now: u64,
        store: &dyn PromotionReceiptReplayStore,
    ) -> Result<AnchoredPromotionTrust, PromotionReceiptError> {
        let actual_genesis_sha256 = history.genesis.canonical_sha256()?;
        if &actual_genesis_sha256 != genesis_sha256 {
            let checkpoint = PinnedPromotionHeadCheckpointV1 {
                source_json_sha256: digest(0xca),
                checkpoint_sha256: digest(0xcb),
                trust_root_id: history.genesis.trust_root_id.clone(),
                genesis_trust_root_sha256: genesis_sha256.clone(),
                terminal_history_chain_sha256: digest(0xcc),
                terminal_trust_root_revision: 1,
                terminal_trust_root_sha256: digest(0xcd),
                terminal_revocation_revision: 1,
                terminal_revocations_sha256: digest(0xce),
            };
            return anchor_with_checkpoint(&checkpoint, history, now, store);
        }
        let checkpoint = checkpoint_for_history(history, now)?;
        anchor_with_checkpoint(&checkpoint, history, now, store)
    }

    fn verify_public(
        packet: &SignedPromotionReceiptV1,
        expected_bindings: &PromotionBindingsV1,
        anchored: &AnchoredPromotionTrust,
        store: &dyn PromotionReceiptReplayStore,
    ) -> Result<VerifiedPromotionEvidence, PromotionReceiptError> {
        futures::executor::block_on(super::verify_and_consume_promotion_receipt_at(
            packet,
            expected_bindings,
            anchored,
            store,
        ))
    }

    fn sign_unchecked_revocation_update(
        trust_root: &PromotionTrustRootV1,
        predecessor: &PromotionRevocationsV1,
        mut revocations: PromotionRevocationsV1,
        issued_at_unix_seconds: u64,
        key: &SigningKey,
    ) -> SignedPromotionRevocationsV1 {
        revocations.effective_at_unix_seconds = issued_at_unix_seconds;
        let mut signed = SignedPromotionRevocationsV1 {
            schema_version: SIGNED_PROMOTION_REVOCATIONS_SCHEMA_VERSION,
            predecessor_trust_root_sha256: trust_root
                .canonical_sha256()
                .expect("current root digest"),
            predecessor_revocation_revision: predecessor.revision,
            predecessor_revocations_sha256: Some(
                predecessor
                    .canonical_sha256()
                    .expect("predecessor revocation digest"),
            ),
            revocations,
            issued_at_unix_seconds,
            signatures: Vec::new(),
        };
        signed.signatures = sign_statement(
            signed
                .canonical_statement()
                .expect("test revocation statement"),
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: key,
            }],
        );
        signed
    }

    struct PausingConsumeStore {
        inner: InMemoryPromotionReceiptReplayStore,
        pause_first: AtomicBool,
        consume_entered: Barrier,
        consume_resume: Barrier,
    }

    impl PausingConsumeStore {
        fn new() -> Self {
            Self {
                inner: InMemoryPromotionReceiptReplayStore::default(),
                pause_first: AtomicBool::new(true),
                consume_entered: Barrier::new(2),
                consume_resume: Barrier::new(2),
            }
        }
    }

    impl PromotionReceiptReplayStore for PausingConsumeStore {
        fn ratchet_trusted_config<'a>(
            &'a self,
            ratchet: PromotionTrustedConfigRatchet<'a>,
        ) -> PromotionReplayFuture<'a> {
            self.inner.ratchet_trusted_config(ratchet)
        }

        fn check_and_consume<'a>(
            &'a self,
            consumption: PromotionReplayConsumption<'a>,
        ) -> PromotionReplayFuture<'a> {
            Box::pin(async move {
                if self.pause_first.swap(false, Ordering::SeqCst) {
                    self.consume_entered.wait();
                    self.consume_resume.wait();
                }
                self.inner.check_and_consume(consumption).await
            })
        }
    }

    #[test]
    fn canonical_bytes_and_hash_are_pinned() {
        let bytes = receipt()
            .canonical_receipt_preimage()
            .expect("canonical bytes");
        assert_eq!(
            &bytes[..PROMOTION_RECEIPT_V1_PREIMAGE_DOMAIN.len()],
            PROMOTION_RECEIPT_V1_PREIMAGE_DOMAIN
        );
        assert_eq!(
            Sha256Digest::for_bytes(&bytes).as_str(),
            "c832e655138e9c0e3e0241288b4b354a3427ddbdc442626824896866b9c78f2c"
        );
        assert_eq!(bytes.len(), 941);
    }

    #[test]
    fn trusted_config_canonical_digests_are_pinned() {
        let key = signing_key(7);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        assert_eq!(
            trust_root
                .canonical_sha256()
                .expect("trust-root canonical digest")
                .as_str(),
            "78536584da6f70b144e1790cfe24f161c46d1b5c6b0f42c30fd4ccf427c89458"
        );
        assert_eq!(
            revocations()
                .canonical_sha256()
                .expect("revocation canonical digest")
                .as_str(),
            "af3f507dcd8be745f3aa9e66d9c6cf0ed1e932fdfaed10cf31cd633cca0b826c"
        );
    }

    #[test]
    fn history_chain_and_configuration_signature_vectors_are_pinned() {
        let key = signing_key(7);
        let (genesis_sha256, history) = complete_history_v2_rev2(&key, vec![], vec![], vec![]);
        let chain_0 = genesis_history_chain_sha256(&genesis_sha256);
        let chain_1 =
            extend_history_chain_sha256(&chain_0, 1, &history.updates[0]).expect("chain one");
        let chain_2 =
            extend_history_chain_sha256(&chain_1, 2, &history.updates[1]).expect("chain two");
        let chain_3 =
            extend_history_chain_sha256(&chain_2, 3, &history.updates[2]).expect("chain three");
        let PromotionTrustUpdateV1::Revocations(revocations_v1) = &history.updates[0] else {
            panic!("fixture revocations")
        };
        let PromotionTrustUpdateV1::TrustRootRotation(rotation_v2) = &history.updates[1] else {
            panic!("fixture rotation")
        };
        let revocation_message = revocations_v1
            .canonical_signing_bytes_for_signer("key-a", PromotionSignatureAlgorithm::Ed25519)
            .expect("revocation signing bytes");
        let rotation_message = rotation_v2
            .canonical_signing_bytes_for_signer("key-a", PromotionSignatureAlgorithm::Ed25519)
            .expect("rotation signing bytes");
        let verified = verify_promotion_trust_history(&genesis_sha256, &history, NOW)
            .expect("verified fixture history");
        let checkpoint = checkpoint_from_verified(&verified);

        assert_eq!(
            chain_0.as_str(),
            "1758707106adbf2111633e4beee56a4e13f648d493ca9ca5d2d0936a9c2b28e7"
        );
        assert_eq!(
            chain_1.as_str(),
            "5d230ace0f2bfadb79742d23e48c177d78c81571e939101815625257be518581"
        );
        assert_eq!(
            chain_2.as_str(),
            "d2815403c25c5b244b23d3eb970376701fd82d2980cb85e150f318e7b0e2f180"
        );
        assert_eq!(
            chain_3.as_str(),
            "cacc025110ab42aaac752a005fc92849ab5fa68873cf9df537e7797981bf2cb7"
        );
        assert_eq!(
            Sha256Digest::for_bytes(&revocation_message).as_str(),
            "b78d2d625883ba2ed900f33c92436bcf3ee4be978fdeb44c0944f15c95b433a1"
        );
        assert_eq!(revocation_message.len(), 226);
        assert_eq!(
            revocations_v1.signatures[0].signature_hex,
            "97ebe89fb3959a8b59f7718a3612b6575dc2bff7f336628003fbec06c32dc092aaf2f9388806a3de392fa7ab86be80c770086599d64d38b0f479e84b3be1ef01"
        );
        assert_eq!(
            Sha256Digest::for_bytes(&rotation_message).as_str(),
            "1c88c3ffa7dca72753644123929fe37c6c71481b155c9bf35952016fca20d972"
        );
        assert_eq!(rotation_message.len(), 291);
        assert_eq!(
            rotation_v2.signatures[0].signature_hex,
            "f0768411402bb82490f0a14548c87b35ec81fe1a588b8e45d7adb7bc6f5ccc980f49b53a8601438a192fafeca323e27c349f03c6b188f467ce92fc215022010d"
        );
        assert_eq!(
            checkpoint.checkpoint_sha256().as_str(),
            "da1f765c4effd1d64a3c8947d396c77c1425161a20d1754a89b1056cdd01548e"
        );

        let mut equivalent_update = history.updates[1].clone();
        let PromotionTrustUpdateV1::TrustRootRotation(rotation) = &mut equivalent_update else {
            panic!("fixture rotation")
        };
        rotation.signatures.clear();
        assert_eq!(
            extend_history_chain_sha256(&chain_1, 2, &equivalent_update)
                .expect("signature-set-independent chain"),
            chain_2
        );
    }

    #[test]
    fn json_field_order_does_not_change_canonical_message() {
        let original = receipt();
        let reordered = json!({
            "promotion_authorized": false,
            "operator_accepted": false,
            "nonce": original.nonce,
            "expires_at_unix_seconds": original.expires_at_unix_seconds,
            "issued_at_unix_seconds": original.issued_at_unix_seconds,
            "bindings": original.bindings,
            "minimum_revocation_revision": original.minimum_revocation_revision,
            "minimum_trust_root_revision": original.minimum_trust_root_revision,
            "trust_root_id": original.trust_root_id,
            "schema_version": original.schema_version,
        });
        let parsed: PromotionReceiptV1 =
            serde_json::from_value(reordered).expect("strict reordered receipt");
        assert_eq!(
            parsed
                .canonical_receipt_preimage()
                .expect("parsed canonical"),
            receipt()
                .canonical_receipt_preimage()
                .expect("fixture canonical")
        );
    }

    #[test]
    fn valid_threshold_packet_verifies_and_has_no_authority() {
        let key_a = signing_key(7);
        let key_b = signing_key(9);
        let packet = sign_promotion_receipt_v1(
            receipt(),
            &[
                PromotionSignerV1 {
                    key_id: "key-a",
                    signing_key: &key_a,
                },
                PromotionSignerV1 {
                    key_id: "key-b",
                    signing_key: &key_b,
                },
            ],
        )
        .expect("two signatures");
        let trust_root = root(
            vec![
                trusted_key("key-a", &key_a, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-b", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            2,
        );
        let store = InMemoryPromotionReceiptReplayStore::default();
        let verified = verify_and_consume_promotion_receipt_v1(
            &packet,
            &bindings(),
            &trust_root,
            &revocations(),
            NOW,
            &store,
        )
        .expect("verified evidence");

        assert_eq!(verified.bindings(), &bindings());
        assert_eq!(verified.trust_root_id(), trust_root.trust_root_id);
        assert_eq!(verified.trust_root_revision(), 1);
        assert_eq!(verified.revocation_revision(), 1);
        assert_eq!(verified.nonce(), "55".repeat(32));
    }

    #[test]
    fn replay_capability_exposes_only_read_only_verified_getters() {
        let key = signing_key(7);
        let packet = packet_one(&key);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        verify_and_consume_promotion_receipt_v1(
            &packet,
            &bindings(),
            &trust_root,
            &revocations(),
            NOW,
            &GetterCheckingReplayStore::default(),
        )
        .expect("verifier minted replay capability");
    }

    #[test]
    fn wrong_domain_signature_is_rejected_without_consuming_nonce() {
        let key = signing_key(7);
        let receipt = receipt();
        let mut wrong_message = b"hepta.vnext/wrong-domain/v1\0".to_vec();
        wrong_message.extend_from_slice(&receipt.canonical_payload_bytes_unchecked());
        let packet = SignedPromotionReceiptV1 {
            receipt,
            signatures: vec![PromotionSignatureV1 {
                key_id: "key-a".to_string(),
                algorithm: PromotionSignatureAlgorithm::Ed25519,
                signature_hex: encode_hex(&key.sign(&wrong_message).to_bytes()),
            }],
        };
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let store = InMemoryPromotionReceiptReplayStore::default();
        assert!(matches!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::SignatureVerificationFailed(key_id)) if key_id == "key-a"
        ));
        verify_and_consume_promotion_receipt_v1(
            &packet_one(&key),
            &bindings(),
            &trust_root,
            &revocations(),
            NOW,
            &store,
        )
        .expect("invalid signature did not burn nonce");
    }

    #[test]
    fn signature_cannot_be_relabelled_to_same_public_key_under_new_id() {
        let key = signing_key(7);
        let mut packet = packet_one(&key);
        packet.signatures[0].key_id = "key-b".to_string();
        let trust_root = root(
            vec![trusted_key("key-b", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::SignatureVerificationFailed(
                "key-b".to_string()
            ))
        );
    }

    #[test]
    fn unknown_versions_and_fields_fail_closed() {
        let key = signing_key(7);
        let mut packet = packet_one(&key);
        packet.receipt.schema_version = 99;
        assert!(matches!(
            packet.validate_structure(),
            Err(PromotionReceiptError::UnsupportedSchemaVersion {
                component: "promotion receipt",
                found: 99,
            })
        ));

        let mut value = serde_json::to_value(packet_one(&key)).expect("packet JSON");
        value
            .as_object_mut()
            .expect("packet object")
            .insert("future_field".to_string(), json!(true));
        assert!(matches!(
            SignedPromotionReceiptV1::from_json_slice(
                &serde_json::to_vec(&value).expect("JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));
        for required_gate in ["canonical_oracle_conformance", "product_shadow_soak"] {
            let mut missing = serde_json::to_value(packet_one(&key)).expect("packet JSON");
            missing["receipt"]["bindings"]
                .as_object_mut()
                .expect("bindings object")
                .remove(required_gate);
            assert!(matches!(
                SignedPromotionReceiptV1::from_json_slice(
                    &serde_json::to_vec(&missing).expect("missing-gate JSON bytes")
                ),
                Err(PromotionReceiptError::Json(_))
            ));
        }
    }

    #[test]
    fn oversized_untrusted_inputs_fail_closed_before_crypto_work() {
        let key = signing_key(7);
        assert!(matches!(
            SignedPromotionReceiptV1::from_json_slice(&vec![
                b' ';
                MAX_PROMOTION_PACKET_JSON_BYTES + 1
            ]),
            Err(PromotionReceiptError::InvalidField {
                field: "signed promotion receipt JSON",
                ..
            })
        ));

        let mut too_many_signatures = packet_one(&key);
        too_many_signatures.signatures = (0..=MAX_PROMOTION_SIGNATURES)
            .map(|index| PromotionSignatureV1 {
                key_id: format!("key-{index:02}"),
                algorithm: PromotionSignatureAlgorithm::Ed25519,
                signature_hex: "00".repeat(64),
            })
            .collect();
        assert!(matches!(
            too_many_signatures.validate_structure(),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion signatures",
                ..
            })
        ));

        let mut too_many_platforms = receipt();
        too_many_platforms.bindings.qualification.platform_receipts = (0
            ..=MAX_QUALIFICATION_PLATFORM_RECEIPTS)
            .map(|index| QualificationPlatformReceiptV1 {
                platform: format!("platform-{index:02}"),
                receipt_sha256: digest(0x22),
            })
            .collect();
        assert!(matches!(
            too_many_platforms.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "qualification platform receipts",
                ..
            })
        ));

        let too_many_keys = root(
            (0..=MAX_TRUSTED_PROMOTION_KEYS)
                .map(|index| {
                    trusted_key(
                        &format!("key-{index:02}"),
                        &signing_key((index + 1) as u8),
                        ISSUED_AT - 10,
                        EXPIRES_AT + 10,
                    )
                })
                .collect(),
            1,
        );
        assert!(matches!(
            too_many_keys.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "trusted promotion keys",
                ..
            })
        ));

        let mut too_many_revocations = revocations();
        too_many_revocations.revoked_nonces = (0..=MAX_PROMOTION_REVOCATIONS_PER_KIND)
            .map(|index| format!("{index:064x}"))
            .collect();
        assert!(matches!(
            too_many_revocations.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "revoked receipt nonces",
                ..
            })
        ));

        let oversized_config = vec![b' '; MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES + 1];
        assert!(matches!(
            PromotionTrustRootV1::from_json_slice(&oversized_config),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion trust-root JSON",
                ..
            })
        ));
        assert!(matches!(
            PromotionRevocationsV1::from_json_slice(&oversized_config),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion revocations JSON",
                ..
            })
        ));
    }

    #[test]
    fn trusted_config_json_rejects_unknown_and_duplicate_fields() {
        let key = signing_key(7);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let mut root_value = serde_json::to_value(&trust_root).expect("trust-root JSON");
        root_value
            .as_object_mut()
            .expect("trust-root object")
            .insert("future_field".to_string(), json!(true));
        assert!(matches!(
            PromotionTrustRootV1::from_json_slice(
                &serde_json::to_vec(&root_value).expect("trust-root JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));
        let root_json = serde_json::to_string(&trust_root).expect("trust-root JSON string");
        let duplicate_root = root_json.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1,
        );
        assert!(matches!(
            PromotionTrustRootV1::from_json_slice(duplicate_root.as_bytes()),
            Err(PromotionReceiptError::Json(_))
        ));
        let mut missing_root_effective =
            serde_json::to_value(&trust_root).expect("trust-root JSON");
        missing_root_effective
            .as_object_mut()
            .expect("trust-root object")
            .remove("effective_at_unix_seconds");
        assert!(matches!(
            PromotionTrustRootV1::from_json_slice(
                &serde_json::to_vec(&missing_root_effective).expect("trust-root JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));

        let revocations = revocations();
        let mut revocation_value = serde_json::to_value(&revocations).expect("revocation JSON");
        revocation_value
            .as_object_mut()
            .expect("revocation object")
            .insert("future_field".to_string(), json!(true));
        assert!(matches!(
            PromotionRevocationsV1::from_json_slice(
                &serde_json::to_vec(&revocation_value).expect("revocation JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));
        let revocation_json = serde_json::to_string(&revocations).expect("revocation JSON string");
        let duplicate_revocation =
            revocation_json.replacen("\"revision\":1", "\"revision\":1,\"revision\":1", 1);
        assert!(matches!(
            PromotionRevocationsV1::from_json_slice(duplicate_revocation.as_bytes()),
            Err(PromotionReceiptError::Json(_))
        ));
        let mut missing_revocation_effective =
            serde_json::to_value(&revocations).expect("revocation JSON");
        missing_revocation_effective
            .as_object_mut()
            .expect("revocation object")
            .remove("effective_at_unix_seconds");
        assert!(matches!(
            PromotionRevocationsV1::from_json_slice(
                &serde_json::to_vec(&missing_revocation_effective).expect("revocation JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));
    }

    #[test]
    fn authority_flags_are_always_rejected() {
        let mut accepted = receipt();
        accepted.operator_accepted = true;
        assert_eq!(
            accepted.validate(),
            Err(PromotionReceiptError::AuthorityClaimed)
        );
        let mut authorized = receipt();
        authorized.promotion_authorized = true;
        assert_eq!(
            authorized.validate(),
            Err(PromotionReceiptError::AuthorityClaimed)
        );
    }

    #[test]
    fn exact_binding_mismatch_fails_before_nonce_consumption() {
        let key = signing_key(7);
        let packet = packet_one(&key);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let store = InMemoryPromotionReceiptReplayStore::default();
        let mut wrong_conformance = bindings();
        wrong_conformance
            .canonical_oracle_conformance
            .manifest_sha256 = digest(0xfe);
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &wrong_conformance,
                &trust_root,
                &revocations(),
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::BindingMismatch)
        );
        let mut wrong_product_shadow = bindings();
        wrong_product_shadow.product_shadow_soak.manifest_sha256 = digest(0xff);
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &wrong_product_shadow,
                &trust_root,
                &revocations(),
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::BindingMismatch)
        );
        verify_and_consume_promotion_receipt_v1(
            &packet,
            &bindings(),
            &trust_root,
            &revocations(),
            NOW,
            &store,
        )
        .expect("neither evidence-gate mismatch burned the nonce");
    }

    #[test]
    fn signed_minimum_trust_revisions_fail_closed() {
        let key = signing_key(7);
        let mut minimum_root = receipt();
        minimum_root.minimum_trust_root_revision = 2;
        let root_packet = sign_promotion_receipt_v1(
            minimum_root,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("minimum root packet");
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &root_packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::TrustRootRevisionTooOld {
                minimum: 2,
                found: 1,
            })
        );

        let mut minimum_revocation = receipt();
        minimum_revocation.minimum_revocation_revision = 2;
        let revocation_packet = sign_promotion_receipt_v1(
            minimum_revocation,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("minimum revocation packet");
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &revocation_packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::RevocationRevisionTooOld {
                minimum: 2,
                found: 1,
            })
        );
    }

    #[test]
    fn replay_store_rejects_trust_configuration_revision_rollback() {
        let key = signing_key(7);
        let mut high_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        high_root.revision = 3;
        let mut high_revocations = revocations();
        high_revocations.revision = 4;
        let store = InMemoryPromotionReceiptReplayStore::default();
        verify_and_consume_promotion_receipt_v1(
            &packet_one(&key),
            &bindings(),
            &high_root,
            &high_revocations,
            NOW,
            &store,
        )
        .expect("high revisions establish watermarks");

        let mut root_rollback_receipt = receipt();
        root_rollback_receipt.nonce = "66".repeat(32);
        let root_rollback_packet = sign_promotion_receipt_v1(
            root_rollback_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("root rollback packet");
        let mut low_root = high_root.clone();
        low_root.revision = 2;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &root_rollback_packet,
                &bindings(),
                &low_root,
                &high_revocations,
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::TrustRootRevisionRollback {
                highest_seen: 3,
                found: 2,
            })
        );
        verify_and_consume_promotion_receipt_v1(
            &root_rollback_packet,
            &bindings(),
            &high_root,
            &high_revocations,
            NOW,
            &store,
        )
        .expect("rollback error did not consume root packet");

        let mut revocation_rollback_receipt = receipt();
        revocation_rollback_receipt.nonce = "77".repeat(32);
        let revocation_rollback_packet = sign_promotion_receipt_v1(
            revocation_rollback_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("revocation rollback packet");
        let mut low_revocations = high_revocations.clone();
        low_revocations.revision = 3;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &revocation_rollback_packet,
                &bindings(),
                &high_root,
                &low_revocations,
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::RevocationRevisionRollback {
                highest_seen: 4,
                found: 3,
            })
        );
        verify_and_consume_promotion_receipt_v1(
            &revocation_rollback_packet,
            &bindings(),
            &high_root,
            &high_revocations,
            NOW,
            &store,
        )
        .expect("rollback error did not consume revocation packet");
    }

    #[test]
    fn replay_store_rejects_same_revision_configuration_replacement() {
        let key = signing_key(7);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let revocations = revocations();
        let root_store = InMemoryPromotionReceiptReplayStore::default();
        verify_and_consume_promotion_receipt_v1(
            &packet_one(&key),
            &bindings(),
            &trust_root,
            &revocations,
            NOW,
            &root_store,
        )
        .expect("baseline trusted configuration");

        let mut root_conflict_receipt = receipt();
        root_conflict_receipt.nonce = "66".repeat(32);
        let root_conflict_packet = sign_promotion_receipt_v1(
            root_conflict_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("root conflict packet");
        let mut conflicting_root = trust_root.clone();
        conflicting_root.max_receipt_lifetime_seconds += 1;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &root_conflict_packet,
                &bindings(),
                &conflicting_root,
                &revocations,
                NOW,
                &root_store,
            ),
            Err(PromotionReceiptError::TrustRootRevisionConflict { revision: 1 })
        );
        conflicting_root.revision = 2;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &root_conflict_packet,
                &bindings(),
                &conflicting_root,
                &revocations,
                NOW,
                &root_store,
            ),
            Err(PromotionReceiptError::TrustRootHistoryMismatch)
        );
        verify_and_consume_promotion_receipt_v1(
            &root_conflict_packet,
            &bindings(),
            &trust_root,
            &revocations,
            NOW,
            &root_store,
        )
        .expect("rejected bare successor left nonce and watermarks unchanged");

        let revocation_store = InMemoryPromotionReceiptReplayStore::default();
        verify_and_consume_promotion_receipt_v1(
            &packet_one(&key),
            &bindings(),
            &trust_root,
            &revocations,
            NOW,
            &revocation_store,
        )
        .expect("baseline revocation configuration");
        let mut revocation_conflict_receipt = receipt();
        revocation_conflict_receipt.nonce = "77".repeat(32);
        let revocation_conflict_packet = sign_promotion_receipt_v1(
            revocation_conflict_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("revocation conflict packet");
        let mut conflicting_revocations = revocations.clone();
        conflicting_revocations
            .revoked_key_ids
            .push("key-z".to_string());
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &revocation_conflict_packet,
                &bindings(),
                &trust_root,
                &conflicting_revocations,
                NOW,
                &revocation_store,
            ),
            Err(PromotionReceiptError::RevocationRevisionConflict { revision: 1 })
        );
        conflicting_revocations.revision = 2;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &revocation_conflict_packet,
                &bindings(),
                &trust_root,
                &conflicting_revocations,
                NOW,
                &revocation_store,
            ),
            Err(PromotionReceiptError::TrustRootHistoryMismatch)
        );
        verify_and_consume_promotion_receipt_v1(
            &revocation_conflict_packet,
            &bindings(),
            &trust_root,
            &revocations,
            NOW,
            &revocation_store,
        )
        .expect("rejected bare revocation successor left state unchanged");
    }

    #[test]
    fn replay_error_does_not_partially_ratchet_revisions() {
        let key = signing_key(7);
        let root_v1 = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let revocations_v1 = revocations();
        let packet = packet_one(&key);
        let store = InMemoryPromotionReceiptReplayStore::default();
        verify_and_consume_promotion_receipt_v1(
            &packet,
            &bindings(),
            &root_v1,
            &revocations_v1,
            NOW,
            &store,
        )
        .expect("baseline packet");

        let mut root_v3 = root_v1.clone();
        root_v3.revision = 3;
        let mut revocations_v4 = revocations_v1.clone();
        revocations_v4.revision = 4;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &root_v3,
                &revocations_v4,
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::TrustRootHistoryMismatch)
        );

        let mut packet_v2_receipt = receipt();
        packet_v2_receipt.nonce = "66".repeat(32);
        let packet_v2 = sign_promotion_receipt_v1(
            packet_v2_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("v2 packet");
        verify_and_consume_promotion_receipt_v1(
            &packet_v2,
            &bindings(),
            &root_v1,
            &revocations_v1,
            NOW,
            &store,
        )
        .expect("rejected unproven history left revision watermarks unchanged");
    }

    #[test]
    fn nonce_replay_error_is_also_all_or_nothing() {
        let key = signing_key(7);
        let root_v1 = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let revocations_v1 = revocations();
        let store = InMemoryPromotionReceiptReplayStore::default();
        verify_and_consume_promotion_receipt_v1(
            &packet_one(&key),
            &bindings(),
            &root_v1,
            &revocations_v1,
            NOW,
            &store,
        )
        .expect("baseline nonce");

        let mut nonce_replay_receipt = receipt();
        nonce_replay_receipt.expires_at_unix_seconds += 1;
        let nonce_replay_packet = sign_promotion_receipt_v1(
            nonce_replay_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("nonce replay packet");
        let mut root_v3 = root_v1.clone();
        root_v3.revision = 3;
        let mut revocations_v4 = revocations_v1.clone();
        revocations_v4.revision = 4;
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &nonce_replay_packet,
                &bindings(),
                &root_v3,
                &revocations_v4,
                NOW,
                &store,
            ),
            Err(PromotionReceiptError::TrustRootHistoryMismatch)
        );

        let mut fresh_receipt = receipt();
        fresh_receipt.nonce = "66".repeat(32);
        let fresh_packet = sign_promotion_receipt_v1(
            fresh_receipt,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("fresh packet");
        verify_and_consume_promotion_receipt_v1(
            &fresh_packet,
            &bindings(),
            &root_v1,
            &revocations_v1,
            NOW,
            &store,
        )
        .expect("rejected unproven history left all store state unchanged");
    }

    #[test]
    fn future_expired_and_overlong_receipts_fail_closed() {
        let key = signing_key(7);
        let trust_root = root(
            vec![trusted_key(
                "key-a",
                &key,
                ISSUED_AT - 10,
                EXPIRES_AT + 20_000,
            )],
            1,
        );
        let packet = packet_one(&key);
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                ISSUED_AT - 1,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::ReceiptNotYetValid)
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                EXPIRES_AT,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::ReceiptExpired)
        );

        let mut long = receipt();
        long.expires_at_unix_seconds = ISSUED_AT + 7_201;
        let long_packet = sign_promotion_receipt_v1(
            long,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("long receipt signs structurally");
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &long_packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::ReceiptLifetimeTooLong)
        );
    }

    #[test]
    fn durable_integer_boundary_is_signed_64_bit() {
        let mut boundary_receipt = receipt();
        boundary_receipt.minimum_trust_root_revision = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_receipt.minimum_revocation_revision = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_receipt.issued_at_unix_seconds = MAX_PROMOTION_DURABLE_INTEGER - 1;
        boundary_receipt.expires_at_unix_seconds = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_receipt
            .validate()
            .expect("signed 64-bit boundary is durable");

        let mut oversized_receipt = receipt();
        oversized_receipt.minimum_trust_root_revision = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            oversized_receipt.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "minimum trust-root revision",
                ..
            })
        ));

        let key = signing_key(7);
        let mut boundary_key =
            trusted_key("key-a", &key, ISSUED_AT - 10, MAX_PROMOTION_DURABLE_INTEGER);
        boundary_key.revoked_at_unix_seconds = Some(MAX_PROMOTION_DURABLE_INTEGER);
        boundary_key
            .validate()
            .expect("key timestamps accept signed 64-bit boundary");
        boundary_key.valid_until_unix_seconds = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            boundary_key.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "trusted key valid-until",
                ..
            })
        ));

        let mut boundary_root = root(
            vec![trusted_key(
                "key-a",
                &key,
                ISSUED_AT - 10,
                MAX_PROMOTION_DURABLE_INTEGER,
            )],
            1,
        );
        boundary_root.revision = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_root.max_receipt_lifetime_seconds = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_root
            .validate()
            .expect("trust-root integers accept signed 64-bit boundary");

        let mut oversized_lifetime_root = boundary_root.clone();
        oversized_lifetime_root.max_receipt_lifetime_seconds = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            oversized_lifetime_root.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "maximum receipt lifetime",
                ..
            })
        ));

        let mut oversized_revision_root = boundary_root;
        oversized_revision_root.revision = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            oversized_revision_root.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "trust-root revision",
                ..
            })
        ));

        let mut oversized_effective_root = root(
            vec![trusted_key(
                "key-a",
                &key,
                ISSUED_AT - 10,
                MAX_PROMOTION_DURABLE_INTEGER,
            )],
            1,
        );
        oversized_effective_root.effective_at_unix_seconds = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            oversized_effective_root.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "trust-root effective-at",
                ..
            })
        ));

        let mut boundary_revocations = revocations();
        boundary_revocations.revision = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_revocations.effective_at_unix_seconds = MAX_PROMOTION_DURABLE_INTEGER;
        boundary_revocations
            .validate()
            .expect("revocation revision accepts signed 64-bit boundary");
        boundary_revocations.revision = MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            boundary_revocations.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "revocation revision",
                ..
            })
        ));

        let mut oversized_effective_revocations = revocations();
        oversized_effective_revocations.effective_at_unix_seconds =
            MAX_PROMOTION_DURABLE_INTEGER + 1;
        assert!(matches!(
            oversized_effective_revocations.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "revocation effective-at",
                ..
            })
        ));

        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        assert!(matches!(
            verify_and_consume_promotion_receipt_v1(
                &packet_one(&key),
                &bindings(),
                &trust_root,
                &revocations(),
                MAX_PROMOTION_DURABLE_INTEGER + 1,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::InvalidField {
                field: "verification Unix seconds",
                ..
            })
        ));
    }

    #[test]
    fn key_rotation_boundaries_are_half_open() {
        let old_key = signing_key(7);
        let old_receipt = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            1,
            1,
            bindings(),
            100,
            300,
            "66".repeat(32),
        );
        let old_packet = sign_promotion_receipt_v1(
            old_receipt,
            &[PromotionSignerV1 {
                key_id: "key-old",
                signing_key: &old_key,
            }],
        )
        .expect("old packet");
        let mut old_root = root(vec![trusted_key("key-old", &old_key, 100, 200)], 1);
        old_root.effective_at_unix_seconds = 90;
        let mut boundary_revocations = revocations();
        boundary_revocations.effective_at_unix_seconds = 90;
        verify_and_consume_promotion_receipt_v1(
            &old_packet,
            &bindings(),
            &old_root,
            &boundary_revocations,
            199,
            &InMemoryPromotionReceiptReplayStore::default(),
        )
        .expect("old key active before boundary");
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &old_packet,
                &bindings(),
                &old_root,
                &boundary_revocations,
                200,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::InactiveSigner("key-old".to_string()))
        );

        let new_key = signing_key(9);
        let new_receipt = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            1,
            1,
            bindings(),
            200,
            300,
            "77".repeat(32),
        );
        let new_packet = sign_promotion_receipt_v1(
            new_receipt,
            &[PromotionSignerV1 {
                key_id: "key-new",
                signing_key: &new_key,
            }],
        )
        .expect("new packet");
        let mut new_root = root(vec![trusted_key("key-new", &new_key, 200, 400)], 1);
        new_root.effective_at_unix_seconds = 200;
        verify_and_consume_promotion_receipt_v1(
            &new_packet,
            &bindings(),
            &new_root,
            &boundary_revocations,
            200,
            &InMemoryPromotionReceiptReplayStore::default(),
        )
        .expect("new key active at boundary");
    }

    #[test]
    fn scheduled_and_immediate_revocation_fail_closed() {
        let key = signing_key(7);
        let packet = packet_one(&key);
        let mut scheduled = trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10);
        scheduled.revoked_at_unix_seconds = Some(NOW + 1);
        let trust_root = root(vec![scheduled], 1);
        verify_and_consume_promotion_receipt_v1(
            &packet,
            &bindings(),
            &trust_root,
            &revocations(),
            NOW,
            &InMemoryPromotionReceiptReplayStore::default(),
        )
        .expect("key is active before revocation time");
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW + 1,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::RevokedSigner("key-a".to_string()))
        );

        let mut immediate = revocations();
        immediate.revoked_key_ids.push("key-a".to_string());
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &immediate,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::RevokedSigner("key-a".to_string()))
        );
    }

    #[test]
    fn receipt_and_nonce_revocation_fail_closed() {
        let key = signing_key(7);
        let packet = packet_one(&key);
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let mut receipt_revoked = revocations();
        receipt_revoked
            .revoked_receipt_sha256
            .push(packet.receipt.receipt_sha256().expect("receipt digest"));
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &receipt_revoked,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::ReceiptRevoked)
        );
        let mut nonce_revoked = revocations();
        nonce_revoked
            .revoked_nonces
            .push(packet.receipt.nonce.clone());
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &nonce_revoked,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::NonceRevoked)
        );
    }

    #[test]
    fn duplicate_and_unsorted_collections_are_rejected() {
        let key = signing_key(7);
        let mut duplicate_signatures = packet_one(&key);
        duplicate_signatures
            .signatures
            .push(duplicate_signatures.signatures[0].clone());
        assert_eq!(
            duplicate_signatures.validate_structure(),
            Err(PromotionReceiptError::DuplicateEntry(
                "promotion signatures"
            ))
        );

        let mut duplicate_platform = receipt();
        duplicate_platform
            .bindings
            .qualification
            .platform_receipts
            .push(duplicate_platform.bindings.qualification.platform_receipts[2].clone());
        assert_eq!(
            duplicate_platform.validate(),
            Err(PromotionReceiptError::DuplicateEntry(
                "qualification platform receipts"
            ))
        );

        let key_b = signing_key(9);
        let unsorted_root = root(
            vec![
                trusted_key("key-b", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            1,
        );
        assert_eq!(
            unsorted_root.validate(),
            Err(PromotionReceiptError::NonCanonicalOrder(
                "trusted promotion keys"
            ))
        );

        let duplicate_key_id = root(
            vec![
                trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-a", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            1,
        );
        assert_eq!(
            duplicate_key_id.validate(),
            Err(PromotionReceiptError::DuplicateEntry(
                "trusted promotion keys"
            ))
        );

        let duplicate_public_key = root(
            vec![
                trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-b", &key, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            1,
        );
        assert_eq!(
            duplicate_public_key.validate(),
            Err(PromotionReceiptError::DuplicateEntry(
                "trusted promotion public keys"
            ))
        );

        let mut duplicate_revocation = revocations();
        duplicate_revocation.revoked_nonces = vec!["88".repeat(32), "88".repeat(32)];
        assert_eq!(
            duplicate_revocation.validate(),
            Err(PromotionReceiptError::DuplicateEntry(
                "revoked receipt nonces"
            ))
        );
    }

    #[test]
    fn extra_unknown_signature_fails_closed() {
        let key = signing_key(7);
        let mut packet = packet_one(&key);
        packet.signatures.push(PromotionSignatureV1 {
            key_id: "key-z".to_string(),
            algorithm: PromotionSignatureAlgorithm::Ed25519,
            signature_hex: "00".repeat(64),
        });
        let trust_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::UnknownSigner("key-z".to_string()))
        );
    }

    #[test]
    fn malformed_unused_trusted_key_invalidates_entire_trust_root() {
        let key = signing_key(7);
        let mut malformed = trusted_key("key-b", &signing_key(9), ISSUED_AT - 10, EXPIRES_AT + 10);
        malformed.public_key_hex = (0u8..=u8::MAX)
            .find_map(|byte| {
                let candidate = [byte; 32];
                VerifyingKey::from_bytes(&candidate)
                    .is_err()
                    .then(|| encode_hex(&candidate))
            })
            .expect("fixed-byte search contains an invalid compressed Edwards point");
        let trust_root = root(
            vec![
                trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10),
                malformed,
            ],
            1,
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet_one(&key),
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::MalformedPublicKey(
                "key-b".to_string()
            ))
        );
    }

    #[test]
    fn weak_keys_and_never_reachable_root_thresholds_are_rejected() {
        let key_a = signing_key(7);
        let key_b = signing_key(9);
        let mut weak_key = trusted_key("key-a", &key_a, 100, 300);
        let mut identity_encoding = [0u8; 32];
        identity_encoding[0] = 1;
        weak_key.public_key_hex = encode_hex(&identity_encoding);
        let mut weak_root = root(vec![weak_key], 1);
        weak_root.effective_at_unix_seconds = 90;
        assert_eq!(
            weak_root.validate(),
            Err(PromotionReceiptError::WeakPublicKey("key-a".to_string()))
        );

        let mut unreachable = root(
            vec![
                trusted_key("key-a", &key_a, 100, 200),
                trusted_key("key-b", &key_b, 200, 300),
            ],
            2,
        );
        unreachable.effective_at_unix_seconds = 90;
        assert_eq!(
            unreachable.validate(),
            Err(PromotionReceiptError::UnreachableTrustRootThreshold)
        );

        let mut one_second_overlap = root(
            vec![
                trusted_key("key-a", &key_a, 100, 201),
                trusted_key("key-b", &key_b, 200, 300),
            ],
            2,
        );
        one_second_overlap.effective_at_unix_seconds = 90;
        one_second_overlap
            .validate()
            .expect("threshold is reachable at the half-open boundary second");
    }

    #[test]
    fn threshold_counts_only_unique_active_trusted_keys() {
        let key_a = signing_key(7);
        let key_b = signing_key(9);
        let packet = packet_one(&key_a);
        let trust_root = root(
            vec![
                trusted_key("key-a", &key_a, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-b", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            2,
        );
        assert_eq!(
            verify_and_consume_promotion_receipt_v1(
                &packet,
                &bindings(),
                &trust_root,
                &revocations(),
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::SignatureThresholdNotMet {
                required: 2,
                verified: 1,
            })
        );
    }

    #[test]
    fn replay_store_check_and_consume_is_atomic_under_race() {
        let key = signing_key(7);
        let packet = Arc::new(packet_one(&key));
        let trust_root = Arc::new(root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        ));
        let revocations = Arc::new(revocations());
        let expected = Arc::new(bindings());
        let store = Arc::new(InMemoryPromotionReceiptReplayStore::default());
        let barrier = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let packet = Arc::clone(&packet);
            let trust_root = Arc::clone(&trust_root);
            let revocations = Arc::clone(&revocations);
            let expected = Arc::clone(&expected);
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            handles.push(std::thread::spawn(move || {
                barrier.wait();
                verify_and_consume_promotion_receipt_v1(
                    &packet,
                    &expected,
                    &trust_root,
                    &revocations,
                    NOW,
                    store.as_ref(),
                )
            }));
        }
        barrier.wait();
        let results = handles
            .into_iter()
            .map(|handle| handle.join().expect("race thread"))
            .collect::<Vec<_>>();
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|result| matches!(result, Err(PromotionReceiptError::ReceiptReplay)))
                .count(),
            1
        );
    }

    #[test]
    fn key_specific_algorithm_binding_and_signature_vector_are_pinned() {
        let key = signing_key(7);
        let message = receipt()
            .canonical_signing_bytes_for_signer("key-a", PromotionSignatureAlgorithm::Ed25519)
            .expect("key-specific signing message");
        assert_eq!(
            Sha256Digest::for_bytes(&message).as_str(),
            "4e9b01329c9b60eadd733a7fe2da8f2c7131aefd20472bc00ed49f28c1cd03ea"
        );
        assert_eq!(message.len(), 1000);
        assert_eq!(&message[message.len() - 2..], &[2, 1]);
        assert_eq!(
            packet_one(&key).signatures[0].signature_hex,
            "270ed2449f561afcc0ca7173399b5aa827e0d1fe15dc5b962b94e8279e9458ffde9fe5f8f63f3c716de1b3a617dccdd269fe754ab99290bb82a1273e69f7c109"
        );

        let mut legacy_message = receipt().canonical_receipt_preimage().expect("preimage");
        legacy_message.extend_from_slice(PROMOTION_RECEIPT_V1_SIGNER_BINDING_DOMAIN);
        put_string(&mut legacy_message, 1, "key-a");
        let legacy_packet = SignedPromotionReceiptV1 {
            receipt: receipt(),
            signatures: vec![PromotionSignatureV1 {
                key_id: "key-a".to_string(),
                algorithm: PromotionSignatureAlgorithm::Ed25519,
                signature_hex: encode_hex(&key.sign(&legacy_message).to_bytes()),
            }],
        };
        let (genesis, history) = complete_history_v1(&key);
        let store = InMemoryPromotionReceiptReplayStore::default();
        let anchored = anchor_public(&genesis, &history, NOW, &store).expect("anchored trust");
        assert_eq!(
            verify_public(&legacy_packet, &bindings(), &anchored, &store),
            Err(PromotionReceiptError::SignatureVerificationFailed(
                "key-a".to_string()
            ))
        );
    }

    #[test]
    fn complete_genesis_history_mints_evidence_only_capability_on_fresh_store() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v2_rev2(&key, vec![], vec![], vec![]);
        let store = InMemoryPromotionReceiptReplayStore::default();
        let anchored = anchor_public(&genesis, &history, NOW, &store)
            .expect("full history can initialize a fresh durable state at current revisions");
        assert_eq!(anchored.trust_root().revision, 2);
        assert_eq!(anchored.revocations().revision, 2);
        let verified = verify_public(&packet_one(&key), &bindings(), &anchored, &store)
            .expect("receipt verifies only through anchored capability");
        assert_eq!(verified.trust_root_revision(), 2);
        assert_eq!(verified.revocation_revision(), 2);
    }

    #[test]
    fn trusted_config_ratchet_capability_exposes_read_only_durable_fields() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v1(&key);
        anchor_public(
            &genesis,
            &history,
            NOW,
            &GetterCheckingRatchetStore::default(),
        )
        .expect("durable store inspected verifier-minted ratchet capability");
    }

    #[test]
    fn fixed_genesis_and_complete_history_reject_bare_or_incomplete_successors() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v2(&key);
        assert_eq!(
            anchor_public(
                &digest(0xfa),
                &history,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("wrong fixed digest must fail"),
            PromotionReceiptError::GenesisTrustRootMismatch
        );

        let mut incomplete = history;
        incomplete.updates.remove(0);
        assert!(matches!(
            anchor_public(
                &genesis,
                &incomplete,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion trust history",
                ..
            })
        ));
    }

    #[test]
    fn test_only_checkpoint_loader_is_strict_and_fresh_bootstrap_is_exact() {
        assert!(
            !include_str!("lib.rs").contains("load_pinned_promotion_head_checkpoint_json_v1"),
            "crate root must not export a caller-self-attesting checkpoint loader"
        );
        let key = signing_key(7);
        let (_, history_v2) = complete_history_v2(&key);
        let genesis_sha256 = history_v2
            .genesis
            .canonical_sha256()
            .expect("genesis digest");
        let verified_v2 = verify_promotion_trust_history(&genesis_sha256, &history_v2, NOW)
            .expect("verified v2 history");
        let checkpoint_json = checkpoint_json_from_verified(&verified_v2);
        let expected_json_sha256 = Sha256Digest::for_bytes(&checkpoint_json);
        let checkpoint_v2 = load_test_pinned_promotion_head_checkpoint_json_v1(
            &expected_json_sha256,
            &checkpoint_json,
        )
        .expect("strict test-only checkpoint");
        assert_eq!(checkpoint_v2.source_json_sha256(), &expected_json_sha256);
        assert_eq!(
            checkpoint_v2.terminal_history_chain_sha256(),
            &verified_v2.history_chain_sha256
        );
        assert_eq!(checkpoint_v2.terminal_trust_root_revision(), 2);
        assert_eq!(checkpoint_v2.terminal_revocation_revision(), 1);

        assert_eq!(
            load_test_pinned_promotion_head_checkpoint_json_v1(&digest(0xfa), &checkpoint_json)
                .expect_err("checkpoint bytes cannot self-select their expected digest"),
            PromotionReceiptError::CheckpointJsonDigestMismatch
        );
        let mut unknown =
            serde_json::from_slice::<serde_json::Value>(&checkpoint_json).expect("checkpoint JSON");
        unknown
            .as_object_mut()
            .expect("checkpoint object")
            .insert("future_field".to_string(), json!(true));
        let unknown_json = serde_json::to_vec(&unknown).expect("unknown checkpoint JSON");
        assert!(matches!(
            load_test_pinned_promotion_head_checkpoint_json_v1(
                &Sha256Digest::for_bytes(&unknown_json),
                &unknown_json,
            ),
            Err(PromotionReceiptError::Json(_))
        ));

        let mut future_schema =
            serde_json::from_slice::<serde_json::Value>(&checkpoint_json).expect("checkpoint JSON");
        future_schema["schema_version"] = json!(PROMOTION_HEAD_CHECKPOINT_SCHEMA_VERSION + 1);
        let future_schema_json =
            serde_json::to_vec(&future_schema).expect("future checkpoint JSON");
        assert_eq!(
            load_test_pinned_promotion_head_checkpoint_json_v1(
                &Sha256Digest::for_bytes(&future_schema_json),
                &future_schema_json,
            )
            .expect_err("future checkpoint schema must fail closed"),
            PromotionReceiptError::UnsupportedSchemaVersion {
                component: "promotion head checkpoint",
                found: PROMOTION_HEAD_CHECKPOINT_SCHEMA_VERSION + 1,
            }
        );

        let oversized = vec![b' '; MAX_PROMOTION_HEAD_CHECKPOINT_JSON_BYTES + 1];
        assert!(matches!(
            load_test_pinned_promotion_head_checkpoint_json_v1(
                &Sha256Digest::for_bytes(&oversized),
                &oversized,
            ),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion head checkpoint JSON",
                ..
            })
        ));

        let (_, history_v1) = complete_history_v1(&key);
        let store = InMemoryPromotionReceiptReplayStore::default();
        assert_eq!(
            anchor_with_checkpoint(&checkpoint_v2, &history_v1, NOW, &store)
                .expect_err("a fresh store cannot bootstrap from an old legal prefix"),
            PromotionReceiptError::CheckpointHeadMismatch
        );
        let checkpoint_v1 = checkpoint_for_history(&history_v1, NOW).expect("v1 checkpoint");
        anchor_with_checkpoint(&checkpoint_v1, &history_v1, NOW, &store)
            .expect("failed checkpoint validation left fresh state uninitialized");
    }

    #[test]
    fn genesis_effective_epoch_is_digest_anchored_and_cannot_be_in_future() {
        let key = signing_key(7);
        let (fixed_genesis, history) = complete_history_v1(&key);
        let mut epoch_tamper = history;
        epoch_tamper.genesis.effective_at_unix_seconds += 1;
        assert_eq!(
            anchor_public(
                &fixed_genesis,
                &epoch_tamper,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("genesis effective epoch is part of the fixed digest"),
            PromotionReceiptError::GenesisTrustRootMismatch
        );

        let mut future_genesis = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        future_genesis.effective_at_unix_seconds = NOW + 1;
        let mut future_revocations = revocations();
        future_revocations.effective_at_unix_seconds = NOW + 1;
        let signed_future_revocations = sign_promotion_revocations_v1(
            &future_genesis,
            None,
            future_revocations,
            NOW + 1,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("future state remains structurally signable");
        let future_digest = future_genesis
            .canonical_sha256()
            .expect("future genesis digest");
        let future_history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: future_genesis,
            updates: vec![PromotionTrustUpdateV1::Revocations(
                signed_future_revocations,
            )],
        };
        assert_eq!(
            anchor_public(
                &future_digest,
                &future_history,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("a correctly anchored genesis still cannot take effect in the future"),
            PromotionReceiptError::GenesisTrustRootInFuture
        );
    }

    #[test]
    fn signed_update_time_must_equal_successor_effective_time() {
        let key = signing_key(7);
        let (_, history) = complete_history_v1(&key);
        let predecessor_revocations = match &history.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => panic!("fixture revocations"),
        };
        let mut successor_root = history.genesis.clone();
        successor_root.revision = 2;
        successor_root.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT + 1;
        assert_eq!(
            sign_promotion_trust_root_rotation_v1(
                &history.genesis,
                &predecessor_revocations,
                successor_root,
                ROOT_V2_EFFECTIVE_AT,
                &[PromotionSignerV1 {
                    key_id: "key-a",
                    signing_key: &key,
                }],
            ),
            Err(PromotionReceiptError::TrustRootEffectiveTimeMismatch)
        );

        let mut successor_revocations = predecessor_revocations.clone();
        successor_revocations.revision = 2;
        successor_revocations.effective_at_unix_seconds = REVOCATIONS_V2_EFFECTIVE_AT + 1;
        assert_eq!(
            sign_promotion_revocations_v1(
                &history.genesis,
                Some(&predecessor_revocations),
                successor_revocations,
                REVOCATIONS_V2_EFFECTIVE_AT,
                &[PromotionSignerV1 {
                    key_id: "key-a",
                    signing_key: &key,
                }],
            ),
            Err(PromotionReceiptError::RevocationEffectiveTimeMismatch)
        );
    }

    #[test]
    fn receipt_signed_by_new_key_cannot_predate_root_rotation() {
        let old_key = signing_key(7);
        let new_key = signing_key(9);
        let genesis = root(
            vec![trusted_key(
                "key-old",
                &old_key,
                ISSUED_AT - 10,
                EXPIRES_AT + 10,
            )],
            1,
        );
        let revocations_v1 = revocations();
        let signed_revocations_v1 = sign_promotion_revocations_v1(
            &genesis,
            None,
            revocations_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-old",
                signing_key: &old_key,
            }],
        )
        .expect("genesis revocations");
        let mut successor = root(
            vec![trusted_key(
                "key-new",
                &new_key,
                ISSUED_AT - 10,
                EXPIRES_AT + 10,
            )],
            1,
        );
        successor.revision = 2;
        successor.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        let rotation = sign_promotion_trust_root_rotation_v1(
            &genesis,
            &revocations_v1,
            successor,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-old",
                signing_key: &old_key,
            }],
        )
        .expect("root rotation");
        let fixed_genesis = genesis.canonical_sha256().expect("genesis digest");
        let history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis,
            updates: vec![
                PromotionTrustUpdateV1::Revocations(signed_revocations_v1),
                PromotionTrustUpdateV1::TrustRootRotation(rotation),
            ],
        };
        let store = InMemoryPromotionReceiptReplayStore::default();
        let anchored = anchor_public(&fixed_genesis, &history, NOW, &store)
            .expect("rotation is fully anchored");
        let nonce = "88".repeat(32);
        let backdated = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            2,
            1,
            bindings(),
            ROOT_V2_EFFECTIVE_AT - 1,
            EXPIRES_AT,
            nonce.clone(),
        );
        let backdated_packet = sign_promotion_receipt_v1(
            backdated,
            &[PromotionSignerV1 {
                key_id: "key-new",
                signing_key: &new_key,
            }],
        )
        .expect("cryptographically valid backdated packet");
        assert_eq!(
            verify_public(&backdated_packet, &bindings(), &anchored, &store),
            Err(PromotionReceiptError::ReceiptPredatesTrustRoot)
        );

        let current = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            2,
            1,
            bindings(),
            ISSUED_AT,
            EXPIRES_AT,
            nonce,
        );
        let current_packet = sign_promotion_receipt_v1(
            current,
            &[PromotionSignerV1 {
                key_id: "key-new",
                signing_key: &new_key,
            }],
        )
        .expect("current packet");
        verify_public(&current_packet, &bindings(), &anchored, &store)
            .expect("rejected backdating did not consume the nonce");
    }

    #[test]
    fn receipt_cannot_predate_current_revocation_snapshot() {
        let key = signing_key(7);
        let (fixed_genesis, history) = complete_history_v2_rev2(&key, vec![], vec![], vec![]);
        let store = InMemoryPromotionReceiptReplayStore::default();
        let anchored = anchor_public(&fixed_genesis, &history, NOW, &store)
            .expect("revision-two revocations are anchored");
        let nonce = "89".repeat(32);
        let backdated = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            2,
            2,
            bindings(),
            REVOCATIONS_V2_EFFECTIVE_AT - 1,
            EXPIRES_AT,
            nonce.clone(),
        );
        let backdated_packet = sign_promotion_receipt_v1(
            backdated,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("cryptographically valid backdated packet");
        assert_eq!(
            verify_public(&backdated_packet, &bindings(), &anchored, &store),
            Err(PromotionReceiptError::ReceiptPredatesRevocations)
        );

        let current = PromotionReceiptV1::evidence_only_template(
            "hepta-promotion-root-2026q3",
            2,
            2,
            bindings(),
            ISSUED_AT,
            EXPIRES_AT,
            nonce,
        );
        let current_packet = sign_promotion_receipt_v1(
            current,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("current packet");
        verify_public(&current_packet, &bindings(), &anchored, &store)
            .expect("rejected backdating did not consume the nonce");
    }

    #[test]
    fn root_rotation_binds_full_predecessor_state_and_rejects_forgery() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v2(&key);

        let mut forged_successor = history.clone();
        let PromotionTrustUpdateV1::TrustRootRotation(rotation) = &mut forged_successor.updates[1]
        else {
            panic!("fixture rotation")
        };
        rotation.successor.max_receipt_lifetime_seconds += 1;
        assert!(matches!(
            anchor_public(
                &genesis,
                &forged_successor,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            ),
            Err(PromotionReceiptError::SignatureVerificationFailed(key_id)) if key_id == "key-a"
        ));

        let mut wrong_revocation_state = history;
        let PromotionTrustUpdateV1::TrustRootRotation(rotation) =
            &mut wrong_revocation_state.updates[1]
        else {
            panic!("fixture rotation")
        };
        rotation.predecessor_revocations_sha256 = digest(0xfb);
        assert_eq!(
            anchor_public(
                &genesis,
                &wrong_revocation_state,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("rotation cannot select an older/different revocation state"),
            PromotionReceiptError::TrustRootRotationPredecessorMismatch
        );
    }

    #[test]
    fn trust_update_signatures_cannot_be_relabelled() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v2(&key);
        let mut relabelled_rotation = history.clone();
        let PromotionTrustUpdateV1::TrustRootRotation(rotation) =
            &mut relabelled_rotation.updates[1]
        else {
            panic!("fixture rotation")
        };
        rotation.signatures[0].key_id = "key-b".to_string();
        assert_eq!(
            anchor_public(
                &genesis,
                &relabelled_rotation,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("rotation signer id is part of the signed message"),
            PromotionReceiptError::UnknownSigner("key-b".to_string())
        );

        let mut relabelled_revocations = history;
        let PromotionTrustUpdateV1::Revocations(revocations) =
            &mut relabelled_revocations.updates[0]
        else {
            panic!("fixture revocations")
        };
        revocations.signatures[0].key_id = "key-b".to_string();
        assert_eq!(
            anchor_public(
                &genesis,
                &relabelled_revocations,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("revocation signer id is part of the signed message"),
            PromotionReceiptError::UnknownSigner("key-b".to_string())
        );
    }

    #[test]
    fn historical_rotation_survives_natural_signer_expiry_today() {
        let old_key = signing_key(7);
        let new_key = signing_key(9);
        let genesis_root = root(
            vec![trusted_key("key-old", &old_key, ISSUED_AT - 10, NOW - 10)],
            1,
        );
        let revocations_v1 = revocations();
        let signed_v1 = sign_promotion_revocations_v1(
            &genesis_root,
            None,
            revocations_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-old",
                signing_key: &old_key,
            }],
        )
        .expect("old key signed while active");
        let mut successor = root(
            vec![trusted_key(
                "key-new",
                &new_key,
                ISSUED_AT - 10,
                EXPIRES_AT + 10,
            )],
            1,
        );
        successor.revision = 2;
        successor.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        let rotation = sign_promotion_trust_root_rotation_v1(
            &genesis_root,
            &revocations_v1,
            successor,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-old",
                signing_key: &old_key,
            }],
        )
        .expect("old key rotated while active");
        let genesis_sha256 = genesis_root.canonical_sha256().expect("genesis digest");
        let history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: genesis_root,
            updates: vec![
                PromotionTrustUpdateV1::Revocations(signed_v1),
                PromotionTrustUpdateV1::TrustRootRotation(rotation),
            ],
        };
        let store = InMemoryPromotionReceiptReplayStore::default();
        let anchored = anchor_public(&genesis_sha256, &history, NOW, &store)
            .expect("history is checked at signature issuance, not today's natural expiry");
        let packet = sign_promotion_receipt_v1(
            receipt(),
            &[PromotionSignerV1 {
                key_id: "key-new",
                signing_key: &new_key,
            }],
        )
        .expect("current key packet");
        verify_public(&packet, &bindings(), &anchored, &store)
            .expect("current key verifies after historical rotation replay");
    }

    #[test]
    fn durable_ratchet_rejects_same_revision_signed_branches() {
        let key = signing_key(7);
        let (genesis, base_history) = complete_history_v1(&key);
        let predecessor_revocations = match &base_history.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => panic!("fixture revocations"),
        };

        let mut root_history_a = base_history.clone();
        let mut root_a = root_history_a.genesis.clone();
        root_a.revision = 2;
        root_a.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        root_a.max_receipt_lifetime_seconds += 1;
        root_history_a
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(
                sign_promotion_trust_root_rotation_v1(
                    &root_history_a.genesis,
                    &predecessor_revocations,
                    root_a,
                    ROOT_V2_EFFECTIVE_AT,
                    &[PromotionSignerV1 {
                        key_id: "key-a",
                        signing_key: &key,
                    }],
                )
                .expect("root branch a"),
            ));
        let mut root_history_b = base_history.clone();
        let mut root_b = root_history_b.genesis.clone();
        root_b.revision = 2;
        root_b.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        root_b.max_receipt_lifetime_seconds += 2;
        root_history_b
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(
                sign_promotion_trust_root_rotation_v1(
                    &root_history_b.genesis,
                    &predecessor_revocations,
                    root_b,
                    ROOT_V2_EFFECTIVE_AT,
                    &[PromotionSignerV1 {
                        key_id: "key-a",
                        signing_key: &key,
                    }],
                )
                .expect("root branch b"),
            ));
        let race_store = Arc::new(InMemoryPromotionReceiptReplayStore::default());
        let race_barrier = Arc::new(Barrier::new(3));
        let mut race_handles = Vec::new();
        for branch in [root_history_a.clone(), root_history_b.clone()] {
            let store = Arc::clone(&race_store);
            let barrier = Arc::clone(&race_barrier);
            let genesis = genesis.clone();
            race_handles.push(std::thread::spawn(move || {
                barrier.wait();
                anchor_public(&genesis, &branch, NOW, store.as_ref())
            }));
        }
        race_barrier.wait();
        let race_results = race_handles
            .into_iter()
            .map(|handle| handle.join().expect("checkpoint bootstrap thread"))
            .collect::<Vec<_>>();
        assert_eq!(
            race_results.iter().filter(|result| result.is_ok()).count(),
            1
        );
        assert_eq!(
            race_results
                .iter()
                .filter(|result| matches!(
                    result,
                    Err(PromotionReceiptError::TrustRootRevisionConflict { revision: 2 })
                ))
                .count(),
            1
        );

        let root_store = InMemoryPromotionReceiptReplayStore::default();
        anchor_public(&genesis, &root_history_a, NOW, &root_store).expect("root branch a wins");
        assert_eq!(
            anchor_public(&genesis, &root_history_b, NOW, &root_store)
                .expect_err("same root revision cannot change canonical content"),
            PromotionReceiptError::TrustRootRevisionConflict { revision: 2 }
        );

        let make_revocation_branch = |revoked_key_id: &str| {
            let mut branch = base_history.clone();
            let revocations_v2 = PromotionRevocationsV1 {
                schema_version: PROMOTION_REVOCATIONS_SCHEMA_VERSION,
                trust_root_id: branch.genesis.trust_root_id.clone(),
                revision: 2,
                effective_at_unix_seconds: REVOCATIONS_V2_EFFECTIVE_AT,
                revoked_key_ids: vec![revoked_key_id.to_string()],
                revoked_receipt_sha256: vec![],
                revoked_nonces: vec![],
            };
            let signed = sign_promotion_revocations_v1(
                &branch.genesis,
                Some(&predecessor_revocations),
                revocations_v2,
                REVOCATIONS_V2_EFFECTIVE_AT,
                &[PromotionSignerV1 {
                    key_id: "key-a",
                    signing_key: &key,
                }],
            )
            .expect("revocation branch");
            branch
                .updates
                .push(PromotionTrustUpdateV1::Revocations(signed));
            branch
        };
        let revocation_store = InMemoryPromotionReceiptReplayStore::default();
        anchor_public(
            &genesis,
            &make_revocation_branch("key-y"),
            NOW,
            &revocation_store,
        )
        .expect("revocation branch a wins");
        assert_eq!(
            anchor_public(
                &genesis,
                &make_revocation_branch("key-z"),
                NOW,
                &revocation_store,
            )
            .expect_err("same revocation revision cannot change canonical content"),
            PromotionReceiptError::RevocationRevisionConflict { revision: 2 }
        );
    }

    #[test]
    fn cumulative_chain_rejects_convergent_prefix_key_reintroduction() {
        let key_a = signing_key(7);
        let key_x = signing_key(9);
        let (genesis_sha256, base_history) = complete_history_v1(&key_a);
        let predecessor_revocations = match &base_history.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => panic!("fixture revocations"),
        };

        let mut root_v2_a = base_history.genesis.clone();
        root_v2_a.revision = 2;
        root_v2_a.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        root_v2_a.max_receipt_lifetime_seconds += 1;
        root_v2_a.keys.push(trusted_key(
            "key-x",
            &key_x,
            ISSUED_AT - 10,
            EXPIRES_AT + 10,
        ));
        let rotation_v2_a = sign_promotion_trust_root_rotation_v1(
            &base_history.genesis,
            &predecessor_revocations,
            root_v2_a.clone(),
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("branch a introduces key-x");
        let mut converged_root_v3 = base_history.genesis.clone();
        converged_root_v3.revision = 3;
        converged_root_v3.effective_at_unix_seconds = ROOT_V3_EFFECTIVE_AT;
        converged_root_v3.max_receipt_lifetime_seconds += 2;
        let rotation_v3_a = sign_promotion_trust_root_rotation_v1(
            &root_v2_a,
            &predecessor_revocations,
            converged_root_v3.clone(),
            ROOT_V3_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("branch a retires key-x");
        let mut history_a = base_history.clone();
        history_a
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation_v2_a));
        history_a
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation_v3_a));

        let mut root_v2_b = base_history.genesis.clone();
        root_v2_b.revision = 2;
        root_v2_b.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        root_v2_b.max_receipt_lifetime_seconds += 1;
        let rotation_v2_b = sign_promotion_trust_root_rotation_v1(
            &base_history.genesis,
            &predecessor_revocations,
            root_v2_b.clone(),
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("branch b never introduces key-x");
        let rotation_v3_b = sign_promotion_trust_root_rotation_v1(
            &root_v2_b,
            &predecessor_revocations,
            converged_root_v3.clone(),
            ROOT_V3_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("branch b converges to the same root-v3 bytes");
        let mut history_b_prefix = base_history;
        history_b_prefix
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation_v2_b));
        history_b_prefix
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation_v3_b));
        let verified_a =
            verify_promotion_trust_history(&genesis_sha256, &history_a, NOW).expect("branch a");
        let verified_b_prefix =
            verify_promotion_trust_history(&genesis_sha256, &history_b_prefix, NOW)
                .expect("branch b prefix");
        assert_eq!(
            verified_a.trust_root_sha256,
            verified_b_prefix.trust_root_sha256
        );
        assert_ne!(
            verified_a.history_chain_sha256,
            verified_b_prefix.history_chain_sha256
        );

        let mut root_v4_b = converged_root_v3.clone();
        root_v4_b.revision = 4;
        root_v4_b.effective_at_unix_seconds = ISSUED_AT + 1;
        root_v4_b.max_receipt_lifetime_seconds += 1;
        root_v4_b.keys.push(trusted_key(
            "key-x",
            &key_x,
            ISSUED_AT - 10,
            EXPIRES_AT + 10,
        ));
        let rotation_v4_b = sign_promotion_trust_root_rotation_v1(
            &converged_root_v3,
            &predecessor_revocations,
            root_v4_b,
            ISSUED_AT + 1,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("alternative prefix can sign a reintroduction envelope");
        let mut history_b = history_b_prefix;
        history_b
            .updates
            .push(PromotionTrustUpdateV1::TrustRootRotation(rotation_v4_b));

        let store = InMemoryPromotionReceiptReplayStore::default();
        anchor_public(&genesis_sha256, &history_a, NOW, &store)
            .expect("official converged prefix is durable");
        assert_eq!(
            anchor_public(&genesis_sha256, &history_b, NOW, &store)
                .expect_err("same state bytes cannot erase retirement history"),
            PromotionReceiptError::TrustRootHistoryMismatch
        );
    }

    #[test]
    fn revoked_predecessor_key_cannot_rotate_the_root() {
        let key = signing_key(7);
        let genesis_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        let revoked_v1 = PromotionRevocationsV1 {
            schema_version: PROMOTION_REVOCATIONS_SCHEMA_VERSION,
            trust_root_id: genesis_root.trust_root_id.clone(),
            revision: 1,
            effective_at_unix_seconds: REVOCATIONS_V1_EFFECTIVE_AT,
            revoked_key_ids: vec!["key-a".to_string()],
            revoked_receipt_sha256: vec![],
            revoked_nonces: vec![],
        };
        let signed_v1 = sign_promotion_revocations_v1(
            &genesis_root,
            None,
            revoked_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("a key may sign the update that first revokes it");
        let mut successor = genesis_root.clone();
        successor.revision = 2;
        successor.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        successor.max_receipt_lifetime_seconds += 1;
        let rotation = sign_promotion_trust_root_rotation_v1(
            &genesis_root,
            &revoked_v1,
            successor,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("cryptographic envelope can be created but history policy rejects it");
        let genesis_sha256 = genesis_root.canonical_sha256().expect("genesis digest");
        let history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: genesis_root,
            updates: vec![
                PromotionTrustUpdateV1::Revocations(signed_v1),
                PromotionTrustUpdateV1::TrustRootRotation(rotation),
            ],
        };
        assert_eq!(
            anchor_public(
                &genesis_sha256,
                &history,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("revoked predecessor signer must fail"),
            PromotionReceiptError::RevokedConfigurationSigner("key-a".to_string())
        );
    }

    #[test]
    fn every_trust_update_enforces_the_predecessor_threshold() {
        let key_a = signing_key(7);
        let key_b = signing_key(9);
        let genesis_root = root(
            vec![
                trusted_key("key-a", &key_a, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-b", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            2,
        );
        let revocations_v1 = revocations();
        let signed_by_one = sign_promotion_revocations_v1(
            &genesis_root,
            None,
            revocations_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("under-threshold envelope is structurally representable");
        let genesis_sha256 = genesis_root.canonical_sha256().expect("genesis digest");
        assert_eq!(
            anchor_public(
                &genesis_sha256,
                &PromotionTrustHistoryV1 {
                    schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
                    genesis: genesis_root.clone(),
                    updates: vec![PromotionTrustUpdateV1::Revocations(signed_by_one)],
                },
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("revocation update needs the full predecessor threshold"),
            PromotionReceiptError::SignatureThresholdNotMet {
                required: 2,
                verified: 1,
            }
        );

        let signed_by_two = sign_promotion_revocations_v1(
            &genesis_root,
            None,
            revocations_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[
                PromotionSignerV1 {
                    key_id: "key-a",
                    signing_key: &key_a,
                },
                PromotionSignerV1 {
                    key_id: "key-b",
                    signing_key: &key_b,
                },
            ],
        )
        .expect("threshold revocation update");
        let mut successor = genesis_root.clone();
        successor.revision = 2;
        successor.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        successor.max_receipt_lifetime_seconds += 1;
        let rotation_by_one = sign_promotion_trust_root_rotation_v1(
            &genesis_root,
            &revocations_v1,
            successor,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("under-threshold rotation envelope");
        assert_eq!(
            anchor_public(
                &genesis_sha256,
                &PromotionTrustHistoryV1 {
                    schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
                    genesis: genesis_root,
                    updates: vec![
                        PromotionTrustUpdateV1::Revocations(signed_by_two),
                        PromotionTrustUpdateV1::TrustRootRotation(rotation_by_one),
                    ],
                },
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("root rotation needs the full predecessor threshold"),
            PromotionReceiptError::SignatureThresholdNotMet {
                required: 2,
                verified: 1,
            }
        );
    }

    #[test]
    fn root_and_revocation_revisions_must_form_contiguous_chains() {
        let key = signing_key(7);
        let (genesis, history_v2) = complete_history_v2(&key);
        let mut root_gap = history_v2;
        let PromotionTrustUpdateV1::TrustRootRotation(rotation) = &mut root_gap.updates[1] else {
            panic!("fixture rotation")
        };
        rotation.successor.revision = 3;
        rotation.signatures = sign_statement(
            rotation
                .canonical_statement()
                .expect("gapped root statement"),
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        );
        assert_eq!(
            anchor_public(
                &genesis,
                &root_gap,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("root revision gap must fail"),
            PromotionReceiptError::TrustRootRotationRevisionGap
        );

        let (_, mut revocation_gap) = complete_history_v1(&key);
        let predecessor = match &revocation_gap.updates[0] {
            PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
            PromotionTrustUpdateV1::TrustRootRotation(_) => panic!("fixture revocations"),
        };
        let mut successor = predecessor.clone();
        successor.revision = 3;
        let signed = sign_unchecked_revocation_update(
            &revocation_gap.genesis,
            &predecessor,
            successor,
            REVOCATIONS_V2_EFFECTIVE_AT,
            &key,
        );
        revocation_gap
            .updates
            .push(PromotionTrustUpdateV1::Revocations(signed));
        assert_eq!(
            anchor_public(
                &genesis,
                &revocation_gap,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("revocation revision gap must fail"),
            PromotionReceiptError::RevocationRevisionGap {
                expected: 2,
                found: 3,
            }
        );
    }

    #[test]
    fn revocation_history_never_allows_any_tombstone_removal() {
        let key = signing_key(7);
        let tombstoned_receipt = digest(0xa1);
        let tombstoned_nonce = "a2".repeat(32);
        let (genesis, base_history) = complete_history_v2_rev2(
            &key,
            vec!["key-z".to_string()],
            vec![tombstoned_receipt],
            vec![tombstoned_nonce],
        );
        for removed_kind in 0..3 {
            let mut history = base_history.clone();
            let current_root = match &history.updates[1] {
                PromotionTrustUpdateV1::TrustRootRotation(rotation) => rotation.successor.clone(),
                PromotionTrustUpdateV1::Revocations(_) => panic!("fixture rotation"),
            };
            let predecessor = match history.updates.last().expect("revocation v2") {
                PromotionTrustUpdateV1::Revocations(signed) => signed.revocations.clone(),
                PromotionTrustUpdateV1::TrustRootRotation(_) => panic!("fixture revocations"),
            };
            let mut successor = predecessor.clone();
            successor.revision = 3;
            match removed_kind {
                0 => successor.revoked_key_ids.clear(),
                1 => successor.revoked_receipt_sha256.clear(),
                2 => successor.revoked_nonces.clear(),
                _ => unreachable!(),
            }
            let signed = sign_unchecked_revocation_update(
                &current_root,
                &predecessor,
                successor,
                REVOCATIONS_V3_EFFECTIVE_AT,
                &key,
            );
            history
                .updates
                .push(PromotionTrustUpdateV1::Revocations(signed));
            assert!(matches!(
                anchor_public(
                    &genesis,
                    &history,
                    NOW,
                    &InMemoryPromotionReceiptReplayStore::default(),
                ),
                Err(PromotionReceiptError::RevocationTombstoneRemoved(_))
            ));
        }
    }

    #[test]
    fn root_key_identity_retirement_and_revoked_at_are_irreversible() {
        let key_a = signing_key(7);
        let key_b = signing_key(9);
        let mut genesis_root = root(
            vec![
                trusted_key("key-a", &key_a, ISSUED_AT - 10, EXPIRES_AT + 10),
                trusted_key("key-b", &key_b, ISSUED_AT - 10, EXPIRES_AT + 10),
            ],
            1,
        );
        genesis_root.keys[1].revoked_at_unix_seconds = Some(NOW + 10);
        let revocations_v1 = revocations();
        let signed_v1 = sign_promotion_revocations_v1(
            &genesis_root,
            None,
            revocations_v1.clone(),
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("genesis revocations");
        let genesis_sha256 = genesis_root.canonical_sha256().expect("genesis digest");

        let mut clears_revoked_at = genesis_root.clone();
        clears_revoked_at.revision = 2;
        clears_revoked_at.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        clears_revoked_at.keys[1].revoked_at_unix_seconds = None;
        let clear_rotation = sign_promotion_trust_root_rotation_v1(
            &genesis_root,
            &revocations_v1,
            clears_revoked_at,
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("signed weakening is still rejected by history policy");
        let clear_history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: genesis_root.clone(),
            updates: vec![
                PromotionTrustUpdateV1::Revocations(signed_v1.clone()),
                PromotionTrustUpdateV1::TrustRootRotation(clear_rotation),
            ],
        };
        assert_eq!(
            anchor_public(
                &genesis_sha256,
                &clear_history,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("root revoked-at tombstone cannot be cleared"),
            PromotionReceiptError::TrustRootKeyRevocationRemoved("key-b".to_string())
        );

        let mut root_v2 = genesis_root.clone();
        root_v2.revision = 2;
        root_v2.effective_at_unix_seconds = ROOT_V2_EFFECTIVE_AT;
        root_v2.keys.remove(1);
        let rotation_v2 = sign_promotion_trust_root_rotation_v1(
            &genesis_root,
            &revocations_v1,
            root_v2.clone(),
            ROOT_V2_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("retire key-b");
        let mut root_v3 = genesis_root.clone();
        root_v3.revision = 3;
        root_v3.effective_at_unix_seconds = ROOT_V3_EFFECTIVE_AT;
        let rotation_v3 = sign_promotion_trust_root_rotation_v1(
            &root_v2,
            &revocations_v1,
            root_v3,
            ROOT_V3_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key_a,
            }],
        )
        .expect("signed reintroduction is rejected by history policy");
        let reintroduction_history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: genesis_root,
            updates: vec![
                PromotionTrustUpdateV1::Revocations(signed_v1),
                PromotionTrustUpdateV1::TrustRootRotation(rotation_v2),
                PromotionTrustUpdateV1::TrustRootRotation(rotation_v3),
            ],
        };
        assert_eq!(
            anchor_public(
                &genesis_sha256,
                &reintroduction_history,
                NOW,
                &InMemoryPromotionReceiptReplayStore::default(),
            )
            .expect_err("retired key id cannot return"),
            PromotionReceiptError::TrustRootKeyReintroduced("key-b".to_string())
        );
    }

    #[test]
    fn signature_threshold_cannot_exceed_packet_signature_limit() {
        let keys = (0..=MAX_PROMOTION_SIGNATURES)
            .map(|index| {
                trusted_key(
                    &format!("key-{index:02}"),
                    &signing_key((index + 1) as u8),
                    ISSUED_AT - 10,
                    EXPIRES_AT + 10,
                )
            })
            .collect();
        let oversized_threshold = root(keys, (MAX_PROMOTION_SIGNATURES + 1) as u16);
        assert!(matches!(
            oversized_threshold.validate(),
            Err(PromotionReceiptError::InvalidField {
                field: "signature threshold",
                ..
            })
        ));
    }

    #[test]
    fn trusted_config_ratchets_before_missing_or_invalid_packets() {
        let key = signing_key(7);
        let (genesis, high_history) = complete_history_v2(&key);
        let (_, low_history) = complete_history_v1(&key);

        for mode in 0..5 {
            let store = InMemoryPromotionReceiptReplayStore::default();
            let anchored = anchor_public(&genesis, &high_history, NOW, &store)
                .expect("trusted config ratchets before packet selection");
            if mode != 0 {
                let mut packet = packet_one(&key);
                let mut expected = bindings();
                match mode {
                    1 => packet.receipt.operator_accepted = true,
                    2 => {
                        packet.receipt.expires_at_unix_seconds = NOW;
                        packet = sign_promotion_receipt_v1(
                            packet.receipt,
                            &[PromotionSignerV1 {
                                key_id: "key-a",
                                signing_key: &key,
                            }],
                        )
                        .expect("expired but structurally valid packet");
                    }
                    3 => expected.product_shadow_soak.manifest_sha256 = digest(0xfc),
                    4 => packet.signatures[0].signature_hex = "00".repeat(64),
                    _ => unreachable!(),
                }
                assert!(verify_public(&packet, &expected, &anchored, &store).is_err());
            }
            assert_eq!(
                anchor_public(&genesis, &low_history, NOW, &store)
                    .expect_err("old config must not return after any packet path"),
                PromotionReceiptError::TrustRootRevisionRollback {
                    highest_seen: 2,
                    found: 1,
                }
            );
        }
    }

    #[test]
    fn production_json_flow_resamples_time_and_persists_second_sample_failures() {
        let key = signing_key(7);
        let (_, history) = complete_history_v1(&key);
        let checkpoint = checkpoint_for_history(&history, EXPIRES_AT).expect("checkpoint");
        let history_json = serde_json::to_vec(&history).expect("history JSON");
        let packet_json = serde_json::to_vec(&packet_one(&key)).expect("packet JSON");
        let store = InMemoryPromotionReceiptReplayStore::default();
        assert_eq!(
            futures::executor::block_on(verify_and_consume_promotion_receipt_json_at_times(
                &checkpoint,
                &history_json,
                &packet_json,
                &bindings(),
                EXPIRES_AT - 1,
                EXPIRES_AT,
                &store,
            )),
            Err(PromotionReceiptError::ReceiptExpired)
        );
        assert_eq!(
            anchor_with_checkpoint(&checkpoint, &history, EXPIRES_AT - 1, &store)
                .expect_err("second expiry sample was durably ratcheted before returning"),
            PromotionReceiptError::ClockRollback {
                highest_seen: EXPIRES_AT,
                found: EXPIRES_AT - 1,
            }
        );

        let mut scheduled_root = root(
            vec![trusted_key("key-a", &key, ISSUED_AT - 10, EXPIRES_AT + 10)],
            1,
        );
        scheduled_root.keys[0].revoked_at_unix_seconds = Some(NOW + 1);
        let scheduled_revocations = revocations();
        let signed_revocations = sign_promotion_revocations_v1(
            &scheduled_root,
            None,
            scheduled_revocations,
            REVOCATIONS_V1_EFFECTIVE_AT,
            &[PromotionSignerV1 {
                key_id: "key-a",
                signing_key: &key,
            }],
        )
        .expect("scheduled root revocations");
        let scheduled_history = PromotionTrustHistoryV1 {
            schema_version: PROMOTION_TRUST_HISTORY_SCHEMA_VERSION,
            genesis: scheduled_root,
            updates: vec![PromotionTrustUpdateV1::Revocations(signed_revocations)],
        };
        let scheduled_checkpoint =
            checkpoint_for_history(&scheduled_history, NOW + 1).expect("scheduled checkpoint");
        let scheduled_history_json =
            serde_json::to_vec(&scheduled_history).expect("scheduled history JSON");
        let scheduled_store = InMemoryPromotionReceiptReplayStore::default();
        assert_eq!(
            futures::executor::block_on(verify_and_consume_promotion_receipt_json_at_times(
                &scheduled_checkpoint,
                &scheduled_history_json,
                &packet_json,
                &bindings(),
                NOW,
                NOW + 1,
                &scheduled_store,
            )),
            Err(PromotionReceiptError::RevokedSigner("key-a".to_string()))
        );
        assert_eq!(
            anchor_with_checkpoint(
                &scheduled_checkpoint,
                &scheduled_history,
                NOW,
                &scheduled_store,
            )
            .expect_err("second scheduled-revocation sample was durably ratcheted"),
            PromotionReceiptError::ClockRollback {
                highest_seen: NOW + 1,
                found: NOW,
            }
        );
    }

    #[test]
    fn malformed_packet_json_cannot_suppress_checkpoint_and_time_ratchet() {
        let key = signing_key(7);
        let (_, history_v2) = complete_history_v2(&key);
        let checkpoint_v2 = checkpoint_for_history(&history_v2, NOW).expect("v2 checkpoint");
        let history_v2_json = serde_json::to_vec(&history_v2).expect("v2 history JSON");
        let store = InMemoryPromotionReceiptReplayStore::default();
        assert!(matches!(
            futures::executor::block_on(verify_and_consume_promotion_receipt_json_at_times(
                &checkpoint_v2,
                &history_v2_json,
                b"{",
                &bindings(),
                NOW,
                NOW,
                &store,
            )),
            Err(PromotionReceiptError::Json(_))
        ));
        let (_, history_v1) = complete_history_v1(&key);
        let checkpoint_v1 = checkpoint_for_history(&history_v1, NOW).expect("v1 checkpoint");
        assert_eq!(
            anchor_with_checkpoint(&checkpoint_v1, &history_v1, NOW, &store)
                .expect_err("malformed packet still left v2 trust durable"),
            PromotionReceiptError::TrustRootRevisionRollback {
                highest_seen: 2,
                found: 1,
            }
        );
    }

    #[test]
    fn persisted_observed_time_rejects_clock_rollback_and_stale_capabilities() {
        let key = signing_key(7);
        let (genesis, history) = complete_history_v1(&key);
        let store = InMemoryPromotionReceiptReplayStore::default();
        let stale = anchor_public(&genesis, &history, NOW, &store).expect("initial anchor");
        assert_eq!(
            anchor_public(&genesis, &history, NOW - 1, &store)
                .expect_err("clock rollback must fail before packet handling"),
            PromotionReceiptError::ClockRollback {
                highest_seen: NOW,
                found: NOW - 1,
            }
        );
        let _fresh = anchor_public(&genesis, &history, NOW + 1, &store)
            .expect("time watermark advances independently");
        assert_eq!(
            verify_public(&packet_one(&key), &bindings(), &stale, &store),
            Err(PromotionReceiptError::ClockRollback {
                highest_seen: NOW + 1,
                found: NOW,
            })
        );

        // There is intentionally no universal maximum forward step. A bad
        // trusted-clock jump therefore fails closed on every later lower time
        // until an authenticated, deployment-specific store recovery occurs.
        // Packet timestamps must never be used to undo the durable watermark.
        let forward_jump_store = InMemoryPromotionReceiptReplayStore::default();
        let checkpoint = checkpoint_for_history(&history, NOW).expect("checkpoint");
        anchor_with_checkpoint(&checkpoint, &history, NOW, &forward_jump_store)
            .expect("initial trusted time");
        let jumped_time = NOW + 1_000_000;
        anchor_with_checkpoint(&checkpoint, &history, jumped_time, &forward_jump_store)
            .expect("forward jumps are persisted without packet input");
        assert_eq!(
            anchor_with_checkpoint(&checkpoint, &history, NOW + 1, &forward_jump_store)
                .expect_err("lower trusted time requires external authenticated recovery"),
            PromotionReceiptError::ClockRollback {
                highest_seen: jumped_time,
                found: NOW + 1,
            }
        );
    }

    #[test]
    fn concurrent_higher_ratchet_makes_stale_consumption_fail_and_retry() {
        let key = signing_key(7);
        let (genesis, history_v1) = complete_history_v1(&key);
        let (_, history_v2) = complete_history_v2(&key);
        let store = Arc::new(PausingConsumeStore::new());
        let stale = anchor_public(&genesis, &history_v1, NOW, store.as_ref())
            .expect("revision-one capability");
        let packet = packet_one(&key);
        let expected = bindings();
        let thread_store = Arc::clone(&store);
        let handle = std::thread::spawn(move || {
            verify_public(&packet, &expected, &stale, thread_store.as_ref())
        });
        store.consume_entered.wait();
        let fresh = anchor_public(&genesis, &history_v2, NOW, store.as_ref())
            .expect("higher trusted revision wins atomically");
        store.consume_resume.wait();
        assert_eq!(
            handle.join().expect("consume thread"),
            Err(PromotionReceiptError::TrustedConfigurationNotCurrent)
        );
        verify_public(&packet_one(&key), &bindings(), &fresh, store.as_ref())
            .expect("caller can retry under the current anchored capability");
    }

    #[test]
    fn trust_history_json_and_collection_limits_fail_closed() {
        let key = signing_key(7);
        let (_, history) = complete_history_v1(&key);
        let mut future_history = history.clone();
        future_history.schema_version = 99;
        assert!(matches!(
            future_history.validate_structure(),
            Err(PromotionReceiptError::UnsupportedSchemaVersion {
                component: "promotion trust history",
                found: 99,
            })
        ));
        let mut value = serde_json::to_value(&history).expect("history JSON");
        value
            .as_object_mut()
            .expect("history object")
            .insert("future_field".to_string(), json!(true));
        assert!(matches!(
            PromotionTrustHistoryV1::from_json_slice(
                &serde_json::to_vec(&value).expect("history JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));

        let mut wrapper_value = serde_json::to_value(&history).expect("history JSON");
        wrapper_value["updates"][0]
            .as_object_mut()
            .expect("tagged update wrapper")
            .insert("future_wrapper_field".to_string(), json!(true));
        assert!(matches!(
            PromotionTrustHistoryV1::from_json_slice(
                &serde_json::to_vec(&wrapper_value).expect("wrapper JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));

        let mut inner_value = serde_json::to_value(&history).expect("history JSON");
        inner_value["updates"][0]["update"]
            .as_object_mut()
            .expect("revocation update envelope")
            .insert("future_inner_field".to_string(), json!(true));
        assert!(matches!(
            PromotionTrustHistoryV1::from_json_slice(
                &serde_json::to_vec(&inner_value).expect("inner JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));

        let (_, rotation_history) = complete_history_v2(&key);
        let mut rotation_inner = serde_json::to_value(&rotation_history).expect("rotation JSON");
        rotation_inner["updates"][1]["update"]
            .as_object_mut()
            .expect("rotation update envelope")
            .insert("future_rotation_field".to_string(), json!(true));
        assert!(matches!(
            PromotionTrustHistoryV1::from_json_slice(
                &serde_json::to_vec(&rotation_inner).expect("rotation JSON bytes")
            ),
            Err(PromotionReceiptError::Json(_))
        ));
        assert!(matches!(
            PromotionTrustHistoryV1::from_json_slice(&vec![
                b' ';
                MAX_PROMOTION_TRUST_CONFIG_JSON_BYTES
                    + 1
            ]),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion trust history JSON",
                ..
            })
        ));

        let mut too_many = history.clone();
        too_many.updates =
            vec![too_many.updates[0].clone(); MAX_PROMOTION_TRUST_HISTORY_UPDATES + 1];
        assert!(matches!(
            too_many.validate_structure(),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion trust history",
                ..
            })
        ));

        let rotation = rotation_history.updates[1].clone();
        let mut too_many_rotations = history;
        too_many_rotations
            .updates
            .extend(vec![rotation; MAX_PROMOTION_TRUST_ROOT_ROTATIONS + 1]);
        assert!(matches!(
            too_many_rotations.validate_structure(),
            Err(PromotionReceiptError::InvalidField {
                field: "promotion trust-root rotations",
                ..
            })
        ));
    }
}
