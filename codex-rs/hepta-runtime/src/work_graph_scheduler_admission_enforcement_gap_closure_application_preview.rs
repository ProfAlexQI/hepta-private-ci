use serde::Serialize;

use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::WorkGraphSchedulerAdmissionClosureReadbackPlanPreview;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::WorkGraphSchedulerAdmissionReadbackBlockerPreview;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::work_graph_scheduler_admission_blocker_mapping_readback_assertions;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::work_graph_scheduler_admission_enforcement_gap_closure_readback_blockers;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::work_graph_scheduler_admission_enforcement_gap_closure_readback_plans;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_readback_preview::work_graph_scheduler_admission_enforcement_gap_closure_readback_required_prior_gates;

pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_scheduler_admission_enforcement_gap_closure_application_preview_v1";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub scheduler_admission_contract_ready_preview_count: usize,
    pub blocker_application_count: usize,
    pub application_group_count: usize,
    pub admission_check_ref_count: usize,
    pub admission_decision_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphSchedulerAdmissionApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphSchedulerAdmissionApplicationSourceOutcomePreview>,
    pub blocker_applications: Vec<WorkGraphSchedulerAdmissionBlockerMappingApplicationPreview>,
    pub application_groups: Vec<WorkGraphSchedulerAdmissionApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphSchedulerAdmissionApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphSchedulerAdmissionApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub target_node_kind: &'static str,
    pub scheduler_blocker_id: &'static str,
    pub readback_probe_id: String,
    pub controller_adapter_blocker_ids: Vec<&'static str>,
    pub admission_check_ids: Vec<&'static str>,
    pub admission_decision_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub enforces_scheduler_admission: bool,
    pub starts_work: bool,
    pub acquires_lease: bool,
    pub consumes_budget: bool,
    pub records_approval: bool,
    pub mutates_idempotency_index: bool,
    pub writes_store: bool,
    pub enables_append_only_store: bool,
    pub enforces_role_manifest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub target_node_kind: &'static str,
    pub application_plan_id: String,
    pub post_application_scheduler_admission_state: &'static str,
    pub scheduler_admission_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionBlockerMappingApplicationPreview {
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
pub struct WorkGraphSchedulerAdmissionApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub admission_check_ids: Vec<&'static str>,
    pub source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub expected_scheduler_admission_ready_source_count_after_application: usize,
    pub mutates_runtime: bool,
    pub enforces_scheduler_admission: bool,
    pub starts_work: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_scheduler_admission_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_scheduler_admission_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub readback_performed: bool,
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

pub fn hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report()
-> WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewReport {
    let readback_plans = work_graph_scheduler_admission_enforcement_gap_closure_readback_plans();
    let application_plans =
        work_graph_scheduler_admission_enforcement_gap_closure_application_plans();
    let source_outcomes =
        work_graph_scheduler_admission_enforcement_gap_closure_application_source_outcomes();
    let blocker_applications = work_graph_scheduler_admission_blocker_mapping_applications();
    let application_groups =
        work_graph_scheduler_admission_enforcement_gap_closure_application_groups();
    let application_guards =
        work_graph_scheduler_admission_enforcement_gap_closure_application_guards();
    let blockers = work_graph_scheduler_admission_enforcement_gap_closure_application_blockers();
    let required_prior_gates =
        work_graph_scheduler_admission_enforcement_gap_closure_application_required_prior_gates();

    WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_scheduler_admission_gap_closure_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        scheduler_admission_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.scheduler_admission_contract_ready_preview)
            .count(),
        blocker_application_count: blocker_applications.len(),
        application_group_count: application_groups.len(),
        admission_check_ref_count: application_plans
            .iter()
            .map(|plan| plan.admission_check_ids.len())
            .sum(),
        admission_decision_ref_count: application_plans
            .iter()
            .map(|plan| plan.admission_decision_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_evidence_fields.len())
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
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: true,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_plans()
-> Vec<WorkGraphSchedulerAdmissionApplicationPlanPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_source_outcomes()
-> Vec<WorkGraphSchedulerAdmissionApplicationSourceOutcomePreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_application_plans()
        .into_iter()
        .map(source_outcome)
        .collect()
}

pub fn work_graph_scheduler_admission_blocker_mapping_applications()
-> Vec<WorkGraphSchedulerAdmissionBlockerMappingApplicationPreview> {
    work_graph_scheduler_admission_blocker_mapping_readback_assertions()
        .into_iter()
        .map(blocker_mapping_application)
        .collect()
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_groups()
-> Vec<WorkGraphSchedulerAdmissionApplicationGroupPreview> {
    let plans = work_graph_scheduler_admission_enforcement_gap_closure_application_plans();
    vec![
        application_group(
            "dependency_and_task_contract_admission_application",
            "p0",
            vec![
                "dependencies_terminal_ready",
                "task_result_contract_preview_present",
            ],
            &plans,
        ),
        application_group(
            "lease_budget_idempotency_admission_application",
            "p0",
            vec![
                "lane_lease_available_and_owned",
                "budget_and_timeout_available",
                "idempotency_replay_window_clear",
            ],
            &plans,
        ),
        application_group(
            "approval_and_side_effect_lock_admission_application",
            "p0",
            vec![
                "approval_authority_present_when_required",
                "side_effect_boundary_locked",
            ],
            &plans,
        ),
        application_group(
            "scheduler_source_adapter_binding_application",
            "p0",
            unique_admission_check_ids(&plans),
            &plans,
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_guards()
-> Vec<WorkGraphSchedulerAdmissionApplicationGuardPreview> {
    vec![
        application_guard(
            "scheduler_admission_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard(
            "scheduler_admission_enforcement_disabled",
            "critical",
            "scheduler_admission",
        ),
        application_guard("lane_lease_acquisition_disabled", "critical", "lease"),
        application_guard(
            "dependency_readback_not_executed",
            "high",
            "dependency_readback",
        ),
        application_guard("approval_recording_disabled", "critical", "approval"),
        application_guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency",
        ),
        application_guard("budget_consumption_disabled", "high", "budget"),
        application_guard(
            "role_manifest_residuals_not_enforced",
            "high",
            "role_manifest",
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
            "enforcement_readiness_scheduler_admission_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_blockers()
-> Vec<WorkGraphSchedulerAdmissionApplicationBlockerPreview> {
    let plans = work_graph_scheduler_admission_enforcement_gap_closure_application_plans();
    let all_sources = application_plan_sources(&plans, |_| true);
    let all_plan_ids = application_plan_ids(&plans, |_| true);
    let mut blockers = vec![application_blocker(
        "scheduler_admission_application_is_preview_only",
        "medium",
        "application_preview",
        all_sources.clone(),
        all_plan_ids.clone(),
        "keep scheduler admission closure application as a no-mutation preview until readiness rerun proves the blocker moved",
    )];
    for readback_blocker in
        work_graph_scheduler_admission_enforcement_gap_closure_readback_blockers()
    {
        if readback_blocker.id == "scheduler_admission_closure_application_missing" {
            continue;
        }
        blockers.push(application_blocker_from_readback_blocker(
            readback_blocker,
            &plans,
        ));
    }
    blockers.push(application_blocker(
        "scheduler_admission_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        all_sources,
        all_plan_ids,
        "rerun unified projection enforcement-readiness against the scheduler admission application preview outcomes",
    ));
    blockers
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_scheduler_admission_enforcement_gap_closure_readback_required_prior_gates();
    gates.push(WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            readback_performed: false,
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

fn application_plan(
    readback_plan: WorkGraphSchedulerAdmissionClosureReadbackPlanPreview,
) -> WorkGraphSchedulerAdmissionApplicationPlanPreview {
    WorkGraphSchedulerAdmissionApplicationPlanPreview {
        application_plan_id: application_plan_id_for_source(readback_plan.source_surface_id),
        readback_plan_id: readback_plan.id,
        closure_plan_id: readback_plan.closure_plan_id,
        source_surface_id: readback_plan.source_surface_id,
        source_category: readback_plan.source_category,
        target_node_kind: readback_plan.target_node_kind,
        scheduler_blocker_id: readback_plan.scheduler_blocker_id,
        readback_probe_id: readback_plan.readback_probe_id,
        controller_adapter_blocker_ids: readback_plan.controller_adapter_blocker_ids,
        admission_check_ids: readback_plan.admission_check_ids,
        admission_decision_ids: readback_plan.admission_decision_ids,
        required_evidence_fields: readback_plan.required_evidence_fields,
        application_scope: "scheduler_admission_runtime_enforcement_binding",
        application_state: "preview_application_defined_scheduler_admission_not_attached",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        enforces_scheduler_admission: false,
        starts_work: false,
        acquires_lease: false,
        consumes_budget: false,
        records_approval: false,
        mutates_idempotency_index: false,
        writes_store: false,
        enables_append_only_store: false,
        enforces_role_manifest: false,
    }
}

fn source_outcome(
    plan: WorkGraphSchedulerAdmissionApplicationPlanPreview,
) -> WorkGraphSchedulerAdmissionApplicationSourceOutcomePreview {
    WorkGraphSchedulerAdmissionApplicationSourceOutcomePreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        target_node_kind: plan.target_node_kind,
        application_plan_id: plan.application_plan_id,
        post_application_scheduler_admission_state: "scheduler_admission_contract_ready_preview_after_application",
        scheduler_admission_contract_ready_preview: true,
        ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: true,
        ready_for_scheduler_admission_enforcement: false,
        applies_to_runtime: false,
    }
}

fn blocker_mapping_application(
    assertion: WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview,
) -> WorkGraphSchedulerAdmissionBlockerMappingApplicationPreview {
    WorkGraphSchedulerAdmissionBlockerMappingApplicationPreview {
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
    admission_check_ids: Vec<&'static str>,
    plans: &[WorkGraphSchedulerAdmissionApplicationPlanPreview],
) -> WorkGraphSchedulerAdmissionApplicationGroupPreview {
    WorkGraphSchedulerAdmissionApplicationGroupPreview {
        id,
        priority,
        admission_check_ids,
        source_surface_ids: plans.iter().map(|plan| plan.source_surface_id).collect(),
        application_plan_ids: plans
            .iter()
            .map(|plan| plan.application_plan_id.clone())
            .collect(),
        expected_scheduler_admission_ready_source_count_after_application: plans.len(),
        mutates_runtime: false,
        enforces_scheduler_admission: false,
        starts_work: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphSchedulerAdmissionApplicationGuardPreview {
    WorkGraphSchedulerAdmissionApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_scheduler_admission_enforcement: true,
        satisfied_by_preview: false,
    }
}

fn application_blocker_from_readback_blocker(
    blocker: WorkGraphSchedulerAdmissionReadbackBlockerPreview,
    plans: &[WorkGraphSchedulerAdmissionApplicationPlanPreview],
) -> WorkGraphSchedulerAdmissionApplicationBlockerPreview {
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
) -> WorkGraphSchedulerAdmissionApplicationBlockerPreview {
    WorkGraphSchedulerAdmissionApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_scheduler_admission_enforcement: true,
        recommended_fix,
    }
}

fn application_plan_sources(
    plans: &[WorkGraphSchedulerAdmissionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn application_plan_ids(
    plans: &[WorkGraphSchedulerAdmissionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn unique_admission_check_ids(
    plans: &[WorkGraphSchedulerAdmissionApplicationPlanPreview],
) -> Vec<&'static str> {
    let mut check_ids = Vec::new();
    for plan in plans {
        for check_id in &plan.admission_check_ids {
            if !check_ids.contains(check_id) {
                check_ids.push(*check_id);
            }
        }
    }
    check_ids
}

fn application_plan_id_for_source(source_surface_id: &str) -> String {
    format!("apply_{source_surface_id}_scheduler_admission_gap_closure_preview")
}

fn blocker_application_id_for_blocker(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_scheduler_admission_blocker_mapping_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_admission_application_covers_readback_verified_plans() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report(
            );
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
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
            ]
        );
        assert_eq!(report.readback_plan_count, 5);
        assert_eq!(report.application_plan_count, 5);
        assert_eq!(report.blocker_application_count, 10);
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn scheduler_admission_application_preserves_admission_contracts_and_no_mutation() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report(
            );

        assert_eq!(report.admission_check_ref_count, 35);
        assert_eq!(report.admission_decision_ref_count, 35);
        assert_eq!(report.evidence_field_ref_count, 90);
        assert!(report.application_plans.iter().all(|plan| {
            plan.application_scope == "scheduler_admission_runtime_enforcement_binding"
                && plan.application_state
                    == "preview_application_defined_scheduler_admission_not_attached"
                && plan.admission_check_ids.len() == 7
                && plan.admission_decision_ids.len() == 7
                && plan.required_evidence_fields.len() == 18
                && !plan.applies_to_runtime
                && !plan.enforces_scheduler_admission
                && !plan.starts_work
                && !plan.acquires_lease
                && !plan.consumes_budget
                && !plan.records_approval
                && !plan.mutates_idempotency_index
                && !plan.writes_store
                && !plan.enables_append_only_store
                && !plan.enforces_role_manifest
        }));
    }

    #[test]
    fn scheduler_admission_application_marks_sources_ready_for_rerun_only() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report(
            );

        assert_eq!(report.source_outcome_count, 5);
        assert_eq!(report.scheduler_admission_contract_ready_preview_count, 5);
        assert!(report.source_outcomes.iter().all(|outcome| {
            outcome.post_application_scheduler_admission_state
                == "scheduler_admission_contract_ready_preview_after_application"
                && outcome.scheduler_admission_contract_ready_preview
                && outcome
                    .ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview
                && !outcome.ready_for_scheduler_admission_enforcement
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
    fn scheduler_admission_application_declares_groups_guards_and_blockers() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report(
            );
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
                ("dependency_and_task_contract_admission_application", 5),
                ("lease_budget_idempotency_admission_application", 5),
                ("approval_and_side_effect_lock_admission_application", 5),
                ("scheduler_source_adapter_binding_application", 5),
            ]
        );
        assert_eq!(report.application_group_count, 4);
        assert_eq!(report.application_guard_count, 13);
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_scheduler_admission_enforcement && !guard.satisfied_by_preview
        }));
        assert_eq!(
            blocker_counts,
            [
                ("scheduler_admission_application_is_preview_only", 5),
                ("readback_execution_disabled", 5),
                ("scheduler_admission_enforcement_disabled", 5),
                ("lane_lease_acquisition_disabled", 5),
                ("dependency_readback_not_executed", 5),
                ("approval_recording_disabled", 5),
                ("idempotency_index_mutation_disabled", 5),
                ("budget_consumption_disabled", 5),
                ("role_manifest_residuals_not_enforced", 3),
                ("projection_timeline_runtime_residuals_not_promoted", 4),
                ("append_only_store_runtime_enablement_disabled", 5),
                ("operator_review_required", 5),
                ("scheduler_admission_readiness_rerun_missing", 5),
            ]
        );
        assert_eq!(report.blocker_count, 13);
    }

    #[test]
    fn scheduler_admission_application_advances_only_to_readiness_rerun() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_report(
            );

        assert_eq!(report.required_prior_gate_count, 34);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview
        );
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphSchedulerAdmissionEnforcementGapClosureApplicationPreviewSideEffects::none()
        );
    }
}
