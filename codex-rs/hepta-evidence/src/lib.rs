#![forbid(unsafe_code)]

mod canonical;
mod frozen_oracle_qualification;
mod governance_store;
mod governance_validation;
mod promotion_replay_store;
mod provider_claim;
mod provider_insert;
mod provider_record;
mod provider_store;
mod schema_validation;
mod store;

pub use frozen_oracle_qualification::FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION;
pub use frozen_oracle_qualification::FrozenOracleConformanceAppend;
pub use frozen_oracle_qualification::FrozenOracleConformanceRecord;
pub use frozen_oracle_qualification::FrozenOracleConformanceStatus;
pub use frozen_oracle_qualification::FrozenOracleQualificationPlan;
pub use frozen_oracle_qualification::FrozenOracleQualificationRegistration;
pub use frozen_oracle_qualification::FrozenOracleQualificationRunId;
pub use frozen_oracle_qualification::FrozenOracleQualificationSummary;
pub use frozen_oracle_qualification::FrozenOracleQualificationTerminalId;
pub use frozen_oracle_qualification::FrozenOracleQualificationTerminalRecord;
pub use frozen_oracle_qualification::StoredFrozenOracleConformanceObservation;
pub use frozen_oracle_qualification::StoredFrozenOracleQualificationTerminal;
pub use frozen_oracle_qualification::VerifiedFrozenOracleCorpus;
pub use frozen_oracle_qualification::pinned_frozen_oracle_corpus_bytes;
pub use promotion_replay_store::SqlitePromotionReceiptReplayStore;
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

#[cfg(test)]
#[path = "frozen_oracle_qualification_tests.rs"]
mod frozen_oracle_qualification_tests;
