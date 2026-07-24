use crate::ExecutionBus;
use crate::HeptaError;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::ToolRegistry;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use crate::runtime_kernel::execution_bus::CapturedDispatchTerminal;
use crate::runtime_kernel::execution_bus::CapturedTransaction;
use hepta_contracts::OutcomeStatus;
use hepta_core::CorrelationId;
use hepta_core::FilesystemScope;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use serde_json::Value;
use std::fs;

fn material(
    runtime: &RuntimeKernel,
    session: &str,
    tool_name: &str,
    arguments: &str,
) -> ExactApprovalMaterial {
    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(SessionId(session.into())),
            model: Some(model.clone()),
            tool_name: tool_name.into(),
            risk_tier: runtime.tools.risk_tier(tool_name).expect("risk"),
        })
        .expect("policy");
    SafetyGateClient::prepare_candidate(runtime, session, &model, tool_name, arguments, &decision)
        .expect("candidate")
}

fn authorize(
    runtime: &RuntimeKernel,
    session: &str,
    tool_name: &str,
    arguments: &str,
    correlation: &str,
) -> Result<AuthorizedToolExecution, HeptaError> {
    let candidate = material(runtime, session, tool_name, arguments);
    let epoch = runtime.capture_execution_epoch(session)?;
    let lease = runtime.begin_execution_lease(epoch)?;
    let lease =
        lease.bind_tool_resources(runtime, session, tool_name, &candidate.canonical_arguments)?;
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId(correlation.into()),
        &candidate,
        &candidate,
        lease,
    )
}

fn unique_relative(prefix: &str) -> String {
    format!(
        "artifacts/.architecture-v2-sealed-read-{prefix}-{}",
        uuid::Uuid::new_v4()
    )
}

fn captured_json(
    captured: &crate::runtime_kernel::execution_bus::CapturedToolExecution<'_>,
) -> Value {
    serde_json::from_str(
        captured
            .tool_result()
            .and_then(|result| result.structured_json.as_deref())
            .expect("structured output"),
    )
    .expect("valid output")
}

fn finalize(
    runtime: &RuntimeKernel,
    captured: &mut crate::runtime_kernel::execution_bus::CapturedToolExecution<'_>,
) {
    captured.capture_write_transaction();
    assert!(matches!(
        captured.transaction(),
        CapturedTransaction::NotApplicable
    ));
    OutcomeRecorder::new(runtime)
        .finalize_tool_dispatch(captured)
        .expect("terminal receipt");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_sealed_read_serves_captured_read_file_read_and_memory_get_bytes() {
    let workspace = crate::tool_workspace_root_path();
    let relative = unique_relative("success.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "alpha\nbeta\ngamma\n").expect("fixture");
    let memory_relative = format!(
        "memory/.architecture-v2-sealed-read-{}.md",
        uuid::Uuid::new_v4()
    );
    let memory_target = workspace.join(&memory_relative);
    fs::create_dir_all(memory_target.parent().expect("memory parent")).expect("memory dir");
    fs::write(&memory_target, "remembered\nstable\n").expect("memory fixture");

    let runtime = RuntimeKernel::new();
    let read_file_arguments = serde_json::json!({"path": relative}).to_string();
    let read_file = authorize(
        &runtime,
        "session-main",
        "read_file",
        &read_file_arguments,
        "read-file",
    )
    .expect("sealed read_file authorization");
    assert!(read_file.holds_read_capability());
    assert!(!read_file.holds_write_target_reservation());
    let mut captured = ExecutionBus::new(&runtime).dispatch(read_file).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    assert_eq!(captured_json(&captured)["line_count"], serde_json::json!(3));
    finalize(&runtime, &mut captured);

    let native_arguments =
        serde_json::json!({"path": relative, "offset": 2, "limit": 1}).to_string();
    let native = authorize(
        &runtime,
        "session-main",
        "read",
        &native_arguments,
        "native-read",
    )
    .expect("sealed native read authorization");
    let mut captured = ExecutionBus::new(&runtime).dispatch(native).await;
    assert_eq!(
        captured_json(&captured)["result"]["text"],
        serde_json::json!("beta")
    );
    finalize(&runtime, &mut captured);

    let memory_arguments =
        serde_json::json!({"path": memory_relative, "from": 2, "lines": 1}).to_string();
    let memory = authorize(
        &runtime,
        "session-main",
        "memory_get",
        &memory_arguments,
        "memory-read",
    )
    .expect("sealed memory_get authorization");
    let mut captured = ExecutionBus::new(&runtime).dispatch(memory).await;
    assert_eq!(
        captured_json(&captured)["result"]["text"],
        serde_json::json!("stable")
    );
    finalize(&runtime, &mut captured);

    assert_eq!(runtime.tools.provider_invocation_count("read_file"), 1);
    assert_eq!(runtime.tools.provider_invocation_count("read"), 1);
    assert_eq!(runtime.tools.provider_invocation_count("memory_get"), 1);
    fs::remove_file(target).expect("remove read fixture");
    fs::remove_file(memory_target).expect("remove memory fixture");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_sealed_read_absolute_scope_is_checked_before_provider() {
    let outside = tempfile::tempdir().expect("outside");
    let outside_file = outside.path().join("outside.txt");
    fs::write(&outside_file, "outside\n").expect("outside fixture");
    let canonical_outside = fs::canonicalize(&outside_file).expect("canonical outside");
    let arguments = serde_json::json!({"path": canonical_outside}).to_string();

    let scoped = RuntimeKernel::new();
    let error = authorize(&scoped, "session-main", "read", &arguments, "scope-denied")
        .expect_err("workspace scope must reject external absolute read");
    assert!(error.0.contains("outside workspace"));
    assert_eq!(scoped.tools.provider_invocation_count("read"), 0);

    let any_path = RuntimeKernel::new();
    any_path
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("any path");
    let execution = authorize(
        &any_path,
        "session-main",
        "read",
        &arguments,
        "scope-allowed",
    )
    .expect("canonical external path should seal under AnyPath");
    let mut captured = ExecutionBus::new(&any_path).dispatch(execution).await;
    assert_eq!(
        captured_json(&captured)["result"]["text"],
        serde_json::json!("outside")
    );
    finalize(&any_path, &mut captured);
}

#[cfg(unix)]
#[test]
fn architecture_v2_sealed_read_rejects_symlink_ancestors_and_hardlink_aliases() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    let outside = tempfile::tempdir().expect("outside");
    fs::write(outside.path().join("secret.txt"), "secret").expect("outside fixture");
    let link_relative = unique_relative("ancestor-link");
    let link = workspace.join(&link_relative);
    symlink(outside.path(), &link).expect("ancestor link");
    let symlink_arguments =
        serde_json::json!({"path": format!("{link_relative}/secret.txt")}).to_string();
    let runtime = RuntimeKernel::new();
    let symlink_error = authorize(
        &runtime,
        "session-main",
        "read",
        &symlink_arguments,
        "symlink",
    )
    .expect_err("symlink ancestor must be rejected");
    assert!(
        symlink_error
            .0
            .contains("sealed read refused directory component")
    );
    assert_eq!(runtime.tools.provider_invocation_count("read"), 0);

    let source_relative = unique_relative("hardlink-source.txt");
    let alias_relative = unique_relative("hardlink-alias.txt");
    let source = workspace.join(&source_relative);
    let alias = workspace.join(&alias_relative);
    fs::write(&source, "hardlinked").expect("hardlink source");
    fs::hard_link(&source, &alias).expect("hardlink alias");
    let hardlink_arguments = serde_json::json!({"path": alias_relative}).to_string();
    let hardlink_error = authorize(
        &runtime,
        "session-main",
        "read",
        &hardlink_arguments,
        "hardlink",
    )
    .expect_err("hardlink read target must be rejected");
    assert!(hardlink_error.0.contains("hard links"));
    assert_eq!(runtime.tools.provider_invocation_count("read"), 0);

    fs::remove_file(link).expect("remove symlink");
    fs::remove_file(alias).expect("remove alias");
    fs::remove_file(source).expect("remove source");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_sealed_read_rejects_ancestor_swap_before_provider_invocation() {
    use std::os::unix::fs::symlink;

    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let parent_relative = unique_relative("swap-parent");
    let parent = workspace.join(&parent_relative);
    let retained = workspace.join(format!("{parent_relative}-retained"));
    fs::create_dir(&parent).expect("parent");
    fs::write(parent.join("value.txt"), "authorized\n").expect("authorized fixture");
    let arguments = serde_json::json!({"path": format!("{parent_relative}/value.txt")}).to_string();
    let execution = authorize(
        &runtime,
        "session-main",
        "read",
        &arguments,
        "ancestor-swap",
    )
    .expect("authorization");

    let outside = tempfile::tempdir().expect("outside");
    fs::write(outside.path().join("value.txt"), "outside\n").expect("outside fixture");
    fs::rename(&parent, &retained).expect("retain parent");
    symlink(outside.path(), &parent).expect("replacement symlink");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::ToolError(error)
            if error.contains("sealed read refused directory component")
    ));
    assert_eq!(runtime.tools.provider_invocation_count("read"), 0);
    finalize(&runtime, &mut captured);

    fs::remove_file(&parent).expect("remove replacement");
    fs::rename(&retained, &parent).expect("restore parent");
    fs::remove_file(parent.join("value.txt")).expect("remove fixture");
    fs::remove_dir(parent).expect("remove parent");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_sealed_read_rejects_leaf_replacement_before_provider_invocation() {
    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let relative = unique_relative("leaf-swap.txt");
    let retained_relative = format!("{relative}-retained");
    let target = workspace.join(&relative);
    let retained = workspace.join(&retained_relative);
    fs::write(&target, "authorized\n").expect("fixture");
    let arguments = serde_json::json!({"path": relative}).to_string();
    let execution = authorize(
        &runtime,
        "session-main",
        "read_file",
        &arguments,
        "leaf-swap",
    )
    .expect("authorization");

    fs::rename(&target, &retained).expect("retain authorized inode");
    fs::write(&target, "replacement\n").expect("replacement");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::ToolError(error)
            if error.contains("target identity changed")
    ));
    assert_eq!(runtime.tools.provider_invocation_count("read_file"), 0);
    finalize(&runtime, &mut captured);

    fs::remove_file(target).expect("remove replacement");
    fs::remove_file(retained).expect("remove retained");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_sealed_read_uses_authorized_bytes_after_in_place_change() {
    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let relative = unique_relative("stable-bytes.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "authorized-bytes\n").expect("fixture");
    let arguments = serde_json::json!({"path": relative}).to_string();
    let execution = authorize(
        &runtime,
        "session-main",
        "read_file",
        &arguments,
        "stable-bytes",
    )
    .expect("authorization");

    fs::write(&target, "changed-after-authorization\n").expect("in-place update");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    assert!(matches!(
        captured.terminal(),
        CapturedDispatchTerminal::Succeeded
    ));
    assert_eq!(
        captured_json(&captured)["preview"],
        serde_json::json!("authorized-bytes")
    );
    assert_eq!(runtime.tools.provider_invocation_count("read_file"), 1);
    finalize(&runtime, &mut captured);
    fs::remove_file(target).expect("remove fixture");
}

#[cfg(unix)]
#[test]
fn architecture_v2_sealed_read_memory_get_rejects_non_memory_paths_before_provider() {
    let runtime = RuntimeKernel::new();
    for path in [
        "README.md",
        "memory/nested/value.md",
        "memory/value.txt",
        "../MEMORY.md",
        "/tmp/MEMORY.md",
    ] {
        let arguments = serde_json::json!({"path": path}).to_string();
        let error = authorize(
            &runtime,
            "session-main",
            "memory_get",
            &arguments,
            "memory-scope",
        )
        .expect_err("memory_get path must fail closed");
        assert!(
            error
                .0
                .contains("only permits relative MEMORY.md or memory/*.md"),
            "{path}: {error}"
        );
    }
    assert_eq!(runtime.tools.provider_invocation_count("memory_get"), 0);
}

#[test]
fn architecture_v2_sealed_read_quarantines_directory_media_and_generator_surfaces() {
    let runtime = RuntimeKernel::new();
    for tool in [
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(
            runtime.tools.risk_tier(tool).is_err(),
            "{tool} must not be production registered"
        );
    }
    let test_registry = ToolRegistry::new_with_all_quarantined_tools_for_test();
    for tool in [
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(
            test_registry.risk_tier(tool).is_ok(),
            "{tool} lower implementation must remain test reachable"
        );
    }
}

#[cfg(unix)]
#[test]
fn architecture_v2_sealed_read_drop_has_no_mutation_receipt() {
    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let relative = unique_relative("drop.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "drop\n").expect("fixture");
    let arguments = serde_json::json!({"path": relative}).to_string();
    let execution = authorize(&runtime, "session-main", "read_file", &arguments, "drop")
        .expect("authorization");
    let attempt_id = execution.attempt_id().to_string();
    assert!(execution.holds_read_capability());
    assert!(!execution.holds_write_target_reservation());
    drop(execution);

    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("outcome read")
        .expect("drop receipt");
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code }
            if reason_code == "tool.dispatch_future_dropped"
    ));
    assert!(record.canonical_evidence().contains("not_applicable"));
    fs::remove_file(target).expect("remove fixture");
}
