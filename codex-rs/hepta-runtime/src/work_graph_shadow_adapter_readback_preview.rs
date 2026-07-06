use serde::Serialize;

pub const WORK_GRAPH_SHADOW_ADAPTER_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_shadow_adapter_readback_preview_gate";
pub const WORK_GRAPH_SHADOW_ADAPTER_READBACK_SCHEMA_VERSION: &str =
    "work_graph_shadow_adapter_readback_preview_v1";
pub const WORK_GRAPH_SHADOW_ADAPTER_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_feature_flag_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowAdapterReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub adapter_shadow_count: usize,
    pub collection_readback_count: usize,
    pub mismatch_detector_count: usize,
    pub evidence_packet_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub adapter_shadows: Vec<WorkGraphAdapterShadowPreview>,
    pub collection_readbacks: Vec<WorkGraphShadowCollectionReadbackPreview>,
    pub mismatch_detectors: Vec<WorkGraphShadowMismatchDetectorPreview>,
    pub evidence_packets: Vec<WorkGraphShadowEvidencePacketPreview>,
    pub durable_identity_evidence: WorkGraphShadowDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphShadowAdapterReadbackInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_persistence_feature_flag_preview: bool,
    pub ready_for_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphShadowAdapterReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterShadowPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub fixture_id: &'static str,
    pub expected_collection_ids: Vec<&'static str>,
    pub required_readback_ids: Vec<&'static str>,
    pub match_policy: &'static str,
    pub shadow_execution_enabled: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowCollectionReadbackPreview {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub mismatch_detector_ids: Vec<&'static str>,
    pub blocks_activation: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowMismatchDetectorPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_adapter_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEvidencePacketPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_adapter_shadow_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowAdapterReadbackInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowAdapterReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub shadow_adapter_executed: bool,
    pub adapter_projection_enforced: bool,
    pub readback_performed: bool,
    pub mismatch_state_persisted: bool,
    pub activation_performed: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_shadow_adapter_readback_preview_report()
-> WorkGraphShadowAdapterReadbackPreviewReport {
    let adapter_shadows = work_graph_shadow_adapter_shadows();
    let collection_readbacks = work_graph_shadow_adapter_collection_readbacks();
    let mismatch_detectors = work_graph_shadow_adapter_mismatch_detectors();
    let evidence_packets = work_graph_shadow_adapter_evidence_packets();
    let durable_identity_evidence = work_graph_shadow_adapter_durable_identity_evidence();
    let invariants = work_graph_shadow_adapter_readback_invariants();

    WorkGraphShadowAdapterReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SHADOW_ADAPTER_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_SHADOW_ADAPTER_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_shadow_adapter_readback_preview_no_adapter_execution",
        adapter_shadow_count: adapter_shadows.len(),
        collection_readback_count: collection_readbacks.len(),
        mismatch_detector_count: mismatch_detectors.len(),
        evidence_packet_count: evidence_packets.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_shadow_adapter_readback_required_prior_gates(),
        adapter_shadows,
        collection_readbacks,
        mismatch_detectors,
        evidence_packets,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_SHADOW_ADAPTER_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_persistence_feature_flag_preview: true,
        ready_for_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphShadowAdapterReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_shadow_adapter_readback_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_promotion_precondition_preview_gate",
        "hepta_work_graph_activation_enforcement_blocker_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_shadow_adapter_shadow_ids() -> Vec<&'static str> {
    vec![
        "shadow_update_plan_step_projection",
        "shadow_multi_agent_thread_spawn_projection",
        "shadow_agent_job_item_result_projection",
        "shadow_runtime_worker_task_artifact_projection",
        "shadow_scheduler_run_admission_projection",
        "shadow_approval_broker_human_approval_projection",
        "shadow_agent_harness_external_handoff_projection",
    ]
}

pub fn work_graph_shadow_adapter_durable_identity_field_ids() -> Vec<&'static str> {
    vec![
        "workflow_id",
        "run_id",
        "step_id",
        "checkpoint",
        "replay_key",
        "rollback_anchor",
        "receipt_hash",
    ]
}

pub fn work_graph_shadow_adapter_shadows() -> Vec<WorkGraphAdapterShadowPreview> {
    vec![
        adapter_shadow(
            "shadow_update_plan_step_projection",
            "update_plan_tool",
            "update_plan_step_projection",
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_edges_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_multi_agent_thread_spawn_projection",
            "multi_agent_v2_thread_spawn",
            "multi_agent_thread_spawn_projection",
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_edges_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_agent_job_item_result_projection",
            "agent_jobs_batch_workers",
            "agent_job_item_result_projection",
            vec!["nodes", "taskResults", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_task_results_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_runtime_worker_task_artifact_projection",
            "hepta_runtime_worker_tasks",
            "runtime_worker_task_artifact_projection",
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_task_results_shadow_match",
                "readback_artifacts_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_scheduler_run_admission_projection",
            "hepta_runtime_scheduler_store",
            "scheduler_run_admission_projection",
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_edges_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_approval_broker_human_approval_projection",
            "hepta_runtime_approval_broker",
            "approval_broker_human_approval_projection",
            vec!["nodes", "approvals", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_approvals_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
        adapter_shadow(
            "shadow_agent_harness_external_handoff_projection",
            "hepta_runtime_agent_harness",
            "agent_harness_external_handoff_projection",
            vec!["nodes", "edges", "artifacts", "timelineEvents"],
            vec![
                "readback_nodes_shadow_match",
                "readback_edges_shadow_match",
                "readback_artifacts_shadow_match",
                "readback_timeline_events_shadow_match",
            ],
        ),
    ]
}

pub fn work_graph_shadow_adapter_collection_readbacks()
-> Vec<WorkGraphShadowCollectionReadbackPreview> {
    vec![
        collection_readback(
            "readback_nodes_shadow_match",
            "nodes",
            vec!["traceId", "nodeId", "nodeKind", "status", "sourceSurfaceId"],
            vec!["detect_shadow_node_identity_mismatch"],
        ),
        collection_readback(
            "readback_edges_shadow_match",
            "edges",
            vec!["traceId", "edgeId", "edgeKind", "fromNodeId", "toNodeId"],
            vec!["detect_shadow_edge_link_mismatch"],
        ),
        collection_readback(
            "readback_task_results_shadow_match",
            "taskResults",
            vec!["traceId", "taskId", "status", "summaryHash", "evidenceRefs"],
            vec!["detect_shadow_task_result_contract_mismatch"],
        ),
        collection_readback(
            "readback_artifacts_shadow_match",
            "artifacts",
            vec!["traceId", "artifactId", "producerNodeId", "artifactHash"],
            vec!["detect_shadow_artifact_redaction_mismatch"],
        ),
        collection_readback(
            "readback_approvals_shadow_match",
            "approvals",
            vec![
                "traceId",
                "approvalId",
                "operatorScopeHash",
                "expiresAtUnixMs",
            ],
            vec!["detect_shadow_approval_scope_mismatch"],
        ),
        collection_readback(
            "readback_timeline_events_shadow_match",
            "timelineEvents",
            vec![
                "traceId",
                "eventId",
                "eventKind",
                "nodeId",
                "redactionState",
            ],
            vec!["detect_shadow_timeline_order_mismatch"],
        ),
    ]
}

pub fn work_graph_shadow_adapter_mismatch_detectors() -> Vec<WorkGraphShadowMismatchDetectorPreview>
{
    vec![
        mismatch_detector(
            "detect_shadow_node_identity_mismatch",
            vec!["nodeId", "nodeKind", "status", "sourceSurfaceId"],
            "critical",
        ),
        mismatch_detector(
            "detect_shadow_edge_link_mismatch",
            vec!["edgeId", "edgeKind", "fromNodeId", "toNodeId"],
            "critical",
        ),
        mismatch_detector(
            "detect_shadow_task_result_contract_mismatch",
            vec!["taskId", "status", "summaryHash", "evidenceRefs"],
            "critical",
        ),
        mismatch_detector(
            "detect_shadow_artifact_redaction_mismatch",
            vec![
                "artifactId",
                "artifactHash",
                "redactionState",
                "payloadHash",
            ],
            "critical",
        ),
        mismatch_detector(
            "detect_shadow_approval_scope_mismatch",
            vec!["approvalId", "operatorScopeHash", "expiresAtUnixMs"],
            "critical",
        ),
        mismatch_detector(
            "detect_shadow_timeline_order_mismatch",
            vec!["traceId", "eventId", "eventKind", "eventSequence"],
            "high",
        ),
    ]
}

pub fn work_graph_shadow_adapter_evidence_packets() -> Vec<WorkGraphShadowEvidencePacketPreview> {
    vec![
        evidence_packet("update_plan_shadow_evidence", "update_plan_tool"),
        evidence_packet(
            "multi_agent_thread_spawn_shadow_evidence",
            "multi_agent_v2_thread_spawn",
        ),
        evidence_packet("agent_job_item_shadow_evidence", "agent_jobs_batch_workers"),
        evidence_packet("worker_task_shadow_evidence", "hepta_runtime_worker_tasks"),
        evidence_packet(
            "scheduler_run_shadow_evidence",
            "hepta_runtime_scheduler_store",
        ),
        evidence_packet(
            "approval_broker_shadow_evidence",
            "hepta_runtime_approval_broker",
        ),
        evidence_packet(
            "agent_harness_shadow_evidence",
            "hepta_runtime_agent_harness",
        ),
    ]
}

pub fn work_graph_shadow_adapter_readback_invariants()
-> Vec<WorkGraphShadowAdapterReadbackInvariantPreview> {
    vec![
        invariant(
            "shadow_readback_requires_durable_identity_evidence",
            "shadow adapter readback evidence must carry workflow, run, step, checkpoint, replay, rollback, and receipt hashes",
        ),
        invariant(
            "shadow_readback_matches_projection_before_enforcement",
            "adapter enforcement cannot be enabled until projected ids match shadow readback",
        ),
        invariant(
            "shadow_readback_covers_every_projected_collection",
            "nodes, edges, taskResults, artifacts, approvals, and timeline events must each have readback rules",
        ),
        invariant(
            "mismatch_blocks_adapter_enforcement",
            "any identity, edge, TaskResult, artifact, approval, or timeline mismatch blocks enforcement",
        ),
        invariant(
            "shadow_evidence_is_redacted_and_non_persistent",
            "shadow evidence packets carry hashes and refs only and cannot be persisted by this gate",
        ),
        invariant(
            "shadow_adapter_does_not_execute_source_adapters",
            "this preview describes shadow comparisons without running source adapters",
        ),
        invariant(
            "shadow_adapter_readback_preview_has_no_side_effects",
            "this gate cannot read live state, enforce adapters, activate persistence, or send externally",
        ),
    ]
}

pub fn work_graph_shadow_adapter_durable_identity_evidence()
-> WorkGraphShadowDurableIdentityEvidencePreview {
    WorkGraphShadowDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_shadow_adapter_durable_identity_field_ids(),
        required_for_adapter_shadow_ids: work_graph_shadow_adapter_shadow_ids(),
        currently_satisfied: false,
    }
}

impl WorkGraphShadowAdapterReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            shadow_adapter_executed: false,
            adapter_projection_enforced: false,
            readback_performed: false,
            mismatch_state_persisted: false,
            activation_performed: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn adapter_shadow(
    id: &'static str,
    source_surface_id: &'static str,
    fixture_id: &'static str,
    expected_collection_ids: Vec<&'static str>,
    required_readback_ids: Vec<&'static str>,
) -> WorkGraphAdapterShadowPreview {
    WorkGraphAdapterShadowPreview {
        id,
        source_surface_id,
        fixture_id,
        expected_collection_ids,
        required_readback_ids,
        match_policy: "all_projected_ids_and_hashes_must_match_shadow_readback",
        shadow_execution_enabled: false,
        enforcement_enabled: false,
    }
}

fn collection_readback(
    id: &'static str,
    collection_id: &'static str,
    required_fields: Vec<&'static str>,
    mismatch_detector_ids: Vec<&'static str>,
) -> WorkGraphShadowCollectionReadbackPreview {
    WorkGraphShadowCollectionReadbackPreview {
        id,
        collection_id,
        required_fields: with_durable_identity_fields(required_fields),
        mismatch_detector_ids,
        blocks_activation: true,
        mutates_store: false,
    }
}

fn mismatch_detector(
    id: &'static str,
    compared_fields: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphShadowMismatchDetectorPreview {
    WorkGraphShadowMismatchDetectorPreview {
        id,
        compared_fields,
        severity,
        blocks_adapter_enforcement: true,
    }
}

fn evidence_packet(
    id: &'static str,
    source_surface_id: &'static str,
) -> WorkGraphShadowEvidencePacketPreview {
    WorkGraphShadowEvidencePacketPreview {
        id,
        source_surface_id,
        required_fields: with_durable_identity_fields(vec![
            "traceId",
            "sourceSurfaceId",
            "fixtureId",
            "projectedHash",
            "readbackHash",
            "mismatchDetectorIds",
            "redactionState",
        ]),
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_durable_identity_fields(fields: Vec<&'static str>) -> Vec<&'static str> {
    let mut merged = work_graph_shadow_adapter_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphShadowAdapterReadbackInvariantPreview {
    WorkGraphShadowAdapterReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_adapter_readback_preview_declares_adapter_sources() {
        let report = hepta_work_graph_shadow_adapter_readback_preview_report();
        let source_surface_ids = report
            .adapter_shadows
            .iter()
            .map(|shadow| shadow.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_surface_ids,
            [
                "update_plan_tool",
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_approval_broker",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.adapter_shadow_count, 7);
        assert!(
            report
                .adapter_shadows
                .iter()
                .all(|shadow| { !shadow.shadow_execution_enabled && !shadow.enforcement_enabled })
        );
    }

    #[test]
    fn shadow_adapter_readback_preview_covers_store_collections() {
        let report = hepta_work_graph_shadow_adapter_readback_preview_report();
        let collection_ids = report
            .collection_readbacks
            .iter()
            .map(|readback| readback.collection_id)
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
        assert_eq!(report.collection_readback_count, 6);
        assert!(report.collection_readbacks.iter().all(|readback| {
            readback.blocks_activation
                && !readback.mutates_store
                && work_graph_shadow_adapter_durable_identity_field_ids()
                    .iter()
                    .all(|field| readback.required_fields.contains(field))
        }));
    }

    #[test]
    fn shadow_adapter_readback_preview_blocks_on_mismatch() {
        let report = hepta_work_graph_shadow_adapter_readback_preview_report();
        let detector_ids = report
            .mismatch_detectors
            .iter()
            .map(|detector| detector.id)
            .collect::<Vec<_>>();

        assert_eq!(
            detector_ids,
            [
                "detect_shadow_node_identity_mismatch",
                "detect_shadow_edge_link_mismatch",
                "detect_shadow_task_result_contract_mismatch",
                "detect_shadow_artifact_redaction_mismatch",
                "detect_shadow_approval_scope_mismatch",
                "detect_shadow_timeline_order_mismatch",
            ]
        );
        assert_eq!(report.mismatch_detector_count, 6);
        assert!(
            report
                .mismatch_detectors
                .iter()
                .all(|detector| detector.blocks_adapter_enforcement)
        );
    }

    #[test]
    fn shadow_adapter_readback_preview_keeps_evidence_non_persistent() {
        let report = hepta_work_graph_shadow_adapter_readback_preview_report();

        assert_eq!(report.evidence_packet_count, 7);
        assert!(report.evidence_packets.iter().all(|packet| {
            !packet.persistence_enabled
                && !packet.external_delivery_enabled
                && work_graph_shadow_adapter_durable_identity_field_ids()
                    .iter()
                    .all(|field| packet.required_fields.contains(field))
        }));
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_shadow_adapter_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_adapter_shadow_ids,
            work_graph_shadow_adapter_shadow_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.side_effects,
            WorkGraphShadowAdapterReadbackPreviewSideEffects::none()
        );
        assert!(report.ready_for_persistence_feature_flag_preview);
        assert!(!report.ready_for_adapter_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn shadow_adapter_readback_preview_requires_prior_gates() {
        let report = hepta_work_graph_shadow_adapter_readback_preview_report();

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
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_promotion_precondition_preview_gate",
                "hepta_work_graph_activation_enforcement_blocker_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SHADOW_ADAPTER_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
