use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::WorkGraphEventsReplayReadbackApplicationBlockerPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::work_graph_append_only_work_graph_events_replay_readback_application_blockers;
use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::work_graph_append_only_work_graph_events_replay_readback_application_required_prior_gates;
use crate::work_graph_append_only_work_graph_events_replay_readback_application_preview::work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::WorkGraphEventsShadowWriteRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub replay_readback_outcome_count: usize,
    pub replay_readback_application_covered_source_count: usize,
    pub previous_ready_surface_count: usize,
    pub replay_readback_contract_ready_source_count: usize,
    pub previous_append_only_work_graph_events_primary_blocked_surface_count: usize,
    pub replay_readback_application_missing_surface_count_after: usize,
    pub append_only_work_graph_events_primary_blocked_surface_count: usize,
    pub replay_readback_execution_blocked_surface_count: usize,
    pub partial_or_gap_blocked_surface_count: usize,
    pub append_only_work_graph_events_enabled_source_count: usize,
    pub replay_readback_enabled_source_count: usize,
    pub runtime_canonical_adapter_enforcement_enabled_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphEventsReplayReadbackRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphEventsReplayReadbackRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphEventsReplayReadbackRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphEventsReplayReadbackRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_work_graph_events_persistence_guard_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_enforcement_decision: &'static str,
    pub work_graph_events_replay_readback_rerun_enforcement_decision: &'static str,
    pub covered_by_replay_readback_application_preview: bool,
    pub replay_readback_contract_ready: bool,
    pub replay_readback_application_applied: bool,
    pub append_only_work_graph_events_enabled: bool,
    pub replay_readback_execution_enabled: bool,
    pub runtime_canonical_adapter_enforcement_enabled: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_work_graph_events_replay_readback_application_blockers();
    let decision_deltas = work_graph_events_replay_readback_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    );
    let cleared_blockers = work_graph_events_replay_readback_rerun_cleared_blockers_from(
        &previous_decisions,
        &decision_deltas,
    );
    let residual_blockers =
        work_graph_events_replay_readback_rerun_residual_blockers_from(&application_blockers);
    let enforcement_stages = work_graph_events_replay_readback_rerun_stages_from(
        &decision_deltas,
        application_outcomes.len(),
    );
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_required_prior_gates();

    let previous_append_only_work_graph_events_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.work_graph_events_shadow_write_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .count();
    let replay_readback_application_missing_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.work_graph_events_replay_readback_rerun_enforcement_decision
                == "deny_work_graph_events_replay_readback_application_missing"
        })
        .count();
    let append_only_work_graph_events_primary_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.work_graph_events_replay_readback_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .count();
    let replay_readback_execution_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"replay_readback_execution_disabled")
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
            decision.work_graph_events_replay_readback_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        replay_readback_outcome_count: application_outcomes.len(),
        replay_readback_application_covered_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.covered_by_replay_readback_application_preview)
            .count(),
        previous_ready_surface_count: previous_decisions
            .iter()
            .filter(|decision| {
                decision.work_graph_events_shadow_write_rerun_enforcement_decision
                    == "allow_preview_only"
            })
            .count(),
        replay_readback_contract_ready_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.replay_readback_contract_ready)
            .count(),
        previous_append_only_work_graph_events_primary_blocked_surface_count,
        replay_readback_application_missing_surface_count_after,
        append_only_work_graph_events_primary_blocked_surface_count,
        replay_readback_execution_blocked_surface_count,
        partial_or_gap_blocked_surface_count,
        append_only_work_graph_events_enabled_source_count: 0,
        replay_readback_enabled_source_count: 0,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_work_graph_events_persistence_guard_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_source_decisions()
-> Vec<WorkGraphEventsReplayReadbackRerunSourceDecisionPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_work_graph_events_replay_readback_application_blockers();
    work_graph_events_replay_readback_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    )
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_residual_blockers()
-> Vec<WorkGraphEventsReplayReadbackRerunResidualBlockerPreview> {
    work_graph_events_replay_readback_rerun_residual_blockers_from(
        &work_graph_append_only_work_graph_events_replay_readback_application_blockers(),
    )
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_stages()
-> Vec<WorkGraphEventsReplayReadbackRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_source_decisions();
    work_graph_events_replay_readback_rerun_stages_from(&decisions, decisions.len())
}

pub fn work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_replay_readback_application_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE);
    gates
}

fn work_graph_events_replay_readback_rerun_source_decisions_from(
    previous_decisions: &[WorkGraphEventsShadowWriteRerunSourceDecisionPreview],
    application_outcomes: &[WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphEventsReplayReadbackApplicationBlockerPreview],
) -> Vec<WorkGraphEventsReplayReadbackRerunSourceDecisionPreview> {
    previous_decisions
        .iter()
        .map(|previous| {
            let covered = application_outcomes.iter().any(|outcome| {
                outcome.source_surface_id == previous.source_surface_id
                    && outcome.replay_readback_contract_ready_preview
                    && !outcome.ready_for_replay_readback_execution
            });
            let residual_source_blocker_ids =
                source_blocker_ids(application_blockers, previous.source_surface_id);
            let decision = work_graph_events_replay_readback_rerun_decision(
                covered,
                &residual_source_blocker_ids,
            );

            WorkGraphEventsReplayReadbackRerunSourceDecisionPreview {
                source_surface_id: previous.source_surface_id,
                source_category: previous.source_category,
                previous_enforcement_decision: previous
                    .work_graph_events_shadow_write_rerun_enforcement_decision,
                work_graph_events_replay_readback_rerun_enforcement_decision: decision,
                covered_by_replay_readback_application_preview: covered,
                replay_readback_contract_ready: covered,
                replay_readback_application_applied: false,
                append_only_work_graph_events_enabled: false,
                replay_readback_execution_enabled: false,
                runtime_canonical_adapter_enforcement_enabled: false,
                residual_source_blocker_ids,
                next_required_gate: next_gate_for_decision(decision),
            }
        })
        .collect()
}

fn work_graph_events_replay_readback_rerun_cleared_blockers_from(
    previous_decisions: &[WorkGraphEventsShadowWriteRerunSourceDecisionPreview],
    decision_deltas: &[WorkGraphEventsReplayReadbackRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsReplayReadbackRerunClearedBlockerPreview> {
    let before = previous_decisions
        .iter()
        .filter(|decision| {
            decision.work_graph_events_shadow_write_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.work_graph_events_replay_readback_rerun_enforcement_decision
                == "deny_work_graph_events_replay_readback_application_missing"
        })
        .count();

    vec![WorkGraphEventsReplayReadbackRerunClearedBlockerPreview {
        id: "work_graph_events_replay_readback_application_required_for_enforcement",
        cleared_source_surface_ids: before.clone(),
        source_count_before: before.len(),
        source_count_after: after_count,
        closure_gate_id:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE,
    }]
}

fn work_graph_events_replay_readback_rerun_residual_blockers_from(
    application_blockers: &[WorkGraphEventsReplayReadbackApplicationBlockerPreview],
) -> Vec<WorkGraphEventsReplayReadbackRerunResidualBlockerPreview> {
    application_blockers
        .iter()
        .filter(|blocker| !cleared_application_blocker(blocker.id))
        .map(
            |blocker| WorkGraphEventsReplayReadbackRerunResidualBlockerPreview {
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

fn work_graph_events_replay_readback_rerun_stages_from(
    decision_deltas: &[WorkGraphEventsReplayReadbackRerunSourceDecisionPreview],
    replay_readback_outcome_count: usize,
) -> Vec<WorkGraphEventsReplayReadbackRerunStagePreview> {
    let partial_gap_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"canonical_adapter_projection_partial_or_gap")
        })
        .count();

    vec![
        rerun_stage(
            "work_graph_events_replay_readback_contracts",
            replay_readback_outcome_count,
            0,
            decision_deltas
                .iter()
                .filter(|decision| decision.covered_by_replay_readback_application_preview)
                .count(),
            vec!["work_graph_events_replay_readback_readiness_rerun_missing"],
        ),
        rerun_stage(
            "append_only_work_graph_events_persistence",
            decision_deltas.len(),
            0,
            0,
            vec!["append_only_work_graph_events_disabled"],
        ),
        rerun_stage(
            "replay_readback_execution_readiness",
            decision_deltas.len(),
            0,
            0,
            vec!["replay_readback_execution_disabled"],
        ),
        rerun_stage(
            "canonical_adapter_partial_gap_closure",
            partial_gap_count,
            0,
            0,
            vec!["canonical_adapter_projection_partial_or_gap"],
        ),
        rerun_stage(
            "runtime_canonical_adapter_enforcement_dry_run",
            decision_deltas.len(),
            0,
            0,
            vec!["runtime_canonical_adapter_enforcement_disabled"],
        ),
        rerun_stage(
            "projection_enforcement_dry_run",
            decision_deltas.len(),
            0,
            0,
            vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "runtime_canonical_adapter_enforcement_disabled",
            ],
        ),
    ]
}

fn source_blocker_ids(
    application_blockers: &[WorkGraphEventsReplayReadbackApplicationBlockerPreview],
    source_surface_id: &'static str,
) -> Vec<&'static str> {
    application_blockers
        .iter()
        .filter(|blocker| {
            !cleared_application_blocker(blocker.id)
                && blocker
                    .affected_source_surface_ids
                    .contains(&source_surface_id)
        })
        .map(|blocker| blocker.id)
        .collect()
}

fn work_graph_events_replay_readback_rerun_decision(
    contract_ready: bool,
    source_blockers: &[&'static str],
) -> &'static str {
    if !contract_ready {
        "deny_work_graph_events_replay_readback_application_missing"
    } else if source_blockers.contains(&"append_only_work_graph_events_disabled") {
        "deny_append_only_work_graph_events_disabled"
    } else if source_blockers.contains(&"replay_readback_execution_disabled") {
        "deny_replay_readback_execution_disabled"
    } else if source_blockers.contains(&"canonical_adapter_projection_partial_or_gap") {
        "deny_canonical_adapter_projection_partial_or_gap"
    } else if source_blockers.contains(&"runtime_canonical_adapter_enforcement_disabled") {
        "deny_runtime_canonical_adapter_enforcement_disabled"
    } else {
        "allow_preview_only"
    }
}

fn next_gate_for_decision(decision: &'static str) -> &'static str {
    if decision == "deny_work_graph_events_replay_readback_application_missing" {
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE
    } else {
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_RECOMMENDED_NEXT_GATE
    }
}

fn cleared_application_blocker(blocker_id: &'static str) -> bool {
    blocker_id == "work_graph_events_replay_readback_readiness_rerun_missing"
}

fn rerun_stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphEventsReplayReadbackRerunStagePreview {
    WorkGraphEventsReplayReadbackRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

impl
    WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects
{
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_events_replay_readback_rerun_moves_application_missing_to_zero() {
        let decisions = work_graph_events_replay_readback_rerun_source_decisions_from(
            &sample_previous_decisions(),
            &sample_application_outcomes(),
            &sample_application_blockers(),
        );

        assert_eq!(decisions.len(), 2);
        assert!(decisions.iter().all(|decision| {
            decision.covered_by_replay_readback_application_preview
                && decision.replay_readback_contract_ready
                && !decision.replay_readback_application_applied
                && decision.work_graph_events_replay_readback_rerun_enforcement_decision
                    == "deny_append_only_work_graph_events_disabled"
                && decision.next_required_gate
                    == WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_RECOMMENDED_NEXT_GATE
        }));
    }

    #[test]
    fn work_graph_events_replay_readback_rerun_preserves_no_enforcement_boundary() {
        let decisions = work_graph_events_replay_readback_rerun_source_decisions_from(
            &sample_previous_decisions(),
            &sample_application_outcomes(),
            &sample_application_blockers(),
        );

        assert!(decisions.iter().all(|decision| {
            !decision.append_only_work_graph_events_enabled
                && !decision.replay_readback_execution_enabled
                && !decision.runtime_canonical_adapter_enforcement_enabled
        }));
    }

    #[test]
    fn work_graph_events_replay_readback_rerun_tracks_residuals_and_stages() {
        let blockers = work_graph_events_replay_readback_rerun_residual_blockers_from(
            &sample_application_blockers(),
        );
        let decisions = work_graph_events_replay_readback_rerun_source_decisions_from(
            &sample_previous_decisions(),
            &sample_application_outcomes(),
            &sample_application_blockers(),
        );
        let stages = work_graph_events_replay_readback_rerun_stages_from(&decisions, 2);

        assert_eq!(
            blockers
                .iter()
                .map(|blocker| blocker.id)
                .collect::<Vec<_>>(),
            vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled"
            ]
        );
        assert_eq!(stages.len(), 6);
        assert!(stages.iter().all(|stage| !stage.enforcement_enabled));
    }

    #[test]
    fn work_graph_events_replay_readback_rerun_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects::none(),
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsReplayReadbackRerunPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                replay_executed: false,
                readback_executed: false,
                adapter_projection_enforced: false,
                runtime_mutation_performed: false,
                approval_recorded: false,
                side_effect_lock_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_previous_decisions() -> Vec<WorkGraphEventsShadowWriteRerunSourceDecisionPreview> {
        vec![
            sample_previous_decision("update_plan_tool", "planning"),
            sample_previous_decision("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_previous_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsShadowWriteRerunSourceDecisionPreview {
        WorkGraphEventsShadowWriteRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_shadow_write_rerun_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            covered_by_shadow_write_application_preview: true,
            shadow_write_contract_ready: true,
            shadow_write_application_applied: false,
            append_only_work_graph_events_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            scheduler_admission_enforcement_ready: false,
            task_result_enforcement_ready: false,
            role_manifest_enforcement_ready: false,
            residual_source_blocker_ids: vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
            ],
            next_required_gate: "hepta_work_graph_append_only_work_graph_events_replay_readback_preview_gate",
        }
    }

    fn sample_application_outcomes()
    -> Vec<WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview> {
        vec![
            sample_application_outcome("update_plan_tool", "planning"),
            sample_application_outcome("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_application_outcome(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview {
        WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview {
            source_surface_id,
            source_category,
            application_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_replay_readback_application"
            ),
            post_application_replay_readback_state: "work_graph_events_replay_readback_contract_ready_preview_after_application",
            replay_readback_contract_ready_preview: true,
            ready_for_replay_readback_readiness_rerun_preview: true,
            ready_for_append_only_work_graph_events: false,
            ready_for_replay_readback_execution: false,
        }
    }

    fn sample_application_blockers() -> Vec<WorkGraphEventsReplayReadbackApplicationBlockerPreview>
    {
        vec![
            sample_application_blocker(
                "append_only_work_graph_events_disabled",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
            sample_application_blocker(
                "replay_readback_execution_disabled",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
            sample_application_blocker(
                "work_graph_events_replay_readback_readiness_rerun_missing",
                vec!["update_plan_tool", "multi_agent_v2_thread_spawn"],
            ),
        ]
    }

    fn sample_application_blocker(
        id: &'static str,
        affected_source_surface_ids: Vec<&'static str>,
    ) -> WorkGraphEventsReplayReadbackApplicationBlockerPreview {
        WorkGraphEventsReplayReadbackApplicationBlockerPreview {
            id,
            severity: "high",
            category: "test",
            affected_source_surface_ids,
            affected_application_plan_ids: vec!["sample_plan".to_string()],
            required_before_append_only_events: true,
            recommended_fix: "keep preview-only",
        }
    }
}
