use serde::Serialize;

pub const WORK_GRAPH_UNIFIED_STATE_STORE_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_state_store_preview_gate";
pub const WORK_GRAPH_UNIFIED_STATE_STORE_SCHEMA_VERSION: &str =
    "work_graph_unified_state_store_preview_v1";
pub const WORK_GRAPH_UNIFIED_STATE_STORE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_adapter_projection_fixture_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedStateStorePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub collection_count: usize,
    pub index_count: usize,
    pub operation_count: usize,
    pub invariant_count: usize,
    pub adapter_preview_count: usize,
    pub collections: Vec<WorkGraphStoreCollectionPreview>,
    pub indexes: Vec<WorkGraphStoreIndexPreview>,
    pub operations: Vec<WorkGraphStoreOperationPreview>,
    pub invariants: Vec<WorkGraphStoreInvariantPreview>,
    pub adapter_previews: Vec<WorkGraphStoreAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_adapter_projection_fixtures: bool,
    pub ready_for_store_persistence: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphUnifiedStateStorePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreCollectionPreview {
    pub id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIndexPreview {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub fields: Vec<&'static str>,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreOperationPreview {
    pub id: &'static str,
    pub mutates_store: bool,
    pub required_inputs: Vec<&'static str>,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreAdapterPreview {
    pub source_surface_id: &'static str,
    pub projected_collection_ids: Vec<&'static str>,
    pub required_contract_gates: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedStateStorePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub store_persistence_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub adapter_projection_enforced: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_state_store_preview_report()
-> WorkGraphUnifiedStateStorePreviewReport {
    let collections = work_graph_unified_state_store_collections();
    let indexes = work_graph_unified_state_store_indexes();
    let operations = work_graph_unified_state_store_operations();
    let invariants = work_graph_unified_state_store_invariants();
    let adapter_previews = work_graph_unified_state_store_adapter_previews();

    WorkGraphUnifiedStateStorePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_UNIFIED_STATE_STORE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_UNIFIED_STATE_STORE_SCHEMA_VERSION,
        preview_mode: "read_only_store_shape_preview_no_persistence",
        collection_count: collections.len(),
        index_count: indexes.len(),
        operation_count: operations.len(),
        invariant_count: invariants.len(),
        adapter_preview_count: adapter_previews.len(),
        collections,
        indexes,
        operations,
        invariants,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_UNIFIED_STATE_STORE_RECOMMENDED_NEXT_GATE,
        ready_for_adapter_projection_fixtures: true,
        ready_for_store_persistence: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphUnifiedStateStorePreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_state_store_collections() -> Vec<WorkGraphStoreCollectionPreview> {
    vec![
        collection(
            "nodes",
            vec!["nodeId", "nodeKind"],
            "PlanStep, AgentTask, WorkerTask, SchedulerRun, gate, artifact, approval, and handoff nodes",
        ),
        collection(
            "edges",
            vec!["edgeId", "edgeKind", "fromNodeId", "toNodeId"],
            "depends_on, spawned_by, produces, verifies, blocks, retries, and replaces edges",
        ),
        collection(
            "taskResults",
            vec!["taskId", "traceId"],
            "terminal and non-terminal TaskResult projections",
        ),
        collection(
            "artifacts",
            vec!["artifactId", "producerNodeId"],
            "redacted artifact metadata, hashes, and path hints",
        ),
        collection(
            "approvals",
            vec!["approvalId", "operatorScope"],
            "operator decisions, expiry, supersession, and authority boundaries",
        ),
        collection(
            "timelineEvents",
            vec!["traceId", "eventKind", "nodeId"],
            "redacted trace timeline events for local audit views",
        ),
    ]
}

pub fn work_graph_unified_state_store_indexes() -> Vec<WorkGraphStoreIndexPreview> {
    vec![
        index(
            "byTraceId",
            "nodes",
            vec!["traceId"],
            "join every node in one work trace",
        ),
        index(
            "bySourceSurface",
            "nodes",
            vec!["sourceSurfaceId"],
            "audit which source surface produced a node",
        ),
        index(
            "byStatus",
            "nodes",
            vec!["status"],
            "find blocked, runnable, terminal, or superseded work",
        ),
        index(
            "byEdgeKind",
            "edges",
            vec!["edgeKind"],
            "query dependency, spawn, evidence, retry, and replacement paths",
        ),
        index(
            "byTaskResultStatus",
            "taskResults",
            vec!["status"],
            "find terminal results that need promotion review",
        ),
        index(
            "byTimelineTrace",
            "timelineEvents",
            vec!["traceId", "eventKind"],
            "render ordered redacted timeline views",
        ),
    ]
}

pub fn work_graph_unified_state_store_operations() -> Vec<WorkGraphStoreOperationPreview> {
    vec![
        operation(
            "preview_project_node",
            vec!["sourceSurfaceId", "nodeKind", "traceId"],
            "explain how a source record would become a WorkGraph node",
        ),
        operation(
            "preview_project_edge",
            vec!["edgeKind", "fromNodeId", "toNodeId", "traceId"],
            "explain how a relationship would be represented without writing it",
        ),
        operation(
            "preview_validate_task_result",
            vec!["taskId", "status", "traceId"],
            "check a TaskResult against the preview schema",
        ),
        operation(
            "preview_explain_admission",
            vec!["nodeId", "dependencyStatus", "budgetState"],
            "explain scheduler admission allow or deny decisions",
        ),
        operation(
            "preview_render_timeline",
            vec!["traceId", "redactionState"],
            "render a local timeline view from projected events",
        ),
        operation(
            "preview_role_manifest_projection",
            vec!["roleId", "capabilities", "toolPermissions"],
            "explain role capability and permission projection",
        ),
    ]
}

pub fn work_graph_unified_state_store_invariants() -> Vec<WorkGraphStoreInvariantPreview> {
    vec![
        invariant(
            "deterministic_identity_required",
            "node, edge, result, artifact, approval, and event ids must be deterministic",
        ),
        invariant(
            "append_only_evidence_required",
            "promotion cannot delete or rewrite prior evidence",
        ),
        invariant(
            "redacted_payload_only",
            "store previews carry hashes and references, not raw secrets or private payloads",
        ),
        invariant(
            "idempotent_projection_required",
            "re-running a projection preview must produce the same ids and decisions",
        ),
        invariant(
            "readback_before_promotion",
            "terminal state promotion needs readback, verifier, or gate evidence",
        ),
        invariant(
            "preview_store_does_not_persist",
            "this gate cannot write graph state or enable adapter enforcement",
        ),
    ]
}

pub fn work_graph_unified_state_store_adapter_previews() -> Vec<WorkGraphStoreAdapterPreview> {
    let required_contracts = vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
    ];

    vec![
        adapter(
            "update_plan_tool",
            vec!["nodes", "edges", "timelineEvents"],
            required_contracts.clone(),
            vec!["plan_step_store_projection_not_enforced"],
        ),
        adapter(
            "multi_agent_v2_thread_spawn",
            vec!["nodes", "edges", "timelineEvents"],
            required_contracts.clone(),
            vec!["agent_task_store_projection_not_enforced"],
        ),
        adapter(
            "agent_jobs_batch_workers",
            vec!["nodes", "taskResults", "timelineEvents"],
            required_contracts.clone(),
            vec!["agent_job_store_projection_not_enforced"],
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            required_contracts.clone(),
            vec!["worker_task_store_projection_not_enforced"],
        ),
        adapter(
            "hepta_runtime_scheduler_store",
            vec!["nodes", "edges", "timelineEvents"],
            required_contracts.clone(),
            vec!["scheduler_store_projection_not_enforced"],
        ),
        adapter(
            "hepta_runtime_agent_harness",
            vec!["nodes", "artifacts", "timelineEvents"],
            required_contracts,
            vec!["agent_harness_store_projection_not_enforced"],
        ),
    ]
}

impl WorkGraphUnifiedStateStorePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            store_persistence_enabled: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            adapter_projection_enforced: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn collection(
    id: &'static str,
    key_fields: Vec<&'static str>,
    purpose: &'static str,
) -> WorkGraphStoreCollectionPreview {
    WorkGraphStoreCollectionPreview {
        id,
        key_fields,
        purpose,
    }
}

fn index(
    id: &'static str,
    collection_id: &'static str,
    fields: Vec<&'static str>,
    purpose: &'static str,
) -> WorkGraphStoreIndexPreview {
    WorkGraphStoreIndexPreview {
        id,
        collection_id,
        fields,
        purpose,
    }
}

fn operation(
    id: &'static str,
    required_inputs: Vec<&'static str>,
    purpose: &'static str,
) -> WorkGraphStoreOperationPreview {
    WorkGraphStoreOperationPreview {
        id,
        mutates_store: false,
        required_inputs,
        purpose,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphStoreInvariantPreview {
    WorkGraphStoreInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    projected_collection_ids: Vec<&'static str>,
    required_contract_gates: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphStoreAdapterPreview {
    WorkGraphStoreAdapterPreview {
        source_surface_id,
        projected_collection_ids,
        required_contract_gates,
        persistence_enabled: false,
        blocker_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_state_store_preview_declares_collections_and_indexes() {
        let report = hepta_work_graph_unified_state_store_preview_report();
        let collection_ids = report
            .collections
            .iter()
            .map(|collection| collection.id)
            .collect::<Vec<_>>();
        let index_ids = report
            .indexes
            .iter()
            .map(|index| index.id)
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
        assert_eq!(
            index_ids,
            [
                "byTraceId",
                "bySourceSurface",
                "byStatus",
                "byEdgeKind",
                "byTaskResultStatus",
                "byTimelineTrace",
            ]
        );
        assert_eq!(report.collection_count, 6);
        assert_eq!(report.index_count, 6);
    }

    #[test]
    fn unified_state_store_preview_keeps_persistence_disabled() {
        let report = hepta_work_graph_unified_state_store_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedStateStorePreviewSideEffects::none()
        );
        assert!(report.ready_for_adapter_projection_fixtures);
        assert!(!report.ready_for_store_persistence);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.persistence_enabled)
        );
    }

    #[test]
    fn unified_state_store_preview_operations_are_non_mutating() {
        let report = hepta_work_graph_unified_state_store_preview_report();
        let operation_ids = report
            .operations
            .iter()
            .map(|operation| operation.id)
            .collect::<Vec<_>>();

        assert_eq!(
            operation_ids,
            [
                "preview_project_node",
                "preview_project_edge",
                "preview_validate_task_result",
                "preview_explain_admission",
                "preview_render_timeline",
                "preview_role_manifest_projection",
            ]
        );
        assert_eq!(report.operation_count, 6);
        assert!(
            report
                .operations
                .iter()
                .all(|operation| !operation.mutates_store)
        );
    }

    #[test]
    fn unified_state_store_preview_projects_core_source_surfaces() {
        let report = hepta_work_graph_unified_state_store_preview_report();
        let adapter_ids = report
            .adapter_previews
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "update_plan_tool",
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.adapter_preview_count, 6);
        assert_eq!(report.invariant_count, 6);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_STATE_STORE_RECOMMENDED_NEXT_GATE
        );
    }
}
