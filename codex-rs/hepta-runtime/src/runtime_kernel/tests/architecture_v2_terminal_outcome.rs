use crate::runtime_kernel::terminal_outcome::*;
use hepta_contracts::AdmissionId;
use hepta_contracts::AuthorizationId;
use hepta_contracts::CandidateId;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityRequest;
use hepta_contracts::CapabilityRequestId;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationSnapshot;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_core::ApprovalRequirement;
use hepta_core::PolicyDecision;
use hepta_core::PolicyRule;
use hepta_core::RiskTier;
use hepta_kernel::HeptaKernelAdmissionEvidence;
use hepta_kernel::HeptaKernelPolicyEvidence;
use hepta_kernel::HeptaKernelSafetyAuthorization;
use hepta_kernel::HeptaKernelSafetyGate;

#[test]
fn architecture_v2_outcome_receipt_has_golden_framing_and_redacts_raw_material() {
    let fixture = authorization("base", "sha256:state");
    let executor = executor(&fixture.capability, "runtime:tool-host", "builtin");
    let payload = fixture
        .authorization
        .authorization()
        .payload_set_hash()
        .clone();
    let entry_hash = ContentHash::new("sha256:transaction-entry");
    let finalized = finalize_tool_outcome(base_input(
        &fixture.authorization,
        &executor,
        &payload,
        &entry_hash,
    ))
    .expect("receipt finalization");

    assert_eq!(
        finalized.evidence_hash().as_str(),
        "sha256:6322735cd97cb97e4ccfe672b04090b4f9b63b1dcac38cc115cabbe8e3c4b484"
    );
    assert_eq!(
        finalized.receipt().outcome_hash(),
        finalized.evidence_hash()
    );
    assert_eq!(
        finalized.receipt().receipt_hash().as_str(),
        "sha256:632a786f57b2f5e4976cc8e454e94ea6b3c218602142730dd06d546d0e53d433"
    );
    assert!(matches!(
        finalized.receipt().status(),
        OutcomeStatus::Succeeded
    ));
    assert!(!finalized.canonical_evidence().contains("secret-content"));
    assert!(!finalized.canonical_evidence().contains("secret-provider"));
    assert!(!finalized.canonical_evidence().contains("secret-final"));
    assert!(
        finalized
            .canonical_evidence()
            .contains("\"domain\":\"hepta.runtime.tool-outcome.v1\"")
    );
}

#[test]
fn architecture_v2_outcome_receipt_maps_every_terminal_status() {
    let fixture = authorization("status", "sha256:state");
    let executor = executor(&fixture.capability, "runtime:tool-host", "builtin");
    let payload = fixture
        .authorization
        .authorization()
        .payload_set_hash()
        .clone();
    let entry_hash = ContentHash::new("sha256:transaction-entry");
    let cases = [
        (ToolDispatchTerminal::Succeeded, "succeeded", ""),
        (
            ToolDispatchTerminal::DispatchBlocked {
                reason: "breaker-open",
            },
            "failed",
            "runtime.outcome_receipt_breaker_open",
        ),
        (
            ToolDispatchTerminal::ToolError { error: "invoke" },
            "failed",
            "tool.invoke_error",
        ),
        (
            ToolDispatchTerminal::StructuredOutputMissing,
            "failed",
            "tool.output_missing",
        ),
        (
            ToolDispatchTerminal::OutputValidationFailed { error: "schema" },
            "failed",
            "tool.output_validation_failed",
        ),
        (
            ToolDispatchTerminal::ToolReportedFailure { error: "provider" },
            "failed",
            "tool.reported_failure",
        ),
        (
            ToolDispatchTerminal::TimedOut {
                timeout_ms: 50,
                error: "timeout",
            },
            "cancelled",
            "tool.native_timeout",
        ),
        (
            ToolDispatchTerminal::EventRecordingFailed { error: "event" },
            "failed",
            "runtime.tool_invoked_event_failed",
        ),
        (
            ToolDispatchTerminal::TransactionRecordingFailed {
                error: "transaction",
            },
            "failed",
            "runtime.write_transaction_record_failed",
        ),
        (
            ToolDispatchTerminal::DispatchFutureDropped,
            "cancelled",
            "tool.dispatch_future_dropped",
        ),
    ];

    for (terminal, expected_tag, expected_code) in cases {
        let has_partial_transaction = matches!(
            terminal,
            ToolDispatchTerminal::TransactionRecordingFailed { .. }
        );
        let mut input = base_input(&fixture.authorization, &executor, &payload, &entry_hash);
        input.terminal = terminal;
        if matches!(terminal, ToolDispatchTerminal::StructuredOutputMissing) {
            input.validation = ToolOutputValidationStatus::Missing;
        } else if matches!(
            terminal,
            ToolDispatchTerminal::OutputValidationFailed { .. }
        ) {
            input.validation = ToolOutputValidationStatus::Invalid { error: "schema" };
        } else if matches!(
            terminal,
            ToolDispatchTerminal::TransactionRecordingFailed { .. }
        ) {
            input.transaction = ToolTransactionEvidence::Failed {
                error: "transaction",
                transaction_id: Some("transaction-partial"),
                group_id: Some("group-partial"),
                entry_hash: Some(&entry_hash),
            };
            input.effect = ToolEffectDisposition::Unknown;
        }
        let finalized = finalize_tool_outcome(input).expect("terminal receipt");
        if has_partial_transaction {
            assert!(
                finalized
                    .canonical_evidence()
                    .contains("[\"effect.disposition\",\"unknown\"]")
            );
            assert!(
                finalized
                    .canonical_evidence()
                    .contains("[\"transaction.id\",\"transaction-partial\"]")
            );
            assert!(
                finalized
                    .canonical_evidence()
                    .contains("[\"transaction.group_id\",\"group-partial\"]")
            );
            assert!(
                finalized
                    .canonical_evidence()
                    .contains("[\"transaction.entry_hash\",\"sha256:transaction-entry\"]")
            );
        }
        let receipt = finalized.into_parts().0;
        let (tag, code) = status_parts(receipt.status());
        assert_eq!((tag, code), (expected_tag, expected_code));
    }
    let mut false_recorded_failure =
        base_input(&fixture.authorization, &executor, &payload, &entry_hash);
    false_recorded_failure.terminal = ToolDispatchTerminal::TransactionRecordingFailed {
        error: "transaction",
    };
    false_recorded_failure.transaction = ToolTransactionEvidence::Failed {
        error: "transaction",
        transaction_id: Some("transaction-partial"),
        group_id: Some("group-partial"),
        entry_hash: Some(&entry_hash),
    };
    assert_eq!(
        finalize_tool_outcome(false_recorded_failure)
            .expect_err("failed transaction cannot claim a recorded effect"),
        ToolOutcomeFinalizationError::InconsistentTerminalEvidence
    );
    let mut false_success = base_input(&fixture.authorization, &executor, &payload, &entry_hash);
    false_success.validation = ToolOutputValidationStatus::Missing;
    assert_eq!(
        finalize_tool_outcome(false_success).expect_err("false success must fail closed"),
        ToolOutcomeFinalizationError::InconsistentTerminalEvidence
    );
    let mut zero_timeout = base_input(&fixture.authorization, &executor, &payload, &entry_hash);
    zero_timeout.terminal = ToolDispatchTerminal::TimedOut {
        timeout_ms: 0,
        error: "ToolTimeout/echo timed out after 0 ms",
    };
    assert_eq!(
        finalize_tool_outcome(zero_timeout).expect_err("zero timeout must fail closed"),
        ToolOutcomeFinalizationError::InconsistentTerminalEvidence
    );
    let mut empty_record = base_input(&fixture.authorization, &executor, &payload, &entry_hash);
    empty_record.transaction = ToolTransactionEvidence::Recorded {
        transaction_id: "",
        group_id: None,
        entry_hash: &entry_hash,
    };
    assert_eq!(
        finalize_tool_outcome(empty_record).expect_err("empty transaction id must fail closed"),
        ToolOutcomeFinalizationError::InconsistentTerminalEvidence
    );
    let mut false_effect = base_input(&fixture.authorization, &executor, &payload, &entry_hash);
    false_effect.effect = ToolEffectDisposition::Unknown;
    assert_eq!(
        finalize_tool_outcome(false_effect).expect_err("recorded transaction needs exact effect"),
        ToolOutcomeFinalizationError::InconsistentTerminalEvidence
    );
}

#[test]
fn architecture_v2_outcome_hash_is_sensitive_to_every_execution_field() {
    let fixture = authorization("sensitivity", "sha256:state");
    let baseline_executor = executor(&fixture.capability, "runtime:tool-host", "builtin");
    let other_executor = executor(&fixture.capability, "runtime:other-host", "native");
    let payload = fixture
        .authorization
        .authorization()
        .payload_set_hash()
        .clone();
    let entry_hash = ContentHash::new("sha256:transaction-entry");
    let other_hash = ContentHash::new("sha256:other");
    let base = base_input(
        &fixture.authorization,
        &baseline_executor,
        &payload,
        &entry_hash,
    );
    let baseline = finalize_tool_outcome(base)
        .expect("baseline")
        .evidence_hash()
        .clone();

    let mut variants = Vec::new();
    let mut input = base;
    input.session_id = "session-other";
    variants.push(input);
    input = base;
    input.correlation_id = "correlation-other";
    variants.push(input);
    input = base;
    input.tool_name = "other-tool";
    variants.push(input);
    input = base;
    input.operation = "other-operation";
    variants.push(input);
    input = base;
    input.executor = &other_executor;
    variants.push(input);
    input = base;
    input.started_at_unix_ms = 9;
    variants.push(input);
    input = base;
    input.finished_at_unix_ms = 12;
    variants.push(input);
    input = base;
    input.terminal = ToolDispatchTerminal::ToolReportedFailure { error: "failed" };
    variants.push(input);
    input = base;
    input.effect = ToolEffectDisposition::Unknown;
    input.transaction = ToolTransactionEvidence::NotApplicable;
    variants.push(input);
    input = base;
    input.validation = ToolOutputValidationStatus::Invalid { error: "invalid" };
    input.terminal = ToolDispatchTerminal::OutputValidationFailed { error: "invalid" };
    variants.push(input);
    input = base;
    input.content = OutcomeMaterial::Hashed(&other_hash);
    variants.push(input);
    input = base;
    input.provider_output = OutcomeMaterial::Absent;
    variants.push(input);
    input = base;
    input.final_output = OutcomeMaterial::Hashed(&other_hash);
    variants.push(input);
    input = base;
    input.transaction = ToolTransactionEvidence::Preview;
    input.effect = ToolEffectDisposition::None;
    variants.push(input);

    for variant in variants {
        assert_ne!(
            finalize_tool_outcome(variant)
                .expect("sensitive variant")
                .evidence_hash(),
            &baseline
        );
    }
}

#[test]
fn architecture_v2_receipt_identity_is_unique_per_attempt_and_authority() {
    let first_fixture = authorization("first", "sha256:state");
    let second_fixture = authorization("second", "sha256:other-state");
    let first_executor = executor(&first_fixture.capability, "runtime:tool-host", "builtin");
    let second_executor = executor(&second_fixture.capability, "runtime:tool-host", "builtin");
    let first_payload = first_fixture
        .authorization
        .authorization()
        .payload_set_hash()
        .clone();
    let second_payload = second_fixture
        .authorization
        .authorization()
        .payload_set_hash()
        .clone();
    let entry_hash = ContentHash::new("sha256:transaction-entry");
    let first = finalize_tool_outcome(base_input(
        &first_fixture.authorization,
        &first_executor,
        &first_payload,
        &entry_hash,
    ))
    .expect("first");
    let mut second_attempt = base_input(
        &first_fixture.authorization,
        &first_executor,
        &first_payload,
        &entry_hash,
    );
    second_attempt.attempt_id = "attempt-2";
    let second_attempt = finalize_tool_outcome(second_attempt).expect("second attempt");
    let second_authority = finalize_tool_outcome(base_input(
        &second_fixture.authorization,
        &second_executor,
        &second_payload,
        &entry_hash,
    ))
    .expect("second authority");

    assert_ne!(first.receipt().id(), second_attempt.receipt().id());
    assert_ne!(
        first.receipt().receipt_hash(),
        second_attempt.receipt().receipt_hash()
    );
    assert_ne!(
        first.receipt().receipt_hash(),
        second_authority.receipt().receipt_hash()
    );
}

fn base_input<'a>(
    authorization: &'a HeptaKernelSafetyAuthorization,
    executor: &'a ToolExecutorBinding,
    payload: &'a ContentHash,
    entry_hash: &'a ContentHash,
) -> ToolOutcomeFinalizationInput<'a> {
    ToolOutcomeFinalizationInput {
        attempt_id: "attempt-1",
        authorization,
        session_id: "session-main",
        correlation_id: "correlation-1",
        tool_name: "echo",
        operation: "tool.echo",
        executor,
        payload_hash: payload,
        execution_idempotency_key: None,
        execution_resource_summary_hash: None,
        execution_effect_plan_hash: None,
        execution_effect_ack_hash: None,
        started_at_unix_ms: 10,
        finished_at_unix_ms: 11,
        terminal: ToolDispatchTerminal::Succeeded,
        effect: ToolEffectDisposition::Recorded,
        validation: ToolOutputValidationStatus::Valid,
        content: OutcomeMaterial::Raw("secret-content"),
        provider_output: OutcomeMaterial::Raw("secret-provider"),
        final_output: OutcomeMaterial::Raw("secret-final"),
        transaction: ToolTransactionEvidence::Recorded {
            transaction_id: "transaction-1",
            group_id: Some("group-1"),
            entry_hash,
        },
    }
}

fn status_parts(status: &OutcomeStatus) -> (&str, &str) {
    match status {
        OutcomeStatus::Succeeded => ("succeeded", ""),
        OutcomeStatus::Failed { error_code } => ("failed", error_code),
        OutcomeStatus::Cancelled { reason_code } => ("cancelled", reason_code),
        _ => ("unknown", "unknown"),
    }
}

fn executor(
    capability: &hepta_contracts::CapabilityManifestRef,
    principal: &str,
    provider: &str,
) -> ToolExecutorBinding {
    ToolExecutorBinding::try_new(
        capability.clone(),
        PrincipalId::new(principal),
        provider,
        capability.manifest_hash().clone(),
    )
    .expect("executor binding")
}

fn stamp(hash: &str) -> RevisionStamp {
    RevisionStamp::new(Revision::new(1), ContentHash::new(hash))
}
