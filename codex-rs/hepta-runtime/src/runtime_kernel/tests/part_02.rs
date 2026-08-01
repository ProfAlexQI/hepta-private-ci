use super::*;

#[tokio::test]
async fn rollback_group_partial_failure_records_status_and_resume_path() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path_a = format!("artifacts/hepta-partial-rollback-a-{}.txt", unique);
    let logical_path_b = format!("artifacts/hepta-partial-rollback-b-{}.txt", unique);
    let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_a);
    let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_b);
    fs::create_dir_all(path_a.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path_a, "before-a").expect("seed file a should be writable");
    fs::write(&path_b, "before-b").expect("seed file b should be writable");

    let group = runtime
        .begin_write_transaction_group(Some("grp-partial"))
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
        .await
        .expect("append a should succeed");
    runtime
        .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
        .await
        .expect("append b should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let plan = runtime
        .rollback_write_plan(&group.group_id)
        .expect("rollback plan should load");
    let fail_txn = plan.steps[1].transaction_id.clone();
    runtime
        .rollback_failure_injection_state
        .lock()
        .expect("failure injection state should lock")
        .push(fail_txn.clone());

    let partial = runtime
        .rollback_write_group(&group.group_id)
        .expect("rollback group should return partial failure report");
    assert_eq!(partial.status, RollbackGroupAttemptStatus::PartialFailed);
    assert_eq!(
        partial.failed_transaction_id.as_deref(),
        Some(fail_txn.as_str())
    );
    assert_eq!(partial.executed_transaction_ids.len(), 1);
    assert_eq!(partial.pending_transaction_ids, vec![fail_txn.clone()]);
    assert!(partial.resume_command.is_some());
    assert_eq!(
        fs::read_to_string(&path_b).expect("path b should be restored"),
        "before-b"
    );
    assert_eq!(
        fs::read_to_string(&path_a).expect("path a should still be appended"),
        "before-a+after-a"
    );

    let status = runtime
        .rollback_group_status(&group.group_id)
        .expect("rollback status should load");
    assert_eq!(
        status.schema_version,
        super::super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
    );
    assert!(status.group_locked);
    assert_eq!(
        status.group_lock_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(status.target_lock_count, 2);
    assert_eq!(status.orphaned_lock_count, 0);
    assert!(status.latest_attempt_owns_lock_set);
    assert_eq!(status.attempt_lifecycle.attempt_count, 1);
    assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 0);
    assert_eq!(
        status.attempt_lifecycle.active_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(status.lock_diagnostics.target_lock_count, 2);
    assert_eq!(
        status
            .latest_attempt
            .as_ref()
            .expect("attempt should exist")
            .status,
        RollbackGroupAttemptStatus::PartialFailed
    );
    assert!(status.resume_command.is_some());

    let status_json = serde_json::to_value(&status).expect("status should serialize");
    assert_eq!(
        status_json.get("schema_version").and_then(Value::as_u64),
        Some(super::super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        status_json
            .get("lock_diagnostics")
            .and_then(|value| value.get("group_lock_attempt_id"))
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        status_json
            .get("attempt_lifecycle")
            .and_then(|value| value.get("active_attempt_id"))
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );

    let locks = runtime.write_locks().expect("write locks should load");
    assert_eq!(
        locks.schema_version,
        super::super::WRITE_LOCK_REPORT_SCHEMA_VERSION
    );
    assert_eq!(locks.summary.total_target_locks, 2);
    assert_eq!(locks.summary.total_group_locks, 1);
    assert_eq!(locks.summary.rollback_bound_target_locks, 2);
    assert_eq!(locks.summary.rollback_bound_group_locks, 1);
    assert_eq!(locks.summary.orphaned_target_locks, 0);
    assert_eq!(locks.summary.orphaned_group_locks, 0);
    let group_lock = locks
        .group_locks
        .iter()
        .find(|lock| lock.group_id == group.group_id)
        .expect("group lock should exist");
    assert_eq!(
        group_lock.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        group_lock.rollback_status,
        Some(RollbackGroupAttemptStatus::PartialFailed)
    );
    assert_eq!(group_lock.pending_transaction_ids, vec![fail_txn.clone()]);
    let target_lock_a = locks
        .target_locks
        .iter()
        .find(|lock| lock.target_path.ends_with(&logical_path_a))
        .expect("target lock a should exist");
    assert_eq!(
        target_lock_a.rollback_group_id.as_deref(),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        target_lock_a.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    let target_lock_b = locks
        .target_locks
        .iter()
        .find(|lock| lock.target_path.ends_with(&logical_path_b))
        .expect("target lock b should exist");
    assert_eq!(
        target_lock_b.rollback_group_id.as_deref(),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        target_lock_b.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );

    let locks_json = serde_json::to_value(&locks).expect("locks should serialize");
    assert_eq!(
        locks_json.get("schema_version").and_then(Value::as_u64),
        Some(super::super::WRITE_LOCK_REPORT_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        locks_json
            .get("summary")
            .and_then(|value| value.get("rollback_bound_target_locks"))
            .and_then(Value::as_u64),
        Some(2)
    );
    assert_eq!(
        locks_json
            .get("summary")
            .and_then(|value| value.get("orphaned_group_locks"))
            .and_then(Value::as_u64),
        Some(0)
    );

    let blocked_write = runtime
        .run_demo_turn(&format!("append:{} => +blocked", logical_path_a))
        .await
        .expect("blocked write should still return a turn result");
    assert!(
        blocked_write
            .blocked_reason
            .expect("blocked reason should exist")
            .contains("write lock blocks write_file")
    );

    let resumed = runtime
        .resume_rollback_write_group(&group.group_id)
        .expect("resume rollback should succeed");
    assert_eq!(resumed.status, RollbackGroupAttemptStatus::Completed);
    assert_eq!(
        resumed.resumed_from_attempt_id,
        Some(partial.attempt_id.clone())
    );
    assert_eq!(
        fs::read_to_string(&path_a).expect("path a should be restored"),
        "before-a"
    );
    let post_resume_status = runtime
        .rollback_group_status(&group.group_id)
        .expect("post-resume rollback status should load");
    assert_eq!(post_resume_status.attempt_count, 2);
    assert_eq!(post_resume_status.superseded_attempt_count, 1);
    assert_eq!(
        post_resume_status.active_attempt_id.as_deref(),
        Some(resumed.attempt_id.as_str())
    );
    let superseded_partial = runtime
        .rollback_group_attempt_by_id(&partial.attempt_id)
        .expect("partial attempt lookup should succeed")
        .expect("partial attempt should exist");
    assert_eq!(
        superseded_partial.superseded_by_attempt_id.as_deref(),
        Some(resumed.attempt_id.as_str())
    );
    assert!(
        !runtime
            .write_locks()
            .expect("write locks should load")
            .group_locks
            .iter()
            .any(|lock| lock.group_id == group.group_id)
    );

    let events = runtime.events(60).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksAcquired)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksReleased)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRolledBack)
    );

    let failed_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
        .and_then(|event| event.event.payload.as_ref())
        .expect("failed rollback event payload should exist");
    assert_eq!(
        failed_event_payload
            .get("schema_version")
            .and_then(Value::as_u64),
        Some(super::super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        failed_event_payload.get("group_id").and_then(Value::as_str),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        failed_event_payload
            .get("attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        failed_event_payload
            .get("failed_transaction_id")
            .and_then(Value::as_str),
        Some(fail_txn.as_str())
    );

    let resumed_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
        .and_then(|event| event.event.payload.as_ref())
        .expect("resumed rollback event payload should exist");
    assert_eq!(
        resumed_event_payload
            .get("resumed_from_attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        resumed_event_payload
            .get("resumed_attempt_id")
            .and_then(Value::as_str),
        Some(resumed.attempt_id.as_str())
    );

    let rolled_back_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRolledBack)
        .and_then(|event| event.event.payload.as_ref())
        .expect("completed rollback event payload should exist");
    assert_eq!(
        rolled_back_event_payload
            .get("status")
            .and_then(Value::as_str),
        Some("completed")
    );
    assert_eq!(
        rolled_back_event_payload
            .get("attempt_id")
            .and_then(Value::as_str),
        Some(resumed.attempt_id.as_str())
    );

    let conflict_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLockConflict)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write lock conflict payload should exist");
    assert_eq!(
        conflict_event_payload
            .get("operation")
            .and_then(Value::as_str),
        Some("write_file")
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_group_id")
            .and_then(Value::as_str),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );

    let released_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLocksReleased)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write locks released payload should exist");
    assert_eq!(
        released_event_payload
            .get("released_group_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        released_event_payload
            .get("released_target_locks")
            .and_then(Value::as_u64),
        Some(2)
    );

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path_a)
            || entry.target_path.ends_with(&logical_path_b)
        {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for logical_path in [&logical_path_a, &logical_path_b] {
        let backups = runtime
            .backup_index(Some(logical_path))
            .expect("backup index should load");
        for backup in backups.backups {
            let _ = fs::remove_file(backup.backup_path);
        }
    }
    let _ = fs::remove_file(&path_a);
    let _ = fs::remove_file(&path_b);
}

#[test]
fn rollback_status_flags_orphaned_locks_and_recommends_prune() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .groups
        .push(WriteTransactionGroup {
            group_id: "grp-orphaned".into(),
            session_id: "session-main".into(),
            opened_at_unix_ms: 1,
            closed_at_unix_ms: Some(2),
            transaction_ids: vec![],
        });
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(RollbackGroupAttempt {
            attempt_id: "rbk-orphaned".into(),
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            started_at_unix_ms: 1,
            finished_at_unix_ms: Some(2),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: None,
            superseded_by_attempt_id: Some("rbk-current".into()),
            executed_transaction_ids: vec![],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-orphaned".into()],
            failed_transaction_id: Some("txn-orphaned".into()),
            failure_reason: Some("boom".into()),
            target_paths_restored: vec![],
        });
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(RollbackGroupAttempt {
            attempt_id: "rbk-current".into(),
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            started_at_unix_ms: 3,
            finished_at_unix_ms: Some(4),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: Some("rbk-orphaned".into()),
            superseded_by_attempt_id: None,
            executed_transaction_ids: vec![],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-current".into()],
            failed_transaction_id: Some("txn-current".into()),
            failure_reason: Some("still broken".into()),
            target_paths_restored: vec![],
        });
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .group_locks
        .push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-orphaned".into(),
            rollback_attempt_id: Some("rbk-orphaned".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let status = runtime
        .rollback_group_status("grp-orphaned")
        .expect("rollback status should load");
    assert_eq!(
        status.schema_version,
        super::super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
    );
    assert!(status.group_locked);
    assert_eq!(
        status.group_lock_attempt_id.as_deref(),
        Some("rbk-orphaned")
    );
    assert_eq!(status.orphaned_lock_count, 1);
    assert!(!status.latest_attempt_owns_lock_set);
    assert_eq!(status.active_attempt_id.as_deref(), Some("rbk-current"));
    assert_eq!(status.lock_diagnostics.orphaned_lock_count, 1);
    assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 1);
    assert_eq!(status.resume_command.as_deref(), Some("/prune-stale-locks"));
    assert!(
        status
            .suggested_next_action
            .contains("prune orphaned locks")
    );

    let status_json = serde_json::to_value(&status).expect("status should serialize");
    assert_eq!(
        status_json
            .get("lock_diagnostics")
            .and_then(|value| value.get("orphaned_lock_count"))
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        status_json
            .get("attempt_lifecycle")
            .and_then(|value| value.get("superseded_attempt_count"))
            .and_then(Value::as_u64),
        Some(1)
    );

    let locks = runtime.write_locks().expect("write locks should load");
    assert_eq!(locks.summary.orphaned_group_locks, 1);
}

#[tokio::test]
async fn overlap_lock_blocks_write_to_descendant_path() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_dir = format!("artifacts/hepta-locked-dir-{}", unique);
    let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_dir);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: locked_dir_path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-overlap".into(),
            rollback_group_id: Some("grp-overlap".into()),
            rollback_attempt_id: Some("rbk-overlap".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let result = runtime
        .run_demo_turn(&format!("append:{}/child.txt => +blocked", logical_dir))
        .await
        .expect("blocked write should still produce a turn result");
    assert!(
        result
            .blocked_reason
            .expect("blocked reason should exist")
            .contains("write lock blocks write_file")
    );

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
}

#[tokio::test]
async fn active_write_reservation_blocks_parallel_write_and_rollback() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");
    let relative_path = format!(
        "artifacts/hepta-write-reservation-{}.txt",
        current_unix_ms().expect("timestamp should exist")
    );
    let target_path = crate::tool_workspace_root_path().join(&relative_path);
    fs::create_dir_all(target_path.parent().expect("target parent"))
        .expect("target parent should be created");
    fs::write(&target_path, "before").expect("target should be seeded");
    let write = runtime
        .run_demo_turn(&format!("overwrite:{} => after", relative_path))
        .await
        .expect("seed write should succeed");
    let transaction_id = extract_json_string_field(
        write.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    )
    .expect("transaction id");
    let backup = runtime
        .backup_index(Some(&relative_path))
        .expect("backup index")
        .backups[0]
        .clone();
    let arguments = json!({
        "path": &relative_path,
        "content": "reserved",
        "mode": "overwrite"
    })
    .to_string();
    let (reserved_tx, reserved_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();

    std::thread::scope(|scope| {
        let runtime_ref = &runtime;
        let arguments_ref = &arguments;
        let holder = scope.spawn(move || {
            let prepared = runtime_ref
                .prepare_write_transaction_with_lock_check(
                    "session-main",
                    "write_file",
                    arguments_ref,
                )
                .expect("first writer should reserve target")
                .expect("write preparation should exist");
            reserved_tx.send(()).expect("reservation signal");
            release_rx.recv().expect("release signal");
            drop(prepared);
        });

        reserved_rx.recv().expect("reservation should be active");
        let write_error = runtime
            .prepare_write_transaction_with_lock_check("session-other", "write_file", &arguments)
            .expect_err("parallel writer must be blocked");
        assert!(write_error.0.contains("tool_execution_reservation"));

        let target_path = crate::tool_workspace_root_path().join(&relative_path);
        let rollback_error = runtime
            .acquire_group_rollback_locks(
                "session-main",
                "group-concurrent",
                "attempt-concurrent",
                &[target_path.display().to_string()],
            )
            .expect_err("rollback must not cross an active write");
        assert!(
            rollback_error
                .0
                .contains("write lock blocks rollback_group")
        );
        let restore_error = runtime
            .restore_backup(&backup.id)
            .expect_err("active write must block restore");
        assert!(restore_error.0.contains("tool_execution_reservation"));
        let transaction_error = runtime
            .rollback_write_transaction(&transaction_id)
            .expect_err("active write must block public rollback");
        assert!(transaction_error.0.contains("tool_execution_reservation"));

        release_tx.send(()).expect("release holder");
        holder.join().expect("reservation holder should finish");
    });

    let prepared = runtime
        .prepare_write_transaction_with_lock_check("session-main", "write_file", &arguments)
        .expect("released target should be reservable");
    drop(prepared);
    assert!(
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state")
            .active_target_reservations
            .is_empty()
    );
    runtime
        .acquire_group_rollback_locks(
            "session-main",
            "group-after-release",
            "attempt-after-release",
            &[target_path.display().to_string()],
        )
        .expect("released target should permit rollback lock");
    runtime
        .release_group_rollback_locks("session-main", "group-after-release")
        .expect("rollback lock should release");
    let _ = fs::remove_file(target_path);
    let _ = fs::remove_file(backup.backup_path);
}

#[tokio::test]
async fn overlap_lock_blocks_rollback_group_on_descendant_target() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_dir = format!("artifacts/hepta-overlap-rollback-{}", unique);
    let logical_path = format!("{}/child.txt", logical_dir);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let group = runtime
        .begin_write_transaction_group(Some("grp-overlap-rollback"))
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after", logical_path))
        .await
        .expect("append should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_dir);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: locked_dir_path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-external".into(),
            rollback_group_id: Some("grp-external".into()),
            rollback_attempt_id: Some("rbk-external".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let err = runtime
        .rollback_write_group(&group.group_id)
        .expect_err("overlap lock should block rollback group");
    assert!(err.0.contains("write lock blocks rollback_group"));

    let events = runtime.events(30).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
    let conflict_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLockConflict)
        .and_then(|event| event.event.payload.as_ref())
        .expect("rollback-group conflict payload should exist");
    assert_eq!(
        conflict_event_payload
            .get("operation")
            .and_then(Value::as_str),
        Some("rollback_group")
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_group_id")
            .and_then(Value::as_str),
        Some("grp-external")
    );

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path) {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for backup in runtime
        .backup_index(Some(&logical_path))
        .expect("backup index should load")
        .backups
    {
        let _ = fs::remove_file(backup.backup_path);
    }
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn expired_write_lock_is_pruned_and_does_not_block_write() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-expired-lock-{}.txt", unique);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-stale".into(),
            rollback_group_id: Some("grp-stale".into()),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });

    let result = runtime
        .run_demo_turn(&format!("append:{} => +after", logical_path))
        .await
        .expect("write should succeed after stale lock pruning");
    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    assert!(
        runtime
            .write_locks()
            .expect("write locks should load")
            .target_locks
            .is_empty()
    );

    for entry in runtime
        .write_transactions(Some(&logical_path))
        .expect("transactions should load")
        .transactions
    {
        if let Some(checkpoint) = entry.rollback_checkpoint_path {
            let _ = fs::remove_file(checkpoint);
        }
    }
    let _ = fs::remove_file(&path);
}

#[test]
fn prune_stale_write_locks_removes_expired_entries_and_emits_event() {
    let runtime = RuntimeKernel::new();
    {
        let mut guard = runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock");
        guard.group_locks.push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "grp-stale".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-stale".into(),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });
        guard.target_locks.push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: "/tmp/hepta-stale".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-stale".into(),
            rollback_group_id: Some("grp-stale".into()),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });
    }

    let report = runtime
        .prune_stale_write_locks()
        .expect("stale lock prune should succeed");
    assert_eq!(report.pruned_target_locks, 1);
    assert_eq!(report.pruned_group_locks, 1);
    assert_eq!(report.remaining_target_locks, 0);
    assert_eq!(report.remaining_group_locks, 0);

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksPruned)
    );
    let pruned_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLocksPruned)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write locks pruned payload should exist");
    assert_eq!(
        pruned_event_payload
            .get("schema_version")
            .and_then(Value::as_u64),
        Some(super::super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        pruned_event_payload
            .get("pruned_target_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        pruned_event_payload
            .get("pruned_group_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
}

#[test]
fn snapshot_roundtrip_preserves_write_transactions() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(format!("artifacts/hepta-write-txn-snapshot-{}.txt", unique));
    fs::create_dir_all(target_path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&target_path, "before").expect("seed file should be writable");

    let checkpoint_path = preview_transaction_checkpoint_path(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        &target_path,
        "txn-snapshot",
    )
    .expect("checkpoint path should build");
    fs::create_dir_all(
        checkpoint_path
            .parent()
            .expect("checkpoint parent should exist"),
    )
    .expect("checkpoint parent should be creatable");
    fs::write(&checkpoint_path, "before").expect("checkpoint should be writable");
    runtime
        .write_transaction_state
        .lock()
        .expect("write transaction state should lock")
        .push(WriteTransactionEntry {
            transaction_id: "txn-snapshot".into(),
            session_id: "session-main".into(),
            action: "write_file".into(),
            target_path: target_path.display().to_string(),
            created_at_unix_ms: unique,
            mode: "append".into(),
            target_existed_before: true,
            bytes_before: 6,
            bytes_after: 12,
            before_content_hash: None,
            after_content_hash: None,
            effect_plan_hash: None,
            effect_ack_hash: None,
            before_file_identity: None,
            after_file_identity: None,
            rollback_strategy: "restore_checkpoint".into(),
            rollback_checkpoint_path: Some(checkpoint_path.display().to_string()),
            source_backup_path: None,
            rolled_back_at_unix_ms: None,
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(format!(
            "artifacts/hepta-write-txn-snapshot-{}.json",
            unique
        ));
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let report = restored
        .write_transactions(None)
        .expect("transactions should load");
    assert!(
        report
            .transactions
            .iter()
            .any(|entry| entry.transaction_id == "txn-snapshot")
    );

    let _ = fs::remove_file(&target_path);
    let _ = fs::remove_file(&checkpoint_path);
    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_write_transaction_groups() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .groups
        .push(WriteTransactionGroup {
            group_id: "txngrp-snapshot".into(),
            session_id: "session-main".into(),
            opened_at_unix_ms: 1,
            closed_at_unix_ms: Some(2),
            transaction_ids: vec!["txn-a".into(), "txn-b".into()],
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-write-group-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let report = restored
        .write_transaction_groups()
        .expect("groups should load");
    assert!(
        report
            .groups
            .iter()
            .any(|group| group.group_id == "txngrp-snapshot")
    );

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_rollback_group_attempts() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(super::super::RollbackGroupAttempt {
            attempt_id: "rbk-snapshot".into(),
            session_id: "session-main".into(),
            group_id: "txngrp-snapshot".into(),
            started_at_unix_ms: 1,
            finished_at_unix_ms: Some(2),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: None,
            superseded_by_attempt_id: None,
            executed_transaction_ids: vec!["txn-a".into()],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-b".into()],
            failed_transaction_id: Some("txn-b".into()),
            failure_reason: Some("boom".into()),
            target_paths_restored: vec!["/tmp/a".into()],
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-rollback-attempt-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let status = restored
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .iter()
        .find(|attempt| attempt.attempt_id == "rbk-snapshot")
        .cloned();
    assert!(status.is_some());

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_write_locks() {
    let runtime = RuntimeKernel::new();
    let lease_expires_at_unix_ms = current_unix_ms().expect("timestamp should exist") + 60_000;
    {
        let mut guard = runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock");
        guard.group_locks.push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "txngrp-snapshot".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-snapshot".into(),
            rollback_attempt_id: Some("rbk-snapshot".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms,
        });
        guard.target_locks.push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: "/tmp/a".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "txngrp-snapshot".into(),
            rollback_group_id: Some("txngrp-snapshot".into()),
            rollback_attempt_id: Some("rbk-snapshot".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms,
        });
    }

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-write-lock-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let locks = restored.write_locks().expect("write locks should load");
    assert!(
        locks
            .group_locks
            .iter()
            .any(|lock| lock.group_id == "txngrp-snapshot")
    );
    assert!(
        locks
            .target_locks
            .iter()
            .any(|lock| lock.target_path == "/tmp/a")
    );
    assert!(
        locks
            .group_locks
            .iter()
            .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
    );
    assert!(
        locks
            .target_locks
            .iter()
            .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
    );

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn preview_prune_backups_plans_deletion_of_older_backups() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-prune-preview-test-{}.txt", unique);
    let older = write_fake_workspace_backup(&logical_path, unique, "older");
    let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

    let report = runtime
        .preview_prune_backups(Some(&logical_path), 1, None)
        .expect("preview prune should succeed");

    assert_eq!(report.scanned_backups, 2);
    assert_eq!(report.deleted_count, 1);
    assert_eq!(report.kept_backups.len(), 1);
    assert_eq!(report.deleted_backups[0].created_at_unix_ms, unique);
    assert_eq!(report.kept_backups[0].created_at_unix_ms, unique + 1);

    let _ = fs::remove_file(&older);
    let _ = fs::remove_file(&newer);
}

#[test]
fn prune_backups_deletes_older_backups_and_emits_event() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-prune-exec-test-{}.txt", unique);
    let older = write_fake_workspace_backup(&logical_path, unique, "older");
    let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

    let report = runtime
        .prune_backups(Some(&logical_path), 1, None)
        .expect("prune backups should succeed");

    assert!(report.executed);
    assert_eq!(report.deleted_count, 1);
    assert!(!older.exists());
    assert!(newer.exists());

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::BackupsPruned)
    );

    let _ = fs::remove_file(&newer);
}

#[test]
fn session_export_roundtrip_preserves_write_path_scope() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_write_path_scope(WritePathScope::WorkspaceOnly)
        .expect("write scope switch should succeed");
    let export = runtime
        .session_export("session-main")
        .expect("session export should succeed");
    assert_eq!(export.write_path_scope, WritePathScope::WorkspaceOnly);

    runtime
        .switch_write_path_scope(WritePathScope::ArtifactsOnly)
        .expect("write scope reset should succeed");
    runtime
        .apply_session_export(export)
        .expect("session import should succeed");

    assert_eq!(
        runtime
            .write_path_scope_for_session("session-main")
            .expect("write scope should load"),
        WritePathScope::WorkspaceOnly
    );
}

#[tokio::test]
async fn session_export_roundtrip_preserves_topic_sessions_and_graph_store() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    source
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    source
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    source
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    source
        .route_topics(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("mixed route should succeed");

    let export = source
        .session_export("alpha")
        .expect("session export should succeed");
    assert_eq!(export.topic_sessions.len(), 2);
    assert!(export.topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let restored = RuntimeKernel::new();
    restored
        .apply_session_export(export)
        .expect("session import should succeed");

    let raw_topic_sessions = restored
        .topic_session_state
        .lock()
        .expect("topic session state lock should succeed")
        .sessions
        .clone();
    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert_eq!(raw_topic_sessions.len(), 2);
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let topic_sessions = restored
        .topic_sessions_for_surface("alpha")
        .expect("topic sessions should load");
    assert!(topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:alpha"
            && !topic_session.graph_edges.is_empty()
    }));
}

#[tokio::test]
async fn exposes_sessions_memory_and_history_snapshots() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello session control plane")
        .await
        .expect("plain turn should succeed");
    runtime
        .run_demo_turn("tool:history probe")
        .await
        .expect("tool turn should succeed");

    let sessions = runtime.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");

    let memories = runtime
        .memory_snapshot(10)
        .expect("memory snapshot should load");
    assert!(memories.iter().any(|item| {
        item.content
            .contains("assistant:hello session control plane")
    }));

    let history = runtime
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(history.len() >= 2);
    assert_eq!(history[0].input, "tool:history probe");
}

#[test]
fn fresh_active_session_is_consistent_across_control_plane_views() {
    let runtime = RuntimeKernel::new();
    let sessions = runtime.sessions().expect("sessions should load");
    let session = runtime
        .active_session_snapshot()
        .expect("active session snapshot should load");
    let overview = runtime
        .session_activity_overview(0, 0)
        .expect("session activity overview should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");
    assert!(sessions[0].is_active);
    assert_eq!(session.session_id, "session-main");
    assert!(session.is_active);
    assert_eq!(overview.sessions.len(), 1);
    assert_eq!(overview.active_sessions, 1);
    assert_eq!(overview.archived_sessions, 0);
    assert_eq!(overview.sessions[0].session.session_id, "session-main");
}

#[tokio::test]
async fn doctor_reports_provider_probes_and_integrity_checks() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello doctor")
        .await
        .expect("plain turn should succeed");
    runtime
        .route_topics("session-main", Some("hello doctor"), 4, 4, 4, 1)
        .expect("topic route should succeed");

    let report = runtime
        .doctor_report()
        .await
        .expect("doctor report should succeed");
    assert_eq!(
        report.overall_status,
        DoctorStatus::Warn,
        "unexpected doctor report: {report:#?}"
    );
    assert_eq!(report.total_topic_sessions, 1);
    assert_eq!(report.total_topic_graph_edges, 0);
    assert_eq!(report.active_topic_sessions, 1);
    assert_eq!(report.active_topic_sessions_with_transcript_provenance, 1);
    assert_eq!(
        report.active_topic_sessions_missing_transcript_provenance,
        0
    );
    assert!(report.active_session_recall_transcript_evidence_spans > 0);
    assert_eq!(report.active_session_recall_omitted_items, 0);
    assert!(report.active_session_intuition_transcript_evidence_spans > 0);
    assert_eq!(report.active_session_intuition_foreground_topic_sessions, 1);
    assert!(
        report
            .provider_probes
            .iter()
            .any(|probe| probe.provider_name == "demo" && probe.status == DoctorStatus::Ok)
    );
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "runtime snapshot roundtrip" && check.status == DoctorStatus::Ok
    }));
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "active session export roundtrip" && check.status == DoctorStatus::Ok
    }));
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "topic sessions carry transcript provenance"
            && check.status == DoctorStatus::Ok
    }));

    let summary = runtime
        .doctor_summary()
        .await
        .expect("doctor summary should succeed");

    for needle in [
        "Hepta doctor: warn",
        "- topic sessions: 1",
        "- topic graph edges: 0",
        "- active topic sessions with transcript provenance: ",
        "- active topic sessions missing transcript provenance: ",
        "- active session recall transcript evidence spans: ",
        "- active session recall omitted items: 0",
        "- active session intuition transcript evidence spans: ",
        "- active session intuition foreground topic sessions: 1",
        "demo: ok via demo/demo-chat",
        "mock-ollama: ok via mock-ollama/local-chat",
        "history session references: ok",
        "runtime snapshot roundtrip: ok",
        "topic sessions carry transcript provenance: ok",
    ] {
        assert!(summary.iter().any(|line| line.contains(needle)), "{needle}");
    }
}

#[tokio::test]
async fn doctor_warns_when_active_topic_sessions_lose_transcript_provenance() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello doctor provenance gap")
        .await
        .expect("plain turn should succeed");
    runtime
        .route_topics(
            "session-main",
            Some("hello doctor provenance gap"),
            4,
            4,
            4,
            1,
        )
        .expect("topic route should succeed");

    {
        let mut topic_state = runtime
            .topic_session_state
            .lock()
            .expect("topic session state mutex should not poison");
        let topic_session = topic_state
            .sessions
            .iter_mut()
            .find(|topic_session| {
                topic_session.topic_session_id == "topic-session-bootstrap:session-main"
            })
            .expect("bootstrap topic session should exist");
        topic_session.linked_transcript_spans.clear();
    }

    let report = runtime
        .doctor_report()
        .await
        .expect("doctor report should succeed");
    assert_eq!(report.overall_status, DoctorStatus::Warn);
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "topic sessions carry transcript provenance"
            && check.status == DoctorStatus::Warn
            && check
                .detail
                .contains("topic-session-bootstrap:session-main")
    }));
}

#[test]
fn rejects_invalid_tool_arguments_against_schema() {
    let runtime = RuntimeKernel::new();
    let err = runtime
        .validate_tool_input("read_file", r#"{"path":""}"#)
        .expect_err("empty path should be rejected");
    assert!(err.0.contains("must be at least 1 characters"));

    let err = runtime
        .validate_tool_input("echo", r#"{"wrong":"value"}"#)
        .expect_err("missing required field should be rejected");
    assert!(err.0.contains("missing required field 'text'"));

    let err = runtime
        .validate_tool_input(
            "write_file",
            r#"{"path":"artifacts/x.txt","content":"x","mode":"replace"}"#,
        )
        .expect_err("invalid write mode should be rejected");
    assert!(err.0.contains("must be one of: create, overwrite, append"));

    let err = runtime
        .validate_tool_input(
            "write_file",
            r#"{"path":"artifacts/x.txt","content":"x","confirm_destructive":"yes"}"#,
        )
        .expect_err("non-boolean destructive confirm should be rejected");
    assert!(err.0.contains("must be a boolean"));
}

#[tokio::test]
async fn returns_and_validates_structured_tool_output() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn("tool:typed output")
        .await
        .expect("echo turn should succeed");

    let output_json = result
        .tool_output_json
        .expect("structured tool output should be present");
    assert!(output_json.contains("\"text\":\"typed output\""));
    runtime
        .validate_tool_output("echo", &output_json)
        .expect("echo output should match schema");
}

#[test]
fn native_tool_result_reply_hides_structured_json() {
    let structured = json!({
        "backend": "hepta-rust-native",
        "content": "8 native background process(es)",
        "native_runtime": true,
        "openclaw_gateway_invoked": false,
        "proxy_used": false,
        "tool": "process",
        "result": {
            "action": "list",
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"],
            "processes": [
                {"id": "hepta-proc-1", "log_path": "/private/path/one.log"},
                {"id": "hepta-proc-2", "log_path": "/private/path/two.log"}
            ]
        }
    });
    let reply = render_native_tool_result_reply(&format!(
        "8 native background process(es) | structured={}",
        structured
    ));

    assert!(reply.contains("共有 2 条后台进程记录"));
    assert!(reply.contains("结构化 JSON 已保留在本地"));
    assert!(!reply.contains("structured="));
    assert!(!reply.contains("log_path"));
    assert!(!reply.contains("/private/path"));
    assert!(!reply.contains("Hepta native tool result"));
}

#[test]
fn exposes_tool_descriptors_for_discovery() {
    let runtime = RuntimeKernel::new();
    let tools = runtime.tool_descriptors();
    assert_eq!(tools.len(), 42);
    assert!(tools.iter().any(|tool| {
        tool.name == "echo"
            && tool.description.contains("Return the provided input as-is")
            && tool.execution_metadata.read_only
            && tool.execution_metadata.idempotent
            && tool.execution_metadata.produces_structured_output
            && tool.default_approval_requirement == ApprovalRequirement::None
            && tool.input_schema_json.contains("text")
            && tool.output_schema_json.contains("text")
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "read_file"
            && tool
                .description
                .contains("Read a UTF-8 text file from disk")
            && tool.execution_metadata.read_only
            && !tool.execution_metadata.destructive
            && tool.execution_metadata.idempotent
            && tool.default_approval_requirement == ApprovalRequirement::Ask
            && tool.input_schema_json.contains("path")
            && tool.output_schema_json.contains("line_count")
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "disk_junk_audit"
            && tool.description.contains("read-only local disk cleanup")
            && tool.execution_metadata.read_only
            && !tool.execution_metadata.destructive
            && tool.default_approval_requirement == ApprovalRequirement::None
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "write_file"
            && tool.description.contains("Write a UTF-8 text file to disk")
            && !tool.execution_metadata.read_only
            && tool.execution_metadata.destructive
            && !tool.execution_metadata.idempotent
            && tool.default_approval_requirement == ApprovalRequirement::Deny
            && tool.input_schema_json.contains("content")
            && tool.output_schema_json.contains("bytes_written")
    }));
    for expected in [
        "json_get",
        "skill_propose",
        "skill_scan",
        "skill_apply_plan",
        "tool_manifest_validate",
        "tool_generate_stub",
        "read",
        "write",
        "edit",
        "apply_patch",
        "web_search",
        "web_fetch",
        "sessions_list",
        "message",
        "memory_get",
        "feishu_doc",
    ] {
        assert!(
            tools.iter().any(|tool| tool.name == expected),
            "missing expanded native tool {expected}"
        );
    }
    let read = tools
        .iter()
        .find(|tool| tool.name == "read")
        .expect("OpenClaw-compatible read tool should exist");
    assert!(read.description.contains("Rust-native"));
    assert!(!read.description.contains("Gateway proxy"));
    for quarantined in [
        "exec",
        "process",
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(!tools.iter().any(|tool| tool.name == quarantined));
    }
    let model_tools = runtime.tools.model_tool_specs();
    assert_eq!(model_tools.len(), 42);
    for quarantined in [
        "exec",
        "process",
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(!model_tools.iter().any(|tool| tool.name == quarantined));
    }
}

#[tokio::test]
async fn generated_skill_and_tool_helpers_are_invokable() {
    let registry = ToolRegistry::new();
    let context = ToolContext {
        session_id: Some(SessionId("session-test".into())),
        correlation_id: Some(CorrelationId("corr-test".into())),
        execution_attempt_id: None,
        idempotency_key: None,
    };

    let skill = registry
        .invoke(
            "skill_propose",
            context.clone(),
            ToolCallRequest {
                name: "skill_propose".into(),
                input_json: r#"{"transcript":"Build a safe local skill workshop flow"}"#.into(),
            },
        )
        .await
        .expect("skill proposal helper should invoke");
    let skill_json: Value = serde_json::from_str(
        skill
            .structured_json
            .as_deref()
            .expect("skill proposal should be structured"),
    )
    .expect("skill proposal output should parse");
    assert_eq!(skill_json["safe_to_apply"], json!(true));
    assert_eq!(
        skill_json["skill_name"],
        json!("build-a-safe-local-skill-workshop-flow")
    );

    let generated = registry
        .invoke(
            "tool_generate_stub",
            context.clone(),
            ToolCallRequest {
                name: "tool_generate_stub".into(),
                input_json:
                    r#"{"name":"Summarize Local File","description":"Summarize a local file"}"#
                        .into(),
            },
        )
        .await
        .expect("tool generator should invoke");
    let generated_json = generated
        .structured_json
        .clone()
        .expect("tool generator should return structured json");
    let manifest: Value =
        serde_json::from_str(&generated_json).expect("generated tool manifest should parse");
    assert_eq!(manifest["name"], json!("summarize_local_file"));

    let validation = registry
        .invoke(
            "tool_manifest_validate",
            context,
            ToolCallRequest {
                name: "tool_manifest_validate".into(),
                input_json: json!({ "manifest_json": generated_json }).to_string(),
            },
        )
        .await
        .expect("tool manifest validator should invoke");
    let validation_json: Value = serde_json::from_str(
        validation
            .structured_json
            .as_deref()
            .expect("validation should be structured"),
    )
    .expect("validation output should parse");
    assert_eq!(validation_json["valid"], json!(true));
    assert_eq!(validation_json["issue_count"], json!(0));
}

#[tokio::test]
async fn openclaw_compatible_tools_are_native_not_gateway_proxy() {
    let registry = ToolRegistry::new_with_all_quarantined_tools_for_test();
    let context = ToolContext {
        session_id: Some(SessionId("session-native-tools".into())),
        correlation_id: Some(CorrelationId("corr-native-tools".into())),
        execution_attempt_id: None,
        idempotency_key: None,
    };
    for (tool, input_json) in [
            (
                "write",
                json!({"path":"artifacts/direct-write.txt","content":"blocked"}).to_string(),
            ),
            (
                "edit",
                json!({"path":"artifacts/direct-edit.txt","edits":[]}).to_string(),
            ),
            (
                "apply_patch",
                json!({"input":"*** Begin Patch\n*** Add File: artifacts/direct.txt\n+x\n*** End Patch"}).to_string(),
            ),
        ] {
            let error = registry
                .invoke(
                    tool,
                    context.clone(),
                    ToolCallRequest {
                        name: tool.into(),
                        input_json,
                    },
                )
                .await
                .expect_err("direct native mutation must fail closed");
            assert!(
                error.0.contains("identity-bound"),
                "{tool} failed with unexpected error: {}",
                error.0
            );
        }

    let exec = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-exec"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "printf native-exec"}).to_string(),
            },
        )
        .await
        .expect("native exec should invoke");
    let exec_json: Value = serde_json::from_str(exec.structured_json.as_deref().unwrap())
        .expect("exec output should parse");
    assert_eq!(exec_json["proxy_used"], json!(false));
    assert_eq!(exec_json["result"]["stdout"], json!("native-exec"));

    let started = std::time::Instant::now();
    let timed_out_exec = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-timeout"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "sleep 5", "timeoutMs": 100}).to_string(),
            },
        )
        .await
        .expect("native exec timeout should return structured result, not hang");
    assert!(started.elapsed() < std::time::Duration::from_secs(3));
    assert!(
        timed_out_exec
            .content
            .contains("ToolTimeout/exec timed out")
    );
    let timed_out_json: Value =
        serde_json::from_str(timed_out_exec.structured_json.as_deref().unwrap())
            .expect("timeout output should parse");
    assert_eq!(timed_out_json["status"], json!("timeout"));
    assert_eq!(timed_out_json["error_kind"], json!("ToolTimeout"));
    assert_eq!(timed_out_json["result"]["timeout"], json!(true));
    assert_eq!(
        timed_out_json["result"]["duplicate_tool_replay_prevented"],
        json!(true)
    );

    let background = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-background"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "cat", "background": true}).to_string(),
            },
        )
        .await
        .expect("native background exec should invoke");
    let background_json: Value =
        serde_json::from_str(background.structured_json.as_deref().unwrap())
            .expect("background output should parse");
    let process_id = background_json["result"]["sessionId"]
        .as_str()
        .expect("background should return process id")
        .to_string();
    assert_eq!(background_json["proxy_used"], json!(false));

    registry
                .invoke(
                    "process",
                    provider_test_context("session-native-tools", "corr-native-process-write"),
                    ToolCallRequest {
                        name: "process".into(),
                        input_json: json!({"action":"write", "sessionId": process_id, "data":"native-process\n", "eof": true}).to_string(),
                    },
                )
                .await
                .expect("native process write should invoke");
    let process_poll = registry
        .invoke(
            "process",
            ToolContext {
                session_id: Some(SessionId("session-native-tools".into())),
                correlation_id: Some(CorrelationId("corr-native-tools".into())),
                execution_attempt_id: None,
                idempotency_key: None,
            },
            ToolCallRequest {
                name: "process".into(),
                input_json: json!({"action":"poll", "sessionId": process_id, "timeout": 3000})
                    .to_string(),
            },
        )
        .await
        .expect("native process poll should invoke");
    let process_poll_json: Value =
        serde_json::from_str(process_poll.structured_json.as_deref().unwrap())
            .expect("process poll output should parse");
    assert_eq!(process_poll_json["tool"], json!("process"));
    assert_eq!(process_poll_json["proxy_used"], json!(false));

    let tts_error = registry
        .invoke(
            "tts",
            context.clone(),
            ToolCallRequest {
                name: "tts".into(),
                input_json: json!({
                    "text":"hello",
                    "path":"artifacts/direct-tts.aiff",
                    "dryRun":true
                })
                .to_string(),
            },
        )
        .await
        .expect_err("direct TTS output must fail closed");
    assert!(tts_error.0.contains("identity-bound"));

    for (tool, payload) in [
        (
            "message",
            json!({"action":"send", "channel":"telegram", "target":"6476198178", "message":"dry run", "dryRun": true}),
        ),
        (
            "image_generate",
            json!({"prompt":"tiny red dot", "dryRun": true}),
        ),
        (
            "music_generate",
            json!({"prompt":"tiny tune", "dryRun": true}),
        ),
        (
            "video_generate",
            json!({"prompt":"tiny clip", "dryRun": true}),
        ),
    ] {
        let result = registry
            .invoke(
                tool,
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                    execution_attempt_id: None,
                    idempotency_key: None,
                },
                ToolCallRequest {
                    name: tool.into(),
                    input_json: payload.to_string(),
                },
            )
            .await
            .expect("native dry-run surface should invoke");
        let parsed: Value = serde_json::from_str(result.structured_json.as_deref().unwrap())
            .expect("native dry-run output should parse");
        assert_eq!(parsed["proxy_used"], json!(false));
        assert_ne!(parsed["status"], json!("native_surface_registered"));
    }
}
