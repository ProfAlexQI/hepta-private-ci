use serde::Serialize;

pub const WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_PREVIEW_GATE: &str =
    "hepta_work_graph_idempotency_readback_adapter_preview_gate";
pub const WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_SCHEMA_VERSION: &str =
    "work_graph_idempotency_readback_adapter_preview_v1";
pub const WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyReadbackAdapterPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_adapter_count: usize,
    pub replay_key_contract_count: usize,
    pub readback_probe_contract_count: usize,
    pub gap_closure_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub source_adapters: Vec<WorkGraphIdempotencySourceAdapterPreview>,
    pub replay_key_contracts: Vec<WorkGraphReplayKeyContractPreview>,
    pub readback_probe_contracts: Vec<WorkGraphSourceReadbackProbeContractPreview>,
    pub gap_closures: Vec<WorkGraphSourceGapClosurePreview>,
    pub blockers: Vec<WorkGraphIdempotencyReadbackAdapterBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_task_result_wrapper_preview: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphIdempotencyReadbackAdapterPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencySourceAdapterPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub emitted_event_contract_ids: Vec<&'static str>,
    pub replay_key_contract_id: &'static str,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub expected_collection_ids: Vec<&'static str>,
    pub closes_source_gap_id: &'static str,
    pub adapter_state: &'static str,
    pub redaction_policy: &'static str,
    pub requires_task_result_wrapper: bool,
    pub attaches_runtime_adapter: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayKeyContractPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub key_formula: &'static str,
    pub collision_policy: &'static str,
    pub replay_scope: &'static str,
    pub redaction_policy: &'static str,
    pub mutates_idempotency_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceReadbackProbeContractPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub collection_id: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub evidence_fields: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceGapClosurePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closes_replay_readback_gap_id: &'static str,
    pub adapter_id: &'static str,
    pub closure_state: &'static str,
    pub required_before_replay_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyReadbackAdapterBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_replay_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyReadbackAdapterPreviewSideEffects {
    pub filesystem_written: bool,
    pub event_record_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub readback_performed: bool,
    pub runtime_adapter_attached: bool,
    pub replay_executed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_idempotency_readback_adapter_preview_report()
-> WorkGraphIdempotencyReadbackAdapterPreviewReport {
    let source_adapters = work_graph_idempotency_readback_source_adapters();
    let replay_key_contracts = work_graph_idempotency_readback_replay_key_contracts();
    let readback_probe_contracts = work_graph_idempotency_readback_probe_contracts();
    let gap_closures = work_graph_idempotency_readback_gap_closures();
    let blockers = work_graph_idempotency_readback_adapter_blockers();
    let required_prior_gates = work_graph_idempotency_readback_adapter_required_prior_gates();

    WorkGraphIdempotencyReadbackAdapterPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_SCHEMA_VERSION,
        preview_mode: "read_only_idempotency_readback_adapter_preview_no_attachment",
        source_adapter_count: source_adapters.len(),
        replay_key_contract_count: replay_key_contracts.len(),
        readback_probe_contract_count: readback_probe_contracts.len(),
        gap_closure_count: gap_closures.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_adapters,
        replay_key_contracts,
        readback_probe_contracts,
        gap_closures,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_RECOMMENDED_NEXT_GATE,
        ready_for_task_result_wrapper_preview: true,
        ready_for_replay_execution: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphIdempotencyReadbackAdapterPreviewSideEffects::none(),
    }
}

pub fn work_graph_idempotency_readback_source_adapters()
-> Vec<WorkGraphIdempotencySourceAdapterPreview> {
    vec![
        source_adapter(
            "plan_mode_proposed_plan_blocks_idempotency_readback_adapter",
            "plan_mode_proposed_plan_blocks",
            "planning",
            vec!["plan_step_event_intake"],
            "plan_mode_plan_step_replay_key",
            vec![
                "plan_mode_nodes_readback_probe",
                "plan_mode_edges_readback_probe",
                "plan_mode_timeline_readback_probe",
            ],
            vec!["nodes", "edges", "timelineEvents"],
            "gap_plan_mode_proposed_plan_blocks_replay_key",
            false,
            "hash raw proposed plan text and expose only trace, step, proposal hash, and redacted refs",
        ),
        source_adapter(
            "app_server_turn_plan_notification_idempotency_readback_adapter",
            "app_server_turn_plan_notification",
            "planning",
            vec!["plan_step_event_intake"],
            "app_server_turn_plan_notification_replay_key",
            vec![
                "app_server_plan_nodes_readback_probe",
                "app_server_plan_edges_readback_probe",
                "app_server_plan_timeline_readback_probe",
            ],
            vec!["nodes", "edges", "timelineEvents"],
            "gap_app_server_turn_plan_notification_replay_key",
            false,
            "store notification sequence and proposal hash only, never raw plan payload",
        ),
        source_adapter(
            "multi_agent_mailbox_delivery_idempotency_readback_adapter",
            "multi_agent_v2_mailbox_wait",
            "multi_agent",
            vec!["mailbox_delivery_event_intake"],
            "multi_agent_mailbox_delivery_replay_key",
            vec![
                "mailbox_edges_readback_probe",
                "mailbox_timeline_readback_probe",
            ],
            vec!["edges", "timelineEvents"],
            "gap_multi_agent_mailbox_delivery_replay_key",
            false,
            "hash inter-agent message content and expose mailbox seq, agent path, and delivery state refs",
        ),
        source_adapter(
            "multi_agent_reducer_task_result_idempotency_readback_adapter",
            "hepta_runtime_multi_agent_reducer",
            "multi_agent",
            vec!["task_result_event_intake"],
            "multi_agent_reducer_task_result_replay_key",
            vec![
                "multi_agent_reducer_task_result_readback_probe",
                "multi_agent_reducer_timeline_readback_probe",
            ],
            vec!["taskResults", "timelineEvents"],
            "gap_multi_agent_reducer_task_result_replay_key",
            true,
            "store reducer strategy, status, and evidence hash without raw subagent payload",
        ),
        source_adapter(
            "task_board_worker_task_idempotency_readback_adapter",
            "hepta_runtime_task_board",
            "runtime_scheduler",
            vec!["worker_task_event_intake"],
            "task_board_worker_task_replay_key",
            vec![
                "task_board_nodes_readback_probe",
                "task_board_task_results_readback_probe",
                "task_board_artifacts_readback_probe",
                "task_board_timeline_readback_probe",
            ],
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            "gap_task_board_worker_task_replay_key",
            true,
            "store lane, lease state, attempt, and artifact hashes without command payload",
        ),
    ]
}

pub fn work_graph_idempotency_readback_replay_key_contracts()
-> Vec<WorkGraphReplayKeyContractPreview> {
    vec![
        replay_key_contract(
            "plan_mode_plan_step_replay_key",
            "plan_mode_proposed_plan_blocks",
            vec!["traceId", "turnId", "stepIndex", "proposalHash"],
            "sha256(traceId || turnId || stepIndex || proposalHash)",
            "collision_blocks_plan_node_materialization",
            "turn_plan_step",
            "proposalHash replaces raw proposed plan text",
        ),
        replay_key_contract(
            "app_server_turn_plan_notification_replay_key",
            "app_server_turn_plan_notification",
            vec!["traceId", "turnId", "notificationSeq", "proposalHash"],
            "sha256(traceId || turnId || notificationSeq || proposalHash)",
            "collision_blocks_app_server_plan_notification_materialization",
            "turn_plan_notification",
            "proposalHash and notificationSeq replace raw notification payload",
        ),
        replay_key_contract(
            "multi_agent_mailbox_delivery_replay_key",
            "multi_agent_v2_mailbox_wait",
            vec!["traceId", "agentPath", "mailboxSeq", "deliveryState"],
            "sha256(traceId || agentPath || mailboxSeq || deliveryState)",
            "collision_blocks_mailbox_delivery_materialization",
            "agent_mailbox_delivery",
            "message content is hashed and delivery refs are redacted",
        ),
        replay_key_contract(
            "multi_agent_reducer_task_result_replay_key",
            "hepta_runtime_multi_agent_reducer",
            vec![
                "traceId",
                "taskId",
                "reducerStrategy",
                "status",
                "evidenceHash",
            ],
            "sha256(traceId || taskId || reducerStrategy || status || evidenceHash)",
            "collision_blocks_reducer_task_result_materialization",
            "multi_agent_reducer_task_result",
            "evidenceHash and verifier refs replace raw subagent output",
        ),
        replay_key_contract(
            "task_board_worker_task_replay_key",
            "hepta_runtime_task_board",
            vec![
                "workerTaskId",
                "attempt",
                "lane",
                "leaseState",
                "artifactHash",
            ],
            "sha256(workerTaskId || attempt || lane || leaseState || artifactHash)",
            "collision_blocks_task_board_worker_task_materialization",
            "scheduler_worker_task",
            "artifactHash and lease state replace command payload",
        ),
    ]
}

pub fn work_graph_idempotency_readback_probe_contracts()
-> Vec<WorkGraphSourceReadbackProbeContractPreview> {
    vec![
        readback_probe(
            "plan_mode_nodes_readback_probe",
            "plan_mode_proposed_plan_blocks",
            "nodes",
            vec!["traceId", "planStepId", "proposalHash"],
            vec!["nodeHash", "status", "redactionState"],
        ),
        readback_probe(
            "plan_mode_edges_readback_probe",
            "plan_mode_proposed_plan_blocks",
            "edges",
            vec!["traceId", "planStepId", "dependencyRefs"],
            vec!["edgeHash", "dependencyCount", "missingEdgeIds"],
        ),
        readback_probe(
            "plan_mode_timeline_readback_probe",
            "plan_mode_proposed_plan_blocks",
            "timelineEvents",
            vec!["traceId", "eventKind", "proposalHash"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
        readback_probe(
            "app_server_plan_nodes_readback_probe",
            "app_server_turn_plan_notification",
            "nodes",
            vec!["traceId", "turnId", "notificationSeq"],
            vec!["nodeHash", "notificationStatus", "redactionState"],
        ),
        readback_probe(
            "app_server_plan_edges_readback_probe",
            "app_server_turn_plan_notification",
            "edges",
            vec!["traceId", "turnId", "notificationSeq"],
            vec!["edgeHash", "dependencyCount", "missingEdgeIds"],
        ),
        readback_probe(
            "app_server_plan_timeline_readback_probe",
            "app_server_turn_plan_notification",
            "timelineEvents",
            vec!["traceId", "turnId", "notificationSeq"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
        readback_probe(
            "mailbox_edges_readback_probe",
            "multi_agent_v2_mailbox_wait",
            "edges",
            vec!["traceId", "agentPath", "mailboxSeq"],
            vec!["edgeHash", "deliveryState", "missingEdgeIds"],
        ),
        readback_probe(
            "mailbox_timeline_readback_probe",
            "multi_agent_v2_mailbox_wait",
            "timelineEvents",
            vec!["traceId", "agentPath", "mailboxSeq"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
        readback_probe(
            "multi_agent_reducer_task_result_readback_probe",
            "hepta_runtime_multi_agent_reducer",
            "taskResults",
            vec!["traceId", "taskId", "reducerStrategy", "evidenceHash"],
            vec!["taskResultHash", "terminalStatusObserved", "evidenceRefs"],
        ),
        readback_probe(
            "multi_agent_reducer_timeline_readback_probe",
            "hepta_runtime_multi_agent_reducer",
            "timelineEvents",
            vec!["traceId", "taskId", "reducerStrategy"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
        readback_probe(
            "task_board_nodes_readback_probe",
            "hepta_runtime_task_board",
            "nodes",
            vec!["workerTaskId", "attempt", "lane"],
            vec!["nodeHash", "leaseState", "status"],
        ),
        readback_probe(
            "task_board_task_results_readback_probe",
            "hepta_runtime_task_board",
            "taskResults",
            vec!["workerTaskId", "attempt", "artifactHash"],
            vec!["taskResultHash", "terminalStatusObserved", "evidenceRefs"],
        ),
        readback_probe(
            "task_board_artifacts_readback_probe",
            "hepta_runtime_task_board",
            "artifacts",
            vec!["workerTaskId", "artifactHash", "producerNodeId"],
            vec!["artifactCount", "artifactHash", "redactionState"],
        ),
        readback_probe(
            "task_board_timeline_readback_probe",
            "hepta_runtime_task_board",
            "timelineEvents",
            vec!["workerTaskId", "attempt", "lane"],
            vec!["timelineHash", "eventCount", "redactionState"],
        ),
    ]
}

pub fn work_graph_idempotency_readback_gap_closures() -> Vec<WorkGraphSourceGapClosurePreview> {
    vec![
        gap_closure(
            "close_plan_mode_proposed_plan_blocks_replay_key_gap",
            "plan_mode_proposed_plan_blocks",
            "gap_plan_mode_proposed_plan_blocks_replay_key",
            "plan_mode_proposed_plan_blocks_idempotency_readback_adapter",
        ),
        gap_closure(
            "close_app_server_turn_plan_notification_replay_key_gap",
            "app_server_turn_plan_notification",
            "gap_app_server_turn_plan_notification_replay_key",
            "app_server_turn_plan_notification_idempotency_readback_adapter",
        ),
        gap_closure(
            "close_multi_agent_mailbox_delivery_replay_key_gap",
            "multi_agent_v2_mailbox_wait",
            "gap_multi_agent_mailbox_delivery_replay_key",
            "multi_agent_mailbox_delivery_idempotency_readback_adapter",
        ),
        gap_closure(
            "close_multi_agent_reducer_task_result_replay_key_gap",
            "hepta_runtime_multi_agent_reducer",
            "gap_multi_agent_reducer_task_result_replay_key",
            "multi_agent_reducer_task_result_idempotency_readback_adapter",
        ),
        gap_closure(
            "close_task_board_worker_task_replay_key_gap",
            "hepta_runtime_task_board",
            "gap_task_board_worker_task_replay_key",
            "task_board_worker_task_idempotency_readback_adapter",
        ),
    ]
}

pub fn work_graph_idempotency_readback_adapter_blockers()
-> Vec<WorkGraphIdempotencyReadbackAdapterBlockerPreview> {
    vec![
        blocker(
            "runtime_adapter_attachment_disabled",
            "high",
            work_graph_idempotency_readback_gap_source_surface_ids(),
            "wire these contracts to source adapters only after replay fixtures pass",
        ),
        blocker(
            "terminal_task_result_wrapper_not_enforced",
            "high",
            vec![
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ],
            "wrap reducer and task_board terminal outputs in TaskResult before execution",
        ),
        blocker(
            "append_only_store_still_disabled",
            "medium",
            work_graph_idempotency_readback_gap_source_surface_ids(),
            "keep store writes disabled until replay/readback evidence is deterministic",
        ),
        blocker(
            "replay_execution_disabled_by_design",
            "medium",
            work_graph_idempotency_readback_gap_source_surface_ids(),
            "run fixture-only replay verification before any WAL replay execution",
        ),
    ]
}

pub fn work_graph_idempotency_readback_adapter_required_prior_gates() -> Vec<&'static str> {
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
    ]
}

pub fn work_graph_idempotency_readback_gap_source_surface_ids() -> Vec<&'static str> {
    vec![
        "plan_mode_proposed_plan_blocks",
        "app_server_turn_plan_notification",
        "multi_agent_v2_mailbox_wait",
        "hepta_runtime_multi_agent_reducer",
        "hepta_runtime_task_board",
    ]
}

impl WorkGraphIdempotencyReadbackAdapterPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_record_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            readback_performed: false,
            runtime_adapter_attached: false,
            replay_executed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn source_adapter(
    id: &'static str,
    source_surface_id: &'static str,
    source_category: &'static str,
    emitted_event_contract_ids: Vec<&'static str>,
    replay_key_contract_id: &'static str,
    readback_probe_contract_ids: Vec<&'static str>,
    expected_collection_ids: Vec<&'static str>,
    closes_source_gap_id: &'static str,
    requires_task_result_wrapper: bool,
    redaction_policy: &'static str,
) -> WorkGraphIdempotencySourceAdapterPreview {
    WorkGraphIdempotencySourceAdapterPreview {
        id,
        source_surface_id,
        source_category,
        emitted_event_contract_ids,
        replay_key_contract_id,
        readback_probe_contract_ids,
        expected_collection_ids,
        closes_source_gap_id,
        adapter_state: "preview_contract_ready_runtime_attachment_disabled",
        redaction_policy,
        requires_task_result_wrapper,
        attaches_runtime_adapter: false,
        performs_readback: false,
        mutates_store: false,
    }
}

fn replay_key_contract(
    id: &'static str,
    source_surface_id: &'static str,
    key_fields: Vec<&'static str>,
    key_formula: &'static str,
    collision_policy: &'static str,
    replay_scope: &'static str,
    redaction_policy: &'static str,
) -> WorkGraphReplayKeyContractPreview {
    WorkGraphReplayKeyContractPreview {
        id,
        source_surface_id,
        key_fields,
        key_formula,
        collision_policy,
        replay_scope,
        redaction_policy,
        mutates_idempotency_index: false,
    }
}

fn readback_probe(
    id: &'static str,
    source_surface_id: &'static str,
    collection_id: &'static str,
    required_inputs: Vec<&'static str>,
    evidence_fields: Vec<&'static str>,
) -> WorkGraphSourceReadbackProbeContractPreview {
    WorkGraphSourceReadbackProbeContractPreview {
        id,
        source_surface_id,
        collection_id,
        required_inputs,
        evidence_fields,
        drift_detector_ids: vec![
            "detect_identity_drift",
            "detect_ordering_drift",
            "detect_hash_drift",
        ],
        performs_readback: false,
        mutates_store: false,
    }
}

fn gap_closure(
    id: &'static str,
    source_surface_id: &'static str,
    closes_replay_readback_gap_id: &'static str,
    adapter_id: &'static str,
) -> WorkGraphSourceGapClosurePreview {
    WorkGraphSourceGapClosurePreview {
        id,
        source_surface_id,
        closes_replay_readback_gap_id,
        adapter_id,
        closure_state: "preview_contract_defined_runtime_not_attached",
        required_before_replay_execution: true,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphIdempotencyReadbackAdapterBlockerPreview {
    WorkGraphIdempotencyReadbackAdapterBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_replay_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_readback_preview_declares_five_source_adapters() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();
        let source_ids = report
            .source_adapters
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.source_adapter_count, 5);
        assert_eq!(
            source_ids,
            [
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert!(report.source_adapters.iter().all(|adapter| {
            !adapter.attaches_runtime_adapter
                && !adapter.performs_readback
                && !adapter.mutates_store
        }));
    }

    #[test]
    fn idempotency_readback_preview_defines_stable_replay_keys() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();

        assert_eq!(report.replay_key_contract_count, 5);
        assert!(report.replay_key_contracts.iter().all(|contract| {
            !contract.mutates_idempotency_index
                && contract.key_formula.starts_with("sha256(")
                && contract.key_fields.len() >= 4
        }));
    }

    #[test]
    fn idempotency_readback_preview_covers_expected_collections() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();
        let collection_ids = report
            .readback_probe_contracts
            .iter()
            .map(|probe| probe.collection_id)
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(report.readback_probe_contract_count, 14);
        assert_eq!(
            collection_ids,
            [
                "artifacts",
                "edges",
                "nodes",
                "taskResults",
                "timelineEvents"
            ]
            .into_iter()
            .collect()
        );
        assert!(
            report
                .readback_probe_contracts
                .iter()
                .all(|probe| !probe.performs_readback && !probe.mutates_store)
        );
    }

    #[test]
    fn idempotency_readback_preview_closes_replay_readback_source_gaps_as_contracts_only() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();
        let closed_gap_ids = report
            .gap_closures
            .iter()
            .map(|closure| closure.closes_replay_readback_gap_id)
            .collect::<Vec<_>>();

        assert_eq!(report.gap_closure_count, 5);
        assert_eq!(
            closed_gap_ids,
            [
                "gap_plan_mode_proposed_plan_blocks_replay_key",
                "gap_app_server_turn_plan_notification_replay_key",
                "gap_multi_agent_mailbox_delivery_replay_key",
                "gap_multi_agent_reducer_task_result_replay_key",
                "gap_task_board_worker_task_replay_key",
            ]
        );
        assert!(report.gap_closures.iter().all(
            |closure| closure.closure_state == "preview_contract_defined_runtime_not_attached"
        ));
    }

    #[test]
    fn idempotency_readback_preview_blocks_execution_and_store_enablement() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(report.ready_for_task_result_wrapper_preview);
        assert!(!report.ready_for_replay_execution);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphIdempotencyReadbackAdapterPreviewSideEffects::none()
        );
    }

    #[test]
    fn idempotency_readback_preview_requires_replay_readback_prior() {
        let report = hepta_work_graph_idempotency_readback_adapter_preview_report();

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
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_IDEMPOTENCY_READBACK_ADAPTER_RECOMMENDED_NEXT_GATE
        );
    }
}
