use serde::Serialize;

use crate::work_graph_append_only_store_runtime_write_boundary_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::WorkGraphRuntimeWriteBoundaryBlockerPreview;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::WorkGraphRuntimeWriteBoundaryGuardPreview;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::WorkGraphRuntimeWriteBoundarySourcePlanPreview;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::WorkGraphRuntimeWriteBoundaryStagePlanPreview;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::work_graph_append_only_store_runtime_write_boundary_blockers;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::work_graph_append_only_store_runtime_write_boundary_guards;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::work_graph_append_only_store_runtime_write_boundary_plans;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::work_graph_append_only_store_runtime_write_boundary_required_prior_gates;
use crate::work_graph_append_only_store_runtime_write_boundary_preview::work_graph_append_only_store_runtime_write_boundary_stage_plans;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_write_boundary_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_write_boundary_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_runtime_write_boundary_preview_gate: &'static str,
    pub source_surface_count: usize,
    pub runtime_write_boundary_plan_count: usize,
    pub readback_plan_count: usize,
    pub stage_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub runtime_write_boundary_stage_source_ref_count: usize,
    pub runtime_write_boundary_stage_contract_ref_count: usize,
    pub runtime_write_boundary_plan_stage_ref_count: usize,
    pub runtime_write_boundary_plan_evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub blocker_mapping_stage_ref_count: usize,
    pub readback_plans: Vec<WorkGraphRuntimeWriteBoundaryReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphRuntimeWriteBoundaryStageReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphRuntimeWriteBoundaryEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphRuntimeWriteBoundaryGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphRuntimeWriteBoundaryBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphRuntimeWriteBoundaryReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_write_boundary_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_durable_store_switch: bool,
    pub ready_for_idempotency_mutation: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_write_boundary_plan_id: String,
    pub required_write_boundary_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub required_before_application: bool,
    pub performs_readback: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryStageReadbackAssertionPreview {
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
    pub declared_switches_durable_store: bool,
    pub declared_mutates_idempotency_index: bool,
    pub declared_executes_readback: bool,
    pub declared_executes_rollback: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_runtime_write_boundary: bool,
    pub satisfied_by_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryBlockerMappingReadbackAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_write_boundary_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocks_runtime_write_boundary: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_application_preview: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_write_boundary_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocks_runtime_write_boundary: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewSideEffects {
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
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_write_boundary_readback_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewReport {
    let source_plans = work_graph_append_only_store_runtime_write_boundary_plans();
    let stage_plans = work_graph_append_only_store_runtime_write_boundary_stage_plans();
    let guards = work_graph_append_only_store_runtime_write_boundary_guards();
    let preview_blockers = work_graph_append_only_store_runtime_write_boundary_blockers();
    let readback_plans =
        work_graph_append_only_store_runtime_write_boundary_readback_plans_from(&source_plans);
    let stage_assertions =
        work_graph_append_only_store_runtime_write_boundary_stage_readback_assertions_from(
            &stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_store_runtime_write_boundary_evidence_field_readback_assertions_from(
            &readback_plans,
        );
    let guard_assertions =
        work_graph_append_only_store_runtime_write_boundary_guard_readback_assertions_from(&guards);
    let blockers = work_graph_append_only_store_runtime_write_boundary_readback_blockers_from(
        &preview_blockers,
        &readback_plans,
    );
    let blocker_mapping_assertions =
        work_graph_append_only_store_runtime_write_boundary_blocker_mapping_assertions_from(
            &blockers,
        );
    let drift_detectors =
        work_graph_append_only_store_runtime_write_boundary_readback_drift_detectors();
    let required_prior_gates =
        work_graph_append_only_store_runtime_write_boundary_readback_required_prior_gates();

    WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_write_boundary_readback_no_execution",
        upstream_runtime_write_boundary_preview_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE,
        source_surface_count: source_plans.len(),
        runtime_write_boundary_plan_count: source_plans.len(),
        readback_plan_count: readback_plans.len(),
        stage_assertion_count: stage_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        runtime_write_boundary_stage_source_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        runtime_write_boundary_stage_contract_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        runtime_write_boundary_plan_stage_ref_count: readback_plans
            .iter()
            .map(|plan| plan.required_write_boundary_stage_ids.len())
            .sum(),
        runtime_write_boundary_plan_evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        blocker_mapping_source_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_stage_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_runtime_write_boundary_stage_ids.len())
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
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_write_boundary_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_durable_store_switch: false,
        ready_for_idempotency_mutation: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewSideEffects::none(
        ),
    }
}

pub fn work_graph_append_only_store_runtime_write_boundary_readback_plans()
-> Vec<WorkGraphRuntimeWriteBoundaryReadbackPlanPreview> {
    let source_plans = work_graph_append_only_store_runtime_write_boundary_plans();
    work_graph_append_only_store_runtime_write_boundary_readback_plans_from(&source_plans)
}

pub fn work_graph_append_only_store_runtime_write_boundary_stage_readback_assertions()
-> Vec<WorkGraphRuntimeWriteBoundaryStageReadbackAssertionPreview> {
    let stage_plans = work_graph_append_only_store_runtime_write_boundary_stage_plans();
    work_graph_append_only_store_runtime_write_boundary_stage_readback_assertions_from(&stage_plans)
}

pub fn work_graph_append_only_store_runtime_write_boundary_evidence_field_readback_assertions()
-> Vec<WorkGraphRuntimeWriteBoundaryEvidenceFieldReadbackAssertionPreview> {
    let readback_plans = work_graph_append_only_store_runtime_write_boundary_readback_plans();
    work_graph_append_only_store_runtime_write_boundary_evidence_field_readback_assertions_from(
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_write_boundary_guard_readback_assertions()
-> Vec<WorkGraphRuntimeWriteBoundaryGuardReadbackAssertionPreview> {
    let guards = work_graph_append_only_store_runtime_write_boundary_guards();
    work_graph_append_only_store_runtime_write_boundary_guard_readback_assertions_from(&guards)
}

pub fn work_graph_append_only_store_runtime_write_boundary_readback_blockers()
-> Vec<WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview> {
    let preview_blockers = work_graph_append_only_store_runtime_write_boundary_blockers();
    let readback_plans = work_graph_append_only_store_runtime_write_boundary_readback_plans();
    work_graph_append_only_store_runtime_write_boundary_readback_blockers_from(
        &preview_blockers,
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_write_boundary_blocker_mapping_assertions()
-> Vec<WorkGraphRuntimeWriteBoundaryBlockerMappingReadbackAssertionPreview> {
    let blockers = work_graph_append_only_store_runtime_write_boundary_readback_blockers();
    work_graph_append_only_store_runtime_write_boundary_blocker_mapping_assertions_from(&blockers)
}

pub fn work_graph_append_only_store_runtime_write_boundary_readback_drift_detectors()
-> Vec<WorkGraphRuntimeWriteBoundaryReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "runtime_write_boundary_plan_alignment",
            vec![
                "runtime_write_boundary_plan_id",
                "required_write_boundary_stage_ids",
            ],
        ),
        drift_detector(
            "runtime_write_boundary_stage_contract_alignment",
            vec!["stage_id", "required_contract_ref_ids"],
        ),
        drift_detector(
            "runtime_write_boundary_evidence_field_alignment",
            vec!["source_surface_id", "required_evidence_field_ids"],
        ),
        drift_detector(
            "runtime_write_boundary_guard_no_mutation_alignment",
            vec!["guard_id", "mutates_runtime"],
        ),
        drift_detector(
            "runtime_write_boundary_blocker_mapping_alignment",
            vec!["blocker_id", "affected_readback_plan_ids"],
        ),
        drift_detector(
            "runtime_write_boundary_side_effect_alignment",
            vec!["side_effects", "wal_written", "runtime_mutation_performed"],
        ),
        drift_detector(
            "runtime_write_boundary_upstream_gate_alignment",
            vec![
                "upstream_runtime_write_boundary_preview_gate",
                "recommended_next_gate",
            ],
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_write_boundary_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_store_runtime_write_boundary_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewSideEffects {
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
            rollback_executed: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn work_graph_append_only_store_runtime_write_boundary_readback_plans_from(
    source_plans: &[WorkGraphRuntimeWriteBoundarySourcePlanPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryReadbackPlanPreview> {
    source_plans
        .iter()
        .map(|plan| WorkGraphRuntimeWriteBoundaryReadbackPlanPreview {
            id: runtime_write_boundary_readback_plan_id(plan.source_surface_id),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            runtime_write_boundary_plan_id: plan.runtime_write_boundary_plan_id.clone(),
            required_write_boundary_stage_ids: plan.required_write_boundary_stage_ids.clone(),
            residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
            required_evidence_field_ids: plan.expected_evidence_field_ids.clone(),
            readback_state: "readback_verified_from_runtime_write_boundary_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_runtime_write_boundary_stage_readback_assertions_from(
    stage_plans: &[WorkGraphRuntimeWriteBoundaryStagePlanPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryStageReadbackAssertionPreview> {
    stage_plans
        .iter()
        .map(
            |stage| WorkGraphRuntimeWriteBoundaryStageReadbackAssertionPreview {
                id: format!(
                    "runtime_write_boundary_stage_readback_assertion__{}",
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
                declared_switches_durable_store: stage.switches_durable_store,
                declared_mutates_idempotency_index: stage.mutates_idempotency_index,
                declared_executes_readback: stage.executes_readback,
                declared_executes_rollback: stage.executes_rollback,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_write_boundary_evidence_field_readback_assertions_from(
    readback_plans: &[WorkGraphRuntimeWriteBoundaryReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryEvidenceFieldReadbackAssertionPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphRuntimeWriteBoundaryEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "runtime_write_boundary_evidence_field_readback_assertion__{}",
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

fn work_graph_append_only_store_runtime_write_boundary_guard_readback_assertions_from(
    guards: &[WorkGraphRuntimeWriteBoundaryGuardPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryGuardReadbackAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphRuntimeWriteBoundaryGuardReadbackAssertionPreview {
                id: format!(
                    "runtime_write_boundary_guard_readback_assertion__{}",
                    guard.id
                ),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
                required_before_runtime_write_boundary: guard
                    .required_before_runtime_write_boundary,
                satisfied_by_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_write_boundary_readback_blockers_from(
    preview_blockers: &[WorkGraphRuntimeWriteBoundaryBlockerPreview],
    readback_plans: &[WorkGraphRuntimeWriteBoundaryReadbackPlanPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview> {
    let mut blockers = preview_blockers
        .iter()
        .map(|blocker| readback_blocker_from_preview(blocker, readback_plans))
        .collect::<Vec<_>>();
    let affected_source_surface_ids = readback_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let affected_runtime_write_boundary_stage_ids = readback_plans
        .first()
        .map(|plan| plan.required_write_boundary_stage_ids.clone())
        .unwrap_or_default();
    blockers.push(readback_blocker(
        "runtime_write_boundary_application_missing",
        "high",
        "application_preview",
        affected_source_surface_ids,
        affected_runtime_write_boundary_stage_ids,
        readback_plans,
        "apply readback-verified runtime write-boundary plans before WAL, store, checkpoint, idempotency, or rollback execution can be promoted",
    ));
    blockers
}

fn work_graph_append_only_store_runtime_write_boundary_blocker_mapping_assertions_from(
    blockers: &[WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview],
) -> Vec<WorkGraphRuntimeWriteBoundaryBlockerMappingReadbackAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphRuntimeWriteBoundaryBlockerMappingReadbackAssertionPreview {
                id: format!(
                    "runtime_write_boundary_blocker_mapping_readback_assertion__{}",
                    blocker.id
                ),
                blocker_id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_runtime_write_boundary_stage_ids: blocker
                    .affected_runtime_write_boundary_stage_ids
                    .clone(),
                affected_readback_plan_ids: blocker.affected_readback_plan_ids.clone(),
                expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
                blocks_runtime_write_boundary: blocker.blocks_runtime_write_boundary,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn readback_blocker_from_preview(
    blocker: &WorkGraphRuntimeWriteBoundaryBlockerPreview,
    readback_plans: &[WorkGraphRuntimeWriteBoundaryReadbackPlanPreview],
) -> WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview {
    readback_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        blocker.affected_runtime_write_boundary_stage_ids.clone(),
        readback_plans,
        blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_runtime_write_boundary_stage_ids: Vec<&'static str>,
    readback_plans: &[WorkGraphRuntimeWriteBoundaryReadbackPlanPreview],
    recommended_fix: &'static str,
) -> WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview {
    WorkGraphRuntimeWriteBoundaryReadbackBlockerPreview {
        affected_readback_plan_ids: affected_readback_plan_ids(
            &affected_source_surface_ids,
            readback_plans,
        ),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_runtime_write_boundary_stage_ids,
        blocks_runtime_write_boundary: true,
        recommended_fix,
    }
}

fn affected_readback_plan_ids(
    source_ids: &[&'static str],
    readback_plans: &[WorkGraphRuntimeWriteBoundaryReadbackPlanPreview],
) -> Vec<String> {
    readback_plans
        .iter()
        .filter(|plan| source_ids.contains(&plan.source_surface_id))
        .map(|plan| plan.id.clone())
        .collect()
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
) -> WorkGraphRuntimeWriteBoundaryReadbackDriftDetectorPreview {
    WorkGraphRuntimeWriteBoundaryReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity: "high",
        blocks_application_preview: true,
        performs_readback: false,
    }
}

fn runtime_write_boundary_readback_plan_id(source_surface_id: &str) -> String {
    format!("append_only_store_runtime_write_boundary_readback_plan__{source_surface_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_write_boundary_readback_declares_no_execution_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_write_boundary_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeWriteBoundaryReadbackPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.durable_store_switch_enabled);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
        assert!(!side_effects.agent_spawn_performed);
    }

    #[test]
    fn runtime_write_boundary_readback_plans_preserve_preview_sources() {
        let plans = work_graph_append_only_store_runtime_write_boundary_readback_plans_from(&[
            sample_source_plan("update_plan_tool", "planning"),
            sample_source_plan("hepta_runtime_scheduler_store", "runtime_scheduler"),
        ]);

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_state
                == "readback_verified_from_runtime_write_boundary_preview_no_execution"
                && plan.required_before_application
                && !plan.performs_readback
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.switches_durable_store
                && !plan.mutates_idempotency_index
                && !plan.executes_rollback
                && !plan.mutates_runtime
                && plan.required_write_boundary_stage_ids.len() == 5
                && plan.required_evidence_field_ids.len() == 9
        }));
    }

    #[test]
    fn runtime_write_boundary_stage_readback_assertions_keep_runtime_disabled() {
        let assertions =
            work_graph_append_only_store_runtime_write_boundary_stage_readback_assertions_from(&[
                sample_stage_plan("wal_write_boundary", "wal_boundary", true, false),
                sample_stage_plan("no_mutation_guard", "preview_no_mutation", false, false),
            ]);

        assert_eq!(assertions.len(), 2);
        assert_eq!(
            assertions[0].expected_runtime_state,
            "readback_verified_contract_ready_runtime_disabled"
        );
        assert!(assertions[0].declared_writes_wal);
        assert!(!assertions[0].runtime_enabled_after_readback);
        assert!(!assertions[0].performs_readback);
        assert!(!assertions[0].mutates_runtime);
        assert!(!assertions[1].declared_writes_wal);
    }

    #[test]
    fn runtime_write_boundary_readback_blockers_map_to_readback_plans() {
        let readback_plans =
            work_graph_append_only_store_runtime_write_boundary_readback_plans_from(&[
                sample_source_plan("update_plan_tool", "planning"),
                sample_source_plan("hepta_runtime_scheduler_store", "runtime_scheduler"),
            ]);
        let blockers = work_graph_append_only_store_runtime_write_boundary_readback_blockers_from(
            &[sample_blocker()],
            &readback_plans,
        );

        assert_eq!(blockers.len(), 2);
        assert_eq!(blockers[0].affected_readback_plan_ids.len(), 1);
        assert_eq!(
            blockers[0].affected_readback_plan_ids[0],
            "append_only_store_runtime_write_boundary_readback_plan__update_plan_tool"
        );
        assert_eq!(blockers[1].id, "runtime_write_boundary_application_missing");
        assert_eq!(blockers[1].affected_readback_plan_ids.len(), 2);
    }

    fn sample_source_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphRuntimeWriteBoundarySourcePlanPreview {
        WorkGraphRuntimeWriteBoundarySourcePlanPreview {
            source_surface_id,
            source_category,
            runtime_write_boundary_plan_id: format!(
                "append_only_store_runtime_write_boundary_{source_surface_id}_preview"
            ),
            previous_enforcement_decision: "deny_runtime_append_only_store_write_boundary_disabled",
            write_boundary_state: "runtime_write_boundary_contract_defined_preview_only",
            required_write_boundary_stage_ids: vec![
                "wal_write_boundary",
                "durable_store_runtime_switch_guard",
                "idempotency_mutation_policy",
                "rollback_readback_execution_boundary",
                "no_mutation_guard",
            ],
            residual_source_blocker_ids: vec![
                "wal_write_boundary_not_enabled",
                "rollback_readback_not_executed",
            ],
            expected_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "operator_review_side_effect_lock_rerun_decision_ref",
                "wal_write_boundary_contract_id",
                "durable_store_switch_guard_id",
                "idempotency_mutation_policy_id",
                "rollback_readback_boundary_id",
                "residual_source_blocker_ids",
                "no_mutation_guard_ref",
            ],
            write_boundary_contract_ready_preview: true,
            applies_to_runtime: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_readback: false,
            executes_rollback: false,
            records_approval: false,
            mutates_runtime: false,
        }
    }

    fn sample_stage_plan(
        id: &'static str,
        category: &'static str,
        writes_wal: bool,
        executes_readback: bool,
    ) -> WorkGraphRuntimeWriteBoundaryStagePlanPreview {
        WorkGraphRuntimeWriteBoundaryStagePlanPreview {
            id,
            priority: "p0",
            category,
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["contract_ready"],
            expected_runtime_state: "contract_ready_preview_runtime_disabled",
            prerequisite_gate_ids: vec!["prior_gate"],
            contract_ready_preview: true,
            runtime_enabled_after_preview: false,
            writes_wal,
            writes_checkpoint: writes_wal || executes_readback,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_readback,
            executes_rollback: executes_readback,
        }
    }

    fn sample_blocker() -> WorkGraphRuntimeWriteBoundaryBlockerPreview {
        WorkGraphRuntimeWriteBoundaryBlockerPreview {
            id: "wal_write_boundary_not_enabled",
            severity: "critical",
            category: "wal_boundary",
            affected_source_surface_ids: vec!["update_plan_tool"],
            affected_runtime_write_boundary_stage_ids: vec!["wal_write_boundary"],
            affected_runtime_write_boundary_plan_ids: vec![
                "append_only_store_runtime_write_boundary_update_plan_tool_preview".to_string(),
            ],
            required_before_runtime_write_boundary: true,
            recommended_fix: "keep WAL writes disabled until write-boundary contracts are promoted",
        }
    }
}
