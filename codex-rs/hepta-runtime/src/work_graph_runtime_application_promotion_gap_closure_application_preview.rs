use serde::Serialize;

use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_report;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_binding_readback_assertions;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_blocker_mapping_readback_assertions;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_gap_closure_readback_blockers;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_gap_closure_readback_plans;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_gap_closure_readback_required_prior_gates;
use crate::work_graph_runtime_application_promotion_gap_closure_readback_preview::work_graph_runtime_application_promotion_group_readback_assertions;

pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_runtime_application_promotion_gap_closure_application_preview_v1";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub runtime_application_contract_ready_preview_count: usize,
    pub promotion_binding_application_count: usize,
    pub promotion_group_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub promotion_domain_ref_count: usize,
    pub promotion_binding_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub group_source_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub application_plans: Vec<WorkGraphRuntimeApplicationPromotionApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphRuntimeApplicationPromotionApplicationSourceOutcomePreview>,
    pub promotion_binding_applications:
        Vec<WorkGraphRuntimeApplicationPromotionBindingApplicationPreview>,
    pub promotion_group_applications:
        Vec<WorkGraphRuntimeApplicationPromotionGroupApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphRuntimeApplicationPromotionBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphRuntimeApplicationPromotionApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview:
        bool,
    pub ready_for_runtime_application_promotion: bool,
    pub ready_for_operator_review_side_effect_lock: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_rerun_decision: &'static str,
    pub promotion_domain_ids: Vec<&'static str>,
    pub promotion_binding_ids: Vec<String>,
    pub readback_probe_id: String,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub promotes_runtime_application: bool,
    pub attaches_runtime_wrapper: bool,
    pub enforces_scheduler_admission: bool,
    pub enforces_role_manifest: bool,
    pub enables_task_result_enforcement: bool,
    pub writes_store: bool,
    pub writes_wal: bool,
    pub records_approval: bool,
    pub executes_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_runtime_promotion_state: &'static str,
    pub runtime_application_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview:
        bool,
    pub ready_for_runtime_application_promotion: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBindingApplicationPreview {
    pub application_id: String,
    pub binding_id: String,
    pub source_surface_id: &'static str,
    pub promotion_domain_id: &'static str,
    pub closes_blocker_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_binding_state: &'static str,
    pub binding_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub promotes_runtime_application: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGroupApplicationPreview {
    pub application_id: String,
    pub group_id: &'static str,
    pub promotion_domain_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub promotion_binding_ids: Vec<String>,
    pub expected_contract_count_after_application: usize,
    pub group_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub promotes_runtime_application: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub blocked_promotion_domain_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_runtime_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_runtime_application_promotion: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub blocked_promotion_domain_ids: Vec<&'static str>,
    pub required_before_runtime_application_promotion: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewSideEffects {
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

pub fn hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_report()
-> WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_report();
    let application_plans = application_plans_from(&readback_report.readback_plans);
    let source_outcomes = source_outcomes_from(&application_plans);
    let promotion_binding_applications =
        binding_applications_from(&readback_report.promotion_binding_assertions);
    let promotion_group_applications = group_applications_from(
        &readback_report.promotion_group_assertions,
        &application_plans,
    );
    let blocker_applications = blocker_applications_from(
        &readback_report.blocker_mapping_assertions,
        &application_plans,
    );
    let application_guards = work_graph_runtime_application_promotion_application_guards();
    let blockers = application_blockers_from(&readback_report.blockers, &application_plans);
    let required_prior_gates =
        work_graph_runtime_application_promotion_gap_closure_application_required_prior_gates();

    WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_runtime_application_promotion_gap_closure_application_no_runtime_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        runtime_application_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.runtime_application_contract_ready_preview)
            .count(),
        promotion_binding_application_count: promotion_binding_applications.len(),
        promotion_group_application_count: promotion_group_applications.len(),
        blocker_application_count: blocker_applications.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        promotion_domain_ref_count: application_plans
            .iter()
            .map(|plan| plan.promotion_domain_ids.len())
            .sum(),
        promotion_binding_ref_count: application_plans
            .iter()
            .map(|plan| plan.promotion_binding_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        group_source_ref_count: promotion_group_applications
            .iter()
            .map(|group| group.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_source_surface_ids.len())
            .sum(),
        application_plans,
        source_outcomes,
        promotion_binding_applications,
        promotion_group_applications,
        blocker_applications,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview: true,
        ready_for_runtime_application_promotion: false,
        ready_for_operator_review_side_effect_lock: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_runtime_application_promotion_gap_closure_application_plans()
-> Vec<WorkGraphRuntimeApplicationPromotionApplicationPlanPreview> {
    let readback_plans = work_graph_runtime_application_promotion_gap_closure_readback_plans();
    application_plans_from(&readback_plans)
}

pub fn work_graph_runtime_application_promotion_gap_closure_application_source_outcomes()
-> Vec<WorkGraphRuntimeApplicationPromotionApplicationSourceOutcomePreview> {
    let application_plans =
        work_graph_runtime_application_promotion_gap_closure_application_plans();
    source_outcomes_from(&application_plans)
}

pub fn work_graph_runtime_application_promotion_binding_applications()
-> Vec<WorkGraphRuntimeApplicationPromotionBindingApplicationPreview> {
    let binding_assertions = work_graph_runtime_application_promotion_binding_readback_assertions();
    binding_applications_from(&binding_assertions)
}

pub fn work_graph_runtime_application_promotion_group_applications()
-> Vec<WorkGraphRuntimeApplicationPromotionGroupApplicationPreview> {
    let group_assertions = work_graph_runtime_application_promotion_group_readback_assertions();
    let application_plans =
        work_graph_runtime_application_promotion_gap_closure_application_plans();
    group_applications_from(&group_assertions, &application_plans)
}

pub fn work_graph_runtime_application_promotion_blocker_applications()
-> Vec<WorkGraphRuntimeApplicationPromotionBlockerApplicationPreview> {
    let blocker_assertions =
        work_graph_runtime_application_promotion_blocker_mapping_readback_assertions();
    let application_plans =
        work_graph_runtime_application_promotion_gap_closure_application_plans();
    blocker_applications_from(&blocker_assertions, &application_plans)
}

pub fn work_graph_runtime_application_promotion_application_guards()
-> Vec<WorkGraphRuntimeApplicationPromotionApplicationGuardPreview> {
    vec![
        application_guard(
            "runtime_application_promotion_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard(
            "runtime_application_promotion_disabled",
            "critical",
            "runtime_application",
        ),
        application_guard(
            "runtime_wrapper_attachment_disabled",
            "high",
            "runtime_wrapper",
        ),
        application_guard("task_result_enforcement_disabled", "high", "task_result"),
        application_guard(
            "scheduler_admission_runtime_enforcement_disabled",
            "high",
            "scheduler",
        ),
        application_guard(
            "role_manifest_runtime_enforcement_disabled",
            "high",
            "role_manifest",
        ),
        application_guard("operator_review_required", "high", "operator_review"),
        application_guard(
            "side_effect_lock_not_established",
            "critical",
            "side_effect_lock",
        ),
        application_guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        application_guard(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        application_guard(
            "append_only_store_enablement_disabled",
            "critical",
            "append_only_store",
        ),
    ]
}

pub fn work_graph_runtime_application_promotion_gap_closure_application_blockers()
-> Vec<WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview> {
    let readback_blockers =
        work_graph_runtime_application_promotion_gap_closure_readback_blockers();
    let application_plans =
        work_graph_runtime_application_promotion_gap_closure_application_plans();
    application_blockers_from(&readback_blockers, &application_plans)
}

pub fn work_graph_runtime_application_promotion_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_runtime_application_promotion_gap_closure_readback_required_prior_gates();
    gates.push(WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewSideEffects {
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

fn application_plans_from(
    readback_plans: &[WorkGraphRuntimeApplicationPromotionClosureReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphRuntimeApplicationPromotionApplicationPlanPreview {
                application_plan_id: application_plan_id_for(&plan.id),
                readback_plan_id: plan.id.clone(),
                closure_plan_id: plan.closure_plan_id.clone(),
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                runtime_rerun_decision: plan.runtime_rerun_decision,
                promotion_domain_ids: plan.promotion_domain_ids.clone(),
                promotion_binding_ids: plan.promotion_binding_ids.clone(),
                readback_probe_id: plan.readback_probe_id.clone(),
                expected_evidence_field_ids: plan.required_evidence_fields.clone(),
                residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
                application_scope: "runtime_application_promotion_gap_closure_application_binding",
                application_state: "preview_application_defined_runtime_application_not_promoted",
                readback_verified_by_preview: true,
                applies_to_runtime: false,
                promotes_runtime_application: false,
                attaches_runtime_wrapper: false,
                enforces_scheduler_admission: false,
                enforces_role_manifest: false,
                enables_task_result_enforcement: false,
                writes_store: false,
                writes_wal: false,
                records_approval: false,
                executes_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn source_outcomes_from(
    application_plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(|plan| WorkGraphRuntimeApplicationPromotionApplicationSourceOutcomePreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            application_plan_id: plan.application_plan_id.clone(),
            post_application_runtime_promotion_state:
                "runtime_application_promotion_contract_ready_preview_after_application",
            runtime_application_contract_ready_preview: true,
            ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview: true,
            ready_for_runtime_application_promotion: false,
            applies_to_runtime: false,
        })
        .collect()
}

fn binding_applications_from(
    assertions: &[WorkGraphRuntimeApplicationPromotionBindingReadbackAssertionPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionBindingApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphRuntimeApplicationPromotionBindingApplicationPreview {
            application_id: binding_application_id_for(&assertion.binding_id),
            binding_id: assertion.binding_id.clone(),
            source_surface_id: assertion.source_surface_id,
            promotion_domain_id: assertion.promotion_domain_id,
            closes_blocker_id: assertion.closes_blocker_id,
            required_evidence_field_ids: assertion.required_evidence_field_ids.clone(),
            expected_binding_state:
                "binding_contract_ready_preview_after_application_runtime_still_blocked",
            binding_contract_ready_preview: true,
            readback_verified_by_preview: true,
            promotes_runtime_application: false,
            writes_store: false,
        })
        .collect()
}

fn group_applications_from(
    assertions: &[WorkGraphRuntimeApplicationPromotionGroupReadbackAssertionPreview],
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionGroupApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphRuntimeApplicationPromotionGroupApplicationPreview {
                application_id: group_application_id_for(assertion.group_id),
                group_id: assertion.group_id,
                promotion_domain_id: assertion.promotion_domain_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                application_plan_ids: application_plan_ids_for_sources(
                    plans,
                    &assertion.affected_source_surface_ids,
                ),
                promotion_binding_ids: assertion.promotion_binding_ids.clone(),
                expected_contract_count_after_application: assertion
                    .expected_contract_count_after_closure,
                group_contract_ready_preview: true,
                readback_verified_by_preview: true,
                promotes_runtime_application: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn blocker_applications_from(
    assertions: &[WorkGraphRuntimeApplicationPromotionBlockerMappingReadbackAssertionPreview],
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionBlockerApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphRuntimeApplicationPromotionBlockerApplicationPreview {
            application_id: blocker_application_id_for(assertion.blocker_id),
            blocker_id: assertion.blocker_id,
            severity: assertion.severity,
            affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
            affected_readback_plan_ids: assertion.affected_readback_plan_ids.clone(),
            affected_application_plan_ids: application_plan_ids_for_readback_plans(
                plans,
                &assertion.affected_readback_plan_ids,
            ),
            blocked_promotion_domain_ids: assertion.blocked_promotion_domain_ids.clone(),
            expected_blocker_state:
                "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
            blocker_contract_ready_preview: true,
            readback_verified_by_preview: true,
            clears_runtime_blocker: false,
            mutates_runtime: false,
        })
        .collect()
}

fn application_blockers_from(
    readback_blockers: &[WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview],
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview> {
    let mut blockers = readback_blockers
        .iter()
        .map(|blocker| application_blocker_from_readback_blocker(blocker, plans))
        .collect::<Vec<_>>();
    blockers.push(application_blocker(
        "runtime_application_promotion_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        affected_sources(plans, |_| true),
        application_plan_ids(plans, |_| true),
        vec![
            "projection_adapter_runtime_closure",
            "store_guard_runtime_application",
            "terminal_task_result_runtime_wrapper",
            "scheduler_admission_runtime_application",
            "role_manifest_runtime_application",
        ],
        "rerun unified projection enforcement-readiness against runtime application promotion application preview outcomes",
    ));
    blockers
}

fn application_blocker_from_readback_blocker(
    blocker: &WorkGraphRuntimeApplicationPromotionReadbackBlockerPreview,
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
) -> WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        "runtime_application_promotion",
        blocker.affected_source_surface_ids.clone(),
        application_plan_ids_for_sources(plans, &blocker.affected_source_surface_ids),
        blocker.blocked_promotion_domain_ids.clone(),
        blocker.recommended_fix,
    )
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    blocked_promotion_domain_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview {
    WorkGraphRuntimeApplicationPromotionApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        blocked_promotion_domain_ids,
        required_before_runtime_application_promotion: true,
        recommended_fix,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphRuntimeApplicationPromotionApplicationGuardPreview {
    WorkGraphRuntimeApplicationPromotionApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_runtime_application_promotion: true,
        satisfied_by_preview: false,
    }
}

fn application_plan_ids_for_sources(
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
    source_ids: &[&'static str],
) -> Vec<String> {
    application_plan_ids(plans, |plan| source_ids.contains(&plan.source_surface_id))
}

fn application_plan_ids_for_readback_plans(
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
    readback_plan_ids: &[String],
) -> Vec<String> {
    application_plan_ids(plans, |plan| {
        readback_plan_ids.contains(&plan.readback_plan_id)
    })
}

fn application_plan_ids(
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphRuntimeApplicationPromotionApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphRuntimeApplicationPromotionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphRuntimeApplicationPromotionApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        if !source_ids.contains(&plan.source_surface_id) {
            source_ids.push(plan.source_surface_id);
        }
    }
    source_ids
}

fn application_plan_id_for(readback_plan_id: &str) -> String {
    format!("apply_{readback_plan_id}_runtime_application_promotion_preview")
}

fn binding_application_id_for(binding_id: &str) -> String {
    format!("apply_{binding_id}_promotion_binding_preview")
}

fn group_application_id_for(group_id: &str) -> String {
    format!("apply_{group_id}_promotion_group_preview")
}

fn blocker_application_id_for(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_runtime_application_blocker_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_application_promotion_application_covers_readback_contracts_without_mutation() {
        let report =
            hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(report.application_plan_count, 12);
        assert_eq!(report.source_outcome_count, 12);
        assert_eq!(report.runtime_application_contract_ready_preview_count, 12);
        assert_eq!(report.promotion_binding_application_count, 27);
        assert_eq!(report.promotion_group_application_count, 5);
        assert_eq!(report.blocker_application_count, 14);
        assert_eq!(report.application_guard_count, 12);
        assert_eq!(report.blocker_count, 15);
        assert_eq!(report.required_prior_gate_count, 46);
        assert_eq!(report.promotion_domain_ref_count, 27);
        assert_eq!(report.promotion_binding_ref_count, 27);
        assert_eq!(report.evidence_field_ref_count, 96);
        assert_eq!(report.group_source_ref_count, 27);
        assert_eq!(report.blocker_mapping_source_ref_count, 125);
        assert_eq!(
            report.side_effects,
            WorkGraphRuntimeApplicationPromotionGapClosureApplicationPreviewSideEffects::none()
        );
    }

    #[test]
    fn runtime_application_promotion_application_preserves_preview_boundaries() {
        let application_plans =
            work_graph_runtime_application_promotion_gap_closure_application_plans();
        let required_prior_gates =
            work_graph_runtime_application_promotion_gap_closure_application_required_prior_gates();

        assert_eq!(
            required_prior_gates.last().copied(),
            Some(WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert!(application_plans.iter().all(|plan| {
            plan.readback_verified_by_preview
                && !plan.applies_to_runtime
                && !plan.promotes_runtime_application
                && !plan.attaches_runtime_wrapper
                && !plan.enforces_scheduler_admission
                && !plan.enforces_role_manifest
                && !plan.enables_task_result_enforcement
                && !plan.writes_store
                && !plan.writes_wal
                && !plan.records_approval
                && !plan.executes_readback
                && !plan.mutates_runtime
        }));
    }
}
