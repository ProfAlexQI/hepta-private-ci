use serde::Serialize;

use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE;
use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::WorkGraphRoleManifestApplicationPlanPreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::WorkGraphRoleManifestApplicationSourceOutcomePreview;
use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::work_graph_role_manifest_enforcement_gap_closure_application_plans;
use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::work_graph_role_manifest_enforcement_gap_closure_application_required_prior_gates;
use crate::work_graph_role_manifest_enforcement_gap_closure_application_preview::work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::WorkGraphSchedulerAdmissionRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview::work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_PREVIEW_GATE:
    &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_SCHEMA_VERSION:
    &str = "work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub role_manifest_application_outcome_count: usize,
    pub previous_contract_ready_surface_count: usize,
    pub role_manifest_rerun_contract_ready_surface_count: usize,
    pub previous_role_manifest_primary_blocked_surface_count: usize,
    pub role_manifest_primary_blocked_surface_count_after: usize,
    pub role_manifest_application_source_count: usize,
    pub role_manifest_contract_ready_surface_count: usize,
    pub role_manifest_runtime_residual_source_count: usize,
    pub scheduler_admission_runtime_residual_source_count: usize,
    pub runtime_append_only_residual_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphRoleManifestRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphRoleManifestRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphRoleManifestRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphRoleManifestRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_store_runtime_enablement_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestRerunSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_projection_coverage_state: &'static str,
    pub previous_store_guard_rerun_state: &'static str,
    pub previous_terminal_task_result_rerun_state: &'static str,
    pub previous_append_only_store_rerun_state: &'static str,
    pub previous_scheduler_admission_rerun_state: &'static str,
    pub role_manifest_rerun_state: &'static str,
    pub covered_by_role_manifest_application_preview: bool,
    pub previous_enforcement_decision: &'static str,
    pub role_manifest_rerun_enforcement_decision: &'static str,
    pub role_manifest_primary_gap_closed_by_application_preview: bool,
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
    pub scheduler_admission_enforcement_ready: bool,
    pub role_manifest_contract_ready: bool,
    pub role_manifest_enforcement_ready: bool,
    pub runtime_append_only_store_enabled: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub residual_route_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestRerunClearedBlockerPreview {
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestRerunResidualBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRoleManifestRerunStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
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
    pub timeline_persisted: bool,
    pub readback_executed: bool,
    pub closure_applied_to_runtime: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewReport {
    let previous_decisions =
        work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions();
    let application_outcomes =
        work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes();
    let application_plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
    let decision_deltas =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let cleared_blockers =
        work_graph_unified_projection_enforcement_role_manifest_rerun_cleared_blockers();
    let residual_blockers =
        work_graph_unified_projection_enforcement_role_manifest_rerun_residual_blockers();
    let enforcement_stages = work_graph_unified_projection_enforcement_role_manifest_rerun_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_required_prior_gates(
        );
    let previous_contract_ready_surface_count = previous_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let role_manifest_rerun_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let previous_role_manifest_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision.scheduler_admission_rerun_enforcement_decision
                == "deny_role_manifest_not_enforced"
        })
        .count();
    let role_manifest_primary_blocked_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.role_manifest_rerun_enforcement_decision == "deny_role_manifest_not_enforced"
        })
        .count();
    let role_manifest_application_source_count =
        role_manifest_application_sources(&application_plans).len();
    let role_manifest_contract_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| decision.role_manifest_contract_ready)
        .count();
    let role_manifest_runtime_residual_source_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"role_manifest_runtime_application_disabled")
        })
        .count();
    let scheduler_admission_runtime_residual_source_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"scheduler_admission_runtime_application_disabled")
        })
        .count();
    let runtime_append_only_residual_source_count = decision_deltas
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
            decision.role_manifest_rerun_enforcement_decision == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_SCHEMA_VERSION,
        preview_mode: "read_only_projection_enforcement_readiness_role_manifest_rerun_no_enforcement",
        source_surface_count: previous_decisions.len(),
        role_manifest_application_outcome_count: application_outcomes.len(),
        previous_contract_ready_surface_count,
        role_manifest_rerun_contract_ready_surface_count,
        previous_role_manifest_primary_blocked_surface_count,
        role_manifest_primary_blocked_surface_count_after,
        role_manifest_application_source_count,
        role_manifest_contract_ready_surface_count,
        role_manifest_runtime_residual_source_count,
        scheduler_admission_runtime_residual_source_count,
        runtime_append_only_residual_source_count,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_store_runtime_enablement_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions()
-> Vec<WorkGraphRoleManifestRerunSourceDecisionPreview> {
    let application_outcomes =
        work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes();
    work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions()
        .into_iter()
        .map(|decision| role_manifest_rerun_source_decision(decision, &application_outcomes))
        .collect()
}

pub fn work_graph_unified_projection_enforcement_role_manifest_rerun_cleared_blockers()
-> Vec<WorkGraphRoleManifestRerunClearedBlockerPreview> {
    let previous_decisions =
        work_graph_unified_projection_enforcement_scheduler_admission_rerun_source_decisions();
    let decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let before_sources = previous_decisions
        .iter()
        .filter(|decision| {
            decision.scheduler_admission_rerun_enforcement_decision
                == "deny_role_manifest_not_enforced"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let after_sources = decisions
        .iter()
        .filter(|decision| {
            decision.role_manifest_rerun_enforcement_decision == "deny_role_manifest_not_enforced"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![WorkGraphRoleManifestRerunClearedBlockerPreview {
        id: "role_manifest_not_enforced_for_enforcement",
        source_count_before: before_sources.len(),
        source_count_after: after_sources.len(),
        cleared_source_surface_ids: before_sources,
        closure_gate_id: WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
    }]
}

pub fn work_graph_unified_projection_enforcement_role_manifest_rerun_residual_blockers()
-> Vec<WorkGraphRoleManifestRerunResidualBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let application_plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
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
    let scheduler_sources = affected_sources(&decisions, |decision| {
        decision.previous_scheduler_admission_rerun_state
            == "scheduler_admission_contract_ready_preview_after_application"
    });
    let role_sources = role_manifest_application_sources(&application_plans);
    let runtime_append_only_sources = affected_sources(&decisions, |decision| {
        decision
            .residual_source_blocker_ids
            .contains(&"append_only_store_runtime_enablement_disabled")
    });
    let mut operator_sources = union_sources(&terminal_sources, &scheduler_sources);
    operator_sources = union_sources(&operator_sources, &role_sources);

    vec![
        residual_blocker(
            "projection_adapter_runtime_closure_application_disabled",
            "high",
            projection_sources.clone(),
            "keep projection adapter closures preview-only until runtime application gates are promoted",
        ),
        residual_blocker(
            "store_guard_runtime_application_disabled",
            "high",
            store_sources.clone(),
            "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted",
        ),
        residual_blocker(
            "idempotency_index_mutation_disabled",
            "critical",
            runtime_append_only_sources.clone(),
            "keep idempotency indexes immutable until mutation policy and replay evidence are enforced",
        ),
        residual_blocker(
            "state_store_guard_persistence_disabled",
            "high",
            store_sources,
            "do not persist candidate guard rows until append-only store intake is promoted",
        ),
        residual_blocker(
            "terminal_task_result_runtime_application_disabled",
            "high",
            terminal_sources.clone(),
            "attach terminal TaskResult wrappers to runtime only after persistence, replay, and operator-review gates are promoted",
        ),
        residual_blocker(
            "task_result_persistence_disabled",
            "high",
            terminal_sources,
            "keep TaskResult rows preview-only until append-only store intake is promoted",
        ),
        residual_blocker(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            runtime_append_only_sources.clone(),
            "keep durable store enablement disabled until WAL, readback, rollback, and operator readiness gates are promoted",
        ),
        residual_blocker(
            "wal_write_boundary_not_enabled",
            "critical",
            runtime_append_only_sources.clone(),
            "preserve no-WAL boundary until append-only event intake and replay receipts are promoted",
        ),
        residual_blocker(
            "rollback_readback_not_executed",
            "critical",
            runtime_append_only_sources,
            "execute rollback and readback gates before any append-only store enablement",
        ),
        residual_blocker(
            "scheduler_admission_runtime_application_disabled",
            "high",
            scheduler_sources.clone(),
            "keep scheduler admission runtime application disabled until leases, budgets, approvals, and store writes are promoted",
        ),
        residual_blocker(
            "lane_lease_acquisition_disabled",
            "critical",
            scheduler_sources.clone(),
            "do not acquire or mutate lane leases from the role-manifest readiness rerun",
        ),
        residual_blocker(
            "dependency_readback_not_executed",
            "high",
            scheduler_sources.clone(),
            "read back dependency terminal states before scheduler admission can become authoritative",
        ),
        residual_blocker(
            "approval_recording_disabled",
            "critical",
            scheduler_sources.clone(),
            "approval evidence must be recorded by a later runtime boundary, not this rerun",
        ),
        residual_blocker(
            "budget_consumption_disabled",
            "high",
            scheduler_sources,
            "scheduler budget checks remain contract-only and cannot consume resource or retry budget",
        ),
        residual_blocker(
            "role_manifest_runtime_application_disabled",
            "high",
            role_sources.clone(),
            "keep role manifests contract-ready but unattached until runtime application gates are promoted",
        ),
        residual_blocker(
            "tool_permission_runtime_application_disabled",
            "critical",
            role_sources.clone(),
            "do not change tool permission bindings from a readiness rerun preview",
        ),
        residual_blocker(
            "role_budget_consumption_disabled",
            "high",
            role_sources.clone(),
            "role budget envelopes remain preview-only and cannot consume budget",
        ),
        residual_blocker(
            "role_lane_binding_mutation_disabled",
            "high",
            role_sources.clone(),
            "role lane bindings remain contract-only until admission and store writes are promoted",
        ),
        residual_blocker(
            "role_terminal_contract_runtime_application_disabled",
            "high",
            role_sources,
            "terminal output schema bindings cannot become authoritative before TaskResult and role runtime application gates are promoted",
        ),
        residual_blocker(
            "operator_review_required",
            "high",
            operator_sources,
            "operator review must accept store enablement, scheduler admission, and role bindings before promotion",
        ),
        residual_blocker(
            "runtime_application_residuals_not_promoted",
            "high",
            projection_sources,
            "promote projection adapter and store guard runtime applications only after readback and operator-review gates are satisfied",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_role_manifest_rerun_stages()
-> Vec<WorkGraphRoleManifestRerunStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let application_outcomes =
        work_graph_role_manifest_enforcement_gap_closure_application_source_outcomes();
    let application_plans = work_graph_role_manifest_enforcement_gap_closure_application_plans();
    let scheduler_sources = affected_sources(&decisions, |decision| {
        decision.previous_scheduler_admission_rerun_state
            == "scheduler_admission_contract_ready_preview_after_application"
    });
    let role_sources = role_manifest_application_sources(&application_plans);
    let runtime_sources = affected_sources(&decisions, |decision| {
        decision
            .residual_source_blocker_ids
            .contains(&"append_only_store_runtime_enablement_disabled")
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
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
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "terminal_task_result_contracts",
            6,
            6,
            6,
            vec![
                "terminal_task_result_runtime_application_disabled",
                "task_result_persistence_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "append_only_store_preconditions",
            decisions.len(),
            decisions
                .iter()
                .filter(|decision| decision.append_only_store_precondition_ready)
                .count(),
            decisions
                .iter()
                .filter(|decision| decision.append_only_store_precondition_ready)
                .count(),
            vec![
                "append_only_store_runtime_enablement_disabled",
                "wal_write_boundary_not_enabled",
                "rollback_readback_not_executed",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "append_only_store_runtime_enablement",
            runtime_sources.len(),
            0,
            0,
            vec![
                "append_only_store_runtime_enablement_disabled",
                "wal_write_boundary_not_enabled",
                "idempotency_index_mutation_disabled",
                "rollback_readback_not_executed",
                "operator_review_required",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "scheduler_admission_contracts",
            scheduler_sources.len(),
            scheduler_sources.len(),
            scheduler_sources.len(),
            vec!["scheduler_admission_runtime_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "scheduler_admission_runtime_application",
            scheduler_sources.len(),
            0,
            0,
            vec![
                "scheduler_admission_runtime_application_disabled",
                "lane_lease_acquisition_disabled",
                "approval_recording_disabled",
                "budget_consumption_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "role_manifest_contracts",
            application_outcomes.len(),
            0,
            application_outcomes
                .iter()
                .filter(|outcome| outcome.role_manifest_contract_ready_preview)
                .count(),
            vec!["role_manifest_runtime_application_disabled"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "role_manifest_runtime_application",
            role_sources.len(),
            0,
            0,
            vec![
                "role_manifest_runtime_application_disabled",
                "tool_permission_runtime_application_disabled",
                "role_budget_consumption_disabled",
                "role_lane_binding_mutation_disabled",
                "role_terminal_contract_runtime_application_disabled",
            ],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_role_manifest_enforcement_gap_closure_application_required_prior_gates();
    gates.push(WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE);
    gates
}

impl WorkGraphUnifiedProjectionEnforcementReadinessRoleManifestRerunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
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
            timeline_persisted: false,
            readback_executed: false,
            closure_applied_to_runtime: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn role_manifest_rerun_source_decision(
    previous: WorkGraphSchedulerAdmissionRerunSourceDecisionPreview,
    application_outcomes: &[WorkGraphRoleManifestApplicationSourceOutcomePreview],
) -> WorkGraphRoleManifestRerunSourceDecisionPreview {
    let covered_by_role_manifest_application_preview = application_outcomes.iter().any(|outcome| {
        outcome.source_surface_id == previous.source_surface_id
            && outcome.role_manifest_contract_ready_preview
            && !outcome.applies_to_runtime
    });
    let role_manifest_contract_ready = !has_contains(
        &previous.residual_source_blocker_ids,
        "role_manifest_not_enforced",
    ) || covered_by_role_manifest_application_preview;
    let role_manifest_primary_gap_closed_by_application_preview = previous
        .scheduler_admission_rerun_enforcement_decision
        == "deny_role_manifest_not_enforced"
        && role_manifest_contract_ready;
    let mut residual_source_blocker_ids = previous
        .residual_source_blocker_ids
        .into_iter()
        .filter(|blocker| !blocker.contains("role_manifest_not_enforced"))
        .collect::<Vec<_>>();
    if covered_by_role_manifest_application_preview {
        push_unique(
            &mut residual_source_blocker_ids,
            "role_manifest_runtime_application_disabled",
        );
        push_unique(
            &mut residual_source_blocker_ids,
            "tool_permission_runtime_application_disabled",
        );
        push_unique(
            &mut residual_source_blocker_ids,
            "role_budget_consumption_disabled",
        );
        push_unique(
            &mut residual_source_blocker_ids,
            "role_lane_binding_mutation_disabled",
        );
        push_unique(
            &mut residual_source_blocker_ids,
            "role_terminal_contract_runtime_application_disabled",
        );
    }

    let role_manifest_rerun_enforcement_decision = role_manifest_rerun_enforcement_decision_for(
        previous.unified_store_projection_ready,
        previous.timeline_projection_ready,
        previous.task_result_projection_ready,
        previous.append_only_route_ready,
        previous.store_idempotency_guard_ready,
        previous.terminal_task_result_contract_ready,
        previous.append_only_store_precondition_ready,
        previous.readback_probe_contract_ready,
        previous.scheduler_admission_contract_ready,
        role_manifest_contract_ready,
        &previous.residual_route_blocker_ids,
        &residual_source_blocker_ids,
    );

    WorkGraphRoleManifestRerunSourceDecisionPreview {
        source_surface_id: previous.source_surface_id,
        source_category: previous.source_category,
        previous_projection_coverage_state: previous.previous_projection_coverage_state,
        previous_store_guard_rerun_state: previous.previous_store_guard_rerun_state,
        previous_terminal_task_result_rerun_state: previous
            .previous_terminal_task_result_rerun_state,
        previous_append_only_store_rerun_state: previous.previous_append_only_store_rerun_state,
        previous_scheduler_admission_rerun_state: previous.scheduler_admission_rerun_state,
        role_manifest_rerun_state: if covered_by_role_manifest_application_preview {
            "role_manifest_contract_ready_preview_after_application"
        } else {
            "role_manifest_not_required_for_source"
        },
        covered_by_role_manifest_application_preview,
        previous_enforcement_decision: previous.scheduler_admission_rerun_enforcement_decision,
        role_manifest_rerun_enforcement_decision,
        role_manifest_primary_gap_closed_by_application_preview,
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
        scheduler_admission_enforcement_ready: false,
        role_manifest_contract_ready,
        role_manifest_enforcement_ready: false,
        runtime_append_only_store_enabled: false,
        residual_source_blocker_ids,
        residual_route_blocker_ids: previous.residual_route_blocker_ids,
        next_required_gate: role_manifest_rerun_next_required_gate_for(
            role_manifest_rerun_enforcement_decision,
        ),
    }
}

fn role_manifest_rerun_enforcement_decision_for(
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
    } else if !role_manifest_contract_ready
        || has_contains(residual_source_blocker_ids, "role_manifest_not_enforced")
    {
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

fn role_manifest_rerun_next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_runtime_append_only_store_enablement_disabled" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

fn affected_sources(
    decisions: &[WorkGraphRoleManifestRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphRoleManifestRerunSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn role_manifest_application_sources(
    plans: &[WorkGraphRoleManifestApplicationPlanPreview],
) -> Vec<&'static str> {
    plans.iter().map(|plan| plan.source_surface_id).collect()
}

fn union_sources(left: &[&'static str], right: &[&'static str]) -> Vec<&'static str> {
    let mut sources = left.to_vec();
    for source in right {
        push_unique(&mut sources, source);
    }
    sources
}

fn residual_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphRoleManifestRerunResidualBlockerPreview {
    WorkGraphRoleManifestRerunResidualBlockerPreview {
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
) -> WorkGraphRoleManifestRerunStagePreview {
    WorkGraphRoleManifestRerunStagePreview {
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

fn has_contains(values: &[&'static str], needle: &str) -> bool {
    values.iter().any(|value| value.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_manifest_rerun_reclassifies_primary_role_blocker() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.role_manifest_application_outcome_count, 4);
        assert_eq!(report.previous_contract_ready_surface_count, 12);
        assert_eq!(report.role_manifest_rerun_contract_ready_surface_count, 12);
        assert_eq!(
            report.previous_role_manifest_primary_blocked_surface_count,
            4
        );
        assert_eq!(report.role_manifest_primary_blocked_surface_count_after, 0);
        assert_eq!(report.role_manifest_application_source_count, 4);
        assert_eq!(report.role_manifest_contract_ready_surface_count, 12);
        assert_eq!(report.role_manifest_runtime_residual_source_count, 4);
        assert_eq!(report.scheduler_admission_runtime_residual_source_count, 5);
        assert_eq!(report.runtime_append_only_residual_source_count, 12);
        assert_eq!(report.rerun_ready_surface_count, 0);
        assert_eq!(report.rerun_blocked_surface_count, 12);
    }

    #[test]
    fn role_manifest_rerun_exposes_runtime_append_only_next() {
        let decisions =
            work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
        let thread_spawn = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "multi_agent_v2_thread_spawn")
            .expect("thread spawn role manifest rerun decision");
        let task_board = decisions
            .iter()
            .find(|decision| decision.source_surface_id == "hepta_runtime_task_board")
            .expect("task board role manifest rerun decision");

        assert_eq!(
            thread_spawn.previous_enforcement_decision,
            "deny_role_manifest_not_enforced"
        );
        assert_eq!(
            thread_spawn.role_manifest_rerun_enforcement_decision,
            "deny_runtime_append_only_store_enablement_disabled"
        );
        assert!(thread_spawn.covered_by_role_manifest_application_preview);
        assert_eq!(
            thread_spawn.role_manifest_rerun_state,
            "role_manifest_contract_ready_preview_after_application"
        );
        assert_eq!(
            task_board.role_manifest_rerun_enforcement_decision,
            "deny_runtime_append_only_store_enablement_disabled"
        );
        assert!(!task_board.covered_by_role_manifest_application_preview);
        assert!(decisions.iter().all(|decision| {
            decision.role_manifest_contract_ready
                && !has_contains(
                    &decision.residual_source_blocker_ids,
                    "role_manifest_not_enforced",
                )
        }));
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| decision.role_manifest_rerun_enforcement_decision
                    == "deny_runtime_append_only_store_enablement_disabled")
                .count(),
            12
        );
    }

    #[test]
    fn role_manifest_rerun_declares_cleared_blocker_and_residuals() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_report();
        let cleared = &report.cleared_blockers[0];
        let blocker_counts = report
            .residual_blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.cleared_blocker_count, 1);
        assert_eq!(cleared.id, "role_manifest_not_enforced_for_enforcement");
        assert_eq!(cleared.source_count_before, 4);
        assert_eq!(cleared.source_count_after, 0);
        assert_eq!(
            cleared.closure_gate_id,
            WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE
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
                ("scheduler_admission_runtime_application_disabled", 5),
                ("lane_lease_acquisition_disabled", 5),
                ("dependency_readback_not_executed", 5),
                ("approval_recording_disabled", 5),
                ("budget_consumption_disabled", 5),
                ("role_manifest_runtime_application_disabled", 4),
                ("tool_permission_runtime_application_disabled", 4),
                ("role_budget_consumption_disabled", 4),
                ("role_lane_binding_mutation_disabled", 4),
                ("role_terminal_contract_runtime_application_disabled", 4),
                ("operator_review_required", 7),
                ("runtime_application_residuals_not_promoted", 7),
            ]
        );
        assert_eq!(report.residual_blocker_count, 21);
    }

    #[test]
    fn role_manifest_rerun_declares_next_frontier_and_stages() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_report();

        assert_eq!(report.enforcement_stage_count, 9);
        assert_eq!(report.required_prior_gate_count, 39);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_ROLE_MANIFEST_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
        assert!(report.ready_for_append_only_store_runtime_enablement_preview);
    }

    #[test]
    fn role_manifest_rerun_preserves_no_side_effect_boundary() {
        let report =
            hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_report();
        let side_effects = report.side_effects;

        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.graph_state_persisted);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.store_guard_attached);
        assert!(!side_effects.append_only_store_enabled);
        assert!(!side_effects.projection_enforcement_enabled);
        assert!(!side_effects.scheduler_admission_enforced);
        assert!(!side_effects.lane_lease_acquired);
        assert!(!side_effects.work_started);
        assert!(!side_effects.budget_consumed);
        assert!(!side_effects.approval_recorded);
        assert!(!side_effects.task_result_enforcement_enabled);
        assert!(!side_effects.task_result_persisted);
        assert!(!side_effects.role_manifest_enforcement_enabled);
        assert!(!side_effects.tool_permission_changed);
        assert!(!side_effects.role_budget_consumed);
        assert!(!side_effects.role_lane_binding_mutated);
        assert!(!side_effects.timeline_persisted);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.closure_applied_to_runtime);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.agent_spawn_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
    }
}
