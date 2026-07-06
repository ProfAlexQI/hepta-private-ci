use serde::Serialize;

use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_preview::work_graph_terminal_task_result_enforcement_gap_closure_plans;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_readback_preview::WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_readback_preview::WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_readback_preview::work_graph_terminal_task_result_enforcement_gap_closure_readback_plans;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_readback_preview::work_graph_terminal_task_result_enforcement_gap_closure_readback_required_prior_gates;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_enforcement_gap_closure_application_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub source_terminal_task_result_contract_ready_preview_count: usize,
    pub application_group_count: usize,
    pub wire_field_ref_count: usize,
    pub collection_assertion_ref_count: usize,
    pub drift_detector_ref_count: usize,
    pub terminal_source_blocker_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview>,
    pub application_groups: Vec<WorkGraphTerminalTaskResultEnforcementApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphTerminalTaskResultEnforcementApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphTerminalTaskResultEnforcementApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview: bool,
    pub ready_for_runtime_wrapper_attachment: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_persistence: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub wrapper_id: &'static str,
    pub terminal_source_kind: &'static str,
    pub enforcement_binding_id: String,
    pub readback_probe_binding_id: String,
    pub wrapper_readback_plan_id: &'static str,
    pub expected_evidence_contract_id: &'static str,
    pub application_scope: &'static str,
    pub required_wire_fields: Vec<&'static str>,
    pub required_collection_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub terminal_source_blocker_ids: Vec<&'static str>,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub attaches_runtime_wrapper: bool,
    pub executes_wrapper: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
    pub enables_append_only_store: bool,
    pub enforces_projection: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub wrapper_id: &'static str,
    pub application_plan_id: String,
    pub post_application_terminal_task_result_state: &'static str,
    pub terminal_task_result_contract_ready_preview: bool,
    pub ready_for_enforcement_readiness_terminal_task_result_rerun: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_projection_enforcement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub expected_terminal_task_result_ready_source_count_after_application: usize,
    pub mutates_runtime: bool,
    pub persists_task_result: bool,
    pub enables_task_result_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_projection_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewSideEffects {
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

pub fn hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report()
-> WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewReport {
    let readback_plans = work_graph_terminal_task_result_enforcement_gap_closure_readback_plans();
    let application_plans =
        work_graph_terminal_task_result_enforcement_gap_closure_application_plans();
    let source_outcomes =
        work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes();
    let application_groups =
        work_graph_terminal_task_result_enforcement_gap_closure_application_groups();
    let application_guards =
        work_graph_terminal_task_result_enforcement_gap_closure_application_guards();
    let blockers = work_graph_terminal_task_result_enforcement_gap_closure_application_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_enforcement_gap_closure_application_required_prior_gates();

    WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_terminal_task_result_enforcement_gap_closure_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        source_terminal_task_result_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.terminal_task_result_contract_ready_preview)
            .count(),
        application_group_count: application_groups.len(),
        wire_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_wire_fields.len())
            .sum(),
        collection_assertion_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_collection_assertion_ids.len())
            .sum(),
        drift_detector_ref_count: application_plans
            .iter()
            .map(|plan| plan.drift_detector_ids.len())
            .sum(),
        terminal_source_blocker_ref_count: application_plans
            .iter()
            .map(|plan| plan.terminal_source_blocker_ids.len())
            .sum(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        application_plans,
        source_outcomes,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview: true,
        ready_for_runtime_wrapper_attachment: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_persistence: false,
        ready_for_task_result_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_plans()
-> Vec<WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes()
-> Vec<WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview> {
    work_graph_terminal_task_result_enforcement_gap_closure_application_plans()
        .into_iter()
        .map(source_outcome)
        .collect()
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_groups()
-> Vec<WorkGraphTerminalTaskResultEnforcementApplicationGroupPreview> {
    let plans = work_graph_terminal_task_result_enforcement_gap_closure_application_plans();
    vec![
        application_group(
            "multi_agent_terminal_task_result_application",
            "p0",
            vec![
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_multi_agent_reducer",
            ],
            &plans,
        ),
        application_group(
            "batch_agent_jobs_terminal_task_result_application",
            "p0",
            vec!["agent_jobs_batch_workers"],
            &plans,
        ),
        application_group(
            "runtime_scheduler_terminal_task_result_application",
            "p0",
            vec![
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
            ],
            &plans,
        ),
        application_group(
            "agent_harness_terminal_task_result_application",
            "p0",
            vec!["hepta_runtime_agent_harness"],
            &plans,
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_guards()
-> Vec<WorkGraphTerminalTaskResultEnforcementApplicationGuardPreview> {
    vec![
        application_guard("runtime_wrapper_attachment_disabled", "critical", "runtime"),
        application_guard("wrapper_execution_disabled", "critical", "runtime"),
        application_guard(
            "task_result_persistence_disabled",
            "critical",
            "task_result_store",
        ),
        application_guard(
            "terminal_task_result_enforcement_disabled",
            "critical",
            "task_result_enforcement",
        ),
        application_guard(
            "append_only_store_enablement_disabled",
            "critical",
            "append_only_store",
        ),
        application_guard(
            "scheduler_admission_enforcement_disabled",
            "high",
            "scheduler_admission",
        ),
        application_guard(
            "role_manifest_enforcement_disabled",
            "high",
            "role_manifest",
        ),
        application_guard("operator_review_required", "high", "operator_review"),
        application_guard(
            "enforcement_readiness_task_result_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_blockers()
-> Vec<WorkGraphTerminalTaskResultEnforcementApplicationBlockerPreview> {
    let plans = work_graph_terminal_task_result_enforcement_gap_closure_application_plans();
    let all_sources = affected_sources(&plans, |_| true);

    vec![
        blocker(
            "terminal_task_result_application_is_preview_only",
            "medium",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep terminal TaskResult application as a no-mutation preview until readiness rerun proves the blocker moved",
        ),
        blocker(
            "wrapper_runtime_attachment_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "attach terminal wrappers to runtime only after operator review, persistence, and replay gates are promoted",
        ),
        blocker(
            "wrapper_execution_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "do not execute terminal wrappers until application and readiness rerun previews pass",
        ),
        blocker(
            "task_result_persistence_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep TaskResult rows preview-only until append-only store intake is promoted",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "critical",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "keep terminal TaskResult enforcement disabled until readiness rerun proves no terminal route blocker remains",
        ),
        blocker(
            "scheduler_admission_or_role_manifest_residuals_not_enforced",
            "high",
            affected_sources(&plans, |plan| {
                closure_plan_for_application(plan)
                    .residual_source_blocker_ids
                    .iter()
                    .any(|blocker| {
                        blocker.ends_with("_admission_not_enforced")
                            || blocker.contains("role_manifest_not_enforced")
                    })
            }),
            application_plan_ids(&plans, |plan| {
                closure_plan_for_application(plan)
                    .residual_source_blocker_ids
                    .iter()
                    .any(|blocker| {
                        blocker.ends_with("_admission_not_enforced")
                            || blocker.contains("role_manifest_not_enforced")
                    })
            }),
            "preserve scheduler admission and role-manifest blockers as separate gates after TaskResult rerun",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "do not allow append-only writes until TaskResult application, replay, and operator readiness are promoted",
        ),
        blocker(
            "enforcement_readiness_task_result_rerun_missing",
            "high",
            all_sources.clone(),
            application_plan_ids(&plans, |_| true),
            "rerun unified projection enforcement-readiness against the terminal TaskResult application preview outcomes",
        ),
        blocker(
            "operator_review_required",
            "medium",
            all_sources,
            application_plan_ids(&plans, |_| true),
            "operator review must accept terminal wrapper bindings, evidence contracts, and enforcement routing before promotion",
        ),
    ]
}

pub fn work_graph_terminal_task_result_enforcement_gap_closure_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_terminal_task_result_enforcement_gap_closure_readback_required_prior_gates();
    if !gates
        .contains(&WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE)
    {
        gates.push(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE);
    }
    gates
}

impl WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewSideEffects {
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

fn application_plan(
    readback_plan: WorkGraphTerminalTaskResultEnforcementGapClosureReadbackPlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview {
    let closure_plan = closure_plan_for_readback(&readback_plan);
    WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview {
        application_plan_id: application_plan_id_for_source(readback_plan.source_surface_id),
        readback_plan_id: readback_plan.id,
        closure_plan_id: readback_plan.closure_plan_id,
        source_surface_id: readback_plan.source_surface_id,
        source_category: closure_plan.source_category,
        wrapper_id: readback_plan.wrapper_id,
        terminal_source_kind: closure_plan.terminal_source_kind,
        enforcement_binding_id: readback_plan.enforcement_binding_id,
        readback_probe_binding_id: readback_plan.readback_probe_binding_id,
        wrapper_readback_plan_id: readback_plan.wrapper_readback_plan_id,
        expected_evidence_contract_id: readback_plan.expected_evidence_contract_id,
        application_scope: "terminal_task_result_runtime_enforcement_binding",
        required_wire_fields: readback_plan.required_wire_fields,
        required_collection_assertion_ids: readback_plan.required_collection_assertion_ids,
        drift_detector_ids: readback_plan.drift_detector_ids,
        terminal_source_blocker_ids: closure_plan.terminal_source_blocker_ids,
        application_state: "preview_application_defined_terminal_task_result_enforcement_not_attached",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        attaches_runtime_wrapper: false,
        executes_wrapper: false,
        persists_task_result: false,
        enables_task_result_enforcement: false,
        enables_append_only_store: false,
        enforces_projection: false,
        mutates_store: false,
    }
}

fn source_outcome(
    plan: WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview {
    WorkGraphTerminalTaskResultEnforcementApplicationSourceOutcomePreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        wrapper_id: plan.wrapper_id,
        application_plan_id: plan.application_plan_id,
        post_application_terminal_task_result_state: "terminal_task_result_contract_ready_preview_after_application",
        terminal_task_result_contract_ready_preview: true,
        ready_for_enforcement_readiness_terminal_task_result_rerun: true,
        ready_for_task_result_enforcement: false,
        ready_for_projection_enforcement: false,
        applies_to_runtime: false,
    }
}

fn application_group(
    id: &'static str,
    priority: &'static str,
    source_surface_ids: Vec<&'static str>,
    plans: &[WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview],
) -> WorkGraphTerminalTaskResultEnforcementApplicationGroupPreview {
    let application_plan_ids = plans
        .iter()
        .filter(|plan| source_surface_ids.contains(&plan.source_surface_id))
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    WorkGraphTerminalTaskResultEnforcementApplicationGroupPreview {
        id,
        priority,
        expected_terminal_task_result_ready_source_count_after_application: source_surface_ids
            .len(),
        source_surface_ids,
        application_plan_ids,
        mutates_runtime: false,
        persists_task_result: false,
        enables_task_result_enforcement: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementApplicationGuardPreview {
    WorkGraphTerminalTaskResultEnforcementApplicationGuardPreview {
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
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphTerminalTaskResultEnforcementApplicationBlockerPreview {
    WorkGraphTerminalTaskResultEnforcementApplicationBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn affected_sources(
    plans: &[WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        if !source_ids.contains(&plan.source_surface_id) {
            source_ids.push(plan.source_surface_id);
        }
    }
    source_ids
}

fn application_plan_ids(
    plans: &[WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
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

fn closure_plan_for_application(
    application_plan: &WorkGraphTerminalTaskResultEnforcementApplicationPlanPreview,
) -> WorkGraphTerminalTaskResultEnforcementGapClosurePlanPreview {
    work_graph_terminal_task_result_enforcement_gap_closure_plans()
        .into_iter()
        .find(|plan| plan.id == application_plan.closure_plan_id)
        .unwrap_or_else(|| {
            panic!(
                "missing terminal TaskResult closure plan {}",
                application_plan.closure_plan_id
            )
        })
}

fn application_plan_id_for_source(source_surface_id: &str) -> String {
    format!("apply_{source_surface_id}_terminal_task_result_enforcement_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_enforcement_gap_closure_application_covers_readback_verified_plans() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report(
            );
        let sources = report
            .application_plans
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
        assert_eq!(report.readback_plan_count, 6);
        assert_eq!(report.application_plan_count, 6);
        assert_eq!(report.source_outcome_count, 6);
        assert_eq!(
            report.source_terminal_task_result_contract_ready_preview_count,
            6
        );
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn task_result_enforcement_gap_closure_application_preserves_no_mutation_boundary() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report(
            );

        assert_eq!(report.wire_field_ref_count, 66);
        assert_eq!(report.collection_assertion_ref_count, 18);
        assert_eq!(report.drift_detector_ref_count, 30);
        assert_eq!(report.terminal_source_blocker_ref_count, 6);
        assert!(report.application_plans.iter().all(|plan| {
            !plan.applies_to_runtime
                && !plan.attaches_runtime_wrapper
                && !plan.executes_wrapper
                && !plan.persists_task_result
                && !plan.enables_task_result_enforcement
                && !plan.enables_append_only_store
                && !plan.enforces_projection
                && !plan.mutates_store
        }));
    }

    #[test]
    fn task_result_enforcement_gap_closure_application_groups_sources_for_rerun() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report(
            );
        let group_counts = report
            .application_groups
            .iter()
            .map(|group| (group.id, group.application_plan_ids.len()))
            .collect::<Vec<_>>();
        let reducer = report
            .source_outcomes
            .iter()
            .find(|outcome| outcome.source_surface_id == "hepta_runtime_multi_agent_reducer")
            .expect("reducer outcome");

        assert_eq!(
            group_counts,
            [
                ("multi_agent_terminal_task_result_application", 2),
                ("batch_agent_jobs_terminal_task_result_application", 1),
                ("runtime_scheduler_terminal_task_result_application", 2),
                ("agent_harness_terminal_task_result_application", 1),
            ]
        );
        assert_eq!(report.application_group_count, 4);
        assert!(report.source_outcomes.iter().all(|outcome| {
            outcome.terminal_task_result_contract_ready_preview
                && outcome.ready_for_enforcement_readiness_terminal_task_result_rerun
                && !outcome.ready_for_task_result_enforcement
                && !outcome.ready_for_projection_enforcement
                && !outcome.applies_to_runtime
        }));
        assert_eq!(
            reducer.post_application_terminal_task_result_state,
            "terminal_task_result_contract_ready_preview_after_application"
        );
    }

    #[test]
    fn task_result_enforcement_gap_closure_application_preserves_blockers_and_next_frontier() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report(
            );
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.application_guard_count, 9);
        assert_eq!(report.blocker_count, 9);
        assert_eq!(
            blocker_counts,
            [
                ("terminal_task_result_application_is_preview_only", 6),
                ("wrapper_runtime_attachment_disabled", 6),
                ("wrapper_execution_disabled", 6),
                ("task_result_persistence_disabled", 6),
                ("terminal_task_result_enforcement_disabled", 6),
                (
                    "scheduler_admission_or_role_manifest_residuals_not_enforced",
                    5,
                ),
                ("append_only_store_enablement_disabled", 6),
                ("enforcement_readiness_task_result_rerun_missing", 6),
                ("operator_review_required", 6),
            ]
        );
        assert_eq!(report.required_prior_gate_count, 26);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_ENFORCEMENT_GAP_CLOSURE_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .ready_for_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview
        );
        assert!(!report.ready_for_task_result_enforcement);
    }

    #[test]
    fn task_result_enforcement_gap_closure_application_keeps_all_side_effects_disabled() {
        let report =
            hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report(
            );

        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultEnforcementGapClosureApplicationPreviewSideEffects::none()
        );
    }
}
