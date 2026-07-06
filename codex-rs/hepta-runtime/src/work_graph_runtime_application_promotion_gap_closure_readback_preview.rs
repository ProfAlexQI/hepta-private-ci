use serde::Serialize;

use crate::work_graph_runtime_application_promotion_gap_closure_preview::WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::WorkGraphRuntimeApplicationPromotionBindingPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::WorkGraphRuntimeApplicationPromotionBlockerPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::WorkGraphRuntimeApplicationPromotionClosurePlanPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::WorkGraphRuntimeApplicationPromotionGroupPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::WorkGraphRuntimeApplicationPromotionGuardPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_bindings;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_blockers;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_groups;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_guards;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_plans;
use crate::work_graph_runtime_application_promotion_gap_closure_preview::work_graph_runtime_application_promotion_gap_closure_required_prior_gates;

pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_runtime_application_promotion_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub closure_plan_count: usize,
    pub promotion_binding_count: usize,
    pub promotion_group_count: usize,
    pub readback_probe_binding_count: usize,
    pub readback_plan_count: usize,
    pub promotion_binding_assertion_count: usize,
    pub promotion_group_assertion_count: usize,
    pub readback_probe_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub promotion_domain_ref_count: usize,
    pub promotion_binding_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub group_source_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview>,
    pub promotion_binding_assertions:
        Vec<WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview>,
    pub promotion_group_assertions:
        Vec<WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview>,
    pub readback_probe_assertions:
        Vec<WorkGraphRuntimeApplicationPromotionProbeReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphRuntimeApplicationPromotionEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphRuntimeApplicationPromotionGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphRuntimeApplicationPromotionReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_application_promotion_closure_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_runtime_application_promotion: bool,
    pub ready_for_operator_review_side_effect_lock: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub source_category: &'static str,
    pub runtime_rerun_decision: &'static str,
    pub promotion_domain_ids: Vec<&'static str>,
    pub promotion_binding_ids: Vec<String>,
    pub readback_probe_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub required_before_closure_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub promotes_runtime_application: bool,
    pub attaches_runtime_wrapper: bool,
    pub enforces_scheduler_admission: bool,
    pub enforces_role_manifest: bool,
    pub mutates_store: bool,
    pub writes_wal: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub binding_id: String,
    pub promotion_domain_id: &'static str,
    pub closes_blocker_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_binding_state: &'static str,
    pub promotes_runtime_application: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview {
    pub id: String,
    pub group_id: &'static str,
    pub promotion_domain_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub closure_plan_ids: Vec<String>,
    pub promotion_binding_ids: Vec<String>,
    pub expected_contract_count_after_closure: usize,
    pub expected_group_state: &'static str,
    pub promotes_runtime_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionProbeReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub readback_probe_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub expected_probe_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
    pub promotes_runtime_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub promotes_runtime_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_runtime_application_promotion: bool,
    pub satisfied_by_readback_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocked_promotion_domain_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub required_before_runtime_application_promotion: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_closure_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocked_promotion_domain_ids: Vec<&'static str>,
    pub required_before_runtime_application_promotion: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub readback_performed: bool,
    pub runtime_application_promoted: bool,
    pub runtime_wrapper_attached: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub rollback_executed: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_report()
-> WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewReport {
    let closure_plans = work_graph_runtime_application_promotion_gap_closure_plans();
    let promotion_bindings = work_graph_runtime_application_promotion_gap_closure_bindings();
    let promotion_groups = work_graph_runtime_application_promotion_gap_closure_groups();
    let guards = work_graph_runtime_application_promotion_gap_closure_guards();
    let closure_blockers = work_graph_runtime_application_promotion_gap_closure_blockers();
    let readback_plans =
        work_graph_runtime_application_promotion_gap_closure_readback_plans_from(&closure_plans);
    let promotion_binding_assertions =
        promotion_binding_readback_assertions_from(&promotion_bindings);
    let promotion_group_assertions = promotion_group_readback_assertions_from(&promotion_groups);
    let readback_probe_assertions = probe_readback_assertions_from(&readback_plans);
    let evidence_field_assertions = evidence_field_readback_assertions_from(&readback_plans);
    let guard_assertions = guard_readback_assertions_from(&guards);
    let blockers = work_graph_runtime_application_promotion_gap_closure_readback_blockers_from(
        &closure_plans,
        closure_blockers,
    );
    let blocker_mapping_assertions =
        blocker_mapping_readback_assertions_from(&blockers, &readback_plans);
    let drift_detectors =
        work_graph_runtime_application_promotion_gap_closure_readback_drift_detectors();
    let required_prior_gates =
        work_graph_runtime_application_promotion_gap_closure_readback_required_prior_gates();

    WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_runtime_application_promotion_gap_closure_readback_no_execution",
        closure_plan_count: closure_plans.len(),
        promotion_binding_count: promotion_bindings.len(),
        promotion_group_count: promotion_groups.len(),
        readback_probe_binding_count: closure_plans.len(),
        readback_plan_count: readback_plans.len(),
        promotion_binding_assertion_count: promotion_binding_assertions.len(),
        promotion_group_assertion_count: promotion_group_assertions.len(),
        readback_probe_assertion_count: readback_probe_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        promotion_domain_ref_count: readback_plans
            .iter()
            .map(|plan| plan.promotion_domain_ids.len())
            .sum(),
        promotion_binding_ref_count: readback_plans
            .iter()
            .map(|plan| plan.promotion_binding_ids.len())
            .sum(),
        evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        group_source_ref_count: promotion_groups
            .iter()
            .map(|group| group.affected_source_surface_ids.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        promotion_binding_assertions,
        promotion_group_assertions,
        readback_probe_assertions,
        evidence_field_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_application_promotion_closure_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_runtime_application_promotion: false,
        ready_for_operator_review_side_effect_lock: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_runtime_application_promotion_gap_closure_readback_plans()
-> Vec<WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview> {
    let closure_plans = work_graph_runtime_application_promotion_gap_closure_plans();
    work_graph_runtime_application_promotion_gap_closure_readback_plans_from(&closure_plans)
}

pub fn work_graph_runtime_application_promotion_binding_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview> {
    let promotion_bindings = work_graph_runtime_application_promotion_gap_closure_bindings();
    promotion_binding_readback_assertions_from(&promotion_bindings)
}

pub fn work_graph_runtime_application_promotion_group_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview> {
    let promotion_groups = work_graph_runtime_application_promotion_gap_closure_groups();
    promotion_group_readback_assertions_from(&promotion_groups)
}

pub fn work_graph_runtime_application_promotion_probe_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionProbeReadbackAssertionPreview> {
    let readback_plans = work_graph_runtime_application_promotion_gap_closure_readback_plans();
    probe_readback_assertions_from(&readback_plans)
}

pub fn work_graph_runtime_application_promotion_evidence_field_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionEvidenceFieldReadbackAssertionPreview> {
    let readback_plans = work_graph_runtime_application_promotion_gap_closure_readback_plans();
    evidence_field_readback_assertions_from(&readback_plans)
}

pub fn work_graph_runtime_application_promotion_guard_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionGuardReadbackAssertionPreview> {
    let guards = work_graph_runtime_application_promotion_gap_closure_guards();
    guard_readback_assertions_from(&guards)
}

pub fn work_graph_runtime_application_promotion_blocker_mapping_readback_assertions()
-> Vec<WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview> {
    let readback_plans = work_graph_runtime_application_promotion_gap_closure_readback_plans();
    let blockers = work_graph_runtime_application_promotion_gap_closure_readback_blockers();
    blocker_mapping_readback_assertions_from(&blockers, &readback_plans)
}

pub fn work_graph_runtime_application_promotion_gap_closure_readback_drift_detectors()
-> Vec<WorkGraphRuntimeApplicationPromotionReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "source_surface_alignment",
            vec!["source_surface_id", "source_category"],
        ),
        drift_detector(
            "promotion_binding_alignment",
            vec!["promotion_binding_ids", "closes_blocker_id"],
        ),
        drift_detector(
            "promotion_group_alignment",
            vec!["promotion_domain_id", "affected_source_surface_ids"],
        ),
        drift_detector(
            "evidence_field_alignment",
            vec!["required_evidence_fields", "required_field_count"],
        ),
        drift_detector(
            "guard_no_mutation_alignment",
            vec!["guard_id", "mutates_runtime"],
        ),
        drift_detector(
            "blocker_mapping_alignment",
            vec!["blocker_id", "affected_readback_plan_ids"],
        ),
        drift_detector(
            "side_effect_boundary_alignment",
            vec!["side_effects", "runtime_mutation_performed"],
        ),
    ]
}

pub fn work_graph_runtime_application_promotion_gap_closure_readback_blockers()
-> Vec<WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview> {
    let closure_plans = work_graph_runtime_application_promotion_gap_closure_plans();
    let closure_blockers = work_graph_runtime_application_promotion_gap_closure_blockers();
    work_graph_runtime_application_promotion_gap_closure_readback_blockers_from(
        &closure_plans,
        closure_blockers,
    )
}

fn work_graph_runtime_application_promotion_gap_closure_readback_blockers_from(
    closure_plans: &[WorkGraphRuntimeApplicationPromotionClosurePlanPreview],
    closure_blockers: Vec<WorkGraphRuntimeApplicationPromotionBlockerPreview>,
) -> Vec<WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview> {
    let readback_plans =
        work_graph_runtime_application_promotion_gap_closure_readback_plans_from(closure_plans);
    let mut blockers = closure_blockers
        .into_iter()
        .map(|blocker| readback_blocker_from_closure_blocker(blocker, &readback_plans))
        .collect::<Vec<_>>();
    blockers.push(WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview {
        id: "runtime_application_promotion_closure_application_missing",
        severity: "high",
        affected_source_surface_ids: closure_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect(),
        affected_readback_plan_ids: readback_plans
            .iter()
            .map(|plan| plan.id.clone())
            .collect(),
        blocked_promotion_domain_ids: vec![
            "projection_adapter_runtime_closure",
            "store_guard_runtime_application",
            "terminal_task_result_runtime_wrapper",
            "scheduler_admission_runtime_application",
            "role_manifest_runtime_application",
        ],
        required_before_runtime_application_promotion: true,
        recommended_fix: "apply readback-verified runtime application promotion plans before readiness rerun",
    });
    blockers
}

pub fn work_graph_runtime_application_promotion_gap_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_runtime_application_promotion_gap_closure_required_prior_gates();
    gates.push(WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_PREVIEW_GATE);
    gates
}

impl WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            readback_performed: false,
            runtime_application_promoted: false,
            runtime_wrapper_attached: false,
            scheduler_admission_enforced: false,
            role_manifest_enforced: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            rollback_executed: false,
            approval_recorded: false,
            operator_review_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_runtime_application_promotion_gap_closure_readback_plans_from(
    closure_plans: &[WorkGraphRuntimeApplicationPromotionClosurePlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview> {
    closure_plans
        .iter()
        .map(
            |plan| WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview {
                id: format!(
                    "runtime_application_promotion_closure_readback_plan__{}",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id.clone(),
                source_category: plan.source_category,
                runtime_rerun_decision: plan.runtime_rerun_decision,
                promotion_domain_ids: plan.promotion_domain_ids.clone(),
                promotion_binding_ids: plan.promotion_binding_ids.clone(),
                readback_probe_id: plan.readback_probe_id.clone(),
                required_evidence_fields: plan.evidence_field_ids.clone(),
                residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
                required_before_closure_application: true,
                readback_state: "asserted_from_closure_preview_no_execution",
                performs_readback: false,
                promotes_runtime_application: false,
                attaches_runtime_wrapper: false,
                enforces_scheduler_admission: false,
                enforces_role_manifest: false,
                mutates_store: false,
                writes_wal: false,
                records_approval: false,
            },
        )
        .collect()
}

fn promotion_binding_readback_assertions_from(
    promotion_bindings: &[WorkGraphRuntimeApplicationPromotionBindingPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview> {
    promotion_bindings
        .iter()
        .map(
            |binding| WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_binding_readback_assertion__{}",
                    binding.id
                ),
                source_surface_id: binding.source_surface_id,
                binding_id: binding.id.clone(),
                promotion_domain_id: binding.promotion_domain_id,
                closes_blocker_id: binding.closes_blocker_id,
                required_evidence_field_ids: binding.required_evidence_field_ids.clone(),
                expected_binding_state: "readback_verified_no_mutation",
                promotes_runtime_application: false,
                writes_store: false,
            },
        )
        .collect()
}

fn promotion_group_readback_assertions_from(
    promotion_groups: &[WorkGraphRuntimeApplicationPromotionGroupPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview> {
    promotion_groups
        .iter()
        .map(
            |group| WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_group_readback_assertion__{}",
                    group.id
                ),
                group_id: group.id,
                promotion_domain_id: group.promotion_domain_id,
                affected_source_surface_ids: group.affected_source_surface_ids.clone(),
                closure_plan_ids: group.closure_plan_ids.clone(),
                promotion_binding_ids: group.promotion_binding_ids.clone(),
                expected_contract_count_after_closure: group.expected_contract_count_after_closure,
                expected_group_state: "readback_verified_no_mutation",
                promotes_runtime_application: false,
            },
        )
        .collect()
}

fn probe_readback_assertions_from(
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionProbeReadbackAssertionPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphRuntimeApplicationPromotionProbeReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_probe_readback_assertion__{}",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id.clone(),
                readback_probe_id: plan.readback_probe_id.clone(),
                required_evidence_fields: plan.required_evidence_fields.clone(),
                expected_probe_state: "readback_contract_declared_not_executed",
                performs_readback: false,
                persists_evidence: false,
                promotes_runtime_application: false,
            },
        )
        .collect()
}

fn evidence_field_readback_assertions_from(
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionEvidenceFieldReadbackAssertionPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphRuntimeApplicationPromotionEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_evidence_field_readback_assertion__{}",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.closure_plan_id.clone(),
                required_field_count: plan.required_evidence_fields.len(),
                required_evidence_fields: plan.required_evidence_fields.clone(),
                expected_evidence_state: "evidence_fields_declared_not_persisted",
                performs_readback: false,
                promotes_runtime_application: false,
            },
        )
        .collect()
}

fn guard_readback_assertions_from(
    guards: &[WorkGraphRuntimeApplicationPromotionGuardPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionGuardReadbackAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphRuntimeApplicationPromotionGuardReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_guard_readback_assertion__{}",
                    guard.id
                ),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.scope,
                expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
                required_before_runtime_application_promotion: true,
                satisfied_by_readback_preview: true,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn blocker_mapping_readback_assertions_from(
    blockers: &[WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview],
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview {
                id: format!(
                    "runtime_application_promotion_blocker_mapping_readback_assertion__{}",
                    blocker.id
                ),
                blocker_id: blocker.id,
                severity: blocker.severity,
                affected_readback_plan_ids: affected_readback_plan_ids_for(
                    &blocker.affected_source_surface_ids,
                    readback_plans,
                ),
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                blocked_promotion_domain_ids: blocker.blocked_promotion_domain_ids.clone(),
                expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
                required_before_runtime_application_promotion: true,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn readback_blocker_from_closure_blocker(
    blocker: WorkGraphRuntimeApplicationPromotionBlockerPreview,
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview {
    WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview {
        id: blocker.id,
        severity: blocker.severity,
        affected_readback_plan_ids: affected_readback_plan_ids_for(
            &blocker.affected_source_surface_ids,
            readback_plans,
        ),
        affected_source_surface_ids: blocker.affected_source_surface_ids,
        blocked_promotion_domain_ids: blocker.blocked_promotion_domain_ids,
        required_before_runtime_application_promotion: true,
        recommended_fix: blocker.recommended_fix,
    }
}

fn affected_readback_plan_ids_for(
    affected_sources: &[&'static str],
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> Vec<String> {
    readback_plans
        .iter()
        .filter(|plan| affected_sources.contains(&plan.source_surface_id))
        .map(|plan| plan.id.clone())
        .collect()
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
) -> WorkGraphRuntimeApplicationPromotionReadbackDriftDetectorPreview {
    WorkGraphRuntimeApplicationPromotionReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity: "high",
        blocks_closure_application: true,
        performs_readback: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_application_promotion_readback_asserts_closure_contracts() {
        let report =
            hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_report();

        assert_eq!(report.closure_plan_count, 12);
        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(report.promotion_binding_count, 27);
        assert_eq!(report.promotion_binding_assertion_count, 27);
        assert_eq!(report.promotion_group_assertion_count, 5);
        assert_eq!(report.readback_probe_assertion_count, 12);
        assert_eq!(report.evidence_field_assertion_count, 12);
        assert_eq!(report.guard_assertion_count, 11);
        assert_eq!(report.blocker_mapping_assertion_count, 14);
        assert_eq!(report.promotion_domain_ref_count, 27);
        assert_eq!(report.promotion_binding_ref_count, 27);
        assert_eq!(report.evidence_field_ref_count, 96);
        assert_eq!(report.group_source_ref_count, 27);
        assert_eq!(report.drift_detector_count, 7);
        assert_eq!(report.blocker_count, 14);
        assert_eq!(report.required_prior_gate_count, 45);
    }

    #[test]
    fn runtime_application_promotion_readback_remains_preview_only() {
        let readback_plans = work_graph_runtime_application_promotion_gap_closure_readback_plans();
        let required_prior_gates =
            work_graph_runtime_application_promotion_gap_closure_readback_required_prior_gates();

        assert_eq!(
            required_prior_gates.last(),
            Some(&WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert!(
            !WorkGraphRuntimeApplicationPromotionGapClosureReadbackPreviewSideEffects::none()
                .runtime_mutation_performed
        );
        assert!(readback_plans.iter().all(|plan| {
            plan.required_before_closure_application
                && !plan.performs_readback
                && !plan.promotes_runtime_application
                && !plan.mutates_store
                && !plan.writes_wal
                && !plan.records_approval
        }));
    }
}
