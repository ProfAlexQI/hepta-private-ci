use std::fs;

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
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::WritePathScope;

fn material(runtime: &RuntimeKernel, session: &str, arguments: &str) -> ExactApprovalMaterial {
    let active_model = runtime.model_selection().expect("active model").active;
    let context = || PolicyEvaluationContext {
        session_id: Some(SessionId(session.into())),
        model: Some(active_model.clone()),
        tool_name: "write_file".into(),
        risk_tier: runtime.tools.risk_tier("write_file").expect("risk"),
    };
    if runtime
        .policy
        .evaluate_with_match(context())
        .expect("current policy")
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
                Some("process reservation test"),
            )
            .expect("write policy");
    }
    let decision = runtime
        .policy
        .evaluate_with_match(context())
        .expect("exact policy");
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
    let material = material(runtime, session, arguments);
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

fn arguments(path: &str, mode: &str, content: &str) -> String {
    serde_json::json!({
        "path": path,
        "content": content,
        "mode": mode,
    })
    .to_string()
}

fn assert_conflict_before_provider(runtime: &RuntimeKernel, arguments: &str, correlation: &str) {
    let error = authorize(runtime, "session-contender", arguments, correlation)
        .expect_err("second runtime must fail before dispatch");
    assert!(
        error.0.contains("tool_execution_reservation"),
        "{}",
        error.0
    );
    assert_eq!(runtime.tools.provider_invocation_count("write_file"), 0);
}

fn assert_reacquired(runtime: &RuntimeKernel, arguments: &str, correlation: &str) {
    let execution = authorize(runtime, "session-contender", arguments, correlation)
        .expect("released process reservation must be reusable");
    drop(execution);
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_existing_overwrite_and_append_are_global() {
    let workspace = crate::tool_workspace_root_path();
    for mode in ["overwrite", "append"] {
        let name = format!(
            ".architecture-v2-process-existing-{mode}-{}.txt",
            uuid::Uuid::new_v4()
        );
        let relative = format!("artifacts/{name}");
        let target = workspace.join(&relative);
        fs::write(&target, "before").expect("existing target");
        let input = arguments(&relative, mode, "after");
        let first_runtime = RuntimeKernel::new();
        let second_runtime = RuntimeKernel::new();

        let first = authorize(&first_runtime, "session-holder", &input, "existing-holder")
            .expect("first runtime reservation");
        assert_conflict_before_provider(&second_runtime, &input, "existing-contender");
        drop(first);
        assert_reacquired(&second_runtime, &input, "existing-reacquired");

        fs::remove_file(target).expect("remove existing target");
    }
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_missing_leaf_is_global_and_reusable() {
    let workspace = crate::tool_workspace_root_path();
    let name = format!(
        ".architecture-v2-process-missing-{}.txt",
        uuid::Uuid::new_v4()
    );
    let relative = format!("artifacts/{name}");
    let input = arguments(&relative, "create", "new");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();

    let first = authorize(&first_runtime, "session-holder", &input, "missing-holder")
        .expect("first missing reservation");
    assert_conflict_before_provider(&second_runtime, &input, "missing-contender");
    drop(first);
    assert_reacquired(&second_runtime, &input, "missing-reacquired");

    assert!(!workspace.join(relative).exists());
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_existing_hardlinks_fail_closed() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let real_relative = format!(".architecture-v2-process-hardlink-source-{suffix}.txt");
    let alias_relative = format!("artifacts/.architecture-v2-process-hardlink-{suffix}.txt");
    let real = workspace.join(&real_relative);
    let alias = workspace.join(&alias_relative);
    fs::write(&real, "before").expect("real target");
    fs::hard_link(&real, &alias).expect("hardlink alias");
    let input = arguments(&alias_relative, "append", "second");
    let runtime = RuntimeKernel::new();
    runtime
        .switch_write_path_scope(WritePathScope::ArtifactsOnly)
        .expect("artifacts write scope");

    let error = authorize(&runtime, "session-holder", &input, "hardlink-holder")
        .expect_err("hardlink target must fail before reservation");
    assert!(error.0.contains("hard links"), "{}", error.0);
    assert_eq!(runtime.tools.provider_invocation_count("write_file"), 0);
    assert_eq!(fs::read_to_string(&real).expect("real target"), "before");

    fs::remove_file(alias).expect("remove hardlink");
    fs::remove_file(real).expect("remove real target");
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_symlink_aliases_share_canonical_namespace() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let real_name = format!(".architecture-v2-process-parent-{suffix}");
    let alias_name = format!(".architecture-v2-process-parent-alias-{suffix}");
    let real_parent = workspace.join("artifacts").join(&real_name);
    let alias_parent = workspace.join("artifacts").join(&alias_name);
    fs::create_dir(&real_parent).expect("real parent");
    symlink(&real_parent, &alias_parent).expect("parent alias");
    let real_relative = format!("artifacts/{real_name}/leaf.txt");
    let alias_relative = format!("artifacts/{alias_name}/leaf.txt");
    let real_input = arguments(&real_relative, "create", "real");
    let alias_input = arguments(&alias_relative, "create", "alias");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();

    let first = authorize(
        &first_runtime,
        "session-holder",
        &alias_input,
        "alias-holder",
    )
    .expect("alias holder");
    assert_conflict_before_provider(&second_runtime, &real_input, "alias-contender");
    drop(first);
    assert_reacquired(&second_runtime, &real_input, "alias-reacquired");

    fs::remove_file(alias_parent).expect("remove alias");
    fs::remove_dir(real_parent).expect("remove real parent");
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_parent_rename_keeps_anchor_identity() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let old_name = format!(".architecture-v2-process-old-{suffix}");
    let new_name = format!(".architecture-v2-process-new-{suffix}");
    let old_parent = workspace.join("artifacts").join(&old_name);
    let new_parent = workspace.join("artifacts").join(&new_name);
    fs::create_dir(&old_parent).expect("old parent");
    let old_relative = format!("artifacts/{old_name}/leaf.txt");
    let new_relative = format!("artifacts/{new_name}/leaf.txt");
    let old_input = arguments(&old_relative, "create", "old");
    let new_input = arguments(&new_relative, "create", "new");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();

    let first = authorize(
        &first_runtime,
        "session-holder",
        &old_input,
        "rename-holder",
    )
    .expect("rename holder");
    fs::rename(&old_parent, &new_parent).expect("rename sealed parent");
    assert_conflict_before_provider(&second_runtime, &new_input, "rename-contender");
    drop(first);
    assert_reacquired(&second_runtime, &new_input, "rename-reacquired");

    fs::remove_dir(new_parent).expect("remove renamed parent");
}

#[cfg(unix)]
#[test]
fn architecture_v2_process_reservation_unrelated_missing_targets_remain_parallel() {
    let suffix = uuid::Uuid::new_v4();
    let first_relative = format!("artifacts/.architecture-v2-process-a-{suffix}.txt");
    let second_relative = format!("artifacts/.architecture-v2-process-b-{suffix}.txt");
    let first_input = arguments(&first_relative, "create", "first");
    let second_input = arguments(&second_relative, "create", "second");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();

    let first = authorize(
        &first_runtime,
        "session-holder",
        &first_input,
        "parallel-first",
    )
    .expect("first unrelated reservation");
    let second = authorize(
        &second_runtime,
        "session-contender",
        &second_input,
        "parallel-second",
    )
    .expect("second unrelated reservation");
    assert_eq!(
        first_runtime.tools.provider_invocation_count("write_file"),
        0
    );
    assert_eq!(
        second_runtime.tools.provider_invocation_count("write_file"),
        0
    );
    drop(second);
    drop(first);
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_process_reservation_lives_through_terminal_receipt() {
    let workspace = crate::tool_workspace_root_path();
    let name = format!(
        ".architecture-v2-process-receipt-{}.txt",
        uuid::Uuid::new_v4()
    );
    let relative = format!("artifacts/{name}");
    let target = workspace.join(&relative);
    let input = arguments(&relative, "create", "receipt");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();
    let execution = authorize(&first_runtime, "session-holder", &input, "receipt-holder")
        .expect("receipt holder");

    let mut captured = ExecutionBus::new(&first_runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    captured.capture_write_transaction();
    assert_conflict_before_provider(&second_runtime, &input, "receipt-contender");
    OutcomeRecorder::new(&first_runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("terminal receipt");
    assert_reacquired(&second_runtime, &input, "receipt-reacquired");

    fs::remove_file(target).expect("remove written target");
}

#[cfg(target_os = "macos")]
#[test]
fn architecture_v2_process_reservation_ascii_case_aliases_follow_volume_policy() {
    let workspace = crate::tool_workspace_root_path();
    let artifacts = workspace.join("artifacts");
    let directory = fs::File::open(&artifacts).expect("artifacts directory");
    if !crate::directory_namespace_case_insensitive(&directory) {
        return;
    }
    let suffix = uuid::Uuid::new_v4();
    let upper = format!("artifacts/.ARCHITECTURE-V2-PROCESS-CASE-{suffix}.txt");
    let lower = upper.to_ascii_lowercase();
    let upper_input = arguments(&upper, "create", "upper");
    let lower_input = arguments(&lower, "create", "lower");
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();

    let first = authorize(
        &first_runtime,
        "session-holder",
        &upper_input,
        "case-holder",
    )
    .expect("case holder");
    assert_conflict_before_provider(&second_runtime, &lower_input, "case-contender");
    drop(first);
    assert_reacquired(&second_runtime, &lower_input, "case-reacquired");
}
