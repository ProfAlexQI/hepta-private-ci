use crate::outcome_store::DurableOutcomeStore;

use super::DurableOutcomeWriterError;
use super::IntentControl;
use super::IntentControlResult;

pub(super) fn run_intent_control(
    path: std::path::PathBuf,
    identity: crate::durable::DurableDatabaseIdentity,
    integrity: crate::durable::DurableIntegrityContext,
    command: IntentControl,
) -> Result<IntentControlResult, DurableOutcomeWriterError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| DurableOutcomeWriterError::WorkerStartup {
            detail: format!("could not initialize outcome intent runtime: {error}"),
        })?;
    let store = runtime
        .block_on(DurableOutcomeStore::open_existing_bound_with_integrity(
            path, identity, integrity,
        ))
        .map_err(|source| DurableOutcomeWriterError::Backend { source })?;
    match command {
        IntentControl::Stage {
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
        } => runtime
            .block_on(store.stage_intent(
                attempt_id.clone(),
                *receipt,
                canonical_evidence,
                canonical_evidence_hash,
            ))
            .map(IntentControlResult::Staged)
            .map_err(|source| super::super::error::map_stage_error(attempt_id, source)),
        IntentControl::Acknowledge { attempt_id } => runtime
            .block_on(store.acknowledge_intent(&attempt_id))
            .map(|()| IntentControlResult::Acknowledged)
            .map_err(|source| super::super::error::map_acknowledgement_error(attempt_id, source)),
        IntentControl::Read { attempt_id } => runtime
            .block_on(store.pending_intent(&attempt_id))
            .map(Box::new)
            .map(IntentControlResult::Read)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
        IntentControl::List => runtime
            .block_on(store.pending_intents())
            .map(IntentControlResult::Listed)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
        IntentControl::StageExecution { intent } => {
            let attempt_id = intent.attempt_id().to_owned();
            runtime
                .block_on(store.stage_execution_intent(*intent))
                .map(IntentControlResult::ExecutionStaged)
                .map_err(|source| {
                    super::super::error::map_execution_stage_error(attempt_id, source)
                })
        }
        IntentControl::ResolveExecution {
            attempt_id,
            idempotency_key,
        } => runtime
            .block_on(store.resolve_execution_intent(&attempt_id, &idempotency_key))
            .map(IntentControlResult::ExecutionResolved)
            .map_err(|source| {
                super::super::error::map_execution_resolution_error(attempt_id, source)
            }),
        IntentControl::ReadExecution { attempt_id } => runtime
            .block_on(store.pending_execution_intent(&attempt_id))
            .map(Box::new)
            .map(IntentControlResult::ExecutionRead)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
        IntentControl::ListExecution => runtime
            .block_on(store.pending_execution_intents())
            .map(IntentControlResult::ExecutionListed)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
        IntentControl::RecordEffectAck { ack } => {
            let attempt_id = ack.attempt_id().to_owned();
            runtime
                .block_on(store.record_execution_effect_ack(*ack))
                .map(IntentControlResult::EffectAckRecorded)
                .map_err(|source| {
                    super::super::error::map_effect_ack_record_error(attempt_id, source)
                })
        }
        IntentControl::StageProviderCompletion {
            ack,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
        } => {
            let attempt_id = ack.attempt_id().to_owned();
            runtime
                .block_on(store.stage_provider_completion(
                    *ack,
                    attempt_id.clone(),
                    *receipt,
                    canonical_evidence,
                    canonical_evidence_hash,
                ))
                .map(IntentControlResult::ProviderCompletionStaged)
                .map_err(|source| {
                    super::super::error::map_provider_completion_stage_error(attempt_id, source)
                })
        }
        IntentControl::ReadEffectAck { attempt_id } => runtime
            .block_on(store.execution_effect_ack(&attempt_id))
            .map(Box::new)
            .map(IntentControlResult::EffectAckRead)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
        IntentControl::MonotonicState => runtime
            .block_on(store.monotonic_state())
            .map(IntentControlResult::MonotonicState)
            .map_err(|source| DurableOutcomeWriterError::Backend { source }),
    }
}
