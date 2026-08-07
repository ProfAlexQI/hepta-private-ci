#![forbid(unsafe_code)]

mod canonical;
mod governance_store;
mod governance_validation;
mod provider_claim;
mod provider_insert;
mod provider_record;
mod provider_store;
mod schema_validation;
mod store;

pub use provider_claim::ProviderBindingState;
pub use provider_claim::ProviderIntentClaimDisposition;
pub use provider_store::StoredProviderAttemptEvidence;
pub use provider_store::StoredProviderIntent;
pub use provider_store::StoredProviderReceipt;
pub use store::AppendDisposition;
pub use store::HeptaEvidenceStore;
pub use store::StoredActionEvidence;
pub use store::StoredReceipt;

#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    #[error("failed to serialize governance evidence: {0}")]
    Serialization(String),
    #[error("governance evidence backend is unavailable: {0}")]
    Unavailable(String),
    #[error("governance evidence identity conflict for {record_id}")]
    IdempotencyConflict { record_id: String },
    #[error("invalid governance evidence record: {0}")]
    InvalidRecord(String),
    #[error("governance evidence is corrupt: {0}")]
    Corrupt(String),
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "provider_tests.rs"]
mod provider_tests;

#[cfg(test)]
#[path = "provider_claim_tests.rs"]
mod provider_claim_tests;
