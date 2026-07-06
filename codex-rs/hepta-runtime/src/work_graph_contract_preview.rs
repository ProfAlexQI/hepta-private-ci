use serde::Serialize;

pub const WORK_GRAPH_CONTRACT_PREVIEW_GATE: &str = "hepta_work_graph_contract_preview_gate";
pub const WORK_GRAPH_CONTRACT_PREVIEW_SCHEMA_VERSION: &str = "work_graph_contract_preview_v1";
pub const WORK_GRAPH_CONTRACT_PREVIEW_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_task_result_contract_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphContractPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub node_type_count: usize,
    pub edge_type_count: usize,
    pub invariant_count: usize,
    pub adapter_preview_count: usize,
    pub node_types: Vec<WorkGraphNodeTypePreview>,
    pub edge_types: Vec<WorkGraphEdgeTypePreview>,
    pub invariants: Vec<WorkGraphInvariantPreview>,
    pub adapter_previews: Vec<WorkGraphAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_task_result_contract_preview: bool,
    pub ready_for_scheduler_admission_preview: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphContractPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphNodeTypePreview {
    pub id: &'static str,
    pub purpose: &'static str,
    pub required_identity_fields: Vec<&'static str>,
    pub required_status_fields: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEdgeTypePreview {
    pub id: &'static str,
    pub from_node_kinds: Vec<&'static str>,
    pub to_node_kinds: Vec<&'static str>,
    pub invariant: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterPreview {
    pub source_surface_id: &'static str,
    pub preview_node_kind: &'static str,
    pub preview_edge_kinds: Vec<&'static str>,
    pub live_mutation_enabled: bool,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphContractPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_contract_preview_report() -> WorkGraphContractPreviewReport {
    let node_types = work_graph_contract_preview_node_types();
    let edge_types = work_graph_contract_preview_edge_types();
    let invariants = work_graph_contract_preview_invariants();
    let adapter_previews = work_graph_contract_preview_adapters();
    WorkGraphContractPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CONTRACT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_CONTRACT_PREVIEW_SCHEMA_VERSION,
        preview_mode: "read_only_contract_preview_no_state_store",
        node_type_count: node_types.len(),
        edge_type_count: edge_types.len(),
        invariant_count: invariants.len(),
        adapter_preview_count: adapter_previews.len(),
        node_types,
        edge_types,
        invariants,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_CONTRACT_PREVIEW_RECOMMENDED_NEXT_GATE,
        ready_for_task_result_contract_preview: true,
        ready_for_scheduler_admission_preview: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphContractPreviewSideEffects::none(),
    }
}

pub fn work_graph_contract_preview_node_types() -> Vec<WorkGraphNodeTypePreview> {
    vec![
        node_type(
            "plan_step",
            "durable projection of update_plan or Plan Mode planning intent",
            vec![
                "node_id",
                "source_thread_id",
                "source_turn_id",
                "step_index",
            ],
            vec!["status", "owner_agent_path", "blocked_reason"],
            vec!["trace_id", "source_event_id"],
        ),
        node_type(
            "agent_task",
            "subagent or delegated thread task with targetable lifecycle status",
            vec!["node_id", "agent_path", "thread_id", "parent_thread_id"],
            vec!["status", "role_id", "budget_state"],
            vec!["trace_id", "last_mailbox_sequence"],
        ),
        node_type(
            "worker_task",
            "runtime worker or batch item that may produce artifacts and patches",
            vec!["node_id", "task_id", "workspace_id"],
            vec!["status", "lease_state", "attempt_count"],
            vec!["trace_id", "artifact_ids", "command_run_ids"],
        ),
        node_type(
            "scheduler_run",
            "scheduled job run and wake handoff with idempotency/readback metadata",
            vec!["node_id", "job_id", "run_id"],
            vec!["status", "timeout_state", "active_state"],
            vec!["trace_id", "idempotency_key_hash", "readback_evidence_id"],
        ),
        node_type(
            "verification_gate",
            "local static, unit, integration, or operator gate result",
            vec!["node_id", "gate_id", "scope"],
            vec!["status", "blocking", "rerun_required"],
            vec!["trace_id", "report_hash", "log_excerpt_hash"],
        ),
        node_type(
            "artifact",
            "file, patch, report, evidence bundle, or external handoff material",
            vec!["node_id", "artifact_id", "artifact_kind"],
            vec!["status", "retention_class", "redaction_state"],
            vec!["trace_id", "content_hash", "producer_node_id"],
        ),
        node_type(
            "human_approval",
            "operator approval, denial, supersession, or acknowledgement boundary",
            vec!["node_id", "approval_id", "operator_scope"],
            vec!["status", "authority_state", "expiry_state"],
            vec!["trace_id", "request_hash", "decision_hash"],
        ),
        node_type(
            "external_handoff",
            "queued or proposed external/channel/gateway action without live execution",
            vec!["node_id", "handoff_id", "target_kind"],
            vec!["status", "policy_state", "delivery_state"],
            vec!["trace_id", "payload_preview_hash", "readback_evidence_id"],
        ),
    ]
}

pub fn work_graph_contract_preview_edge_types() -> Vec<WorkGraphEdgeTypePreview> {
    vec![
        edge_type(
            "depends_on",
            vec!["plan_step", "agent_task", "worker_task", "scheduler_run"],
            vec![
                "plan_step",
                "agent_task",
                "worker_task",
                "verification_gate",
            ],
            "target cannot become runnable before all blocking dependencies are terminal-ready",
        ),
        edge_type(
            "spawned_by",
            vec!["agent_task", "worker_task"],
            vec!["plan_step", "agent_task", "scheduler_run"],
            "child task must retain a parent trace and source authority",
        ),
        edge_type(
            "produces",
            vec!["artifact"],
            vec![
                "agent_task",
                "worker_task",
                "scheduler_run",
                "verification_gate",
            ],
            "artifact producer and content hash must be recorded before promotion",
        ),
        edge_type(
            "verifies",
            vec!["verification_gate"],
            vec!["plan_step", "agent_task", "worker_task", "artifact"],
            "verification cannot promote a node without a report hash and trace id",
        ),
        edge_type(
            "blocks",
            vec!["verification_gate", "human_approval", "external_handoff"],
            vec!["plan_step", "agent_task", "worker_task", "scheduler_run"],
            "blocked nodes require an explicit blocker id and unblock condition",
        ),
        edge_type(
            "retries",
            vec![
                "agent_task",
                "worker_task",
                "scheduler_run",
                "external_handoff",
            ],
            vec![
                "agent_task",
                "worker_task",
                "scheduler_run",
                "external_handoff",
            ],
            "retry edges must preserve original idempotency and increment attempt evidence",
        ),
        edge_type(
            "replaces",
            vec![
                "plan_step",
                "artifact",
                "human_approval",
                "external_handoff",
            ],
            vec![
                "plan_step",
                "artifact",
                "human_approval",
                "external_handoff",
            ],
            "replacement must supersede older nodes without deleting audit evidence",
        ),
    ]
}

pub fn work_graph_contract_preview_invariants() -> Vec<WorkGraphInvariantPreview> {
    vec![
        invariant(
            "stable_node_identity_required",
            "every projected node must have a deterministic node_id before it can be referenced",
        ),
        invariant(
            "source_surface_required",
            "every node and edge must carry the source surface that produced it",
        ),
        invariant(
            "trace_id_required",
            "plan, spawn, mailbox, tool, artifact, gate, and result evidence must be joinable",
        ),
        invariant(
            "task_result_not_optional_for_terminal_tasks",
            "terminal agent, worker, and scheduler nodes must eventually point at a TaskResult",
        ),
        invariant(
            "admission_before_execution",
            "execution adapters must not start until dependency, approval, lease, idempotency, and budget checks pass",
        ),
        invariant(
            "preview_gate_is_side_effect_free",
            "this preview gate cannot write graph state, spawn agents, call models, or send externally",
        ),
    ]
}

pub fn work_graph_contract_preview_adapters() -> Vec<WorkGraphAdapterPreview> {
    vec![
        adapter(
            "update_plan_tool",
            "plan_step",
            vec!["depends_on", "blocks", "replaces"],
            vec!["plan_step_identity_projection_missing"],
        ),
        adapter(
            "multi_agent_v2_thread_spawn",
            "agent_task",
            vec!["spawned_by", "depends_on", "retries"],
            vec!["agent_task_lifecycle_fact_source_missing"],
        ),
        adapter(
            "agent_jobs_batch_workers",
            "worker_task",
            vec!["spawned_by", "produces", "retries"],
            vec!["task_result_contract_not_enforced"],
        ),
        adapter(
            "hepta_runtime_task_board",
            "worker_task",
            vec!["depends_on", "blocks", "produces"],
            vec!["task_board_work_graph_adapter_missing"],
        ),
        adapter(
            "hepta_runtime_scheduler_store",
            "scheduler_run",
            vec!["depends_on", "retries", "blocks"],
            vec!["scheduler_admission_controller_not_enforced"],
        ),
        adapter(
            "hepta_runtime_agent_harness",
            "external_handoff",
            vec!["spawned_by", "produces", "blocks"],
            vec!["agent_harness_work_graph_projection_missing"],
        ),
    ]
}

impl WorkGraphContractPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn node_type(
    id: &'static str,
    purpose: &'static str,
    required_identity_fields: Vec<&'static str>,
    required_status_fields: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphNodeTypePreview {
    WorkGraphNodeTypePreview {
        id,
        purpose,
        required_identity_fields,
        required_status_fields,
        required_evidence_fields,
    }
}

fn edge_type(
    id: &'static str,
    from_node_kinds: Vec<&'static str>,
    to_node_kinds: Vec<&'static str>,
    invariant: &'static str,
) -> WorkGraphEdgeTypePreview {
    WorkGraphEdgeTypePreview {
        id,
        from_node_kinds,
        to_node_kinds,
        invariant,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphInvariantPreview {
    WorkGraphInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    preview_node_kind: &'static str,
    preview_edge_kinds: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphAdapterPreview {
    WorkGraphAdapterPreview {
        source_surface_id,
        preview_node_kind,
        preview_edge_kinds,
        live_mutation_enabled: false,
        blocker_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_preview_declares_the_minimal_node_taxonomy() {
        let report = hepta_work_graph_contract_preview_report();
        let node_ids = report
            .node_types
            .iter()
            .map(|node| node.id)
            .collect::<Vec<_>>();

        assert_eq!(
            node_ids,
            [
                "plan_step",
                "agent_task",
                "worker_task",
                "scheduler_run",
                "verification_gate",
                "artifact",
                "human_approval",
                "external_handoff",
            ]
        );
        assert_eq!(report.node_type_count, 8);
    }

    #[test]
    fn contract_preview_declares_dependency_and_evidence_edges() {
        let report = hepta_work_graph_contract_preview_report();
        let edge_ids = report
            .edge_types
            .iter()
            .map(|edge| edge.id)
            .collect::<Vec<_>>();

        assert_eq!(
            edge_ids,
            [
                "depends_on",
                "spawned_by",
                "produces",
                "verifies",
                "blocks",
                "retries",
                "replaces",
            ]
        );
        assert_eq!(report.edge_type_count, 7);
    }

    #[test]
    fn contract_preview_keeps_execution_and_persistence_disabled() {
        let report = hepta_work_graph_contract_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphContractPreviewSideEffects::none()
        );
        assert!(report.ready_for_task_result_contract_preview);
        assert!(!report.ready_for_scheduler_admission_preview);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.live_mutation_enabled)
        );
    }

    #[test]
    fn contract_preview_points_to_task_result_as_the_next_gate() {
        let report = hepta_work_graph_contract_preview_report();

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CONTRACT_PREVIEW_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 6);
        assert_eq!(report.adapter_preview_count, 6);
    }
}
