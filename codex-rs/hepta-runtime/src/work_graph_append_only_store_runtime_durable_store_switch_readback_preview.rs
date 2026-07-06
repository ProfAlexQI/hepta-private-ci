use serde::Serialize;

use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::WorkGraphDurableStoreSwitchBlockerPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::WorkGraphDurableStoreSwitchGuardPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::WorkGraphDurableStoreSwitchSourcePlanPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::WorkGraphDurableStoreSwitchStagePlanPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::work_graph_append_only_store_runtime_durable_store_switch_blockers;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::work_graph_append_only_store_runtime_durable_store_switch_guards;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::work_graph_append_only_store_runtime_durable_store_switch_plans;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::work_graph_append_only_store_runtime_durable_store_switch_required_prior_gates;
use crate::work_graph_append_only_store_runtime_durable_store_switch_preview::work_graph_append_only_store_runtime_durable_store_switch_stage_plans;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_durable_store_switch_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_durable_store_switch_preview_gate: &'static str,
    pub source_surface_count: usize,
    pub durable_store_switch_plan_count: usize,
    pub readback_plan_count: usize,
    pub stage_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub durable_store_switch_stage_source_ref_count: usize,
    pub durable_store_switch_stage_contract_ref_count: usize,
    pub durable_store_switch_plan_stage_ref_count: usize,
    pub durable_store_switch_plan_evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub blocker_mapping_stage_ref_count: usize,
    pub readback_plans: Vec<WorkGraphDurableStoreSwitchReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphDurableStoreSwitchStageReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphDurableStoreSwitchReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphDurableStoreSwitchReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_durable_store_switch_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_durable_store_switch: bool,
    pub ready_for_idempotency_mutation: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub durable_store_switch_plan_id: String,
    pub required_durable_store_switch_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub required_before_application: bool,
    pub performs_readback: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchStageReadbackAssertionPreview {
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
    pub declared_executes_replay: bool,
    pub declared_executes_readback: bool,
    pub declared_executes_rollback: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_durable_store_switch: bool,
    pub satisfied_by_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_durable_store_switch_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocks_durable_store_switch: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_application_preview: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_durable_store_switch_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocks_durable_store_switch: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewSideEffects {
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

pub fn hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewReport {
    let source_plans = work_graph_append_only_store_runtime_durable_store_switch_plans();
    let stage_plans = work_graph_append_only_store_runtime_durable_store_switch_stage_plans();
    let guards = work_graph_append_only_store_runtime_durable_store_switch_guards();
    let preview_blockers = work_graph_append_only_store_runtime_durable_store_switch_blockers();
    let readback_plans =
        work_graph_append_only_store_runtime_durable_store_switch_readback_plans_from(
            &source_plans,
        );
    let stage_assertions =
        work_graph_append_only_store_runtime_durable_store_switch_stage_readback_assertions_from(
            &stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_store_runtime_durable_store_switch_evidence_field_readback_assertions_from(
            &readback_plans,
        );
    let guard_assertions =
        work_graph_append_only_store_runtime_durable_store_switch_guard_readback_assertions_from(
            &guards,
        );
    let blockers = work_graph_append_only_store_runtime_durable_store_switch_readback_blockers_from(
        &preview_blockers,
        &readback_plans,
    );
    let blocker_mapping_assertions =
        work_graph_append_only_store_runtime_durable_store_switch_blocker_mapping_assertions_from(
            &blockers,
        );
    let drift_detectors =
        work_graph_append_only_store_runtime_durable_store_switch_readback_drift_detectors();
    let required_prior_gates =
        work_graph_append_only_store_runtime_durable_store_switch_readback_required_prior_gates();

    WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_durable_store_switch_readback_no_execution",
        upstream_durable_store_switch_preview_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_PREVIEW_GATE,
        source_surface_count: source_plans.len(),
        durable_store_switch_plan_count: source_plans.len(),
        readback_plan_count: readback_plans.len(),
        stage_assertion_count: stage_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        durable_store_switch_stage_source_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        durable_store_switch_stage_contract_ref_count: stage_assertions
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        durable_store_switch_plan_stage_ref_count: readback_plans
            .iter()
            .map(|plan| plan.required_durable_store_switch_stage_ids.len())
            .sum(),
        durable_store_switch_plan_evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        blocker_mapping_source_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_stage_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_durable_store_switch_stage_ids.len())
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
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_durable_store_switch_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_replay_execution: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_durable_store_switch: false,
        ready_for_idempotency_mutation: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_readback_plans()
-> Vec<WorkGraphDurableStoreSwitchReadbackPlanPreview> {
    let source_plans = work_graph_append_only_store_runtime_durable_store_switch_plans();
    work_graph_append_only_store_runtime_durable_store_switch_readback_plans_from(&source_plans)
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_stage_readback_assertions()
-> Vec<WorkGraphDurableStoreSwitchStageReadbackAssertionPreview> {
    let stage_plans = work_graph_append_only_store_runtime_durable_store_switch_stage_plans();
    work_graph_append_only_store_runtime_durable_store_switch_stage_readback_assertions_from(
        &stage_plans,
    )
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_evidence_field_readback_assertions()
-> Vec<WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview> {
    let readback_plans = work_graph_append_only_store_runtime_durable_store_switch_readback_plans();
    work_graph_append_only_store_runtime_durable_store_switch_evidence_field_readback_assertions_from(
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_guard_readback_assertions()
-> Vec<WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview> {
    let guards = work_graph_append_only_store_runtime_durable_store_switch_guards();
    work_graph_append_only_store_runtime_durable_store_switch_guard_readback_assertions_from(
        &guards,
    )
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_readback_blockers()
-> Vec<WorkGraphDurableStoreSwitchReadbackBlockerPreview> {
    let preview_blockers = work_graph_append_only_store_runtime_durable_store_switch_blockers();
    let readback_plans = work_graph_append_only_store_runtime_durable_store_switch_readback_plans();
    work_graph_append_only_store_runtime_durable_store_switch_readback_blockers_from(
        &preview_blockers,
        &readback_plans,
    )
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_blocker_mapping_assertions()
-> Vec<WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview> {
    let blockers = work_graph_append_only_store_runtime_durable_store_switch_readback_blockers();
    work_graph_append_only_store_runtime_durable_store_switch_blocker_mapping_assertions_from(
        &blockers,
    )
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_readback_drift_detectors()
-> Vec<WorkGraphDurableStoreSwitchReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "durable_store_switch_plan_alignment",
            vec![
                "durable_store_switch_plan_id",
                "required_durable_store_switch_stage_ids",
            ],
        ),
        drift_detector(
            "durable_store_switch_stage_contract_alignment",
            vec!["stage_id", "required_contract_ref_ids"],
        ),
        drift_detector(
            "durable_store_switch_evidence_field_alignment",
            vec!["source_surface_id", "required_evidence_field_ids"],
        ),
        drift_detector(
            "durable_store_switch_guard_no_mutation_alignment",
            vec!["guard_id", "mutates_runtime"],
        ),
        drift_detector(
            "durable_store_switch_blocker_mapping_alignment",
            vec!["blocker_id", "affected_readback_plan_ids"],
        ),
        drift_detector(
            "durable_store_switch_side_effect_alignment",
            vec![
                "side_effects",
                "durable_store_switch_enabled",
                "runtime_mutation_performed",
            ],
        ),
        drift_detector(
            "durable_store_switch_upstream_gate_alignment",
            vec![
                "upstream_durable_store_switch_preview_gate",
                "recommended_next_gate",
            ],
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_durable_store_switch_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewSideEffects {
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

fn work_graph_append_only_store_runtime_durable_store_switch_readback_plans_from(
    source_plans: &[WorkGraphDurableStoreSwitchSourcePlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchReadbackPlanPreview> {
    source_plans
        .iter()
        .map(|plan| WorkGraphDurableStoreSwitchReadbackPlanPreview {
            id: durable_store_switch_readback_plan_id(plan.source_surface_id),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            durable_store_switch_plan_id: plan.durable_store_switch_plan_id.clone(),
            required_durable_store_switch_stage_ids: plan
                .required_durable_store_switch_stage_ids
                .clone(),
            residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
            required_evidence_field_ids: plan.expected_evidence_field_ids.clone(),
            readback_state: "readback_verified_from_durable_store_switch_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_runtime_durable_store_switch_stage_readback_assertions_from(
    stage_plans: &[WorkGraphDurableStoreSwitchStagePlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchStageReadbackAssertionPreview> {
    stage_plans
        .iter()
        .map(
            |stage| WorkGraphDurableStoreSwitchStageReadbackAssertionPreview {
                id: format!(
                    "durable_store_switch_stage_readback_assertion__{}",
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
                declared_executes_replay: stage.executes_replay,
                declared_executes_readback: stage.executes_readback,
                declared_executes_rollback: stage.executes_rollback,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_durable_store_switch_evidence_field_readback_assertions_from(
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview> {
    readback_plans
        .iter()
        .map(
            |plan| WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "durable_store_switch_evidence_field_readback_assertion__{}",
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

fn work_graph_append_only_store_runtime_durable_store_switch_guard_readback_assertions_from(
    guards: &[WorkGraphDurableStoreSwitchGuardPreview],
) -> Vec<WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview {
                id: format!(
                    "durable_store_switch_guard_readback_assertion__{}",
                    guard.id
                ),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
                required_before_durable_store_switch: guard.required_before_durable_store_switch,
                satisfied_by_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_store_runtime_durable_store_switch_readback_blockers_from(
    preview_blockers: &[WorkGraphDurableStoreSwitchBlockerPreview],
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchReadbackBlockerPreview> {
    let mut blockers = preview_blockers
        .iter()
        .map(|blocker| readback_blocker_from_preview(blocker, readback_plans))
        .collect::<Vec<_>>();
    let affected_source_surface_ids = readback_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let affected_durable_store_switch_stage_ids = readback_plans
        .first()
        .map(|plan| plan.required_durable_store_switch_stage_ids.clone())
        .unwrap_or_default();
    blockers.push(readback_blocker(
        "durable_store_switch_application_missing",
        "high",
        "application_preview",
        affected_source_surface_ids,
        affected_durable_store_switch_stage_ids,
        readback_plans,
        "apply readback-verified durable-store switch plans before any runtime store selector, WAL, replay, checkpoint, idempotency, or rollback promotion",
    ));
    blockers
}

fn work_graph_append_only_store_runtime_durable_store_switch_blocker_mapping_assertions_from(
    blockers: &[WorkGraphDurableStoreSwitchReadbackBlockerPreview],
) -> Vec<WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview {
                id: format!(
                    "durable_store_switch_blocker_mapping_readback_assertion__{}",
                    blocker.id
                ),
                blocker_id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_durable_store_switch_stage_ids: blocker
                    .affected_durable_store_switch_stage_ids
                    .clone(),
                affected_readback_plan_ids: blocker.affected_readback_plan_ids.clone(),
                expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
                blocks_durable_store_switch: blocker.blocks_durable_store_switch,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn readback_blocker_from_preview(
    blocker: &WorkGraphDurableStoreSwitchBlockerPreview,
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
) -> WorkGraphDurableStoreSwitchReadbackBlockerPreview {
    readback_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        blocker.affected_durable_store_switch_stage_ids.clone(),
        readback_plans,
        blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_durable_store_switch_stage_ids: Vec<&'static str>,
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
    recommended_fix: &'static str,
) -> WorkGraphDurableStoreSwitchReadbackBlockerPreview {
    WorkGraphDurableStoreSwitchReadbackBlockerPreview {
        affected_readback_plan_ids: affected_readback_plan_ids(
            &affected_source_surface_ids,
            readback_plans,
        ),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_durable_store_switch_stage_ids,
        blocks_durable_store_switch: true,
        recommended_fix,
    }
}

fn affected_readback_plan_ids(
    source_ids: &[&'static str],
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
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
) -> WorkGraphDurableStoreSwitchReadbackDriftDetectorPreview {
    WorkGraphDurableStoreSwitchReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity: "high",
        blocks_application_preview: true,
        performs_readback: false,
    }
}

fn durable_store_switch_readback_plan_id(source_surface_id: &str) -> String {
    format!("append_only_store_runtime_durable_store_switch_readback_plan__{source_surface_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_store_switch_readback_declares_no_execution_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_gate"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchReadbackPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.durable_store_switch_enabled);
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
    fn durable_store_switch_readback_plans_preserve_preview_sources() {
        let plans =
            work_graph_append_only_store_runtime_durable_store_switch_readback_plans_from(&[
                sample_source_plan("update_plan_tool", "planning"),
                sample_source_plan("hepta_runtime_scheduler_store", "runtime_scheduler"),
            ]);

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_state
                == "readback_verified_from_durable_store_switch_preview_no_execution"
                && plan.required_before_application
                && !plan.performs_readback
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.switches_durable_store
                && !plan.mutates_idempotency_index
                && !plan.executes_replay
                && !plan.executes_rollback
                && !plan.mutates_runtime
                && plan.required_durable_store_switch_stage_ids.len() == 5
                && plan.required_evidence_field_ids.len() == 9
        }));
    }

    #[test]
    fn durable_store_switch_stage_readback_assertions_keep_runtime_disabled() {
        let assertions =
            work_graph_append_only_store_runtime_durable_store_switch_stage_readback_assertions_from(
                &[
                    sample_stage_plan(
                        "runtime_durable_store_switch_contract",
                        "durable_store_switch",
                        true,
                        false,
                        false,
                    ),
                    sample_stage_plan(
                        "wal_replay_prerequisite_contract",
                        "wal_replay_prerequisite",
                        false,
                        true,
                        false,
                    ),
                ],
            );

        assert_eq!(assertions.len(), 2);
        assert_eq!(
            assertions[0].expected_runtime_state,
            "readback_verified_contract_ready_runtime_disabled"
        );
        assert!(assertions[0].declared_switches_durable_store);
        assert!(!assertions[0].runtime_enabled_after_readback);
        assert!(!assertions[0].performs_readback);
        assert!(!assertions[0].mutates_runtime);
        assert!(assertions[1].declared_writes_wal);
        assert!(assertions[1].declared_executes_replay);
    }

    #[test]
    fn durable_store_switch_readback_blockers_map_to_readback_plans() {
        let readback_plans =
            work_graph_append_only_store_runtime_durable_store_switch_readback_plans_from(&[
                sample_source_plan("update_plan_tool", "planning"),
                sample_source_plan("hepta_runtime_scheduler_store", "runtime_scheduler"),
            ]);
        let blockers =
            work_graph_append_only_store_runtime_durable_store_switch_readback_blockers_from(
                &[sample_blocker()],
                &readback_plans,
            );

        assert_eq!(blockers.len(), 2);
        assert_eq!(blockers[0].affected_readback_plan_ids.len(), 1);
        assert_eq!(
            blockers[0].affected_readback_plan_ids[0],
            "append_only_store_runtime_durable_store_switch_readback_plan__update_plan_tool"
        );
        assert_eq!(blockers[1].id, "durable_store_switch_application_missing");
        assert_eq!(blockers[1].affected_readback_plan_ids.len(), 2);
    }

    fn sample_source_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphDurableStoreSwitchSourcePlanPreview {
        WorkGraphDurableStoreSwitchSourcePlanPreview {
            source_surface_id,
            source_category,
            durable_store_switch_plan_id: format!(
                "append_only_store_runtime_durable_store_switch_{source_surface_id}_preview"
            ),
            previous_enforcement_decision: "deny_runtime_durable_store_switch_disabled",
            durable_store_switch_state: "durable_store_switch_contract_defined_preview_only",
            required_durable_store_switch_stage_ids: vec![
                "runtime_durable_store_switch_contract",
                "wal_replay_prerequisite_contract",
                "operator_review_rollback_guard",
                "durable_store_switch_no_mutation_guard",
                "durable_store_switch_blocker_mapping",
            ],
            residual_source_blocker_ids: vec![
                "wal_write_boundary_not_enabled",
                "durable_store_runtime_switch_disabled",
            ],
            expected_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "runtime_write_boundary_rerun_decision_ref",
                "durable_store_switch_contract_id",
                "wal_replay_prerequisite_id",
                "operator_review_rollback_guard_id",
                "no_mutation_guard_ref",
                "residual_source_blocker_ids",
                "next_required_gate",
            ],
            durable_store_switch_contract_ready_preview: true,
            applies_to_runtime: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_runtime: false,
        }
    }

    fn sample_stage_plan(
        id: &'static str,
        category: &'static str,
        switches_durable_store: bool,
        writes_wal: bool,
        executes_readback: bool,
    ) -> WorkGraphDurableStoreSwitchStagePlanPreview {
        WorkGraphDurableStoreSwitchStagePlanPreview {
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
            switches_durable_store,
            mutates_idempotency_index: false,
            executes_replay: writes_wal,
            executes_readback,
            executes_rollback: executes_readback,
        }
    }

    fn sample_blocker() -> WorkGraphDurableStoreSwitchBlockerPreview {
        WorkGraphDurableStoreSwitchBlockerPreview {
            id: "durable_store_runtime_switch_disabled",
            severity: "critical",
            category: "durable_store_switch",
            affected_source_surface_ids: vec!["update_plan_tool"],
            affected_durable_store_switch_stage_ids: vec!["runtime_durable_store_switch_contract"],
            affected_durable_store_switch_plan_ids: vec![
                "append_only_store_runtime_durable_store_switch_update_plan_tool_preview"
                    .to_string(),
            ],
            required_before_durable_store_switch: true,
            recommended_fix: "keep durable-store switch disabled until promotion",
        }
    }
}
