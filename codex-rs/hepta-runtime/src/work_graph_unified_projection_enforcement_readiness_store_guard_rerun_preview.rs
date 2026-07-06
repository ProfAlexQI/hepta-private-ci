use serde::Serialize;

use crate::work_graph_store_idempotency_guard_gap_closure_application_preview::WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE;
use crate::work_graph_store_idempotency_guard_gap_closure_application_preview::WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview;
use crate::work_graph_store_idempotency_guard_gap_closure_application_preview::work_graph_store_idempotency_guard_gap_closure_application_required_prior_gates;
use crate::work_graph_store_idempotency_guard_gap_closure_application_preview::work_graph_store_idempotency_guard_gap_closure_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_rerun_preview::WorkGraphProjectionEnforcementRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_rerun_preview::work_graph_unified_projection_enforcement_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_SCHEMA_VERSION:
    &str = "work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub store_guard_application_outcome_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub store_guard_rerun_contract_ready_surface_count: usize,
    pub previous_store_guard_ready_surface_count: usize,
    pub store_guard_rerun_store_guard_ready_surface_count: usize,
    pub previous_store_guard_gap_source_count: usize,
    pub store_guard_gap_source_count_after: usize,
    pub store_guard_application_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphStoreGuardRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphStoreGuardRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphStoreGuardRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphStoreGuardRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_terminal_task_result_enforcement_gap_closure_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreGuardRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_coverage_state: &'static str,
    pub store_guard_rerun_state: &'static str,
    pub covered_by_store_guard_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub store_guard_rerun_enforcement_decision: &'static str,
    pub store_guard_gap_closed_by_application_preview: bool,
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
pub struct WorkGraphStoreGuardRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreGuardRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreGuardRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
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

pub fn hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewReport {
    let previous_decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
    let store_guard_application_outcomes =
        work_graph_store_idempotency_guard_gap_closure_application_source_outcomes();
    let decision_deltas =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let cleared_blockers =
        work_graph_unified_projection_enforcement_store_guard_rerun_cleared_blockers();
    let residual_blockers =
        work_graph_unified_projection_enforcement_store_guard_rerun_residual_blockers();
    let enforcement_stages = work_graph_unified_projection_enforcement_store_guard_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_store_guard_rerun_required_prior_gates(
        );
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let store_guard_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_store_guard_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.store_idempotency_guard_ready)
        .count();
    let store_guard_rerun_store_guard_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.store_idempotency_guard_ready)
        .count();
    let previous_store_guard_gap_source_count = previous_decisions
        .iter()
        .filter(|decision| !decision.store_idempotency_guard_ready)
        .count();
    let store_guard_gap_source_count_after = decision_deltas
        .iter()
        .filter(|decision| !decision.store_idempotency_guard_ready)
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.store_guard_rerun_enforcement_decision == "allow_preview_only")
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_SCHEMA_VERSION,
        preview_mode: "read_only_projection_enforcement_readiness_store_guard_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        store_guard_application_outcome_count: store_guard_application_outcomes.len(),
        previous_contract_ready_surface_count,
        store_guard_rerun_contract_ready_surface_count,
        previous_store_guard_ready_surface_count,
        store_guard_rerun_store_guard_ready_surface_count,
        previous_store_guard_gap_source_count,
        store_guard_gap_source_count_after,
        store_guard_application_source_count: store_guard_application_outcomes.len(),
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_terminal_task_result_enforcement_gap_closure_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions()
-> Vec<WorkGraphStoreGuardRerunSourceDecisionPreview> {
    let outcomes = work_graph_store_idempotency_guard_gap_closure_application_source_outcomes();
    work_graph_unified_projection_enforcement_rerun_source_decisions()
        .into_iter()
        .map(|decision| store_guard_rerun_source_decision(decision, &outcomes))
        .collect()
}

pub fn work_graph_unified_projection_enforcement_store_guard_rerun_cleared_blockers()
-> Vec<WorkGraphStoreGuardRerunClearedBlockerPreview> {
    let previous_decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
    let rerun_decisions =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| !decision.store_idempotency_guard_ready)
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_count = rerun_decisions
        .iter()
        .filter(|decision| !decision.store_idempotency_guard_ready)
        .count();

    vec![WorkGraphStoreGuardRerunClearedBlockerPreview {
        id: "store_idempotency_guards_missing_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_count,
        cleared_source_surface_ids: before_sources,
        closure_gate_id: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_store_guard_rerun_residual_blockers()
-> Vec<WorkGraphStoreGuardRerunResidualBlockerPreview> {
    let decisions = work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    vec![
        residual_blocker(
            "projection_adapter_runtime_closure_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                matches!(
                    decision.previous_enforcement_decision,
                    "deny_missing_store_idempotency_guard"
                        | "deny_terminal_task_result_enforcement_disabled"
                        | "deny_append_only_store_disabled"
                ) && decision.previous_coverage_state == "contract_ready_preview_after_application"
            }),
            "keep projection adapter closures preview-only until store guards, terminal TaskResult, and operator-review gates are promoted",
        ),
        residual_blocker(
            "store_guard_runtime_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_store_guard_application_preview
            }),
            "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted",
        ),
        residual_blocker(
            "idempotency_index_mutation_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_store_guard_application_preview
            }),
            "keep idempotency indexes immutable until collision policy and replay evidence are enforced",
        ),
        residual_blocker(
            "state_store_guard_persistence_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_store_guard_application_preview
            }),
            "do not persist candidate guard rows until append-only store intake is promoted",
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

pub fn work_graph_unified_projection_enforcement_store_guard_rerun_stages()
-> Vec<WorkGraphStoreGuardRerunStagePreview> {
    let previous_decisions = work_graph_unified_projection_enforcement_rerun_source_decisions();
    let rerun_decisions =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let store_guard_application_count = rerun_decisions
        .iter()
        .filter(|decision| decision.covered_by_store_guard_application_preview)
        .count();

    vec![
        rerun_stage(
            "unified_projection_contracts",
            previous_decisions.len(),
            previous_decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            rerun_decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            vec!["projection_adapter_runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "projection_adapter_runtime_application",
            7,
            0,
            0,
            vec!["projection_adapter_runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "store_idempotency_guard_contracts",
            previous_decisions.len(),
            previous_decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            rerun_decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            vec!["store_guard_runtime_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "store_guard_runtime_application",
            store_guard_application_count,
            0,
            0,
            vec![
                "store_guard_runtime_application_disabled",
                "idempotency_index_mutation_disabled",
                "state_store_guard_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
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

pub fn work_graph_unified_projection_enforcement_readiness_store_guard_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_store_idempotency_guard_gap_closure_application_required_prior_gates();
    gates.push(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
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

fn store_guard_rerun_source_decision(
    previous: WorkGraphProjectionEnforcementRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphStoreIdempotencyGuardApplicationSourceOutcomePreview],
) -> WorkGraphStoreGuardRerunSourceDecisionPreview {
    let application_outcome = application_outcomes
        .iter()
        .find(|outcome| outcome.source_surface_id == previous.source_surface_id);
    let covered_by_store_guard_application_preview = application_outcome.is_some();
    let store_idempotency_guard_ready = previous.store_idempotency_guard_ready
        || application_outcome
            .map(|outcome| outcome.store_idempotency_guard_ready_preview)
            .unwrap_or(false);
    let residual_route_blocker_ids = if covered_by_store_guard_application_preview {
        previous
            .residual_route_blocker_ids
            .into_iter()
            .filter(|blocker| *blocker != "event_intake_idempotency_guard_missing")
            .collect()
    } else {
        previous.residual_route_blocker_ids.clone()
    };
    let residual_source_blocker_ids = previous.residual_source_blocker_ids.clone();
    let store_guard_rerun_enforcement_decision = store_guard_rerun_enforcement_decision_for(
        previous.unified_store_projection_ready,
        previous.timeline_projection_ready,
        previous.task_result_projection_ready,
        previous.append_only_route_ready,
        store_idempotency_guard_ready,
        previous.readback_probe_contract_ready,
        &residual_route_blocker_ids,
        &residual_source_blocker_ids,
    );

    WorkGraphStoreGuardRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_coverage_state: previous.rerun_coverage_state,
        store_guard_rerun_state: if covered_by_store_guard_application_preview {
            "store_guard_contract_ready_preview_after_application"
        } else if previous.store_idempotency_guard_ready {
            "store_guard_ready_before_application"
        } else {
            "store_guard_missing"
        },
        covered_by_store_guard_application_preview,
        previous_enforcement_decision: previous.rerun_enforcement_decision,
        store_guard_rerun_enforcement_decision,
        store_guard_gap_closed_by_application_preview: covered_by_store_guard_application_preview
            && !previous.store_idempotency_guard_ready
            && store_idempotency_guard_ready,
        projection_contract_ready: previous.projection_contract_ready,
        unified_store_projection_ready: previous.unified_store_projection_ready,
        timeline_projection_ready: previous.timeline_projection_ready,
        task_result_projection_ready: previous.task_result_projection_ready,
        store_idempotency_guard_ready,
        append_only_route_ready: previous.append_only_route_ready,
        readback_probe_contract_ready: previous.readback_probe_contract_ready,
        residual_source_blocker_ids,
        residual_route_blocker_ids,
        next_required_gate: store_guard_rerun_next_required_gate_for(
            store_guard_rerun_enforcement_decision,
        ),
    }
}

fn store_guard_rerun_enforcement_decision_for(
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

fn store_guard_rerun_next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_terminal_task_result_enforcement_disabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE
        }
        "deny_scheduler_admission_not_enforced" => {
            "hepta_work_graph_scheduler_admission_controller_preview_gate"
        }
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_append_only_store_disabled" => {
            "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

fn affected_sources(
    decisions: &[WorkGraphStoreGuardRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphStoreGuardRerunSourceDecisionPreview) -> bool,
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
) -> WorkGraphStoreGuardRerunResidualBlockerPreview {
    WorkGraphStoreGuardRerunResidualBlockerPreview {
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
) -> WorkGraphStoreGuardRerunStagePreview {
    WorkGraphStoreGuardRerunStagePreview {
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
    fn store_guard_rerun_clears_store_idempotency_guard_gap_preview() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.store_guard_application_outcome_count, 5);
        assert_eq!(report.previous_contract_ready_surface_count, 12);
        assert_eq!(report.store_guard_rerun_contract_ready_surface_count, 12);
        assert_eq!(report.previous_store_guard_ready_surface_count, 7);
        assert_eq!(report.store_guard_rerun_store_guard_ready_surface_count, 12);
        assert_eq!(report.previous_store_guard_gap_source_count, 5);
        assert_eq!(report.store_guard_gap_source_count_after, 0);
        assert_eq!(report.rerun_ready_surface_count, 0);
        assert_eq!(report.rerun_blocked_surface_count, 12);
    }

    #[test]
    fn store_guard_rerun_reclassifies_application_covered_sources() {
        let decisions =
            work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
        let plan_mode = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "plan_mode_proposed_plan_blocks")
            .expect("plan mode store guard rerun decision");
        let reducer = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer store guard rerun decision");
        let task_board = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_task_board")
            .expect("task_board store guard rerun decision");

        assert_eq!(
            plan_mode.previous_enforcement_decision,
            "deny_missing_store_idempotency_guard"
        );
        assert_eq!(
            plan_mode.store_guard_rerun_enforcement_decision,
            "deny_append_only_store_disabled"
        );
        assert_eq!(
            reducer.store_guard_rerun_enforcement_decision,
            "deny_terminal_task_result_enforcement_disabled"
        );
        assert_eq!(
            task_board.store_guard_rerun_enforcement_decision,
            "deny_scheduler_admission_not_enforced"
        );
        assert!(
            decisions
                .iter()
                .filter(|decision| decision.covered_by_store_guard_application_preview)
                .all(|decision| decision.store_idempotency_guard_ready
                    && decision.store_guard_gap_closed_by_application_preview)
        );
    }

    #[test]
    fn store_guard_rerun_declares_cleared_blocker_and_residuals() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report();
        let cleared = &report.cleared_blockers[0];
        let blocker_counts = report
            .residual_blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(
            cleared.id,
            "store_idempotency_guards_missing_for_enforcement"
        );
        assert_eq!(cleared.source_count_before, 5);
        assert_eq!(cleared.source_count_after, 0);
        assert_eq!(
            cleared.closure_gate_id,
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE
        );
        assert_eq!(
            blocker_counts,
            [
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("idempotency_index_mutation_disabled", 5),
                ("state_store_guard_persistence_disabled", 5),
                ("terminal_task_result_enforcement_disabled", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("append_only_store_enablement_disabled", 12),
            ]
        );
    }

    #[test]
    fn store_guard_rerun_declares_next_frontier_and_stages() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report();

        assert_eq!(report.enforcement_stage_count, 8);
        assert_eq!(report.required_prior_gate_count, 20);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
    }

    #[test]
    fn store_guard_rerun_keeps_all_enforcement_side_effects_disabled() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedProjectionEnforcementReadinessStoreGuardRerunPreviewSideEffects::none()
        );
        assert!(report.ready_for_terminal_task_result_enforcement_gap_closure_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
    }
}
