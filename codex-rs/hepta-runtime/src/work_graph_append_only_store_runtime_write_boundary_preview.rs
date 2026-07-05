use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview;
use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_residual_blockers;
use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions;
use crate::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview::work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_write_boundary_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_write_boundary_readback_preview_gate";

const RUNTIME_WRITE_BOUNDARY_STAGE_IDS: [&str; 5] = [
    "wal_write_boundary",
    "durable_store_runtime_switch_guard",
    "idempotency_mutation_policy",
    "rollback_readback_execution_boundary",
    "no_mutation_guard",
];

const RUNTIME_WRITE_BOUNDARY_EVIDENCE_FIELDS: [&str; 9] = [
    "source_surface_id",
    "source_category",
    "operator_review_side_effect_lock_rerun_decision_ref",
    "wal_write_boundary_contract_id",
    "durable_store_switch_guard_id",
    "idempotency_mutation_policy_id",
    "rollback_readback_boundary_id",
    "residual_source_blocker_ids",
    "no_mutation_guard_ref",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_operator_review_side_effect_lock_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub runtime_write_boundary_source_count: usize,
    pub runtime_write_boundary_plan_count: usize,
    pub runtime_write_boundary_stage_count: usize,
    pub runtime_write_boundary_stage_source_ref_count: usize,
    pub runtime_write_boundary_stage_contract_ref_count: usize,
    pub runtime_write_boundary_plan_stage_ref_count: usize,
    pub runtime_write_boundary_plan_evidence_field_ref_count: usize,
    pub wal_boundary_residual_source_count: usize,
    pub durable_store_residual_source_count: usize,
    pub idempotency_residual_source_count: usize,
    pub rollback_readback_residual_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub runtime_write_boundary_plans: Vec<WorkGraphRuntimeWriteBoundarySourcePlanPreview>,
    pub runtime_write_boundary_stage_plans: Vec<WorkGraphRuntimeWriteBoundaryStagePlanPreview>,
    pub guards: Vec<WorkGraphRuntimeWriteBoundaryGuardPreview>,
    pub blockers: Vec<WorkGraphRuntimeWriteBoundaryBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_write_boundary_readback_preview: bool,
    pub ready_for_runtime_write_boundary_application_preview: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_durable_store_switch: bool,
    pub ready_for_idempotency_mutation: bool,
    pub ready_for_rollback_readback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundarySourcePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_write_boundary_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub write_boundary_state: &'static str,
    pub required_write_boundary_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub write_boundary_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub records_approval: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryStagePlanPreview {
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
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_runtime_write_boundary: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeWriteBoundaryBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_write_boundary_stage_ids: Vec<&'static str>,
    pub affected_runtime_write_boundary_plan_ids: Vec<String>,
    pub required_before_runtime_write_boundary: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewSideEffects {
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
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub runtime_wrapper_attached: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_write_boundary_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions();
    let runtime_write_boundary_plans =
        work_graph_append_only_store_runtime_write_boundary_plans_from(&source_decisions);
    let runtime_write_boundary_stage_plans =
        work_graph_append_only_store_runtime_write_boundary_stage_plans();
    let guards = work_graph_append_only_store_runtime_write_boundary_guards();
    let blockers = work_graph_append_only_store_runtime_write_boundary_blockers();
    let required_prior_gates =
        work_graph_append_only_store_runtime_write_boundary_required_prior_gates();
    let wal_boundary_residual_source_count =
        sources_for_residual_blocker("wal_write_boundary_not_enabled").len();
    let durable_store_residual_source_count =
        sources_for_residual_blocker("durable_store_runtime_switch_disabled").len();
    let idempotency_residual_source_count =
        sources_for_residual_blocker("idempotency_index_mutation_disabled").len();
    let rollback_readback_residual_source_count = residual_union_source_count(&[
        "readback_execution_disabled",
        "rollback_readback_not_executed",
    ]);

    WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_runtime_write_boundary_preview_no_wal_or_store_mutation",
        upstream_operator_review_side_effect_lock_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        runtime_write_boundary_source_count: runtime_write_boundary_plans.len(),
        runtime_write_boundary_plan_count: runtime_write_boundary_plans.len(),
        runtime_write_boundary_stage_count: runtime_write_boundary_stage_plans.len(),
        runtime_write_boundary_stage_source_ref_count: runtime_write_boundary_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        runtime_write_boundary_stage_contract_ref_count: runtime_write_boundary_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        runtime_write_boundary_plan_stage_ref_count: runtime_write_boundary_plans
            .iter()
            .map(|plan| plan.required_write_boundary_stage_ids.len())
            .sum(),
        runtime_write_boundary_plan_evidence_field_ref_count: runtime_write_boundary_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        wal_boundary_residual_source_count,
        durable_store_residual_source_count,
        idempotency_residual_source_count,
        rollback_readback_residual_source_count,
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        runtime_write_boundary_plans,
        runtime_write_boundary_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_write_boundary_readback_preview: true,
        ready_for_runtime_write_boundary_application_preview: false,
        ready_for_wal_write: false,
        ready_for_durable_store_switch: false,
        ready_for_idempotency_mutation: false,
        ready_for_rollback_readback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_write_boundary_plans()
-> Vec<WorkGraphRuntimeWriteBoundarySourcePlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions();
    work_graph_append_only_store_runtime_write_boundary_plans_from(&source_decisions)
}

pub fn work_graph_append_only_store_runtime_write_boundary_stage_plans()
-> Vec<WorkGraphRuntimeWriteBoundaryStagePlanPreview> {
    let source_ids = runtime_write_boundary_source_ids();
    vec![
        write_boundary_stage_plan(
            "wal_write_boundary",
            "p0",
            "wal_boundary",
            source_ids.clone(),
            vec![
                "wal_append_record_contract_ready",
                "wal_ordering_contract_ready",
                "wal_redaction_contract_ready",
                "checkpoint_boundary_contract_ready",
                "replay_cursor_contract_ready",
                "durable_store_no_rewrite_contract_ready",
            ],
            RuntimeWriteBoundaryStageEffect::Wal,
        ),
        write_boundary_stage_plan(
            "durable_store_runtime_switch_guard",
            "p0",
            "durable_store_switch",
            source_ids.clone(),
            vec![
                "durable_store_switch_remains_disabled",
                "append_only_store_enablement_guard_ready",
                "state_store_write_boundary_contract_ready",
                "operator_disable_switch_contract_ready",
                "runtime_store_switch_readback_contract_ready",
            ],
            RuntimeWriteBoundaryStageEffect::DurableStoreSwitch,
        ),
        write_boundary_stage_plan(
            "idempotency_mutation_policy",
            "p0",
            "idempotency_policy",
            source_ids.clone(),
            vec![
                "idempotency_key_formula_contract_ready",
                "idempotency_collision_policy_contract_ready",
                "idempotency_mutation_policy_contract_ready",
                "idempotency_replay_evidence_contract_ready",
                "idempotency_readback_probe_contract_ready",
            ],
            RuntimeWriteBoundaryStageEffect::IdempotencyMutation,
        ),
        write_boundary_stage_plan(
            "rollback_readback_execution_boundary",
            "p0",
            "rollback_readback",
            source_ids.clone(),
            vec![
                "rollback_execution_gate_contract_ready",
                "readback_probe_execution_contract_ready",
                "replay_receipt_contract_ready",
                "checkpoint_readback_contract_ready",
                "rollback_non_promotion_contract_ready",
            ],
            RuntimeWriteBoundaryStageEffect::RollbackReadback,
        ),
        write_boundary_stage_plan(
            "no_mutation_guard",
            "p0",
            "preview_no_mutation",
            source_ids,
            vec![
                "filesystem_no_write_guard_ready",
                "graph_state_no_persist_guard_ready",
                "wal_no_write_guard_ready",
                "checkpoint_no_write_guard_ready",
                "idempotency_no_mutation_guard_ready",
                "runtime_no_mutation_guard_ready",
            ],
            RuntimeWriteBoundaryStageEffect::NoMutation,
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_write_boundary_guards()
-> Vec<WorkGraphRuntimeWriteBoundaryGuardPreview> {
    vec![
        guard(
            "runtime_write_boundary_preview_only",
            "medium",
            "preview_boundary",
        ),
        guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        guard(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_index",
        ),
        guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback_readback",
        ),
        guard("readback_execution_disabled", "critical", "readback"),
        guard("checkpoint_write_disabled", "critical", "checkpoint"),
        guard("runtime_mutation_disabled", "critical", "runtime_mutation"),
    ]
}

pub fn work_graph_append_only_store_runtime_write_boundary_blockers()
-> Vec<WorkGraphRuntimeWriteBoundaryBlockerPreview> {
    let mut blockers = work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_residual_blockers()
        .into_iter()
        .map(write_boundary_blocker_from_residual)
        .collect::<Vec<_>>();
    blockers.push(write_boundary_blocker(
        "runtime_write_boundary_readback_missing",
        "high",
        "readback_preview",
        runtime_write_boundary_source_ids(),
        RUNTIME_WRITE_BOUNDARY_STAGE_IDS.to_vec(),
        "read back runtime write-boundary plans before WAL, store, checkpoint, idempotency, or rollback execution can be promoted",
    ));
    blockers
}

pub fn work_graph_append_only_store_runtime_write_boundary_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewSideEffects {
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

fn work_graph_append_only_store_runtime_write_boundary_plans_from(
    source_decisions: &[WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview],
) -> Vec<WorkGraphRuntimeWriteBoundarySourcePlanPreview> {
    source_decisions
        .iter()
        .filter(|decision| {
            decision.operator_review_side_effect_lock_rerun_enforcement_decision
                == "deny_runtime_append_only_store_write_boundary_disabled"
        })
        .cloned()
        .map(runtime_write_boundary_plan)
        .collect()
}

fn runtime_write_boundary_plan(
    decision: WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview,
) -> WorkGraphRuntimeWriteBoundarySourcePlanPreview {
    WorkGraphRuntimeWriteBoundarySourcePlanPreview {
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        runtime_write_boundary_plan_id: runtime_write_boundary_plan_id(decision.source_surface_id),
        previous_enforcement_decision: decision
            .operator_review_side_effect_lock_rerun_enforcement_decision,
        write_boundary_state: "runtime_write_boundary_contract_defined_preview_only",
        required_write_boundary_stage_ids: RUNTIME_WRITE_BOUNDARY_STAGE_IDS.to_vec(),
        residual_source_blocker_ids: decision.residual_source_blocker_ids,
        expected_evidence_field_ids: RUNTIME_WRITE_BOUNDARY_EVIDENCE_FIELDS.to_vec(),
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

#[derive(Debug, Clone, Copy)]
enum RuntimeWriteBoundaryStageEffect {
    Wal,
    DurableStoreSwitch,
    IdempotencyMutation,
    RollbackReadback,
    NoMutation,
}

fn write_boundary_stage_plan(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
    effect: RuntimeWriteBoundaryStageEffect,
) -> WorkGraphRuntimeWriteBoundaryStagePlanPreview {
    WorkGraphRuntimeWriteBoundaryStagePlanPreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "contract_ready_preview_runtime_disabled",
        prerequisite_gate_ids:
            work_graph_append_only_store_runtime_write_boundary_required_prior_gates(),
        contract_ready_preview: true,
        runtime_enabled_after_preview: false,
        writes_wal: matches!(effect, RuntimeWriteBoundaryStageEffect::Wal),
        writes_checkpoint: matches!(
            effect,
            RuntimeWriteBoundaryStageEffect::Wal
                | RuntimeWriteBoundaryStageEffect::RollbackReadback
        ),
        switches_durable_store: matches!(
            effect,
            RuntimeWriteBoundaryStageEffect::DurableStoreSwitch
        ),
        mutates_idempotency_index: matches!(
            effect,
            RuntimeWriteBoundaryStageEffect::IdempotencyMutation
        ),
        executes_readback: matches!(effect, RuntimeWriteBoundaryStageEffect::RollbackReadback),
        executes_rollback: matches!(effect, RuntimeWriteBoundaryStageEffect::RollbackReadback),
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphRuntimeWriteBoundaryGuardPreview {
    WorkGraphRuntimeWriteBoundaryGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_runtime_write_boundary: true,
        satisfied_by_preview: false,
    }
}

fn write_boundary_blocker_from_residual(
    blocker: WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview,
) -> WorkGraphRuntimeWriteBoundaryBlockerPreview {
    write_boundary_blocker(
        blocker.id,
        blocker.severity,
        category_for_blocker(blocker.id),
        blocker.affected_source_surface_ids,
        stage_ids_for_blocker(blocker.id),
        blocker.recommended_fix,
    )
}

fn write_boundary_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_runtime_write_boundary_stage_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphRuntimeWriteBoundaryBlockerPreview {
    WorkGraphRuntimeWriteBoundaryBlockerPreview {
        affected_runtime_write_boundary_plan_ids: affected_source_surface_ids
            .iter()
            .map(|source| runtime_write_boundary_plan_id(source))
            .collect(),
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_runtime_write_boundary_stage_ids,
        required_before_runtime_write_boundary: true,
        recommended_fix,
    }
}

fn category_for_blocker(id: &str) -> &'static str {
    match id {
        "readback_execution_disabled" | "rollback_readback_not_executed" => "rollback_readback",
        "durable_store_runtime_switch_disabled" => "durable_store_switch",
        "wal_write_boundary_not_enabled" => "wal_boundary",
        "idempotency_index_mutation_disabled" => "idempotency_policy",
        _ => "runtime_write_boundary",
    }
}

fn stage_ids_for_blocker(id: &str) -> Vec<&'static str> {
    match id {
        "readback_execution_disabled" | "rollback_readback_not_executed" => {
            vec!["rollback_readback_execution_boundary"]
        }
        "durable_store_runtime_switch_disabled" => vec!["durable_store_runtime_switch_guard"],
        "wal_write_boundary_not_enabled" => vec!["wal_write_boundary"],
        "idempotency_index_mutation_disabled" => vec!["idempotency_mutation_policy"],
        _ => RUNTIME_WRITE_BOUNDARY_STAGE_IDS.to_vec(),
    }
}

fn runtime_write_boundary_source_ids() -> Vec<&'static str> {
    work_graph_append_only_store_runtime_write_boundary_plans()
        .into_iter()
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn sources_for_residual_blocker(blocker_id: &str) -> Vec<&'static str> {
    work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_residual_blockers()
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

fn runtime_write_boundary_plan_id(source_surface_id: &str) -> String {
    format!("append_only_store_runtime_write_boundary_{source_surface_id}_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_write_boundary_preview_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_write_boundary_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_WRITE_BOUNDARY_SCHEMA_VERSION,
            "work_graph_append_only_store_runtime_write_boundary_preview_v1"
        );
        let side_effects = WorkGraphAppendOnlyStoreRuntimeWriteBoundaryPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.durable_store_switch_enabled);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.agent_spawn_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
    }

    #[test]
    fn runtime_write_boundary_plans_cover_sources_without_writes() {
        let plans =
            work_graph_append_only_store_runtime_write_boundary_plans_from(&sample_decisions());

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.previous_enforcement_decision
                == "deny_runtime_append_only_store_write_boundary_disabled"
                && plan.write_boundary_state
                    == "runtime_write_boundary_contract_defined_preview_only"
                && plan.required_write_boundary_stage_ids.as_slice()
                    == RUNTIME_WRITE_BOUNDARY_STAGE_IDS
                && plan.expected_evidence_field_ids.as_slice()
                    == RUNTIME_WRITE_BOUNDARY_EVIDENCE_FIELDS
                && plan.write_boundary_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.switches_durable_store
                && !plan.mutates_idempotency_index
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.records_approval
                && !plan.mutates_runtime
        }));
    }

    #[test]
    fn runtime_write_boundary_stages_preserve_runtime_disabled() {
        let wal_stage = write_boundary_stage_plan(
            "wal_write_boundary",
            "p0",
            "wal_boundary",
            vec!["sample"],
            vec!["wal_append_record_contract_ready"],
            RuntimeWriteBoundaryStageEffect::Wal,
        );
        let no_mutation_stage = write_boundary_stage_plan(
            "no_mutation_guard",
            "p0",
            "preview_no_mutation",
            vec!["sample"],
            vec!["runtime_no_mutation_guard_ready"],
            RuntimeWriteBoundaryStageEffect::NoMutation,
        );

        assert!(wal_stage.contract_ready_preview);
        assert!(!wal_stage.runtime_enabled_after_preview);
        assert!(wal_stage.writes_wal);
        assert!(wal_stage.writes_checkpoint);
        assert!(!wal_stage.switches_durable_store);
        assert!(!no_mutation_stage.writes_wal);
        assert!(!no_mutation_stage.writes_checkpoint);
        assert!(!no_mutation_stage.switches_durable_store);
        assert!(!no_mutation_stage.mutates_idempotency_index);
        assert!(!no_mutation_stage.executes_readback);
        assert!(!no_mutation_stage.executes_rollback);
    }

    #[test]
    fn runtime_write_boundary_blockers_map_to_stages() {
        let blocker = write_boundary_blocker(
            "runtime_write_boundary_readback_missing",
            "high",
            "readback_preview",
            vec!["sample"],
            RUNTIME_WRITE_BOUNDARY_STAGE_IDS.to_vec(),
            "read back runtime write-boundary plans before promotion",
        );

        assert_eq!(
            blocker.affected_runtime_write_boundary_stage_ids.as_slice(),
            RUNTIME_WRITE_BOUNDARY_STAGE_IDS
        );
        assert!(blocker.required_before_runtime_write_boundary);
        assert_eq!(
            stage_ids_for_blocker("wal_write_boundary_not_enabled"),
            vec!["wal_write_boundary"]
        );
        assert_eq!(
            category_for_blocker("idempotency_index_mutation_disabled"),
            "idempotency_policy"
        );
    }

    fn sample_decisions() -> Vec<WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview> {
        vec![
            sample_decision("update_plan_tool", "planning"),
            sample_decision("hepta_runtime_agent_harness", "external_handoff"),
        ]
    }

    fn sample_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview {
        WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_runtime_application_promotion_rerun_state: "runtime_application_promotion_contract_ready_preview_after_application",
            operator_review_side_effect_lock_rerun_state: "operator_review_side_effect_lock_contract_ready_preview_after_application",
            covered_by_operator_review_side_effect_lock_application_preview: true,
            previous_enforcement_decision: "deny_operator_review_required",
            operator_review_side_effect_lock_rerun_enforcement_decision: "deny_runtime_append_only_store_write_boundary_disabled",
            operator_review_primary_gap_closed_by_application_preview: true,
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
            runtime_application_promoted: false,
            operator_review_contract_ready: true,
            side_effect_lock_contract_ready: true,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            runtime_append_only_store_enabled: false,
            scheduler_admission_enforcement_ready: false,
            role_manifest_enforcement_ready: false,
            residual_source_blocker_ids: vec![
                "wal_write_boundary_not_enabled",
                "rollback_readback_not_executed",
                "readback_execution_disabled",
                "durable_store_runtime_switch_disabled",
                "idempotency_index_mutation_disabled",
            ],
            residual_route_blocker_ids: Vec::new(),
            next_required_gate: "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate",
        }
    }
}
