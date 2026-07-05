use serde::Serialize;

use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ROLLBACK_READBACK_EXECUTION_APPLICATION_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::WorkGraphRollbackReadbackExecutionApplicationBlockerPreview;
use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::WorkGraphRollbackReadbackExecutionApplicationSourceOutcomePreview;
use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::work_graph_append_only_store_runtime_rollback_readback_execution_application_blockers;
use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::work_graph_append_only_store_runtime_rollback_readback_execution_application_required_prior_gates;
use crate::work_graph_append_only_store_runtime_rollback_readback_execution_application_preview::work_graph_append_only_store_runtime_rollback_readback_execution_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview::WorkGraphRuntimeIdempotencyMutationRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview::work_graph_unified_projection_enforcement_runtime_idempotency_mutation_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub rollback_readback_execution_outcome_count: usize,
    pub rollback_readback_execution_application_covered_source_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub runtime_rollback_readback_execution_rerun_contract_ready_surface_count: usize,
    pub previous_rollback_readback_primary_blocked_surface_count: usize,
    pub rollback_readback_primary_blocked_surface_count_after: usize,
    pub wal_write_primary_blocked_surface_count: usize,
    pub rollback_readback_execution_contract_ready_source_count: usize,
    pub wal_write_enabled_source_count: usize,
    pub durable_store_switch_enabled_source_count: usize,
    pub rollback_readback_execution_enabled_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_rollback_readback_execution_preview: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_rollback_readback_execution: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_runtime_idempotency_mutation_rerun_state: &'static str,
    pub runtime_rollback_readback_execution_rerun_state: &'static str,
    pub covered_by_rollback_readback_execution_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub runtime_rollback_readback_execution_rerun_enforcement_decision: &'static str,
    pub rollback_readback_execution_primary_gap_closed_by_application_preview: bool,
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
    pub operator_review_contract_ready: bool,
    pub side_effect_lock_contract_ready: bool,
    pub durable_store_switch_contract_ready: bool,
    pub rollback_readback_execution_contract_ready: bool,
    pub rollback_readback_execution_applied: bool,
    pub wal_write_enabled: bool,
    pub checkpoint_write_enabled: bool,
    pub durable_store_switch_enabled: bool,
    pub rollback_readback_execution_enabled: bool,
    pub readback_execution_enabled: bool,
    pub rollback_execution_enabled: bool,
    pub runtime_append_only_store_enabled: bool,
    pub scheduler_admission_enforcement_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeRollbackReadbackExecutionRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewSideEffects
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

pub fn hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewReport
{
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_idempotency_mutation_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_blockers();
    let decision_deltas = runtime_rollback_readback_execution_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    );
    let cleared_blockers = runtime_rollback_readback_execution_rerun_cleared_blockers_from(
        &previous_decisions,
        &decision_deltas,
    );
    let residual_blockers =
        runtime_rollback_readback_execution_rerun_residual_blockers_from(&application_blockers);
    let enforcement_stages = runtime_rollback_readback_execution_rerun_stages_from(
        &decision_deltas,
        application_outcomes.len(),
    );
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_required_prior_gates();
    let previous_rollback_readback_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_idempotency_mutation_rerun_enforcement_decision
                == "deny_runtime_rollback_readback_execution_disabled"
        })
        .count();
    let rollback_readback_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.runtime_rollback_readback_execution_rerun_enforcement_decision
                == "deny_runtime_rollback_readback_execution_disabled"
        })
        .count();
    let wal_write_primary_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.runtime_rollback_readback_execution_rerun_enforcement_decision
                == "deny_runtime_wal_write_boundary_not_enabled"
        })
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.runtime_rollback_readback_execution_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        rollback_readback_execution_outcome_count: application_outcomes.len(),
        rollback_readback_execution_application_covered_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.covered_by_rollback_readback_execution_application_preview)
            .count(),
        previous_contract_ready_surface_count: previous_decisions
            .iter()
            .filter(|decision| decision.projection_contract_ready)
            .count(),
        runtime_rollback_readback_execution_rerun_contract_ready_surface_count: decision_deltas
            .iter()
            .filter(|decision| decision.projection_contract_ready)
            .count(),
        previous_rollback_readback_primary_blocked_surface_count,
        rollback_readback_primary_blocked_surface_count_after,
        wal_write_primary_blocked_surface_count,
        rollback_readback_execution_contract_ready_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.rollback_readback_execution_contract_ready)
            .count(),
        wal_write_enabled_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.wal_write_enabled)
            .count(),
        durable_store_switch_enabled_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.durable_store_switch_enabled)
            .count(),
        rollback_readback_execution_enabled_source_count: decision_deltas
            .iter()
            .filter(|decision| {
                decision.readback_execution_enabled && decision.rollback_execution_enabled
            })
            .count(),
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_rollback_readback_execution_preview: true,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_rollback_readback_execution: false,
        ready_for_readback_execution: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_source_decisions()
-> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_idempotency_mutation_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_blockers();
    runtime_rollback_readback_execution_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    )
}

pub fn work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_residual_blockers()
-> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview> {
    let application_blockers =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_blockers();
    runtime_rollback_readback_execution_rerun_residual_blockers_from(&application_blockers)
}

pub fn work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_stages()
-> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_rollback_readback_execution_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_source_outcomes();
    runtime_rollback_readback_execution_rerun_stages_from(&decisions, application_outcomes.len())
}

pub fn work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_rollback_readback_execution_application_required_prior_gates(
        );
    gates.push(
        WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ROLLBACK_READBACK_EXECUTION_APPLICATION_PREVIEW_GATE,
    );
    gates
}

impl
    WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewSideEffects
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

fn runtime_rollback_readback_execution_rerun_source_decisions_from(
    previous_decisions: &[WorkGraphRuntimeIdempotencyMutationRerunSourceDecisionPreview],
    application_outcomes: &[WorkGraphRollbackReadbackExecutionApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphRollbackReadbackExecutionApplicationBlockerPreview],
) -> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview> {
    previous_decisions
        .iter()
        .cloned()
        .map(|decision| {
            runtime_rollback_readback_execution_rerun_source_decision(
                decision,
                application_outcomes,
                application_blockers,
            )
        })
        .collect()
}

fn runtime_rollback_readback_execution_rerun_cleared_blockers_from(
    previous_decisions: &[WorkGraphRuntimeIdempotencyMutationRerunSourceDecisionPreview],
    decisions: &[WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview],
) -> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunClearedBlockerPreview> {
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision.runtime_idempotency_mutation_rerun_enforcement_decision
                == "deny_runtime_rollback_readback_execution_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.runtime_rollback_readback_execution_rerun_enforcement_decision
                == "deny_runtime_rollback_readback_execution_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        WorkGraphRuntimeRollbackReadbackExecutionRerunClearedBlockerPreview {
            id: "rollback_readback_execution_required_for_enforcement",
            source_count_before: before_sources.len(),
            source_count_after: after_sources.len(),
            cleared_source_surface_ids: before_sources,
            closure_gate_id:
                WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ROLLBACK_READBACK_EXECUTION_APPLICATION_PREVIEW_GATE,
        },
    ]
}

fn runtime_rollback_readback_execution_rerun_residual_blockers_from(
    application_blockers: &[WorkGraphRollbackReadbackExecutionApplicationBlockerPreview],
) -> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview> {
    application_blockers
        .iter()
        .filter(|blocker| !is_cleared_runtime_rollback_readback_execution_blocker(blocker.id))
        .map(residual_blocker_from_application_blocker)
        .collect()
}

fn runtime_rollback_readback_execution_rerun_stages_from(
    decisions: &[WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview],
    application_outcome_count: usize,
) -> Vec<WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview> {
    let covered_contract_count = decisions
        .iter()
        .filter(|decision| decision.covered_by_rollback_readback_execution_application_preview)
        .count();
    let wal_sources = residual_union_sources(&["wal_write_boundary_not_enabled"], decisions);
    let rollback_sources = residual_union_sources(
        &[
            "readback_execution_disabled",
            "rollback_readback_not_executed",
        ],
        decisions,
    );
    vec![
        stage(
            "rollback_readback_execution_contracts",
            application_outcome_count,
            0,
            covered_contract_count,
            vec!["rollback_readback_not_executed"],
        ),
        stage(
            "wal_write_boundary_execution",
            wal_sources.len(),
            0,
            0,
            vec!["wal_write_boundary_not_enabled"],
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
            "append_only_store_enablement_dry_run",
            decisions.len(),
            0,
            0,
            vec![
                "wal_write_boundary_not_enabled",
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
                "rollback_readback_not_executed",
            ],
        ),
    ]
}

fn runtime_rollback_readback_execution_rerun_source_decision(
    previous: WorkGraphRuntimeIdempotencyMutationRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphRollbackReadbackExecutionApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphRollbackReadbackExecutionApplicationBlockerPreview],
) -> WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview {
    let covered_by_rollback_readback_execution_application_preview =
        application_outcomes.iter().any(|outcome| {
            outcome.source_surface_id == previous.source_surface_id
                && outcome.rollback_readback_execution_policy_contract_ready_preview
                && outcome.collision_replay_evidence_contract_ready_preview
                && !outcome.applies_to_runtime
        });
    let rollback_readback_execution_contract_ready =
        covered_by_rollback_readback_execution_application_preview
            || !previous
                .residual_source_blocker_ids
                .contains(&"rollback_readback_execution_application_missing");
    let rollback_readback_execution_primary_gap_closed_by_application_preview = previous
        .runtime_idempotency_mutation_rerun_enforcement_decision
        == "deny_runtime_rollback_readback_execution_disabled"
        && rollback_readback_execution_contract_ready;
    let mut residual_source_blocker_ids = previous
        .residual_source_blocker_ids
        .into_iter()
        .filter(|blocker| !is_cleared_runtime_rollback_readback_execution_blocker(blocker))
        .collect::<Vec<_>>();
    for blocker in application_blockers.iter().filter(|blocker| {
        blocker
            .affected_source_surface_ids
            .contains(&previous.source_surface_id)
            && !is_cleared_runtime_rollback_readback_execution_blocker(blocker.id)
    }) {
        push_unique(&mut residual_source_blocker_ids, blocker.id);
    }
    let runtime_rollback_readback_execution_rerun_enforcement_decision =
        runtime_rollback_readback_execution_rerun_enforcement_decision_for(
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
            previous.operator_review_contract_ready,
            previous.side_effect_lock_contract_ready,
            previous.durable_store_switch_contract_ready,
            rollback_readback_execution_contract_ready,
            &previous.residual_route_blocker_ids,
            &residual_source_blocker_ids,
        );

    WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_runtime_idempotency_mutation_rerun_state: previous
            .runtime_idempotency_mutation_rerun_state,
        runtime_rollback_readback_execution_rerun_state:
            if covered_by_rollback_readback_execution_application_preview {
                "rollback_readback_execution_contract_ready_preview_after_application"
            } else {
                "rollback_readback_execution_application_not_required_for_source"
            },
        covered_by_rollback_readback_execution_application_preview,
        previous_enforcement_decision: previous
            .runtime_idempotency_mutation_rerun_enforcement_decision,
        runtime_rollback_readback_execution_rerun_enforcement_decision,
        rollback_readback_execution_primary_gap_closed_by_application_preview,
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
        operator_review_contract_ready: previous.operator_review_contract_ready,
        side_effect_lock_contract_ready: previous.side_effect_lock_contract_ready,
        durable_store_switch_contract_ready: previous.durable_store_switch_contract_ready,
        rollback_readback_execution_contract_ready,
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
        residual_source_blocker_ids,
        residual_route_blocker_ids: previous.residual_route_blocker_ids,
        next_required_gate: runtime_rollback_readback_execution_rerun_next_required_gate_for(
            runtime_rollback_readback_execution_rerun_enforcement_decision,
        ),
    }
}

fn runtime_rollback_readback_execution_rerun_enforcement_decision_for(
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
    durable_store_switch_contract_ready: bool,
    rollback_readback_execution_contract_ready: bool,
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
    } else if !durable_store_switch_contract_ready {
        "deny_runtime_durable_store_switch_disabled"
    } else if !rollback_readback_execution_contract_ready
        || residual_source_blocker_ids.contains(&"rollback_readback_execution_readback_missing")
        || residual_source_blocker_ids.contains(&"rollback_readback_execution_application_missing")
    {
        "deny_runtime_rollback_readback_execution_disabled"
    } else if residual_source_blocker_ids.contains(&"rollback_readback_not_executed")
        || residual_source_blocker_ids.contains(&"readback_execution_disabled")
    {
        "deny_runtime_rollback_readback_execution_disabled"
    } else if residual_source_blocker_ids.contains(&"wal_write_boundary_not_enabled") {
        "deny_runtime_wal_write_boundary_not_enabled"
    } else if residual_route_blocker_ids.contains(&"append_only_store_disabled_by_design") {
        "deny_append_only_store_disabled"
    } else {
        "allow_preview_only"
    }
}

fn runtime_rollback_readback_execution_rerun_next_required_gate_for(
    enforcement_decision: &str,
) -> &'static str {
    match enforcement_decision {
        "deny_runtime_rollback_readback_execution_disabled" => {
            "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_application_preview_gate"
        }
        "deny_runtime_wal_write_boundary_not_enabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE
        }
    }
}

fn is_cleared_runtime_rollback_readback_execution_blocker(id: &str) -> bool {
    matches!(
        id,
        "readback_execution_disabled"
            | "rollback_readback_not_executed"
            | "rollback_readback_execution_readback_missing"
            | "rollback_readback_execution_application_missing"
            | "rollback_readback_execution_readiness_rerun_missing"
    )
}

fn residual_blocker_from_application_blocker(
    blocker: &WorkGraphRollbackReadbackExecutionApplicationBlockerPreview,
) -> WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview {
    WorkGraphRuntimeRollbackReadbackExecutionRerunResidualBlockerPreview {
        id: blocker.id,
        severity: blocker.severity,
        category: blocker.category,
        affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
        required_before_projection_enforcement: true,
        recommended_fix: blocker.recommended_fix,
    }
}

fn residual_union_sources(
    blocker_ids: &[&'static str],
    decisions: &[WorkGraphRuntimeRollbackReadbackExecutionRerunSourceDecisionPreview],
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

fn stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview {
    WorkGraphRuntimeRollbackReadbackExecutionRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE,
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
    fn runtime_rollback_readback_execution_rerun_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_PREVIEW_GATE,
            "hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate"
        );
        let side_effects =
            WorkGraphUnifiedProjectionEnforcementReadinessRuntimeRollbackReadbackExecutionRerunPreviewSideEffects::none();
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
    fn runtime_rollback_readback_execution_rerun_classifies_next_primary_layer() {
        assert!(is_cleared_runtime_rollback_readback_execution_blocker(
            "readback_execution_disabled"
        ));
        assert!(is_cleared_runtime_rollback_readback_execution_blocker(
            "rollback_readback_not_executed"
        ));
        assert!(is_cleared_runtime_rollback_readback_execution_blocker(
            "rollback_readback_execution_readback_missing"
        ));
        assert!(is_cleared_runtime_rollback_readback_execution_blocker(
            "rollback_readback_execution_application_missing"
        ));
        assert!(is_cleared_runtime_rollback_readback_execution_blocker(
            "rollback_readback_execution_readiness_rerun_missing"
        ));
        assert!(!is_cleared_runtime_rollback_readback_execution_blocker(
            "wal_write_boundary_not_enabled"
        ));
        assert_eq!(
            runtime_rollback_readback_execution_rerun_enforcement_decision_for(
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                &[],
                &["rollback_readback_not_executed"],
            ),
            "deny_runtime_rollback_readback_execution_disabled"
        );
        assert_eq!(
            runtime_rollback_readback_execution_rerun_next_required_gate_for(
                "deny_runtime_rollback_readback_execution_disabled"
            ),
            "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_application_preview_gate"
        );
        assert_eq!(
            runtime_rollback_readback_execution_rerun_next_required_gate_for(
                "deny_runtime_wal_write_boundary_not_enabled"
            ),
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_ROLLBACK_READBACK_EXECUTION_RERUN_RECOMMENDED_NEXT_GATE
        );
    }
}
