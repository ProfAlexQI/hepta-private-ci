use std::fs;

use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::approval_state::ExactApprovalMaterial;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;
use hepta_core::ApprovalRequirement;
use hepta_core::CorrelationId;
use hepta_core::FilesystemScope;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_core::WritePathScope;

fn allow_write_file(runtime: &RuntimeKernel) {
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("maintenance mutation test"),
        )
        .expect("write_file policy");
}

fn material(runtime: &RuntimeKernel, session: &str, arguments: &str) -> ExactApprovalMaterial {
    let active_model = runtime.model_selection().expect("active model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(SessionId(session.into())),
            model: Some(active_model.clone()),
            tool_name: "write_file".into(),
            risk_tier: runtime.tools.risk_tier("write_file").expect("risk"),
        })
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
) -> AuthorizedToolExecution {
    let material = material(runtime, session, arguments);
    let epoch = runtime
        .capture_execution_epoch(session)
        .expect("execution epoch");
    let lease = runtime
        .begin_execution_lease(epoch)
        .expect("execution lease");
    let lease = lease
        .bind_tool_resources(
            runtime,
            session,
            "write_file",
            &material.canonical_arguments,
        )
        .expect("sealed resources");
    SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &SessionId(session.into()),
        &CorrelationId(correlation.into()),
        &material,
        &material,
        lease,
    )
    .expect("authorized execution")
}

fn write_arguments(relative: &str, content: &str) -> String {
    serde_json::json!({
        "path": relative,
        "content": content,
        "mode": "overwrite",
    })
    .to_string()
}

fn output_field(output: &str, field: &str) -> String {
    serde_json::from_str::<serde_json::Value>(output)
        .expect("structured output")
        .get(field)
        .and_then(serde_json::Value::as_str)
        .expect("string field")
        .to_string()
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_maintenance_restore_and_rollback_conflict_across_runtimes() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-maintenance-{suffix}.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "before").expect("seed target");

    let owner = RuntimeKernel::new();
    allow_write_file(&owner);
    let result = owner
        .run_demo_turn(&format!("overwrite:{relative} => after"))
        .await
        .expect("seed transaction");
    let transaction_id = output_field(
        result.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    );
    let backup = owner
        .backup_index(Some(&relative))
        .expect("backup index")
        .backups
        .into_iter()
        .next()
        .expect("backup");

    let contender = RuntimeKernel::new();
    allow_write_file(&contender);
    let holder = authorize(
        &contender,
        "session-main",
        &write_arguments(&relative, "reserved"),
        "maintenance-holder",
    );
    let restore_error = owner
        .restore_backup(&backup.id)
        .expect_err("process reservation must block restore");
    assert!(restore_error.0.contains("tool_execution_reservation"));
    let rollback_error = owner
        .rollback_write_transaction(&transaction_id)
        .expect_err("process reservation must block rollback");
    assert!(rollback_error.0.contains("tool_execution_reservation"));
    assert_eq!(fs::read_to_string(&target).expect("target"), "after");
    assert_eq!(contender.tools.provider_invocation_count("write_file"), 0);

    drop(holder);
    let restore = owner.restore_backup(&backup.id).expect("released restore");
    assert_eq!(
        fs::read_to_string(&target).expect("restored target"),
        "before"
    );
    let rollback_error = owner
        .rollback_write_transaction(&transaction_id)
        .expect_err("restore changed the recorded post-state");
    assert!(
        rollback_error
            .0
            .contains("rollback target contents changed")
    );
    assert_eq!(
        fs::read_to_string(&target).expect("rolled back target"),
        "before"
    );

    fs::remove_file(&target).expect("remove target");
    fs::remove_file(&backup.backup_path).expect("remove source backup");
    if let Some(path) = restore.previous_target_backup_path {
        fs::remove_file(path).expect("remove restore safety backup");
    }
}

#[cfg(unix)]
#[test]
fn architecture_v2_maintenance_group_rollback_reserves_process_identity_set() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-group-global-{suffix}.txt");
    let target = workspace.join(&relative);
    let other_relative = format!("artifacts/.architecture-v2-group-other-{suffix}.txt");
    let other_target = workspace.join(&other_relative);
    fs::write(&target, "before").expect("seed target");
    fs::write(&other_target, "other").expect("seed other target");
    let owner = RuntimeKernel::new();
    let contender = RuntimeKernel::new();
    allow_write_file(&contender);

    let reservation = owner
        .acquire_group_rollback_locks(
            "session-main",
            "group-process-global",
            "attempt-process-global",
            &[target.display().to_string()],
        )
        .expect("group lock");
    let steal_error = contender
        .acquire_group_rollback_locks(
            "session-other",
            "group-process-global",
            "attempt-stolen",
            &[other_target.display().to_string()],
        )
        .expect_err("another runtime cannot steal the same public group id");
    assert!(steal_error.0.contains("rollback_group"));
    let wrong_release_error = contender
        .release_group_rollback_reservation(&reservation)
        .expect_err("another runtime cannot release the owner token");
    assert!(wrong_release_error.0.contains("does not own"));
    let error = contender
        .prepare_write_transactions_with_lock_check(
            "session-main",
            "write_file",
            &write_arguments(&relative, "contender"),
        )
        .expect_err("second runtime must observe group reservation");
    assert!(error.0.contains("rollback_group"));
    owner
        .release_group_rollback_reservation(&reservation)
        .expect("release group");
    let stale_owner_error = owner
        .prepare_sealed_write_target(
            "session-main",
            "rollback_write_transaction",
            "rollback_write_transaction",
            &target.display().to_string(),
            "overwrite",
            false,
            Some(&reservation),
        )
        .expect_err("released group owner must not authorize a sealed mutation");
    assert!(
        stale_owner_error
            .0
            .contains("does not hold the exact sealed identity")
    );
    let prepared = contender
        .prepare_write_transactions_with_lock_check(
            "session-main",
            "write_file",
            &write_arguments(&relative, "released"),
        )
        .expect("released identity");
    drop(prepared);
    fs::remove_file(target).expect("remove target");
    fs::remove_file(other_target).expect("remove other target");
}

#[cfg(unix)]
#[test]
fn architecture_v2_maintenance_group_step_rejects_path_identity_rebind() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-group-rebind-{suffix}.txt");
    let target = workspace.join(&relative);
    let retained = workspace.join(format!(
        "artifacts/.architecture-v2-group-rebind-retained-{suffix}.txt"
    ));
    fs::write(&target, "captured").expect("seed target");
    let runtime = RuntimeKernel::new();
    let reservation = runtime
        .acquire_group_rollback_locks(
            "session-main",
            "group-rebind",
            "attempt-rebind",
            &[target.display().to_string()],
        )
        .expect("group lock");

    fs::rename(&target, &retained).expect("retain captured inode");
    fs::write(&target, "replacement").expect("replace target identity");
    let rebind_error = runtime
        .prepare_sealed_write_target(
            "session-main",
            "rollback_write_transaction",
            "rollback_write_transaction",
            &target.display().to_string(),
            "overwrite",
            false,
            Some(&reservation),
        )
        .expect_err("group step must require the exact reserved identity");
    assert!(
        rebind_error
            .0
            .contains("does not hold the exact sealed identity")
    );
    assert_eq!(
        fs::read_to_string(&target).expect("replacement"),
        "replacement"
    );

    runtime
        .release_group_rollback_reservation(&reservation)
        .expect("release exact owner token");
    fs::remove_file(target).expect("remove replacement");
    fs::remove_file(retained).expect("remove retained target");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_maintenance_stored_target_cannot_rebind_through_symlink() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let parent_name = format!(".architecture-v2-maintenance-parent-{suffix}");
    let retained_name = format!(".architecture-v2-maintenance-retained-{suffix}");
    let parent = workspace.join("artifacts").join(&parent_name);
    let retained = workspace.join("artifacts").join(&retained_name);
    fs::create_dir(&parent).expect("parent");
    let relative = format!("artifacts/{parent_name}/leaf.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "before").expect("seed target");

    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let result = runtime
        .run_demo_turn(&format!("overwrite:{relative} => after"))
        .await
        .expect("seed transaction");
    let transaction_id = output_field(
        result.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    );
    let backup = runtime
        .backup_index(Some(&relative))
        .expect("backup index")
        .backups
        .into_iter()
        .next()
        .expect("backup");
    runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("any filesystem");
    runtime
        .switch_write_path_scope(WritePathScope::AnyPath)
        .expect("any write path");

    let outside = tempfile::tempdir().expect("outside");
    fs::rename(&parent, &retained).expect("retain authorized parent");
    symlink(outside.path(), &parent).expect("external replacement");

    let restore_error = runtime
        .restore_backup(&backup.id)
        .expect_err("restore target rebind must fail closed");
    assert!(restore_error.0.contains("identity changed"));
    let rollback_error = runtime
        .rollback_write_transaction(&transaction_id)
        .expect_err("rollback target rebind must fail closed");
    assert!(rollback_error.0.contains("identity changed"));
    assert!(!outside.path().join("leaf.txt").exists());

    fs::remove_file(&parent).expect("remove replacement");
    fs::rename(&retained, &parent).expect("restore parent");
    fs::remove_file(&target).expect("remove target");
    fs::remove_dir(&parent).expect("remove parent");
    fs::remove_file(&backup.backup_path).expect("remove backup");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_maintenance_stored_target_cannot_rebind_through_hardlink() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-maintenance-hardlink-{suffix}.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "before").expect("seed target");

    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let result = runtime
        .run_demo_turn(&format!("overwrite:{relative} => after"))
        .await
        .expect("seed transaction");
    let transaction_id = output_field(
        result.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    );
    let backup = runtime
        .backup_index(Some(&relative))
        .expect("backup index")
        .backups
        .into_iter()
        .next()
        .expect("backup");

    fs::remove_file(&target).expect("remove captured target");
    let external = workspace.join(format!(
        ".architecture-v2-maintenance-hardlink-source-{suffix}.txt"
    ));
    fs::write(&external, "external\n").expect("external source");
    fs::hard_link(&external, &target).expect("hardlink replacement");

    let restore_error = runtime
        .restore_backup(&backup.id)
        .expect_err("restore hardlink rebind must fail closed");
    assert!(
        restore_error.0.contains("hard links"),
        "{}",
        restore_error.0
    );
    let rollback_error = runtime
        .rollback_write_transaction(&transaction_id)
        .expect_err("rollback hardlink rebind must fail closed");
    assert!(
        rollback_error.0.contains("hard links"),
        "{}",
        rollback_error.0
    );
    assert_eq!(
        fs::read_to_string(&external).expect("external source"),
        "external\n"
    );

    fs::remove_file(target).expect("remove hardlink replacement");
    fs::remove_file(external).expect("remove external source");
    fs::remove_file(&backup.backup_path).expect("remove backup");
}

#[cfg(unix)]
#[test]
fn architecture_v2_maintenance_backup_prune_refuses_symlink_and_hardlink_aliases() {
    use std::os::unix::fs::symlink;

    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let logical_path = format!("artifacts/.architecture-v2-prune-alias-{suffix}.txt");
    let timestamp = super::current_unix_ms().expect("timestamp");
    let older = super::write_fake_workspace_backup(&logical_path, timestamp, "older");
    let newer = super::write_fake_workspace_backup(&logical_path, timestamp + 1, "newer");
    let external = workspace.join(format!(".architecture-v2-prune-source-{suffix}.txt"));
    fs::write(&external, "outside").expect("outside source");

    fs::remove_file(&older).expect("replace older backup");
    symlink(&external, &older).expect("symlink alias");
    let symlink_error = runtime
        .prune_backups(Some(&logical_path), 1, None)
        .expect_err("prune traversal must reject symlinks");
    assert!(symlink_error.0.contains("refuses symlink"));
    assert_eq!(fs::read_to_string(&external).expect("outside"), "outside");

    fs::remove_file(&older).expect("remove symlink");
    fs::hard_link(&external, &older).expect("hardlink alias");
    let hardlink_error = runtime
        .prune_backups(Some(&logical_path), 1, None)
        .expect_err("sealed prune must reject hardlink aliases");
    assert!(hardlink_error.0.contains("hard links"));
    assert_eq!(fs::read_to_string(&external).expect("outside"), "outside");

    fs::remove_file(older).expect("remove hardlink");
    fs::remove_file(newer).expect("remove newer");
    fs::remove_file(external).expect("remove source");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_maintenance_rollback_verifies_checkpoint_and_post_state_hashes() {
    let runtime = RuntimeKernel::new();
    allow_write_file(&runtime);
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-rollback-hash-{suffix}.txt");
    let target = workspace.join(&relative);
    fs::write(&target, "before").expect("seed target");
    let turn = runtime
        .run_demo_turn(&format!("overwrite:{relative} => after"))
        .await
        .expect("record mutation");
    let transaction_id = output_field(
        turn.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    );
    let entry = runtime
        .write_transactions(Some(&relative))
        .expect("transaction index")
        .transactions
        .into_iter()
        .find(|entry| entry.transaction_id == transaction_id)
        .expect("transaction");
    assert!(entry.before_content_hash.is_some());
    assert!(entry.after_content_hash.is_some());
    let checkpoint = entry
        .rollback_checkpoint_path
        .as_deref()
        .expect("checkpoint");

    fs::write(checkpoint, "attacker checkpoint").expect("tamper checkpoint");
    let checkpoint_error = runtime
        .rollback_write_transaction(&transaction_id)
        .expect_err("tampered checkpoint must fail closed");
    assert!(checkpoint_error.0.contains("checkpoint contents changed"));
    assert_eq!(fs::read_to_string(&target).expect("target"), "after");

    fs::write(checkpoint, "before").expect("restore checkpoint");
    fs::write(&target, "attacker target").expect("tamper target");
    let target_error = runtime
        .rollback_write_transaction(&transaction_id)
        .expect_err("post-state drift must fail closed");
    assert!(target_error.0.contains("target contents changed"));
    assert_eq!(
        fs::read_to_string(&target).expect("target"),
        "attacker target"
    );

    fs::remove_file(target).expect("remove target");
    fs::remove_file(checkpoint).expect("remove checkpoint");
    for backup in runtime
        .backup_index(Some(&relative))
        .expect("backup index")
        .backups
    {
        if std::path::Path::new(&backup.backup_path).exists() {
            fs::remove_file(backup.backup_path).expect("remove rollback safety backup");
        }
    }
}
