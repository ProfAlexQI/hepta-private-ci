fn assert_lease_error(error: crate::HeptaError, code: &str) {
    assert!(error.0.contains(code), "expected {code}, got {}", error.0);
}

#[test]
fn architecture_v2_execution_lease_detects_epoch_drift_after_failed_mutation() {
    let runtime = RuntimeKernel::new();
    let expected = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch capture");
    runtime
        .set_path_capability_gate("unknown-tool", "path", FilesystemScope::WorkspaceOnly)
        .expect_err("invalid mutation should fail after acquiring its marker");
    let error = runtime
        .begin_execution_lease(expected)
        .expect_err("failed mutation must still invalidate a frozen epoch");
    assert_lease_error(error, "execution_lease.frozen_context_changed");
}

#[test]
fn architecture_v2_execution_lease_blocks_same_session_and_global_mutation() {
    let runtime = RuntimeKernel::new();
    let expected = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch capture");
    let lease = runtime
        .begin_execution_lease(expected)
        .expect("execution lease");

    let session_error = runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect_err("same-session mutation must be excluded");
    assert_lease_error(session_error, "execution_lease.execution_in_flight");

    let policy_error = runtime
        .add_policy_rule(
            None,
            None,
            Some("echo"),
            None,
            ApprovalRequirement::Deny,
            Some("lease test"),
        )
        .expect_err("global policy mutation must be excluded");
    assert_lease_error(policy_error, "execution_lease.execution_in_flight");

    drop(lease);
    runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect("session mutation should resume after drop");
    runtime
        .add_policy_rule(
            None,
            None,
            Some("echo"),
            None,
            ApprovalRequirement::Deny,
            Some("lease released"),
        )
        .expect("global mutation should resume after drop");
}

#[test]
fn architecture_v2_execution_lease_allows_unrelated_session_mutation() {
    let runtime = RuntimeKernel::new();
    let expected = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch capture");
    let lease = runtime
        .begin_execution_lease(expected)
        .expect("execution lease");

    let switched = runtime
        .switch_execution_profile_in_session("session-beta", ExecutionProfile::ReadOnlyTools)
        .expect("another session may mutate independently");
    assert_eq!(switched.current, ExecutionProfile::ReadOnlyTools);

    drop(lease);
}

#[tokio::test]
async fn architecture_v2_session_turn_reservation_rejects_competing_same_session_turn() {
    let runtime = RuntimeKernel::new();
    let reservation = runtime
        .begin_session_turn_reservation("session-main")
        .expect("reserve session turn");
    let error = runtime
        .run_demo_turn_in_session("session-main", "hello")
        .await
        .expect_err("competing turn must fail before model or tool execution");
    assert_lease_error(error, "execution_lease.session_turn_active");
    drop(reservation);

    runtime
        .run_demo_turn_in_session("session-main", "hello")
        .await
        .expect("turn resumes after reservation release");
}

#[test]
fn architecture_v2_execution_lease_composites_fail_closed_without_nesting() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_execution_profile_in_session("session-beta", ExecutionProfile::FullAccess)
        .expect("materialize beta");
    let snapshot = runtime.runtime_snapshot().expect("snapshot");
    let expected = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch capture");
    let lease = runtime
        .begin_execution_lease(expected)
        .expect("execution lease");

    let fork_error = runtime
        .fork_session("session-main", "session-fork")
        .expect_err("fork must not read an in-flight source");
    assert_lease_error(fork_error, "execution_lease.execution_in_flight");

    let snapshot_error = runtime
        .apply_runtime_snapshot(snapshot.clone())
        .expect_err("global snapshot mutation must be excluded");
    assert_lease_error(snapshot_error, "execution_lease.execution_in_flight");

    drop(lease);
    runtime
        .fork_session("session-main", "session-fork")
        .expect("fork should complete without nested mutation markers");
    runtime
        .apply_runtime_snapshot(snapshot)
        .expect("snapshot should complete without nested mutation markers");
}

#[tokio::test]
async fn architecture_v2_execution_lease_is_owned_by_execution_bus_until_dispatch_finishes() {
    let runtime = RuntimeKernel::new();
    let expected = runtime
        .capture_execution_epoch("session-main")
        .expect("epoch capture");
    let lease = runtime
        .begin_execution_lease(expected)
        .expect("execution lease");
    let bus = ExecutionBus::new(&runtime);
    let session_id = SessionId("session-main".to_string());
    let correlation_id = CorrelationId("lease-dispatch".to_string());
    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(hepta_core::PolicyEvaluationContext {
            session_id: Some(session_id.clone()),
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
        r#"{"text":"lease-held"}"#,
        &decision,
    )
    .expect("candidate");
    let lease = lease
        .bind_tool_resources(
            &runtime,
            "session-main",
            "echo",
            &candidate.canonical_arguments,
        )
        .expect("resource-bound lease");
    let authorization = SafetyGateClient::authorize_execution_without_grant(
        &runtime,
        &session_id,
        &correlation_id,
        &candidate,
        &candidate,
        lease,
    )
    .expect("authorization");
    let dispatch = bus.dispatch(authorization);

    for error in [
        runtime
            .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
            .expect_err("profile mutation must wait for dispatch"),
        runtime
            .switch_filesystem_scope(FilesystemScope::AnyPath)
            .expect_err("filesystem mutation must wait for dispatch"),
        runtime
            .switch_write_path_scope(WritePathScope::WorkspaceOnly)
            .expect_err("write-scope mutation must wait for dispatch"),
        runtime
            .set_path_capability_gate("read_file", "path", FilesystemScope::WorkspaceOnly)
            .expect_err("path-gate mutation must wait for dispatch"),
    ] {
        assert_lease_error(error, "execution_lease.execution_in_flight");
    }
    let policy_error = runtime
        .add_policy_rule(
            None,
            None,
            Some("echo"),
            None,
            ApprovalRequirement::Deny,
            Some("dispatch barrier"),
        )
        .expect_err("policy mutation must wait for dispatch");
    assert_lease_error(policy_error, "execution_lease.execution_in_flight");

    let mut captured = dispatch.await;
    crate::OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("terminal receipt");
    let result = captured.tool_result().expect("echo dispatch");
    assert_eq!(result.content, "echo:lease-held");
    runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect("lease must release after dispatch");
}
