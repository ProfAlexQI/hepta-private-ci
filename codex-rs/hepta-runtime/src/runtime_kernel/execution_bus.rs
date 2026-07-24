//! Single-use tool dispatch and complete terminal-result capture.

mod result_classification;

use std::future::Future;

use hepta_contracts::ContentHash;
use hepta_core::EventKind;
use hepta_core::HeptaError;
use hepta_core::ToolCallRequest;
use hepta_core::ToolContext;
use hepta_core::ToolResult;

use super::context_freezer::now_ms;
use super::execution_attempt::AuthorizedToolExecution;
use super::outcome_recorder::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::tool_result_is_timeout;
use result_classification::timeout_ms;
use result_classification::tool_reported_failure;

/// Owned terminal classification retained until its receipt is durable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CapturedDispatchTerminal {
    Succeeded,
    DispatchBlocked(String),
    ToolError(String),
    StructuredOutputMissing,
    OutputValidationFailed(String),
    ToolReportedFailure(String),
    TimedOut { timeout_ms: u64, error: String },
    EventRecordingFailed(String),
    WriteTransactionFailed(String),
}

/// Owned structured-output validation evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CapturedValidation {
    NotRequired,
    Missing,
    Valid,
    Invalid(String),
}

/// Owned write-transaction evidence retained until receipt finalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CapturedTransaction {
    NotApplicable,
    Preview,
    Recorded {
        transaction_id: String,
        group_id: Option<String>,
        entry_hash: ContentHash,
    },
    Failed {
        error: String,
        transaction_id: Option<String>,
        group_id: Option<String>,
        entry_hash: Option<ContentHash>,
    },
}

/// Successful result of attempting the write-transaction recording boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapturedTransactionResult {
    pub(crate) final_output_json: Option<String>,
    pub(crate) evidence: CapturedTransaction,
}

/// Terminal dispatch material guarded until OutcomeRecorder disarms it.
pub(crate) struct CapturedToolExecution<'a> {
    pub(super) kernel: &'a RuntimeKernel,
    pub(super) execution: Option<AuthorizedToolExecution>,
    terminal: CapturedDispatchTerminal,
    validation: CapturedValidation,
    transaction: CapturedTransaction,
    tool_result: Option<ToolResult>,
    provider_output_json: Option<String>,
    outward_error: Option<HeptaError>,
    finished_at_unix_ms: u64,
    provider_invocation_started: bool,
    provider_invocation_completed: bool,
}

impl<'a> CapturedToolExecution<'a> {
    fn armed(kernel: &'a RuntimeKernel, execution: AuthorizedToolExecution) -> Self {
        Self {
            kernel,
            execution: Some(execution),
            terminal: CapturedDispatchTerminal::Succeeded,
            validation: CapturedValidation::NotRequired,
            transaction: CapturedTransaction::NotApplicable,
            tool_result: None,
            provider_output_json: None,
            outward_error: None,
            finished_at_unix_ms: now_ms(),
            provider_invocation_started: false,
            provider_invocation_completed: false,
        }
    }

    #[cfg(test)]
    pub(crate) fn attempt_id(&self) -> &str {
        self.execution()
            .map(AuthorizedToolExecution::attempt_id)
            .unwrap_or("")
    }

    #[cfg(test)]
    pub(crate) fn simulate_process_loss_after_provider_for_test(mut self) {
        if let Some(mut execution) = self.disarm() {
            execution.disarm_receipt_guard();
            execution.release_execution_lease();
        }
    }

    pub(crate) fn terminal(&self) -> &CapturedDispatchTerminal {
        &self.terminal
    }

    pub(crate) fn validation(&self) -> &CapturedValidation {
        &self.validation
    }

    pub(crate) fn transaction(&self) -> &CapturedTransaction {
        &self.transaction
    }

    pub(crate) fn tool_result(&self) -> Option<&ToolResult> {
        self.tool_result.as_ref()
    }

    pub(crate) fn provider_output_json(&self) -> Option<&str> {
        self.provider_output_json.as_deref()
    }

    pub(crate) fn outward_error(&self) -> Option<&HeptaError> {
        self.outward_error.as_ref()
    }

    pub(crate) const fn finished_at_unix_ms(&self) -> u64 {
        self.finished_at_unix_ms
    }

    pub(crate) fn record_transaction(
        &mut self,
        transaction: CapturedTransaction,
        final_output_json: Option<String>,
    ) {
        if let CapturedTransaction::Failed { error, .. } = &transaction {
            if matches!(self.terminal, CapturedDispatchTerminal::Succeeded) {
                self.terminal = CapturedDispatchTerminal::WriteTransactionFailed(error.clone());
            }
            if self.outward_error.is_none() {
                self.outward_error = Some(HeptaError(error.clone()));
            }
        }
        self.transaction = transaction;
        if let Some(result) = self.tool_result.as_mut() {
            result.structured_json = final_output_json;
        }
        self.finished_at_unix_ms = now_ms().max(self.finished_at_unix_ms);
    }

    /// Captures transaction evidence while the authorized execution still owns
    /// any tool-specific reservation sealed into its resource-bound lease.
    pub(crate) fn capture_write_transaction(&mut self) {
        let tool_output_json = self
            .tool_result()
            .and_then(|result| result.structured_json.clone());
        let transaction_result = match self.execution() {
            Some(execution) => self.kernel.record_mutation_transactions_from_tool_result(
                execution.session_id(),
                execution.prepared_write_transactions(),
                tool_output_json.clone(),
                execution
                    .execution_intent()
                    .and_then(hepta_memory::ExecutionIntent::effect_plan_hash),
                execution
                    .execution_effect_ack()
                    .map(hepta_memory::ExecutionEffectAck::ack_hash),
            ),
            None => Err(HeptaError(
                "write transaction capture lost exact execution authority".into(),
            )),
        };
        match transaction_result {
            Ok(transaction) => {
                self.record_transaction(transaction.evidence, transaction.final_output_json);
            }
            Err(error) => {
                self.record_transaction(
                    CapturedTransaction::Failed {
                        error: error.0,
                        transaction_id: None,
                        group_id: None,
                        entry_hash: None,
                    },
                    tool_output_json,
                );
            }
        }
    }

    pub(super) fn execution(&self) -> Option<&AuthorizedToolExecution> {
        self.execution.as_ref()
    }

    pub(super) fn execution_mut(&mut self) -> Option<&mut AuthorizedToolExecution> {
        self.execution.as_mut()
    }

    pub(super) fn disarm(&mut self) -> Option<AuthorizedToolExecution> {
        self.execution.take()
    }
}

impl Drop for CapturedToolExecution<'_> {
    fn drop(&mut self) {
        if self.provider_invocation_started && !self.provider_invocation_completed {
            self.kernel.trip_outcome_breaker(
                "provider completion is unknown; durable execution intent requires reconciliation",
            );
            if let Some(execution) = self.execution.as_mut() {
                execution.disarm_receipt_guard();
                execution.release_execution_lease();
            }
            return;
        }
        if self.execution.is_some() {
            OutcomeRecorder::new(self.kernel).finalize_dropped_dispatch(self);
        }
    }
}

/// Private execution seam: its only dispatch input is sealed exact authority.
pub(crate) struct ExecutionBus<'a> {
    kernel: &'a RuntimeKernel,
}

impl<'a> ExecutionBus<'a> {
    pub(crate) fn new(kernel: &'a RuntimeKernel) -> Self {
        Self { kernel }
    }

    /// Starts one dispatch guard synchronously, then returns its async work.
    ///
    /// Constructing the guard before the future means dropping an unpolled,
    /// pending, or completed-but-unfinalized future all produce cancellation
    /// evidence through the guard's synchronous `Drop`.
    pub(crate) fn dispatch(
        &self,
        execution: AuthorizedToolExecution,
    ) -> impl Future<Output = CapturedToolExecution<'a>> + 'a {
        let kernel = self.kernel;
        let mut captured = CapturedToolExecution::armed(kernel, execution);
        async move {
            if let Err(error) = kernel.ensure_outcome_dispatch_open() {
                captured.terminal = CapturedDispatchTerminal::DispatchBlocked(error.0.clone());
                captured.outward_error = Some(error);
                captured.finished_at_unix_ms = now_ms();
                if let Some(execution) = captured.execution_mut() {
                    execution.release_execution_lease();
                }
                return captured;
            }
            let selector_validation = captured
                .execution()
                .ok_or_else(|| {
                    HeptaError(
                        "execution guard lost exact authority before selector binding".into(),
                    )
                })
                .and_then(|execution| execution.validate_dispatch_selector(kernel));
            if let Err(error) = selector_validation {
                captured.terminal = CapturedDispatchTerminal::DispatchBlocked(error.0.clone());
                captured.outward_error = Some(error);
                captured.finished_at_unix_ms = now_ms();
                if let Some(execution) = captured.execution_mut() {
                    execution.release_execution_lease();
                }
                return captured;
            }
            let intent_staging = captured
                .execution_mut()
                .ok_or_else(|| {
                    HeptaError(
                        "execution guard lost exact authority before durable intent staging".into(),
                    )
                })
                .and_then(AuthorizedToolExecution::stage_execution_intent);
            if let Err(error) = intent_staging {
                captured.terminal = CapturedDispatchTerminal::DispatchBlocked(error.0.clone());
                captured.outward_error = Some(error);
                captured.finished_at_unix_ms = now_ms();
                if let Some(execution) = captured.execution_mut() {
                    execution.release_execution_lease();
                }
                return captured;
            }
            let (tool_name, input_json, session_id, correlation_id, idempotency_key) = {
                let Some(execution) = captured.execution() else {
                    kernel.trip_outcome_breaker(
                        "execution guard lost exact authority before dispatch",
                    );
                    return captured;
                };
                let Some(idempotency_key) = execution.idempotency_key() else {
                    kernel.trip_outcome_breaker(
                        "execution guard lost durable idempotency binding before dispatch",
                    );
                    return captured;
                };
                (
                    execution.tool_name().to_string(),
                    execution.canonical_arguments().to_string(),
                    execution.session_id().clone(),
                    execution.correlation_id().clone(),
                    idempotency_key.to_owned(),
                )
            };

            captured.provider_invocation_started = true;
            let invoked = {
                let Some(execution) = captured.execution() else {
                    kernel.trip_outcome_breaker(
                        "execution guard lost exact authority during selector dispatch",
                    );
                    return captured;
                };
                let prepared_writes = execution.prepared_write_transactions();
                let prepared_read = execution.prepared_read_capability();
                let Some(execution_intent) = execution.execution_intent() else {
                    kernel.trip_outcome_breaker(
                        "execution guard lost staged intent during provider dispatch",
                    );
                    return captured;
                };
                kernel
                    .tools
                    .invoke_authorized(
                        &tool_name,
                        ToolContext {
                            session_id: Some(session_id.clone()),
                            correlation_id: Some(correlation_id.clone()),
                            execution_attempt_id: Some(execution.attempt_id().to_owned()),
                            idempotency_key: Some(idempotency_key.clone()),
                        },
                        ToolCallRequest {
                            name: tool_name.clone(),
                            input_json,
                        },
                        prepared_writes,
                        prepared_read,
                        execution.executor(),
                        execution.capability(),
                        execution.attempt_id(),
                        &idempotency_key,
                        execution_intent,
                        execution.outcome_sink(),
                    )
                    .await
            };
            captured.provider_invocation_completed = true;
            let effect_confirmation = captured
                .execution_mut()
                .ok_or_else(|| {
                    HeptaError(
                        "execution guard lost exact authority before provider effect confirmation"
                            .into(),
                    )
                })
                .and_then(AuthorizedToolExecution::confirm_provider_effect_ack);
            if let Err(error) = effect_confirmation {
                let reason = format!(
                    "provider effect is in doubt and requires reconciliation: {}",
                    error.0
                );
                captured.terminal = CapturedDispatchTerminal::ToolError(reason.clone());
                captured.outward_error = Some(HeptaError(reason.clone()));
                captured.finished_at_unix_ms = now_ms();
                if let Some(mut execution) = captured.disarm() {
                    execution.trip_outcome_breaker(reason);
                    execution.disarm_receipt_guard();
                    execution.release_execution_lease();
                }
                return captured;
            }

            capture_invocation_result(
                kernel,
                &mut captured,
                &tool_name,
                session_id,
                correlation_id,
                invoked,
            );
            captured.finished_at_unix_ms = now_ms().max(
                captured
                    .execution()
                    .map(AuthorizedToolExecution::started_at_unix_ms)
                    .unwrap_or_default(),
            );
            if let Some(execution) = captured.execution_mut() {
                execution.release_execution_lease();
            }
            captured
        }
    }

    #[cfg(test)]
    pub(crate) fn capture_invocation_for_test(
        &self,
        execution: AuthorizedToolExecution,
        invoked: Result<ToolResult, HeptaError>,
    ) -> CapturedToolExecution<'a> {
        let mut captured = CapturedToolExecution::armed(self.kernel, execution);
        let (tool_name, session_id, correlation_id) = match captured.execution() {
            Some(execution) => (
                execution.tool_name().to_string(),
                execution.session_id().clone(),
                execution.correlation_id().clone(),
            ),
            None => return captured,
        };
        capture_invocation_result(
            self.kernel,
            &mut captured,
            &tool_name,
            session_id,
            correlation_id,
            invoked,
        );
        captured.finished_at_unix_ms = now_ms();
        if let Some(execution) = captured.execution_mut() {
            execution.release_execution_lease();
        }
        captured
    }
}

fn capture_invocation_result(
    kernel: &RuntimeKernel,
    captured: &mut CapturedToolExecution<'_>,
    tool_name: &str,
    session_id: hepta_core::SessionId,
    correlation_id: hepta_core::CorrelationId,
    invoked: Result<ToolResult, HeptaError>,
) {
    match invoked {
        Err(error) => {
            captured.terminal = CapturedDispatchTerminal::ToolError(error.0.clone());
            captured.outward_error = Some(error);
        }
        Ok(result) => {
            captured.provider_output_json = result.structured_json.clone();
            captured.tool_result = Some(result);
            let timed_out = captured
                .tool_result
                .as_ref()
                .is_some_and(|result| tool_result_is_timeout(tool_name, result));
            capture_output_validation(kernel, tool_name, captured);
            if timed_out {
                capture_provider_terminal(tool_name, captured);
            } else if !matches!(
                captured.terminal,
                CapturedDispatchTerminal::StructuredOutputMissing
                    | CapturedDispatchTerminal::OutputValidationFailed(_)
            ) {
                capture_provider_terminal(tool_name, captured);
            }
            if timed_out
                || !matches!(
                    captured.terminal,
                    CapturedDispatchTerminal::StructuredOutputMissing
                        | CapturedDispatchTerminal::OutputValidationFailed(_)
                )
            {
                if let Err(error) = kernel.emit_event(
                    EventKind::ToolInvoked,
                    Some(session_id),
                    Some(correlation_id),
                    format!("invoked tool {tool_name}"),
                ) {
                    captured.terminal =
                        CapturedDispatchTerminal::EventRecordingFailed(error.0.clone());
                    captured.outward_error = Some(error);
                }
            }
        }
    }
}

fn capture_output_validation(
    kernel: &RuntimeKernel,
    tool_name: &str,
    captured: &mut CapturedToolExecution<'_>,
) {
    let Some(result) = captured.tool_result.as_ref() else {
        return;
    };
    let Some(metadata) = captured.execution().map(AuthorizedToolExecution::metadata) else {
        let error = "execution guard lost exact authority during output validation".to_string();
        captured.terminal = CapturedDispatchTerminal::EventRecordingFailed(error.clone());
        captured.outward_error = Some(HeptaError(error.clone()));
        captured.kernel.trip_outcome_breaker(error);
        return;
    };
    let Some(output_json) = result.structured_json.as_deref() else {
        if metadata.produces_structured_output {
            let error = format!("tool {tool_name} did not return required structured output");
            captured.validation = CapturedValidation::Missing;
            captured.terminal = CapturedDispatchTerminal::StructuredOutputMissing;
            captured.outward_error = Some(HeptaError(error));
        } else {
            captured.validation = CapturedValidation::NotRequired;
        }
        return;
    };
    match kernel.validate_tool_output(tool_name, output_json) {
        Ok(()) => captured.validation = CapturedValidation::Valid,
        Err(error) => {
            captured.validation = CapturedValidation::Invalid(error.0.clone());
            captured.terminal = CapturedDispatchTerminal::OutputValidationFailed(error.0.clone());
            captured.outward_error = Some(error);
        }
    }
}

fn capture_provider_terminal(tool_name: &str, captured: &mut CapturedToolExecution<'_>) {
    let Some(result) = captured.tool_result.as_ref() else {
        return;
    };
    if tool_result_is_timeout(tool_name, result) {
        if let Some(timeout_ms) = timeout_ms(tool_name, result) {
            captured.terminal = CapturedDispatchTerminal::TimedOut {
                timeout_ms,
                error: result.content.clone(),
            };
            captured.outward_error = None;
        } else if !matches!(
            captured.terminal,
            CapturedDispatchTerminal::StructuredOutputMissing
                | CapturedDispatchTerminal::OutputValidationFailed(_)
        ) {
            captured.terminal = CapturedDispatchTerminal::ToolReportedFailure(
                "timeout result omitted a non-zero exact duration".into(),
            );
            captured.outward_error = None;
        }
        return;
    }
    if let Some(error) = tool_reported_failure(result) {
        captured.terminal = CapturedDispatchTerminal::ToolReportedFailure(error);
    }
}
