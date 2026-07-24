use crate::ExecutionBus;
use crate::HeptaError;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::FilesystemScope;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::ToolCallRequest;
use hepta_core::ToolContext;
use hepta_core::WritePathScope;
use std::fs;

fn candidate(runtime: &RuntimeKernel, session: &str, arguments: &str) -> ExactApprovalMaterial {
    let active_model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(SessionId(session.into())),
            model: Some(active_model.clone()),
            tool_name: "write_file".into(),
            risk_tier: runtime.tools.risk_tier("write_file").expect("risk"),
        })
        .expect("exact policy decision");
    SafetyGateClient::prepare_candidate(
        runtime,
        session,
        &active_model,
        "write_file",
        arguments,
        &decision,
    )
    .expect("candidate")
}

fn authorize(
    runtime: &RuntimeKernel,
    session: &str,
    arguments: &str,
    correlation: &str,
) -> Result<AuthorizedToolExecution, HeptaError> {
    let material = candidate(runtime, session, arguments);
    let epoch = runtime.capture_execution_epoch(session)?;
    let lease = runtime.begin_execution_lease(epoch)?;
    let lease = lease.bind_tool_resources(
        runtime,
        session,
        "write_file",
        &material.canonical_arguments,
    )?;
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId(correlation.into()),
        &material,
        &material,
        lease,
    )
}

fn allow_write_file(runtime: &RuntimeKernel) {
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("symlink reservation test"),
        )
        .expect("write policy");
}

fn create_arguments(path: &str, content: &str) -> String {
    serde_json::json!({
        "path": path,
        "content": content,
        "mode": "create"
    })
    .to_string()
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_symlink_reservation_nonexistent_leaf_uses_sealed_dispatch() {
    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let workspace = crate::tool_workspace_root_path();
    let parent_name = format!(".architecture-v2-sealed-parent-{}", uuid::Uuid::new_v4());
    let relative = format!("artifacts/{parent_name}/leaf.txt");
    let parent = workspace.join("artifacts").join(&parent_name);
    let target = workspace.join(&relative);
    fs::create_dir_all(&parent).expect("sealed parent");
    let arguments = create_arguments(&relative, "sealed-content");

    let direct_error = runtime
        .tools
        .invoke(
            "write_file",
            ToolContext {
                session_id: Some(SessionId("session-main".into())),
                correlation_id: Some(CorrelationId("unreserved".into())),
                execution_attempt_id: None,
                idempotency_key: None,
            },
            ToolCallRequest {
                name: "write_file".into(),
                input_json: arguments.clone(),
            },
        )
        .await
        .expect_err("path-based write provider must be unreachable");
    assert!(
        direct_error
            .0
            .contains("identity-bound execution reservation")
    );

    let execution =
        authorize(&runtime, "session-main", &arguments, "sealed-create").expect("authorize");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("receipt");
    assert_eq!(
        fs::read_to_string(&target).expect("target"),
        "sealed-content"
    );

    fs::remove_file(target).expect("remove target");
    fs::remove_dir(parent).expect("remove parent");
}

#[cfg(unix)]
#[test]
fn architecture_v2_symlink_reservation_scope_combinations_reject_external_ancestor() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    let outside = tempfile::tempdir().expect("outside directory");
    let link_name = format!(".architecture-v2-external-link-{}", uuid::Uuid::new_v4());
    let link = workspace.join("artifacts").join(&link_name);
    fs::create_dir_all(link.parent().expect("artifacts root")).expect("artifacts root");
    symlink(outside.path(), &link).expect("external ancestor symlink");
    let arguments = create_arguments(
        &format!("artifacts/{link_name}/missing.txt"),
        "must-not-escape",
    );

    let artifacts_runtime = RuntimeKernel::new();
    artifacts_runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("filesystem any");
    let artifacts_error = artifacts_runtime
        .prepare_write_transaction_with_lock_check("session-main", "write_file", &arguments)
        .expect_err("ArtifactsOnly must reject canonical external ancestor");
    assert!(artifacts_error.0.contains("outside artifacts root"));

    let workspace_runtime = RuntimeKernel::new();
    workspace_runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("filesystem any");
    workspace_runtime
        .switch_write_path_scope(WritePathScope::WorkspaceOnly)
        .expect("workspace writes");
    let workspace_error = workspace_runtime
        .prepare_write_transaction_with_lock_check("session-main", "write_file", &arguments)
        .expect_err("WorkspaceOnly must reject canonical external ancestor");
    assert!(workspace_error.0.contains("outside workspace"));

    let combined_runtime = RuntimeKernel::new();
    combined_runtime
        .switch_filesystem_scope(FilesystemScope::WorkspaceOnly)
        .expect("filesystem workspace");
    combined_runtime
        .switch_write_path_scope(WritePathScope::AnyPath)
        .expect("write any");
    let combined_error = combined_runtime
        .prepare_write_transaction_with_lock_check("session-main", "write_file", &arguments)
        .expect_err("filesystem WorkspaceOnly must dominate write AnyPath");
    assert!(combined_error.0.contains("filesystem scope workspace_only"));

    assert!(!outside.path().join("missing.txt").exists());
    fs::remove_file(link).expect("remove external symlink");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_symlink_reservation_replacement_after_authorization_fails_closed() {
    use std::os::unix::fs::symlink;

    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let workspace = crate::tool_workspace_root_path();
    let outside = tempfile::tempdir().expect("outside directory");
    let parent_name = format!(".architecture-v2-replaced-parent-{}", uuid::Uuid::new_v4());
    let relative = format!("artifacts/{parent_name}/leaf.txt");
    let parent = workspace.join("artifacts").join(&parent_name);
    let retained_parent = workspace
        .join("artifacts")
        .join(format!("{parent_name}-retained"));
    fs::create_dir_all(&parent).expect("authorized parent");
    let arguments = create_arguments(&relative, "must-not-escape");
    let execution = authorize(
        &runtime,
        "session-main",
        &arguments,
        "replace-after-authorize",
    )
    .expect("authorization should retain the real parent handle");

    fs::rename(&parent, &retained_parent).expect("move authorized parent");
    symlink(outside.path(), &parent).expect("replace parent with external symlink");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    match captured.terminal() {
        CapturedDispatchTerminal::DispatchBlocked(error) => {
            assert!(error.contains("sealed write ancestor"));
        }
        terminal => panic!("symlink replacement must fail before provider dispatch: {terminal:?}"),
    }
    assert_eq!(runtime.tools.provider_invocation_count("write_file"), 0);
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("failed dispatch receipt");
    assert!(!outside.path().join("leaf.txt").exists());
    assert!(!retained_parent.join("leaf.txt").exists());

    fs::remove_file(&parent).expect("remove replacement symlink");
    fs::rename(&retained_parent, &parent).expect("restore authorized parent");
    fs::remove_dir(parent).expect("remove restored parent");
}

#[cfg(unix)]
#[test]
fn architecture_v2_symlink_reservation_aliases_share_one_canonical_reservation() {
    use std::os::unix::fs::symlink;

    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let real_name = format!(".architecture-v2-real-parent-{suffix}");
    let alias_name = format!(".architecture-v2-alias-parent-{suffix}");
    let real_parent = workspace.join("artifacts").join(&real_name);
    let alias_parent = workspace.join("artifacts").join(&alias_name);
    fs::create_dir_all(&real_parent).expect("real parent");
    symlink(&real_parent, &alias_parent).expect("in-workspace alias");
    let alias_arguments = create_arguments(&format!("artifacts/{alias_name}/leaf.txt"), "alias");
    let real_arguments = create_arguments(&format!("artifacts/{real_name}/leaf.txt"), "real");

    let first = authorize(
        &runtime,
        "session-main",
        &alias_arguments,
        "canonical-alias",
    )
    .expect("first alias should own the canonical target");
    let second_error = authorize(&runtime, "session-other", &real_arguments, "canonical-real")
        .expect_err("second path to the same canonical target must conflict");
    assert!(second_error.0.contains("tool_execution_reservation"));
    assert_eq!(
        runtime
            .write_lock_state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .active_target_reservations
            .len(),
        1
    );

    drop(first);
    assert!(
        runtime
            .write_lock_state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .active_target_reservations
            .is_empty()
    );
    fs::remove_file(alias_parent).expect("remove alias");
    fs::remove_dir(real_parent).expect("remove real parent");
}
