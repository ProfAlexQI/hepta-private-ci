//! Canonical terminal tool outcomes and immutable V2 receipts.

use std::error::Error;
use std::fmt;

use hepta_contracts::AuthorizationDecision;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::PrincipalId;
use hepta_kernel::HeptaKernelSafetyAuthorization;

mod canonical;
mod hashing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolDispatchTerminal<'a> {
    Succeeded,
    DispatchBlocked { reason: &'a str },
    ToolError { error: &'a str },
    StructuredOutputMissing,
    OutputValidationFailed { error: &'a str },
    ToolReportedFailure { error: &'a str },
    ProviderErrorAfterCommit { error: &'a str, error_code: &'a str },
    TimedOut { timeout_ms: u64, error: &'a str },
    EventRecordingFailed { error: &'a str },
    TransactionRecordingFailed { error: &'a str },
    DispatchFutureDropped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolEffectDisposition {
    None,
    Recorded,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolOutputValidationStatus<'a> {
    NotRequired,
    Missing,
    Valid,
    Invalid { error: &'a str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolTransactionEvidence<'a> {
    NotApplicable,
    Preview,
    Recorded {
        transaction_id: &'a str,
        group_id: Option<&'a str>,
        entry_hash: &'a ContentHash,
    },
    Failed {
        error: &'a str,
        transaction_id: Option<&'a str>,
        group_id: Option<&'a str>,
        entry_hash: Option<&'a ContentHash>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutcomeMaterial<'a> {
    Absent,
    Raw(&'a str),
    #[cfg(test)]
    Hashed(&'a ContentHash),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ToolExecutorBinding {
    capability: CapabilityManifestRef,
    executor: PrincipalId,
    provider: String,
    executor_manifest_hash: ContentHash,
    binding_hash: ContentHash,
}

impl ToolExecutorBinding {
    pub(crate) fn try_new(
        capability: CapabilityManifestRef,
        executor: PrincipalId,
        provider: impl Into<String>,
        executor_manifest_hash: ContentHash,
    ) -> Result<Self, ToolOutcomeFinalizationError> {
        let provider = provider.into();
        for (name, value) in [
            ("capability_id", capability.id().as_str()),
            (
                "capability_manifest_hash",
                capability.manifest_hash().as_str(),
            ),
            ("executor", executor.as_str()),
            ("executor_provider", provider.as_str()),
            ("executor_manifest_hash", executor_manifest_hash.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ToolOutcomeFinalizationError::EmptyField(name));
            }
        }
        if executor_manifest_hash != *capability.manifest_hash() {
            return Err(ToolOutcomeFinalizationError::ExecutorBindingMismatch);
        }
        let binding_hash = hashing::executor_binding_hash(
            &capability,
            &executor,
            &provider,
            &executor_manifest_hash,
        );
        Ok(Self {
            capability,
            executor,
            provider,
            executor_manifest_hash,
            binding_hash,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ToolOutcomeFinalizationInput<'a> {
    pub(crate) attempt_id: &'a str,
    pub(crate) authorization: &'a HeptaKernelSafetyAuthorization,
    pub(crate) session_id: &'a str,
    pub(crate) correlation_id: &'a str,
    pub(crate) tool_name: &'a str,
    pub(crate) operation: &'a str,
    pub(crate) executor: &'a ToolExecutorBinding,
    pub(crate) payload_hash: &'a ContentHash,
    pub(crate) execution_idempotency_key: Option<&'a str>,
    pub(crate) execution_resource_summary_hash: Option<&'a ContentHash>,
    pub(crate) execution_effect_plan_hash: Option<&'a ContentHash>,
    pub(crate) execution_effect_ack_hash: Option<&'a ContentHash>,
    pub(crate) started_at_unix_ms: u64,
    pub(crate) finished_at_unix_ms: u64,
    pub(crate) terminal: ToolDispatchTerminal<'a>,
    pub(crate) effect: ToolEffectDisposition,
    pub(crate) validation: ToolOutputValidationStatus<'a>,
    pub(crate) content: OutcomeMaterial<'a>,
    pub(crate) provider_output: OutcomeMaterial<'a>,
    pub(crate) final_output: OutcomeMaterial<'a>,
    pub(crate) transaction: ToolTransactionEvidence<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FinalizedToolOutcome {
    receipt: OutcomeReceipt,
    canonical_evidence: String,
    evidence_hash: ContentHash,
}

impl FinalizedToolOutcome {
    #[cfg(test)]
    pub(crate) fn receipt(&self) -> &OutcomeReceipt {
        &self.receipt
    }

    #[cfg(test)]
    pub(crate) fn canonical_evidence(&self) -> &str {
        &self.canonical_evidence
    }

    #[cfg(test)]
    pub(crate) fn evidence_hash(&self) -> &ContentHash {
        &self.evidence_hash
    }

    pub(crate) fn into_parts(self) -> (OutcomeReceipt, String, ContentHash) {
        (self.receipt, self.canonical_evidence, self.evidence_hash)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolOutcomeFinalizationError {
    EmptyField(&'static str),
    InvalidTimeRange,
    PayloadBindingMismatch,
    CapabilityBindingMismatch,
    ExecutorBindingMismatch,
    InconsistentTerminalEvidence,
    AuthorizationNotAuthorized,
    EvidenceSerialization,
    ReceiptConstruction,
}

impl fmt::Display for ToolOutcomeFinalizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "tool outcome field {field} is empty"),
            Self::InvalidTimeRange => formatter.write_str("tool outcome time range is invalid"),
            Self::PayloadBindingMismatch => {
                formatter.write_str("tool outcome payload differs from authorization")
            }
            Self::CapabilityBindingMismatch => {
                formatter.write_str("tool outcome capability differs from frozen catalog")
            }
            Self::ExecutorBindingMismatch => {
                formatter.write_str("tool outcome executor differs from capability provider")
            }
            Self::InconsistentTerminalEvidence => {
                formatter.write_str("tool outcome terminal evidence is inconsistent")
            }
            Self::AuthorizationNotAuthorized => {
                formatter.write_str("tool outcome witness is not authorized")
            }
            Self::EvidenceSerialization => {
                formatter.write_str("canonical tool outcome evidence serialization failed")
            }
            Self::ReceiptConstruction => formatter.write_str("outcome receipt construction failed"),
        }
    }
}

impl Error for ToolOutcomeFinalizationError {}

pub(crate) fn finalize_tool_outcome(
    input: ToolOutcomeFinalizationInput<'_>,
) -> Result<FinalizedToolOutcome, ToolOutcomeFinalizationError> {
    validate_input(&input)?;
    let authorization = input.authorization.authorization();
    let scope_hash = match authorization.decision() {
        AuthorizationDecision::Authorized { scope_hash } => scope_hash,
        _ => return Err(ToolOutcomeFinalizationError::AuthorizationNotAuthorized),
    };
    let status = outcome_status(input.terminal);
    let (canonical_evidence, evidence_hash) =
        canonical::build_canonical_evidence(&input, scope_hash)?;
    let authorization_ref = authorization.reference();
    let receipt_id = hashing::receipt_id(input.attempt_id, &authorization_ref, &evidence_hash);
    let receipt_hash = hashing::receipt_hash(
        &receipt_id,
        authorization.candidate(),
        &authorization_ref,
        &input.executor.capability,
        input.payload_hash,
        input.executor,
        &status,
        &evidence_hash,
    );
    let receipt = OutcomeReceipt::try_new(
        receipt_id,
        receipt_hash,
        authorization,
        input.executor.executor.clone(),
        evidence_hash.clone(),
        status,
    )
    .map_err(|_| ToolOutcomeFinalizationError::ReceiptConstruction)?;
    Ok(FinalizedToolOutcome {
        receipt,
        canonical_evidence,
        evidence_hash,
    })
}

fn validate_input(
    input: &ToolOutcomeFinalizationInput<'_>,
) -> Result<(), ToolOutcomeFinalizationError> {
    for (name, value) in [
        ("attempt_id", input.attempt_id),
        ("session_id", input.session_id),
        ("correlation_id", input.correlation_id),
        ("tool_name", input.tool_name),
        ("operation", input.operation),
    ] {
        if value.trim().is_empty() {
            return Err(ToolOutcomeFinalizationError::EmptyField(name));
        }
    }
    if input.finished_at_unix_ms < input.started_at_unix_ms {
        return Err(ToolOutcomeFinalizationError::InvalidTimeRange);
    }
    if input.payload_hash != input.authorization.authorization().payload_set_hash()
        || input.payload_hash != input.authorization.binding().payload_set_hash()
    {
        return Err(ToolOutcomeFinalizationError::PayloadBindingMismatch);
    }
    let context = input.authorization.authorization().candidate().context();
    if input.executor.capability.catalog() != context.capability_catalog() {
        return Err(ToolOutcomeFinalizationError::CapabilityBindingMismatch);
    }
    let expected_executor = hashing::executor_binding_hash(
        &input.executor.capability,
        &input.executor.executor,
        &input.executor.provider,
        &input.executor.executor_manifest_hash,
    );
    if expected_executor != input.executor.binding_hash {
        return Err(ToolOutcomeFinalizationError::ExecutorBindingMismatch);
    }
    let consistent = match input.terminal {
        ToolDispatchTerminal::Succeeded => {
            !matches!(
                input.validation,
                ToolOutputValidationStatus::Missing | ToolOutputValidationStatus::Invalid { .. }
            ) && !matches!(input.transaction, ToolTransactionEvidence::Failed { .. })
        }
        ToolDispatchTerminal::StructuredOutputMissing => {
            matches!(input.validation, ToolOutputValidationStatus::Missing)
        }
        ToolDispatchTerminal::OutputValidationFailed { .. } => {
            matches!(input.validation, ToolOutputValidationStatus::Invalid { .. })
        }
        ToolDispatchTerminal::ProviderErrorAfterCommit { error_code, .. } => {
            !error_code.trim().is_empty()
        }
        ToolDispatchTerminal::TransactionRecordingFailed { .. } => {
            matches!(input.transaction, ToolTransactionEvidence::Failed { .. })
        }
        ToolDispatchTerminal::TimedOut { timeout_ms: 0, .. } => false,
        _ => true,
    };
    let transaction_is_recorded = match input.transaction {
        ToolTransactionEvidence::Recorded {
            transaction_id,
            entry_hash,
            ..
        } => !transaction_id.trim().is_empty() && !entry_hash.as_str().trim().is_empty(),
        _ => false,
    };
    let effect_is_recorded = matches!(input.effect, ToolEffectDisposition::Recorded);
    if !consistent || transaction_is_recorded != effect_is_recorded {
        return Err(ToolOutcomeFinalizationError::InconsistentTerminalEvidence);
    }
    Ok(())
}

fn outcome_status(terminal: ToolDispatchTerminal<'_>) -> OutcomeStatus {
    let failed = |code: &str| OutcomeStatus::Failed {
        error_code: code.into(),
    };
    match terminal {
        ToolDispatchTerminal::Succeeded => OutcomeStatus::Succeeded,
        ToolDispatchTerminal::DispatchBlocked { .. } => {
            failed("runtime.outcome_receipt_breaker_open")
        }
        ToolDispatchTerminal::ToolError { .. } => failed("tool.invoke_error"),
        ToolDispatchTerminal::StructuredOutputMissing => failed("tool.output_missing"),
        ToolDispatchTerminal::OutputValidationFailed { .. } => {
            failed("tool.output_validation_failed")
        }
        ToolDispatchTerminal::ToolReportedFailure { .. } => failed("tool.reported_failure"),
        ToolDispatchTerminal::ProviderErrorAfterCommit { error_code, .. } => failed(error_code),
        ToolDispatchTerminal::TimedOut { .. } => OutcomeStatus::Cancelled {
            reason_code: "tool.native_timeout".into(),
        },
        ToolDispatchTerminal::EventRecordingFailed { .. } => {
            failed("runtime.tool_invoked_event_failed")
        }
        ToolDispatchTerminal::TransactionRecordingFailed { .. } => {
            failed("runtime.write_transaction_record_failed")
        }
        ToolDispatchTerminal::DispatchFutureDropped => OutcomeStatus::Cancelled {
            reason_code: "tool.dispatch_future_dropped".into(),
        },
    }
}
