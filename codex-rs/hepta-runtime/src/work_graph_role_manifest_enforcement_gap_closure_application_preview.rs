use serde::Serialize;

use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::WorkGraphRoleManifestBlockerMappingAssertionPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::WorkGraphRoleManifestClosureReadbackPlanPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::WorkGraphRoleManifestReadbackBlockerPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::work_graph_role_manifest_blocker_mapping_assertions;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::work_graph_role_manifest_enforcement_gap_closure_readback_blockers;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::work_graph_role_manifest_enforcement_gap_closure_readback_plans;
use crate::work_graph_role_manifest_enforcement_gap_closure_readback_preview::work_graph_role_manifest_enforcement_gap_closure_readback_required_prior_gates;

pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_gate";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_role_manifest_enforcement_gap_closure_application_preview_v1";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub role_manifest_contract_ready_preview_count: usize,
    pub blocker_application_count: usize,
    pub application_group_count: usize,
    pub role_binding_ref_count: usize,
    pub capability_ref_count: usize,
    pub permission_mode_ref_count: usize,
    pub manifest_field_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphRoleManifestApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphRoleManifestApplicationSourceOutcomePreview>,
    pub blocker_applications: Vec<WorkGraphRoleManifestBlockerMappingApplicationPreview>,
    pub application_groups: Vec<WorkGraphRoleManifestApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphRoleManifestApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphRoleManifestApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub projected_role_kind: &'static str,
    pub role_blocker_id: &'static str,
    pub readback_probe_id: String,
    pub role_binding_ids: Vec<String>,
    pub capability_ids: Vec<&'static str>,
    pub tool_permission_mode_ids: Vec<&'static str>,
    pub covered_wire_fields: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub enforces_role_manifest: bool,
    pub changes_tool_permissions: bool,
    pub consumes_budget: bool,
    pub mutates_lane_binding: bool,
    pub starts_work: bool,
    pub spawns_agent: bool,
    pub writes_store: bool,
    pub enables_append_only_store: bool,
    pub enforces_scheduler_admission: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub projected_role_kind: &'static str,
    pub application_plan_id: String,
    pub post_application_role_manifest_state: &'static str,
    pub role_manifest_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestBlockerMappingApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_runtime_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub role_binding_scope: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub expected_role_manifest_ready_source_count_after_application: usize,
    pub mutates_runtime: bool,
    pub enforces_role_manifest: bool,
    pub starts_work: bool,
    pub spawns_agent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_role_manifest_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_role_manifest_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub readback_performed: bool,
    pub role_manifest_enforced: bool,
    pub tool_permission_changed: bool,
    pub budget_consumed: bool,
    pub lane_binding_mutated: bool,
    pub work_started: bool,
    pub agent_spawned: bool,
    pub scheduler_admission_enforced: bool,
    pub append_only_store_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report()
-> WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewReport {
    let readback_plans = work_graph_role_manifest_enforcement_gap_closure_readback_plans();
    let application_plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
    let source_outcomes =
        work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes();
    let blocker_applications = work_graph_role_manifest_blocker_mapping_applications();
    let application_groups = work_graph_role_manifest_enforcement_gap_closure_application_groups();
    let application_guards = work_graph_role_manifest_enforcement_gap_closure_application_guards();
    let blockers = work_graph_role_manifest_enforcement_gap_closure_application_blockers();
    let required_prior_gates =
        work_graph_role_manifest_enforcement_gap_closure_application_required_prior_gates();

    WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_role_manifest_gap_closure_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        role_manifest_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.role_manifest_contract_ready_preview)
            .count(),
        blocker_application_count: blocker_applications.len(),
        application_group_count: application_groups.len(),
        role_binding_ref_count: application_plans
            .iter()
            .map(|plan| plan.role_binding_ids.len())
            .sum(),
        capability_ref_count: application_plans
            .iter()
            .map(|plan| plan.capability_ids.len())
            .sum(),
        permission_mode_ref_count: application_plans
            .iter()
            .map(|plan| plan.tool_permission_mode_ids.len())
            .sum(),
        manifest_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.covered_wire_fields.len())
            .sum(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        application_plans,
        source_outcomes,
        blocker_applications,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview: true,
        ready_for_role_manifest_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewSideEffects::none(
        ),
    }
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_plans()
-> Vec<WorkGraphRoleManifestApplicationPlanPreview> {
    work_graph_role_manifest_enforcement_gap_closure_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes()
-> Vec<WorkGraphRoleManifestApplicationSourceOutcomePreview> {
    work_graph_role_manifest_enforcement_gap_closure_application_plans()
        .into_iter()
        .map(source_outcome)
        .collect()
}

pub fn work_graph_role_manifest_blocker_mapping_applications()
-> Vec<WorkGraphRoleManifestBlockerMappingApplicationPreview> {
    work_graph_role_manifest_blocker_mapping_assertions()
        .into_iter()
        .map(blocker_mapping_application)
        .collect()
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_groups()
-> Vec<WorkGraphRoleManifestApplicationGroupPreview> {
    let plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
    vec![
        application_group(
            "role_capability_binding_application",
            "p0",
            "capability",
            &plans,
        ),
        application_group(
            "role_tool_permission_binding_application",
            "p0",
            "tool_permission",
            &plans,
        ),
        application_group(
            "role_budget_lane_binding_application",
            "p0",
            "budget_lane",
            &plans,
        ),
        application_group(
            "role_termination_output_schema_application",
            "p0",
            "termination_output_schema",
            &plans,
        ),
        application_group(
            "role_source_adapter_binding_application",
            "p0",
            "source_adapter",
            &plans,
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_guards()
-> Vec<WorkGraphRoleManifestApplicationGuardPreview> {
    vec![
        application_guard(
            "role_manifest_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard(
            "role_manifest_enforcement_disabled",
            "critical",
            "role_manifest",
        ),
        application_guard("role_capability_binding_not_enforced", "high", "capability"),
        application_guard(
            "tool_permission_binding_not_enforced",
            "critical",
            "tool_permission",
        ),
        application_guard(
            "budget_lane_concurrency_not_enforced",
            "high",
            "budget_lane",
        ),
        application_guard(
            "termination_output_schema_not_enforced",
            "high",
            "termination_output",
        ),
        application_guard(
            "scheduler_admission_runtime_application_disabled",
            "high",
            "scheduler_admission",
        ),
        application_guard(
            "projection_timeline_runtime_residuals_not_promoted",
            "high",
            "projection_timeline",
        ),
        application_guard(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            "append_only_store",
        ),
        application_guard("operator_review_required", "high", "operator_review"),
        application_guard(
            "enforcement_readiness_role_manifest_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_blockers()
-> Vec<WorkGraphRoleManifestApplicationBlockerPreview> {
    let plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
    let all_sources = application_plan_sources(&plans, |_| true);
    let all_plan_ids = application_plan_ids(&plans, |_| true);
    let mut blockers = vec![application_blocker(
        "role_manifest_application_is_preview_only",
        "medium",
        "application_preview",
        all_sources.clone(),
        all_plan_ids.clone(),
        "keep role manifest closure application as a no-mutation preview until readiness rerun proves the blocker moved",
    )];
    for readback_blocker in work_graph_role_manifest_enforcement_gap_closure_readback_blockers() {
        if readback_blocker.id == "role_manifest_closure_application_missing" {
            continue;
        }
        blockers.push(application_blocker_from_readback_blocker(
            readback_blocker,
            &plans,
        ));
    }
    blockers.push(application_blocker(
        "role_manifest_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        all_sources,
        all_plan_ids,
        "rerun unified projection enforcement-readiness against the role manifest application preview outcomes",
    ));
    blockers
}

pub fn work_graph_role_manifest_enforcement_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_role_manifest_enforcement_gap_closure_readback_required_prior_gates();
    gates.push(WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            readback_performed: false,
            role_manifest_enforced: false,
            tool_permission_changed: false,
            budget_consumed: false,
            lane_binding_mutated: false,
            work_started: false,
            agent_spawned: false,
            scheduler_admission_enforced: false,
            append_only_store_enabled: false,
            task_result_enforcement_enabled: false,
            projection_enforcement_enabled: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn application_plan(
    readback_plan: WorkGraphRoleManifestClosureReadbackPlanPreview,
) -> WorkGraphRoleManifestApplicationPlanPreview {
    WorkGraphRoleManifestApplicationPlanPreview {
        application_plan_id: application_plan_id_for_source(readback_plan.source_surface_id),
        readback_plan_id: readback_plan.id,
        closure_plan_id: readback_plan.closure_plan_id,
        source_surface_id: readback_plan.source_surface_id,
        source_category: readback_plan.source_category,
        projected_role_kind: readback_plan.projected_role_kind,
        role_blocker_id: readback_plan.role_blocker_id,
        readback_probe_id: readback_plan.readback_probe_id,
        role_binding_ids: readback_plan.role_binding_ids,
        capability_ids: readback_plan.capability_ids,
        tool_permission_mode_ids: readback_plan.tool_permission_mode_ids,
        covered_wire_fields: readback_plan.covered_wire_fields,
        application_scope: "role_manifest_runtime_enforcement_binding",
        application_state: "preview_application_defined_role_manifest_not_attached",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        enforces_role_manifest: false,
        changes_tool_permissions: false,
        consumes_budget: false,
        mutates_lane_binding: false,
        starts_work: false,
        spawns_agent: false,
        writes_store: false,
        enables_append_only_store: false,
        enforces_scheduler_admission: false,
    }
}

fn source_outcome(
    plan: WorkGraphRoleManifestApplicationPlanPreview,
) -> WorkGraphRoleManifestApplicationSourceOutcomePreview {
    WorkGraphRoleManifestApplicationSourceOutcomePreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        projected_role_kind: plan.projected_role_kind,
        application_plan_id: plan.application_plan_id,
        post_application_role_manifest_state: "role_manifest_contract_ready_preview_after_application",
        role_manifest_contract_ready_preview: true,
        ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview: true,
        ready_for_role_manifest_enforcement: false,
        applies_to_runtime: false,
    }
}

fn blocker_mapping_application(
    assertion: WorkGraphRoleManifestBlockerMappingAssertionPreview,
) -> WorkGraphRoleManifestBlockerMappingApplicationPreview {
    WorkGraphRoleManifestBlockerMappingApplicationPreview {
        application_id: blocker_application_id_for_blocker(assertion.blocker_id),
        blocker_id: assertion.blocker_id,
        category: assertion.category,
        affected_source_surface_ids: assertion.affected_source_surface_ids,
        affected_closure_plan_ids: assertion.affected_closure_plan_ids,
        expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
        blocker_contract_ready_preview: true,
        readback_verified_by_preview: true,
        clears_runtime_blocker: false,
        mutates_runtime: false,
    }
}

fn application_group(
    id: &'static str,
    priority: &'static str,
    role_binding_scope: &'static str,
    plans: &[WorkGraphRoleManifestApplicationPlanPreview],
) -> WorkGraphRoleManifestApplicationGroupPreview {
    WorkGraphRoleManifestApplicationGroupPreview {
        id,
        priority,
        role_binding_scope,
        source_surface_ids: plans.iter().map(|plan| plan.source_surface_id).collect(),
        application_plan_ids: plans
            .iter()
            .map(|plan| plan.application_plan_id.clone())
            .collect(),
        expected_role_manifest_ready_source_count_after_application: plans.len(),
        mutates_runtime: false,
        enforces_role_manifest: false,
        starts_work: false,
        spawns_agent: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphRoleManifestApplicationGuardPreview {
    WorkGraphRoleManifestApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_role_manifest_enforcement: true,
        satisfied_by_preview: false,
    }
}

fn application_blocker_from_readback_blocker(
    blocker: WorkGraphRoleManifestReadbackBlockerPreview,
    plans: &[WorkGraphRoleManifestApplicationPlanPreview],
) -> WorkGraphRoleManifestApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        application_plan_ids(plans, |plan| {
            blocker
                .affected_source_surface_ids
                .contains(&plan.source_surface_id)
        }),
        blocker.recommended_fix,
    )
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphRoleManifestApplicationBlockerPreview {
    WorkGraphRoleManifestApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_role_manifest_enforcement: true,
        recommended_fix,
    }
}

fn application_plan_sources(
    plans: &[WorkGraphRoleManifestApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn application_plan_ids(
    plans: &[WorkGraphRoleManifestApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn application_plan_id_for_source(source_surface_id: &str) -> String {
    format!("apply_{source_surface_id}_role_manifest_gap_closure_preview")
}

fn blocker_application_id_for_blocker(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_role_manifest_blocker_mapping_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_manifest_application_covers_readback_verified_plans() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report();
        let sources = report
            .application_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.readback_plan_count, 4);
        assert_eq!(report.application_plan_count, 4);
        assert_eq!(report.blocker_application_count, 10);
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn role_manifest_application_preserves_role_contracts_and_no_mutation() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report();

        assert_eq!(report.role_binding_ref_count, 24);
        assert_eq!(report.capability_ref_count, 13);
        assert_eq!(report.permission_mode_ref_count, 12);
        assert_eq!(report.manifest_field_ref_count, 27);
        assert!(report.application_plans.iter().all(|plan| {
            plan.application_scope == "role_manifest_runtime_enforcement_binding"
                && plan.application_state
                    == "preview_application_defined_role_manifest_not_attached"
                && plan.role_binding_ids.len() == 6
                && !plan.applies_to_runtime
                && !plan.enforces_role_manifest
                && !plan.changes_tool_permissions
                && !plan.consumes_budget
                && !plan.mutates_lane_binding
                && !plan.starts_work
                && !plan.spawns_agent
                && !plan.writes_store
                && !plan.enables_append_only_store
                && !plan.enforces_scheduler_admission
        }));
    }

    #[test]
    fn role_manifest_application_marks_sources_ready_for_rerun_only() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report();

        assert_eq!(report.source_outcome_count, 4);
        assert_eq!(report.role_manifest_contract_ready_preview_count, 4);
        assert!(report.source_outcomes.iter().all(|outcome| {
            outcome.post_application_role_manifest_state
                == "role_manifest_contract_ready_preview_after_application"
                && outcome.role_manifest_contract_ready_preview
                && outcome
                    .ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview
                && !outcome.ready_for_role_manifest_enforcement
                && !outcome.applies_to_runtime
        }));
        assert!(report.blocker_applications.iter().all(|application| {
            application.expected_blocker_state
                == "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked"
                && application.blocker_contract_ready_preview
                && application.readback_verified_by_preview
                && !application.clears_runtime_blocker
                && !application.mutates_runtime
        }));
    }

    #[test]
    fn role_manifest_application_declares_groups_guards_and_blockers() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report();
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
                ("role_capability_binding_application", 4),
                ("role_tool_permission_binding_application", 4),
                ("role_budget_lane_binding_application", 4),
                ("role_termination_output_schema_application", 4),
                ("role_source_adapter_binding_application", 4),
            ]
        );
        assert_eq!(report.application_group_count, 5);
        assert_eq!(report.application_guard_count, 12);
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_role_manifest_enforcement && !guard.satisfied_by_preview
        }));
        assert_eq!(
            blocker_counts,
            [
                ("role_manifest_application_is_preview_only", 4),
                ("readback_execution_disabled", 4),
                ("role_manifest_enforcement_disabled", 4),
                ("role_capability_binding_not_enforced", 4),
                ("tool_permission_binding_not_enforced", 4),
                ("budget_lane_concurrency_not_enforced", 4),
                ("termination_output_schema_not_enforced", 4),
                ("scheduler_admission_runtime_application_disabled", 3),
                ("projection_timeline_runtime_residuals_not_promoted", 4),
                ("append_only_store_runtime_enablement_disabled", 4),
                ("operator_review_required", 4),
                ("role_manifest_readiness_rerun_missing", 4),
            ]
        );
        assert_eq!(report.blocker_count, 12);
    }

    #[test]
    fn role_manifest_application_advances_only_to_readiness_rerun() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_report();

        assert_eq!(report.required_prior_gate_count, 38);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report.ready_for_unified_projection_enforcement_readiness_role_manifest_rerun_preview
        );
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphRoleManifestEnforcementGapClosureApplicationPreviewSideEffects::none()
        );
    }
}
