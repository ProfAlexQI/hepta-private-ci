use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_observability_timeline::work_graph_observability_timeline_adapter_previews;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_adapter_previews;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_adapter_previews;
use crate::work_graph_task_result_contract::work_graph_task_result_adapter_previews;
use crate::work_graph_unified_projection_audit_preview::WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_PREVIEW_GATE;
use crate::work_graph_unified_projection_audit_preview::WorkGraphUnifiedProjectionSourceAudit;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_coverage_gaps;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_required_prior_gates;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_source_audits;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_wal_write_boundary_execution_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_state_store::work_graph_unified_state_store_adapter_previews;
use crate::work_graph_unified_state_store::work_graph_unified_state_store_collections;

pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE: &str =
    "hepta_work_graph_canonical_adapter_inventory_preview_gate";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_SCHEMA_VERSION: &str =
    "work_graph_canonical_adapter_inventory_preview_v1";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_canonical_adapter_inventory_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub source_category_count: usize,
    pub canonical_adapter_count: usize,
    pub canonical_node_kind_count: usize,
    pub canonical_edge_kind_count: usize,
    pub canonical_collection_count: usize,
    pub canonical_collection_binding_count: usize,
    pub canonical_timeline_event_binding_count: usize,
    pub terminal_task_result_required_count: usize,
    pub store_adapter_present_count: usize,
    pub task_result_adapter_present_count: usize,
    pub timeline_adapter_present_count: usize,
    pub scheduler_admission_adapter_present_count: usize,
    pub role_manifest_adapter_present_count: usize,
    pub contract_ready_adapter_count: usize,
    pub partial_or_gap_adapter_count: usize,
    pub inventory_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub canonical_collections: Vec<&'static str>,
    pub adapters: Vec<WorkGraphCanonicalAdapterInventorySourcePreview>,
    pub inventory_blockers: Vec<WorkGraphCanonicalAdapterInventoryBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_canonical_adapter_inventory_readback_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalAdapterInventoryPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventorySourcePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub canonical_node_kind: &'static str,
    pub canonical_edge_kinds: Vec<&'static str>,
    pub canonical_identity_fields: Vec<&'static str>,
    pub canonical_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub terminal_task_result_required: bool,
    pub store_adapter_state: &'static str,
    pub task_result_adapter_state: &'static str,
    pub timeline_adapter_state: &'static str,
    pub scheduler_admission_adapter_state: &'static str,
    pub role_manifest_adapter_state: &'static str,
    pub canonical_inventory_state: &'static str,
    pub source_blocker_ids: Vec<&'static str>,
    pub inventory_blocker_ids: Vec<&'static str>,
    pub next_inventory_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_append_only_fact_source: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_canonical_adapter_inventory_preview_report()
-> WorkGraphCanonicalAdapterInventoryPreviewReport {
    let adapters = work_graph_canonical_adapter_inventory_sources();
    let inventory_blockers = work_graph_canonical_adapter_inventory_blockers();
    let required_prior_gates = work_graph_canonical_adapter_inventory_required_prior_gates();
    let canonical_collections = work_graph_unified_state_store_collections()
        .into_iter()
        .map(|collection| collection.id)
        .collect::<Vec<_>>();

    WorkGraphCanonicalAdapterInventoryPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE,
        schema_version: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_SCHEMA_VERSION,
        preview_mode: "read_only_canonical_adapter_inventory_no_persistence",
        source_surface_count: adapters.len(),
        source_category_count: unique_count(adapters.iter().map(|adapter| adapter.source_category)),
        canonical_adapter_count: adapters.len(),
        canonical_node_kind_count: unique_count(
            adapters.iter().map(|adapter| adapter.canonical_node_kind),
        ),
        canonical_edge_kind_count: unique_count(
            adapters
                .iter()
                .flat_map(|adapter| adapter.canonical_edge_kinds.iter().copied()),
        ),
        canonical_collection_count: canonical_collections.len(),
        canonical_collection_binding_count: adapters
            .iter()
            .map(|adapter| adapter.canonical_collection_ids.len())
            .sum(),
        canonical_timeline_event_binding_count: adapters
            .iter()
            .map(|adapter| adapter.timeline_event_type_ids.len())
            .sum(),
        terminal_task_result_required_count: adapters
            .iter()
            .filter(|adapter| adapter.terminal_task_result_required)
            .count(),
        store_adapter_present_count: adapters
            .iter()
            .filter(|adapter| adapter.store_adapter_state == "present_preview")
            .count(),
        task_result_adapter_present_count: adapters
            .iter()
            .filter(|adapter| adapter.task_result_adapter_state == "present_preview")
            .count(),
        timeline_adapter_present_count: adapters
            .iter()
            .filter(|adapter| adapter.timeline_adapter_state == "present_preview")
            .count(),
        scheduler_admission_adapter_present_count: adapters
            .iter()
            .filter(|adapter| adapter.scheduler_admission_adapter_state == "present_preview")
            .count(),
        role_manifest_adapter_present_count: adapters
            .iter()
            .filter(|adapter| adapter.role_manifest_adapter_state == "present_preview")
            .count(),
        contract_ready_adapter_count: adapters
            .iter()
            .filter(|adapter| {
                adapter.canonical_inventory_state == "canonical_contract_ready_preview"
            })
            .count(),
        partial_or_gap_adapter_count: adapters
            .iter()
            .filter(|adapter| {
                adapter.canonical_inventory_state != "canonical_contract_ready_preview"
            })
            .count(),
        inventory_blocker_count: inventory_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        canonical_collections,
        adapters,
        inventory_blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_RECOMMENDED_NEXT_GATE,
        ready_for_canonical_adapter_inventory_readback_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalAdapterInventoryPreviewSideEffects::none(),
    }
}

pub fn work_graph_canonical_adapter_inventory_sources()
-> Vec<WorkGraphCanonicalAdapterInventorySourcePreview> {
    let store_adapter_ids = work_graph_unified_state_store_adapter_previews()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();
    let task_result_adapter_ids = work_graph_task_result_adapter_previews()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();
    let timeline_adapter_ids = work_graph_observability_timeline_adapter_previews()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();
    let scheduler_adapter_ids = work_graph_scheduler_admission_adapter_previews()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();
    let role_adapter_ids = work_graph_role_manifest_adapter_previews()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();

    work_graph_unified_projection_source_audits()
        .iter()
        .map(|source| {
            canonical_adapter_inventory_source_from(
                source,
                &store_adapter_ids,
                &task_result_adapter_ids,
                &timeline_adapter_ids,
                &scheduler_adapter_ids,
                &role_adapter_ids,
            )
        })
        .collect()
}

pub fn work_graph_canonical_adapter_inventory_blockers()
-> Vec<WorkGraphCanonicalAdapterInventoryBlockerPreview> {
    let all_source_surface_ids = work_graph_unified_projection_source_audits()
        .into_iter()
        .map(|source| source.source_surface_id)
        .collect::<Vec<_>>();
    let mut blockers = vec![
        inventory_blocker(
            "canonical_adapter_inventory_readback_missing",
            "high",
            all_source_surface_ids.clone(),
            "prove every canonical adapter row can be read back before promoting a fact source",
        ),
        inventory_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            all_source_surface_ids.clone(),
            "shadow-write PlanStep, AgentTask, TaskResult, Artifact, Approval, Lease, Gate, and Timeline events before enforcement",
        ),
        inventory_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            all_source_surface_ids,
            "keep runtime adapters preview-only until readback, replay, and operator-review gates are complete",
        ),
    ];
    blockers.extend(
        work_graph_unified_projection_coverage_gaps()
            .into_iter()
            .map(|gap| {
                inventory_blocker(
                    gap.id,
                    gap.severity,
                    gap.source_surface_ids,
                    gap.recommended_fix,
                )
            }),
    );
    blockers
}

pub fn work_graph_canonical_adapter_inventory_required_prior_gates() -> Vec<&'static str> {
    let mut gates = work_graph_unified_projection_required_prior_gates();
    gates.push(WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_PREVIEW_GATE);
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphCanonicalAdapterInventoryPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn canonical_adapter_inventory_source_from(
    source: &WorkGraphUnifiedProjectionSourceAudit,
    store_adapter_ids: &BTreeSet<&'static str>,
    task_result_adapter_ids: &BTreeSet<&'static str>,
    timeline_adapter_ids: &BTreeSet<&'static str>,
    scheduler_adapter_ids: &BTreeSet<&'static str>,
    role_adapter_ids: &BTreeSet<&'static str>,
) -> WorkGraphCanonicalAdapterInventorySourcePreview {
    WorkGraphCanonicalAdapterInventorySourcePreview {
        source_surface_id: source.source_surface_id,
        source_category: source.source_category,
        canonical_node_kind: canonical_node_kind_for_source(source.source_surface_id),
        canonical_edge_kinds: canonical_edge_kinds_for_source(source.source_surface_id),
        canonical_identity_fields: canonical_identity_fields_for_source(source.source_category),
        canonical_collection_ids: source.projected_collection_ids.clone(),
        timeline_event_type_ids: source.timeline_event_type_ids.clone(),
        terminal_task_result_required: source.requires_terminal_task_result,
        store_adapter_state: adapter_state(store_adapter_ids.contains(source.source_surface_id)),
        task_result_adapter_state: task_result_adapter_state(
            source.requires_terminal_task_result,
            task_result_adapter_ids.contains(source.source_surface_id),
        ),
        timeline_adapter_state: adapter_state(
            timeline_adapter_ids.contains(source.source_surface_id),
        ),
        scheduler_admission_adapter_state: adapter_state(
            scheduler_adapter_ids.contains(source.source_surface_id),
        ),
        role_manifest_adapter_state: adapter_state(
            role_adapter_ids.contains(source.source_surface_id),
        ),
        canonical_inventory_state: canonical_inventory_state(source.coverage_state),
        source_blocker_ids: source.blocker_ids.clone(),
        inventory_blocker_ids: inventory_blocker_ids_for_source(source),
        next_inventory_step: next_inventory_step_for_source(source),
    }
}

fn canonical_node_kind_for_source(source_surface_id: &'static str) -> &'static str {
    match source_surface_id {
        "update_plan_tool"
        | "plan_mode_proposed_plan_blocks"
        | "app_server_turn_plan_notification" => "plan_step",
        "multi_agent_v2_thread_spawn"
        | "multi_agent_v2_mailbox_wait"
        | "hepta_runtime_multi_agent_reducer" => "agent_task",
        "agent_jobs_batch_workers" | "hepta_runtime_task_board" | "hepta_runtime_worker_tasks" => {
            "worker_task"
        }
        "hepta_runtime_scheduler_store" => "scheduler_run",
        "hepta_runtime_approval_broker" => "human_approval",
        "hepta_runtime_agent_harness" => "external_handoff",
        _ => "unknown",
    }
}

fn canonical_edge_kinds_for_source(source_surface_id: &'static str) -> Vec<&'static str> {
    match source_surface_id {
        "update_plan_tool" => vec!["depends_on", "observes"],
        "plan_mode_proposed_plan_blocks" => vec!["depends_on", "replaces"],
        "app_server_turn_plan_notification" => vec!["observes", "updates"],
        "multi_agent_v2_thread_spawn" => vec!["spawned_by", "depends_on", "reports_to"],
        "multi_agent_v2_mailbox_wait" => vec!["observes", "unblocks"],
        "hepta_runtime_multi_agent_reducer" => vec!["reduces", "verifies"],
        "agent_jobs_batch_workers" => vec!["assigned_to", "reports_result"],
        "hepta_runtime_task_board" => vec!["depends_on", "lease_claim"],
        "hepta_runtime_worker_tasks" => vec!["depends_on", "produces", "reports_result"],
        "hepta_runtime_scheduler_store" => vec!["admits", "blocks", "leases"],
        "hepta_runtime_approval_broker" => vec!["requires_approval", "supersedes"],
        "hepta_runtime_agent_harness" => vec!["handoff_to", "produces", "blocked_by_approval"],
        _ => Vec::new(),
    }
}

fn canonical_identity_fields_for_source(source_category: &'static str) -> Vec<&'static str> {
    match source_category {
        "planning" => vec!["sourceSurfaceId", "traceId", "planStepId"],
        "multi_agent" => vec!["sourceSurfaceId", "traceId", "agentPath", "threadId"],
        "batch_agent_jobs" => vec!["sourceSurfaceId", "traceId", "jobId", "itemId"],
        "runtime_scheduler" => vec!["sourceSurfaceId", "traceId", "taskId", "leaseId"],
        "operator_control" => vec!["sourceSurfaceId", "traceId", "approvalId"],
        "external_handoff" => vec!["sourceSurfaceId", "traceId", "handoffId"],
        _ => vec!["sourceSurfaceId", "traceId", "sourceRecordId"],
    }
}

fn adapter_state(present: bool) -> &'static str {
    if present {
        "present_preview"
    } else {
        "missing"
    }
}

fn task_result_adapter_state(required: bool, present: bool) -> &'static str {
    if present {
        "present_preview"
    } else if required {
        "missing"
    } else {
        "not_required"
    }
}

fn canonical_inventory_state(coverage_state: &'static str) -> &'static str {
    match coverage_state {
        "contract_ready_preview" => "canonical_contract_ready_preview",
        "partial_projection_preview" => "canonical_partial_preview",
        "timeline_only_preview" => "canonical_timeline_only_preview",
        "projection_gap" => "canonical_projection_gap",
        _ => "canonical_unknown",
    }
}

fn inventory_blocker_ids_for_source(
    source: &WorkGraphUnifiedProjectionSourceAudit,
) -> Vec<&'static str> {
    let mut blocker_ids = source.blocker_ids.iter().copied().collect::<BTreeSet<_>>();
    blocker_ids.insert("canonical_adapter_inventory_readback_missing");
    blocker_ids.insert("append_only_work_graph_events_disabled");
    blocker_ids.insert("runtime_canonical_adapter_enforcement_disabled");
    if source.coverage_state != "contract_ready_preview" {
        blocker_ids.insert("canonical_adapter_projection_incomplete");
    }
    blocker_ids.into_iter().collect()
}

fn next_inventory_step_for_source(source: &WorkGraphUnifiedProjectionSourceAudit) -> &'static str {
    if source.coverage_state == "contract_ready_preview" {
        "readback_canonical_adapter_inventory_projection"
    } else {
        source.next_projection_step
    }
}

fn inventory_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphCanonicalAdapterInventoryBlockerPreview {
    WorkGraphCanonicalAdapterInventoryBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        blocks_append_only_fact_source: true,
        recommended_fix,
    }
}

fn unique_count(values: impl Iterator<Item = &'static str>) -> usize {
    values.collect::<BTreeSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn canonical_adapter_inventory_covers_every_unified_projection_surface() {
        let report = hepta_work_graph_canonical_adapter_inventory_preview_report();
        let adapter_ids = report
            .adapters
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
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
        assert_eq!(report.canonical_adapter_count, 12);
        assert_eq!(report.source_category_count, 6);
        assert_eq!(report.canonical_node_kind_count, 6);
        assert_eq!(report.canonical_collection_count, 6);
    }

    #[test]
    fn canonical_adapter_inventory_derives_adapter_coverage_counts() {
        let report = hepta_work_graph_canonical_adapter_inventory_preview_report();

        assert_eq!(report.canonical_edge_kind_count, 20);
        assert_eq!(report.canonical_collection_binding_count, 36);
        assert_eq!(report.canonical_timeline_event_binding_count, 12);
        assert_eq!(report.terminal_task_result_required_count, 6);
        assert_eq!(report.store_adapter_present_count, 6);
        assert_eq!(report.task_result_adapter_present_count, 6);
        assert_eq!(report.timeline_adapter_present_count, 7);
        assert_eq!(report.scheduler_admission_adapter_present_count, 5);
        assert_eq!(report.role_manifest_adapter_present_count, 4);
        assert_eq!(report.contract_ready_adapter_count, 5);
        assert_eq!(report.partial_or_gap_adapter_count, 7);
    }

    #[test]
    fn canonical_adapter_inventory_keeps_gap_sources_actionable() {
        let report = hepta_work_graph_canonical_adapter_inventory_preview_report();
        let adapters_by_id = report
            .adapters
            .iter()
            .map(|adapter| (adapter.source_surface_id, adapter))
            .collect::<BTreeMap<_, _>>();

        assert_eq!(
            adapters_by_id["multi_agent_v2_thread_spawn"].next_inventory_step,
            "readback_canonical_adapter_inventory_projection"
        );
        assert_eq!(
            adapters_by_id["hepta_runtime_task_board"].canonical_inventory_state,
            "canonical_partial_preview"
        );
        assert!(
            adapters_by_id["plan_mode_proposed_plan_blocks"]
                .inventory_blocker_ids
                .contains(&"canonical_adapter_projection_incomplete")
        );
        assert_eq!(
            adapters_by_id["hepta_runtime_scheduler_store"].canonical_node_kind,
            "scheduler_run"
        );
    }

    #[test]
    fn canonical_adapter_inventory_declares_priors_and_no_side_effects() {
        let report = hepta_work_graph_canonical_adapter_inventory_preview_report();
        let blocker_ids = report
            .inventory_blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "canonical_adapter_inventory_readback_missing",
                "append_only_work_graph_events_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "planning_identity_is_split_between_update_plan_and_plan_mode",
                "mailbox_wait_lacks_structured_task_result_join",
                "task_board_has_admission_shape_without_unified_store_projection",
                "batch_and_worker_results_are_not_enforced_task_results",
                "role_manifest_and_scheduler_admission_remain_preview_only",
            ]
        );
        assert_eq!(report.inventory_blocker_count, 8);
        assert_eq!(report.required_prior_gate_count, 9);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_canonical_adapter_inventory_readback_preview);
        assert!(!report.ready_for_append_only_work_graph_events);
        assert!(!report.ready_for_runtime_adapter_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalAdapterInventoryPreviewSideEffects::none()
        );
    }
}
