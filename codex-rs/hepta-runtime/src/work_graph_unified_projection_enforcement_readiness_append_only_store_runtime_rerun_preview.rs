use serde::Serialize;

use crate::work_graph_append_only_store_runtime_enablement_application_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::work_graph_append_only_store_runtime_enablement_application_blockers;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::work_graph_append_only_store_runtime_enablement_application_required_prior_gates;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::work_graph_append_only_store_runtime_enablement_application_source_outcomes;
use crate::work_graph_append_only_store_runtime_enablement_application_preview::work_graph_append_only_store_runtime_enablement_stage_applications;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::WorkGraphRoleManifestRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub runtime_application_outcome_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub runtime_rerun_contract_ready_surface_count: usize,
    pub previous_runtime_append_only_primary_blocked_surface_count: usize,
    pub runtime_append_only_primary_blocked_surface_count_after: usize,
    pub runtime_enablement_contract_ready_surface_count: usize,
    pub runtime_append_only_store_enabled_surface_count: usize,
    pub runtime_application_residual_source_count: usize,
    pub operator_review_residual_source_count: usize,
    pub wal_boundary_residual_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphAppendOnlyStoreRuntimeRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphAppendOnlyStoreRuntimeRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_application_promotion_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_role_manifest_rerun_state: &'static str,
    pub append_only_store_runtime_rerun_state: &'static str,
    pub covered_by_runtime_enablement_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub append_only_store_runtime_rerun_enforcement_decision: &'static str,
    pub runtime_append_only_primary_gap_closed_by_application_preview: bool,
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
    pub runtime_application_promotion_ready: bool,
    pub operator_review_ready: bool,
    pub runtime_append_only_store_enabled: bool,
    pub scheduler_admission_enforcement_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub lane_lease_acquired: bool,
    pub work_started: bool,
    pub budget_consumed: bool,
    pub approval_recorded: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub tool_permission_changed: bool,
    pub role_budget_consumed: bool,
    pub role_lane_binding_mutated: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_store_runtime_enablement_application_source_outcomes();
    let decision_deltas =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    let cleared_blockers =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_cleared_blockers(
        );
    let residual_blockers =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_residual_blockers(
        );
    let enforcement_stages =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_required_prior_gates();
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let runtime_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_runtime_append_only_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.role_manifest_rerun_enforcement_decision
                == "deny_runtime_append_only_store_enablement_disabled"
        })
        .count();
    let runtime_append_only_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.append_only_store_runtime_rerun_enforcement_decision
                == "deny_runtime_append_only_store_enablement_disabled"
        })
        .count();
    let runtime_enablement_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.append_only_store_runtime_enablement_ready)
        .count();
    let runtime_append_only_store_enabled_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.runtime_append_only_store_enabled)
        .count();
    let runtime_application_residual_source_count =
        affected_sources(&decision_deltas, |decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"runtime_application_residuals_not_promoted")
        })
        .len();
    let operator_review_residual_source_count = affected_sources(&decision_deltas, |decision| {
        decision
            .residual_source_blocker_ids
            .contains(&"operator_review_required")
    })
    .len();
    let wal_boundary_residual_source_count = affected_sources(&decision_deltas, |decision| {
        decision
            .residual_source_blocker_ids
            .contains(&"wal_write_boundary_not_enabled")
    })
    .len();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision.append_only_store_runtime_rerun_enforcement_decision == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_SCHEMA_VERSION,
        preview_mode: "read_only_projection_enforcement_readiness_append_only_store_runtime_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        runtime_application_outcome_count: application_outcomes.len(),
        previous_contract_ready_surface_count,
        runtime_rerun_contract_ready_surface_count,
        previous_runtime_append_only_primary_blocked_surface_count,
        runtime_append_only_primary_blocked_surface_count_after,
        runtime_enablement_contract_ready_surface_count,
        runtime_append_only_store_enabled_surface_count,
        runtime_application_residual_source_count,
        operator_review_residual_source_count,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_application_promotion_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview> {
    let application_outcomes =
        work_graph_append_only_store_runtime_enablement_application_source_outcomes();
    let application_blockers =
        work_graph_append_only_store_runtime_enablement_application_blockers();
    work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions()
        .into_iter()
        .map(|decision| {
            append_only_store_runtime_rerun_source_decision(
                decision,
                &application_outcomes,
                &application_blockers,
            )
        })
        .collect()
}

pub fn work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_cleared_blockers()
-> Vec<WorkGraphAppendOnlyStoreRuntimeRerunClearedBlockerPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision.role_manifest_rerun_enforcement_decision
                == "deny_runtime_append_only_store_enablement_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.append_only_store_runtime_rerun_enforcement_decision
                == "deny_runtime_append_only_store_enablement_disabled"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![WorkGraphAppendOnlyStoreRuntimeRerunClearedBlockerPreview {
        id: "append_only_store_runtime_enablement_disabled_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_sources.len(),
        cleared_source_surface_ids: before_sources,
        closure_gate_id: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_residual_blockers()
-> Vec<WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview> {
    work_graph_append_only_store_runtime_enablement_application_blockers()
        .into_iter()
        .filter(|blocker| !is_cleared_runtime_application_blocker(blocker.id))
        .map(residual_blocker_from_application_blocker)
        .collect()
}

pub fn work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_stages()
-> Vec<WorkGraphAppendOnlyStoreRuntimeRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    let outcomes = work_graph_append_only_store_runtime_enablement_application_source_outcomes();
    let stage_applications = work_graph_append_only_store_runtime_enablement_stage_applications();
    let projection_store_task_result_sources = residual_union_sources(
        &[
            "projection_adapter_runtime_closure_application_disabled",
            "store_guard_runtime_application_disabled",
            "terminal_task_result_runtime_application_disabled",
        ],
        &decisions,
    );
    let scheduler_role_sources = residual_union_sources(
        &[
            "scheduler_admission_runtime_application_disabled",
            "role_manifest_runtime_application_disabled",
        ],
        &decisions,
    );
    let mut stages = vec![stage(
        "append_only_store_runtime_enablement_contracts",
        decisions.len(),
        0,
        outcomes
            .iter()
            .filter(|outcome| outcome.runtime_enablement_contract_ready_preview)
            .count(),
        vec![
            "durable_store_runtime_switch_disabled",
            "wal_write_boundary_not_enabled",
            "idempotency_index_mutation_disabled",
            "rollback_readback_not_executed",
        ],
    )];
    for stage_application in stage_applications {
        stages.push(stage(
            stage_application.runtime_stage_id,
            stage_application.affected_source_surface_ids.len(),
            0,
            stage_application.affected_source_surface_ids.len(),
            runtime_stage_hard_blockers(stage_application.runtime_stage_id),
        ));
    }
    stages.push(stage(
        "projection_store_task_result_runtime_application",
        projection_store_task_result_sources.len(),
        0,
        0,
        vec![
            "projection_adapter_runtime_closure_application_disabled",
            "store_guard_runtime_application_disabled",
            "terminal_task_result_runtime_application_disabled",
        ],
    ));
    stages.push(stage(
        "scheduler_role_runtime_application",
        scheduler_role_sources.len(),
        0,
        0,
        vec![
            "scheduler_admission_runtime_application_disabled",
            "role_manifest_runtime_application_disabled",
        ],
    ));
    stages
}

pub fn work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_enablement_application_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessAppendOnlyStoreRuntimeRerunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            lane_lease_acquired: false,
            work_started: false,
            budget_consumed: false,
            approval_recorded: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            role_manifest_enforcement_enabled: false,
            tool_permission_changed: false,
            role_budget_consumed: false,
            role_lane_binding_mutated: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn append_only_store_runtime_rerun_source_decision(
    previous: WorkGraphRoleManifestRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview],
    application_blockers: &[WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview],
) -> WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview {
    let covered_by_runtime_enablement_application_preview =
        application_outcomes.iter().any(|outcome| {
            outcome.source_surface_id == previous.source_surface_id
                && outcome.runtime_enablement_contract_ready_preview
                && !outcome.applies_to_runtime
        });
    let append_only_store_runtime_enablement_ready =
        covered_by_runtime_enablement_application_preview
            || !previous
                .residual_source_blocker_ids
                .contains(&"append_only_store_runtime_enablement_disabled");
    let runtime_append_only_primary_gap_closed_by_application_preview = previous
        .role_manifest_rerun_enforcement_decision
        == "deny_runtime_append_only_store_enablement_disabled"
        && append_only_store_runtime_enablement_ready;
    let mut residual_source_blocker_ids = previous
        .residual_source_blocker_ids
        .into_iter()
        .filter(|blocker| *blocker != "append_only_store_runtime_enablement_disabled")
        .collect::<Vec<_>>();
    for blocker in application_blockers.iter().filter(|blocker| {
        blocker
            .affected_source_surface_ids
            .contains(&previous.source_surface_id)
            && !is_cleared_runtime_application_blocker(blocker.id)
    }) {
        push_unique(&mut residual_source_blocker_ids, blocker.id);
    }
    let runtime_application_promotion_ready =
        !residual_source_blocker_ids.contains(&"runtime_application_residuals_not_promoted");
    let operator_review_ready = !residual_source_blocker_ids.contains(&"operator_review_required");
    let append_only_store_runtime_rerun_enforcement_decision =
        append_only_store_runtime_rerun_enforcement_decision_for(
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
            append_only_store_runtime_enablement_ready,
            &previous.residual_route_blocker_ids,
            &residual_source_blocker_ids,
        );

    WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_role_manifest_rerun_state: previous.role_manifest_rerun_state,
        append_only_store_runtime_rerun_state: if covered_by_runtime_enablement_application_preview
        {
            "runtime_enablement_contract_ready_preview_after_application"
        } else {
            "runtime_enablement_not_required_for_source"
        },
        covered_by_runtime_enablement_application_preview,
        previous_enforcement_decision: previous.role_manifest_rerun_enforcement_decision,
        append_only_store_runtime_rerun_enforcement_decision,
        runtime_append_only_primary_gap_closed_by_application_preview,
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
        append_only_store_runtime_enablement_ready,
        runtime_application_promotion_ready,
        operator_review_ready,
        runtime_append_only_store_enabled: false,
        scheduler_admission_enforcement_ready: false,
        role_manifest_enforcement_ready: false,
        residual_source_blocker_ids,
        residual_route_blocker_ids: previous.residual_route_blocker_ids,
        next_required_gate: append_only_store_runtime_rerun_next_required_gate_for(
            append_only_store_runtime_rerun_enforcement_decision,
        ),
    }
}

fn append_only_store_runtime_rerun_enforcement_decision_for(
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
    } else if residual_source_blocker_ids.contains(&"runtime_application_residuals_not_promoted") {
        "deny_runtime_application_residuals_not_promoted"
    } else if residual_source_blocker_ids.contains(&"operator_review_required") {
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

fn append_only_store_runtime_rerun_next_required_gate_for(
    enforcement_decision: &str,
) -> &'static str {
    match enforcement_decision {
        "deny_runtime_application_residuals_not_promoted" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_RECOMMENDED_NEXT_GATE
        }
        "deny_operator_review_required" => {
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
        }
        "deny_runtime_append_only_store_write_boundary_disabled" => {
            "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_RECOMMENDED_NEXT_GATE
        }
    }
}

fn is_cleared_runtime_application_blocker(id: &str) -> bool {
    matches!(
        id,
        "append_only_store_runtime_enablement_disabled"
            | "append_only_store_runtime_enablement_readback_missing"
            | "append_only_store_runtime_readiness_rerun_missing"
    )
}

fn residual_blocker_from_application_blocker(
    blocker: WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview,
) -> WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview {
    WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview {
        id: blocker.id,
        severity: blocker.severity,
        affected_source_surface_ids: blocker.affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix: blocker.recommended_fix,
    }
}

fn runtime_stage_hard_blockers(stage_id: &str) -> Vec<&'static str> {
    match stage_id {
        "durable_store_runtime_switch" => vec!["durable_store_runtime_switch_disabled"],
        "wal_write_boundary" => vec!["wal_write_boundary_not_enabled"],
        "idempotency_mutation_policy" => vec!["idempotency_index_mutation_disabled"],
        "rollback_readback_execution_gate" => {
            vec![
                "readback_execution_disabled",
                "rollback_readback_not_executed",
            ]
        }
        "operator_review_side_effect_lock" => vec!["operator_review_required"],
        "runtime_application_promotion" => vec!["runtime_application_residuals_not_promoted"],
        _ => Vec::new(),
    }
}

fn residual_union_sources(
    blocker_ids: &[&'static str],
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
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
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview) -> bool,
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
) -> WorkGraphAppendOnlyStoreRuntimeRerunStagePreview {
    WorkGraphAppendOnlyStoreRuntimeRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_RECOMMENDED_NEXT_GATE,
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
    fn append_only_store_runtime_rerun_exposes_runtime_application_and_operator_residuals() {
        let decisions =
            work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions();

        assert_eq!(
            decisions
                .iter()
                .filter(|decision| decision.covered_by_runtime_enablement_application_preview)
                .count(),
            12
        );
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| decision
                    .runtime_append_only_primary_gap_closed_by_application_preview)
                .count(),
            12
        );
        assert_eq!(
            decisions
                .iter()
                .filter(
                    |decision| decision.append_only_store_runtime_rerun_enforcement_decision
                        == "deny_runtime_application_residuals_not_promoted"
                )
                .count(),
            7
        );
        assert_eq!(
            decisions
                .iter()
                .filter(
                    |decision| decision.append_only_store_runtime_rerun_enforcement_decision
                        == "deny_operator_review_required"
                )
                .count(),
            5
        );
        assert!(decisions.iter().all(|decision| {
            !decision
                .residual_source_blocker_ids
                .contains(&"append_only_store_runtime_enablement_disabled")
                && !decision.runtime_append_only_store_enabled
        }));
    }

    #[test]
    fn append_only_store_runtime_rerun_preserves_no_mutation_residuals() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_report();
        let blocker_counts = report
            .residual_blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(
            report.previous_runtime_append_only_primary_blocked_surface_count,
            12
        );
        assert_eq!(
            report.runtime_append_only_primary_blocked_surface_count_after,
            0
        );
        assert_eq!(report.runtime_enablement_contract_ready_surface_count, 12);
        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(report.cleared_blockers[0].source_count_before, 12);
        assert_eq!(report.cleared_blockers[0].source_count_after, 0);
        assert_eq!(report.residual_blocker_count, 12);
        assert_eq!(
            blocker_counts,
            [
                ("readback_execution_disabled", 12),
                ("durable_store_runtime_switch_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 7),
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("terminal_task_result_runtime_application_disabled", 6),
                ("scheduler_admission_runtime_application_disabled", 5),
                ("role_manifest_runtime_application_disabled", 4),
                ("runtime_application_residuals_not_promoted", 7),
            ]
        );
        assert_eq!(report.enforcement_stage_count, 9);
        assert_eq!(report.required_prior_gate_count, 43);
    }
}
