use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_RUNTIME_ADAPTER_ENFORCEMENT_CLOSURE_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview::WorkGraphEventsEventStoreCutoverRuntimeAdapterEnforcementClosureRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_readback_preview_gate";

const EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_STAGE_IDS: [&str; 6] = [
    "work_graph_events_replay_readback_execution_closure_packet",
    "work_graph_events_replay_execution_disabled_guard",
    "work_graph_events_readback_execution_disabled_guard",
    "work_graph_events_append_only_event_store_prerequisite",
    "work_graph_events_event_store_cutover_no_execution_proof",
    "work_graph_events_replay_readback_execution_closure_blocker_mapping",
];

const EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_EVIDENCE_FIELDS: [&str; 10] = [
    "source_surface_id",
    "source_category",
    "event_store_cutover_runtime_adapter_enforcement_closure_rerun_decision_ref",
    "replay_readback_execution_closure_packet_id",
    "replay_execution_disabled_guard_id",
    "readback_execution_disabled_guard_id",
    "append_only_event_store_prerequisite_contract_id",
    "event_store_cutover_no_execution_proof_id",
    "residual_source_blocker_ids",
    "next_required_gate",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_event_store_cutover_runtime_adapter_enforcement_closure_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_plan_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_stage_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_stage_source_ref_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_stage_contract_ref_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_plan_stage_ref_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_plan_evidence_field_ref_count: usize,
    pub append_only_work_graph_events_primary_blocked_source_count: usize,
    pub replay_readback_execution_blocked_source_count: usize,
    pub runtime_adapter_enforcement_blocked_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub event_store_cutover_replay_readback_execution_closure_plans:
        Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview>,
    pub event_store_cutover_replay_readback_execution_closure_stage_plans:
        Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview>,
    pub guards: Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureGuardPreview>,
    pub blockers: Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_event_store_cutover_replay_readback_execution_closure_readback_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_replay_readback_execution_closure: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub event_store_cutover_replay_readback_execution_closure_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub event_store_cutover_replay_readback_execution_closure_state: &'static str,
    pub required_event_store_cutover_replay_readback_execution_closure_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub event_store_cutover_replay_readback_execution_closure_contract_ready_preview: bool,
    pub append_only_event_store_prerequisite_ready_preview: bool,
    pub replay_execution_disabled_guard_ready_preview: bool,
    pub readback_execution_disabled_guard_ready_preview: bool,
    pub no_execution_proof_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub enables_event_store: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub enforces_adapter_projection: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub prerequisite_gate_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events_after_preview: bool,
    pub enables_event_store_after_preview: bool,
    pub writes_wal_after_preview: bool,
    pub writes_checkpoint_after_preview: bool,
    pub executes_replay_after_preview: bool,
    pub executes_readback_after_preview: bool,
    pub enforces_adapter_projection_after_preview: bool,
    pub mutates_runtime_after_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_cutover_replay_readback_execution_closure: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_event_store_cutover_replay_readback_execution_closure_stage_ids: Vec<&'static str>,
    pub affected_event_store_cutover_replay_readback_execution_closure_plan_ids: Vec<String>,
    pub required_before_event_store_cutover_replay_readback_execution_closure: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub event_store_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub approval_recorded: bool,
    pub replay_readback_execution_closure_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_source_decisions();
    let event_store_cutover_replay_readback_execution_closure_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
            &source_decisions,
        );
    let event_store_cutover_replay_readback_execution_closure_stage_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_stage_plans_from(
            &event_store_cutover_replay_readback_execution_closure_plans,
        );
    let guards =
        work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_guards();
    let blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_blockers_from(
            &event_store_cutover_replay_readback_execution_closure_plans,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_required_prior_gates(
        );

    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_no_persistence",
        upstream_event_store_cutover_runtime_adapter_enforcement_closure_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_RUNTIME_ADAPTER_ENFORCEMENT_CLOSURE_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        event_store_cutover_replay_readback_execution_closure_plan_count: event_store_cutover_replay_readback_execution_closure_plans.len(),
        event_store_cutover_replay_readback_execution_closure_stage_count: event_store_cutover_replay_readback_execution_closure_stage_plans.len(),
        event_store_cutover_replay_readback_execution_closure_stage_source_ref_count: event_store_cutover_replay_readback_execution_closure_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        event_store_cutover_replay_readback_execution_closure_stage_contract_ref_count: event_store_cutover_replay_readback_execution_closure_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        event_store_cutover_replay_readback_execution_closure_plan_stage_ref_count: event_store_cutover_replay_readback_execution_closure_plans
            .iter()
            .map(|plan| plan.required_event_store_cutover_replay_readback_execution_closure_stage_ids.len())
            .sum(),
        event_store_cutover_replay_readback_execution_closure_plan_evidence_field_ref_count: event_store_cutover_replay_readback_execution_closure_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        append_only_work_graph_events_primary_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision
                    .residual_source_blocker_ids
                    .contains(&"append_only_work_graph_events_disabled")
            })
            .count(),
        replay_readback_execution_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision
                    .residual_source_blocker_ids
                    .contains(&"replay_readback_execution_disabled")
            })
            .count(),
        runtime_adapter_enforcement_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision
                    .residual_source_blocker_ids
                    .contains(&"runtime_canonical_adapter_enforcement_disabled")
            })
            .count(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        event_store_cutover_replay_readback_execution_closure_plans,
        event_store_cutover_replay_readback_execution_closure_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_event_store_cutover_replay_readback_execution_closure_readback_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_replay_readback_execution_closure: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans()
-> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_source_decisions();
    work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
        &source_decisions,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_stage_plans()
-> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview> {
    work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_stage_plans_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_guards()
-> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureGuardPreview> {
    vec![
        guard(
            "work_graph_events_persistence_disabled",
            "critical",
            "event_store",
        ),
        guard(
            "event_store_cutover_replay_readback_execution_closure_disabled",
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
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency",
        ),
        guard("approval_recording_disabled", "high", "operator_review"),
        guard(
            "side_effect_lock_not_established",
            "critical",
            "side_effect_lock",
        ),
        guard(
            "no_agent_spawn_or_external_effect",
            "high",
            "external_effects",
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_blockers()
-> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview> {
    work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_blockers_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_RUNTIME_ADAPTER_ENFORCEMENT_CLOSURE_RERUN_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
    source_decisions: &[WorkGraphEventsEventStoreCutoverRuntimeAdapterEnforcementClosureRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview> {
    source_decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"replay_readback_execution_disabled")
        })
        .map(|decision| WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview {
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            event_store_cutover_replay_readback_execution_closure_plan_id: format!(
                "{}_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure",
                decision.source_surface_id
            ),
            previous_enforcement_decision: decision
                .work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_enforcement_decision,
            event_store_cutover_replay_readback_execution_closure_state:
                "work_graph_events_event_store_cutover_replay_readback_execution_closure_packet_ready_preview",
            required_event_store_cutover_replay_readback_execution_closure_stage_ids: EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_STAGE_IDS.to_vec(),
            expected_evidence_field_ids: EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_EVIDENCE_FIELDS.to_vec(),
            residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
            event_store_cutover_replay_readback_execution_closure_contract_ready_preview: true,
            append_only_event_store_prerequisite_ready_preview: true,
            replay_execution_disabled_guard_ready_preview: true,
            readback_execution_disabled_guard_ready_preview: true,
            no_execution_proof_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            enables_event_store: false,
            writes_wal: false,
            writes_checkpoint: false,
            executes_replay: false,
            executes_readback: false,
            enforces_adapter_projection: false,
            mutates_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_stage_plans_from(
    plans: &[WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        stage(
            "work_graph_events_replay_readback_execution_closure_packet",
            "critical",
            "event_store_cutover_replay_readback_execution_closure",
            all_sources.clone(),
            vec![
                "replay_readback_execution_closure_contract_ready",
                "runtime_adapter_enforcement_closure_ready",
                "replay_readback_execution_contract_ready",
                "event_schema_registry_replay_readback_execution_closure_ready",
                "event_store_no_execution_contract_ready",
            ],
        ),
        stage(
            "work_graph_events_replay_execution_disabled_guard",
            "critical",
            "replay_execution_disabled_guard",
            all_sources.clone(),
            vec![
                "replay_executor_disabled_guard_ready",
                "deterministic_replay_no_run_guard_ready",
                "rollback_replay_no_run_guard_ready",
                "cursor_replay_no_advance_guard_ready",
                "no_replay_runtime_mutation_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_readback_execution_disabled_guard",
            "critical",
            "readback_execution_disabled_guard",
            all_sources.clone(),
            vec![
                "readback_probe_disabled_guard_ready",
                "projection_readback_no_probe_guard_ready",
                "timeline_readback_no_query_guard_ready",
                "state_store_readback_no_open_guard_ready",
                "no_readback_runtime_mutation_guard_ready",
                "readback_result_no_persistence_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_append_only_event_store_prerequisite",
            "critical",
            "append_only_event_store_prerequisite",
            all_sources.clone(),
            vec![
                "append_only_work_graph_events_prerequisite_ready",
                "event_store_cutover_switch_prerequisite_ready",
                "wal_write_prerequisite_ready",
                "checkpoint_write_prerequisite_ready",
                "idempotency_index_prerequisite_ready",
            ],
        ),
        stage(
            "work_graph_events_event_store_cutover_no_execution_proof",
            "critical",
            "event_store_cutover_no_execution_proof",
            all_sources.clone(),
            vec![
                "event_store_cutover_no_execution_proof_ready",
                "replay_execution_disabled_proof_ready",
                "readback_execution_disabled_proof_ready",
                "no_wal_checkpoint_replay_proof_ready",
                "no_graph_state_readback_proof_ready",
            ],
        ),
        stage(
            "work_graph_events_replay_readback_execution_closure_blocker_mapping",
            "high",
            "blocker_mapping",
            all_sources,
            vec![
                "append_only_events_disabled_blocker_mapping_ready",
                "replay_readback_disabled_blocker_mapping_ready",
                "no_execution_guard_blocker_mapping_ready",
                "replay_readback_execution_closure_blocker_mapping_ready",
                "replay_readback_execution_closure_readback_missing_blocker_mapping_ready",
            ],
        ),
    ]
}

fn work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_blockers_from(
    plans: &[WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = plans
        .iter()
        .map(|plan| {
            plan.event_store_cutover_replay_readback_execution_closure_plan_id
                .clone()
        })
        .collect::<Vec<_>>();
    vec![
        blocker(
            "append_only_work_graph_events_disabled",
            "critical",
            "event_store_cutover_replay_readback_execution_closure",
            all_sources.clone(),
            EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_STAGE_IDS.to_vec(),
            all_plan_ids.clone(),
            "keep event persistence disabled until event-store cutover replay/readback execution closure readback and append-only event-store cutover readiness are promoted",
        ),
        blocker(
            "replay_readback_execution_disabled",
            "critical",
            "replay_readback",
            all_sources.clone(),
            vec![
                "work_graph_events_replay_execution_disabled_guard",
                "work_graph_events_readback_execution_disabled_guard",
            ],
            all_plan_ids.clone(),
            "keep replay/readback execution disabled until no-execution proof and rollback/readback prerequisites are promoted",
        ),
        blocker(
            "append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_readback_missing",
            "medium",
            "readback_preview",
            all_sources,
            EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_STAGE_IDS.to_vec(),
            all_plan_ids,
            "run event-store cutover replay/readback execution closure readback before applying no-execution outcomes",
        ),
    ]
}

fn stage(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
) -> WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview {
    WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureStagePreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "preview_only_no_event_store_cutover_replay_readback_execution_closure",
        prerequisite_gate_ids: vec![
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_RUNTIME_ADAPTER_ENFORCEMENT_CLOSURE_RERUN_PREVIEW_GATE,
        ],
        contract_ready_preview: true,
        persists_work_graph_events_after_preview: false,
        enables_event_store_after_preview: false,
        writes_wal_after_preview: false,
        writes_checkpoint_after_preview: false,
        executes_replay_after_preview: false,
        executes_readback_after_preview: false,
        enforces_adapter_projection_after_preview: false,
        mutates_runtime_after_preview: false,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureGuardPreview {
    WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_event_store_cutover_replay_readback_execution_closure: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_event_store_cutover_replay_readback_execution_closure_stage_ids: Vec<&'static str>,
    affected_event_store_cutover_replay_readback_execution_closure_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview {
    WorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosureBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_event_store_cutover_replay_readback_execution_closure_stage_ids,
        affected_event_store_cutover_replay_readback_execution_closure_plan_ids,
        required_before_event_store_cutover_replay_readback_execution_closure: true,
        recommended_fix,
    }
}

impl
    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects
{
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            event_store_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            approval_recorded: false,
            replay_readback_execution_closure_established: false,
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
    fn event_store_cutover_replay_readback_execution_closure_plans_preserve_no_persistence_boundary()
     {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
                &sample_source_decisions(),
            );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.event_store_cutover_replay_readback_execution_closure_contract_ready_preview
                && plan.append_only_event_store_prerequisite_ready_preview
                && plan.replay_execution_disabled_guard_ready_preview
                && plan.readback_execution_disabled_guard_ready_preview
                && plan.no_execution_proof_ready_preview
                && !plan.persists_work_graph_events
                && !plan.enables_event_store
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.enforces_adapter_projection
        }));
    }

    #[test]
    fn event_store_cutover_replay_readback_execution_closure_stages_cover_expected_contracts() {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
                &sample_source_decisions(),
            );
        let stages =
            work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_stage_plans_from(
                &plans,
            );

        assert_eq!(stages.len(), 6);
        assert_eq!(
            stages
                .iter()
                .map(|stage| stage.required_contract_ref_ids.len())
                .sum::<usize>(),
            31
        );
        assert!(stages.iter().all(|stage| stage.contract_ready_preview
            && !stage.enables_event_store_after_preview
            && !stage.persists_work_graph_events_after_preview
            && !stage.writes_wal_after_preview
            && !stage.writes_checkpoint_after_preview));
    }

    #[test]
    fn event_store_cutover_replay_readback_execution_closure_blockers_track_primary_residuals() {
        let blockers =
            work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_blockers_from(
                &work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_plans_from(
                    &sample_source_decisions(),
                ),
            );

        assert_eq!(
            blockers
                .iter()
                .map(|blocker| blocker.id)
                .collect::<Vec<_>>(),
            vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
                "append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_readback_missing",
            ]
        );
        assert!(blockers.iter().all(|blocker| {
            blocker.required_before_event_store_cutover_replay_readback_execution_closure
        }));
    }

    #[test]
    fn event_store_cutover_replay_readback_execution_closure_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverReplayReadbackExecutionClosurePreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                event_store_enabled: false,
                wal_written: false,
                checkpoint_written: false,
                replay_executed: false,
                readback_executed: false,
                adapter_projection_enforced: false,
                runtime_mutation_performed: false,
                approval_recorded: false,
                replay_readback_execution_closure_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_source_decisions() -> Vec<
        WorkGraphEventsEventStoreCutoverRuntimeAdapterEnforcementClosureRerunSourceDecisionPreview,
    > {
        vec![
            sample_source_decision("update_plan_tool", "planning"),
            sample_source_decision("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_source_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverRuntimeAdapterEnforcementClosureRerunSourceDecisionPreview
    {
        WorkGraphEventsEventStoreCutoverRuntimeAdapterEnforcementClosureRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_enforcement_decision:
                "deny_append_only_work_graph_events_disabled",
            covered_by_event_store_cutover_runtime_adapter_enforcement_closure_application_preview:
                true,
            event_store_cutover_runtime_adapter_enforcement_closure_contract_ready: true,
            event_store_cutover_runtime_adapter_enforcement_closure_application_applied: false,
            append_only_work_graph_events_enabled: false,
            event_store_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            residual_source_blocker_ids: vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
            ],
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_REPLAY_READBACK_EXECUTION_CLOSURE_PREVIEW_GATE,
        }
    }
}
