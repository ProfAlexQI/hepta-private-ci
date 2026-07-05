use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_NO_CUTOVER_GUARD_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview::WorkGraphEventsEventStoreCutoverNoCutoverGuardRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_no_cutover_guard_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_readback_preview_gate";

const EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_STAGE_IDS: [&str; 6] = [
    "work_graph_events_event_store_cutover_adapter_projection_gap_closure_packet",
    "work_graph_events_source_projection_gap_evidence",
    "work_graph_events_runtime_enforcement_prerequisite",
    "work_graph_events_no_enforcement_guard",
    "work_graph_events_replay_readback_prerequisite",
    "work_graph_events_event_store_cutover_adapter_projection_gap_closure_blocker_mapping",
];

const EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_EVIDENCE_FIELDS: [&str; 10] = [
    "source_surface_id",
    "source_category",
    "event_store_cutover_no_cutover_guard_rerun_decision_ref",
    "event_store_cutover_adapter_projection_gap_closure_packet_id",
    "source_projection_gap_evidence_id",
    "runtime_enforcement_prerequisite_contract_id",
    "no_enforcement_guard_id",
    "replay_readback_prerequisite_contract_id",
    "residual_source_blocker_ids",
    "next_required_gate",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_event_store_cutover_no_cutover_guard_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_plan_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_stage_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_stage_source_ref_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_stage_contract_ref_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_plan_stage_ref_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_plan_evidence_field_ref_count: usize,
    pub append_only_work_graph_events_primary_blocked_source_count: usize,
    pub replay_readback_execution_blocked_source_count: usize,
    pub partial_or_gap_blocked_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub event_store_cutover_adapter_projection_gap_closure_plans:
        Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview>,
    pub event_store_cutover_adapter_projection_gap_closure_stage_plans:
        Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview>,
    pub guards: Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureGuardPreview>,
    pub blockers: Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_event_store_cutover_adapter_projection_gap_closure_readback_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_adapter_projection_gap_closure: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub event_store_cutover_adapter_projection_gap_closure_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub event_store_cutover_adapter_projection_gap_closure_state: &'static str,
    pub required_event_store_cutover_adapter_projection_gap_closure_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub event_store_cutover_adapter_projection_gap_closure_contract_ready_preview: bool,
    pub append_only_event_store_persistence_guard_ready_preview: bool,
    pub operator_review_no_approval_guard_ready_preview: bool,
    pub replay_readback_prerequisite_ready_preview: bool,
    pub adapter_enforcement_prerequisite_ready_preview: bool,
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
pub struct WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview {
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
pub struct WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_cutover_adapter_projection_gap_closure: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_event_store_cutover_adapter_projection_gap_closure_stage_ids: Vec<&'static str>,
    pub affected_event_store_cutover_adapter_projection_gap_closure_plan_ids: Vec<String>,
    pub required_before_event_store_cutover_adapter_projection_gap_closure: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects
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
    pub adapter_projection_gap_closure_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_no_cutover_guard_rerun_source_decisions();
    let event_store_cutover_adapter_projection_gap_closure_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
            &source_decisions,
        );
    let event_store_cutover_adapter_projection_gap_closure_stage_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_stage_plans_from(
            &event_store_cutover_adapter_projection_gap_closure_plans,
        );
    let guards =
        work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_guards();
    let blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_blockers_from(
            &event_store_cutover_adapter_projection_gap_closure_plans,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_required_prior_gates(
        );

    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_no_persistence",
        upstream_event_store_cutover_no_cutover_guard_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_NO_CUTOVER_GUARD_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        event_store_cutover_adapter_projection_gap_closure_plan_count: event_store_cutover_adapter_projection_gap_closure_plans.len(),
        event_store_cutover_adapter_projection_gap_closure_stage_count: event_store_cutover_adapter_projection_gap_closure_stage_plans.len(),
        event_store_cutover_adapter_projection_gap_closure_stage_source_ref_count: event_store_cutover_adapter_projection_gap_closure_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        event_store_cutover_adapter_projection_gap_closure_stage_contract_ref_count: event_store_cutover_adapter_projection_gap_closure_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        event_store_cutover_adapter_projection_gap_closure_plan_stage_ref_count: event_store_cutover_adapter_projection_gap_closure_plans
            .iter()
            .map(|plan| plan.required_event_store_cutover_adapter_projection_gap_closure_stage_ids.len())
            .sum(),
        event_store_cutover_adapter_projection_gap_closure_plan_evidence_field_ref_count: event_store_cutover_adapter_projection_gap_closure_plans
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
        partial_or_gap_blocked_source_count: source_decisions
            .iter()
            .filter(|decision| {
                decision
                    .residual_source_blocker_ids
                    .contains(&"canonical_adapter_projection_partial_or_gap")
            })
            .count(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        event_store_cutover_adapter_projection_gap_closure_plans,
        event_store_cutover_adapter_projection_gap_closure_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_event_store_cutover_adapter_projection_gap_closure_readback_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_adapter_projection_gap_closure: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans()
-> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_no_cutover_guard_rerun_source_decisions();
    work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
        &source_decisions,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_stage_plans()
-> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview> {
    work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_stage_plans_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_guards()
-> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureGuardPreview> {
    vec![
        guard(
            "work_graph_events_persistence_disabled",
            "critical",
            "event_store",
        ),
        guard(
            "event_store_cutover_adapter_projection_gap_closure_disabled",
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

pub fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_blockers()
-> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview> {
    work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_blockers_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_NO_CUTOVER_GUARD_RERUN_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
    source_decisions: &[WorkGraphEventsEventStoreCutoverNoCutoverGuardRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview> {
    source_decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"canonical_adapter_projection_partial_or_gap")
        })
        .map(|decision| WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview {
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            event_store_cutover_adapter_projection_gap_closure_plan_id: format!(
                "{}_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure",
                decision.source_surface_id
            ),
            previous_enforcement_decision: decision
                .work_graph_events_event_store_cutover_no_cutover_guard_rerun_enforcement_decision,
            event_store_cutover_adapter_projection_gap_closure_state:
                "work_graph_events_event_store_cutover_adapter_projection_gap_closure_packet_ready_preview",
            required_event_store_cutover_adapter_projection_gap_closure_stage_ids: EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_STAGE_IDS.to_vec(),
            expected_evidence_field_ids: EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_EVIDENCE_FIELDS.to_vec(),
            residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
            event_store_cutover_adapter_projection_gap_closure_contract_ready_preview: true,
            append_only_event_store_persistence_guard_ready_preview: true,
            operator_review_no_approval_guard_ready_preview: true,
            replay_readback_prerequisite_ready_preview: true,
            adapter_enforcement_prerequisite_ready_preview: true,
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

fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_stage_plans_from(
    plans: &[WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        stage(
            "work_graph_events_event_store_cutover_adapter_projection_gap_closure_packet",
            "critical",
            "event_store_cutover_adapter_projection_gap_closure",
            all_sources.clone(),
            vec![
                "adapter_projection_gap_closure_contract_ready",
                "canonical_adapter_coverage_closure_ready",
                "event_store_cutover_switch_disabled_ready",
                "event_schema_registry_adapter_projection_gap_closure_ready",
                "event_store_no_enforcement_contract_ready",
            ],
        ),
        stage(
            "work_graph_events_source_projection_gap_evidence",
            "critical",
            "source_projection_gap_evidence",
            all_sources.clone(),
            vec![
                "source_projection_gap_evidence_packet_ready",
                "adapter_source_surface_mapping_ready",
                "missing_projection_path_evidence_ready",
                "partial_projection_gap_evidence_ready",
                "source_category_projection_contract_ready",
                "no_projection_mutation_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_runtime_enforcement_prerequisite",
            "critical",
            "runtime_enforcement_prerequisite",
            all_sources.clone(),
            vec![
                "canonical_adapter_enforcement_prerequisite_ready",
                "scheduler_admission_prerequisite_ready",
                "terminal_task_result_prerequisite_ready",
                "role_manifest_prerequisite_ready",
                "projection_partial_gap_prerequisite_ready",
            ],
        ),
        stage(
            "work_graph_events_no_enforcement_guard",
            "critical",
            "no_enforcement_guard",
            all_sources.clone(),
            vec![
                "runtime_adapter_enforcement_disabled_guard_ready",
                "projection_enforcement_disabled_guard_ready",
                "scheduler_admission_disabled_guard_ready",
                "terminal_task_result_enforcement_disabled_guard_ready",
                "no_runtime_mutation_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_replay_readback_prerequisite",
            "critical",
            "replay_readback",
            all_sources.clone(),
            vec![
                "replay_cursor_prerequisite_contract_ready",
                "readback_probe_prerequisite_contract_ready",
                "rollback_anchor_prerequisite_contract_ready",
                "duplicate_suppression_prerequisite_contract_ready",
                "timeline_order_prerequisite_contract_ready",
            ],
        ),
        stage(
            "work_graph_events_event_store_cutover_adapter_projection_gap_closure_blocker_mapping",
            "high",
            "blocker_mapping",
            all_sources,
            vec![
                "append_only_events_disabled_blocker_mapping_ready",
                "replay_readback_disabled_blocker_mapping_ready",
                "adapter_enforcement_disabled_blocker_mapping_ready",
                "partial_gap_blocker_mapping_ready",
                "adapter_projection_gap_closure_readback_missing_blocker_mapping_ready",
            ],
        ),
    ]
}

fn work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_blockers_from(
    plans: &[WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = plans
        .iter()
        .map(|plan| {
            plan.event_store_cutover_adapter_projection_gap_closure_plan_id
                .clone()
        })
        .collect::<Vec<_>>();
    let partial_gap_sources = plans
        .iter()
        .filter(|plan| {
            plan.residual_source_blocker_ids
                .contains(&"canonical_adapter_projection_partial_or_gap")
        })
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        blocker(
            "append_only_work_graph_events_disabled",
            "critical",
            "event_store_cutover_adapter_projection_gap_closure",
            all_sources.clone(),
            EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_STAGE_IDS.to_vec(),
            all_plan_ids.clone(),
            "keep event persistence disabled until event-store cutover adapter-projection gap closure readback and cutover_adapter_projection_gap_closure readiness are promoted",
        ),
        blocker(
            "replay_readback_execution_disabled",
            "critical",
            "replay_readback",
            all_sources.clone(),
            vec!["work_graph_events_replay_readback_prerequisite"],
            all_plan_ids.clone(),
            "keep replay/readback execution disabled until event-store cutover adapter-projection gap closure and rollback anchors are promoted",
        ),
        blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "adapter_enforcement",
            all_sources.clone(),
            vec![
                "work_graph_events_event_store_cutover_adapter_projection_gap_closure_blocker_mapping",
            ],
            all_plan_ids.clone(),
            "keep runtime adapter enforcement disabled until append-only event-store cutover adapter-projection gap closure is promoted",
        ),
        blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            partial_gap_sources,
            vec![
                "work_graph_events_event_store_cutover_adapter_projection_gap_closure_blocker_mapping",
            ],
            all_plan_ids.clone(),
            "close partial/gap adapter projections before authoritative event-store cutover adapter-projection gap closure",
        ),
        blocker(
            "append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_readback_missing",
            "medium",
            "readback_preview",
            all_sources,
            EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_STAGE_IDS.to_vec(),
            all_plan_ids,
            "run event-store cutover adapter-projection gap closure readback before applying no-persistence outcomes",
        ),
    ]
}

fn stage(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
) -> WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview {
    WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureStagePreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "preview_only_no_event_store_cutover_adapter_projection_gap_closure",
        prerequisite_gate_ids: vec![
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_NO_CUTOVER_GUARD_RERUN_PREVIEW_GATE,
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
) -> WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureGuardPreview {
    WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_event_store_cutover_adapter_projection_gap_closure: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_event_store_cutover_adapter_projection_gap_closure_stage_ids: Vec<&'static str>,
    affected_event_store_cutover_adapter_projection_gap_closure_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview {
    WorkGraphEventsEventStoreCutoverAdapterProjectionGapClosureBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_event_store_cutover_adapter_projection_gap_closure_stage_ids,
        affected_event_store_cutover_adapter_projection_gap_closure_plan_ids,
        required_before_event_store_cutover_adapter_projection_gap_closure: true,
        recommended_fix,
    }
}

impl
    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects
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
            adapter_projection_gap_closure_established: false,
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
    fn event_store_cutover_adapter_projection_gap_closure_plans_preserve_no_persistence_boundary() {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
                &sample_source_decisions(),
            );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.event_store_cutover_adapter_projection_gap_closure_contract_ready_preview
                && plan.append_only_event_store_persistence_guard_ready_preview
                && plan.operator_review_no_approval_guard_ready_preview
                && plan.replay_readback_prerequisite_ready_preview
                && plan.adapter_enforcement_prerequisite_ready_preview
                && !plan.persists_work_graph_events
                && !plan.enables_event_store
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.enforces_adapter_projection
        }));
    }

    #[test]
    fn event_store_cutover_adapter_projection_gap_closure_stages_cover_expected_contracts() {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
                &sample_source_decisions(),
            );
        let stages =
            work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_stage_plans_from(
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
    fn event_store_cutover_adapter_projection_gap_closure_blockers_track_primary_residuals() {
        let blockers =
            work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_blockers_from(
                &work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_plans_from(
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
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_readback_missing",
            ]
        );
        assert!(blockers.iter().all(|blocker| {
            blocker.required_before_event_store_cutover_adapter_projection_gap_closure
        }));
    }

    #[test]
    fn event_store_cutover_adapter_projection_gap_closure_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAdapterProjectionGapClosurePreviewSideEffects {
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
                adapter_projection_gap_closure_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_source_decisions()
    -> Vec<WorkGraphEventsEventStoreCutoverNoCutoverGuardRerunSourceDecisionPreview> {
        vec![
            sample_source_decision("update_plan_tool", "planning"),
            sample_source_decision("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_source_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverNoCutoverGuardRerunSourceDecisionPreview {
        WorkGraphEventsEventStoreCutoverNoCutoverGuardRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_event_store_cutover_no_cutover_guard_rerun_enforcement_decision:
                "deny_append_only_work_graph_events_disabled",
            covered_by_event_store_cutover_no_cutover_guard_application_preview: true,
            event_store_cutover_no_cutover_guard_contract_ready: true,
            event_store_cutover_no_cutover_guard_application_applied: false,
            append_only_work_graph_events_enabled: false,
            event_store_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            residual_source_blocker_ids: vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
                "canonical_adapter_projection_partial_or_gap",
            ],
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_ADAPTER_PROJECTION_GAP_CLOSURE_PREVIEW_GATE,
        }
    }
}
