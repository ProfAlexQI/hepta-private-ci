use super::*;

pub(super) fn capture_invocation_result(
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
    if let Some(provider_error) = provider_error_after_commit(result) {
        captured.terminal = CapturedDispatchTerminal::ProviderErrorAfterCommit {
            error: provider_error.error.clone(),
            error_code: provider_error.error_code,
        };
        captured.outward_error = Some(HeptaError(provider_error.error));
        return;
    }
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
