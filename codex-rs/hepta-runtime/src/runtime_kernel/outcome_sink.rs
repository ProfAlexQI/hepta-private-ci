//! Synchronous terminal-outcome persistence selected at runtime construction.

mod breaker;

#[cfg(test)]
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use hepta_core::HeptaError;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::DurableOutcomeWriterError;
use hepta_memory::ExecutionEffectAck;
use hepta_memory::ExecutionEffectAckRecordResult;
use hepta_memory::ExecutionIntent;
use hepta_memory::ExecutionIntentStageResult;
#[cfg(test)]
use hepta_memory::InMemoryOutcomeStore;
use hepta_memory::OutcomeIntent;
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

    fn record_execution_effect_ack(
        &self,
        _ack: &ExecutionEffectAck,
    ) -> Result<ExecutionEffectAckRecordResult, OutcomeReceiptSinkError> {
        Err(OutcomeReceiptSinkError::Coordination {
            detail: "outcome sink does not support provider effect acknowledgements".into(),
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
    effect_acks: Mutex<BTreeMap<String, ExecutionEffectAck>>,
}

#[cfg(test)]
impl Default for InMemoryOutcomeReceiptSink {
    fn default() -> Self {
        Self {
            store: InMemoryOutcomeStore::default(),
            effect_acks: Mutex::new(BTreeMap::new()),
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
        self.record(exact)
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

    fn record_execution_effect_ack(
        &self,
        ack: &ExecutionEffectAck,
    ) -> Result<ExecutionEffectAckRecordResult, OutcomeReceiptSinkError> {
        let mut effect_acks =
            self.effect_acks
                .lock()
                .map_err(|_| OutcomeReceiptSinkError::Coordination {
                    detail: "in-memory effect ACK mutex poisoned".into(),
                })?;
        match effect_acks.get(ack.attempt_id()) {
            Some(existing) if existing == ack => {
                Ok(ExecutionEffectAckRecordResult::AlreadyRecorded)
            }
            Some(_) => Err(OutcomeReceiptSinkError::Coordination {
                detail: format!(
                    "execution effect ACK {} has conflicting exact material",
                    ack.attempt_id()
                ),
            }),
            None => {
                effect_acks.insert(ack.attempt_id().to_owned(), ack.clone());
                Ok(ExecutionEffectAckRecordResult::Recorded)
            }
        }
    }

    fn execution_effect_ack(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, OutcomeReceiptSinkError> {
        self.effect_acks
            .lock()
            .map(|effect_acks| effect_acks.get(attempt_id).cloned())
            .map_err(|_| OutcomeReceiptSinkError::Coordination {
                detail: "in-memory effect ACK mutex poisoned".into(),
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

    fn record_execution_effect_ack(
        &self,
        ack: &ExecutionEffectAck,
    ) -> Result<ExecutionEffectAckRecordResult, OutcomeReceiptSinkError> {
        let mut state = self.lock_state()?;
        self.ensure_recovered(&mut state)?;
        let result = state.writer.record_execution_effect_ack(ack.clone());
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

impl RuntimeKernel {
    /// Replays exact material from local breaker state or the durable intent journal.
    pub fn reconcile_pending_outcome(
        &self,
        attempt_id: &str,
    ) -> Result<OutcomeRecordResult, HeptaError> {
        let attempt_id = attempt_id.trim();
        if attempt_id.is_empty() {
            return Err(HeptaError(
                "pending outcome attempt id must not be empty".into(),
            ));
        }
        let local_pending = {
            let mut state = self
                .execution_outcome_state
                .lock()
                .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
            state.breaker.begin_reconciliation(attempt_id)?
        };
        let execution_intent = self
            .outcome_sink
            .pending_execution_intent(attempt_id)
            .map_err(|error| {
                HeptaError(format!(
                    "failed to inspect execution intent {attempt_id}: {error}"
                ))
            })?;
        let (exact, _kind) = match local_pending {
            Some(pending) => pending,
            None => {
                let pending = self
                    .outcome_sink
                    .pending_intent(attempt_id)
                    .map_err(|error| {
                        HeptaError(format!(
                            "failed to recover pending outcome intent {attempt_id}: {error}"
                        ))
                    })?;
                if let Some(pending) = pending {
                    (pending.exact, pending.kind)
                } else if execution_intent.is_some() {
                    let record = self
                        .outcome_sink
                        .read_by_attempt(attempt_id)
                        .map_err(|error| {
                            HeptaError(format!(
                                "failed to recover committed outcome {attempt_id}: {error}"
                            ))
                        })?
                        .ok_or_else(|| {
                            HeptaError(format!(
                                "execution attempt {attempt_id} is in doubt without exact terminal material"
                            ))
                        })?;
                    (
                        ExactOutcomeRecord::from_record(&record),
                        PendingOutcomeKind::CommitAmbiguous,
                    )
                } else {
                    return Err(HeptaError(format!(
                        "no retryable outcome is pending for attempt {attempt_id}"
                    )));
                }
            }
        };
        let replay = match execution_intent.as_ref() {
            Some(intent) => self
                .outcome_sink
                .record_and_resolve_execution(&exact, intent),
            None => self.outcome_sink.record(&exact),
        };
        let result = match replay {
            Ok(OutcomeRecordResult::Recorded) => OutcomeRecordResult::Recorded,
            Ok(OutcomeRecordResult::AlreadyRecorded) => OutcomeRecordResult::AlreadyRecorded,
            Err(error) if error.pending_kind().is_some() => {
                let mut state = self
                    .execution_outcome_state
                    .lock()
                    .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
                if let Err(reason) = state.breaker.retain_pending(exact, error.clone()) {
                    state.breaker.trip_fatal(reason);
                }
                return Err(HeptaError(format!(
                    "outcome reconciliation remains pending: {error}"
                )));
            }
            Err(error) => {
                let reason = format!("outcome reconciliation failed: {error}");
                match self.execution_outcome_state.lock() {
                    Ok(mut state) => {
                        state.breaker.finish_nonretryable_failure(&exact);
                        state.breaker.trip_fatal(reason.clone());
                    }
                    Err(poisoned) => {
                        let mut state = poisoned.into_inner();
                        state.breaker.finish_nonretryable_failure(&exact);
                        state.breaker.trip_fatal(reason.clone());
                    }
                }
                return Err(HeptaError(reason));
            }
        };
        let mut state = self
            .execution_outcome_state
            .lock()
            .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
        if state.breaker.resolve(&exact) {
            state.active_attempts.remove(attempt_id);
            state.finalized_attempts = state.finalized_attempts.saturating_add(1);
        }
        Ok(result)
    }

    pub(super) fn durable_pending_outcome_reason(&self) -> Result<Option<String>, HeptaError> {
        let execution_intent = self
            .outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "durable execution-intent inspection failed closed: {error}"
                ))
            })?
            .into_iter()
            .next();
        if let Some(intent) = execution_intent {
            return Ok(Some(format!(
                "durable execution attempt {} is in doubt before terminal resolution",
                intent.attempt_id()
            )));
        }
        self.outcome_sink
            .first_pending_intent()
            .map(|pending| {
                pending.map(|pending| {
                    format!(
                        "durable outcome attempt {} requires exact {} from the producer-intent journal",
                        pending.exact.attempt_id(),
                        match pending.kind {
                            PendingOutcomeKind::SafeRetry => "retry",
                            PendingOutcomeKind::CommitAmbiguous => "reconciliation",
                        }
                    )
                })
            })
            .map_err(|error| {
                HeptaError(format!(
                    "durable outcome intent inspection failed closed: {error}"
                ))
            })
    }

    /// Enumerates exact pre-dispatch plans that still block provider execution.
    pub fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, HeptaError> {
        self.outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "failed to enumerate pending execution intents: {error}"
                ))
            })
    }

    /// Inspects unresolved provider effects without replaying or mutating them.
    pub fn pending_execution_effect_inspections(
        &self,
    ) -> Result<Vec<crate::PendingExecutionEffectInspection>, HeptaError> {
        self.outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "failed to enumerate pending execution intents: {error}"
                ))
            })?
            .into_iter()
            .map(|intent| {
                let ack = self
                    .outcome_sink
                    .execution_effect_ack(intent.attempt_id())
                    .map_err(|error| {
                        HeptaError(format!(
                            "failed to inspect provider effect ACK {}: {error}",
                            intent.attempt_id()
                        ))
                    })?;
                super::provider_effect::inspect_pending_effect(&intent, ack.as_ref())
            })
            .collect()
    }

    /// Compatibility name for commit-ambiguity reconciliation.
    pub fn reconcile_ambiguous_outcome(
        &self,
        attempt_id: &str,
    ) -> Result<OutcomeRecordResult, HeptaError> {
        self.reconcile_pending_outcome(attempt_id)
    }
}
