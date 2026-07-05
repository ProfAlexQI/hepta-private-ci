use serde::Serialize;

use crate::work_graph_append_only_store_enablement_precondition_readback_preview::WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::work_graph_append_only_store_enablement_precondition_blocker_mapping_readback_assertions;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::work_graph_append_only_store_enablement_precondition_readback_blockers;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::work_graph_append_only_store_enablement_precondition_readback_plans;
use crate::work_graph_append_only_store_enablement_precondition_readback_preview::work_graph_append_only_store_enablement_precondition_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_enablement_precondition_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub precondition_outcome_count: usize,
    pub precondition_contract_ready_preview_count: usize,
    pub blocker_application_count: usize,
    pub application_group_count: usize,
    pub contract_ref_count: usize,
    pub source_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview>,
    pub precondition_outcomes: Vec<WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview>,
    pub blocker_applications:
        Vec<WorkGraphAppendOnlyStorePreconditionBlockerMappingApplicationPreview>,
    pub application_groups: Vec<WorkGraphAppendOnlyStorePreconditionApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphAppendOnlyStorePreconditionApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_precondition_id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_contract_ref_ids: Vec<&'static str>,
    pub expected_blocker_id: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_precondition_state: bool,
    pub enables_append_only_store: bool,
    pub mutates_store: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub enforces_scheduler_admission: bool,
    pub enforces_role_manifest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview {
    pub precondition_id: &'static str,
    pub category: &'static str,
    pub application_plan_id: String,
    pub post_application_precondition_state: &'static str,
    pub precondition_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionBlockerMappingApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub affected_precondition_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_runtime_blocker: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub precondition_ids: Vec<&'static str>,
    pub application_plan_ids: Vec<String>,
    pub expected_precondition_contract_ready_count_after_application: usize,
    pub mutates_runtime: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_append_only_store_enablement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_precondition_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_append_only_store_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub precondition_state_persisted: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub runtime_wrapper_attached: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_enablement_precondition_application_preview_report()
-> WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewReport {
    let readback_plans = work_graph_append_only_store_enablement_precondition_readback_plans();
    let application_plans =
        work_graph_append_only_store_enablement_precondition_application_plans();
    let precondition_outcomes =
        work_graph_append_only_store_enablement_precondition_application_outcomes();
    let blocker_applications =
        work_graph_append_only_store_enablement_precondition_blocker_mapping_applications();
    let application_groups =
        work_graph_append_only_store_enablement_precondition_application_groups();
    let application_guards =
        work_graph_append_only_store_enablement_precondition_application_guards();
    let blockers = work_graph_append_only_store_enablement_precondition_application_blockers();
    let required_prior_gates =
        work_graph_append_only_store_enablement_precondition_application_required_prior_gates();

    WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_enablement_precondition_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        precondition_outcome_count: precondition_outcomes.len(),
        precondition_contract_ready_preview_count: precondition_outcomes
            .iter()
            .filter(|outcome| outcome.precondition_contract_ready_preview)
            .count(),
        blocker_application_count: blocker_applications.len(),
        application_group_count: application_groups.len(),
        contract_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_contract_ref_ids.len())
            .sum(),
        source_ref_count: application_plans
            .iter()
            .map(|plan| plan.affected_source_surface_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_evidence_fields.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_source_surface_ids.len())
            .sum(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        application_plans,
        precondition_outcomes,
        blocker_applications,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_enablement_precondition_application_plans()
-> Vec<WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview> {
    work_graph_append_only_store_enablement_precondition_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_application_outcomes()
-> Vec<WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview> {
    work_graph_append_only_store_enablement_precondition_application_plans()
        .into_iter()
        .map(precondition_outcome)
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_blocker_mapping_applications()
-> Vec<WorkGraphAppendOnlyStorePreconditionBlockerMappingApplicationPreview> {
    work_graph_append_only_store_enablement_precondition_blocker_mapping_readback_assertions()
        .into_iter()
        .map(blocker_mapping_application)
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_application_groups()
-> Vec<WorkGraphAppendOnlyStorePreconditionApplicationGroupPreview> {
    let plans = work_graph_append_only_store_enablement_precondition_application_plans();
    vec![
        application_group(
            "append_only_store_core_precondition_application",
            "p0",
            vec![
                "durable_store_enablement_switch",
                "wal_append_boundary_contract",
            ],
            &plans,
        ),
        application_group(
            "append_only_replay_safety_precondition_application",
            "p0",
            vec!["idempotency_mutation_policy", "rollback_readback_gate"],
            &plans,
        ),
        application_group(
            "append_only_operator_lock_precondition_application",
            "p0",
            vec!["operator_review_and_side_effect_lock"],
            &plans,
        ),
        application_group(
            "append_only_scheduler_role_precondition_application",
            "p0",
            vec![
                "scheduler_admission_enforcement_precondition",
                "role_manifest_enforcement_precondition",
            ],
            &plans,
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_precondition_application_guards()
-> Vec<WorkGraphAppendOnlyStorePreconditionApplicationGuardPreview> {
    vec![
        application_guard(
            "precondition_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        application_guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        application_guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_index",
        ),
        application_guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback_readback",
        ),
        application_guard("operator_review_required", "high", "operator_review"),
        application_guard(
            "scheduler_admission_not_enforced",
            "high",
            "scheduler_admission",
        ),
        application_guard("role_manifest_not_enforced", "high", "role_manifest"),
        application_guard(
            "runtime_application_residuals_not_promoted",
            "high",
            "runtime_application",
        ),
        application_guard(
            "append_only_store_readiness_rerun_required",
            "high",
            "readiness_rerun",
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_precondition_application_blockers()
-> Vec<WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview> {
    let plans = work_graph_append_only_store_enablement_precondition_application_plans();
    let mut blockers = work_graph_append_only_store_enablement_precondition_readback_blockers()
        .into_iter()
        .map(|blocker| application_blocker_from_readback_blocker(blocker, &plans))
        .collect::<Vec<_>>();
    blockers.push(application_blocker(
        "append_only_store_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        plans
            .iter()
            .map(|plan| plan.readback_precondition_id)
            .collect(),
        affected_sources(&plans, |_| true),
        application_plan_ids(&plans, |_| true),
        "rerun unified projection enforcement-readiness against the append-only store precondition application preview outcomes",
    ));
    blockers
}

pub fn work_graph_append_only_store_enablement_precondition_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_enablement_precondition_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            precondition_state_persisted: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_executed: false,
            rollback_executed: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            runtime_wrapper_attached: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn application_plan(
    readback_plan: WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview,
) -> WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview {
    WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview {
        application_plan_id: application_plan_id_for_precondition(readback_plan.precondition_id),
        readback_precondition_id: readback_plan.precondition_id,
        category: readback_plan.category,
        severity: readback_plan.severity,
        affected_source_surface_ids: readback_plan.affected_source_surface_ids,
        expected_contract_ref_ids: readback_plan.expected_contract_ref_ids,
        expected_blocker_id: readback_plan.expected_blocker_id,
        required_evidence_fields: readback_plan.required_evidence_fields,
        application_scope: "append_only_store_enablement_precondition_runtime_binding",
        application_state: "preview_application_defined_precondition_not_applied_to_runtime",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        persists_precondition_state: false,
        enables_append_only_store: false,
        mutates_store: false,
        writes_wal: false,
        writes_checkpoint: false,
        mutates_idempotency_index: false,
        enforces_scheduler_admission: false,
        enforces_role_manifest: false,
    }
}

fn precondition_outcome(
    plan: WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview,
) -> WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview {
    WorkGraphAppendOnlyStorePreconditionApplicationOutcomePreview {
        precondition_id: plan.readback_precondition_id,
        category: plan.category,
        application_plan_id: plan.application_plan_id,
        post_application_precondition_state: "precondition_contract_ready_preview_after_application",
        precondition_contract_ready_preview: true,
        ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: true,
        ready_for_append_only_store_enablement: false,
        applies_to_runtime: false,
    }
}

fn blocker_mapping_application(
    assertion: WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview,
) -> WorkGraphAppendOnlyStorePreconditionBlockerMappingApplicationPreview {
    WorkGraphAppendOnlyStorePreconditionBlockerMappingApplicationPreview {
        application_id: blocker_application_id_for_blocker(assertion.blocker_id),
        blocker_id: assertion.blocker_id,
        category: assertion.category,
        affected_precondition_ids: assertion.affected_precondition_ids,
        affected_source_surface_ids: assertion.affected_source_surface_ids,
        expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
        blocker_contract_ready_preview: true,
        readback_verified_by_preview: true,
        clears_runtime_blocker: false,
        mutates_store: false,
    }
}

fn application_group(
    id: &'static str,
    priority: &'static str,
    precondition_ids: Vec<&'static str>,
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
) -> WorkGraphAppendOnlyStorePreconditionApplicationGroupPreview {
    let application_plan_ids = plans
        .iter()
        .filter(|plan| precondition_ids.contains(&plan.readback_precondition_id))
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    WorkGraphAppendOnlyStorePreconditionApplicationGroupPreview {
        id,
        priority,
        expected_precondition_contract_ready_count_after_application: precondition_ids.len(),
        precondition_ids,
        application_plan_ids,
        mutates_runtime: false,
        enables_append_only_store: false,
        writes_wal: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphAppendOnlyStorePreconditionApplicationGuardPreview {
    WorkGraphAppendOnlyStorePreconditionApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_append_only_store_enablement: true,
        satisfied_by_preview: false,
    }
}

fn application_blocker_from_readback_blocker(
    blocker: WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview,
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
) -> WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_precondition_ids.clone(),
        blocker.affected_source_surface_ids,
        application_plan_ids(plans, |plan| {
            blocker
                .affected_precondition_ids
                .contains(&plan.readback_precondition_id)
        }),
        blocker.recommended_fix,
    )
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_precondition_ids: Vec<&'static str>,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview {
    WorkGraphAppendOnlyStorePreconditionApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_precondition_ids,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_append_only_store_enablement: true,
        recommended_fix,
    }
}

fn application_plan_ids(
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStorePreconditionApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        for source_id in &plan.affected_source_surface_ids {
            if !source_ids.contains(source_id) {
                source_ids.push(*source_id);
            }
        }
    }
    source_ids
}

fn application_plan_id_for_precondition(precondition_id: &str) -> String {
    format!("apply_{precondition_id}_append_only_store_precondition_preview")
}

fn blocker_application_id_for_blocker(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_append_only_store_blocker_mapping_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_store_precondition_application_covers_readback_verified_plans() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_application_preview_report();
        let plan_summary = report
            .application_plans
            .iter()
            .map(|plan| {
                (
                    plan.readback_precondition_id,
                    plan.expected_contract_ref_ids.len(),
                    plan.affected_source_surface_ids.len(),
                    plan.required_evidence_fields.len(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            plan_summary,
            [
                ("durable_store_enablement_switch", 9, 12, 5),
                ("wal_append_boundary_contract", 6, 12, 5),
                ("idempotency_mutation_policy", 12, 12, 5),
                ("rollback_readback_gate", 10, 12, 5),
                ("operator_review_and_side_effect_lock", 3, 6, 5),
                ("scheduler_admission_enforcement_precondition", 5, 5, 6),
                ("role_manifest_enforcement_precondition", 4, 4, 5),
            ]
        );
        assert_eq!(report.readback_plan_count, 7);
        assert_eq!(report.application_plan_count, 7);
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn append_only_store_precondition_application_preserves_no_mutation_boundary() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_application_preview_report();

        assert_eq!(report.contract_ref_count, 49);
        assert_eq!(report.source_ref_count, 63);
        assert_eq!(report.evidence_field_ref_count, 36);
        assert_eq!(report.blocker_mapping_source_ref_count, 70);
        assert!(report.application_plans.iter().all(|plan| {
            !plan.applies_to_runtime
                && !plan.persists_precondition_state
                && !plan.enables_append_only_store
                && !plan.mutates_store
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.mutates_idempotency_index
                && !plan.enforces_scheduler_admission
                && !plan.enforces_role_manifest
        }));
    }

    #[test]
    fn append_only_store_precondition_application_marks_outcomes_ready_for_rerun_only() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_application_preview_report();

        assert_eq!(report.precondition_outcome_count, 7);
        assert_eq!(report.precondition_contract_ready_preview_count, 7);
        assert!(report.precondition_outcomes.iter().all(|outcome| {
            outcome.precondition_contract_ready_preview
                && outcome
                    .ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview
                && !outcome.ready_for_append_only_store_enablement
                && !outcome.applies_to_runtime
        }));
    }

    #[test]
    fn append_only_store_precondition_application_declares_groups_guards_and_blockers() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_application_preview_report();
        let group_counts = report
            .application_groups
            .iter()
            .map(|group| (group.id, group.application_plan_ids.len()))
            .collect::<Vec<_>>();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            group_counts,
            [
                ("append_only_store_core_precondition_application", 2),
                ("append_only_replay_safety_precondition_application", 2),
                ("append_only_operator_lock_precondition_application", 1),
                ("append_only_scheduler_role_precondition_application", 2),
            ]
        );
        assert_eq!(report.application_group_count, 4);
        assert_eq!(report.application_guard_count, 10);
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_append_only_store_enablement && !guard.satisfied_by_preview
        }));
        assert_eq!(
            blocker_counts,
            [
                ("readback_execution_disabled", 12),
                ("durable_store_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("runtime_application_residuals_not_promoted", 7),
                ("append_only_store_readiness_rerun_missing", 12),
            ]
        );
        assert_eq!(report.blocker_count, 10);
        assert_eq!(report.blocker_application_count, 8);
    }

    #[test]
    fn append_only_store_precondition_application_advances_only_to_readiness_rerun() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_application_preview_report();

        assert_eq!(report.required_prior_gate_count, 30);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview
        );
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreEnablementPreconditionApplicationPreviewSideEffects::none()
        );
    }
}
