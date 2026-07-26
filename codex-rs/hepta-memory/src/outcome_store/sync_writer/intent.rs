use std::sync::mpsc;
use std::thread;

mod worker;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;

use super::DurableOutcomeWriterError;
use super::SyncDurableOutcomeWriter;
use super::duration_millis;
use crate::DurableMonotonicState;
use crate::ExecutionEffectAck;
use crate::ExecutionEffectAckRecordResult;
use crate::ExecutionIntent;
use crate::ExecutionIntentResolveResult;
use crate::ExecutionIntentStageResult;
use crate::OutcomeIntent;
use crate::OutcomeIntentStageResult;

use self::worker::run_intent_control;

enum IntentControl {
    Stage {
        attempt_id: String,
        receipt: Box<OutcomeReceipt>,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
    },
    Acknowledge {
        attempt_id: String,
    },
    Read {
        attempt_id: String,
    },
    List,
    StageExecution {
        intent: Box<ExecutionIntent>,
    },
    ResolveExecution {
        attempt_id: String,
        idempotency_key: String,
    },
    ReadExecution {
        attempt_id: String,
    },
    ListExecution,
    RecordEffectAck {
        ack: Box<ExecutionEffectAck>,
    },
    StageProviderCompletion {
        ack: Box<ExecutionEffectAck>,
        receipt: Box<OutcomeReceipt>,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
    },
    ReadEffectAck {
        attempt_id: String,
    },
    MonotonicState,
}

enum IntentControlResult {
    Staged(OutcomeIntentStageResult),
    Acknowledged,
    Read(Box<Option<OutcomeIntent>>),
    Listed(Vec<OutcomeIntent>),
    ExecutionStaged(ExecutionIntentStageResult),
    ExecutionResolved(ExecutionIntentResolveResult),
    ExecutionRead(Box<Option<ExecutionIntent>>),
    ExecutionListed(Vec<ExecutionIntent>),
    EffectAckRecorded(ExecutionEffectAckRecordResult),
    ProviderCompletionStaged(OutcomeIntentStageResult),
    EffectAckRead(Box<Option<ExecutionEffectAck>>),
    MonotonicState(DurableMonotonicState),
}

enum IntentControlDeadline {
    Mutation(String),
    Read(String),
    List,
}

impl SyncDurableOutcomeWriter {
    /// Reads one bounded authenticated snapshot for external rollback anchoring.
    pub fn monotonic_state(&self) -> Result<DurableMonotonicState, DurableOutcomeWriterError> {
        match self.run_intent_control(IntentControl::MonotonicState, IntentControlDeadline::List)? {
            IntentControlResult::MonotonicState(state) => Ok(state),
            _ => Err(invalid_control_result("read durable monotonic state")),
        }
    }

    /// Persists one provider-owned acknowledgement after its exact effect commits.
    pub fn record_execution_effect_ack(
        &self,
        ack: ExecutionEffectAck,
    ) -> Result<ExecutionEffectAckRecordResult, DurableOutcomeWriterError> {
        let attempt_id = ack.attempt_id().to_owned();
        match self.run_intent_control(
            IntentControl::RecordEffectAck { ack: Box::new(ack) },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::EffectAckRecorded(result) => Ok(result),
            _ => Err(invalid_control_result("record execution effect ACK")),
        }
    }

    /// Atomically stages one provider effect ACK and its exact terminal
    /// completion material in the same SQLite transaction.
    pub fn stage_provider_completion(
        &self,
        ack: ExecutionEffectAck,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeIntentStageResult, DurableOutcomeWriterError> {
        let attempt_id = ack.attempt_id().to_owned();
        match self.run_intent_control(
            IntentControl::StageProviderCompletion {
                ack: Box::new(ack),
                receipt: Box::new(receipt),
                canonical_evidence: canonical_evidence.into(),
                canonical_evidence_hash,
            },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::ProviderCompletionStaged(result) => Ok(result),
            _ => Err(invalid_control_result("stage provider completion")),
        }
    }

    /// Reads one durable provider-owned effect acknowledgement.
    pub fn execution_effect_ack(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        match self.run_intent_control(
            IntentControl::ReadEffectAck {
                attempt_id: attempt_id.clone(),
            },
            IntentControlDeadline::Read(attempt_id),
        )? {
            IntentControlResult::EffectAckRead(ack) => Ok(*ack),
            _ => Err(invalid_control_result("read execution effect ACK")),
        }
    }

    /// Persists one exact plan before its provider may be invoked.
    pub fn stage_execution_intent(
        &self,
        intent: ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, DurableOutcomeWriterError> {
        let attempt_id = intent.attempt_id().to_owned();
        match self.run_intent_control(
            IntentControl::StageExecution {
                intent: Box::new(intent),
            },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::ExecutionStaged(result) => Ok(result),
            _ => Err(invalid_control_result("stage execution intent")),
        }
    }

    /// Lists every unresolved pre-dispatch plan.
    pub fn pending_execution_intents(
        &self,
    ) -> Result<Vec<ExecutionIntent>, DurableOutcomeWriterError> {
        match self.run_intent_control(IntentControl::ListExecution, IntentControlDeadline::List)? {
            IntentControlResult::ExecutionListed(intents) => Ok(intents),
            _ => Err(invalid_control_result("list execution intents")),
        }
    }

    /// Reads one unresolved pre-dispatch plan by attempt identity.
    pub fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        match self.run_intent_control(
            IntentControl::ReadExecution {
                attempt_id: attempt_id.clone(),
            },
            IntentControlDeadline::Read(attempt_id),
        )? {
            IntentControlResult::ExecutionRead(intent) => Ok(*intent),
            _ => Err(invalid_control_result("read execution intent")),
        }
    }

    pub(super) fn resolve_execution_intent(
        &self,
        attempt_id: &str,
        idempotency_key: &str,
    ) -> Result<ExecutionIntentResolveResult, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        match self.run_intent_control(
            IntentControl::ResolveExecution {
                attempt_id: attempt_id.clone(),
                idempotency_key: idempotency_key.to_owned(),
            },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::ExecutionResolved(result) => Ok(result),
            _ => Err(invalid_control_result("resolve execution intent")),
        }
    }

    /// Persists exact producer material outside the bounded commit queue.
    pub fn stage_intent(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeIntentStageResult, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.into();
        match self.run_intent_control(
            IntentControl::Stage {
                attempt_id: attempt_id.clone(),
                receipt: Box::new(receipt),
                canonical_evidence: canonical_evidence.into(),
                canonical_evidence_hash,
            },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::Staged(result) => Ok(result),
            _ => Err(invalid_control_result("stage outcome intent")),
        }
    }

    /// Lists exact intents that survived commit queue rejection or lost ACK.
    pub fn pending_intents(&self) -> Result<Vec<OutcomeIntent>, DurableOutcomeWriterError> {
        match self.run_intent_control(IntentControl::List, IntentControlDeadline::List)? {
            IntentControlResult::Listed(intents) => Ok(intents),
            _ => Err(invalid_control_result("list outcome intents")),
        }
    }

    /// Reads one exact unresolved intent by attempt identity.
    pub fn pending_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeIntent>, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        match self.run_intent_control(
            IntentControl::Read {
                attempt_id: attempt_id.clone(),
            },
            IntentControlDeadline::Read(attempt_id),
        )? {
            IntentControlResult::Read(intent) => Ok(*intent),
            _ => Err(invalid_control_result("read outcome intent")),
        }
    }

    pub(super) fn acknowledge_intent(
        &self,
        attempt_id: &str,
    ) -> Result<(), DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        match self.run_intent_control(
            IntentControl::Acknowledge {
                attempt_id: attempt_id.clone(),
            },
            IntentControlDeadline::Mutation(attempt_id),
        )? {
            IntentControlResult::Acknowledged => Ok(()),
            _ => Err(invalid_control_result("acknowledge outcome intent")),
        }
    }

    fn run_intent_control(
        &self,
        command: IntentControl,
        deadline: IntentControlDeadline,
    ) -> Result<IntentControlResult, DurableOutcomeWriterError> {
        let path = self.inner.path.clone();
        let identity = self.inner.identity.clone();
        let integrity = self.inner.integrity.clone();
        let (acknowledgement, receiver) = mpsc::sync_channel(1);
        thread::Builder::new()
            .name("hepta-durable-outcome-intent".into())
            .spawn(move || {
                let result = run_intent_control(path, identity, integrity, command);
                let _ = acknowledgement.send(result);
            })
            .map_err(|error| DurableOutcomeWriterError::WorkerStartup {
                detail: format!("could not start outcome intent worker: {error}"),
            })?;

        match receiver.recv_timeout(self.inner.startup_timeout) {
            Ok(result) => result,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                Err(deadline_error(deadline, self.inner.startup_timeout))
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(disconnected_error(deadline)),
        }
    }
}

fn deadline_error(
    deadline: IntentControlDeadline,
    timeout: std::time::Duration,
) -> DurableOutcomeWriterError {
    match deadline {
        IntentControlDeadline::Mutation(attempt_id) => DurableOutcomeWriterError::CommitAmbiguous {
            attempt_id,
            detail: format!(
                "producer-intent operation exceeded its {}ms acknowledgement deadline",
                duration_millis(timeout)
            ),
        },
        IntentControlDeadline::Read(attempt_id) => {
            DurableOutcomeWriterError::ReadAcknowledgementTimeout {
                attempt_id,
                timeout_ms: duration_millis(timeout),
            }
        }
        IntentControlDeadline::List => DurableOutcomeWriterError::WorkerUnavailable,
    }
}

fn disconnected_error(deadline: IntentControlDeadline) -> DurableOutcomeWriterError {
    match deadline {
        IntentControlDeadline::Mutation(attempt_id) => DurableOutcomeWriterError::CommitAmbiguous {
            attempt_id,
            detail: "producer-intent worker exited before acknowledgement".into(),
        },
        IntentControlDeadline::Read(_) | IntentControlDeadline::List => {
            DurableOutcomeWriterError::WorkerUnavailable
        }
    }
}

fn invalid_control_result(operation: &str) -> DurableOutcomeWriterError {
    DurableOutcomeWriterError::InvalidConfiguration {
        detail: format!("intent worker returned the wrong result for {operation}"),
    }
}
