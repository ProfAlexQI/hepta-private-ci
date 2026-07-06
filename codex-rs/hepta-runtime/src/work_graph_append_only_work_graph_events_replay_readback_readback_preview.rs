use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewReport;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WorkGraphEventsReplayReadbackBlockerPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WorkGraphEventsReplayReadbackGuardPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WorkGraphEventsReplayReadbackPlanPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::WorkGraphEventsReplayReadbackStagePreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::hepta_work_graph_append_only_work_graph_events_replay_readback_preview_report;
use crate::work_graph_append_only_work_graph_events_replay_readback_preview::work_graph_append_only_work_graph_events_replay_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_replay_readback_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_replay_readback_preview_gate: &'static str,
    pub source_surface_count: usize,
    pub preview_plan_count: usize,
    pub readback_plan_count: usize,
    pub stage_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphEventsReplayReadbackReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphEventsReplayReadbackStageAssertionPreview>,
    pub evidence_field_assertions: Vec<WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphEventsReplayReadbackGuardAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphEventsReplayReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphEventsReplayReadbackReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_replay_readback_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub replay_readback_plan_id: String,
    pub expected_stage_count: usize,
    pub expected_evidence_field_count: usize,
    pub expected_residual_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub replay_execution_enabled: bool,
    pub rollback_execution_enabled: bool,
    pub persists_work_graph_events: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackStageAssertionPreview {
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub execution_enabled_after_readback: bool,
    pub persistence_enabled_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview {
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackGuardAssertionPreview {
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_replay_readback_execution: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview {
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_replay_readback_stage_ids: Vec<&'static str>,
    pub blocks_replay_readback_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewReport {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_replay_readback_preview_report();
    let readback_plans =
        work_graph_append_only_work_graph_events_replay_readback_readback_plans_from(
            &preview_report.replay_readback_plans,
        );
    let stage_assertions =
        work_graph_append_only_work_graph_events_replay_readback_stage_assertions_from(
            &preview_report.replay_readback_stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_work_graph_events_replay_readback_evidence_field_assertions_from(
            &preview_report.replay_readback_plans,
        );
    let guard_assertions =
        work_graph_append_only_work_graph_events_replay_readback_guard_assertions_from(
            &preview_report.guards,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_work_graph_events_replay_readback_blocker_mapping_assertions_from(
            &preview_report.blockers,
        );
    let drift_detectors =
        work_graph_append_only_work_graph_events_replay_readback_drift_detectors();
    let blockers = work_graph_append_only_work_graph_events_replay_readback_readback_blockers_from(
        &preview_report,
    );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_replay_readback_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_work_graph_events_replay_readback_readback_preview_no_execution",
        upstream_replay_readback_preview_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE,
        source_surface_count: preview_report.source_surface_count,
        preview_plan_count: preview_report.replay_readback_plan_count,
        readback_plan_count: readback_plans.len(),
        stage_assertion_count: stage_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        stage_assertions,
        evidence_field_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_replay_readback_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_replay_readback_readback_plans()
-> Vec<WorkGraphEventsReplayReadbackReadbackPlanPreview> {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_replay_readback_preview_report();
    work_graph_append_only_work_graph_events_replay_readback_readback_plans_from(
        &preview_report.replay_readback_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_replay_readback_drift_detectors()
-> Vec<WorkGraphEventsReplayReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "replay_cursor_contract_drift",
            vec!["replay_cursor_contract_id"],
        ),
        drift_detector(
            "readback_probe_contract_drift",
            vec!["readback_probe_contract_id"],
        ),
        drift_detector(
            "duplicate_suppression_contract_drift",
            vec!["duplicate_suppression_contract_id"],
        ),
        drift_detector(
            "timeline_ordering_contract_drift",
            vec!["timeline_ordering_contract_id"],
        ),
        drift_detector(
            "rollback_anchor_contract_drift",
            vec!["rollback_anchor_contract_id"],
        ),
        drift_detector(
            "event_integrity_digest_contract_drift",
            vec!["event_integrity_digest_contract_id"],
        ),
        drift_detector(
            "no_execution_guard_drift",
            vec!["guard_id", "satisfied_by_preview"],
        ),
    ]
}

pub fn work_graph_append_only_work_graph_events_replay_readback_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_work_graph_events_replay_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE);
    gates
}

fn work_graph_append_only_work_graph_events_replay_readback_readback_plans_from(
    plans: &[WorkGraphEventsReplayReadbackPlanPreview],
) -> Vec<WorkGraphEventsReplayReadbackReadbackPlanPreview> {
    plans
        .iter()
        .map(|plan| WorkGraphEventsReplayReadbackReadbackPlanPreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            replay_readback_plan_id: plan.replay_readback_plan_id.clone(),
            expected_stage_count: plan.required_replay_readback_stage_ids.len(),
            expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
            expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            rollback_execution_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_RECOMMENDED_NEXT_GATE,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_stage_assertions_from(
    stages: &[WorkGraphEventsReplayReadbackStagePreview],
) -> Vec<WorkGraphEventsReplayReadbackStageAssertionPreview> {
    stages
        .iter()
        .map(|stage| WorkGraphEventsReplayReadbackStageAssertionPreview {
            stage_id: stage.id,
            affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
            required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
            contract_ready_preview: stage.contract_ready_preview,
            execution_enabled_after_readback: false,
            persistence_enabled_after_readback: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_evidence_field_assertions_from(
    plans: &[WorkGraphEventsReplayReadbackPlanPreview],
) -> Vec<WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview> {
    plans
        .iter()
        .map(
            |plan| WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview {
                source_surface_id: plan.source_surface_id,
                evidence_field_ids: plan.expected_evidence_field_ids.clone(),
                evidence_contract_ready_preview: true,
                persists_evidence_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_guard_assertions_from(
    guards: &[WorkGraphEventsReplayReadbackGuardPreview],
) -> Vec<WorkGraphEventsReplayReadbackGuardAssertionPreview> {
    guards
        .iter()
        .map(|guard| WorkGraphEventsReplayReadbackGuardAssertionPreview {
            guard_id: guard.id,
            severity: guard.severity,
            guard_scope: guard.guard_scope,
            required_before_replay_readback_execution: guard
                .required_before_replay_readback_execution,
            satisfied_by_preview: guard.satisfied_by_preview,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_blocker_mapping_assertions_from(
    blockers: &[WorkGraphEventsReplayReadbackBlockerPreview],
) -> Vec<WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_replay_readback_stage_ids: blocker
                    .affected_replay_readback_stage_ids
                    .clone(),
                blocks_replay_readback_execution: true,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_readback_blockers_from(
    preview_report: &WorkGraphAppendOnlyWorkGraphEventsReplayReadbackPreviewReport,
) -> Vec<WorkGraphEventsReplayReadbackReadbackBlockerPreview> {
    let all_sources = preview_report
        .replay_readback_plans
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
            "append_only_work_graph_events_replay_readback_readback_not_executed",
            "high",
            all_sources.clone(),
            "keep replay/readback readback as a preview until execution is explicitly enabled",
        ),
        readback_blocker(
            "append_only_work_graph_events_replay_readback_application_missing",
            "high",
            all_sources.clone(),
            "apply readback-verified replay/readback contracts into no-execution outcomes",
        ),
        readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            all_sources.clone(),
            "keep WorkGraph event persistence disabled until replay/readback application is verified",
        ),
        readback_blocker(
            "replay_readback_execution_disabled",
            "high",
            all_sources.clone(),
            "keep replay/readback execution disabled until operator review and side-effect lock are promoted",
        ),
        readback_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            all_sources,
            "keep runtime adapter enforcement disabled until append-only event replay/readback is promoted",
        ),
        readback_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            partial_gap_sources,
            "close partial/gap adapter source mappings before authoritative event replay/readback",
        ),
    ]
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphEventsReplayReadbackDriftDetectorPreview {
    WorkGraphEventsReplayReadbackDriftDetectorPreview {
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
) -> WorkGraphEventsReplayReadbackReadbackBlockerPreview {
    WorkGraphEventsReplayReadbackReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects {
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            rollback_executed: false,
            adapter_projection_enforced: false,
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
    fn replay_readback_readback_plans_preserve_no_execution() {
        let plans = work_graph_append_only_work_graph_events_replay_readback_readback_plans_from(
            &sample_replay_readback_plans(),
        );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_status == "readback_plan_ready"
                && plan.expected_stage_count == 8
                && plan.expected_evidence_field_count == 10
                && !plan.readback_execution_enabled
                && !plan.replay_execution_enabled
                && !plan.rollback_execution_enabled
                && !plan.persists_work_graph_events
        }));
    }

    #[test]
    fn replay_readback_readback_assertions_do_not_enable_execution() {
        let stage_assertions =
            work_graph_append_only_work_graph_events_replay_readback_stage_assertions_from(
                &sample_stages(),
            );
        let evidence_assertions =
            work_graph_append_only_work_graph_events_replay_readback_evidence_field_assertions_from(
                &sample_replay_readback_plans(),
            );

        assert!(
            stage_assertions
                .iter()
                .all(|assertion| assertion.contract_ready_preview
                    && !assertion.execution_enabled_after_readback
                    && !assertion.persistence_enabled_after_readback)
        );
        assert!(
            evidence_assertions
                .iter()
                .all(|assertion| assertion.evidence_contract_ready_preview
                    && !assertion.persists_evidence_after_readback)
        );
    }

    #[test]
    fn replay_readback_readback_drift_detectors_cover_core_contracts() {
        let detectors = work_graph_append_only_work_graph_events_replay_readback_drift_detectors();

        assert_eq!(detectors.len(), 7);
        assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
        assert!(
            detectors
                .iter()
                .any(|detector| detector.id == "event_integrity_digest_contract_drift")
        );
    }

    #[test]
    fn replay_readback_readback_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackReadbackPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                work_graph_events_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                replay_executed: false,
                readback_executed: false,
                rollback_executed: false,
                adapter_projection_enforced: false,
                runtime_mutation_performed: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_replay_readback_plans() -> Vec<WorkGraphEventsReplayReadbackPlanPreview> {
        vec![
            sample_replay_readback_plan("update_plan_tool", "planning"),
            sample_replay_readback_plan("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_replay_readback_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsReplayReadbackPlanPreview {
        WorkGraphEventsReplayReadbackPlanPreview {
            source_surface_id,
            source_category,
            replay_readback_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_replay_readback"
            ),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            replay_readback_state: "work_graph_events_replay_readback_contract_ready_preview",
            required_replay_readback_stage_ids: vec![
                "work_graph_events_replay_cursor_contract",
                "work_graph_events_readback_probe_contract",
                "work_graph_events_duplicate_suppression_contract",
                "work_graph_events_timeline_ordering_contract",
                "work_graph_events_rollback_anchor_contract",
                "work_graph_events_integrity_digest_contract",
                "work_graph_events_no_execution_guard",
                "work_graph_events_replay_readback_blocker_mapping",
            ],
            expected_evidence_field_ids: vec![
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
            ],
            residual_source_blocker_ids: vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
            ],
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
        }
    }

    fn sample_stages() -> Vec<WorkGraphEventsReplayReadbackStagePreview> {
        vec![WorkGraphEventsReplayReadbackStagePreview {
            id: "work_graph_events_replay_cursor_contract",
            priority: "critical",
            category: "replay_cursor",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["shadow_replay_cursor_contract_ready"],
            expected_runtime_state: "preview_only_no_replay_readback_execution",
            prerequisite_gate_ids: vec![
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_PREVIEW_GATE,
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
        }]
    }
}
