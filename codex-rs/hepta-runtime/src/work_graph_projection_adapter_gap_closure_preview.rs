use serde::Serialize;

use crate::work_graph_unified_projection_audit_preview::WorkGraphUnifiedProjectionSourceAudit;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_source_audits;
use crate::work_graph_unified_projection_enforcement_readiness_preview::WorkGraphProjectionEnforcementSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_preview::work_graph_unified_projection_enforcement_source_decisions;

pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_preview_gate";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_projection_adapter_gap_closure_preview_v1";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_gap_count: usize,
    pub closure_action_count: usize,
    pub store_adapter_closure_count: usize,
    pub timeline_adapter_closure_count: usize,
    pub adapter_fixture_closure_count: usize,
    pub task_result_adapter_closure_count: usize,
    pub closure_plan_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub source_gaps: Vec<WorkGraphProjectionAdapterSourceGapPreview>,
    pub closure_actions: Vec<WorkGraphProjectionAdapterClosureActionPreview>,
    pub closure_plans: Vec<WorkGraphProjectionAdapterClosurePlanPreview>,
    pub blockers: Vec<WorkGraphProjectionAdapterGapClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_projection_adapter_gap_closure_readback_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphProjectionAdapterGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterSourceGapPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub current_coverage_state: &'static str,
    pub enforcement_decision: &'static str,
    pub missing_adapter_fixture: bool,
    pub missing_unified_store_adapter: bool,
    pub missing_timeline_adapter: bool,
    pub missing_task_result_adapter: bool,
    pub closure_action_ids: Vec<&'static str>,
    pub current_blocker_ids: Vec<&'static str>,
    pub expected_post_closure_state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureActionPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub adapter_kind: &'static str,
    pub projected_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub closes_blocker_ids: Vec<&'static str>,
    pub mutates_runtime: bool,
    pub enforces_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosurePlanPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub closure_action_ids: Vec<&'static str>,
    pub closes_coverage_gap_ids: Vec<&'static str>,
    pub expected_contract_ready_source_count_after_closure: usize,
    pub next_gate: &'static str,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub closure_applied_to_runtime: bool,
    pub append_only_store_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_projection_adapter_gap_closure_preview_report()
-> WorkGraphProjectionAdapterGapClosurePreviewReport {
    let source_gaps = work_graph_projection_adapter_source_gaps();
    let closure_actions = work_graph_projection_adapter_closure_actions();
    let closure_plans = work_graph_projection_adapter_closure_plans();
    let blockers = work_graph_projection_adapter_gap_closure_blockers();
    let required_prior_gates = work_graph_projection_adapter_gap_closure_required_prior_gates();

    WorkGraphProjectionAdapterGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_projection_adapter_gap_closure_plan_no_runtime_attachment",
        source_gap_count: source_gaps.len(),
        closure_action_count: closure_actions.len(),
        store_adapter_closure_count: closure_actions
            .iter()
            .filter(|action| action.adapter_kind == "unified_store_projection")
            .count(),
        timeline_adapter_closure_count: closure_actions
            .iter()
            .filter(|action| action.adapter_kind == "observability_timeline_projection")
            .count(),
        adapter_fixture_closure_count: closure_actions
            .iter()
            .filter(|action| action.adapter_kind == "adapter_projection_fixture")
            .count(),
        task_result_adapter_closure_count: closure_actions
            .iter()
            .filter(|action| action.adapter_kind == "task_result_projection")
            .count(),
        closure_plan_count: closure_plans.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_gaps,
        closure_actions,
        closure_plans,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_projection_adapter_gap_closure_readback_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphProjectionAdapterGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_projection_adapter_source_gaps() -> Vec<WorkGraphProjectionAdapterSourceGapPreview>
{
    let audits = work_graph_unified_projection_source_audits();
    let decisions = work_graph_unified_projection_enforcement_source_decisions();

    audits
        .into_iter()
        .filter(|source| {
            !source.has_unified_store_projection
                || !source.has_observability_timeline_projection
                || (source.requires_terminal_task_result && !source.has_task_result_projection)
        })
        .map(|source| {
            let decision = decisions
                .iter()
                .find(|decision| decision.source_surface_id == source.source_surface_id);
            source_gap(source, decision)
        })
        .collect()
}

pub fn work_graph_projection_adapter_closure_actions()
-> Vec<WorkGraphProjectionAdapterClosureActionPreview> {
    let mut actions = Vec::new();
    for source in work_graph_projection_adapter_source_gaps() {
        if source.missing_adapter_fixture {
            actions.push(closure_action(
                adapter_fixture_action_id(source.source_surface_id),
                source.source_surface_id,
                "adapter_projection_fixture",
                vec![],
                vec![],
                vec!["sourceSurfaceId", "nodeKind", "traceId", "fixtureHash"],
                vec!["adapter_projection_fixture_missing"],
            ));
        }
        if source.missing_unified_store_adapter {
            actions.push(closure_action(
                store_action_id(source.source_surface_id),
                source.source_surface_id,
                "unified_store_projection",
                store_collections_for_source(source.source_surface_id),
                vec![],
                vec!["traceId", "nodeId", "sourceSurfaceId", "redactionState"],
                vec!["unified_store_projection_missing"],
            ));
        }
        if source.missing_timeline_adapter {
            actions.push(closure_action(
                timeline_action_id(source.source_surface_id),
                source.source_surface_id,
                "observability_timeline_projection",
                vec!["timelineEvents"],
                timeline_events_for_source(source.source_surface_id),
                vec!["traceId", "nodeId", "eventKind", "evidenceRefs"],
                vec!["timeline_projection_missing"],
            ));
        }
        if source.missing_task_result_adapter {
            actions.push(closure_action(
                task_result_action_id(source.source_surface_id),
                source.source_surface_id,
                "task_result_projection",
                vec!["taskResults"],
                vec!["task_result_observed"],
                vec!["taskId", "status", "summaryHash", "evidenceRefs", "traceId"],
                vec!["task_result_projection_missing"],
            ));
        }
    }
    actions
}

pub fn work_graph_projection_adapter_closure_plans()
-> Vec<WorkGraphProjectionAdapterClosurePlanPreview> {
    let source_gaps = work_graph_projection_adapter_source_gaps();
    vec![
        closure_plan(
            "planning_projection_adapter_gap_closure",
            "P0",
            vec![
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
            ],
            &source_gaps,
            vec!["planning_identity_is_split_between_update_plan_and_plan_mode"],
            3,
            "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate",
        ),
        closure_plan(
            "multi_agent_mailbox_projection_adapter_gap_closure",
            "P0",
            vec!["multi_agent_v2_mailbox_wait"],
            &source_gaps,
            vec!["mailbox_wait_lacks_structured_task_result_join"],
            1,
            "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate",
        ),
        closure_plan(
            "multi_agent_reducer_projection_adapter_gap_closure",
            "P1",
            vec!["hepta_runtime_multi_agent_reducer"],
            &source_gaps,
            vec!["batch_and_worker_results_are_not_enforced_task_results"],
            1,
            "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
        ),
        closure_plan(
            "task_board_projection_adapter_gap_closure",
            "P1",
            vec!["hepta_runtime_task_board"],
            &source_gaps,
            vec!["task_board_has_admission_shape_without_unified_store_projection"],
            1,
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
        ),
        closure_plan(
            "approval_broker_projection_adapter_gap_closure",
            "P1",
            vec!["hepta_runtime_approval_broker"],
            &source_gaps,
            vec!["role_manifest_and_scheduler_admission_remain_preview_only"],
            1,
            "hepta_work_graph_state_store_persistence_preview_gate",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_blockers()
-> Vec<WorkGraphProjectionAdapterGapClosureBlockerPreview> {
    let source_gaps = work_graph_projection_adapter_source_gaps();
    vec![
        blocker(
            "gap_closure_is_preview_only",
            "medium",
            source_gaps
                .iter()
                .map(|gap| gap.source_surface_id)
                .collect(),
            "keep closure as a read-only plan until the closure readback gate verifies the adapter shapes",
        ),
        blocker(
            "adapter_fixture_closure_not_applied",
            "high",
            affected_sources(&source_gaps, |gap| gap.missing_adapter_fixture),
            "add fixture coverage for every newly closed source before treating reports as contract-ready",
        ),
        blocker(
            "unified_store_adapter_closure_not_applied",
            "high",
            affected_sources(&source_gaps, |gap| gap.missing_unified_store_adapter),
            "add deterministic node, edge, taskResult, artifact, or approval collection mapping for each store gap",
        ),
        blocker(
            "timeline_adapter_closure_not_applied",
            "high",
            affected_sources(&source_gaps, |gap| gap.missing_timeline_adapter),
            "add redacted timeline event mapping for each source that lacks observable trace events",
        ),
        blocker(
            "post_closure_enforcement_readiness_not_rerun",
            "high",
            source_gaps
                .iter()
                .map(|gap| gap.source_surface_id)
                .collect(),
            "rerun the unified projection enforcement-readiness gate after closure plans become adapter previews",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_unified_projection_enforcement_readiness_preview_gate",
    ]
}

impl WorkGraphProjectionAdapterGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            adapter_projection_enforced: false,
            closure_applied_to_runtime: false,
            append_only_store_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            timeline_persisted: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn source_gap(
    source: WorkGraphUnifiedProjectionSourceAudit,
    decision: Option<&WorkGraphProjectionEnforcementSourceDecisionPreview>,
) -> WorkGraphProjectionAdapterSourceGapPreview {
    let missing_adapter_fixture = !source.has_adapter_fixture;
    let missing_unified_store_adapter = !source.has_unified_store_projection;
    let missing_timeline_adapter = !source.has_observability_timeline_projection;
    let missing_task_result_adapter =
        source.requires_terminal_task_result && !source.has_task_result_projection;
    let current_blocker_ids = decision
        .map(|decision| decision.source_blocker_ids.clone())
        .unwrap_or_else(|| source.blocker_ids.clone());
    let enforcement_decision = decision
        .map(|decision| decision.enforcement_decision)
        .unwrap_or("deny_missing_unified_store_projection");
    let closure_action_ids = closure_action_ids_for(
        source.source_surface_id,
        missing_adapter_fixture,
        missing_unified_store_adapter,
        missing_timeline_adapter,
        missing_task_result_adapter,
    );

    WorkGraphProjectionAdapterSourceGapPreview {
        source_surface_id: source.source_surface_id,
        source_category: source.source_category,
        current_coverage_state: source.coverage_state,
        enforcement_decision,
        missing_adapter_fixture,
        missing_unified_store_adapter,
        missing_timeline_adapter,
        missing_task_result_adapter,
        closure_action_ids,
        current_blocker_ids,
        expected_post_closure_state: "contract_ready_preview_after_gap_closure",
    }
}

fn closure_action_ids_for(
    source_surface_id: &'static str,
    missing_adapter_fixture: bool,
    missing_unified_store_adapter: bool,
    missing_timeline_adapter: bool,
    missing_task_result_adapter: bool,
) -> Vec<&'static str> {
    let mut ids = Vec::new();
    if missing_adapter_fixture {
        ids.push(adapter_fixture_action_id(source_surface_id));
    }
    if missing_unified_store_adapter {
        ids.push(store_action_id(source_surface_id));
    }
    if missing_timeline_adapter {
        ids.push(timeline_action_id(source_surface_id));
    }
    if missing_task_result_adapter {
        ids.push(task_result_action_id(source_surface_id));
    }
    ids
}

fn closure_plan(
    id: &'static str,
    priority: &'static str,
    source_surface_ids: Vec<&'static str>,
    source_gaps: &[WorkGraphProjectionAdapterSourceGapPreview],
    closes_coverage_gap_ids: Vec<&'static str>,
    expected_contract_ready_source_count_after_closure: usize,
    next_gate: &'static str,
) -> WorkGraphProjectionAdapterClosurePlanPreview {
    let closure_action_ids = source_surface_ids
        .iter()
        .flat_map(|source_id| {
            source_gaps
                .iter()
                .find(|gap| gap.source_surface_id == *source_id)
                .map(|gap| gap.closure_action_ids.clone())
                .unwrap_or_default()
        })
        .collect();

    WorkGraphProjectionAdapterClosurePlanPreview {
        id,
        priority,
        source_surface_ids,
        closure_action_ids,
        closes_coverage_gap_ids,
        expected_contract_ready_source_count_after_closure,
        next_gate,
        mutates_runtime: false,
    }
}

fn closure_action(
    id: &'static str,
    source_surface_id: &'static str,
    adapter_kind: &'static str,
    projected_collection_ids: Vec<&'static str>,
    timeline_event_type_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
    closes_blocker_ids: Vec<&'static str>,
) -> WorkGraphProjectionAdapterClosureActionPreview {
    WorkGraphProjectionAdapterClosureActionPreview {
        id,
        source_surface_id,
        adapter_kind,
        projected_collection_ids,
        timeline_event_type_ids,
        required_evidence_fields,
        closes_blocker_ids,
        mutates_runtime: false,
        enforces_projection: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphProjectionAdapterGapClosureBlockerPreview {
    WorkGraphProjectionAdapterGapClosureBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn affected_sources(
    gaps: &[WorkGraphProjectionAdapterSourceGapPreview],
    predicate: impl Fn(&WorkGraphProjectionAdapterSourceGapPreview) -> bool,
) -> Vec<&'static str> {
    gaps.iter()
        .filter(|gap| predicate(gap))
        .map(|gap| gap.source_surface_id)
        .collect()
}

fn store_collections_for_source(source_surface_id: &str) -> Vec<&'static str> {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" | "app_server_turn_plan_notification" => {
            vec!["nodes", "edges", "timelineEvents"]
        }
        "multi_agent_v2_mailbox_wait" => vec!["edges", "timelineEvents"],
        "hepta_runtime_multi_agent_reducer" => vec!["nodes", "taskResults", "timelineEvents"],
        "hepta_runtime_task_board" => {
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"]
        }
        "hepta_runtime_approval_broker" => vec!["nodes", "approvals", "timelineEvents"],
        _ => vec!["nodes", "timelineEvents"],
    }
}

fn timeline_events_for_source(source_surface_id: &str) -> Vec<&'static str> {
    match source_surface_id {
        "update_plan_tool" | "plan_mode_proposed_plan_blocks" => vec!["plan_step_observed"],
        "hepta_runtime_multi_agent_reducer" => vec!["task_result_observed"],
        "hepta_runtime_task_board" => vec!["scheduler_admission_decision_observed"],
        "hepta_runtime_approval_broker" => vec!["approval_decision_observed"],
        _ => vec!["verification_gate_observed"],
    }
}

fn adapter_fixture_action_id(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => "close_plan_mode_adapter_projection_fixture",
        "app_server_turn_plan_notification" => "close_app_server_plan_adapter_projection_fixture",
        "multi_agent_v2_mailbox_wait" => "close_mailbox_wait_adapter_projection_fixture",
        "hepta_runtime_multi_agent_reducer" => {
            "close_multi_agent_reducer_adapter_projection_fixture"
        }
        "hepta_runtime_task_board" => "close_task_board_adapter_projection_fixture",
        _ => "close_adapter_projection_fixture",
    }
}

fn store_action_id(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => "close_plan_mode_unified_store_projection",
        "app_server_turn_plan_notification" => "close_app_server_plan_unified_store_projection",
        "multi_agent_v2_mailbox_wait" => "close_mailbox_wait_unified_store_projection",
        "hepta_runtime_multi_agent_reducer" => "close_multi_agent_reducer_unified_store_projection",
        "hepta_runtime_task_board" => "close_task_board_unified_store_projection",
        "hepta_runtime_approval_broker" => "close_approval_broker_unified_store_projection",
        _ => "close_unified_store_projection",
    }
}

fn timeline_action_id(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "update_plan_tool" => "close_update_plan_timeline_projection",
        "plan_mode_proposed_plan_blocks" => "close_plan_mode_timeline_projection",
        "hepta_runtime_multi_agent_reducer" => "close_multi_agent_reducer_timeline_projection",
        "hepta_runtime_task_board" => "close_task_board_timeline_projection",
        "hepta_runtime_approval_broker" => "close_approval_broker_timeline_projection",
        _ => "close_timeline_projection",
    }
}

fn task_result_action_id(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "hepta_runtime_multi_agent_reducer" => "close_multi_agent_reducer_task_result_projection",
        _ => "close_task_result_projection",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_adapter_gap_closure_covers_enforcement_gap_sources() {
        let report = hepta_work_graph_projection_adapter_gap_closure_preview_report();
        let source_ids = report
            .source_gaps
            .iter()
            .map(|gap| gap.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_ids,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
                "hepta_runtime_approval_broker",
            ]
        );
        assert_eq!(report.source_gap_count, 7);
        assert!(report.source_gaps.iter().all(
            |gap| gap.expected_post_closure_state == "contract_ready_preview_after_gap_closure"
        ));
    }

    #[test]
    fn projection_adapter_gap_closure_splits_store_timeline_and_fixture_actions() {
        let report = hepta_work_graph_projection_adapter_gap_closure_preview_report();
        let action_kinds = report
            .closure_actions
            .iter()
            .map(|action| action.adapter_kind)
            .collect::<Vec<_>>();

        assert_eq!(report.closure_action_count, 11);
        assert_eq!(report.store_adapter_closure_count, 6);
        assert_eq!(report.timeline_adapter_closure_count, 5);
        assert_eq!(report.adapter_fixture_closure_count, 0);
        assert_eq!(report.task_result_adapter_closure_count, 0);
        assert!(action_kinds.contains(&"unified_store_projection"));
        assert!(action_kinds.contains(&"observability_timeline_projection"));
        assert!(!action_kinds.contains(&"adapter_projection_fixture"));
        assert!(
            report
                .closure_actions
                .iter()
                .all(|action| !action.mutates_runtime && !action.enforces_projection)
        );
    }

    #[test]
    fn projection_adapter_gap_closure_declares_ordered_closure_plans() {
        let report = hepta_work_graph_projection_adapter_gap_closure_preview_report();
        let plan_ids = report
            .closure_plans
            .iter()
            .map(|plan| plan.id)
            .collect::<Vec<_>>();

        assert_eq!(
            plan_ids,
            [
                "planning_projection_adapter_gap_closure",
                "multi_agent_mailbox_projection_adapter_gap_closure",
                "multi_agent_reducer_projection_adapter_gap_closure",
                "task_board_projection_adapter_gap_closure",
                "approval_broker_projection_adapter_gap_closure",
            ]
        );
        assert_eq!(report.closure_plan_count, 5);
        assert_eq!(report.closure_plans[0].closure_action_ids.len(), 4);
        assert!(
            report
                .closure_plans
                .iter()
                .all(|plan| !plan.mutates_runtime)
        );
    }

    #[test]
    fn projection_adapter_gap_closure_keeps_enforcement_blocked() {
        let report = hepta_work_graph_projection_adapter_gap_closure_preview_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "gap_closure_is_preview_only",
                "adapter_fixture_closure_not_applied",
                "unified_store_adapter_closure_not_applied",
                "timeline_adapter_closure_not_applied",
                "post_closure_enforcement_readiness_not_rerun",
            ]
        );
        assert_eq!(report.blocker_count, 5);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn projection_adapter_gap_closure_keeps_all_side_effects_disabled() {
        let report = hepta_work_graph_projection_adapter_gap_closure_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphProjectionAdapterGapClosurePreviewSideEffects::none()
        );
        assert!(report.ready_for_projection_adapter_gap_closure_readback_preview);
        assert_eq!(
            report.required_prior_gate_count,
            work_graph_projection_adapter_gap_closure_required_prior_gates().len()
        );
        assert_eq!(report.required_prior_gate_count, 13);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
    }
}
