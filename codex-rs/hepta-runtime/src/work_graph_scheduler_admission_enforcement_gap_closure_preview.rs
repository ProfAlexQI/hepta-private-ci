use serde::Serialize;

use crate::work_graph_scheduler_admission_controller::WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE;
use crate::work_graph_scheduler_admission_controller::WorkGraphSchedulerAdmissionAdapterPreview;
use crate::work_graph_scheduler_admission_controller::WorkGraphSchedulerAdmissionCheckPreview;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_adapter_previews;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_checks;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_decisions;
use crate::work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview::WorkGraphAppendOnlyStoreRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview::work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions;
use crate::work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview::work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_required_prior_gates;

pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_gate";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_scheduler_admission_enforcement_gap_closure_preview_v1";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub scheduler_blocked_source_count: usize,
    pub controller_check_count: usize,
    pub controller_decision_count: usize,
    pub controller_adapter_count: usize,
    pub closure_plan_count: usize,
    pub admission_binding_count: usize,
    pub readback_probe_binding_count: usize,
    pub evidence_field_ref_count: usize,
    pub closure_group_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub closure_plans: Vec<WorkGraphSchedulerAdmissionClosurePlanPreview>,
    pub closure_groups: Vec<WorkGraphSchedulerAdmissionClosureGroupPreview>,
    pub guards: Vec<WorkGraphSchedulerAdmissionClosureGuardPreview>,
    pub blockers: Vec<WorkGraphSchedulerAdmissionClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_scheduler_admission_readback_preview: bool,
    pub ready_for_scheduler_admission_application_preview: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionClosurePlanPreview {
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub target_node_kind: &'static str,
    pub scheduler_blocker_id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub controller_adapter_blocker_ids: Vec<&'static str>,
    pub admission_check_ids: Vec<&'static str>,
    pub admission_decision_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub readback_probe_id: String,
    pub closure_scope: &'static str,
    pub closure_state: &'static str,
    pub ready_for_readback_preview: bool,
    pub applies_to_runtime: bool,
    pub enforces_scheduler_admission: bool,
    pub starts_work: bool,
    pub acquires_lease: bool,
    pub writes_store: bool,
    pub mutates_idempotency_index: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionClosureGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub check_ids: Vec<&'static str>,
    pub closure_plan_ids: Vec<String>,
    pub source_surface_ids: Vec<&'static str>,
    pub mutates_runtime: bool,
    pub enforces_scheduler_admission: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionClosureGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_scheduler_admission_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub required_before_scheduler_admission_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub scheduler_admission_enforced: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub budget_consumed: bool,
    pub approval_recorded: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report()
-> WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewReport {
    let controller_checks = work_graph_scheduler_admission_checks();
    let controller_decisions = work_graph_scheduler_admission_decisions();
    let controller_adapters = work_graph_scheduler_admission_adapter_previews();
    let scheduler_blocked_sources = scheduler_admission_blocked_source_decisions();
    let closure_plans = work_graph_scheduler_admission_enforcement_gap_closure_plans();
    let closure_groups = work_graph_scheduler_admission_enforcement_gap_closure_groups();
    let guards = work_graph_scheduler_admission_enforcement_gap_closure_guards();
    let blockers = work_graph_scheduler_admission_enforcement_gap_closure_blockers();
    let required_prior_gates =
        work_graph_scheduler_admission_enforcement_gap_closure_required_prior_gates();

    WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_scheduler_admission_gap_closure_no_enforcement",
        scheduler_blocked_source_count: scheduler_blocked_sources.len(),
        controller_check_count: controller_checks.len(),
        controller_decision_count: controller_decisions.len(),
        controller_adapter_count: controller_adapters.len(),
        closure_plan_count: closure_plans.len(),
        admission_binding_count: closure_plans.len(),
        readback_probe_binding_count: closure_plans.len(),
        evidence_field_ref_count: closure_plans
            .iter()
            .map(|plan| plan.required_evidence_fields.len())
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
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_scheduler_admission_readback_preview: true,
        ready_for_scheduler_admission_application_preview: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_plans()
-> Vec<WorkGraphSchedulerAdmissionClosurePlanPreview> {
    let adapters = work_graph_scheduler_admission_adapter_previews();
    let checks = work_graph_scheduler_admission_checks();
    let decisions = work_graph_scheduler_admission_decisions();
    let admission_check_ids = checks.iter().map(|check| check.id).collect::<Vec<_>>();
    let admission_decision_ids = decisions
        .iter()
        .map(|decision| decision.id)
        .collect::<Vec<_>>();
    let required_evidence_fields = required_evidence_fields(&checks);

    scheduler_admission_blocked_source_decisions()
        .into_iter()
        .filter_map(|decision| {
            let adapter = adapters
                .iter()
                .find(|adapter| adapter.source_surface_id == decision.source_surface_id)?;
            closure_plan(
                decision,
                adapter,
                admission_check_ids.clone(),
                admission_decision_ids.clone(),
                required_evidence_fields.clone(),
            )
        })
        .collect()
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_groups()
-> Vec<WorkGraphSchedulerAdmissionClosureGroupPreview> {
    let plans = work_graph_scheduler_admission_enforcement_gap_closure_plans();
    vec![
        closure_group(
            "dependency_and_task_contract_admission_closure",
            "p0",
            vec![
                "dependencies_terminal_ready",
                "task_result_contract_preview_present",
            ],
            &plans,
        ),
        closure_group(
            "lease_budget_idempotency_admission_closure",
            "p0",
            vec![
                "lane_lease_available_and_owned",
                "budget_and_timeout_available",
                "idempotency_replay_window_clear",
            ],
            &plans,
        ),
        closure_group(
            "approval_and_side_effect_lock_admission_closure",
            "p0",
            vec![
                "approval_authority_present_when_required",
                "side_effect_boundary_locked",
            ],
            &plans,
        ),
        closure_group(
            "scheduler_source_adapter_binding_closure",
            "p0",
            work_graph_scheduler_admission_checks()
                .iter()
                .map(|check| check.id)
                .collect(),
            &plans,
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_guards()
-> Vec<WorkGraphSchedulerAdmissionClosureGuardPreview> {
    vec![
        guard(
            "scheduler_admission_closure_is_preview_only",
            "medium",
            "closure_preview",
        ),
        guard(
            "controller_adapter_contract_required",
            "high",
            "controller_adapter",
        ),
        guard(
            "dependency_terminal_evidence_required",
            "high",
            "dependency_evidence",
        ),
        guard("lane_lease_not_acquired", "critical", "lane_lease"),
        guard("approval_not_recorded", "critical", "approval"),
        guard("idempotency_index_not_mutated", "critical", "idempotency"),
        guard("budget_not_consumed", "high", "budget"),
        guard(
            "scheduler_admission_not_enforced",
            "critical",
            "scheduler_admission",
        ),
        guard(
            "append_only_store_runtime_not_enabled",
            "critical",
            "append_only_store",
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_blockers()
-> Vec<WorkGraphSchedulerAdmissionClosureBlockerPreview> {
    let plans = work_graph_scheduler_admission_enforcement_gap_closure_plans();
    let all_sources = closure_plan_sources(&plans, |_| true);
    let all_plan_ids = closure_plan_ids(&plans, |_| true);
    let role_sources = closure_plan_sources(&plans, |plan| {
        source_has_residual(plan.source_surface_id, "role_manifest_not_enforced")
    });
    let role_plan_ids = closure_plan_ids(&plans, |plan| {
        source_has_residual(plan.source_surface_id, "role_manifest_not_enforced")
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
            "scheduler_admission_enforcement_disabled",
            "critical",
            "scheduler_admission",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep admission checks preview-only until readback, application, and operator-review gates are promoted",
        ),
        blocker(
            "lane_lease_acquisition_disabled",
            "critical",
            "lease",
            all_sources.clone(),
            all_plan_ids.clone(),
            "do not acquire or mutate lane leases from the scheduler admission closure preview",
        ),
        blocker(
            "dependency_readback_not_executed",
            "high",
            "dependency_readback",
            all_sources.clone(),
            all_plan_ids.clone(),
            "read back dependency terminal states before scheduler admission can become authoritative",
        ),
        blocker(
            "approval_recording_disabled",
            "critical",
            "approval",
            all_sources.clone(),
            all_plan_ids.clone(),
            "approval evidence must be recorded by a later runtime boundary, not this preview",
        ),
        blocker(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency",
            all_sources.clone(),
            all_plan_ids.clone(),
            "idempotency index mutation remains blocked until append-only store and replay gates are promoted",
        ),
        blocker(
            "budget_consumption_disabled",
            "high",
            "budget",
            all_sources.clone(),
            all_plan_ids.clone(),
            "budget checks are contract-only here and cannot consume resource or retry budget",
        ),
        blocker(
            "role_manifest_residuals_not_enforced",
            "high",
            "role_manifest",
            role_sources,
            role_plan_ids,
            "role-manifest residuals must be handled by the role manifest enforcement frontier before scheduler cutover",
        ),
        blocker(
            "projection_timeline_runtime_residuals_not_promoted",
            "high",
            "projection_timeline",
            projection_timeline_sources,
            projection_timeline_plan_ids,
            "store projection and timeline runtime residuals remain preview-only for scheduler-backed sources",
        ),
        blocker(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            "append_only_store",
            all_sources.clone(),
            all_plan_ids.clone(),
            "scheduler admission cannot become authoritative before append-only store runtime enablement is promoted",
        ),
        blocker(
            "scheduler_admission_closure_readback_missing",
            "high",
            "readback",
            all_sources,
            all_plan_ids,
            "next gate must read back admission bindings, evidence fields, and no-mutation guards before application preview",
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_required_prior_gates();
    push_unique(
        &mut gates,
        WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE,
    );
    push_unique(
        &mut gates,
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphSchedulerAdmissionEnforcementGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            scheduler_admission_enforced: false,
            lease_acquired: false,
            work_started: false,
            budget_consumed: false,
            approval_recorded: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            projection_enforcement_enabled: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn scheduler_admission_blocked_source_decisions()
-> Vec<WorkGraphAppendOnlyStoreRerunSourceDecisionPreview> {
    work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions()
        .into_iter()
        .filter(|decision| {
            decision.append_only_store_rerun_enforcement_decision
                == "deny_scheduler_admission_not_enforced"
        })
        .collect()
}

fn closure_plan(
    decision: WorkGraphAppendOnlyStoreRerunSourceDecisionPreview,
    adapter: &WorkGraphSchedulerAdmissionAdapterPreview,
    admission_check_ids: Vec<&'static str>,
    admission_decision_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> Option<WorkGraphSchedulerAdmissionClosurePlanPreview> {
    let scheduler_blocker_id = decision
        .residual_source_blocker_ids
        .iter()
        .copied()
        .find(|blocker| blocker.ends_with("_admission_not_enforced"))?;

    Some(WorkGraphSchedulerAdmissionClosurePlanPreview {
        closure_plan_id: format!(
            "scheduler_admission_closure_plan:{}",
            decision.source_surface_id
        ),
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        target_node_kind: adapter.target_node_kind,
        scheduler_blocker_id,
        source_fields: adapter.source_fields.clone(),
        controller_adapter_blocker_ids: adapter.blocker_ids.clone(),
        admission_check_ids,
        admission_decision_ids,
        required_evidence_fields,
        readback_probe_id: format!(
            "scheduler_admission_readback_probe:{}",
            decision.source_surface_id
        ),
        closure_scope: "scheduler_admission_contract_preview_only",
        closure_state: "scheduler_admission_contract_ready_preview",
        ready_for_readback_preview: true,
        applies_to_runtime: false,
        enforces_scheduler_admission: false,
        starts_work: false,
        acquires_lease: false,
        writes_store: false,
        mutates_idempotency_index: false,
        records_approval: false,
    })
}

fn required_evidence_fields(
    checks: &[WorkGraphSchedulerAdmissionCheckPreview],
) -> Vec<&'static str> {
    let mut fields = Vec::new();
    for check in checks {
        for field in &check.required_evidence_fields {
            push_unique(&mut fields, field);
        }
    }
    fields
}

fn closure_group(
    id: &'static str,
    priority: &'static str,
    check_ids: Vec<&'static str>,
    plans: &[WorkGraphSchedulerAdmissionClosurePlanPreview],
) -> WorkGraphSchedulerAdmissionClosureGroupPreview {
    WorkGraphSchedulerAdmissionClosureGroupPreview {
        id,
        priority,
        check_ids,
        closure_plan_ids: plans
            .iter()
            .map(|plan| plan.closure_plan_id.clone())
            .collect(),
        source_surface_ids: plans.iter().map(|plan| plan.source_surface_id).collect(),
        mutates_runtime: false,
        enforces_scheduler_admission: false,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphSchedulerAdmissionClosureGuardPreview {
    WorkGraphSchedulerAdmissionClosureGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_scheduler_admission_enforcement: true,
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
) -> WorkGraphSchedulerAdmissionClosureBlockerPreview {
    WorkGraphSchedulerAdmissionClosureBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_closure_plan_ids,
        required_before_scheduler_admission_enforcement: true,
        recommended_fix,
    }
}

fn closure_plan_sources(
    plans: &[WorkGraphSchedulerAdmissionClosurePlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionClosurePlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn closure_plan_ids(
    plans: &[WorkGraphSchedulerAdmissionClosurePlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionClosurePlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.closure_plan_id.clone())
        .collect()
}

fn source_has_residual(source_surface_id: &str, needle: &str) -> bool {
    work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions()
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
    fn scheduler_admission_gap_closure_targets_current_scheduler_blockers() {
        let report = hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report();
        let sources = report
            .closure_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.scheduler_blocked_source_count, 5);
        assert_eq!(report.closure_plan_count, 5);
        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
            ]
        );
        assert!(report.closure_plans.iter().all(|plan| {
            plan.scheduler_blocker_id
                .ends_with("_admission_not_enforced")
        }));
    }

    #[test]
    fn scheduler_admission_gap_closure_binds_controller_checks_and_readbacks() {
        let report = hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report();

        assert_eq!(report.controller_check_count, 7);
        assert_eq!(report.controller_decision_count, 7);
        assert_eq!(report.controller_adapter_count, 5);
        assert_eq!(report.admission_binding_count, 5);
        assert_eq!(report.readback_probe_binding_count, 5);
        assert_eq!(report.evidence_field_ref_count, 90);
        assert!(report.closure_plans.iter().all(|plan| {
            plan.admission_check_ids.len() == 7
                && plan.admission_decision_ids.len() == 7
                && plan.required_evidence_fields.len() == 18
                && plan.ready_for_readback_preview
        }));
    }

    #[test]
    fn scheduler_admission_gap_closure_declares_groups_guards_and_blockers() {
        let report = hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.closure_group_count, 4);
        assert_eq!(report.guard_count, 9);
        assert_eq!(report.blocker_count, 10);
        assert_eq!(
            blocker_counts,
            [
                ("scheduler_admission_enforcement_disabled", 5),
                ("lane_lease_acquisition_disabled", 5),
                ("dependency_readback_not_executed", 5),
                ("approval_recording_disabled", 5),
                ("idempotency_index_mutation_disabled", 5),
                ("budget_consumption_disabled", 5),
                ("role_manifest_residuals_not_enforced", 3),
                ("projection_timeline_runtime_residuals_not_promoted", 4),
                ("append_only_store_runtime_enablement_disabled", 5),
                ("scheduler_admission_closure_readback_missing", 5),
            ]
        );
        assert!(report.guards.iter().all(|guard| {
            guard.required_before_scheduler_admission_enforcement && !guard.satisfied_by_preview
        }));
    }

    #[test]
    fn scheduler_admission_gap_closure_keeps_no_mutation_boundary() {
        let report = hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report();
        let side_effects = report.side_effects;

        assert!(report.ready_for_scheduler_admission_readback_preview);
        assert!(!report.ready_for_scheduler_admission_application_preview);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(report.closure_plans.iter().all(|plan| {
            !plan.applies_to_runtime
                && !plan.enforces_scheduler_admission
                && !plan.starts_work
                && !plan.acquires_lease
                && !plan.writes_store
                && !plan.mutates_idempotency_index
                && !plan.records_approval
        }));
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.scheduler_admission_enforced);
        assert!(!side_effects.lease_acquired);
        assert!(!side_effects.work_started);
        assert!(!side_effects.budget_consumed);
        assert!(!side_effects.approval_recorded);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.append_only_store_enabled);
        assert!(!side_effects.task_result_enforcement_enabled);
        assert!(!side_effects.role_manifest_enforcement_enabled);
        assert!(!side_effects.projection_enforcement_enabled);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.agent_spawn_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
    }

    #[test]
    fn scheduler_admission_gap_closure_declares_next_frontier() {
        let report = hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report();

        assert_eq!(report.required_prior_gate_count, 32);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(
                WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_PREVIEW_GATE,
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
    }
}
