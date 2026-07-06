use serde::Serialize;

use crate::work_graph_projection_adapter_gap_closure_preview::work_graph_projection_adapter_closure_plans;
use crate::work_graph_projection_adapter_gap_closure_preview::work_graph_projection_adapter_source_gaps;
use crate::work_graph_projection_adapter_gap_closure_readback_preview::WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_projection_adapter_gap_closure_readback_preview::WorkGraphProjectionAdapterClosureReadbackPlanPreview;
use crate::work_graph_projection_adapter_gap_closure_readback_preview::work_graph_projection_adapter_gap_closure_readback_plans;
use crate::work_graph_projection_adapter_gap_closure_readback_preview::work_graph_projection_adapter_gap_closure_readback_required_prior_gates;

pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_application_preview_gate";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_projection_adapter_gap_closure_application_preview_v1";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_gap_count: usize,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub fixture_application_count: usize,
    pub store_projection_application_count: usize,
    pub timeline_projection_application_count: usize,
    pub source_outcome_count: usize,
    pub source_contract_ready_preview_count: usize,
    pub application_group_count: usize,
    pub projected_collection_reference_count: usize,
    pub timeline_event_type_reference_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphProjectionAdapterClosureApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphProjectionAdapterClosureSourceOutcomePreview>,
    pub application_groups: Vec<WorkGraphProjectionAdapterClosureApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphProjectionAdapterClosureApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphProjectionAdapterClosureApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_rerun_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphProjectionAdapterGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureApplicationPlanPreview {
    pub application_plan_id: &'static str,
    pub closure_action_id: &'static str,
    pub source_surface_id: &'static str,
    pub adapter_kind: &'static str,
    pub application_scope: &'static str,
    pub expected_projected_collection_ids: Vec<&'static str>,
    pub expected_timeline_event_type_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub mutates_store: bool,
    pub persists_timeline: bool,
    pub enforces_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub closure_action_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<&'static str>,
    pub projected_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub fixture_application_required: bool,
    pub store_projection_application_required: bool,
    pub timeline_projection_application_required: bool,
    pub post_application_coverage_state: &'static str,
    pub ready_for_enforcement_readiness_rerun: bool,
    pub ready_for_projection_enforcement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub closure_action_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<&'static str>,
    pub expected_contract_ready_source_count_after_application: usize,
    pub mutates_runtime: bool,
    pub enforces_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_projection_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosureApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub closure_applied_to_runtime: bool,
    pub append_only_store_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub readback_performed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_projection_adapter_gap_closure_application_preview_report()
-> WorkGraphProjectionAdapterGapClosureApplicationPreviewReport {
    let source_gaps = work_graph_projection_adapter_source_gaps();
    let readback_plans = work_graph_projection_adapter_gap_closure_readback_plans();
    let application_plans = work_graph_projection_adapter_gap_closure_application_plans();
    let source_outcomes = work_graph_projection_adapter_gap_closure_application_source_outcomes();
    let application_groups = work_graph_projection_adapter_gap_closure_application_groups();
    let application_guards = work_graph_projection_adapter_gap_closure_application_guards();
    let blockers = work_graph_projection_adapter_gap_closure_application_blockers();
    let required_prior_gates =
        work_graph_projection_adapter_gap_closure_application_required_prior_gates();

    WorkGraphProjectionAdapterGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_projection_adapter_gap_closure_application_preview_no_runtime_mutation",
        source_gap_count: source_gaps.len(),
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        fixture_application_count: count_application_kind(
            &application_plans,
            "adapter_projection_fixture",
        ),
        store_projection_application_count: count_application_kind(
            &application_plans,
            "unified_store_projection",
        ),
        timeline_projection_application_count: count_application_kind(
            &application_plans,
            "observability_timeline_projection",
        ),
        source_outcome_count: source_outcomes.len(),
        source_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| {
                outcome.post_application_coverage_state
                    == "contract_ready_preview_after_application"
            })
            .count(),
        application_group_count: application_groups.len(),
        projected_collection_reference_count: application_plans
            .iter()
            .filter(|plan| plan.adapter_kind == "unified_store_projection")
            .map(|plan| plan.expected_projected_collection_ids.len())
            .sum(),
        timeline_event_type_reference_count: application_plans
            .iter()
            .filter(|plan| plan.adapter_kind == "observability_timeline_projection")
            .map(|plan| plan.expected_timeline_event_type_ids.len())
            .sum(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        application_plans,
        source_outcomes,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_rerun_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphProjectionAdapterGapClosureApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_projection_adapter_gap_closure_application_plans()
-> Vec<WorkGraphProjectionAdapterClosureApplicationPlanPreview> {
    work_graph_projection_adapter_gap_closure_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_application_source_outcomes()
-> Vec<WorkGraphProjectionAdapterClosureSourceOutcomePreview> {
    let plans = work_graph_projection_adapter_gap_closure_application_plans();
    work_graph_projection_adapter_source_gaps()
        .into_iter()
        .map(|source| source_outcome(source.source_surface_id, &plans))
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_application_groups()
-> Vec<WorkGraphProjectionAdapterClosureApplicationGroupPreview> {
    work_graph_projection_adapter_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphProjectionAdapterClosureApplicationGroupPreview {
                id: application_group_id(plan.id),
                priority: plan.priority,
                source_surface_ids: plan.source_surface_ids,
                application_plan_ids: plan.closure_action_ids.clone(),
                closure_action_ids: plan.closure_action_ids,
                expected_contract_ready_source_count_after_application: plan
                    .expected_contract_ready_source_count_after_closure,
                mutates_runtime: false,
                enforces_projection: false,
            },
        )
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_application_guards()
-> Vec<WorkGraphProjectionAdapterClosureApplicationGuardPreview> {
    vec![
        application_guard("runtime_attachment_disabled", "critical", "runtime"),
        application_guard("store_mutation_disabled", "critical", "unified_store"),
        application_guard("timeline_persistence_disabled", "critical", "timeline"),
        application_guard("projection_enforcement_disabled", "critical", "projection"),
        application_guard("task_result_enforcement_disabled", "high", "task_result"),
        application_guard(
            "enforcement_readiness_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_application_blockers()
-> Vec<WorkGraphProjectionAdapterClosureApplicationBlockerPreview> {
    let plans = work_graph_projection_adapter_gap_closure_application_plans();
    let source_outcomes = work_graph_projection_adapter_gap_closure_application_source_outcomes();
    let all_sources = source_outcomes
        .iter()
        .map(|outcome| outcome.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        blocker(
            "gap_closure_application_is_preview_only",
            "medium",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep application as a no-mutation preview until enforcement-readiness rerun confirms contract-ready projection",
        ),
        blocker(
            "runtime_closure_application_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "attach adapter closures to runtime only after operator review and store/timeline guards are satisfied",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            affected_sources(&plans, |plan| {
                plan.adapter_kind == "unified_store_projection"
            }),
            application_plan_ids(&plans, |plan| {
                plan.adapter_kind == "unified_store_projection"
            }),
            "keep store writes disabled until append-only store enablement has its own promotion gate",
        ),
        blocker(
            "timeline_persistence_disabled",
            "high",
            affected_sources(&plans, |plan| {
                plan.adapter_kind == "observability_timeline_projection"
            }),
            application_plan_ids(&plans, |plan| {
                plan.adapter_kind == "observability_timeline_projection"
            }),
            "keep timeline persistence disabled until redaction and event ordering are enforced",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "high",
            vec![
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ],
            application_plan_ids(&plans, |plan| {
                plan.source_surface_id == "hepta_runtime_multi_agent_reducer"
                    || plan.source_surface_id == "hepta_runtime_task_board"
            }),
            "enforce terminal TaskResult output before promoting reducer and task_board adapter closures",
        ),
        blocker(
            "enforcement_readiness_rerun_missing",
            "high",
            all_sources,
            application_plan_ids(&plans, |_| true),
            "rerun unified projection enforcement-readiness against the application preview outcomes",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_projection_adapter_gap_closure_readback_required_prior_gates();
    gates.push(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphProjectionAdapterGapClosureApplicationPreviewSideEffects {
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
            readback_performed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn application_plan(
    readback_plan: WorkGraphProjectionAdapterClosureReadbackPlanPreview,
) -> WorkGraphProjectionAdapterClosureApplicationPlanPreview {
    WorkGraphProjectionAdapterClosureApplicationPlanPreview {
        application_plan_id: readback_plan.closure_action_id,
        closure_action_id: readback_plan.closure_action_id,
        source_surface_id: readback_plan.source_surface_id,
        adapter_kind: readback_plan.adapter_kind,
        application_scope: readback_plan.readback_scope,
        expected_projected_collection_ids: readback_plan.expected_projected_collection_ids,
        expected_timeline_event_type_ids: readback_plan.expected_timeline_event_type_ids,
        required_evidence_fields: readback_plan.required_evidence_fields,
        application_state: "preview_application_defined_runtime_not_mutated",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        mutates_store: false,
        persists_timeline: false,
        enforces_projection: false,
    }
}

fn source_outcome(
    source_surface_id: &'static str,
    plans: &[WorkGraphProjectionAdapterClosureApplicationPlanPreview],
) -> WorkGraphProjectionAdapterClosureSourceOutcomePreview {
    let source_plans = plans
        .iter()
        .filter(|plan| plan.source_surface_id == source_surface_id)
        .collect::<Vec<_>>();
    let closure_action_ids = source_plans
        .iter()
        .map(|plan| plan.closure_action_id)
        .collect::<Vec<_>>();
    let application_plan_ids = source_plans
        .iter()
        .map(|plan| plan.application_plan_id)
        .collect::<Vec<_>>();
    let mut projected_collection_ids = Vec::new();
    let mut timeline_event_type_ids = Vec::new();
    for plan in &source_plans {
        push_unique_all(
            &mut projected_collection_ids,
            &plan.expected_projected_collection_ids,
        );
        push_unique_all(
            &mut timeline_event_type_ids,
            &plan.expected_timeline_event_type_ids,
        );
    }

    WorkGraphProjectionAdapterClosureSourceOutcomePreview {
        source_surface_id,
        closure_action_ids,
        application_plan_ids,
        projected_collection_ids,
        timeline_event_type_ids,
        fixture_application_required: source_plans
            .iter()
            .any(|plan| plan.adapter_kind == "adapter_projection_fixture"),
        store_projection_application_required: source_plans
            .iter()
            .any(|plan| plan.adapter_kind == "unified_store_projection"),
        timeline_projection_application_required: source_plans
            .iter()
            .any(|plan| plan.adapter_kind == "observability_timeline_projection"),
        post_application_coverage_state: "contract_ready_preview_after_application",
        ready_for_enforcement_readiness_rerun: true,
        ready_for_projection_enforcement: false,
        applies_to_runtime: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphProjectionAdapterClosureApplicationGuardPreview {
    WorkGraphProjectionAdapterClosureApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_projection_enforcement: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphProjectionAdapterClosureApplicationBlockerPreview {
    WorkGraphProjectionAdapterClosureApplicationBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn application_group_id(closure_plan_id: &str) -> &'static str {
    match closure_plan_id {
        "planning_projection_adapter_gap_closure" => {
            "planning_projection_adapter_gap_closure_application"
        }
        "multi_agent_mailbox_projection_adapter_gap_closure" => {
            "multi_agent_mailbox_projection_adapter_gap_closure_application"
        }
        "multi_agent_reducer_projection_adapter_gap_closure" => {
            "multi_agent_reducer_projection_adapter_gap_closure_application"
        }
        "task_board_projection_adapter_gap_closure" => {
            "task_board_projection_adapter_gap_closure_application"
        }
        "approval_broker_projection_adapter_gap_closure" => {
            "approval_broker_projection_adapter_gap_closure_application"
        }
        _ => "projection_adapter_gap_closure_application",
    }
}

fn count_application_kind(
    plans: &[WorkGraphProjectionAdapterClosureApplicationPlanPreview],
    adapter_kind: &str,
) -> usize {
    plans
        .iter()
        .filter(|plan| plan.adapter_kind == adapter_kind)
        .count()
}

fn application_plan_ids(
    plans: &[WorkGraphProjectionAdapterClosureApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphProjectionAdapterClosureApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id)
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphProjectionAdapterClosureApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphProjectionAdapterClosureApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        push_unique(&mut source_ids, plan.source_surface_id);
    }
    source_ids
}

fn push_unique_all(target: &mut Vec<&'static str>, values: &[&'static str]) {
    for value in values {
        push_unique(target, value);
    }
}

fn push_unique(target: &mut Vec<&'static str>, value: &'static str) {
    if !target.contains(&value) {
        target.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_adapter_gap_closure_application_covers_readback_verified_plans() {
        let report = hepta_work_graph_projection_adapter_gap_closure_application_preview_report();
        let plan_ids = report
            .application_plans
            .iter()
            .map(|plan| plan.application_plan_id)
            .collect::<Vec<_>>();

        assert_eq!(report.status, "ready");
        assert_eq!(report.source_gap_count, 7);
        assert_eq!(report.readback_plan_count, 11);
        assert_eq!(report.application_plan_count, 11);
        assert!(plan_ids.contains(&"close_update_plan_timeline_projection"));
        assert!(plan_ids.contains(&"close_task_board_unified_store_projection"));
        assert!(plan_ids.contains(&"close_approval_broker_timeline_projection"));
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn projection_adapter_gap_closure_application_splits_application_kinds() {
        let report = hepta_work_graph_projection_adapter_gap_closure_application_preview_report();

        assert_eq!(report.fixture_application_count, 0);
        assert_eq!(report.store_projection_application_count, 6);
        assert_eq!(report.timeline_projection_application_count, 5);
        assert_eq!(report.projected_collection_reference_count, 18);
        assert_eq!(report.timeline_event_type_reference_count, 5);
        assert!(report.application_plans.iter().all(|plan| {
            !plan.applies_to_runtime
                && !plan.mutates_store
                && !plan.persists_timeline
                && !plan.enforces_projection
        }));
    }

    #[test]
    fn projection_adapter_gap_closure_application_marks_all_sources_contract_ready_preview() {
        let report = hepta_work_graph_projection_adapter_gap_closure_application_preview_report();
        let task_board = report
            .source_outcomes
            .iter()
            .find(|outcome| outcome.source_surface_id == "hepta_runtime_task_board")
            .expect("task board outcome");

        assert_eq!(report.source_outcome_count, 7);
        assert_eq!(report.source_contract_ready_preview_count, 7);
        assert!(report.source_outcomes.iter().all(|outcome| {
            outcome.ready_for_enforcement_readiness_rerun
                && !outcome.ready_for_projection_enforcement
                && !outcome.applies_to_runtime
        }));
        assert_eq!(
            task_board.projected_collection_ids,
            ["nodes", "taskResults", "artifacts", "timelineEvents"]
        );
        assert_eq!(
            task_board.timeline_event_type_ids,
            ["scheduler_admission_decision_observed"]
        );
    }

    #[test]
    fn projection_adapter_gap_closure_application_declares_groups_guards_and_blockers() {
        let report = hepta_work_graph_projection_adapter_gap_closure_application_preview_report();
        let group_ids = report
            .application_groups
            .iter()
            .map(|group| group.id)
            .collect::<Vec<_>>();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            group_ids,
            [
                "planning_projection_adapter_gap_closure_application",
                "multi_agent_mailbox_projection_adapter_gap_closure_application",
                "multi_agent_reducer_projection_adapter_gap_closure_application",
                "task_board_projection_adapter_gap_closure_application",
                "approval_broker_projection_adapter_gap_closure_application",
            ]
        );
        assert_eq!(report.application_group_count, 5);
        assert_eq!(report.application_groups[0].application_plan_ids.len(), 4);
        assert_eq!(report.application_guard_count, 6);
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_projection_enforcement && !guard.satisfied_by_preview
        }));
        assert_eq!(
            blocker_ids,
            [
                "gap_closure_application_is_preview_only",
                "runtime_closure_application_disabled",
                "append_only_store_enablement_disabled",
                "timeline_persistence_disabled",
                "terminal_task_result_enforcement_disabled",
                "enforcement_readiness_rerun_missing",
            ]
        );
        assert_eq!(report.blocker_count, 6);
    }

    #[test]
    fn projection_adapter_gap_closure_application_advances_only_to_readiness_rerun() {
        let report = hepta_work_graph_projection_adapter_gap_closure_application_preview_report();

        assert_eq!(report.required_prior_gate_count, 15);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_unified_projection_enforcement_readiness_rerun_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphProjectionAdapterGapClosureApplicationPreviewSideEffects::none()
        );
    }
}
