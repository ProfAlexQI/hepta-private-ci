use serde::Serialize;

use crate::work_graph_terminal_task_result_wrapper_preview::WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PREVIEW_GATE;
use crate::work_graph_terminal_task_result_wrapper_preview::WorkGraphTerminalTaskResultWrapperPreview;
use crate::work_graph_terminal_task_result_wrapper_preview::work_graph_terminal_task_result_wrapper_required_prior_gates;
use crate::work_graph_terminal_task_result_wrapper_preview::work_graph_terminal_task_result_wrappers;
use crate::work_graph_terminal_task_result_wrapper_readback_preview::WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_PREVIEW_GATE;
use crate::work_graph_terminal_task_result_wrapper_readback_preview::WorkGraphTerminalTaskResultReadbackPlanPreview;
use crate::work_graph_terminal_task_result_wrapper_readback_preview::work_graph_terminal_task_result_wrapper_readback_plans;
use crate::work_graph_terminal_task_result_wrapper_readback_preview::work_graph_terminal_task_result_wrapper_readback_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::WorkGraphStoreGuardRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview::work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_enforcement_gap_closure_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub terminal_task_result_blocker_source_count: usize,
    pub wrapper_candidate_source_count: usize,
    pub closure_plan_count: usize,
    pub enforcement_binding_count: usize,
    pub readback_probe_binding_count: usize,
    pub application_group_count: usize,
    pub terminal_source_blocker_ref_count: usize,
    pub terminal_route_blocker_count_before: usize,
    pub terminal_route_blocker_count_after_preview: usize,
    pub readback_collection_assertion_ref_count: usize,
    pub drift_detector_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub closure_plans: Vec<WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview>,
    pub enforcement_bindings: Vec<WorkGraphTerminalTaskResultEnforcementBindingPreview>,
    pub readback_probe_bindings:
        Vec<WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview>,
    pub application_groups: Vec<WorkGraphTerminalTaskResultEnforcementGapClosureGroupPreview>,
    pub application_guards: Vec<WorkGraphTerminalTaskResultEnforcementGapClosureGuardPreview>,
    pub blockers: Vec<WorkGraphTerminalTaskResultEnforcementGapClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_terminal_task_result_enforcement_gap_closure_readback_preview: bool,
    pub ready_for_runtime_wrapper_attachment: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_persistence: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultEnforcementGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub wrapper_id: &'static str,
    pub terminal_source_kind: &'static str,
    pub emitted_event_contract_id: &'static str,
    pub replay_key_contract_id: &'static str,
    pub readback_plan_id: &'static str,
    pub evidence_contract_id: &'static str,
    pub enforcement_binding_id: String,
    pub readback_probe_binding_id: String,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub terminal_source_blocker_ids: Vec<&'static str>,
    pub route_blocker_ids_before: Vec<&'static str>,
    pub route_blocker_ids_after_preview: Vec<&'static str>,
    pub required_wire_fields: Vec<&'static str>,
    pub readback_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub attaches_runtime_wrapper: bool,
    pub executes_wrapper: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementBindingPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub wrapper_id: &'static str,
    pub task_result_collection_id: &'static str,
    pub timeline_collection_id: &'static str,
    pub route_blocker_id: &'static str,
    pub terminal_source_blocker_ids: Vec<&'static str>,
    pub binding_state: &'static str,
    pub attaches_runtime_wrapper: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub readback_plan_id: &'static str,
    pub wrapper_id: &'static str,
    pub required_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub probe_state: &'static str,
    pub performs_readback: bool,
    pub persists_drift: bool,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub closure_plan_ids: Vec<String>,
    pub source_surface_ids: Vec<&'static str>,
    pub expected_contract_count_after_closure: usize,
    pub mutates_runtime: bool,
    pub persists_task_result: bool,
    pub enables_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_projection_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_closure_plan_ids: Vec<String>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub wrapper_executed: bool,
    pub runtime_wrapper_attached: bool,
    pub readback_performed: bool,
    pub drift_state_persisted: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report()
-> WorkGraphTerminalTaskResultEnforcementGapClosurePreviewReport {
    let closure_plans = work_graph_terminal_task_result_enforcement_gap_closure_plans();
    let enforcement_bindings = work_graph_terminal_task_result_enforcement_gap_bindings();
    let readback_probe_bindings =
        work_graph_terminal_task_result_enforcement_readback_probe_bindings();
    let application_groups = work_graph_terminal_task_result_enforcement_gap_closure_groups();
    let application_guards = work_graph_terminal_task_result_enforcement_gap_closure_guards();
    let blockers = work_graph_terminal_task_result_enforcement_gap_closure_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_enforcement_gap_closure_required_prior_gates();
    let terminal_source_blocker_ref_count = closure_plans
        .iter()
        .map(|plan| plan.terminal_source_blocker_ids.len())
        .sum();
    let terminal_route_blocker_count_before = closure_plans
        .iter()
        .filter(|plan| {
            plan.route_blocker_ids_before
                .contains(&"terminal_task_result_enforcement_disabled")
        })
        .count();
    let readback_collection_assertion_ref_count = closure_plans
        .iter()
        .map(|plan| plan.readback_collection_assertion_ids.len())
        .sum();
    let drift_detector_ref_count = closure_plans
        .iter()
        .map(|plan| plan.drift_detector_ids.len())
        .sum();

    WorkGraphTerminalTaskResultEnforcementGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_enforcement_gap_closure_preview_no_runtime_attachment",
        terminal_task_result_blocker_source_count: closure_plans.len(),
        wrapper_candidate_source_count: closure_plans.len(),
        closure_plan_count: closure_plans.len(),
        enforcement_binding_count: enforcement_bindings.len(),
        readback_probe_binding_count: readback_probe_bindings.len(),
        application_group_count: application_groups.len(),
        terminal_source_blocker_ref_count,
        terminal_route_blocker_count_before,
        terminal_route_blocker_count_after_preview: 0,
        readback_collection_assertion_ref_count,
        drift_detector_ref_count,
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        closure_plans,
        enforcement_bindings,
        readback_probe_bindings,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_terminal_task_result_enforcement_gap_closure_readback_preview: true,
        ready_for_runtime_wrapper_attachment: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_persistence: false,
        ready_for_task_result_enforcement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultEnforcementGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_plans()
-> Vec<WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview> {
    let wrappers = work_graph_terminal_task_result_wrappers();
    let readback_plans = work_graph_terminal_task_result_wrapper_readback_plans();

    work_graph_terminal_task_result_enforcement_gap_source_decisions()
        .into_iter()
        .map(|decision| {
            let wrapper = wrappers
                .iter()
                .find(|wrapper| wrapper.source_surface_id == decision.source_surface_id)
                .expect("terminal TaskResult wrapper for blocker source");
            let readback_plan = readback_plans
                .iter()
                .find(|plan| plan.source_surface_id == decision.source_surface_id)
                .expect("terminal TaskResult readback plan for blocker source");
            terminal_task_result_closure_plan(&decision, wrapper, readback_plan)
        })
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_gap_bindings()
-> Vec<WorkGraphTerminalTaskResultEnforcementBindingPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphTerminalTaskResultEnforcementBindingPreview {
                id: plan.enforcement_binding_id,
                source_surface_id: plan.source_surface_id,
                wrapper_id: plan.wrapper_id,
                task_result_collection_id: "taskResults",
                timeline_collection_id: "timelineEvents",
                route_blocker_id: "terminal_task_result_enforcement_disabled",
                terminal_source_blocker_ids: plan.terminal_source_blocker_ids,
                binding_state: "preview_binding_defined_runtime_attachment_disabled",
                attaches_runtime_wrapper: false,
                persists_task_result: false,
                enables_task_result_enforcement: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_readback_probe_bindings()
-> Vec<WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview {
                id: plan.readback_probe_binding_id,
                source_surface_id: plan.source_surface_id,
                readback_plan_id: plan.readback_plan_id,
                wrapper_id: plan.wrapper_id,
                required_collection_assertion_ids: plan.readback_collection_assertion_ids,
                drift_detector_ids: plan.drift_detector_ids,
                probe_state: "preview_probe_binding_defined_readback_execution_disabled",
                performs_readback: false,
                persists_drift: false,
                enables_task_result_enforcement: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_groups()
-> Vec<WorkGraphTerminalTaskResultEnforcementGapClosureGroupPreview> {
    let plans = work_graph_terminal_task_result_enforcement_gap_closure_plans();
    vec![
        closure_group("terminal_wrapper_contract_closure", "p0", &plans, "wrapper"),
        closure_group(
            "terminal_enforcement_binding_closure",
            "p0",
            &plans,
            "enforcement",
        ),
        closure_group("terminal_readback_probe_closure", "p0", &plans, "readback"),
        closure_group(
            "terminal_readiness_rerun_input_closure",
            "p0",
            &plans,
            "application",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_guards()
-> Vec<WorkGraphTerminalTaskResultEnforcementGapClosureGuardPreview> {
    vec![
        guard("closure_preview_only", "medium", "preview"),
        guard("wrapper_runtime_attachment_disabled", "high", "runtime"),
        guard("wrapper_execution_disabled", "high", "runtime"),
        guard(
            "task_result_persistence_disabled",
            "high",
            "task_result_store",
        ),
        guard("readback_execution_disabled", "high", "readback"),
        guard(
            "terminal_task_result_enforcement_disabled",
            "critical",
            "task_result_enforcement",
        ),
        guard(
            "scheduler_admission_or_role_manifest_residuals_not_enforced",
            "high",
            "residual_admission_role",
        ),
        guard(
            "append_only_store_enablement_disabled",
            "high",
            "append_only_store",
        ),
        guard(
            "enforcement_readiness_task_result_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_blockers()
-> Vec<WorkGraphTerminalTaskResultEnforcementGapClosureBlockerPreview> {
    let plans = work_graph_terminal_task_result_enforcement_gap_closure_plans();
    vec![
        blocker(
            "terminal_task_result_closure_is_preview_only",
            "medium",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "keep terminal TaskResult closure as a no-mutation preview until readback verifies every wrapper binding",
        ),
        blocker(
            "wrapper_runtime_attachment_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "attach terminal wrappers to runtime only after operator review and persistence gates are promoted",
        ),
        blocker(
            "wrapper_execution_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "execute no terminal wrapper until readback, drift budget, and promotion-precondition previews are clean",
        ),
        blocker(
            "task_result_persistence_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "do not persist TaskResult rows until append-only store intake and replay evidence are promoted",
        ),
        blocker(
            "readback_execution_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "run terminal TaskResult readback only after fixture and wrapper contracts are promoted",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "critical",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "keep terminal TaskResult enforcement disabled until wrapper application and readiness rerun prove no route blocker remains",
        ),
        blocker(
            "scheduler_admission_or_role_manifest_residuals_not_enforced",
            "high",
            affected_sources(&plans, |plan| {
                plan.residual_source_blocker_ids.iter().any(|blocker| {
                    blocker.ends_with("_admission_not_enforced")
                        || blocker.contains("role_manifest_not_enforced")
                })
            }),
            affected_plan_ids(&plans, |plan| {
                plan.residual_source_blocker_ids.iter().any(|blocker| {
                    blocker.ends_with("_admission_not_enforced")
                        || blocker.contains("role_manifest_not_enforced")
                })
            }),
            "preserve admission and role-manifest blockers as separate readiness gates after TaskResult closure",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "keep append-only store disabled until TaskResult enforcement, replay, and operator readiness are promoted",
        ),
        blocker(
            "enforcement_readiness_task_result_rerun_missing",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "rerun unified projection enforcement-readiness after terminal TaskResult closure application preview",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_store_guard_rerun_required_prior_gates(
        );
    push_unique(
        &mut gates,
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_STORE_GUARD_RERUN_PREVIEW_GATE,
    );
    for gate in work_graph_terminal_task_result_wrapper_required_prior_gates() {
        push_unique(&mut gates, gate);
    }
    push_unique(
        &mut gates,
        WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PREVIEW_GATE,
    );
    for gate in work_graph_terminal_task_result_wrapper_readback_required_prior_gates() {
        push_unique(&mut gates, gate);
    }
    push_unique(
        &mut gates,
        WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphTerminalTaskResultEnforcementGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            wrapper_executed: false,
            runtime_wrapper_attached: false,
            readback_performed: false,
            drift_state_persisted: false,
            event_record_persisted: false,
            task_result_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_terminal_task_result_enforcement_gap_source_decisions()
-> Vec<WorkGraphStoreGuardRerunSourceDecisionPreview> {
    work_graph_unified_projection_enforcement_store_guard_rerun_source_decisions()
        .into_iter()
        .filter(|decision| {
            decision.store_guard_rerun_enforcement_decision
                == "deny_terminal_task_result_enforcement_disabled"
        })
        .collect()
}

fn terminal_task_result_closure_plan(
    decision: &WorkGraphStoreGuardRerunSourceDecisionPreview,
    wrapper: &WorkGraphTerminalTaskResultWrapperPreview,
    readback_plan: &WorkGraphTerminalTaskResultReadbackPlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview {
    WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview {
        id: format!(
            "close_{}_terminal_task_result_enforcement_gap",
            decision.source_surface_id
        ),
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        wrapper_id: wrapper.id,
        terminal_source_kind: wrapper.terminal_source_kind,
        emitted_event_contract_id: wrapper.emitted_event_contract_id,
        replay_key_contract_id: wrapper.replay_key_contract_id,
        readback_plan_id: readback_plan.id,
        evidence_contract_id: readback_plan.expected_evidence_contract_id,
        enforcement_binding_id: format!(
            "bind_{}_terminal_task_result_enforcement_preview",
            decision.source_surface_id
        ),
        readback_probe_binding_id: format!(
            "bind_{}_terminal_task_result_readback_probe_preview",
            decision.source_surface_id
        ),
        residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
        terminal_source_blocker_ids: terminal_source_blocker_ids(decision),
        route_blocker_ids_before: decision.residual_route_blocker_ids.clone(),
        route_blocker_ids_after_preview: decision
            .residual_route_blocker_ids
            .iter()
            .copied()
            .filter(|blocker| *blocker != "terminal_task_result_enforcement_disabled")
            .collect(),
        required_wire_fields: wrapper.required_wire_fields.clone(),
        readback_collection_assertion_ids: readback_plan.required_collection_assertion_ids.clone(),
        drift_detector_ids: readback_plan.drift_detector_ids.clone(),
        application_state: "preview_closure_defined_terminal_task_result_enforcement_not_attached",
        readback_verified_by_preview: true,
        attaches_runtime_wrapper: false,
        executes_wrapper: false,
        persists_task_result: false,
        enables_task_result_enforcement: false,
        mutates_store: false,
    }
}

fn terminal_source_blocker_ids(
    decision: &WorkGraphStoreGuardRerunSourceDecisionPreview,
) -> Vec<&'static str> {
    decision
        .residual_source_blocker_ids
        .iter()
        .copied()
        .filter(|blocker| terminal_task_result_source_blocker(blocker))
        .collect()
}

fn terminal_task_result_source_blocker(blocker: &str) -> bool {
    blocker.contains("task_result")
        || blocker.contains("TaskResult")
        || blocker.contains("terminal_task_result")
        || blocker.contains("verifier_and_reducer")
        || blocker.contains("result_json")
}

fn closure_group(
    id: &'static str,
    priority: &'static str,
    plans: &[WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview],
    expected_contract_kind: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementGapClosureGroupPreview {
    WorkGraphTerminalTaskResultEnforcementGapClosureGroupPreview {
        id,
        priority,
        closure_plan_ids: plans.iter().map(|plan| plan.id.clone()).collect(),
        source_surface_ids: plans.iter().map(|plan| plan.source_surface_id).collect(),
        expected_contract_count_after_closure: plans
            .iter()
            .filter(|plan| match expected_contract_kind {
                "wrapper" => !plan.wrapper_id.is_empty(),
                "enforcement" => !plan.enforcement_binding_id.is_empty(),
                "readback" => !plan.readback_probe_binding_id.is_empty(),
                "application" => plan.readback_verified_by_preview,
                _ => false,
            })
            .count(),
        mutates_runtime: false,
        persists_task_result: false,
        enables_enforcement: false,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementGapClosureGuardPreview {
    WorkGraphTerminalTaskResultEnforcementGapClosureGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_projection_enforcement: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_closure_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementGapClosureBlockerPreview {
    WorkGraphTerminalTaskResultEnforcementGapClosureBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        affected_closure_plan_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn affected_sources(
    plans: &[WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn affected_plan_ids(
    plans: &[WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.id.clone())
        .collect()
}

fn push_unique(gates: &mut Vec<&'static str>, gate: &'static str) {
    if !gates.contains(&gate) {
        gates.push(gate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_enforcement_gap_closure_targets_current_terminal_blockers_only() {
        let report = hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report();
        let sources = report
            .closure_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.terminal_task_result_blocker_source_count, 6);
        assert_eq!(report.wrapper_candidate_source_count, 6);
        assert_eq!(report.closure_plan_count, 6);
        assert_eq!(report.enforcement_binding_count, 6);
        assert_eq!(report.readback_probe_binding_count, 6);
        assert_eq!(
            sources,
            [
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
    }

    #[test]
    fn task_result_enforcement_gap_closure_binds_wrappers_and_readback() {
        let report = hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report();
        let reducer = report
            .closure_plans
            .iter()
            .find(|plan| plan.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer closure plan");

        assert_eq!(
            reducer.wrapper_id,
            "multi_agent_reducer_terminal_task_result_wrapper"
        );
        assert_eq!(
            reducer.readback_plan_id,
            "readback_fixture_multi_agent_reducer_ok"
        );
        assert_eq!(reducer.evidence_contract_id, "reducer_consensus_evidence");
        assert_eq!(
            reducer.terminal_source_blocker_ids,
            ["multi_agent_reducer_task_result_projection_report_only_not_enforced"]
        );
        assert_eq!(report.terminal_source_blocker_ref_count, 6);
        assert_eq!(report.terminal_route_blocker_count_before, 6);
        assert_eq!(report.terminal_route_blocker_count_after_preview, 0);
        assert_eq!(report.readback_collection_assertion_ref_count, 18);
        assert_eq!(report.drift_detector_ref_count, 30);
    }

    #[test]
    fn task_result_enforcement_gap_closure_declares_groups_guards_and_blockers() {
        let report = hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.application_group_count, 4);
        assert!(report.application_groups.iter().all(|group| {
            group.expected_contract_count_after_closure == 6
                && !group.mutates_runtime
                && !group.persists_task_result
                && !group.enables_enforcement
        }));
        assert_eq!(report.application_guard_count, 9);
        assert_eq!(
            blocker_counts,
            [
                ("terminal_task_result_closure_is_preview_only", 6),
                ("wrapper_runtime_attachment_disabled", 6),
                ("wrapper_execution_disabled", 6),
                ("task_result_persistence_disabled", 6),
                ("readback_execution_disabled", 6),
                ("terminal_task_result_enforcement_disabled", 6),
                (
                    "scheduler_admission_or_role_manifest_residuals_not_enforced",
                    5,
                ),
                ("append_only_store_enablement_disabled", 6),
                ("enforcement_readiness_task_result_rerun_missing", 6),
            ]
        );
    }

    #[test]
    fn task_result_enforcement_gap_closure_advances_to_readback_only() {
        let report = hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report();

        assert_eq!(report.required_prior_gate_count, 24);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_terminal_task_result_enforcement_gap_closure_readback_preview);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn task_result_enforcement_gap_closure_keeps_all_side_effects_disabled() {
        let report = hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultEnforcementGapClosurePreviewSideEffects::none()
        );
        assert!(
            report
                .closure_plans
                .iter()
                .all(|plan| !plan.attaches_runtime_wrapper
                    && !plan.executes_wrapper
                    && !plan.persists_task_result
                    && !plan.enables_task_result_enforcement
                    && !plan.mutates_store)
        );
    }
}
