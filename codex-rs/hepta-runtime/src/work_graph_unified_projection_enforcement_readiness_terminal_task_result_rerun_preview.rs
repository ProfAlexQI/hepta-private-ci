use serde::Serialize;

use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::work_graph_terminal_task_result_enforcement_gap_closure_application_plans;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::work_graph_terminal_task_result_enforcement_gap_closure_application_required_prior_gates;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::WorkGraphStoreGuardRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_SCHEMA_VERSION:
    &str = "work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_enablement_precondition_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub terminal_task_result_application_outcome_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub terminal_task_result_rerun_contract_ready_surface_count: usize,
    pub previous_terminal_task_result_ready_surface_count: usize,
    pub terminal_task_result_rerun_ready_surface_count: usize,
    pub previous_terminal_task_result_gap_source_count: usize,
    pub terminal_task_result_gap_source_count_after: usize,
    pub terminal_task_result_application_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphTerminalTaskResultRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphTerminalTaskResultRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphTerminalTaskResultRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphTerminalTaskResultRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_store_enablement_precondition_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_terminal_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_projection_coverage_state: &'static str,
    pub previous_store_guard_rerun_state: &'static str,
    pub terminal_task_result_rerun_state: &'static str,
    pub covered_by_terminal_task_result_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub terminal_task_result_rerun_enforcement_decision: &'static str,
    pub terminal_task_result_gap_closed_by_application_preview: bool,
    pub projection_contract_ready: bool,
    pub unified_store_projection_ready: bool,
    pub timeline_projection_ready: bool,
    pub task_result_projection_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub terminal_task_result_contract_ready: bool,
    pub append_only_route_ready: bool,
    pub readback_probe_contract_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub wrapper_executed: bool,
    pub runtime_wrapper_attached: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub closure_applied_to_runtime: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let terminal_task_result_application_outcomes =
        work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes();
    let decision_deltas =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let cleared_blockers =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_cleared_blockers();
    let residual_blockers =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_residual_blockers();
    let enforcement_stages =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_required_prior_gates();
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let terminal_task_result_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_terminal_task_result_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            !decision
                .residual_route_blocker_ids
                .contains(&"terminal_task_result_enforcement_disabled")
        })
        .count();
    let terminal_task_result_rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.terminal_task_result_contract_ready)
        .count();
    let previous_terminal_task_result_gap_source_count =
        previous_decisions.len() - previous_terminal_task_result_ready_surface_count;
    let terminal_task_result_gap_source_count_after = decision_deltas
        .iter()
        .filter(|decision| !decision.terminal_task_result_contract_ready)
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.terminal_task_result_rerun_enforcement_decision == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_terminal_task_result_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        terminal_task_result_application_outcome_count: terminal_task_result_application_outcomes
            .len(),
        previous_contract_ready_surface_count,
        terminal_task_result_rerun_contract_ready_surface_count,
        previous_terminal_task_result_ready_surface_count,
        terminal_task_result_rerun_ready_surface_count,
        previous_terminal_task_result_gap_source_count,
        terminal_task_result_gap_source_count_after,
        terminal_task_result_application_source_count: terminal_task_result_application_outcomes
            .len(),
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_store_enablement_precondition_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_terminal_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions()
-> Vec<WorkGraphTerminalTaskResultRerunSourceDecisionPreview> {
    let outcomes =
        work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes();
    let application_plans =
        work_graph_terminal_task_result_enforcement_gap_closure_application_plans();
    work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions()
        .into_iter()
        .map(|decision| {
            terminal_task_result_rerun_source_decision(decision, &outcomes, &application_plans)
        })
        .collect()
}

pub fn work_graph_unified_projection_enforcement_terminal_task_result_rerun_cleared_blockers()
-> Vec<WorkGraphTerminalTaskResultRerunClearedBlockerPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let rerun_decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_route_blocker_ids
                .contains(&"terminal_task_result_enforcement_disabled")
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_count = rerun_decisions
        .iter()
        .filter(|decision| {
            !decision.terminal_task_result_contract_ready
                || decision
                    .residual_route_blocker_ids
                    .contains(&"terminal_task_result_enforcement_disabled")
        })
        .count();

    vec![WorkGraphTerminalTaskResultRerunClearedBlockerPreview {
        id: "terminal_task_result_enforcement_disabled_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_count,
        cleared_source_surface_ids: before_sources,
        closure_gate_id:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_terminal_task_result_rerun_residual_blockers()
-> Vec<WorkGraphTerminalTaskResultRerunResidualBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();

    vec![
        residual_blocker(
            "projection_adapter_runtime_closure_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.previous_projection_coverage_state
                    == "contract_ready_preview_after_application"
            }),
            "keep projection adapter closures preview-only until store guards, terminal TaskResult, and operator-review gates are promoted",
        ),
        residual_blocker(
            "store_guard_runtime_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                matches!(
                    decision.previous_store_guard_rerun_state,
                    "store_guard_contract_ready_preview_after_application"
                )
            }),
            "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted",
        ),
        residual_blocker(
            "idempotency_index_mutation_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                matches!(
                    decision.previous_store_guard_rerun_state,
                    "store_guard_contract_ready_preview_after_application"
                )
            }),
            "keep idempotency indexes immutable until collision policy and replay evidence are enforced",
        ),
        residual_blocker(
            "state_store_guard_persistence_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                matches!(
                    decision.previous_store_guard_rerun_state,
                    "store_guard_contract_ready_preview_after_application"
                )
            }),
            "do not persist candidate guard rows until append-only store intake is promoted",
        ),
        residual_blocker(
            "terminal_task_result_runtime_application_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_terminal_task_result_application_preview
            }),
            "attach terminal TaskResult wrappers to runtime only after persistence, replay, and operator-review gates are promoted",
        ),
        residual_blocker(
            "task_result_persistence_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.covered_by_terminal_task_result_application_preview
            }),
            "keep TaskResult rows preview-only until append-only store intake is promoted",
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
        residual_blocker(
            "operator_review_required",
            "medium",
            affected_sources(&decisions, |decision| {
                decision.covered_by_terminal_task_result_application_preview
            }),
            "operator review must accept terminal wrapper bindings, evidence contracts, and enforcement routing before promotion",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_terminal_task_result_rerun_stages()
-> Vec<WorkGraphTerminalTaskResultRerunStagePreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions();
    let rerun_decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let terminal_application_count = rerun_decisions
        .iter()
        .filter(|decision| decision.covered_by_terminal_task_result_application_preview)
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "projection_adapter_runtime_application",
            7,
            0,
            0,
            vec!["projection_adapter_runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "store_guard_runtime_application",
            5,
            0,
            0,
            vec![
                "store_guard_runtime_application_disabled",
                "idempotency_index_mutation_disabled",
                "state_store_guard_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "terminal_task_result_contracts",
            terminal_application_count,
            0,
            rerun_decisions
                .iter()
                .filter(|decision| {
                    decision.covered_by_terminal_task_result_application_preview
                        && decision.terminal_task_result_contract_ready
                })
                .count(),
            vec![
                "terminal_task_result_runtime_application_disabled",
                "task_result_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        rerun_stage(
            "terminal_task_result_runtime_application",
            terminal_application_count,
            0,
            0,
            vec![
                "terminal_task_result_runtime_application_disabled",
                "operator_review_required",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_terminal_task_result_enforcement_gap_closure_application_required_prior_gates();
    gates.push(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewSideEffects {
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
            task_result_persisted: false,
            wrapper_executed: false,
            runtime_wrapper_attached: false,
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

fn terminal_task_result_rerun_source_decision(
    previous: WorkGraphStoreGuardRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview],
    application_plans: &[WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview],
) -> WorkGraphTerminalTaskResultRerunSourceDecisionPreview {
    let application_outcome = application_outcomes
        .iter()
        .find(|outcome| outcome.source_surface_id == previous.source_surface_id);
    let application_plan = application_plans
        .iter()
        .find(|plan| plan.source_surface_id == previous.source_surface_id);
    let covered_by_terminal_task_result_application_preview = application_outcome.is_some();
    let previous_terminal_route_blocked = previous
        .residual_route_blocker_ids
        .contains(&"terminal_task_result_enforcement_disabled");
    let terminal_task_result_contract_ready = !previous_terminal_route_blocked
        || application_outcome
            .map(|outcome| outcome.terminal_task_result_contract_ready_preview)
            .unwrap_or(false);
    let residual_route_blocker_ids = if covered_by_terminal_task_result_application_preview {
        previous
            .residual_route_blocker_ids
            .into_iter()
            .filter(|blocker| *blocker != "terminal_task_result_enforcement_disabled")
            .collect()
    } else {
        previous.residual_route_blocker_ids.clone()
    };
    let terminal_source_blocker_ids = application_plan
        .map(|plan| plan.terminal_source_blocker_ids.as_slice())
        .unwrap_or(&[]);
    let residual_source_blocker_ids = if covered_by_terminal_task_result_application_preview {
        previous
            .residual_source_blocker_ids
            .into_iter()
            .filter(|blocker| !terminal_source_blocker_ids.contains(blocker))
            .collect()
    } else {
        previous.residual_source_blocker_ids.clone()
    };
    let terminal_task_result_rerun_enforcement_decision =
        terminal_task_result_rerun_enforcement_decision_for(
            previous.unified_store_projection_ready,
            previous.timeline_projection_ready,
            previous.task_result_projection_ready,
            previous.append_only_route_ready,
            previous.store_idempotency_guard_ready,
            terminal_task_result_contract_ready,
            previous.readback_probe_contract_ready,
            &residual_route_blocker_ids,
            &residual_source_blocker_ids,
        );

    WorkGraphTerminalTaskResultRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_projection_coverage_state: previous.previous_coverage_state,
        previous_store_guard_rerun_state: previous.store_guard_rerun_state,
        terminal_task_result_rerun_state: if covered_by_terminal_task_result_application_preview {
            "terminal_task_result_contract_ready_preview_after_application"
        } else if terminal_task_result_contract_ready {
            "terminal_task_result_ready_before_application"
        } else {
            "terminal_task_result_missing"
        },
        covered_by_terminal_task_result_application_preview,
        previous_enforcement_decision: previous.store_guard_rerun_enforcement_decision,
        terminal_task_result_rerun_enforcement_decision,
        terminal_task_result_gap_closed_by_application_preview:
            covered_by_terminal_task_result_application_preview
                && previous_terminal_route_blocked
                && terminal_task_result_contract_ready,
        projection_contract_ready: previous.projection_contract_ready,
        unified_store_projection_ready: previous.unified_store_projection_ready,
        timeline_projection_ready: previous.timeline_projection_ready,
        task_result_projection_ready: previous.task_result_projection_ready,
        store_idempotency_guard_ready: previous.store_idempotency_guard_ready,
        terminal_task_result_contract_ready,
        append_only_route_ready: previous.append_only_route_ready,
        readback_probe_contract_ready: previous.readback_probe_contract_ready,
        residual_source_blocker_ids,
        residual_route_blocker_ids,
        next_required_gate: terminal_task_result_rerun_next_required_gate_for(
            terminal_task_result_rerun_enforcement_decision,
        ),
    }
}

fn terminal_task_result_rerun_enforcement_decision_for(
    unified_store_projection_ready: bool,
    timeline_projection_ready: bool,
    task_result_projection_ready: bool,
    append_only_route_ready: bool,
    store_idempotency_guard_ready: bool,
    terminal_task_result_contract_ready: bool,
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
    } else if !terminal_task_result_contract_ready {
        "deny_terminal_task_result_contract_missing"
    } else if !readback_probe_contract_ready {
        "deny_missing_readback_probe"
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

fn terminal_task_result_rerun_next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_scheduler_admission_not_enforced" => {
            "hepta_work_graph_scheduler_admission_controller_preview_gate"
        }
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_append_only_store_disabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

fn affected_sources(
    decisions: &[WorkGraphTerminalTaskResultRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultRerunSourceDecisionPreview) -> bool,
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
) -> WorkGraphTerminalTaskResultRerunResidualBlockerPreview {
    WorkGraphTerminalTaskResultRerunResidualBlockerPreview {
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
) -> WorkGraphTerminalTaskResultRerunStagePreview {
    WorkGraphTerminalTaskResultRerunStagePreview {
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
    fn terminal_task_result_rerun_clears_terminal_route_blocker_preview() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.terminal_task_result_application_outcome_count, 6);
        assert_eq!(report.previous_contract_ready_surface_count, 12);
        assert_eq!(
            report.terminal_task_result_rerun_contract_ready_surface_count,
            12
        );
        assert_eq!(report.previous_terminal_task_result_ready_surface_count, 6);
        assert_eq!(report.terminal_task_result_rerun_ready_surface_count, 12);
        assert_eq!(report.previous_terminal_task_result_gap_source_count, 6);
        assert_eq!(report.terminal_task_result_gap_source_count_after, 0);
        assert_eq!(report.rerun_ready_surface_count, 0);
        assert_eq!(report.rerun_blocked_surface_count, 12);
    }

    #[test]
    fn terminal_task_result_rerun_reclassifies_terminal_sources() {
        let decisions =
            work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
        let reducer = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer terminal TaskResult rerun decision");
        let thread_spawn = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "multi_agent_v2_thread_spawn")
            .expect("thread spawn terminal TaskResult rerun decision");
        let harness = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_agent_harness")
            .expect("agent harness terminal TaskResult rerun decision");

        assert_eq!(
            reducer.previous_enforcement_decision,
            "deny_terminal_task_result_enforcement_disabled"
        );
        assert_eq!(
            reducer.terminal_task_result_rerun_enforcement_decision,
            "deny_append_only_store_disabled"
        );
        assert_eq!(
            thread_spawn.terminal_task_result_rerun_enforcement_decision,
            "deny_scheduler_admission_not_enforced"
        );
        assert_eq!(
            harness.terminal_task_result_rerun_enforcement_decision,
            "deny_role_manifest_not_enforced"
        );
        assert!(
            decisions
                .iter()
                .filter(|decision| decision.covered_by_terminal_task_result_application_preview)
                .all(|decision| decision.terminal_task_result_contract_ready
                    && decision.terminal_task_result_gap_closed_by_application_preview
                    && !decision
                        .residual_route_blocker_ids
                        .contains(&"terminal_task_result_enforcement_disabled"))
        );
    }

    #[test]
    fn terminal_task_result_rerun_declares_cleared_blocker_and_residuals() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_report();
        let cleared = &report.cleared_blockers[0];
        let blocker_counts = report
            .residual_blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(
            cleared.id,
            "terminal_task_result_enforcement_disabled_for_enforcement"
        );
        assert_eq!(cleared.source_count_before, 6);
        assert_eq!(cleared.source_count_after, 0);
        assert_eq!(
            cleared.closure_gate_id,
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE
        );
        assert_eq!(
            blocker_counts,
            [
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("idempotency_index_mutation_disabled", 5),
                ("state_store_guard_persistence_disabled", 5),
                ("terminal_task_result_runtime_application_disabled", 6),
                ("task_result_persistence_disabled", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("append_only_store_enablement_disabled", 12),
                ("operator_review_required", 6),
            ]
        );
    }

    #[test]
    fn terminal_task_result_rerun_declares_next_frontier_and_stages() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_report();

        assert_eq!(report.enforcement_stage_count, 9);
        assert_eq!(report.required_prior_gate_count, 27);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
    }

    #[test]
    fn terminal_task_result_rerun_keeps_all_enforcement_side_effects_disabled() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedProjectionEnforcementReadinessTerminalTaskResultRerunPreviewSideEffects::none()
        );
        assert!(report.ready_for_append_only_store_enablement_precondition_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_terminal_task_result_enforcement);
        assert!(!report.ready_for_live_execution);
    }
}
