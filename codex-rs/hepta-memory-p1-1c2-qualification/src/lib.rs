#![forbid(unsafe_code)]

mod digest;
mod evaluation;
mod projection;

pub use digest::Digest32;
pub use evaluation::{
    evaluate_reviewed_corpus, EfficacyPolicy, EvaluationRequest, LaneDeltaReceipt,
    ReviewedCorpusEvaluationReceipt,
};
pub use projection::{
    candidate_projection_digest, ProjectionAudit, ProjectionEntry, ReviewProjection,
};

pub const P1_1C2_SCHEMA_VERSION: u32 = 1;
pub const P1_1C2_NAMESPACE: &str =
    "hepta_intelligence_p1_1c2_reviewed_corpus_offline_efficacy_v1";
pub const P1_1C2_SOURCE_BRANCH: &str = "codex/hepta-p1c2-eval-20260828";
pub const P1_1C1_SOURCE_BRANCH: &str =
    "codex/hepta-intelligence-reviewed-corpus-acceptance-p1c1-20260828";
pub const P1_1C1_SOURCE_COMMIT: &str =
    "f961a056ac0a35c1967a934de7cf5bf7ffb92a05";
pub const P1_1C_SOURCE_COMMIT: &str =
    "fe33565ce74c013e574c307e4fab101820c0ea88";

pub const P1_1C2_IMPLEMENTED: bool = true;
pub const P1_1C2_SOURCE_QUALIFIED: bool = false;
pub const P1_1C2_REVIEWED_CORPUS_PRESENT: bool = false;
pub const P1_1C2_REVIEWED_CORPUS_EVALUATED: bool = false;
pub const P1_1C2_EFFICACY_VALIDATION: bool = false;
pub const P1_1C2_EFFICACY_CLAIM: bool = false;
pub const P1_1C2_PRODUCT_WORKSPACE_MEMBER: bool = false;
pub const P1_1C2_PRODUCT_MODULE_REGISTERED: bool = false;
pub const P1_1C2_RUNTIME_WIRED: bool = false;
pub const P1_1C2_DEFAULT_RECALL_CHANGED: bool = false;
pub const P1_1C2_FEDERATION_RECALL_CHANGED: bool = false;
pub const P1_1C2_CONTEXT_ATTACHMENT: bool = false;
pub const P1_1C2_PHYSICAL_SEND: bool = false;
pub const P1_1C2_NETWORK_ACCESS: bool = false;
pub const P1_1C2_MODEL_DOWNLOAD: bool = false;
pub const P1_1C2_EXTERNAL_EFFECTS: bool = false;
pub const P1_1C2_PRODUCTION_AUTHORITY: bool = false;
pub const P1_1C2_OPERATOR_ACCEPTANCE: bool = false;
pub const P1_1C2_PROMOTION: bool = false;
pub const P1_1C2_CALLERS_RATCHET: bool = false;

pub const MAX_PROJECTION_ENTRIES: usize = 32_768;
pub const MAX_BLOCKED_REASONS: usize = 32;
pub const MAX_ID_BYTES: usize = 192;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    Invalid(String),
    Duplicate(String),
    Missing(String),
    Corrupt(String),
    Dependency(String),
    Overflow,
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(formatter, "invalid P1.1c.2 contract: {message}"),
            Self::Duplicate(message) => write!(formatter, "duplicate P1.1c.2 value: {message}"),
            Self::Missing(message) => write!(formatter, "missing P1.1c.2 value: {message}"),
            Self::Corrupt(message) => write!(formatter, "corrupt P1.1c.2 evidence: {message}"),
            Self::Dependency(message) => write!(formatter, "blocked P1.1c.2 dependency: {message}"),
            Self::Overflow => formatter.write_str("P1.1c.2 arithmetic overflow"),
        }
    }
}

impl std::error::Error for ContractError {}

impl From<hepta_memory_p1_1c_qualification::ContractError> for ContractError {
    fn from(error: hepta_memory_p1_1c_qualification::ContractError) -> Self {
        Self::Dependency(error.to_string())
    }
}

impl From<hepta_memory_p1_1c1_qualification::ContractError> for ContractError {
    fn from(error: hepta_memory_p1_1c1_qualification::ContractError) -> Self {
        Self::Dependency(error.to_string())
    }
}

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

pub(crate) fn validate_commit_oid(value: &str, label: &str) -> Result<(), ContractError> {
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

pub(crate) fn p1c_digest(value: hepta_memory_p1_1c_qualification::Digest32) -> Result<Digest32, ContractError> {
    Digest32::from_hex(&value.to_string())
}

pub(crate) fn p1c1_digest(value: hepta_memory_p1_1c1_qualification::Digest32) -> Result<Digest32, ContractError> {
    Digest32::from_hex(&value.to_string())
}
