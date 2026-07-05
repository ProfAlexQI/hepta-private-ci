use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_adapter_projection_fixture::work_graph_adapter_projection_fixtures;
use crate::work_graph_observability_timeline::work_graph_observability_timeline_adapter_previews;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_adapter_previews;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_adapter_previews;
use crate::work_graph_task_result_contract::work_graph_task_result_adapter_previews;
use crate::work_graph_unified_state_store::work_graph_unified_state_store_adapter_previews;

pub const WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_audit_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_audit_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_state_store_persistence_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionAuditPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub source_category_count: usize,
    pub projected_node_kind_count: usize,
    pub projected_collection_count: usize,
    pub required_prior_gate_count: usize,
    pub coverage_gap_count: usize,
    pub next_cut_count: usize,
    pub source_surfaces: Vec<WorkGraphUnifiedProjectionSourceAudit>,
    pub required_prior_gates: Vec<&'static str>,
    pub coverage_gaps: Vec<WorkGraphUnifiedProjectionCoverageGap>,
    pub next_cuts: Vec<WorkGraphUnifiedProjectionNextCut>,
    pub recommended_next_gate: &'static str,
    pub ready_for_state_store_persistence_preview: bool,
    pub ready_for_store_persistence: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphUnifiedProjectionAuditPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionSourceAudit {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub projected_node_kinds: Vec<&'static str>,
    pub projected_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub has_adapter_fixture: bool,
    pub has_unified_store_projection: bool,
    pub has_task_result_projection: bool,
    pub has_scheduler_admission_projection: bool,
    pub has_observability_timeline_projection: bool,
    pub has_role_manifest_projection: bool,
    pub requires_terminal_task_result: bool,
    pub coverage_state: &'static str,
    pub blocker_ids: Vec<&'static str>,
    pub next_projection_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionCoverageGap {
    pub id: &'static str,
    pub severity: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub blocker_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionNextCut {
    pub id: &'static str,
    pub priority: &'static str,
    pub gate: &'static str,
    pub purpose: &'static str,
    pub must_remain_side_effect_free: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionAuditPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub store_persistence_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

struct ExpectedSourceSurface {
    id: &'static str,
    category: &'static str,
    node_kind: &'static str,
    requires_terminal_task_result: bool,
}

pub fn hepta_work_graph_unified_projection_audit_preview_report()
-> WorkGraphUnifiedProjectionAuditPreviewReport {
    let source_surfaces = work_graph_unified_projection_source_audits();
    let required_prior_gates = work_graph_unified_projection_required_prior_gates();
    let coverage_gaps = work_graph_unified_projection_coverage_gaps();
    let next_cuts = work_graph_unified_projection_next_cuts();
    let source_category_count = source_surfaces
        .iter()
        .map(|surface| surface.source_category)
        .collect::<BTreeSet<_>>()
        .len();
    let projected_node_kind_count = source_surfaces
        .iter()
        .flat_map(|surface| surface.projected_node_kinds.iter().copied())
        .collect::<BTreeSet<_>>()
        .len();
    let projected_collection_count = source_surfaces
        .iter()
        .flat_map(|surface| surface.projected_collection_ids.iter().copied())
        .collect::<BTreeSet<_>>()
        .len();

    WorkGraphUnifiedProjectionAuditPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_SCHEMA_VERSION,
        preview_mode: "read_only_cross_surface_projection_audit_no_persistence",
        source_surface_count: source_surfaces.len(),
        source_category_count,
        projected_node_kind_count,
        projected_collection_count,
        required_prior_gate_count: required_prior_gates.len(),
        coverage_gap_count: coverage_gaps.len(),
        next_cut_count: next_cuts.len(),
        source_surfaces,
        required_prior_gates,
        coverage_gaps,
        next_cuts,
        recommended_next_gate: WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_RECOMMENDED_NEXT_GATE,
        ready_for_state_store_persistence_preview: true,
        ready_for_store_persistence: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphUnifiedProjectionAuditPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_source_audits() -> Vec<WorkGraphUnifiedProjectionSourceAudit> {
    let fixture_sources = work_graph_adapter_projection_fixtures()
        .into_iter()
        .map(|fixture| fixture.source_surface_id)
        .collect::<BTreeSet<_>>();
    let store_adapters = work_graph_unified_state_store_adapter_previews();
    let task_result_adapters = work_graph_task_result_adapter_previews();
    let scheduler_adapters = work_graph_scheduler_admission_adapter_previews();
    let timeline_adapters = work_graph_observability_timeline_adapter_previews();
    let role_adapters = work_graph_role_manifest_adapter_previews();

    expected_source_surfaces()
        .into_iter()
        .map(|source| {
            let store_adapter = store_adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == source.id);
            let task_result_adapter = task_result_adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == source.id);
            let scheduler_adapter = scheduler_adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == source.id);
            let timeline_adapter = timeline_adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == source.id);
            let role_adapter = role_adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == source.id);
            let has_adapter_fixture = fixture_sources.contains(source.id);
            let has_unified_store_projection = store_adapter.is_some();
            let has_task_result_projection = task_result_adapter.is_some();
            let has_scheduler_admission_projection = scheduler_adapter.is_some();
            let has_observability_timeline_projection = timeline_adapter.is_some();
            let has_role_manifest_projection = role_adapter.is_some();
            let mut projected_node_kinds = BTreeSet::from([source.node_kind]);
            if let Some(adapter) = task_result_adapter {
                projected_node_kinds.insert(adapter.projected_result_node_kind);
            }
            if let Some(adapter) = scheduler_adapter {
                projected_node_kinds.insert(adapter.target_node_kind);
            }
            let mut projected_collection_ids = store_adapter
                .map(|adapter| adapter.projected_collection_ids.iter().copied().collect())
                .unwrap_or_else(BTreeSet::new);
            if has_adapter_fixture {
                projected_collection_ids.insert("nodes");
                projected_collection_ids.insert("timelineEvents");
            }
            if has_task_result_projection {
                projected_collection_ids.insert("taskResults");
            }
            if source.id == "hepta_runtime_approval_broker" {
                projected_collection_ids.insert("approvals");
            }
            let timeline_event_type_ids = timeline_adapter
                .map(|adapter| adapter.event_type_ids.clone())
                .unwrap_or_default();
            let blocker_ids = projection_blockers(
                source.id,
                store_adapter.map(|adapter| adapter.blocker_ids.as_slice()),
                task_result_adapter.map(|adapter| adapter.blocker_ids.as_slice()),
                scheduler_adapter.map(|adapter| adapter.blocker_ids.as_slice()),
                timeline_adapter.map(|adapter| adapter.blocker_ids.as_slice()),
                role_adapter.map(|adapter| adapter.blocker_ids.as_slice()),
                source.requires_terminal_task_result,
            );

            WorkGraphUnifiedProjectionSourceAudit {
                source_surface_id: source.id,
                source_category: source.category,
                projected_node_kinds: projected_node_kinds.into_iter().collect(),
                projected_collection_ids: projected_collection_ids.into_iter().collect(),
                timeline_event_type_ids,
                has_adapter_fixture,
                has_unified_store_projection,
                has_task_result_projection,
                has_scheduler_admission_projection,
                has_observability_timeline_projection,
                has_role_manifest_projection,
                requires_terminal_task_result: source.requires_terminal_task_result,
                coverage_state: projection_coverage_state(
                    has_adapter_fixture,
                    has_unified_store_projection,
                    has_observability_timeline_projection,
                    has_task_result_projection,
                    source.requires_terminal_task_result,
                ),
                blocker_ids,
                next_projection_step: projection_next_step(
                    has_unified_store_projection,
                    has_observability_timeline_projection,
                    has_task_result_projection,
                    has_role_manifest_projection,
                    source.requires_terminal_task_result,
                ),
            }
        })
        .collect()
}

pub fn work_graph_unified_projection_required_prior_gates() -> Vec<&'static str> {
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

pub fn work_graph_unified_projection_coverage_gaps() -> Vec<WorkGraphUnifiedProjectionCoverageGap> {
    vec![
        coverage_gap(
            "planning_identity_is_split_between_update_plan_and_plan_mode",
            "high",
            vec![
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
            ],
            vec![
                "plan_step_store_projection_not_enforced",
                "plan_mode_store_projection_missing",
            ],
            "project both checklist updates and Plan Mode proposals into the same plan_step node namespace",
        ),
        coverage_gap(
            "mailbox_wait_lacks_structured_task_result_join",
            "high",
            vec!["multi_agent_v2_mailbox_wait", "multi_agent_v2_thread_spawn"],
            vec![
                "mailbox_progress_timeline_adapter_not_enforced",
                "thread_spawn_edge_missing_terminal_task_result",
            ],
            "return WorkGraph mailbox event refs and terminal TaskResult refs from wait_agent",
        ),
        coverage_gap(
            "task_board_has_admission_shape_without_unified_store_projection",
            "high",
            vec!["hepta_runtime_task_board"],
            vec![
                "task_board_admission_not_enforced",
                "unified_store_projection_missing",
            ],
            "add a task_board store adapter before it can be a schedulable source of truth",
        ),
        coverage_gap(
            "batch_and_worker_results_are_not_enforced_task_results",
            "high",
            vec!["agent_jobs_batch_workers", "hepta_runtime_worker_tasks"],
            vec![
                "agent_job_result_json_is_not_task_result_schema",
                "worker_task_missing_verifier_and_reducer_projection",
            ],
            "wrap agent_jobs and worker_tasks completions in the TaskResult contract before terminal promotion",
        ),
        coverage_gap(
            "role_manifest_and_scheduler_admission_remain_preview_only",
            "medium",
            vec![
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
            ],
            vec![
                "multi_agent_v2_role_manifest_not_enforced",
                "agent_task_admission_not_enforced",
                "scheduler_run_admission_not_enforced",
            ],
            "make role manifest and scheduler admission gates authoritative after the durable store exists",
        ),
    ]
}

pub fn work_graph_unified_projection_next_cuts() -> Vec<WorkGraphUnifiedProjectionNextCut> {
    vec![
        next_cut(
            "p0_projection_report_gate",
            "P0",
            WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_PREVIEW_GATE,
            "keep a single read-only audit view over planning, subagent, batch, worker, and scheduler surfaces",
        ),
        next_cut(
            "p1_append_only_store_events",
            "P1",
            "hepta_work_graph_state_store_persistence_preview_gate",
            "promote projected nodes, edges, TaskResults, artifacts, approvals, and timeline events into append-only records",
        ),
        next_cut(
            "p2_scheduler_admission_cutover",
            "P2",
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
            "make dependency, lease, budget, role, approval, and side-effect checks the spawn and promotion authority",
        ),
        next_cut(
            "p3_structured_multi_agent_results",
            "P3",
            "hepta_work_graph_task_result_contract_preview_gate",
            "make wait_agent, subagent completion notifications, reducers, and agent_jobs results return structured refs",
        ),
    ]
}

impl WorkGraphUnifiedProjectionAuditPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            store_persistence_enabled: false,
            runtime_mutation_performed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn expected_source_surfaces() -> Vec<ExpectedSourceSurface> {
    vec![
        expected_surface("update_plan_tool", "planning", "plan_step", false),
        expected_surface(
            "plan_mode_proposed_plan_blocks",
            "planning",
            "plan_step",
            false,
        ),
        expected_surface(
            "app_server_turn_plan_notification",
            "planning",
            "plan_step",
            false,
        ),
        expected_surface(
            "multi_agent_v2_thread_spawn",
            "multi_agent",
            "agent_task",
            true,
        ),
        expected_surface(
            "multi_agent_v2_mailbox_wait",
            "multi_agent",
            "agent_task",
            false,
        ),
        expected_surface(
            "hepta_runtime_multi_agent_reducer",
            "multi_agent",
            "agent_task",
            true,
        ),
        expected_surface(
            "agent_jobs_batch_workers",
            "batch_agent_jobs",
            "worker_task",
            true,
        ),
        expected_surface(
            "hepta_runtime_task_board",
            "runtime_scheduler",
            "worker_task",
            false,
        ),
        expected_surface(
            "hepta_runtime_worker_tasks",
            "runtime_scheduler",
            "worker_task",
            true,
        ),
        expected_surface(
            "hepta_runtime_scheduler_store",
            "runtime_scheduler",
            "scheduler_run",
            true,
        ),
        expected_surface(
            "hepta_runtime_approval_broker",
            "operator_control",
            "human_approval",
            false,
        ),
        expected_surface(
            "hepta_runtime_agent_harness",
            "external_handoff",
            "external_handoff",
            true,
        ),
    ]
}

fn expected_surface(
    id: &'static str,
    category: &'static str,
    node_kind: &'static str,
    requires_terminal_task_result: bool,
) -> ExpectedSourceSurface {
    ExpectedSourceSurface {
        id,
        category,
        node_kind,
        requires_terminal_task_result,
    }
}

fn projection_blockers(
    source_surface_id: &'static str,
    store_blockers: Option<&[&'static str]>,
    task_result_blockers: Option<&[&'static str]>,
    scheduler_blockers: Option<&[&'static str]>,
    timeline_blockers: Option<&[&'static str]>,
    role_blockers: Option<&[&'static str]>,
    requires_terminal_task_result: bool,
) -> Vec<&'static str> {
    let mut blockers = BTreeSet::new();
    for blocker in store_blockers.into_iter().flatten() {
        blockers.insert(*blocker);
    }
    for blocker in task_result_blockers.into_iter().flatten() {
        blockers.insert(*blocker);
    }
    for blocker in scheduler_blockers.into_iter().flatten() {
        blockers.insert(*blocker);
    }
    for blocker in timeline_blockers.into_iter().flatten() {
        blockers.insert(*blocker);
    }
    for blocker in role_blockers.into_iter().flatten() {
        blockers.insert(*blocker);
    }
    if store_blockers.is_none() {
        blockers.insert("unified_store_projection_missing");
    }
    if requires_terminal_task_result && task_result_blockers.is_none() {
        blockers.insert("task_result_projection_missing");
    }
    if timeline_blockers.is_none() {
        blockers.insert("timeline_projection_missing");
    }
    if source_surface_id == "plan_mode_proposed_plan_blocks" {
        blockers.insert("plan_mode_store_projection_missing");
    }
    blockers.into_iter().collect()
}

fn projection_coverage_state(
    has_adapter_fixture: bool,
    has_unified_store_projection: bool,
    has_observability_timeline_projection: bool,
    has_task_result_projection: bool,
    requires_terminal_task_result: bool,
) -> &'static str {
    if has_adapter_fixture
        && has_unified_store_projection
        && has_observability_timeline_projection
        && (!requires_terminal_task_result || has_task_result_projection)
    {
        "contract_ready_preview"
    } else if has_observability_timeline_projection && !has_unified_store_projection {
        "timeline_only_preview"
    } else if has_unified_store_projection || has_adapter_fixture {
        "partial_projection_preview"
    } else {
        "projection_gap"
    }
}

fn projection_next_step(
    has_unified_store_projection: bool,
    has_observability_timeline_projection: bool,
    has_task_result_projection: bool,
    has_role_manifest_projection: bool,
    requires_terminal_task_result: bool,
) -> &'static str {
    if !has_unified_store_projection {
        "add_unified_store_adapter_projection"
    } else if !has_observability_timeline_projection {
        "add_timeline_adapter_projection"
    } else if requires_terminal_task_result && !has_task_result_projection {
        "wrap_terminal_result_in_task_result_contract"
    } else if requires_terminal_task_result && !has_role_manifest_projection {
        "add_role_manifest_projection"
    } else {
        "keep_read_only_until_append_only_store_exists"
    }
}

fn coverage_gap(
    id: &'static str,
    severity: &'static str,
    source_surface_ids: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphUnifiedProjectionCoverageGap {
    WorkGraphUnifiedProjectionCoverageGap {
        id,
        severity,
        source_surface_ids,
        blocker_ids,
        recommended_fix,
    }
}

fn next_cut(
    id: &'static str,
    priority: &'static str,
    gate: &'static str,
    purpose: &'static str,
) -> WorkGraphUnifiedProjectionNextCut {
    WorkGraphUnifiedProjectionNextCut {
        id,
        priority,
        gate,
        purpose,
        must_remain_side_effect_free: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_projection_audit_covers_planning_agent_and_scheduler_surfaces() {
        let report = hepta_work_graph_unified_projection_audit_preview_report();
        let source_surface_ids = report
            .source_surfaces
            .iter()
            .map(|surface| surface.source_surface_id)
            .collect::<Vec<_>>();
        let source_categories = report
            .source_surfaces
            .iter()
            .map(|surface| surface.source_category)
            .collect::<BTreeSet<_>>();

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
        assert_eq!(source_categories.len(), report.source_category_count);
        assert!(source_categories.contains("planning"));
        assert!(source_categories.contains("multi_agent"));
        assert!(source_categories.contains("runtime_scheduler"));
    }

    #[test]
    fn unified_projection_audit_detects_expected_coverage_gaps() {
        let report = hepta_work_graph_unified_projection_audit_preview_report();
        let gap_ids = report
            .coverage_gaps
            .iter()
            .map(|gap| gap.id)
            .collect::<Vec<_>>();
        let surface_by_id = report
            .source_surfaces
            .iter()
            .map(|surface| (surface.source_surface_id, surface))
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            gap_ids,
            [
                "planning_identity_is_split_between_update_plan_and_plan_mode",
                "mailbox_wait_lacks_structured_task_result_join",
                "task_board_has_admission_shape_without_unified_store_projection",
                "batch_and_worker_results_are_not_enforced_task_results",
                "role_manifest_and_scheduler_admission_remain_preview_only",
            ]
        );
        assert_eq!(report.coverage_gap_count, 5);
        assert_eq!(
            surface_by_id["multi_agent_v2_mailbox_wait"].coverage_state,
            "timeline_only_preview"
        );
        assert!(
            report
                .source_surfaces
                .iter()
                .all(|surface| surface.has_adapter_fixture)
        );
        assert_eq!(
            surface_by_id["plan_mode_proposed_plan_blocks"].coverage_state,
            "partial_projection_preview"
        );
        assert!(
            surface_by_id["hepta_runtime_task_board"]
                .blocker_ids
                .contains(&"unified_store_projection_missing")
        );
    }

    #[test]
    fn unified_projection_audit_requires_current_contract_gates() {
        let report = hepta_work_graph_unified_projection_audit_preview_report();

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
        assert_eq!(report.required_prior_gate_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_AUDIT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn unified_projection_audit_remains_read_only() {
        let report = hepta_work_graph_unified_projection_audit_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedProjectionAuditPreviewSideEffects::none()
        );
        assert!(report.ready_for_state_store_persistence_preview);
        assert!(!report.ready_for_store_persistence);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .next_cuts
                .iter()
                .all(|cut| cut.must_remain_side_effect_free)
        );
    }
}
