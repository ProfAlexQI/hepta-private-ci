use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerMappingAssertionPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldAssertionPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardAssertionPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackBlockerPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackPlanPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageAssertionPreview;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview_report;
use crate::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE:
    &str =
        "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview_count: usize,
    pub stage_application_count: usize,
    pub evidence_field_application_count: usize,
    pub guard_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview>,
    pub source_outcomes:
        Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationSourceOutcomePreview>,
    pub stage_applications: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageApplicationPreview>,
    pub evidence_field_applications:
        Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldApplicationPreview>,
    pub guard_applications: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardApplicationPreview>,
    pub blocker_applications:
        Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview
{
    pub application_plan_id: String,
    pub readback_source_surface_id: &'static str,
    pub source_category: &'static str,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_plan_id:
        String,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview:
        bool,
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
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationSourceOutcomePreview
{
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_state:
        &'static str,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview:
        bool,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview:
        bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt:
        bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageApplicationPreview
{
    pub application_id: String,
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events: bool,
    pub enables_event_store: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldApplicationPreview
{
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardApplicationPreview
{
    pub application_id: String,
    pub guard_id: &'static str,
    pub guard_scope: &'static str,
    pub required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt:
        bool,
    pub satisfied_by_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerApplicationPreview
{
    pub application_id: String,
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_stage_ids:
        Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub clears_readback_missing_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationGuardPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_append_only_events: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_append_only_events: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects
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

pub fn hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewReport{
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview_report();
    let application_plans =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
            &readback_report.readback_plans,
        );
    let source_outcomes =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_source_outcomes_from(
            &application_plans,
        );
    let application_guards =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_guards();
    let blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers_from(
            &application_plans,
            &readback_report.blockers,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_preview_no_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview)
            .count(),
        stage_application_count: readback_report.stage_assertions.len(),
        evidence_field_application_count: readback_report.evidence_field_assertions.len(),
        guard_application_count: readback_report.guard_assertions.len(),
        blocker_application_count: readback_report.blocker_mapping_assertions.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        stage_applications:
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_stage_applications_from(
                &readback_report.stage_assertions,
            ),
        evidence_field_applications:
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_evidence_field_applications_from(
                &readback_report.evidence_field_assertions,
            ),
        guard_applications:
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_guard_applications_from(
                &readback_report.guard_assertions,
            ),
        blocker_applications:
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_blocker_applications_from(
                &readback_report.blocker_mapping_assertions,
            ),
        application_guards,
        application_plans,
        source_outcomes,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans()
-> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview,
>{
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview_report();
    work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
        &readback_report.readback_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_source_outcomes()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationSourceOutcomePreview>{
    work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_source_outcomes_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview>{
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_preview_report();
    work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans(),
        &readback_report.blockers,
    )
}

pub fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_required_prior_gates(
        );
    gates
        .push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_READBACK_PREVIEW_GATE);
    gates
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
    readback_plans: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackPlanPreview],
) -> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview,
>{
    readback_plans
        .iter()
        .map(|plan| WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview {
            application_plan_id: format!(
                "{}_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application",
                plan.source_surface_id
            ),
            readback_source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_plan_id: plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_plan_id.clone(),
            application_state:
                "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_ready_preview_after_application",
            readback_verified_by_preview: plan.readback_status == "readback_plan_ready",
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
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

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_source_outcomes_from(
    application_plans: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationSourceOutcomePreview>{
    application_plans
        .iter()
        .map(
            |plan| WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationSourceOutcomePreview {
                source_surface_id: plan.readback_source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_state: plan.application_state,
                event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: plan
                    .event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview,
                ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview: true,
                ready_for_append_only_work_graph_events: false,
                ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_stage_applications_from(
    assertions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageAssertionPreview],
) -> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageApplicationPreview,
>{
    assertions
        .iter()
        .map(|assertion| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptStageApplicationPreview {
                application_id: format!("{}_stage_application", assertion.stage_id),
                stage_id: assertion.stage_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                required_contract_ref_ids: assertion.required_contract_ref_ids.clone(),
                contract_ready_preview: assertion.contract_ready_preview,
                persists_work_graph_events: false,
                enables_event_store: false,
                executes_replay: false,
                executes_readback: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_evidence_field_applications_from(
    assertions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldAssertionPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldApplicationPreview>{
    assertions
        .iter()
        .map(|assertion| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptEvidenceFieldApplicationPreview {
                application_id: format!(
                    "{}_evidence_field_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                evidence_field_ids: assertion.evidence_field_ids.clone(),
                evidence_contract_ready_preview: assertion.evidence_contract_ready_preview,
                persists_evidence: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_guard_applications_from(
    assertions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardAssertionPreview],
) -> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardApplicationPreview,
>{
    assertions
        .iter()
        .map(|assertion| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptGuardApplicationPreview {
                application_id: format!("{}_guard_application", assertion.guard_id),
                guard_id: assertion.guard_id,
                guard_scope: assertion.guard_scope,
                required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt: assertion
                    .required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt,
                satisfied_by_preview: assertion.satisfied_by_preview,
                mutates_runtime: false,
            }
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_blocker_applications_from(
    assertions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerMappingAssertionPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerApplicationPreview>{
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptBlockerApplicationPreview {
                application_id: format!("{}_blocker_application", assertion.blocker_id),
                blocker_id: assertion.blocker_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_stage_ids: assertion
                    .affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_stage_ids
                    .clone(),
                expected_blocker_state:
                    "mapped_for_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview",
                readback_verified_by_preview: true,
                clears_readback_missing_blocker: assertion.blocker_id
                    == "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_readback_missing",
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_guards()
-> Vec<
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationGuardPreview,
>{
    vec![
        application_guard("no_work_graph_event_persistence", "critical", "event_store"),
        application_guard(
            "no_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt",
            "critical",
            "event_store",
        ),
        application_guard("no_wal_write", "critical", "wal"),
        application_guard("no_checkpoint_write", "critical", "checkpoint"),
        application_guard("no_replay_execution", "critical", "replay"),
        application_guard("no_readback_execution", "critical", "readback"),
        application_guard(
            "no_adapter_projection_enforcement",
            "critical",
            "adapter_projection",
        ),
        application_guard("no_git_mutation", "critical", "git"),
        application_guard("no_agent_spawn", "high", "agent_spawn"),
        application_guard(
            "no_external_send_or_model_invocation",
            "high",
            "external_effects",
        ),
        application_guard(
            "no_append_only_events_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_without_rerun",
            "high",
            "readiness_rerun",
        ),
    ]
}

fn work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers_from(
    application_plans: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview],
    _readback_blockers: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackBlockerPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview>{
    let all_sources = application_plans
        .iter()
        .map(|plan| plan.readback_source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = application_plans
        .iter()
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    vec![application_blocker(
        "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_missing",
        "medium",
        "readiness_rerun",
        all_sources,
        all_plan_ids,
        "rerun enforcement readiness after terminal no-cutover receipt acknowledgement replay idempotency closeout outcomes are available",
    )]
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationGuardPreview
{
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_append_only_events: true,
        satisfied_by_preview: true,
    }
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview
{
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_append_only_events: true,
        recommended_fix,
    }
}

impl
    WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects
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
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_maps_readback_verified_sources()
     {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
                &sample_readback_plans(),
            );
        let outcomes =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_source_outcomes_from(
                &plans,
            );

        assert_eq!(plans.len(), 2);
        assert_eq!(outcomes.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.application_state
                == "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_ready_preview_after_application"
                && plan.readback_verified_by_preview
                && plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview
                && !plan.applies_to_runtime
        }));
        assert!(outcomes.iter().all(|outcome| {
            outcome.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview
                && outcome.ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_preview
                && !outcome.ready_for_append_only_work_graph_events
                && !outcome.ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt
        }));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_keeps_runtime_mutation_disabled()
     {
        let plans =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
                &sample_readback_plans(),
            );

        assert!(plans.iter().all(|plan| {
            plan.readback_verified_by_preview
                && plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.persists_work_graph_events
                && !plan.enables_event_store
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.enforces_adapter_projection
        }));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers_keep_event_store_disabled()
     {
        let blockers =
            work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_blockers_from(
                &sample_application_plans(),
                &sample_readback_blockers(),
            );

        assert_eq!(blockers.len(), 1);
        assert_eq!(
            blockers
                .iter()
                .map(|blocker| blocker.id)
                .collect::<Vec<_>>(),
            vec![
                "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_rerun_missing"
            ]
        );
        assert!(
            blockers
                .iter()
                .all(|blocker| blocker.required_before_append_only_events)
        );
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_side_effects_remain_disabled()
     {
        assert_eq!(
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects::none(),
            WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPreviewSideEffects {
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

    fn sample_readback_plans() -> Vec<
        WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackPlanPreview,
    >{
        vec![
            sample_readback_plan("update_plan_tool", "planning"),
            sample_readback_plan("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_readback_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackPlanPreview
    {
        WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackPlanPreview {
            source_surface_id,
            source_category,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt"
            ),
            expected_stage_count: 6,
            expected_evidence_field_count: 10,
            expected_residual_blocker_count: 0,
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE,
        }
    }

    fn sample_application_plans()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptApplicationPlanPreview>{
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_application_plans_from(
            &sample_readback_plans(),
        )
    }

    fn sample_readback_blockers()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReadbackBlockerPreview>{
        Vec::new()
    }
}
