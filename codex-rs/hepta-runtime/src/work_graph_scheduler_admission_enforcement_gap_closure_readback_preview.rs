use serde::Serialize;

use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::WorkGraphSchedulerAdmissionClosureBlockerPreview;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::WorkGraphSchedulerAdmissionClosurePlanPreview;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::work_graph_scheduler_admission_enforcement_gap_closure_blockers;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::work_graph_scheduler_admission_enforcement_gap_closure_guards;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::work_graph_scheduler_admission_enforcement_gap_closure_plans;
use crate::work_graph_scheduler_admission_enforcement_gap_closure_preview::work_graph_scheduler_admission_enforcement_gap_closure_required_prior_gates;

pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub closure_plan_count: usize,
    pub admission_binding_count: usize,
    pub readback_probe_binding_count: usize,
    pub readback_plan_count: usize,
    pub admission_binding_assertion_count: usize,
    pub readback_probe_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub admission_check_ref_count: usize,
    pub admission_decision_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphSchedulerAdmissionClosureReadbackPlanPreview>,
    pub admission_binding_assertions:
        Vec<WorkGraphSchedulerAdmissionBindingReadbackAssertionPreview>,
    pub readback_probe_assertions: Vec<WorkGraphSchedulerAdmissionProbeReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphSchedulerAdmissionEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphSchedulerAdmissionGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphSchedulerAdmissionReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphSchedulerAdmissionReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_scheduler_admission_closure_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionClosureReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub source_category: &'static str,
    pub target_node_kind: &'static str,
    pub scheduler_blocker_id: &'static str,
    pub readback_probe_id: String,
    pub source_fields: Vec<&'static str>,
    pub controller_adapter_blocker_ids: Vec<&'static str>,
    pub admission_check_ids: Vec<&'static str>,
    pub admission_decision_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_before_closure_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub enforces_scheduler_admission: bool,
    pub starts_work: bool,
    pub acquires_lease: bool,
    pub consumes_budget: bool,
    pub records_approval: bool,
    pub mutates_idempotency_index: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub target_node_kind: &'static str,
    pub scheduler_blocker_id: &'static str,
    pub controller_adapter_blocker_ids: Vec<&'static str>,
    pub admission_check_ids: Vec<&'static str>,
    pub admission_decision_ids: Vec<&'static str>,
    pub expected_binding_state: &'static str,
    pub enforces_scheduler_admission: bool,
    pub starts_work: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionProbeReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub readback_probe_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub expected_probe_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
    pub enforces_scheduler_admission: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub enforces_scheduler_admission: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_scheduler_admission_enforcement: bool,
    pub satisfied_by_readback_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub required_before_scheduler_admission_enforcement: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_closure_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub required_before_scheduler_admission_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewSideEffects {
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

pub fn hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report()
-> WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewReport {
    let closure_plans = work_graph_scheduler_admission_enforcement_gap_closure_plans();
    let readback_plans = work_graph_scheduler_admission_enforcement_gap_closure_readback_plans();
    let admission_binding_assertions = work_graph_scheduler_admission_binding_readback_assertions();
    let readback_probe_assertions = work_graph_scheduler_admission_probe_readback_assertions();
    let evidence_field_assertions =
        work_graph_scheduler_admission_evidence_field_readback_assertions();
    let guard_assertions = work_graph_scheduler_admission_guard_readback_assertions();
    let blocker_mapping_assertions =
        work_graph_scheduler_admission_blocker_mapping_readback_assertions();
    let drift_detectors =
        work_graph_scheduler_admission_enforcement_gap_closure_readback_drift_detectors();
    let blockers = work_graph_scheduler_admission_enforcement_gap_closure_readback_blockers();
    let required_prior_gates =
        work_graph_scheduler_admission_enforcement_gap_closure_readback_required_prior_gates();

    WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_scheduler_admission_gap_closure_readback_no_execution",
        closure_plan_count: closure_plans.len(),
        admission_binding_count: closure_plans.len(),
        readback_probe_binding_count: closure_plans.len(),
        readback_plan_count: readback_plans.len(),
        admission_binding_assertion_count: admission_binding_assertions.len(),
        readback_probe_assertion_count: readback_probe_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        admission_check_ref_count: readback_plans
            .iter()
            .map(|plan| plan.admission_check_ids.len())
            .sum(),
        admission_decision_ref_count: readback_plans
            .iter()
            .map(|plan| plan.admission_decision_ids.len())
            .sum(),
        evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        admission_binding_assertions,
        readback_probe_assertions,
        evidence_field_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_scheduler_admission_closure_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_readback_plans()
-> Vec<WorkGraphSchedulerAdmissionClosureReadbackPlanPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| readback_plan(&plan))
        .collect()
}

pub fn work_graph_scheduler_admission_binding_readback_assertions()
-> Vec<WorkGraphSchedulerAdmissionBindingReadbackAssertionPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphSchedulerAdmissionBindingReadbackAssertionPreview {
                id: format!(
                    "assert_{}_scheduler_admission_binding_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id,
                target_node_kind: plan.target_node_kind,
                scheduler_blocker_id: plan.scheduler_blocker_id,
                controller_adapter_blocker_ids: plan.controller_adapter_blocker_ids,
                admission_check_ids: plan.admission_check_ids,
                admission_decision_ids: plan.admission_decision_ids,
                expected_binding_state: "scheduler_admission_binding_defined_enforcement_disabled",
                enforces_scheduler_admission: false,
                starts_work: false,
            },
        )
        .collect()
}

pub fn work_graph_scheduler_admission_probe_readback_assertions()
-> Vec<WorkGraphSchedulerAdmissionProbeReadbackAssertionPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| WorkGraphSchedulerAdmissionProbeReadbackAssertionPreview {
            id: format!(
                "assert_{}_scheduler_admission_probe_readback",
                plan.source_surface_id
            ),
            source_surface_id: plan.source_surface_id,
            closure_plan_id: plan.closure_plan_id,
            readback_probe_id: plan.readback_probe_id,
            required_evidence_fields: plan.required_evidence_fields,
            expected_probe_state:
                "scheduler_admission_readback_probe_defined_execution_disabled",
            performs_readback: false,
            persists_evidence: false,
            enforces_scheduler_admission: false,
        })
        .collect()
}

pub fn work_graph_scheduler_admission_evidence_field_readback_assertions()
-> Vec<WorkGraphSchedulerAdmissionEvidenceFieldReadbackAssertionPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| {
            let required_field_count = plan.required_evidence_fields.len();
            WorkGraphSchedulerAdmissionEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "assert_{}_scheduler_admission_evidence_fields_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id,
                required_evidence_fields: plan.required_evidence_fields,
                required_field_count,
                expected_evidence_state:
                    "scheduler_admission_evidence_contract_defined_no_readback_execution",
                performs_readback: false,
                enforces_scheduler_admission: false,
            }
        })
        .collect()
}

pub fn work_graph_scheduler_admission_guard_readback_assertions()
-> Vec<WorkGraphSchedulerAdmissionGuardReadbackAssertionPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_guards()
        .into_iter()
        .map(
            |guard| WorkGraphSchedulerAdmissionGuardReadbackAssertionPreview {
                id: format!("assert_{}_readback", guard.id),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                expected_guard_state: "guard_declared_satisfied_by_runtime_false",
                required_before_scheduler_admission_enforcement: guard
                    .required_before_scheduler_admission_enforcement,
                satisfied_by_readback_preview: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

pub fn work_graph_scheduler_admission_blocker_mapping_readback_assertions()
-> Vec<WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview> {
    work_graph_scheduler_admission_enforcement_gap_closure_blockers()
        .into_iter()
        .map(|blocker| {
            WorkGraphSchedulerAdmissionBlockerMappingReadbackAssertionPreview {
                id: format!("assert_{}_mapping_readback", blocker.id),
                blocker_id: blocker.id,
                category: blocker.category,
                severity: blocker.severity,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                affected_closure_plan_ids: blocker.affected_closure_plan_ids,
                expected_blocker_state:
                    "blocks_scheduler_admission_until_readback_and_application_preview",
                required_before_scheduler_admission_enforcement:
                    blocker.required_before_scheduler_admission_enforcement,
                performs_readback: false,
                mutates_runtime: false,
            }
        })
        .collect()
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_readback_drift_detectors()
-> Vec<WorkGraphSchedulerAdmissionReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "scheduler_admission_source_coverage_drift",
            vec!["sourceSurfaceId", "closurePlanId", "targetNodeKind"],
            "critical",
        ),
        drift_detector(
            "scheduler_admission_check_binding_drift",
            vec!["admissionCheckIds", "admissionDecisionIds"],
            "critical",
        ),
        drift_detector(
            "scheduler_admission_evidence_field_drift",
            vec!["requiredEvidenceFields", "readbackProbeId"],
            "critical",
        ),
        drift_detector(
            "scheduler_admission_no_mutation_guard_drift",
            vec![
                "performsReadback",
                "acquiresLease",
                "startsWork",
                "recordsApproval",
                "mutatesIdempotencyIndex",
            ],
            "critical",
        ),
        drift_detector(
            "scheduler_admission_blocker_mapping_drift",
            vec![
                "blockerId",
                "affectedSourceSurfaceIds",
                "affectedClosurePlanIds",
            ],
            "high",
        ),
        drift_detector(
            "scheduler_admission_prior_gate_drift",
            vec!["requiredPriorGates", "closurePreviewGate"],
            "medium",
        ),
    ]
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_readback_blockers()
-> Vec<WorkGraphSchedulerAdmissionReadbackBlockerPreview> {
    let plans = work_graph_scheduler_admission_enforcement_gap_closure_readback_plans();
    let closure_blockers = work_graph_scheduler_admission_enforcement_gap_closure_blockers();
    let all_sources = readback_sources(&plans, |_| true);
    let all_readback_ids = readback_plan_ids(&plans, |_| true);

    let mut blockers = vec![readback_blocker(
        "readback_execution_disabled",
        "critical",
        "readback_execution",
        all_sources.clone(),
        all_readback_ids.clone(),
        "this preview defines scheduler admission readback assertions but does not execute readback",
    )];

    for closure_blocker in closure_blockers {
        if closure_blocker.id == "scheduler_admission_closure_readback_missing" {
            continue;
        }
        blockers.push(readback_blocker_from_closure(&closure_blocker, &plans));
    }

    blockers.push(readback_blocker(
        "scheduler_admission_closure_application_missing",
        "high",
        "application_preview",
        all_sources.clone(),
        all_readback_ids.clone(),
        "run closure application preview after readback assertions are defined and reviewed",
    ));
    blockers.push(readback_blocker(
        "operator_review_required",
        "medium",
        "operator_review",
        all_sources,
        all_readback_ids,
        "operator review must accept scheduler admission bindings, evidence fields, guards, and blockers before promotion",
    ));

    blockers
}

pub fn work_graph_scheduler_admission_enforcement_gap_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_scheduler_admission_enforcement_gap_closure_required_prior_gates();
    push_unique(
        &mut gates,
        WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphSchedulerAdmissionEnforcementGapClosureReadbackPreviewSideEffects {
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

fn readback_plan(
    plan: &WorkGraphSchedulerAdmissionClosurePlanPreview,
) -> WorkGraphSchedulerAdmissionClosureReadbackPlanPreview {
    WorkGraphSchedulerAdmissionClosureReadbackPlanPreview {
        id: readback_id_for_source(plan.source_surface_id),
        source_surface_id: plan.source_surface_id,
        closure_plan_id: plan.closure_plan_id.clone(),
        source_category: plan.source_category,
        target_node_kind: plan.target_node_kind,
        scheduler_blocker_id: plan.scheduler_blocker_id,
        readback_probe_id: plan.readback_probe_id.clone(),
        source_fields: plan.source_fields.clone(),
        controller_adapter_blocker_ids: plan.controller_adapter_blocker_ids.clone(),
        admission_check_ids: plan.admission_check_ids.clone(),
        admission_decision_ids: plan.admission_decision_ids.clone(),
        required_evidence_fields: plan.required_evidence_fields.clone(),
        required_before_closure_application: true,
        readback_state: "readback_assertions_defined_execution_disabled",
        performs_readback: false,
        enforces_scheduler_admission: false,
        starts_work: false,
        acquires_lease: false,
        consumes_budget: false,
        records_approval: false,
        mutates_idempotency_index: false,
        writes_store: false,
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphSchedulerAdmissionReadbackDriftDetectorPreview {
    WorkGraphSchedulerAdmissionReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_closure_application: true,
        performs_readback: false,
    }
}

fn readback_blocker_from_closure(
    closure_blocker: &WorkGraphSchedulerAdmissionClosureBlockerPreview,
    plans: &[WorkGraphSchedulerAdmissionClosureReadbackPlanPreview],
) -> WorkGraphSchedulerAdmissionReadbackBlockerPreview {
    readback_blocker(
        closure_blocker.id,
        closure_blocker.severity,
        closure_blocker.category,
        closure_blocker.affected_source_surface_ids.clone(),
        closure_blocker
            .affected_source_surface_ids
            .iter()
            .map(|source| readback_id_for_source(source))
            .filter(|id| plans.iter().any(|plan| plan.id == *id))
            .collect(),
        closure_blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_readback_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphSchedulerAdmissionReadbackBlockerPreview {
    WorkGraphSchedulerAdmissionReadbackBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_readback_plan_ids,
        required_before_scheduler_admission_enforcement: true,
        recommended_fix,
    }
}

fn readback_sources(
    plans: &[WorkGraphSchedulerAdmissionClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionClosureReadbackPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn readback_plan_ids(
    plans: &[WorkGraphSchedulerAdmissionClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphSchedulerAdmissionClosureReadbackPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.id.clone())
        .collect()
}

fn readback_id_for_source(source_surface_id: &str) -> String {
    format!("readback_{source_surface_id}_scheduler_admission_gap_closure")
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
    fn scheduler_admission_gap_closure_readback_targets_current_closure_plans() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report();
        let sources = report
            .readback_plans
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
        assert_eq!(report.closure_plan_count, 5);
        assert_eq!(report.admission_binding_count, 5);
        assert_eq!(report.readback_probe_binding_count, 5);
        assert_eq!(report.readback_plan_count, 5);
        assert_eq!(report.admission_binding_assertion_count, 5);
        assert_eq!(report.readback_probe_assertion_count, 5);
        assert_eq!(report.evidence_field_assertion_count, 5);
    }

    #[test]
    fn scheduler_admission_gap_closure_readback_preserves_admission_contracts() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report();

        assert_eq!(report.admission_check_ref_count, 35);
        assert_eq!(report.admission_decision_ref_count, 35);
        assert_eq!(report.evidence_field_ref_count, 90);
        assert!(report.readback_plans.iter().all(|plan| {
            plan.admission_check_ids.len() == 7
                && plan.admission_decision_ids.len() == 7
                && plan.required_evidence_fields.len() == 18
                && plan.required_before_closure_application
                && !plan.performs_readback
                && !plan.enforces_scheduler_admission
                && !plan.starts_work
                && !plan.acquires_lease
                && !plan.consumes_budget
                && !plan.records_approval
                && !plan.mutates_idempotency_index
                && !plan.writes_store
        }));
        assert!(report.evidence_field_assertions.iter().all(|assertion| {
            assertion.required_field_count == 18
                && !assertion.performs_readback
                && !assertion.enforces_scheduler_admission
        }));
    }

    #[test]
    fn scheduler_admission_gap_closure_readback_declares_guards_blockers_and_drift() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.guard_assertion_count, 9);
        assert_eq!(report.blocker_mapping_assertion_count, 10);
        assert_eq!(report.drift_detector_count, 6);
        assert_eq!(report.blocker_count, 12);
        assert!(
            report
                .guard_assertions
                .iter()
                .all(|guard| !guard.satisfied_by_readback_preview && !guard.mutates_runtime)
        );
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.blocks_closure_application && !detector.performs_readback)
        );
        assert_eq!(
            blocker_counts,
            [
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
                ("scheduler_admission_closure_application_missing", 5),
                ("operator_review_required", 5),
            ]
        );
    }

    #[test]
    fn scheduler_admission_gap_closure_readback_advances_to_application_only() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 33);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_scheduler_admission_closure_application_preview);
        assert!(!report.ready_for_readback_execution);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn scheduler_admission_gap_closure_readback_keeps_all_side_effects_disabled() {
        let report =
            hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_report();
        let side_effects = report.side_effects;

        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.readback_performed);
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
}
