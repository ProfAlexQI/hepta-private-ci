use crate::{ContractError, Digest32, framed_digest, validate_id, validate_locale};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::collections::BTreeSet;

pub const ED25519_ALGORITHM: &str = "ed25519";
pub const MAX_TRUST_KEYS: usize = 256;
pub const MAX_KEY_LOCALES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TrustRole {
    CiQualification,
    Reviewer,
    Adjudicator,
    LicenseApprover,
    ProvenanceApprover,
    PrivacyApprover,
    Operator,
}

impl TrustRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CiQualification => "ci_qualification",
            Self::Reviewer => "reviewer",
            Self::Adjudicator => "adjudicator",
            Self::LicenseApprover => "license_approver",
            Self::ProvenanceApprover => "provenance_approver",
            Self::PrivacyApprover => "privacy_approver",
            Self::Operator => "operator",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TrustDomain {
    QualificationFixture,
    ExternalAttested,
}

impl TrustDomain {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationFixture => "qualification_fixture",
            Self::ExternalAttested => "external_attested",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedKey {
    pub key_id: String,
    pub public_key: [u8; 32],
    pub role: TrustRole,
    pub affiliation_id: String,
    pub allowed_locales: Vec<String>,
    pub valid_from_unix_seconds: u64,
    pub valid_to_unix_seconds: u64,
    pub domain: TrustDomain,
    pub revoked: bool,
    pub key_sha256: Digest32,
}

impl TrustedKey {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.key_id, "trust key ID")?;
        validate_id(&self.affiliation_id, "trust affiliation ID")?;
        if self.valid_from_unix_seconds >= self.valid_to_unix_seconds {
            return Err(ContractError::Invalid(
                "trust key validity window is empty or inverted".to_string(),
            ));
        }
        if self.allowed_locales.len() > MAX_KEY_LOCALES {
            return Err(ContractError::Limit(
                "trust key locale allowlist exceeds its bound".to_string(),
            ));
        }
        let mut previous: Option<&str> = None;
        for locale in &self.allowed_locales {
            validate_locale(locale)?;
            if previous.is_some_and(|value| value >= locale.as_str()) {
                return Err(ContractError::Invalid(
                    "trust key locale allowlist must be strictly sorted and unique".to_string(),
                ));
            }
            previous = Some(locale);
        }
        if self.key_sha256 != trusted_key_digest(self) {
            return Err(ContractError::Corrupt(
                "trusted key digest mismatch".to_string(),
            ));
        }
        VerifyingKey::from_bytes(&self.public_key).map_err(|_| {
            ContractError::Invalid("trusted Ed25519 public key is invalid".to_string())
        })?;
        Ok(())
    }

    #[must_use]
    pub fn commitment_sha256(&self) -> Digest32 {
        Digest32::for_bytes(&self.public_key)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustStore {
    pub store_id: String,
    pub version: u32,
    pub keys: Vec<TrustedKey>,
    pub store_sha256: Digest32,
}

impl TrustStore {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.store_id, "trust store ID")?;
        if self.version == 0 {
            return Err(ContractError::Invalid(
                "trust store version must be positive".to_string(),
            ));
        }
        if self.keys.is_empty() || self.keys.len() > MAX_TRUST_KEYS {
            return Err(ContractError::Limit(format!(
                "trust store must contain 1..={MAX_TRUST_KEYS} keys"
            )));
        }
        let mut ids = BTreeSet::new();
        let mut commitments = BTreeSet::new();
        let mut previous: Option<&str> = None;
        for key in &self.keys {
            key.validate()?;
            if previous.is_some_and(|value| value >= key.key_id.as_str()) {
                return Err(ContractError::Invalid(
                    "trust store keys must be strictly sorted by key ID".to_string(),
                ));
            }
            previous = Some(&key.key_id);
            if !ids.insert(key.key_id.as_str()) {
                return Err(ContractError::Duplicate(format!(
                    "trust key {}",
                    key.key_id
                )));
            }
            if !commitments.insert(key.commitment_sha256()) {
                return Err(ContractError::Duplicate(
                    "trust store contains the same public key under multiple identities"
                        .to_string(),
                ));
            }
        }
        if self.store_sha256 != trust_store_digest(self) {
            return Err(ContractError::Corrupt(
                "trust store digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    pub fn find_key(&self, key_id: &str) -> Result<&TrustedKey, ContractError> {
        self.keys
            .binary_search_by(|key| key.key_id.as_str().cmp(key_id))
            .ok()
            .map(|index| &self.keys[index])
            .ok_or_else(|| ContractError::Missing(format!("trusted key {key_id}")))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedDigest {
    pub key_id: String,
    pub algorithm: String,
    pub payload_sha256: Digest32,
    pub signature: [u8; 64],
    pub envelope_sha256: Digest32,
}

impl SignedDigest {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.key_id, "signed envelope key ID")?;
        if self.algorithm != ED25519_ALGORITHM {
            return Err(ContractError::Invalid(
                "signed envelope algorithm must be ed25519".to_string(),
            ));
        }
        if self.envelope_sha256 != signed_digest_envelope_digest(self) {
            return Err(ContractError::Corrupt(
                "signed envelope digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSignatureReceipt {
    pub key_id: String,
    pub role: TrustRole,
    pub affiliation_id: String,
    pub domain: TrustDomain,
    pub payload_sha256: Digest32,
    pub trust_store_sha256: Digest32,
    pub verified_at_unix_seconds: u64,
    pub receipt_sha256: Digest32,
    verified: bool,
}

impl VerifiedSignatureReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if !self.verified {
            return Err(ContractError::Corrupt(
                "signature receipt is not verified".to_string(),
            ));
        }
        if self.receipt_sha256 != verified_signature_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "verified signature receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verified
    }
}

pub fn verify_signed_digest(
    store: &TrustStore,
    signed: &SignedDigest,
    expected_role: TrustRole,
    locale: Option<&str>,
    now_unix_seconds: u64,
    require_external_domain: bool,
) -> Result<VerifiedSignatureReceipt, ContractError> {
    store.validate()?;
    signed.validate()?;
    let trusted = store.find_key(&signed.key_id)?;
    if trusted.revoked {
        return Err(ContractError::Invalid(
            "signature key is revoked".to_string(),
        ));
    }
    if trusted.role != expected_role {
        return Err(ContractError::Invalid(format!(
            "signature key role {} does not satisfy expected role {}",
            trusted.role.as_str(),
            expected_role.as_str()
        )));
    }
    if now_unix_seconds < trusted.valid_from_unix_seconds
        || now_unix_seconds >= trusted.valid_to_unix_seconds
    {
        return Err(ContractError::Invalid(
            "signature key is outside its validity window".to_string(),
        ));
    }
    if require_external_domain && trusted.domain != TrustDomain::ExternalAttested {
        return Err(ContractError::Invalid(
            "qualification fixture key cannot satisfy an external trust policy"
                .to_string(),
        ));
    }
    if let Some(locale) = locale {
        validate_locale(locale)?;
        if !trusted.allowed_locales.is_empty()
            && trusted
                .allowed_locales
                .binary_search_by(|value| value.as_str().cmp(locale))
                .is_err()
        {
            return Err(ContractError::Invalid(format!(
                "trusted key {} is not authorized for locale {locale}",
                trusted.key_id
            )));
        }
    }
    let verifying_key = VerifyingKey::from_bytes(&trusted.public_key).map_err(|_| {
        ContractError::Invalid("trusted Ed25519 public key is invalid".to_string())
    })?;
    let signature = Signature::try_from(signed.signature.as_slice()).map_err(|_| {
        ContractError::Invalid("Ed25519 signature bytes are invalid".to_string())
    })?;
    verifying_key
        .verify(signed.payload_sha256.as_bytes(), &signature)
        .map_err(|_| ContractError::Invalid("Ed25519 signature verification failed".to_string()))?;

    let mut receipt = VerifiedSignatureReceipt {
        key_id: trusted.key_id.clone(),
        role: trusted.role,
        affiliation_id: trusted.affiliation_id.clone(),
        domain: trusted.domain,
        payload_sha256: signed.payload_sha256,
        trust_store_sha256: store.store_sha256,
        verified_at_unix_seconds: now_unix_seconds,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        verified: true,
    };
    receipt.receipt_sha256 = verified_signature_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

#[must_use]
pub fn trusted_key_digest(key: &TrustedKey) -> Digest32 {
    let mut locales = Vec::new();
    for locale in &key.allowed_locales {
        locales.extend_from_slice(&u64::try_from(locale.len()).unwrap_or(u64::MAX).to_be_bytes());
        locales.extend_from_slice(locale.as_bytes());
    }
    framed_digest(
        b"hepta:intelligence:p1.1c3:trusted-key:v1",
        &[
            key.key_id.as_bytes(),
            &key.public_key,
            key.role.as_str().as_bytes(),
            key.affiliation_id.as_bytes(),
            &locales,
            &key.valid_from_unix_seconds.to_be_bytes(),
            &key.valid_to_unix_seconds.to_be_bytes(),
            key.domain.as_str().as_bytes(),
            &[u8::from(key.revoked)],
        ],
    )
}

#[must_use]
pub fn trust_store_digest(store: &TrustStore) -> Digest32 {
    let mut keys = Vec::new();
    for key in &store.keys {
        keys.extend_from_slice(key.key_sha256.as_bytes());
    }
    framed_digest(
        b"hepta:intelligence:p1.1c3:trust-store:v1",
        &[
            store.store_id.as_bytes(),
            &store.version.to_be_bytes(),
            &keys,
        ],
    )
}

#[must_use]
pub fn signed_digest_envelope_digest(signed: &SignedDigest) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:signed-digest:v1",
        &[
            signed.key_id.as_bytes(),
            signed.algorithm.as_bytes(),
            signed.payload_sha256.as_bytes(),
            &signed.signature,
        ],
    )
}

fn verified_signature_receipt_digest(receipt: &VerifiedSignatureReceipt) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1c3:verified-signature:v1",
        &[
            receipt.key_id.as_bytes(),
            receipt.role.as_str().as_bytes(),
            receipt.affiliation_id.as_bytes(),
            receipt.domain.as_str().as_bytes(),
            receipt.payload_sha256.as_bytes(),
            receipt.trust_store_sha256.as_bytes(),
            &receipt.verified_at_unix_seconds.to_be_bytes(),
            &[u8::from(receipt.verified)],
        ],
    )
}
