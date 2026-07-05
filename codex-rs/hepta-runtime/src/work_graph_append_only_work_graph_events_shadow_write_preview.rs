use serde::Serialize;

use crate::work_graph_canonical_adapter_inventory_application_preview::WorkGraphCanonicalAdapterInventoryApplicationPreviewReport;
use crate::work_graph_canonical_adapter_inventory_application_preview::hepta_work_graph_canonical_adapter_inventory_application_preview_report;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_residual_blockers;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_source_decisions;
use crate::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview::work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_shadow_write_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate";

const SHADOW_WRITE_STAGE_IDS: [&str; 6] = [
    "work_graph_event_schema_contract",
    "work_graph_event_source_surface_mapping",
    "work_graph_event_idempotency_key_contract",
    "work_graph_event_replay_readback_guard",
    "work_graph_event_no_persistence_guard",
    "work_graph_event_blocker_mapping",
];

const IDEMPOTENCY_KEY_FIELDS: [&str; 5] = [
    "sourceSurfaceId",
    "traceId",
    "canonicalNodeId",
    "eventType",
    "sequenceKey",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_canonical_adapter_inventory_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub shadow_write_plan_count: usize,
    pub event_schema_count: usize,
    pub source_event_binding_count: usize,
    pub shadow_write_stage_count: usize,
    pub shadow_write_stage_source_ref_count: usize,
    pub shadow_write_stage_contract_ref_count: usize,
    pub shadow_write_plan_stage_ref_count: usize,
    pub shadow_write_plan_event_schema_ref_count: usize,
    pub idempotency_key_field_ref_count: usize,
    pub shadow_write_contract_ready_preview_count: usize,
    pub append_only_work_graph_events_primary_blocked_source_count: usize,
    pub partial_or_gap_blocked_source_count: usize,
    pub shadow_write_enabled_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub shadow_write_plans: Vec<WorkGraphEventsShadowWritePlanPreview>,
    pub event_schemas: Vec<WorkGraphEventsShadowWriteSchemaPreview>,
    pub stage_plans: Vec<WorkGraphEventsShadowWriteStagePreview>,
    pub guards: Vec<WorkGraphEventsShadowWriteGuardPreview>,
    pub blockers: Vec<WorkGraphEventsShadowWriteBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_write_readback_preview: bool,
    pub ready_for_shadow_write_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWritePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub shadow_write_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub shadow_write_state: &'static str,
    pub canonical_node_kind: &'static str,
    pub required_identity_fields: Vec<&'static str>,
    pub canonical_collection_ids: Vec<&'static str>,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub event_schema_ids: Vec<&'static str>,
    pub required_shadow_write_stage_ids: Vec<&'static str>,
    pub idempotency_key_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub canonical_adapter_inventory_contract_ready: bool,
    pub shadow_write_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteSchemaPreview {
    pub id: &'static str,
    pub category: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub idempotency_scope: &'static str,
    pub redaction_required: bool,
    pub payload_hash_required: bool,
    pub replay_readback_required: bool,
    pub shadow_write_only: bool,
    pub persists_event_after_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteStagePreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events_after_preview: bool,
    pub executes_replay_after_preview: bool,
    pub executes_readback_after_preview: bool,
    pub mutates_runtime_after_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_shadow_write: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsShadowWriteBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_shadow_write_stage_ids: Vec<&'static str>,
    pub affected_shadow_write_plan_ids: Vec<String>,
    pub required_before_shadow_write: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_source_decisions();
    let application_report =
        hepta_work_graph_canonical_adapter_inventory_application_preview_report();
    let shadow_write_plans = work_graph_append_only_work_graph_events_shadow_write_plans_from(
        &source_decisions,
        &application_report,
    );
    let event_schemas = work_graph_append_only_work_graph_events_shadow_write_event_schemas();
    let stage_plans = work_graph_append_only_work_graph_events_shadow_write_stage_plans_from(
        &source_ids_from_plans(&shadow_write_plans),
    );
    let guards = work_graph_append_only_work_graph_events_shadow_write_guards();
    let blockers = work_graph_append_only_work_graph_events_shadow_write_blockers_from(
        &work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_residual_blockers(),
        &shadow_write_plans,
    );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_shadow_write_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_work_graph_events_shadow_write_preview_no_persistence",
        upstream_canonical_adapter_inventory_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        shadow_write_plan_count: shadow_write_plans.len(),
        event_schema_count: event_schemas.len(),
        source_event_binding_count: shadow_write_plans
            .iter()
            .map(|plan| plan.event_schema_ids.len())
            .sum(),
        shadow_write_stage_count: stage_plans.len(),
        shadow_write_stage_source_ref_count: stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        shadow_write_stage_contract_ref_count: stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        shadow_write_plan_stage_ref_count: shadow_write_plans
            .iter()
            .map(|plan| plan.required_shadow_write_stage_ids.len())
            .sum(),
        shadow_write_plan_event_schema_ref_count: shadow_write_plans
            .iter()
            .map(|plan| plan.event_schema_ids.len())
            .sum(),
        idempotency_key_field_ref_count: shadow_write_plans
            .iter()
            .map(|plan| plan.idempotency_key_field_ids.len())
            .sum(),
        shadow_write_contract_ready_preview_count: shadow_write_plans
            .iter()
            .filter(|plan| plan.shadow_write_contract_ready_preview)
            .count(),
        append_only_work_graph_events_primary_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision.canonical_adapter_inventory_rerun_enforcement_decision
                    == "deny_append_only_work_graph_events_disabled"
            })
            .count(),
        partial_or_gap_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision
                    .residual_source_blocker_ids
                    .contains(&"canonical_adapter_projection_partial_or_gap")
            })
            .count(),
        shadow_write_enabled_source_count: 0,
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        shadow_write_plans,
        event_schemas,
        stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_write_readback_preview: true,
        ready_for_shadow_write_application_preview: false,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_shadow_write_plans()
-> Vec<WorkGraphEventsShadowWritePlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_source_decisions();
    let application_report =
        hepta_work_graph_canonical_adapter_inventory_application_preview_report();
    work_graph_append_only_work_graph_events_shadow_write_plans_from(
        &source_decisions,
        &application_report,
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_event_schemas()
-> Vec<WorkGraphEventsShadowWriteSchemaPreview> {
    vec![
        event_schema("PlanStepCreated", "planning"),
        event_schema("AgentTaskSpawned", "multi_agent"),
        event_schema("MessageLinked", "multi_agent"),
        event_schema("TaskResultReported", "task_result"),
        event_schema("ArtifactProduced", "artifact"),
        event_schema("ApprovalRequired", "operator_control"),
        event_schema("ApprovalRecorded", "operator_control"),
        event_schema("LeaseAcquired", "scheduler"),
        event_schema("LeaseReleased", "scheduler"),
        event_schema("GateEvaluated", "gate"),
        event_schema("TimelineEventAppended", "timeline"),
    ]
}

pub fn work_graph_append_only_work_graph_events_shadow_write_stage_plans()
-> Vec<WorkGraphEventsShadowWriteStagePreview> {
    let plans = work_graph_append_only_work_graph_events_shadow_write_plans();
    work_graph_append_only_work_graph_events_shadow_write_stage_plans_from(&source_ids_from_plans(
        &plans,
    ))
}

pub fn work_graph_append_only_work_graph_events_shadow_write_blockers()
-> Vec<WorkGraphEventsShadowWriteBlockerPreview> {
    let plans = work_graph_append_only_work_graph_events_shadow_write_plans();
    work_graph_append_only_work_graph_events_shadow_write_blockers_from(
        &work_graph_unified_projection_enforcement_canonical_adapter_inventory_rerun_residual_blockers(),
        &plans,
    )
}

pub fn work_graph_append_only_work_graph_events_shadow_write_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_CANONICAL_ADAPTER_INVENTORY_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects {
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
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_append_only_work_graph_events_shadow_write_plans_from(
    source_decisions: &[WorkGraphCanonicalAdapterInventoryRerunSourceDecisionPreview],
    application_report: &WorkGraphCanonicalAdapterInventoryApplicationPreviewReport,
) -> Vec<WorkGraphEventsShadowWritePlanPreview> {
    source_decisions
        .iter()
        .map(|decision| {
            let identity = application_report
                .identity_applications
                .iter()
                .find(|application| application.source_surface_id == decision.source_surface_id);
            let collection = application_report
                .collection_binding_applications
                .iter()
                .find(|application| application.source_surface_id == decision.source_surface_id);
            let timeline = application_report
                .timeline_event_applications
                .iter()
                .find(|application| application.source_surface_id == decision.source_surface_id);

            WorkGraphEventsShadowWritePlanPreview {
                source_surface_id: decision.source_surface_id,
                source_category: decision.source_category,
                shadow_write_plan_id: format!(
                    "{}_append_only_work_graph_events_shadow_write",
                    decision.source_surface_id
                ),
                previous_enforcement_decision: decision
                    .canonical_adapter_inventory_rerun_enforcement_decision,
                shadow_write_state: "append_only_work_graph_events_shadow_write_contract_defined_preview_only",
                canonical_node_kind: identity
                    .map(|application| application.canonical_node_kind)
                    .unwrap_or("unknown"),
                required_identity_fields: identity
                    .map(|application| application.required_identity_fields.clone())
                    .unwrap_or_default(),
                canonical_collection_ids: collection
                    .map(|application| application.canonical_collection_ids.clone())
                    .unwrap_or_default(),
                timeline_event_type_ids: timeline
                    .map(|application| application.timeline_event_type_ids.clone())
                    .unwrap_or_default(),
                event_schema_ids: event_schema_ids_for_source(decision.source_surface_id),
                required_shadow_write_stage_ids: SHADOW_WRITE_STAGE_IDS.to_vec(),
                idempotency_key_field_ids: IDEMPOTENCY_KEY_FIELDS.to_vec(),
                residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
                canonical_adapter_inventory_contract_ready: decision
                    .canonical_adapter_inventory_contract_ready,
                shadow_write_contract_ready_preview: true,
                applies_to_runtime: false,
                persists_work_graph_events: false,
                writes_wal: false,
                writes_checkpoint: false,
                executes_replay: false,
                executes_readback: false,
                mutates_runtime: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_shadow_write_stage_plans_from(
    source_ids: &[&'static str],
) -> Vec<WorkGraphEventsShadowWriteStagePreview> {
    vec![
        stage(
            "work_graph_event_schema_contract",
            "p0",
            "event_schema",
            source_ids.to_vec(),
            vec![
                "event_type_contract_ready",
                "event_payload_contract_ready",
                "event_redaction_contract_ready",
                "event_hash_contract_ready",
                "event_artifact_ref_contract_ready",
                "event_version_contract_ready",
            ],
        ),
        stage(
            "work_graph_event_source_surface_mapping",
            "p0",
            "source_surface_mapping",
            source_ids.to_vec(),
            vec![
                "source_surface_id_mapping_ready",
                "canonical_node_kind_mapping_ready",
                "canonical_edge_kind_mapping_ready",
                "canonical_collection_mapping_ready",
                "timeline_event_mapping_ready",
                "task_result_mapping_ready",
            ],
        ),
        stage(
            "work_graph_event_idempotency_key_contract",
            "p0",
            "idempotency_key",
            source_ids.to_vec(),
            vec![
                "source_surface_id_key_ready",
                "trace_id_key_ready",
                "canonical_node_id_key_ready",
                "event_type_key_ready",
                "sequence_key_ready",
            ],
        ),
        stage(
            "work_graph_event_replay_readback_guard",
            "p0",
            "replay_readback_guard",
            source_ids.to_vec(),
            vec![
                "shadow_replay_cursor_contract_ready",
                "readback_probe_contract_ready",
                "duplicate_suppression_contract_ready",
                "timeline_ordering_contract_ready",
                "rollback_anchor_contract_ready",
                "event_integrity_digest_contract_ready",
            ],
        ),
        stage(
            "work_graph_event_no_persistence_guard",
            "p0",
            "preview_no_persistence_guard",
            source_ids.to_vec(),
            vec![
                "work_graph_events_no_persist_guard_ready",
                "wal_no_write_guard_ready",
                "checkpoint_no_write_guard_ready",
                "durable_store_switch_disabled_guard_ready",
                "scheduler_no_admission_guard_ready",
                "runtime_no_mutation_guard_ready",
                "external_send_noop_guard_ready",
            ],
        ),
        stage(
            "work_graph_event_blocker_mapping",
            "p0",
            "blocker_mapping",
            source_ids.to_vec(),
            vec![
                "append_only_events_blocker_mapping_ready",
                "canonical_adapter_enforcement_blocker_mapping_ready",
                "partial_gap_blocker_mapping_ready",
                "readback_missing_blocker_mapping_ready",
            ],
        ),
    ]
}

fn work_graph_append_only_work_graph_events_shadow_write_guards()
-> Vec<WorkGraphEventsShadowWriteGuardPreview> {
    vec![
        guard(
            "work_graph_events_shadow_write_preview_only",
            "medium",
            "preview_boundary",
        ),
        guard(
            "work_graph_events_persistence_disabled",
            "critical",
            "event_store",
        ),
        guard("wal_write_disabled", "critical", "wal"),
        guard("checkpoint_write_disabled", "critical", "checkpoint"),
        guard("replay_execution_disabled", "critical", "replay"),
        guard("readback_execution_disabled", "critical", "readback"),
        guard(
            "adapter_projection_enforcement_disabled",
            "critical",
            "adapter_projection",
        ),
        guard(
            "scheduler_admission_enforcement_disabled",
            "high",
            "scheduler_admission",
        ),
        guard("no_agent_spawn", "high", "agent_spawn"),
        guard(
            "no_external_send_or_model_invocation",
            "high",
            "external_effects",
        ),
    ]
}

fn work_graph_append_only_work_graph_events_shadow_write_blockers_from(
    residual_blockers: &[WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview],
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> Vec<WorkGraphEventsShadowWriteBlockerPreview> {
    let mut blockers = residual_blockers
        .iter()
        .map(|blocker| shadow_write_blocker_from_residual(blocker, plans))
        .collect::<Vec<_>>();
    blockers.push(shadow_write_blocker(
        "append_only_work_graph_events_shadow_write_readback_missing",
        "high",
        "readback_preview",
        source_ids_from_plans(plans),
        SHADOW_WRITE_STAGE_IDS.to_vec(),
        "read back shadow WorkGraph event contracts before any event-store persistence, replay/readback execution, or adapter enforcement",
    ));
    blockers
}

fn event_schema(
    id: &'static str,
    category: &'static str,
) -> WorkGraphEventsShadowWriteSchemaPreview {
    WorkGraphEventsShadowWriteSchemaPreview {
        id,
        category,
        required_field_ids: vec![
            "eventId",
            "eventType",
            "sourceSurfaceId",
            "traceId",
            "canonicalNodeId",
            "idempotencyKey",
            "payloadHash",
            "redactedEvidenceRefs",
        ],
        idempotency_scope: "source_surface_trace_event_type_sequence",
        redaction_required: true,
        payload_hash_required: true,
        replay_readback_required: true,
        shadow_write_only: true,
        persists_event_after_preview: false,
    }
}

fn stage(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
) -> WorkGraphEventsShadowWriteStagePreview {
    WorkGraphEventsShadowWriteStagePreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "contract_ready_preview_persistence_disabled",
        contract_ready_preview: true,
        persists_work_graph_events_after_preview: false,
        executes_replay_after_preview: false,
        executes_readback_after_preview: false,
        mutates_runtime_after_preview: false,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsShadowWriteGuardPreview {
    WorkGraphEventsShadowWriteGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_shadow_write: true,
        satisfied_by_preview: false,
    }
}

fn shadow_write_blocker_from_residual(
    blocker: &WorkGraphCanonicalAdapterInventoryRerunResidualBlockerPreview,
    plans: &[WorkGraphEventsShadowWritePlanPreview],
) -> WorkGraphEventsShadowWriteBlockerPreview {
    shadow_write_blocker(
        blocker.id,
        blocker.severity,
        match blocker.id {
            "append_only_work_graph_events_disabled" => "append_only_fact_source",
            "runtime_canonical_adapter_enforcement_disabled" => "runtime_adapter_enforcement",
            "canonical_adapter_projection_partial_or_gap" => "projection_coverage",
            _ => blocker.category,
        },
        blocker.affected_source_surface_ids.clone(),
        match blocker.id {
            "append_only_work_graph_events_disabled" => SHADOW_WRITE_STAGE_IDS.to_vec(),
            "runtime_canonical_adapter_enforcement_disabled" => vec![
                "work_graph_event_replay_readback_guard",
                "work_graph_event_no_persistence_guard",
            ],
            "canonical_adapter_projection_partial_or_gap" => vec![
                "work_graph_event_source_surface_mapping",
                "work_graph_event_blocker_mapping",
            ],
            _ => vec!["work_graph_event_blocker_mapping"],
        },
        blocker.recommended_fix,
    )
    .with_plan_ids(plans)
}

fn shadow_write_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_shadow_write_stage_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphEventsShadowWriteBlockerPreview {
    let affected_shadow_write_plan_ids = affected_source_surface_ids
        .iter()
        .map(|source| format!("{source}_append_only_work_graph_events_shadow_write"))
        .collect::<Vec<_>>();
    WorkGraphEventsShadowWriteBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_shadow_write_stage_ids,
        affected_shadow_write_plan_ids,
        required_before_shadow_write: true,
        recommended_fix,
    }
}

trait WorkGraphEventsShadowWriteBlockerPlanFilter {
    fn with_plan_ids(self, plans: &[WorkGraphEventsShadowWritePlanPreview]) -> Self;
}

impl WorkGraphEventsShadowWriteBlockerPlanFilter for WorkGraphEventsShadowWriteBlockerPreview {
    fn with_plan_ids(mut self, plans: &[WorkGraphEventsShadowWritePlanPreview]) -> Self {
        self.affected_shadow_write_plan_ids = plans
            .iter()
            .filter(|plan| {
                self.affected_source_surface_ids
                    .contains(&plan.source_surface_id)
            })
            .map(|plan| plan.shadow_write_plan_id.clone())
            .collect();
        self
    }
}

fn event_schema_ids_for_source(source_surface_id: &str) -> Vec<&'static str> {
    match source_surface_id {
        "update_plan_tool" => vec!["PlanStepCreated"],
        "plan_mode_proposed_plan_blocks" => vec!["PlanStepCreated"],
        "app_server_turn_plan_notification" => {
            vec!["PlanStepCreated", "TimelineEventAppended"]
        }
        "multi_agent_v2_thread_spawn" => {
            vec!["AgentTaskSpawned", "MessageLinked", "TimelineEventAppended"]
        }
        "multi_agent_v2_mailbox_wait" => {
            vec!["MessageLinked", "TimelineEventAppended"]
        }
        "hepta_runtime_multi_agent_reducer" => {
            vec!["AgentTaskSpawned", "TaskResultReported"]
        }
        "agent_jobs_batch_workers" => {
            vec![
                "AgentTaskSpawned",
                "TaskResultReported",
                "TimelineEventAppended",
            ]
        }
        "hepta_runtime_task_board" => {
            vec!["LeaseAcquired", "LeaseReleased", "TaskResultReported"]
        }
        "hepta_runtime_worker_tasks" => {
            vec![
                "TaskResultReported",
                "ArtifactProduced",
                "TimelineEventAppended",
            ]
        }
        "hepta_runtime_scheduler_store" => vec!["GateEvaluated", "LeaseAcquired"],
        "hepta_runtime_approval_broker" => vec!["ApprovalRequired", "ApprovalRecorded"],
        "hepta_runtime_agent_harness" => {
            vec!["ArtifactProduced", "GateEvaluated", "TimelineEventAppended"]
        }
        _ => vec!["TimelineEventAppended"],
    }
}

fn source_ids_from_plans(plans: &[WorkGraphEventsShadowWritePlanPreview]) -> Vec<&'static str> {
    plans.iter().map(|plan| plan.source_surface_id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_write_event_schemas_cover_expected_contracts() {
        let schemas = work_graph_append_only_work_graph_events_shadow_write_event_schemas();
        let schema_ids = schemas.iter().map(|schema| schema.id).collect::<Vec<_>>();

        assert_eq!(schemas.len(), 11);
        assert!(schema_ids.contains(&"PlanStepCreated"));
        assert!(schema_ids.contains(&"AgentTaskSpawned"));
        assert!(schema_ids.contains(&"TaskResultReported"));
        assert!(schema_ids.contains(&"TimelineEventAppended"));
        assert!(
            schemas
                .iter()
                .all(|schema| schema.shadow_write_only && !schema.persists_event_after_preview)
        );
    }

    #[test]
    fn shadow_write_source_mappings_cover_planning_agents_and_results() {
        assert_eq!(
            event_schema_ids_for_source("update_plan_tool"),
            ["PlanStepCreated"]
        );
        assert_eq!(
            event_schema_ids_for_source("multi_agent_v2_thread_spawn"),
            ["AgentTaskSpawned", "MessageLinked", "TimelineEventAppended"]
        );
        assert_eq!(
            event_schema_ids_for_source("hepta_runtime_worker_tasks"),
            [
                "TaskResultReported",
                "ArtifactProduced",
                "TimelineEventAppended"
            ]
        );
    }

    #[test]
    fn shadow_write_stages_are_no_persistence_preview_only() {
        let source_ids = vec!["update_plan_tool", "multi_agent_v2_thread_spawn"];
        let stages =
            work_graph_append_only_work_graph_events_shadow_write_stage_plans_from(&source_ids);

        assert_eq!(stages.len(), 6);
        assert!(stages.iter().all(|stage| {
            stage.contract_ready_preview
                && !stage.persists_work_graph_events_after_preview
                && !stage.executes_replay_after_preview
                && !stage.executes_readback_after_preview
                && !stage.mutates_runtime_after_preview
        }));
    }

    #[test]
    fn shadow_write_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsShadowWritePreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                replay_executed: false,
                readback_executed: false,
                adapter_projection_enforced: false,
                scheduler_admission_enforced: false,
                task_result_enforcement_enabled: false,
                role_manifest_enforcement_enabled: false,
                approval_recorded: false,
                side_effect_lock_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }
}
