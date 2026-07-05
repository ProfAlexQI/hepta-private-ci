use std::collections::BTreeSet;

use serde::Serialize;

pub const WORK_GRAPH_STATE_STORE_PERSISTENCE_PREVIEW_GATE: &str =
    "hepta_work_graph_state_store_persistence_preview_gate";
pub const WORK_GRAPH_STATE_STORE_PERSISTENCE_SCHEMA_VERSION: &str =
    "work_graph_state_store_persistence_preview_v1";
pub const WORK_GRAPH_STATE_STORE_PERSISTENCE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_replay_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStateStorePersistencePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub wal_operation_count: usize,
    pub checkpoint_contract_count: usize,
    pub idempotency_guard_count: usize,
    pub readback_probe_count: usize,
    pub invariant_count: usize,
    pub source_surface_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub wal_operations: Vec<WorkGraphPersistenceWalOperationPreview>,
    pub checkpoint_contracts: Vec<WorkGraphCheckpointContractPreview>,
    pub idempotency_guards: Vec<WorkGraphIdempotencyGuardPreview>,
    pub readback_probes: Vec<WorkGraphReadbackProbePreview>,
    pub invariants: Vec<WorkGraphPersistenceInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_replay_readback_preview: bool,
    pub ready_for_store_persistence: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphStateStorePersistencePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceWalOperationPreview {
    pub id: &'static str,
    pub record_kind: &'static str,
    pub required_fields: Vec<&'static str>,
    pub ordering_rule: &'static str,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCheckpointContractPreview {
    pub id: &'static str,
    pub included_collection_ids: Vec<&'static str>,
    pub required_hash_fields: Vec<&'static str>,
    pub write_policy: &'static str,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyGuardPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub collision_policy: &'static str,
    pub required_before_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReadbackProbePreview {
    pub id: &'static str,
    pub target_collection_id: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub promotion_blocker: &'static str,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphStateStorePersistencePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub recovery_performed: bool,
    pub idempotency_index_mutated: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_state_store_persistence_preview_report()
-> WorkGraphStateStorePersistencePreviewReport {
    let wal_operations = work_graph_state_store_persistence_wal_operations();
    let checkpoint_contracts = work_graph_state_store_checkpoint_contracts();
    let idempotency_guards = work_graph_state_store_idempotency_guards();
    let readback_probes = work_graph_state_store_readback_probes();
    let invariants = work_graph_state_store_persistence_invariants();
    let source_surface_count = idempotency_guards
        .iter()
        .map(|guard| guard.source_surface_id)
        .collect::<BTreeSet<_>>()
        .len();

    WorkGraphStateStorePersistencePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_STATE_STORE_PERSISTENCE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_STATE_STORE_PERSISTENCE_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_contract_preview_no_store_writes",
        wal_operation_count: wal_operations.len(),
        checkpoint_contract_count: checkpoint_contracts.len(),
        idempotency_guard_count: idempotency_guards.len(),
        readback_probe_count: readback_probes.len(),
        invariant_count: invariants.len(),
        source_surface_count,
        required_prior_gates: work_graph_state_store_persistence_required_prior_gates(),
        wal_operations,
        checkpoint_contracts,
        idempotency_guards,
        readback_probes,
        invariants,
        recommended_next_gate: WORK_GRAPH_STATE_STORE_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        ready_for_replay_readback_preview: true,
        ready_for_store_persistence: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphStateStorePersistencePreviewSideEffects::none(),
    }
}

pub fn work_graph_state_store_persistence_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
    ]
}

pub fn work_graph_state_store_persistence_wal_operations()
-> Vec<WorkGraphPersistenceWalOperationPreview> {
    vec![
        wal_operation(
            "preview_append_node_record",
            "node",
            vec!["traceId", "nodeId", "nodeKind", "status", "sourceSurfaceId"],
            "append_after_projection_validation_before_index_visibility",
        ),
        wal_operation(
            "preview_append_edge_record",
            "edge",
            vec!["traceId", "edgeId", "edgeKind", "fromNodeId", "toNodeId"],
            "append_after_endpoint_nodes_are_known",
        ),
        wal_operation(
            "preview_append_task_result_record",
            "task_result",
            vec!["traceId", "taskId", "status", "summaryHash", "evidenceRefs"],
            "append_after_task_result_contract_validation",
        ),
        wal_operation(
            "preview_append_artifact_record",
            "artifact",
            vec!["traceId", "artifactId", "producerNodeId", "artifactHash"],
            "append_after_redaction_and_hashing",
        ),
        wal_operation(
            "preview_append_approval_record",
            "approval",
            vec![
                "traceId",
                "approvalId",
                "operatorScope",
                "status",
                "expiresAtUnixMs",
            ],
            "append_without_recording_live_operator_decisions",
        ),
        wal_operation(
            "preview_append_timeline_event_record",
            "timeline_event",
            vec![
                "traceId",
                "eventId",
                "eventKind",
                "nodeId",
                "redactionState",
            ],
            "append_after_source_record_projection",
        ),
    ]
}

pub fn work_graph_state_store_checkpoint_contracts() -> Vec<WorkGraphCheckpointContractPreview> {
    vec![
        checkpoint_contract(
            "preview_full_graph_checkpoint",
            vec![
                "nodes",
                "edges",
                "taskResults",
                "artifacts",
                "approvals",
                "timelineEvents",
            ],
            vec!["walHeadHash", "checkpointHash", "collectionMerkleRoot"],
            "disabled_until_wal_replay_and_readback_gate_passes",
        ),
        checkpoint_contract(
            "preview_trace_checkpoint",
            vec!["nodes", "edges", "taskResults", "timelineEvents"],
            vec!["traceId", "traceHash", "timelineHash"],
            "disabled_until_trace_replay_is_deterministic",
        ),
        checkpoint_contract(
            "preview_artifact_checkpoint",
            vec!["artifacts", "timelineEvents"],
            vec!["artifactHash", "producerNodeHash", "redactionHash"],
            "disabled_until_artifact_redaction_readback_is_verified",
        ),
        checkpoint_contract(
            "preview_approval_checkpoint",
            vec!["approvals", "timelineEvents"],
            vec!["approvalHash", "operatorScopeHash", "expiryHash"],
            "disabled_until_operator_authority_and_expiry_are_enforced",
        ),
    ]
}

pub fn work_graph_state_store_idempotency_guards() -> Vec<WorkGraphIdempotencyGuardPreview> {
    vec![
        idempotency_guard(
            "update_plan_projection_idempotency",
            "update_plan_tool",
            vec!["turnId", "stepIndex", "traceId"],
            "same_key_replays_same_plan_step_node",
        ),
        idempotency_guard(
            "plan_mode_projection_idempotency",
            "plan_mode_proposed_plan_blocks",
            vec!["proposalId", "blockIndex", "traceId"],
            "same_plan_mode_block_replays_same_plan_step_node",
        ),
        idempotency_guard(
            "app_server_turn_plan_notification_idempotency",
            "app_server_turn_plan_notification",
            vec!["turnId", "notificationSeq", "traceId"],
            "same_plan_notification_replays_same_observed_plan_step",
        ),
        idempotency_guard(
            "multi_agent_spawn_projection_idempotency",
            "multi_agent_v2_thread_spawn",
            vec!["parentThreadId", "childThreadId", "roleId"],
            "same_child_thread_cannot_spawn_duplicate_agent_task",
        ),
        idempotency_guard(
            "multi_agent_mailbox_wait_projection_idempotency",
            "multi_agent_v2_mailbox_wait",
            vec!["parentThreadId", "mailboxSeq", "traceId"],
            "same_mailbox_progress_sequence_replays_same_wait_event",
        ),
        idempotency_guard(
            "multi_agent_reducer_projection_idempotency",
            "hepta_runtime_multi_agent_reducer",
            vec!["reducerRunId", "agentPath", "traceId"],
            "same_reducer_run_replays_same_terminal_result",
        ),
        idempotency_guard(
            "agent_job_result_projection_idempotency",
            "agent_jobs_batch_workers",
            vec!["jobId", "itemId", "attempt"],
            "same_job_item_attempt_replays_same_worker_task_result",
        ),
        idempotency_guard(
            "task_board_projection_idempotency",
            "hepta_runtime_task_board",
            vec!["taskId", "claimTokenHash", "traceId"],
            "same_task_board_claim_replays_same_worker_task_node",
        ),
        idempotency_guard(
            "worker_task_projection_idempotency",
            "hepta_runtime_worker_tasks",
            vec!["workerTaskId", "attempt", "artifactHash"],
            "same_worker_attempt_replays_same_artifact_and_result",
        ),
        idempotency_guard(
            "scheduler_run_projection_idempotency",
            "hepta_runtime_scheduler_store",
            vec!["schedulerRunId", "leaseId", "admissionDecision"],
            "same_scheduler_decision_cannot_double_promote_or_double_block",
        ),
        idempotency_guard(
            "approval_projection_idempotency",
            "hepta_runtime_approval_broker",
            vec!["approvalId", "operatorScope", "requestHash"],
            "same_approval_request_replays_same_pending_approval_node",
        ),
        idempotency_guard(
            "agent_harness_handoff_projection_idempotency",
            "hepta_runtime_agent_harness",
            vec!["handoffId", "target", "payloadHash"],
            "same_handoff_replays_same_blocked_external_handoff_node",
        ),
    ]
}

pub fn work_graph_state_store_readback_probes() -> Vec<WorkGraphReadbackProbePreview> {
    vec![
        readback_probe(
            "preview_readback_nodes_by_trace",
            "nodes",
            vec!["traceId", "expectedNodeIds"],
            "node_visibility_required_before_edge_or_status_promotion",
        ),
        readback_probe(
            "preview_readback_edges_by_trace",
            "edges",
            vec!["traceId", "expectedEdgeIds"],
            "edge_visibility_required_before_dependency_resolution",
        ),
        readback_probe(
            "preview_readback_task_results_by_status",
            "taskResults",
            vec!["status", "taskId", "traceId"],
            "terminal_result_visibility_required_before_reducer_promotion",
        ),
        readback_probe(
            "preview_readback_artifacts_by_producer",
            "artifacts",
            vec!["producerNodeId", "artifactHash"],
            "artifact_visibility_required_before_handoff_or_verifier_promotion",
        ),
        readback_probe(
            "preview_readback_approvals_by_scope",
            "approvals",
            vec!["operatorScope", "approvalId", "expiresAtUnixMs"],
            "approval_visibility_required_before_scheduler_unblock",
        ),
        readback_probe(
            "preview_readback_timeline_by_trace",
            "timelineEvents",
            vec!["traceId", "eventKind", "redactionState"],
            "timeline_visibility_required_before_operator_audit_or_replay",
        ),
    ]
}

pub fn work_graph_state_store_persistence_invariants() -> Vec<WorkGraphPersistenceInvariantPreview>
{
    vec![
        invariant(
            "wal_records_are_append_only",
            "state persistence must append immutable records before indexes observe them",
        ),
        invariant(
            "checkpoints_are_derived_from_wal",
            "checkpoint snapshots cannot be authoritative without replayable WAL evidence",
        ),
        invariant(
            "idempotency_index_precedes_write",
            "duplicate source records must be detected before a persistence write is allowed",
        ),
        invariant(
            "readback_precedes_promotion",
            "status, delivery, approval, and scheduler promotion require readback evidence",
        ),
        invariant(
            "payloads_are_redacted_before_persistence",
            "raw prompts, tool payloads, credentials, and private transcripts are never persisted",
        ),
        invariant(
            "recovery_is_preview_only",
            "this gate describes recovery contracts but cannot replay or mutate runtime state",
        ),
        invariant(
            "persistence_preview_has_no_side_effects",
            "the preview cannot write graph state, checkpoints, WAL records, or indexes",
        ),
    ]
}

impl WorkGraphStateStorePersistencePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            recovery_performed: false,
            idempotency_index_mutated: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn wal_operation(
    id: &'static str,
    record_kind: &'static str,
    required_fields: Vec<&'static str>,
    ordering_rule: &'static str,
) -> WorkGraphPersistenceWalOperationPreview {
    WorkGraphPersistenceWalOperationPreview {
        id,
        record_kind,
        required_fields,
        ordering_rule,
        mutates_store: false,
    }
}

fn checkpoint_contract(
    id: &'static str,
    included_collection_ids: Vec<&'static str>,
    required_hash_fields: Vec<&'static str>,
    write_policy: &'static str,
) -> WorkGraphCheckpointContractPreview {
    WorkGraphCheckpointContractPreview {
        id,
        included_collection_ids,
        required_hash_fields,
        write_policy,
        mutates_store: false,
    }
}

fn idempotency_guard(
    id: &'static str,
    source_surface_id: &'static str,
    key_fields: Vec<&'static str>,
    collision_policy: &'static str,
) -> WorkGraphIdempotencyGuardPreview {
    WorkGraphIdempotencyGuardPreview {
        id,
        source_surface_id,
        key_fields,
        collision_policy,
        required_before_persistence: true,
    }
}

fn readback_probe(
    id: &'static str,
    target_collection_id: &'static str,
    required_inputs: Vec<&'static str>,
    promotion_blocker: &'static str,
) -> WorkGraphReadbackProbePreview {
    WorkGraphReadbackProbePreview {
        id,
        target_collection_id,
        required_inputs,
        promotion_blocker,
        mutates_store: false,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphPersistenceInvariantPreview {
    WorkGraphPersistenceInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_store_persistence_preview_declares_wal_and_checkpoints() {
        let report = hepta_work_graph_state_store_persistence_preview_report();
        let wal_ids = report
            .wal_operations
            .iter()
            .map(|operation| operation.id)
            .collect::<Vec<_>>();
        let checkpoint_ids = report
            .checkpoint_contracts
            .iter()
            .map(|checkpoint| checkpoint.id)
            .collect::<Vec<_>>();

        assert_eq!(
            wal_ids,
            [
                "preview_append_node_record",
                "preview_append_edge_record",
                "preview_append_task_result_record",
                "preview_append_artifact_record",
                "preview_append_approval_record",
                "preview_append_timeline_event_record",
            ]
        );
        assert_eq!(
            checkpoint_ids,
            [
                "preview_full_graph_checkpoint",
                "preview_trace_checkpoint",
                "preview_artifact_checkpoint",
                "preview_approval_checkpoint",
            ]
        );
        assert_eq!(report.wal_operation_count, 6);
        assert_eq!(report.checkpoint_contract_count, 4);
    }

    #[test]
    fn state_store_persistence_preview_guards_adapter_sources() {
        let report = hepta_work_graph_state_store_persistence_preview_report();
        let source_surface_ids = report
            .idempotency_guards
            .iter()
            .map(|guard| guard.source_surface_id)
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
        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.idempotency_guard_count, 12);
        assert!(
            report
                .idempotency_guards
                .iter()
                .all(|guard| guard.required_before_persistence)
        );
    }

    #[test]
    fn state_store_persistence_preview_requires_readback_for_all_collections() {
        let report = hepta_work_graph_state_store_persistence_preview_report();
        let readback_collection_ids = report
            .readback_probes
            .iter()
            .map(|probe| probe.target_collection_id)
            .collect::<Vec<_>>();

        assert_eq!(
            readback_collection_ids,
            [
                "nodes",
                "edges",
                "taskResults",
                "artifacts",
                "approvals",
                "timelineEvents",
            ]
        );
        assert_eq!(report.readback_probe_count, 6);
        assert!(
            report
                .readback_probes
                .iter()
                .all(|probe| !probe.mutates_store)
        );
    }

    #[test]
    fn state_store_persistence_preview_keeps_writes_disabled() {
        let report = hepta_work_graph_state_store_persistence_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphStateStorePersistencePreviewSideEffects::none()
        );
        assert!(report.ready_for_replay_readback_preview);
        assert!(!report.ready_for_store_persistence);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .wal_operations
                .iter()
                .all(|operation| !operation.mutates_store)
        );
        assert!(
            report
                .checkpoint_contracts
                .iter()
                .all(|checkpoint| !checkpoint.mutates_store)
        );
    }

    #[test]
    fn state_store_persistence_preview_requires_prior_gates() {
        let report = hepta_work_graph_state_store_persistence_preview_report();

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
            ]
        );
        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_STATE_STORE_PERSISTENCE_RECOMMENDED_NEXT_GATE
        );
    }
}
