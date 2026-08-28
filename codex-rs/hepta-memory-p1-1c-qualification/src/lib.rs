#![forbid(unsafe_code)]

mod corpus;
mod digest;
mod evaluation;
mod kg;
mod metrics;
mod reranker;

pub use corpus::{
    CandidateFixture, CorpusCase, CorpusHeader, CorpusProvenance, OfflineCorpus,
    MAX_CANDIDATES_PER_CASE, MAX_CASES, MAX_ID_BYTES, MAX_LOCALES, MAX_QUERY_BYTES,
};
pub use digest::Digest32;
pub use evaluation::{evaluate_corpus, EvaluationReceipt, LaneReceipt};
pub use kg::{KgEdge, KgEvidence, KgGraph, MAX_GRAPH_EDGES, MAX_GRAPH_NODES, MAX_KG_HOPS};
pub use metrics::{CaseMetrics, LaneMetrics};
pub use reranker::{
    AblationLane, CalibrationContract, CandidateFeatures, RankedCandidate, PPM_DENOMINATOR,
};

use std::fmt::{Display, Formatter};

pub const P1_1C_SCHEMA: &str = "hepta.intelligence.p1_1c.offline_efficacy.v1";
pub const QUALIFICATION_NAMESPACE: &str = "hepta-memory-p1-1c-qualification";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    Invalid(String),
    Corrupt(String),
    Limit(String),
    Overflow,
}

impl Display for ContractError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(formatter, "invalid contract: {message}"),
            Self::Corrupt(message) => write!(formatter, "corrupt input: {message}"),
            Self::Limit(message) => write!(formatter, "bounded limit exceeded: {message}"),
            Self::Overflow => formatter.write_str("checked arithmetic overflow"),
        }
    }
}

impl std::error::Error for ContractError {}

pub(crate) fn validate_id(value: &str, field: &str) -> Result<(), ContractError> {
    if value.is_empty() || value.len() > MAX_ID_BYTES {
        return Err(ContractError::Invalid(format!(
            "{field} must contain 1..={MAX_ID_BYTES} UTF-8 bytes"
        )));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(ContractError::Invalid(format!(
            "{field} contains characters outside the stable identifier alphabet"
        )));
    }
    Ok(())
}

pub(crate) fn checked_ppm(value: u32, field: &str) -> Result<u32, ContractError> {
    if value > PPM_DENOMINATOR {
        return Err(ContractError::Invalid(format!(
            "{field} exceeds {PPM_DENOMINATOR} ppm"
        )));
    }
    Ok(value)
}
