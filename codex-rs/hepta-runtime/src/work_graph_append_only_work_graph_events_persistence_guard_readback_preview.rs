use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewReport;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WorkGraphEventsPersistenceGuardBlockerPreview;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WorkGraphEventsPersistenceGuardGuardPreview;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WorkGraphEventsPersistenceGuardPlanPreview;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::WorkGraphEventsPersistenceGuardStagePreview;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_report;
use crate::work_graph_append_only_work_graph_events_persistence_guard_preview::work_graph_append_only_work_graph_events_persistence_guard_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_persistence_guard_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_work_graph_events_persistence_guard_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
        "hepta_work_graph_append_only_work_graph_events_persistence_guard_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_persistence_guard_preview_gate: &'static str,
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
    pub readback_plans: Vec<WorkGraphEventsPersistenceGuardReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphEventsPersistenceGuardStageAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphEventsPersistenceGuardEvidenceFieldAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphEventsPersistenceGuardGuardAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphEventsPersistenceGuardBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphEventsPersistenceGuardDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphEventsPersistenceGuardReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_persistence_guard_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_enablement: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub persistence_guard_plan_id: String,
    pub expected_stage_count: usize,
    pub expected_evidence_field_count: usize,
    pub expected_residual_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub replay_execution_enabled: bool,
    pub event_store_enablement_enabled: bool,
    pub persists_work_graph_events: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardStageAssertionPreview {
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub event_store_enabled_after_readback: bool,
    pub execution_enabled_after_readback: bool,
    pub persistence_enabled_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardEvidenceFieldAssertionPreview {
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardGuardAssertionPreview {
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_enablement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardBlockerMappingAssertionPreview {
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_persistence_guard_stage_ids: Vec<&'static str>,
    pub blocks_event_store_enablement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardDriftDetectorPreview {
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsPersistenceGuardReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects {
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
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_persistence_guard_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewReport {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_report();
    let readback_plans =
        work_graph_append_only_work_graph_events_persistence_guard_readback_plans_from(
            &preview_report.persistence_guard_plans,
        );
    let stage_assertions =
        work_graph_append_only_work_graph_events_persistence_guard_stage_assertions_from(
            &preview_report.persistence_guard_stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_work_graph_events_persistence_guard_evidence_field_assertions_from(
            &preview_report.persistence_guard_plans,
        );
    let guard_assertions =
        work_graph_append_only_work_graph_events_persistence_guard_guard_assertions_from(
            &preview_report.guards,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_work_graph_events_persistence_guard_blocker_mapping_assertions_from(
            &preview_report.blockers,
        );
    let drift_detectors =
        work_graph_append_only_work_graph_events_persistence_guard_drift_detectors();
    let blockers =
        work_graph_append_only_work_graph_events_persistence_guard_readback_blockers_from(
            &preview_report,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_persistence_guard_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_persistence_guard_readback_preview_no_execution",
        upstream_persistence_guard_preview_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE,
        source_surface_count: preview_report.source_surface_count,
        preview_plan_count: preview_report.persistence_guard_plan_count,
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
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_persistence_guard_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_enablement: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_readback_plans()
-> Vec<WorkGraphEventsPersistenceGuardReadbackPlanPreview> {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_report();
    work_graph_append_only_work_graph_events_persistence_guard_readback_plans_from(
        &preview_report.persistence_guard_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_drift_detectors()
-> Vec<WorkGraphEventsPersistenceGuardDriftDetectorPreview> {
    vec![
        drift_detector(
            "persistence_guard_contract_drift",
            vec!["persistence_guard_contract_id"],
        ),
        drift_detector(
            "event_store_enablement_contract_drift",
            vec!["event_store_enablement_contract_id"],
        ),
        drift_detector(
            "replay_readback_prerequisite_contract_drift",
            vec!["replay_readback_prerequisite_contract_id"],
        ),
        drift_detector(
            "adapter_enforcement_guard_contract_drift",
            vec!["adapter_enforcement_guard_contract_id"],
        ),
        drift_detector(
            "no_persistence_guard_drift",
            vec!["no_persistence_guard_id", "satisfied_by_preview"],
        ),
        drift_detector(
            "residual_blocker_mapping_drift",
            vec!["residual_source_blocker_ids"],
        ),
        drift_detector("next_required_gate_drift", vec!["next_required_gate"]),
    ]
}

pub fn work_graph_append_only_work_graph_events_persistence_guard_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_persistence_guard_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE);
    gates
}

fn work_graph_append_only_work_graph_events_persistence_guard_readback_plans_from(
    plans: &[WorkGraphEventsPersistenceGuardPlanPreview],
) -> Vec<WorkGraphEventsPersistenceGuardReadbackPlanPreview> {
    plans
        .iter()
        .map(|plan| WorkGraphEventsPersistenceGuardReadbackPlanPreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            persistence_guard_plan_id: plan.persistence_guard_plan_id.clone(),
            expected_stage_count: plan.required_persistence_guard_stage_ids.len(),
            expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
            expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            event_store_enablement_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_READBACK_RECOMMENDED_NEXT_GATE,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_persistence_guard_stage_assertions_from(
    stages: &[WorkGraphEventsPersistenceGuardStagePreview],
) -> Vec<WorkGraphEventsPersistenceGuardStageAssertionPreview> {
    stages
        .iter()
        .map(
            |stage| WorkGraphEventsPersistenceGuardStageAssertionPreview {
                stage_id: stage.id,
                affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
                contract_ready_preview: stage.contract_ready_preview,
                event_store_enabled_after_readback: false,
                execution_enabled_after_readback: false,
                persistence_enabled_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_persistence_guard_evidence_field_assertions_from(
    plans: &[WorkGraphEventsPersistenceGuardPlanPreview],
) -> Vec<WorkGraphEventsPersistenceGuardEvidenceFieldAssertionPreview> {
    plans
        .iter()
        .map(
            |plan| WorkGraphEventsPersistenceGuardEvidenceFieldAssertionPreview {
                source_surface_id: plan.source_surface_id,
                evidence_field_ids: plan.expected_evidence_field_ids.clone(),
                evidence_contract_ready_preview: true,
                persists_evidence_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_persistence_guard_guard_assertions_from(
    guards: &[WorkGraphEventsPersistenceGuardGuardPreview],
) -> Vec<WorkGraphEventsPersistenceGuardGuardAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphEventsPersistenceGuardGuardAssertionPreview {
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                required_before_event_store_enablement: guard
                    .required_before_event_store_enablement,
                satisfied_by_preview: guard.satisfied_by_preview,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_persistence_guard_blocker_mapping_assertions_from(
    blockers: &[WorkGraphEventsPersistenceGuardBlockerPreview],
) -> Vec<WorkGraphEventsPersistenceGuardBlockerMappingAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphEventsPersistenceGuardBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_persistence_guard_stage_ids: blocker
                    .affected_persistence_guard_stage_ids
                    .clone(),
                blocks_event_store_enablement: true,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_persistence_guard_readback_blockers_from(
    preview_report: &WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardPreviewReport,
) -> Vec<WorkGraphEventsPersistenceGuardReadbackBlockerPreview> {
    let all_sources = preview_report
        .persistence_guard_plans
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
            "append_only_work_graph_events_persistence_guard_readback_not_executed",
            "high",
            all_sources.clone(),
            "keep persistence guard readback as a preview until execution is explicitly enabled",
        ),
        readback_blocker(
            "append_only_work_graph_events_persistence_guard_application_missing",
            "high",
            all_sources.clone(),
            "apply readback-verified persistence guard contracts into no-persistence outcomes",
        ),
        readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            all_sources.clone(),
            "keep WorkGraph event persistence disabled until persistence guard application is verified",
        ),
        readback_blocker(
            "replay_readback_execution_disabled",
            "high",
            all_sources.clone(),
            "keep replay/readback execution disabled until append-only event persistence is promoted",
        ),
        readback_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            all_sources,
            "keep runtime adapter enforcement disabled until append-only event persistence is promoted",
        ),
        readback_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            partial_gap_sources,
            "close partial/gap adapter source mappings before authoritative event persistence",
        ),
    ]
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphEventsPersistenceGuardDriftDetectorPreview {
    WorkGraphEventsPersistenceGuardDriftDetectorPreview {
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
) -> WorkGraphEventsPersistenceGuardReadbackBlockerPreview {
    WorkGraphEventsPersistenceGuardReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects {
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
    fn persistence_guard_readback_plans_preserve_no_execution() {
        let plans = work_graph_append_only_work_graph_events_persistence_guard_readback_plans_from(
            &sample_persistence_guard_plans(),
        );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_status == "readback_plan_ready"
                && plan.expected_stage_count == 6
                && plan.expected_evidence_field_count == 10
                && !plan.readback_execution_enabled
                && !plan.replay_execution_enabled
                && !plan.event_store_enablement_enabled
                && !plan.persists_work_graph_events
        }));
    }

    #[test]
    fn persistence_guard_readback_assertions_do_not_enable_persistence() {
        let stage_assertions =
            work_graph_append_only_work_graph_events_persistence_guard_stage_assertions_from(
                &sample_stages(),
            );
        let evidence_assertions =
            work_graph_append_only_work_graph_events_persistence_guard_evidence_field_assertions_from(
                &sample_persistence_guard_plans(),
            );

        assert!(
            stage_assertions
                .iter()
                .all(|assertion| assertion.contract_ready_preview
                    && !assertion.event_store_enabled_after_readback
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
    fn persistence_guard_readback_drift_detectors_cover_core_contracts() {
        let detectors =
            work_graph_append_only_work_graph_events_persistence_guard_drift_detectors();

        assert_eq!(detectors.len(), 7);
        assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
        assert!(
            detectors
                .iter()
                .any(|detector| detector.id == "event_store_enablement_contract_drift")
        );
    }

    #[test]
    fn persistence_guard_readback_side_effects_remain_disabled() {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsPersistenceGuardReadbackPreviewSideEffects {
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
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_persistence_guard_plans() -> Vec<WorkGraphEventsPersistenceGuardPlanPreview> {
        vec![
            sample_persistence_guard_plan("update_plan_tool", "planning"),
            sample_persistence_guard_plan("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_persistence_guard_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsPersistenceGuardPlanPreview {
        WorkGraphEventsPersistenceGuardPlanPreview {
            source_surface_id,
            source_category,
            persistence_guard_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_persistence_guard"
            ),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            persistence_guard_state: "work_graph_events_persistence_guard_contract_ready_preview",
            required_persistence_guard_stage_ids: vec![
                "work_graph_events_persistence_guard_contract",
                "work_graph_events_event_store_enablement_contract",
                "work_graph_events_replay_readback_execution_prerequisite",
                "work_graph_events_adapter_enforcement_guard",
                "work_graph_events_no_persistence_guard",
                "work_graph_events_persistence_guard_blocker_mapping",
            ],
            expected_evidence_field_ids: vec![
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
            ],
            residual_source_blocker_ids: vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
            ],
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
        }
    }

    fn sample_stages() -> Vec<WorkGraphEventsPersistenceGuardStagePreview> {
        vec![WorkGraphEventsPersistenceGuardStagePreview {
            id: "work_graph_events_persistence_guard_contract",
            priority: "critical",
            category: "persistence_guard",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["event_persistence_guard_contract_ready"],
            expected_runtime_state: "preview_only_no_event_persistence",
            prerequisite_gate_ids: vec![
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_PERSISTENCE_GUARD_PREVIEW_GATE,
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
        }]
    }
}
