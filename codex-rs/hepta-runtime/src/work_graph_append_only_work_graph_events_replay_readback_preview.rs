use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_SHADOW_WRITE_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::WorkGraphEventsShadowWriteRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_replay_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_replay_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_gate";

const REPLAY_READBACK_STAGE_IDS: [&str; 8] = [
    "work_graph_events_replay_cursor_contract",
    "work_graph_events_readback_probe_contract",
    "work_graph_events_duplicate_suppression_contract",
    "work_graph_events_timeline_ordering_contract",
    "work_graph_events_rollback_anchor_contract",
    "work_graph_events_integrity_digest_contract",
    "work_graph_events_no_execution_guard",
    "work_graph_events_replay_readback_blocker_mapping",
];

const REPLAY_READBACK_EVIDENCE_FIELDS: [&str; 10] = [
    "source_surface_id",
    "source_category",
    "shadow_write_rerun_decision_ref",
    "replay_cursor_contract_id",
    "readback_probe_contract_id",
    "duplicate_suppression_contract_id",
    "timeline_ordering_contract_id",
    "rollback_anchor_contract_id",
    "event_integrity_digest_contract_id",
    "residual_source_blocker_ids",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_shadow_write_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub replay_readback_plan_count: usize,
    pub replay_readback_stage_count: usize,
    pub replay_readback_stage_source_ref_count: usize,
    pub replay_readback_stage_contract_ref_count: usize,
    pub replay_readback_plan_stage_ref_count: usize,
    pub replay_readback_plan_evidence_field_ref_count: usize,
    pub replay_cursor_contract_ready_preview_count: usize,
    pub readback_probe_contract_ready_preview_count: usize,
    pub duplicate_suppression_contract_ready_preview_count: usize,
    pub timeline_ordering_contract_ready_preview_count: usize,
    pub rollback_anchor_contract_ready_preview_count: usize,
    pub event_integrity_digest_contract_ready_preview_count: usize,
    pub append_only_work_graph_events_primary_blocked_source_count: usize,
    pub replay_readback_blocked_source_count: usize,
    pub partial_or_gap_blocked_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub replay_readback_plans: Vec<WorkGraphEventsReplayReadbackPlanPreview>,
    pub replay_readback_stage_plans: Vec<WorkGraphEventsReplayReadbackStagePreview>,
    pub guards: Vec<WorkGraphEventsReplayReadbackGuardPreview>,
    pub blockers: Vec<WorkGraphEventsReplayReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_replay_readback_readback_preview: bool,
    pub ready_for_replay_readback_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub replay_readback_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub replay_readback_state: &'static str,
    pub required_replay_readback_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub replay_cursor_contract_ready_preview: bool,
    pub readback_probe_contract_ready_preview: bool,
    pub duplicate_suppression_contract_ready_preview: bool,
    pub timeline_ordering_contract_ready_preview: bool,
    pub rollback_anchor_contract_ready_preview: bool,
    pub event_integrity_digest_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub enforces_adapter_projection: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackStagePreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub prerequisite_gate_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events_after_preview: bool,
    pub writes_wal_after_preview: bool,
    pub writes_checkpoint_after_preview: bool,
    pub mutates_idempotency_index_after_preview: bool,
    pub executes_replay_after_preview: bool,
    pub executes_readback_after_preview: bool,
    pub executes_rollback_after_preview: bool,
    pub mutates_runtime_after_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_replay_readback_execution: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_replay_readback_stage_ids: Vec<&'static str>,
    pub affected_replay_readback_plan_ids: Vec<String>,
    pub required_before_replay_readback_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_replay_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions();
    let replay_readback_plans =
        work_graph_append_only_work_graph_events_replay_readback_plans_from(&source_decisions);
    let replay_readback_stage_plans =
        work_graph_append_only_work_graph_events_replay_readback_stage_plans();
    let guards = work_graph_append_only_work_graph_events_replay_readback_guards();
    let blockers = work_graph_append_only_work_graph_events_replay_readback_blockers();
    let required_prior_gates =
        work_graph_append_only_work_graph_events_replay_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_work_graph_events_replay_readback_preview_no_execution",
        upstream_shadow_write_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_SHADOW_WRITE_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        replay_readback_plan_count: replay_readback_plans.len(),
        replay_readback_stage_count: replay_readback_stage_plans.len(),
        replay_readback_stage_source_ref_count: replay_readback_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        replay_readback_stage_contract_ref_count: replay_readback_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        replay_readback_plan_stage_ref_count: replay_readback_plans
            .iter()
            .map(|plan| plan.required_replay_readback_stage_ids.len())
            .sum(),
        replay_readback_plan_evidence_field_ref_count: replay_readback_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        replay_cursor_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.replay_cursor_contract_ready_preview)
            .count(),
        readback_probe_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.readback_probe_contract_ready_preview)
            .count(),
        duplicate_suppression_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.duplicate_suppression_contract_ready_preview)
            .count(),
        timeline_ordering_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.timeline_ordering_contract_ready_preview)
            .count(),
        rollback_anchor_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.rollback_anchor_contract_ready_preview)
            .count(),
        event_integrity_digest_contract_ready_preview_count: replay_readback_plans
            .iter()
            .filter(|plan| plan.event_integrity_digest_contract_ready_preview)
            .count(),
        append_only_work_graph_events_primary_blocked_source_count: sources_for_blocker(
            &source_decisions,
            "append_only_work_graph_events_disabled",
        )
        .len(),
        replay_readback_blocked_source_count: sources_for_blocker(
            &source_decisions,
            "replay_readback_execution_disabled",
        )
        .len(),
        partial_or_gap_blocked_source_count: sources_for_blocker(
            &source_decisions,
            "canonical_adapter_projection_partial_or_gap",
        )
        .len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        replay_readback_plans,
        replay_readback_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_replay_readback_readback_preview: true,
        ready_for_replay_readback_application_preview: false,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_replay_readback_plans()
-> Vec<WorkGraphEventsReplayReadbackPlanPreview> {
    work_graph_append_only_work_graph_events_replay_readback_plans_from(
        &work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions(),
    )
}

pub fn work_graph_append_only_work_graph_events_replay_readback_stage_plans()
-> Vec<WorkGraphEventsReplayReadbackStagePreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions();
    let all_sources = source_decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        replay_readback_stage(
            "work_graph_events_replay_cursor_contract",
            "critical",
            "replay_cursor",
            all_sources.clone(),
            vec![
                "shadow_replay_cursor_contract_ready",
                "event_sequence_cursor_ready",
                "source_surface_cursor_partition_ready",
                "idempotency_cursor_watermark_ready",
                "cursor_resume_digest_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_readback_probe_contract",
            "critical",
            "readback_probe",
            all_sources.clone(),
            vec![
                "event_projection_probe_contract_ready",
                "source_surface_readback_probe_ready",
                "task_result_readback_probe_ready",
                "timeline_readback_probe_ready",
                "redacted_evidence_probe_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_duplicate_suppression_contract",
            "high",
            "duplicate_suppression",
            all_sources.clone(),
            vec![
                "idempotency_key_collision_check_ready",
                "event_id_duplicate_check_ready",
                "payload_hash_duplicate_check_ready",
                "sequence_gap_duplicate_check_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_timeline_ordering_contract",
            "high",
            "timeline_ordering",
            all_sources.clone(),
            vec![
                "timeline_event_order_contract_ready",
                "parent_child_order_contract_ready",
                "message_link_order_contract_ready",
                "gate_evaluation_order_contract_ready",
                "artifact_order_contract_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_rollback_anchor_contract",
            "high",
            "rollback_anchor",
            all_sources.clone(),
            vec![
                "rollback_anchor_event_ref_ready",
                "checkpoint_anchor_no_write_guard_ready",
                "replay_rewind_boundary_ready",
                "operator_review_anchor_ready",
                "event_integrity_anchor_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_integrity_digest_contract",
            "high",
            "event_integrity",
            all_sources.clone(),
            vec![
                "event_payload_hash_contract_ready",
                "redacted_evidence_digest_ready",
                "event_schema_version_digest_ready",
                "source_surface_digest_ready",
                "timeline_digest_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_no_execution_guard",
            "critical",
            "no_execution_guard",
            all_sources.clone(),
            vec![
                "work_graph_events_no_persist_guard_ready",
                "wal_no_write_guard_ready",
                "checkpoint_no_write_guard_ready",
                "replay_execution_disabled_guard_ready",
                "readback_execution_disabled_guard_ready",
                "rollback_execution_disabled_guard_ready",
                "agent_spawn_noop_guard_ready",
            ],
        ),
        replay_readback_stage(
            "work_graph_events_replay_readback_blocker_mapping",
            "high",
            "blocker_mapping",
            all_sources,
            vec![
                "append_only_events_disabled_blocker_mapping_ready",
                "replay_readback_disabled_blocker_mapping_ready",
                "partial_gap_blocker_mapping_ready",
                "adapter_enforcement_blocker_mapping_ready",
            ],
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_replay_readback_guards()
-> Vec<WorkGraphEventsReplayReadbackGuardPreview> {
    vec![
        replay_readback_guard(
            "work_graph_events_persistence_disabled",
            "critical",
            "event_store",
        ),
        replay_readback_guard("wal_write_disabled", "critical", "wal"),
        replay_readback_guard("checkpoint_write_disabled", "critical", "checkpoint"),
        replay_readback_guard("replay_execution_disabled", "critical", "replay"),
        replay_readback_guard("readback_execution_disabled", "critical", "readback"),
        replay_readback_guard("rollback_execution_disabled", "critical", "rollback"),
        replay_readback_guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency",
        ),
        replay_readback_guard(
            "adapter_projection_enforcement_disabled",
            "critical",
            "adapter_projection",
        ),
        replay_readback_guard(
            "scheduler_admission_enforcement_disabled",
            "high",
            "scheduler_admission",
        ),
        replay_readback_guard("no_agent_spawn", "high", "agent_spawn"),
        replay_readback_guard(
            "no_external_send_or_model_invocation",
            "high",
            "external_effects",
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_replay_readback_blockers()
-> Vec<WorkGraphEventsReplayReadbackBlockerPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_shadow_write_rerun_source_decisions();
    let plans =
        work_graph_append_only_work_graph_events_replay_readback_plans_from(&source_decisions);
    let all_sources = source_decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = plans
        .iter()
        .map(|plan| plan.replay_readback_plan_id.clone())
        .collect::<Vec<_>>();
    let all_stage_ids = REPLAY_READBACK_STAGE_IDS.to_vec();

    vec![
        replay_readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            "append_only_fact_source",
            all_sources.clone(),
            all_stage_ids.clone(),
            all_plan_ids.clone(),
            "keep WorkGraph event persistence disabled until replay/readback contracts are read back and applied",
        ),
        replay_readback_blocker(
            "replay_readback_execution_disabled",
            "high",
            "replay_readback",
            all_sources.clone(),
            all_stage_ids.clone(),
            all_plan_ids.clone(),
            "keep replay/readback execution disabled until operator review and side-effect lock are promoted",
        ),
        replay_readback_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "runtime_adapter_enforcement",
            all_sources.clone(),
            vec![
                "work_graph_events_readback_probe_contract",
                "work_graph_events_timeline_ordering_contract",
                "work_graph_events_replay_readback_blocker_mapping",
            ],
            all_plan_ids.clone(),
            "keep canonical adapter enforcement disabled until append-only events replay/readback is verified",
        ),
        replay_readback_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            sources_for_blocker(
                &source_decisions,
                "canonical_adapter_projection_partial_or_gap",
            ),
            vec![
                "work_graph_events_readback_probe_contract",
                "work_graph_events_replay_readback_blocker_mapping",
            ],
            all_plan_ids.clone(),
            "close partial/gap adapter source mappings before authoritative event replay/readback",
        ),
        replay_readback_blocker(
            "append_only_work_graph_events_replay_readback_readback_missing",
            "medium",
            "readback_preview",
            all_sources,
            all_stage_ids,
            all_plan_ids,
            "run replay/readback readback preview before applying no-execution outcomes",
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_replay_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_SHADOW_WRITE_RERUN_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_replay_readback_plans_from(
    source_decisions: &[WorkGraphEventsShadowWriteRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsReplayReadbackPlanPreview> {
    source_decisions
        .iter()
        .map(|decision| WorkGraphEventsReplayReadbackPlanPreview {
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            replay_readback_plan_id: format!(
                "{}_append_only_work_graph_events_replay_readback",
                decision.source_surface_id
            ),
            previous_enforcement_decision: decision
                .work_graph_events_shadow_write_rerun_enforcement_decision,
            replay_readback_state: "work_graph_events_replay_readback_contract_ready_preview",
            required_replay_readback_stage_ids: REPLAY_READBACK_STAGE_IDS.to_vec(),
            expected_evidence_field_ids: REPLAY_READBACK_EVIDENCE_FIELDS.to_vec(),
            residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
            replay_cursor_contract_ready_preview: true,
            readback_probe_contract_ready_preview: true,
            duplicate_suppression_contract_ready_preview: true,
            timeline_ordering_contract_ready_preview: true,
            rollback_anchor_contract_ready_preview: true,
            event_integrity_digest_contract_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            writes_wal: false,
            writes_checkpoint: false,
            mutates_idempotency_index: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            enforces_adapter_projection: false,
            mutates_runtime: false,
        })
        .collect()
}

fn sources_for_blocker(
    source_decisions: &[WorkGraphEventsShadowWriteRerunSourceDecisionPreview],
    blocker_id: &'static str,
) -> Vec<&'static str> {
    source_decisions
        .iter()
        .filter(|decision| decision.residual_source_blocker_ids.contains(&blocker_id))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn replay_readback_stage(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
) -> WorkGraphEventsReplayReadbackStagePreview {
    WorkGraphEventsReplayReadbackStagePreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "preview_only_no_replay_readback_execution",
        prerequisite_gate_ids: vec![
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_SHADOW_WRITE_RERUN_PREVIEW_GATE,
        ],
        contract_ready_preview: true,
        persists_work_graph_events_after_preview: false,
        writes_wal_after_preview: false,
        writes_checkpoint_after_preview: false,
        mutates_idempotency_index_after_preview: false,
        executes_replay_after_preview: false,
        executes_readback_after_preview: false,
        executes_rollback_after_preview: false,
        mutates_runtime_after_preview: false,
    }
}

fn replay_readback_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsReplayReadbackGuardPreview {
    WorkGraphEventsReplayReadbackGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_replay_readback_execution: true,
        satisfied_by_preview: false,
    }
}

fn replay_readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_replay_readback_stage_ids: Vec<&'static str>,
    affected_replay_readback_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsReplayReadbackBlockerPreview {
    WorkGraphEventsReplayReadbackBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_replay_readback_stage_ids,
        affected_replay_readback_plan_ids,
        required_before_replay_readback_execution: true,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects {
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            replay_executed: false,
            readback_executed: false,
            rollback_executed: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_readback_plans_preserve_no_execution_boundary() {
        let plans = work_graph_append_only_work_graph_events_replay_readback_plans_from(
            &sample_decisions(),
        );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.replay_cursor_contract_ready_preview
                && plan.readback_probe_contract_ready_preview
                && plan.duplicate_suppression_contract_ready_preview
                && plan.timeline_ordering_contract_ready_preview
                && plan.rollback_anchor_contract_ready_preview
                && plan.event_integrity_digest_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.persists_work_graph_events
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.mutates_idempotency_index
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.enforces_adapter_projection
                && !plan.mutates_runtime
        }));
    }

    #[test]
    fn replay_readback_stages_cover_core_contracts() {
        let stages = sample_stages();
        let stage_ids = stages.iter().map(|stage| stage.id).collect::<Vec<_>>();

        assert_eq!(stage_ids, REPLAY_READBACK_STAGE_IDS);
        assert!(stages.iter().all(|stage| {
            stage.contract_ready_preview
                && !stage.persists_work_graph_events_after_preview
                && !stage.writes_wal_after_preview
                && !stage.writes_checkpoint_after_preview
                && !stage.mutates_idempotency_index_after_preview
                && !stage.executes_replay_after_preview
                && !stage.executes_readback_after_preview
                && !stage.executes_rollback_after_preview
                && !stage.mutates_runtime_after_preview
        }));
    }

    #[test]
    fn replay_readback_guards_and_side_effects_stay_disabled() {
        let guards = work_graph_append_only_work_graph_events_replay_readback_guards();

        assert_eq!(guards.len(), 11);
        assert!(
            guards
                .iter()
                .all(|guard| guard.required_before_replay_readback_execution
                    && !guard.satisfied_by_preview)
        );
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                durable_store_switch_enabled: false,
                idempotency_index_mutated: false,
                replay_executed: false,
                readback_executed: false,
                rollback_executed: false,
                adapter_projection_enforced: false,
                runtime_mutation_performed: false,
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

    #[test]
    fn replay_readback_source_blocker_selection_is_deterministic() {
        let decisions = sample_decisions();

        assert_eq!(
            sources_for_blocker(&decisions, "append_only_work_graph_events_disabled"),
            vec!["update_plan_tool", "multi_agent_v2_thread_spawn"]
        );
        assert_eq!(
            sources_for_blocker(&decisions, "canonical_adapter_projection_partial_or_gap"),
            vec!["update_plan_tool"]
        );
    }

    fn sample_decisions() -> Vec<WorkGraphEventsShadowWriteRerunSourceDecisionPreview> {
        vec![
            sample_decision(
                "update_plan_tool",
                "planning",
                vec![
                    "append_only_work_graph_events_disabled",
                    "replay_readback_execution_disabled",
                    "canonical_adapter_projection_partial_or_gap",
                ],
            ),
            sample_decision(
                "multi_agent_v2_thread_spawn",
                "multi_agent",
                vec![
                    "append_only_work_graph_events_disabled",
                    "replay_readback_execution_disabled",
                ],
            ),
        ]
    }

    fn sample_stages() -> Vec<WorkGraphEventsReplayReadbackStagePreview> {
        REPLAY_READBACK_STAGE_IDS
            .iter()
            .map(|id| {
                replay_readback_stage(
                    id,
                    "high",
                    "sample",
                    vec!["update_plan_tool"],
                    vec!["sample_contract_ready"],
                )
            })
            .collect()
    }

    fn sample_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
        residual_source_blocker_ids: Vec<&'static str>,
    ) -> WorkGraphEventsShadowWriteRerunSourceDecisionPreview {
        WorkGraphEventsShadowWriteRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_shadow_write_rerun_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            covered_by_shadow_write_application_preview: true,
            shadow_write_contract_ready: true,
            shadow_write_application_applied: false,
            append_only_work_graph_events_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            scheduler_admission_enforcement_ready: false,
            task_result_enforcement_ready: false,
            role_manifest_enforcement_ready: false,
            residual_source_blocker_ids,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE,
        }
    }
}
