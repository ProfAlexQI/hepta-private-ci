use serde::Serialize;

use crate::work_graph_projection_adapter_gap_closure_application_preview::WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE;
use crate::work_graph_projection_adapter_gap_closure_application_preview::WorkGraphProjectionAdapterClosureSourceOutcomePreview;
use crate::work_graph_projection_adapter_gap_closure_application_preview::work_graph_projection_adapter_gap_closure_application_required_prior_gates;
use crate::work_graph_projection_adapter_gap_closure_application_preview::work_graph_projection_adapter_gap_closure_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_preview::WorkGraphProjectionEnforcementSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_preview::work_graph_unified_projection_enforcement_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub application_outcome_count: usize,
    pub original_contract_ready_surface_count: usize,
    pub rerun_contract_ready_surface_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub projection_gap_source_count_before: usize,
    pub projection_gap_source_count_after: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphProjectionEnforcementRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphProjectionEnforcementRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphProjectionEnforcementRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphProjectionEnforcementRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_store_idempotency_guard_gap_closure_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub original_coverage_state: &'static str,
    pub rerun_coverage_state: &'static str,
    pub covered_by_application_preview: bool,
    pub original_enforcement_decision: &'static str,
    pub rerun_enforcement_decision: &'static str,
    pub projection_gap_closed_by_application_preview: bool,
    pub projection_contract_ready: bool,
    pub unified_store_projection_ready: bool,
    pub timeline_projection_ready: bool,
    pub task_result_projection_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub append_only_route_ready: bool,
    pub readback_probe_contract_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub closure_applied_to_runtime: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewReport {
    let original_decisions = work_graph_unified_projection_enforcement_source_decisions();
    let application_outcomes =
        work_graph_projection_adapter_gap_closure_application_source_outcomes();
    let decision_deltas = work_graph_unified_projection_enforcement_rerun_source_decisions();
    let cleared_blockers = work_graph_unified_projection_enforcement_rerun_cleared_blockers();
    let residual_blockers = work_graph_unified_projection_enforcement_rerun_residual_blockers();
    let enforcement_stages = work_graph_unified_projection_enforcement_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_rerun_required_prior_gates();
    let original_contract_ready_surface_count = original_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.rerun_enforcement_decision == "allow_preview_only")
        .count();
    let projection_gap_source_count_before = original_decisions
        .iter()
        .filter(|decision| projection_adapter_gap_open(decision))
        .count();
    let projection_gap_source_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            !decision.unified_store_projection_ready
                || !decision.timeline_projection_ready
                || !decision.task_result_projection_ready
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_PREVIEW_GATE,
        schema_version: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_SCHEMA_VERSION,
        preview_mode: "read_only_projection_enforcement_readiness_rerun_no_enforcement",
        source_surface_count: original_decisions.len(),
        application_outcome_count: application_outcomes.len(),
        original_contract_ready_surface_count,
        rerun_contract_ready_surface_count,
        rerun_ready_surface_count,
        rerun_blocked_surface_count: decision_deltas.len() - rerun_ready_surface_count,
        projection_gap_source_count_before,
        projection_gap_source_count_after,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_store_idempotency_guard_gap_closure_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_task_result_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_rerun_source_decisions()
-> Vec<WorkGraphProjectionEnforcementRerunSourceDecisionPreview> {
    let application_outcomes =
        work_graph_projection_adapter_gap_closure_application_source_outcomes();
    work_graph_unified_projection_enforcement_source_decisions()
        .into_iter()
        .map(|decision| rerun_source_decision(decision, &application_outcomes))
        .collect()
}

pub fn work_graph_unified_projection_enforcement_rerun_cleared_blockers()
-> Vec<WorkGraphProjectionEnforcementRerunClearedBlockerPreview> {
    let original_decisions = work_graph_unified_projection_enforcement_source_decisions();
    let rerun_decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
    let before_sources = original_decisions
        .iter()
        .filter(|decision| projection_adapter_gap_open(decision))
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_count = rerun_decisions
        .iter()
        .filter(|decision| {
            !decision.unified_store_projection_ready
                || !decision.timeline_projection_ready
                || !decision.task_result_projection_ready
        })
        .count();

    vec![WorkGraphProjectionEnforcementRerunClearedBlockerPreview {
        id: "projection_adapters_missing_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_count,
        cleared_source_surface_ids: before_sources,
        closure_gate_id: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_rerun_residual_blockers()
-> Vec<WorkGraphProjectionEnforcementRerunResidualBlockerPreview> {
    let decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
    vec![
        residual_blocker(
            "runtime_closure_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_application_preview
            }),
            "attach projection adapter closures to runtime only after operator review, store guards, and persistence gates are promoted",
        ),
        residual_blocker(
            "store_idempotency_guards_missing_for_enforcement",
            "high",
            affected_sources(&decisions, |decision| {
                !decision.store_idempotency_guard_ready
            }),
            "promote idempotency readback adapters into state-store guards before any append-only intake writes",
        ),
        residual_blocker(
            "terminal_task_result_enforcement_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision
                    .residual_route_blocker_ids
                    .contains(&"terminal_task_result_enforcement_disabled")
            }),
            "make every terminal worker, agent, scheduler, and handoff path emit the canonical TaskResult contract",
        ),
        residual_blocker(
            "scheduler_admission_not_enforced",
            "high",
            affected_sources(&decisions, |decision| {
                has_suffix(
                    &decision.residual_source_blocker_ids,
                    "_admission_not_enforced",
                )
            }),
            "make dependency, lease, budget, approval, role, and idempotency checks authoritative before work start",
        ),
        residual_blocker(
            "role_manifest_not_enforced",
            "medium",
            affected_sources(&decisions, |decision| {
                has_contains(
                    &decision.residual_source_blocker_ids,
                    "role_manifest_not_enforced",
                )
            }),
            "bind multi-agent, batch, worker, and handoff sources to role manifests with budgets and tool permissions",
        ),
        residual_blocker(
            "append_only_store_enablement_disabled",
            "medium",
            decisions
                .iter()
                .map(|decision| decision.source_surface_id)
                .collect(),
            "keep projection enforcement disabled until WAL, readback, replay, and operator readiness gates are promoted",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_rerun_stages()
-> Vec<WorkGraphProjectionEnforcementRerunStagePreview> {
    let original_decisions = work_graph_unified_projection_enforcement_source_decisions();
    let rerun_decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();

    vec![
        rerun_stage(
            "unified_projection_contracts",
            original_decisions.len(),
            original_decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            rerun_decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            vec!["runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "projection_adapter_gap_closure_application",
            7,
            0,
            7,
            vec!["runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "store_idempotency_guards",
            rerun_decisions.len(),
            original_decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            rerun_decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            vec!["store_idempotency_guards_missing_for_enforcement"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "terminal_task_result_contracts",
            rerun_decisions
                .iter()
                .filter(|decision| {
                    decision
                        .residual_route_blocker_ids
                        .contains(&"terminal_task_result_enforcement_disabled")
                })
                .count(),
            0,
            0,
            vec!["terminal_task_result_enforcement_disabled"],
            "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
        ),
        rerun_stage(
            "scheduler_admission_contracts",
            rerun_decisions
                .iter()
                .filter(|decision| {
                    has_suffix(
                        &decision.residual_source_blocker_ids,
                        "_admission_not_enforced",
                    )
                })
                .count(),
            0,
            0,
            vec!["scheduler_admission_not_enforced"],
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
        ),
        rerun_stage(
            "role_manifest_contracts",
            rerun_decisions
                .iter()
                .filter(|decision| {
                    has_contains(
                        &decision.residual_source_blocker_ids,
                        "role_manifest_not_enforced",
                    )
                })
                .count(),
            0,
            0,
            vec!["role_manifest_not_enforced"],
            "hepta_work_graph_role_manifest_contract_preview_gate",
        ),
        rerun_stage(
            "append_only_store_enablement",
            rerun_decisions.len(),
            0,
            0,
            vec!["append_only_store_enablement_disabled"],
            "hepta_work_graph_append_only_store_enablement_precondition_preview_gate",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_readiness_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_projection_adapter_gap_closure_application_required_prior_gates();
    gates.push(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            timeline_persisted: false,
            closure_applied_to_runtime: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn rerun_source_decision(
    original: WorkGraphProjectionEnforcementSourceDecisionPreview,
    application_outcomes: &[WorkGraphProjectionAdapterClosureSourceOutcomePreview],
) -> WorkGraphProjectionEnforcementRerunSourceDecisionPreview {
    let application_outcome = application_outcomes
        .iter()
        .find(|outcome| outcome.source_surface_id == original.source_surface_id);
    let covered_by_application_preview = application_outcome.is_some();
    let projection_gap_closed_by_application_preview =
        covered_by_application_preview && projection_adapter_gap_open(&original);
    let projection_contract_ready =
        original.projection_contract_ready || covered_by_application_preview;
    let unified_store_projection_ready = original.unified_store_projection_ready
        || application_outcome
            .map(|outcome| outcome.store_projection_application_required)
            .unwrap_or(false);
    let timeline_projection_ready = original.timeline_projection_ready
        || application_outcome
            .map(|outcome| outcome.timeline_projection_application_required)
            .unwrap_or(false);
    let task_result_projection_ready = original.task_result_projection_ready;
    let residual_source_blocker_ids = if covered_by_application_preview {
        residual_source_blockers(&original.source_blocker_ids)
    } else {
        original.source_blocker_ids.clone()
    };
    let residual_route_blocker_ids = if covered_by_application_preview {
        original
            .route_blocker_ids
            .into_iter()
            .filter(|blocker| *blocker != "source_projection_not_contract_ready")
            .collect()
    } else {
        original.route_blocker_ids.clone()
    };
    let rerun_enforcement_decision = rerun_enforcement_decision_for(
        unified_store_projection_ready,
        timeline_projection_ready,
        task_result_projection_ready,
        original.append_only_route_ready,
        original.store_idempotency_guard_ready,
        original.readback_probe_contract_ready,
        &residual_route_blocker_ids,
        &residual_source_blocker_ids,
    );

    WorkGraphProjectionEnforcementRerunSourceDecisionPreview {
        source_surface_id: original.source_surface_id,
        source_category: original.source_category,
        original_coverage_state: original.coverage_state,
        rerun_coverage_state: if covered_by_application_preview {
            "contract_ready_preview_after_application"
        } else {
            original.coverage_state
        },
        covered_by_application_preview,
        original_enforcement_decision: original.enforcement_decision,
        rerun_enforcement_decision,
        projection_gap_closed_by_application_preview,
        projection_contract_ready,
        unified_store_projection_ready,
        timeline_projection_ready,
        task_result_projection_ready,
        store_idempotency_guard_ready: original.store_idempotency_guard_ready,
        append_only_route_ready: original.append_only_route_ready,
        readback_probe_contract_ready: original.readback_probe_contract_ready,
        residual_source_blocker_ids,
        residual_route_blocker_ids,
        next_required_gate: rerun_next_required_gate_for(rerun_enforcement_decision),
    }
}

fn rerun_enforcement_decision_for(
    unified_store_projection_ready: bool,
    timeline_projection_ready: bool,
    task_result_projection_ready: bool,
    append_only_route_ready: bool,
    store_idempotency_guard_ready: bool,
    readback_probe_contract_ready: bool,
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
    } else if !readback_probe_contract_ready {
        "deny_missing_readback_probe"
    } else if residual_route_blocker_ids.contains(&"terminal_task_result_enforcement_disabled") {
        "deny_terminal_task_result_enforcement_disabled"
    } else if has_suffix(residual_source_blocker_ids, "_admission_not_enforced") {
        "deny_scheduler_admission_not_enforced"
    } else if has_contains(residual_source_blocker_ids, "role_manifest_not_enforced") {
        "deny_role_manifest_not_enforced"
    } else if residual_route_blocker_ids.contains(&"append_only_store_disabled_by_design") {
        "deny_append_only_store_disabled"
    } else {
        "allow_preview_only"
    }
}

fn rerun_next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_missing_unified_store_projection" => {
            "hepta_work_graph_projection_adapter_gap_closure_preview_gate"
        }
        "deny_missing_timeline_projection" => {
            "hepta_work_graph_observability_timeline_preview_gate"
        }
        "deny_missing_task_result_projection" => {
            "hepta_work_graph_task_result_contract_preview_gate"
        }
        "deny_missing_append_only_route" => {
            "hepta_work_graph_append_only_event_intake_preview_gate"
        }
        "deny_missing_store_idempotency_guard" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE
        }
        "deny_missing_readback_probe" => "hepta_work_graph_replay_readback_preview_gate",
        "deny_terminal_task_result_enforcement_disabled" => {
            "hepta_work_graph_terminal_task_result_wrapper_preview_gate"
        }
        "deny_scheduler_admission_not_enforced" => {
            "hepta_work_graph_scheduler_admission_controller_preview_gate"
        }
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_append_only_store_disabled" => {
            "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

fn residual_source_blockers(source_blocker_ids: &[&'static str]) -> Vec<&'static str> {
    source_blocker_ids
        .iter()
        .copied()
        .filter(|blocker| !projection_gap_blocker(blocker))
        .collect()
}

fn projection_adapter_gap_open(
    decision: &WorkGraphProjectionEnforcementSourceDecisionPreview,
) -> bool {
    !decision.unified_store_projection_ready
        || !decision.timeline_projection_ready
        || !decision.task_result_projection_ready
}

fn projection_gap_blocker(blocker: &str) -> bool {
    blocker.contains("projection_missing")
        || blocker.contains("projection_not_enforced")
        || blocker.contains("timeline_adapter_not_enforced")
        || blocker.contains("timeline_projection_missing")
        || blocker.contains("store_projection_missing")
        || blocker.contains("unified_store_projection_missing")
        || blocker.contains("missing_verifier_and_reducer_projection")
        || blocker.contains("missing_task_result_projection")
}

fn affected_sources(
    decisions: &[WorkGraphProjectionEnforcementRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphProjectionEnforcementRerunSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn residual_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphProjectionEnforcementRerunResidualBlockerPreview {
    WorkGraphProjectionEnforcementRerunResidualBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn rerun_stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
    next_gate: &'static str,
) -> WorkGraphProjectionEnforcementRerunStagePreview {
    WorkGraphProjectionEnforcementRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate,
    }
}

fn has_suffix(values: &[&'static str], suffix: &str) -> bool {
    values.iter().any(|value| value.ends_with(suffix))
}

fn has_contains(values: &[&'static str], needle: &str) -> bool {
    values.iter().any(|value| value.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_rerun_closes_projection_adapter_gap_preview() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.application_outcome_count, 7);
        assert_eq!(report.original_contract_ready_surface_count, 5);
        assert_eq!(report.rerun_contract_ready_surface_count, 12);
        assert_eq!(report.projection_gap_source_count_before, 7);
        assert_eq!(report.projection_gap_source_count_after, 0);
        assert_eq!(report.rerun_ready_surface_count, 0);
        assert_eq!(report.rerun_blocked_surface_count, 12);
    }

    #[test]
    fn readiness_rerun_reclassifies_application_covered_sources() {
        let decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
        let task_board = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_task_board")
            .expect("task_board rerun decision");
        let plan_mode = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "plan_mode_proposed_plan_blocks")
            .expect("plan mode rerun decision");
        let approval = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_approval_broker")
            .expect("approval rerun decision");

        assert_eq!(
            task_board.rerun_coverage_state,
            "contract_ready_preview_after_application"
        );
        assert_eq!(
            task_board.original_enforcement_decision,
            "deny_missing_unified_store_projection"
        );
        assert_eq!(
            task_board.rerun_enforcement_decision,
            "deny_missing_store_idempotency_guard"
        );
        assert_eq!(
            plan_mode.rerun_enforcement_decision,
            "deny_missing_store_idempotency_guard"
        );
        assert_eq!(
            approval.rerun_enforcement_decision,
            "deny_append_only_store_disabled"
        );
        assert!(
            decisions
                .iter()
                .filter(|decision| decision.covered_by_application_preview)
                .all(|decision| decision.projection_contract_ready
                    && decision.unified_store_projection_ready
                    && decision.timeline_projection_ready)
        );
    }

    #[test]
    fn readiness_rerun_preserves_residual_blockers() {
        let blockers = work_graph_unified_projection_enforcement_rerun_residual_blockers();
        let blocker_counts = blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_counts,
            [
                ("runtime_closure_application_disabled", 7),
                ("store_idempotency_guards_missing_for_enforcement", 5),
                ("terminal_task_result_enforcement_disabled", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("append_only_store_enablement_disabled", 12),
            ]
        );
        assert!(
            blockers
                .iter()
                .all(|blocker| blocker.required_before_projection_enforcement)
        );
    }

    #[test]
    fn readiness_rerun_declares_cleared_blocker_and_next_frontier() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report();
        let cleared = &report.cleared_blockers[0];

        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(cleared.id, "projection_adapters_missing_for_enforcement");
        assert_eq!(cleared.source_count_before, 7);
        assert_eq!(cleared.source_count_after, 0);
        assert_eq!(
            cleared.closure_gate_id,
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE
        );
        assert_eq!(report.residual_blocker_count, 6);
        assert_eq!(report.required_prior_gate_count, 16);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn readiness_rerun_keeps_all_enforcement_side_effects_disabled() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedProjectionEnforcementReadinessRerunPreviewSideEffects::none()
        );
        assert!(report.ready_for_store_idempotency_guard_gap_closure_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
    }
}
