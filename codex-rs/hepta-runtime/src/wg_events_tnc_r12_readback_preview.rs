use serde::Serialize;

use crate::wg_events_tnc_r12_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_PREVIEW_GATE;
use crate::wg_events_tnc_r12_preview::WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPreviewReport;
use crate::wg_events_tnc_r12_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerPreview;
use crate::wg_events_tnc_r12_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardPreview;
use crate::wg_events_tnc_r12_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview;
use crate::wg_events_tnc_r12_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStagePreview;
use crate::wg_events_tnc_r12_preview::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_report;
use crate::wg_events_tnc_r12_preview::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
        "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_gate: &'static str,
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
    pub readback_plans: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPlanPreview>,
    pub stage_assertions: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStageAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptEvidenceFieldAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPlanPreview
{
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id:
        String,
    pub expected_stage_count: usize,
    pub expected_evidence_field_count: usize,
    pub expected_residual_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub replay_execution_enabled: bool,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_enabled:
        bool,
    pub persists_work_graph_events: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStageAssertionPreview
{
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub event_store_enabled_after_readback: bool,
    pub execution_enabled_after_readback: bool,
    pub persistence_enabled_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptEvidenceFieldAssertionPreview
{
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence_after_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardAssertionPreview
{
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt:
        bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerMappingAssertionPreview
{
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids:
        Vec<&'static str>,
    pub blocks_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt:
        bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptDriftDetectorPreview
{
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects
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

pub fn hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewReport
{
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_report(
        );
    let readback_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans_from(
            &preview_report.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans,
        );
    let stage_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_assertions_from(
            &preview_report.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_plans,
        );
    let evidence_field_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_evidence_field_assertions_from(
            &preview_report.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans,
        );
    let guard_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_guard_assertions_from(
            &preview_report.guards,
        );
    let blocker_mapping_assertions =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_blocker_mapping_assertions_from(
            &preview_report.blockers,
        );
    let drift_detectors =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_drift_detectors();
    let blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_blockers_from(
            &preview_report,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_no_execution",
        upstream_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_PREVIEW_GATE,
        source_surface_count: preview_report.source_surface_count,
        preview_plan_count: preview_report.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_count,
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
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPlanPreview>{
    let preview_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_report(
        );
    work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans_from(
        &preview_report.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_drift_detectors()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptDriftDetectorPreview>
{
    vec![
        drift_detector(
            "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_contract_drift",
            vec![
                "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_id",
            ],
        ),
        drift_detector(
            "non_recording_replay_idempotency_drift",
            vec!["non_recording_replay_idempotency_confirmation_id"],
        ),
        drift_detector(
            "non_send_replay_idempotency_boundary_drift",
            vec!["non_send_replay_idempotency_boundary_id"],
        ),
        drift_detector(
            "git_mutation_replay_idempotency_boundary_drift",
            vec!["git_mutation_replay_idempotency_boundary_id"],
        ),
        drift_detector(
            "no_enablement_regression_guard_drift",
            vec!["no_enablement_regression_replay_idempotency_id"],
        ),
        drift_detector(
            "residual_blocker_mapping_drift",
            vec!["residual_source_blocker_ids"],
        ),
        drift_detector("next_required_gate_drift", vec!["next_required_gate"]),
    ]
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_required_prior_gates(
        );
    gates.push(
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_PREVIEW_GATE,
    );
    gates
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans_from(
    plans: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPlanPreview>
{
    plans
        .iter()
        .map(|plan| WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPlanPreview {
            source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id: plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id.clone(),
            expected_stage_count: plan.required_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids.len(),
            expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
            expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_assertions_from(
    stages: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStagePreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStageAssertionPreview>
{
    stages
        .iter()
        .map(|stage| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStageAssertionPreview {
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

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_evidence_field_assertions_from(
    plans: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptEvidenceFieldAssertionPreview>{
    plans
        .iter()
        .map(|plan| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptEvidenceFieldAssertionPreview {
                source_surface_id: plan.source_surface_id,
                evidence_field_ids: plan.expected_evidence_field_ids.clone(),
                evidence_contract_ready_preview: true,
                persists_evidence_after_readback: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_guard_assertions_from(
    guards: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardAssertionPreview>
{
    guards
        .iter()
        .map(|guard| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardAssertionPreview {
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: guard
                    .required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt,
                satisfied_by_preview: guard.satisfied_by_preview,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_blocker_mapping_assertions_from(
    blockers: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerMappingAssertionPreview>{
    blockers
        .iter()
        .map(|blocker| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids: blocker
                    .affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids
                    .clone(),
                blocks_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: true,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_blockers_from(
    preview_report: &WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPreviewReport,
) -> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackBlockerPreview,
>{
    let all_sources = preview_report
        .event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    vec![
        readback_blocker(
            "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_not_executed",
            "high",
            all_sources.clone(),
            "keep terminal no-cutover receipt acknowledgement replay idempotency closeout readback as a preview until terminal review is explicitly requested",
        ),
        readback_blocker(
            "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing",
            "high",
            all_sources,
            "apply readback-verified terminal no-cutover receipt acknowledgement replay idempotency closeout contracts into terminal no-enable outcomes",
        ),
    ]
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptDriftDetectorPreview{
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptDriftDetectorPreview {
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
) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackBlockerPreview{
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects {
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
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans_preserve_no_execution()
     {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_plans_from(
                &sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plans(),
            );

        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.readback_status == "readback_plan_ready"
                && plan.expected_stage_count == 6
                && plan.expected_evidence_field_count == 10
                && !plan.readback_execution_enabled
                && !plan.replay_execution_enabled
                && !plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_enabled
                && !plan.persists_work_graph_events
        }));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_assertions_do_not_enable_persistence()
     {
        let stage_assertions =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_assertions_from(
                &sample_stages(),
            );
        let evidence_assertions =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_evidence_field_assertions_from(
                &sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plans(),
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
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_drift_detectors_cover_core_contracts()
     {
        let detectors =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_drift_detectors(
            );

        assert_eq!(detectors.len(), 7);
        assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
        assert!(detectors.iter().any(|detector| detector.id
            == "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_contract_drift"));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_side_effects_remain_disabled()
     {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReadbackPreviewSideEffects {
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

    fn sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plans()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview>{
        vec![
            sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plan(
                "update_plan_tool",
                "planning",
            ),
            sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plan(
                "multi_agent_v2_thread_spawn",
                "multi_agent",
            ),
        ]
    }

    fn sample_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview{
        WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview {
            source_surface_id,
            source_category,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt"
            ),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_state: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_ready_preview",
            required_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids: vec![
                "work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout",
                "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_recording",
                "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_send_boundary",
                "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_git_mutation_boundary",
                "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_no_enablement_regression_guard",
                "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_terminal_frontier_mapping",
            ],
            expected_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_decision_ref",
                "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_id",
                "non_recording_replay_idempotency_confirmation_id",
                "non_send_replay_idempotency_boundary_id",
                "git_mutation_replay_idempotency_boundary_id",
                "no_enablement_regression_replay_idempotency_id",
                "residual_source_blocker_ids",
                "next_required_gate",
            ],
            residual_source_blocker_ids: vec![],
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
            non_recording_replay_idempotency_ready_preview: true,
            non_send_replay_idempotency_boundary_ready_preview: true,
            git_mutation_replay_idempotency_boundary_ready_preview: true,
            terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_ready_preview: true,
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
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStagePreview>
    {
        vec![WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStagePreview {
            id: "work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout",
            priority: "critical",
            category: "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec![
                "append_only_event_store_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_contract_ready",
            ],
            expected_runtime_state: "preview_only_no_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt",
            prerequisite_gate_ids: vec![
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_PREVIEW_GATE,
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

pub(crate) mod family {
    use crate::wg_events_tnc_r12_preview::family::RenderedPreview;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct FamilyManifest {
        pub(crate) contract_drift_id: &'static str,
        pub(crate) contract_source_field: &'static str,
        pub(crate) readback_not_executed_id: &'static str,
        pub(crate) application_missing_id: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct ReadbackPlan {
        pub(crate) source_surface_id: &'static str,
        pub(crate) source_category: &'static str,
        pub(crate) plan_id: String,
        pub(crate) expected_stage_count: usize,
        pub(crate) expected_evidence_field_count: usize,
        pub(crate) expected_residual_blocker_count: usize,
        pub(crate) next_required_gate: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct StageAssertion {
        pub(crate) stage_id: &'static str,
        pub(crate) affected_source_surface_ids: Vec<&'static str>,
        pub(crate) required_contract_ref_ids: Vec<&'static str>,
        pub(crate) contract_ready_preview: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct EvidenceFieldAssertion {
        pub(crate) source_surface_id: &'static str,
        pub(crate) evidence_field_ids: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct GuardAssertion {
        pub(crate) guard_id: &'static str,
        pub(crate) severity: &'static str,
        pub(crate) guard_scope: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct BlockerMappingAssertion {
        pub(crate) blocker_id: &'static str,
        pub(crate) affected_source_surface_ids: Vec<&'static str>,
        pub(crate) affected_stage_ids: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct DriftDetector {
        pub(crate) id: &'static str,
        pub(crate) source_fields: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct ReadbackBlocker {
        pub(crate) id: &'static str,
        pub(crate) severity: &'static str,
        pub(crate) affected_source_surface_ids: Vec<&'static str>,
        pub(crate) recommended_fix: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedReadback {
        pub(crate) source_surface_count: usize,
        pub(crate) preview_plan_count: usize,
        pub(crate) readback_plans: Vec<ReadbackPlan>,
        pub(crate) stage_assertions: Vec<StageAssertion>,
        pub(crate) evidence_field_assertions: Vec<EvidenceFieldAssertion>,
        pub(crate) guard_assertions: Vec<GuardAssertion>,
        pub(crate) blocker_mapping_assertions: Vec<BlockerMappingAssertion>,
        pub(crate) drift_detectors: Vec<DriftDetector>,
        pub(crate) blockers: Vec<ReadbackBlocker>,
        pub(crate) required_prior_gates: Vec<&'static str>,
    }

    pub(crate) fn render(
        preview: RenderedPreview,
        preview_gate: &'static str,
        next_gate: &'static str,
        manifest: FamilyManifest,
    ) -> RenderedReadback {
        let source_surface_count = preview.source_surface_count;
        let preview_plan_count = preview.plan_count;
        let all_sources = preview
            .plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();
        let readback_plans = preview
            .plans
            .iter()
            .map(|plan| ReadbackPlan {
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                plan_id: plan.plan_id.clone(),
                expected_stage_count: plan.required_stage_ids.len(),
                expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
                expected_residual_blocker_count: plan.residual_source_blocker_ids.len(),
                next_required_gate: next_gate,
            })
            .collect();
        let stage_assertions = preview
            .stages
            .iter()
            .map(|stage| StageAssertion {
                stage_id: stage.id,
                affected_source_surface_ids: stage.affected_source_surface_ids.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.clone(),
                contract_ready_preview: true,
            })
            .collect();
        let evidence_field_assertions = preview
            .plans
            .iter()
            .map(|plan| EvidenceFieldAssertion {
                source_surface_id: plan.source_surface_id,
                evidence_field_ids: plan.expected_evidence_field_ids.clone(),
            })
            .collect();
        let guard_assertions = preview
            .guards
            .iter()
            .map(|guard| GuardAssertion {
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
            })
            .collect();
        let blocker_mapping_assertions = preview
            .blockers
            .iter()
            .map(|blocker| BlockerMappingAssertion {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                affected_stage_ids: blocker.affected_stage_ids.clone(),
            })
            .collect();
        let blockers = vec![
            ReadbackBlocker {
                id: manifest.readback_not_executed_id,
                severity: "high",
                affected_source_surface_ids: all_sources.clone(),
                recommended_fix: "keep terminal no-cutover receipt acknowledgement replay idempotency closeout readback as a preview until terminal review is explicitly requested",
            },
            ReadbackBlocker {
                id: manifest.application_missing_id,
                severity: "high",
                affected_source_surface_ids: all_sources,
                recommended_fix: "apply readback-verified terminal no-cutover receipt acknowledgement replay idempotency closeout contracts into terminal no-enable outcomes",
            },
        ];
        let mut required_prior_gates = preview.required_prior_gates;
        required_prior_gates.push(preview_gate);

        RenderedReadback {
            source_surface_count,
            preview_plan_count,
            readback_plans,
            stage_assertions,
            evidence_field_assertions,
            guard_assertions,
            blocker_mapping_assertions,
            drift_detectors: drift_detectors(manifest),
            blockers,
            required_prior_gates,
        }
    }

    pub(crate) fn drift_detectors(manifest: FamilyManifest) -> Vec<DriftDetector> {
        vec![
            DriftDetector {
                id: manifest.contract_drift_id,
                source_fields: vec![manifest.contract_source_field],
            },
            DriftDetector {
                id: "non_recording_replay_idempotency_drift",
                source_fields: vec!["non_recording_replay_idempotency_confirmation_id"],
            },
            DriftDetector {
                id: "non_send_replay_idempotency_boundary_drift",
                source_fields: vec!["non_send_replay_idempotency_boundary_id"],
            },
            DriftDetector {
                id: "git_mutation_replay_idempotency_boundary_drift",
                source_fields: vec!["git_mutation_replay_idempotency_boundary_id"],
            },
            DriftDetector {
                id: "no_enablement_regression_guard_drift",
                source_fields: vec!["no_enablement_regression_replay_idempotency_id"],
            },
            DriftDetector {
                id: "residual_blocker_mapping_drift",
                source_fields: vec!["residual_source_blocker_ids"],
            },
            DriftDetector {
                id: "next_required_gate_drift",
                source_fields: vec!["next_required_gate"],
            },
        ]
    }

    #[cfg(test)]
    pub(crate) fn render_sample(
        preview_gate: &'static str,
        next_gate: &'static str,
        manifest: FamilyManifest,
        sample_plan_suffix: &'static str,
    ) -> RenderedReadback {
        use crate::wg_events_tnc_r12_preview::family::RenderedBlocker;
        use crate::wg_events_tnc_r12_preview::family::RenderedGuard;
        use crate::wg_events_tnc_r12_preview::family::RenderedPlan;
        use crate::wg_events_tnc_r12_preview::family::RenderedStage;
        use crate::wg_events_tnc_r12_preview::family::STAGE_IDS;

        let plans = [
            ("update_plan_tool", "planning"),
            ("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
        .into_iter()
        .map(|(source_surface_id, source_category)| RenderedPlan {
            source_surface_id,
            source_category,
            plan_id: format!("{source_surface_id}{sample_plan_suffix}"),
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            state: "sample_ready_preview",
            required_stage_ids: STAGE_IDS.to_vec(),
            expected_evidence_field_ids: vec![
                "source_surface_id",
                "source_category",
                "decision_ref",
                "terminal_frontier_id",
                "non_recording_replay_idempotency_confirmation_id",
                "non_send_replay_idempotency_boundary_id",
                "git_mutation_replay_idempotency_boundary_id",
                "no_enablement_regression_replay_idempotency_id",
                "residual_source_blocker_ids",
                "next_required_gate",
            ],
            residual_source_blocker_ids: vec![],
        })
        .collect::<Vec<_>>();
        let preview = RenderedPreview {
            source_surface_count: plans.len(),
            plan_count: plans.len(),
            stage_count: 1,
            stage_source_ref_count: 1,
            stage_contract_ref_count: 1,
            plan_stage_ref_count: plans.len() * STAGE_IDS.len(),
            plan_evidence_field_ref_count: plans.len() * 10,
            append_only_blocked_source_count: 0,
            replay_readback_blocked_source_count: 0,
            runtime_adapter_blocked_source_count: 0,
            plans,
            stages: vec![RenderedStage {
                id: STAGE_IDS[0],
                priority: "critical",
                category: "sample",
                affected_source_surface_ids: vec!["update_plan_tool"],
                required_contract_ref_ids: vec!["sample_contract_ready"],
                expected_runtime_state: "preview_only",
                prerequisite_gate_ids: vec![preview_gate],
            }],
            guards: Vec::<RenderedGuard>::new(),
            blockers: Vec::<RenderedBlocker>::new(),
            required_prior_gates: vec!["sample_prior_gate"],
        };

        render(preview, preview_gate, next_gate, manifest)
    }
}

macro_rules! define_tnc_readback_compat {
    (
        preview_render = $preview_render:path;
        preview_required_gates = $preview_required_gates:path;
        preview_gate = $preview_gate:path;
        gate_const = $gate_const:ident => $gate_value:literal;
        schema_const = $schema_const:ident => $schema_value:literal;
        next_const = $next_const:ident => $next_value:literal;
        report_type = $report_type:ident;
        plan_type = $plan_type:ident;
        stage_type = $stage_type:ident;
        evidence_type = $evidence_type:ident;
        guard_type = $guard_type:ident;
        blocker_mapping_type = $blocker_mapping_type:ident;
        drift_type = $drift_type:ident;
        blocker_type = $blocker_type:ident;
        effects_type = $effects_type:ident;
        report_upstream_gate = $report_upstream_gate:ident;
        report_ready_application = $report_ready_application:ident;
        report_ready_event_store = $report_ready_event_store:ident;
        plan_id = $plan_id:ident;
        plan_event_store_enabled = $plan_event_store_enabled:ident;
        guard_required = $guard_required:ident;
        blocker_stage_ids = $blocker_stage_ids:ident;
        blocker_blocks_event_store = $blocker_blocks_event_store:ident;
        report_fn = $report_fn:ident;
        plans_fn = $plans_fn:ident;
        drift_fn = $drift_fn:ident;
        required_gates_fn = $required_gates_fn:ident;
        preview_mode = $preview_mode:literal;
        contract_drift_id = $contract_drift_id:literal;
        contract_source_field = $contract_source_field:literal;
        readback_not_executed_id = $readback_not_executed_id:literal;
        application_missing_id = $application_missing_id:literal;
        sample_plan_suffix = $sample_plan_suffix:literal;
        plans_test = $plans_test:ident;
        assertions_test = $assertions_test:ident;
        drift_test = $drift_test:ident;
        effects_test = $effects_test:ident;
        parity_test = $parity_test:ident;
        baseline_sha256 = $baseline_sha256:literal;
    ) => {
        pub const $gate_const: &str = $gate_value;
        pub const $schema_const: &str = $schema_value;
        pub const $next_const: &str = $next_value;

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $report_type {
            pub product: &'static str,
            pub runtime: &'static str,
            pub status: &'static str,
            pub gate: &'static str,
            pub schema_version: &'static str,
            pub preview_mode: &'static str,
            pub $report_upstream_gate: &'static str,
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
            pub readback_plans: Vec<$plan_type>,
            pub stage_assertions: Vec<$stage_type>,
            pub evidence_field_assertions: Vec<$evidence_type>,
            pub guard_assertions: Vec<$guard_type>,
            pub blocker_mapping_assertions: Vec<$blocker_mapping_type>,
            pub drift_detectors: Vec<$drift_type>,
            pub blockers: Vec<$blocker_type>,
            pub required_prior_gates: Vec<&'static str>,
            pub recommended_next_gate: &'static str,
            pub $report_ready_application: bool,
            pub ready_for_append_only_work_graph_events: bool,
            pub $report_ready_event_store: bool,
            pub ready_for_replay_readback_execution: bool,
            pub ready_for_runtime_adapter_enforcement: bool,
            pub ready_for_live_execution: bool,
            pub side_effects: $effects_type,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $plan_type {
            pub source_surface_id: &'static str,
            pub source_category: &'static str,
            pub $plan_id: String,
            pub expected_stage_count: usize,
            pub expected_evidence_field_count: usize,
            pub expected_residual_blocker_count: usize,
            pub readback_status: &'static str,
            pub readback_execution_enabled: bool,
            pub replay_execution_enabled: bool,
            pub $plan_event_store_enabled: bool,
            pub persists_work_graph_events: bool,
            pub next_required_gate: &'static str,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $stage_type {
            pub stage_id: &'static str,
            pub affected_source_surface_ids: Vec<&'static str>,
            pub required_contract_ref_ids: Vec<&'static str>,
            pub contract_ready_preview: bool,
            pub event_store_enabled_after_readback: bool,
            pub execution_enabled_after_readback: bool,
            pub persistence_enabled_after_readback: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $evidence_type {
            pub source_surface_id: &'static str,
            pub evidence_field_ids: Vec<&'static str>,
            pub evidence_contract_ready_preview: bool,
            pub persists_evidence_after_readback: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $guard_type {
            pub guard_id: &'static str,
            pub severity: &'static str,
            pub guard_scope: &'static str,
            pub $guard_required: bool,
            pub satisfied_by_preview: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $blocker_mapping_type {
            pub blocker_id: &'static str,
            pub affected_source_surface_ids: Vec<&'static str>,
            pub $blocker_stage_ids: Vec<&'static str>,
            pub $blocker_blocks_event_store: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $drift_type {
            pub id: &'static str,
            pub source_fields: Vec<&'static str>,
            pub drift_budget: usize,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $blocker_type {
            pub id: &'static str,
            pub severity: &'static str,
            pub affected_source_surface_ids: Vec<&'static str>,
            pub recommended_fix: &'static str,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
        pub struct $effects_type {
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

        fn family_manifest() -> $crate::wg_events_tnc_r12_readback_preview::family::FamilyManifest {
            $crate::wg_events_tnc_r12_readback_preview::family::FamilyManifest {
                contract_drift_id: $contract_drift_id,
                contract_source_field: $contract_source_field,
                readback_not_executed_id: $readback_not_executed_id,
                application_missing_id: $application_missing_id,
            }
        }

        fn render_current() -> $crate::wg_events_tnc_r12_readback_preview::family::RenderedReadback
        {
            $crate::wg_events_tnc_r12_readback_preview::family::render(
                $preview_render(),
                $preview_gate,
                $next_const,
                family_manifest(),
            )
        }

        fn map_plan(
            plan: $crate::wg_events_tnc_r12_readback_preview::family::ReadbackPlan,
        ) -> $plan_type {
            $plan_type {
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                $plan_id: plan.plan_id,
                expected_stage_count: plan.expected_stage_count,
                expected_evidence_field_count: plan.expected_evidence_field_count,
                expected_residual_blocker_count: plan.expected_residual_blocker_count,
                readback_status: "readback_plan_ready",
                readback_execution_enabled: false,
                replay_execution_enabled: false,
                $plan_event_store_enabled: false,
                persists_work_graph_events: false,
                next_required_gate: plan.next_required_gate,
            }
        }

        fn map_stage(
            stage: $crate::wg_events_tnc_r12_readback_preview::family::StageAssertion,
        ) -> $stage_type {
            $stage_type {
                stage_id: stage.stage_id,
                affected_source_surface_ids: stage.affected_source_surface_ids,
                required_contract_ref_ids: stage.required_contract_ref_ids,
                contract_ready_preview: stage.contract_ready_preview,
                event_store_enabled_after_readback: false,
                execution_enabled_after_readback: false,
                persistence_enabled_after_readback: false,
            }
        }

        fn map_evidence(
            evidence: $crate::wg_events_tnc_r12_readback_preview::family::EvidenceFieldAssertion,
        ) -> $evidence_type {
            $evidence_type {
                source_surface_id: evidence.source_surface_id,
                evidence_field_ids: evidence.evidence_field_ids,
                evidence_contract_ready_preview: true,
                persists_evidence_after_readback: false,
            }
        }

        fn map_guard(
            guard: $crate::wg_events_tnc_r12_readback_preview::family::GuardAssertion,
        ) -> $guard_type {
            $guard_type {
                guard_id: guard.guard_id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                $guard_required: true,
                satisfied_by_preview: false,
            }
        }

        fn map_blocker_mapping(
            blocker: $crate::wg_events_tnc_r12_readback_preview::family::BlockerMappingAssertion,
        ) -> $blocker_mapping_type {
            $blocker_mapping_type {
                blocker_id: blocker.blocker_id,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                $blocker_stage_ids: blocker.affected_stage_ids,
                $blocker_blocks_event_store: true,
            }
        }

        fn map_drift(
            detector: $crate::wg_events_tnc_r12_readback_preview::family::DriftDetector,
        ) -> $drift_type {
            $drift_type {
                id: detector.id,
                source_fields: detector.source_fields,
                drift_budget: 0,
            }
        }

        fn map_blocker(
            blocker: $crate::wg_events_tnc_r12_readback_preview::family::ReadbackBlocker,
        ) -> $blocker_type {
            $blocker_type {
                id: blocker.id,
                severity: blocker.severity,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                recommended_fix: blocker.recommended_fix,
            }
        }

        pub fn $report_fn() -> $report_type {
            let rendered = render_current();
            let source_surface_count = rendered.source_surface_count;
            let preview_plan_count = rendered.preview_plan_count;
            let readback_plans = rendered
                .readback_plans
                .into_iter()
                .map(map_plan)
                .collect::<Vec<_>>();
            let stage_assertions = rendered
                .stage_assertions
                .into_iter()
                .map(map_stage)
                .collect::<Vec<_>>();
            let evidence_field_assertions = rendered
                .evidence_field_assertions
                .into_iter()
                .map(map_evidence)
                .collect::<Vec<_>>();
            let guard_assertions = rendered
                .guard_assertions
                .into_iter()
                .map(map_guard)
                .collect::<Vec<_>>();
            let blocker_mapping_assertions = rendered
                .blocker_mapping_assertions
                .into_iter()
                .map(map_blocker_mapping)
                .collect::<Vec<_>>();
            let drift_detectors = rendered
                .drift_detectors
                .into_iter()
                .map(map_drift)
                .collect::<Vec<_>>();
            let blockers = rendered
                .blockers
                .into_iter()
                .map(map_blocker)
                .collect::<Vec<_>>();
            let required_prior_gates = rendered.required_prior_gates;

            $report_type {
                product: "Hepta",
                runtime: "hepta",
                status: "ready",
                gate: $gate_const,
                schema_version: $schema_const,
                preview_mode: $preview_mode,
                $report_upstream_gate: $preview_gate,
                source_surface_count,
                preview_plan_count,
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
                recommended_next_gate: $next_const,
                $report_ready_application: true,
                ready_for_append_only_work_graph_events: false,
                $report_ready_event_store: false,
                ready_for_replay_readback_execution: false,
                ready_for_runtime_adapter_enforcement: false,
                ready_for_live_execution: false,
                side_effects: $effects_type::none(),
            }
        }

        pub fn $plans_fn() -> Vec<$plan_type> {
            render_current()
                .readback_plans
                .into_iter()
                .map(map_plan)
                .collect()
        }

        pub fn $drift_fn() -> Vec<$drift_type> {
            $crate::wg_events_tnc_r12_readback_preview::family::drift_detectors(family_manifest())
                .into_iter()
                .map(map_drift)
                .collect()
        }

        pub fn $required_gates_fn() -> Vec<&'static str> {
            let mut gates = $preview_required_gates();
            gates.push($preview_gate);
            gates
        }

        impl $effects_type {
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
            use sha2::Digest as _;

            fn sample_rendered()
            -> $crate::wg_events_tnc_r12_readback_preview::family::RenderedReadback {
                $crate::wg_events_tnc_r12_readback_preview::family::render_sample(
                    $preview_gate,
                    $next_const,
                    family_manifest(),
                    $sample_plan_suffix,
                )
            }

            #[test]
            fn $plans_test() {
                let plans = sample_rendered()
                    .readback_plans
                    .into_iter()
                    .map(map_plan)
                    .collect::<Vec<_>>();

                assert_eq!(plans.len(), 2);
                assert!(plans.iter().all(|plan| {
                    plan.readback_status == "readback_plan_ready"
                        && plan.expected_stage_count == 6
                        && plan.expected_evidence_field_count == 10
                        && !plan.readback_execution_enabled
                        && !plan.replay_execution_enabled
                        && !plan.$plan_event_store_enabled
                        && !plan.persists_work_graph_events
                }));
            }

            #[test]
            fn $assertions_test() {
                let rendered = sample_rendered();
                let stage_assertions = rendered
                    .stage_assertions
                    .into_iter()
                    .map(map_stage)
                    .collect::<Vec<_>>();
                let evidence_assertions = rendered
                    .evidence_field_assertions
                    .into_iter()
                    .map(map_evidence)
                    .collect::<Vec<_>>();

                assert!(stage_assertions.iter().all(|assertion| {
                    assertion.contract_ready_preview
                        && !assertion.event_store_enabled_after_readback
                        && !assertion.execution_enabled_after_readback
                        && !assertion.persistence_enabled_after_readback
                }));
                assert!(evidence_assertions.iter().all(|assertion| {
                    assertion.evidence_contract_ready_preview
                        && !assertion.persists_evidence_after_readback
                }));
            }

            #[test]
            fn $drift_test() {
                let detectors = $drift_fn();
                assert_eq!(detectors.len(), 7);
                assert!(detectors.iter().all(|detector| detector.drift_budget == 0));
                assert!(
                    detectors
                        .iter()
                        .any(|detector| detector.id == $contract_drift_id)
                );
            }

            #[test]
            fn $effects_test() {
                assert_eq!(
                    $effects_type::none(),
                    $effects_type {
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

            #[test]
            fn $parity_test() {
                let plans = sample_rendered()
                    .readback_plans
                    .into_iter()
                    .map(map_plan)
                    .collect::<Vec<_>>();
                let json =
                    serde_json::to_vec(&plans).expect("serialize WorkGraph readback parity plans");
                let digest = format!("{:x}", sha2::Sha256::digest(json));
                assert_eq!(digest, $baseline_sha256);
            }
        }
    };
}

pub(crate) use define_tnc_readback_compat;
