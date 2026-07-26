mod recording;

use hepta_core::EventKind;
use hepta_core::HeptaError;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::MemoryStore;
use hepta_core::MessageRole;
use hepta_core::SessionId;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;

use super::context_freezer::now_ms;
use super::execution_attempt::AuthorizedToolExecution;
use super::execution_bus::CapturedDispatchTerminal;
use super::execution_bus::CapturedToolExecution;
use super::execution_bus::CapturedTransaction;
use super::execution_bus::CapturedValidation;
use super::outcome_sink::ExactOutcomeRecord;
use super::outcome_sink::fail_fatal;
use super::outcome_sink::record_first_outcome;
use super::terminal_outcome::OutcomeMaterial;
use super::terminal_outcome::ToolDispatchTerminal;
use super::terminal_outcome::ToolEffectDisposition;
use super::terminal_outcome::ToolExecutorBinding;
use super::terminal_outcome::ToolOutcomeFinalizationInput;
use super::terminal_outcome::ToolOutputValidationStatus;
use super::terminal_outcome::ToolTransactionEvidence;
use super::terminal_outcome::finalize_tool_outcome;
use crate::RuntimeKernel;
use crate::TurnRecord;
use crate::current_unix_ms;
use recording::effect_evidence;
use recording::terminal_evidence;
use recording::transaction_evidence;
use recording::validation_evidence;

/// Private persistence seam that preserves the runtime's existing memory,
/// transcript, and turn-history behavior.
pub(crate) struct OutcomeRecorder<'a> {
    kernel: &'a RuntimeKernel,
}

impl<'a> OutcomeRecorder<'a> {
    pub(crate) fn new(kernel: &'a RuntimeKernel) -> Self {
        Self { kernel }
    }

    pub(crate) fn finalize_tool_dispatch(
        &self,
        captured: &mut CapturedToolExecution<'_>,
    ) -> Result<(), HeptaError> {
        let terminal = captured.terminal().clone();
        let validation = captured.validation().clone();
        let transaction = captured.transaction().clone();
        let tool_result = captured.tool_result().cloned();
        let provider_output_json = captured.provider_output_json().map(str::to_string);
        let finished_at_unix_ms = captured.finished_at_unix_ms();
        let staged_terminal = captured.staged_terminal().cloned();
        let result = match captured.execution_mut() {
            Some(execution) => match staged_terminal {
                Some(exact) => record_first_outcome(execution, exact),
                None => Self::record_terminal(
                    execution,
                    &terminal,
                    &validation,
                    &transaction,
                    tool_result.as_ref(),
                    provider_output_json.as_deref(),
                    finished_at_unix_ms,
                    false,
                ),
            },
            None => Err(HeptaError(
                "terminal tool dispatch lost exact execution authority".into(),
            )),
        };
        drop(captured.disarm());
        result
    }

    pub(super) fn finalize_dropped_dispatch(&self, captured: &mut CapturedToolExecution<'_>) {
        let terminal = captured.terminal().clone();
        let validation = captured.validation().clone();
        let transaction = captured.transaction().clone();
        let tool_result = captured.tool_result().cloned();
        let provider_output_json = captured.provider_output_json().map(str::to_string);
        let staged_terminal = captured.staged_terminal().cloned();
        let _ = match captured.execution_mut() {
            Some(execution) => match staged_terminal {
                Some(exact) => record_first_outcome(execution, exact),
                None => Self::record_terminal(
                    execution,
                    &terminal,
                    &validation,
                    &transaction,
                    tool_result.as_ref(),
                    provider_output_json.as_deref(),
                    now_ms(),
                    true,
                ),
            },
            None => Ok(()),
        };
        drop(captured.disarm());
    }

    pub(super) fn finalize_authorized_drop(execution: &mut AuthorizedToolExecution) {
        let transaction = if !execution.prepared_write_transactions().is_empty() {
            CapturedTransaction::Failed {
                error: "dispatch dropped before write transaction capture".into(),
                transaction_id: None,
                group_id: None,
                entry_hash: None,
            }
        } else {
            CapturedTransaction::NotApplicable
        };
        let _ = Self::record_terminal(
            execution,
            &CapturedDispatchTerminal::Succeeded,
            &CapturedValidation::NotRequired,
            &transaction,
            None,
            None,
            now_ms(),
            true,
        );
    }

    /// Builds and atomically stages the exact terminal material paired with a
    /// provider effect ACK. Non-effect executions intentionally remain on the
    /// normal terminal path.
    pub(crate) fn stage_provider_completion(
        &self,
        captured: &mut CapturedToolExecution<'_>,
    ) -> Result<(), HeptaError> {
        let Some(execution) = captured.execution() else {
            return Err(HeptaError(
                "provider completion lost exact execution authority".into(),
            ));
        };
        if execution.execution_effect_ack().is_none() {
            return Ok(());
        }
        let exact = Self::build_exact_terminal(
            execution,
            captured.terminal(),
            captured.validation(),
            captured.transaction(),
            captured.tool_result(),
            captured.provider_output_json(),
            captured.finished_at_unix_ms(),
            false,
        )?;
        execution.stage_provider_completion(&exact)?;
        captured.set_staged_terminal(exact);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn record_terminal(
        execution: &mut AuthorizedToolExecution,
        terminal: &CapturedDispatchTerminal,
        validation: &CapturedValidation,
        transaction: &CapturedTransaction,
        tool_result: Option<&hepta_core::ToolResult>,
        provider_output_json: Option<&str>,
        finished_at_unix_ms: u64,
        dropped: bool,
    ) -> Result<(), HeptaError> {
        match Self::build_exact_terminal(
            execution,
            terminal,
            validation,
            transaction,
            tool_result,
            provider_output_json,
            finished_at_unix_ms,
            dropped,
        ) {
            Ok(exact) => record_first_outcome(execution, exact),
            Err(error) => fail_fatal(execution, error.0),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn build_exact_terminal(
        execution: &AuthorizedToolExecution,
        terminal: &CapturedDispatchTerminal,
        validation: &CapturedValidation,
        transaction: &CapturedTransaction,
        tool_result: Option<&hepta_core::ToolResult>,
        provider_output_json: Option<&str>,
        finished_at_unix_ms: u64,
        dropped: bool,
    ) -> Result<ExactOutcomeRecord, HeptaError> {
        let executor = ToolExecutorBinding::try_new(
            execution.capability().clone(),
            execution.executor().principal().clone(),
            execution.executor().provider(),
            execution.executor().manifest_hash().clone(),
        )
        .map_err(|error| {
            HeptaError(format!(
                "failed to bind exact tool outcome executor: {error}"
            ))
        });
        let finalized = executor.and_then(|executor| {
            finalize_tool_outcome(ToolOutcomeFinalizationInput {
                attempt_id: execution.attempt_id(),
                authorization: execution.authorization(),
                session_id: &execution.session_id().0,
                correlation_id: &execution.correlation_id().0,
                tool_name: execution.tool_name(),
                operation: execution.executor().operation(),
                executor: &executor,
                payload_hash: execution.payload_hash(),
                execution_idempotency_key: execution.idempotency_key(),
                execution_resource_summary_hash: execution
                    .execution_intent()
                    .map(hepta_memory::ExecutionIntent::resource_summary_hash),
                execution_effect_plan_hash: execution
                    .execution_intent()
                    .and_then(hepta_memory::ExecutionIntent::effect_plan_hash),
                execution_effect_ack_hash: execution
                    .execution_effect_ack()
                    .map(hepta_memory::ExecutionEffectAck::ack_hash),
                started_at_unix_ms: execution.started_at_unix_ms(),
                finished_at_unix_ms: finished_at_unix_ms.max(execution.started_at_unix_ms()),
                terminal: terminal_evidence(terminal, dropped),
                effect: effect_evidence(execution, transaction, dropped),
                validation: validation_evidence(validation),
                content: tool_result
                    .map(|result| OutcomeMaterial::Raw(result.content.as_str()))
                    .unwrap_or(OutcomeMaterial::Absent),
                provider_output: provider_output_json
                    .map(OutcomeMaterial::Raw)
                    .unwrap_or(OutcomeMaterial::Absent),
                final_output: tool_result
                    .and_then(|result| result.structured_json.as_deref())
                    .map(OutcomeMaterial::Raw)
                    .unwrap_or(OutcomeMaterial::Absent),
                transaction: transaction_evidence(execution, transaction, dropped),
            })
            .map_err(|error| {
                HeptaError(format!(
                    "failed to finalize exact tool outcome receipt: {error}"
                ))
            })
        });

        finalized.map(|finalized| {
            let (receipt, canonical_evidence, evidence_hash) = finalized.into_parts();
            ExactOutcomeRecord::new(
                execution.attempt_id(),
                receipt,
                canonical_evidence,
                evidence_hash,
            )
        })
    }

    pub(crate) async fn store_memory(
        &self,
        session_id: Option<&SessionId>,
        id_prefix: &str,
        scope: MemoryScope,
        content: String,
    ) -> Result<(), HeptaError> {
        let memory_id = {
            let existing = self
                .kernel
                .memory
                .list_memories()
                .map_err(|err| HeptaError(err.0))?;
            format!("{}-{}", id_prefix, existing.len() + 1)
        };
        self.kernel
            .memory
            .put(MemoryRecord {
                id: memory_id.clone(),
                scope,
                content,
            })
            .await
            .map_err(|e| HeptaError(e.0))?;
        self.kernel.emit_event(
            EventKind::MemoryWritten,
            session_id.cloned(),
            None,
            format!("stored memory {}", memory_id),
        )?;
        Ok(())
    }

    pub(crate) fn record_turn(&self, record: TurnRecord) -> Result<(), HeptaError> {
        let existing_entries = self
            .kernel
            .memory
            .list_transcript_entries()
            .map_err(|err| HeptaError(err.0))?;
        let next_sequence = existing_entries
            .iter()
            .filter(|entry| entry.session_id.0 == record.session_id)
            .count() as u64
            + 1;
        let now = current_unix_ms()?;

        self.kernel
            .memory
            .append_transcript_sync(TranscriptEntry {
                entry_id: format!("{}-{}-user", record.session_id, next_sequence),
                session_id: SessionId(record.session_id.clone()),
                sequence: next_sequence,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: record.input.clone(),
                created_at_unix_ms: now,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            })
            .map_err(|err| HeptaError(err.0))?;

        self.kernel
            .memory
            .append_transcript_sync(TranscriptEntry {
                entry_id: format!("{}-{}-assistant", record.session_id, next_sequence + 1),
                session_id: SessionId(record.session_id.clone()),
                sequence: next_sequence + 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::Assistant),
                content: record.final_text.clone(),
                created_at_unix_ms: now,
                tool_name: record.invoked_tool.clone(),
                correlation_id: None,
                summary_of_range: None,
            })
            .map_err(|err| HeptaError(err.0))?;

        if let Some(reason) = &record.blocked_reason {
            self.kernel
                .memory
                .append_transcript_sync(TranscriptEntry {
                    entry_id: format!("{}-{}-event", record.session_id, next_sequence + 2),
                    session_id: SessionId(record.session_id.clone()),
                    sequence: next_sequence + 2,
                    kind: TranscriptEntryKind::Event,
                    role: None,
                    content: format!("blocked_reason:{}", reason),
                    created_at_unix_ms: now,
                    tool_name: record.invoked_tool.clone(),
                    correlation_id: None,
                    summary_of_range: None,
                })
                .map_err(|err| HeptaError(err.0))?;
        }

        let mut guard = self
            .kernel
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        guard.push(record);
        Ok(())
    }
}
