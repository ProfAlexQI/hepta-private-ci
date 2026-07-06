use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_contracts;
use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_routes;

pub const WORK_GRAPH_REPLAY_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_replay_readback_preview_gate";
pub const WORK_GRAPH_REPLAY_READBACK_SCHEMA_VERSION: &str = "work_graph_replay_readback_preview_v1";
pub const WORK_GRAPH_REPLAY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_idempotency_readback_adapter_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub replay_stage_count: usize,
    pub readback_assertion_count: usize,
    pub drift_detector_count: usize,
    pub recovery_preview_count: usize,
    pub invariant_count: usize,
    pub append_only_event_contract_count: usize,
    pub append_only_source_route_count: usize,
    pub event_replay_plan_count: usize,
    pub source_readback_gap_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub replay_stages: Vec<WorkGraphReplayStagePreview>,
    pub readback_assertions: Vec<WorkGraphReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphDriftDetectorPreview>,
    pub recovery_previews: Vec<WorkGraphRecoveryPreview>,
    pub invariants: Vec<WorkGraphReplayReadbackInvariantPreview>,
    pub append_only_event_replay_plans: Vec<WorkGraphAppendOnlyEventReplayPlanPreview>,
    pub source_readback_gaps: Vec<WorkGraphReplayReadbackSourceGapPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_promotion_precondition_preview: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphReplayReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayStagePreview {
    pub id: &'static str,
    pub input_contract_ids: Vec<&'static str>,
    pub output_contract_ids: Vec<&'static str>,
    pub failure_mode: &'static str,
    pub executes_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReadbackAssertionPreview {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub evidence_fields: Vec<&'static str>,
    pub promotion_gate: &'static str,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDriftDetectorPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRecoveryPreview {
    pub id: &'static str,
    pub trigger_detector_ids: Vec<&'static str>,
    pub recovery_action: &'static str,
    pub requires_operator_approval: bool,
    pub executes_recovery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayReadbackInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventReplayPlanPreview {
    pub id: &'static str,
    pub event_contract_id: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub expected_collection_ids: Vec<&'static str>,
    pub deterministic_replay_key_fields: Vec<&'static str>,
    pub readback_assertion_ids: Vec<&'static str>,
    pub blocking_source_surface_ids: Vec<&'static str>,
    pub blocking_reason_ids: Vec<&'static str>,
    pub executes_replay: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayReadbackSourceGapPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub missing_capability: &'static str,
    pub affected_event_contract_ids: Vec<&'static str>,
    pub required_before_replay_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphReplayReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub event_record_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_replayed: bool,
    pub checkpoint_loaded: bool,
    pub idempotency_index_mutated: bool,
    pub readback_performed: bool,
    pub drift_state_persisted: bool,
    pub recovery_performed: bool,
    pub promotion_performed: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub adapter_projection_enforced: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_replay_readback_preview_report() -> WorkGraphReplayReadbackPreviewReport {
    let replay_stages = work_graph_replay_readback_stages();
    let readback_assertions = work_graph_replay_readback_assertions();
    let drift_detectors = work_graph_replay_readback_drift_detectors();
    let recovery_previews = work_graph_replay_readback_recovery_previews();
    let invariants = work_graph_replay_readback_invariants();
    let append_only_event_contracts = work_graph_append_only_event_contracts();
    let append_only_source_routes = work_graph_append_only_event_routes();
    let append_only_event_replay_plans = work_graph_replay_readback_event_plans();
    let source_readback_gaps = work_graph_replay_readback_source_gaps();

    WorkGraphReplayReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_REPLAY_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_REPLAY_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_replay_readback_contract_preview_no_replay",
        replay_stage_count: replay_stages.len(),
        readback_assertion_count: readback_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        recovery_preview_count: recovery_previews.len(),
        invariant_count: invariants.len(),
        append_only_event_contract_count: append_only_event_contracts.len(),
        append_only_source_route_count: append_only_source_routes.len(),
        event_replay_plan_count: append_only_event_replay_plans.len(),
        source_readback_gap_count: source_readback_gaps.len(),
        required_prior_gates: work_graph_replay_readback_required_prior_gates(),
        replay_stages,
        readback_assertions,
        drift_detectors,
        recovery_previews,
        invariants,
        append_only_event_replay_plans,
        source_readback_gaps,
        recommended_next_gate: WORK_GRAPH_REPLAY_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_promotion_precondition_preview: true,
        ready_for_replay_execution: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphReplayReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_replay_readback_required_prior_gates() -> Vec<&'static str> {
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
    ]
}

pub fn work_graph_replay_readback_stages() -> Vec<WorkGraphReplayStagePreview> {
    vec![
        replay_stage(
            "preview_load_wal_head",
            vec!["walHeadHash", "walSegmentManifest"],
            vec!["orderedWalSegmentRefs"],
            "missing_or_untrusted_wal_manifest_blocks_replay",
        ),
        replay_stage(
            "preview_validate_wal_hash_chain",
            vec!["orderedWalSegmentRefs", "previousRecordHash"],
            vec!["validatedWalHeadHash"],
            "hash_mismatch_blocks_checkpoint_compare",
        ),
        replay_stage(
            "preview_apply_idempotency_window",
            vec!["validatedWalHeadHash", "sourceRecordKeys"],
            vec!["dedupedRecordSet"],
            "duplicate_key_collision_blocks_collection_materialization",
        ),
        replay_stage(
            "preview_materialize_collections",
            vec!["dedupedRecordSet", "collectionSchemas"],
            vec!["materializedCollectionHashes"],
            "schema_or_redaction_mismatch_blocks_readback",
        ),
        replay_stage(
            "preview_compare_checkpoint",
            vec!["materializedCollectionHashes", "checkpointHash"],
            vec!["checkpointComparisonResult"],
            "checkpoint_drift_blocks_promotion",
        ),
        replay_stage(
            "preview_emit_readback_report",
            vec!["checkpointComparisonResult", "readbackAssertions"],
            vec!["redactedReadbackEvidenceRefs"],
            "missing_readback_evidence_blocks_operator_summary",
        ),
    ]
}

pub fn work_graph_replay_readback_assertions() -> Vec<WorkGraphReadbackAssertionPreview> {
    vec![
        readback_assertion(
            "assert_nodes_readback_matches_wal",
            "nodes",
            vec!["traceId", "expectedNodeIds", "validatedWalHeadHash"],
            vec!["nodeCount", "nodeHash", "missingNodeIds"],
            "block_node_status_promotion_until_readback_matches",
        ),
        readback_assertion(
            "assert_edges_readback_matches_wal",
            "edges",
            vec!["traceId", "expectedEdgeIds", "validatedWalHeadHash"],
            vec!["edgeCount", "edgeHash", "missingEdgeIds"],
            "block_dependency_resolution_until_readback_matches",
        ),
        readback_assertion(
            "assert_task_results_readback_matches_wal",
            "taskResults",
            vec!["traceId", "taskId", "status", "validatedWalHeadHash"],
            vec!["taskResultHash", "terminalStatusObserved", "evidenceRefs"],
            "block_reducer_promotion_until_task_result_readback_matches",
        ),
        readback_assertion(
            "assert_artifacts_readback_matches_wal",
            "artifacts",
            vec!["producerNodeId", "artifactHash", "validatedWalHeadHash"],
            vec!["artifactCount", "artifactHash", "redactionState"],
            "block_handoff_until_artifact_readback_matches",
        ),
        readback_assertion(
            "assert_approvals_readback_matches_wal",
            "approvals",
            vec!["approvalId", "operatorScope", "expiresAtUnixMs"],
            vec!["approvalHash", "approvalStatus", "operatorScopeHash"],
            "block_scheduler_unblock_until_approval_readback_matches",
        ),
        readback_assertion(
            "assert_timeline_readback_matches_wal",
            "timelineEvents",
            vec!["traceId", "eventKind", "redactionState"],
            vec!["timelineHash", "eventCount", "redactionState"],
            "block_operator_audit_until_timeline_readback_matches",
        ),
    ]
}

pub fn work_graph_replay_readback_drift_detectors() -> Vec<WorkGraphDriftDetectorPreview> {
    vec![
        drift_detector(
            "detect_identity_drift",
            vec!["nodeId", "edgeId", "taskId", "artifactId", "approvalId"],
            "critical",
        ),
        drift_detector(
            "detect_ordering_drift",
            vec!["walOffset", "eventSequence", "parentTraceId"],
            "critical",
        ),
        drift_detector(
            "detect_status_drift",
            vec!["status", "terminalStatusObserved", "promotionGate"],
            "critical",
        ),
        drift_detector(
            "detect_hash_drift",
            vec!["walHeadHash", "checkpointHash", "collectionMerkleRoot"],
            "critical",
        ),
        drift_detector(
            "detect_redaction_drift",
            vec!["redactionState", "payloadHash", "evidenceRefs"],
            "high",
        ),
    ]
}

pub fn work_graph_replay_readback_recovery_previews() -> Vec<WorkGraphRecoveryPreview> {
    vec![
        recovery_preview(
            "preview_quarantine_checkpoint",
            vec!["detect_hash_drift", "detect_ordering_drift"],
            "mark checkpoint unusable and require WAL replay review",
        ),
        recovery_preview(
            "preview_rebuild_projection_indexes",
            vec!["detect_identity_drift", "detect_ordering_drift"],
            "derive indexes from WAL again after operator review",
        ),
        recovery_preview(
            "preview_hold_terminal_promotion",
            vec!["detect_status_drift", "detect_hash_drift"],
            "keep terminal status blocked until readback evidence is repaired",
        ),
        recovery_preview(
            "preview_request_redaction_review",
            vec!["detect_redaction_drift"],
            "require privacy review before any replay evidence is exposed",
        ),
        recovery_preview(
            "preview_require_operator_replay_approval",
            vec![
                "detect_identity_drift",
                "detect_ordering_drift",
                "detect_status_drift",
                "detect_hash_drift",
                "detect_redaction_drift",
            ],
            "operator must approve any future recovery execution path",
        ),
    ]
}

pub fn work_graph_replay_readback_invariants() -> Vec<WorkGraphReplayReadbackInvariantPreview> {
    vec![
        invariant(
            "replay_is_deterministic",
            "the same WAL and checkpoint inputs must yield the same materialized hashes",
        ),
        invariant(
            "readback_is_required_before_promotion",
            "no terminal promotion, scheduler unblock, or handoff can proceed without readback",
        ),
        invariant(
            "drift_blocks_promotion",
            "identity, ordering, status, hash, and redaction drift must block promotion",
        ),
        invariant(
            "recovery_requires_operator_approval",
            "future recovery execution must be explicitly approved and traceable",
        ),
        invariant(
            "readback_evidence_is_redacted",
            "readback evidence stores ids, hashes, and refs instead of raw private payloads",
        ),
        invariant(
            "replay_readback_preview_has_no_side_effects",
            "this gate cannot replay WAL, read graph state, recover, promote, or persist drift",
        ),
    ]
}

pub fn work_graph_replay_readback_event_plans() -> Vec<WorkGraphAppendOnlyEventReplayPlanPreview> {
    let source_routes = work_graph_append_only_event_routes();

    work_graph_append_only_event_contracts()
        .into_iter()
        .map(|contract| {
            let mut blocking_source_surface_ids = BTreeSet::new();
            let mut blocking_reason_ids = BTreeSet::new();

            for route in source_routes.iter().filter(|route| {
                route
                    .emitted_event_kind_ids
                    .iter()
                    .any(|event_kind_id| *event_kind_id == contract.id)
            }) {
                if route.idempotency_guard_id.is_none()
                    || route
                        .blocker_ids
                        .iter()
                        .any(|blocker_id| *blocker_id != "append_only_store_disabled_by_design")
                {
                    blocking_source_surface_ids.insert(route.source_surface_id);
                }

                for blocker_id in &route.blocker_ids {
                    blocking_reason_ids.insert(*blocker_id);
                }
            }

            event_replay_plan(
                replay_plan_id_for_event_contract(contract.id),
                contract.id,
                contract.source_surface_ids,
                contract.target_collection_ids.clone(),
                contract.idempotency_key_fields,
                contract
                    .target_collection_ids
                    .iter()
                    .map(|collection_id| readback_assertion_id_for_collection(collection_id))
                    .collect(),
                blocking_source_surface_ids.into_iter().collect(),
                blocking_reason_ids.into_iter().collect(),
            )
        })
        .collect()
}

pub fn work_graph_replay_readback_source_gaps() -> Vec<WorkGraphReplayReadbackSourceGapPreview> {
    vec![
        source_gap(
            "gap_plan_mode_proposed_plan_blocks_replay_key",
            "plan_mode_proposed_plan_blocks",
            "stable_plan_block_projection_idempotency_guard",
            vec!["plan_step_event_intake"],
            "derive replay key from traceId, turnId, stepIndex, and proposalHash before readback can materialize plan nodes",
        ),
        source_gap(
            "gap_app_server_turn_plan_notification_replay_key",
            "app_server_turn_plan_notification",
            "stable_turn_plan_notification_idempotency_guard",
            vec!["plan_step_event_intake"],
            "derive replay key from traceId, turnId, notification sequence, and proposalHash before app-server plan readback",
        ),
        source_gap(
            "gap_multi_agent_mailbox_delivery_replay_key",
            "multi_agent_v2_mailbox_wait",
            "mailbox_delivery_idempotency_guard_and_task_result_join",
            vec!["mailbox_delivery_event_intake"],
            "promote mailbox seq plus agentPath into a replay key and join wait results to timeline evidence refs",
        ),
        source_gap(
            "gap_multi_agent_reducer_task_result_replay_key",
            "hepta_runtime_multi_agent_reducer",
            "reducer_task_result_idempotency_guard",
            vec!["task_result_event_intake"],
            "derive reducer output keys from traceId, taskId, reducer strategy, status, and evidenceHash before terminal replay",
        ),
        source_gap(
            "gap_task_board_worker_task_replay_key",
            "hepta_runtime_task_board",
            "task_board_worker_task_projection_idempotency_guard",
            vec!["worker_task_event_intake"],
            "derive task board replay keys from workerTaskId, attempt, lane, leaseState, and artifactHash before scheduler readback",
        ),
    ]
}

impl WorkGraphReplayReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_record_persisted: false,
            graph_state_persisted: false,
            wal_replayed: false,
            checkpoint_loaded: false,
            idempotency_index_mutated: false,
            readback_performed: false,
            drift_state_persisted: false,
            recovery_performed: false,
            promotion_performed: false,
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

fn replay_stage(
    id: &'static str,
    input_contract_ids: Vec<&'static str>,
    output_contract_ids: Vec<&'static str>,
    failure_mode: &'static str,
) -> WorkGraphReplayStagePreview {
    WorkGraphReplayStagePreview {
        id,
        input_contract_ids,
        output_contract_ids,
        failure_mode,
        executes_replay: false,
    }
}

fn readback_assertion(
    id: &'static str,
    collection_id: &'static str,
    required_inputs: Vec<&'static str>,
    evidence_fields: Vec<&'static str>,
    promotion_gate: &'static str,
) -> WorkGraphReadbackAssertionPreview {
    WorkGraphReadbackAssertionPreview {
        id,
        collection_id,
        required_inputs,
        evidence_fields,
        promotion_gate,
        mutates_store: false,
    }
}

fn event_replay_plan(
    id: &'static str,
    event_contract_id: &'static str,
    source_surface_ids: Vec<&'static str>,
    expected_collection_ids: Vec<&'static str>,
    deterministic_replay_key_fields: Vec<&'static str>,
    readback_assertion_ids: Vec<&'static str>,
    blocking_source_surface_ids: Vec<&'static str>,
    blocking_reason_ids: Vec<&'static str>,
) -> WorkGraphAppendOnlyEventReplayPlanPreview {
    WorkGraphAppendOnlyEventReplayPlanPreview {
        id,
        event_contract_id,
        source_surface_ids,
        expected_collection_ids,
        deterministic_replay_key_fields,
        readback_assertion_ids,
        blocking_source_surface_ids,
        blocking_reason_ids,
        executes_replay: false,
        performs_readback: false,
        mutates_store: false,
    }
}

fn source_gap(
    id: &'static str,
    source_surface_id: &'static str,
    missing_capability: &'static str,
    affected_event_contract_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphReplayReadbackSourceGapPreview {
    WorkGraphReplayReadbackSourceGapPreview {
        id,
        source_surface_id,
        missing_capability,
        affected_event_contract_ids,
        required_before_replay_execution: true,
        recommended_fix,
    }
}

fn replay_plan_id_for_event_contract(event_contract_id: &str) -> &'static str {
    match event_contract_id {
        "plan_step_event_intake" => "replay_plan_step_event_intake",
        "agent_spawn_event_intake" => "replay_agent_spawn_event_intake",
        "mailbox_delivery_event_intake" => "replay_mailbox_delivery_event_intake",
        "agent_job_item_event_intake" => "replay_agent_job_item_event_intake",
        "worker_task_event_intake" => "replay_worker_task_event_intake",
        "scheduler_run_event_intake" => "replay_scheduler_run_event_intake",
        "artifact_event_intake" => "replay_artifact_event_intake",
        "approval_event_intake" => "replay_approval_event_intake",
        "task_result_event_intake" => "replay_task_result_event_intake",
        _ => "replay_unknown_event_intake",
    }
}

fn readback_assertion_id_for_collection(collection_id: &str) -> &'static str {
    match collection_id {
        "nodes" => "assert_nodes_readback_matches_wal",
        "edges" => "assert_edges_readback_matches_wal",
        "taskResults" => "assert_task_results_readback_matches_wal",
        "artifacts" => "assert_artifacts_readback_matches_wal",
        "approvals" => "assert_approvals_readback_matches_wal",
        "timelineEvents" => "assert_timeline_readback_matches_wal",
        _ => "assert_unknown_collection_readback_matches_wal",
    }
}

fn drift_detector(
    id: &'static str,
    compared_fields: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphDriftDetectorPreview {
    WorkGraphDriftDetectorPreview {
        id,
        compared_fields,
        severity,
        blocks_promotion: true,
    }
}

fn recovery_preview(
    id: &'static str,
    trigger_detector_ids: Vec<&'static str>,
    recovery_action: &'static str,
) -> WorkGraphRecoveryPreview {
    WorkGraphRecoveryPreview {
        id,
        trigger_detector_ids,
        recovery_action,
        requires_operator_approval: true,
        executes_recovery: false,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphReplayReadbackInvariantPreview {
    WorkGraphReplayReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_readback_preview_declares_replay_stage_order() {
        let report = hepta_work_graph_replay_readback_preview_report();
        let stage_ids = report
            .replay_stages
            .iter()
            .map(|stage| stage.id)
            .collect::<Vec<_>>();

        assert_eq!(
            stage_ids,
            [
                "preview_load_wal_head",
                "preview_validate_wal_hash_chain",
                "preview_apply_idempotency_window",
                "preview_materialize_collections",
                "preview_compare_checkpoint",
                "preview_emit_readback_report",
            ]
        );
        assert_eq!(report.replay_stage_count, 6);
        assert!(
            report
                .replay_stages
                .iter()
                .all(|stage| !stage.executes_replay)
        );
    }

    #[test]
    fn replay_readback_preview_asserts_every_store_collection() {
        let report = hepta_work_graph_replay_readback_preview_report();
        let collection_ids = report
            .readback_assertions
            .iter()
            .map(|assertion| assertion.collection_id)
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
        assert_eq!(report.readback_assertion_count, 6);
        assert!(
            report
                .readback_assertions
                .iter()
                .all(|assertion| !assertion.mutates_store)
        );
    }

    #[test]
    fn replay_readback_preview_blocks_promotion_on_drift() {
        let report = hepta_work_graph_replay_readback_preview_report();
        let detector_ids = report
            .drift_detectors
            .iter()
            .map(|detector| detector.id)
            .collect::<Vec<_>>();

        assert_eq!(
            detector_ids,
            [
                "detect_identity_drift",
                "detect_ordering_drift",
                "detect_status_drift",
                "detect_hash_drift",
                "detect_redaction_drift",
            ]
        );
        assert_eq!(report.drift_detector_count, 5);
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.blocks_promotion)
        );
    }

    #[test]
    fn replay_readback_preview_keeps_recovery_disabled() {
        let report = hepta_work_graph_replay_readback_preview_report();

        assert_eq!(report.recovery_preview_count, 5);
        assert!(report.recovery_previews.iter().all(|recovery| {
            recovery.requires_operator_approval && !recovery.executes_recovery
        }));
        assert!(report.ready_for_promotion_precondition_preview);
        assert!(!report.ready_for_replay_execution);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphReplayReadbackPreviewSideEffects::none()
        );
    }

    #[test]
    fn replay_readback_preview_covers_append_only_event_intake_contracts() {
        let report = hepta_work_graph_replay_readback_preview_report();
        let event_contract_ids = work_graph_append_only_event_contracts()
            .iter()
            .map(|contract| contract.id)
            .collect::<Vec<_>>();
        let replay_plan_event_contract_ids = report
            .append_only_event_replay_plans
            .iter()
            .map(|plan| plan.event_contract_id)
            .collect::<Vec<_>>();

        assert_eq!(report.append_only_event_contract_count, 9);
        assert_eq!(report.append_only_source_route_count, 12);
        assert_eq!(report.event_replay_plan_count, 9);
        assert_eq!(replay_plan_event_contract_ids, event_contract_ids);
        assert!(report.append_only_event_replay_plans.iter().all(|plan| {
            !plan.executes_replay
                && !plan.performs_readback
                && !plan.mutates_store
                && !plan.readback_assertion_ids.is_empty()
                && !plan.deterministic_replay_key_fields.is_empty()
        }));
    }

    #[test]
    fn replay_readback_preview_surfaces_source_idempotency_gaps() {
        let report = hepta_work_graph_replay_readback_preview_report();
        let gap_source_surface_ids = report
            .source_readback_gaps
            .iter()
            .map(|gap| gap.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.source_readback_gap_count, 5);
        assert_eq!(
            gap_source_surface_ids,
            [
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert!(report.source_readback_gaps.iter().all(|gap| {
            gap.required_before_replay_execution
                && !gap.affected_event_contract_ids.is_empty()
                && gap.missing_capability.contains("idempotency")
        }));
    }

    #[test]
    fn replay_readback_preview_requires_prior_gates() {
        let report = hepta_work_graph_replay_readback_preview_report();

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
            ]
        );
        assert_eq!(report.invariant_count, 6);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_REPLAY_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
