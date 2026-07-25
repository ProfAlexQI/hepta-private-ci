use serde::Serialize;

use crate::work_graph_role_manifest_contract::WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE;
use crate::work_graph_role_manifest_contract::WorkGraphRoleManifestAdapterPreview;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_adapter_previews;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_capabilities;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_permission_modes;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_required_fields;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_SCHEDULER_ADMISSION_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::WorkGraphSchedulerAdmissionRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions;

pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_role_manifest_enforcement_gap_closure_preview_v1";
pub const WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub role_blocked_source_count: usize,
    pub contract_adapter_count: usize,
    pub manifest_required_field_count: usize,
    pub capability_count: usize,
    pub permission_mode_count: usize,
    pub closure_plan_count: usize,
    pub role_binding_count: usize,
    pub capability_ref_count: usize,
    pub permission_mode_ref_count: usize,
    pub manifest_field_ref_count: usize,
    pub closure_group_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub closure_plans: Vec<WorkGraphRoleManifestClosurePlanPreview>,
    pub closure_groups: Vec<WorkGraphRoleManifestClosureGroupPreview>,
    pub guards: Vec<WorkGraphRoleManifestClosureGuardPreview>,
    pub blockers: Vec<WorkGraphRoleManifestClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_role_manifest_readback_preview: bool,
    pub ready_for_role_manifest_application_preview: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRoleManifestEnforcementGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestClosurePlanPreview {
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub projected_role_kind: &'static str,
    pub role_blocker_id: &'static str,
    pub covered_wire_fields: Vec<&'static str>,
    pub capability_ids: Vec<&'static str>,
    pub tool_permission_mode_ids: Vec<&'static str>,
    pub capability_binding_id: String,
    pub tool_permission_binding_id: String,
    pub budget_binding_id: String,
    pub lane_binding_id: String,
    pub termination_binding_id: String,
    pub output_schema_binding_id: String,
    pub readback_probe_id: String,
    pub closure_scope: &'static str,
    pub closure_state: &'static str,
    pub ready_for_readback_preview: bool,
    pub applies_to_runtime: bool,
    pub enforces_role_manifest: bool,
    pub changes_tool_permissions: bool,
    pub consumes_budget: bool,
    pub starts_work: bool,
    pub spawns_agent: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestClosureGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub binding_type: &'static str,
    pub closure_plan_ids: Vec<String>,
    pub source_surface_ids: Vec<&'static str>,
    pub mutates_runtime: bool,
    pub enforces_role_manifest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestClosureGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_role_manifest_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub required_before_role_manifest_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestEnforcementGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
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

pub fn hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report()
-> WorkGraphRoleManifestEnforcementGapClosurePreviewReport {
    let adapters = work_graph_role_manifest_adapter_previews();
    let required_fields = work_graph_role_manifest_required_fields();
    let capabilities = work_graph_role_manifest_capabilities();
    let permission_modes = work_graph_role_manifest_permission_modes();
    let role_blocked_sources = role_manifest_blocked_source_decisions();
    let closure_plans = work_graph_role_manifest_enforcement_gap_closure_plans();
    let closure_groups = work_graph_role_manifest_enforcement_gap_closure_groups();
    let guards = work_graph_role_manifest_enforcement_gap_closure_guards();
    let blockers = work_graph_role_manifest_enforcement_gap_closure_blockers();
    let required_prior_gates =
        work_graph_role_manifest_enforcement_gap_closure_required_prior_gates();

    WorkGraphRoleManifestEnforcementGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_role_manifest_gap_closure_no_enforcement",
        role_blocked_source_count: role_blocked_sources.len(),
        contract_adapter_count: adapters.len(),
        manifest_required_field_count: required_fields.len(),
        capability_count: capabilities.len(),
        permission_mode_count: permission_modes.len(),
        closure_plan_count: closure_plans.len(),
        role_binding_count: closure_plans.len() * 6,
        capability_ref_count: closure_plans
            .iter()
            .map(|plan| plan.capability_ids.len())
            .sum(),
        permission_mode_ref_count: closure_plans
            .iter()
            .map(|plan| plan.tool_permission_mode_ids.len())
            .sum(),
        manifest_field_ref_count: closure_plans
            .iter()
            .map(|plan| plan.covered_wire_fields.len())
            .sum(),
        closure_group_count: closure_groups.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        closure_plans,
        closure_groups,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_role_manifest_readback_preview: true,
        ready_for_role_manifest_application_preview: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphRoleManifestEnforcementGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_role_manifest_enforcement_gap_closure_plans()
-> Vec<WorkGraphRoleManifestClosurePlanPreview> {
    let adapters = work_graph_role_manifest_adapter_previews();
    role_manifest_blocked_source_decisions()
        .into_iter()
        .filter_map(|decision| {
            let adapter = adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == decision.source_surface_id)?;
            closure_plan(decision, adapter)
        })
        .collect()
}

pub fn work_graph_role_manifest_enforcement_gap_closure_groups()
-> Vec<WorkGraphRoleManifestClosureGroupPreview> {
    let plans = work_graph_role_manifest_enforcement_gap_closure_plans();
    vec![
        closure_group(
            "role_capability_binding_closure",
            "p0",
            "capability",
            &plans,
        ),
        closure_group(
            "tool_permission_binding_closure",
            "p0",
            "tool_permission",
            &plans,
        ),
        closure_group(
            "budget_lane_concurrency_binding_closure",
            "p0",
            "budget_lane_concurrency",
            &plans,
        ),
        closure_group(
            "termination_output_schema_binding_closure",
            "p0",
            "termination_output_schema",
            &plans,
        ),
        closure_group(
            "trace_approval_readback_binding_closure",
            "p0",
            "trace_approval_readback",
            &plans,
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_guards()
-> Vec<WorkGraphRoleManifestClosureGuardPreview> {
    vec![
        guard(
            "role_manifest_closure_is_preview_only",
            "medium",
            "closure_preview",
        ),
        guard("role_contract_adapter_required", "high", "role_contract"),
        guard(
            "capability_permission_binding_required",
            "high",
            "capability_permission",
        ),
        guard("output_schema_verifier_required", "high", "output_schema"),
        guard("budget_concurrency_lane_required", "high", "budget_lane"),
        guard("trace_policy_required", "high", "trace_policy"),
        guard("role_manifest_not_enforced", "critical", "role_manifest"),
        guard(
            "tool_permissions_not_changed",
            "critical",
            "tool_permission",
        ),
        guard(
            "scheduler_admission_runtime_not_applied",
            "high",
            "scheduler_admission",
        ),
        guard(
            "append_only_store_runtime_not_enabled",
            "critical",
            "append_only_store",
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_blockers()
-> Vec<WorkGraphRoleManifestClosureBlockerPreview> {
    let plans = work_graph_role_manifest_enforcement_gap_closure_plans();
    let all_sources = closure_plan_sources(&plans, |_| true);
    let all_plan_ids = closure_plan_ids(&plans, |_| true);
    let scheduler_sources = closure_plan_sources(&plans, |plan| {
        source_has_residual(
            plan.source_surface_id,
            "scheduler_admission_runtime_application_disabled",
        )
    });
    let scheduler_plan_ids = closure_plan_ids(&plans, |plan| {
        source_has_residual(
            plan.source_surface_id,
            "scheduler_admission_runtime_application_disabled",
        )
    });
    let projection_timeline_sources = closure_plan_sources(&plans, |plan| {
        source_has_residual(plan.source_surface_id, "store_projection_not_enforced")
            || source_has_residual(plan.source_surface_id, "timeline_adapter_not_enforced")
    });
    let projection_timeline_plan_ids = closure_plan_ids(&plans, |plan| {
        source_has_residual(plan.source_surface_id, "store_projection_not_enforced")
            || source_has_residual(plan.source_surface_id, "timeline_adapter_not_enforced")
    });

    vec![
        blocker(
            "role_manifest_enforcement_disabled",
            "critical",
            "role_manifest",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep role manifest enforcement preview-only until readback, application, and operator-review gates are promoted",
        ),
        blocker(
            "role_capability_binding_not_enforced",
            "high",
            "capability",
            all_sources.clone(),
            all_plan_ids.clone(),
            "bind each role source to declared capabilities before role admission can be authoritative",
        ),
        blocker(
            "tool_permission_binding_not_enforced",
            "critical",
            "tool_permission",
            all_sources.clone(),
            all_plan_ids.clone(),
            "do not change or enforce tool permission modes from this preview",
        ),
        blocker(
            "budget_lane_concurrency_not_enforced",
            "high",
            "budget_lane",
            all_sources.clone(),
            all_plan_ids.clone(),
            "role budgets, concurrency, and lane bindings remain contract-only until runtime promotion",
        ),
        blocker(
            "termination_output_schema_not_enforced",
            "high",
            "termination_output",
            all_sources.clone(),
            all_plan_ids.clone(),
            "terminal output schema, verifier, and termination contracts need readback before enforcement",
        ),
        blocker(
            "role_manifest_closure_readback_missing",
            "high",
            "readback",
            all_sources.clone(),
            all_plan_ids.clone(),
            "next gate must read back role capability, tool permission, budget, lane, termination, and output-schema bindings",
        ),
        blocker(
            "scheduler_admission_runtime_application_disabled",
            "high",
            "scheduler_admission",
            scheduler_sources,
            scheduler_plan_ids,
            "role manifests must stay separate from scheduler runtime application until lease, approval, and budget gates are promoted",
        ),
        blocker(
            "projection_timeline_runtime_residuals_not_promoted",
            "high",
            "projection_timeline",
            projection_timeline_sources,
            projection_timeline_plan_ids,
            "store projection and timeline runtime residuals remain preview-only for role-manifest sources",
        ),
        blocker(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            "append_only_store",
            all_sources.clone(),
            all_plan_ids.clone(),
            "role enforcement cannot become authoritative before append-only store runtime enablement is promoted",
        ),
        blocker(
            "operator_review_required",
            "high",
            "operator_review",
            all_sources,
            all_plan_ids,
            "operator review must accept role capability, permission, budget, lane, termination, and output-schema bindings before promotion",
        ),
    ]
}

pub fn work_graph_role_manifest_enforcement_gap_closure_required_prior_gates() -> Vec<&'static str>
{
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_required_prior_gates();
    push_unique(&mut gates, WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE);
    push_unique(
        &mut gates,
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_SCHEDULER_ADMISSION_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphRoleManifestEnforcementGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
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

fn role_manifest_blocked_source_decisions()
-> Vec<WorkGraphSchedulerAdmissionRerunSourceDecisionPreview> {
    work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions()
        .into_iter()
        .filter(|decision| {
            decision.scheduler_admission_rerun_enforcement_decision
                == "deny_role_manifest_not_enforced"
        })
        .collect()
}

fn closure_plan(
    decision: WorkGraphSchedulerAdmissionRerunSourceDecisionPreview,
    adapter: &WorkGraphRoleManifestAdapterPreview,
) -> Option<WorkGraphRoleManifestClosurePlanPreview> {
    let role_blocker_id = decision
        .residual_source_blocker_ids
        .iter()
        .copied()
        .find(|blocker| blocker.contains("role_manifest_not_enforced"))?;
    let source_surface_id = decision.source_surface_id;

    Some(WorkGraphRoleManifestClosurePlanPreview {
        closure_plan_id: format!("role_manifest_closure_plan:{source_surface_id}"),
        source_surface_id,
        source_category: decision.source_category,
        projected_role_kind: adapter.projected_role_kind,
        role_blocker_id,
        covered_wire_fields: adapter.covered_wire_fields.clone(),
        capability_ids: capability_ids_for(adapter.projected_role_kind),
        tool_permission_mode_ids: tool_permission_mode_ids_for(adapter.projected_role_kind),
        capability_binding_id: format!("role_capability_binding:{source_surface_id}"),
        tool_permission_binding_id: format!("role_tool_permission_binding:{source_surface_id}"),
        budget_binding_id: format!("role_budget_binding:{source_surface_id}"),
        lane_binding_id: format!("role_lane_binding:{source_surface_id}"),
        termination_binding_id: format!("role_termination_binding:{source_surface_id}"),
        output_schema_binding_id: format!("role_output_schema_binding:{source_surface_id}"),
        readback_probe_id: format!("role_manifest_readback_probe:{source_surface_id}"),
        closure_scope: "role_manifest_contract_preview_only",
        closure_state: "role_manifest_contract_ready_preview",
        ready_for_readback_preview: true,
        applies_to_runtime: false,
        enforces_role_manifest: false,
        changes_tool_permissions: false,
        consumes_budget: false,
        starts_work: false,
        spawns_agent: false,
        writes_store: false,
    })
}

fn capability_ids_for(projected_role_kind: &str) -> Vec<&'static str> {
    match projected_role_kind {
        "agent_task_role" => vec!["planning", "agent_delegation", "research", "verification"],
        "batch_worker_role" => vec!["agent_delegation", "code_editing", "verification"],
        "runtime_worker_role" => vec!["code_editing", "verification", "scheduler_control"],
        "external_handoff_role" => vec!["external_handoff_proposal", "research", "verification"],
        _ => Vec::new(),
    }
}

fn tool_permission_mode_ids_for(projected_role_kind: &str) -> Vec<&'static str> {
    match projected_role_kind {
        "agent_task_role" | "batch_worker_role" | "external_handoff_role" => {
            vec!["preview", "read_only", "approval_required"]
        }
        "runtime_worker_role" => vec!["read_only", "write_scoped", "approval_required"],
        _ => Vec::new(),
    }
}

fn closure_group(
    id: &'static str,
    priority: &'static str,
    binding_type: &'static str,
    plans: &[WorkGraphRoleManifestClosurePlanPreview],
) -> WorkGraphRoleManifestClosureGroupPreview {
    WorkGraphRoleManifestClosureGroupPreview {
        id,
        priority,
        binding_type,
        closure_plan_ids: plans
            .iter()
            .map(|plan| plan.closure_plan_id.clone())
            .collect(),
        source_surface_ids: plans.iter().map(|plan| plan.source_surface_id).collect(),
        mutates_runtime: false,
        enforces_role_manifest: false,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphRoleManifestClosureGuardPreview {
    WorkGraphRoleManifestClosureGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_role_manifest_enforcement: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_closure_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphRoleManifestClosureBlockerPreview {
    WorkGraphRoleManifestClosureBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_closure_plan_ids,
        required_before_role_manifest_enforcement: true,
        recommended_fix,
    }
}

fn closure_plan_sources(
    plans: &[WorkGraphRoleManifestClosurePlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestClosurePlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn closure_plan_ids(
    plans: &[WorkGraphRoleManifestClosurePlanPreview],
    predicate: impl Fn(&WorkGraphRoleManifestClosurePlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.closure_plan_id.clone())
        .collect()
}

fn source_has_residual(source_surface_id: &str, needle: &str) -> bool {
    work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions()
        .iter()
        .find(|decision| decision.source_surface_id == source_surface_id)
        .map(|decision| {
            decision
                .residual_source_blocker_ids
                .iter()
                .any(|blocker| blocker.contains(needle))
        })
        .unwrap_or(false)
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
    fn role_manifest_gap_closure_targets_current_role_blockers() {
        let report = hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report();
        let sources = report
            .closure_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.role_blocked_source_count, 4);
        assert_eq!(report.closure_plan_count, 4);
        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_agent_harness",
            ]
        );
        assert!(
            report
                .closure_plans
                .iter()
                .all(|plan| plan.role_blocker_id.contains("role_manifest_not_enforced"))
        );
    }

    #[test]
    fn role_manifest_gap_closure_binds_role_contract_parts_without_runtime_apply() {
        let report = hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report();

        assert_eq!(report.contract_adapter_count, 4);
        assert_eq!(report.manifest_required_field_count, 12);
        assert_eq!(report.capability_count, 7);
        assert_eq!(report.permission_mode_count, 5);
        assert_eq!(report.role_binding_count, 24);
        assert_eq!(report.capability_ref_count, 13);
        assert_eq!(report.permission_mode_ref_count, 12);
        assert_eq!(report.manifest_field_ref_count, 27);
        assert!(report.closure_plans.iter().all(|plan| {
            plan.ready_for_readback_preview
                && !plan.applies_to_runtime
                && !plan.enforces_role_manifest
                && !plan.changes_tool_permissions
                && !plan.consumes_budget
                && !plan.starts_work
                && !plan.spawns_agent
                && !plan.writes_store
        }));
    }

    #[test]
    fn role_manifest_gap_closure_preserves_blockers_and_guards() {
        let report = hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.closure_group_count, 5);
        assert_eq!(report.guard_count, 10);
        assert_eq!(report.blocker_count, 10);
        assert_eq!(
            blocker_counts,
            [
                ("role_manifest_enforcement_disabled", 4),
                ("role_capability_binding_not_enforced", 4),
                ("tool_permission_binding_not_enforced", 4),
                ("budget_lane_concurrency_not_enforced", 4),
                ("termination_output_schema_not_enforced", 4),
                ("role_manifest_closure_readback_missing", 4),
                ("scheduler_admission_runtime_application_disabled", 3),
                ("projection_timeline_runtime_residuals_not_promoted", 4),
                ("append_only_store_runtime_enablement_disabled", 4),
                ("operator_review_required", 4),
            ]
        );
        assert!(report.guards.iter().all(|guard| {
            guard.required_before_role_manifest_enforcement && !guard.satisfied_by_preview
        }));
    }

    #[test]
    fn role_manifest_gap_closure_requires_readback_before_enforcement() {
        let report = hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report();

        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.required_prior_gate_count, 36);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(
                WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_SCHEDULER_ADMISSION_RERUN_PREVIEW_GATE
            )
        );
        assert!(report.ready_for_role_manifest_readback_preview);
        assert!(!report.ready_for_role_manifest_application_preview);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphRoleManifestEnforcementGapClosurePreviewSideEffects::none()
        );
    }
}
