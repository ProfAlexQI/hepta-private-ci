#![allow(dead_code)]

use serde::Serialize;

use crate::wg_events_tnc_r15_application_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE;
use crate::wg_events_tnc_r15_application_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview;
use crate::wg_events_tnc_r15_application_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationSourceOutcomePreview;
use crate::wg_events_tnc_r15_application_preview::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_blockers;
use crate::wg_events_tnc_r15_application_preview::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_required_prior_gates;
use crate::wg_events_tnc_r15_application_preview::work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_source_outcomes;
use crate::wg_upe_tnc_r14_rerun_preview::WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview;
use crate::wg_upe_tnc_r14_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions as previous_work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_outcome_count: usize,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_covered_source_count: usize,
    pub previous_ready_surface_count: usize,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_source_count: usize,
    pub previous_append_only_work_graph_events_primary_blocked_surface_count: usize,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing_surface_count_after: usize,
    pub append_only_work_graph_events_primary_blocked_surface_count: usize,
    pub replay_readback_execution_blocked_surface_count: usize,
    pub runtime_adapter_enforcement_blocked_surface_count: usize,
    pub append_only_work_graph_events_enabled_source_count: usize,
    pub event_store_enabled_source_count: usize,
    pub replay_readback_enabled_source_count: usize,
    pub runtime_canonical_adapter_enforcement_enabled_source_count: usize,
    pub rerun_ready_surface_count: usize,
    pub rerun_blocked_surface_count: usize,
    pub decision_delta_count: usize,
    pub cleared_blocker_count: usize,
    pub residual_blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub decision_deltas: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview>,
    pub cleared_blockers: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunClearedBlockerPreview>,
    pub residual_blockers: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunResidualBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt: bool,
    pub ready_for_replay_readback: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview
{
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_enforcement_decision: &'static str,
    pub work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision:
        &'static str,
    pub covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview:
        bool,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready:
        bool,
    pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_applied:
        bool,
    pub append_only_work_graph_events_enabled: bool,
    pub event_store_enabled: bool,
    pub replay_readback_execution_enabled: bool,
    pub runtime_canonical_adapter_enforcement_enabled: bool,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunClearedBlockerPreview
{
    pub id: &'static str,
    pub cleared_source_surface_ids: Vec<&'static str>,
    pub source_count_before: usize,
    pub source_count_after: usize,
    pub closure_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunResidualBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview
{
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub ready_contract_count_before: usize,
    pub ready_contract_count_after: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects
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
    pub terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_established:
        bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewReport{
    let previous_decisions =
        previous_work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_source_outcomes(
        );
    let application_blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_blockers(
        );
    let decision_deltas =
        work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
            &previous_decisions,
            &application_outcomes,
            &application_blockers,
        );
    let cleared_blockers =
        work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_cleared_blockers_from(
            &previous_decisions,
            &decision_deltas,
        );
    let residual_blockers =
        work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_residual_blockers_from(
            &application_blockers,
        );
    let enforcement_stages =
        work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_stages_from(
            &decision_deltas,
            application_outcomes.len(),
        );
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_required_prior_gates();

    let previous_append_only_work_graph_events_primary_blocked_surface_count = previous_decisions
        .iter()
        .filter(|decision| {
            decision
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .count();
    let event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing_surface_count_after = decision_deltas
        .iter()
        .filter(|decision| {
            decision.work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "deny_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing"
        })
        .count();
    let append_only_work_graph_events_primary_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "deny_append_only_work_graph_events_disabled"
        })
        .count();
    let replay_readback_execution_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"replay_readback_execution_disabled")
        })
        .count();
    let runtime_adapter_enforcement_blocked_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"runtime_canonical_adapter_enforcement_disabled")
        })
        .count();
    let rerun_ready_surface_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .count();

    WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_SCHEMA_VERSION,
        preview_mode:
            "read_only_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_no_execution",
        source_surface_count: previous_decisions.len(),
        event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_outcome_count: application_outcomes.len(),
        event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_covered_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview)
            .count(),
        previous_ready_surface_count: previous_decisions
            .iter()
            .filter(|decision| {
                decision.work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                    == "allow_preview_only"
            })
            .count(),
        event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_source_count: decision_deltas
            .iter()
            .filter(|decision| decision.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready)
            .count(),
        previous_append_only_work_graph_events_primary_blocked_surface_count,
        event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing_surface_count_after,
        append_only_work_graph_events_primary_blocked_surface_count,
        replay_readback_execution_blocked_surface_count,
        runtime_adapter_enforcement_blocked_surface_count,
        append_only_work_graph_events_enabled_source_count: 0,
        event_store_enabled_source_count: 0,
        replay_readback_enabled_source_count: 0,
        runtime_canonical_adapter_enforcement_enabled_source_count: 0,
        rerun_ready_surface_count,
        rerun_blocked_surface_count: decision_deltas.len() - rerun_ready_surface_count,
        decision_delta_count: decision_deltas.len(),
        cleared_blocker_count: cleared_blockers.len(),
        residual_blocker_count: residual_blockers.len(),
        enforcement_stage_count: enforcement_stages.len(),
        required_prior_gate_count: required_prior_gates.len(),
        decision_deltas,
        cleared_blockers,
        residual_blockers,
        enforcement_stages,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE,
        ready_for_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt: false,
        ready_for_replay_readback: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview>{
    let previous_decisions =
        previous_work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions();
    let application_outcomes =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_source_outcomes(
        );
    let application_blockers =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_blockers(
        );
    work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
        &previous_decisions,
        &application_outcomes,
        &application_blockers,
    )
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_residual_blockers()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunResidualBlockerPreview>{
    work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_residual_blockers_from(
        &work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_blockers(),
    )
}

pub fn work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_stages()
-> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview>{
    let decisions =
        work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions();
    work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_stages_from(
        &decisions,
        decisions.len(),
    )
}

pub fn work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_required_prior_gates();
    gates.push(
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE,
    );
    gates
}

fn work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
    previous_decisions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview],
    application_outcomes: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationSourceOutcomePreview],
    application_blockers: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview>{
    previous_decisions
        .iter()
        .filter(|previous| {
            previous
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .map(|previous| {
            let covered = application_outcomes.iter().any(|outcome| {
                outcome.source_surface_id == previous.source_surface_id
                    && outcome.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview
                    && !outcome.ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt
            });
            let residual_source_blocker_ids =
                source_blocker_ids(application_blockers, previous.source_surface_id);
            let decision = work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_decision(
                covered,
                &residual_source_blocker_ids,
            );

            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview {
                source_surface_id: previous.source_surface_id,
                source_category: previous.source_category,
                previous_enforcement_decision: previous
                    .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision,
                work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision:
                    decision,
                covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview: covered,
                event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready: covered,
                event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_applied: false,
                append_only_work_graph_events_enabled: false,
                event_store_enabled: false,
                replay_readback_execution_enabled: false,
                runtime_canonical_adapter_enforcement_enabled: false,
                residual_source_blocker_ids,
                next_required_gate: next_gate_for_decision(decision),
            }
        })
        .collect()
}

fn work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_cleared_blockers_from(
    previous_decisions: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview],
    decision_deltas: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunClearedBlockerPreview>{
    let before = previous_decisions
        .iter()
        .filter(|decision| {
            decision
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "allow_preview_only"
        })
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let application_missing_after_count = decision_deltas
        .iter()
        .filter(|decision| {
            decision
                .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                == "deny_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing"
        })
        .count();
    vec![WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunClearedBlockerPreview {
        id: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_required_for_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt",
        cleared_source_surface_ids: before.clone(),
        source_count_before: before.len(),
        source_count_after: application_missing_after_count,
        closure_gate_id:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE,
    }]
}

fn work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_residual_blockers_from(
    application_blockers: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview],
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunResidualBlockerPreview>{
    application_blockers
        .iter()
        .filter(|blocker| !cleared_application_blocker(blocker.id))
        .map(|blocker| {
            WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunResidualBlockerPreview {
                id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                required_before_projection_enforcement: true,
                recommended_fix: blocker.recommended_fix,
            }
        })
        .collect()
}

fn work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_stages_from(
    decision_deltas: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview],
    event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_outcome_count: usize,
) -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview>{
    vec![
        rerun_stage(
            "work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout",
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_outcome_count,
            0,
            decision_deltas
                .iter()
                .filter(|decision| {
                    decision.covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview
                })
                .count(),
            vec![],
        ),
        rerun_stage(
            "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_recording",
            decision_deltas.len(),
            0,
            decision_deltas.len(),
            vec![],
        ),
        rerun_stage(
            "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_send_boundary",
            decision_deltas.len(),
            0,
            decision_deltas.len(),
            vec![],
        ),
        rerun_stage(
            "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_git_mutation_boundary",
            decision_deltas.len(),
            0,
            decision_deltas.len(),
            vec![],
        ),
        rerun_stage(
            "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_no_enablement_regression_guard",
            decision_deltas.len(),
            0,
            decision_deltas.len(),
            vec![],
        ),
        rerun_stage(
            "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_terminal_frontier_mapping",
            decision_deltas.len(),
            0,
            decision_deltas.len(),
            vec![],
        ),
    ]
}

fn source_blocker_ids(
    application_blockers: &[WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview],
    source_surface_id: &'static str,
) -> Vec<&'static str> {
    application_blockers
        .iter()
        .filter(|blocker| {
            !cleared_application_blocker(blocker.id)
                && blocker
                    .affected_source_surface_ids
                    .contains(&source_surface_id)
        })
        .map(|blocker| blocker.id)
        .collect()
}

fn work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_decision(
    contract_ready: bool,
    source_blockers: &[&'static str],
) -> &'static str {
    if !contract_ready {
        "deny_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing"
    } else if source_blockers.contains(&"append_only_work_graph_events_disabled") {
        "deny_append_only_work_graph_events_disabled"
    } else if source_blockers.contains(&"runtime_canonical_adapter_enforcement_disabled") {
        "deny_runtime_canonical_adapter_enforcement_disabled"
    } else {
        "allow_preview_only"
    }
}

fn next_gate_for_decision(decision: &'static str) -> &'static str {
    if decision
        == "deny_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application_missing"
    {
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_APPLICATION_PREVIEW_GATE
    } else {
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE
    }
}

fn cleared_application_blocker(blocker_id: &'static str) -> bool {
    matches!(
        blocker_id,
        "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_missing"
    )
}

fn rerun_stage(
    id: &'static str,
    observed_contract_count: usize,
    ready_contract_count_before: usize,
    ready_contract_count_after: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview{
    WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunStagePreview {
        id,
        observed_contract_count,
        ready_contract_count_before,
        ready_contract_count_after,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE,
    }
}

impl
    WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects
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
            terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_established: false,
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
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_moves_application_missing_to_zero()
     {
        let decisions =
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
                &sample_previous_decisions(),
                &sample_application_outcomes(),
                &sample_application_blockers(),
            );

        assert_eq!(decisions.len(), 2);
        assert!(decisions.iter().all(|decision| {
            decision.covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview
                && decision.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready
                && !decision.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_applied
                && decision.work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision
                    == "allow_preview_only"
                && decision.next_required_gate
                    == WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE
        }));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preserves_no_enforcement_boundary()
     {
        let decisions =
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
                &sample_previous_decisions(),
                &sample_application_outcomes(),
                &sample_application_blockers(),
            );

        assert!(decisions.iter().all(|decision| {
            !decision.append_only_work_graph_events_enabled
                && !decision.event_store_enabled
                && !decision.replay_readback_execution_enabled
                && !decision.runtime_canonical_adapter_enforcement_enabled
        }));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_tracks_residuals_and_stages()
     {
        let blockers =
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_residual_blockers_from(
                &sample_application_blockers(),
            );
        let decisions =
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions_from(
                &sample_previous_decisions(),
                &sample_application_outcomes(),
                &sample_application_blockers(),
            );
        let stages =
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_stages_from(
                &decisions, 2,
            );

        assert_eq!(
            blockers
                .iter()
                .map(|blocker| blocker.id)
                .collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
        assert_eq!(stages.len(), 6);
        assert!(stages.iter().all(|stage| !stage.enforcement_enabled));
    }

    #[test]
    fn event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_side_effects_remain_disabled()
     {
        assert_eq!(
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects::none(),
            WorkGraphUnifiedProjectionEnforcementReadinessWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunPreviewSideEffects {
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
                terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_established: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }

    fn sample_previous_decisions()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview>{
        vec![
            sample_previous_decision("update_plan_tool", "planning"),
            sample_previous_decision("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_previous_decision(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview{
        WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptRerunSourceDecisionPreview {
            source_surface_id,
            source_category,
            previous_enforcement_decision: "deny_append_only_work_graph_events_disabled",
            work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision:
                "allow_preview_only",
            covered_by_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview: true,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready: true,
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_applied: false,
            append_only_work_graph_events_enabled: false,
            event_store_enabled: false,
            replay_readback_execution_enabled: false,
            runtime_canonical_adapter_enforcement_enabled: false,
            residual_source_blocker_ids: Vec::new(),
            next_required_gate:
                WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_RECOMMENDED_NEXT_GATE,
        }
    }

    fn sample_application_outcomes()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationSourceOutcomePreview>
    {
        vec![
            sample_application_outcome("update_plan_tool", "planning"),
            sample_application_outcome("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_application_outcome(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationSourceOutcomePreview
    {
        WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationSourceOutcomePreview {
            source_surface_id,
            source_category,
            application_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application"
            ),
            post_application_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_state: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_ready_preview_after_application",
            event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
            ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview: true,
            ready_for_append_only_work_graph_events: false,
            ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt: false,
        }
    }

    fn sample_application_blockers()
    -> Vec<WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview>{
        vec![WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptApplicationBlockerPreview {
                id: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_missing",
                severity: "medium",
                category: "readiness_rerun",
                affected_source_surface_ids: vec![
                    "update_plan_tool",
                    "multi_agent_v2_thread_spawn",
                ],
                affected_application_plan_ids: vec![
                    "update_plan_tool_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application"
                        .to_string(),
                    "multi_agent_v2_thread_spawn_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_application"
                        .to_string(),
                ],
                required_before_append_only_events: true,
                recommended_fix:
                    "rerun enforcement readiness after terminal no-cutover receipt acknowledgement replay idempotency closeout outcomes are available",
            }]
    }
}
