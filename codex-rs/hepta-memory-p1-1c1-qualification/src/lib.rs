#![forbid(unsafe_code)]

mod acceptance;
mod agreement;
mod digest;
mod review;

pub use acceptance::evaluate_review_batch;
pub use acceptance::AcceptancePolicy;
pub use acceptance::AcceptanceReceipt;
pub use acceptance::DependencyState;
pub use acceptance::ResolvedItemReceipt;
pub use agreement::AgreementMetrics;
pub use digest::Digest32;
pub use review::AdjudicationRecord;
pub use review::CitationLabel;
pub use review::ContradictionLabel;
pub use review::CorpusProvenance;
pub use review::PrivacyDecision;
pub use review::ReviewBatch;
pub use review::ReviewHeader;
pub use review::ReviewRecord;
pub use review::ReviewTuple;

pub const P1_1C1_SCHEMA_VERSION: u32 = 1;
pub const P1_1C1_NAMESPACE: &str =
    "hepta_intelligence_p1_1c1_reviewed_corpus_acceptance_v1";

pub const P1_1C1_IMPLEMENTED: bool = true;
pub const P1_1C1_SOURCE_QUALIFIED: bool = false;
pub const P1_1C1_REVIEW_PIPELINE_VALIDATED: bool = true;
pub const P1_1C1_REVIEWED_CORPUS_ACCEPTED: bool = false;
pub const P1_1C1_CORPUS_REVIEWED: bool = false;
pub const P1_1C1_HUMAN_REVIEW_ATTESTED: bool = false;
pub const P1_1C1_EFFICACY_VALIDATION: bool = false;
pub const P1_1C1_EFFICACY_CLAIM: bool = false;
pub const P1_1C1_PRODUCT_WORKSPACE_MEMBER: bool = false;
pub const P1_1C1_PRODUCT_MODULE_REGISTERED: bool = false;
pub const P1_1C1_RUNTIME_WIRED: bool = false;
pub const P1_1C1_DEFAULT_RECALL_CHANGED: bool = false;
pub const P1_1C1_FEDERATION_RECALL_CHANGED: bool = false;
pub const P1_1C1_CONTEXT_ATTACHMENT: bool = false;
pub const P1_1C1_PHYSICAL_SEND: bool = false;
pub const P1_1C1_NETWORK_ACCESS: bool = false;
pub const P1_1C1_MODEL_DOWNLOAD: bool = false;
pub const P1_1C1_EXTERNAL_EFFECTS: bool = false;
pub const P1_1C1_PRODUCTION_AUTHORITY: bool = false;
pub const P1_1C1_OPERATOR_ACCEPTANCE: bool = false;
pub const P1_1C1_PROMOTION: bool = false;
pub const P1_1C1_CALLERS_RATCHET: bool = false;

pub const SCORE_SCALE_PPM: u32 = 1_000_000;
pub const MAX_ID_BYTES: usize = 192;
pub const MAX_LOCALES: usize = 64;
pub const MAX_REVIEW_ITEMS: usize = 4096;
pub const MAX_REVIEW_ROWS: usize = MAX_REVIEW_ITEMS * 2;
pub const MAX_ADJUDICATION_ROWS: usize = MAX_REVIEW_ITEMS;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    Invalid(String),
    Duplicate(String),
    Missing(String),
    Corrupt(String),
    Overflow,
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(formatter, "invalid P1.1c.1 contract: {message}"),
            Self::Duplicate(message) => write!(formatter, "duplicate P1.1c.1 value: {message}"),
            Self::Missing(message) => write!(formatter, "missing P1.1c.1 value: {message}"),
            Self::Corrupt(message) => write!(formatter, "corrupt P1.1c.1 evidence: {message}"),
            Self::Overflow => formatter.write_str("P1.1c.1 arithmetic overflow"),
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
    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'
    }) {
        return Err(ContractError::Invalid(
            "locale must contain only ASCII letters, digits, '-' or '_'".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_commit_oid(value: &str) -> Result<(), ContractError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractError::Invalid(
            "source P1.1c commit must be a 40-character lowercase Git object ID".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn framed_digest(domain: &[u8], parts: &[&[u8]]) -> Digest32 {
    let mut bytes = Vec::new();
    append_frame(&mut bytes, domain);
    for part in parts {
        append_frame(&mut bytes, part);
    }
    Digest32::for_bytes(&bytes)
}

fn append_frame(output: &mut Vec<u8>, value: &[u8]) {
    let length = u64::try_from(value.len()).unwrap_or(u64::MAX);
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
}

pub(crate) fn usize_to_u32(value: usize) -> Result<u32, ContractError> {
    u32::try_from(value).map_err(|_| ContractError::Overflow)
}
