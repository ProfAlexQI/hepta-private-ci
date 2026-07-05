use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteBlockerMappingAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteEventBindingAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteGuardAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteReadbackPlanPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteSchemaAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteSourceMappingAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WorkGraphEventsShadowWriteStageAssertionPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_report;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::work_graph_append_only_work_graph_events_shadow_write_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_shadow_write_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub shadow_write_contract_ready_preview_count: usize,
    pub event_schema_application_count: usize,
    pub stage_application_count: usize,
    pub source_mapping_application_count: usize,
    pub event_binding_application_count: usize,
    pub idempotency_key_application_count: usize,
    pub guard_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphEventsShadowWriteApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphEventsShadowWriteApplicationSourceOutcomePreview>,
    pub event_schema_applications: Vec<WorkGraphEventsShadowWriteSchemaApplicationPreview>,
    pub stage_applications: Vec<WorkGraphEventsShadowWriteStageApplicationPreview>,
    pub source_mapping_applications: Vec<WorkGraphEventsShadowWriteSourceMappingApplicationPreview>,
    pub event_binding_applications: Vec<WorkGraphEventsShadowWriteEventBindingApplicationPreview>,
    pub idempotency_key_applications:
        Vec<WorkGraphEventsShadowWriteIdempotencyKeyApplicationPreview>,
    pub guard_applications: Vec<WorkGraphEventsShadowWriteGuardApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphEventsShadowWriteBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphEventsShadowWriteApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphEventsShadowWriteApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_write_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_source_surface_id: &'static str,
    pub source_category: &'static str,
    pub shadow_write_plan_id: String,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub shadow_write_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub mutates_idempotency_index: bool,
    pub enforces_adapter_projection: bool,
    pub mutates_scheduler_admission: bool,
    pub mutates_task_result_enforcement: bool,
    pub mutates_role_manifest_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_shadow_write_state: &'static str,
    pub shadow_write_contract_ready_preview: bool,
    pub ready_for_shadow_write_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteSchemaApplicationPreview {
    pub application_id: String,
    pub event_schema_id: &'static str,
    pub category: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub shadow_write_only: bool,
    pub persists_event_schema: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteStageApplicationPreview {
    pub application_id: String,
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteSourceMappingApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub canonical_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub source_mapping_ready_preview: bool,
    pub persists_mapping: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteEventBindingApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub event_schema_id: &'static str,
    pub binding_ready_preview: bool,
    pub persists_event_binding: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteIdempotencyKeyApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub idempotency_key_field_ids: Vec<&'static str>,
    pub idempotency_key_ready_preview: bool,
    pub mutates_idempotency_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteGuardApplicationPreview {
    pub application_id: String,
    pub guard_id: &'static str,
    pub guard_scope: &'static str,
    pub required_before_shadow_write: bool,
    pub satisfied_by_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_shadow_write_stage_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub clears_application_missing_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_append_only_events: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_append_only_events: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub idempotency_index_mutated: bool,
    pub adapter_projection_enforced: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_report();
    let application_plans =
        work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
            &readback_report.readback_plans,
        );
    let source_outcomes =
        work_graph_append_only_work_graph_events_shadow_write_application_source_outcomes_from(
            &application_plans,
        );
    let application_guards =
        work_graph_append_only_work_graph_events_shadow_write_application_guards();
    let blockers = work_graph_append_only_work_graph_events_shadow_write_application_blockers_from(
        &application_plans,
    );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_shadow_write_application_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_work_graph_events_shadow_write_application_preview_no_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        shadow_write_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.shadow_write_contract_ready_preview)
            .count(),
        event_schema_application_count: readback_report.event_schema_assertions.len(),
        stage_application_count: readback_report.stage_assertions.len(),
        source_mapping_application_count: readback_report.source_mapping_assertions.len(),
        event_binding_application_count: readback_report.event_binding_assertions.len(),
        idempotency_key_application_count: readback_report.idempotency_key_assertions.len(),
        guard_application_count: readback_report.guard_assertions.len(),
        blocker_application_count: readback_report.blocker_mapping_assertions.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        event_schema_applications:
            work_graph_append_only_work_graph_events_shadow_write_schema_applications_from(
                &readback_report.event_schema_assertions,
            ),
        stage_applications:
            work_graph_append_only_work_graph_events_shadow_write_stage_applications_from(
                &readback_report.stage_assertions,
            ),
        source_mapping_applications:
            work_graph_append_only_work_graph_events_shadow_write_source_mapping_applications_from(
                &readback_report.source_mapping_assertions,
            ),
        event_binding_applications:
            work_graph_append_only_work_graph_events_shadow_write_event_binding_applications_from(
                &readback_report.event_binding_assertions,
            ),
        idempotency_key_applications:
            work_graph_append_only_work_graph_events_shadow_write_idempotency_key_applications_from(
                &readback_report.idempotency_key_assertions,
            ),
        guard_applications:
            work_graph_append_only_work_graph_events_shadow_write_guard_applications_from(
                &readback_report.guard_assertions,
            ),
        blocker_applications:
            work_graph_append_only_work_graph_events_shadow_write_blocker_applications_from(
                &readback_report.blocker_mapping_assertions,
            ),
        application_guards,
        application_plans,
        source_outcomes,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_write_readiness_rerun_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_shadow_write_application_plans()
-> Vec<WorkGraphEventsShadowWriteApplicationPlanPreview> {
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_report();
    work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
        &readback_report.readback_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_application_source_outcomes()
-> Vec<WorkGraphEventsShadowWriteApplicationSourceOutcomePreview> {
    work_graph_append_only_work_graph_events_shadow_write_application_source_outcomes_from(
        &work_graph_append_only_work_graph_events_shadow_write_application_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_application_blockers()
-> Vec<WorkGraphEventsShadowWriteApplicationBlockerPreview> {
    work_graph_append_only_work_graph_events_shadow_write_application_blockers_from(
        &work_graph_append_only_work_graph_events_shadow_write_application_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_shadow_write_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE);
    gates
}

fn work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
    readback_plans: &[WorkGraphEventsShadowWriteReadbackPlanPreview],
) -> Vec<WorkGraphEventsShadowWriteApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphEventsShadowWriteApplicationPlanPreview {
            application_plan_id: format!(
                "{}_append_only_work_graph_events_shadow_write_application",
                plan.source_surface_id
            ),
            readback_source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            shadow_write_plan_id: plan.shadow_write_plan_id.clone(),
            application_state:
                "work_graph_events_shadow_write_contract_ready_preview_after_application",
            readback_verified_by_preview: plan.readback_status == "readback_plan_ready",
            shadow_write_contract_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            writes_wal: false,
            writes_checkpoint: false,
            executes_replay: false,
            executes_readback: false,
            mutates_idempotency_index: false,
            enforces_adapter_projection: false,
            mutates_scheduler_admission: false,
            mutates_task_result_enforcement: false,
            mutates_role_manifest_enforcement: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_application_source_outcomes_from(
    application_plans: &[WorkGraphEventsShadowWriteApplicationPlanPreview],
) -> Vec<WorkGraphEventsShadowWriteApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(
            |plan| WorkGraphEventsShadowWriteApplicationSourceOutcomePreview {
                source_surface_id: plan.readback_source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_shadow_write_state: plan.application_state,
                shadow_write_contract_ready_preview: plan.shadow_write_contract_ready_preview,
                ready_for_shadow_write_readiness_rerun_preview: true,
                ready_for_append_only_work_graph_events: false,
                applies_to_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_schema_applications_from(
    assertions: &[WorkGraphEventsShadowWriteSchemaAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteSchemaApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteSchemaApplicationPreview {
                application_id: format!("{}_event_schema_application", assertion.event_schema_id),
                event_schema_id: assertion.event_schema_id,
                category: assertion.category,
                required_field_ids: assertion.required_field_ids.clone(),
                shadow_write_only: assertion.shadow_write_only,
                persists_event_schema: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_stage_applications_from(
    assertions: &[WorkGraphEventsShadowWriteStageAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteStageApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteStageApplicationPreview {
                application_id: format!("{}_stage_application", assertion.stage_id),
                stage_id: assertion.stage_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                required_contract_ref_ids: assertion.required_contract_ref_ids.clone(),
                contract_ready_preview: assertion.contract_ready_preview,
                persists_work_graph_events: false,
                executes_replay: false,
                executes_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_source_mapping_applications_from(
    assertions: &[WorkGraphEventsShadowWriteSourceMappingAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteSourceMappingApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteSourceMappingApplicationPreview {
                application_id: format!(
                    "{}_source_mapping_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                canonical_node_kind: assertion.canonical_node_kind,
                canonical_collection_ids: assertion.canonical_collection_ids.clone(),
                timeline_event_type_ids: assertion.timeline_event_type_ids.clone(),
                source_mapping_ready_preview: assertion.source_mapping_ready_preview,
                persists_mapping: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_event_binding_applications_from(
    assertions: &[WorkGraphEventsShadowWriteEventBindingAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteEventBindingApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteEventBindingApplicationPreview {
                application_id: format!(
                    "{}_{}_event_binding_application",
                    assertion.source_surface_id, assertion.event_schema_id
                ),
                source_surface_id: assertion.source_surface_id,
                event_schema_id: assertion.event_schema_id,
                binding_ready_preview: assertion.binding_ready_preview,
                persists_event_binding: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_idempotency_key_applications_from(
    assertions: &[WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteIdempotencyKeyApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteIdempotencyKeyApplicationPreview {
                application_id: format!(
                    "{}_idempotency_key_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                idempotency_key_field_ids: assertion.idempotency_key_field_ids.clone(),
                idempotency_key_ready_preview: assertion.idempotency_key_ready_preview,
                mutates_idempotency_index: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_guard_applications_from(
    assertions: &[WorkGraphEventsShadowWriteGuardAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteGuardApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteGuardApplicationPreview {
                application_id: format!("{}_guard_application", assertion.guard_id),
                guard_id: assertion.guard_id,
                guard_scope: assertion.guard_scope,
                required_before_shadow_write: assertion.required_before_shadow_write,
                satisfied_by_preview: assertion.satisfied_by_preview,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_blocker_applications_from(
    assertions: &[WorkGraphEventsShadowWriteBlockerMappingAssertionPreview],
) -> Vec<WorkGraphEventsShadowWriteBlockerApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsShadowWriteBlockerApplicationPreview {
                application_id: format!("{}_blocker_application", assertion.blocker_id),
                blocker_id: assertion.blocker_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                affected_shadow_write_stage_ids: assertion.affected_shadow_write_stage_ids.clone(),
                expected_blocker_state: "mapped_for_work_graph_events_shadow_write_rerun_preview",
                readback_verified_by_preview: true,
                clears_application_missing_blocker: assertion.blocker_id
                    == "append_only_work_graph_events_shadow_write_readback_missing",
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_application_guards()
-> Vec<WorkGraphEventsShadowWriteApplicationGuardPreview> {
    vec![
        application_guard("no_work_graph_event_persistence", "critical", "event_store"),
        application_guard("no_wal_write", "critical", "wal"),
        application_guard("no_checkpoint_write", "critical", "checkpoint"),
        application_guard("no_replay_execution", "critical", "replay"),
        application_guard("no_readback_execution", "critical", "readback"),
        application_guard("no_idempotency_index_mutation", "critical", "idempotency"),
        application_guard(
            "no_adapter_projection_enforcement",
            "critical",
            "adapter_projection",
        ),
        application_guard(
            "no_scheduler_admission_enforcement",
            "high",
            "scheduler_admission",
        ),
        application_guard("no_agent_spawn", "high", "agent_spawn"),
        application_guard(
            "no_external_send_or_model_invocation",
            "high",
            "external_effects",
        ),
    ]
}

fn work_graph_append_only_work_graph_events_shadow_write_application_blockers_from(
    application_plans: &[WorkGraphEventsShadowWriteApplicationPlanPreview],
) -> Vec<WorkGraphEventsShadowWriteApplicationBlockerPreview> {
    let all_sources = application_plans
        .iter()
        .map(|plan| plan.readback_source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = application_plans
        .iter()
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    let partial_gap_sources = vec![
        "update_plan_tool",
        "plan_mode_proposed_plan_blocks",
        "app_server_turn_plan_notification",
        "multi_agent_v2_mailbox_wait",
        "hepta_runtime_multi_agent_reducer",
        "hepta_runtime_task_board",
        "hepta_runtime_approval_broker",
    ];

    vec![
        application_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            "append_only_fact_source",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep WorkGraph event persistence disabled until shadow-write readiness rerun is verified",
        ),
        application_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "runtime_adapter_enforcement",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep canonical adapter enforcement disabled until append-only events are promoted",
        ),
        application_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            partial_gap_sources,
            all_plan_ids.clone(),
            "close partial/gap adapter source mappings before authoritative event projection",
        ),
        application_blocker(
            "replay_readback_execution_disabled",
            "high",
            "replay_readback",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep replay/readback disabled until shadow-write evidence is promoted behind operator review",
        ),
        application_blocker(
            "work_graph_events_shadow_write_readiness_rerun_missing",
            "medium",
            "readiness_rerun",
            all_sources,
            all_plan_ids,
            "rerun enforcement readiness after no-mutation shadow-write application outcomes are available",
        ),
    ]
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsShadowWriteApplicationGuardPreview {
    WorkGraphEventsShadowWriteApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_append_only_events: true,
        satisfied_by_preview: true,
    }
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsShadowWriteApplicationBlockerPreview {
    WorkGraphEventsShadowWriteApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_append_only_events: true,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewSideEffects {
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            idempotency_index_mutated: false,
            adapter_projection_enforced: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_write_application_maps_readback_verified_sources() {
        let plans = work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
            &sample_readback_plans(),
        );
        let outcomes =
            work_graph_append_only_work_graph_events_shadow_write_application_source_outcomes_from(
                &plans,
            );

        assert_eq!(plans.len(), 2);
        assert_eq!(outcomes.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.application_state
                == "work_graph_events_shadow_write_contract_ready_preview_after_application"
                && plan.readback_verified_by_preview
                && plan.shadow_write_contract_ready_preview
                && !plan.applies_to_runtime
        }));
        assert!(outcomes.iter().all(|outcome| {
            outcome.shadow_write_contract_ready_preview
                && outcome.ready_for_shadow_write_readiness_rerun_preview
                && !outcome.ready_for_append_only_work_graph_events
                && !outcome.applies_to_runtime
        }));
    }

    #[test]
    fn shadow_write_application_keeps_runtime_mutation_disabled() {
        let plans = work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
            &sample_readback_plans(),
        );

        assert!(plans.iter().all(|plan| {
            plan.readback_verified_by_preview
                && plan.shadow_write_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.persists_work_graph_events
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.mutates_idempotency_index
                && !plan.enforces_adapter_projection
                && !plan.mutates_scheduler_admission
                && !plan.mutates_task_result_enforcement
                && !plan.mutates_role_manifest_enforcement
        }));
    }

    #[test]
    fn shadow_write_application_artifacts_remain_preview_only() {
        let schema_applications =
            work_graph_append_only_work_graph_events_shadow_write_schema_applications_from(
                &sample_schema_assertions(),
            );
        let stage_applications =
            work_graph_append_only_work_graph_events_shadow_write_stage_applications_from(
                &sample_stage_assertions(),
            );
        let source_mapping_applications =
            work_graph_append_only_work_graph_events_shadow_write_source_mapping_applications_from(
                &sample_source_mapping_assertions(),
            );
        let event_binding_applications =
            work_graph_append_only_work_graph_events_shadow_write_event_binding_applications_from(
                &sample_event_binding_assertions(),
            );
        let idempotency_key_applications =
            work_graph_append_only_work_graph_events_shadow_write_idempotency_key_applications_from(
                &sample_idempotency_key_assertions(),
            );

        assert!(
            schema_applications
                .iter()
                .all(|application| !application.persists_event_schema)
        );
        assert!(stage_applications.iter().all(|application| {
            application.contract_ready_preview
                && !application.persists_work_graph_events
                && !application.executes_replay
                && !application.executes_readback
        }));
        assert!(
            source_mapping_applications
                .iter()
                .all(|application| application.source_mapping_ready_preview
                    && !application.persists_mapping)
        );
        assert!(
            event_binding_applications
                .iter()
                .all(|application| application.binding_ready_preview
                    && !application.persists_event_binding)
        );
        assert!(
            idempotency_key_applications
                .iter()
                .all(|application| application.idempotency_key_ready_preview
                    && !application.mutates_idempotency_index)
        );
    }

    #[test]
    fn shadow_write_application_tracks_remaining_blockers_and_side_effects() {
        let plans = work_graph_append_only_work_graph_events_shadow_write_application_plans_from(
            &sample_readback_plans(),
        );
        let blockers =
            work_graph_append_only_work_graph_events_shadow_write_application_blockers_from(&plans);
        let blocker_applications =
            work_graph_append_only_work_graph_events_shadow_write_blocker_applications_from(
                &sample_blocker_mapping_assertions(),
            );
        let application_guards =
            work_graph_append_only_work_graph_events_shadow_write_application_guards();
        let blocker_ids = blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            vec![
                "append_only_work_graph_events_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "replay_readback_execution_disabled",
                "work_graph_events_shadow_write_readiness_rerun_missing"
            ]
        );
        assert!(
            blocker_applications
                .iter()
                .any(|application| application.clears_application_missing_blocker)
        );
        assert!(
            application_guards
                .iter()
                .all(|guard| guard.required_before_append_only_events && guard.satisfied_by_preview)
        );
        assert_eq!(
            blockers
                .iter()
                .find(|blocker| blocker.id == "canonical_adapter_projection_partial_or_gap")
                .map(|blocker| blocker.affected_source_surface_ids.len()),
            Some(7)
        );
        assert!(
            !WorkGraphAppendOnlyWorkGraphEventsShadowWriteApplicationPreviewSideEffects::none()
                .work_graph_events_persisted
        );
    }

    fn sample_readback_plans() -> Vec<WorkGraphEventsShadowWriteReadbackPlanPreview> {
        vec![
            sample_readback_plan("update_plan_tool", "planning"),
            sample_readback_plan("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_readback_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsShadowWriteReadbackPlanPreview {
        WorkGraphEventsShadowWriteReadbackPlanPreview {
            source_surface_id,
            source_category,
            shadow_write_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_shadow_write"
            ),
            expected_event_schema_count: 1,
            expected_stage_count: 6,
            expected_idempotency_key_field_count: 5,
            expected_residual_blocker_count: 3,
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_APPLICATION_PREVIEW_GATE,
        }
    }

    fn sample_schema_assertions() -> Vec<WorkGraphEventsShadowWriteSchemaAssertionPreview> {
        vec![WorkGraphEventsShadowWriteSchemaAssertionPreview {
            event_schema_id: "PlanStepCreated",
            category: "planning",
            required_field_ids: vec!["eventId", "eventType", "idempotencyKey"],
            shadow_write_only: true,
            persists_event_after_readback: false,
        }]
    }

    fn sample_stage_assertions() -> Vec<WorkGraphEventsShadowWriteStageAssertionPreview> {
        vec![WorkGraphEventsShadowWriteStageAssertionPreview {
            stage_id: "work_graph_event_schema_contract",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["event_type_contract_ready"],
            contract_ready_preview: true,
            persistence_enabled_after_readback: false,
        }]
    }

    fn sample_source_mapping_assertions()
    -> Vec<WorkGraphEventsShadowWriteSourceMappingAssertionPreview> {
        vec![WorkGraphEventsShadowWriteSourceMappingAssertionPreview {
            source_surface_id: "update_plan_tool",
            canonical_node_kind: "plan_step",
            canonical_collection_ids: vec!["nodes", "edges"],
            timeline_event_type_ids: vec![],
            source_mapping_ready_preview: true,
            persists_mapping_after_readback: false,
        }]
    }

    fn sample_event_binding_assertions()
    -> Vec<WorkGraphEventsShadowWriteEventBindingAssertionPreview> {
        vec![WorkGraphEventsShadowWriteEventBindingAssertionPreview {
            source_surface_id: "update_plan_tool",
            event_schema_id: "PlanStepCreated",
            binding_ready_preview: true,
            persists_event_after_readback: false,
        }]
    }

    fn sample_idempotency_key_assertions()
    -> Vec<WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview> {
        vec![WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview {
            source_surface_id: "update_plan_tool",
            idempotency_key_field_ids: vec!["sourceSurfaceId", "traceId", "sequenceKey"],
            idempotency_key_ready_preview: true,
            mutates_idempotency_index_after_readback: false,
        }]
    }

    fn sample_blocker_mapping_assertions()
    -> Vec<WorkGraphEventsShadowWriteBlockerMappingAssertionPreview> {
        vec![WorkGraphEventsShadowWriteBlockerMappingAssertionPreview {
            blocker_id: "append_only_work_graph_events_shadow_write_readback_missing",
            affected_source_surface_ids: vec!["update_plan_tool"],
            affected_shadow_write_stage_ids: vec!["work_graph_event_schema_contract"],
            blocks_shadow_write_persistence: true,
        }]
    }
}
