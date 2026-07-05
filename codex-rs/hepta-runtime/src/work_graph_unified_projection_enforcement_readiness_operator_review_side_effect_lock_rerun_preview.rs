use serde::Serialize;

use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::work_graph_append_only_store_operator_review_side_effect_lock_application_blockers;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::work_graph_append_only_store_operator_review_side_effect_lock_application_required_prior_gates;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_application_preview::work_graph_append_only_store_operator_review_side_effect_lock_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview::WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview::work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub operator_review_side_effect_lock_outcome_count: usize,
    pub operator_review_side_effect_lock_application_covered_source_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub operator_review_side_effect_lock_rerun_contract_ready_surface_count: usize,
    pub previous_operator_review_primary_blocked_surface_count: usize,
    pub operator_review_primary_blocked_surface_count_after: usize,
    pub previous_write_boundary_primary_blocked_surface_count: usize,
    pub write_boundary_primary_blocked_surface_count: usize,
    pub operator_review_contract_ready_source_count: usize,
    pub side_effect_lock_contract_ready_source_count: usize,
    pub operator_review_recorded_source_count: usize,
    pub side_effect_lock_established_source_count: usize,
    pub wal_boundary_residual_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphOperatorReviewSideEffectLockRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphOperatorReviewSideEffectLockRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_review_recording: bool,
    pub ready_for_side_effect_lock_establishment: bool,
    pub ready_for_runtime_write_boundary_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_runtime_application_promotion_rerun_state: &'static str,
    pub operator_review_side_effect_lock_rerun_state: &'static str,
    pub covered_by_operator_review_side_effect_lock_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub operator_review_side_effect_lock_rerun_enforcement_decision: &'static str,
    pub operator_review_primary_gap_closed_by_application_preview: bool,
    pub projection_contract_ready: bool,
    pub unified_store_projection_ready: bool,
    pub timeline_projection_ready: bool,
    pub task_result_projection_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub terminal_task_result_contract_ready: bool,
    pub append_only_route_ready: bool,
    pub append_only_store_precondition_ready: bool,
    pub readback_probe_contract_ready: bool,
    pub scheduler_admission_contract_ready: bool,
    pub role_manifest_contract_ready: bool,
    pub append_only_store_runtime_enablement_ready: bool,
    pub runtime_application_promotion_contract_ready: bool,
    pub runtime_application_promoted: bool,
    pub operator_review_contract_ready: bool,
    pub side_effect_lock_contract_ready: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub runtime_append_only_store_enabled: bool,
    pub scheduler_admission_enforcement_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects
{
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

pub fn hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_operator_review_side_effect_lock_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_store_operator_review_side_effect_lock_application_blockers();
    let decision_deltas = operator_review_side_effect_lock_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    );
    let cleared_blockers = operator_review_side_effect_lock_rerun_cleared_blockers_from(
        &previous_decisions,
        &decision_deltas,
    );
    let residual_blockers =
        operator_review_side_effect_lock_rerun_residual_blockers_from(&application_blockers);
    let enforcement_stages = operator_review_side_effect_lock_rerun_stages_from(
        &decision_deltas,
        application_outcomes.len(),
    );
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_required_prior_gates();
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let operator_review_side_effect_lock_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_operator_review_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_application_promotion_rerun_enforcement_decision
                == "deny_operator_review_required"
        })
        .count();
    let operator_review_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.operator_review_side_effect_lock_rerun_enforcement_decision
                == "deny_operator_review_required"
        })
        .count();
    let previous_write_boundary_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_application_promotion_rerun_enforcement_decision
                == "deny_runtime_append_only_store_write_boundary_disabled"
        })
        .count();
    let write_boundary_primary_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.operator_review_side_effect_lock_rerun_enforcement_decision
                == "deny_runtime_append_only_store_write_boundary_disabled"
        })
        .count();
    let operator_review_contract_ready_source_count = decision_deltas
        .iter()
        .filter(|decision| decision.operator_review_contract_ready)
        .count();
    let side_effect_lock_contract_ready_source_count = decision_deltas
        .iter()
        .filter(|decision| decision.side_effect_lock_contract_ready)
        .count();
    let operator_review_recorded_source_count = decision_deltas
        .iter()
        .filter(|decision| decision.operator_review_recorded)
        .count();
    let side_effect_lock_established_source_count = decision_deltas
        .iter()
        .filter(|decision| decision.side_effect_lock_established)
        .count();
    let wal_boundary_residual_source_count = affected_sources(&decision_deltas, |decision| {
        decision
            .residual_source_blocker_ids
            .contains(&"wal_write_boundary_not_enabled")
    })
    .len();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.operator_review_side_effect_lock_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .count();
    let operator_review_side_effect_lock_application_covered_source_count = decision_deltas
        .iter()
        .filter(|decision| decision.covered_by_operator_review_side_effect_lock_application_preview)
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        operator_review_side_effect_lock_outcome_count: application_outcomes.len(),
        operator_review_side_effect_lock_application_covered_source_count,
        previous_contract_ready_surface_count,
        operator_review_side_effect_lock_rerun_contract_ready_surface_count,
        previous_operator_review_primary_blocked_surface_count,
        operator_review_primary_blocked_surface_count_after,
        previous_write_boundary_primary_blocked_surface_count,
        write_boundary_primary_blocked_surface_count,
        operator_review_contract_ready_source_count,
        side_effect_lock_contract_ready_source_count,
        operator_review_recorded_source_count,
        side_effect_lock_established_source_count,
        wal_boundary_residual_source_count,
        rerun_ready_surface_count,
        rerun_blocked_surface_count: decision_deltas.len() - rerun_ready_surface_count,
        decision_delta_count: decision_deltas.len(),
        cleared_blocker_count: cleared_blockers.len(),
        residual_blocker_count: residual_blockers.len(),
        enforcement_stage_count: enforcement_stages.len(),
        required_prior_gate_count: required_prior_gates.len(),
        decision_deltas,
        cleared_blockers,
        residual_blockers,
        enforcement_stages,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_operator_review_recording: false,
        ready_for_side_effect_lock_establishment: false,
        ready_for_runtime_write_boundary_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions()
-> Vec<WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_operator_review_side_effect_lock_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_store_operator_review_side_effect_lock_application_blockers();
    operator_review_side_effect_lock_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    )
}

pub fn work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_cleared_blockers()
-> Vec<WorkGraphOperatorReviewSideEffectLockRerunClearedBlockerPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    let decisions =
        work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions();
    operator_review_side_effect_lock_rerun_cleared_blockers_from(&previous_decisions, &decisions)
}

pub fn work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_residual_blockers()
-> Vec<WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview> {
    let application_blockers =
        work_graph_append_only_store_operator_review_side_effect_lock_application_blockers();
    operator_review_side_effect_lock_rerun_residual_blockers_from(&application_blockers)
}

pub fn work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_stages()
-> Vec<WorkGraphOperatorReviewSideEffectLockRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_operator_review_side_effect_lock_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_operator_review_side_effect_lock_application_source_outcomes();
    operator_review_side_effect_lock_rerun_stages_from(&decisions, application_outcomes.len())
}

pub fn work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_operator_review_side_effect_lock_application_required_prior_gates(
        );
    gates.push(
        WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE,
    );
    gates
}

impl
    WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects
{
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

fn operator_review_side_effect_lock_rerun_source_decisions_from(
    previous_decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
    application_outcomes: &[WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview> {
    previous_decisions
        .iter()
        .cloned()
        .map(|decision| {
            operator_review_side_effect_lock_rerun_source_decision(
                decision,
                application_outcomes,
                application_blockers,
            )
        })
        .collect()
}

fn operator_review_side_effect_lock_rerun_cleared_blockers_from(
    previous_decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
    decisions: &[WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockRerunClearedBlockerPreview> {
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_application_promotion_rerun_enforcement_decision
                == "deny_operator_review_required"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.operator_review_side_effect_lock_rerun_enforcement_decision
                == "deny_operator_review_required"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        WorkGraphOperatorReviewSideEffectLockRerunClearedBlockerPreview {
            id: "operator_review_side_effect_lock_required_for_enforcement",
            source_count_before: before_sources.len(),
            source_count_after: after_sources.len(),
            cleared_source_surface_ids: before_sources,
            closure_gate_id:
                WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE,
        },
    ]
}

fn operator_review_side_effect_lock_rerun_residual_blockers_from(
    application_blockers: &[WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview> {
    application_blockers
        .iter()
        .filter(|blocker| !is_cleared_operator_review_side_effect_lock_blocker(blocker.id))
        .map(residual_blocker_from_application_blocker)
        .collect()
}

fn operator_review_side_effect_lock_rerun_stages_from(
    decisions: &[WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview],
    application_outcome_count: usize,
) -> Vec<WorkGraphOperatorReviewSideEffectLockRerunStagePreview> {
    let covered_contract_count = decisions
        .iter()
        .filter(|decision| decision.covered_by_operator_review_side_effect_lock_application_preview)
        .count();
    let wal_boundary_sources =
        residual_union_sources(&["wal_write_boundary_not_enabled"], decisions);
    let durable_switch_sources =
        residual_union_sources(&["durable_store_runtime_switch_disabled"], decisions);
    let idempotency_sources =
        residual_union_sources(&["idempotency_index_mutation_disabled"], decisions);
    let rollback_sources = residual_union_sources(
        &[
            "readback_execution_disabled",
            "rollback_readback_not_executed",
        ],
        decisions,
    );
    vec![
        stage(
            "operator_review_side_effect_lock_contracts",
            application_outcome_count,
            0,
            covered_contract_count,
            vec!["wal_write_boundary_not_enabled"],
        ),
        stage(
            "durable_store_runtime_switch",
            durable_switch_sources.len(),
            0,
            0,
            vec!["durable_store_runtime_switch_disabled"],
        ),
        stage(
            "wal_write_boundary",
            wal_boundary_sources.len(),
            0,
            0,
            vec!["wal_write_boundary_not_enabled"],
        ),
        stage(
            "idempotency_mutation_policy",
            idempotency_sources.len(),
            0,
            0,
            vec!["idempotency_index_mutation_disabled"],
        ),
        stage(
            "rollback_readback_execution_gate",
            rollback_sources.len(),
            0,
            0,
            vec![
                "readback_execution_disabled",
                "rollback_readback_not_executed",
            ],
        ),
        stage(
            "projection_enforcement_dry_run",
            decisions.len(),
            0,
            0,
            vec![
                "wal_write_boundary_not_enabled",
                "durable_store_runtime_switch_disabled",
                "idempotency_index_mutation_disabled",
                "rollback_readback_not_executed",
            ],
        ),
    ]
}

fn operator_review_side_effect_lock_rerun_source_decision(
    previous: WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphOperatorReviewSideEffectLockApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview],
) -> WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview {
    let covered_by_operator_review_side_effect_lock_application_preview =
        application_outcomes.iter().any(|outcome| {
            outcome.source_surface_id == previous.source_surface_id
                && outcome.operator_review_contract_ready_preview
                && outcome.side_effect_lock_contract_ready_preview
                && !outcome.applies_to_runtime
        });
    let operator_review_contract_ready =
        covered_by_operator_review_side_effect_lock_application_preview
            || !previous
                .residual_source_blocker_ids
                .contains(&"operator_review_required");
    let side_effect_lock_contract_ready =
        covered_by_operator_review_side_effect_lock_application_preview
            || !previous
                .residual_source_blocker_ids
                .contains(&"side_effect_lock_not_established");
    let operator_review_primary_gap_closed_by_application_preview = previous
        .runtime_application_promotion_rerun_enforcement_decision
        == "deny_operator_review_required"
        && operator_review_contract_ready
        && side_effect_lock_contract_ready;
    let mut residual_source_blocker_ids = previous
        .residual_source_blocker_ids
        .into_iter()
        .filter(|blocker| !is_cleared_operator_review_side_effect_lock_blocker(blocker))
        .collect::<Vec<_>>();
    for blocker in application_blockers.iter().filter(|blocker| {
        blocker
            .affected_source_surface_ids
            .contains(&previous.source_surface_id)
            && !is_cleared_operator_review_side_effect_lock_blocker(blocker.id)
    }) {
        push_unique(&mut residual_source_blocker_ids, blocker.id);
    }
    let operator_review_side_effect_lock_rerun_enforcement_decision =
        operator_review_side_effect_lock_rerun_enforcement_decision_for(
            previous.unified_store_projection_ready,
            previous.timeline_projection_ready,
            previous.task_result_projection_ready,
            previous.append_only_route_ready,
            previous.store_idempotency_guard_ready,
            previous.terminal_task_result_contract_ready,
            previous.append_only_store_precondition_ready,
            previous.readback_probe_contract_ready,
            previous.scheduler_admission_contract_ready,
            previous.role_manifest_contract_ready,
            previous.append_only_store_runtime_enablement_ready,
            previous.runtime_application_promotion_contract_ready,
            operator_review_contract_ready,
            side_effect_lock_contract_ready,
            &previous.residual_route_blocker_ids,
            &residual_source_blocker_ids,
        );

    WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_runtime_application_promotion_rerun_state: previous
            .runtime_application_promotion_rerun_state,
        operator_review_side_effect_lock_rerun_state:
            if covered_by_operator_review_side_effect_lock_application_preview {
                "operator_review_side_effect_lock_contract_ready_preview_after_application"
            } else {
                "operator_review_side_effect_lock_not_required_for_source"
            },
        covered_by_operator_review_side_effect_lock_application_preview,
        previous_enforcement_decision: previous
            .runtime_application_promotion_rerun_enforcement_decision,
        operator_review_side_effect_lock_rerun_enforcement_decision,
        operator_review_primary_gap_closed_by_application_preview,
        projection_contract_ready: previous.projection_contract_ready,
        unified_store_projection_ready: previous.unified_store_projection_ready,
        timeline_projection_ready: previous.timeline_projection_ready,
        task_result_projection_ready: previous.task_result_projection_ready,
        store_idempotency_guard_ready: previous.store_idempotency_guard_ready,
        terminal_task_result_contract_ready: previous.terminal_task_result_contract_ready,
        append_only_route_ready: previous.append_only_route_ready,
        append_only_store_precondition_ready: previous.append_only_store_precondition_ready,
        readback_probe_contract_ready: previous.readback_probe_contract_ready,
        scheduler_admission_contract_ready: previous.scheduler_admission_contract_ready,
        role_manifest_contract_ready: previous.role_manifest_contract_ready,
        append_only_store_runtime_enablement_ready: previous
            .append_only_store_runtime_enablement_ready,
        runtime_application_promotion_contract_ready: previous
            .runtime_application_promotion_contract_ready,
        runtime_application_promoted: false,
        operator_review_contract_ready,
        side_effect_lock_contract_ready,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        runtime_append_only_store_enabled: false,
        scheduler_admission_enforcement_ready: false,
        role_manifest_enforcement_ready: false,
        residual_source_blocker_ids,
        residual_route_blocker_ids: previous.residual_route_blocker_ids,
        next_required_gate: operator_review_side_effect_lock_rerun_next_required_gate_for(
            operator_review_side_effect_lock_rerun_enforcement_decision,
        ),
    }
}

fn operator_review_side_effect_lock_rerun_enforcement_decision_for(
    unified_store_projection_ready: bool,
    timeline_projection_ready: bool,
    task_result_projection_ready: bool,
    append_only_route_ready: bool,
    store_idempotency_guard_ready: bool,
    terminal_task_result_contract_ready: bool,
    append_only_store_precondition_ready: bool,
    readback_probe_contract_ready: bool,
    scheduler_admission_contract_ready: bool,
    role_manifest_contract_ready: bool,
    append_only_store_runtime_enablement_ready: bool,
    runtime_application_promotion_contract_ready: bool,
    operator_review_contract_ready: bool,
    side_effect_lock_contract_ready: bool,
    residual_route_blocker_ids: &[&'static str],
    residual_source_blocker_ids: &[&'static str],
) -> &'static str {
    if !unified_store_projection_ready {
        "deny_missing_unified_store_projection"
    } else if !timeline_projection_ready {
        "deny_missing_timeline_projection"
    } else if !task_result_projection_ready {
        "deny_missing_task_result_projection"
    } else if !append_only_route_ready {
        "deny_missing_append_only_route"
    } else if !store_idempotency_guard_ready {
        "deny_missing_store_idempotency_guard"
    } else if !terminal_task_result_contract_ready {
        "deny_terminal_task_result_contract_missing"
    } else if !append_only_store_precondition_ready {
        "deny_append_only_store_precondition_missing"
    } else if !readback_probe_contract_ready {
        "deny_missing_readback_probe"
    } else if !scheduler_admission_contract_ready {
        "deny_scheduler_admission_not_enforced"
    } else if !role_manifest_contract_ready {
        "deny_role_manifest_not_enforced"
    } else if !append_only_store_runtime_enablement_ready
        || residual_source_blocker_ids.contains(&"append_only_store_runtime_enablement_disabled")
    {
        "deny_runtime_append_only_store_enablement_disabled"
    } else if !runtime_application_promotion_contract_ready
        || residual_source_blocker_ids.contains(&"runtime_application_residuals_not_promoted")
    {
        "deny_runtime_application_residuals_not_promoted"
    } else if !operator_review_contract_ready
        || !side_effect_lock_contract_ready
        || residual_source_blocker_ids.contains(&"operator_review_required")
        || residual_source_blocker_ids.contains(&"side_effect_lock_not_established")
    {
        "deny_operator_review_required"
    } else if residual_source_blocker_ids.contains(&"wal_write_boundary_not_enabled")
        || residual_source_blocker_ids.contains(&"durable_store_runtime_switch_disabled")
        || residual_source_blocker_ids.contains(&"idempotency_index_mutation_disabled")
        || residual_source_blocker_ids.contains(&"rollback_readback_not_executed")
        || residual_source_blocker_ids.contains(&"readback_execution_disabled")
    {
        "deny_runtime_append_only_store_write_boundary_disabled"
    } else if residual_route_blocker_ids.contains(&"append_only_store_disabled_by_design") {
        "deny_append_only_store_disabled"
    } else {
        "allow_preview_only"
    }
}

fn operator_review_side_effect_lock_rerun_next_required_gate_for(
    enforcement_decision: &str,
) -> &'static str {
    match enforcement_decision {
        "deny_runtime_append_only_store_write_boundary_disabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE
        }
    }
}

fn is_cleared_operator_review_side_effect_lock_blocker(id: &str) -> bool {
    matches!(
        id,
        "operator_review_required"
            | "side_effect_lock_not_established"
            | "operator_review_side_effect_lock_readback_missing"
            | "operator_review_side_effect_lock_application_missing"
            | "operator_review_side_effect_lock_readiness_rerun_missing"
    )
}

fn residual_blocker_from_application_blocker(
    blocker: &WorkGraphOperatorReviewSideEffectLockApplicationBlockerPreview,
) -> WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview {
    WorkGraphOperatorReviewSideEffectLockRerunResidualBlockerPreview {
        id: blocker.id,
        severity: blocker.severity,
        affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
        required_before_projection_enforcement: true,
        recommended_fix: blocker.recommended_fix,
    }
}

fn residual_union_sources(
    blocker_ids: &[&'static str],
    decisions: &[WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview],
) -> Vec<&'static str> {
    let mut sources = Vec::new();
    for decision in decisions {
        if blocker_ids
            .iter()
            .any(|blocker_id| decision.residual_source_blocker_ids.contains(blocker_id))
        {
            push_unique(&mut sources, decision.source_surface_id);
        }
    }
    sources
}

fn affected_sources(
    decisions: &[WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphOperatorReviewSideEffectLockRerunSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphOperatorReviewSideEffectLockRerunStagePreview {
    WorkGraphOperatorReviewSideEffectLockRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE,
    }
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
    fn operator_review_side_effect_lock_rerun_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_PREVIEW_GATE,
            "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
        );
        assert_eq!(
            WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects::none(),
            WorkGraphUnifiedProjectionEnforcementReadinessOperatorReviewSideEffectLockRerunPreviewSideEffects {
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
        );
    }

    #[test]
    fn operator_review_side_effect_lock_rerun_classifies_cleared_and_next_gates() {
        let required_prior_gates =
            work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_required_prior_gates();

        assert_eq!(required_prior_gates.len(), 51);
        assert_eq!(
            required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_APPLICATION_PREVIEW_GATE)
        );
        assert!(is_cleared_operator_review_side_effect_lock_blocker(
            "operator_review_required"
        ));
        assert!(is_cleared_operator_review_side_effect_lock_blocker(
            "side_effect_lock_not_established"
        ));
        assert!(is_cleared_operator_review_side_effect_lock_blocker(
            "operator_review_side_effect_lock_readiness_rerun_missing"
        ));
        assert!(!is_cleared_operator_review_side_effect_lock_blocker(
            "wal_write_boundary_not_enabled"
        ));
        assert_eq!(
            operator_review_side_effect_lock_rerun_next_required_gate_for(
                "deny_runtime_append_only_store_write_boundary_disabled"
            ),
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RERUN_RECOMMENDED_NEXT_GATE
        );
    }
}
