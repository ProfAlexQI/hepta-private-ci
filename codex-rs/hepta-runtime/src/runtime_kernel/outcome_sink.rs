//! Synchronous terminal-outcome persistence selected at runtime construction.

mod breaker;
mod monotonic_state;
#[cfg(test)]
use std::collections::BTreeMap;
mod runtime;

use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use hepta_core::HeptaError;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::DurableMonotonicState;
use hepta_memory::DurableOutcomeWriterError;
use hepta_memory::ExecutionEffectAck;
use hepta_memory::ExecutionIntent;
use hepta_memory::ExecutionIntentStageResult;
#[cfg(test)]
use hepta_memory::InMemoryOutcomeStore;
use hepta_memory::OutcomeIntent;
use hepta_memory::OutcomeIntentStageResult;
use hepta_memory::OutcomeIntentState;
use hepta_memory::OutcomeRecord;
use hepta_memory::OutcomeRecordResult;
#[cfg(test)]
use hepta_memory::OutcomeStoreError;
use hepta_memory::SyncDurableOutcomeWriter;

use super::execution_attempt::AuthorizedToolExecution;
use crate::RuntimeKernel;
pub(crate) use breaker::OutcomeBreakerState;

#[cfg(test)]
mod tests;

pub(crate) type SharedOutcomeReceiptSink = Arc<dyn OutcomeReceiptSink>;

/// Exact immutable material retained across a retryable durable failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactOutcomeRecord {
    attempt_id: String,
    receipt: OutcomeReceipt,
    canonical_evidence: String,
    canonical_evidence_hash: ContentHash,
}

impl ExactOutcomeRecord {
    pub(crate) fn new(
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Self {
        Self {
            attempt_id: attempt_id.into(),
            receipt,
            canonical_evidence: canonical_evidence.into(),
            canonical_evidence_hash,
        }
    }

    pub(crate) fn attempt_id(&self) -> &str {
        &self.attempt_id
    }

    fn from_intent(intent: &OutcomeIntent) -> Self {
        Self::from_record(intent.record())
    }

    fn from_record(record: &OutcomeRecord) -> Self {
        Self::new(
            record.attempt_id(),
            record.receipt().clone(),
            record.canonical_evidence(),
            record.canonical_evidence_hash().clone(),
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RecoveredPendingOutcome {
    exact: ExactOutcomeRecord,
    kind: PendingOutcomeKind,
}

/// Object-safe synchronous boundary required by terminal `Drop` paths.
pub(crate) trait OutcomeReceiptSink: Send + Sync {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError>;

    fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError>;

    fn stage_execution_intent(
        &self,
        intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError>;

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError>;

    fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError>;

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError>;

    fn stage_provider_completion(
        &self,
        _ack: &ExecutionEffectAck,
        _exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeIntentStageResult, OutcomeReceiptSinkError> {
        Err(OutcomeReceiptSinkError::Coordination {
            detail: "outcome sink does not support atomic provider completion staging".into(),
        })
    }

    fn execution_effect_ack(
        &self,
        _attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, OutcomeReceiptSinkError> {
        Ok(None)
    }

    fn pending_intent(
        &self,
        _attempt_id: &str,
    ) -> Result<Option<RecoveredPendingOutcome>, OutcomeReceiptSinkError> {
        Ok(None)
    }

    fn first_pending_intent(
        &self,
    ) -> Result<Option<RecoveredPendingOutcome>, OutcomeReceiptSinkError> {
        Ok(None)
    }

    fn monotonic_state(&self) -> Result<DurableMonotonicState, OutcomeReceiptSinkError> {
        Err(OutcomeReceiptSinkError::Coordination {
            detail: "outcome sink does not expose a durable monotonic state".into(),
        })
    }
}

/// Preserves the concrete backend classification across the runtime boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OutcomeReceiptSinkError {
    #[cfg(test)]
    InMemory(OutcomeStoreError),
    Durable(DurableOutcomeWriterError),
    Coordination {
        detail: String,
    },
}

impl OutcomeReceiptSinkError {
    pub(crate) fn is_commit_ambiguous(&self) -> bool {
        matches!(
            self,
            Self::Durable(
                DurableOutcomeWriterError::AcknowledgementTimeout { .. }
                    | DurableOutcomeWriterError::CommitAmbiguous { .. }
            )
        )
    }

    fn pending_kind(&self) -> Option<PendingOutcomeKind> {
        if self.is_commit_ambiguous() {
            Some(PendingOutcomeKind::CommitAmbiguous)
        } else if matches!(self, Self::Durable(_) | Self::Coordination { .. }) {
            Some(PendingOutcomeKind::SafeRetry)
        } else {
            None
        }
    }
}

impl fmt::Display for OutcomeReceiptSinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(test)]
            Self::InMemory(source) => write!(formatter, "in-memory outcome store failed: {source}"),
            Self::Durable(source) => write!(formatter, "{source}"),
            Self::Coordination { detail } => {
                write!(formatter, "outcome sink coordination failed: {detail}")
            }
        }
    }
}

impl Error for OutcomeReceiptSinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(test)]
            Self::InMemory(source) => Some(source),
            Self::Durable(source) => Some(source),
            Self::Coordination { .. } => None,
        }
    }
}

#[cfg(test)]
struct InMemoryOutcomeReceiptSink {
    store: InMemoryOutcomeStore,
    provider_state: Mutex<InMemoryProviderState>,
}

#[cfg(test)]
#[derive(Default)]
struct InMemoryProviderState {
    effect_acks: BTreeMap<String, ExecutionEffectAck>,
    completions: BTreeMap<String, ExactOutcomeRecord>,
}

#[cfg(test)]
impl Default for InMemoryOutcomeReceiptSink {
    fn default() -> Self {
        Self {
            store: InMemoryOutcomeStore::default(),
            provider_state: Mutex::new(InMemoryProviderState::default()),
        }
    }
}

#[cfg(test)]
impl OutcomeReceiptSink for InMemoryOutcomeReceiptSink {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        self.store
            .record(
                exact.attempt_id.clone(),
                exact.receipt.clone(),
                exact.canonical_evidence.clone(),
                exact.canonical_evidence_hash.clone(),
            )
            .map_err(OutcomeReceiptSinkError::InMemory)
    }

    fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError> {
        self.store
            .read_by_attempt(attempt_id)
            .map_err(OutcomeReceiptSinkError::InMemory)
    }

    fn stage_execution_intent(
        &self,
        _intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError> {
        Ok(ExecutionIntentStageResult::Staged)
    }

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        _intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        let result = self.record(exact)?;
        self.provider_state
            .lock()
            .map_err(|_| OutcomeReceiptSinkError::Coordination {
                detail: "in-memory provider completion mutex poisoned".into(),
            })?
            .completions
            .remove(exact.attempt_id());
        Ok(result)
    }

    fn pending_execution_intent(
        &self,
        _attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(None)
    }

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(Vec::new())
    }

    fn stage_provider_completion(
        &self,
        ack: &ExecutionEffectAck,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeIntentStageResult, OutcomeReceiptSinkError> {
        if ack.attempt_id() != exact.attempt_id() {
            return Err(OutcomeReceiptSinkError::Coordination {
                detail: "provider ACK and completion attempt bindings differ".into(),
            });
        }
        let mut state =
            self.provider_state
                .lock()
                .map_err(|_| OutcomeReceiptSinkError::Coordination {
                    detail: "in-memory provider completion mutex poisoned".into(),
                })?;
        match state.effect_acks.get(ack.attempt_id()) {
            Some(existing) if existing != ack => {
                return Err(OutcomeReceiptSinkError::Coordination {
                    detail: format!(
                        "execution effect ACK {} has conflicting exact material",
                        ack.attempt_id()
                    ),
                });
            }
            _ => {}
        }
        match state.completions.get(exact.attempt_id()) {
            Some(existing) if existing != exact => {
                return Err(OutcomeReceiptSinkError::Coordination {
                    detail: format!(
                        "provider completion {} has conflicting exact material",
                        exact.attempt_id()
                    ),
                });
            }
            _ => {}
        }
        state
            .effect_acks
            .insert(ack.attempt_id().to_owned(), ack.clone());
        state
            .completions
            .insert(exact.attempt_id().to_owned(), exact.clone());
        Ok(OutcomeIntentStageResult::Pending)
    }

    fn execution_effect_ack(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, OutcomeReceiptSinkError> {
        self.provider_state
            .lock()
            .map(|state| state.effect_acks.get(attempt_id).cloned())
            .map_err(|_| OutcomeReceiptSinkError::Coordination {
                detail: "in-memory provider completion mutex poisoned".into(),
            })
    }

    fn pending_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RecoveredPendingOutcome>, OutcomeReceiptSinkError> {
        self.provider_state
            .lock()
            .map(|state| {
                state
                    .completions
                    .get(attempt_id)
                    .cloned()
                    .map(|exact| RecoveredPendingOutcome {
                        exact,
                        kind: PendingOutcomeKind::SafeRetry,
                    })
            })
            .map_err(|_| OutcomeReceiptSinkError::Coordination {
                detail: "in-memory provider completion mutex poisoned".into(),
            })
    }
}

struct DurableOutcomeReceiptSink {
    state: Mutex<DurableSinkState>,
}

struct DurableSinkState {
    writer: SyncDurableOutcomeWriter,
    recover_before_next_command: bool,
}

impl DurableOutcomeReceiptSink {
    fn bootstrap_new(
        path: &Path,
        integrity_key: DurableIntegrityKey,
    ) -> Result<Self, DurableOutcomeWriterError> {
        Ok(Self::from_writer(
            SyncDurableOutcomeWriter::bootstrap_new_keyed(path, integrity_key)?,
        ))
    }

    fn open_existing(
        path: &Path,
        integrity_key: DurableIntegrityKey,
    ) -> Result<Self, DurableOutcomeWriterError> {
        Ok(Self::from_writer(
            SyncDurableOutcomeWriter::open_existing_keyed(path, integrity_key)?,
        ))
    }

    fn from_writer(writer: SyncDurableOutcomeWriter) -> Self {
        Self {
            state: Mutex::new(DurableSinkState {
                writer,
                recover_before_next_command: false,
            }),
        }
    }

    fn lock_state(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, DurableSinkState>, OutcomeReceiptSinkError> {
        self.state
            .lock()
            .map_err(|_| OutcomeReceiptSinkError::Coordination {
                detail: "durable outcome adapter mutex poisoned".into(),
            })
    }

    fn ensure_recovered(
        &self,
        state: &mut DurableSinkState,
    ) -> Result<(), OutcomeReceiptSinkError> {
        if !state.recover_before_next_command {
            return Ok(());
        }
        state.writer = state
            .writer
            .reopen_existing_bound()
            .map_err(OutcomeReceiptSinkError::Durable)?;
        state.recover_before_next_command = false;
        Ok(())
    }
}

impl OutcomeReceiptSink for DurableOutcomeReceiptSink {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        let result = state.writer.record(
            exact.attempt_id.clone(),
            exact.receipt.clone(),
            exact.canonical_evidence.clone(),
            exact.canonical_evidence_hash.clone(),
        );
        if matches!(
            result,
            Err(DurableOutcomeWriterError::AcknowledgementTimeout { .. }
                | DurableOutcomeWriterError::CommitAmbiguous { .. }
                | DurableOutcomeWriterError::WorkerUnavailable)
        ) {
            state.recover_before_next_command = true;
        }
        result.map_err(OutcomeReceiptSinkError::Durable)
    }

    fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .read_by_attempt(attempt_id)
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn stage_execution_intent(
        &self,
        intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        let result = state.writer.stage_execution_intent(intent.clone());
        if matches!(
            result,
            Err(DurableOutcomeWriterError::AcknowledgementTimeout { .. }
                | DurableOutcomeWriterError::CommitAmbiguous { .. }
                | DurableOutcomeWriterError::WorkerUnavailable)
        ) {
            state.recover_before_next_command = true;
        }
        result.map_err(OutcomeReceiptSinkError::Durable)
    }

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        let result = state.writer.record_and_resolve_execution(
            exact.attempt_id.clone(),
            exact.receipt.clone(),
            exact.canonical_evidence.clone(),
            exact.canonical_evidence_hash.clone(),
            intent,
        );
        if matches!(
            result,
            Err(DurableOutcomeWriterError::AcknowledgementTimeout { .. }
                | DurableOutcomeWriterError::CommitAmbiguous { .. }
                | DurableOutcomeWriterError::WorkerUnavailable)
        ) {
            state.recover_before_next_command = true;
        }
        result.map_err(OutcomeReceiptSinkError::Durable)
    }

    fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .pending_execution_intent(attempt_id)
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .pending_execution_intents()
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn stage_provider_completion(
        &self,
        ack: &ExecutionEffectAck,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeIntentStageResult, OutcomeReceiptSinkError> {
        if ack.attempt_id() != exact.attempt_id() {
            return Err(OutcomeReceiptSinkError::Coordination {
                detail: "provider ACK and completion attempt bindings differ".into(),
            });
        }
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        let result = state.writer.stage_provider_completion(
            ack.clone(),
            exact.receipt.clone(),
            exact.canonical_evidence.clone(),
            exact.canonical_evidence_hash.clone(),
        );
        if matches!(
            result,
            Err(DurableOutcomeWriterError::AcknowledgementTimeout { .. }
                | DurableOutcomeWriterError::CommitAmbiguous { .. }
                | DurableOutcomeWriterError::WorkerUnavailable)
        ) {
            state.recover_before_next_command = true;
        }
        result.map_err(OutcomeReceiptSinkError::Durable)
    }

    fn execution_effect_ack(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .execution_effect_ack(attempt_id)
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn pending_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RecoveredPendingOutcome>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .pending_intent(attempt_id)
            .map(|intent| intent.map(recovered_pending_outcome))
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn first_pending_intent(
        &self,
    ) -> Result<Option<RecoveredPendingOutcome>, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .pending_intents()
            .map(|intents| intents.into_iter().next().map(recovered_pending_outcome))
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn monotonic_state(&self) -> Result<DurableMonotonicState, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        state
            .writer
            .monotonic_state()
            .map_err(OutcomeReceiptSinkError::Durable)
    }
}

fn recovered_pending_outcome(intent: OutcomeIntent) -> RecoveredPendingOutcome {
    let kind = match intent.state() {
        OutcomeIntentState::Pending => PendingOutcomeKind::SafeRetry,
        OutcomeIntentState::Committed => PendingOutcomeKind::CommitAmbiguous,
    };
    RecoveredPendingOutcome {
        exact: ExactOutcomeRecord::from_intent(&intent),
        kind,
    }
}

#[cfg(test)]
pub(crate) fn in_memory_outcome_sink() -> SharedOutcomeReceiptSink {
    Arc::new(InMemoryOutcomeReceiptSink::default())
}

pub(crate) fn bootstrap_new_durable_outcome_sink(
    path: &Path,
    integrity_key: DurableIntegrityKey,
) -> Result<SharedOutcomeReceiptSink, DurableOutcomeWriterError> {
    DurableOutcomeReceiptSink::bootstrap_new(path, integrity_key)
        .map(|sink| Arc::new(sink) as SharedOutcomeReceiptSink)
}

pub(crate) fn open_existing_durable_outcome_sink(
    path: &Path,
    integrity_key: DurableIntegrityKey,
) -> Result<SharedOutcomeReceiptSink, DurableOutcomeWriterError> {
    DurableOutcomeReceiptSink::open_existing(path, integrity_key)
        .map(|sink| Arc::new(sink) as SharedOutcomeReceiptSink)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingOutcomeKind {
    SafeRetry,
    CommitAmbiguous,
}

impl PendingOutcomeKind {
    fn merge(self, next: Self) -> Self {
        if matches!(self, Self::CommitAmbiguous) || matches!(next, Self::CommitAmbiguous) {
            Self::CommitAmbiguous
        } else {
            Self::SafeRetry
        }
    }
}

pub(crate) fn record_first_outcome(
    execution: &mut AuthorizedToolExecution,
    exact: ExactOutcomeRecord,
) -> Result<(), HeptaError> {
    let result = match execution.execution_intent() {
        Some(intent) => execution
            .outcome_sink()
            .record_and_resolve_execution(&exact, intent),
        None => execution.outcome_sink().record(&exact),
    };
    match result {
        Ok(OutcomeRecordResult::Recorded) => {
            execution.mark_receipt_finalized();
            Ok(())
        }
        Ok(OutcomeRecordResult::AlreadyRecorded) if execution.execution_intent().is_some() => {
            execution.mark_receipt_finalized();
            Ok(())
        }
        Ok(OutcomeRecordResult::AlreadyRecorded) => fail_fatal(
            execution,
            "outcome receipt invariant failed: attempt was already finalized".into(),
        ),
        Err(error) if error.pending_kind().is_some() => {
            let message = if error.is_commit_ambiguous() {
                format!("outcome receipt sink has an ambiguous commit: {error}")
            } else {
                format!("outcome receipt sink retained an exact pending intent: {error}")
            };
            let retained = match execution.reservation.state.lock() {
                Ok(mut state) => state.breaker.retain_pending(exact, error),
                Err(poisoned) => poisoned.into_inner().breaker.retain_pending(exact, error),
            };
            if let Err(reason) = retained {
                execution.trip_outcome_breaker(reason);
            }
            execution.disarm_receipt_guard();
            Err(HeptaError(message))
        }
        Err(error) => fail_fatal(execution, format!("outcome receipt sink failed: {error}")),
    }
}

pub(crate) fn fail_fatal(
    execution: &mut AuthorizedToolExecution,
    reason: String,
) -> Result<(), HeptaError> {
    execution.trip_outcome_breaker(reason.clone());
    execution.disarm_receipt_guard();
    Err(HeptaError(reason))
}
