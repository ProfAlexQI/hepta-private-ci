use crate::ExecutionBus;
use crate::HeptaError;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use crate::runtime_kernel::execution_bus::CapturedTransaction;
use hepta_contracts::OutcomeStatus;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::ToolResult;

fn candidate(
    runtime: &RuntimeKernel,
    session: &str,
    arguments: &str,
    reason: &str,
) -> ExactApprovalMaterial {
    let active_model = runtime.model_selection().expect("model").active;
    let context = || PolicyEvaluationContext {
        session_id: Some(SessionId(session.into())),
        model: Some(active_model.clone()),
        tool_name: "write_file".into(),
        risk_tier: runtime.tools.risk_tier("write_file").expect("risk"),
    };
    if runtime
        .policy
        .evaluate_with_match(context())
        .expect("current policy decision")
        .requirement
        != ApprovalRequirement::None
    {
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("resource reservation test"),
            )
            .expect("write policy");
    }
    let decision = runtime
        .policy
        .evaluate_with_match(context())
        .expect("exact policy decision");
    SafetyGateClient::prepare_candidate(
        runtime,
        session,
        &active_model,
        "write_file",
        arguments,
        &decision,
    )
    .unwrap_or_else(|error| panic!("{reason}: {}", error.0))
}

fn authorize_for_session(
    runtime: &RuntimeKernel,
    session: &str,
    arguments: &str,
    correlation: &str,
) -> Result<(AuthorizedToolExecution, String), HeptaError> {
    let material = candidate(runtime, session, arguments, correlation);
    let epoch = runtime.capture_execution_epoch(session)?;
    let lease = runtime.begin_execution_lease(epoch)?;
    let lease = lease.bind_tool_resources(
        runtime,
        session,
        "write_file",
        &material.canonical_arguments,
    )?;
    let execution = SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId(correlation.into()),
        &material,
        &material,
        lease,
    )?;
    let attempt_id = execution.attempt_id().to_string();
    Ok((execution, attempt_id))
}

fn write_arguments(label: &str) -> String {
    serde_json::json!({
        "path": format!(
            "artifacts/.architecture-v2-{label}-{}.txt",
            uuid::Uuid::new_v4()
        ),
        "content": label,
        "mode": "create"
    })
    .to_string()
}

fn active_reservations(runtime: &RuntimeKernel) -> usize {
    runtime
        .write_lock_state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .active_target_reservations
        .len()
}

#[test]
fn architecture_v2_resource_reservation_authorization_failure_releases_every_guard() {
    let runtime = RuntimeKernel::new();
    let approved_arguments = write_arguments("authorization-approved");
    let mut presented_value: serde_json::Value =
        serde_json::from_str(&approved_arguments).expect("arguments");
    presented_value["content"] = serde_json::json!("different");
    let presented_arguments = presented_value.to_string();
    let approved = candidate(&runtime, "session-main", &approved_arguments, "approved");
    let presented = candidate(&runtime, "session-main", &presented_arguments, "presented");
    let epoch = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch");
    let lease = runtime.begin_execution_lease(epoch).expect("lease");
    let lease = lease
        .bind_tool_resources(
            &runtime,
            "session-main",
            "write_file",
            &presented.canonical_arguments,
        )
        .expect("resource-bound lease");

    SafetyGateClient::authorize_execution_without_grant(
        &runtime,
        &SessionId("session-main".into()),
        &CorrelationId("authorization-failure".into()),
        &approved,
        &presented,
        lease,
    )
    .expect_err("mismatched exact material must fail");

    assert_eq!(active_reservations(&runtime), 0);
    assert_eq!(runtime.outcome_record_count().expect("outcome count"), 0);
    let epoch = runtime
        .capture_execution_epoch("session-main")
        .expect("context lease released");
    drop(
        runtime
            .begin_execution_lease(epoch)
            .expect("reusable lease"),
    );
}

#[test]
fn architecture_v2_resource_reservation_blocks_parallel_write_and_rollback_until_drop() {
    let runtime = RuntimeKernel::new();
    let arguments = write_arguments("reservation-concurrency");
    let (execution, attempt_id) =
        authorize_for_session(&runtime, "session-main", &arguments, "holder")
            .expect("first authorization");
    assert!(execution.holds_write_target_reservation());

    let parallel_error = authorize_for_session(&runtime, "session-other", &arguments, "contender")
        .expect_err("parallel writer must fail");
    assert!(parallel_error.0.contains("tool_execution_reservation"));
    let target = serde_json::from_str::<serde_json::Value>(&arguments).expect("arguments")["path"]
        .as_str()
        .expect("path")
        .to_string();
    let target = crate::tool_workspace_root_path().join(target);
    let rollback_error = runtime
        .acquire_group_rollback_locks(
            "session-main",
            "group-held",
            "attempt-held",
            &[target.display().to_string()],
        )
        .expect_err("rollback must not overlap the witness");
    assert!(
        rollback_error
            .0
            .contains("write lock blocks rollback_group")
    );

    drop(execution);
    assert_eq!(active_reservations(&runtime), 0);
    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("outcome read")
        .expect("drop receipt");
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code }
            if reason_code == "tool.dispatch_future_dropped"
    ));
    runtime
        .acquire_group_rollback_locks(
            "session-main",
            "group-released",
            "attempt-released",
            &[target.display().to_string()],
        )
        .expect("rollback lock after release");
    runtime
        .release_group_rollback_locks("session-main", "group-released")
        .expect("rollback lock release");
}

#[test]
fn architecture_v2_resource_reservation_releases_on_authorized_and_future_drop() {
    let runtime = RuntimeKernel::new();
    let (authorized, authorized_attempt) = authorize_for_session(
        &runtime,
        "session-main",
        &write_arguments("authorized-drop"),
        "authorized-drop",
    )
    .expect("authorization");
    drop(authorized);
    assert_eq!(active_reservations(&runtime), 0);

    let (future_execution, future_attempt) = authorize_for_session(
        &runtime,
        "session-main",
        &write_arguments("future-drop"),
        "future-drop",
    )
    .expect("future authorization");
    let future = ExecutionBus::new(&runtime).dispatch(future_execution);
    assert_eq!(active_reservations(&runtime), 1);
    drop(future);
    assert_eq!(active_reservations(&runtime), 0);

    for attempt_id in [authorized_attempt, future_attempt] {
        let record = runtime
            .outcome_record_by_attempt(&attempt_id)
            .expect("outcome read")
            .expect("drop receipt");
        assert!(matches!(
            record.receipt().status(),
            OutcomeStatus::Cancelled { reason_code }
                if reason_code == "tool.dispatch_future_dropped"
        ));
    }
}

#[test]
fn architecture_v2_resource_reservation_releases_on_tool_error_and_timeout() {
    let error_runtime = RuntimeKernel::new();
    let (error_execution, _) = authorize_for_session(
        &error_runtime,
        "session-main",
        &write_arguments("tool-error"),
        "tool-error",
    )
    .expect("error authorization");
    let mut errored = ExecutionBus::new(&error_runtime).capture_invocation_for_test(
        error_execution,
        Err(HeptaError("injected write tool error".into())),
    );
    assert_eq!(active_reservations(&error_runtime), 1);
    errored.capture_write_transaction();
    OutcomeRecorder::new(&error_runtime)
        .finalize_tool_dispatch(&mut errored)
        .expect("error receipt");
    assert_eq!(active_reservations(&error_runtime), 0);

    let timeout_runtime = RuntimeKernel::new();
    let (timeout_execution, timeout_attempt) = authorize_for_session(
        &timeout_runtime,
        "session-main",
        &write_arguments("tool-timeout"),
        "tool-timeout",
    )
    .expect("timeout authorization");
    let mut timed_out = ExecutionBus::new(&timeout_runtime).capture_invocation_for_test(
        timeout_execution,
        Ok(ToolResult {
            content: "ToolTimeout/write_file timed out after 25 ms".into(),
            structured_json: Some(
                serde_json::json!({
                    "status": "timeout",
                    "timeout": true,
                    "result": {"timeout": true, "timeout_ms": 25}
                })
                .to_string(),
            ),
        }),
    );
    assert!(matches!(
        timed_out.terminal(),
        CapturedDispatchTerminal::TimedOut { timeout_ms: 25, .. }
    ));
    let final_output = timed_out
        .tool_result()
        .and_then(|result| result.structured_json.clone());
    timed_out.record_transaction(
        CapturedTransaction::Failed {
            error: "timeout prevented trusted write transaction capture".into(),
            transaction_id: None,
            group_id: None,
            entry_hash: None,
        },
        final_output,
    );
    OutcomeRecorder::new(&timeout_runtime)
        .finalize_tool_dispatch(&mut timed_out)
        .expect("timeout receipt");
    assert_eq!(active_reservations(&timeout_runtime), 0);
    let record = timeout_runtime
        .outcome_record_by_attempt(&timeout_attempt)
        .expect("outcome read")
        .expect("timeout receipt");
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code } if reason_code == "tool.native_timeout"
    ));
}
