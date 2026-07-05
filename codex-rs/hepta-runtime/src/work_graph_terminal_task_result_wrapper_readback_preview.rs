use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_readback_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub collection_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphTerminalTaskResultReadbackPlanPreview>,
    pub collection_assertions: Vec<WorkGraphTaskResultReadbackCollectionAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphTaskResultReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphTaskResultReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_drift_budget_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultReadbackPlanPreview {
    pub id: &'static str,
    pub fixture_id: &'static str,
    pub wrapper_id: &'static str,
    pub source_surface_id: &'static str,
    pub expected_task_result_collection_id: &'static str,
    pub expected_timeline_collection_id: &'static str,
    pub expected_evidence_contract_id: &'static str,
    pub required_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub redaction_policy: &'static str,
    pub performs_readback: bool,
    pub persists_drift: bool,
    pub mutates_store: bool,
    pub enforces_task_result: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultReadbackCollectionAssertionPreview {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub evidence_fields: Vec<&'static str>,
    pub blocks_wrapper_execution: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_wrapper_execution: bool,
    pub persists_drift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_readback_plan_ids: Vec<&'static str>,
    pub required_before_readback_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub fixture_executed: bool,
    pub wrapper_executed: bool,
    pub readback_performed: bool,
    pub drift_state_persisted: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub replay_executed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_wrapper_readback_preview_report()
-> WorkGraphTerminalTaskResultWrapperReadbackPreviewReport {
    let readback_plans = work_graph_terminal_task_result_wrapper_readback_plans();
    let collection_assertions =
        work_graph_terminal_task_result_wrapper_readback_collection_assertions();
    let drift_detectors = work_graph_terminal_task_result_wrapper_readback_drift_detectors();
    let blockers = work_graph_terminal_task_result_wrapper_readback_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_readback_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_readback_preview_no_execution",
        readback_plan_count: readback_plans.len(),
        collection_assertion_count: collection_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        collection_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_drift_budget_preview: true,
        ready_for_readback_execution: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultWrapperReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_readback_plans()
-> Vec<WorkGraphTerminalTaskResultReadbackPlanPreview> {
    vec![
        readback_plan(
            "readback_fixture_multi_agent_thread_spawn_success",
            "fixture_multi_agent_thread_spawn_success",
            "multi_agent_thread_spawn_terminal_task_result_wrapper",
            "multi_agent_v2_thread_spawn",
            "thread_spawn_completion_evidence",
        ),
        readback_plan(
            "readback_fixture_multi_agent_mailbox_wait_success",
            "fixture_multi_agent_mailbox_wait_success",
            "multi_agent_mailbox_wait_terminal_task_result_wrapper",
            "multi_agent_v2_mailbox_wait",
            "mailbox_wait_delivery_evidence",
        ),
        readback_plan(
            "readback_fixture_multi_agent_reducer_ok",
            "fixture_multi_agent_reducer_ok",
            "multi_agent_reducer_terminal_task_result_wrapper",
            "hepta_runtime_multi_agent_reducer",
            "reducer_consensus_evidence",
        ),
        readback_plan(
            "readback_fixture_agent_job_item_failed",
            "fixture_agent_job_item_failed",
            "agent_job_item_terminal_task_result_wrapper",
            "agent_jobs_batch_workers",
            "agent_job_result_schema_evidence",
        ),
        readback_plan(
            "readback_fixture_worker_task_blocked",
            "fixture_worker_task_blocked",
            "worker_task_terminal_task_result_wrapper",
            "hepta_runtime_worker_tasks",
            "worker_task_artifact_gate_evidence",
        ),
        readback_plan(
            "readback_fixture_task_board_success",
            "fixture_task_board_success",
            "task_board_terminal_task_result_wrapper",
            "hepta_runtime_task_board",
            "task_board_lease_readback_evidence",
        ),
        readback_plan(
            "readback_fixture_scheduler_run_superseded",
            "fixture_scheduler_run_superseded",
            "scheduler_run_terminal_task_result_wrapper",
            "hepta_runtime_scheduler_store",
            "scheduler_admission_decision_evidence",
        ),
        readback_plan(
            "readback_fixture_agent_harness_cancelled",
            "fixture_agent_harness_cancelled",
            "agent_harness_terminal_task_result_wrapper",
            "hepta_runtime_agent_harness",
            "agent_harness_handoff_evidence",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_readback_collection_assertions()
-> Vec<WorkGraphTaskResultReadbackCollectionAssertionPreview> {
    vec![
        collection_assertion(
            "assert_fixture_task_result_collection_hash_matches",
            "taskResults",
            vec!["taskId", "status", "summaryHash", "evidenceHash", "traceId"],
            vec!["taskResultHash", "terminalStatusObserved", "evidenceRefs"],
        ),
        collection_assertion(
            "assert_fixture_timeline_collection_hash_matches",
            "timelineEvents",
            vec!["traceId", "eventKind", "taskId", "wrapperId"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
        collection_assertion(
            "assert_fixture_verifier_refs_match",
            "verifierRefs",
            vec!["verifierRef", "gateReportHash", "schemaVersion"],
            vec!["verifierHash", "schemaVersion", "redactionState"],
        ),
        collection_assertion(
            "assert_fixture_artifact_refs_match",
            "artifacts",
            vec!["taskId", "artifactHash", "producerNodeId"],
            vec!["artifactHash", "artifactCount", "redactionState"],
        ),
        collection_assertion(
            "assert_fixture_scheduler_refs_match",
            "schedulerRefs",
            vec!["schedulerRunId", "leaseId", "admissionDecision"],
            vec!["schedulerRefHash", "leaseState", "decisionHash"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_readback_drift_detectors()
-> Vec<WorkGraphTaskResultReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "detect_fixture_identity_drift",
            vec!["taskId", "traceId", "wrapperId"],
        ),
        drift_detector(
            "detect_fixture_status_drift",
            vec!["status", "terminalStatusObserved"],
        ),
        drift_detector(
            "detect_fixture_evidence_drift",
            vec!["evidenceHash", "evidenceRefs"],
        ),
        drift_detector(
            "detect_fixture_verifier_drift",
            vec!["verifierRef", "gateReportHash"],
        ),
        drift_detector(
            "detect_fixture_redaction_drift",
            vec!["summaryHash", "redactionState"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_readback_blockers()
-> Vec<WorkGraphTaskResultReadbackBlockerPreview> {
    vec![
        blocker(
            "readback_execution_disabled",
            "high",
            "keep readback as contract-only until fixture runner output is reviewed",
        ),
        blocker(
            "drift_persistence_disabled",
            "high",
            "do not persist drift state before operator-readable drift budget preview exists",
        ),
        blocker(
            "wrapper_execution_disabled",
            "medium",
            "do not execute wrappers until readback and drift budget previews pass",
        ),
        blocker(
            "task_result_enforcement_disabled",
            "medium",
            "keep TaskResult enforcement disabled until readback proves all terminal fixture outputs",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_readback_required_prior_gates() -> Vec<&'static str>
{
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
        "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
        "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate",
    ]
}

impl WorkGraphTerminalTaskResultWrapperReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            fixture_executed: false,
            wrapper_executed: false,
            readback_performed: false,
            drift_state_persisted: false,
            event_record_persisted: false,
            task_result_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            replay_executed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_plan(
    id: &'static str,
    fixture_id: &'static str,
    wrapper_id: &'static str,
    source_surface_id: &'static str,
    expected_evidence_contract_id: &'static str,
) -> WorkGraphTerminalTaskResultReadbackPlanPreview {
    WorkGraphTerminalTaskResultReadbackPlanPreview {
        id,
        fixture_id,
        wrapper_id,
        source_surface_id,
        expected_task_result_collection_id: "taskResults",
        expected_timeline_collection_id: "timelineEvents",
        expected_evidence_contract_id,
        required_collection_assertion_ids: vec![
            "assert_fixture_task_result_collection_hash_matches",
            "assert_fixture_timeline_collection_hash_matches",
            "assert_fixture_verifier_refs_match",
        ],
        drift_detector_ids: work_graph_terminal_task_result_wrapper_readback_drift_detector_ids(),
        readback_state: "preview_contract_defined_readback_execution_disabled",
        redaction_policy: "compare ids, hashes, refs, and redaction state without raw payload",
        performs_readback: false,
        persists_drift: false,
        mutates_store: false,
        enforces_task_result: false,
    }
}

fn work_graph_terminal_task_result_wrapper_readback_drift_detector_ids() -> Vec<&'static str> {
    work_graph_terminal_task_result_wrapper_readback_drift_detectors()
        .iter()
        .map(|detector| detector.id)
        .collect()
}

fn collection_assertion(
    id: &'static str,
    collection_id: &'static str,
    required_inputs: Vec<&'static str>,
    evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultReadbackCollectionAssertionPreview {
    WorkGraphTaskResultReadbackCollectionAssertionPreview {
        id,
        collection_id,
        required_inputs,
        evidence_fields,
        blocks_wrapper_execution: true,
        performs_readback: false,
        mutates_store: false,
    }
}

fn drift_detector(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphTaskResultReadbackDriftDetectorPreview {
    WorkGraphTaskResultReadbackDriftDetectorPreview {
        id,
        compared_fields,
        severity: "critical",
        blocks_wrapper_execution: true,
        persists_drift: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    recommended_fix: &'static str,
) -> WorkGraphTaskResultReadbackBlockerPreview {
    WorkGraphTaskResultReadbackBlockerPreview {
        id,
        severity,
        affected_readback_plan_ids: work_graph_terminal_task_result_wrapper_readback_plans()
            .iter()
            .map(|plan| plan.id)
            .collect(),
        required_before_readback_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_readback_preview_declares_one_plan_per_fixture() {
        let report = hepta_work_graph_terminal_task_result_wrapper_readback_preview_report();

        assert_eq!(report.readback_plan_count, 8);
        assert!(report.readback_plans.iter().all(|plan| {
            plan.expected_task_result_collection_id == "taskResults"
                && plan.expected_timeline_collection_id == "timelineEvents"
                && !plan.performs_readback
                && !plan.persists_drift
                && !plan.mutates_store
                && !plan.enforces_task_result
        }));
    }

    #[test]
    fn wrapper_readback_preview_asserts_expected_collections() {
        let report = hepta_work_graph_terminal_task_result_wrapper_readback_preview_report();
        let collection_ids = report
            .collection_assertions
            .iter()
            .map(|assertion| assertion.collection_id)
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(report.collection_assertion_count, 5);
        assert_eq!(
            collection_ids,
            [
                "artifacts",
                "schedulerRefs",
                "taskResults",
                "timelineEvents",
                "verifierRefs",
            ]
            .into_iter()
            .collect()
        );
        assert!(
            report
                .collection_assertions
                .iter()
                .all(|assertion| assertion.blocks_wrapper_execution && !assertion.mutates_store)
        );
    }

    #[test]
    fn wrapper_readback_preview_defines_critical_drift_detectors() {
        let report = hepta_work_graph_terminal_task_result_wrapper_readback_preview_report();

        assert_eq!(report.drift_detector_count, 5);
        assert!(report.drift_detectors.iter().all(|detector| {
            detector.severity == "critical"
                && detector.blocks_wrapper_execution
                && !detector.persists_drift
        }));
        assert!(
            report
                .readback_plans
                .iter()
                .all(|plan| plan.drift_detector_ids.len() == 5)
        );
    }

    #[test]
    fn wrapper_readback_preview_blocks_execution_and_persistence() {
        let report = hepta_work_graph_terminal_task_result_wrapper_readback_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(report.ready_for_drift_budget_preview);
        assert!(!report.ready_for_readback_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperReadbackPreviewSideEffects::none()
        );
    }

    #[test]
    fn wrapper_readback_preview_requires_fixture_prior() {
        let report = hepta_work_graph_terminal_task_result_wrapper_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 14);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
