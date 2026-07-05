use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewReport;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphEventsShadowWriteBlockerPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphEventsShadowWriteGuardPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphEventsShadowWritePlanPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphEventsShadowWriteSchemaPreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WorkGraphEventsShadowWriteStagePreview;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::work_graph_append_only_work_graph_events_shadow_write_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_shadow_write_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_shadow_write_preview_gate: &'static str,
    pub source_surface_count: usize,
    pub preview_plan_count: usize,
    pub readback_plan_count: usize,
    pub event_schema_assertion_count: usize,
    pub stage_assertion_count: usize,
    pub source_mapping_assertion_count: usize,
    pub event_binding_assertion_count: usize,
    pub idempotency_key_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphEventsShadowWriteReadbackPlanPreview>,
    pub event_schema_assertions: Vec<WorkGraphEventsShadowWriteSchemaAssertionPreview>,
    pub stage_assertions: Vec<WorkGraphEventsShadowWriteStageAssertionPreview>,
    pub source_mapping_assertions: Vec<WorkGraphEventsShadowWriteSourceMappingAssertionPreview>,
    pub event_binding_assertions: Vec<WorkGraphEventsShadowWriteEventBindingAssertionPreview>,
    pub idempotency_key_assertions: Vec<WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphEventsShadowWriteGuardAssertionPreview>,
    pub blocker_mapping_assertions: Vec<WorkGraphEventsShadowWriteBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphEventsShadowWriteReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphEventsShadowWriteReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_write_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub shadow_write_plan_id: String,
    pub expected_event_schema_count: usize,
    pub expected_stage_count: usize,
    pub expected_idempotency_key_field_count: usize,
    pub expected_residual_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub persists_work_graph_events: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteSchemaAssertionPreview {
    pub event_schema_id: &'static str,
    pub category: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub shadow_write_only: bool,
    pub persists_event_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteStageAssertionPreview {
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persistence_enabled_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteSourceMappingAssertionPreview {
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub canonical_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub source_mapping_ready_preview: bool,
    pub persists_mapping_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteEventBindingAssertionPreview {
    pub source_surface_id: &'static str,
    pub event_schema_id: &'static str,
    pub binding_ready_preview: bool,
    pub persists_event_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview {
    pub source_surface_id: &'static str,
    pub idempotency_key_field_ids: Vec<&'static str>,
    pub idempotency_key_ready_preview: bool,
    pub mutates_idempotency_index_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteGuardAssertionPreview {
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_shadow_write: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteBlockerMappingAssertionPreview {
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_shadow_write_stage_ids: Vec<&'static str>,
    pub blocks_shadow_write_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewReport {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report();
    let readback_plans = work_graph_append_only_work_graph_events_shadow_write_readback_plans_from(
        &preview_report.shadow_write_plans,
    );
    let event_schema_assertions =
        work_graph_append_only_work_graph_events_shadow_write_schema_assertions_from(
            &preview_report.event_schemas,
        );
    let stage_assertions =
        work_graph_append_only_work_graph_events_shadow_write_stage_assertions_from(
            &preview_report.stage_plans,
        );
    let source_mapping_assertions =
        work_graph_append_only_work_graph_events_shadow_write_source_mapping_assertions_from(
            &preview_report.shadow_write_plans,
        );
    let event_binding_assertions =
        work_graph_append_only_work_graph_events_shadow_write_event_binding_assertions_from(
            &preview_report.shadow_write_plans,
        );
    let idempotency_key_assertions =
        work_graph_append_only_work_graph_events_shadow_write_idempotency_key_assertions_from(
            &preview_report.shadow_write_plans,
        );
    let guard_assertions =
        work_graph_append_only_work_graph_events_shadow_write_guard_assertions_from(
            &preview_report.guards,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_work_graph_events_shadow_write_blocker_mapping_assertions_from(
            &preview_report.blockers,
        );
    let drift_detectors = work_graph_append_only_work_graph_events_shadow_write_drift_detectors();
    let blockers = work_graph_append_only_work_graph_events_shadow_write_readback_blockers_from(
        &preview_report,
    );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_shadow_write_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_work_graph_events_shadow_write_readback_preview_no_execution",
        upstream_shadow_write_preview_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE,
        source_surface_count: preview_report.source_surface_count,
        preview_plan_count: preview_report.shadow_write_plan_count,
        readback_plan_count: readback_plans.len(),
        event_schema_assertion_count: event_schema_assertions.len(),
        stage_assertion_count: stage_assertions.len(),
        source_mapping_assertion_count: source_mapping_assertions.len(),
        event_binding_assertion_count: event_binding_assertions.len(),
        idempotency_key_assertion_count: idempotency_key_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        event_schema_assertions,
        stage_assertions,
        source_mapping_assertions,
        event_binding_assertions,
        idempotency_key_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_write_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects::none(
        ),
    }
}

pub fn work_graph_append_only_work_graph_events_shadow_write_readback_plans()
-> Vec<WorkGraphEventsShadowWriteReadbackPlanPreview> {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report();
    work_graph_append_only_work_graph_events_shadow_write_readback_plans_from(
        &preview_report.shadow_write_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_drift_detectors()
-> Vec<WorkGraphEventsShadowWriteReadbackDriftDetectorPreview> {
    vec![
        drift_detector("event_schema_order_drift", vec!["event_schema_id"]),
        drift_detector(
            "source_event_binding_drift",
            vec!["source_surface_id", "event_schema_ids"],
        ),
        drift_detector(
            "idempotency_key_field_drift",
            vec!["source_surface_id", "idempotency_key_field_ids"],
        ),
        drift_detector(
            "source_surface_mapping_drift",
            vec!["source_surface_id", "canonical_node_kind"],
        ),
        drift_detector(
            "replay_readback_guard_drift",
            vec!["stage_id", "required_contract_ref_ids"],
        ),
        drift_detector(
            "no_persistence_guard_drift",
            vec!["guard_id", "satisfied_by_preview"],
        ),
        drift_detector(
            "shadow_write_blocker_mapping_drift",
            vec!["blocker_id", "affected_source_surface_ids"],
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_shadow_write_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_work_graph_events_shadow_write_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_append_only_work_graph_events_shadow_write_readback_plans_from(
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> Vec<WorkGraphEventsShadowWriteReadbackPlanPreview> {
    plans
        .iter()
        .map(|plan| WorkGraphEventsShadowWriteReadbackPlanPreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            shadow_write_plan_id: plan.shadow_write_plan_id.clone(),
            expected_event_schema_count: plan.event_schema_ids.len(),
            expected_stage_count: plan.required_shadow_write_stage_ids.len(),
            expected_idempotency_key_field_count: plan.idempotency_key_field_ids.len(),
            expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            persists_work_graph_events: false,
            next_required_gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_RECOMMENDED_NEXT_GATE,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_schema_assertions_from(
    schemas: &[WorkGraphEventsShadowWriteSchemaPreview],
) -> Vec<WorkGraphEventsShadowWriteSchemaAssertionPreview> {
    schemas
        .iter()
        .map(|schema| WorkGraphEventsShadowWriteSchemaAssertionPreview {
            event_schema_id: schema.id,
            category: schema.category,
            required_field_ids: schema.required_field_ids.clone(),
            shadow_write_only: schema.shadow_write_only,
            persists_event_after_readback: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_stage_assertions_from(
    stages: &[WorkGraphEventsShadowWriteStagePreview],
) -> Vec<WorkGraphEventsShadowWriteStageAssertionPreview> {
    stages
        .iter()
        .map(|stage| WorkGraphEventsShadowWriteStageAssertionPreview {
            stage_id: stage.id,
            affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
            required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
            contract_ready_preview: stage.contract_ready_preview,
            persistence_enabled_after_readback: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_source_mapping_assertions_from(
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> Vec<WorkGraphEventsShadowWriteSourceMappingAssertionPreview> {
    plans
        .iter()
        .map(
            |plan| WorkGraphEventsShadowWriteSourceMappingAssertionPreview {
                source_surface_id: plan.source_surface_id,
                canonical_node_kind: plan.canonical_node_kind,
                canonical_collection_ids: plan.canonical_collection_ids.clone(),
                timeline_event_type_ids: plan.timeline_event_type_ids.clone(),
                source_mapping_ready_preview: true,
                persists_mapping_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_event_binding_assertions_from(
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> Vec<WorkGraphEventsShadowWriteEventBindingAssertionPreview> {
    plans
        .iter()
        .flat_map(|plan| {
            plan.event_schema_ids.iter().map(|event_schema_id| {
                WorkGraphEventsShadowWriteEventBindingAssertionPreview {
                    source_surface_id: plan.source_surface_id,
                    event_schema_id,
                    binding_ready_preview: true,
                    persists_event_after_readback: false,
                }
            })
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_idempotency_key_assertions_from(
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> Vec<WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview> {
    plans
        .iter()
        .map(
            |plan| WorkGraphEventsShadowWriteIdempotencyKeyAssertionPreview {
                source_surface_id: plan.source_surface_id,
                idempotency_key_field_ids: plan.idempotency_key_field_ids.clone(),
                idempotency_key_ready_preview: true,
                mutates_idempotency_index_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_guard_assertions_from(
    guards: &[WorkGraphEventsShadowWriteGuardPreview],
) -> Vec<WorkGraphEventsShadowWriteGuardAssertionPreview> {
    guards
        .iter()
        .map(|guard| WorkGraphEventsShadowWriteGuardAssertionPreview {
            guard_id: guard.id,
            severity: guard.severity,
            guard_scope: guard.guard_scope,
            required_before_shadow_write: guard.required_before_shadow_write,
            satisfied_by_preview: guard.satisfied_by_preview,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_blocker_mapping_assertions_from(
    blockers: &[WorkGraphEventsShadowWriteBlockerPreview],
) -> Vec<WorkGraphEventsShadowWriteBlockerMappingAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphEventsShadowWriteBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_shadow_write_stage_ids: blocker.affected_shadow_write_stage_ids.clone(),
                blocks_shadow_write_persistence: true,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_readback_blockers_from(
    preview_report: &WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewReport,
) -> Vec<WorkGraphEventsShadowWriteReadbackBlockerPreview> {
    let all_sources = preview_report
        .shadow_write_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let partial_gap_sources = preview_report
        .blockers
        .iter()
        .find(|blocker| blocker.id == "canonical_adapter_projection_partial_or_gap")
        .map(|blocker| blocker.affected_source_surface_ids.clone())
        .unwrap_or_default();
    vec![
        readback_blocker(
            "append_only_work_graph_events_shadow_write_readback_not_executed",
            "high",
            all_sources.clone(),
            "keep shadow-write readback as a preview until replay/readback execution is explicitly enabled",
        ),
        readback_blocker(
            "append_only_work_graph_events_shadow_write_application_missing",
            "high",
            all_sources.clone(),
            "apply readback-verified shadow-write contracts into no-mutation application outcomes",
        ),
        readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            all_sources.clone(),
            "keep WorkGraph event persistence disabled until shadow write application and replay/readback readiness are verified",
        ),
        readback_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            all_sources,
            "keep runtime canonical adapter enforcement disabled until append-only WorkGraph events are promoted",
        ),
        readback_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            partial_gap_sources,
            "close partial/gap adapter source mappings before authoritative event projection",
        ),
    ]
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphEventsShadowWriteReadbackDriftDetectorPreview {
    WorkGraphEventsShadowWriteReadbackDriftDetectorPreview {
        id,
        source_fields,
        drift_budget: 0,
    }
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphEventsShadowWriteReadbackBlockerPreview {
    WorkGraphEventsShadowWriteReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_write_readback_drift_detectors_cover_core_contracts() {
        let detectors = work_graph_append_only_work_graph_events_shadow_write_drift_detectors();
        let detector_ids = detectors
            .iter()
            .map(|detector| detector.id)
            .collect::<Vec<_>>();

        assert_eq!(detectors.len(), 7);
        assert!(detector_ids.contains(&"event_schema_order_drift"));
        assert!(detector_ids.contains(&"source_event_binding_drift"));
        assert!(detector_ids.contains(&"idempotency_key_field_drift"));
        assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
    }

    #[test]
    fn shadow_write_readback_plans_preserve_no_execution_boundary() {
        let plan = WorkGraphEventsShadowWritePlanPreview {
            source_surface_id: "update_plan_tool",
            source_category: "planning",
            shadow_write_plan_id: "update_plan_tool_append_only_work_graph_events_shadow_write"
                .to_string(),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            shadow_write_state: "append_only_work_graph_events_shadow_write_contract_defined_preview_only",
            canonical_node_kind: "plan_step",
            required_identity_fields: vec!["sourceSurfaceId"],
            canonical_collection_ids: vec!["nodes"],
            timeline_event_type_ids: Vec::new(),
            event_schema_ids: vec!["PlanStepCreated"],
            required_shadow_write_stage_ids: vec!["work_graph_event_schema_contract"],
            idempotency_key_field_ids: vec!["sourceSurfaceId", "traceId"],
            residual_source_blocker_ids: vec!["append_only_work_graph_events_disabled"],
            canonical_adapter_inventory_contract_ready: true,
            shadow_write_contract_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            writes_wal: false,
            writes_checkpoint: false,
            executes_replay: false,
            executes_readback: false,
            mutates_runtime: false,
        };
        let plans =
            work_graph_append_only_work_graph_events_shadow_write_readback_plans_from(&[plan]);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].readback_status, "readback_plan_ready");
        assert!(!plans[0].readback_execution_enabled);
        assert!(!plans[0].persists_work_graph_events);
    }

    #[test]
    fn shadow_write_readback_schema_assertions_do_not_persist_events() {
        let schemas = vec![WorkGraphEventsShadowWriteSchemaPreview {
            id: "PlanStepCreated",
            category: "planning",
            required_field_ids: vec!["eventId", "eventType"],
            idempotency_scope: "source_surface_trace_event_type_sequence",
            redaction_required: true,
            payload_hash_required: true,
            replay_readback_required: true,
            shadow_write_only: true,
            persists_event_after_preview: false,
        }];
        let assertions =
            work_graph_append_only_work_graph_events_shadow_write_schema_assertions_from(&schemas);

        assert_eq!(assertions.len(), 1);
        assert!(assertions[0].shadow_write_only);
        assert!(!assertions[0].persists_event_after_readback);
    }

    #[test]
    fn shadow_write_readback_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsShadowWriteReadbackPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                replay_executed: false,
                readback_executed: false,
                adapter_projection_enforced: false,
                runtime_mutation_performed: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }
}
