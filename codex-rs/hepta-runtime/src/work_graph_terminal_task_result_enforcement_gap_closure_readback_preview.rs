use serde::Serialize;

use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::WorkGraphTerminalTaskResultEnforcementBindingPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::work_graph_terminal_task_result_enforcement_gap_bindings;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::work_graph_terminal_task_result_enforcement_gap_closure_plans;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::work_graph_terminal_task_result_enforcement_gap_closure_required_prior_gates;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::work_graph_terminal_task_result_enforcement_readback_probe_bindings;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub closure_plan_count: usize,
    pub enforcement_binding_count: usize,
    pub readback_probe_binding_count: usize,
    pub readback_plan_count: usize,
    pub wrapper_binding_assertion_count: usize,
    pub enforcement_binding_assertion_count: usize,
    pub readback_probe_assertion_count: usize,
    pub wire_field_assertion_count: usize,
    pub wire_field_ref_count: usize,
    pub collection_assertion_ref_count: usize,
    pub drift_detector_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview>,
    pub wrapper_binding_assertions:
        Vec<WorkGraphTerminalTaskResultWrapperBindingReadbackAssertionPreview>,
    pub enforcement_binding_assertions:
        Vec<WorkGraphTerminalTaskResultEnforcementBindingReadbackAssertionPreview>,
    pub readback_probe_assertions:
        Vec<WorkGraphTerminalTaskResultReadbackProbeBindingReadbackAssertionPreview>,
    pub wire_field_assertions: Vec<WorkGraphTerminalTaskResultWireFieldReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphTerminalTaskResultEnforcementReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphTerminalTaskResultEnforcementReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_terminal_task_result_enforcement_gap_closure_application_preview: bool,
    pub ready_for_runtime_wrapper_attachment: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_persistence: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub wrapper_id: &'static str,
    pub enforcement_binding_id: String,
    pub readback_probe_binding_id: String,
    pub wrapper_readback_plan_id: &'static str,
    pub expected_evidence_contract_id: &'static str,
    pub required_wire_fields: Vec<&'static str>,
    pub required_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub required_before_closure_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub attaches_runtime_wrapper: bool,
    pub executes_wrapper: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub closure_plan_id: String,
    pub wrapper_id: &'static str,
    pub terminal_source_kind: &'static str,
    pub emitted_event_contract_id: &'static str,
    pub replay_key_contract_id: &'static str,
    pub terminal_source_blocker_ids: Vec<&'static str>,
    pub expected_wrapper_state: &'static str,
    pub attaches_runtime_wrapper: bool,
    pub executes_wrapper: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub enforcement_binding_id: String,
    pub wrapper_id: &'static str,
    pub task_result_collection_id: &'static str,
    pub timeline_collection_id: &'static str,
    pub route_blocker_id: &'static str,
    pub terminal_source_blocker_ids: Vec<&'static str>,
    pub expected_binding_state: &'static str,
    pub attaches_runtime_wrapper: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultReadbackProbeBindingReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub readback_probe_binding_id: String,
    pub wrapper_readback_plan_id: &'static str,
    pub wrapper_id: &'static str,
    pub required_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub expected_probe_state: &'static str,
    pub performs_readback: bool,
    pub persists_drift: bool,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWireFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub wrapper_id: &'static str,
    pub required_wire_fields: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_wire_state: &'static str,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_closure_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewSideEffects {
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

pub fn hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report()
-> WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewReport {
    let closure_plans = work_graph_terminal_task_result_enforcement_gap_closure_plans();
    let enforcement_bindings = work_graph_terminal_task_result_enforcement_gap_bindings();
    let readback_probe_bindings =
        work_graph_terminal_task_result_enforcement_readback_probe_bindings();
    let readback_plans = work_graph_terminal_task_result_enforcement_gap_closure_readback_plans();
    let wrapper_binding_assertions =
        work_graph_terminal_task_result_wrapper_binding_readback_assertions();
    let enforcement_binding_assertions =
        work_graph_terminal_task_result_enforcement_binding_readback_assertions();
    let readback_probe_assertions =
        work_graph_terminal_task_result_readback_probe_binding_readback_assertions();
    let wire_field_assertions = work_graph_terminal_task_result_wire_field_readback_assertions();
    let drift_detectors =
        work_graph_terminal_task_result_enforcement_gap_closure_readback_drift_detectors();
    let blockers = work_graph_terminal_task_result_enforcement_gap_closure_readback_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_enforcement_gap_closure_readback_required_prior_gates();
    let wire_field_ref_count = wire_field_assertions
        .iter()
        .map(|assertion| assertion.required_field_count)
        .sum();
    let collection_assertion_ref_count = readback_probe_assertions
        .iter()
        .map(|assertion| assertion.required_collection_assertion_ids.len())
        .sum();
    let drift_detector_ref_count = readback_probe_assertions
        .iter()
        .map(|assertion| assertion.drift_detector_ids.len())
        .sum();

    WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_enforcement_gap_closure_readback_no_execution",
        closure_plan_count: closure_plans.len(),
        enforcement_binding_count: enforcement_bindings.len(),
        readback_probe_binding_count: readback_probe_bindings.len(),
        readback_plan_count: readback_plans.len(),
        wrapper_binding_assertion_count: wrapper_binding_assertions.len(),
        enforcement_binding_assertion_count: enforcement_binding_assertions.len(),
        readback_probe_assertion_count: readback_probe_assertions.len(),
        wire_field_assertion_count: wire_field_assertions.len(),
        wire_field_ref_count,
        collection_assertion_ref_count,
        drift_detector_ref_count,
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        wrapper_binding_assertions,
        enforcement_binding_assertions,
        readback_probe_assertions,
        wire_field_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_terminal_task_result_enforcement_gap_closure_application_preview: true,
        ready_for_runtime_wrapper_attachment: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_persistence: false,
        ready_for_task_result_enforcement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_readback_plans()
-> Vec<WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .map(|plan| readback_plan(&plan))
        .collect()
}

pub fn work_graph_terminal_task_result_wrapper_binding_readback_assertions()
-> Vec<WorkGraphTerminalTaskResultWrapperBindingReadbackAssertionPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphTerminalTaskResultWrapperBindingReadbackAssertionPreview {
                id: format!(
                    "assert_{}_terminal_wrapper_binding_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                closure_plan_id: plan.id,
                wrapper_id: plan.wrapper_id,
                terminal_source_kind: plan.terminal_source_kind,
                emitted_event_contract_id: plan.emitted_event_contract_id,
                replay_key_contract_id: plan.replay_key_contract_id,
                terminal_source_blocker_ids: plan.terminal_source_blocker_ids,
                expected_wrapper_state: "wrapper_contract_defined_runtime_attachment_disabled",
                attaches_runtime_wrapper: false,
                executes_wrapper: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_binding_readback_assertions()
-> Vec<WorkGraphTerminalTaskResultEnforcementBindingReadbackAssertionPreview> {
    work_graph_terminal_task_result_enforcement_gap_bindings()
        .into_iter()
        .map(|binding| enforcement_binding_assertion(&binding))
        .collect()
}

pub fn work_graph_terminal_task_result_readback_probe_binding_readback_assertions()
-> Vec<WorkGraphTerminalTaskResultReadbackProbeBindingReadbackAssertionPreview> {
    work_graph_terminal_task_result_enforcement_readback_probe_bindings()
        .into_iter()
        .map(|binding| readback_probe_assertion(&binding))
        .collect()
}

pub fn work_graph_terminal_task_result_wire_field_readback_assertions()
-> Vec<WorkGraphTerminalTaskResultWireFieldReadbackAssertionPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .map(
            |plan| WorkGraphTerminalTaskResultWireFieldReadbackAssertionPreview {
                id: format!(
                    "assert_{}_terminal_task_result_wire_fields_readback",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                wrapper_id: plan.wrapper_id,
                required_field_count: plan.required_wire_fields.len(),
                required_wire_fields: plan.required_wire_fields,
                expected_wire_state: "task_result_wire_contract_defined_no_execution",
                enables_task_result_enforcement: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_readback_drift_detectors()
-> Vec<WorkGraphTerminalTaskResultEnforcementReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "terminal_task_result_identity_drift",
            vec!["taskId", "traceId", "wrapperId"],
            "high",
        ),
        drift_detector(
            "terminal_task_result_status_drift",
            vec!["status", "terminalSourceKind"],
            "high",
        ),
        drift_detector(
            "terminal_task_result_evidence_drift",
            vec!["evidenceContractId", "terminalSourceBlockerIds"],
            "high",
        ),
        drift_detector(
            "terminal_task_result_readback_probe_drift",
            vec!["readbackProbeBindingId", "requiredCollectionAssertionIds"],
            "high",
        ),
        drift_detector(
            "terminal_task_result_redaction_drift",
            vec!["summary", "artifacts", "risks"],
            "medium",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_readback_blockers()
-> Vec<WorkGraphTerminalTaskResultEnforcementReadbackBlockerPreview> {
    let plans = work_graph_terminal_task_result_enforcement_gap_closure_readback_plans();
    vec![
        blocker(
            "readback_execution_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "this preview defines readback assertions but does not query fixtures, runtime wrappers, or WorkGraph state",
        ),
        blocker(
            "wrapper_runtime_attachment_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "runtime wrapper attachment remains disabled until readback assertions and operator review pass",
        ),
        blocker(
            "wrapper_execution_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "terminal wrappers remain unexecuted until readback and application previews are promoted",
        ),
        blocker(
            "task_result_persistence_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "TaskResult rows remain preview-only and are not persisted before append-only store enablement",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "critical",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "terminal TaskResult enforcement stays disabled until closure application and readiness rerun pass",
        ),
        blocker(
            "scheduler_admission_or_role_manifest_residuals_not_enforced",
            "high",
            affected_sources(&plans, |plan| {
                closure_plan_for_readback(plan)
                    .residual_source_blocker_ids
                    .iter()
                    .any(|blocker| {
                        blocker.ends_with("_admission_not_enforced")
                            || blocker.contains("role_manifest_not_enforced")
                    })
            }),
            affected_plan_ids(&plans, |plan| {
                closure_plan_for_readback(plan)
                    .residual_source_blocker_ids
                    .iter()
                    .any(|blocker| {
                        blocker.ends_with("_admission_not_enforced")
                            || blocker.contains("role_manifest_not_enforced")
                    })
            }),
            "admission and role-manifest residual blockers remain separate gates after TaskResult readback",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "append-only store enablement remains disabled until TaskResult closure application and replay are promoted",
        ),
        blocker(
            "terminal_task_result_closure_application_missing",
            "high",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "run closure application preview after readback asserts wrapper bindings, probes, and wire fields",
        ),
        blocker(
            "operator_review_required",
            "medium",
            affected_sources(&plans, |_| true),
            affected_plan_ids(&plans, |_| true),
            "operator review must accept terminal wrapper bindings, evidence contracts, and drift detectors before promotion",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_terminal_task_result_enforcement_gap_closure_required_prior_gates();
    if !gates.contains(&WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE) {
        gates.push(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE);
    }
    gates
}

impl WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewSideEffects {
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

fn readback_plan(
    plan: &WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview {
    WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview {
        id: readback_id_for_source(plan.source_surface_id),
        source_surface_id: plan.source_surface_id,
        closure_plan_id: plan.id.clone(),
        wrapper_id: plan.wrapper_id,
        enforcement_binding_id: plan.enforcement_binding_id.clone(),
        readback_probe_binding_id: plan.readback_probe_binding_id.clone(),
        wrapper_readback_plan_id: plan.readback_plan_id,
        expected_evidence_contract_id: plan.evidence_contract_id,
        required_wire_fields: plan.required_wire_fields.clone(),
        required_collection_assertion_ids: plan.readback_collection_assertion_ids.clone(),
        drift_detector_ids: plan.drift_detector_ids.clone(),
        required_before_closure_application: true,
        readback_state: "readback_assertions_defined_execution_disabled",
        performs_readback: false,
        attaches_runtime_wrapper: false,
        executes_wrapper: false,
        persists_task_result: false,
        enables_task_result_enforcement: false,
        mutates_store: false,
    }
}

fn enforcement_binding_assertion(
    binding: &WorkGraphTerminalTaskResultEnforcementBindingPreview,
) -> WorkGraphTerminalTaskResultEnforcementBindingReadbackAssertionPreview {
    WorkGraphTerminalTaskResultEnforcementBindingReadbackAssertionPreview {
        id: format!(
            "assert_{}_terminal_task_result_enforcement_binding_readback",
            binding.source_surface_id
        ),
        source_surface_id: binding.source_surface_id,
        enforcement_binding_id: binding.id.clone(),
        wrapper_id: binding.wrapper_id,
        task_result_collection_id: binding.task_result_collection_id,
        timeline_collection_id: binding.timeline_collection_id,
        route_blocker_id: binding.route_blocker_id,
        terminal_source_blocker_ids: binding.terminal_source_blocker_ids.clone(),
        expected_binding_state: binding.binding_state,
        attaches_runtime_wrapper: false,
        persists_task_result: false,
        enables_task_result_enforcement: false,
    }
}

fn readback_probe_assertion(
    binding: &WorkGraphTerminalTaskResultEnforcementReadbackProbeBindingPreview,
) -> WorkGraphTerminalTaskResultReadbackProbeBindingReadbackAssertionPreview {
    WorkGraphTerminalTaskResultReadbackProbeBindingReadbackAssertionPreview {
        id: format!(
            "assert_{}_terminal_task_result_readback_probe_binding_readback",
            binding.source_surface_id
        ),
        source_surface_id: binding.source_surface_id,
        readback_probe_binding_id: binding.id.clone(),
        wrapper_readback_plan_id: binding.readback_plan_id,
        wrapper_id: binding.wrapper_id,
        required_collection_assertion_ids: binding.required_collection_assertion_ids.clone(),
        drift_detector_ids: binding.drift_detector_ids.clone(),
        expected_probe_state: binding.probe_state,
        performs_readback: false,
        persists_drift: false,
        enables_task_result_enforcement: false,
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementReadbackDriftDetectorPreview {
    WorkGraphTerminalTaskResultEnforcementReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_closure_application: true,
        performs_readback: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_readback_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementReadbackBlockerPreview {
    WorkGraphTerminalTaskResultEnforcementReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        affected_readback_plan_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn affected_sources(
    plans: &[WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview) -> bool,
) -> Vec<&'static str> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn affected_plan_ids(
    plans: &[WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.id.clone())
        .collect()
}

fn closure_plan_for_readback(
    readback_plan: &WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .find(|plan| plan.id == readback_plan.closure_plan_id)
        .unwrap_or_else(|| {
            panic!(
                "missing terminal TaskResult closure plan {}",
                readback_plan.closure_plan_id
            )
        })
}

fn readback_id_for_source(source_surface_id: &str) -> String {
    format!("readback_{source_surface_id}_terminal_task_result_enforcement_gap_closure")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_enforcement_gap_closure_readback_targets_current_closure_plans() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report();
        let sources = report
            .readback_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

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
        assert_eq!(report.closure_plan_count, 6);
        assert_eq!(report.enforcement_binding_count, 6);
        assert_eq!(report.readback_probe_binding_count, 6);
        assert_eq!(report.readback_plan_count, 6);
        assert_eq!(report.wrapper_binding_assertion_count, 6);
        assert_eq!(report.enforcement_binding_assertion_count, 6);
        assert_eq!(report.readback_probe_assertion_count, 6);
        assert_eq!(report.wire_field_assertion_count, 6);
    }

    #[test]
    fn task_result_enforcement_gap_closure_readback_preserves_bindings_and_wire_contracts() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report();
        let reducer = report
            .readback_plans
            .iter()
            .find(|plan| plan.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer readback plan");

        assert_eq!(
            reducer.wrapper_id,
            "multi_agent_reducer_terminal_task_result_wrapper"
        );
        assert_eq!(
            reducer.wrapper_readback_plan_id,
            "readback_fixture_multi_agent_reducer_ok"
        );
        assert_eq!(
            reducer.expected_evidence_contract_id,
            "reducer_consensus_evidence"
        );
        assert_eq!(report.wire_field_ref_count, 66);
        assert_eq!(report.collection_assertion_ref_count, 18);
        assert_eq!(report.drift_detector_ref_count, 30);
        assert!(
            report
                .wire_field_assertions
                .iter()
                .all(|assertion| assertion.required_field_count == 11
                    && !assertion.enables_task_result_enforcement)
        );
        assert!(report.readback_probe_assertions.iter().all(|assertion| {
            assertion.required_collection_assertion_ids.len() == 3
                && assertion.drift_detector_ids.len() == 5
                && !assertion.performs_readback
                && !assertion.persists_drift
        }));
    }

    #[test]
    fn task_result_enforcement_gap_closure_readback_defines_drift_and_blockers() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.drift_detector_count, 5);
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.blocks_closure_application && !detector.performs_readback)
        );
        assert_eq!(
            blocker_counts,
            [
                ("readback_execution_disabled", 6),
                ("wrapper_runtime_attachment_disabled", 6),
                ("wrapper_execution_disabled", 6),
                ("task_result_persistence_disabled", 6),
                ("terminal_task_result_enforcement_disabled", 6),
                (
                    "scheduler_admission_or_role_manifest_residuals_not_enforced",
                    5,
                ),
                ("append_only_store_enablement_disabled", 6),
                ("terminal_task_result_closure_application_missing", 6),
                ("operator_review_required", 6),
            ]
        );
        assert_eq!(report.blocker_count, 9);
    }

    #[test]
    fn task_result_enforcement_gap_closure_readback_advances_to_application_only() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 25);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_terminal_task_result_enforcement_gap_closure_application_preview);
        assert!(!report.ready_for_runtime_wrapper_attachment);
        assert!(!report.ready_for_task_result_persistence);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn task_result_enforcement_gap_closure_readback_keeps_all_side_effects_disabled() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPreviewSideEffects::none()
        );
        assert!(
            report
                .readback_plans
                .iter()
                .all(|plan| !plan.performs_readback
                    && !plan.attaches_runtime_wrapper
                    && !plan.executes_wrapper
                    && !plan.persists_task_result
                    && !plan.enables_task_result_enforcement
                    && !plan.mutates_store)
        );
        assert!(
            report
                .enforcement_binding_assertions
                .iter()
                .all(|assertion| !assertion.attaches_runtime_wrapper
                    && !assertion.persists_task_result
                    && !assertion.enables_task_result_enforcement)
        );
    }
}
