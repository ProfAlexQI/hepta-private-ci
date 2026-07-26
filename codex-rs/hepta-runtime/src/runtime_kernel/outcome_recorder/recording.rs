use super::*;

pub(super) fn terminal_evidence<'a>(
    terminal: &'a CapturedDispatchTerminal,
    dropped: bool,
) -> ToolDispatchTerminal<'a> {
    if dropped {
        return ToolDispatchTerminal::DispatchFutureDropped;
    }
    match terminal {
        CapturedDispatchTerminal::Succeeded => ToolDispatchTerminal::Succeeded,
        CapturedDispatchTerminal::DispatchBlocked(reason) => {
            ToolDispatchTerminal::DispatchBlocked { reason }
        }
        CapturedDispatchTerminal::ToolError(error) => ToolDispatchTerminal::ToolError { error },
        CapturedDispatchTerminal::StructuredOutputMissing => {
            ToolDispatchTerminal::StructuredOutputMissing
        }
        CapturedDispatchTerminal::OutputValidationFailed(error) => {
            ToolDispatchTerminal::OutputValidationFailed { error }
        }
        CapturedDispatchTerminal::ToolReportedFailure(error) => {
            ToolDispatchTerminal::ToolReportedFailure { error }
        }
        CapturedDispatchTerminal::ProviderErrorAfterCommit { error, error_code } => {
            ToolDispatchTerminal::ProviderErrorAfterCommit { error, error_code }
        }
        CapturedDispatchTerminal::TimedOut { timeout_ms, error } => {
            ToolDispatchTerminal::TimedOut {
                timeout_ms: *timeout_ms,
                error,
            }
        }
        CapturedDispatchTerminal::EventRecordingFailed(error) => {
            ToolDispatchTerminal::EventRecordingFailed { error }
        }
        CapturedDispatchTerminal::WriteTransactionFailed(error) => {
            ToolDispatchTerminal::TransactionRecordingFailed { error }
        }
    }
}

pub(super) fn validation_evidence(
    validation: &CapturedValidation,
) -> ToolOutputValidationStatus<'_> {
    match validation {
        CapturedValidation::NotRequired => ToolOutputValidationStatus::NotRequired,
        CapturedValidation::Missing => ToolOutputValidationStatus::Missing,
        CapturedValidation::Valid => ToolOutputValidationStatus::Valid,
        CapturedValidation::Invalid(error) => ToolOutputValidationStatus::Invalid { error },
    }
}

pub(super) fn transaction_evidence<'a>(
    execution: &AuthorizedToolExecution,
    transaction: &'a CapturedTransaction,
    dropped: bool,
) -> ToolTransactionEvidence<'a> {
    if dropped
        && !execution.prepared_write_transactions().is_empty()
        && matches!(transaction, CapturedTransaction::NotApplicable)
    {
        return ToolTransactionEvidence::Failed {
            error: "dispatch dropped before write transaction capture",
            transaction_id: None,
            group_id: None,
            entry_hash: None,
        };
    }
    match transaction {
        CapturedTransaction::NotApplicable => ToolTransactionEvidence::NotApplicable,
        CapturedTransaction::Preview => ToolTransactionEvidence::Preview,
        CapturedTransaction::Recorded {
            transaction_id,
            group_id,
            entry_hash,
        } => ToolTransactionEvidence::Recorded {
            transaction_id,
            group_id: group_id.as_deref(),
            entry_hash,
        },
        CapturedTransaction::Failed {
            error,
            transaction_id,
            group_id,
            entry_hash,
        } => ToolTransactionEvidence::Failed {
            error,
            transaction_id: transaction_id.as_deref(),
            group_id: group_id.as_deref(),
            entry_hash: entry_hash.as_ref(),
        },
    }
}

pub(super) fn effect_evidence(
    execution: &AuthorizedToolExecution,
    transaction: &CapturedTransaction,
    dropped: bool,
) -> ToolEffectDisposition {
    if execution.metadata().read_only || matches!(transaction, CapturedTransaction::Preview) {
        return ToolEffectDisposition::None;
    }
    if dropped {
        return ToolEffectDisposition::Unknown;
    }
    if matches!(transaction, CapturedTransaction::Recorded { .. }) {
        ToolEffectDisposition::Recorded
    } else {
        ToolEffectDisposition::Unknown
    }
}
