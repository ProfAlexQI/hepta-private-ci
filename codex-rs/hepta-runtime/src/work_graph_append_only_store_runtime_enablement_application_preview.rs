use serde::Serialize;

use crate::work_graph_append_only_store_runtime_enablement_readback_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::work_graph_append_only_store_runtime_blocker_mapping_readback_assertions;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::work_graph_append_only_store_runtime_enablement_readback_plans;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::work_graph_append_only_store_runtime_enablement_readback_required_prior_gates;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::work_graph_append_only_store_runtime_readback_blockers;
use crate::work_graph_append_only_store_runtime_enablement_readback_preview::work_graph_append_only_store_runtime_stage_plan_readback_assertions;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_enablement_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_RECOMMENDED_NEXT_GATE: &str = "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub runtime_enablement_contract_ready_preview_count: usize,
    pub stage_application_count: usize,
    pub blocker_application_count: usize,
    pub application_group_count: usize,
    pub runtime_plan_stage_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub stage_contract_ref_count: usize,
    pub stage_source_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview>,
    pub stage_applications: Vec<WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphAppendOnlyStoreRuntimeBlockerApplicationPreview>,
    pub application_groups: Vec<WorkGraphAppendOnlyStoreRuntimeApplicationGroupPreview>,
    pub application_guards: Vec<WorkGraphAppendOnlyStoreRuntimeApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview:
        bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_runtime_enablement_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub expected_runtime_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub records_approval: bool,
    pub promotes_runtime_application: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_runtime_enablement_state: &'static str,
    pub runtime_enablement_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview:
        bool,
    pub ready_for_append_only_store_enablement: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview {
    pub application_id: String,
    pub runtime_stage_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_contract_ref_ids: Vec<&'static str>,
    pub expected_stage_state: &'static str,
    pub stage_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub applies_to_runtime: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_stage_ids: Vec<&'static str>,
    pub affected_runtime_enablement_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_runtime_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeApplicationGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub runtime_stage_ids: Vec<&'static str>,
    pub stage_application_ids: Vec<String>,
    pub expected_stage_contract_ready_count_after_application: usize,
    pub mutates_runtime: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_append_only_store_runtime_enablement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_enablement_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_append_only_store_runtime_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_enablement_application_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewReport {
    let readback_plans = work_graph_append_only_store_runtime_enablement_readback_plans();
    let application_plans = work_graph_append_only_store_runtime_enablement_application_plans();
    let source_outcomes =
        work_graph_append_only_store_runtime_enablement_application_source_outcomes();
    let stage_applications = work_graph_append_only_store_runtime_enablement_stage_applications();
    let blocker_applications =
        work_graph_append_only_store_runtime_enablement_blocker_applications();
    let application_groups = work_graph_append_only_store_runtime_enablement_application_groups();
    let application_guards = work_graph_append_only_store_runtime_enablement_application_guards();
    let blockers = work_graph_append_only_store_runtime_enablement_application_blockers();
    let required_prior_gates =
        work_graph_append_only_store_runtime_enablement_application_required_prior_gates();

    WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_runtime_enablement_application_preview_no_runtime_mutation",
        readback_plan_count: readback_plans.len(),
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        runtime_enablement_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.runtime_enablement_contract_ready_preview)
            .count(),
        stage_application_count: stage_applications.len(),
        blocker_application_count: blocker_applications.len(),
        application_group_count: application_groups.len(),
        runtime_plan_stage_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_runtime_stage_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        stage_contract_ref_count: stage_applications
            .iter()
            .map(|stage| stage.expected_contract_ref_ids.len())
            .sum(),
        stage_source_ref_count: stage_applications
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_source_surface_ids.len())
            .sum(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        application_plans,
        source_outcomes,
        stage_applications,
        blocker_applications,
        application_groups,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview:
            true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_enablement_application_plans()
-> Vec<WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview> {
    work_graph_append_only_store_runtime_enablement_readback_plans()
        .into_iter()
        .map(application_plan)
        .collect()
}

pub fn work_graph_append_only_store_runtime_enablement_application_source_outcomes()
-> Vec<WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview> {
    work_graph_append_only_store_runtime_enablement_application_plans()
        .into_iter()
        .map(source_outcome)
        .collect()
}

pub fn work_graph_append_only_store_runtime_enablement_stage_applications()
-> Vec<WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview> {
    work_graph_append_only_store_runtime_stage_plan_readback_assertions()
        .into_iter()
        .map(stage_application)
        .collect()
}

pub fn work_graph_append_only_store_runtime_enablement_blocker_applications()
-> Vec<WorkGraphAppendOnlyStoreRuntimeBlockerApplicationPreview> {
    let plans = work_graph_append_only_store_runtime_enablement_application_plans();
    work_graph_append_only_store_runtime_blocker_mapping_readback_assertions()
        .into_iter()
        .map(|assertion| blocker_application(assertion, &plans))
        .collect()
}

pub fn work_graph_append_only_store_runtime_enablement_application_groups()
-> Vec<WorkGraphAppendOnlyStoreRuntimeApplicationGroupPreview> {
    let stage_applications = work_graph_append_only_store_runtime_enablement_stage_applications();
    vec![
        application_group(
            "append_only_store_runtime_core_application",
            "p0",
            vec!["durable_store_runtime_switch", "wal_write_boundary"],
            &stage_applications,
        ),
        application_group(
            "append_only_store_runtime_replay_safety_application",
            "p0",
            vec![
                "idempotency_mutation_policy",
                "rollback_readback_execution_gate",
            ],
            &stage_applications,
        ),
        application_group(
            "append_only_store_runtime_operator_lock_application",
            "p0",
            vec!["operator_review_side_effect_lock"],
            &stage_applications,
        ),
        application_group(
            "append_only_store_runtime_application_promotion_preview",
            "p0",
            vec!["runtime_application_promotion"],
            &stage_applications,
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_enablement_application_guards()
-> Vec<WorkGraphAppendOnlyStoreRuntimeApplicationGuardPreview> {
    vec![
        application_guard(
            "runtime_enablement_application_is_preview_only",
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
            "runtime_application_promotion_disabled",
            "high",
            "runtime_application",
        ),
        application_guard(
            "scheduler_role_runtime_application_disabled",
            "high",
            "scheduler_role",
        ),
        application_guard(
            "append_only_store_runtime_readiness_rerun_required",
            "high",
            "readiness_rerun",
        ),
        application_guard(
            "side_effect_lock_not_established",
            "critical",
            "side_effect_lock",
        ),
        application_guard(
            "graph_state_persistence_disabled",
            "critical",
            "graph_state_persistence",
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_enablement_application_blockers()
-> Vec<WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview> {
    let plans = work_graph_append_only_store_runtime_enablement_application_plans();
    let mut blockers = work_graph_append_only_store_runtime_readback_blockers()
        .into_iter()
        .map(|blocker| application_blocker_from_readback_blocker(blocker, &plans))
        .collect::<Vec<_>>();
    blockers.push(application_blocker(
        "append_only_store_runtime_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        affected_sources(&plans, |_| true),
        readback_runtime_plan_ids(&plans, |_| true),
        application_plan_ids(&plans, |_| true),
        "rerun unified projection enforcement-readiness against the append-only store runtime enablement application preview outcomes",
    ));
    blockers
}

pub fn work_graph_append_only_store_runtime_enablement_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_store_runtime_enablement_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewSideEffects {
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
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn application_plan(
    readback_plan: WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview,
) -> WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview {
    WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview {
        application_plan_id: application_plan_id_for(&readback_plan.runtime_enablement_plan_id),
        readback_runtime_enablement_plan_id: readback_plan.runtime_enablement_plan_id,
        source_surface_id: readback_plan.source_surface_id,
        source_category: readback_plan.source_category,
        expected_runtime_stage_ids: readback_plan.expected_runtime_stage_ids,
        expected_evidence_field_ids: readback_plan.expected_evidence_field_ids,
        residual_source_blocker_ids: readback_plan.residual_source_blocker_ids,
        application_scope: "append_only_store_runtime_enablement_application_binding",
        application_state: "preview_application_defined_runtime_enablement_not_applied",
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        enables_append_only_store: false,
        writes_wal: false,
        writes_checkpoint: false,
        mutates_idempotency_index: false,
        executes_readback: false,
        executes_rollback: false,
        records_approval: false,
        promotes_runtime_application: false,
        mutates_store: false,
    }
}

fn source_outcome(
    plan: WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview,
) -> WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview {
    WorkGraphAppendOnlyStoreRuntimeApplicationOutcomePreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        application_plan_id: plan.application_plan_id,
        post_application_runtime_enablement_state:
            "runtime_enablement_contract_ready_preview_after_application",
        runtime_enablement_contract_ready_preview: true,
        ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview:
            true,
        ready_for_append_only_store_enablement: false,
        applies_to_runtime: false,
    }
}

fn stage_application(
    assertion: WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview,
) -> WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview {
    WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview {
        application_id: stage_application_id_for(assertion.runtime_stage_id),
        runtime_stage_id: assertion.runtime_stage_id,
        category: assertion.category,
        affected_source_surface_ids: assertion.expected_source_surface_ids,
        expected_contract_ref_ids: assertion.expected_contract_ref_ids,
        expected_stage_state: "stage_contract_ready_preview_after_application_runtime_disabled",
        stage_contract_ready_preview: true,
        readback_verified_by_preview: true,
        applies_to_runtime: false,
        enables_append_only_store: false,
        writes_wal: false,
        mutates_idempotency_index: false,
        executes_readback: false,
    }
}

fn blocker_application(
    assertion: WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview,
    plans: &[WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview],
) -> WorkGraphAppendOnlyStoreRuntimeBlockerApplicationPreview {
    let affected_application_plan_ids = application_plan_ids(plans, |plan| {
        assertion
            .affected_runtime_enablement_plan_ids
            .contains(&plan.readback_runtime_enablement_plan_id)
    });
    WorkGraphAppendOnlyStoreRuntimeBlockerApplicationPreview {
        application_id: blocker_application_id_for(assertion.blocker_id),
        blocker_id: assertion.blocker_id,
        category: assertion.category,
        affected_source_surface_ids: assertion.affected_source_surface_ids,
        affected_runtime_stage_ids: assertion.affected_runtime_stage_ids,
        affected_runtime_enablement_plan_ids: assertion.affected_runtime_enablement_plan_ids,
        affected_application_plan_ids,
        expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
        blocker_contract_ready_preview: true,
        readback_verified_by_preview: true,
        clears_runtime_blocker: false,
        mutates_runtime: false,
    }
}

fn application_group(
    id: &'static str,
    priority: &'static str,
    runtime_stage_ids: Vec<&'static str>,
    stages: &[WorkGraphAppendOnlyStoreRuntimeStageApplicationPreview],
) -> WorkGraphAppendOnlyStoreRuntimeApplicationGroupPreview {
    let stage_application_ids = stages
        .iter()
        .filter(|stage| runtime_stage_ids.contains(&stage.runtime_stage_id))
        .map(|stage| stage.application_id.clone())
        .collect::<Vec<_>>();
    WorkGraphAppendOnlyStoreRuntimeApplicationGroupPreview {
        id,
        priority,
        expected_stage_contract_ready_count_after_application: runtime_stage_ids.len(),
        runtime_stage_ids,
        stage_application_ids,
        mutates_runtime: false,
        enables_append_only_store: false,
        writes_wal: false,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeApplicationGuardPreview {
    WorkGraphAppendOnlyStoreRuntimeApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_append_only_store_runtime_enablement: true,
        satisfied_by_preview: false,
    }
}

fn application_blocker_from_readback_blocker(
    blocker: WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview,
    plans: &[WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview],
) -> WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids,
        blocker.affected_runtime_enablement_plan_ids.clone(),
        application_plan_ids(plans, |plan| {
            blocker
                .affected_runtime_enablement_plan_ids
                .contains(&plan.readback_runtime_enablement_plan_id)
        }),
        blocker.recommended_fix,
    )
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_runtime_enablement_plan_ids: Vec<String>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview {
    WorkGraphAppendOnlyStoreRuntimeApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_runtime_enablement_plan_ids,
        affected_application_plan_ids,
        required_before_append_only_store_runtime_enablement: true,
        recommended_fix,
    }
}

fn application_plan_ids(
    plans: &[WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn readback_runtime_plan_ids(
    plans: &[WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.readback_runtime_enablement_plan_id.clone())
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphAppendOnlyStoreRuntimeApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        if !source_ids.contains(&plan.source_surface_id) {
            source_ids.push(plan.source_surface_id);
        }
    }
    source_ids
}

fn application_plan_id_for(runtime_enablement_plan_id: &str) -> String {
    format!("apply_{runtime_enablement_plan_id}_runtime_enablement_preview")
}

fn stage_application_id_for(runtime_stage_id: &str) -> String {
    format!("apply_{runtime_stage_id}_runtime_stage_preview")
}

fn blocker_application_id_for(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_runtime_blocker_mapping_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_enablement_application_covers_readback_verified_plans_without_mutation() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_application_preview_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(report.application_plan_count, 12);
        assert_eq!(report.source_outcome_count, 12);
        assert_eq!(report.runtime_enablement_contract_ready_preview_count, 12);
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview
                    && plan.expected_runtime_stage_ids.len() == 6
                    && plan.expected_evidence_field_ids.len() == 8
                    && !plan.applies_to_runtime
                    && !plan.enables_append_only_store
                    && !plan.mutates_idempotency_index
                    && !plan.mutates_store)
        );
        assert_eq!(report.runtime_plan_stage_ref_count, 72);
        assert_eq!(report.evidence_field_ref_count, 96);
        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreRuntimeEnablementApplicationPreviewSideEffects::none()
        );
    }

    #[test]
    fn runtime_enablement_application_preserves_stage_and_blocker_mapping_counts() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_application_preview_report();

        assert_eq!(report.stage_application_count, 6);
        assert_eq!(report.blocker_application_count, 13);
        assert_eq!(report.stage_contract_ref_count, 29);
        assert_eq!(report.stage_source_ref_count, 62);
        assert_eq!(report.blocker_mapping_source_ref_count, 113);
        assert!(report.stage_applications.iter().all(|stage| {
            stage.stage_contract_ready_preview
                && stage.readback_verified_by_preview
                && !stage.applies_to_runtime
                && !stage.enables_append_only_store
                && !stage.writes_wal
                && !stage.mutates_idempotency_index
                && !stage.executes_readback
        }));
        assert!(report.blocker_applications.iter().all(|application| {
            application.blocker_contract_ready_preview
                && application.readback_verified_by_preview
                && !application.clears_runtime_blocker
                && !application.mutates_runtime
        }));
    }

    #[test]
    fn runtime_enablement_application_groups_blockers_and_next_gate_are_stable() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_application_preview_report();
        let groups = report
            .application_groups
            .iter()
            .map(|group| (group.id, group.stage_application_ids.len()))
            .collect::<Vec<_>>();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| {
                (
                    blocker.id,
                    blocker.affected_source_surface_ids.len(),
                    blocker.affected_application_plan_ids.len(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            groups,
            [
                ("append_only_store_runtime_core_application", 2),
                ("append_only_store_runtime_replay_safety_application", 2),
                ("append_only_store_runtime_operator_lock_application", 1),
                ("append_only_store_runtime_application_promotion_preview", 1),
            ]
        );
        assert_eq!(report.application_guard_count, 11);
        assert_eq!(report.blocker_count, 15);
        assert_eq!(blocker_counts[0], ("readback_execution_disabled", 12, 12));
        assert_eq!(
            blocker_counts.last().copied(),
            Some(("append_only_store_runtime_readiness_rerun_missing", 12, 12))
        );
        assert!(
            report
                .application_groups
                .iter()
                .all(|group| !group.mutates_runtime
                    && !group.enables_append_only_store
                    && !group.writes_wal)
        );
        assert!(report.application_guards.iter().all(|guard| {
            guard.required_before_append_only_store_runtime_enablement
                && !guard.satisfied_by_preview
        }));
        assert_eq!(report.required_prior_gate_count, 42);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(
            report
                .ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview
        );
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
    }
}
