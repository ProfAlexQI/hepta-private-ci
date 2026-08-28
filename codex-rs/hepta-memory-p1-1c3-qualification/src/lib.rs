#![forbid(unsafe_code)]

mod digest;
mod intake;
mod qualification;
mod review_trust;
mod trust;

pub use digest::Digest32;
pub use intake::{
    CorpusIntakeRequest, IntakePolicy, LicenseEvidence, OperatorApprovalEvidence,
    PrivacyEvidence, ProvenanceEvidence, TrustedCorpusIntakeReceipt,
    evaluate_corpus_intake, intake_policy_digest, intake_subject_digest,
};
pub use qualification::{
    QualificationEvidence, QualificationGateSet, QualificationPolicy,
    VerifiedQualificationReceipt, qualification_policy_digest, verify_qualification,
};
pub use review_trust::{
    AdjudicationAttestation, ReviewAttestation, ReviewTrustBundle, ReviewTrustPolicy,
    ReviewTrustReceipt, adjudication_batch_digest, adjudication_record_digest,
    review_batch_digest, review_record_digest, review_trust_policy_digest,
    verify_review_trust,
};
pub use trust::{
    ED25519_ALGORITHM, SignedDigest, TrustDomain, TrustRole, TrustStore, TrustedKey,
    VerifiedSignatureReceipt, signed_digest_envelope_digest, trust_store_digest,
    trusted_key_digest, verify_signed_digest,
};

pub const P1_1C3_SCHEMA_VERSION: u32 = 1;
pub const P1_1C3_IMPLEMENTED: bool = true;
pub const P1_1C3_SOURCE_QUALIFIED: bool = false;
pub const P1_1C3_REAL_EXTERNAL_EVIDENCE_PRESENT: bool = false;
pub const P1_1C3_TRUSTED_CORPUS_ACCEPTED: bool = false;
pub const P1_1C3_EFFICACY_VALIDATION: bool = false;
pub const P1_1C3_EFFICACY_CLAIM: bool = false;
pub const P1_1C3_PRODUCT_WORKSPACE_MEMBER: bool = false;
pub const P1_1C3_PRODUCT_MODULE_REGISTERED: bool = false;
pub const P1_1C3_RUNTIME_WIRED: bool = false;
pub const P1_1C3_DEFAULT_RECALL_CHANGED: bool = false;
pub const P1_1C3_FEDERATION_RECALL_CHANGED: bool = false;
pub const P1_1C3_CONTEXT_ATTACHMENT: bool = false;
pub const P1_1C3_PHYSICAL_SEND: bool = false;
pub const P1_1C3_NETWORK_ACCESS: bool = false;
pub const P1_1C3_MODEL_DOWNLOAD: bool = false;
pub const P1_1C3_EXTERNAL_EFFECTS: bool = false;
pub const P1_1C3_PRODUCTION_AUTHORITY: bool = false;
pub const P1_1C3_OPERATOR_ACCEPTANCE: bool = false;
pub const P1_1C3_PROMOTION: bool = false;
pub const P1_1C3_CALLERS_RATCHET: bool = false;

pub const MAX_ID_BYTES: usize = 256;
pub const MAX_LOCALES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    Invalid(String),
    Duplicate(String),
    Missing(String),
    Corrupt(String),
    Limit(String),
    Overflow,
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(formatter, "invalid P1.1c.3 contract: {message}"),
            Self::Duplicate(message) => write!(formatter, "duplicate P1.1c.3 value: {message}"),
            Self::Missing(message) => write!(formatter, "missing P1.1c.3 value: {message}"),
            Self::Corrupt(message) => write!(formatter, "corrupt P1.1c.3 evidence: {message}"),
            Self::Limit(message) => write!(formatter, "P1.1c.3 limit exceeded: {message}"),
            Self::Overflow => formatter.write_str("P1.1c.3 arithmetic overflow"),
        }
    }
}

impl std::error::Error for ContractError {}

pub(crate) fn validate_id(value: &str, label: &str) -> Result<(), ContractError> {
    if value.is_empty()
        || value.len() > MAX_ID_BYTES
        || value.as_bytes().contains(&0)
        || value.chars().any(char::is_whitespace)
    {
        return Err(ContractError::Invalid(format!(
            "{label} must contain 1..={MAX_ID_BYTES} non-whitespace, non-NUL bytes"
        )));
    }
    Ok(())
}

pub(crate) fn validate_locale(value: &str) -> Result<(), ContractError> {
    validate_id(value, "locale")?;
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err(ContractError::Invalid(
            "locale must contain only ASCII letters, digits, '-' or '_'".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_git_oid(value: &str, label: &str) -> Result<(), ContractError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractError::Invalid(format!(
            "{label} must be a 40-character lowercase Git object ID"
        )));
    }
    Ok(())
}
