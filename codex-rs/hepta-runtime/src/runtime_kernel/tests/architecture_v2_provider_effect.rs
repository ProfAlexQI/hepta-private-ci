use std::fs;

use crate::ExecutionBus;
use crate::ExecutionEffectInspectionState;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::OutcomeRecordResult;
use serde_json::Value;
use serde_json::json;

fn durable_integrity_key() -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([0x79; 32])
}

fn authorize_mutation(
    runtime: &RuntimeKernel,
    session: &str,
    tool: &str,
    arguments: &str,
) -> AuthorizedToolExecution {
    let active_model = runtime.model_selection().expect("active model").active;
    let context = PolicyEvaluationContext {
        session_id: Some(SessionId(session.into())),
        model: Some(active_model.clone()),
        tool_name: tool.into(),
        risk_tier: runtime.tools.risk_tier(tool).expect("risk"),
    };
    if runtime
        .policy
        .evaluate_with_match(context.clone())
        .expect("policy")
        .requirement
        != ApprovalRequirement::None
    {
        runtime
            .add_policy_rule(
                None,
                None,
                Some(tool),
                None,
                ApprovalRequirement::None,
                Some("provider effect recovery regression"),
            )
            .expect("allow write");
    }
    let decision = runtime
        .policy
        .evaluate_with_match(context)
        .expect("exact policy");
    let material: ExactApprovalMaterial = SafetyGateClient::prepare_candidate(
        runtime,
        session,
        &active_model,
        tool,
        arguments,
        &decision,
    )
    .expect("candidate");
    let epoch = runtime.capture_execution_epoch(session).expect("epoch");
    let lease = runtime.begin_execution_lease(epoch).expect("lease");
    let lease = lease
        .bind_tool_resources(runtime, session, tool, &material.canonical_arguments)
        .expect("sealed write");
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId("provider-effect-recovery".into()),
        &material,
        &material,
        lease,
    )
    .expect("authorized write")
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_provider_effect_completion_recovers_terminal_without_provider_replay() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("provider-effect.sqlite3");
    let workspace = crate::tool_workspace_root_path();
    let relative = format!(
        "artifacts/.architecture-v2-provider-effect-{}.txt",
        uuid::Uuid::new_v4()
    );
    let target = workspace.join(&relative);
    let arguments = json!({
        "path": relative,
        "content": "durable provider effect\n",
        "dryRun": false
    })
    .to_string();

    let runtime =
        RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("durable runtime");
    let execution = authorize_mutation(&runtime, "session-provider-effect", "write", &arguments);
    let attempt_id = execution.attempt_id().to_owned();
    let captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    let output: Value = serde_json::from_str(
        captured
            .tool_result()
            .and_then(|result| result.structured_json.as_deref())
            .expect("provider output"),
    )
    .expect("provider output JSON");
    assert_eq!(output["provider_effect_ack"]["status"], json!("committed"));
    assert!(
        output["provider_effect_ack_hash"]
            .as_str()
            .is_some_and(|hash| hash.starts_with("sha256:"))
    );
    captured.simulate_process_loss_after_provider_for_test();
    drop(runtime);

    let recovered =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("recovered runtime");
    let inspections = recovered
        .pending_execution_effect_inspections()
        .expect("read-only effect inspection");
    let [inspection] = inspections.as_slice() else {
        panic!("expected one pending provider effect: {inspections:?}");
    };
    assert_eq!(inspection.attempt_id, attempt_id);
    assert_eq!(
        inspection.state,
        ExecutionEffectInspectionState::AppliedAcknowledged
    );
    assert_eq!(inspection.target_path.as_deref(), target.to_str());
    assert!(inspection.effect_plan_hash.is_some());
    assert!(inspection.effect_ack_hash.is_some());
    assert!(
        recovered
            .ensure_outcome_dispatch_open()
            .expect_err("pending effect must keep provider dispatch closed")
            .0
            .contains("in doubt")
    );
    assert!(
        recovered
            .outcome_record_by_attempt(&attempt_id)
            .expect("outcome read")
            .is_none()
    );
    assert_eq!(recovered.tools.provider_invocation_count("write"), 0);
    assert_eq!(
        recovered
            .reconcile_pending_outcome(&attempt_id)
            .expect("exact completion capsule must materialize terminal only"),
        OutcomeRecordResult::Recorded
    );
    assert_eq!(recovered.tools.provider_invocation_count("write"), 0);
    assert!(
        recovered
            .pending_execution_intents()
            .expect("resolved execution intents")
            .is_empty()
    );
    let terminal = recovered
        .outcome_record_by_attempt(&attempt_id)
        .expect("terminal readback")
        .expect("recovered terminal");
    assert!(
        terminal
            .canonical_evidence_hash()
            .as_str()
            .starts_with("sha256:")
    );
    drop(recovered);

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("second restart");
    assert!(reopened.ensure_outcome_dispatch_open().is_ok());
    assert_eq!(reopened.tools.provider_invocation_count("write"), 0);
    assert!(
        reopened
            .pending_execution_intents()
            .expect("no pending provider replay")
            .is_empty()
    );
    assert_eq!(
        reopened
            .outcome_record_by_attempt(&attempt_id)
            .expect("persisted terminal")
            .expect("terminal after second restart"),
        terminal
    );
    drop(reopened);
    fs::remove_file(target).expect("remove provider effect fixture");
}

#[cfg(all(unix, test))]
#[tokio::test]
async fn architecture_v2_post_commit_provider_error_recovers_exact_failed_terminal() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory
        .path()
        .join("provider-effect-post-commit-error.sqlite3");
    let workspace = crate::tool_workspace_root_path();
    let relative = format!(
        "artifacts/.architecture-v2-provider-effect-error-{}.txt",
        uuid::Uuid::new_v4()
    );
    let target = workspace.join(&relative);
    let arguments = json!({
        "path": relative,
        "content": "committed before provider error\n",
        "dryRun": false
    })
    .to_string();

    let runtime =
        RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("durable runtime");
    let execution = authorize_mutation(
        &runtime,
        "session-provider-effect-error",
        "write",
        &arguments,
    );
    let attempt_id = execution.attempt_id().to_owned();
    crate::inject_atomic_install_post_commit_failure_for_test();
    let captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    let error = captured
        .outward_error()
        .expect("post-commit provider error must remain outwardly visible");
    assert!(error.0.starts_with("mutation_durability_ambiguous:"));
    let output: Value = serde_json::from_str(
        captured
            .tool_result()
            .and_then(|result| result.structured_json.as_deref())
            .expect("synthetic provider completion output"),
    )
    .expect("synthetic provider completion JSON");
    assert_eq!(output["provider_error_after_commit"], json!(true));
    assert_eq!(output["provider_effect_ack"]["status"], json!("committed"));
    assert_eq!(
        fs::read_to_string(&target).expect("committed target"),
        "committed before provider error\n"
    );
    captured.simulate_process_loss_after_provider_for_test();
    drop(runtime);

    let recovered =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("recovered runtime");
    assert_eq!(recovered.tools.provider_invocation_count("write"), 0);
    assert_eq!(
        recovered
            .reconcile_pending_outcome(&attempt_id)
            .expect("exact failed completion capsule"),
        OutcomeRecordResult::Recorded
    );
    assert_eq!(recovered.tools.provider_invocation_count("write"), 0);
    let terminal = recovered
        .outcome_record_by_attempt(&attempt_id)
        .expect("terminal readback")
        .expect("failed terminal");
    assert!(matches!(
        terminal.receipt().status(),
        hepta_contracts::OutcomeStatus::Failed { error_code }
            if error_code == "mutation_durability_ambiguous"
    ));
    let evidence = terminal.canonical_evidence();
    assert!(evidence.contains(
        r#"["terminal.code","mutation_durability_ambiguous"]"#
    ));
    assert!(evidence.contains(r#"["provider_output.presence","present"]"#));
    assert!(!evidence.contains("provider_error_after_commit"));
    drop(recovered);
    fs::remove_file(target).expect("remove post-commit provider error fixture");
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn architecture_v2_provider_effect_tts_ack_survives_restart() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("provider-effect-tts.sqlite3");
    let workspace = crate::tool_workspace_root_path();
    let relative = format!(
        "artifacts/.architecture-v2-provider-effect-tts-{}.aiff",
        uuid::Uuid::new_v4()
    );
    let target = workspace.join(&relative);
    let arguments = json!({
        "filename": relative,
        "text": "durable staged speech",
        "dryRun": false
    })
    .to_string();

    let runtime =
        RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("durable runtime");
    let execution = authorize_mutation(&runtime, "session-provider-effect-tts", "tts", &arguments);
    let attempt_id = execution.attempt_id().to_owned();
    let captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    let output: Value = serde_json::from_str(
        captured
            .tool_result()
            .and_then(|result| result.structured_json.as_deref())
            .expect("provider output"),
    )
    .expect("provider output JSON");
    assert_eq!(
        output["result"]["synthesis_staged_before_intent"],
        json!(true)
    );
    assert_eq!(output["provider_effect_ack"]["status"], json!("committed"));
    assert!(fs::metadata(&target).expect("committed TTS target").len() > 0);
    captured.simulate_process_loss_after_provider_for_test();
    drop(runtime);

    let recovered =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("recovered runtime");
    let inspections = recovered
        .pending_execution_effect_inspections()
        .expect("read-only effect inspection");
    let [inspection] = inspections.as_slice() else {
        panic!("expected one pending TTS provider effect: {inspections:?}");
    };
    assert_eq!(inspection.attempt_id, attempt_id);
    assert_eq!(
        inspection.state,
        ExecutionEffectInspectionState::AppliedAcknowledged
    );
    assert_eq!(inspection.target_path.as_deref(), target.to_str());
    drop(recovered);
    fs::remove_file(target).expect("remove TTS provider effect fixture");
}
