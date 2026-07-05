use serde::Serialize;

use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview;
use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::WorkGraphStoreIdempotencyGuardReadbackPlanPreview;
use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::work_graph_store_idempotency_guard_gap_closure_readback_plans;
use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::work_graph_store_idempotency_guard_gap_closure_readback_required_prior_gates;
use crate::work_graph_store_idempotency_guard_gap_closure_readback_preview::work_graph_store_idempotency_guard_probe_binding_readback_assertions;

pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_gate";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_store_idempotency_guard_gap_closure_application_preview_v1";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub source_store_guard_contract_ready_preview_count: usize,
    pub application_group_count: usize,
    pub expected_collection_ref_count: usize,
    pub readback_probe_contract_ref_count: usize,
    pub readback_evidence_field_ref_count: usize,
    pub task_result_guard_dependency_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphStoreIdempotencyGuardApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview>,
    pub application_groups: Vec<WorkGraphStoreIdempotencyGuardApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphStoreIdempotencyGuardApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphStoreIdempotencyGuardApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_store_guard_rerun_preview: bool,
    pub ready_for_runtime_guard_application: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardApplicationPlanPreview {
    pub application_plan_id: &'static str,
    pub readback_plan_id: &'static str,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub candidate_guard_id: &'static str,
    pub key_formula_assertion_id: &'static str,
    pub collision_policy_assertion_id: &'static str,
    pub probe_binding_assertion_id: &'static str,
    pub collection_ref_assertion_id: &'static str,
    pub application_scope: &'static str,
    pub expected_key_fields: Vec<&'static str>,
    pub expected_collection_ids: Vec<&'static str>,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub readback_evidence_fields: Vec<&'static str>,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub mutates_idempotency_index: bool,
    pub persists_state_store_guard: bool,
    pub enables_append_only_store: bool,
    pub enforces_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub candidate_guard_id: &'static str,
    pub application_plan_id: &'static str,
    pub expected_collection_ids: Vec<&'static str>,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub post_application_store_guard_state: &'static str,
    pub store_idempotency_guard_ready_preview: bool,
    pub ready_for_enforcement_readiness_store_guard_rerun: bool,
    pub ready_for_projection_enforcement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<&'static str>,
    pub expected_store_guard_ready_source_count_after_application: usize,
    pub mutates_runtime: bool,
    pub mutates_idempotency_index: bool,
    pub enables_append_only_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_projection_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_performed: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report()
-> WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewReport {
    let readback_plans = work_graph_store_idempotency_guard_gap_closure_readback_plans();
    let application_plans = work_graph_store_idempotency_guard_gap_closure_application_plans();
    let source_outcomes =
        work_graph_store_idempotency_guard_gap_closure_application_source_outcomes();
    let application_groups = work_graph_store_idempotency_guard_gap_closure_application_groups();
    let application_guards = work_graph_store_idempotency_guard_gap_closure_application_guards();
    let blockers = work_graph_store_idempotency_guard_gap_closure_application_blockers();
    let required_prior_gates =
        work_graph_store_idempotency_guard_gap_closure_application_required_prior_gates();

    WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_store_idempotency_guard_gap_closure_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        source_store_guard_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.store_idempotency_guard_ready_preview)
            .count(),
        application_group_count: application_groups.len(),
        expected_collection_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_collection_ids.len())
            .sum(),
        readback_probe_contract_ref_count: application_plans
            .iter()
            .map(|plan| plan.readback_probe_contract_ids.len())
            .sum(),
        readback_evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.readback_evidence_fields.len())
            .sum(),
        task_result_guard_dependency_count: application_plans
            .iter()
            .filter(|plan| requires_task_result_guard(plan.source_surface_id))
            .count(),
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
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_store_guard_rerun_preview: true,
        ready_for_runtime_guard_application: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_plans()
-> Vec<WorkGraphStoreIdempotencyGuardApplicationPlanPreview> {
    work_graph_store_idempotency_guard_gap_closure_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_source_outcomes()
-> Vec<WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview> {
    work_graph_store_idempotency_guard_gap_closure_application_plans()
        .into_iter()
        .map(source_outcome)
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_groups()
-> Vec<WorkGraphStoreIdempotencyGuardApplicationGroupPreview> {
    let plans = work_graph_store_idempotency_guard_gap_closure_application_plans();
    vec![
        application_group(
            "planning_store_idempotency_guard_application",
            "p0",
            vec![
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
            ],
            &plans,
        ),
        application_group(
            "multi_agent_store_idempotency_guard_application",
            "p0",
            vec![
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
            ],
            &plans,
        ),
        application_group(
            "task_board_store_idempotency_guard_application",
            "p0",
            vec!["hepta_runtime_task_board"],
            &plans,
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_guards()
-> Vec<WorkGraphStoreIdempotencyGuardApplicationGuardPreview> {
    vec![
        application_guard("runtime_guard_attachment_disabled", "critical", "runtime"),
        application_guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_index",
        ),
        application_guard(
            "state_store_guard_persistence_disabled",
            "critical",
            "state_store",
        ),
        application_guard(
            "append_only_store_enablement_disabled",
            "critical",
            "append_only_store",
        ),
        application_guard("task_result_enforcement_disabled", "high", "task_result"),
        application_guard("operator_review_required", "high", "operator_review"),
        application_guard(
            "enforcement_readiness_store_guard_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_blockers()
-> Vec<WorkGraphStoreIdempotencyGuardApplicationBlockerPreview> {
    let plans = work_graph_store_idempotency_guard_gap_closure_application_plans();
    let all_sources = affected_sources(&plans, |_| true);

    vec![
        blocker(
            "store_guard_application_is_preview_only",
            "medium",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep store guard application as a no-mutation preview until readiness rerun proves the blocker moved",
        ),
        blocker(
            "runtime_guard_application_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "attach store idempotency guards to runtime adapters only after operator review and persistence gates are promoted",
        ),
        blocker(
            "idempotency_index_mutation_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "do not mutate idempotency indexes until collision handling and replay evidence are enforced",
        ),
        blocker(
            "state_store_guard_persistence_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep candidate guard rows preview-only until append-only store intake is promoted",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "do not allow append-only writes until store guard application is promoted and rerun confirms readiness",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "high",
            affected_sources(&plans, |plan| {
                requires_task_result_guard(plan.source_surface_id)
            }),
            application_plan_ids(&plans, |plan| {
                requires_task_result_guard(plan.source_surface_id)
            }),
            "enforce terminal TaskResult output before promoting reducer and task_board store guards",
        ),
        blocker(
            "enforcement_readiness_store_guard_rerun_missing",
            "high",
            all_sources,
            application_plan_ids(&plans, |_| true),
            "rerun unified projection enforcement-readiness against the store guard application preview outcomes",
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_store_idempotency_guard_gap_closure_readback_required_prior_gates();
    gates.push(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_performed: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn application_plan(
    readback_plan: WorkGraphStoreIdempotencyGuardReadbackPlanPreview,
) -> WorkGraphStoreIdempotencyGuardApplicationPlanPreview {
    let probe_binding = probe_binding_for_source(readback_plan.source_surface_id);
    WorkGraphStoreIdempotencyGuardApplicationPlanPreview {
        application_plan_id: application_plan_id_for_source(readback_plan.source_surface_id),
        readback_plan_id: readback_plan.id,
        source_surface_id: readback_plan.source_surface_id,
        source_category: source_category_for_source(readback_plan.source_surface_id),
        candidate_guard_id: readback_plan.candidate_guard_id,
        key_formula_assertion_id: readback_plan.key_formula_assertion_id,
        collision_policy_assertion_id: readback_plan.collision_policy_assertion_id,
        probe_binding_assertion_id: readback_plan.probe_binding_assertion_id,
        collection_ref_assertion_id: readback_plan.collection_ref_assertion_id,
        application_scope: "store_idempotency_guard_runtime_binding",
        expected_key_fields: readback_plan.expected_key_fields,
        expected_collection_ids: readback_plan.expected_collection_ids,
        readback_probe_contract_ids: readback_plan.readback_probe_contract_ids,
        readback_evidence_fields: probe_binding.readback_evidence_fields,
        application_state: "preview_application_defined_runtime_guard_not_attached",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        mutates_idempotency_index: false,
        persists_state_store_guard: false,
        enables_append_only_store: false,
        enforces_projection: false,
    }
}

fn source_outcome(
    plan: WorkGraphStoreIdempotencyGuardApplicationPlanPreview,
) -> WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview {
    WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        candidate_guard_id: plan.candidate_guard_id,
        application_plan_id: plan.application_plan_id,
        expected_collection_ids: plan.expected_collection_ids,
        readback_probe_contract_ids: plan.readback_probe_contract_ids,
        post_application_store_guard_state: "store_guard_contract_ready_preview_after_application",
        store_idempotency_guard_ready_preview: true,
        ready_for_enforcement_readiness_store_guard_rerun: true,
        ready_for_projection_enforcement: false,
        applies_to_runtime: false,
    }
}

fn application_group(
    id: &'static str,
    priority: &'static str,
    source_surface_ids: Vec<&'static str>,
    plans: &[WorkGraphStoreIdempotencyGuardApplicationPlanPreview],
) -> WorkGraphStoreIdempotencyGuardApplicationGroupPreview {
    let application_plan_ids = plans
        .iter()
        .filter(|plan| source_surface_ids.contains(&plan.source_surface_id))
        .map(|plan| plan.application_plan_id)
        .collect::<Vec<_>>();
    WorkGraphStoreIdempotencyGuardApplicationGroupPreview {
        id,
        priority,
        expected_store_guard_ready_source_count_after_application: source_surface_ids.len(),
        source_surface_ids,
        application_plan_ids,
        mutates_runtime: false,
        mutates_idempotency_index: false,
        enables_append_only_store: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphStoreIdempotencyGuardApplicationGuardPreview {
    WorkGraphStoreIdempotencyGuardApplicationGuardPreview {
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
) -> WorkGraphStoreIdempotencyGuardApplicationBlockerPreview {
    WorkGraphStoreIdempotencyGuardApplicationBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn probe_binding_for_source(
    source_surface_id: &str,
) -> WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview {
    work_graph_store_idempotency_guard_probe_binding_readback_assertions()
        .into_iter()
        .find(|assertion| assertion.source_surface_id == source_surface_id)
        .unwrap_or_else(|| panic!("missing probe binding assertion for {source_surface_id}"))
}

fn application_plan_ids(
    plans: &[WorkGraphStoreIdempotencyGuardApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphStoreIdempotencyGuardApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id)
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphStoreIdempotencyGuardApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphStoreIdempotencyGuardApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        if !source_ids.contains(&plan.source_surface_id) {
            source_ids.push(plan.source_surface_id);
        }
    }
    source_ids
}

fn source_category_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" | "app_server_turn_plan_notification" => "planning",
        "multi_agent_v2_mailbox_wait" | "hepta_runtime_multi_agent_reducer" => "multi_agent",
        "hepta_runtime_task_board" => "runtime_scheduler",
        _ => "unknown",
    }
}

fn requires_task_result_guard(source_surface_id: &str) -> bool {
    matches!(
        source_surface_id,
        "hepta_runtime_multi_agent_reducer" | "hepta_runtime_task_board"
    )
}

fn application_plan_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "apply_plan_mode_proposed_plan_blocks_store_idempotency_guard_preview"
        }
        "app_server_turn_plan_notification" => {
            "apply_app_server_turn_plan_notification_store_idempotency_guard_preview"
        }
        "multi_agent_v2_mailbox_wait" => {
            "apply_multi_agent_v2_mailbox_wait_store_idempotency_guard_preview"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "apply_hepta_runtime_multi_agent_reducer_store_idempotency_guard_preview"
        }
        "hepta_runtime_task_board" => {
            "apply_hepta_runtime_task_board_store_idempotency_guard_preview"
        }
        _ => "apply_unknown_store_idempotency_guard_preview",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_idempotency_guard_application_covers_readback_verified_plans() {
        let report =
            hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report();
        let source_surface_ids = report
            .application_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_surface_ids,
            [
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert_eq!(report.status, "ready");
        assert_eq!(report.readback_plan_count, 5);
        assert_eq!(report.application_plan_count, 5);
        assert_eq!(report.source_outcome_count, 5);
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn store_idempotency_guard_application_preserves_no_mutation_boundary() {
        let report =
            hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report();

        assert_eq!(report.expected_collection_ref_count, 14);
        assert_eq!(report.readback_probe_contract_ref_count, 14);
        assert_eq!(report.readback_evidence_field_ref_count, 39);
        assert_eq!(report.task_result_guard_dependency_count, 2);
        assert!(report.application_plans.iter().all(|plan| {
            !plan.applies_to_runtime
                && !plan.mutates_idempotency_index
                && !plan.persists_state_store_guard
                && !plan.enables_append_only_store
                && !plan.enforces_projection
        }));
    }

    #[test]
    fn store_idempotency_guard_application_marks_sources_ready_for_rerun_only() {
        let report =
            hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report();
        let task_board = report
            .source_outcomes
            .iter()
            .find(|outcome| outcome.source_surface_id == "hepta_runtime_task_board")
            .expect("task board outcome");

        assert_eq!(report.source_store_guard_contract_ready_preview_count, 5);
        assert!(report.source_outcomes.iter().all(|outcome| {
            outcome.store_idempotency_guard_ready_preview
                && outcome.ready_for_enforcement_readiness_store_guard_rerun
                && !outcome.ready_for_projection_enforcement
                && !outcome.applies_to_runtime
        }));
        assert_eq!(
            task_board.expected_collection_ids,
            ["nodes", "taskResults", "artifacts", "timelineEvents"]
        );
        assert_eq!(
            task_board.readback_probe_contract_ids,
            [
                "task_board_nodes_readback_probe",
                "task_board_task_results_readback_probe",
                "task_board_artifacts_readback_probe",
                "task_board_timeline_readback_probe"
            ]
        );
    }

    #[test]
    fn store_idempotency_guard_application_declares_groups_guards_and_blockers() {
        let report =
            hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report();
        let group_counts = report
            .application_groups
            .iter()
            .map(|group| (group.id, group.application_plan_ids.len()))
            .collect::<Vec<_>>();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            group_counts,
            [
                ("planning_store_idempotency_guard_application", 2),
                ("multi_agent_store_idempotency_guard_application", 2),
                ("task_board_store_idempotency_guard_application", 1),
            ]
        );
        assert_eq!(report.application_group_count, 3);
        assert_eq!(report.application_guard_count, 7);
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_projection_enforcement && !guard.satisfied_by_preview
        }));
        assert_eq!(
            blocker_counts,
            [
                ("store_guard_application_is_preview_only", 5),
                ("runtime_guard_application_disabled", 5),
                ("idempotency_index_mutation_disabled", 5),
                ("state_store_guard_persistence_disabled", 5),
                ("append_only_store_enablement_disabled", 5),
                ("terminal_task_result_enforcement_disabled", 2),
                ("enforcement_readiness_store_guard_rerun_missing", 5),
            ]
        );
        assert_eq!(report.blocker_count, 7);
    }

    #[test]
    fn store_idempotency_guard_application_advances_only_to_store_guard_readiness_rerun() {
        let report =
            hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_report();

        assert_eq!(report.required_prior_gate_count, 19);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report.ready_for_unified_projection_enforcement_readiness_store_guard_rerun_preview
        );
        assert!(!report.ready_for_runtime_guard_application);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphStoreIdempotencyGuardGapClosureApplicationPreviewSideEffects::none()
        );
    }
}
