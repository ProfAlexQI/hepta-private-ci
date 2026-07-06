use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub terminal_wrapper_count: usize,
    pub terminal_source_count: usize,
    pub canonical_wire_field_count: usize,
    pub terminal_required_field_count: usize,
    pub evidence_contract_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub terminal_wrappers: Vec<WorkGraphTerminalTaskResultWrapperPreview>,
    pub canonical_wire_fields: Vec<&'static str>,
    pub terminal_required_fields: Vec<&'static str>,
    pub evidence_contracts: Vec<WorkGraphTaskResultWrapperEvidenceContractPreview>,
    pub blockers: Vec<WorkGraphTerminalTaskResultWrapperBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_wrapper_fixture_preview: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub terminal_source_kind: &'static str,
    pub source_terminal_field: &'static str,
    pub emitted_event_contract_id: &'static str,
    pub task_result_node_kind: &'static str,
    pub replay_key_contract_id: &'static str,
    pub required_wire_fields: Vec<&'static str>,
    pub terminal_required_fields: Vec<&'static str>,
    pub evidence_contract_ids: Vec<&'static str>,
    pub canonical_status_mappings: Vec<WorkGraphTaskResultStatusMappingPreview>,
    pub wrapper_state: &'static str,
    pub redaction_policy: &'static str,
    pub attaches_runtime_adapter: bool,
    pub executes_wrapper: bool,
    pub persists_task_result: bool,
    pub enforces_task_result: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultStatusMappingPreview {
    pub source_status: &'static str,
    pub canonical_status: &'static str,
    pub terminal: bool,
    pub promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperEvidenceContractPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub evidence_ref_fields: Vec<&'static str>,
    pub verifier_ref_fields: Vec<&'static str>,
    pub redaction_policy: &'static str,
    pub stores_raw_payload: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_task_result_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperPreviewSideEffects {
    pub filesystem_written: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub wrapper_executed: bool,
    pub runtime_adapter_attached: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub readback_performed: bool,
    pub replay_executed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_wrapper_preview_report()
-> WorkGraphTerminalTaskResultWrapperPreviewReport {
    let terminal_wrappers = work_graph_terminal_task_result_wrappers();
    let canonical_wire_fields = work_graph_terminal_task_result_canonical_wire_fields();
    let terminal_required_fields = work_graph_terminal_task_result_required_terminal_fields();
    let evidence_contracts = work_graph_terminal_task_result_wrapper_evidence_contracts();
    let blockers = work_graph_terminal_task_result_wrapper_blockers();
    let required_prior_gates = work_graph_terminal_task_result_wrapper_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_preview_no_execution",
        terminal_wrapper_count: terminal_wrappers.len(),
        terminal_source_count: work_graph_terminal_task_result_wrapper_source_surface_ids().len(),
        canonical_wire_field_count: canonical_wire_fields.len(),
        terminal_required_field_count: terminal_required_fields.len(),
        evidence_contract_count: evidence_contracts.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        terminal_wrappers,
        canonical_wire_fields,
        terminal_required_fields,
        evidence_contracts,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_RECOMMENDED_NEXT_GATE,
        ready_for_wrapper_fixture_preview: true,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultWrapperPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrappers() -> Vec<WorkGraphTerminalTaskResultWrapperPreview>
{
    vec![
        wrapper(
            "multi_agent_thread_spawn_terminal_task_result_wrapper",
            "multi_agent_v2_thread_spawn",
            "multi_agent_spawn",
            "thread_spawn_edge.status",
            "agent_task",
            "multi_agent_spawn_projection_idempotency",
            vec!["thread_spawn_completion_evidence"],
            "hash spawn prompt and expose only parent/child thread ids, role id, agent path, and evidence refs",
        ),
        wrapper(
            "multi_agent_mailbox_wait_terminal_task_result_wrapper",
            "multi_agent_v2_mailbox_wait",
            "mailbox_delivery",
            "mailbox_wait.deliveryState",
            "agent_task",
            "multi_agent_mailbox_delivery_replay_key",
            vec!["mailbox_wait_delivery_evidence"],
            "hash mailbox payload and expose only mailbox seq, agent path, delivery state, and evidence refs",
        ),
        wrapper(
            "multi_agent_reducer_terminal_task_result_wrapper",
            "hepta_runtime_multi_agent_reducer",
            "multi_agent_reducer",
            "AgentRuntimeRunReport.reducer_passed",
            "agent_task",
            "multi_agent_reducer_task_result_replay_key",
            vec!["reducer_consensus_evidence"],
            "hash subagent outputs and expose only reducer strategy, decision, status, and evidence refs",
        ),
        wrapper(
            "agent_job_item_terminal_task_result_wrapper",
            "agent_jobs_batch_workers",
            "batch_agent_job_item",
            "AgentJobItem.status",
            "worker_task",
            "agent_job_result_projection_idempotency",
            vec!["agent_job_result_schema_evidence"],
            "hash worker JSON result and expose only schema ref, job id, item id, attempt, and evidence refs",
        ),
        wrapper(
            "worker_task_terminal_task_result_wrapper",
            "hepta_runtime_worker_tasks",
            "worker_task",
            "WorkerTaskRecord.status",
            "worker_task",
            "worker_task_projection_idempotency",
            vec!["worker_task_artifact_gate_evidence"],
            "hash command output and expose only artifact refs, verifier refs, lane, attempt, and status",
        ),
        wrapper(
            "task_board_terminal_task_result_wrapper",
            "hepta_runtime_task_board",
            "task_board_worker_task",
            "TaskBoardRecord.status",
            "worker_task",
            "task_board_worker_task_replay_key",
            vec!["task_board_lease_readback_evidence"],
            "hash task board payload and expose only lease state, artifact hash, lane, and evidence refs",
        ),
        wrapper(
            "scheduler_run_terminal_task_result_wrapper",
            "hepta_runtime_scheduler_store",
            "scheduler_run",
            "SchedulerRunRecord.status",
            "scheduler_run",
            "scheduler_run_projection_idempotency",
            vec!["scheduler_admission_decision_evidence"],
            "hash scheduler decision inputs and expose only lease id, admission decision, status, and evidence refs",
        ),
        wrapper(
            "agent_harness_terminal_task_result_wrapper",
            "hepta_runtime_agent_harness",
            "agent_harness_handoff",
            "AgentHarnessRunRecord.status",
            "external_handoff",
            "agent_harness_handoff_projection_idempotency",
            vec!["agent_harness_handoff_evidence"],
            "hash harness payload and expose only handoff refs, artifact refs, verifier refs, and redaction state",
        ),
    ]
}

pub fn work_graph_terminal_task_result_canonical_wire_fields() -> Vec<&'static str> {
    vec![
        "taskId",
        "status",
        "summary",
        "artifacts",
        "evidence",
        "risks",
        "nextActions",
        "verifier",
        "reducer",
        "usage",
        "traceId",
    ]
}

pub fn work_graph_terminal_task_result_required_terminal_fields() -> Vec<&'static str> {
    vec![
        "taskId", "status", "summary", "evidence", "verifier", "traceId",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_evidence_contracts()
-> Vec<WorkGraphTaskResultWrapperEvidenceContractPreview> {
    vec![
        evidence(
            "thread_spawn_completion_evidence",
            "multi_agent_v2_thread_spawn",
            vec!["parentThreadId", "childThreadId", "agentPath", "roleId"],
            vec!["roleManifestGate", "spawnProjectionGate"],
        ),
        evidence(
            "mailbox_wait_delivery_evidence",
            "multi_agent_v2_mailbox_wait",
            vec![
                "mailboxSeq",
                "agentPath",
                "deliveryState",
                "timelineEventRef",
            ],
            vec!["mailboxReadbackProbe", "deliveryStateGate"],
        ),
        evidence(
            "reducer_consensus_evidence",
            "hepta_runtime_multi_agent_reducer",
            vec![
                "reducerStrategy",
                "decisionHash",
                "participantCount",
                "evidenceHash",
            ],
            vec!["reducerVerifier", "taskResultContractGate"],
        ),
        evidence(
            "agent_job_result_schema_evidence",
            "agent_jobs_batch_workers",
            vec!["jobId", "itemId", "attempt", "resultSchemaRef"],
            vec!["agentJobSchemaGate", "taskResultContractGate"],
        ),
        evidence(
            "worker_task_artifact_gate_evidence",
            "hepta_runtime_worker_tasks",
            vec!["workerTaskId", "attempt", "artifactHash", "gateReportHash"],
            vec!["workerTaskVerifier", "artifactRedactionGate"],
        ),
        evidence(
            "task_board_lease_readback_evidence",
            "hepta_runtime_task_board",
            vec!["workerTaskId", "lane", "leaseState", "artifactHash"],
            vec!["taskBoardLeaseGate", "idempotencyReadbackAdapterGate"],
        ),
        evidence(
            "scheduler_admission_decision_evidence",
            "hepta_runtime_scheduler_store",
            vec![
                "schedulerRunId",
                "leaseId",
                "admissionDecision",
                "decisionHash",
            ],
            vec!["schedulerAdmissionGate", "roleManifestGate"],
        ),
        evidence(
            "agent_harness_handoff_evidence",
            "hepta_runtime_agent_harness",
            vec![
                "harnessRunId",
                "handoffRef",
                "artifactHash",
                "redactionState",
            ],
            vec!["agentHarnessVerifier", "artifactRedactionGate"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_blockers()
-> Vec<WorkGraphTerminalTaskResultWrapperBlockerPreview> {
    vec![
        blocker(
            "wrapper_fixture_execution_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_source_surface_ids(),
            "run fixture-only wrapping checks before any runtime adapter executes these wrappers",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_source_surface_ids(),
            "keep TaskResult validation preview-only until fixtures prove every terminal source maps canonical fields",
        ),
        blocker(
            "append_only_store_still_disabled",
            "medium",
            work_graph_terminal_task_result_wrapper_source_surface_ids(),
            "do not persist TaskResult records until replay/readback fixtures are deterministic",
        ),
        blocker(
            "scheduler_admission_consumes_preview_only",
            "medium",
            vec![
                "hepta_runtime_scheduler_store",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
            ],
            "scheduler admission must keep reading preview contracts until TaskResult enforcement is explicitly enabled",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_unified_projection_audit_preview_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_append_only_event_intake_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_idempotency_readback_adapter_preview_gate",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_source_surface_ids() -> Vec<&'static str> {
    vec![
        "multi_agent_v2_thread_spawn",
        "multi_agent_v2_mailbox_wait",
        "hepta_runtime_multi_agent_reducer",
        "agent_jobs_batch_workers",
        "hepta_runtime_worker_tasks",
        "hepta_runtime_task_board",
        "hepta_runtime_scheduler_store",
        "hepta_runtime_agent_harness",
    ]
}

impl WorkGraphTerminalTaskResultWrapperPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_record_persisted: false,
            task_result_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            wrapper_executed: false,
            runtime_adapter_attached: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            readback_performed: false,
            replay_executed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn wrapper(
    id: &'static str,
    source_surface_id: &'static str,
    terminal_source_kind: &'static str,
    source_terminal_field: &'static str,
    task_result_node_kind: &'static str,
    replay_key_contract_id: &'static str,
    evidence_contract_ids: Vec<&'static str>,
    redaction_policy: &'static str,
) -> WorkGraphTerminalTaskResultWrapperPreview {
    WorkGraphTerminalTaskResultWrapperPreview {
        id,
        source_surface_id,
        terminal_source_kind,
        source_terminal_field,
        emitted_event_contract_id: "task_result_event_intake",
        task_result_node_kind,
        replay_key_contract_id,
        required_wire_fields: work_graph_terminal_task_result_canonical_wire_fields(),
        terminal_required_fields: work_graph_terminal_task_result_required_terminal_fields(),
        evidence_contract_ids,
        canonical_status_mappings: status_mappings(),
        wrapper_state: "preview_contract_defined_wrapper_execution_disabled",
        redaction_policy,
        attaches_runtime_adapter: false,
        executes_wrapper: false,
        persists_task_result: false,
        enforces_task_result: false,
        mutates_store: false,
    }
}

fn status_mappings() -> Vec<WorkGraphTaskResultStatusMappingPreview> {
    vec![
        status_mapping("success", "succeeded", true, true),
        status_mapping("ok", "succeeded", true, true),
        status_mapping("failed", "failed", true, true),
        status_mapping("error", "failed", true, true),
        status_mapping("cancelled", "cancelled", true, true),
        status_mapping("blocked", "blocked", true, false),
        status_mapping("superseded", "superseded", true, true),
    ]
}

fn evidence(
    id: &'static str,
    source_surface_id: &'static str,
    evidence_ref_fields: Vec<&'static str>,
    verifier_ref_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperEvidenceContractPreview {
    WorkGraphTaskResultWrapperEvidenceContractPreview {
        id,
        source_surface_id,
        evidence_ref_fields,
        verifier_ref_fields,
        redaction_policy: "store ids, hashes, schema refs, and verifier refs only",
        stores_raw_payload: false,
        performs_readback: false,
        mutates_store: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphTerminalTaskResultWrapperBlockerPreview {
    WorkGraphTerminalTaskResultWrapperBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_task_result_enforcement: true,
        recommended_fix,
    }
}

const fn status_mapping(
    source_status: &'static str,
    canonical_status: &'static str,
    terminal: bool,
    promotion_allowed: bool,
) -> WorkGraphTaskResultStatusMappingPreview {
    WorkGraphTaskResultStatusMappingPreview {
        source_status,
        canonical_status,
        terminal,
        promotion_allowed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_task_result_wrapper_covers_expected_sources() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();
        let source_ids = report
            .terminal_wrappers
            .iter()
            .map(|wrapper| wrapper.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.terminal_wrapper_count, 8);
        assert_eq!(report.terminal_source_count, 8);
        assert_eq!(
            source_ids,
            work_graph_terminal_task_result_wrapper_source_surface_ids()
        );
        assert!(report.terminal_wrappers.iter().all(|wrapper| {
            wrapper.emitted_event_contract_id == "task_result_event_intake"
                && !wrapper.attaches_runtime_adapter
                && !wrapper.executes_wrapper
                && !wrapper.persists_task_result
                && !wrapper.enforces_task_result
                && !wrapper.mutates_store
        }));
    }

    #[test]
    fn terminal_task_result_wrapper_declares_canonical_fields() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();

        assert_eq!(report.canonical_wire_field_count, 11);
        assert_eq!(report.terminal_required_field_count, 6);
        assert_eq!(
            report.terminal_required_fields,
            [
                "taskId", "status", "summary", "evidence", "verifier", "traceId"
            ]
        );
        assert!(report.terminal_wrappers.iter().all(|wrapper| {
            wrapper.required_wire_fields == report.canonical_wire_fields
                && wrapper.terminal_required_fields == report.terminal_required_fields
        }));
    }

    #[test]
    fn terminal_task_result_wrapper_maps_terminal_statuses() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();
        let canonical_statuses = report.terminal_wrappers[0]
            .canonical_status_mappings
            .iter()
            .map(|mapping| mapping.canonical_status)
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(
            canonical_statuses,
            ["blocked", "cancelled", "failed", "succeeded", "superseded"]
                .into_iter()
                .collect()
        );
        assert!(report.terminal_wrappers.iter().all(|wrapper| {
            wrapper
                .canonical_status_mappings
                .iter()
                .all(|mapping| mapping.terminal)
        }));
    }

    #[test]
    fn terminal_task_result_wrapper_evidence_is_redacted_contract_only() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();
        let evidence_sources = report
            .evidence_contracts
            .iter()
            .map(|contract| contract.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.evidence_contract_count, 8);
        assert_eq!(
            evidence_sources,
            work_graph_terminal_task_result_wrapper_source_surface_ids()
        );
        assert!(report.evidence_contracts.iter().all(|contract| {
            !contract.stores_raw_payload && !contract.performs_readback && !contract.mutates_store
        }));
    }

    #[test]
    fn terminal_task_result_wrapper_blocks_enforcement_and_live_execution() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(report.ready_for_wrapper_fixture_preview);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperPreviewSideEffects::none()
        );
    }

    #[test]
    fn terminal_task_result_wrapper_requires_adapter_prior() {
        let report = hepta_work_graph_terminal_task_result_wrapper_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_unified_projection_audit_preview_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_append_only_event_intake_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_idempotency_readback_adapter_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_RECOMMENDED_NEXT_GATE
        );
    }
}
