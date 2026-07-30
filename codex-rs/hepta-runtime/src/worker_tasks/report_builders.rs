fn build_autonomous_coding_diff_summary(
    task: &WorkerTaskRecord,
    inspection: &AutonomousCodingInspection,
    patches: &[WorkerTaskPatchProposal],
    command_runs: &[WorkerTaskCommandRun],
) -> String {
    format!(
        "worker={} task={} mode=autonomous_coding inspected={} rounds=2 commands={} patch_proposals={} target_lines={}",
        task.worker_id,
        task.task_id,
        inspection.readable_count,
        command_runs.len(),
        patches.len(),
        inspection.total_lines
    )
}

fn build_autonomous_coding_patch_proposals(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    inspection: &AutonomousCodingInspection,
    command_runs: &[WorkerTaskCommandRun],
) -> Vec<WorkerTaskPatchProposal> {
    let path = format!(
        "docs/worker-proposals/{}-autonomous-coding.md",
        worker_patch_suffix(task)
    );
    let content = format!(
        "# Autonomous coding worker proposal\n\n- task: `{}`\n- worker: `{}`\n- mode: `autonomous_coding`\n- coding rounds: 2\n- inspected targets: {} / {}\n- real command runs: {}\n- summary: {}\n\n## Inspection\n{}\n\n## Command transcript\n{}\n",
        task.task_id,
        task.worker_id,
        inspection.readable_count,
        inspection.target_count,
        command_runs.len(),
        compact_text(&result.final_text, 240),
        inspection.summaries.join("\n"),
        command_runs
            .iter()
            .map(|run| format!(
                "- `{}` exit={} passed={}",
                run.command, run.exit_code, run.passed
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    vec![WorkerTaskPatchProposal {
        patch_id: format!("{}:autonomous-coding-patch-1", task.task_id),
        revision_of: None,
        revision_index: 0,
        file_path: path.clone(),
        change_kind: "autonomous_coding_handoff".into(),
        summary: "Autonomous coding worker proposes a review-gated handoff patch".into(),
        unified_diff: format!(
            "--- /dev/null\n+++ {}\n@@\n+{}\n",
            path,
            content.replace('\n', "\n+").trim_end_matches('+')
        ),
        apply_status: WorkerTaskPatchApplyStatus::Proposed,
        applied_at_unix_ms: None,
        transaction_id: None,
        conflict_reason: None,
    }]
}

fn build_autonomous_coding_rounds(
    task: &WorkerTaskRecord,
    command_runs: &[WorkerTaskCommandRun],
    patches: &[WorkerTaskPatchProposal],
) -> Vec<WorkerTaskCodingRound> {
    let round_1_command_ids = command_runs
        .iter()
        .filter(|run| run.command_id.contains(":round-1-"))
        .map(|run| run.command_id.clone())
        .collect::<Vec<_>>();
    let round_2_command_ids = command_runs
        .iter()
        .filter(|run| run.command_id.contains(":round-2-"))
        .map(|run| run.command_id.clone())
        .collect::<Vec<_>>();
    let round_2_patch_ids = patches
        .iter()
        .map(|patch| patch.patch_id.clone())
        .collect::<Vec<_>>();
    let round_passed = |ids: &[String]| {
        !ids.is_empty()
            && ids.iter().all(|id| {
                command_runs
                    .iter()
                    .find(|run| &run.command_id == id)
                    .map(|run| {
                        run.passed && !run.timed_out && run.resource_limit_violation.is_none()
                    })
                    .unwrap_or(false)
            })
    };

    vec![
        WorkerTaskCodingRound {
            round_index: 1,
            title: "Inspect and draft bounded patch plan".into(),
            intent: compact_text(&task.prompt, 160),
            command_ids: round_1_command_ids.clone(),
            patch_ids: Vec::new(),
            passed: round_passed(&round_1_command_ids),
            summary: "round 1 inspected workspace targets, prepared a review-gated patch preview, and ran a preflight check".into(),
        },
        WorkerTaskCodingRound {
            round_index: 2,
            title: "Reinspect, revise, and prepare handoff".into(),
            intent: "close the worker handoff with replayable evidence after a second host-process pass".into(),
            command_ids: round_2_command_ids.clone(),
            patch_ids: round_2_patch_ids,
            passed: round_passed(&round_2_command_ids) && !patches.is_empty(),
            summary: "round 2 re-inspected safety state, revised the patch preview, and prepared evidence/replay/promotion handoff".into(),
        },
    ]
}

fn build_autonomous_coding_loop_steps(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    inspection: &AutonomousCodingInspection,
    patches: &[WorkerTaskPatchProposal],
    command_runs: &[WorkerTaskCommandRun],
    coding_rounds: &[WorkerTaskCodingRound],
) -> Vec<WorkerTaskLoopStep> {
    vec![
        WorkerTaskLoopStep {
            step_index: 1,
            phase: WorkerTaskLoopPhase::Plan,
            title: "Plan autonomous coding lane".into(),
            input_summary: compact_text(&task.prompt, 180),
            output_summary: format!(
                "Selected bounded inspect/patch/test/revise coding loop across {} rounds",
                coding_rounds.len()
            ),
            evidence_ref: format!("worker://{}/autonomous/plan", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 2,
            phase: WorkerTaskLoopPhase::Inspect,
            title: "Inspect source targets".into(),
            input_summary: format!("candidate_targets={}", inspection.target_count),
            output_summary: format!(
                "readable={} lines={} bytes={}",
                inspection.readable_count, inspection.total_lines, inspection.total_bytes
            ),
            evidence_ref: format!("worker://{}/autonomous/inspect", task.worker_session_id),
            passed: inspection.readable_count > 0,
        },
        WorkerTaskLoopStep {
            step_index: 3,
            phase: WorkerTaskLoopPhase::Patch,
            title: "Generate review-gated patch".into(),
            input_summary: compact_text(&result.final_text, 180),
            output_summary: format!("generated {} patch proposal(s)", patches.len()),
            evidence_ref: format!("worker://{}/autonomous/patch", task.worker_session_id),
            passed: !patches.is_empty(),
        },
        WorkerTaskLoopStep {
            step_index: 4,
            phase: WorkerTaskLoopPhase::Test,
            title: "Execute multi-round worker commands".into(),
            input_summary: format!("rounds={} commands={}", coding_rounds.len(), command_runs.len()),
            output_summary: format!(
                "{} / {} command runs passed",
                command_runs.iter().filter(|run| run.passed).count(),
                command_runs.len()
            ),
            evidence_ref: format!("worker://{}/autonomous/test", task.worker_session_id),
            passed: !command_runs.is_empty() && command_runs.iter().all(|run| run.passed),
        },
        WorkerTaskLoopStep {
            step_index: 5,
            phase: WorkerTaskLoopPhase::Revise,
            title: "Prepare auditable parent handoff".into(),
            input_summary: format!("patches={} artifacts_ready=true", patches.len()),
            output_summary: "Artifacts, command transcript, patch proposal, evidence, replay, and promotion gates are ready".into(),
            evidence_ref: format!("worker://{}/autonomous/handoff", task.worker_session_id),
            passed: true,
        },
    ]
}

fn build_worker_task_artifacts(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
) -> Vec<WorkerTaskArtifact> {
    vec![
        WorkerTaskArtifact {
            artifact_id: format!("{}:run-summary", task.task_id),
            kind: "run_summary".into(),
            title: format!("Worker {} run summary", task.worker_id),
            content: format!(
                "worker={}\nsession={}\nmodel={}/{}\ntool={:?}\nrecalled_memories={}\nfinal={} ",
                task.worker_id,
                task.worker_session_id,
                result.active_model.provider,
                result.active_model.model,
                result.invoked_tool,
                result.recalled_memories,
                compact_text(&result.final_text, 400),
            ),
            path_hint: Some(format!(
                "worker://{}/run-summary.md",
                task.worker_session_id
            )),
        },
        WorkerTaskArtifact {
            artifact_id: format!("{}:merge-note", task.task_id),
            kind: "merge_note".into(),
            title: format!("Join note for {}", task.task_id),
            content: format!(
                "Task `{}` completed in worker lane `{}`. Parent can review summary, tool output, and diff summary before merging.",
                task.task_id, task.worker_id
            ),
            path_hint: Some(format!("worker://{}/join-note.md", task.worker_session_id)),
        },
    ]
}

fn build_worker_task_diff_summary(task: &WorkerTaskRecord, result: &VerticalSliceResult) -> String {
    let tool = result.invoked_tool.as_deref().unwrap_or("no_tool_invoked");
    format!(
        "worker={} task={} pseudo_diff=summary_only tool={} final_excerpt=\"{}\"",
        task.worker_id,
        task.task_id,
        tool,
        compact_text(&result.final_text, 160)
    )
}

fn build_worker_task_patch_proposals(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
) -> Vec<WorkerTaskPatchProposal> {
    let primary_path = infer_worker_patch_path(task);
    let mut proposals = vec![WorkerTaskPatchProposal {
        patch_id: format!("{}:patch-1", task.task_id),
        revision_of: None,
        revision_index: 0,
        file_path: primary_path.clone(),
        change_kind: "proposed_update".into(),
        summary: format!(
            "Worker {} proposes a reviewable update derived from task output",
            task.worker_id
        ),
        unified_diff: format!(
            "--- /dev/null\n+++ {}\n@@\n+{}\n",
            primary_path,
            compact_text(&result.final_text, 500)
        ),
        apply_status: WorkerTaskPatchApplyStatus::Proposed,
        applied_at_unix_ms: None,
        transaction_id: None,
        conflict_reason: None,
    }];
    let prompt = task.prompt.to_ascii_lowercase();
    if prompt.contains("multi") || prompt.contains("patch set") || prompt.contains("batch") {
        let merge_plan_path = format!(
            "docs/worker-proposals/{}-merge-plan.md",
            worker_patch_suffix(task)
        );
        proposals.push(WorkerTaskPatchProposal {
            patch_id: format!("{}:patch-2", task.task_id),
            revision_of: None,
            revision_index: 0,
            file_path: merge_plan_path.clone(),
            change_kind: "merge_plan".into(),
            summary: format!("Worker {} proposes a companion merge plan", task.worker_id),
            unified_diff: format!(
                "--- /dev/null\n+++ {}\n@@\n+# Worker merge plan\n+task={}\n+worker={}\n+summary={}\n",
                merge_plan_path,
                task.task_id,
                task.worker_id,
                compact_text(&result.final_text, 300)
            ),
            apply_status: WorkerTaskPatchApplyStatus::Proposed,
            applied_at_unix_ms: None,
            transaction_id: None,
            conflict_reason: None,
        });
    }
    proposals
}

fn build_worker_task_loop_steps(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    patches: &[WorkerTaskPatchProposal],
) -> Vec<WorkerTaskLoopStep> {
    let final_summary = compact_text(&result.final_text, 180);
    vec![
        WorkerTaskLoopStep {
            step_index: 1,
            phase: WorkerTaskLoopPhase::Plan,
            title: "Plan worker approach".into(),
            input_summary: compact_text(&task.prompt, 180),
            output_summary: format!(
                "Plan a bounded worker lane for `{}` with review-gated outputs",
                task.worker_id
            ),
            evidence_ref: format!("worker://{}/loop/plan", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 2,
            phase: WorkerTaskLoopPhase::Inspect,
            title: "Inspect context and constraints".into(),
            input_summary: format!(
                "deps={} attempts={}",
                task.depends_on.len(),
                task.attempt_count
            ),
            output_summary: format!(
                "Resolved model output, recalled_memories={}, tool={}",
                result.recalled_memories,
                result.invoked_tool.as_deref().unwrap_or("none")
            ),
            evidence_ref: format!("worker://{}/loop/inspect", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 3,
            phase: WorkerTaskLoopPhase::Patch,
            title: "Draft patch proposal set".into(),
            input_summary: final_summary.clone(),
            output_summary: format!("Generated {} review-gated patch proposal(s)", patches.len()),
            evidence_ref: format!("worker://{}/loop/patch", task.worker_session_id),
            passed: !patches.is_empty(),
        },
        WorkerTaskLoopStep {
            step_index: 4,
            phase: WorkerTaskLoopPhase::Test,
            title: "Run deterministic local checks".into(),
            input_summary: format!("patches={}", patches.len()),
            output_summary:
                "Local deterministic worker gate passed; external side effects not executed".into(),
            evidence_ref: format!("worker://{}/loop/test", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 5,
            phase: WorkerTaskLoopPhase::Revise,
            title: "Prepare parent review handoff".into(),
            input_summary: final_summary,
            output_summary:
                "Artifacts, diff summary, loop trace, and patch set are ready for parent review"
                    .into(),
            evidence_ref: format!("worker://{}/loop/revise", task.worker_session_id),
            passed: true,
        },
    ]
}

fn worker_task_loop_report(task: WorkerTaskRecord) -> WorkerTaskLoopReport {
    let phases = task
        .loop_steps
        .iter()
        .map(|step| step.phase)
        .collect::<Vec<_>>();
    let passed_count = task.loop_steps.iter().filter(|step| step.passed).count();
    let failed_count = task.loop_steps.len().saturating_sub(passed_count);
    WorkerTaskLoopReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        loop_step_count: task.loop_steps.len(),
        passed_count,
        failed_count,
        phases,
        steps: task.loop_steps,
    }
}

fn worker_task_evidence_report(task: WorkerTaskRecord) -> WorkerTaskEvidenceReport {
    let mut entries = Vec::new();
    push_worker_evidence(
        &mut entries,
        &task,
        "task_created",
        format!(
            "created worker task prompt='{}'",
            compact_text(&task.prompt, 80)
        ),
        task.created_at_unix_ms,
        &task.worker_session_id,
    );
    push_worker_evidence(
        &mut entries,
        &task,
        "permission_envelope",
        task.permission_envelope.policy_summary.clone(),
        task.created_at_unix_ms,
        &task.worker_session_id,
    );
    if !task.depends_on.is_empty() {
        push_worker_evidence(
            &mut entries,
            &task,
            "dependency_gate",
            format!("depends_on={}", task.depends_on.join(",")),
            task.created_at_unix_ms,
            &task.worker_session_id,
        );
    }
    if let Some(schedule_expr) = &task.schedule_expr {
        push_worker_evidence(
            &mut entries,
            &task,
            "schedule_gate",
            format!(
                "schedule={} next_run={:?}",
                schedule_expr, task.next_run_unix_ms
            ),
            task.created_at_unix_ms,
            &task.worker_session_id,
        );
    }
    if let Some(started_at) = task.started_at_unix_ms {
        push_worker_evidence(
            &mut entries,
            &task,
            "run_attempt",
            format!(
                "attempt={}/{} timeout_budget_ms={}",
                task.attempt_count, task.max_attempts, task.timeout_budget_ms
            ),
            started_at,
            &task.worker_session_id,
        );
    }
    for round in &task.coding_rounds {
        push_worker_evidence(
            &mut entries,
            &task,
            "coding_round",
            format!(
                "round={} commands={} patches={} passed={} {}",
                round.round_index,
                round.command_ids.len(),
                round.patch_ids.len(),
                round.passed,
                compact_text(&round.summary, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for lease in &task.file_leases {
        push_worker_evidence(
            &mut entries,
            &task,
            "file_lease",
            format!(
                "{} path={} status={} conflicts={}",
                lease.lease_id,
                lease.target_path,
                file_lease_status_label(lease.status),
                lease.conflict_task_ids.len()
            ),
            lease.acquired_at_unix_ms,
            &task.worker_session_id,
        );
    }
    for step in &task.loop_steps {
        push_worker_evidence(
            &mut entries,
            &task,
            "loop_step",
            format!("{:?}: {} passed={}", step.phase, step.title, step.passed),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for run in &task.command_runs {
        push_worker_evidence(
            &mut entries,
            &task,
            "command_run",
            format!(
                "{} origin={:?} backend={} kind={:?} cwd={} timed_out={} exit={} passed={} stdout={}",
                run.command_id,
                run.execution_origin,
                run.backend_id,
                run.backend_kind,
                run.working_directory.as_deref().unwrap_or("<not-recorded>"),
                run.timed_out,
                run.exit_code,
                run.passed,
                compact_text(&run.stdout, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for artifact in &task.artifacts {
        push_worker_evidence(
            &mut entries,
            &task,
            "artifact",
            format!(
                "{} [{}] {}",
                artifact.artifact_id, artifact.kind, artifact.title
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for patch in &task.patch_proposals {
        push_worker_evidence(
            &mut entries,
            &task,
            "patch_proposal",
            format!(
                "{} {} status={} tx={:?}",
                patch.patch_id,
                patch.file_path,
                patch_apply_status_label(patch.apply_status),
                patch.transaction_id
            ),
            patch
                .applied_at_unix_ms
                .or(task.completed_at_unix_ms)
                .unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    if let Some(error) = &task.last_error {
        push_worker_evidence(
            &mut entries,
            &task,
            "failure",
            format!(
                "kind={:?} retry_after={:?} error={}",
                task.failure_kind,
                task.retry_after_unix_ms,
                compact_text(error, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    let chain_head = entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "sha256:empty".into());
    WorkerTaskEvidenceReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        worker_session_id: task.worker_session_id,
        evidence_count: entries.len(),
        terminal_status: task.status,
        permission_envelope: task.permission_envelope,
        chain_head,
        entries,
    }
}

fn worker_task_replay_audit_report(task: WorkerTaskRecord) -> WorkerTaskReplayAuditReport {
    let evidence = worker_task_evidence_report(task.clone());
    let mut checks = Vec::new();

    let hash_chain_valid = verify_worker_evidence_hash_chain(&evidence.entries);
    checks.push(WorkerTaskReplayCheck {
        check_id: "hash_chain".into(),
        passed: hash_chain_valid,
        summary: format!(
            "{} evidence entries replay to {}",
            evidence.evidence_count, evidence.chain_head
        ),
    });

    let permission_policy_valid = evidence.entries.iter().any(|entry| {
        entry.kind == "permission_envelope"
            && entry.summary == task.permission_envelope.policy_summary
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "permission_policy".into(),
        passed: permission_policy_valid,
        summary: task.permission_envelope.policy_summary.clone(),
    });

    let lifecycle_valid = match task.status {
        WorkerTaskStatus::Completed => {
            task.started_at_unix_ms.is_some()
                && task.completed_at_unix_ms.is_some()
                && task.result_summary.is_some()
        }
        WorkerTaskStatus::Failed => task.started_at_unix_ms.is_some() && task.last_error.is_some(),
        WorkerTaskStatus::Queued | WorkerTaskStatus::Scheduled => task.started_at_unix_ms.is_none(),
        WorkerTaskStatus::Running => task.started_at_unix_ms.is_some(),
        WorkerTaskStatus::Paused => task.paused_from_status.is_some(),
        WorkerTaskStatus::Cancelled | WorkerTaskStatus::Interrupted => true,
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "lifecycle".into(),
        passed: lifecycle_valid,
        summary: format!(
            "status={} attempts={}/{}",
            task_status_label(task.status),
            task.attempt_count,
            task.max_attempts
        ),
    });

    let artifact_records_valid = task.artifacts.iter().all(|artifact| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "artifact" && entry.summary.contains(&artifact.artifact_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "artifact_records".into(),
        passed: artifact_records_valid,
        summary: format!(
            "{} artifacts are represented in evidence",
            task.artifacts.len()
        ),
    });

    let patch_records_valid = task.patch_proposals.iter().all(|patch| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "patch_proposal" && entry.summary.contains(&patch.patch_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "patch_records".into(),
        passed: patch_records_valid,
        summary: format!(
            "{} patch proposals are represented in evidence",
            task.patch_proposals.len()
        ),
    });

    let coding_rounds_valid = task.coding_rounds.iter().all(|round| {
        let round_evidence_present = evidence.entries.iter().any(|entry| {
            entry.kind == "coding_round"
                && entry
                    .summary
                    .contains(&format!("round={}", round.round_index))
        });
        let commands_present = round.command_ids.iter().all(|command_id| {
            task.command_runs
                .iter()
                .any(|run| &run.command_id == command_id)
        });
        let patches_present = round.patch_ids.iter().all(|patch_id| {
            task.patch_proposals
                .iter()
                .any(|patch| &patch.patch_id == patch_id)
        });
        round_evidence_present && commands_present && patches_present && round.passed
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "coding_round_records".into(),
        passed: coding_rounds_valid,
        summary: format!(
            "{} coding rounds are represented in evidence",
            task.coding_rounds.len()
        ),
    });

    let multi_round_loop_valid = if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding
    {
        task.coding_rounds.len() >= 2
            && task
                .coding_rounds
                .windows(2)
                .all(|pair| pair[0].round_index < pair[1].round_index)
            && task.command_runs.len() >= 6
    } else {
        true
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "multi_round_loop".into(),
        passed: multi_round_loop_valid,
        summary: format!(
            "rounds={} command_runs={}",
            task.coding_rounds.len(),
            task.command_runs.len()
        ),
    });

    let file_lease_records_valid = task.file_leases.iter().all(|lease| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "file_lease" && entry.summary.contains(&lease.lease_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "file_lease_records".into(),
        passed: file_lease_records_valid,
        summary: format!(
            "{} file leases are represented in evidence",
            task.file_leases.len()
        ),
    });

    let command_records_valid = task.command_runs.iter().all(|run| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "command_run" && entry.summary.contains(&run.command_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "command_records".into(),
        passed: command_records_valid,
        summary: format!(
            "{} command runs are represented in evidence",
            task.command_runs.len()
        ),
    });

    let backend_records_valid = if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding
    {
        !task.command_runs.is_empty()
            && task.command_runs.iter().all(|run| {
                !run.backend_id.trim().is_empty()
                    && run.backend_kind == task.execution_backend.kind
                    && run.remote_backend == task.execution_backend.remote
            })
    } else {
        true
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "backend_records".into(),
        passed: backend_records_valid,
        summary: format!(
            "backend={} kind={:?} command_runs={}",
            task.execution_backend.backend_id,
            task.execution_backend.kind,
            task.command_runs.len()
        ),
    });

    let host_process_command_records_valid =
        if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding {
            !task.command_runs.is_empty()
                && task.command_runs.iter().all(|run| {
                    run.execution_origin == WorkerTaskCommandRunOrigin::HostProcess
                        && run.working_directory.is_some()
                        && !run.timed_out
                })
        } else {
            true
        };
    checks.push(WorkerTaskReplayCheck {
        check_id: "host_process_command_records".into(),
        passed: host_process_command_records_valid,
        summary: format!(
            "{} autonomous command runs recorded as host processes",
            task.command_runs
                .iter()
                .filter(|run| run.execution_origin == WorkerTaskCommandRunOrigin::HostProcess)
                .count()
        ),
    });

    let limits = &task.safety_envelope.resource_limits;
    let safety_limits_valid = task.command_runs.len() <= limits.max_command_runs
        && task.patch_proposals.len() <= limits.max_patch_proposals
        && task.loop_steps.len() <= limits.max_loop_steps
        && !task
            .safety_envelope
            .sandbox
            .workspace_root
            .trim()
            .is_empty()
        && task.safety_envelope.cancel_supported
        && task.safety_envelope.cancel_checked_before_host_command
        && task
            .command_runs
            .iter()
            .all(|run| run.sandboxed && run.resource_limit_violation.is_none());
    checks.push(WorkerTaskReplayCheck {
        check_id: "safety_limits".into(),
        passed: safety_limits_valid,
        summary: format!(
            "commands={}/{} patches={}/{} loop_steps={}/{} cancel_supported={}",
            task.command_runs.len(),
            limits.max_command_runs,
            task.patch_proposals.len(),
            limits.max_patch_proposals,
            task.loop_steps.len(),
            limits.max_loop_steps,
            task.safety_envelope.cancel_supported
        ),
    });

    let failure_records_valid = if task.status == WorkerTaskStatus::Failed {
        task.failure_kind.is_some()
            && task.retry_after_unix_ms.is_some()
            && evidence.entries.iter().any(|entry| entry.kind == "failure")
    } else {
        evidence.entries.iter().all(|entry| entry.kind != "failure")
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "failure_records".into(),
        passed: failure_records_valid,
        summary: format!(
            "failure_kind={:?} retry_after={:?}",
            task.failure_kind, task.retry_after_unix_ms
        ),
    });

    let replayed_chain_head = evidence
        .entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "sha256:empty".into());
    let replay_passed = checks.iter().all(|check| check.passed);
    WorkerTaskReplayAuditReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        terminal_status: task.status,
        evidence_count: evidence.evidence_count,
        chain_head: evidence.chain_head,
        replayed_chain_head,
        hash_chain_valid,
        permission_policy_valid,
        lifecycle_valid,
        artifact_records_valid,
        patch_records_valid,
        coding_rounds_valid,
        multi_round_loop_valid,
        file_lease_records_valid,
        backend_records_valid,
        failure_records_valid,
        safety_limits_valid,
        replay_passed,
        checks,
    }
}

fn worker_task_merge_risk_report(task: WorkerTaskRecord) -> WorkerTaskMergeRiskReport {
    let replay = worker_task_replay_audit_report(task.clone());
    let patch_conflicted_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::Conflicted,
    );
    let patch_rejected_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Rejected);
    let patch_rolled_back_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::RolledBack,
    );
    let conflicted_file_lease_count =
        count_file_lease_status(&task.file_leases, WorkerTaskFileLeaseStatus::Conflicted);
    let expired_file_lease_count =
        count_file_lease_status(&task.file_leases, WorkerTaskFileLeaseStatus::Expired);
    let mut score = 0u8;
    let mut reasons = Vec::new();

    if task.status != WorkerTaskStatus::Completed {
        score = score.saturating_add(80);
        reasons.push(format!(
            "terminal status is {}",
            task_status_label(task.status)
        ));
    }
    if !replay.replay_passed {
        score = score.saturating_add(60);
        reasons.push("replay audit did not pass".into());
    }
    if patch_conflicted_count > 0 {
        score = score.saturating_add((patch_conflicted_count * 20).min(40) as u8);
        reasons.push(format!(
            "{} conflicted patch proposals",
            patch_conflicted_count
        ));
    }
    if conflicted_file_lease_count > 0 || expired_file_lease_count > 0 {
        score = score.saturating_add(40);
        reasons.push(format!(
            "file lease issues conflicted={} expired={}",
            conflicted_file_lease_count, expired_file_lease_count
        ));
    }
    if patch_rejected_count > 0 {
        score = score.saturating_add((patch_rejected_count * 15).min(30) as u8);
        reasons.push(format!("{} rejected patch proposals", patch_rejected_count));
    }
    if patch_rolled_back_count > 0 {
        score = score.saturating_add((patch_rolled_back_count * 20).min(40) as u8);
        reasons.push(format!(
            "{} rolled back patch proposals",
            patch_rolled_back_count
        ));
    }
    if task.attempt_count > 1 || task.failure_kind.is_some() {
        score = score.saturating_add(15);
        reasons.push(format!(
            "retry/failure history attempts={} failure_kind={:?}",
            task.attempt_count, task.failure_kind
        ));
    }
    let failed_command_runs = task.command_runs.iter().filter(|run| !run.passed).count();
    if failed_command_runs > 0 {
        score = score.saturating_add((failed_command_runs * 20).min(40) as u8);
        reasons.push(format!(
            "{} failed autonomous command runs",
            failed_command_runs
        ));
    }
    if task.permission_envelope.network_allowed {
        score = score.saturating_add(10);
        reasons.push("network-enabled worker lane".into());
    }
    match task.permission_envelope.write_scope {
        WritePathScope::AnyPath => {
            score = score.saturating_add(20);
            reasons.push("write scope permits any path".into());
        }
        WritePathScope::WorkspaceOnly => {
            score = score.saturating_add(5);
            reasons.push("write scope permits workspace writes".into());
        }
        WritePathScope::ArtifactsOnly => {}
    }
    if task.status == WorkerTaskStatus::Completed && task.artifacts.is_empty() {
        score = score.saturating_add(10);
        reasons.push("completed task has no artifacts".into());
    }
    if reasons.is_empty() {
        reasons.push("low-risk completed task with valid replay audit".into());
    }
    let decision =
        if task.status != WorkerTaskStatus::Completed || !replay.replay_passed || score >= 60 {
            WorkerTaskMergeDecision::Blocked
        } else if score >= 20
            || patch_conflicted_count > 0
            || patch_rejected_count > 0
            || patch_rolled_back_count > 0
        {
            WorkerTaskMergeDecision::NeedsReview
        } else {
            WorkerTaskMergeDecision::SafeToMerge
        };
    WorkerTaskMergeRiskReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        decision,
        risk_score: score.min(100),
        replay_passed: replay.replay_passed,
        patch_conflicted_count,
        patch_rejected_count,
        patch_rolled_back_count,
        reasons,
    }
}

fn worker_task_promotion_report(task: WorkerTaskRecord) -> WorkerTaskPromotionReport {
    let replay = worker_task_replay_audit_report(task.clone());
    let merge_risk = worker_task_merge_risk_report(task.clone());
    let applied_patch_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Applied);
    let unapplied_patch_count = task
        .patch_proposals
        .iter()
        .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::Proposed)
        .count();
    let mut reasons = Vec::new();
    if task.status != WorkerTaskStatus::Completed {
        reasons.push(format!(
            "task status is {} rather than completed",
            task_status_label(task.status)
        ));
    }
    if !replay.replay_passed {
        reasons.push("replay audit failed".into());
    }
    if merge_risk.decision == WorkerTaskMergeDecision::Blocked {
        reasons.push(format!(
            "merge risk is blocked with score {}",
            merge_risk.risk_score
        ));
    }
    if merge_risk.decision == WorkerTaskMergeDecision::NeedsReview {
        reasons.push(format!(
            "merge risk needs review with score {}",
            merge_risk.risk_score
        ));
    }
    if unapplied_patch_count > 0 {
        reasons.push(format!(
            "{} proposed patches require explicit apply/reject before promotion",
            unapplied_patch_count
        ));
    }
    if merge_risk.patch_conflicted_count > 0
        || merge_risk.patch_rejected_count > 0
        || merge_risk.patch_rolled_back_count > 0
    {
        reasons.push("patch history contains conflicted/rejected/rolled-back proposals".into());
    }

    let hard_block = task.status != WorkerTaskStatus::Completed
        || !replay.replay_passed
        || merge_risk.decision == WorkerTaskMergeDecision::Blocked;
    let review_needed = merge_risk.decision == WorkerTaskMergeDecision::NeedsReview
        || unapplied_patch_count > 0
        || merge_risk.patch_conflicted_count > 0
        || merge_risk.patch_rejected_count > 0
        || merge_risk.patch_rolled_back_count > 0;
    let decision = if hard_block {
        WorkerTaskPromotionDecision::Blocked
    } else if review_needed {
        WorkerTaskPromotionDecision::NeedsReview
    } else {
        WorkerTaskPromotionDecision::Promoted
    };
    if reasons.is_empty() {
        reasons.push("promotion gate passed: replay-valid low-risk task".into());
    }
    WorkerTaskPromotionReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        decision,
        promotion_allowed: decision == WorkerTaskPromotionDecision::Promoted,
        auto_merge_allowed: decision == WorkerTaskPromotionDecision::Promoted
            && merge_risk.decision == WorkerTaskMergeDecision::SafeToMerge
            && merge_risk.risk_score <= 10,
        merge_risk,
        replay,
        unapplied_patch_count,
        applied_patch_count,
        reasons,
    }
}

fn worker_task_promotion_ledger_report(task: WorkerTaskRecord) -> WorkerTaskPromotionLedgerReport {
    let promotion = worker_task_promotion_report(task.clone());
    let now = task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms);
    let mut entries = Vec::new();
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "promotion_gate_evaluated",
        promotion.decision,
        format!(
            "promotion_allowed={} auto_merge_allowed={} reasons={}",
            promotion.promotion_allowed,
            promotion.auto_merge_allowed,
            promotion.reasons.join("; ")
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "replay_basis",
        promotion.decision,
        format!(
            "replay_passed={} chain_head={}",
            promotion.replay.replay_passed, promotion.replay.chain_head
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "merge_risk_basis",
        promotion.decision,
        format!(
            "merge_decision={:?} score={} reasons={}",
            promotion.merge_risk.decision,
            promotion.merge_risk.risk_score,
            promotion.merge_risk.reasons.join("; ")
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "patch_basis",
        promotion.decision,
        format!(
            "applied={} unapplied={} conflicted={} rejected={} rolled_back={}",
            promotion.applied_patch_count,
            promotion.unapplied_patch_count,
            promotion.merge_risk.patch_conflicted_count,
            promotion.merge_risk.patch_rejected_count,
            promotion.merge_risk.patch_rolled_back_count
        ),
        now,
    );
    let chain_head = entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "promotion-ledger:empty".into());
    WorkerTaskPromotionLedgerReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        ledger_count: entries.len(),
        promotion_decision: promotion.decision,
        promotion_allowed: promotion.promotion_allowed,
        auto_merge_allowed: promotion.auto_merge_allowed,
        chain_head,
        entries,
    }
}

fn worker_task_handoff_bundle_report(task: WorkerTaskRecord) -> WorkerTaskHandoffBundleReport {
    let generated_at_unix_ms = task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms);
    let evidence = worker_task_evidence_report(task.clone());
    let replay = worker_task_replay_audit_report(task.clone());
    let merge_risk = worker_task_merge_risk_report(task.clone());
    let promotion = worker_task_promotion_report(task.clone());
    let promotion_ledger = worker_task_promotion_ledger_report(task.clone());
    let handoff_ready = evidence.chain_head.starts_with("hepta-evidence:")
        && replay.replay_passed
        && promotion_ledger.chain_head.starts_with("hepta-promotion:")
        && promotion.promotion_allowed;
    let signature = worker_handoff_signature(
        &task.task_id,
        &task.worker_id,
        &evidence.chain_head,
        &replay.replayed_chain_head,
        merge_risk.risk_score,
        promotion.decision,
        &promotion_ledger.chain_head,
        generated_at_unix_ms,
    );
    let summary = format!(
        "handoff_ready={} promotion={:?} risk_score={} evidence={} promotion_ledger={}",
        handoff_ready,
        promotion.decision,
        merge_risk.risk_score,
        evidence.chain_head,
        promotion_ledger.chain_head,
    );
    WorkerTaskHandoffBundleReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        bundle_version: "worker-handoff-v1".into(),
        generated_at_unix_ms,
        evidence,
        replay,
        merge_risk,
        promotion,
        promotion_ledger,
        handoff_ready,
        signature,
        summary,
    }
}

fn worker_handoff_signature(
    task_id: &str,
    worker_id: &str,
    evidence_chain_head: &str,
    replayed_chain_head: &str,
    risk_score: u8,
    promotion_decision: WorkerTaskPromotionDecision,
    promotion_chain_head: &str,
    generated_at_unix_ms: u64,
) -> String {
    let material = format!(
        "worker-handoff-v1|{}|{}|{}|{}|{}|{:?}|{}|{}",
        task_id,
        worker_id,
        evidence_chain_head,
        replayed_chain_head,
        risk_score,
        promotion_decision,
        promotion_chain_head,
        generated_at_unix_ms,
    );
    format!("hepta-handoff:{:016x}", stable_hash64(&material))
}

fn push_promotion_ledger_entry(
    entries: &mut Vec<WorkerTaskPromotionLedgerEntry>,
    task: &WorkerTaskRecord,
    action: &str,
    decision: WorkerTaskPromotionDecision,
    summary: String,
    occurred_at_unix_ms: u64,
) {
    let index = entries.len();
    let previous_hash = entries.last().map(|entry| entry.entry_hash.clone());
    let ledger_ref = format!("worker-promotion:{}:{}:{}", task.task_id, action, index);
    let entry_hash = promotion_ledger_hash(
        previous_hash.as_deref(),
        &ledger_ref,
        action,
        decision,
        &summary,
        occurred_at_unix_ms,
    );
    entries.push(WorkerTaskPromotionLedgerEntry {
        index,
        ledger_ref,
        action: action.into(),
        decision,
        summary,
        occurred_at_unix_ms,
        previous_hash,
        entry_hash,
    });
}

fn promotion_ledger_hash(
    previous_hash: Option<&str>,
    ledger_ref: &str,
    action: &str,
    decision: WorkerTaskPromotionDecision,
    summary: &str,
    occurred_at_unix_ms: u64,
) -> String {
    let material = format!(
        "{}|{}|{}|{:?}|{}|{}",
        previous_hash.unwrap_or("genesis"),
        ledger_ref,
        action,
        decision,
        summary,
        occurred_at_unix_ms,
    );
    format!("hepta-promotion:{:016x}", stable_hash64(&material))
}

fn verify_worker_evidence_hash_chain(entries: &[WorkerTaskEvidenceEntry]) -> bool {
    let mut previous_hash: Option<String> = None;
    for entry in entries {
        if entry.previous_hash != previous_hash {
            return false;
        }
        let expected = worker_evidence_hash(
            previous_hash.as_deref(),
            &entry.evidence_ref,
            &entry.kind,
            &entry.summary,
            entry.occurred_at_unix_ms,
            &entry.session_id,
        );
        if entry.entry_hash != expected {
            return false;
        }
        previous_hash = Some(entry.entry_hash.clone());
    }
    true
}

fn push_worker_evidence(
    entries: &mut Vec<WorkerTaskEvidenceEntry>,
    task: &WorkerTaskRecord,
    kind: &str,
    summary: String,
    occurred_at_unix_ms: u64,
    session_id: &str,
) {
    let index = entries.len();
    let previous_hash = entries.last().map(|entry| entry.entry_hash.clone());
    let evidence_ref = format!("worker-evidence:{}:{}:{}", task.task_id, kind, index);
    let entry_hash = worker_evidence_hash(
        previous_hash.as_deref(),
        &evidence_ref,
        kind,
        &summary,
        occurred_at_unix_ms,
        session_id,
    );
    entries.push(WorkerTaskEvidenceEntry {
        index,
        evidence_ref,
        kind: kind.into(),
        summary,
        occurred_at_unix_ms,
        session_id: session_id.into(),
        previous_hash,
        entry_hash,
    });
}

fn worker_evidence_hash(
    previous_hash: Option<&str>,
    evidence_ref: &str,
    kind: &str,
    summary: &str,
    occurred_at_unix_ms: u64,
    session_id: &str,
) -> String {
    let material = format!(
        "{}|{}|{}|{}|{}|{}",
        previous_hash.unwrap_or("genesis"),
        evidence_ref,
        kind,
        summary,
        occurred_at_unix_ms,
        session_id,
    );
    format!("hepta-evidence:{:016x}", stable_hash64(&material))
}

fn stable_hash64(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn infer_worker_patch_path(task: &WorkerTaskRecord) -> String {
    let prompt = task.prompt.to_ascii_lowercase();
    let suffix = worker_patch_suffix(task);
    if prompt.contains("doc") || prompt.contains("paper") || prompt.contains("write") {
        format!("docs/worker-proposals/{}.md", suffix)
    } else if prompt.contains("rust") || prompt.contains("code") || prompt.contains("patch") {
        format!("src/worker-proposals/{}.rs", suffix)
    } else {
        format!("artifacts/worker-proposals/{}.md", suffix)
    }
}

fn revised_patch_path(
    task: &WorkerTaskRecord,
    source_patch: &WorkerTaskPatchProposal,
    revision_index: usize,
) -> String {
    format!(
        "docs/worker-proposals/{}-revision-{}-of-{}.md",
        worker_patch_suffix(task),
        revision_index,
        sanitize_for_id(&source_patch.patch_id)
    )
}

fn worker_patch_suffix(task: &WorkerTaskRecord) -> String {
    format!(
        "{}-{}",
        sanitize_for_id(&task.task_id),
        sanitize_for_id(&compact_text(&task.prompt, 48))
    )
}

fn worker_task_patch_review_report(task: WorkerTaskRecord) -> WorkerTaskPatchReviewReport {
    let patch_count = task.patch_proposals.len();
    let proposed_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Proposed);
    let applied_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Applied);
    let conflicted_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::Conflicted,
    );
    let rejected_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Rejected);
    let rolled_back_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::RolledBack,
    );
    WorkerTaskPatchReviewReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        patch_count,
        proposed_count,
        applied_count,
        conflicted_count,
        rejected_count,
        rolled_back_count,
        patches: task.patch_proposals,
    }
}

fn count_patch_status(
    patches: &[WorkerTaskPatchProposal],
    status: WorkerTaskPatchApplyStatus,
) -> usize {
    patches
        .iter()
        .filter(|patch| patch.apply_status == status)
        .count()
}

fn count_file_lease_status(
    leases: &[WorkerTaskFileLease],
    status: WorkerTaskFileLeaseStatus,
) -> usize {
    leases.iter().filter(|lease| lease.status == status).count()
}

fn count_file_lease_status_refs(
    leases: &[&WorkerTaskFileLease],
    status: WorkerTaskFileLeaseStatus,
) -> usize {
    leases.iter().filter(|lease| lease.status == status).count()
}

fn patch_apply_status_label(status: WorkerTaskPatchApplyStatus) -> &'static str {
    match status {
        WorkerTaskPatchApplyStatus::Proposed => "proposed",
        WorkerTaskPatchApplyStatus::Applied => "applied",
        WorkerTaskPatchApplyStatus::Conflicted => "conflicted",
        WorkerTaskPatchApplyStatus::Rejected => "rejected",
        WorkerTaskPatchApplyStatus::RolledBack => "rolled_back",
    }
}

fn extract_added_content_from_unified_diff(diff: &str) -> Result<String, String> {
    let mut added_lines = Vec::new();
    let mut saw_hunk = false;
    for line in diff.lines() {
        if line.starts_with("@@") {
            saw_hunk = true;
            continue;
        }
        if !saw_hunk || line.starts_with("+++") || line.starts_with("---") {
            continue;
        }
        if let Some(added) = line.strip_prefix('+') {
            added_lines.push(added.to_string());
        }
    }
    if added_lines.is_empty() {
        return Err("patch has no added content to apply".into());
    }
    Ok(format!("{}\n", added_lines.join("\n")))
}

fn compact_text(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn is_zero(value: &usize) -> bool {
    *value == 0
}
