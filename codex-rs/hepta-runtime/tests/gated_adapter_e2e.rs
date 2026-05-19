use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use hepta_runtime::{
    ModelProviderInvocationHandoffInput, ModelProviderLocalInvocationInput, ModelProviderRouter,
    ModelProviderStatus, ProcessStartExecutionInput, ProcessStartHandoffInput, ProcessSupervisor,
    ReadbackEvidenceLedger, SchedulerJobInput, SchedulerScheduleKind, SchedulerStore,
    SchedulerWakeHandoffInput, SchedulerWakeMaterializationInput, SupervisedProcessStatus,
    live_adapter_activation_discipline_sample,
};

fn temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("hepta-gated-adapter-e2e-{label}-{nanos}.json"))
}

fn scheduler_job(idempotency_key: &str) -> SchedulerJobInput {
    SchedulerJobInput {
        name: "installed gated adapter e2e".into(),
        schedule_kind: SchedulerScheduleKind::Cron,
        schedule_expr: "*/5 * * * *".into(),
        timezone: Some("Asia/Shanghai".into()),
        payload_kind: "agentTurn".into(),
        payload_preview: "local e2e wake handoff without external delivery".into(),
        idempotency_key: idempotency_key.into(),
        explicit_timeout_seconds: Some(0),
        session_bound_heartbeat_route: Some("session:gated-e2e".into()),
        next_due_unix_ms: Some(0),
    }
}

#[test]
fn confirmed_gated_adapters_execute_local_e2e_with_readback_without_external_effects() {
    let ledger_path = temp_path("ledger");
    let process_path = temp_path("process");
    let scheduler_path = temp_path("scheduler");
    let model_path = temp_path("model");

    let ledger = ReadbackEvidenceLedger::new(&ledger_path);

    let activation_reports = live_adapter_activation_discipline_sample().unwrap();
    assert_eq!(activation_reports.len(), 4);
    assert!(activation_reports.iter().all(|report| {
        report.discipline_ready
            && report.dry_run
            && !report.activation_permitted
            && !report.provider_invoked_by_gate
            && !report.channel_delivery_performed_by_gate
            && !report.node_invoked_by_gate
            && !report.process_spawned_by_gate
    }));

    let process = ProcessSupervisor::new(&process_path);
    let cwd = std::env::temp_dir();
    let planned_process = process
        .plan_process(
            "/bin/echo hepta-gated-adapter-e2e",
            &cwd.display().to_string(),
            "e2e-process-plan",
        )
        .unwrap();
    let process_handoff = process
        .gated_start_handoff(
            &ledger,
            &planned_process.process.process_id,
            ProcessStartHandoffInput {
                policy_decision: "allow-exec-start".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-process-handoff".into(),
            },
        )
        .unwrap();
    let process_execution = process
        .execute_start_handoff_once(
            &ledger,
            ProcessStartExecutionInput {
                handoff_id: process_handoff.handoff.handoff_id.clone(),
                policy_decision: "approved-local-exec".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-process-execution".into(),
                timeout_ms: 1_000,
            },
        )
        .unwrap();
    assert!(process_execution.native_process_spawned_by_adapter);
    assert!(process_execution.supervisor_mutated_by_adapter);
    assert_eq!(
        process_execution.process_status,
        SupervisedProcessStatus::Exited
    );
    assert_eq!(process_execution.execution.exit_code, 0);
    assert!(!process_execution.stdin_written_by_adapter);
    assert!(!process_execution.signal_sent_by_adapter);
    assert!(
        process_execution
            .execution
            .stdout_tail
            .iter()
            .any(|line| line.contains("hepta-gated-adapter-e2e"))
    );

    let scheduler = SchedulerStore::new(&scheduler_path);
    let scheduled = scheduler
        .schedule_job(scheduler_job("e2e-scheduler-job"))
        .unwrap();
    scheduler.mark_due(&scheduled.job.job_id).unwrap();
    let wake_handoff = scheduler
        .gated_wake_session_handoff(
            &ledger,
            &scheduled.job.job_id,
            SchedulerWakeHandoffInput {
                run_id: None,
                session_target: "session:gated-e2e".into(),
                wake_mode: "now".into(),
                payload_preview: "queue local wake only".into(),
                policy_decision: "allow-wake-session-target".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-wake-handoff".into(),
            },
        )
        .unwrap();
    let wake = scheduler
        .materialize_wake_from_handoff(
            &ledger,
            SchedulerWakeMaterializationInput {
                handoff_id: wake_handoff.handoff.handoff_id.clone(),
                policy_decision: "approved-local-wake-queue".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-wake-materialize".into(),
            },
        )
        .unwrap();
    assert!(wake.wake_enqueued_by_adapter);
    assert!(wake.scheduler_store_mutated_by_adapter);
    assert!(!wake.session_mutated_by_adapter);
    assert!(!wake.gateway_rpc_performed_by_adapter);
    assert_eq!(wake.wake.session_target, "session:gated-e2e");

    let router = ModelProviderRouter::new(&model_path);
    router
        .register_provider(
            "openai-codex",
            "gpt-5.5",
            "agent_text",
            ModelProviderStatus::Degraded,
            2,
            "fallback",
        )
        .unwrap();
    router
        .register_provider(
            "hepta-local",
            "hepta-fixture-small",
            "agent_text",
            ModelProviderStatus::Available,
            1,
            "local-fixture",
        )
        .unwrap();
    let model_handoff = router
        .gated_invocation_handoff(
            &ledger,
            ModelProviderInvocationHandoffInput {
                capability: "agent_text".into(),
                request_preview: "summarize e2e adapter state".into(),
                auth_readiness: "not_required".into(),
                policy_decision: "allow-provider-invocation".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-model-handoff".into(),
            },
        )
        .unwrap();
    let local_model = router
        .invoke_local_handoff(
            &ledger,
            ModelProviderLocalInvocationInput {
                handoff_id: model_handoff.handoff.handoff_id.clone(),
                policy_decision: "approved-local-provider".into(),
                operator_confirmed: true,
                idempotency_key: "e2e-model-local-invoke".into(),
            },
        )
        .unwrap();
    assert!(local_model.provider_invoked_by_adapter);
    assert!(local_model.usage_recorded_by_adapter);
    assert!(!local_model.auth_secret_read_by_adapter);
    assert_eq!(local_model.invocation.provider_id, "hepta-local");
    assert!(!local_model.invocation.response_preview.is_empty());

    let ledger_report = ledger.report(None).unwrap();
    let subject_kinds = ledger_report
        .ledger
        .entries
        .iter()
        .map(|entry| entry.subject_kind.as_str())
        .collect::<Vec<_>>();
    assert!(subject_kinds.contains(&"process_start_handoff"));
    assert!(subject_kinds.contains(&"process_start_execution"));
    assert!(subject_kinds.contains(&"scheduler_wake_handoff"));
    assert!(subject_kinds.contains(&"scheduler_wake_queue"));
    assert!(subject_kinds.contains(&"model_provider_invocation_handoff"));
    assert!(subject_kinds.contains(&"model_provider_local_invocation"));

    let _ = fs::remove_file(ledger_path);
    let _ = fs::remove_file(process_path);
    let _ = fs::remove_file(scheduler_path);
    let _ = fs::remove_file(model_path);
}
