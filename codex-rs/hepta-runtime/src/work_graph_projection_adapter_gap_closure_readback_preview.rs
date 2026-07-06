use serde::Serialize;

use crate::work_graph_projection_adapter_gap_closure_preview::WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_projection_adapter_gap_closure_preview::WorkGraphProjectionAdapterClosureActionPreview;
use crate::work_graph_projection_adapter_gap_closure_preview::work_graph_projection_adapter_closure_actions;
use crate::work_graph_projection_adapter_gap_closure_preview::work_graph_projection_adapter_gap_closure_required_prior_gates;
use crate::work_graph_projection_adapter_gap_closure_preview::work_graph_projection_adapter_source_gaps;

pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_projection_adapter_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_gap_count: usize,
    pub closure_action_count: usize,
    pub readback_plan_count: usize,
    pub fixture_readback_assertion_count: usize,
    pub store_readback_assertion_count: usize,
    pub timeline_readback_assertion_count: usize,
    pub projected_collection_reference_count: usize,
    pub timeline_event_type_reference_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphProjectionAdapterClosureReadbackPlanPreview>,
    pub fixture_readback_assertions: Vec<WorkGraphProjectionAdapterFixtureReadbackAssertionPreview>,
    pub store_readback_assertions: Vec<WorkGraphProjectionAdapterStoreReadbackAssertionPreview>,
    pub timeline_readback_assertions:
        Vec<WorkGraphProjectionAdapterTimelineReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphProjectionAdapterClosureReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphProjectionAdapterClosureReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_projection_adapter_gap_closure_application_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphProjectionAdapterGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureReadbackPlanPreview {
    pub closure_action_id: &'static str,
    pub source_surface_id: &'static str,
    pub adapter_kind: &'static str,
    pub readback_scope: &'static str,
    pub expected_projected_collection_ids: Vec<&'static str>,
    pub expected_timeline_event_type_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_before_closure_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterFixtureReadbackAssertionPreview {
    pub closure_action_id: &'static str,
    pub source_surface_id: &'static str,
    pub required_fixture_fields: Vec<&'static str>,
    pub expected_fixture_state: &'static str,
    pub redaction_policy: &'static str,
    pub performs_readback: bool,
    pub mutates_fixture: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterStoreReadbackAssertionPreview {
    pub closure_action_id: &'static str,
    pub source_surface_id: &'static str,
    pub projected_collection_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub idempotency_guard_required: bool,
    pub expected_projection_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterTimelineReadbackAssertionPreview {
    pub closure_action_id: &'static str,
    pub source_surface_id: &'static str,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub redaction_policy: &'static str,
    pub expected_event_state: &'static str,
    pub performs_readback: bool,
    pub persists_timeline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_closure_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterClosureReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_closure_action_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionAdapterGapClosureReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub closure_applied_to_runtime: bool,
    pub append_only_store_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub readback_performed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_projection_adapter_gap_closure_readback_preview_report()
-> WorkGraphProjectionAdapterGapClosureReadbackPreviewReport {
    let source_gaps = work_graph_projection_adapter_source_gaps();
    let closure_actions = work_graph_projection_adapter_closure_actions();
    let readback_plans = work_graph_projection_adapter_gap_closure_readback_plans();
    let fixture_readback_assertions =
        work_graph_projection_adapter_gap_closure_fixture_readback_assertions();
    let store_readback_assertions =
        work_graph_projection_adapter_gap_closure_store_readback_assertions();
    let timeline_readback_assertions =
        work_graph_projection_adapter_gap_closure_timeline_readback_assertions();
    let drift_detectors = work_graph_projection_adapter_gap_closure_readback_drift_detectors();
    let blockers = work_graph_projection_adapter_gap_closure_readback_blockers();
    let required_prior_gates =
        work_graph_projection_adapter_gap_closure_readback_required_prior_gates();

    WorkGraphProjectionAdapterGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_projection_adapter_gap_closure_readback_preview_no_execution",
        source_gap_count: source_gaps.len(),
        closure_action_count: closure_actions.len(),
        readback_plan_count: readback_plans.len(),
        fixture_readback_assertion_count: fixture_readback_assertions.len(),
        store_readback_assertion_count: store_readback_assertions.len(),
        timeline_readback_assertion_count: timeline_readback_assertions.len(),
        projected_collection_reference_count: store_readback_assertions
            .iter()
            .map(|assertion| assertion.projected_collection_ids.len())
            .sum(),
        timeline_event_type_reference_count: timeline_readback_assertions
            .iter()
            .map(|assertion| assertion.timeline_event_type_ids.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        fixture_readback_assertions,
        store_readback_assertions,
        timeline_readback_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_projection_adapter_gap_closure_application_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphProjectionAdapterGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_projection_adapter_gap_closure_readback_plans()
-> Vec<WorkGraphProjectionAdapterClosureReadbackPlanPreview> {
    work_graph_projection_adapter_closure_actions()
        .into_iter()
        .map(readback_plan)
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_fixture_readback_assertions()
-> Vec<WorkGraphProjectionAdapterFixtureReadbackAssertionPreview> {
    work_graph_projection_adapter_closure_actions()
        .into_iter()
        .filter(|action| action.adapter_kind == "adapter_projection_fixture")
        .map(
            |action| WorkGraphProjectionAdapterFixtureReadbackAssertionPreview {
                closure_action_id: action.id,
                source_surface_id: action.source_surface_id,
                required_fixture_fields: action.required_evidence_fields,
                expected_fixture_state: "fixture_shape_defined_runtime_not_attached",
                redaction_policy: redaction_policy_for_source(action.source_surface_id),
                performs_readback: false,
                mutates_fixture: false,
            },
        )
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_store_readback_assertions()
-> Vec<WorkGraphProjectionAdapterStoreReadbackAssertionPreview> {
    work_graph_projection_adapter_closure_actions()
        .into_iter()
        .filter(|action| action.adapter_kind == "unified_store_projection")
        .map(
            |action| WorkGraphProjectionAdapterStoreReadbackAssertionPreview {
                closure_action_id: action.id,
                source_surface_id: action.source_surface_id,
                projected_collection_ids: action.projected_collection_ids,
                required_evidence_fields: action.required_evidence_fields,
                idempotency_guard_required: true,
                expected_projection_state: "store_projection_shape_defined_store_write_disabled",
                performs_readback: false,
                mutates_store: false,
            },
        )
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_timeline_readback_assertions()
-> Vec<WorkGraphProjectionAdapterTimelineReadbackAssertionPreview> {
    work_graph_projection_adapter_closure_actions()
        .into_iter()
        .filter(|action| action.adapter_kind == "observability_timeline_projection")
        .map(
            |action| WorkGraphProjectionAdapterTimelineReadbackAssertionPreview {
                closure_action_id: action.id,
                source_surface_id: action.source_surface_id,
                timeline_event_type_ids: action.timeline_event_type_ids,
                required_evidence_fields: action.required_evidence_fields,
                redaction_policy: redaction_policy_for_source(action.source_surface_id),
                expected_event_state: "timeline_event_shape_defined_timeline_persistence_disabled",
                performs_readback: false,
                persists_timeline: false,
            },
        )
        .collect()
}

pub fn work_graph_projection_adapter_gap_closure_readback_drift_detectors()
-> Vec<WorkGraphProjectionAdapterClosureReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "detect_closure_action_source_drift",
            vec!["closure_action_id", "source_surface_id", "adapter_kind"],
            "critical",
        ),
        drift_detector(
            "detect_store_projection_collection_drift",
            vec!["projected_collection_ids", "required_evidence_fields"],
            "critical",
        ),
        drift_detector(
            "detect_timeline_event_type_drift",
            vec!["timeline_event_type_ids", "eventKind", "evidenceRefs"],
            "critical",
        ),
        drift_detector(
            "detect_fixture_redaction_drift",
            vec!["fixtureHash", "redactionState", "sourceSurfaceId"],
            "high",
        ),
        drift_detector(
            "detect_closure_plan_coverage_drift",
            vec![
                "closure_plan_id",
                "closure_action_ids",
                "required_prior_gates",
            ],
            "high",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_readback_blockers()
-> Vec<WorkGraphProjectionAdapterClosureReadbackBlockerPreview> {
    let actions = work_graph_projection_adapter_closure_actions();
    vec![
        blocker(
            "gap_closure_readback_is_preview_only",
            "medium",
            action_ids(&actions, |_| true),
            "keep closure readback as contract-only until adapter fixtures and store/timeline mappings are materialized",
        ),
        blocker(
            "adapter_fixture_readback_not_executed",
            "high",
            action_ids(&actions, |action| {
                action.adapter_kind == "adapter_projection_fixture"
            }),
            "run fixture readback against deterministic source fixtures before closure application",
        ),
        blocker(
            "unified_store_projection_readback_not_executed",
            "high",
            action_ids(&actions, |action| {
                action.adapter_kind == "unified_store_projection"
            }),
            "verify projected collection hashes before enabling append-only store projection",
        ),
        blocker(
            "timeline_projection_readback_not_executed",
            "high",
            action_ids(&actions, |action| {
                action.adapter_kind == "observability_timeline_projection"
            }),
            "verify redacted timeline event shapes before enabling timeline persistence",
        ),
        blocker(
            "closure_application_preview_not_run",
            "high",
            action_ids(&actions, |_| true),
            "run the closure application preview before rerunning enforcement readiness",
        ),
        blocker(
            "post_readback_enforcement_readiness_not_rerun",
            "medium",
            action_ids(&actions, |_| true),
            "rerun unified projection enforcement-readiness after readback and application previews agree",
        ),
    ]
}

pub fn work_graph_projection_adapter_gap_closure_readback_required_prior_gates() -> Vec<&'static str>
{
    let mut gates = work_graph_projection_adapter_gap_closure_required_prior_gates();
    gates.push(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_PREVIEW_GATE);
    gates
}

impl WorkGraphProjectionAdapterGapClosureReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            adapter_projection_enforced: false,
            closure_applied_to_runtime: false,
            append_only_store_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            timeline_persisted: false,
            readback_performed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_plan(
    action: WorkGraphProjectionAdapterClosureActionPreview,
) -> WorkGraphProjectionAdapterClosureReadbackPlanPreview {
    WorkGraphProjectionAdapterClosureReadbackPlanPreview {
        closure_action_id: action.id,
        source_surface_id: action.source_surface_id,
        adapter_kind: action.adapter_kind,
        readback_scope: readback_scope_for_adapter_kind(action.adapter_kind),
        expected_projected_collection_ids: action.projected_collection_ids,
        expected_timeline_event_type_ids: action.timeline_event_type_ids,
        required_evidence_fields: action.required_evidence_fields,
        required_before_closure_application: true,
        readback_state: "preview_readback_defined_not_executed",
        performs_readback: false,
        mutates_runtime: false,
    }
}

fn readback_scope_for_adapter_kind(adapter_kind: &str) -> &'static str {
    match adapter_kind {
        "adapter_projection_fixture" => "fixture_catalog_shape",
        "unified_store_projection" => "unified_store_collection_shape",
        "observability_timeline_projection" => "timeline_event_shape",
        "task_result_projection" => "task_result_terminal_shape",
        _ => "adapter_projection_shape",
    }
}

fn redaction_policy_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "update_plan_tool" | "plan_mode_proposed_plan_blocks" => {
            "hash proposed plan text and expose only trace, step, node, and evidence refs"
        }
        "app_server_turn_plan_notification" => {
            "hash notification payload and expose only trace, turn, and notification refs"
        }
        "multi_agent_v2_mailbox_wait" => {
            "hash mailbox content and expose only trace, agent path, mailbox seq, and delivery refs"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "hash subagent output and expose only task result status, reducer strategy, and evidence refs"
        }
        "hepta_runtime_task_board" => {
            "hash worker commands and artifacts; expose only lane, lease, status, and artifact refs"
        }
        "hepta_runtime_approval_broker" => {
            "hash approval payload and expose only approver role, decision state, and evidence refs"
        }
        _ => "hash raw payloads and expose only redacted evidence refs",
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphProjectionAdapterClosureReadbackDriftDetectorPreview {
    WorkGraphProjectionAdapterClosureReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_closure_application: true,
        performs_readback: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_closure_action_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphProjectionAdapterClosureReadbackBlockerPreview {
    WorkGraphProjectionAdapterClosureReadbackBlockerPreview {
        id,
        severity,
        affected_closure_action_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn action_ids(
    actions: &[WorkGraphProjectionAdapterClosureActionPreview],
    predicate: impl Fn(&WorkGraphProjectionAdapterClosureActionPreview) -> bool,
) -> Vec<&'static str> {
    actions
        .iter()
        .filter(|action| predicate(action))
        .map(|action| action.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_adapter_gap_closure_readback_covers_all_closure_actions() {
        let report = hepta_work_graph_projection_adapter_gap_closure_readback_preview_report();
        let action_ids = report
            .readback_plans
            .iter()
            .map(|plan| plan.closure_action_id)
            .collect::<Vec<_>>();

        assert_eq!(report.status, "ready");
        assert_eq!(report.source_gap_count, 7);
        assert_eq!(report.closure_action_count, 11);
        assert_eq!(report.readback_plan_count, 11);
        assert!(action_ids.contains(&"close_update_plan_timeline_projection"));
        assert!(action_ids.contains(&"close_task_board_unified_store_projection"));
        assert!(action_ids.contains(&"close_approval_broker_timeline_projection"));
        assert!(
            report
                .readback_plans
                .iter()
                .all(|plan| plan.required_before_closure_application)
        );
    }

    #[test]
    fn projection_adapter_gap_closure_readback_splits_fixture_store_and_timeline_assertions() {
        let report = hepta_work_graph_projection_adapter_gap_closure_readback_preview_report();
        let task_board_store = report
            .store_readback_assertions
            .iter()
            .find(|assertion| {
                assertion.closure_action_id == "close_task_board_unified_store_projection"
            })
            .expect("task board store assertion");

        assert_eq!(report.fixture_readback_assertion_count, 0);
        assert_eq!(report.store_readback_assertion_count, 6);
        assert_eq!(report.timeline_readback_assertion_count, 5);
        assert_eq!(report.projected_collection_reference_count, 18);
        assert_eq!(report.timeline_event_type_reference_count, 5);
        assert_eq!(
            task_board_store.projected_collection_ids,
            ["nodes", "taskResults", "artifacts", "timelineEvents"]
        );
        assert!(
            report
                .store_readback_assertions
                .iter()
                .all(|assertion| assertion.idempotency_guard_required)
        );
    }

    #[test]
    fn projection_adapter_gap_closure_readback_declares_drift_detectors_and_blockers() {
        let report = hepta_work_graph_projection_adapter_gap_closure_readback_preview_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(report.drift_detector_count, 5);
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.blocks_closure_application && !detector.performs_readback)
        );
        assert_eq!(
            blocker_ids,
            [
                "gap_closure_readback_is_preview_only",
                "adapter_fixture_readback_not_executed",
                "unified_store_projection_readback_not_executed",
                "timeline_projection_readback_not_executed",
                "closure_application_preview_not_run",
                "post_readback_enforcement_readiness_not_rerun",
            ]
        );
        assert_eq!(report.blocker_count, 6);
    }

    #[test]
    fn projection_adapter_gap_closure_readback_advances_to_application_preview_only() {
        let report = hepta_work_graph_projection_adapter_gap_closure_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 14);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PROJECTION_ADAPTER_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_projection_adapter_gap_closure_application_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn projection_adapter_gap_closure_readback_keeps_side_effects_disabled() {
        let report = hepta_work_graph_projection_adapter_gap_closure_readback_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphProjectionAdapterGapClosureReadbackPreviewSideEffects::none()
        );
        assert!(
            report
                .readback_plans
                .iter()
                .all(|plan| !plan.performs_readback && !plan.mutates_runtime)
        );
        assert!(
            report
                .fixture_readback_assertions
                .iter()
                .all(|assertion| !assertion.performs_readback && !assertion.mutates_fixture)
        );
        assert!(
            report
                .timeline_readback_assertions
                .iter()
                .all(|assertion| !assertion.performs_readback && !assertion.persists_timeline)
        );
    }
}
