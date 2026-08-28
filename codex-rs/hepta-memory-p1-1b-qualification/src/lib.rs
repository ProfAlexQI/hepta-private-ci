#![forbid(unsafe_code)]

mod digest;
mod embedding;
mod index;
mod route;
mod tokenizer;

pub use digest::Digest32;
pub use embedding::EmbeddedVector;
pub use embedding::EmbeddingMetric;
pub use embedding::EmbeddingProviderKind;
pub use embedding::EmbeddingRegistry;
pub use embedding::LocalEmbeddingDescriptor;
pub use embedding::LocalEmbeddingProvider;
pub use embedding::QualificationHashOneHotProvider;
pub use embedding::VectorQuantization;
pub use index::AnnAlgorithm;
pub use index::AnnIndexBuildDraft;
pub use index::AnnIndexItemDraft;
pub use index::AnnIndexManifest;
pub use index::AnnSearchReceipt;
pub use index::AnnSearchResult;
pub use index::ExpectedAnnIndexBinding;
pub use index::IndexWriteReceipt;
pub use index::LocalAnnIndex;
pub use index::build_local_ann_index;
pub use index::reopen_local_ann_index;
pub use route::LocalSemanticReadiness;
pub use route::LocalSemanticRoute;
pub use route::LocalSemanticRouteReceipt;
pub use route::SemanticFallbackReason;
pub use route::decide_local_semantic_route;
pub use tokenizer::AlphanumericPunctuationTokenizer;
pub use tokenizer::LocalTokenizerDescriptor;
pub use tokenizer::LocalTokenizerEngine;
pub use tokenizer::LocalTokenizerRegistry;
pub use tokenizer::TokenCountMode;
pub use tokenizer::TokenCountReceipt;
pub use tokenizer::TokenizerContract;
pub use tokenizer::TokenizerImplementationKind;

pub const P1_1B_SCHEMA_VERSION: u32 = 1;
pub const P1_1B_NAMESPACE: &str = "hepta_intelligence_p1_1b_local_embedding_index_v1";

pub const P1_1B_IMPLEMENTED: bool = true;
pub const P1_1B_WIRED: bool = false;
pub const P1_1B_QUALIFIED: bool = false;
pub const P1_1B_PRODUCT_WORKSPACE_MEMBER: bool = false;
pub const P1_1B_PRODUCT_MODULE_REGISTERED: bool = false;
pub const P1_1B_DEFAULT_RECALL_CHANGED: bool = false;
pub const P1_1B_FEDERATION_RECALL_CHANGED: bool = false;
pub const P1_1B_CONTEXT_ATTACHMENT: bool = false;
pub const P1_1B_PHYSICAL_SEND: bool = false;
pub const P1_1B_REMOTE_EMBEDDING: bool = false;
pub const P1_1B_MODEL_DOWNLOAD: bool = false;
pub const P1_1B_NETWORK_ACCESS: bool = false;
pub const P1_1B_EXTERNAL_EFFECTS: bool = false;
pub const P1_1B_PRODUCTION_AUTHORITY: bool = false;
pub const P1_1B_OPERATOR_ACCEPTANCE: bool = false;
pub const P1_1B_PROMOTION: bool = false;
pub const P1_1B_CALLERS_RATCHET: bool = false;

pub const MAX_ID_BYTES: usize = 256;
pub const MAX_INPUT_BYTES: usize = 256 * 1024;

#[derive(Debug)]
pub enum ContractError {
    Invalid(String),
    Limit { label: &'static str, max: usize },
    Duplicate(String),
    Unavailable(&'static str),
    Corrupt(String),
    Overflow,
    Io(String),
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => write!(formatter, "invalid P1.1b contract: {message}"),
            Self::Limit { label, max } => {
                write!(formatter, "P1.1b {label} exceeds limit {max}")
            }
            Self::Duplicate(message) => write!(formatter, "duplicate P1.1b value: {message}"),
            Self::Unavailable(message) => {
                write!(formatter, "P1.1b component unavailable: {message}")
            }
            Self::Corrupt(message) => write!(formatter, "corrupt P1.1b evidence: {message}"),
            Self::Overflow => write!(formatter, "P1.1b arithmetic overflow"),
            Self::Io(message) => write!(formatter, "P1.1b I/O failure: {message}"),
        }
    }
}

impl std::error::Error for ContractError {}

impl From<std::io::Error> for ContractError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub(crate) fn validate_id(value: &str, label: &'static str) -> Result<(), ContractError> {
    if value.is_empty() || value.len() > MAX_ID_BYTES || value.as_bytes().contains(&0) {
        return Err(ContractError::Invalid(format!(
            "{label} must contain 1..={MAX_ID_BYTES} non-NUL bytes"
        )));
    }
    if value.chars().any(char::is_whitespace) {
        return Err(ContractError::Invalid(format!(
            "{label} must not contain whitespace"
        )));
    }
    Ok(())
}

pub(crate) fn usize_to_u32(value: usize, label: &'static str) -> Result<u32, ContractError> {
    u32::try_from(value).map_err(|_| ContractError::Invalid(format!("{label} exceeds u32")))
}

pub(crate) fn usize_to_u64(value: usize, label: &'static str) -> Result<u64, ContractError> {
    u64::try_from(value).map_err(|_| ContractError::Invalid(format!("{label} exceeds u64")))
}
