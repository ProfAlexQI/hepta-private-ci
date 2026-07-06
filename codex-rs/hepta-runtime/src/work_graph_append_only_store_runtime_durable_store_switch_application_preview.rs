use serde::Serialize;

use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchReadbackBlockerPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchReadbackPlanPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::WorkGraphDurableStoreSwitchStageReadbackAssertionPreview;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_report;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::work_graph_append_only_store_runtime_durable_store_switch_readback_plans;
use crate::work_graph_append_only_store_runtime_durable_store_switch_readback_preview::work_graph_append_only_store_runtime_durable_store_switch_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_SCHEMA_VERSION:
    &str = "work_graph_append_only_store_runtime_durable_store_switch_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub durable_store_switch_contract_ready_preview_count: usize,
    pub stage_application_count: usize,
    pub evidence_field_application_count: usize,
    pub guard_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub stage_source_ref_count: usize,
    pub stage_contract_ref_count: usize,
    pub plan_stage_ref_count: usize,
    pub evidence_field_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub blocker_mapping_stage_ref_count: usize,
    pub application_plans: Vec<WorkGraphDurableStoreSwitchApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphDurableStoreSwitchApplicationSourceOutcomePreview>,
    pub stage_applications: Vec<WorkGraphDurableStoreSwitchStageApplicationPreview>,
    pub evidence_field_applications:
        Vec<WorkGraphDurableStoreSwitchEvidenceFieldApplicationPreview>,
    pub guard_applications: Vec<WorkGraphDurableStoreSwitchGuardApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphDurableStoreSwitchBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphDurableStoreSwitchApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphDurableStoreSwitchApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview:
        bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_durable_store_switch: bool,
    pub ready_for_idempotency_mutation: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub durable_store_switch_plan_id: String,
    pub required_durable_store_switch_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub application_scope: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub durable_store_switch_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_durable_store_switch_state: &'static str,
    pub durable_store_switch_contract_ready_preview: bool,
    pub ready_for_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview:
        bool,
    pub ready_for_wal_write: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchStageApplicationPreview {
    pub application_id: String,
    pub stage_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_stage_state: &'static str,
    pub stage_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub declared_writes_wal: bool,
    pub declared_writes_checkpoint: bool,
    pub declared_switches_durable_store: bool,
    pub declared_mutates_idempotency_index: bool,
    pub declared_executes_replay: bool,
    pub declared_executes_readback: bool,
    pub declared_executes_rollback: bool,
    pub enables_runtime_after_application: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub switches_durable_store: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchEvidenceFieldApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub expected_evidence_state: &'static str,
    pub evidence_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub persists_evidence: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchGuardApplicationPreview {
    pub application_id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub guard_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub satisfied_by_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_durable_store_switch_stage_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub affected_application_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocker_contract_ready_preview: bool,
    pub readback_verified_by_preview: bool,
    pub clears_durable_store_switch_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_durable_store_switch: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableStoreSwitchApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_durable_store_switch_stage_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_durable_store_switch: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub readback_executed: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_report();
    let application_plans = application_plans_from(&readback_report.readback_plans);
    let source_outcomes = source_outcomes_from(&application_plans);
    let stage_applications = stage_applications_from(&readback_report.stage_assertions);
    let evidence_field_applications =
        evidence_field_applications_from(&readback_report.evidence_field_assertions);
    let guard_applications = guard_applications_from(&readback_report.guard_assertions);
    let blocker_applications = blocker_applications_from(
        &readback_report.blocker_mapping_assertions,
        &application_plans,
    );
    let application_guards =
        work_graph_append_only_store_runtime_durable_store_switch_application_guards();
    let blockers = application_blockers_from(&readback_report.blockers, &application_plans);
    let required_prior_gates =
        work_graph_append_only_store_runtime_durable_store_switch_application_required_prior_gates(
        );

    WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_durable_store_switch_application_no_store_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        durable_store_switch_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.durable_store_switch_contract_ready_preview)
            .count(),
        stage_application_count: stage_applications.len(),
        evidence_field_application_count: evidence_field_applications.len(),
        guard_application_count: guard_applications.len(),
        blocker_application_count: blocker_applications.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        stage_source_ref_count: stage_applications
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        stage_contract_ref_count: stage_applications
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        plan_stage_ref_count: application_plans
            .iter()
            .map(|plan| plan.required_durable_store_switch_stage_ids.len())
            .sum(),
        evidence_field_ref_count: application_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        blocker_mapping_source_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_source_surface_ids.len())
            .sum(),
        blocker_mapping_stage_ref_count: blocker_applications
            .iter()
            .map(|application| application.affected_durable_store_switch_stage_ids.len())
            .sum(),
        application_plans,
        source_outcomes,
        stage_applications,
        evidence_field_applications,
        guard_applications,
        blocker_applications,
        application_guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview:
            true,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_durable_store_switch: false,
        ready_for_idempotency_mutation: false,
        ready_for_readback_execution: false,
        ready_for_replay_execution: false,
        ready_for_rollback_execution: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_application_plans()
-> Vec<WorkGraphDurableStoreSwitchApplicationPlanPreview> {
    let readback_plans = work_graph_append_only_store_runtime_durable_store_switch_readback_plans();
    application_plans_from(&readback_plans)
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_application_source_outcomes()
-> Vec<WorkGraphDurableStoreSwitchApplicationSourceOutcomePreview> {
    let application_plans =
        work_graph_append_only_store_runtime_durable_store_switch_application_plans();
    source_outcomes_from(&application_plans)
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_application_guards()
-> Vec<WorkGraphDurableStoreSwitchApplicationGuardPreview> {
    vec![
        application_guard(
            "durable_store_switch_application_is_preview_only",
            "medium",
            "application_preview",
        ),
        application_guard("readback_execution_disabled", "critical", "readback"),
        application_guard("wal_write_boundary_disabled", "critical", "wal_boundary"),
        application_guard("checkpoint_write_disabled", "critical", "checkpoint"),
        application_guard("replay_execution_disabled", "critical", "replay"),
        application_guard(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
        ),
        application_guard("idempotency_mutation_disabled", "critical", "idempotency"),
        application_guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback_readback",
        ),
        application_guard(
            "append_only_store_enablement_disabled",
            "critical",
            "append_only_store",
        ),
        application_guard("runtime_mutation_disabled", "critical", "runtime_mutation"),
        application_guard("model_invocation_disabled", "high", "model_boundary"),
    ]
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_application_blockers()
-> Vec<WorkGraphDurableStoreSwitchApplicationBlockerPreview> {
    let readback_report =
        hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_report();
    let application_plans = application_plans_from(&readback_report.readback_plans);
    application_blockers_from(&readback_report.blockers, &application_plans)
}

pub fn work_graph_append_only_store_runtime_durable_store_switch_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_runtime_durable_store_switch_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            role_manifest_enforcement_enabled: false,
            readback_executed: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn application_plans_from(
    readback_plans: &[WorkGraphDurableStoreSwitchReadbackPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphDurableStoreSwitchApplicationPlanPreview {
            application_plan_id: application_plan_id_for(&plan.id),
            readback_plan_id: plan.id.clone(),
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            durable_store_switch_plan_id: plan.durable_store_switch_plan_id.clone(),
            required_durable_store_switch_stage_ids: plan
                .required_durable_store_switch_stage_ids
                .clone(),
            residual_source_blocker_ids: plan.residual_source_blocker_ids.clone(),
            expected_evidence_field_ids: plan.required_evidence_field_ids.clone(),
            application_scope: "durable_store_switch_application_binding",
            application_state: "preview_application_defined_durable_store_switch_not_enabled",
            readback_verified_by_preview: true,
            durable_store_switch_contract_ready_preview: true,
            applies_to_runtime: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn source_outcomes_from(
    application_plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(
            |plan| WorkGraphDurableStoreSwitchApplicationSourceOutcomePreview {
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_durable_store_switch_state:
                    "durable_store_switch_contract_ready_preview_after_application",
                durable_store_switch_contract_ready_preview: true,
                ready_for_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview: true,
                ready_for_wal_write: false,
                applies_to_runtime: false,
            },
        )
        .collect()
}

fn stage_applications_from(
    assertions: &[WorkGraphDurableStoreSwitchStageReadbackAssertionPreview],
) -> Vec<WorkGraphDurableStoreSwitchStageApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphDurableStoreSwitchStageApplicationPreview {
            application_id: stage_application_id_for(assertion.stage_id),
            stage_id: assertion.stage_id,
            category: assertion.category,
            affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
            required_contract_ref_ids: assertion.required_contract_ref_ids.clone(),
            expected_stage_state:
                "stage_contract_ready_preview_after_application_runtime_disabled",
            stage_contract_ready_preview: true,
            readback_verified_by_preview: true,
            declared_writes_wal: assertion.declared_writes_wal,
            declared_writes_checkpoint: assertion.declared_writes_checkpoint,
            declared_switches_durable_store: assertion.declared_switches_durable_store,
            declared_mutates_idempotency_index: assertion.declared_mutates_idempotency_index,
            declared_executes_replay: assertion.declared_executes_replay,
            declared_executes_readback: assertion.declared_executes_readback,
            declared_executes_rollback: assertion.declared_executes_rollback,
            enables_runtime_after_application: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_runtime: false,
        })
        .collect()
}

fn evidence_field_applications_from(
    assertions: &[WorkGraphDurableStoreSwitchEvidenceFieldReadbackAssertionPreview],
) -> Vec<WorkGraphDurableStoreSwitchEvidenceFieldApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphDurableStoreSwitchEvidenceFieldApplicationPreview {
                application_id: evidence_field_application_id_for(assertion.source_surface_id),
                source_surface_id: assertion.source_surface_id,
                required_evidence_field_ids: assertion.required_evidence_field_ids.clone(),
                expected_evidence_state:
                    "evidence_contract_ready_preview_after_application_not_persisted",
                evidence_contract_ready_preview: true,
                readback_verified_by_preview: true,
                persists_evidence: false,
                writes_store: false,
            },
        )
        .collect()
}

fn guard_applications_from(
    assertions: &[WorkGraphDurableStoreSwitchGuardReadbackAssertionPreview],
) -> Vec<WorkGraphDurableStoreSwitchGuardApplicationPreview> {
    assertions
        .iter()
        .map(|assertion| WorkGraphDurableStoreSwitchGuardApplicationPreview {
            application_id: guard_application_id_for(assertion.guard_id),
            guard_id: assertion.guard_id,
            severity: assertion.severity,
            guard_scope: assertion.guard_scope,
            expected_guard_state:
                "guard_contract_ready_preview_after_application_runtime_mutation_prevented",
            guard_contract_ready_preview: true,
            readback_verified_by_preview: true,
            satisfied_by_preview: false,
            mutates_runtime: false,
        })
        .collect()
}

fn blocker_applications_from(
    assertions: &[WorkGraphDurableStoreSwitchBlockerMappingReadbackAssertionPreview],
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchBlockerApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphDurableStoreSwitchBlockerApplicationPreview {
                application_id: blocker_application_id_for(assertion.blocker_id),
                blocker_id: assertion.blocker_id,
                severity: assertion.severity,
                category: assertion.category,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                affected_durable_store_switch_stage_ids: assertion
                    .affected_durable_store_switch_stage_ids
                    .clone(),
                affected_readback_plan_ids: assertion.affected_readback_plan_ids.clone(),
                affected_application_plan_ids: application_plan_ids_for_readback_plans(
                    plans,
                    &assertion.affected_readback_plan_ids,
                ),
                expected_blocker_state:
                    "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
                blocker_contract_ready_preview: true,
                readback_verified_by_preview: true,
                clears_durable_store_switch_blocker: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn application_blockers_from(
    readback_blockers: &[WorkGraphDurableStoreSwitchReadbackBlockerPreview],
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
) -> Vec<WorkGraphDurableStoreSwitchApplicationBlockerPreview> {
    let mut blockers = readback_blockers
        .iter()
        .map(|blocker| application_blocker_from_readback_blocker(blocker, plans))
        .collect::<Vec<_>>();
    blockers.push(application_blocker(
        "durable_store_switch_readiness_rerun_missing",
        "high",
        "readiness_rerun",
        affected_sources(plans, |_| true),
        affected_stages(plans, |_| true),
        application_plan_ids(plans, |_| true),
        "rerun unified projection enforcement-readiness against durable-store switch application preview outcomes",
    ));
    blockers
}

fn application_blocker_from_readback_blocker(
    blocker: &WorkGraphDurableStoreSwitchReadbackBlockerPreview,
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
) -> WorkGraphDurableStoreSwitchApplicationBlockerPreview {
    application_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_source_surface_ids.clone(),
        blocker.affected_durable_store_switch_stage_ids.clone(),
        application_plan_ids_for_sources(plans, &blocker.affected_source_surface_ids),
        blocker.recommended_fix,
    )
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_durable_store_switch_stage_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphDurableStoreSwitchApplicationBlockerPreview {
    WorkGraphDurableStoreSwitchApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_durable_store_switch_stage_ids,
        affected_application_plan_ids,
        required_before_durable_store_switch: true,
        recommended_fix,
    }
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphDurableStoreSwitchApplicationGuardPreview {
    WorkGraphDurableStoreSwitchApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_durable_store_switch: true,
        satisfied_by_preview: false,
    }
}

fn application_plan_ids_for_sources(
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
    source_ids: &[&'static str],
) -> Vec<String> {
    application_plan_ids(plans, |plan| source_ids.contains(&plan.source_surface_id))
}

fn application_plan_ids_for_readback_plans(
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
    readback_plan_ids: &[String],
) -> Vec<String> {
    application_plan_ids(plans, |plan| {
        readback_plan_ids.contains(&plan.readback_plan_id)
    })
}

fn application_plan_ids(
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphDurableStoreSwitchApplicationPlanPreview) -> bool,
) -> Vec<String> {
    plans
        .iter()
        .filter(|plan| predicate(plan))
        .map(|plan| plan.application_plan_id.clone())
        .collect()
}

fn affected_sources(
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphDurableStoreSwitchApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        if !source_ids.contains(&plan.source_surface_id) {
            source_ids.push(plan.source_surface_id);
        }
    }
    source_ids
}

fn affected_stages(
    plans: &[WorkGraphDurableStoreSwitchApplicationPlanPreview],
    predicate: impl Fn(&WorkGraphDurableStoreSwitchApplicationPlanPreview) -> bool,
) -> Vec<&'static str> {
    let mut stage_ids = Vec::new();
    for plan in plans.iter().filter(|plan| predicate(plan)) {
        for stage_id in &plan.required_durable_store_switch_stage_ids {
            if !stage_ids.contains(stage_id) {
                stage_ids.push(*stage_id);
            }
        }
    }
    stage_ids
}

fn application_plan_id_for(readback_plan_id: &str) -> String {
    format!("apply_{readback_plan_id}_durable_store_switch_preview")
}

fn stage_application_id_for(stage_id: &str) -> String {
    format!("apply_{stage_id}_durable_store_switch_stage_preview")
}

fn evidence_field_application_id_for(source_surface_id: &str) -> String {
    format!("apply_{source_surface_id}_durable_store_switch_evidence_preview")
}

fn guard_application_id_for(guard_id: &str) -> String {
    format!("apply_{guard_id}_durable_store_switch_guard_preview")
}

fn blocker_application_id_for(blocker_id: &str) -> String {
    format!("apply_{blocker_id}_durable_store_switch_blocker_preview")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_store_switch_application_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_DURABLE_STORE_SWITCH_APPLICATION_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_gate"
        );
        let side_effects =
            WorkGraphAppendOnlyStoreRuntimeDurableStoreSwitchApplicationPreviewSideEffects::none();
        assert!(!side_effects.filesystem_written);
        assert!(!side_effects.wal_written);
        assert!(!side_effects.checkpoint_written);
        assert!(!side_effects.durable_store_switch_enabled);
        assert!(!side_effects.idempotency_index_mutated);
        assert!(!side_effects.readback_executed);
        assert!(!side_effects.replay_executed);
        assert!(!side_effects.rollback_executed);
        assert!(!side_effects.runtime_mutation_performed);
        assert!(!side_effects.external_send_performed);
        assert!(!side_effects.model_invoked);
        assert!(!side_effects.agent_spawn_performed);
    }

    #[test]
    fn durable_store_switch_application_plans_cover_readback_without_writes() {
        let plans = application_plans_from(&[
            sample_readback_plan("update_plan_tool", "planning"),
            sample_readback_plan("hepta_runtime_agent_harness", "external_handoff"),
        ]);

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.application_scope == "durable_store_switch_application_binding"
                && plan.application_state
                    == "preview_application_defined_durable_store_switch_not_enabled"
                && plan.readback_verified_by_preview
                && plan.durable_store_switch_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.switches_durable_store
                && !plan.mutates_idempotency_index
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.mutates_runtime
                && plan.required_durable_store_switch_stage_ids.len() == 5
                && plan.expected_evidence_field_ids.len() == 9
        }));
    }

    #[test]
    fn durable_store_switch_stage_applications_preserve_declarations_only() {
        let applications = stage_applications_from(&[
            sample_stage_assertion(
                "wal_replay_prerequisite_contract",
                "wal_replay_prerequisite",
                true,
                true,
                false,
            ),
            sample_stage_assertion(
                "durable_store_switch_no_mutation_guard",
                "preview_no_mutation",
                false,
                false,
                false,
            ),
        ]);

        assert_eq!(applications.len(), 2);
        assert!(applications[0].declared_writes_wal);
        assert!(applications[0].declared_executes_replay);
        assert!(!applications[0].writes_wal);
        assert!(!applications[0].executes_replay);
        assert!(!applications[0].enables_runtime_after_application);
        assert!(!applications[0].mutates_runtime);
        assert!(!applications[1].declared_writes_wal);
        assert!(!applications[1].writes_checkpoint);
    }

    #[test]
    fn durable_store_switch_application_blockers_add_rerun_missing() {
        let plans = application_plans_from(&[
            sample_readback_plan("update_plan_tool", "planning"),
            sample_readback_plan("hepta_runtime_agent_harness", "external_handoff"),
        ]);
        let blockers = application_blockers_from(&[sample_readback_blocker()], &plans);

        assert_eq!(blockers.len(), 2);
        assert_eq!(blockers[0].affected_application_plan_ids.len(), 1);
        assert_eq!(
            blockers[1].id,
            "durable_store_switch_readiness_rerun_missing"
        );
        assert_eq!(blockers[1].affected_source_surface_ids.len(), 2);
        assert_eq!(blockers[1].affected_durable_store_switch_stage_ids.len(), 5);
    }

    fn sample_readback_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphDurableStoreSwitchReadbackPlanPreview {
        WorkGraphDurableStoreSwitchReadbackPlanPreview {
            id: format!(
                "append_only_store_runtime_durable_store_switch_readback_plan__{source_surface_id}"
            ),
            source_surface_id,
            source_category,
            durable_store_switch_plan_id: format!(
                "append_only_store_runtime_durable_store_switch_{source_surface_id}_preview"
            ),
            required_durable_store_switch_stage_ids: vec![
                "runtime_durable_store_switch_contract",
                "wal_replay_prerequisite_contract",
                "operator_review_rollback_guard",
                "durable_store_switch_no_mutation_guard",
                "durable_store_switch_blocker_mapping",
            ],
            residual_source_blocker_ids: vec![
                "wal_write_boundary_not_enabled",
                "rollback_readback_not_executed",
            ],
            required_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "runtime_write_boundary_rerun_decision_ref",
                "durable_store_switch_contract_id",
                "wal_replay_prerequisite_id",
                "operator_review_rollback_guard_id",
                "no_mutation_guard_ref",
                "residual_source_blocker_ids",
                "required_prior_gate_ids",
            ],
            readback_state: "readback_verified_from_durable_store_switch_preview_no_execution",
            required_before_application: true,
            performs_readback: false,
            writes_wal: false,
            writes_checkpoint: false,
            switches_durable_store: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_rollback: false,
            mutates_runtime: false,
        }
    }

    fn sample_stage_assertion(
        stage_id: &'static str,
        category: &'static str,
        declared_writes_wal: bool,
        declared_executes_replay: bool,
        declared_executes_readback: bool,
    ) -> WorkGraphDurableStoreSwitchStageReadbackAssertionPreview {
        WorkGraphDurableStoreSwitchStageReadbackAssertionPreview {
            id: format!("durable_store_switch_stage_readback_assertion__{stage_id}"),
            stage_id,
            category,
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["contract_ready"],
            expected_runtime_state: "readback_verified_contract_ready_runtime_disabled",
            contract_ready_preview: true,
            runtime_enabled_after_readback: false,
            declared_writes_wal,
            declared_writes_checkpoint: declared_writes_wal || declared_executes_readback,
            declared_switches_durable_store: false,
            declared_mutates_idempotency_index: false,
            declared_executes_replay,
            declared_executes_readback,
            declared_executes_rollback: declared_executes_readback,
            performs_readback: false,
            mutates_runtime: false,
        }
    }

    fn sample_readback_blocker() -> WorkGraphDurableStoreSwitchReadbackBlockerPreview {
        WorkGraphDurableStoreSwitchReadbackBlockerPreview {
            id: "wal_write_boundary_not_enabled",
            severity: "critical",
            category: "wal_replay_prerequisite",
            affected_source_surface_ids: vec!["update_plan_tool"],
            affected_durable_store_switch_stage_ids: vec!["wal_replay_prerequisite_contract"],
            affected_readback_plan_ids: vec![
                "append_only_store_runtime_durable_store_switch_readback_plan__update_plan_tool"
                    .to_string(),
            ],
            blocks_durable_store_switch: true,
            recommended_fix: "keep WAL writes disabled until durable-store switch contracts are promoted",
        }
    }
}
