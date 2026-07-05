use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_fixture_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperFixturePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub fixture_count: usize,
    pub expected_task_result_count: usize,
    pub field_assertion_count: usize,
    pub terminal_required_field_count: usize,
    pub verifier_contract_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub fixtures: Vec<WorkGraphTerminalTaskResultWrapperFixturePreview>,
    pub field_assertions: Vec<WorkGraphTaskResultFixtureFieldAssertionPreview>,
    pub verifier_contracts: Vec<WorkGraphTaskResultFixtureVerifierContractPreview>,
    pub blockers: Vec<WorkGraphTerminalTaskResultWrapperFixtureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_wrapper_readback_preview: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperFixturePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperFixturePreview {
    pub id: &'static str,
    pub wrapper_id: &'static str,
    pub source_surface_id: &'static str,
    pub source_fixture_ref: &'static str,
    pub source_status: &'static str,
    pub expected_task_result_id_formula: &'static str,
    pub expected_canonical_status: &'static str,
    pub expected_task_result_node_kind: &'static str,
    pub expected_event_contract_id: &'static str,
    pub required_evidence_contract_id: &'static str,
    pub expected_wire_fields: Vec<&'static str>,
    pub expected_terminal_fields: Vec<&'static str>,
    pub fixture_state: &'static str,
    pub redaction_policy: &'static str,
    pub executes_fixture: bool,
    pub persists_task_result: bool,
    pub enforces_task_result: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultFixtureFieldAssertionPreview {
    pub wire_name: &'static str,
    pub required: bool,
    pub terminal_required: bool,
    pub assertion_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultFixtureVerifierContractPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub verifier_ref_fields: Vec<&'static str>,
    pub golden_hash_fields: Vec<&'static str>,
    pub stores_raw_payload: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperFixtureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_fixture_ids: Vec<&'static str>,
    pub required_before_wrapper_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperFixturePreviewSideEffects {
    pub filesystem_written: bool,
    pub fixture_executed: bool,
    pub wrapper_executed: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub readback_performed: bool,
    pub replay_executed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report()
-> WorkGraphTerminalTaskResultWrapperFixturePreviewReport {
    let fixtures = work_graph_terminal_task_result_wrapper_fixtures();
    let field_assertions = work_graph_terminal_task_result_wrapper_fixture_field_assertions();
    let verifier_contracts = work_graph_terminal_task_result_wrapper_fixture_verifier_contracts();
    let blockers = work_graph_terminal_task_result_wrapper_fixture_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_fixture_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperFixturePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_fixture_preview_no_execution",
        fixture_count: fixtures.len(),
        expected_task_result_count: fixtures.len(),
        field_assertion_count: field_assertions.len(),
        terminal_required_field_count: field_assertions
            .iter()
            .filter(|field| field.terminal_required)
            .count(),
        verifier_contract_count: verifier_contracts.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        fixtures,
        field_assertions,
        verifier_contracts,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_RECOMMENDED_NEXT_GATE,
        ready_for_wrapper_readback_preview: true,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultWrapperFixturePreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_fixtures()
-> Vec<WorkGraphTerminalTaskResultWrapperFixturePreview> {
    vec![
        fixture(
            "fixture_multi_agent_thread_spawn_success",
            "multi_agent_thread_spawn_terminal_task_result_wrapper",
            "multi_agent_v2_thread_spawn",
            "fixtures/work_graph/task_result/thread_spawn_success.json",
            "success",
            "succeeded",
            "agent_task",
            "thread_spawn_completion_evidence",
        ),
        fixture(
            "fixture_multi_agent_mailbox_wait_success",
            "multi_agent_mailbox_wait_terminal_task_result_wrapper",
            "multi_agent_v2_mailbox_wait",
            "fixtures/work_graph/task_result/mailbox_wait_success.json",
            "success",
            "succeeded",
            "agent_task",
            "mailbox_wait_delivery_evidence",
        ),
        fixture(
            "fixture_multi_agent_reducer_ok",
            "multi_agent_reducer_terminal_task_result_wrapper",
            "hepta_runtime_multi_agent_reducer",
            "fixtures/work_graph/task_result/reducer_ok.json",
            "ok",
            "succeeded",
            "agent_task",
            "reducer_consensus_evidence",
        ),
        fixture(
            "fixture_agent_job_item_failed",
            "agent_job_item_terminal_task_result_wrapper",
            "agent_jobs_batch_workers",
            "fixtures/work_graph/task_result/agent_job_failed.json",
            "failed",
            "failed",
            "worker_task",
            "agent_job_result_schema_evidence",
        ),
        fixture(
            "fixture_worker_task_blocked",
            "worker_task_terminal_task_result_wrapper",
            "hepta_runtime_worker_tasks",
            "fixtures/work_graph/task_result/worker_task_blocked.json",
            "blocked",
            "blocked",
            "worker_task",
            "worker_task_artifact_gate_evidence",
        ),
        fixture(
            "fixture_task_board_success",
            "task_board_terminal_task_result_wrapper",
            "hepta_runtime_task_board",
            "fixtures/work_graph/task_result/task_board_success.json",
            "success",
            "succeeded",
            "worker_task",
            "task_board_lease_readback_evidence",
        ),
        fixture(
            "fixture_scheduler_run_superseded",
            "scheduler_run_terminal_task_result_wrapper",
            "hepta_runtime_scheduler_store",
            "fixtures/work_graph/task_result/scheduler_run_superseded.json",
            "superseded",
            "superseded",
            "scheduler_run",
            "scheduler_admission_decision_evidence",
        ),
        fixture(
            "fixture_agent_harness_cancelled",
            "agent_harness_terminal_task_result_wrapper",
            "hepta_runtime_agent_harness",
            "fixtures/work_graph/task_result/agent_harness_cancelled.json",
            "cancelled",
            "cancelled",
            "external_handoff",
            "agent_harness_handoff_evidence",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_fixture_field_assertions()
-> Vec<WorkGraphTaskResultFixtureFieldAssertionPreview> {
    vec![
        field("taskId", true),
        field("status", true),
        field("summary", true),
        field("artifacts", false),
        field("evidence", true),
        field("risks", false),
        field("nextActions", false),
        field("verifier", true),
        field("reducer", false),
        field("usage", false),
        field("traceId", true),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_fixture_verifier_contracts()
-> Vec<WorkGraphTaskResultFixtureVerifierContractPreview> {
    work_graph_terminal_task_result_wrapper_fixtures()
        .into_iter()
        .map(
            |fixture| WorkGraphTaskResultFixtureVerifierContractPreview {
                id: fixture.required_evidence_contract_id,
                source_surface_id: fixture.source_surface_id,
                verifier_ref_fields: vec!["verifierRef", "gateReportHash", "schemaVersion"],
                golden_hash_fields: vec!["taskId", "status", "summaryHash", "evidenceHash"],
                stores_raw_payload: false,
                performs_readback: false,
                mutates_store: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_wrapper_fixture_blockers()
-> Vec<WorkGraphTerminalTaskResultWrapperFixtureBlockerPreview> {
    vec![
        blocker(
            "fixture_runner_disabled",
            "high",
            "add a fixture runner that produces redacted TaskResult JSON without touching runtime state",
        ),
        blocker(
            "golden_task_result_hashes_preview_only",
            "high",
            "record expected hashes only after fixture runner output is stable and reviewed",
        ),
        blocker(
            "wrapper_execution_disabled",
            "medium",
            "keep runtime wrapper execution disabled until fixture and readback previews pass",
        ),
        blocker(
            "task_result_enforcement_disabled",
            "medium",
            "do not enforce TaskResult validation until fixtures cover every terminal source",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_fixture_required_prior_gates() -> Vec<&'static str> {
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperFixturePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            fixture_executed: false,
            wrapper_executed: false,
            event_record_persisted: false,
            task_result_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
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

fn fixture(
    id: &'static str,
    wrapper_id: &'static str,
    source_surface_id: &'static str,
    source_fixture_ref: &'static str,
    source_status: &'static str,
    expected_canonical_status: &'static str,
    expected_task_result_node_kind: &'static str,
    required_evidence_contract_id: &'static str,
) -> WorkGraphTerminalTaskResultWrapperFixturePreview {
    WorkGraphTerminalTaskResultWrapperFixturePreview {
        id,
        wrapper_id,
        source_surface_id,
        source_fixture_ref,
        source_status,
        expected_task_result_id_formula: "sha256(traceId || taskId || sourceSurfaceId || status || evidenceHash)",
        expected_canonical_status,
        expected_task_result_node_kind,
        expected_event_contract_id: "task_result_event_intake",
        required_evidence_contract_id,
        expected_wire_fields: vec![
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
        ],
        expected_terminal_fields: vec![
            "taskId", "status", "summary", "evidence", "verifier", "traceId",
        ],
        fixture_state: "fixture_declared_not_executed",
        redaction_policy: "fixture payload is referenced by hash and never executed or persisted",
        executes_fixture: false,
        persists_task_result: false,
        enforces_task_result: false,
        mutates_store: false,
    }
}

fn field(
    wire_name: &'static str,
    terminal_required: bool,
) -> WorkGraphTaskResultFixtureFieldAssertionPreview {
    WorkGraphTaskResultFixtureFieldAssertionPreview {
        wire_name,
        required: true,
        terminal_required,
        assertion_id: "task_result_fixture_field_present_and_redacted",
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    recommended_fix: &'static str,
) -> WorkGraphTerminalTaskResultWrapperFixtureBlockerPreview {
    WorkGraphTerminalTaskResultWrapperFixtureBlockerPreview {
        id,
        severity,
        affected_fixture_ids: work_graph_terminal_task_result_wrapper_fixtures()
            .iter()
            .map(|fixture| fixture.id)
            .collect(),
        required_before_wrapper_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_fixture_preview_declares_one_fixture_per_terminal_wrapper() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();
        let sources = report
            .fixtures
            .iter()
            .map(|fixture| fixture.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.fixture_count, 8);
        assert_eq!(report.expected_task_result_count, 8);
        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_task_board",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert!(report.fixtures.iter().all(|fixture| {
            fixture.expected_event_contract_id == "task_result_event_intake"
                && !fixture.executes_fixture
                && !fixture.persists_task_result
                && !fixture.enforces_task_result
                && !fixture.mutates_store
        }));
    }

    #[test]
    fn wrapper_fixture_preview_asserts_canonical_task_result_fields() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();

        assert_eq!(report.field_assertion_count, 11);
        assert_eq!(report.terminal_required_field_count, 6);
        assert!(report.field_assertions.iter().all(|field| field.required));
        assert!(report.fixtures.iter().all(|fixture| {
            fixture.expected_wire_fields.len() == 11 && fixture.expected_terminal_fields.len() == 6
        }));
    }

    #[test]
    fn wrapper_fixture_preview_covers_terminal_status_variety() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();
        let statuses = report
            .fixtures
            .iter()
            .map(|fixture| fixture.expected_canonical_status)
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(
            statuses,
            ["blocked", "cancelled", "failed", "succeeded", "superseded"]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn wrapper_fixture_preview_keeps_verifier_contracts_redacted() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();

        assert_eq!(report.verifier_contract_count, 8);
        assert!(report.verifier_contracts.iter().all(|contract| {
            !contract.stores_raw_payload && !contract.performs_readback && !contract.mutates_store
        }));
    }

    #[test]
    fn wrapper_fixture_preview_blocks_execution_and_live_paths() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(report.ready_for_wrapper_readback_preview);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperFixturePreviewSideEffects::none()
        );
    }

    #[test]
    fn wrapper_fixture_preview_requires_terminal_wrapper_prior() {
        let report = hepta_work_graph_terminal_task_result_wrapper_fixture_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_terminal_task_result_wrapper_preview_gate")
        );
        assert_eq!(report.required_prior_gate_count, 13);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_FIXTURE_RECOMMENDED_NEXT_GATE
        );
    }
}
