//! In-memory and durable storage for terminal outcome receipts.
//!
//! The receipt, opaque canonical evidence envelope, and producer-owned hashes
//! are untrusted caller inputs: neither implementation mints or authenticates
//! them. Both provide atomic deduplication and conflict detection. The durable
//! implementation additionally verifies a storage-owned hash of its canonical
//! row before rehydrating contracts from SQLite WAL.

mod durable;
mod effect_ack;
mod error;
mod execution_intent;
mod intent;
mod sync_writer;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::ReceiptId;

pub use durable::DurableOutcomeStore;
pub use effect_ack::ExecutionEffectAck;
pub use effect_ack::ExecutionEffectAckError;
pub use effect_ack::ExecutionEffectAckParts;
pub use effect_ack::ExecutionEffectAckRecordResult;
pub use error::OutcomeStoreError;
pub use execution_intent::ExecutionIntent;
pub use execution_intent::ExecutionIntentError;
pub use execution_intent::ExecutionIntentParts;
pub use execution_intent::ExecutionIntentResolveResult;
pub use execution_intent::ExecutionIntentStageResult;
pub use execution_intent::candidate_reference_hash;
pub use intent::OutcomeIntent;
pub use intent::OutcomeIntentStageResult;
pub use intent::OutcomeIntentState;
pub use sync_writer::DurableOutcomeWriterError;
pub use sync_writer::SyncDurableOutcomeWriter;
#[cfg(test)]
pub(crate) use sync_writer::SyncDurableOutcomeWriterTestHooks;

/// One terminal outcome record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeRecord {
    attempt_id: String,
    receipt: OutcomeReceipt,
    canonical_evidence: String,
    canonical_evidence_hash: ContentHash,
}

impl OutcomeRecord {
    /// Returns the caller-provided execution-attempt identity.
    pub fn attempt_id(&self) -> &str {
        &self.attempt_id
    }

    /// Returns the complete caller-provided outcome receipt.
    pub fn receipt(&self) -> &OutcomeReceipt {
        &self.receipt
    }

    /// Returns the complete caller-provided opaque canonical evidence envelope.
    pub fn canonical_evidence(&self) -> &str {
        &self.canonical_evidence
    }

    /// Returns the caller-provided digest of canonical outcome evidence.
    pub fn canonical_evidence_hash(&self) -> &ContentHash {
        &self.canonical_evidence_hash
    }
}

/// Result of atomically recording one terminal outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeRecordResult {
    /// The terminal outcome was inserted by this call.
    Recorded,
    /// The exact attempt, receipt envelope, and evidence were already present.
    AlreadyRecorded,
}

impl OutcomeRecordResult {
    /// Reports whether this call inserted the record.
    pub const fn recorded_now(self) -> bool {
        matches!(self, Self::Recorded)
    }
}

/// Non-durable reference implementation of terminal outcome storage.
///
/// Clones share one mutex-protected state. This type neither validates receipt
/// authority nor parses evidence or recalculates canonical evidence hashes,
/// and it has no persistence, runtime, memory-recall, KG, or live-system
/// integration.
#[derive(Clone, Default)]
pub struct InMemoryOutcomeStore {
    state: Arc<Mutex<OutcomeStoreState>>,
}

#[derive(Default)]
struct OutcomeStoreState {
    receipts: BTreeMap<ReceiptId, OutcomeRecord>,
    attempts: BTreeMap<String, ReceiptId>,
}

impl InMemoryOutcomeStore {
    /// Atomically records an already-constructed terminal outcome.
    ///
    /// Exact replay is idempotent. A receipt identity cannot change hash,
    /// envelope, opaque evidence envelope, evidence hash, or attempt, and an
    /// attempt can have only one terminal receipt. Receipt and evidence
    /// authenticity remain the caller's responsibility.
    pub fn record(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        let attempt_id = attempt_id.into();
        let canonical_evidence = canonical_evidence.into();
        let mut guard = self.lock_state()?;

        if let Some(existing) = guard.receipts.get(receipt.id()) {
            return classify_existing_record(
                existing,
                attempt_id,
                receipt,
                canonical_evidence,
                canonical_evidence_hash,
            );
        }

        if let Some(existing_receipt) = guard.attempts.get(&attempt_id) {
            return Err(OutcomeStoreError::AttemptAlreadyFinalized {
                attempt_id,
                existing_receipt: existing_receipt.clone(),
                attempted_receipt: receipt.id().clone(),
            });
        }

        let receipt_id = receipt.id().clone();
        let record = OutcomeRecord {
            attempt_id: attempt_id.clone(),
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
        };
        guard.attempts.insert(attempt_id, receipt_id.clone());
        guard.receipts.insert(receipt_id, record);
        Ok(OutcomeRecordResult::Recorded)
    }

    /// Reads a record by exact receipt identity.
    pub fn read_by_receipt(
        &self,
        receipt: &ReceiptId,
    ) -> Result<Option<OutcomeRecord>, OutcomeStoreError> {
        let guard = self.lock_state()?;
        Ok(guard.receipts.get(receipt).cloned())
    }

    /// Reads a record by caller-provided execution-attempt identity.
    pub fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeStoreError> {
        let guard = self.lock_state()?;
        let Some(receipt) = guard.attempts.get(attempt_id) else {
            return Ok(None);
        };
        Ok(guard.receipts.get(receipt).cloned())
    }

    fn lock_state(&self) -> Result<MutexGuard<'_, OutcomeStoreState>, OutcomeStoreError> {
        self.state
            .lock()
            .map_err(|_| OutcomeStoreError::StorePoisoned)
    }
}

fn classify_existing_record(
    existing: &OutcomeRecord,
    attempt_id: String,
    receipt: OutcomeReceipt,
    canonical_evidence: String,
    canonical_evidence_hash: ContentHash,
) -> Result<OutcomeRecordResult, OutcomeStoreError> {
    if existing.receipt.receipt_hash() != receipt.receipt_hash() {
        return Err(OutcomeStoreError::ReceiptHashConflict {
            receipt: receipt.id().clone(),
            existing: existing.receipt.receipt_hash().clone(),
            attempted: receipt.receipt_hash().clone(),
        });
    }
    if existing.attempt_id != attempt_id {
        return Err(OutcomeStoreError::ReceiptAttemptConflict {
            receipt: receipt.id().clone(),
            existing_attempt: existing.attempt_id.clone(),
            attempted_attempt: attempt_id,
        });
    }
    if existing.receipt != receipt {
        return Err(OutcomeStoreError::ReceiptEnvelopeConflict {
            receipt: existing.receipt.id().clone(),
        });
    }
    let same_evidence = existing.canonical_evidence == canonical_evidence;
    let same_evidence_hash = existing.canonical_evidence_hash == canonical_evidence_hash;
    match (same_evidence, same_evidence_hash) {
        (true, true) => Ok(OutcomeRecordResult::AlreadyRecorded),
        (true, false) => Err(OutcomeStoreError::EvidenceHashConflict {
            receipt: existing.receipt.id().clone(),
            existing: existing.canonical_evidence_hash.clone(),
            attempted: canonical_evidence_hash,
        }),
        (false, true) => Err(OutcomeStoreError::EvidenceEnvelopeConflict {
            receipt: existing.receipt.id().clone(),
            evidence_hash: existing.canonical_evidence_hash.clone(),
        }),
        (false, false) => Err(OutcomeStoreError::EvidenceEnvelopeAndHashConflict {
            receipt: existing.receipt.id().clone(),
            existing_hash: existing.canonical_evidence_hash.clone(),
            attempted_hash: canonical_evidence_hash,
        }),
    }
}

fn map_durable_error(error: crate::durable::DurableStorageError) -> OutcomeStoreError {
    match error {
        crate::durable::DurableStorageError::Persistence { operation, detail } => {
            OutcomeStoreError::Persistence { operation, detail }
        }
        crate::durable::DurableStorageError::Corrupt { detail } => {
            OutcomeStoreError::Corrupt { detail }
        }
    }
}
