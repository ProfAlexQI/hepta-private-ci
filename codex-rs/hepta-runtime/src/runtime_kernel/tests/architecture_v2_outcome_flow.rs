use crate::ExecutionBus;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::native_tool_invocation_timeout_result;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use crate::runtime_kernel::execution_bus::CapturedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedValidation;
use hepta_contracts::OutcomeStatus;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::ToolResult;
use hepta_memory::OutcomeRecord;
use serde_json::Value;
use std::path::PathBuf;

fn authorize(
    runtime: &RuntimeKernel,
    tool_name: &str,
    arguments_json: &str,
    correlation: &str,
) -> (AuthorizedToolExecution, String) {
    let session_id = SessionId("session-main".into());
    let correlation_id = CorrelationId(correlation.into());
    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(session_id.clone()),
            model: Some(model.clone()),
            tool_name: tool_name.into(),
            risk_tier: runtime.tools.risk_tier(tool_name).expect("risk"),
        })
        .expect("exact policy decision");
    let candidate = SafetyGateClient::prepare_candidate(
        runtime,
        &session_id.0,
        &model,
        tool_name,
        arguments_json,
        &decision,
    )
    .expect("candidate");
    let epoch = runtime
        .capture_execution_epoch(&session_id.0)
        .expect("execution epoch");
    let lease = runtime
        .begin_execution_lease(epoch)
        .expect("execution lease");
    let lease = lease
        .bind_tool_resources(
            runtime,
            &session_id.0,
            tool_name,
            &candidate.canonical_arguments,
        )
        .expect("resource-bound execution lease");
    let execution = SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &session_id,
        &correlation_id,
        &candidate,
        &candidate,
        lease,
    )
    .expect("exact authorization");
    let attempt_id = execution.attempt_id().to_string();
    (execution, attempt_id)
}

fn finalize(runtime: &RuntimeKernel, captured: &mut CapturedToolExecution<'_>) -> OutcomeRecord {
    let attempt_id = captured.attempt_id().to_string();
    captured.capture_write_transaction();
    OutcomeRecorder::new(runtime)
        .finalize_tool_dispatch(captured)
        .expect("terminal outcome receipt");
    runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("outcome read")
        .expect("terminal outcome record")
}

fn evidence_field(record: &OutcomeRecord, name: &str) -> Value {
    let envelope: Value =
        serde_json::from_str(record.canonical_evidence()).expect("canonical evidence");
    let fields = envelope["fields"].as_array().expect("evidence fields");
    fields
        .iter()
        .find_map(|field| {
            let pair = field.as_array()?;
            (pair.first()?.as_str()? == name)
                .then(|| pair.get(1))
                .flatten()
        })
        .cloned()
        .expect("named evidence field")
}

fn assert_failed_code(record: &OutcomeRecord, expected: &str) {
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Failed { error_code } if error_code == expected
    ));
}

#[tokio::test]
async fn architecture_v2_outcome_flow_success_is_readable_with_full_evidence() {
    let runtime = RuntimeKernel::new();
    let (execution, attempt_id) = authorize(
        &runtime,
        "echo",
        r#"{"text":"receipt timed out"}"#,
        "success",
    );
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    let record = finalize(&runtime, &mut captured);

    assert_eq!(record.attempt_id(), attempt_id);
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Succeeded
    ));
    assert_eq!(
        record.receipt().outcome_hash(),
        record.canonical_evidence_hash()
    );
    assert_eq!(
        evidence_field(&record, "attempt.id").as_str(),
        Some(attempt_id.as_str())
    );
    assert_eq!(
        evidence_field(&record, "capability.id").as_str(),
        Some("tool:echo")
    );
    assert_eq!(
        evidence_field(&record, "executor.provider").as_str(),
        Some("hepta-runtime-builtin")
    );
    assert_eq!(
        evidence_field(&record, "terminal.code").as_str(),
        Some("ok")
    );
}

#[tokio::test]
async fn architecture_v2_outcome_flow_captures_tool_error() {
    let runtime = RuntimeKernel::new();
    let (execution, _) = authorize(&runtime, "echo", r#"{"wrong":"argument"}"#, "tool-error");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(captured.outward_error().is_some());
    let record = finalize(&runtime, &mut captured);

    assert_failed_code(&record, "tool.invoke_error");
    assert_eq!(
        evidence_field(&record, "effect.disposition").as_str(),
        Some("none")
    );
}

#[test]
fn architecture_v2_outcome_flow_captures_missing_and_invalid_output() {
    let missing_runtime = RuntimeKernel::new();
    let (missing_execution, _) = authorize(&missing_runtime, "echo", r#"{"text":"x"}"#, "missing");
    let mut missing = ExecutionBus::new(&missing_runtime).capture_invocation_for_test(
        missing_execution,
        Ok(ToolResult {
            content: "missing".into(),
            structured_json: None,
        }),
    );
    let missing_record = finalize(&missing_runtime, &mut missing);
    assert_failed_code(&missing_record, "tool.output_missing");
    assert_eq!(
        evidence_field(&missing_record, "validation.status").as_str(),
        Some("missing")
    );

    let invalid_runtime = RuntimeKernel::new();
    let (invalid_execution, _) = authorize(&invalid_runtime, "echo", r#"{"text":"x"}"#, "invalid");
    let mut invalid = ExecutionBus::new(&invalid_runtime).capture_invocation_for_test(
        invalid_execution,
        Ok(ToolResult {
            content: "invalid".into(),
            structured_json: Some("{}".into()),
        }),
    );
    let invalid_record = finalize(&invalid_runtime, &mut invalid);
    assert_failed_code(&invalid_record, "tool.output_validation_failed");
    assert_eq!(
        evidence_field(&invalid_record, "validation.status").as_str(),
        Some("invalid")
    );
}

#[test]
fn architecture_v2_outcome_flow_captures_timeout_and_reported_failure() {
    let timeout_runtime = RuntimeKernel::new();
    let (timeout_execution, _) = authorize(
        &timeout_runtime,
        "web_search",
        r#"{"query":"timeout fixture"}"#,
        "timeout",
    );
    let mut timed_out = ExecutionBus::new(&timeout_runtime).capture_invocation_for_test(
        timeout_execution,
        Ok(native_tool_invocation_timeout_result("web_search", 100)),
    );
    let timeout_record = finalize(&timeout_runtime, &mut timed_out);
    assert!(matches!(
        timeout_record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code } if reason_code == "tool.native_timeout"
    ));
    assert_eq!(
        evidence_field(&timeout_record, "effect.disposition").as_str(),
        Some("none")
    );
    assert_eq!(
        evidence_field(&timeout_record, "terminal.timeout_ms").as_u64(),
        Some(100)
    );

    let malformed_timeout_runtime = RuntimeKernel::new();
    let (malformed_timeout_execution, _) = authorize(
        &malformed_timeout_runtime,
        "web_search",
        r#"{"query":"malformed timeout fixture"}"#,
        "malformed-timeout",
    );
    let mut malformed_timeout = ExecutionBus::new(&malformed_timeout_runtime)
        .capture_invocation_for_test(
            malformed_timeout_execution,
            Ok(ToolResult {
                content: "ToolTimeout/web_search timed out after 100 ms".into(),
                structured_json: Some("{not-json".into()),
            }),
        );
    assert!(matches!(
        malformed_timeout.terminal(),
        CapturedDispatchTerminal::TimedOut { .. }
    ));
    assert!(matches!(
        malformed_timeout.validation(),
        CapturedValidation::Invalid(_)
    ));
    assert!(malformed_timeout.outward_error().is_none());
    let malformed_timeout_record = finalize(&malformed_timeout_runtime, &mut malformed_timeout);
    assert!(matches!(
        malformed_timeout_record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code } if reason_code == "tool.native_timeout"
    ));
    assert_eq!(
        evidence_field(&malformed_timeout_record, "validation.status").as_str(),
        Some("invalid")
    );

    let durationless_timeout_runtime = RuntimeKernel::new();
    let (durationless_timeout_execution, _) = authorize(
        &durationless_timeout_runtime,
        "web_search",
        r#"{"query":"durationless timeout fixture"}"#,
        "durationless-timeout",
    );
    let mut durationless_timeout = ExecutionBus::new(&durationless_timeout_runtime)
        .capture_invocation_for_test(
            durationless_timeout_execution,
            Ok(ToolResult {
                content: "ToolTimeout/web_search timed out".into(),
                structured_json: Some("{not-json".into()),
            }),
        );
    assert!(matches!(
        durationless_timeout.terminal(),
        CapturedDispatchTerminal::OutputValidationFailed(_)
    ));
    let durationless_timeout_record =
        finalize(&durationless_timeout_runtime, &mut durationless_timeout);
    assert_failed_code(
        &durationless_timeout_record,
        "tool.output_validation_failed",
    );
    assert_eq!(
        evidence_field(&durationless_timeout_record, "terminal.timeout_ms").as_u64(),
        Some(0)
    );

    let (mismatched_execution, _) = authorize(
        &malformed_timeout_runtime,
        "echo",
        r#"{"text":"x"}"#,
        "mismatched-timeout-tool",
    );
    let mut mismatched_timeout = ExecutionBus::new(&malformed_timeout_runtime)
        .capture_invocation_for_test(
            mismatched_execution,
            Ok(ToolResult {
                content: "ToolTimeout/other_tool timed out".into(),
                structured_json: Some(r#"{"text":"x"}"#.into()),
            }),
        );
    assert!(matches!(
        mismatched_timeout.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    let mismatched_record = finalize(&malformed_timeout_runtime, &mut mismatched_timeout);
    assert!(matches!(
        mismatched_record.receipt().status(),
        OutcomeStatus::Succeeded
    ));

    let failure_runtime = RuntimeKernel::new();
    let (failure_execution, _) = authorize(
        &failure_runtime,
        "web_search",
        r#"{"query":"reported failure fixture"}"#,
        "reported-failure",
    );
    let mut failure = ExecutionBus::new(&failure_runtime).capture_invocation_for_test(
        failure_execution,
        Ok(ToolResult {
            content: "provider refused".into(),
            structured_json: Some(r#"{"status":"error","error":"provider refused"}"#.into()),
        }),
    );
    assert!(failure.outward_error().is_none());
    let failure_record = finalize(&failure_runtime, &mut failure);
    assert_failed_code(&failure_record, "tool.reported_failure");
}

#[tokio::test]
async fn architecture_v2_outcome_flow_records_write_transaction_before_receipt() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("outcome flow write test"),
        )
        .expect("write policy");
    let relative = format!(
        "artifacts/.architecture-v2-outcome-{}.txt",
        uuid::Uuid::new_v4()
    );
    let cleanup = TestFileGuard(crate::tool_workspace_root_path().join(&relative));
    std::fs::create_dir_all(cleanup.0.parent().expect("write fixture parent"))
        .expect("create write fixture parent");
    let arguments = serde_json::json!({
        "path": relative,
        "content": "receipt-backed write",
        "mode": "create"
    })
    .to_string();
    let (execution, attempt_id) =
        authorize(&runtime, "write_file", &arguments, "write-transaction");
    assert!(execution.holds_write_target_reservation());
    assert_eq!(
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state")
            .active_target_reservations
            .len(),
        1
    );
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert_eq!(
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state")
            .active_target_reservations
            .len(),
        1
    );
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("write outcome receipt");
    assert!(
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state")
            .active_target_reservations
            .is_empty()
    );
    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("outcome read")
        .expect("write outcome");

    assert!(cleanup.0.exists());
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Succeeded
    ));
    assert_eq!(
        evidence_field(&record, "transaction.status").as_str(),
        Some("recorded")
    );
    assert_eq!(
        evidence_field(&record, "effect.disposition").as_str(),
        Some("recorded")
    );
}

#[test]
fn architecture_v2_outcome_flow_drop_paths_are_cancelled_once() {
    let runtime = RuntimeKernel::new();
    let (authorized, authorized_attempt) = authorize(
        &runtime,
        "echo",
        r#"{"text":"drop-authorized"}"#,
        "drop-authorized",
    );
    drop(authorized);
    let first = runtime
        .outcome_record_by_attempt(&authorized_attempt)
        .expect("read")
        .expect("authorized drop receipt");

    let (future_execution, future_attempt) =
        authorize(&runtime, "echo", r#"{"text":"drop-future"}"#, "drop-future");
    let future = ExecutionBus::new(&runtime).dispatch(future_execution);
    drop(future);
    let second = runtime
        .outcome_record_by_attempt(&future_attempt)
        .expect("read")
        .expect("future drop receipt");

    assert_ne!(authorized_attempt, future_attempt);
    for record in [&first, &second] {
        assert!(matches!(
            record.receipt().status(),
            OutcomeStatus::Cancelled { reason_code }
                if reason_code == "tool.dispatch_future_dropped"
        ));
    }
    assert_eq!(runtime.outcome_record_count().expect("count"), 2);
}

#[tokio::test]
async fn architecture_v2_outcome_flow_blocked_paths_do_not_mint_receipts() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            Some("session-main"),
            Some("demo"),
            Some("echo"),
            None,
            ApprovalRequirement::Deny,
            Some("outcome flow denial"),
        )
        .expect("deny rule");
    let result = runtime
        .run_demo_turn("tool:blocked echo")
        .await
        .expect("blocked turn");
    assert_eq!(result.invoked_tool, None);
    assert_eq!(runtime.outcome_record_count().expect("count"), 0);

    let ask_runtime = RuntimeKernel::new();
    ask_runtime
        .add_policy_rule(
            Some("session-main"),
            Some("demo"),
            Some("echo"),
            None,
            ApprovalRequirement::Ask,
            Some("outcome flow approval"),
        )
        .expect("ask rule");
    let pending = ask_runtime
        .run_demo_turn("tool:approval echo")
        .await
        .expect("approval turn");
    assert_eq!(pending.invoked_tool, None);
    assert_eq!(pending.approval_required.as_deref(), Some("echo"));
    assert_eq!(ask_runtime.outcome_record_count().expect("count"), 0);
}

#[tokio::test]
async fn architecture_v2_outcome_flow_breaker_rejects_dispatch_before_invoke() {
    let runtime = RuntimeKernel::new();
    let (execution, _) = authorize(&runtime, "echo", r#"{"text":"must-not-run"}"#, "breaker");
    runtime.trip_outcome_breaker("injected outcome store failure");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(captured.tool_result().is_none());
    assert!(
        captured
            .outward_error()
            .expect("breaker error")
            .0
            .contains("outcome receipt breaker is open")
    );
    let invoked_events = runtime
        .events(usize::MAX)
        .expect("events")
        .into_iter()
        .filter(|event| event.event.kind == hepta_core::EventKind::ToolInvoked)
        .count();
    assert_eq!(invoked_events, 0);
    let blocked = finalize(&runtime, &mut captured);
    assert_failed_code(&blocked, "runtime.outcome_receipt_breaker_open");
    assert_eq!(
        evidence_field(&blocked, "terminal.code").as_str(),
        Some("runtime.outcome_receipt_breaker_open")
    );

    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(SessionId("session-main".into())),
            model: Some(model.clone()),
            tool_name: "echo".into(),
            risk_tier: runtime.tools.risk_tier("echo").expect("risk"),
        })
        .expect("exact policy decision");
    let candidate = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "echo",
        r#"{"text":"second"}"#,
        &decision,
    )
    .expect("candidate");
    let epoch = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch");
    let lease = runtime.begin_execution_lease(epoch).expect("lease");
    let lease = lease
        .bind_tool_resources(
            &runtime,
            "session-main",
            "echo",
            &candidate.canonical_arguments,
        )
        .expect("resource-bound lease");
    let error = SafetyGateClient::authorize_execution_without_grant(
        &runtime,
        &SessionId("session-main".into()),
        &CorrelationId("second".into()),
        &candidate,
        &candidate,
        lease,
    )
    .expect_err("breaker must reject later execution");
    assert!(error.0.contains("outcome receipt breaker is open"));
}

struct TestFileGuard(PathBuf);

impl Drop for TestFileGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
