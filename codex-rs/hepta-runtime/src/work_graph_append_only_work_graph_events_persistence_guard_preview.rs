use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview::WorkGraphEventsReplayReadbackRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview::work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_persistence_guard_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_persistence_guard_readback_preview_gate";

const PERSISTENCE_GUARD_STAGE_IDS: [&str; 6] = [
    "work_graph_events_persistence_guard_contract",
    "work_graph_events_event_store_enablement_contract",
    "work_graph_events_replay_readback_execution_prerequisite",
    "work_graph_events_adapter_enforcement_guard",
    "work_graph_events_no_persistence_guard",
    "work_graph_events_persistence_guard_blocker_mapping",
];

const PERSISTENCE_GUARD_EVIDENCE_FIELDS: [&str; 10] = [
    "source_surface_id",
    "source_category",
    "replay_readback_rerun_decision_ref",
    "persistence_guard_contract_id",
    "event_store_enablement_contract_id",
    "replay_readback_prerequisite_contract_id",
    "adapter_enforcement_guard_contract_id",
    "no_persistence_guard_id",
    "residual_source_blocker_ids",
    "next_required_gate",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_replay_readback_rerun_gate: &'static str,
    pub source_surface_count: usize,
    pub persistence_guard_plan_count: usize,
    pub persistence_guard_stage_count: usize,
    pub persistence_guard_stage_source_ref_count: usize,
    pub persistence_guard_stage_contract_ref_count: usize,
    pub persistence_guard_plan_stage_ref_count: usize,
    pub persistence_guard_plan_evidence_field_ref_count: usize,
    pub append_only_work_graph_events_primary_blocked_source_count: usize,
    pub replay_readback_execution_blocked_source_count: usize,
    pub partial_or_gap_blocked_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub persistence_guard_plans: Vec<WorkGraphEventsPersistenceGuardPlanPreview>,
    pub persistence_guard_stage_plans: Vec<WorkGraphEventsPersistenceGuardStagePreview>,
    pub guards: Vec<WorkGraphEventsPersistenceGuardGuardPreview>,
    pub blockers: Vec<WorkGraphEventsPersistenceGuardBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_persistence_guard_readback_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_enablement: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub persistence_guard_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub persistence_guard_state: &'static str,
    pub required_persistence_guard_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub persistence_guard_contract_ready_preview: bool,
    pub event_store_enablement_contract_ready_preview: bool,
    pub replay_readback_prerequisite_ready_preview: bool,
    pub adapter_enforcement_guard_ready_preview: bool,
    pub no_persistence_guard_ready_preview: bool,
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
pub struct WorkGraphEventsPersistenceGuardStagePreview {
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
pub struct WorkGraphEventsPersistenceGuardGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_enablement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_persistence_guard_stage_ids: Vec<&'static str>,
    pub affected_persistence_guard_plan_ids: Vec<String>,
    pub required_before_event_store_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects {
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
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_source_decisions();
    let persistence_guard_plans =
        work_graph_append_only_work_graph_events_persistence_guard_plans_from(&source_decisions);
    let persistence_guard_stage_plans =
        work_graph_append_only_work_graph_events_persistence_guard_stage_plans_from(
            &persistence_guard_plans,
        );
    let guards = work_graph_append_only_work_graph_events_persistence_guard_guards();
    let blockers = work_graph_append_only_work_graph_events_persistence_guard_blockers_from(
        &persistence_guard_plans,
    );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_persistence_guard_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_persistence_guard_preview_no_persistence",
        upstream_replay_readback_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE,
        source_surface_count: source_decisions.len(),
        persistence_guard_plan_count: persistence_guard_plans.len(),
        persistence_guard_stage_count: persistence_guard_stage_plans.len(),
        persistence_guard_stage_source_ref_count: persistence_guard_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        persistence_guard_stage_contract_ref_count: persistence_guard_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        persistence_guard_plan_stage_ref_count: persistence_guard_plans
            .iter()
            .map(|plan| plan.required_persistence_guard_stage_ids.len())
            .sum(),
        persistence_guard_plan_evidence_field_ref_count: persistence_guard_plans
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
        persistence_guard_plans,
        persistence_guard_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_RECOMMENDED_NEXT_GATE,
        ready_for_persistence_guard_readback_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_enablement: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_plans()
-> Vec<WorkGraphEventsPersistenceGuardPlanPreview> {
    let source_decisions =
        work_graph_unified_projection_enforcement_work_graph_events_replay_readback_rerun_source_decisions();
    work_graph_append_only_work_graph_events_persistence_guard_plans_from(&source_decisions)
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_stage_plans()
-> Vec<WorkGraphEventsPersistenceGuardStagePreview> {
    work_graph_append_only_work_graph_events_persistence_guard_stage_plans_from(
        &work_graph_append_only_work_graph_events_persistence_guard_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_guards()
-> Vec<WorkGraphEventsPersistenceGuardGuardPreview> {
    vec![
        guard(
            "work_graph_events_persistence_disabled",
            "critical",
            "event_store",
        ),
        guard("event_store_enablement_disabled", "critical", "event_store"),
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

pub fn work_graph_append_only_work_graph_events_persistence_guard_blockers()
-> Vec<WorkGraphEventsPersistenceGuardBlockerPreview> {
    work_graph_append_only_work_graph_events_persistence_guard_blockers_from(
        &work_graph_append_only_work_graph_events_persistence_guard_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_persistence_guard_plans_from(
    source_decisions: &[WorkGraphEventsReplayReadbackRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsPersistenceGuardPlanPreview> {
    source_decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"append_only_work_graph_events_disabled")
        })
        .map(|decision| WorkGraphEventsPersistenceGuardPlanPreview {
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            persistence_guard_plan_id: format!(
                "{}_append_only_work_graph_events_persistence_guard",
                decision.source_surface_id
            ),
            previous_enforcement_decision: decision
                .work_graph_events_replay_readback_rerun_enforcement_decision,
            persistence_guard_state: "work_graph_events_persistence_guard_contract_ready_preview",
            required_persistence_guard_stage_ids: PERSISTENCE_GUARD_STAGE_IDS.to_vec(),
            expected_evidence_field_ids: PERSISTENCE_GUARD_EVIDENCE_FIELDS.to_vec(),
            residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
            persistence_guard_contract_ready_preview: true,
            event_store_enablement_contract_ready_preview: true,
            replay_readback_prerequisite_ready_preview: true,
            adapter_enforcement_guard_ready_preview: true,
            no_persistence_guard_ready_preview: true,
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

fn work_graph_append_only_work_graph_events_persistence_guard_stage_plans_from(
    plans: &[WorkGraphEventsPersistenceGuardPlanPreview],
) -> Vec<WorkGraphEventsPersistenceGuardStagePreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();

    vec![
        stage(
            "work_graph_events_persistence_guard_contract",
            "critical",
            "persistence_guard",
            all_sources.clone(),
            vec![
                "event_persistence_guard_contract_ready",
                "source_surface_persistence_boundary_ready",
                "redacted_evidence_persistence_boundary_ready",
                "idempotency_key_persistence_boundary_ready",
                "event_store_disable_switch_ready",
            ],
        ),
        stage(
            "work_graph_events_event_store_enablement_contract",
            "critical",
            "event_store_enablement",
            all_sources.clone(),
            vec![
                "append_only_event_store_enablement_contract_ready",
                "event_schema_registry_enablement_contract_ready",
                "event_sequence_allocator_contract_ready",
                "event_store_replay_cursor_contract_ready",
                "event_store_operator_disable_contract_ready",
            ],
        ),
        stage(
            "work_graph_events_replay_readback_execution_prerequisite",
            "critical",
            "replay_readback",
            all_sources.clone(),
            vec![
                "replay_execution_prerequisite_contract_ready",
                "readback_probe_prerequisite_contract_ready",
                "rollback_anchor_prerequisite_contract_ready",
                "duplicate_suppression_prerequisite_contract_ready",
                "timeline_order_prerequisite_contract_ready",
            ],
        ),
        stage(
            "work_graph_events_adapter_enforcement_guard",
            "high",
            "adapter_enforcement",
            all_sources.clone(),
            vec![
                "canonical_adapter_enforcement_guard_ready",
                "scheduler_admission_enforcement_guard_ready",
                "terminal_task_result_enforcement_guard_ready",
                "role_manifest_enforcement_guard_ready",
                "projection_partial_gap_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_no_persistence_guard",
            "critical",
            "no_persistence_guard",
            all_sources.clone(),
            vec![
                "no_event_store_write_guard_ready",
                "no_wal_write_guard_ready",
                "no_checkpoint_write_guard_ready",
                "no_replay_execution_guard_ready",
                "no_readback_execution_guard_ready",
                "no_adapter_enforcement_guard_ready",
            ],
        ),
        stage(
            "work_graph_events_persistence_guard_blocker_mapping",
            "high",
            "blocker_mapping",
            all_sources,
            vec![
                "append_only_events_disabled_blocker_mapping_ready",
                "replay_readback_disabled_blocker_mapping_ready",
                "adapter_enforcement_disabled_blocker_mapping_ready",
                "partial_gap_blocker_mapping_ready",
                "readback_missing_blocker_mapping_ready",
            ],
        ),
    ]
}

fn work_graph_append_only_work_graph_events_persistence_guard_blockers_from(
    plans: &[WorkGraphEventsPersistenceGuardPlanPreview],
) -> Vec<WorkGraphEventsPersistenceGuardBlockerPreview> {
    let all_sources = plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = plans
        .iter()
        .map(|plan| plan.persistence_guard_plan_id.clone())
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
            "event_store_enablement",
            all_sources.clone(),
            PERSISTENCE_GUARD_STAGE_IDS.to_vec(),
            all_plan_ids.clone(),
            "keep event persistence disabled until persistence guard readback and event-store enablement are promoted",
        ),
        blocker(
            "replay_readback_execution_disabled",
            "critical",
            "replay_readback",
            all_sources.clone(),
            vec!["work_graph_events_replay_readback_execution_prerequisite"],
            all_plan_ids.clone(),
            "keep replay/readback execution disabled until event-store persistence and rollback anchors are promoted",
        ),
        blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "adapter_enforcement",
            all_sources.clone(),
            vec!["work_graph_events_adapter_enforcement_guard"],
            all_plan_ids.clone(),
            "keep runtime adapter enforcement disabled until append-only events are persisted and read back",
        ),
        blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            partial_gap_sources,
            vec!["work_graph_events_adapter_enforcement_guard"],
            all_plan_ids.clone(),
            "close partial/gap adapter projections before authoritative event persistence",
        ),
        blocker(
            "append_only_work_graph_events_persistence_guard_readback_missing",
            "medium",
            "readback_preview",
            all_sources,
            PERSISTENCE_GUARD_STAGE_IDS.to_vec(),
            all_plan_ids,
            "run persistence guard readback before applying no-persistence outcomes",
        ),
    ]
}

fn stage(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
) -> WorkGraphEventsPersistenceGuardStagePreview {
    WorkGraphEventsPersistenceGuardStagePreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "preview_only_no_event_persistence",
        prerequisite_gate_ids: vec![
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_REPLAY_READBACK_RERUN_PREVIEW_GATE,
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
) -> WorkGraphEventsPersistenceGuardGuardPreview {
    WorkGraphEventsPersistenceGuardGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_event_store_enablement: true,
        satisfied_by_preview: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_persistence_guard_stage_ids: Vec<&'static str>,
    affected_persistence_guard_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsPersistenceGuardBlockerPreview {
    WorkGraphEventsPersistenceGuardBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_persistence_guard_stage_ids,
        affected_persistence_guard_plan_ids,
        required_before_event_store_enablement: true,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects {
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
    fn persistence_guard_plans_preserve_no_persistence_boundary() {
        let plans = work_graph_append_only_work_graph_events_persistence_guard_plans_from(
            &sample_source_decisions(),
        );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.persistence_guard_contract_ready_preview
                && plan.event_store_enablement_contract_ready_preview
                && plan.replay_readback_prerequisite_ready_preview
                && plan.adapter_enforcement_guard_ready_preview
                && plan.no_persistence_guard_ready_preview
                && !plan.persists_work_graph_events
                && !plan.enables_event_store
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.enforces_adapter_projection
        }));
    }

    #[test]
    fn persistence_guard_stages_cover_expected_contracts() {
        let plans = work_graph_append_only_work_graph_events_persistence_guard_plans_from(
            &sample_source_decisions(),
        );
        let stages =
            work_graph_append_only_work_graph_events_persistence_guard_stage_plans_from(&plans);

        assert_eq!(stages.len(), 6);
        assert_eq!(
            stages
                .iter()
                .map(|stage| stage.required_contract_ref_ids.len())
                .sum::<usize>(),
            31
        );
        assert!(stages.iter().all(|stage| {
            stage.contract_ready_preview
                && !stage.persists_work_graph_events_after_preview
                && !stage.enables_event_store_after_preview
                && !stage.executes_replay_after_preview
                && !stage.executes_readback_after_preview
        }));
    }

    #[test]
    fn persistence_guard_blockers_track_primary_residuals() {
        let plans = work_graph_append_only_work_graph_events_persistence_guard_plans_from(
            &sample_source_decisions(),
        );
        let blockers =
            work_graph_append_only_work_graph_events_persistence_guard_blockers_from(&plans);
        let blocker_ids = blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "append_only_work_graph_events_persistence_guard_readback_missing"
            ]
        );
        assert_eq!(
            blockers
                .iter()
                .find(|blocker| blocker.id == "canonical_adapter_projection_partial_or_gap")
                .map(|blocker| blocker.affected_source_surface_ids.len()),
            Some(1)
        );
    }

    #[test]
    fn persistence_guard_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewSideEffects {
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
                side_effect_lock_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_source_decisions() -> Vec<WorkGraphEventsReplayReadbackRerunSourceDecisionPreview> {
        vec![
            sample_source_decision("update_plan_tool", "planning", true),
            sample_source_decision("multi_agent_v2_thread_spawn", "multi_agent", false),
        ]
    }

    fn sample_source_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
        partial_gap: bool,
    ) -> WorkGraphEventsReplayReadbackRerunSourceDecisionPreview {
        let mut residual_source_blocker_ids = vec![
            "append_only_work_graph_events_disabled",
            "replay_readback_execution_disabled",
            "runtime_canonical_adapter_enforcement_disabled",
        ];
        if partial_gap {
            residual_source_blocker_ids.push("canonical_adapter_projection_partial_or_gap");
        }

        WorkGraphEventsReplayReadbackRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_replay_readback_rerun_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            covered_by_replay_readback_application_preview: true,
            replay_readback_contract_ready: true,
            replay_readback_application_applied: false,
            append_only_work_graph_events_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            residual_source_blocker_ids,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE,
        }
    }
}
