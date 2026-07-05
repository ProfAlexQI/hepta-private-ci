use serde::Serialize;

use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationBlockerMappingReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationEvidenceFieldReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationGuardReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationReadbackBlockerPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationReadbackPlanPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::WorkGraphIdempotencyMutationStageReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::hepta_work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_report;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::work_graph_append_only_store_runtime_idempotency_mutation_readback_plans;
use crate::work_graph_append_only_store_runtime_idempotency_mutation_readback_preview::work_graph_append_only_store_runtime_idempotency_mutation_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_SCHEMA_VERSION:
    &str = "work_graph_append_only_store_runtime_idempotency_mutation_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub idempotency_mutation_contract_ready_preview_count: usize,
    pub stage_application_count: usize,
    pub evidence_field_application_count: usize,
    pub guard_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub stage_source_ref_count: usize,
    pub stage_contract_ref_count: usize,
    pub plan_stage_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub blocker_mapping_stage_ref_count: usize,
    pub application_plans: Vec<WorkGraphIdempotencyMutationApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphIdempotencyMutationApplicationSourceOutcomePreview>,
    pub stage_applications: Vec<WorkGraphIdempotencyMutationStageApplicationPreview>,
    pub evidence_field_applications:
        Vec<WorkGraphIdempotencyMutationEvidenceFieldApplicationPreview>,
    pub guard_applications: Vec<WorkGraphIdempotencyMutationGuardApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphIdempotencyMutationBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphIdempotencyMutationApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphIdempotencyMutationApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview:
        bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_idempotency_mutation: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub idempotency_mutation_plan_id: String,
    pub required_idempotency_mutation_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub idempotency_mutation_policy_contract_ready_preview: bool,
    pub collision_replay_evidence_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_idempotency_mutation_state: &'static str,
    pub idempotency_mutation_policy_contract_ready_preview: bool,
    pub collision_replay_evidence_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview:
        bool,
    pub ready_for_wal_write: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationStageApplicationPreview {
    pub application_id: String,
    pub stage_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_stage_state: &'static str,
    pub stage_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub declared_writes_wal: bool,
    pub declared_writes_checkpoint: bool,
    pub declared_mutates_idempotency_index: bool,
    pub declared_executes_replay: bool,
    pub declared_executes_readback: bool,
    pub declared_executes_rollback: bool,
    pub enables_runtime_after_application: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationEvidenceFieldApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_evidence_state: &'static str,
    pub evidence_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub persists_evidence: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationGuardApplicationPreview {
    pub application_id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub guard_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub satisfied_by_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_idempotency_mutation_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_idempotency_mutation_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_idempotency_mutation: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphIdempotencyMutationApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_idempotency_mutation_stage_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_idempotency_mutation: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub readback_executed: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_report();
    let application_plans =
        work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(
            &readback_report.readback_plans,
        );
    let source_outcomes =
        work_graph_append_only_store_runtime_idempotency_mutation_application_source_outcomes_from(
            &application_plans,
        );
    let stage_applications =
        work_graph_append_only_store_runtime_idempotency_mutation_stage_applications_from(
            &readback_report.stage_assertions,
        );
    let evidence_field_applications =
        work_graph_append_only_store_runtime_idempotency_mutation_evidence_field_applications_from(
            &readback_report.evidence_field_assertions,
        );
    let guard_applications =
        work_graph_append_only_store_runtime_idempotency_mutation_guard_applications_from(
            &readback_report.guard_assertions,
        );
    let blocker_applications =
        work_graph_append_only_store_runtime_idempotency_mutation_blocker_applications_from(
            &readback_report.blocker_mapping_assertions,
            &application_plans,
        );
    let application_guards =
        work_graph_append_only_store_runtime_idempotency_mutation_application_guards();
    let blockers =
        work_graph_append_only_store_runtime_idempotency_mutation_application_blockers_from(
            &readback_report.blockers,
            &application_plans,
        );
    let required_prior_gates =
        work_graph_append_only_store_runtime_idempotency_mutation_application_required_prior_gates(
        );

    WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_runtime_idempotency_mutation_application_no_index_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        idempotency_mutation_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| {
                outcome.idempotency_mutation_policy_contract_ready_preview
                    && outcome.collision_replay_evidence_contract_ready_preview
            })
            .count(),
        stage_application_count: stage_applications.len(),
        evidence_field_application_count: evidence_field_applications.len(),
        guard_application_count: guard_applications.len(),
        blocker_application_count: blocker_applications.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        stage_source_ref_count: stage_applications
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        stage_contract_ref_count: stage_applications
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        plan_stage_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_idempotency_mutation_stage_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|blocker| blocker.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_stage_ref_count: blocker_applications
            .iter()
            .map(|blocker| blocker.affected_idempotency_mutation_stage_ids.len())
            .sum(),
        application_plans,
        source_outcomes,
        stage_applications,
        evidence_field_applications,
        guard_applications,
        blocker_applications,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview:
            true,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_idempotency_mutation: false,
        ready_for_readback_execution: false,
        ready_for_replay_execution: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_idempotency_mutation_application_plans()
-> Vec<WorkGraphIdempotencyMutationApplicationPlanPreview> {
    let readback_plans = work_graph_append_only_store_runtime_idempotency_mutation_readback_plans();
    work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_idempotency_mutation_application_source_outcomes()
-> Vec<WorkGraphIdempotencyMutationApplicationSourceOutcomePreview> {
    let application_plans =
        work_graph_append_only_store_runtime_idempotency_mutation_application_plans();
    work_graph_append_only_store_runtime_idempotency_mutation_application_source_outcomes_from(
        &application_plans,
    )
}

pub fn work_graph_append_only_store_runtime_idempotency_mutation_application_blockers()
-> Vec<WorkGraphIdempotencyMutationApplicationBlockerPreview> {
    let readback_report =
        hepta_work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_report();
    let application_plans =
        work_graph_append_only_store_runtime_idempotency_mutation_application_plans();
    work_graph_append_only_store_runtime_idempotency_mutation_application_blockers_from(
        &readback_report.blockers,
        &application_plans,
    )
}

pub fn work_graph_append_only_store_runtime_idempotency_mutation_application_guards()
-> Vec<WorkGraphIdempotencyMutationApplicationGuardPreview> {
    vec![
        application_guard(
            "idempotency_mutation_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        application_guard("checkpoint_write_disabled", "critical", "checkpoint"),
        application_guard("replay_execution_disabled", "critical", "replay"),
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

pub fn work_graph_append_only_store_runtime_idempotency_mutation_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_idempotency_mutation_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewSideEffects {
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
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            role_manifest_enforcement_enabled: false,
            readback_executed: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(
    readback_plans: &[WorkGraphIdempotencyMutationReadbackPlanPreview],
) -> Vec<WorkGraphIdempotencyMutationApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphIdempotencyMutationApplicationPlanPreview {
            application_plan_id: application_plan_id(&plan.id),
            readback_plan_id: plan.id.clone(),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            idempotency_mutation_plan_id: plan.idempotency_mutation_plan_id.clone(),
            required_idempotency_mutation_stage_ids: plan
                .required_idempotency_mutation_stage_ids
                .clone(),
            residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
            expected_evidence_field_ids: plan.required_evidence_field_ids.clone(),
            application_scope: "idempotency_mutation_application_binding",
            application_state: "preview_application_defined_idempotency_mutation_not_enabled",
            readback_verified_by_preview: true,
            idempotency_mutation_policy_contract_ready_preview: true,
            collision_replay_evidence_contract_ready_preview: true,
            applies_to_runtime: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_application_source_outcomes_from(
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
) -> Vec<WorkGraphIdempotencyMutationApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(|plan| WorkGraphIdempotencyMutationApplicationSourceOutcomePreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            application_plan_id: plan.application_plan_id.clone(),
            post_application_idempotency_mutation_state:
                "idempotency_mutation_contract_ready_preview_after_application",
            idempotency_mutation_policy_contract_ready_preview: true,
            collision_replay_evidence_contract_ready_preview: true,
            ready_for_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview:
                true,
            ready_for_wal_write: false,
            applies_to_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_stage_applications_from(
    stage_assertions: &[WorkGraphIdempotencyMutationStageReadbackAssertionPreview],
) -> Vec<WorkGraphIdempotencyMutationStageApplicationPreview> {
    stage_assertions
        .iter()
        .map(
            |stage| WorkGraphIdempotencyMutationStageApplicationPreview {
                application_id: format!("apply_{}_idempotency_mutation_stage_preview", stage.stage_id),
                stage_id: stage.stage_id,
                category: stage.category,
                affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
                expected_stage_state:
                    "stage_contract_ready_preview_after_application_runtime_disabled",
                stage_contract_ready_preview: true,
                readback_verified_by_preview: true,
                declared_writes_wal: stage.declared_writes_wal,
                declared_writes_checkpoint: stage.declared_writes_checkpoint,
                declared_mutates_idempotency_index: stage.declared_mutates_idempotency_index,
                declared_executes_replay: stage.declared_executes_replay,
                declared_executes_readback: stage.declared_executes_readback,
                declared_executes_rollback: stage.declared_executes_rollback,
                enables_runtime_after_application: false,
                writes_wal: false,
                writes_checkpoint: false,
                mutates_idempotency_index: false,
                executes_replay: false,
                executes_readback: false,
                executes_rollback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_evidence_field_applications_from(
    assertions: &[WorkGraphIdempotencyMutationEvidenceFieldReadbackAssertionPreview],
) -> Vec<WorkGraphIdempotencyMutationEvidenceFieldApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphIdempotencyMutationEvidenceFieldApplicationPreview {
                application_id: format!(
                    "apply_{}_idempotency_mutation_evidence_preview",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                required_evidence_field_ids: assertion.required_evidence_field_ids.clone(),
                expected_evidence_state:
                    "evidence_contract_ready_preview_after_application_not_persisted",
                evidence_contract_ready_preview: true,
                readback_verified_by_preview: true,
                persists_evidence: false,
                writes_store: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_guard_applications_from(
    assertions: &[WorkGraphIdempotencyMutationGuardReadbackAssertionPreview],
) -> Vec<WorkGraphIdempotencyMutationGuardApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphIdempotencyMutationGuardApplicationPreview {
                application_id: format!(
                    "apply_{}_idempotency_mutation_guard_preview",
                    assertion.guard_id
                ),
                guard_id: assertion.guard_id,
                severity: assertion.severity,
                guard_scope: assertion.guard_scope,
                expected_guard_state:
                    "guard_contract_ready_preview_after_application_runtime_mutation_prevented",
                guard_contract_ready_preview: true,
                readback_verified_by_preview: true,
                satisfied_by_preview: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_blocker_applications_from(
    assertions: &[WorkGraphIdempotencyMutationBlockerMappingReadbackAssertionPreview],
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
) -> Vec<WorkGraphIdempotencyMutationBlockerApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphIdempotencyMutationBlockerApplicationPreview {
                application_id: format!(
                    "apply_{}_idempotency_mutation_blocker_preview",
                    assertion.blocker_id
                ),
                blocker_id: assertion.blocker_id,
                severity: assertion.severity,
                category: assertion.category,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                affected_idempotency_mutation_stage_ids: assertion
                    .affected_idempotency_mutation_stage_ids
                    .clone(),
                affected_readback_plan_ids: assertion.affected_readback_plan_ids.clone(),
                affected_application_plan_ids: application_plan_ids_for_readback_plans(
                    &assertion.affected_readback_plan_ids,
                    application_plans,
                ),
                expected_blocker_state:
                    "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
                blocker_contract_ready_preview: true,
                readback_verified_by_preview: true,
                clears_idempotency_mutation_blocker: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_idempotency_mutation_application_blockers_from(
    readback_blockers: &[WorkGraphIdempotencyMutationReadbackBlockerPreview],
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
) -> Vec<WorkGraphIdempotencyMutationApplicationBlockerPreview> {
    let mut blockers = readback_blockers
        .iter()
        .map(|blocker| {
            application_blocker(
                blocker.id,
                blocker.severity,
                blocker.category,
                blocker.affected_source_surface_ids.clone(),
                blocker.affected_idempotency_mutation_stage_ids.clone(),
                application_plans,
                blocker.recommended_fix,
            )
        })
        .collect::<Vec<_>>();
    let affected_source_surface_ids = application_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let affected_idempotency_mutation_stage_ids = application_plans
        .first()
        .map(|plan| plan.required_idempotency_mutation_stage_ids.clone())
        .unwrap_or_default();
    blockers.push(application_blocker(
        "idempotency_mutation_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        affected_source_surface_ids,
        affected_idempotency_mutation_stage_ids,
        application_plans,
        "rerun unified projection enforcement-readiness against runtime idempotency mutation application preview outcomes",
    ));
    blockers
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphIdempotencyMutationApplicationGuardPreview {
    WorkGraphIdempotencyMutationApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_idempotency_mutation: true,
        satisfied_by_preview: false,
    }
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_idempotency_mutation_stage_ids: Vec<&'static str>,
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
    recommended_fix: &'static str,
) -> WorkGraphIdempotencyMutationApplicationBlockerPreview {
    WorkGraphIdempotencyMutationApplicationBlockerPreview {
        affected_application_plan_ids: application_plan_ids_for_sources(
            &affected_source_surface_ids,
            application_plans,
        ),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_idempotency_mutation_stage_ids,
        required_before_idempotency_mutation: true,
        recommended_fix,
    }
}

fn application_plan_ids_for_sources(
    affected_source_surface_ids: &[&str],
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
) -> Vec<String> {
    application_plans
        .iter()
        .filter(|plan| affected_source_surface_ids.contains(&plan.source_surface_id))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn application_plan_ids_for_readback_plans(
    readback_plan_ids: &[String],
    application_plans: &[WorkGraphIdempotencyMutationApplicationPlanPreview],
) -> Vec<String> {
    application_plans
        .iter()
        .filter(|plan| readback_plan_ids.contains(&plan.readback_plan_id))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn application_plan_id(readback_plan_id: &str) -> String {
    format!("apply_{readback_plan_id}_idempotency_mutation_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_mutation_application_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_IDEMPOTENCY_MUTATION_APPLICATION_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview_gate"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeIdempotencyMutationApplicationPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.replay_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
        assert!(!side_effects.agent_spawn_performed);
    }

    #[test]
    fn idempotency_mutation_application_plans_preserve_readback_contracts() {
        let plans =
            work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(&[
                sample_readback_plan(),
            ]);

        assert_eq!(plans.len(), 1);
        let plan = &plans[0];
        assert_eq!(
            plan.application_state,
            "preview_application_defined_idempotency_mutation_not_enabled"
        );
        assert!(plan.readback_verified_by_preview);
        assert!(plan.idempotency_mutation_policy_contract_ready_preview);
        assert!(plan.collision_replay_evidence_contract_ready_preview);
        assert!(!plan.applies_to_runtime);
        assert!(!plan.writes_wal);
        assert!(!plan.mutates_idempotency_index);
        assert!(!plan.mutates_runtime);
    }

    #[test]
    fn idempotency_mutation_application_outcomes_prepare_rerun_only() {
        let plans =
            work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(&[
                sample_readback_plan(),
            ]);
        let outcomes =
            work_graph_append_only_store_runtime_idempotency_mutation_application_source_outcomes_from(
                &plans,
            );

        assert_eq!(outcomes.len(), 1);
        assert_eq!(
            outcomes[0].post_application_idempotency_mutation_state,
            "idempotency_mutation_contract_ready_preview_after_application"
        );
        assert!(outcomes[0]
            .ready_for_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview);
        assert!(!outcomes[0].ready_for_wal_write);
        assert!(!outcomes[0].applies_to_runtime);
    }

    #[test]
    fn idempotency_mutation_application_blockers_add_rerun_missing() {
        let plans =
            work_graph_append_only_store_runtime_idempotency_mutation_application_plans_from(&[
                sample_readback_plan(),
            ]);
        let blockers =
            work_graph_append_only_store_runtime_idempotency_mutation_application_blockers_from(
                &[],
                &plans,
            );

        assert_eq!(blockers.len(), 1);
        assert_eq!(
            blockers[0].id,
            "idempotency_mutation_readiness_rerun_missing"
        );
        assert!(blockers[0].required_before_idempotency_mutation);
        assert_eq!(blockers[0].affected_application_plan_ids.len(), 1);
    }

    fn sample_readback_plan() -> WorkGraphIdempotencyMutationReadbackPlanPreview {
        WorkGraphIdempotencyMutationReadbackPlanPreview {
            id: "readback_plan".to_string(),
            source_surface_id: "sample",
            source_category: "planning",
            idempotency_mutation_plan_id: "source_plan".to_string(),
            required_idempotency_mutation_stage_ids: vec![
                "idempotency_mutation_policy_contract",
                "idempotency_collision_replay_evidence_contract",
                "idempotency_index_no_mutation_guard",
                "rollback_readback_prerequisite_contract",
                "idempotency_blocker_mapping",
            ],
            residual_source_blocker_ids: vec!["idempotency_index_mutation_disabled"],
            required_evidence_field_ids: vec!["source_surface_id"],
            readback_state: "readback_verified_from_idempotency_mutation_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_rollback: false,
            mutates_runtime: false,
        }
    }
}
