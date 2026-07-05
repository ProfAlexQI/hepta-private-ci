use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_residual_blockers;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview::work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate";

const WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS: [&str; 5] = [
    "wal_append_contract",
    "wal_replay_readback_prerequisite",
    "durable_store_switch_guard",
    "wal_no_write_guard",
    "wal_blocker_mapping",
];

const WAL_WRITE_BOUNDARY_EXECUTION_EVIDENCE_FIELDS: [&str; 9] = [
    "source_surface_id",
    "source_category",
    "rollback_readback_execution_rerun_decision_ref",
    "wal_append_contract_id",
    "wal_replay_readback_prerequisite_id",
    "durable_store_switch_guard_ref",
    "wal_no_write_guard_ref",
    "residual_source_blocker_ids",
    "next_required_gate",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_runtime_rollback_readback_execution_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub wal_write_boundary_execution_source_count: usize,
    pub wal_write_boundary_execution_plan_count: usize,
    pub wal_write_boundary_execution_stage_count: usize,
    pub wal_write_boundary_execution_stage_source_ref_count: usize,
    pub wal_write_boundary_execution_stage_contract_ref_count: usize,
    pub wal_write_boundary_execution_plan_stage_ref_count: usize,
    pub wal_write_boundary_execution_plan_evidence_field_ref_count: usize,
    pub idempotency_residual_source_count: usize,
    pub wal_boundary_residual_source_count: usize,
    pub rollback_readback_residual_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub wal_write_boundary_execution_plans:
        Vec<WorkGraphWalWriteBoundaryExecutionSourcePlanPreview>,
    pub wal_write_boundary_execution_stage_plans:
        Vec<WorkGraphWalWriteBoundaryExecutionStagePlanPreview>,
    pub guards: Vec<WorkGraphWalWriteBoundaryExecutionGuardPreview>,
    pub blockers: Vec<WorkGraphWalWriteBoundaryExecutionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_wal_write_boundary_execution_readback_preview: bool,
    pub ready_for_runtime_wal_write_boundary_execution_application_preview: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_wal_write_boundary_execution: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionSourcePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub wal_write_boundary_execution_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub wal_write_boundary_execution_state: &'static str,
    pub required_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub wal_write_boundary_execution_policy_contract_ready_preview: bool,
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
pub struct WorkGraphWalWriteBoundaryExecutionStagePlanPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub prerequisite_gate_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub runtime_enabled_after_preview: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_wal_write_boundary_execution: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWalWriteBoundaryExecutionBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    pub affected_wal_write_boundary_execution_plan_ids: Vec<String>,
    pub required_before_wal_write_boundary_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub lane_lease_acquired: bool,
    pub work_started: bool,
    pub budget_consumed: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub tool_permission_changed: bool,
    pub role_budget_consumed: bool,
    pub role_lane_binding_mutated: bool,
    pub readback_executed: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub runtime_wrapper_attached: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_source_decisions();
    let wal_write_boundary_execution_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_plans_from(
            &source_decisions,
        );
    let wal_write_boundary_execution_stage_plans =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_plans();
    let guards = work_graph_append_only_store_runtime_wal_write_boundary_execution_guards();
    let blockers = work_graph_append_only_store_runtime_wal_write_boundary_execution_blockers();
    let required_prior_gates =
        work_graph_append_only_store_runtime_wal_write_boundary_execution_required_prior_gates();
    let idempotency_residual_source_count =
        sources_for_residual_blocker("idempotency_index_mutation_disabled").len();
    let wal_boundary_residual_source_count =
        sources_for_residual_blocker("wal_write_boundary_not_enabled").len();
    let rollback_readback_residual_source_count = residual_union_source_count(&[
        "readback_execution_disabled",
        "rollback_readback_not_executed",
    ]);

    WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_runtime_wal_write_boundary_execution_preview_no_index_mutation",
        upstream_runtime_rollback_readback_execution_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        wal_write_boundary_execution_source_count: wal_write_boundary_execution_plans.len(),
        wal_write_boundary_execution_plan_count: wal_write_boundary_execution_plans.len(),
        wal_write_boundary_execution_stage_count: wal_write_boundary_execution_stage_plans.len(),
        wal_write_boundary_execution_stage_source_ref_count: wal_write_boundary_execution_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        wal_write_boundary_execution_stage_contract_ref_count: wal_write_boundary_execution_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        wal_write_boundary_execution_plan_stage_ref_count: wal_write_boundary_execution_plans
            .iter()
            .map(|plan| plan.required_wal_write_boundary_execution_stage_ids.len())
            .sum(),
        wal_write_boundary_execution_plan_evidence_field_ref_count: wal_write_boundary_execution_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        idempotency_residual_source_count,
        wal_boundary_residual_source_count,
        rollback_readback_residual_source_count,
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        wal_write_boundary_execution_plans,
        wal_write_boundary_execution_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_wal_write_boundary_execution_readback_preview: true,
        ready_for_runtime_wal_write_boundary_execution_application_preview: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_wal_write_boundary_execution: false,
        ready_for_readback_execution: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_plans()
-> Vec<WorkGraphWalWriteBoundaryExecutionSourcePlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_source_decisions();
    work_graph_append_only_store_runtime_wal_write_boundary_execution_plans_from(&source_decisions)
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_stage_plans()
-> Vec<WorkGraphWalWriteBoundaryExecutionStagePlanPreview> {
    let source_ids = wal_write_boundary_execution_source_ids();
    vec![
        wal_write_boundary_execution_stage_plan(
            "wal_append_contract",
            "p0",
            "wal_append_contract",
            source_ids.clone(),
            vec![
                "wal_append_record_contract_ready",
                "wal_ordering_contract_ready",
                "wal_redaction_contract_ready",
                "wal_idempotency_key_contract_ready",
                "wal_checkpoint_reference_contract_ready",
                "wal_operator_gate_contract_ready",
            ],
            WalWriteBoundaryExecutionStageEffect::WalAppend,
        ),
        wal_write_boundary_execution_stage_plan(
            "wal_replay_readback_prerequisite",
            "p0",
            "replay_readback_prerequisite",
            source_ids.clone(),
            vec![
                "replay_cursor_contract_ready",
                "readback_probe_contract_ready",
                "rollback_anchor_contract_ready",
                "checkpoint_observation_contract_ready",
                "duplicate_suppression_readback_contract_ready",
                "wal_evidence_hash_contract_ready",
            ],
            WalWriteBoundaryExecutionStageEffect::ReplayReadback,
        ),
        wal_write_boundary_execution_stage_plan(
            "durable_store_switch_guard",
            "p0",
            "durable_store_switch_guard",
            source_ids.clone(),
            vec![
                "durable_store_switch_contract_ready",
                "operator_review_contract_ready",
                "side_effect_lock_contract_ready",
                "rollback_execution_contract_ready",
                "runtime_application_promotion_contract_ready",
                "append_only_store_precondition_contract_ready",
            ],
            WalWriteBoundaryExecutionStageEffect::DurableStoreGuard,
        ),
        wal_write_boundary_execution_stage_plan(
            "wal_no_write_guard",
            "p0",
            "preview_no_write_guard",
            source_ids.clone(),
            vec![
                "filesystem_no_write_guard_ready",
                "graph_state_no_persist_guard_ready",
                "wal_no_write_guard_ready",
                "checkpoint_no_write_guard_ready",
                "runtime_no_mutation_guard_ready",
            ],
            WalWriteBoundaryExecutionStageEffect::NoMutation,
        ),
        wal_write_boundary_execution_stage_plan(
            "wal_blocker_mapping",
            "p0",
            "blocker_mapping",
            source_ids,
            vec![
                "wal_boundary_blocker_mapping_ready",
                "wal_append_blocker_mapping_ready",
                "rollback_readback_blocker_mapping_ready",
                "projection_enforcement_blocker_mapping_ready",
                "append_only_store_enablement_blocker_mapping_ready",
            ],
            WalWriteBoundaryExecutionStageEffect::NoMutation,
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_guards()
-> Vec<WorkGraphWalWriteBoundaryExecutionGuardPreview> {
    vec![
        guard(
            "wal_write_boundary_execution_preview_only",
            "medium",
            "preview_boundary",
        ),
        guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_index",
        ),
        guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        guard("checkpoint_write_disabled", "critical", "checkpoint"),
        guard("replay_execution_disabled", "critical", "replay"),
        guard(
            "wal_write_boundary_execution_disabled",
            "critical",
            "wal_boundary",
        ),
        guard(
            "durable_store_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        guard("runtime_mutation_disabled", "critical", "runtime_mutation"),
    ]
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_blockers()
-> Vec<WorkGraphWalWriteBoundaryExecutionBlockerPreview> {
    let mut blockers =
        work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_residual_blockers()
            .into_iter()
            .map(wal_write_boundary_execution_blocker_from_residual)
            .collect::<Vec<_>>();
    blockers.push(wal_write_boundary_execution_blocker(
        "wal_write_boundary_execution_readback_missing",
        "high",
        "readback_preview",
        wal_write_boundary_execution_source_ids(),
        WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS.to_vec(),
        "read back WAL write-boundary execution plans before any idempotency index mutation, WAL replay, rollback/readback, or projection enforcement promotion",
    ));
    blockers
}

pub fn work_graph_append_only_store_runtime_wal_write_boundary_execution_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewSideEffects {
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
            lane_lease_acquired: false,
            work_started: false,
            budget_consumed: false,
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            role_manifest_enforcement_enabled: false,
            tool_permission_changed: false,
            role_budget_consumed: false,
            role_lane_binding_mutated: false,
            readback_executed: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            runtime_wrapper_attached: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_append_only_store_runtime_wal_write_boundary_execution_plans_from(
    source_decisions: &[WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview],
) -> Vec<WorkGraphWalWriteBoundaryExecutionSourcePlanPreview> {
    source_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_rollback_readback_execution_rerun_enforcement_decision
                == "deny_runtime_wal_write_boundary_not_enabled"
        })
        .cloned()
        .map(wal_write_boundary_execution_plan)
        .collect()
}

fn wal_write_boundary_execution_plan(
    decision: WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview,
) -> WorkGraphWalWriteBoundaryExecutionSourcePlanPreview {
    WorkGraphWalWriteBoundaryExecutionSourcePlanPreview {
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        wal_write_boundary_execution_plan_id: wal_write_boundary_execution_plan_id(
            decision.source_surface_id,
        ),
        previous_enforcement_decision: decision
            .runtime_rollback_readback_execution_rerun_enforcement_decision,
        wal_write_boundary_execution_state: "wal_write_boundary_execution_contract_defined_preview_only",
        required_wal_write_boundary_execution_stage_ids: WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS
            .to_vec(),
        residual_source_blocker_ids: decision.residual_source_blocker_ids,
        expected_evidence_field_ids: WAL_WRITE_BOUNDARY_EXECUTION_EVIDENCE_FIELDS.to_vec(),
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

#[derive(Debug, Clone, Copy)]
enum WalWriteBoundaryExecutionStageEffect {
    WalAppend,
    ReplayReadback,
    DurableStoreGuard,
    NoMutation,
}

fn wal_write_boundary_execution_stage_plan(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
    effect: WalWriteBoundaryExecutionStageEffect,
) -> WorkGraphWalWriteBoundaryExecutionStagePlanPreview {
    WorkGraphWalWriteBoundaryExecutionStagePlanPreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "contract_ready_preview_runtime_disabled",
        prerequisite_gate_ids:
            work_graph_append_only_store_runtime_wal_write_boundary_execution_required_prior_gates(),
        contract_ready_preview: true,
        runtime_enabled_after_preview: false,
        writes_wal: matches!(effect, WalWriteBoundaryExecutionStageEffect::WalAppend),
        writes_checkpoint: matches!(
            effect,
            WalWriteBoundaryExecutionStageEffect::WalAppend
                | WalWriteBoundaryExecutionStageEffect::ReplayReadback
        ),
        mutates_idempotency_index: false,
        executes_replay: matches!(effect, WalWriteBoundaryExecutionStageEffect::ReplayReadback),
        executes_readback: matches!(effect, WalWriteBoundaryExecutionStageEffect::ReplayReadback),
        executes_rollback: matches!(effect, WalWriteBoundaryExecutionStageEffect::ReplayReadback),
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphWalWriteBoundaryExecutionGuardPreview {
    WorkGraphWalWriteBoundaryExecutionGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_wal_write_boundary_execution: true,
        satisfied_by_preview: false,
    }
}

fn wal_write_boundary_execution_blocker_from_residual(
    blocker: WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview,
) -> WorkGraphWalWriteBoundaryExecutionBlockerPreview {
    wal_write_boundary_execution_blocker(
        blocker.id,
        blocker.severity,
        category_for_blocker(blocker.id),
        blocker.affected_source_surface_ids,
        stage_ids_for_blocker(blocker.id),
        blocker.recommended_fix,
    )
}

fn wal_write_boundary_execution_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_wal_write_boundary_execution_stage_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphWalWriteBoundaryExecutionBlockerPreview {
    WorkGraphWalWriteBoundaryExecutionBlockerPreview {
        affected_wal_write_boundary_execution_plan_ids: affected_source_surface_ids
            .iter()
            .map(|source| wal_write_boundary_execution_plan_id(source))
            .collect(),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_wal_write_boundary_execution_stage_ids,
        required_before_wal_write_boundary_execution: true,
        recommended_fix,
    }
}

fn category_for_blocker(id: &str) -> &'static str {
    match id {
        "wal_write_boundary_not_enabled" => "wal_replay_prerequisite",
        "readback_execution_disabled" | "rollback_readback_not_executed" => "rollback_readback",
        "idempotency_index_mutation_disabled" => "idempotency_policy",
        _ => "wal_boundary",
    }
}

fn stage_ids_for_blocker(id: &str) -> Vec<&'static str> {
    match id {
        "wal_write_boundary_not_enabled" => {
            vec![
                "wal_append_contract",
                "wal_replay_readback_prerequisite",
                "wal_no_write_guard",
            ]
        }
        "readback_execution_disabled" | "rollback_readback_not_executed" => {
            vec!["wal_replay_readback_prerequisite"]
        }
        "idempotency_index_mutation_disabled" => vec!["wal_append_contract"],
        _ => WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS.to_vec(),
    }
}

fn wal_write_boundary_execution_source_ids() -> Vec<&'static str> {
    work_graph_append_only_store_runtime_wal_write_boundary_execution_plans()
        .into_iter()
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn sources_for_residual_blocker(blocker_id: &str) -> Vec<&'static str> {
    work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_residual_blockers()
        .into_iter()
        .find(|blocker| blocker.id == blocker_id)
        .map(|blocker| blocker.affected_source_surface_ids)
        .unwrap_or_default()
}

fn residual_union_source_count(blocker_ids: &[&str]) -> usize {
    let mut sources = Vec::new();
    for blocker_id in blocker_ids {
        for source in sources_for_residual_blocker(blocker_id) {
            if !sources.contains(&source) {
                sources.push(source);
            }
        }
    }
    sources.len()
}

fn wal_write_boundary_execution_plan_id(source_surface_id: &str) -> String {
    format!("append_only_store_runtime_wal_write_boundary_execution_{source_surface_id}_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wal_write_boundary_execution_preview_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WAL_WRITE_BOUNDARY_EXECUTION_SCHEMA_VERSION,
            "work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_v1"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeWalWriteBoundaryExecutionPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.durable_store_switch_enabled);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.replay_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.agent_spawn_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
    }

    #[test]
    fn wal_write_boundary_execution_plans_cover_sources_without_runtime_mutation() {
        let plans = work_graph_append_only_store_runtime_wal_write_boundary_execution_plans_from(
            &sample_decisions(),
        );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.previous_enforcement_decision == "deny_runtime_wal_write_boundary_not_enabled"
                && plan.wal_write_boundary_execution_state
                    == "wal_write_boundary_execution_contract_defined_preview_only"
                && plan
                    .required_wal_write_boundary_execution_stage_ids
                    .as_slice()
                    == WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS
                && plan.expected_evidence_field_ids.as_slice()
                    == WAL_WRITE_BOUNDARY_EXECUTION_EVIDENCE_FIELDS
                && plan.wal_write_boundary_execution_policy_contract_ready_preview
                && plan.collision_replay_evidence_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.mutates_idempotency_index
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.mutates_runtime
        }));
    }

    #[test]
    fn wal_write_boundary_execution_stages_preserve_disabled_runtime() {
        let wal_append_stage = wal_write_boundary_execution_stage_plan(
            "wal_append_contract",
            "p0",
            "wal_append_contract",
            vec!["sample"],
            vec!["wal_append_record_contract_ready"],
            WalWriteBoundaryExecutionStageEffect::WalAppend,
        );
        let no_mutation_stage = wal_write_boundary_execution_stage_plan(
            "wal_no_write_guard",
            "p0",
            "preview_no_mutation",
            vec!["sample"],
            vec!["runtime_no_mutation_guard_ready"],
            WalWriteBoundaryExecutionStageEffect::NoMutation,
        );

        assert!(wal_append_stage.contract_ready_preview);
        assert!(!wal_append_stage.runtime_enabled_after_preview);
        assert!(wal_append_stage.writes_wal);
        assert!(wal_append_stage.writes_checkpoint);
        assert!(!wal_append_stage.mutates_idempotency_index);
        assert!(!no_mutation_stage.mutates_idempotency_index);
        assert!(!no_mutation_stage.writes_wal);
        assert!(!no_mutation_stage.writes_checkpoint);
        assert!(!no_mutation_stage.executes_replay);
        assert!(!no_mutation_stage.executes_readback);
        assert!(!no_mutation_stage.executes_rollback);
    }

    #[test]
    fn wal_write_boundary_execution_blockers_map_to_stages() {
        let blocker = wal_write_boundary_execution_blocker(
            "wal_write_boundary_execution_readback_missing",
            "high",
            "readback_preview",
            vec!["sample"],
            WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS.to_vec(),
            "read back WAL write-boundary execution plans before promotion",
        );

        assert_eq!(
            blocker
                .affected_wal_write_boundary_execution_stage_ids
                .as_slice(),
            WAL_WRITE_BOUNDARY_EXECUTION_STAGE_IDS
        );
        assert!(blocker.required_before_wal_write_boundary_execution);
        assert_eq!(
            stage_ids_for_blocker("wal_write_boundary_not_enabled"),
            vec![
                "wal_append_contract",
                "wal_replay_readback_prerequisite",
                "wal_no_write_guard"
            ]
        );
        assert_eq!(
            category_for_blocker("rollback_readback_not_executed"),
            "rollback_readback"
        );
    }

    fn sample_decisions() -> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview>
    {
        vec![
            sample_decision("update_plan_tool", "planning"),
            sample_decision("hepta_runtime_agent_harness", "external_handoff"),
        ]
    }

    fn sample_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview {
        WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_runtime_idempotency_mutation_rerun_state: "idempotency_mutation_contract_ready_preview_after_application",
            runtime_rollback_readback_execution_rerun_state: "rollback_readback_execution_contract_ready_preview_after_application",
            covered_by_rollback_readback_execution_application_preview: true,
            previous_enforcement_decision: "deny_runtime_rollback_readback_execution_disabled",
            runtime_rollback_readback_execution_rerun_enforcement_decision: "deny_runtime_wal_write_boundary_not_enabled",
            rollback_readback_execution_primary_gap_closed_by_application_preview: true,
            projection_contract_ready: true,
            unified_store_projection_ready: true,
            timeline_projection_ready: true,
            task_result_projection_ready: true,
            store_idempotency_guard_ready: true,
            terminal_task_result_contract_ready: true,
            append_only_route_ready: true,
            append_only_store_precondition_ready: true,
            readback_probe_contract_ready: true,
            scheduler_admission_contract_ready: true,
            role_manifest_contract_ready: true,
            append_only_store_runtime_enablement_ready: true,
            runtime_application_promotion_contract_ready: true,
            operator_review_contract_ready: true,
            side_effect_lock_contract_ready: true,
            durable_store_switch_contract_ready: true,
            rollback_readback_execution_contract_ready: true,
            rollback_readback_execution_applied: false,
            wal_write_enabled: false,
            checkpoint_write_enabled: false,
            durable_store_switch_enabled: false,
            rollback_readback_execution_enabled: false,
            readback_execution_enabled: false,
            rollback_execution_enabled: false,
            runtime_append_only_store_enabled: false,
            scheduler_admission_enforcement_ready: false,
            role_manifest_enforcement_ready: false,
            residual_source_blocker_ids: vec!["wal_write_boundary_not_enabled"],
            residual_route_blocker_ids: Vec::new(),
            next_required_gate: "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate",
        }
    }
}
