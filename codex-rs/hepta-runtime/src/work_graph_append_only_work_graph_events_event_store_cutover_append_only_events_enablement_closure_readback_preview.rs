use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePreviewReport;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStagePreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview_report;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview::work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
        "hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_event_store_cutover_append_only_events_enablement_closure_preview_gate: &'static str,
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
    pub readback_plans: Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStageAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureEvidenceFieldAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_event_store_cutover_append_only_events_enablement_closure_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_append_only_events_enablement_closure: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub event_store_cutover_append_only_events_enablement_closure_plan_id: String,
    pub expected_stage_count: usize,
    pub expected_evidence_field_count: usize,
    pub expected_residual_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub replay_execution_enabled: bool,
    pub event_store_cutover_append_only_events_enablement_closure_enabled: bool,
    pub persists_work_graph_events: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStageAssertionPreview {
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub event_store_enabled_after_readback: bool,
    pub execution_enabled_after_readback: bool,
    pub persistence_enabled_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureEvidenceFieldAssertionPreview
{
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardAssertionPreview {
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_cutover_append_only_events_enablement_closure: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerMappingAssertionPreview
{
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_event_store_cutover_append_only_events_enablement_closure_stage_ids:
        Vec<&'static str>,
    pub blocks_event_store_cutover_append_only_events_enablement_closure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureDriftDetectorPreview {
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects
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
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewReport{
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview_report(
        );
    let readback_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_plans_from(
            &preview_report.event_store_cutover_append_only_events_enablement_closure_plans,
        );
    let stage_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_stage_assertions_from(
            &preview_report.event_store_cutover_append_only_events_enablement_closure_stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_evidence_field_assertions_from(
            &preview_report.event_store_cutover_append_only_events_enablement_closure_plans,
        );
    let guard_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_guard_assertions_from(
            &preview_report.guards,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_blocker_mapping_assertions_from(
            &preview_report.blockers,
        );
    let drift_detectors =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_drift_detectors();
    let blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_blockers_from(
            &preview_report,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_preview_no_execution",
        upstream_event_store_cutover_append_only_events_enablement_closure_preview_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_PREVIEW_GATE,
        source_surface_count: preview_report.source_surface_count,
        preview_plan_count: preview_report.event_store_cutover_append_only_events_enablement_closure_plan_count,
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
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_event_store_cutover_append_only_events_enablement_closure_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_append_only_events_enablement_closure: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_plans()
-> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPlanPreview> {
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_preview_report(
        );
    work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_plans_from(
        &preview_report.event_store_cutover_append_only_events_enablement_closure_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_drift_detectors()
-> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureDriftDetectorPreview> {
    vec![
        drift_detector(
            "event_store_cutover_append_only_events_enablement_closure_contract_drift",
            vec!["event_store_cutover_append_only_events_enablement_closure_packet_id"],
        ),
        drift_detector(
            "append_only_persistence_disabled_guard_drift",
            vec!["append_only_persistence_disabled_guard_id"],
        ),
        drift_detector(
            "event_store_enablement_disabled_guard_drift",
            vec!["event_store_enablement_disabled_guard_id"],
        ),
        drift_detector(
            "wal_checkpoint_no_write_guard_drift",
            vec!["wal_checkpoint_no_write_guard_id"],
        ),
        drift_detector(
            "event_store_cutover_no_enablement_proof_drift",
            vec!["event_store_cutover_no_enablement_proof_id"],
        ),
        drift_detector(
            "residual_blocker_mapping_drift",
            vec!["residual_source_blocker_ids"],
        ),
        drift_detector("next_required_gate_drift", vec!["next_required_gate"]),
    ]
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_required_prior_gates(
        );
    gates.push(
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_plans_from(
    plans: &[WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPlanPreview> {
    plans
        .iter()
        .map(|plan| WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPlanPreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            event_store_cutover_append_only_events_enablement_closure_plan_id: plan.event_store_cutover_append_only_events_enablement_closure_plan_id.clone(),
            expected_stage_count: plan.required_event_store_cutover_append_only_events_enablement_closure_stage_ids.len(),
            expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
            expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            event_store_cutover_append_only_events_enablement_closure_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_stage_assertions_from(
    stages: &[WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStagePreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStageAssertionPreview> {
    stages
        .iter()
        .map(|stage| {
            WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStageAssertionPreview {
                stage_id: stage.id,
                affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
                contract_ready_preview: stage.contract_ready_preview,
                event_store_enabled_after_readback: false,
                execution_enabled_after_readback: false,
                persistence_enabled_after_readback: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_evidence_field_assertions_from(
    plans: &[WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview],
) -> Vec<
    WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureEvidenceFieldAssertionPreview,
> {
    plans
        .iter()
        .map(
            |plan| WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureEvidenceFieldAssertionPreview {
                source_surface_id: plan.source_surface_id,
                evidence_field_ids: plan.expected_evidence_field_ids.clone(),
                evidence_contract_ready_preview: true,
                persists_evidence_after_readback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_guard_assertions_from(
    guards: &[WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardAssertionPreview> {
    guards
        .iter()
        .map(|guard| {
            WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureGuardAssertionPreview {
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                required_before_event_store_cutover_append_only_events_enablement_closure: guard
                    .required_before_event_store_cutover_append_only_events_enablement_closure,
                satisfied_by_preview: guard.satisfied_by_preview,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_blocker_mapping_assertions_from(
    blockers: &[WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerPreview],
) -> Vec<
    WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerMappingAssertionPreview,
> {
    blockers
        .iter()
        .map(|blocker| {
            WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_event_store_cutover_append_only_events_enablement_closure_stage_ids: blocker
                    .affected_event_store_cutover_append_only_events_enablement_closure_stage_ids
                    .clone(),
                blocks_event_store_cutover_append_only_events_enablement_closure: true,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_blockers_from(
    preview_report: &WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePreviewReport,
) -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackBlockerPreview> {
    let all_sources = preview_report
        .event_store_cutover_append_only_events_enablement_closure_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    vec![
        readback_blocker(
            "append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_not_executed",
            "high",
            all_sources.clone(),
            "keep event-store cutover append-only enablement closure readback as a preview until enablement is explicitly approved",
        ),
        readback_blocker(
            "append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_application_missing",
            "high",
            all_sources.clone(),
            "apply readback-verified event-store cutover append-only enablement closure contracts into no-enable outcomes",
        ),
        readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            all_sources,
            "keep WorkGraph event persistence disabled until the append-only enablement closure readiness rerun verifies no live enablement",
        ),
    ]
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureDriftDetectorPreview {
    WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureDriftDetectorPreview {
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
) -> WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackBlockerPreview {
    WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects {
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
    fn event_store_cutover_append_only_events_enablement_closure_readback_plans_preserve_no_execution()
     {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_readback_plans_from(
                &sample_cutover_append_only_events_enablement_closure_plans(),
            );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_status == "readback_plan_ready"
                && plan.expected_stage_count == 6
                && plan.expected_evidence_field_count == 10
                && !plan.readback_execution_enabled
                && !plan.replay_execution_enabled
                && !plan.event_store_cutover_append_only_events_enablement_closure_enabled
                && !plan.persists_work_graph_events
        }));
    }

    #[test]
    fn event_store_cutover_append_only_events_enablement_closure_readback_assertions_do_not_enable_persistence()
     {
        let stage_assertions =
            work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_stage_assertions_from(
                &sample_stages(),
            );
        let evidence_assertions =
            work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_evidence_field_assertions_from(
                &sample_cutover_append_only_events_enablement_closure_plans(),
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
    fn event_store_cutover_append_only_events_enablement_closure_readback_drift_detectors_cover_core_contracts()
     {
        let detectors =
            work_graph_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure_drift_detectors(
            );

        assert_eq!(detectors.len(), 7);
        assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
        assert!(detectors.iter().any(|detector| detector.id
            == "event_store_cutover_append_only_events_enablement_closure_contract_drift"));
    }

    #[test]
    fn event_store_cutover_append_only_events_enablement_closure_readback_side_effects_remain_disabled()
     {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureReadbackPreviewSideEffects {
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

    fn sample_cutover_append_only_events_enablement_closure_plans()
    -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview> {
        vec![
            sample_cutover_append_only_events_enablement_closure_plan(
                "update_plan_tool",
                "planning",
            ),
            sample_cutover_append_only_events_enablement_closure_plan(
                "multi_agent_v2_thread_spawn",
                "multi_agent",
            ),
        ]
    }

    fn sample_cutover_append_only_events_enablement_closure_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview {
        WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosurePlanPreview {
            source_surface_id,
            source_category,
            event_store_cutover_append_only_events_enablement_closure_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_event_store_cutover_append_only_events_enablement_closure"
            ),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            event_store_cutover_append_only_events_enablement_closure_state: "work_graph_events_event_store_cutover_append_only_events_enablement_closure_packet_ready_preview",
            required_event_store_cutover_append_only_events_enablement_closure_stage_ids: vec![
                "work_graph_events_append_only_events_enablement_closure_packet",
                "work_graph_events_append_only_persistence_disabled_guard",
                "work_graph_events_event_store_enablement_disabled_guard",
                "work_graph_events_wal_checkpoint_no_write_guard",
                "work_graph_events_event_store_cutover_no_enablement_proof",
                "work_graph_events_append_only_events_enablement_closure_blocker_mapping",
            ],
            expected_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "event_store_cutover_replay_readback_execution_closure_rerun_decision_ref",
                "append_only_events_enablement_closure_packet_id",
                "append_only_persistence_disabled_guard_id",
                "event_store_enablement_disabled_guard_id",
                "wal_checkpoint_no_write_guard_id",
                "event_store_cutover_no_enablement_proof_id",
                "residual_source_blocker_ids",
                "next_required_gate",
            ],
            residual_source_blocker_ids: vec!["append_only_work_graph_events_disabled"],
            event_store_cutover_append_only_events_enablement_closure_contract_ready_preview: true,
            append_only_persistence_disabled_guard_ready_preview: true,
            event_store_enablement_disabled_guard_ready_preview: true,
            wal_checkpoint_no_write_guard_ready_preview: true,
            no_enablement_proof_ready_preview: true,
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

    fn sample_stages()
    -> Vec<WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStagePreview> {
        vec![WorkGraphEventsEventStoreCutoverAppendOnlyEventsEnablementClosureStagePreview {
            id: "work_graph_events_append_only_events_enablement_closure_packet",
            priority: "critical",
            category: "event_store_cutover_append_only_events_enablement_closure",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec![
                "append_only_event_store_append_only_events_enablement_closure_contract_ready",
            ],
            expected_runtime_state: "preview_only_no_event_store_cutover_append_only_events_enablement_closure",
            prerequisite_gate_ids: vec![
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_APPEND_ONLY_EVENTS_ENABLEMENT_CLOSURE_PREVIEW_GATE,
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
