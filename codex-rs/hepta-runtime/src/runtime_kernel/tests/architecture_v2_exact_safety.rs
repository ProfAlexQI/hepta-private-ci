#[tokio::test]
async fn architecture_v2_exact_safety_payload_substitution_requires_new_approval() {
    let runtime = RuntimeKernel::new();
    let original = existing_read_intent();
    runtime
        .run_demo_turn(&original)
        .await
        .expect("pending turn");
    let binding = exact_pending_binding(&runtime);
    runtime
        .approve_candidate(&binding)
        .expect("exact approval succeeds");

    let substituted = runtime
        .run_demo_turn(&changed_read_intent())
        .await
        .expect("substituted turn");
    assert_eq!(substituted.invoked_tool, None);
    assert_eq!(substituted.approval_required.as_deref(), Some("read_file"));
    assert_ne!(exact_pending_binding(&runtime), binding);
}

#[tokio::test]
async fn architecture_v2_exact_safety_policy_and_context_drift_require_new_approval() {
    let intent = existing_read_intent();
    let policy_runtime = RuntimeKernel::new();
    policy_runtime
        .run_demo_turn(&intent)
        .await
        .expect("initial pending");
    policy_runtime
        .approve_candidate(&exact_pending_binding(&policy_runtime))
        .expect("exact approval");
    policy_runtime
        .add_policy_rule(
            Some("session-main"),
            None,
            Some("read_file"),
            None,
            ApprovalRequirement::Ask,
            Some("changed ask policy"),
        )
        .expect("policy mutation");
    let policy_drift = policy_runtime
        .run_demo_turn(&intent)
        .await
        .expect("policy drift turn");
    assert_eq!(policy_drift.invoked_tool, None);
    assert_eq!(policy_drift.approval_required.as_deref(), Some("read_file"));

    let context_runtime = RuntimeKernel::new();
    context_runtime
        .run_demo_turn(&intent)
        .await
        .expect("initial pending");
    context_runtime
        .approve_candidate(&exact_pending_binding(&context_runtime))
        .expect("exact approval");
    context_runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect("context mutation");
    let context_drift = context_runtime
        .run_demo_turn(&intent)
        .await
        .expect("context drift turn");
    assert_eq!(context_drift.invoked_tool, None);
    assert_eq!(
        context_drift.approval_required.as_deref(),
        Some("read_file")
    );
}

#[tokio::test]
async fn architecture_v2_exact_safety_exact_grant_is_single_use() {
    let runtime = RuntimeKernel::new();
    let intent = existing_read_intent();
    runtime
        .run_demo_turn(&intent)
        .await
        .expect("initial pending");
    runtime
        .approve_candidate(&exact_pending_binding(&runtime))
        .expect("exact approval");

    let first = runtime.run_demo_turn(&intent).await.expect("first use");
    assert_eq!(first.invoked_tool.as_deref(), Some("read_file"));
    let replay = runtime.run_demo_turn(&intent).await.expect("replay");
    assert_eq!(replay.invoked_tool, None);
    assert_eq!(replay.approval_required.as_deref(), Some("read_file"));
}

#[tokio::test]
async fn architecture_v2_exact_safety_proactive_read_only_token_binds_once() {
    let runtime = RuntimeKernel::new();
    let response = runtime
        .approve_tool("read_file")
        .expect("read-only proactive approval");
    assert!(response.contains("proactive one-shot"));
    let intent = existing_read_intent();

    let first = runtime.run_demo_turn(&intent).await.expect("bound use");
    assert_eq!(first.invoked_tool.as_deref(), Some("read_file"));
    let replay = runtime.run_demo_turn(&intent).await.expect("replay");
    assert_eq!(replay.invoked_tool, None);
    assert_eq!(replay.approval_required.as_deref(), Some("read_file"));
}

#[test]
fn architecture_v2_exact_safety_high_risk_proactive_approval_is_rejected() {
    let runtime = RuntimeKernel::new();
    for tool_name in ["write_file", "write", "edit", "apply_patch", "message"] {
        let error = runtime
            .approve_tool(tool_name)
            .expect_err("effectful proactive approval must fail");
        assert!(error.0.contains("exact pending candidate"));
    }
    let quarantined = runtime
        .approve_tool("exec")
        .expect_err("quarantined exec must not have production policy metadata");
    assert!(quarantined.0.contains("unknown tool: exec"));
}

#[test]
fn architecture_v2_exact_safety_expired_exact_pending_never_downgrades_to_proactive() {
    let runtime = RuntimeKernel::new();
    let model = runtime.model_selection().expect("model").active;
    let decision = exact_policy_decision(
        &runtime,
        ApprovalRequirement::Ask,
        "exact approval required",
    );
    let mut expired = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"alpha"}"#,
        &decision,
    )
    .expect("candidate");
    expired.expires_at_unix_ms = 0;
    runtime
        .approval_state
        .lock()
        .expect("approval state")
        .remember_pending_exact("session-main", expired);

    let error = runtime
        .approve_tool("read_file")
        .expect_err("expired exact pending must not become proactive");
    assert!(error.0.contains("expired"));
    assert!(
        runtime
            .approval_snapshot()
            .expect("snapshot")
            .pending
            .is_empty()
    );
}

#[tokio::test]
async fn architecture_v2_exact_safety_imported_legacy_grant_is_display_only() {
    let runtime = RuntimeKernel::new();
    runtime
        .approve_tool("read_file")
        .expect("display grant setup");
    let export = runtime
        .session_export("session-main")
        .expect("session export");
    runtime
        .apply_session_export(export)
        .expect("session import");
    assert!(
        runtime
            .approval_snapshot()
            .expect("display snapshot")
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let result = runtime
        .run_demo_turn(&existing_read_intent())
        .await
        .expect("post-import turn");
    assert_eq!(result.invoked_tool, None);
    assert_eq!(result.approval_required.as_deref(), Some("read_file"));
}

#[test]
fn architecture_v2_exact_admission_rejects_deny_before_approval() {
    let runtime = RuntimeKernel::new();
    let model = runtime.model_selection().expect("model").active;
    let decision = exact_policy_decision(&runtime, ApprovalRequirement::Deny, "denied by policy");
    let error = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"alpha"}"#,
        &decision,
    )
    .expect_err("runtime must not turn a deny decision into admission");
    assert!(error.0.contains("admission.policy_denied"));
}

#[test]
fn architecture_v2_exact_admission_accepts_none_and_ask_evidence() {
    for requirement in [ApprovalRequirement::None, ApprovalRequirement::Ask] {
        let runtime = RuntimeKernel::new();
        let model = runtime.model_selection().expect("model").active;
        let decision = exact_policy_decision(&runtime, requirement, "feasible test policy");
        let material = SafetyGateClient::prepare_candidate(
            &runtime,
            "session-main",
            &model,
            "read_file",
            r#"{"path":"alpha"}"#,
            &decision,
        )
        .expect("none and ask remain feasible");
        assert!(material.admission.is_admitted());
    }
}

#[test]
fn architecture_v2_exact_safety_action_meta_and_context_substitution_fail_closed() {
    let runtime = RuntimeKernel::new();
    let model = runtime.model_selection().expect("model").active;
    let base_decision = exact_policy_decision(&runtime, ApprovalRequirement::Ask, "base");
    let approved = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"alpha"}"#,
        &base_decision,
    )
    .expect("approved candidate");

    let action_substitution = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"beta"}"#,
        &base_decision,
    )
    .expect("action substitution");
    let action_error = SafetyGateClient::authorize_without_grant(&approved, &action_substitution)
        .expect_err("changed payload/action must fail");
    assert!(action_error.0.contains("payload_substitution"));

    let changed_decision =
        exact_policy_decision(&runtime, ApprovalRequirement::Ask, "changed metacontrol");
    let meta_substitution = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"alpha"}"#,
        &changed_decision,
    )
    .expect("meta substitution");
    let meta_error = SafetyGateClient::authorize_without_grant(&approved, &meta_substitution)
        .expect_err("changed metacontrol must fail");
    assert!(meta_error.0.contains("candidate_substitution"));

    runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect("context mutation");
    let context_substitution = SafetyGateClient::prepare_candidate(
        &runtime,
        "session-main",
        &model,
        "read_file",
        r#"{"path":"alpha"}"#,
        &changed_decision,
    )
    .expect("context substitution");
    let context_error = SafetyGateClient::authorize_without_grant(&approved, &context_substitution)
        .expect_err("changed context must fail");
    assert!(context_error.0.contains("candidate_substitution"));
}
