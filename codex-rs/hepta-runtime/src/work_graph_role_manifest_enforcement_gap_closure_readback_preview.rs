use serde::Serialize;

use crate::work_graph_role_manifest_enforcement_gap_closure_preview::WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::WorkGraphRoleManifestClosureBlockerPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::WorkGraphRoleManifestClosurePlanPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::work_graph_role_manifest_enforcement_gap_closure_blockers;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::work_graph_role_manifest_enforcement_gap_closure_guards;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::work_graph_role_manifest_enforcement_gap_closure_plans;
use crate::work_graph_role_manifest_enforcement_gap_closure_preview::work_graph_role_manifest_enforcement_gap_closure_required_prior_gates;

pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_role_manifest_enforcement_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub closure_plan_count: usize,
    pub readback_plan_count: usize,
    pub capability_binding_assertion_count: usize,
    pub tool_permission_assertion_count: usize,
    pub budget_lane_assertion_count: usize,
    pub termination_output_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub role_binding_ref_count: usize,
    pub capability_ref_count: usize,
    pub permission_mode_ref_count: usize,
    pub manifest_field_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphRoleManifestClosureReadbackPlanPreview>,
    pub capability_binding_assertions:
        Vec<WorkGraphRoleManifestCapabilityBindingReadbackAssertionPreview>,
    pub tool_permission_assertions:
        Vec<WorkGraphRoleManifestToolPermissionReadbackAssertionPreview>,
    pub budget_lane_assertions: Vec<WorkGraphRoleManifestBudgetLaneReadbackAssertionPreview>,
    pub termination_output_assertions:
        Vec<WorkGraphRoleManifestTerminationOutputReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphRoleManifestGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions: Vec<WorkGraphRoleManifestBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphRoleManifestReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphRoleManifestReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_role_manifest_closure_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestClosureReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub source_category: &'static str,
    pub projected_role_kind: &'static str,
    pub role_blocker_id: &'static str,
    pub covered_wire_fields: Vec<&'static str>,
    pub capability_ids: Vec<&'static str>,
    pub tool_permission_mode_ids: Vec<&'static str>,
    pub role_binding_ids: Vec<String>,
    pub readback_probe_id: String,
    pub required_before_closure_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub enforces_role_manifest: bool,
    pub changes_tool_permissions: bool,
    pub consumes_budget: bool,
    pub mutates_lane_binding: bool,
    pub starts_work: bool,
    pub spawns_agent: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestCapabilityBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub capability_binding_id: String,
    pub capability_ids: Vec<&'static str>,
    pub expected_binding_state: &'static str,
    pub enforces_role_manifest: bool,
    pub changes_tool_permissions: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestToolPermissionReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub tool_permission_binding_id: String,
    pub tool_permission_mode_ids: Vec<&'static str>,
    pub expected_permission_state: &'static str,
    pub changes_tool_permissions: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestBudgetLaneReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub budget_binding_id: String,
    pub lane_binding_id: String,
    pub expected_budget_lane_state: &'static str,
    pub consumes_budget: bool,
    pub mutates_lane_binding: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestTerminationOutputReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub termination_binding_id: String,
    pub output_schema_binding_id: String,
    pub output_schema_ref_declared: bool,
    pub verifier_ref_declared: bool,
    pub expected_terminal_contract_state: &'static str,
    pub starts_work: bool,
    pub spawns_agent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_role_manifest_enforcement: bool,
    pub satisfied_by_readback_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestBlockerMappingAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub required_before_role_manifest_enforcement: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_closure_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub required_before_role_manifest_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewSideEffects {
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

pub fn hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_report()
-> WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewReport {
    let closure_plans = work_graph_role_manifest_enforcement_gap_closure_plans();
    let readback_plans = work_graph_role_manifest_enforcement_gap_closure_readback_plans();
    let capability_binding_assertions =
        work_graph_role_manifest_capability_binding_readback_assertions();
    let tool_permission_assertions = work_graph_role_manifest_tool_permission_readback_assertions();
    let budget_lane_assertions = work_graph_role_manifest_budget_lane_readback_assertions();
    let termination_output_assertions =
        work_graph_role_manifest_termination_output_readback_assertions();
    let guard_assertions = work_graph_role_manifest_guard_readback_assertions();
    let blocker_mapping_assertions = work_graph_role_manifest_blocker_mapping_assertions();
    let drift_detectors =
        work_graph_role_manifest_enforcement_gap_closure_readback_drift_detectors();
    let blockers = work_graph_role_manifest_enforcement_gap_closure_readback_blockers();
    let required_prior_gates =
        work_graph_role_manifest_enforcement_gap_closure_readback_required_prior_gates();

    WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_role_manifest_gap_closure_readback_no_execution",
        closure_plan_count: closure_plans.len(),
        readback_plan_count: readback_plans.len(),
        capability_binding_assertion_count: capability_binding_assertions.len(),
        tool_permission_assertion_count: tool_permission_assertions.len(),
        budget_lane_assertion_count: budget_lane_assertions.len(),
        termination_output_assertion_count: termination_output_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        role_binding_ref_count: readback_plans
            .iter()
            .map(|plan| plan.role_binding_ids.len())
            .sum(),
        capability_ref_count: readback_plans
            .iter()
            .map(|plan| plan.capability_ids.len())
            .sum(),
        permission_mode_ref_count: readback_plans
            .iter()
            .map(|plan| plan.tool_permission_mode_ids.len())
            .sum(),
        manifest_field_ref_count: readback_plans
            .iter()
            .map(|plan| plan.covered_wire_fields.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        capability_binding_assertions,
        tool_permission_assertions,
        budget_lane_assertions,
        termination_output_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_role_manifest_closure_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_role_manifest_enforcement_gap_closure_readback_plans()
-> Vec<WorkGraphRoleManifestClosureReadbackPlanPreview> {
    work_graph_role_manifest_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| readback_plan(&plan))
        .collect()
}

pub fn work_graph_role_manifest_capability_binding_readback_assertions()
-> Vec<WorkGraphRoleManifestCapabilityBindingReadbackAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphRoleManifestCapabilityBindingReadbackAssertionPreview {
                id: format!(
                    "assert_{}_role_capability_binding_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id,
                capability_binding_id: plan.capability_binding_id,
                capability_ids: plan.capability_ids,
                expected_binding_state: "role_capability_binding_defined_enforcement_disabled",
                enforces_role_manifest: false,
                changes_tool_permissions: false,
            },
        )
        .collect()
}

pub fn work_graph_role_manifest_tool_permission_readback_assertions()
-> Vec<WorkGraphRoleManifestToolPermissionReadbackAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphRoleManifestToolPermissionReadbackAssertionPreview {
                id: format!(
                    "assert_{}_role_tool_permission_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id,
                tool_permission_binding_id: plan.tool_permission_binding_id,
                tool_permission_mode_ids: plan.tool_permission_mode_ids,
                expected_permission_state: "role_tool_permission_defined_no_permission_change",
                changes_tool_permissions: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

pub fn work_graph_role_manifest_budget_lane_readback_assertions()
-> Vec<WorkGraphRoleManifestBudgetLaneReadbackAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| WorkGraphRoleManifestBudgetLaneReadbackAssertionPreview {
            id: format!("assert_{}_role_budget_lane_readback", plan.source_surface_id),
            source_surface_id: plan.source_surface_id,
            closure_plan_id: plan.closure_plan_id,
            budget_binding_id: plan.budget_binding_id,
            lane_binding_id: plan.lane_binding_id,
            expected_budget_lane_state: "role_budget_lane_binding_defined_no_budget_or_lane_mutation",
            consumes_budget: false,
            mutates_lane_binding: false,
        })
        .collect()
}

pub fn work_graph_role_manifest_termination_output_readback_assertions()
-> Vec<WorkGraphRoleManifestTerminationOutputReadbackAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| {
            let output_schema_ref_declared =
                plan.covered_wire_fields.contains(&"outputSchemaRef");
            let verifier_ref_declared = plan.covered_wire_fields.contains(&"verifierRef");
            WorkGraphRoleManifestTerminationOutputReadbackAssertionPreview {
                id: format!(
                    "assert_{}_role_termination_output_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id,
                termination_binding_id: plan.termination_binding_id,
                output_schema_binding_id: plan.output_schema_binding_id,
                output_schema_ref_declared,
                verifier_ref_declared,
                expected_terminal_contract_state:
                    "role_terminal_contract_defined_no_work_or_agent_spawn",
                starts_work: false,
                spawns_agent: false,
            }
        })
        .collect()
}

pub fn work_graph_role_manifest_guard_readback_assertions()
-> Vec<WorkGraphRoleManifestGuardReadbackAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_guards()
        .into_iter()
        .map(|guard| WorkGraphRoleManifestGuardReadbackAssertionPreview {
            id: format!("assert_{}_readback", guard.id),
            guard_id: guard.id,
            severity: guard.severity,
            guard_scope: guard.guard_scope,
            expected_guard_state: "guard_declared_satisfied_by_runtime_false",
            required_before_role_manifest_enforcement: guard
                .required_before_role_manifest_enforcement,
            satisfied_by_readback_preview: false,
            mutates_runtime: false,
        })
        .collect()
}

pub fn work_graph_role_manifest_blocker_mapping_assertions()
-> Vec<WorkGraphRoleManifestBlockerMappingAssertionPreview> {
    work_graph_role_manifest_enforcement_gap_closure_blockers()
        .into_iter()
        .map(|blocker| WorkGraphRoleManifestBlockerMappingAssertionPreview {
            id: format!("assert_{}_mapping_readback", blocker.id),
            blocker_id: blocker.id,
            category: blocker.category,
            severity: blocker.severity,
            affected_source_surface_ids: blocker.affected_source_surface_ids,
            affected_closure_plan_ids: blocker.affected_closure_plan_ids,
            expected_blocker_state: "blocks_role_manifest_until_readback_and_application_preview",
            required_before_role_manifest_enforcement: blocker
                .required_before_role_manifest_enforcement,
            performs_readback: false,
            mutates_runtime: false,
        })
        .collect()
}

pub fn work_graph_role_manifest_enforcement_gap_closure_readback_drift_detectors()
-> Vec<WorkGraphRoleManifestReadbackDriftDetectorPreview> {
    vec![
        drift(
            "role_manifest_source_coverage_drift",
            vec!["sourceSurfaceId", "closurePlanId", "projectedRoleKind"],
            "critical",
        ),
        drift(
            "role_manifest_capability_binding_drift",
            vec!["capabilityIds", "capabilityBindingId"],
            "critical",
        ),
        drift(
            "role_manifest_tool_permission_binding_drift",
            vec!["toolPermissionModeIds", "toolPermissionBindingId"],
            "critical",
        ),
        drift(
            "role_manifest_budget_lane_binding_drift",
            vec!["budgetBindingId", "laneBindingId"],
            "high",
        ),
        drift(
            "role_manifest_termination_output_binding_drift",
            vec![
                "terminationBindingId",
                "outputSchemaBindingId",
                "verifierRef",
            ],
            "high",
        ),
        drift(
            "role_manifest_no_mutation_blocker_mapping_drift",
            vec![
                "performsReadback",
                "changesToolPermissions",
                "consumesBudget",
                "startsWork",
                "spawnsAgent",
                "blockerId",
            ],
            "critical",
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_readback_blockers()
-> Vec<WorkGraphRoleManifestReadbackBlockerPreview> {
    let plans = work_graph_role_manifest_enforcement_gap_closure_readback_plans();
    let all_sources = readback_sources(&plans, |_| true);
    let all_readback_ids = readback_ids(&plans, |_| true);
    let mut blockers = vec![readback_blocker(
        "readback_execution_disabled",
        "critical",
        "readback_execution",
        all_sources.clone(),
        all_readback_ids.clone(),
        "this preview defines role manifest readback assertions but does not execute readback",
    )];
    for blocker in work_graph_role_manifest_enforcement_gap_closure_blockers()
        .into_iter()
        .filter(|blocker| blocker.id != "role_manifest_closure_readback_missing")
    {
        blockers.push(readback_blocker_from_closure(blocker));
    }
    blockers.push(readback_blocker(
        "role_manifest_closure_application_missing",
        "high",
        "application_preview",
        all_sources,
        all_readback_ids,
        "run closure application preview after role manifest readback assertions are defined and reviewed",
    ));
    blockers
}

pub fn work_graph_role_manifest_enforcement_gap_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_role_manifest_enforcement_gap_closure_required_prior_gates();
    push_unique(
        &mut gates,
        WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewSideEffects {
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

fn readback_plan(
    plan: &WorkGraphRoleManifestClosurePlanPreview,
) -> WorkGraphRoleManifestClosureReadbackPlanPreview {
    WorkGraphRoleManifestClosureReadbackPlanPreview {
        id: format!(
            "readback_{}_role_manifest_gap_closure",
            plan.source_surface_id
        ),
        source_surface_id: plan.source_surface_id,
        closure_plan_id: plan.closure_plan_id.clone(),
        source_category: plan.source_category,
        projected_role_kind: plan.projected_role_kind,
        role_blocker_id: plan.role_blocker_id,
        covered_wire_fields: plan.covered_wire_fields.clone(),
        capability_ids: plan.capability_ids.clone(),
        tool_permission_mode_ids: plan.tool_permission_mode_ids.clone(),
        role_binding_ids: role_binding_ids(plan),
        readback_probe_id: plan.readback_probe_id.clone(),
        required_before_closure_application: true,
        readback_state: "readback_assertions_defined_execution_disabled",
        performs_readback: false,
        enforces_role_manifest: false,
        changes_tool_permissions: false,
        consumes_budget: false,
        mutates_lane_binding: false,
        starts_work: false,
        spawns_agent: false,
        writes_store: false,
    }
}

fn role_binding_ids(plan: &WorkGraphRoleManifestClosurePlanPreview) -> Vec<String> {
    vec![
        plan.capability_binding_id.clone(),
        plan.tool_permission_binding_id.clone(),
        plan.budget_binding_id.clone(),
        plan.lane_binding_id.clone(),
        plan.termination_binding_id.clone(),
        plan.output_schema_binding_id.clone(),
    ]
}

fn drift(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphRoleManifestReadbackDriftDetectorPreview {
    WorkGraphRoleManifestReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_closure_application: true,
        performs_readback: false,
    }
}

fn readback_blocker_from_closure(
    blocker: WorkGraphRoleManifestClosureBlockerPreview,
) -> WorkGraphRoleManifestReadbackBlockerPreview {
    readback_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        blocker
            .affected_source_surface_ids
            .iter()
            .map(|source| format!("readback_{source}_role_manifest_gap_closure"))
            .collect(),
        blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_readback_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphRoleManifestReadbackBlockerPreview {
    WorkGraphRoleManifestReadbackBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_readback_plan_ids,
        required_before_role_manifest_enforcement: true,
        recommended_fix,
    }
}

fn readback_sources(
    plans: &[WorkGraphRoleManifestClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestClosureReadbackPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn readback_ids(
    plans: &[WorkGraphRoleManifestClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestClosureReadbackPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.id.clone())
        .collect()
}

fn push_unique(values: &mut Vec<&'static str>, value: &'static str) {
    if !values.contains(&value) {
        values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_manifest_readback_targets_current_closure_plans() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_report();
        let sources = report
            .readback_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.closure_plan_count, 4);
        assert_eq!(report.readback_plan_count, 4);
        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.role_binding_ref_count, 24);
        assert_eq!(report.capability_ref_count, 13);
        assert_eq!(report.permission_mode_ref_count, 12);
        assert_eq!(report.manifest_field_ref_count, 27);
    }

    #[test]
    fn role_manifest_readback_assertions_remain_no_execution() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_report();

        assert_eq!(report.capability_binding_assertion_count, 4);
        assert_eq!(report.tool_permission_assertion_count, 4);
        assert_eq!(report.budget_lane_assertion_count, 4);
        assert_eq!(report.termination_output_assertion_count, 4);
        assert!(report.readback_plans.iter().all(|plan| {
            plan.required_before_closure_application
                && !plan.performs_readback
                && !plan.enforces_role_manifest
                && !plan.changes_tool_permissions
                && !plan.consumes_budget
                && !plan.mutates_lane_binding
                && !plan.starts_work
                && !plan.spawns_agent
                && !plan.writes_store
        }));
    }

    #[test]
    fn role_manifest_readback_preserves_blockers_and_drift_detectors() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.guard_assertion_count, 10);
        assert_eq!(report.blocker_mapping_assertion_count, 10);
        assert_eq!(report.drift_detector_count, 6);
        assert_eq!(report.blocker_count, 11);
        assert_eq!(
            blocker_counts,
            [
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
                ("role_manifest_closure_application_missing", 4),
            ]
        );
    }

    #[test]
    fn role_manifest_readback_requires_application_before_enforcement() {
        let report =
            hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 37);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_role_manifest_closure_application_preview);
        assert!(!report.ready_for_readback_execution);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphRoleManifestEnforcementGapClosureReadbackPreviewSideEffects::none()
        );
    }
}
