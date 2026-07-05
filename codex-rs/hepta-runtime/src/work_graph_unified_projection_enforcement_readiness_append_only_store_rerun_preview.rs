use serde::Serialize;

use crate::work_graph_append_only_store_enablement_precondition_application_preview::WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE;
use crate::work_graph_append_only_store_enablement_precondition_application_preview::WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview;
use crate::work_graph_append_only_store_enablement_precondition_application_preview::WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview;
use crate::work_graph_append_only_store_enablement_precondition_application_preview::work_graph_append_only_store_enablement_precondition_application_outcomes;
use crate::work_graph_append_only_store_enablement_precondition_application_preview::work_graph_append_only_store_enablement_precondition_application_plans;
use crate::work_graph_append_only_store_enablement_precondition_application_preview::work_graph_append_only_store_enablement_precondition_application_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::WorkGraphTerminalTaskResultRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_SCHEMA_VERSION:
    &str = "work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_scheduler_admission_controller_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub append_only_precondition_application_outcome_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub append_only_store_rerun_contract_ready_surface_count: usize,
    pub previous_append_only_store_primary_blocked_surface_count: usize,
    pub append_only_store_primary_blocked_surface_count_after: usize,
    pub append_only_store_precondition_application_source_count: usize,
    pub append_only_store_precondition_ready_surface_count: usize,
    pub append_only_store_runtime_residual_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphAppendOnlyStoreRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphAppendOnlyStoreRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphAppendOnlyStoreRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphAppendOnlyStoreRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_scheduler_admission_gap_closure_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_terminal_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_projection_coverage_state: &'static str,
    pub previous_store_guard_rerun_state: &'static str,
    pub previous_terminal_task_result_rerun_state: &'static str,
    pub append_only_store_rerun_state: &'static str,
    pub covered_by_append_only_store_precondition_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub append_only_store_rerun_enforcement_decision: &'static str,
    pub append_only_store_primary_gap_closed_by_precondition_application_preview: bool,
    pub projection_contract_ready: bool,
    pub unified_store_projection_ready: bool,
    pub timeline_projection_ready: bool,
    pub task_result_projection_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub terminal_task_result_contract_ready: bool,
    pub append_only_route_ready: bool,
    pub append_only_store_precondition_ready: bool,
    pub readback_probe_contract_ready: bool,
    pub scheduler_admission_enforcement_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub runtime_append_only_store_enabled: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
    pub precondition_state_persisted: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub wrapper_executed: bool,
    pub runtime_wrapper_attached: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub closure_applied_to_runtime: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_enablement_precondition_application_outcomes();
    let application_plans =
        work_graph_append_only_store_enablement_precondition_application_plans();
    let decision_deltas =
        work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions();
    let cleared_blockers =
        work_graph_unified_projection_enforcement_append_only_store_rerun_cleared_blockers();
    let residual_blockers =
        work_graph_unified_projection_enforcement_append_only_store_rerun_residual_blockers();
    let enforcement_stages =
        work_graph_unified_projection_enforcement_append_only_store_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_required_prior_gates();
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let append_only_store_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_append_only_store_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.terminal_task_result_rerun_enforcement_decision
                == "deny_append_only_store_disabled"
        })
        .count();
    let append_only_store_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.append_only_store_rerun_enforcement_decision
                == "deny_append_only_store_disabled"
        })
        .count();
    let append_only_store_precondition_application_source_count =
        append_only_store_precondition_application_sources(&application_plans).len();
    let append_only_store_precondition_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.append_only_store_precondition_ready)
        .count();
    let append_only_store_runtime_residual_source_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"append_only_store_runtime_enablement_disabled")
        })
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.append_only_store_rerun_enforcement_decision == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_append_only_store_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        append_only_precondition_application_outcome_count: application_outcomes.len(),
        previous_contract_ready_surface_count,
        append_only_store_rerun_contract_ready_surface_count,
        previous_append_only_store_primary_blocked_surface_count,
        append_only_store_primary_blocked_surface_count_after,
        append_only_store_precondition_application_source_count,
        append_only_store_precondition_ready_surface_count,
        append_only_store_runtime_residual_source_count,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_scheduler_admission_gap_closure_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_terminal_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions()
-> Vec<WorkGraphAppendOnlyStoreRerunSourceDecisionPreview> {
    let application_outcomes =
        work_graph_append_only_store_enablement_precondition_application_outcomes();
    let application_plans =
        work_graph_append_only_store_enablement_precondition_application_plans();
    work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions()
        .into_iter()
        .map(|decision| {
            append_only_store_rerun_source_decision(
                decision,
                &application_outcomes,
                &application_plans,
            )
        })
        .collect()
}

pub fn work_graph_unified_projection_enforcement_append_only_store_rerun_cleared_blockers()
-> Vec<WorkGraphAppendOnlyStoreRerunClearedBlockerPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions();
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision.terminal_task_result_rerun_enforcement_decision
                == "deny_append_only_store_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.append_only_store_rerun_enforcement_decision
                == "deny_append_only_store_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![WorkGraphAppendOnlyStoreRerunClearedBlockerPreview {
        id: "append_only_store_enablement_preconditions_missing_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_sources.len(),
        cleared_source_surface_ids: before_sources,
        closure_gate_id:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_append_only_store_rerun_residual_blockers()
-> Vec<WorkGraphAppendOnlyStoreRerunResidualBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions();
    let application_plans =
        work_graph_append_only_store_enablement_precondition_application_plans();
    let projection_application_sources = affected_sources(&decisions, |decision| {
        decision.previous_projection_coverage_state == "contract_ready_preview_after_application"
    });
    let store_application_sources = affected_sources(&decisions, |decision| {
        decision.previous_store_guard_rerun_state
            == "store_guard_contract_ready_preview_after_application"
    });
    let terminal_application_sources = affected_sources(&decisions, |decision| {
        decision.previous_terminal_task_result_rerun_state
            == "terminal_task_result_contract_ready_preview_after_application"
    });
    let precondition_application_sources =
        append_only_store_precondition_application_sources(&application_plans);
    let operator_review_sources = application_plan_sources(&application_plans, |plan| {
        plan.readback_precondition_id == "operator_review_and_side_effect_lock"
    });
    let scheduler_sources = affected_sources(&decisions, |decision| {
        has_suffix(
            &decision.residual_source_blocker_ids,
            "_admission_not_enforced",
        )
    });
    let role_sources = affected_sources(&decisions, |decision| {
        has_contains(
            &decision.residual_source_blocker_ids,
            "role_manifest_not_enforced",
        )
    });

    vec![
        residual_blocker(
            "projection_adapter_runtime_closure_application_disabled",
            "high",
            projection_application_sources.clone(),
            "keep projection adapter closures preview-only until store guards, terminal TaskResult, and operator-review gates are promoted",
        ),
        residual_blocker(
            "store_guard_runtime_application_disabled",
            "high",
            store_application_sources.clone(),
            "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted",
        ),
        residual_blocker(
            "idempotency_index_mutation_disabled",
            "critical",
            precondition_application_sources.clone(),
            "keep idempotency indexes immutable until mutation policy and replay evidence are enforced",
        ),
        residual_blocker(
            "state_store_guard_persistence_disabled",
            "high",
            store_application_sources.clone(),
            "do not persist candidate guard rows until append-only store intake is promoted",
        ),
        residual_blocker(
            "terminal_task_result_runtime_application_disabled",
            "high",
            terminal_application_sources.clone(),
            "attach terminal TaskResult wrappers to runtime only after persistence, replay, and operator-review gates are promoted",
        ),
        residual_blocker(
            "task_result_persistence_disabled",
            "high",
            terminal_application_sources.clone(),
            "keep TaskResult rows preview-only until append-only store intake is promoted",
        ),
        residual_blocker(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            precondition_application_sources.clone(),
            "keep durable store enablement disabled until WAL, readback, rollback, and operator readiness gates are promoted",
        ),
        residual_blocker(
            "wal_write_boundary_not_enabled",
            "critical",
            precondition_application_sources.clone(),
            "preserve no-WAL boundary until append-only event intake and replay receipts are promoted",
        ),
        residual_blocker(
            "rollback_readback_not_executed",
            "critical",
            precondition_application_sources.clone(),
            "execute rollback and readback gates before any append-only store enablement",
        ),
        residual_blocker(
            "operator_review_required",
            "high",
            operator_review_sources,
            "operator review must accept store enablement, WAL boundary, rollback, and side-effect lock evidence before promotion",
        ),
        residual_blocker(
            "scheduler_admission_not_enforced",
            "high",
            scheduler_sources,
            "make dependency, lease, budget, approval, role, and idempotency checks authoritative before work start",
        ),
        residual_blocker(
            "role_manifest_not_enforced",
            "medium",
            role_sources,
            "bind multi-agent, batch, worker, and handoff sources to role manifests with budgets and tool permissions",
        ),
        residual_blocker(
            "runtime_application_residuals_not_promoted",
            "high",
            projection_application_sources,
            "promote projection adapter and store guard runtime applications only after readback and operator-review gates are satisfied",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_append_only_store_rerun_stages()
-> Vec<WorkGraphAppendOnlyStoreRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_enablement_precondition_application_outcomes();
    let application_plans =
        work_graph_append_only_store_enablement_precondition_application_plans();
    let projection_sources = affected_sources(&decisions, |decision| {
        decision.previous_projection_coverage_state == "contract_ready_preview_after_application"
    });
    let store_sources = affected_sources(&decisions, |decision| {
        decision.previous_store_guard_rerun_state
            == "store_guard_contract_ready_preview_after_application"
    });
    let terminal_sources = affected_sources(&decisions, |decision| {
        decision.previous_terminal_task_result_rerun_state
            == "terminal_task_result_contract_ready_preview_after_application"
    });
    let precondition_sources =
        append_only_store_precondition_application_sources(&application_plans);
    let scheduler_sources = affected_sources(&decisions, |decision| {
        has_suffix(
            &decision.residual_source_blocker_ids,
            "_admission_not_enforced",
        )
    });
    let role_sources = affected_sources(&decisions, |decision| {
        has_contains(
            &decision.residual_source_blocker_ids,
            "role_manifest_not_enforced",
        )
    });

    vec![
        stage(
            "unified_projection_contracts",
            decisions.len(),
            decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            vec!["projection_adapter_runtime_closure_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "projection_adapter_runtime_application",
            projection_sources.len(),
            0,
            0,
            vec![
                "projection_adapter_runtime_closure_application_disabled",
                "runtime_application_residuals_not_promoted",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "store_idempotency_guard_contracts",
            decisions.len(),
            decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            decisions
                .iter()
                .filter(|decision| decision.store_idempotency_guard_ready)
                .count(),
            vec!["store_guard_runtime_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "store_guard_runtime_application",
            store_sources.len(),
            0,
            0,
            vec![
                "store_guard_runtime_application_disabled",
                "idempotency_index_mutation_disabled",
                "state_store_guard_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "terminal_task_result_contracts",
            terminal_sources.len(),
            terminal_sources.len(),
            terminal_sources.len(),
            vec![
                "terminal_task_result_runtime_application_disabled",
                "task_result_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "terminal_task_result_runtime_application",
            terminal_sources.len(),
            0,
            0,
            vec![
                "terminal_task_result_runtime_application_disabled",
                "operator_review_required",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "append_only_store_preconditions",
            application_outcomes.len(),
            0,
            application_outcomes
                .iter()
                .filter(|outcome| outcome.precondition_contract_ready_preview)
                .count(),
            vec![
                "append_only_store_runtime_enablement_disabled",
                "wal_write_boundary_not_enabled",
                "rollback_readback_not_executed",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "append_only_store_runtime_enablement",
            precondition_sources.len(),
            0,
            0,
            vec![
                "append_only_store_runtime_enablement_disabled",
                "wal_write_boundary_not_enabled",
                "idempotency_index_mutation_disabled",
                "rollback_readback_not_executed",
                "operator_review_required",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "scheduler_admission_contracts",
            scheduler_sources.len(),
            0,
            0,
            vec!["scheduler_admission_not_enforced"],
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
        ),
        stage(
            "role_manifest_contracts",
            role_sources.len(),
            0,
            0,
            vec!["role_manifest_not_enforced"],
            "hepta_work_graph_role_manifest_contract_preview_gate",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_enablement_precondition_application_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRerunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
            precondition_state_persisted: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            wrapper_executed: false,
            runtime_wrapper_attached: false,
            role_manifest_enforcement_enabled: false,
            timeline_persisted: false,
            readback_executed: false,
            rollback_executed: false,
            closure_applied_to_runtime: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn append_only_store_rerun_source_decision(
    previous: WorkGraphTerminalTaskResultRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview],
    application_plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
) -> WorkGraphAppendOnlyStoreRerunSourceDecisionPreview {
    let covered_by_append_only_store_precondition_application_preview =
        application_plans.iter().any(|plan| {
            plan.affected_source_surface_ids
                .contains(&previous.source_surface_id)
                && plan.readback_verified_by_preview
                && !plan.applies_to_runtime
        });
    let append_only_store_precondition_ready =
        covered_by_append_only_store_precondition_application_preview
            && application_outcomes
                .iter()
                .all(|outcome| outcome.precondition_contract_ready_preview);
    let append_only_store_primary_gap_closed_by_precondition_application_preview = previous
        .terminal_task_result_rerun_enforcement_decision
        == "deny_append_only_store_disabled"
        && append_only_store_precondition_ready;
    let mut residual_route_blocker_ids = previous
        .residual_route_blocker_ids
        .into_iter()
        .filter(|blocker| *blocker != "append_only_store_disabled_by_design")
        .collect::<Vec<_>>();
    if !append_only_store_precondition_ready {
        push_unique(
            &mut residual_route_blocker_ids,
            "append_only_store_precondition_contract_missing",
        );
    }
    let mut residual_source_blocker_ids = previous.residual_source_blocker_ids.clone();
    push_unique(
        &mut residual_source_blocker_ids,
        "append_only_store_runtime_enablement_disabled",
    );
    push_unique(
        &mut residual_source_blocker_ids,
        "wal_write_boundary_not_enabled",
    );
    push_unique(
        &mut residual_source_blocker_ids,
        "rollback_readback_not_executed",
    );

    let append_only_store_rerun_enforcement_decision =
        append_only_store_rerun_enforcement_decision_for(
            previous.unified_store_projection_ready,
            previous.timeline_projection_ready,
            previous.task_result_projection_ready,
            previous.append_only_route_ready,
            previous.store_idempotency_guard_ready,
            previous.terminal_task_result_contract_ready,
            append_only_store_precondition_ready,
            previous.readback_probe_contract_ready,
            &residual_route_blocker_ids,
            &residual_source_blocker_ids,
        );

    WorkGraphAppendOnlyStoreRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_projection_coverage_state: previous.previous_projection_coverage_state,
        previous_store_guard_rerun_state: previous.previous_store_guard_rerun_state,
        previous_terminal_task_result_rerun_state: previous.terminal_task_result_rerun_state,
        append_only_store_rerun_state: if append_only_store_precondition_ready {
            "append_only_store_precondition_contract_ready_preview_after_application"
        } else {
            "append_only_store_precondition_missing"
        },
        covered_by_append_only_store_precondition_application_preview,
        previous_enforcement_decision: previous.terminal_task_result_rerun_enforcement_decision,
        append_only_store_rerun_enforcement_decision,
        append_only_store_primary_gap_closed_by_precondition_application_preview,
        projection_contract_ready: previous.projection_contract_ready,
        unified_store_projection_ready: previous.unified_store_projection_ready,
        timeline_projection_ready: previous.timeline_projection_ready,
        task_result_projection_ready: previous.task_result_projection_ready,
        store_idempotency_guard_ready: previous.store_idempotency_guard_ready,
        terminal_task_result_contract_ready: previous.terminal_task_result_contract_ready,
        append_only_route_ready: previous.append_only_route_ready,
        append_only_store_precondition_ready,
        readback_probe_contract_ready: previous.readback_probe_contract_ready,
        scheduler_admission_enforcement_ready: false,
        role_manifest_enforcement_ready: false,
        runtime_append_only_store_enabled: false,
        residual_source_blocker_ids,
        residual_route_blocker_ids,
        next_required_gate: append_only_store_rerun_next_required_gate_for(
            append_only_store_rerun_enforcement_decision,
        ),
    }
}

fn append_only_store_rerun_enforcement_decision_for(
    unified_store_projection_ready: bool,
    timeline_projection_ready: bool,
    task_result_projection_ready: bool,
    append_only_route_ready: bool,
    store_idempotency_guard_ready: bool,
    terminal_task_result_contract_ready: bool,
    append_only_store_precondition_ready: bool,
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
    } else if !append_only_store_precondition_ready {
        "deny_append_only_store_precondition_missing"
    } else if !readback_probe_contract_ready {
        "deny_missing_readback_probe"
    } else if has_suffix(residual_source_blocker_ids, "_admission_not_enforced") {
        "deny_scheduler_admission_not_enforced"
    } else if has_contains(residual_source_blocker_ids, "role_manifest_not_enforced") {
        "deny_role_manifest_not_enforced"
    } else if residual_source_blocker_ids.contains(&"append_only_store_runtime_enablement_disabled")
    {
        "deny_runtime_append_only_store_enablement_disabled"
    } else if residual_route_blocker_ids.contains(&"append_only_store_disabled_by_design") {
        "deny_append_only_store_disabled"
    } else {
        "allow_preview_only"
    }
}

fn append_only_store_rerun_next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_scheduler_admission_not_enforced" => {
            "hepta_work_graph_scheduler_admission_controller_preview_gate"
        }
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_runtime_append_only_store_enablement_disabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

fn affected_sources(
    decisions: &[WorkGraphAppendOnlyStoreRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStoreRerunSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn application_plan_sources(
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut sources = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        for source in &plan.affected_source_surface_ids {
            push_unique(&mut sources, source);
        }
    }
    sources
}

fn append_only_store_precondition_application_sources(
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
) -> Vec<&'static str> {
    application_plan_sources(plans, |_| true)
}

fn residual_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreRerunResidualBlockerPreview {
    WorkGraphAppendOnlyStoreRerunResidualBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
    next_gate: &'static str,
) -> WorkGraphAppendOnlyStoreRerunStagePreview {
    WorkGraphAppendOnlyStoreRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate,
    }
}

fn push_unique(values: &mut Vec<&'static str>, value: &'static str) {
    if !values.contains(&value) {
        values.push(value);
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
    fn append_only_store_rerun_reclassifies_primary_append_only_blocker() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.append_only_precondition_application_outcome_count, 7);
        assert_eq!(report.previous_contract_ready_surface_count, 12);
        assert_eq!(
            report.append_only_store_rerun_contract_ready_surface_count,
            12
        );
        assert_eq!(
            report.previous_append_only_store_primary_blocked_surface_count,
            6
        );
        assert_eq!(
            report.append_only_store_primary_blocked_surface_count_after,
            0
        );
        assert_eq!(
            report.append_only_store_precondition_application_source_count,
            12
        );
        assert_eq!(
            report.append_only_store_precondition_ready_surface_count,
            12
        );
        assert_eq!(report.append_only_store_runtime_residual_source_count, 12);
        assert_eq!(report.rerun_ready_surface_count, 0);
        assert_eq!(report.rerun_blocked_surface_count, 12);
    }

    #[test]
    fn append_only_store_rerun_exposes_scheduler_role_and_runtime_enablement() {
        let decisions =
            work_graph_unified_projection_enforcement_append_only_store_rerun_source_decisions();
        let reducer = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer append-only store rerun decision");
        let thread_spawn = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "multi_agent_v2_thread_spawn")
            .expect("thread spawn append-only store rerun decision");
        let harness = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_agent_harness")
            .expect("agent harness append-only store rerun decision");

        assert_eq!(
            reducer.previous_enforcement_decision,
            "deny_append_only_store_disabled"
        );
        assert_eq!(
            reducer.append_only_store_rerun_enforcement_decision,
            "deny_runtime_append_only_store_enablement_disabled"
        );
        assert_eq!(
            thread_spawn.append_only_store_rerun_enforcement_decision,
            "deny_scheduler_admission_not_enforced"
        );
        assert_eq!(
            harness.append_only_store_rerun_enforcement_decision,
            "deny_role_manifest_not_enforced"
        );
        assert!(decisions.iter().all(|decision| {
            decision.append_only_store_precondition_ready
                && !decision
                    .residual_route_blocker_ids
                    .contains(&"append_only_store_disabled_by_design")
        }));
        assert_eq!(
            decisions
                .iter()
                .filter(
                    |decision| decision.append_only_store_rerun_enforcement_decision
                        == "deny_runtime_append_only_store_enablement_disabled"
                )
                .count(),
            6
        );
    }

    #[test]
    fn append_only_store_rerun_declares_cleared_blocker_and_residuals() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_report();
        let cleared = &report.cleared_blockers[0];
        let blocker_counts = report
            .residual_blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(
            cleared.id,
            "append_only_store_enablement_preconditions_missing_for_enforcement"
        );
        assert_eq!(cleared.source_count_before, 6);
        assert_eq!(cleared.source_count_after, 0);
        assert_eq!(
            cleared.closure_gate_id,
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE
        );
        assert_eq!(
            blocker_counts,
            [
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("idempotency_index_mutation_disabled", 12),
                ("state_store_guard_persistence_disabled", 5),
                ("terminal_task_result_runtime_application_disabled", 6),
                ("task_result_persistence_disabled", 6),
                ("append_only_store_runtime_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("runtime_application_residuals_not_promoted", 7),
            ]
        );
    }

    #[test]
    fn append_only_store_rerun_declares_next_frontier_and_stages() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_report();

        assert_eq!(report.enforcement_stage_count, 10);
        assert_eq!(report.required_prior_gate_count, 31);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RERUN_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
    }

    #[test]
    fn append_only_store_rerun_preserves_no_side_effect_boundary() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_report();
        let side_effects = report.side_effects;

        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.store_guard_attached);
        assert!(!side_effects.precondition_state_persisted);
        assert!(!side_effects.append_only_store_enabled);
        assert!(!side_effects.projection_enforcement_enabled);
        assert!(!side_effects.scheduler_admission_enforced);
        assert!(!side_effects.task_result_enforcement_enabled);
        assert!(!side_effects.task_result_persisted);
        assert!(!side_effects.wrapper_executed);
        assert!(!side_effects.runtime_wrapper_attached);
        assert!(!side_effects.role_manifest_enforcement_enabled);
        assert!(!side_effects.timeline_persisted);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.closure_applied_to_runtime);
        assert!(!side_effects.approval_recorded);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.agent_spawn_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
    }
}
