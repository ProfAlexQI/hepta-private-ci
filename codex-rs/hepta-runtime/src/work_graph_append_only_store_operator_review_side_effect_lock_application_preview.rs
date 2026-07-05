use serde::Serialize;

use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewBlockerMappingAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewBoundaryReadbackAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewGroupReadbackAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewPacketReadbackAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewReadbackBlockerPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::WorkGraphSideEffectLockReadbackAssertionPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_report;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::work_graph_append_only_store_operator_review_side_effect_lock_readback_plans;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_readback_preview::work_graph_append_only_store_operator_review_side_effect_lock_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_SCHEMA_VERSION:
    &str = "work_graph_append_only_store_operator_review_side_effect_lock_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub operator_review_contract_ready_preview_count: usize,
    pub side_effect_lock_contract_ready_preview_count: usize,
    pub operator_review_packet_application_count: usize,
    pub side_effect_lock_application_count: usize,
    pub approval_boundary_application_count: usize,
    pub readback_boundary_application_count: usize,
    pub group_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub evidence_field_ref_count: usize,
    pub lock_scope_ref_count: usize,
    pub group_source_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub application_plans: Vec<WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview>,
    pub packet_applications: Vec<WorkGraphOperatorReviewPacketApplicationPreview>,
    pub side_effect_lock_applications: Vec<WorkGraphSideEffectLockApplicationPreview>,
    pub approval_boundary_applications: Vec<WorkGraphApprovalEvidenceBoundaryApplicationPreview>,
    pub readback_boundary_applications: Vec<WorkGraphOperatorReviewBoundaryApplicationPreview>,
    pub group_applications: Vec<WorkGraphOperatorReviewGroupApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphOperatorReviewBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphOperatorReviewSideEffectLockApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview:
        bool,
    pub ready_for_operator_review_recording: bool,
    pub ready_for_side_effect_lock_establishment: bool,
    pub ready_for_runtime_write_boundary_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub operator_review_packet_id: String,
    pub side_effect_lock_plan_id: String,
    pub approval_evidence_boundary_id: String,
    pub readback_boundary_id: String,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub lock_scope_ids: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub operator_review_contract_ready_preview: bool,
    pub side_effect_lock_contract_ready_preview: bool,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub establishes_side_effect_lock: bool,
    pub executes_readback: bool,
    pub writes_store: bool,
    pub writes_wal: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_operator_review_state: &'static str,
    pub operator_review_contract_ready_preview: bool,
    pub side_effect_lock_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview:
        bool,
    pub ready_for_operator_review_recording: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewPacketApplicationPreview {
    pub application_id: String,
    pub packet_id: String,
    pub source_surface_id: &'static str,
    pub required_section_ids: Vec<&'static str>,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_packet_state: &'static str,
    pub packet_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub records_operator_review: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSideEffectLockApplicationPreview {
    pub application_id: String,
    pub lock_plan_id: String,
    pub source_surface_id: &'static str,
    pub lock_scope_ids: Vec<&'static str>,
    pub expected_lock_state: &'static str,
    pub lock_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub establishes_side_effect_lock: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphApprovalEvidenceBoundaryApplicationPreview {
    pub application_id: String,
    pub boundary_id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_boundary_state: &'static str,
    pub boundary_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub persists_receipt: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewBoundaryApplicationPreview {
    pub application_id: String,
    pub boundary_id: String,
    pub readback_probe_id: String,
    pub source_surface_id: &'static str,
    pub expected_boundary_state: &'static str,
    pub boundary_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub executes_readback: bool,
    pub rollback_executed: bool,
    pub writes_checkpoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewGroupApplicationPreview {
    pub application_id: String,
    pub group_id: &'static str,
    pub source_category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub operator_review_packet_ids: Vec<String>,
    pub side_effect_lock_plan_ids: Vec<String>,
    pub expected_contract_count_after_application: usize,
    pub group_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub records_operator_review: bool,
    pub establishes_side_effect_lock: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_operator_review_blocker: bool,
    pub clears_side_effect_lock_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_operator_review_side_effect_lock: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_operator_review_side_effect_lock: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_report()
-> WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_report(
        );
    let application_plans = application_plans_from(&readback_report.readback_plans);
    let source_outcomes = source_outcomes_from(&application_plans);
    let packet_applications = packet_applications_from(&readback_report.packet_assertions);
    let side_effect_lock_applications =
        side_effect_lock_applications_from(&readback_report.side_effect_lock_assertions);
    let approval_boundary_applications =
        approval_boundary_applications_from(&readback_report.approval_boundary_assertions);
    let readback_boundary_applications =
        readback_boundary_applications_from(&readback_report.readback_boundary_assertions);
    let group_applications =
        group_applications_from(&readback_report.group_assertions, &application_plans);
    let blocker_applications = blocker_applications_from(
        &readback_report.blocker_mapping_assertions,
        &application_plans,
    );
    let application_guards =
        work_graph_append_only_store_operator_review_side_effect_lock_application_guards();
    let blockers = application_blockers_from(&readback_report.blockers, &application_plans);
    let required_prior_gates =
        work_graph_append_only_store_operator_review_side_effect_lock_application_required_prior_gates();

    WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_operator_review_side_effect_lock_application_no_runtime_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        operator_review_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.operator_review_contract_ready_preview)
            .count(),
        side_effect_lock_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.side_effect_lock_contract_ready_preview)
            .count(),
        operator_review_packet_application_count: packet_applications.len(),
        side_effect_lock_application_count: side_effect_lock_applications.len(),
        approval_boundary_application_count: approval_boundary_applications.len(),
        readback_boundary_application_count: readback_boundary_applications.len(),
        group_application_count: group_applications.len(),
        blocker_application_count: blocker_applications.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        lock_scope_ref_count: application_plans
            .iter()
            .map(|plan| plan.lock_scope_ids.len())
            .sum(),
        group_source_ref_count: group_applications
            .iter()
            .map(|group| group.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_source_surface_ids.len())
            .sum(),
        application_plans,
        source_outcomes,
        packet_applications,
        side_effect_lock_applications,
        approval_boundary_applications,
        readback_boundary_applications,
        group_applications,
        blocker_applications,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview: true,
        ready_for_operator_review_recording: false,
        ready_for_side_effect_lock_establishment: false,
        ready_for_runtime_write_boundary_preview: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_application_plans()
-> Vec<WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview> {
    let readback_plans =
        work_graph_append_only_store_operator_review_side_effect_lock_readback_plans();
    application_plans_from(&readback_plans)
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_application_source_outcomes()
-> Vec<WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview> {
    let application_plans =
        work_graph_append_only_store_operator_review_side_effect_lock_application_plans();
    source_outcomes_from(&application_plans)
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_application_guards()
-> Vec<WorkGraphOperatorReviewSideEffectLockApplicationGuardPreview> {
    vec![
        application_guard(
            "operator_review_side_effect_lock_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard(
            "operator_review_recording_disabled",
            "high",
            "operator_review",
        ),
        application_guard("approval_recording_disabled", "high", "approval"),
        application_guard(
            "side_effect_lock_establishment_disabled",
            "critical",
            "side_effect_lock",
        ),
        application_guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        application_guard(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        application_guard("idempotency_mutation_disabled", "critical", "idempotency"),
        application_guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback_readback",
        ),
        application_guard(
            "append_only_store_enablement_disabled",
            "critical",
            "append_only_store",
        ),
        application_guard("runtime_mutation_disabled", "critical", "runtime_mutation"),
        application_guard("model_invocation_disabled", "high", "model_boundary"),
    ]
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_application_blockers()
-> Vec<WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview> {
    let readback_report =
        hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_report(
        );
    let application_plans = application_plans_from(&readback_report.readback_plans);
    application_blockers_from(&readback_report.blockers, &application_plans)
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_operator_review_side_effect_lock_readback_required_prior_gates(
        );
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforced: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn application_plans_from(
    readback_plans: &[WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview {
            application_plan_id: application_plan_id_for(&plan.id),
            readback_plan_id: plan.id.clone(),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            operator_review_packet_id: plan.operator_review_packet_id.clone(),
            side_effect_lock_plan_id: plan.side_effect_lock_plan_id.clone(),
            approval_evidence_boundary_id: plan.approval_evidence_boundary_id.clone(),
            readback_boundary_id: plan.readback_boundary_id.clone(),
            expected_evidence_field_ids: plan.required_evidence_fields.clone(),
            lock_scope_ids: plan.lock_scope_ids.clone(),
            application_scope: "operator_review_side_effect_lock_application_binding",
            application_state:
                "preview_application_defined_operator_review_and_lock_not_recorded",
            readback_verified_by_preview: true,
            operator_review_contract_ready_preview: true,
            side_effect_lock_contract_ready_preview: true,
            records_operator_review: false,
            records_approval: false,
            establishes_side_effect_lock: false,
            executes_readback: false,
            writes_store: false,
            writes_wal: false,
            mutates_runtime: false,
        })
        .collect()
}

fn source_outcomes_from(
    application_plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(
            |plan| WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview {
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_operator_review_state:
                    "operator_review_side_effect_lock_contract_ready_preview_after_application",
                operator_review_contract_ready_preview: true,
                side_effect_lock_contract_ready_preview: true,
                ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview: true,
                ready_for_operator_review_recording: false,
                applies_to_runtime: false,
            },
        )
        .collect()
}

fn packet_applications_from(
    assertions: &[WorkGraphOperatorReviewPacketReadbackAssertionPreview],
) -> Vec<WorkGraphOperatorReviewPacketApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphOperatorReviewPacketApplicationPreview {
                application_id: packet_application_id_for(&assertion.packet_id),
                packet_id: assertion.packet_id.clone(),
                source_surface_id: assertion.source_surface_id,
                required_section_ids: assertion.required_section_ids.clone(),
                required_evidence_field_ids: assertion.required_evidence_field_ids.clone(),
                expected_packet_state:
                    "packet_contract_ready_preview_after_application_not_recorded",
                packet_contract_ready_preview: true,
                readback_verified_by_preview: true,
                records_operator_review: false,
                records_approval: false,
            },
        )
        .collect()
}

fn side_effect_lock_applications_from(
    assertions: &[WorkGraphSideEffectLockReadbackAssertionPreview],
) -> Vec<WorkGraphSideEffectLockApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphSideEffectLockApplicationPreview {
            application_id: lock_application_id_for(&assertion.lock_plan_id),
            lock_plan_id: assertion.lock_plan_id.clone(),
            source_surface_id: assertion.source_surface_id,
            lock_scope_ids: assertion.lock_scope_ids.clone(),
            expected_lock_state:
                "side_effect_lock_contract_ready_preview_after_application_not_established",
            lock_contract_ready_preview: true,
            readback_verified_by_preview: true,
            establishes_side_effect_lock: false,
            mutates_runtime: false,
        })
        .collect()
}

fn approval_boundary_applications_from(
    assertions: &[WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview],
) -> Vec<WorkGraphApprovalEvidenceBoundaryApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphApprovalEvidenceBoundaryApplicationPreview {
                application_id: approval_application_id_for(&assertion.boundary_id),
                boundary_id: assertion.boundary_id.clone(),
                source_surface_id: assertion.source_surface_id,
                required_evidence_field_ids: assertion.required_evidence_field_ids.clone(),
                expected_boundary_state:
                    "approval_evidence_contract_ready_preview_after_application_not_recorded",
                boundary_contract_ready_preview: true,
                readback_verified_by_preview: true,
                records_operator_review: false,
                records_approval: false,
                persists_receipt: false,
            },
        )
        .collect()
}

fn readback_boundary_applications_from(
    assertions: &[WorkGraphOperatorReviewBoundaryReadbackAssertionPreview],
) -> Vec<WorkGraphOperatorReviewBoundaryApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphOperatorReviewBoundaryApplicationPreview {
                application_id: readback_boundary_application_id_for(&assertion.boundary_id),
                boundary_id: assertion.boundary_id.clone(),
                readback_probe_id: assertion.readback_probe_id.clone(),
                source_surface_id: assertion.source_surface_id,
                expected_boundary_state:
                    "readback_boundary_contract_ready_preview_after_application_not_executed",
                boundary_contract_ready_preview: true,
                readback_verified_by_preview: true,
                executes_readback: false,
                rollback_executed: false,
                writes_checkpoint: false,
            },
        )
        .collect()
}

fn group_applications_from(
    assertions: &[WorkGraphOperatorReviewGroupReadbackAssertionPreview],
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
) -> Vec<WorkGraphOperatorReviewGroupApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphOperatorReviewGroupApplicationPreview {
            application_id: group_application_id_for(assertion.group_id),
            group_id: assertion.group_id,
            source_category: assertion.source_category,
            affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
            application_plan_ids: application_plan_ids_for_sources(
                plans,
                &assertion.affected_source_surface_ids,
            ),
            operator_review_packet_ids: assertion.operator_review_packet_ids.clone(),
            side_effect_lock_plan_ids: assertion.side_effect_lock_plan_ids.clone(),
            expected_contract_count_after_application: assertion.expected_review_packet_count,
            group_contract_ready_preview: true,
            readback_verified_by_preview: true,
            records_operator_review: false,
            establishes_side_effect_lock: false,
        })
        .collect()
}

fn blocker_applications_from(
    assertions: &[WorkGraphOperatorReviewBlockerMappingAssertionPreview],
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
) -> Vec<WorkGraphOperatorReviewBlockerApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphOperatorReviewBlockerApplicationPreview {
            application_id: blocker_application_id_for(assertion.blocker_id),
            blocker_id: assertion.blocker_id,
            severity: assertion.severity,
            affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
            affected_readback_plan_ids: assertion.affected_readback_plan_ids.clone(),
            affected_application_plan_ids: application_plan_ids_for_readback_plans(
                plans,
                &assertion.affected_readback_plan_ids,
            ),
            expected_blocker_state:
                "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
            blocker_contract_ready_preview: true,
            readback_verified_by_preview: true,
            clears_operator_review_blocker: false,
            clears_side_effect_lock_blocker: false,
            mutates_runtime: false,
        })
        .collect()
}

fn application_blockers_from(
    readback_blockers: &[WorkGraphOperatorReviewReadbackBlockerPreview],
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview> {
    let mut blockers = readback_blockers
        .iter()
        .map(|blocker| application_blocker_from_readback_blocker(blocker, plans))
        .collect::<Vec<_>>();
    blockers.push(application_blocker(
        "operator_review_side_effect_lock_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        affected_sources(plans, |_| true),
        application_plan_ids(plans, |_| true),
        "rerun unified projection enforcement-readiness against operator-review side-effect lock application preview outcomes",
    ));
    blockers
}

fn application_blocker_from_readback_blocker(
    blocker: &WorkGraphOperatorReviewReadbackBlockerPreview,
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
) -> WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        "operator_review_side_effect_lock",
        blocker.affected_source_surface_ids.clone(),
        application_plan_ids_for_sources(plans, &blocker.affected_source_surface_ids),
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
) -> WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview {
    WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_operator_review_side_effect_lock: true,
        recommended_fix,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphOperatorReviewSideEffectLockApplicationGuardPreview {
    WorkGraphOperatorReviewSideEffectLockApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_operator_review_side_effect_lock: true,
        satisfied_by_preview: false,
    }
}

fn application_plan_ids_for_sources(
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
    source_ids: &[&'static str],
) -> Vec<String> {
    application_plan_ids(plans, |plan| source_ids.contains(&plan.source_surface_id))
}

fn application_plan_ids_for_readback_plans(
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
    readback_plan_ids: &[String],
) -> Vec<String> {
    application_plan_ids(plans, |plan| {
        readback_plan_ids.contains(&plan.readback_plan_id)
    })
}

fn application_plan_ids(
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphOperatorReviewSideEffectLockApplicationPlanPreview) -> bool,
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
    format!("apply_{readback_plan_id}_operator_review_side_effect_lock_preview")
}

fn packet_application_id_for(packet_id: &str) -> String {
    format!("apply_{packet_id}_operator_review_packet_preview")
}

fn lock_application_id_for(lock_plan_id: &str) -> String {
    format!("apply_{lock_plan_id}_side_effect_lock_preview")
}

fn approval_application_id_for(boundary_id: &str) -> String {
    format!("apply_{boundary_id}_approval_evidence_preview")
}

fn readback_boundary_application_id_for(boundary_id: &str) -> String {
    format!("apply_{boundary_id}_readback_boundary_preview")
}

fn group_application_id_for(group_id: &str) -> String {
    format!("apply_{group_id}_operator_review_group_preview")
}

fn blocker_application_id_for(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_operator_review_side_effect_lock_blocker_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_review_side_effect_lock_application_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate"
        );
        assert_eq!(
            work_graph_append_only_store_operator_review_side_effect_lock_application_guards()
                .len(),
            12
        );
        assert_eq!(
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects::none(
            ),
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockApplicationPreviewSideEffects::none(
            )
        );
    }

    #[test]
    fn operator_review_side_effect_lock_application_maps_readback_plan_without_runtime_mutation() {
        let readback_plans = vec![WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview {
            id: "operator_review_side_effect_lock_readback_plan__sample".to_string(),
            source_surface_id: "sample",
            source_category: "runtime_scheduler",
            operator_review_packet_id: "operator_review_packet__sample".to_string(),
            side_effect_lock_plan_id: "side_effect_lock_plan__sample".to_string(),
            approval_evidence_boundary_id: "approval_boundary__sample".to_string(),
            readback_boundary_id: "readback_boundary__sample".to_string(),
            required_evidence_fields: vec!["operator_review_packet_id", "side_effect_lock_id"],
            lock_scope_ids: vec!["runtime_mutation", "wal_write"],
            readback_state: "asserted_from_operator_review_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            records_operator_review: false,
            records_approval: false,
            establishes_side_effect_lock: false,
            mutates_store: false,
            writes_wal: false,
        }];
        let application_plans = application_plans_from(&readback_plans);
        let required_prior_gates =
            work_graph_append_only_store_operator_review_side_effect_lock_application_required_prior_gates();

        assert_eq!(application_plans.len(), 1);
        assert_eq!(
            required_prior_gates.last().copied(),
            Some(
                WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE
            )
        );
        assert_eq!(required_prior_gates.len(), 50);
        assert!(application_plans.iter().all(|plan| {
            plan.readback_verified_by_preview
                && plan.operator_review_contract_ready_preview
                && plan.side_effect_lock_contract_ready_preview
                && !plan.records_operator_review
                && !plan.records_approval
                && !plan.establishes_side_effect_lock
                && !plan.executes_readback
                && !plan.writes_store
                && !plan.writes_wal
                && !plan.mutates_runtime
        }));
    }
}
