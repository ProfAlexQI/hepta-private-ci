#![forbid(unsafe_code)]

mod canonical;
mod channel_store;
mod governance_validation;
mod historical;
mod memory_mutation_store;
mod provider_store;
mod schema_validation;
mod store;
mod summary;

pub use channel_store::ChannelIngressClaimDisposition;
pub use channel_store::ChannelIngressState;
pub use channel_store::StoredChannelIngressEvent;
pub use channel_store::StoredChannelIngressEvidence;
pub use channel_store::StoredChannelIngressReceipt;
pub use historical::HISTORICAL_EVIDENCE_DIGEST_DOMAIN;
pub use historical::HISTORICAL_EVIDENCE_RECORD_DIGEST_DOMAIN;
pub use historical::HISTORICAL_EVIDENCE_SCHEMA_VERSION;
pub use historical::HistoricalEvidenceFamily;
pub use historical::HistoricalEvidenceRecord;
pub use historical::HistoricalEvidenceSelector;
pub use historical::HistoricalEvidenceState;
pub use historical::historical_record_sha256;
pub use memory_mutation_store::MEMORY_MUTATION_SHADOW_SCHEMA_VERSION;
pub use memory_mutation_store::MemoryMutationShadowObservation;
pub use memory_mutation_store::StoredMemoryMutationShadowObservation;
pub use provider_store::ProviderBindingState;
pub use provider_store::ProviderIntentClaimDisposition;
pub use provider_store::StoredProviderAttemptEvidence;
pub use provider_store::StoredProviderIntent;
pub use provider_store::StoredProviderReceipt;
pub use store::AppendDisposition;
pub use store::HeptaEvidenceStore;
pub use store::StoredActionEvidence;
pub use store::StoredReceipt;
pub use summary::ChannelIngressEvidenceSummary;
pub use summary::EvidenceSummary;
pub use summary::GovernanceEvidenceSummary;
pub use summary::ProviderEvidenceSummary;

#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    #[error("failed to serialize Hepta evidence: {0}")]
    Serialization(String),
    #[error("Hepta evidence backend is unavailable: {0}")]
    Unavailable(String),
    #[error("Hepta evidence identity conflict for {record_id}")]
    IdempotencyConflict { record_id: String },
    #[error("invalid Hepta evidence record: {0}")]
    InvalidRecord(String),
    #[error("Hepta evidence is corrupt: {0}")]
    Corrupt(String),
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "provider_tests.rs"]
mod provider_tests;

#[cfg(test)]
#[path = "memory_mutation_tests.rs"]
mod memory_mutation_tests;

#[cfg(test)]
#[path = "channel_tests.rs"]
mod channel_tests;

#[cfg(test)]
#[path = "historical_tests.rs"]
mod historical_tests;

#[cfg(test)]
#[path = "summary_tests.rs"]
mod summary_tests;
