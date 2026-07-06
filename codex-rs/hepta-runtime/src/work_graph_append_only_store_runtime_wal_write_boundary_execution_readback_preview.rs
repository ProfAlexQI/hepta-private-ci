use serde::Serialize;

use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::WorkGraphWalWriteBoundaryExecutionBlockerPreview;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::WorkGraphWalWriteBoundaryExecutionGuardPreview;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::WorkGraphWalWriteBoundaryExecutionSourcePlanPreview;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::WorkGraphWalWriteBoundaryExecutionStagePlanPreview;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::work_graph_append_only_store_runtime_wal_write_boundary_execution_blockers;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::work_graph_append_only_store_runtime_wal_write_boundary_execution_guards;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::work_graph_append_only_store_runtime_wal_write_boundary_execution_plans;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::work_graph_append_only_store_runtime_wal_write_boundary_execution_required_prior_gates;
use crate::work_graph_append_only_store_runtime_wal_write_boundary_execution_preview::work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_plans;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_wal_write_boundary_execution_preview_gate: &'static str,
    pub source_surface_count: usize,
    pub wal_write_boundary_execution_plan_count: usize,
    pub readback_plan_count: usize,
    pub stage_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub wal_write_boundary_execution_stage_source_ref_count: usize,
    pub wal_write_boundary_execution_stage_contract_ref_count: usize,
    pub wal_write_boundary_execution_plan_stage_ref_count: usize,
    pub wal_write_boundary_execution_plan_evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub blocker_mapping_stage_ref_count: usize,
    pub readback_plans: Vec<WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphWalWriteBoundaryExecutionStageReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphWalWriteBoundaryExecutionEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphWalWriteBoundaryExecutionGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphWalWriteBoundaryExecutionBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphWalWriteBoundaryExecutionReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_wal_write_boundary_execution_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_wal_write_boundary_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub wal_write_boundary_execution_plan_id: String,
    pub required_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub required_before_application: bool,
    pub performs_readback: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionStageReadbackAssertionPreview {
    pub id: String,
    pub stage_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub contract_ready_preview: bool,
    pub runtime_enabled_after_readback: bool,
    pub declared_writes_wal: bool,
    pub declared_writes_checkpoint: bool,
    pub declared_mutates_idempotency_index: bool,
    pub declared_executes_replay: bool,
    pub declared_executes_readback: bool,
    pub declared_executes_rollback: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_wal_write_boundary_execution: bool,
    pub satisfied_by_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionBlockerMappingReadbackAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocks_wal_write_boundary_execution: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_application_preview: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocks_wal_write_boundary_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewSideEffects {
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

pub fn hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewReport {
    let source_plans = work_graph_append_only_store_runtime_wal_write_boundary_execution_plans();
    let stage_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_plans();
    let guards = work_graph_append_only_store_runtime_wal_write_boundary_execution_guards();
    let preview_blockers =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_blockers();
    let readback_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans_from(
            &source_plans,
        );
    let stage_assertions =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_readback_assertions_from(
            &stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_evidence_field_readback_assertions_from(
            &readback_plans,
        );
    let guard_assertions =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_guard_readback_assertions_from(
            &guards,
        );
    let blockers =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers_from(
            &preview_blockers,
            &readback_plans,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_blocker_mapping_assertions_from(
            &blockers,
        );
    let drift_detectors =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_drift_detectors(
        );
    let required_prior_gates =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_required_prior_gates();

    WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_wal_write_boundary_execution_readback_no_execution",
        upstream_wal_write_boundary_execution_preview_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE,
        source_surface_count: source_plans.len(),
        wal_write_boundary_execution_plan_count: source_plans.len(),
        readback_plan_count: readback_plans.len(),
        stage_assertion_count: stage_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        wal_write_boundary_execution_stage_source_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        wal_write_boundary_execution_stage_contract_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        wal_write_boundary_execution_plan_stage_ref_count: readback_plans
            .iter()
            .map(|plan| plan.required_wal_write_boundary_execution_stage_ids.len())
            .sum(),
        wal_write_boundary_execution_plan_evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        blocker_mapping_source_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_stage_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_wal_write_boundary_execution_stage_ids.len())
            .sum(),
        readback_plans,
        stage_assertions,
        evidence_field_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_wal_write_boundary_execution_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_replay_execution: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_wal_write_boundary_execution: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans()
-> Vec<WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview> {
    let source_plans = work_graph_append_only_store_runtime_wal_write_boundary_execution_plans();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans_from(
        &source_plans,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_readback_assertions()
-> Vec<WorkGraphWalWriteBoundaryExecutionStageReadbackAssertionPreview> {
    let stage_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_plans();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_readback_assertions_from(
        &stage_plans,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_evidence_field_readback_assertions()
-> Vec<WorkGraphWalWriteBoundaryExecutionEvidenceFieldReadbackAssertionPreview> {
    let readback_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_evidence_field_readback_assertions_from(
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_guard_readback_assertions()
-> Vec<WorkGraphWalWriteBoundaryExecutionGuardReadbackAssertionPreview> {
    let guards = work_graph_append_only_store_runtime_wal_write_boundary_execution_guards();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_guard_readback_assertions_from(
        &guards,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers()
-> Vec<WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview> {
    let preview_blockers =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_blockers();
    let readback_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers_from(
        &preview_blockers,
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_blocker_mapping_assertions()
-> Vec<WorkGraphWalWriteBoundaryExecutionBlockerMappingReadbackAssertionPreview> {
    let blockers =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_blocker_mapping_assertions_from(
        &blockers,
    )
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_drift_detectors()
-> Vec<WorkGraphWalWriteBoundaryExecutionReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "wal_write_boundary_execution_plan_alignment",
            vec![
                "wal_write_boundary_execution_plan_id",
                "required_wal_write_boundary_execution_stage_ids",
            ],
        ),
        drift_detector(
            "wal_write_boundary_execution_stage_contract_alignment",
            vec!["stage_id", "required_contract_ref_ids"],
        ),
        drift_detector(
            "wal_write_boundary_execution_evidence_field_alignment",
            vec!["source_surface_id", "required_evidence_field_ids"],
        ),
        drift_detector(
            "wal_write_boundary_execution_guard_no_mutation_alignment",
            vec!["guard_id", "mutates_runtime"],
        ),
        drift_detector(
            "wal_write_boundary_execution_blocker_mapping_alignment",
            vec!["blocker_id", "affected_readback_plan_ids"],
        ),
        drift_detector(
            "wal_write_boundary_execution_side_effect_alignment",
            vec![
                "side_effects",
                "idempotency_index_mutated",
                "runtime_mutation_performed",
            ],
        ),
        drift_detector(
            "wal_write_boundary_execution_upstream_gate_alignment",
            vec![
                "upstream_wal_write_boundary_execution_preview_gate",
                "recommended_next_gate",
            ],
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewSideEffects {
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

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans_from(
    source_plans: &[WorkGraphWalWriteBoundaryExecutionSourcePlanPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview> {
    source_plans
        .iter()
        .map(|plan| WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview {
            id: wal_write_boundary_execution_readback_plan_id(plan.source_surface_id),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            wal_write_boundary_execution_plan_id: plan.wal_write_boundary_execution_plan_id.clone(),
            required_wal_write_boundary_execution_stage_ids: plan
                .required_wal_write_boundary_execution_stage_ids
                .clone(),
            residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
            required_evidence_field_ids: plan.expected_evidence_field_ids.clone(),
            readback_state: "readback_verified_from_wal_write_boundary_execution_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_readback_assertions_from(
    stage_plans: &[WorkGraphWalWriteBoundaryExecutionStagePlanPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionStageReadbackAssertionPreview> {
    stage_plans
        .iter()
        .map(
            |stage| WorkGraphWalWriteBoundaryExecutionStageReadbackAssertionPreview {
                id: format!(
                    "wal_write_boundary_execution_stage_readback_assertion__{}",
                    stage.id
                ),
                stage_id: stage.id,
                category: stage.category,
                affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
                expected_runtime_state: "readback_verified_contract_ready_runtime_disabled",
                contract_ready_preview: stage.contract_ready_preview,
                runtime_enabled_after_readback: false,
                declared_writes_wal: stage.writes_wal,
                declared_writes_checkpoint: stage.writes_checkpoint,
                declared_mutates_idempotency_index: stage.mutates_idempotency_index,
                declared_executes_replay: stage.executes_replay,
                declared_executes_readback: stage.executes_readback,
                declared_executes_rollback: stage.executes_rollback,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_evidence_field_readback_assertions_from(
    readback_plans: &[WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionEvidenceFieldReadbackAssertionPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphWalWriteBoundaryExecutionEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "wal_write_boundary_execution_evidence_field_readback_assertion__{}",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                required_evidence_field_ids: plan.required_evidence_field_ids.clone(),
                required_field_count: plan.required_evidence_field_ids.len(),
                expected_evidence_state: "evidence_fields_declared_not_persisted",
                performs_readback: false,
                persists_evidence: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_guard_readback_assertions_from(
    guards: &[WorkGraphWalWriteBoundaryExecutionGuardPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionGuardReadbackAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphWalWriteBoundaryExecutionGuardReadbackAssertionPreview {
                id: format!(
                    "wal_write_boundary_execution_guard_readback_assertion__{}",
                    guard.id
                ),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
                required_before_wal_write_boundary_execution: guard
                    .required_before_wal_write_boundary_execution,
                satisfied_by_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers_from(
    preview_blockers: &[WorkGraphWalWriteBoundaryExecutionBlockerPreview],
    readback_plans: &[WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview> {
    let mut blockers = preview_blockers
        .iter()
        .map(|blocker| readback_blocker_from_preview(blocker, readback_plans))
        .collect::<Vec<_>>();
    let affected_source_surface_ids = readback_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let affected_wal_write_boundary_execution_stage_ids = readback_plans
        .first()
        .map(|plan| plan.required_wal_write_boundary_execution_stage_ids.clone())
        .unwrap_or_default();
    blockers.push(readback_blocker(
        "wal_write_boundary_execution_application_missing",
        "high",
        "application_preview",
        affected_source_surface_ids,
        affected_wal_write_boundary_execution_stage_ids,
        readback_plans,
        "apply readback-verified WAL write-boundary execution plans before any idempotency index mutation, WAL replay, rollback/readback, or projection enforcement promotion",
    ));
    blockers
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_blocker_mapping_assertions_from(
    blockers: &[WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionBlockerMappingReadbackAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphWalWriteBoundaryExecutionBlockerMappingReadbackAssertionPreview {
                id: format!(
                    "wal_write_boundary_execution_blocker_mapping_readback_assertion__{}",
                    blocker.id
                ),
                blocker_id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_wal_write_boundary_execution_stage_ids: blocker
                    .affected_wal_write_boundary_execution_stage_ids
                    .clone(),
                affected_readback_plan_ids: blocker.affected_readback_plan_ids.clone(),
                expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
                blocks_wal_write_boundary_execution: blocker.blocks_wal_write_boundary_execution,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn readback_blocker_from_preview(
    blocker: &WorkGraphWalWriteBoundaryExecutionBlockerPreview,
    readback_plans: &[WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview],
) -> WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview {
    readback_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        blocker
            .affected_wal_write_boundary_execution_stage_ids
            .clone(),
        readback_plans,
        blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    readback_plans: &[WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview],
    recommended_fix: &'static str,
) -> WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview {
    WorkGraphWalWriteBoundaryExecutionReadbackBlockerPreview {
        affected_readback_plan_ids: affected_readback_plan_ids(
            &affected_source_surface_ids,
            readback_plans,
        ),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_wal_write_boundary_execution_stage_ids,
        blocks_wal_write_boundary_execution: true,
        recommended_fix,
    }
}

fn affected_readback_plan_ids(
    affected_source_surface_ids: &[&str],
    readback_plans: &[WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview],
) -> Vec<String> {
    readback_plans
        .iter()
        .filter(|plan| affected_source_surface_ids.contains(&plan.source_surface_id))
        .map(|plan| plan.id.clone())
        .collect()
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
) -> WorkGraphWalWriteBoundaryExecutionReadbackDriftDetectorPreview {
    WorkGraphWalWriteBoundaryExecutionReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity: "high",
        blocks_application_preview: true,
        performs_readback: false,
    }
}

fn wal_write_boundary_execution_readback_plan_id(source_surface_id: &str) -> String {
    format!(
        "append_only_store_runtime_wal_write_boundary_execution_readback_plan__{source_surface_id}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wal_write_boundary_execution_readback_declares_no_execution_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_application_preview_gate"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionReadbackPreviewSideEffects::none();
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
    fn wal_write_boundary_execution_readback_plans_preserve_preview_boundary() {
        let source_plans = vec![sample_source_plan("update_plan_tool", "planning")];
        let plans =
            work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_plans_from(
                &source_plans,
            );

        assert_eq!(plans.len(), 1);
        let plan = &plans[0];
        assert_eq!(
            plan.readback_state,
            "readback_verified_from_wal_write_boundary_execution_preview_no_execution"
        );
        assert!(plan.required_before_application);
        assert!(!plan.performs_readback);
        assert!(!plan.writes_wal);
        assert!(!plan.writes_checkpoint);
        assert!(!plan.mutates_idempotency_index);
        assert!(!plan.executes_replay);
        assert!(!plan.executes_rollback);
        assert!(!plan.mutates_runtime);
    }

    #[test]
    fn wal_write_boundary_execution_stage_readback_keeps_declared_effects_disabled() {
        let stage = WorkGraphWalWriteBoundaryExecutionStagePlanPreview {
            id: "wal_write_boundary_execution_policy_contract",
            priority: "p0",
            category: "idempotency_policy",
            affected_source_surface_ids: vec!["sample"],
            required_contract_ref_ids: vec!["idempotency_index_key_contract_ready"],
            expected_runtime_state: "contract_ready_preview_runtime_disabled",
            prerequisite_gate_ids: Vec::new(),
            contract_ready_preview: true,
            runtime_enabled_after_preview: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: true,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
        };
        let assertions =
            work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_readback_assertions_from(
                &[stage],
            );

        assert_eq!(assertions.len(), 1);
        assert!(assertions[0].declared_mutates_idempotency_index);
        assert!(!assertions[0].runtime_enabled_after_readback);
        assert!(!assertions[0].performs_readback);
        assert!(!assertions[0].mutates_runtime);
    }

    #[test]
    fn wal_write_boundary_execution_readback_blockers_add_application_missing() {
        let readback_plans = vec![WorkGraphWalWriteBoundaryExecutionReadbackPlanPreview {
            id: "readback_plan".to_string(),
            source_surface_id: "sample",
            source_category: "planning",
            wal_write_boundary_execution_plan_id: "plan".to_string(),
            required_wal_write_boundary_execution_stage_ids: vec![
                "wal_write_boundary_execution_policy_contract",
            ],
            residual_source_blocker_ids: vec!["idempotency_index_mutation_disabled"],
            required_evidence_field_ids: vec!["source_surface_id"],
            readback_state: "readback_verified_from_wal_write_boundary_execution_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_rollback: false,
            mutates_runtime: false,
        }];
        let blockers =
            work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_blockers_from(
                &[],
                &readback_plans,
            );

        assert_eq!(blockers.len(), 1);
        assert_eq!(
            blockers[0].id,
            "wal_write_boundary_execution_application_missing"
        );
        assert!(blockers[0].blocks_wal_write_boundary_execution);
        assert_eq!(
            blockers[0].affected_readback_plan_ids,
            vec!["readback_plan"]
        );
    }

    fn sample_source_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphWalWriteBoundaryExecutionSourcePlanPreview {
        WorkGraphWalWriteBoundaryExecutionSourcePlanPreview {
            source_surface_id,
            source_category,
            wal_write_boundary_execution_plan_id:
                "append_only_store_runtime_wal_write_boundary_execution_sample_preview".to_string(),
            previous_enforcement_decision: "deny_runtime_wal_write_boundary_execution_disabled",
            wal_write_boundary_execution_state: "wal_write_boundary_execution_contract_defined_preview_only",
            required_wal_write_boundary_execution_stage_ids: vec![
                "wal_write_boundary_execution_policy_contract",
                "idempotency_collision_replay_evidence_contract",
                "idempotency_index_no_mutation_guard",
                "rollback_readback_prerequisite_contract",
                "idempotency_blocker_mapping",
            ],
            residual_source_blocker_ids: vec!["idempotency_index_mutation_disabled"],
            expected_evidence_field_ids: vec!["source_surface_id"],
            wal_write_boundary_execution_policy_contract_ready_preview: true,
            collision_replay_evidence_contract_ready_preview: true,
            applies_to_runtime: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_runtime: false,
        }
    }
}
