use hepta_core::EventKind;
use hepta_core::ExecutionProfile;
use hepta_core::FilesystemScope;
use hepta_core::MemoryStore;
use hepta_core::WritePathScope;

use super::WorkerExecutionBackendBinding;
use super::WorkerExecutionBackendKind;
use super::WorkerExecutionBackendStatus;
use super::WorkerPoolPressureLevel;
use super::WorkerTaskCommandRunOrigin;
use super::WorkerTaskContextRecallHandoffPolicy;
use super::WorkerTaskExecutionMode;
use super::WorkerTaskFailureKind;
use super::WorkerTaskFileLeaseStatus;
use super::WorkerTaskLoopPhase;
use super::WorkerTaskMergeDecision;
use super::WorkerTaskPatchApplyStatus;
use super::WorkerTaskPromotionDecision;
use super::WorkerTaskStatus;
use super::effective_worker_task_prompt;
use super::redact_worker_output_exfiltration;
use super::task_status_label;
use crate::RuntimeKernel;

#[test]
fn status_labels_are_stable() {
    assert_eq!(task_status_label(WorkerTaskStatus::Queued), "queued");
    assert_eq!(task_status_label(WorkerTaskStatus::Scheduled), "scheduled");
    assert_eq!(task_status_label(WorkerTaskStatus::Running), "running");
    assert_eq!(task_status_label(WorkerTaskStatus::Paused), "paused");
    assert_eq!(task_status_label(WorkerTaskStatus::Completed), "completed");
    assert_eq!(task_status_label(WorkerTaskStatus::Failed), "failed");
    assert_eq!(task_status_label(WorkerTaskStatus::Cancelled), "cancelled");
    assert_eq!(
        task_status_label(WorkerTaskStatus::Interrupted),
        "interrupted"
    );
}

#[test]
fn worker_task_lifecycle_is_queryable_and_snapshot_backed() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("reviewer", "summarize the release checklist", None)
        .expect("task should spawn");

    assert_eq!(spawned.task.worker_id, "reviewer");
    assert_eq!(spawned.task.status, WorkerTaskStatus::Queued);
    assert_eq!(spawned.task.parent_session_id, "session-main");
    assert!(
        spawned
            .task
            .worker_session_id
            .starts_with("worker-reviewer-")
    );

    let index = runtime.worker_task_index(None).expect("tasks should list");
    assert_eq!(index.total_count, 1);
    assert_eq!(index.queued_count, 1);

    let inventory = runtime.worker_inventory().expect("workers should list");
    assert_eq!(inventory.worker_count, 1);
    assert_eq!(inventory.workers[0].worker_id, "reviewer");
    assert_eq!(inventory.workers[0].active_task_count, 1);

    let join = runtime
        .join_worker_tasks(Some("reviewer"))
        .expect("join should report active blockers");
    assert!(!join.safe_to_join);
    assert_eq!(join.active_count, 1);

    let snapshot = runtime
        .runtime_snapshot()
        .expect("snapshot should include worker tasks");
    assert_eq!(snapshot.worker_tasks.len(), 1);

    let restored = RuntimeKernel::new();
    restored
        .apply_runtime_snapshot(snapshot)
        .expect("snapshot should restore");
    let restored_status = restored
        .worker_task_status(&spawned.task.task_id)
        .expect("restored task should be queryable");
    assert_eq!(restored_status.task.status, WorkerTaskStatus::Queued);
}

#[test]
fn worker_task_pause_resume_and_interrupt_are_observable_controls() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("builder", "prepare controllable worker lane", None)
        .expect("task should spawn");

    let steered = runtime
        .steer_worker_task(&spawned.task.task_id, "tighten scope before execution")
        .expect("task should accept steering");
    assert_eq!(steered.task.steering_directives.len(), 1);
    assert!(effective_worker_task_prompt(&steered.task).contains("Operator steering directives"));

    let paused = runtime
        .pause_worker_task(&spawned.task.task_id)
        .expect("task should pause");
    assert_eq!(paused.task.status, WorkerTaskStatus::Paused);
    assert_eq!(
        paused.task.paused_from_status,
        Some(WorkerTaskStatus::Queued)
    );

    let supervisor = runtime
        .worker_task_supervisor()
        .expect("supervisor should show paused task");
    assert_eq!(supervisor.paused_count, 1);
    assert_eq!(supervisor.paused_control_count, 1);
    assert_eq!(
        supervisor.recommended_next_action,
        "resume_or_interrupt_tasks"
    );
    assert!(supervisor.paused_task_ids.contains(&spawned.task.task_id));

    let observatory = runtime
        .worker_subagent_observatory()
        .expect("observatory should show paused task");
    assert_eq!(observatory.paused_count, 1);
    assert_eq!(
        observatory.recommended_next_action,
        "resume_or_interrupt_paused_subagents"
    );
    assert!(observatory.lanes.iter().any(|lane| {
        lane.task_id == spawned.task.task_id && lane.control_action == "resume_or_interrupt"
    }));

    let resumed = runtime
        .resume_worker_task(&spawned.task.task_id)
        .expect("task should resume");
    assert_eq!(resumed.task.status, WorkerTaskStatus::Queued);
    assert_eq!(resumed.task.paused_from_status, None);

    let interrupted = runtime
        .interrupt_worker_task(&spawned.task.task_id)
        .expect("task should interrupt");
    assert_eq!(interrupted.task.status, WorkerTaskStatus::Interrupted);
    let supervisor = runtime
        .worker_task_supervisor()
        .expect("supervisor should show interrupted task");
    assert_eq!(supervisor.interrupted_count, 1);
    assert_eq!(supervisor.interrupted_control_count, 1);
    assert!(
        supervisor
            .interrupted_task_ids
            .contains(&spawned.task.task_id)
    );

    let console = runtime
        .operator_console()
        .expect("operator console should summarize live controls");
    assert!(console.operator_console_complete);
    assert!(console.task_queue_panel);
    assert!(console.subagent_tree_panel);
    assert!(console.command_stream_panel);
    assert!(console.patch_evidence_review_panel);
    assert!(console.pause_control_ready);
    assert!(console.resume_control_ready);
    assert!(console.interrupt_control_ready);
    assert!(console.steer_control_ready);
    assert!(
        console
            .control_commands
            .iter()
            .any(|command| command.contains("/steer-task"))
    );
    assert!(
        console
            .control_commands
            .iter()
            .any(|command| command.contains("/pause-task"))
    );
    assert!(
        console
            .recent_events
            .iter()
            .any(|event| event.kind == EventKind::TaskSteered)
    );
    assert!(
        console
            .recent_events
            .iter()
            .any(|event| event.kind == EventKind::TaskInterrupted)
    );
}

#[test]
fn worker_task_nested_spawn_depth_blocks_recursive_orchestrators() {
    let runtime = RuntimeKernel::new();
    let root = runtime
        .spawn_worker_task("root", "coordinate one nested worker", None)
        .expect("root task should spawn");
    assert_eq!(root.task.spawn_depth, 0);
    assert_eq!(root.task.max_spawn_depth, 1);

    let child = runtime
        .spawn_worker_task_with_parent(
            "child",
            "nested child worker",
            None,
            Vec::new(),
            Some(root.task.task_id.clone()),
            1,
        )
        .expect("one nested child should fit default depth policy");
    assert_eq!(child.task.parent_task_id, Some(root.task.task_id.clone()));
    assert_eq!(child.task.spawn_depth, 1);

    let denied = runtime
        .spawn_worker_task_with_parent(
            "grandchild",
            "recursive nested worker",
            None,
            Vec::new(),
            Some(child.task.task_id.clone()),
            1,
        )
        .expect_err("recursive grandchild should be denied");
    assert!(denied.0.contains("recursive spawn denied"));
}

#[test]
fn worker_output_url_exfiltration_is_redacted() {
    let raw = "callback=https://example.test/hook?token=sk-live-secret&safe=1\nOPENAI_API_KEY=sk-test-secret\nAuthorization:private-token";
    let redacted = redact_worker_output_exfiltration(raw);

    assert!(redacted.contains("token=[REDACTED]&safe=1"));
    assert!(redacted.contains("OPENAI_API_KEY=[REDACTED]"));
    assert!(redacted.contains("Authorization:[REDACTED]"));
    assert!(!redacted.contains("sk-live-secret"));
    assert!(!redacted.contains("sk-test-secret"));
    assert!(!redacted.contains("private-token"));
}

#[test]
fn worker_execution_backend_report_covers_local_and_remote_contracts() {
    let runtime = RuntimeKernel::new();
    let report = runtime
        .worker_execution_backends()
        .expect("worker backend report should build");

    assert_eq!(report.backend_count, 3);
    assert_eq!(report.active_backend_id, "local-host-process");
    assert_eq!(
        report.active_backend_kind,
        WorkerExecutionBackendKind::LocalHostProcess
    );
    assert!(report.local_backend_ready);
    assert_eq!(report.remote_backend_count, 2);
    assert_eq!(report.configured_remote_backend_count, 0);
    assert!(!report.remote_execution_enabled);
    assert!(report.file_sync_policy_required);
    assert!(report.credential_mount_policy_required);
    assert!(report.remote_path_traversal_denied);
    assert!(report.remote_credential_mounts_deny_by_default);
    assert!(report.remote_file_sync_manifest_required);
    assert!(report.remote_child_side_effects_blocked);
    assert!(report.remote_safety_regression_pack_ready);
    assert!(report.environment_process_evidence_contract);
    assert!(report.backends.iter().any(|backend| {
        backend.kind == WorkerExecutionBackendKind::Docker
            && backend.status == WorkerExecutionBackendStatus::RequiresConfiguration
            && backend.remote
            && backend.file_sync_supported
            && backend
                .file_sync_manifest_policy
                .contains("workspace_sync_manifest")
            && backend.credential_mount_policy.contains("deny_by_default")
            && backend
                .path_traversal_policy
                .contains("deny_path_traversal")
            && backend
                .child_side_effect_policy
                .contains("block_child_side_effects")
    }));
    assert!(report.backends.iter().any(|backend| {
        backend.kind == WorkerExecutionBackendKind::Ssh
            && backend.status == WorkerExecutionBackendStatus::RequiresConfiguration
            && backend.remote
            && backend.credential_mount_policy.contains("deny_by_default")
            && backend
                .path_traversal_policy
                .contains("deny_path_traversal")
            && backend
                .child_side_effect_policy
                .contains("block_child_side_effects")
    }));
}

#[test]
fn remote_worker_backend_denies_execution_until_explicitly_configured() {
    let runtime = RuntimeKernel::new();
    let mut task = runtime
        .spawn_worker_task(
            "remote-coding-builder",
            "autonomous coding subagent remote backend dry run",
            None,
        )
        .expect("task should spawn")
        .task;
    task.execution_backend = WorkerExecutionBackendBinding {
        backend_id: "docker-sandbox".into(),
        kind: WorkerExecutionBackendKind::Docker,
        remote: true,
        evidence_kind: "environment_process".into(),
    };
    let workspace_root = runtime
        .workspace_root()
        .expect("workspace root should resolve");
    let run = super::run_worker_environment_command(
        &task,
        &workspace_root,
        &task.safety_envelope,
        "remote-deny-check",
        "sh -c 'echo should-not-run'",
        "/bin/sh",
        &["-c", "echo should-not-run"],
    );

    assert_eq!(run.backend_id, "docker-sandbox");
    assert_eq!(run.backend_kind, WorkerExecutionBackendKind::Docker);
    assert!(run.remote_backend);
    assert!(!run.passed);
    assert_eq!(run.exit_code, 126);
    assert!(run.stdout.is_empty());
    assert!(
        run.stderr
            .contains("requires explicit remote configuration")
    );
    assert!(
        run.resource_limit_violation
            .as_deref()
            .unwrap_or_default()
            .contains("requires explicit remote configuration")
    );
}

#[test]
fn worker_task_permission_envelope_sandboxes_review_lanes() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_execution_profile(ExecutionProfile::FullAccess)
        .expect("profile switch should work");
    runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("filesystem scope switch should work");
    runtime
        .switch_write_path_scope(WritePathScope::AnyPath)
        .expect("write scope switch should work");

    let review = runtime
        .spawn_worker_task("security-review", "audit permissions", None)
        .expect("review task should spawn");
    assert_eq!(
        review.task.permission_envelope.execution_profile,
        ExecutionProfile::ReadOnlyTools
    );
    assert_eq!(
        review.task.permission_envelope.filesystem_scope,
        FilesystemScope::WorkspaceOnly
    );
    assert_eq!(
        review.task.permission_envelope.write_scope,
        WritePathScope::ArtifactsOnly
    );
    assert!(!review.task.permission_envelope.network_allowed);
    assert_eq!(
        runtime
            .execution_profile_for_session(&review.task.worker_session_id)
            .expect("worker profile should resolve"),
        ExecutionProfile::ReadOnlyTools
    );

    let builder = runtime
        .spawn_worker_task("patch-builder", "prepare patch", None)
        .expect("builder task should spawn");
    assert_eq!(
        builder.task.permission_envelope.execution_profile,
        ExecutionProfile::FullAccess
    );
    assert_eq!(
        builder.task.permission_envelope.write_scope,
        WritePathScope::WorkspaceOnly
    );
}

#[tokio::test]
async fn worker_task_run_completes_in_isolated_worker_session() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("builder", "say hello from a worker lane", None)
        .expect("task should spawn");

    let run = runtime
        .run_worker_task(&spawned.task.task_id)
        .await
        .expect("task should run");

    assert_eq!(run.task.status, WorkerTaskStatus::Completed);
    assert_eq!(run.task.attempt_count, 1);
    assert!(run.result.is_some());
    assert_eq!(run.artifact_count, 2);
    assert_eq!(run.task.artifacts.len(), 2);
    assert!(run.task.diff_summary.is_some());
    assert_eq!(run.patch_proposal_count, 1);
    assert_eq!(run.task.patch_proposals.len(), 1);
    assert_eq!(run.loop_step_count, 5);
    assert_eq!(run.task.loop_steps.len(), 5);
    assert_eq!(run.task.loop_steps[0].phase, WorkerTaskLoopPhase::Plan);
    assert_eq!(
        run.task.patch_proposals[0].apply_status,
        WorkerTaskPatchApplyStatus::Proposed
    );
    assert_eq!(run.task.artifacts[0].kind, "run_summary");
    let task_events = runtime
        .query_events(25, None, Some(&run.task.worker_session_id))
        .expect("task events should be queryable");
    assert!(
        task_events
            .iter()
            .any(|event| event.event.kind == EventKind::TaskSpawned)
    );
    assert!(
        task_events
            .iter()
            .any(|event| event.event.kind == EventKind::TaskStarted)
    );
    assert!(
        task_events
            .iter()
            .any(|event| event.event.kind == EventKind::TaskCompleted)
    );

    let join = runtime
        .join_worker_tasks(Some("builder"))
        .expect("completed tasks should join safely");
    assert!(join.safe_to_join);
    assert_eq!(join.completed_count, 1);
    assert_eq!(join.artifact_count, 2);
    assert_eq!(join.diff_ready_count, 1);
    assert_eq!(join.patch_proposal_count, 1);
    assert_eq!(join.joined[0].worker_id, "builder");
    assert_eq!(join.joined[0].artifacts.len(), 2);
    assert_eq!(join.joined[0].patch_proposals.len(), 1);
    assert_eq!(join.loop_step_count, 5);
    assert_eq!(join.joined[0].loop_steps.len(), 5);
    assert_eq!(join.merge_safe_count, 1);
    assert_eq!(join.merge_needs_review_count, 0);
    assert_eq!(join.merge_blocked_count, 0);
    assert_eq!(
        join.joined[0].merge_risk.decision,
        WorkerTaskMergeDecision::SafeToMerge
    );
    assert!(join.joined[0].merge_risk.replay_passed);

    let loop_report = runtime
        .worker_task_loop(&run.task.task_id)
        .expect("loop should be reviewable");
    assert_eq!(loop_report.loop_step_count, 5);
    assert_eq!(loop_report.failed_count, 0);

    let evidence = runtime
        .worker_task_evidence(&run.task.task_id)
        .expect("evidence should be reviewable");
    assert_eq!(evidence.task_id, run.task.task_id);
    assert!(evidence.evidence_count >= 10);
    assert_eq!(evidence.entries[0].previous_hash.as_deref(), None);
    assert!(
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "permission_envelope")
    );
    assert_eq!(
        evidence.chain_head,
        evidence.entries.last().unwrap().entry_hash
    );

    let replay = runtime
        .worker_task_replay_audit(&run.task.task_id)
        .expect("replay audit should be available");
    assert!(replay.replay_passed);
    assert!(replay.hash_chain_valid);
    assert!(replay.permission_policy_valid);
    assert!(replay.lifecycle_valid);
    assert!(replay.artifact_records_valid);
    assert!(replay.patch_records_valid);
    assert_eq!(replay.chain_head, replay.replayed_chain_head);

    let promotion_before_apply = runtime
        .worker_task_promotion_gate(&run.task.task_id)
        .expect("promotion gate should report before apply");
    assert_eq!(
        promotion_before_apply.decision,
        WorkerTaskPromotionDecision::NeedsReview
    );
    assert_eq!(promotion_before_apply.unapplied_patch_count, 1);
    let ledger_before_apply = runtime
        .worker_task_promotion_ledger(&run.task.task_id)
        .expect("promotion ledger should report before apply");
    assert_eq!(ledger_before_apply.ledger_count, 4);
    assert_eq!(
        ledger_before_apply.promotion_decision,
        WorkerTaskPromotionDecision::NeedsReview
    );
    assert!(
        ledger_before_apply
            .chain_head
            .starts_with("hepta-promotion:")
    );
    let handoff_before_apply = runtime
        .worker_task_handoff_bundle(&run.task.task_id)
        .expect("handoff bundle should report before apply");
    assert!(!handoff_before_apply.handoff_ready);
    assert!(handoff_before_apply.signature.starts_with("hepta-handoff:"));

    let patch_id = join.joined[0].patch_proposals[0].patch_id.clone();
    let applied = runtime
        .mark_worker_task_patch_applied(&run.task.task_id, &patch_id)
        .expect("patch should be markable as applied");
    assert_eq!(applied.applied_count, 1);
    assert_eq!(applied.proposed_count, 0);

    let promotion_after_apply = runtime
        .worker_task_promotion_gate(&run.task.task_id)
        .expect("promotion gate should report after apply");
    assert_eq!(
        promotion_after_apply.decision,
        WorkerTaskPromotionDecision::Promoted
    );
    assert!(promotion_after_apply.promotion_allowed);
    let ledger_after_apply = runtime
        .worker_task_promotion_ledger(&run.task.task_id)
        .expect("promotion ledger should report after apply");
    assert_eq!(
        ledger_after_apply.promotion_decision,
        WorkerTaskPromotionDecision::Promoted
    );
    assert!(ledger_after_apply.promotion_allowed);
    assert_eq!(
        ledger_after_apply.chain_head,
        ledger_after_apply.entries.last().unwrap().entry_hash
    );
    let handoff_after_apply = runtime
        .worker_task_handoff_bundle(&run.task.task_id)
        .expect("handoff bundle should report after apply");
    assert!(handoff_after_apply.handoff_ready);
    assert!(handoff_after_apply.signature.starts_with("hepta-handoff:"));
    assert!(
        handoff_after_apply
            .evidence
            .chain_head
            .starts_with("hepta-evidence:")
    );

    let patch_review = runtime
        .worker_task_patches(&run.task.task_id)
        .expect("patches should be reviewable");
    assert_eq!(patch_review.patch_count, 1);
    assert_eq!(patch_review.applied_count, 1);
    assert!(patch_review.patches[0].transaction_id.is_some());
    let target_path = super::resolve_path_within_root(
        &runtime
            .workspace_root()
            .expect("workspace root should exist"),
        std::path::Path::new(&patch_review.patches[0].file_path),
    );
    let _ = std::fs::remove_file(target_path);
}

#[tokio::test]
async fn worker_task_context_recall_handoff_is_operator_opt_in_without_snippet_leak() {
    let disabled_runtime = RuntimeKernel::new();
    disabled_runtime
        .memory
        .put(hepta_core::MemoryRecord {
            id: "worker-disabled-source-id".into(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: format!("worker-needle {}", "disabled-worker-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    let disabled_task = disabled_runtime
        .spawn_worker_task("builder", "worker-needle", None)
        .expect("task should spawn");

    let disabled_run = disabled_runtime
        .run_worker_task_with_context_recall_handoff(
            &disabled_task.task.task_id,
            WorkerTaskContextRecallHandoffPolicy::Disabled,
        )
        .await
        .expect("disabled worker task should run");

    assert_eq!(disabled_run.run.task.status, WorkerTaskStatus::Completed);
    assert!(!disabled_run.selected_snippets_present);
    assert_eq!(disabled_run.selected_snippet_count, 0);
    assert!(disabled_run.provider_rollup.is_none());

    let opted_runtime = RuntimeKernel::new();
    opted_runtime
        .memory
        .put(hepta_core::MemoryRecord {
            id: "worker-context-source-id".into(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: format!(
                "worker-needle {}",
                "operator-worker-safe-context ".repeat(80)
            ),
        })
        .await
        .expect("memory should store");
    let opted_task = opted_runtime
        .spawn_worker_task("builder", "worker-needle", None)
        .expect("task should spawn");

    let opted_run = opted_runtime
        .run_worker_task_with_context_recall_handoff(
            &opted_task.task.task_id,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
        )
        .await
        .expect("operator-approved worker task should run");
    let encoded = serde_json::to_string(&opted_run).expect("report should serialize");
    let debug = format!("{opted_run:?}");

    assert_eq!(opted_run.run.task.status, WorkerTaskStatus::Completed);
    assert!(opted_run.selected_snippets_present);
    assert!(opted_run.selected_snippet_count > 0);
    assert!(
        opted_run
            .provider_rollup
            .as_ref()
            .expect("provider rollup should be present")
            .recall_selection
            .has_count_integrity()
    );
    assert!(
        opted_run
            .run
            .result
            .as_ref()
            .expect("worker result should be present")
            .final_text
            .contains("[chat] model reply: worker-needle")
    );
    for forbidden in [
        "operator-worker-safe-context",
        "worker-context-source-id",
        "[redacted-query]",
        "source_id",
        "source_memory_ids",
    ] {
        assert!(
            !encoded.contains(forbidden),
            "serialized worker report leaked {forbidden}"
        );
        assert!(
            !debug.contains(forbidden),
            "worker report debug leaked {forbidden}"
        );
    }
}

#[tokio::test]
async fn worker_task_context_recall_handoff_scheduler_policy_is_explicit_without_leak() {
    let disabled_ready_runtime = RuntimeKernel::new();
    disabled_ready_runtime
        .memory
        .put(hepta_core::MemoryRecord {
            id: "worker-ready-disabled-source-id".into(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: format!("ready-needle {}", "disabled-ready-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    disabled_ready_runtime
        .spawn_worker_task("ready", "ready-needle", None)
        .expect("ready task should spawn");

    let disabled_ready = disabled_ready_runtime
        .run_ready_worker_tasks_with_context_recall_handoff(
            Some(10),
            None,
            WorkerTaskContextRecallHandoffPolicy::Disabled,
        )
        .await
        .expect("disabled ready batch should run");

    assert_eq!(disabled_ready.ran_count, 1);
    assert_eq!(
        disabled_ready.context_recall_handoff_policy,
        WorkerTaskContextRecallHandoffPolicy::Disabled
    );
    assert_eq!(disabled_ready.selected_snippets_present_count, 0);
    assert_eq!(disabled_ready.selected_snippet_count, 0);
    assert!(disabled_ready.runs[0].provider_rollup.is_none());

    let opted_ready_runtime = RuntimeKernel::new();
    opted_ready_runtime
        .memory
        .put(hepta_core::MemoryRecord {
            id: "worker-ready-source-id".into(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: format!("ready-needle {}", "operator-ready-safe-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    opted_ready_runtime
        .spawn_worker_task("ready", "ready-needle", None)
        .expect("ready task should spawn");

    let opted_ready = opted_ready_runtime
        .run_ready_worker_tasks_with_context_recall_handoff(
            Some(10),
            None,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
        )
        .await
        .expect("operator-approved ready batch should run");
    let ready_encoded = serde_json::to_string(&opted_ready).expect("report should serialize");
    let ready_debug = format!("{opted_ready:?}");

    assert_eq!(opted_ready.ran_count, 1);
    assert_eq!(
        opted_ready.context_recall_handoff_policy,
        WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
    );
    assert_eq!(opted_ready.selected_snippets_present_count, 1);
    assert!(opted_ready.selected_snippet_count > 0);
    assert!(
        opted_ready.runs[0]
            .provider_rollup
            .as_ref()
            .expect("provider rollup should be present")
            .recall_selection
            .has_count_integrity()
    );

    let opted_due_runtime = RuntimeKernel::new();
    opted_due_runtime
        .memory
        .put(hepta_core::MemoryRecord {
            id: "worker-due-source-id".into(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: format!("due-needle {}", "operator-due-safe-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    let scheduled = opted_due_runtime
        .spawn_worker_task("scheduler", "due-needle", Some("delay:10ms"))
        .expect("scheduled task should spawn");
    let due_at = scheduled
        .task
        .next_run_unix_ms
        .expect("scheduled task should have next run");

    let opted_due = opted_due_runtime
        .run_due_worker_tasks_with_context_recall_handoff(
            Some(due_at),
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
        )
        .await
        .expect("operator-approved due batch should run");
    let due_encoded = serde_json::to_string(&opted_due).expect("report should serialize");
    let due_debug = format!("{opted_due:?}");

    assert_eq!(opted_due.due_count, 1);
    assert_eq!(opted_due.ran_count, 1);
    assert_eq!(
        opted_due.context_recall_handoff_policy,
        WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
    );
    assert_eq!(opted_due.selected_snippets_present_count, 1);
    assert!(opted_due.selected_snippet_count > 0);
    assert!(
        opted_due.runs[0]
            .provider_rollup
            .as_ref()
            .expect("provider rollup should be present")
            .recall_selection
            .has_count_integrity()
    );

    for rendered in [ready_encoded, ready_debug, due_encoded, due_debug] {
        for forbidden in [
            "operator-ready-safe-context",
            "operator-due-safe-context",
            "worker-ready-source-id",
            "worker-due-source-id",
            "[redacted-query]",
            "source_id",
            "source_memory_ids",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "scheduler report leaked {forbidden}"
            );
        }
    }
}

#[tokio::test]
async fn autonomous_coding_worker_runs_real_inspect_command_patch_handoff_loop() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task(
            "coding-builder",
            "autonomous coding subagent implement worker runtime evidence",
            None,
        )
        .expect("autonomous coding task should spawn");
    assert_eq!(
        spawned.task.execution_mode,
        WorkerTaskExecutionMode::AutonomousCoding
    );

    let run = runtime
        .run_worker_task(&spawned.task.task_id)
        .await
        .expect("autonomous coding task should run");
    assert_eq!(run.task.status, WorkerTaskStatus::Completed);
    assert_eq!(
        run.task.execution_mode,
        WorkerTaskExecutionMode::AutonomousCoding
    );
    assert_eq!(run.command_run_count, 6);
    assert_eq!(run.task.command_runs.len(), 6);
    assert_eq!(
        run.task.execution_backend.kind,
        WorkerExecutionBackendKind::LocalHostProcess
    );
    assert_eq!(run.task.execution_backend.backend_id, "local-host-process");
    assert_eq!(run.coding_round_count, 2);
    assert_eq!(run.task.coding_rounds.len(), 2);
    assert!(run.task.coding_rounds.iter().all(|round| round.passed));
    assert!(
        run.task.coding_rounds[0]
            .command_ids
            .iter()
            .all(|id| id.contains(":round-1-"))
    );
    assert!(
        run.task.coding_rounds[1]
            .command_ids
            .iter()
            .all(|id| id.contains(":round-2-"))
    );
    assert_eq!(run.file_lease_count, run.task.file_leases.len());
    assert!(!run.task.file_leases.is_empty());
    assert!(run.task.file_leases.iter().all(|lease| {
        lease.status == WorkerTaskFileLeaseStatus::HeldForReview
            && lease.worker_session_id == run.task.worker_session_id
            && lease.conflict_task_ids.is_empty()
    }));
    assert!(run.task.command_runs.iter().all(|command| command.passed));
    assert!(run.task.command_runs.iter().all(|command| {
        command.execution_origin == WorkerTaskCommandRunOrigin::HostProcess
            && command.backend_id == "local-host-process"
            && command.backend_kind == WorkerExecutionBackendKind::LocalHostProcess
            && !command.remote_backend
            && command.working_directory.is_some()
            && command.sandboxed
            && !command.timed_out
            && command.resource_limit_violation.is_none()
    }));
    assert!(run.task.safety_envelope.cancel_supported);
    assert!(run.task.safety_envelope.cancel_checked_before_host_command);
    assert!(
        run.task
            .safety_envelope
            .sandbox
            .allowed_programs
            .contains(&"/bin/sh".into())
    );
    assert!(
        run.task.command_runs[0]
            .stdout
            .contains("cargo_toml=present")
    );
    assert!(
        run.task
            .artifacts
            .iter()
            .any(|artifact| artifact.kind == "code_inspection")
    );
    assert!(
        run.task
            .artifacts
            .iter()
            .any(|artifact| artifact.kind == "command_transcript")
    );
    assert_eq!(run.patch_proposal_count, 1);
    assert!(
        run.task.patch_proposals[0]
            .unified_diff
            .contains("Autonomous coding worker proposal")
    );
    assert_eq!(run.loop_step_count, 5);
    assert_eq!(run.task.loop_steps[3].phase, WorkerTaskLoopPhase::Test);

    let evidence = runtime
        .worker_task_evidence(&run.task.task_id)
        .expect("autonomous evidence should build");
    assert!(
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "command_run")
    );
    let replay = runtime
        .worker_task_replay_audit(&run.task.task_id)
        .expect("autonomous replay should build");
    assert!(replay.replay_passed);
    assert!(
        replay
            .checks
            .iter()
            .any(|check| check.check_id == "command_records" && check.passed)
    );
    assert!(
        replay
            .checks
            .iter()
            .any(|check| check.check_id == "host_process_command_records" && check.passed)
    );
    assert!(replay.coding_rounds_valid);
    assert!(replay.multi_round_loop_valid);
    assert!(replay.file_lease_records_valid);
    assert!(replay.backend_records_valid);
    assert!(
        replay
            .checks
            .iter()
            .any(|check| check.check_id == "multi_round_loop" && check.passed)
    );
    assert!(
        replay
            .checks
            .iter()
            .any(|check| check.check_id == "file_lease_records" && check.passed)
    );
    assert!(replay.safety_limits_valid);
    assert!(
        replay
            .checks
            .iter()
            .any(|check| check.check_id == "safety_limits" && check.passed)
    );

    let observatory = runtime
        .worker_subagent_observatory()
        .expect("observatory should build");
    assert_eq!(observatory.autonomous_count, 1);
    assert_eq!(
        observatory.held_file_lease_count,
        run.task.file_leases.len()
    );
    assert_eq!(observatory.conflicted_file_lease_count, 0);
    assert!(observatory.lanes.iter().any(|lane| {
        lane.task_id == run.task.task_id && lane.file_lease_count == run.task.file_leases.len()
    }));

    let patch = run.task.patch_proposals[0].clone();
    assert!(run.task.file_leases.iter().any(|lease| {
        lease.target_path == patch.file_path
            && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
    }));
    let applied = runtime
        .apply_worker_task_patch(&run.task.task_id, &patch.patch_id)
        .expect("autonomous patch should apply");
    assert_eq!(applied.applied_count, 1);
    assert!(applied.patches[0].transaction_id.is_some());
    let handoff = runtime
        .worker_task_handoff_bundle(&run.task.task_id)
        .expect("handoff should build after apply");
    assert!(handoff.handoff_ready);
    let target_path = super::resolve_path_within_root(
        &runtime
            .workspace_root()
            .expect("workspace root should exist"),
        std::path::Path::new(&patch.file_path),
    );
    let _ = std::fs::remove_file(target_path);
}

#[test]
fn autonomous_worker_host_command_enforces_sandbox_timeout_and_output_limits() {
    let runtime = RuntimeKernel::new();
    let task = runtime
        .spawn_worker_task(
            "coding-builder",
            "autonomous coding subagent safety controls",
            None,
        )
        .expect("task should spawn")
        .task;
    let workspace_root = runtime
        .workspace_root()
        .expect("workspace root should resolve");
    let mut safety = task.safety_envelope.clone();
    safety.resource_limits.per_command_timeout_ms = 5;
    let timed_out = super::run_worker_host_command(
        &task,
        &workspace_root,
        &safety,
        "timeout-check",
        "sh -c 'sleep 0.05'",
        "/bin/sh",
        &["-c", "sleep 0.05"],
    );
    assert!(timed_out.sandboxed);
    assert!(timed_out.timed_out);
    assert_eq!(
        timed_out.resource_limit_violation.as_deref(),
        Some("command_timeout")
    );

    let mut blocked_safety = task.safety_envelope.clone();
    blocked_safety.sandbox.allowed_programs = vec!["/usr/bin/false".into()];
    let blocked = super::run_worker_host_command(
        &task,
        &workspace_root,
        &blocked_safety,
        "sandbox-check",
        "sh -c 'echo blocked'",
        "/bin/sh",
        &["-c", "echo blocked"],
    );
    assert!(!blocked.passed);
    assert_eq!(blocked.exit_code, 126);
    assert!(
        blocked
            .resource_limit_violation
            .as_deref()
            .unwrap_or_default()
            .contains("sandbox disallows program")
    );

    let mut output_safety = task.safety_envelope.clone();
    output_safety.resource_limits.max_stdout_bytes = 4;
    let truncated = super::run_worker_host_command(
        &task,
        &workspace_root,
        &output_safety,
        "stdout-limit-check",
        "sh -c 'printf abcdef'",
        "/bin/sh",
        &["-c", "printf abcdef"],
    );
    assert!(truncated.passed);
    assert!(truncated.stdout_truncated);
    assert_eq!(
        truncated.resource_limit_violation.as_deref(),
        Some("output_truncated")
    );
}

#[tokio::test]
async fn cancelled_worker_task_does_not_execute_commands_and_is_supervisor_visible() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task(
            "coding-builder",
            "autonomous coding subagent cancel before run",
            None,
        )
        .expect("task should spawn");
    let cancelled = runtime
        .cancel_worker_task(&spawned.task.task_id)
        .expect("task should cancel");
    assert_eq!(cancelled.task.status, WorkerTaskStatus::Cancelled);
    assert!(cancelled.task.safety_envelope.cancel_supported);
    assert!(cancelled.task.command_runs.is_empty());

    let run = runtime.run_worker_task(&spawned.task.task_id).await;
    assert!(
        run.expect_err("cancelled task should not run")
            .0
            .contains("already cancelled")
    );

    let supervisor = runtime
        .worker_task_supervisor()
        .expect("supervisor should build");
    assert_eq!(supervisor.cancelled_count, 1);
    assert_eq!(supervisor.command_run_count, 0);
    assert!(
        supervisor
            .safety_envelopes
            .iter()
            .all(|envelope| envelope.cancel_supported)
    );
}

#[tokio::test]
async fn worker_task_patch_apply_reports_conflicts_without_overwriting() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("builder", "draft patch conflict flow", None)
        .expect("task should spawn");
    let run = runtime
        .run_worker_task(&spawned.task.task_id)
        .await
        .expect("task should run");
    let patch = run.task.patch_proposals[0].clone();
    assert_eq!(
        run.task.permission_envelope.write_scope,
        WritePathScope::ArtifactsOnly
    );
    assert!(run.task.file_leases.iter().any(|lease| {
        lease.target_path == patch.file_path
            && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
    }));
    let target_path = super::resolve_path_within_root(
        &runtime
            .workspace_root()
            .expect("workspace root should exist"),
        std::path::Path::new(&patch.file_path),
    );
    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent).expect("parent should be creatable");
    }
    std::fs::write(&target_path, "different content\n")
        .expect("conflicting target should be writable");

    let review = runtime
        .apply_worker_task_patch(&run.task.task_id, &patch.patch_id)
        .expect("conflict should be reported as review state");
    assert_eq!(review.conflicted_count, 1);
    assert_eq!(review.applied_count, 0);
    assert_eq!(review.proposed_count, 1);
    assert_eq!(review.patch_count, 2);
    assert!(review.patches[0].conflict_reason.is_some());
    let revision = review
        .patches
        .iter()
        .find(|candidate| candidate.revision_of.as_deref() == Some(&patch.patch_id))
        .expect("conflict should generate a revision proposal")
        .clone();
    assert_eq!(revision.revision_index, 1);
    let revised_task = runtime
        .find_worker_task(&run.task.task_id)
        .expect("revised task should remain queryable");
    assert!(revised_task.file_leases.iter().any(|lease| {
        lease.target_path == revision.file_path
            && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
    }));
    let loop_report = runtime
        .worker_task_loop(&run.task.task_id)
        .expect("loop should include revision iteration");
    assert_eq!(loop_report.loop_step_count, 6);
    assert_eq!(
        std::fs::read_to_string(&target_path).expect("target should remain"),
        "different content\n"
    );

    let retry = runtime
        .apply_worker_task_patch_set(&run.task.task_id)
        .expect("revision retry should apply");
    assert_eq!(retry.attempted_count, 1);
    assert_eq!(retry.applied_count, 1);
    assert_eq!(retry.conflicted_count, 1);
    let revised_target = super::resolve_path_within_root(
        &runtime
            .workspace_root()
            .expect("workspace root should exist"),
        std::path::Path::new(&revision.file_path),
    );
    assert!(revised_target.exists());
    let _ = std::fs::remove_file(&target_path);
    let _ = std::fs::remove_file(&revised_target);
}

#[tokio::test]
async fn worker_task_patch_set_applies_multiple_files() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("builder", "draft multi file patch set", None)
        .expect("task should spawn");
    let run = runtime
        .run_worker_task(&spawned.task.task_id)
        .await
        .expect("task should run");
    assert_eq!(run.task.patch_proposals.len(), 2);
    assert_eq!(
        run.task.permission_envelope.write_scope,
        WritePathScope::ArtifactsOnly
    );
    assert!(run.task.patch_proposals.iter().all(|patch| {
        run.task.file_leases.iter().any(|lease| {
            lease.target_path == patch.file_path
                && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
        })
    }));
    let patch = &run.task.patch_proposals[0];
    let ordinary_scope_error = runtime
        .prepare_sealed_write_target(
            &run.task.worker_session_id,
            "write_file",
            "write_file",
            &patch.file_path,
            "create",
            false,
            None,
        )
        .expect_err("ordinary tool scope must remain artifacts-only");
    assert!(ordinary_scope_error.0.contains("outside artifacts root"));
    let mut missing_lease = run.task.clone();
    missing_lease
        .file_leases
        .retain(|lease| lease.target_path != patch.file_path);
    assert!(
        runtime
            .authorize_worker_patch_apply(&missing_lease, patch)
            .expect_err("missing exact lease must fail")
            .0
            .contains("no exact file lease")
    );
    let mut unsafe_envelope = run.task.clone();
    unsafe_envelope.safety_envelope.sandbox.workspace_root = "/".into();
    assert!(
        runtime
            .authorize_worker_patch_apply(&unsafe_envelope, patch)
            .expect_err("mismatched safety root must fail")
            .0
            .contains("safety envelope")
    );

    let applied = runtime
        .apply_worker_task_patch_set(&run.task.task_id)
        .expect("patch set should apply");
    assert_eq!(applied.patch_count, 2);
    assert_eq!(applied.attempted_count, 2);
    assert_eq!(applied.applied_count, 2);
    assert_eq!(applied.conflicted_count, 0);
    assert_eq!(applied.transaction_ids.len(), 2);
    let target_paths = applied
        .review
        .patches
        .iter()
        .map(|patch| {
            super::resolve_path_within_root(
                &runtime
                    .workspace_root()
                    .expect("workspace root should exist"),
                std::path::Path::new(&patch.file_path),
            )
        })
        .collect::<Vec<_>>();
    for target_path in &target_paths {
        assert!(target_path.exists());
    }

    let rollback = runtime
        .rollback_worker_task_patch_set(&run.task.task_id)
        .expect("patch set rollback should succeed");
    assert_eq!(rollback.attempted_count, 2);
    assert_eq!(rollback.rolled_back_count, 2);
    assert_eq!(rollback.failed_count, 0);
    assert_eq!(rollback.review.rolled_back_count, 2);
    for target_path in &target_paths {
        assert!(!target_path.exists());
    }
}

#[tokio::test]
async fn scheduled_worker_tasks_run_when_due() {
    let runtime = RuntimeKernel::new();
    let scheduled = runtime
        .spawn_worker_task("scheduler", "run scheduled task", Some("delay:10ms"))
        .expect("scheduled task should spawn");
    assert_eq!(scheduled.task.status, WorkerTaskStatus::Scheduled);
    let next_run = scheduled
        .task
        .next_run_unix_ms
        .expect("scheduled task should have next run");

    let early = runtime
        .run_due_worker_tasks(Some(next_run.saturating_sub(1)))
        .await
        .expect("early due check should succeed");
    assert_eq!(early.due_count, 0);
    assert_eq!(early.ran_count, 0);

    let due = runtime
        .run_due_worker_tasks(Some(next_run))
        .await
        .expect("due task should run");
    assert_eq!(due.due_count, 1);
    assert_eq!(due.ran_count, 1);
    assert_eq!(due.runs[0].task.status, WorkerTaskStatus::Completed);
}

#[tokio::test]
async fn dependent_worker_tasks_wait_for_completed_dependencies() {
    let runtime = RuntimeKernel::new();
    let parent = runtime
        .spawn_worker_task("parent", "complete first", None)
        .expect("parent should spawn");
    let child = runtime
        .spawn_worker_task_with_dependencies(
            "child",
            "run after parent",
            None,
            vec![parent.task.task_id.clone()],
        )
        .expect("child should spawn with dependency");

    let blocked = runtime
        .run_worker_task(&child.task.task_id)
        .await
        .expect_err("child should wait for dependency");
    assert!(blocked.0.contains("waiting on dependency"));

    runtime
        .run_worker_task(&parent.task.task_id)
        .await
        .expect("parent should complete");
    let child_run = runtime
        .run_worker_task(&child.task.task_id)
        .await
        .expect("child should run after dependency completes");
    assert_eq!(child_run.task.status, WorkerTaskStatus::Completed);
    assert_eq!(child_run.task.depends_on, vec![parent.task.task_id]);
}

#[test]
fn worker_task_spawn_persists_workspace_id() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task_in_workspace(
            "workspace-reviewer",
            Some("agent:workspace-alpha"),
            "summarize workspace-scoped task state",
            None,
        )
        .expect("workspace task should spawn");

    assert_eq!(spawned.task.workspace_id, "agent:workspace-alpha");
    let detail = runtime
        .worker_task_status(&spawned.task.task_id)
        .expect("task detail should resolve");
    assert_eq!(detail.task.workspace_id, "agent:workspace-alpha");
    let patches = runtime
        .worker_task_patches(&spawned.task.task_id)
        .expect("patch review should resolve");
    assert_eq!(patches.workspace_id, "agent:workspace-alpha");
}

#[test]
fn child_worker_task_inherits_parent_workspace() {
    let runtime = RuntimeKernel::new();
    let parent = runtime
        .spawn_worker_task_in_workspace(
            "parent",
            Some("agent:workspace-beta"),
            "coordinate nested worker lane",
            None,
        )
        .expect("parent task should spawn");
    let child = runtime
        .spawn_worker_task_with_parent_in_workspace(
            "child",
            None,
            "follow parent workspace",
            None,
            Vec::new(),
            Some(parent.task.task_id.clone()),
            1,
        )
        .expect("child task should inherit workspace");

    assert_eq!(child.task.workspace_id, "agent:workspace-beta");
}

#[tokio::test]
async fn ready_worker_batch_runs_only_unblocked_candidates() {
    let runtime = RuntimeKernel::new();
    let ready = runtime
        .spawn_worker_task("ready", "run now", None)
        .expect("ready task should spawn");
    let blocker = runtime
        .spawn_worker_task("blocker", "block dependency", None)
        .expect("blocker should spawn");
    let blocked = runtime
        .spawn_worker_task_with_dependencies(
            "blocked",
            "wait for blocker",
            None,
            vec![blocker.task.task_id.clone()],
        )
        .expect("blocked task should spawn");

    let report = runtime
        .run_ready_worker_tasks(Some(10), None)
        .await
        .expect("ready batch should run");

    assert_eq!(report.candidate_count, 3);
    assert_eq!(report.blocked_count, 1);
    assert!(report.blocked_task_ids.contains(&blocked.task.task_id));
    assert!(
        report
            .runs
            .iter()
            .any(|run| run.task.task_id == ready.task.task_id)
    );
    assert!(
        report
            .runs
            .iter()
            .any(|run| run.task.task_id == blocker.task.task_id)
    );
    assert_eq!(report.pressure.max_per_worker_concurrency, 2);
    assert_eq!(
        report.pressure.pressure_level,
        WorkerPoolPressureLevel::Normal
    );

    let second = runtime
        .run_ready_worker_tasks(None, None)
        .await
        .expect("second ready batch should run newly unblocked task");
    assert_eq!(second.ran_count, 1);
    assert_eq!(second.runs[0].task.task_id, blocked.task.task_id);
}

#[tokio::test]
async fn ready_worker_batch_respects_per_worker_pressure_limit() {
    let runtime = RuntimeKernel::new();
    let first = runtime
        .spawn_worker_task("pressure", "pressure one", None)
        .expect("first task should spawn");
    let second = runtime
        .spawn_worker_task("pressure", "pressure two", None)
        .expect("second task should spawn");
    let third = runtime
        .spawn_worker_task("pressure", "pressure three", None)
        .expect("third task should spawn");

    let report = runtime
        .run_ready_worker_tasks(None, None)
        .await
        .expect("ready batch should respect pressure limits");
    assert_eq!(report.candidate_count, 3);
    assert_eq!(report.ready_count, 2);
    assert_eq!(report.ran_count, 2);
    assert_eq!(
        report.pressure.pressure_level,
        WorkerPoolPressureLevel::Throttled
    );
    assert_eq!(
        report.pressure.throttled_task_ids,
        vec![third.task.task_id.clone()]
    );
    assert!(
        report
            .runs
            .iter()
            .any(|run| run.task.task_id == first.task.task_id)
    );
    assert!(
        report
            .runs
            .iter()
            .any(|run| run.task.task_id == second.task.task_id)
    );

    let second_pass = runtime
        .run_ready_worker_tasks(None, None)
        .await
        .expect("throttled task should run later");
    assert_eq!(second_pass.ran_count, 1);
    assert_eq!(second_pass.runs[0].task.task_id, third.task.task_id);
}

#[tokio::test]
async fn worker_task_timeout_failure_sets_retry_backoff_budget() {
    let runtime = RuntimeKernel::new();
    let spawned = runtime
        .spawn_worker_task("retry", "simulate-timeout then retry", None)
        .expect("task should spawn");

    let first = runtime
        .run_worker_task(&spawned.task.task_id)
        .await
        .expect("simulated failure should return a failed run report");
    assert_eq!(first.task.status, WorkerTaskStatus::Failed);
    assert_eq!(
        first.task.failure_kind,
        Some(WorkerTaskFailureKind::Timeout)
    );
    assert!(first.task.retry_after_unix_ms.is_some());

    let early = runtime
        .run_ready_worker_tasks(None, Some(first.task.retry_after_unix_ms.unwrap() - 1))
        .await
        .expect("early retry check should succeed");
    assert_eq!(early.ran_count, 0);
    assert_eq!(early.candidate_count, 0);

    let retry = runtime
        .run_ready_worker_tasks(None, first.task.retry_after_unix_ms)
        .await
        .expect("retry should run after backoff");
    assert_eq!(retry.ran_count, 1);
    assert_eq!(retry.runs[0].task.status, WorkerTaskStatus::Completed);
    assert_eq!(retry.runs[0].task.attempt_count, 2);
}

#[test]
fn worker_task_supervisor_reports_next_action() {
    let runtime = RuntimeKernel::new();
    let queued = runtime
        .spawn_worker_task("supervisor", "ready work", None)
        .expect("queued task should spawn");
    let report = runtime
        .worker_task_supervisor()
        .expect("supervisor report should build");

    assert_eq!(report.total_count, 1);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.ready_task_ids, vec![queued.task.task_id]);
    assert_eq!(report.recommended_next_action, "run_ready_tasks");
    assert_eq!(report.cancelled_count, 0);
    assert_eq!(report.sandbox_envelope_count, 1);
    assert!(
        report
            .safety_envelopes
            .iter()
            .all(|envelope| envelope.cancel_supported)
    );
    assert!(!report.attention_required);
}
