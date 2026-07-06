use serde::Serialize;

pub const WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_GATE: &str =
    "hepta_work_graph_adapter_projection_fixture_gate";
pub const WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_SCHEMA_VERSION: &str =
    "work_graph_adapter_projection_fixture_v1";
pub const WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_state_store_persistence_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterProjectionFixtureReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub fixture_count: usize,
    pub source_surface_count: usize,
    pub projected_collection_count: usize,
    pub invariant_count: usize,
    pub fixtures: Vec<WorkGraphAdapterProjectionFixture>,
    pub projected_collections: Vec<WorkGraphAdapterProjectionCollectionCoverage>,
    pub invariants: Vec<WorkGraphAdapterProjectionInvariant>,
    pub recommended_next_gate: &'static str,
    pub ready_for_state_store_persistence_preview: bool,
    pub ready_for_store_persistence: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAdapterProjectionFixtureSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterProjectionFixture {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub source_record_id: &'static str,
    pub node_kind: &'static str,
    pub projected_node_id: &'static str,
    pub trace_id: &'static str,
    pub status: &'static str,
    pub projected_collection_ids: Vec<&'static str>,
    pub projected_edge_ids: Vec<&'static str>,
    pub projected_task_result_id: Option<&'static str>,
    pub projected_artifact_ids: Vec<&'static str>,
    pub projected_approval_id: Option<&'static str>,
    pub projected_timeline_event_ids: Vec<&'static str>,
    pub idempotency_key_hash: Option<&'static str>,
    pub redaction_state: &'static str,
    pub required_contract_gates: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterProjectionCollectionCoverage {
    pub id: &'static str,
    pub fixture_ids: Vec<&'static str>,
    pub required_before_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterProjectionInvariant {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterProjectionFixtureSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub store_persistence_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub adapter_projection_enforced: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_adapter_projection_fixture_report()
-> WorkGraphAdapterProjectionFixtureReport {
    let fixtures = work_graph_adapter_projection_fixtures();
    let projected_collections = work_graph_adapter_projection_collection_coverage();
    let invariants = work_graph_adapter_projection_invariants();
    let source_surface_count = fixtures
        .iter()
        .map(|fixture| fixture.source_surface_id)
        .collect::<std::collections::BTreeSet<_>>()
        .len();

    WorkGraphAdapterProjectionFixtureReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_GATE,
        schema_version: WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_SCHEMA_VERSION,
        preview_mode: "read_only_adapter_projection_fixture_no_persistence",
        fixture_count: fixtures.len(),
        source_surface_count,
        projected_collection_count: projected_collections.len(),
        invariant_count: invariants.len(),
        fixtures,
        projected_collections,
        invariants,
        recommended_next_gate: WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_RECOMMENDED_NEXT_GATE,
        ready_for_state_store_persistence_preview: true,
        ready_for_store_persistence: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAdapterProjectionFixtureSideEffects::none(),
    }
}

pub fn work_graph_adapter_projection_fixtures() -> Vec<WorkGraphAdapterProjectionFixture> {
    vec![
        fixture(
            "update_plan_step_projection",
            "update_plan_tool",
            "turn-plan-step-preview-0",
            "plan_step",
            "wg-node-plan-step-turn-plan-step-preview-0",
            "wg-trace-preview-plan-001",
            "pending",
            vec!["nodes", "edges", "timelineEvents"],
            vec!["wg-edge-plan-step-depends-on-root"],
            None,
            Vec::new(),
            None,
            vec!["wg-event-plan-step-observed-001"],
            None,
        ),
        fixture(
            "plan_mode_proposal_block_projection",
            "plan_mode_proposed_plan_blocks",
            "plan-mode-proposal-block-preview-001",
            "plan_step",
            "wg-node-plan-step-plan-mode-proposal-block-preview-001",
            "wg-trace-preview-plan-mode-001",
            "pending",
            vec!["nodes", "edges", "timelineEvents"],
            vec!["wg-edge-plan-mode-proposal-replaces-plan-step-001"],
            None,
            Vec::new(),
            None,
            vec!["wg-event-plan-mode-proposal-observed-001"],
            Some("hash:plan-mode-proposal-block-preview-001"),
        ),
        fixture(
            "app_server_turn_plan_notification_projection",
            "app_server_turn_plan_notification",
            "app-server-turn-plan-notification-preview-001",
            "plan_step",
            "wg-node-plan-step-app-server-turn-plan-notification-preview-001",
            "wg-trace-preview-app-server-plan-001",
            "running",
            vec!["nodes", "edges", "timelineEvents"],
            vec!["wg-edge-app-server-plan-notification-updates-plan-001"],
            None,
            Vec::new(),
            None,
            vec!["wg-event-app-server-plan-step-observed-001"],
            Some("hash:app-server-turn-plan-notification-preview-001"),
        ),
        fixture(
            "multi_agent_thread_spawn_projection",
            "multi_agent_v2_thread_spawn",
            "thread-spawn-edge-preview-001",
            "agent_task",
            "wg-node-agent-task-thread-spawn-edge-preview-001",
            "wg-trace-preview-agent-001",
            "queued",
            vec!["nodes", "edges", "taskResults", "timelineEvents"],
            vec!["wg-edge-agent-task-spawned-by-plan-001"],
            Some("wg-result-agent-task-thread-spawn-edge-preview-001"),
            Vec::new(),
            None,
            vec!["wg-event-agent-task-spawned-001"],
            None,
        ),
        fixture(
            "multi_agent_mailbox_wait_projection",
            "multi_agent_v2_mailbox_wait",
            "mailbox-wait-preview-001",
            "agent_task",
            "wg-node-agent-task-mailbox-wait-preview-001",
            "wg-trace-preview-mailbox-wait-001",
            "running",
            vec!["nodes", "edges", "timelineEvents"],
            vec!["wg-edge-mailbox-wait-unblocks-agent-task-001"],
            None,
            Vec::new(),
            None,
            vec!["wg-event-mailbox-progress-observed-001"],
            Some("hash:mailbox-wait-preview-001"),
        ),
        fixture(
            "multi_agent_reducer_result_projection",
            "hepta_runtime_multi_agent_reducer",
            "multi-agent-reducer-preview-001",
            "agent_task",
            "wg-node-agent-task-multi-agent-reducer-preview-001",
            "wg-trace-preview-reducer-001",
            "succeeded",
            vec!["nodes", "edges", "taskResults", "timelineEvents"],
            vec![
                "wg-edge-multi-agent-reducer-reduces-agent-results-001",
                "wg-edge-multi-agent-reducer-verifies-consensus-001",
            ],
            Some("wg-result-multi-agent-reducer-preview-001"),
            Vec::new(),
            None,
            vec!["wg-event-multi-agent-reducer-result-001"],
            Some("hash:multi-agent-reducer-preview-001"),
        ),
        fixture(
            "agent_job_item_result_projection",
            "agent_jobs_batch_workers",
            "agent-job-item-preview-001",
            "worker_task",
            "wg-node-worker-task-agent-job-item-preview-001",
            "wg-trace-preview-agent-job-001",
            "running",
            vec!["nodes", "taskResults", "timelineEvents"],
            Vec::new(),
            Some("wg-result-agent-job-item-preview-001"),
            Vec::new(),
            None,
            vec!["wg-event-task-result-agent-job-001"],
            None,
        ),
        fixture(
            "task_board_lease_claim_projection",
            "hepta_runtime_task_board",
            "task-board-lease-preview-001",
            "worker_task",
            "wg-node-worker-task-board-lease-preview-001",
            "wg-trace-preview-task-board-001",
            "queued",
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "wg-edge-task-board-depends-on-parent-001",
                "wg-edge-task-board-lease-claim-001",
            ],
            None,
            Vec::new(),
            None,
            vec!["wg-event-task-board-lease-observed-001"],
            Some("hash:task-board-lease-preview-001"),
        ),
        fixture(
            "runtime_worker_task_artifact_projection",
            "hepta_runtime_worker_tasks",
            "worker-task-preview-001",
            "worker_task",
            "wg-node-worker-task-preview-001",
            "wg-trace-preview-worker-001",
            "succeeded",
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            Vec::new(),
            Some("wg-result-worker-task-preview-001"),
            vec!["wg-artifact-worker-task-preview-001"],
            None,
            vec![
                "wg-event-worker-artifact-produced-001",
                "wg-event-worker-task-result-001",
            ],
            None,
        ),
        fixture(
            "scheduler_run_admission_projection",
            "hepta_runtime_scheduler_store",
            "scheduler-run-preview-001",
            "scheduler_run",
            "wg-node-scheduler-run-preview-001",
            "wg-trace-preview-scheduler-001",
            "blocked",
            vec!["nodes", "edges", "taskResults", "timelineEvents"],
            vec!["wg-edge-scheduler-run-blocked-by-approval-001"],
            Some("wg-result-scheduler-run-preview-001"),
            Vec::new(),
            None,
            vec!["wg-event-scheduler-admission-denied-001"],
            Some("hash:idempotency-preview-scheduler-001"),
        ),
        fixture(
            "approval_broker_human_approval_projection",
            "hepta_runtime_approval_broker",
            "approval-request-preview-001",
            "human_approval",
            "wg-node-human-approval-preview-001",
            "wg-trace-preview-approval-001",
            "pending",
            vec!["nodes", "approvals", "timelineEvents"],
            Vec::new(),
            None,
            Vec::new(),
            Some("wg-approval-preview-001"),
            vec!["wg-event-human-approval-requested-001"],
            None,
        ),
        fixture(
            "agent_harness_external_handoff_projection",
            "hepta_runtime_agent_harness",
            "agent-harness-handoff-preview-001",
            "external_handoff",
            "wg-node-external-handoff-preview-001",
            "wg-trace-preview-handoff-001",
            "blocked",
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            vec!["wg-edge-external-handoff-blocked-by-approval-001"],
            Some("wg-result-agent-harness-handoff-preview-001"),
            vec!["wg-artifact-handoff-preview-001"],
            None,
            vec!["wg-event-external-handoff-observed-001"],
            Some("hash:handoff-preview-001"),
        ),
    ]
}

pub fn work_graph_adapter_projection_collection_coverage()
-> Vec<WorkGraphAdapterProjectionCollectionCoverage> {
    vec![
        collection_coverage(
            "nodes",
            vec![
                "update_plan_step_projection",
                "plan_mode_proposal_block_projection",
                "app_server_turn_plan_notification_projection",
                "multi_agent_thread_spawn_projection",
                "multi_agent_mailbox_wait_projection",
                "multi_agent_reducer_result_projection",
                "agent_job_item_result_projection",
                "task_board_lease_claim_projection",
                "runtime_worker_task_artifact_projection",
                "scheduler_run_admission_projection",
                "approval_broker_human_approval_projection",
                "agent_harness_external_handoff_projection",
            ],
        ),
        collection_coverage(
            "edges",
            vec![
                "update_plan_step_projection",
                "plan_mode_proposal_block_projection",
                "app_server_turn_plan_notification_projection",
                "multi_agent_thread_spawn_projection",
                "multi_agent_mailbox_wait_projection",
                "multi_agent_reducer_result_projection",
                "task_board_lease_claim_projection",
                "scheduler_run_admission_projection",
                "agent_harness_external_handoff_projection",
            ],
        ),
        collection_coverage(
            "taskResults",
            vec![
                "multi_agent_thread_spawn_projection",
                "multi_agent_reducer_result_projection",
                "agent_job_item_result_projection",
                "runtime_worker_task_artifact_projection",
                "scheduler_run_admission_projection",
                "agent_harness_external_handoff_projection",
            ],
        ),
        collection_coverage(
            "artifacts",
            vec![
                "runtime_worker_task_artifact_projection",
                "agent_harness_external_handoff_projection",
            ],
        ),
        collection_coverage(
            "approvals",
            vec!["approval_broker_human_approval_projection"],
        ),
        collection_coverage(
            "timelineEvents",
            vec![
                "update_plan_step_projection",
                "plan_mode_proposal_block_projection",
                "app_server_turn_plan_notification_projection",
                "multi_agent_thread_spawn_projection",
                "multi_agent_mailbox_wait_projection",
                "multi_agent_reducer_result_projection",
                "agent_job_item_result_projection",
                "task_board_lease_claim_projection",
                "runtime_worker_task_artifact_projection",
                "scheduler_run_admission_projection",
                "approval_broker_human_approval_projection",
                "agent_harness_external_handoff_projection",
            ],
        ),
    ]
}

pub fn work_graph_adapter_projection_invariants() -> Vec<WorkGraphAdapterProjectionInvariant> {
    vec![
        invariant(
            "fixtures_use_deterministic_redacted_ids",
            "fixtures use stable synthetic ids and hashes instead of raw payloads",
        ),
        invariant(
            "every_fixture_has_trace_and_node_id",
            "projection cannot be accepted without traceId and projected node identity",
        ),
        invariant(
            "collection_coverage_includes_nodes_and_timeline",
            "every fixture must project at least a node and a redacted timeline event",
        ),
        invariant(
            "task_result_fixtures_use_task_result_contract",
            "worker-like fixtures must point at TaskResult ids before persistence preview",
        ),
        invariant(
            "approval_and_external_handoff_are_preview_only",
            "human approval and external handoff fixtures cannot record decisions or deliver",
        ),
        invariant(
            "fixture_gate_does_not_persist_or_enforce",
            "this gate cannot write graph state or enable adapter enforcement",
        ),
    ]
}

impl WorkGraphAdapterProjectionFixtureSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            store_persistence_enabled: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            adapter_projection_enforced: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn fixture(
    id: &'static str,
    source_surface_id: &'static str,
    source_record_id: &'static str,
    node_kind: &'static str,
    projected_node_id: &'static str,
    trace_id: &'static str,
    status: &'static str,
    projected_collection_ids: Vec<&'static str>,
    projected_edge_ids: Vec<&'static str>,
    projected_task_result_id: Option<&'static str>,
    projected_artifact_ids: Vec<&'static str>,
    projected_approval_id: Option<&'static str>,
    projected_timeline_event_ids: Vec<&'static str>,
    idempotency_key_hash: Option<&'static str>,
) -> WorkGraphAdapterProjectionFixture {
    WorkGraphAdapterProjectionFixture {
        id,
        source_surface_id,
        source_record_id,
        node_kind,
        projected_node_id,
        trace_id,
        status,
        projected_collection_ids,
        projected_edge_ids,
        projected_task_result_id,
        projected_artifact_ids,
        projected_approval_id,
        projected_timeline_event_ids,
        idempotency_key_hash,
        redaction_state: "redacted_refs_only",
        required_contract_gates: required_contract_gates(),
        persistence_enabled: false,
        enforcement_enabled: false,
    }
}

fn required_contract_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
    ]
}

fn collection_coverage(
    id: &'static str,
    fixture_ids: Vec<&'static str>,
) -> WorkGraphAdapterProjectionCollectionCoverage {
    WorkGraphAdapterProjectionCollectionCoverage {
        id,
        fixture_ids,
        required_before_persistence: true,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphAdapterProjectionInvariant {
    WorkGraphAdapterProjectionInvariant {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_projection_fixture_declares_source_surfaces_and_node_kinds() {
        let report = hepta_work_graph_adapter_projection_fixture_report();
        let source_surface_ids = report
            .fixtures
            .iter()
            .map(|fixture| fixture.source_surface_id)
            .collect::<Vec<_>>();
        let node_kinds = report
            .fixtures
            .iter()
            .map(|fixture| fixture.node_kind)
            .collect::<Vec<_>>();

        assert_eq!(
            source_surface_ids,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_approval_broker",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(
            node_kinds,
            [
                "plan_step",
                "plan_step",
                "plan_step",
                "agent_task",
                "agent_task",
                "agent_task",
                "worker_task",
                "worker_task",
                "worker_task",
                "scheduler_run",
                "human_approval",
                "external_handoff",
            ]
        );
        assert_eq!(report.fixture_count, 12);
        assert_eq!(report.source_surface_count, 12);
    }

    #[test]
    fn adapter_projection_fixture_covers_unified_store_collections() {
        let report = hepta_work_graph_adapter_projection_fixture_report();
        let collection_ids = report
            .projected_collections
            .iter()
            .map(|collection| collection.id)
            .collect::<Vec<_>>();

        assert_eq!(
            collection_ids,
            [
                "nodes",
                "edges",
                "taskResults",
                "artifacts",
                "approvals",
                "timelineEvents",
            ]
        );
        assert_eq!(report.projected_collection_count, 6);
        assert!(report.fixtures.iter().all(|fixture| {
            fixture.projected_collection_ids.contains(&"nodes")
                && fixture.projected_collection_ids.contains(&"timelineEvents")
        }));
    }

    #[test]
    fn adapter_projection_fixture_requires_prior_contract_gates() {
        let report = hepta_work_graph_adapter_projection_fixture_report();

        assert!(
            report
                .fixtures
                .iter()
                .all(|fixture| fixture.required_contract_gates == required_contract_gates())
        );
        assert_eq!(report.invariant_count, 6);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn adapter_projection_fixture_keeps_persistence_and_execution_disabled() {
        let report = hepta_work_graph_adapter_projection_fixture_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAdapterProjectionFixtureSideEffects::none()
        );
        assert!(report.ready_for_state_store_persistence_preview);
        assert!(!report.ready_for_store_persistence);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .fixtures
                .iter()
                .all(|fixture| !fixture.persistence_enabled && !fixture.enforcement_enabled)
        );
    }
}
