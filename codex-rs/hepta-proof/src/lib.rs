#![forbid(unsafe_code)]

//! Bounded local command observations for Hepta review evidence.
//!
//! This crate is caller-zero and non-hermetic. Its filesystem store blocks
//! replay only while one local root remains intact; it is not an authority
//! root, an exactly-once executor, or an anti-rollback ledger.
//!
//! On Unix, cleanup is best-effort for descendants that remain in the spawned
//! process group; this is not a sandbox and a process may escape that group.
//! Program and working-directory checks are path based and retain a check/exec
//! race. The store persists only the command binding and stream digests, not an
//! independently recoverable command manifest or raw stdout/stderr. Receipt
//! digests are unkeyed self-consistency checks, not external tamper anchors.
//!
//! Historical context attachment rereads one caller-selected record from the
//! current local Evidence store immediately before execution. It records an
//! exact association only; it does not prove that the candidate caused or
//! inherited that evidence, and it does not mint exclusive provenance.

mod command;
mod file_hash;
mod provenance;
mod runner;
mod store;
mod validation;

pub use command::MAX_PROOF_ARGUMENT_BYTES;
pub use command::MAX_PROOF_ARGUMENTS;
pub use command::MAX_PROOF_CAPTURE_BYTES;
pub use command::MAX_PROOF_ENVIRONMENT_ENTRIES;
pub use command::MAX_PROOF_ENVIRONMENT_VALUE_BYTES;
pub use command::MAX_PROOF_PATH_BYTES;
pub use command::MAX_PROOF_TIMEOUT_MS;
pub use command::PROOF_SCHEMA_VERSION;
pub use command::ProofCommandSpec;
pub use command::ProofExecutionResult;
pub use command::ProofHarness;
pub use command::ProofInvocation;
pub use command::ProofInvocationId;
pub use command::ProofReceipt;
pub use command::ProofReceiptId;
pub use command::ProofStreamEvidence;
pub use command::ProofStreamKind;
pub use command::ProofSubject;
pub use command::ProofTerminal;
pub use file_hash::sha256_regular_file;
pub use provenance::PROOF_PROVENANCE_SCHEMA_VERSION;
pub use provenance::ProofProvenanceContext;
pub use provenance::ProvenanceEvidenceKind;
pub use provenance::run_historical_observation;
pub use store::ProofAppendDisposition;
pub use store::ProofStore;

#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    #[error("invalid proof input: {0}")]
    InvalidInput(String),
    #[error("proof store is unavailable: {0}")]
    StoreUnavailable(String),
    #[error("proof evidence conflict for {record_id}")]
    EvidenceConflict { record_id: String },
    #[error("proof invocation replay is blocked: {invocation_id}")]
    ReplayBlocked { invocation_id: String },
    #[error("proof evidence is corrupt: {0}")]
    Corrupt(String),
}
