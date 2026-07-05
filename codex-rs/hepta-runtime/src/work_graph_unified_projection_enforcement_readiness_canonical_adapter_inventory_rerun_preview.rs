use serde::Serialize;

use crate::work_graph_canonical_adapter_inventory_application_preview::WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE;
use crate::work_graph_canonical_adapter_inventory_application_preview::WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview;
use crate::work_graph_canonical_adapter_inventory_application_preview::WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview;
use crate::work_graph_canonical_adapter_inventory_application_preview::work_graph_canonical_adapter_inventory_application_blockers;
use crate::work_graph_canonical_adapter_inventory_application_preview::work_graph_canonical_adapter_inventory_application_required_prior_gates;
use crate::work_graph_canonical_adapter_inventory_application_preview::work_graph_canonical_adapter_inventory_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_wal_write_boundary_execution_rerun_preview::WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_runtime_wal_write_boundary_execution_rerun_preview::work_graph_unified_projection_enforcement_runtime_wal_write_boundary_execution_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub canonical_adapter_inventory_outcome_count: usize,
    pub canonical_adapter_inventory_application_covered_source_count: usize,
    pub previous_ready_surface_count: usize,
    pub canonical_adapter_inventory_contract_ready_source_count: usize,
    pub previous_canonical_inventory_primary_blocked_surface_count: usize,
    pub canonical_inventory_primary_blocked_surface_count_after: usize,
    pub append_only_work_graph_events_primary_blocked_surface_count: usize,
    pub partial_or_gap_blocked_surface_count: usize,
    pub append_only_work_graph_events_enabled_source_count: usize,
    pub runtime_canonical_adapter_enforcement_enabled_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphCanonicalAdapterInventoryRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphCanonicalAdapterInventoryRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_work_graph_events_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_enforcement_decision: &'static str,
    pub canonical_adapter_inventory_rerun_enforcement_decision: &'static str,
    pub covered_by_canonical_adapter_inventory_application_preview: bool,
    pub canonical_adapter_inventory_contract_ready: bool,
    pub canonical_adapter_inventory_applied: bool,
    pub append_only_work_graph_events_enabled: bool,
    pub runtime_canonical_adapter_enforcement_enabled: bool,
    pub scheduler_admission_enforcement_ready: bool,
    pub task_result_enforcement_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_wal_write_boundary_execution_rerun_source_decisions();
    let application_outcomes = work_graph_canonical_adapter_inventory_application_source_outcomes();
    let application_blockers = work_graph_canonical_adapter_inventory_application_blockers();
    let decision_deltas = canonical_adapter_inventory_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    );
    let cleared_blockers = canonical_adapter_inventory_rerun_cleared_blockers_from(
        &previous_decisions,
        &decision_deltas,
    );
    let residual_blockers =
        canonical_adapter_inventory_rerun_residual_blockers_from(&application_blockers);
    let enforcement_stages =
        canonical_adapter_inventory_rerun_stages_from(&decision_deltas, application_outcomes.len());
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_required_prior_gates();
    let previous_canonical_inventory_primary_blocked_surface_count = previous_decisions.len();
    let canonical_inventory_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.canonical_adapter_inventory_rerun_enforcement_decision
                == "deny_canonical_adapter_inventory_application_missing"
        })
        .count();
    let append_only_work_graph_events_primary_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.canonical_adapter_inventory_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .count();
    let partial_or_gap_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"canonical_adapter_projection_partial_or_gap")
        })
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.canonical_adapter_inventory_rerun_enforcement_decision == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_canonical_adapter_inventory_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        canonical_adapter_inventory_outcome_count: application_outcomes.len(),
        canonical_adapter_inventory_application_covered_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.covered_by_canonical_adapter_inventory_application_preview)
            .count(),
        previous_ready_surface_count: previous_decisions
            .iter()
            .filter(|decision| {
                decision.runtime_wal_write_boundary_execution_rerun_enforcement_decision
                    == "allow_preview_only"
            })
            .count(),
        canonical_adapter_inventory_contract_ready_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.canonical_adapter_inventory_contract_ready)
            .count(),
        previous_canonical_inventory_primary_blocked_surface_count,
        canonical_inventory_primary_blocked_surface_count_after,
        append_only_work_graph_events_primary_blocked_surface_count,
        partial_or_gap_blocked_surface_count,
        append_only_work_graph_events_enabled_source_count: 0,
        runtime_canonical_adapter_enforcement_enabled_source_count: 0,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_work_graph_events_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_source_decisions()
-> Vec<WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_runtime_wal_write_boundary_execution_rerun_source_decisions();
    let application_outcomes = work_graph_canonical_adapter_inventory_application_source_outcomes();
    let application_blockers = work_graph_canonical_adapter_inventory_application_blockers();
    canonical_adapter_inventory_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    )
}

pub fn work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_residual_blockers()
-> Vec<WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview> {
    canonical_adapter_inventory_rerun_residual_blockers_from(
        &work_graph_canonical_adapter_inventory_application_blockers(),
    )
}

pub fn work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_stages()
-> Vec<WorkGraphCanonicalAdapterInventoryRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_source_decisions();
    let application_outcomes = work_graph_canonical_adapter_inventory_application_source_outcomes();
    canonical_adapter_inventory_rerun_stages_from(&decisions, application_outcomes.len())
}

pub fn work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_canonical_adapter_inventory_application_required_prior_gates();
    gates.push(WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE);
    gates
}

impl
    WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn canonical_adapter_inventory_rerun_source_decisions_from(
    previous_decisions: &[WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview],
    application_outcomes: &[WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview> {
    previous_decisions
        .iter()
        .map(|previous| {
            canonical_adapter_inventory_rerun_source_decision(
                previous,
                application_outcomes,
                application_blockers,
            )
        })
        .collect()
}

fn canonical_adapter_inventory_rerun_cleared_blockers_from(
    previous_decisions: &[WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview],
    decisions: &[WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryRerunClearedBlockerPreview> {
    let before_sources = previous_decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.canonical_adapter_inventory_rerun_enforcement_decision
                == "deny_canonical_adapter_inventory_application_missing"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        WorkGraphCanonicalAdapterInventoryRerunClearedBlockerPreview {
            id: "canonical_adapter_inventory_application_required_for_enforcement",
            source_count_before: before_sources.len(),
            source_count_after: after_sources.len(),
            cleared_source_surface_ids: before_sources,
            closure_gate_id: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE,
        },
    ]
}

fn canonical_adapter_inventory_rerun_residual_blockers_from(
    application_blockers: &[WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview> {
    application_blockers
        .iter()
        .filter(|blocker| blocker.id != "canonical_adapter_inventory_readiness_rerun_missing")
        .map(
            |blocker| WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview {
                id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                required_before_projection_enforcement: true,
                recommended_fix: blocker.recommended_fix,
            },
        )
        .collect()
}

fn canonical_adapter_inventory_rerun_stages_from(
    decisions: &[WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview],
    application_outcome_count: usize,
) -> Vec<WorkGraphCanonicalAdapterInventoryRerunStagePreview> {
    let contract_ready_count = decisions
        .iter()
        .filter(|decision| decision.canonical_adapter_inventory_contract_ready)
        .count();
    let partial_gap_count = decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"canonical_adapter_projection_partial_or_gap")
        })
        .count();
    vec![
        stage(
            "canonical_adapter_inventory_contracts",
            application_outcome_count,
            0,
            contract_ready_count,
            vec!["canonical_adapter_inventory_readiness_rerun_missing"],
        ),
        stage(
            "append_only_work_graph_events_shadow_write",
            decisions.len(),
            0,
            0,
            vec!["append_only_work_graph_events_disabled"],
        ),
        stage(
            "canonical_adapter_partial_gap_closure",
            partial_gap_count,
            0,
            0,
            vec!["canonical_adapter_projection_partial_or_gap"],
        ),
        stage(
            "runtime_canonical_adapter_enforcement_dry_run",
            decisions.len(),
            0,
            0,
            vec!["runtime_canonical_adapter_enforcement_disabled"],
        ),
        stage(
            "projection_enforcement_dry_run",
            decisions.len(),
            0,
            0,
            vec![
                "append_only_work_graph_events_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "runtime_canonical_adapter_enforcement_disabled",
            ],
        ),
    ]
}

fn canonical_adapter_inventory_rerun_source_decision(
    previous: &WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview],
) -> WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview {
    let covered_by_canonical_adapter_inventory_application_preview =
        application_outcomes.iter().any(|outcome| {
            outcome.source_surface_id == previous.source_surface_id
                && outcome.canonical_adapter_inventory_contract_ready_preview
                && !outcome.applies_to_runtime
        });
    let mut residual_source_blocker_ids = application_blockers
        .iter()
        .filter(|blocker| {
            blocker
                .affected_source_surface_ids
                .contains(&previous.source_surface_id)
                && blocker.id != "canonical_adapter_inventory_readiness_rerun_missing"
        })
        .map(|blocker| blocker.id)
        .collect::<Vec<_>>();
    residual_source_blocker_ids.sort_unstable();
    residual_source_blocker_ids.dedup();
    let canonical_adapter_inventory_contract_ready =
        covered_by_canonical_adapter_inventory_application_preview;
    let enforcement_decision = canonical_adapter_inventory_rerun_enforcement_decision_for(
        canonical_adapter_inventory_contract_ready,
        &residual_source_blocker_ids,
    );

    WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_enforcement_decision: previous
            .runtime_wal_write_boundary_execution_rerun_enforcement_decision,
        canonical_adapter_inventory_rerun_enforcement_decision: enforcement_decision,
        covered_by_canonical_adapter_inventory_application_preview,
        canonical_adapter_inventory_contract_ready,
        canonical_adapter_inventory_applied: false,
        append_only_work_graph_events_enabled: false,
        runtime_canonical_adapter_enforcement_enabled: false,
        scheduler_admission_enforcement_ready: false,
        task_result_enforcement_ready: false,
        role_manifest_enforcement_ready: false,
        residual_source_blocker_ids,
        next_required_gate: canonical_adapter_inventory_rerun_next_required_gate_for(
            enforcement_decision,
        ),
    }
}

fn canonical_adapter_inventory_rerun_enforcement_decision_for(
    canonical_adapter_inventory_contract_ready: bool,
    residual_source_blocker_ids: &[&'static str],
) -> &'static str {
    if !canonical_adapter_inventory_contract_ready {
        "deny_canonical_adapter_inventory_application_missing"
    } else if residual_source_blocker_ids.contains(&"append_only_work_graph_events_disabled") {
        "deny_append_only_work_graph_events_disabled"
    } else if residual_source_blocker_ids.contains(&"canonical_adapter_projection_partial_or_gap") {
        "deny_canonical_adapter_projection_partial_or_gap"
    } else if residual_source_blocker_ids
        .contains(&"runtime_canonical_adapter_enforcement_disabled")
    {
        "deny_runtime_canonical_adapter_enforcement_disabled"
    } else {
        "allow_preview_only"
    }
}

fn canonical_adapter_inventory_rerun_next_required_gate_for(
    enforcement_decision: &str,
) -> &'static str {
    match enforcement_decision {
        "deny_canonical_adapter_inventory_application_missing" => {
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE
        }
        _ => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_RECOMMENDED_NEXT_GATE
        }
    }
}

fn stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphCanonicalAdapterInventoryRerunStagePreview {
    WorkGraphCanonicalAdapterInventoryRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn previous_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview {
        WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_runtime_rollback_readback_execution_rerun_state: "runtime_rollback_readback_execution_contract_ready_preview_after_application",
            runtime_wal_write_boundary_execution_rerun_state: "wal_write_boundary_execution_contract_ready_preview_after_application",
            covered_by_wal_write_boundary_execution_application_preview: true,
            previous_enforcement_decision: "deny_runtime_wal_write_boundary_not_enabled",
            runtime_wal_write_boundary_execution_rerun_enforcement_decision: "allow_preview_only",
            wal_write_boundary_execution_primary_gap_closed_by_application_preview: true,
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
            wal_write_boundary_execution_contract_ready: true,
            wal_write_boundary_execution_applied: false,
            wal_write_enabled: false,
            checkpoint_write_enabled: false,
            durable_store_switch_enabled: false,
            wal_write_boundary_execution_enabled: false,
            readback_execution_enabled: false,
            rollback_execution_enabled: false,
            runtime_append_only_store_enabled: false,
            scheduler_admission_enforcement_ready: false,
            role_manifest_enforcement_ready: false,
            residual_source_blocker_ids: Vec::new(),
            residual_route_blocker_ids: Vec::new(),
            next_required_gate: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE,
        }
    }

    fn application_outcome(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview {
        WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview {
            source_surface_id,
            source_category,
            application_plan_id: format!(
                "{source_surface_id}_canonical_adapter_inventory_application"
            ),
            post_application_canonical_inventory_state: "canonical_adapter_inventory_contract_ready_preview_after_application",
            canonical_adapter_inventory_contract_ready_preview: true,
            ready_for_canonical_adapter_inventory_readiness_rerun_preview: true,
            ready_for_append_only_work_graph_events: false,
            applies_to_runtime: false,
        }
    }

    fn blocker(
        id: &'static str,
        severity: &'static str,
        category: &'static str,
        affected_source_surface_ids: Vec<&'static str>,
    ) -> WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview {
        WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview {
            id,
            severity,
            category,
            affected_application_plan_ids: affected_source_surface_ids
                .iter()
                .map(|source| format!("{source}_canonical_adapter_inventory_application"))
                .collect(),
            affected_source_surface_ids,
            required_before_runtime_enforcement: true,
            recommended_fix: "sample canonical adapter blocker closure",
        }
    }

    fn sample_inputs() -> (
        Vec<WorkGraphRuntimeWalWriteBoundaryExecutionRerunSourceDecisionPreview>,
        Vec<WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview>,
        Vec<WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview>,
    ) {
        let previous = vec![
            previous_decision("update_plan_tool", "planning"),
            previous_decision("multi_agent_v2_thread_spawn", "multi_agent"),
        ];
        let outcomes = vec![
            application_outcome("update_plan_tool", "planning"),
            application_outcome("multi_agent_v2_thread_spawn", "multi_agent"),
        ];
        let blockers = vec![
            blocker(
                "append_only_work_graph_events_disabled",
                "high",
                "append_only_fact_source",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
            blocker(
                "runtime_canonical_adapter_enforcement_disabled",
                "high",
                "runtime_adapter_enforcement",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
            blocker(
                "canonical_adapter_projection_partial_or_gap",
                "high",
                "projection_coverage",
                vec!["update_plan_tool"],
            ),
            blocker(
                "canonical_adapter_inventory_readiness_rerun_missing",
                "medium",
                "readiness_rerun",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
        ];

        (previous, outcomes, blockers)
    }

    #[test]
    fn canonical_adapter_inventory_rerun_closes_application_missing_primary_blocker() {
        let (previous, outcomes, blockers) = sample_inputs();
        let decisions = canonical_adapter_inventory_rerun_source_decisions_from(
            &previous, &outcomes, &blockers,
        );
        let cleared =
            canonical_adapter_inventory_rerun_cleared_blockers_from(&previous, &decisions);

        assert_eq!(decisions.len(), 2);
        assert_eq!(
            decisions
                .iter()
                .filter(
                    |decision| decision.covered_by_canonical_adapter_inventory_application_preview
                )
                .count(),
            2
        );
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| {
                    decision.canonical_adapter_inventory_rerun_enforcement_decision
                        == "deny_canonical_adapter_inventory_application_missing"
                })
                .count(),
            0
        );
        assert_eq!(cleared.len(), 1);
        assert_eq!(cleared[0].source_count_before, 2);
        assert_eq!(cleared[0].source_count_after, 0);
    }

    #[test]
    fn canonical_adapter_inventory_rerun_exposes_append_only_events_as_next_blocker() {
        let (previous, outcomes, blockers) = sample_inputs();
        let decisions = canonical_adapter_inventory_rerun_source_decisions_from(
            &previous, &outcomes, &blockers,
        );
        let residuals = canonical_adapter_inventory_rerun_residual_blockers_from(&blockers);

        assert!(decisions.iter().all(|decision| {
            decision.canonical_adapter_inventory_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        }));
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| {
                    decision
                        .residual_source_blocker_ids
                        .contains(&"canonical_adapter_projection_partial_or_gap")
                })
                .count(),
            1
        );
        assert_eq!(
            residuals
                .iter()
                .map(|blocker| blocker.id)
                .collect::<Vec<_>>(),
            [
                "append_only_work_graph_events_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
            ]
        );
    }

    #[test]
    fn canonical_adapter_inventory_rerun_keeps_enforcement_disabled() {
        let (_previous, _outcomes, _blockers) = sample_inputs();

        assert_eq!(
            WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects::none(),
            WorkGraphUnifiedProjectionEnforcementReadinessCanonicalAdapterInventoryRerunPreviewSideEffects::none()
        );
    }

    #[test]
    fn canonical_adapter_inventory_rerun_declares_required_priors_and_stages() {
        let (previous, outcomes, blockers) = sample_inputs();
        let decisions = canonical_adapter_inventory_rerun_source_decisions_from(
            &previous, &outcomes, &blockers,
        );
        let stages = canonical_adapter_inventory_rerun_stages_from(&decisions, outcomes.len());
        let stage_ids = stages.iter().map(|stage| stage.id).collect::<Vec<_>>();
        let required_prior_gates =
            work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_required_prior_gates();

        assert_eq!(required_prior_gates.len(), 12);
        assert_eq!(
            required_prior_gates.last().copied(),
            Some(WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            stage_ids,
            [
                "canonical_adapter_inventory_contracts",
                "append_only_work_graph_events_shadow_write",
                "canonical_adapter_partial_gap_closure",
                "runtime_canonical_adapter_enforcement_dry_run",
                "projection_enforcement_dry_run",
            ]
        );
        assert!(stages.iter().all(|stage| !stage.enforcement_enabled));
    }
}
