use crate::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report;
use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use crate::status_canary_evidence_acceptance_packet::{
    STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID,
    StatusCanaryEvidenceAcceptancePacketSideEffects, status_canary_evidence_acceptance_packet,
};
use crate::status_canary_evidence_packet::{
    PREFLIGHT_ONLY_CONNECTOR_TOOL_ID, SELECTED_STATUS_CANARY_TOOL_ID, StatusCanaryEvidencePacket,
    STATUS_CANARY_EVIDENCE_PACKET_ID, status_canary_evidence_packet,
};
use crate::status_canary_evidence_source_adapter::{
    STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID, StatusCanaryEvidenceSourceAdapterSideEffects,
    status_canary_evidence_source_adapter,
};
use crate::status_canary_evidence_source_reason_packet::{
    STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID,
    StatusCanaryEvidenceSourceReasonPacketSideEffects,
    status_canary_evidence_source_reason_packet_from_adapter,
};
use crate::status_canary_evidence_source_readback::{
    STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID, StatusCanaryEvidenceSourceReadbackSideEffects,
    status_canary_evidence_source_readback_from_fixtures_and_reason_packet,
};
use crate::status_canary_evidence_source_validator::{
    STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID, StatusCanaryEvidenceSourceValidatorSideEffects,
    status_canary_evidence_source_validator_from_observations,
};
use crate::status_canary_start_guard::{
    STATUS_CANARY_START_GUARD_ID, StatusCanaryStartGuardSideEffects, status_canary_start_guard,
};
use crate::status_canary_start_request_gate::{
    STATUS_CANARY_START_REQUEST_GATE_ID, StatusCanaryStartRequestGateSideEffects,
    status_canary_start_request_gate,
};
use crate::status_canary_runner_adapter::{
    STATUS_CANARY_RUNNER_ADAPTER_ID, StatusCanaryRunnerAdapterSideEffects,
    status_canary_runner_adapter_plan,
};
use crate::status_canary_runner_binding_guard::{
    STATUS_CANARY_RUNNER_BINDING_GUARD_ID, StatusCanaryRunnerBindingGuardSideEffects,
    status_canary_runner_binding_guard_plan,
};
use crate::status_canary_runner_dry_run_selector::{
    STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID, StatusCanaryRunnerDryRunSelectorSideEffects,
    status_canary_runner_dry_run_selector_plan,
};
use crate::status_canary_runner_entry_adapter::{
    STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID, StatusCanaryRunnerEntryAdapterSideEffects,
    status_canary_runner_entry_adapter_plan,
};
use crate::status_canary_runner_entry_boundary::{
    STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID, StatusCanaryRunnerEntryBoundarySideEffects,
    status_canary_runner_entry_boundary_plan,
};
use crate::status_canary_runner_start_surface::{
    STATUS_CANARY_RUNNER_START_SURFACE_ID, StatusCanaryRunnerStartSurfaceSideEffects,
    status_canary_runner_start_surface_plan,
};
use serde::Serialize;

use crate::current_reality_capability_registry_count;

pub const CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_GATE: &str =
    "controlled_live_operator_readiness_dashboard_gate";
pub const CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_SCHEMA_VERSION: &str =
    "controlled_live_operator_readiness_dashboard_v1";
pub const CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_RECOMMENDED_NEXT_GATE: &str =
    "close_controlled_live_evidence_before_status_canary_start";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorReadinessDashboardReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_kill_switch_boundary_readback_ready: bool,
    pub source_required_evidence_collection_plan_ready: bool,
    pub status_canary_final_guard_present: bool,
    pub status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub status_canary_candidate_count: usize,
    pub selected_status_canary_count: usize,
    pub preflight_only_non_selected_count: usize,
    pub status_canary_final_gate_ready_count: usize,
    pub status_canary_final_guard_live_blocked_count: usize,
    pub status_canary_final_guard_approval_missing_count: usize,
    pub status_canary_final_guard_live_enabled: bool,
    pub status_canary_final_guard_tool_invocation_enabled: bool,
    pub status_canary_final_guard_ledger_write_enabled: bool,
    pub status_canary_evidence_packet_ready: bool,
    pub status_canary_evidence_packet_id: &'static str,
    pub status_canary_evidence_packet_item_count: usize,
    pub status_canary_evidence_packet_missing_count: usize,
    pub status_canary_evidence_packet_recorded_count: usize,
    pub status_canary_evidence_packet_waived_count: usize,
    pub status_canary_evidence_packet_expired_count: usize,
    pub status_canary_evidence_packet_invalid_count: usize,
    pub status_canary_evidence_packet_decision_reason_audit_count: usize,
    pub status_canary_evidence_packet_decision_reason_audit_ready_count: usize,
    pub status_canary_evidence_packet_decision_reason_audit_rejected_count: usize,
    pub status_canary_evidence_packet_complete: bool,
    pub status_canary_start_blocked_by_evidence_packet: bool,
    pub status_canary_start_allowed_by_evidence_packet: bool,
    pub status_canary_evidence_packet_guard_route: &'static str,
    pub status_canary_evidence_acceptance_packet_ready: bool,
    pub status_canary_evidence_acceptance_packet_id: &'static str,
    pub status_canary_evidence_acceptance_packet_route: &'static str,
    pub status_canary_evidence_acceptance_request_count: usize,
    pub status_canary_evidence_acceptance_known_request_count: usize,
    pub status_canary_evidence_acceptance_unknown_request_count: usize,
    pub status_canary_evidence_acceptance_duplicate_request_count: usize,
    pub status_canary_evidence_acceptance_request_source_validator_bound_count: usize,
    pub status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count:
        usize,
    pub status_canary_evidence_acceptance_request_reason_audit_count: usize,
    pub status_canary_evidence_acceptance_request_reason_audit_ready_count: usize,
    pub status_canary_evidence_acceptance_request_reason_audit_rejected_count: usize,
    pub status_canary_evidence_acceptance_accepted_decision_count: usize,
    pub status_canary_evidence_acceptance_rejected_decision_count: usize,
    pub status_canary_evidence_acceptance_generated_override_count: usize,
    pub status_canary_evidence_acceptance_generated_override_reason_audit_ready_count: usize,
    pub status_canary_evidence_source_adapter_ready: bool,
    pub status_canary_evidence_source_adapter_id: &'static str,
    pub status_canary_evidence_source_adapter_route: &'static str,
    pub status_canary_evidence_source_adapter_count: usize,
    pub status_canary_evidence_source_adapter_input_count: usize,
    pub status_canary_evidence_source_adapter_generated_fixture_count: usize,
    pub status_canary_evidence_source_adapter_missing_input_count: usize,
    pub status_canary_evidence_source_adapter_metadata_contract_count: usize,
    pub status_canary_evidence_source_adapter_metadata_contract_ready_count: usize,
    pub status_canary_evidence_source_adapter_input_contract_field_count: usize,
    pub status_canary_evidence_source_adapter_readback_fixture_contract_field_count: usize,
    pub status_canary_evidence_source_adapter_required_field_validator_count: usize,
    pub status_canary_evidence_source_adapter_required_field_validator_ready_count: usize,
    pub status_canary_evidence_source_adapter_required_field_rejected_count: usize,
    pub status_canary_evidence_source_adapter_missing_required_field_count: usize,
    pub status_canary_evidence_source_reason_packet_ready: bool,
    pub status_canary_evidence_source_reason_packet_id: &'static str,
    pub status_canary_evidence_source_reason_packet_route: &'static str,
    pub status_canary_evidence_source_reason_packet_source_count: usize,
    pub status_canary_evidence_source_decision_reason_count: usize,
    pub status_canary_evidence_source_decision_reason_ready_count: usize,
    pub status_canary_evidence_source_decision_required_field_count: usize,
    pub status_canary_evidence_source_missing_required_field_reason_count: usize,
    pub status_canary_evidence_source_adapter_input_missing_reason_count: usize,
    pub status_canary_evidence_source_adapter_input_other_decision_reason_count: usize,
    pub status_canary_evidence_source_adapter_rejection_reason_count: usize,
    pub status_canary_evidence_source_fixture_generation_allowed_count: usize,
    pub status_canary_evidence_source_fixture_generation_blocked_count: usize,
    pub status_canary_evidence_source_readback_ready: bool,
    pub status_canary_evidence_source_readback_id: &'static str,
    pub status_canary_evidence_source_readback_route: &'static str,
    pub status_canary_evidence_source_readback_fixture_count: usize,
    pub status_canary_evidence_source_readback_observation_count: usize,
    pub status_canary_evidence_source_readback_missing_observation_count: usize,
    pub status_canary_evidence_source_readback_contract_audit_count: usize,
    pub status_canary_evidence_source_readback_contract_audit_ready_count: usize,
    pub status_canary_evidence_source_readback_fixture_contract_audit_ready_count: usize,
    pub status_canary_evidence_source_readback_reason_packet_bound: bool,
    pub status_canary_evidence_source_readback_reason_packet_ready: bool,
    pub status_canary_evidence_source_readback_reason_packet_route: &'static str,
    pub status_canary_evidence_source_readback_fixture_reason_audit_count: usize,
    pub status_canary_evidence_source_readback_fixture_reason_audit_ready_count: usize,
    pub status_canary_evidence_source_readback_fixture_reason_audit_rejected_count: usize,
    pub status_canary_evidence_source_validator_ready: bool,
    pub status_canary_evidence_source_validator_id: &'static str,
    pub status_canary_evidence_source_validator_route: &'static str,
    pub status_canary_evidence_source_validator_contract_audit_count: usize,
    pub status_canary_evidence_source_validator_contract_audit_ready_count: usize,
    pub status_canary_evidence_source_validator_contract_audit_rejected_count: usize,
    pub status_canary_evidence_source_validator_reason_audit_count: usize,
    pub status_canary_evidence_source_validator_reason_audit_ready_count: usize,
    pub status_canary_evidence_source_validator_reason_audit_rejected_count: usize,
    pub status_canary_evidence_source_observation_count: usize,
    pub status_canary_evidence_source_missing_count: usize,
    pub status_canary_evidence_source_validated_count: usize,
    pub status_canary_evidence_source_rejected_count: usize,
    pub status_canary_evidence_source_generated_request_count: usize,
    pub status_canary_start_guard_ready: bool,
    pub status_canary_start_guard_id: &'static str,
    pub status_canary_start_guard_route: &'static str,
    pub status_canary_start_guard_switch_enabled: bool,
    pub status_canary_start_guard_evidence_packet_reason_audit_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_ready_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_rejected_count: usize,
    pub status_canary_start_guard_evidence_packet_reason_audit_ready: bool,
    pub status_canary_start_guard_blocked: bool,
    pub status_canary_start_guard_allowed: bool,
    pub status_canary_start_request_gate_ready: bool,
    pub status_canary_start_request_gate_id: &'static str,
    pub status_canary_start_request_gate_route: &'static str,
    pub status_canary_start_request_present: bool,
    pub status_canary_start_request_requested_tool_id: &'static str,
    pub status_canary_start_request_selected_status_canary: bool,
    pub status_canary_start_request_preflight_only_connector: bool,
    pub status_canary_start_request_source_start_guard_reason_audit_ready: bool,
    pub status_canary_start_request_blocked: bool,
    pub status_canary_start_request_allowed: bool,
    pub status_canary_runner_adapter_ready: bool,
    pub status_canary_runner_adapter_id: &'static str,
    pub status_canary_runner_adapter_route: &'static str,
    pub status_canary_runner_adapter_request_present: bool,
    pub status_canary_runner_adapter_source_gate_bound: bool,
    pub status_canary_runner_adapter_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_adapter_source_start_request_allowed: bool,
    pub status_canary_runner_adapter_blocked: bool,
    pub status_canary_runner_adapter_allowed: bool,
    pub status_canary_runner_start_surface_ready: bool,
    pub status_canary_runner_start_surface_id: &'static str,
    pub status_canary_runner_start_surface_route: &'static str,
    pub status_canary_runner_start_request_present: bool,
    pub status_canary_runner_start_surface_source_adapter_bound: bool,
    pub status_canary_runner_start_surface_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_start_surface_source_adapter_allowed: bool,
    pub status_canary_runner_start_surface_blocked: bool,
    pub status_canary_runner_start_surface_allowed: bool,
    pub status_canary_runner_entry_boundary_ready: bool,
    pub status_canary_runner_entry_boundary_id: &'static str,
    pub status_canary_runner_entry_boundary_route: &'static str,
    pub status_canary_runner_entry_request_present: bool,
    pub status_canary_runner_entry_boundary_source_start_surface_bound: bool,
    pub status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_entry_boundary_source_start_surface_allowed: bool,
    pub status_canary_runner_entry_boundary_blocked: bool,
    pub status_canary_runner_entry_boundary_allowed: bool,
    pub status_canary_runner_entry_adapter_ready: bool,
    pub status_canary_runner_entry_adapter_id: &'static str,
    pub status_canary_runner_entry_adapter_route: &'static str,
    pub status_canary_runner_entry_adapter_request_present: bool,
    pub status_canary_runner_entry_adapter_source_boundary_bound: bool,
    pub status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_entry_adapter_source_boundary_allowed: bool,
    pub status_canary_runner_entry_adapter_blocked: bool,
    pub status_canary_runner_entry_adapter_allowed: bool,
    pub status_canary_runner_binding_guard_ready: bool,
    pub status_canary_runner_binding_guard_id: &'static str,
    pub status_canary_runner_binding_guard_route: &'static str,
    pub status_canary_runner_binding_request_present: bool,
    pub status_canary_runner_binding_guard_source_entry_adapter_bound: bool,
    pub status_canary_runner_binding_guard_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_binding_guard_source_entry_adapter_allowed: bool,
    pub status_canary_runner_binding_guard_blocked: bool,
    pub status_canary_runner_binding_guard_allowed: bool,
    pub status_canary_runner_dry_run_selector_ready: bool,
    pub status_canary_runner_dry_run_selector_id: &'static str,
    pub status_canary_runner_dry_run_selector_route: &'static str,
    pub status_canary_runner_dry_run_selector_request_present: bool,
    pub status_canary_runner_dry_run_selector_source_binding_guard_bound: bool,
    pub status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready: bool,
    pub status_canary_runner_dry_run_selector_source_binding_guard_allowed: bool,
    pub status_canary_runner_dry_run_selector_blocked: bool,
    pub status_canary_runner_dry_run_selector_allowed: bool,
    pub status_canary_evidence_closure_entry_count: usize,
    pub status_canary_evidence_closure_ready_count: usize,
    pub status_canary_evidence_closure_missing_count: usize,
    pub status_canary_evidence_closure_recorded_count: usize,
    pub status_canary_evidence_closure_waived_count: usize,
    pub status_canary_evidence_closure_actionable_precondition_count: usize,
    pub capability_row_count: usize,
    pub live_enabled_count: usize,
    pub all_live_paths_blocked: bool,
    pub blocker_entry_count: usize,
    pub operator_visible_blocker_count: usize,
    pub missing_evidence_blocker_count: usize,
    pub accepted_blocker_count: usize,
    pub waived_blocker_count: usize,
    pub evidence_recorded_count: usize,
    pub approval_request_sent: bool,
    pub approval_accepted: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_execution_allowed: bool,
    pub dashboard_ready: bool,
    pub entries: Vec<ControlledLiveOperatorReadinessDashboardEntry>,
    pub status_canary_evidence_closure_entries: Vec<ControlledLiveStatusCanaryEvidenceClosureEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveOperatorReadinessDashboardSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorReadinessDashboardEntry {
    pub source_blocker_id: &'static str,
    pub dashboard_key: &'static str,
    pub dashboard_route: &'static str,
    pub source_readback_route: &'static str,
    pub operator_display_order: usize,
    pub operator_status: &'static str,
    pub evidence_state: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub acceptance_allowed: bool,
    pub waiver_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveStatusCanaryEvidenceClosureEntry {
    pub source_blocker_id: &'static str,
    pub selected_status_canary_tool_id: &'static str,
    pub preflight_only_connector_tool_id: &'static str,
    pub closure_key: &'static str,
    pub closure_route: &'static str,
    pub action_kind: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub evidence_state: &'static str,
    pub operator_visible: bool,
    pub action_required: bool,
    pub canary_start_blocked: bool,
    pub evidence_recorded: bool,
    pub evidence_waived: bool,
    pub evidence_expired: bool,
    pub evidence_invalid: bool,
    pub evidence_recording_allowed: bool,
    pub waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorReadinessDashboardSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub transport_mutated: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_operator_readiness_dashboard_report()
-> ControlledLiveOperatorReadinessDashboardReport {
    let source =
        controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report();
    let evidence_plan = controlled_live_required_evidence_collection_plan_report();
    let evidence_packet = status_canary_evidence_packet();
    let evidence_acceptance_packet = status_canary_evidence_acceptance_packet();
    let evidence_source_adapter = status_canary_evidence_source_adapter();
    let evidence_source_reason_packet =
        status_canary_evidence_source_reason_packet_from_adapter(&evidence_source_adapter);
    let evidence_source_readback =
        status_canary_evidence_source_readback_from_fixtures_and_reason_packet(
            &evidence_source_adapter.generated_fixtures,
            &evidence_source_reason_packet,
        );
    let evidence_source_validator = status_canary_evidence_source_validator_from_observations(
        &evidence_source_readback.observations,
    );
    let status_canary_start_guard = status_canary_start_guard();
    let status_canary_start_request_gate = status_canary_start_request_gate();
    let status_canary_runner_adapter = status_canary_runner_adapter_plan();
    let status_canary_runner_start_surface = status_canary_runner_start_surface_plan();
    let status_canary_runner_entry_boundary = status_canary_runner_entry_boundary_plan();
    let status_canary_runner_entry_adapter = status_canary_runner_entry_adapter_plan();
    let status_canary_runner_binding_guard = status_canary_runner_binding_guard_plan();
    let status_canary_runner_dry_run_selector = status_canary_runner_dry_run_selector_plan();
    let entries = controlled_live_operator_readiness_dashboard_entries();
    let status_canary_evidence_closure_entries =
        controlled_live_status_canary_evidence_closure_entries_from_packet(&evidence_packet);
    let operator_visible_blocker_count = entries
        .iter()
        .filter(|entry| entry.operator_visible)
        .count();
    let missing_evidence_blocker_count = entries
        .iter()
        .filter(|entry| entry.evidence_state == "missing")
        .count();
    let accepted_blocker_count = entries
        .iter()
        .filter(|entry| entry.acceptance_allowed)
        .count();
    let waived_blocker_count = entries.iter().filter(|entry| entry.waiver_allowed).count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let status_canary_evidence_closure_ready_count = status_canary_evidence_closure_entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.action_required
                && entry.canary_start_blocked
                && entry.evidence_state == "missing"
                && !entry.evidence_recorded
                && !entry.evidence_recording_allowed
                && !entry.waiver_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        })
        .count();
    let status_canary_evidence_closure_missing_count = status_canary_evidence_closure_entries
        .iter()
        .filter(|entry| entry.evidence_state == "missing")
        .count();
    let status_canary_evidence_closure_recorded_count = status_canary_evidence_closure_entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let status_canary_evidence_closure_waived_count = status_canary_evidence_closure_entries
        .iter()
        .filter(|entry| entry.evidence_waived)
        .count();
    let status_canary_evidence_closure_actionable_precondition_count =
        status_canary_evidence_closure_entries
            .iter()
            .filter(|entry| entry.action_required)
            .count();
    let persistence_allowed = entries.iter().any(|entry| entry.persistence_allowed);
    let status_canary_evidence_acceptance_packet_ready = evidence_acceptance_packet.packet_id
        == STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID
        && evidence_acceptance_packet.source_required_evidence_collection_plan_ready
        && evidence_acceptance_packet.checklist_item_count == 7
        && evidence_acceptance_packet.request_count == 0
        && evidence_acceptance_packet.known_request_count == 0
        && evidence_acceptance_packet.unknown_request_count == 0
        && evidence_acceptance_packet.duplicate_request_count == 0
        && evidence_acceptance_packet.request_source_validator_bound_count == 0
        && evidence_acceptance_packet.request_source_validator_contract_audit_ready_count == 0
        && evidence_acceptance_packet.request_reason_audit_count == 0
        && evidence_acceptance_packet.request_reason_audit_ready_count == 0
        && evidence_acceptance_packet.request_reason_audit_rejected_count == 0
        && evidence_acceptance_packet.accepted_decision_count == 0
        && evidence_acceptance_packet.rejected_decision_count == 0
        && evidence_acceptance_packet.generated_override_count == 0
        && evidence_acceptance_packet.generated_override_reason_audit_ready_count == 0
        && evidence_acceptance_packet.default_missing_decisions
        && evidence_acceptance_packet.source_evidence_packet_ready
        && !evidence_acceptance_packet.source_evidence_packet_complete
        && evidence_acceptance_packet.source_evidence_packet_missing_count == 7
        && evidence_acceptance_packet.source_evidence_packet_recorded_count == 0
        && evidence_acceptance_packet.source_evidence_packet_waived_count == 0
        && evidence_acceptance_packet.source_evidence_packet_expired_count == 0
        && evidence_acceptance_packet.source_evidence_packet_invalid_count == 0
        && evidence_acceptance_packet.acceptance_packet_ready
        && evidence_acceptance_packet.acceptance_packet_route
            == "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
        && evidence_acceptance_packet.side_effects
            == StatusCanaryEvidenceAcceptancePacketSideEffects::none();
    let status_canary_evidence_source_adapter_ready = evidence_source_adapter.adapter_id
        == STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID
        && evidence_source_adapter.source_required_evidence_collection_plan_ready
        && evidence_source_adapter.source_adapter_count == 7
        && evidence_source_adapter.adapter_input_count == 0
        && evidence_source_adapter.known_adapter_input_count == 0
        && evidence_source_adapter.unknown_adapter_input_count == 0
        && evidence_source_adapter.duplicate_adapter_input_count == 0
        && evidence_source_adapter.generated_fixture_count == 0
        && evidence_source_adapter.missing_adapter_input_count == 7
        && evidence_source_adapter.rejected_adapter_input_count == 0
        && evidence_source_adapter.metadata_contract_count == 7
        && evidence_source_adapter.metadata_contract_ready_count == 7
        && evidence_source_adapter.input_contract_field_count == 21
        && evidence_source_adapter.readback_fixture_contract_field_count == 70
        && evidence_source_adapter.required_field_validator_count == 7
        && evidence_source_adapter.required_field_validator_ready_count == 7
        && evidence_source_adapter.required_field_rejected_count == 0
        && evidence_source_adapter.missing_required_field_count == 0
        && evidence_source_adapter.source_adapter_ready
        && evidence_source_adapter.source_adapter_route
            == "status_canary_evidence_source_adapter_ready_no_inputs"
        && evidence_source_adapter.side_effects
            == StatusCanaryEvidenceSourceAdapterSideEffects::none();
    let status_canary_evidence_source_reason_packet_ready = evidence_source_reason_packet
        .reason_packet_id
        == STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID
        && evidence_source_reason_packet.source_adapter_bound
        && evidence_source_reason_packet.source_adapter_ready
        && evidence_source_reason_packet.source_adapter_id == evidence_source_adapter.adapter_id
        && evidence_source_reason_packet.source_adapter_route
            == evidence_source_adapter.source_adapter_route
        && evidence_source_reason_packet.source_count == 7
        && evidence_source_reason_packet.source_decision_reason_count == 28
        && evidence_source_reason_packet.source_decision_reason_ready_count == 28
        && evidence_source_reason_packet.decision_required_field_count == 84
        && evidence_source_reason_packet.missing_required_field_reason_count == 84
        && evidence_source_reason_packet.source_adapter_input_missing_reason_count == 28
        && evidence_source_reason_packet.source_adapter_input_other_decision_reason_count == 0
        && evidence_source_reason_packet.source_adapter_rejection_reason_count == 0
        && evidence_source_reason_packet.fixture_generation_allowed_count == 0
        && evidence_source_reason_packet.fixture_generation_blocked_count == 28
        && evidence_source_reason_packet.reason_packet_ready
        && evidence_source_reason_packet.reason_packet_route
            == "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
        && evidence_source_reason_packet.side_effects
            == StatusCanaryEvidenceSourceReasonPacketSideEffects::none();
    let status_canary_evidence_source_readback_ready = evidence_source_readback.readback_id
        == STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID
        && status_canary_evidence_source_adapter_ready
        && status_canary_evidence_source_reason_packet_ready
        && evidence_source_readback.source_required_evidence_collection_plan_ready
        && evidence_source_readback.source_item_count == 7
        && evidence_source_readback.fixture_input_count == 0
        && evidence_source_readback.known_fixture_count == 0
        && evidence_source_readback.unknown_fixture_count == 0
        && evidence_source_readback.duplicate_fixture_count == 0
        && evidence_source_readback.observation_count == 0
        && evidence_source_readback.missing_observation_count == 7
        && evidence_source_readback.rejected_fixture_count == 0
        && evidence_source_readback.source_contract_audit_count == 7
        && evidence_source_readback.source_contract_audit_ready_count == 7
        && evidence_source_readback.fixture_contract_audit_ready_count == 0
        && evidence_source_readback.source_reason_packet_bound
        && evidence_source_readback.source_reason_packet_ready
        && evidence_source_readback.source_reason_packet_route
            == evidence_source_reason_packet.reason_packet_route
        && evidence_source_readback.fixture_reason_audit_count == 0
        && evidence_source_readback.fixture_reason_audit_ready_count == 0
        && evidence_source_readback.fixture_reason_audit_rejected_count == 0
        && evidence_source_readback.source_readback_ready
        && evidence_source_readback.source_readback_route
            == "status_canary_evidence_source_readback_ready_no_fixtures"
        && evidence_source_readback.side_effects
            == StatusCanaryEvidenceSourceReadbackSideEffects::none();
    let status_canary_evidence_source_validator_ready = evidence_source_validator.validator_id
        == STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID
        && evidence_source_validator.source_required_evidence_collection_plan_ready
        && evidence_source_validator.source_item_count == 7
        && evidence_source_validator.observation_count == 0
        && evidence_source_validator.known_observation_count == 0
        && evidence_source_validator.unknown_observation_count == 0
        && evidence_source_validator.duplicate_observation_count == 0
        && evidence_source_validator.observation_contract_audit_count == 0
        && evidence_source_validator.observation_contract_audit_ready_count == 0
        && evidence_source_validator.observation_contract_audit_rejected_count == 0
        && evidence_source_validator.observation_reason_audit_count == 0
        && evidence_source_validator.observation_reason_audit_ready_count == 0
        && evidence_source_validator.observation_reason_audit_rejected_count == 0
        && evidence_source_validator.source_missing_count == 7
        && evidence_source_validator.source_validated_count == 0
        && evidence_source_validator.source_rejected_count == 0
        && evidence_source_validator.generated_request_count == 0
        && evidence_source_validator.generated_recorded_request_count == 0
        && evidence_source_validator.generated_waived_request_count == 0
        && evidence_source_validator.generated_expired_request_count == 0
        && evidence_source_validator.generated_invalid_request_count == 0
        && evidence_source_validator.source_acceptance_packet_ready
        && evidence_source_validator.source_acceptance_request_count == 0
        && evidence_source_validator.source_acceptance_generated_override_count == 0
        && !evidence_source_validator.source_acceptance_evidence_complete
        && evidence_source_validator.source_validator_ready
        && evidence_source_validator.source_validator_route
            == "status_canary_evidence_source_validator_ready_no_observations"
        && evidence_source_validator.side_effects
            == StatusCanaryEvidenceSourceValidatorSideEffects::none();
    let status_canary_start_guard_ready = status_canary_start_guard.guard_id
        == STATUS_CANARY_START_GUARD_ID
        && status_canary_start_guard.source_evidence_packet_id == evidence_packet.packet_id
        && status_canary_start_guard.source_evidence_packet_ready
        && !status_canary_start_guard.source_evidence_packet_complete
        && status_canary_start_guard.source_evidence_packet_missing_count == 7
        && status_canary_start_guard.source_evidence_packet_recorded_count == 0
        && status_canary_start_guard.source_evidence_packet_waived_count == 0
        && status_canary_start_guard.source_evidence_packet_expired_count == 0
        && status_canary_start_guard.source_evidence_packet_invalid_count == 0
        && status_canary_start_guard.source_evidence_packet_decision_reason_audit_count == 0
        && status_canary_start_guard.source_evidence_packet_decision_reason_audit_ready_count == 0
        && status_canary_start_guard.source_evidence_packet_decision_reason_audit_rejected_count
            == 0
        && status_canary_start_guard.source_evidence_packet_reason_audit_ready
        && !status_canary_start_guard.canary_start_switch_enabled
        && status_canary_start_guard.canary_start_blocked
        && !status_canary_start_guard.canary_start_allowed
        && status_canary_start_guard.guard_route
            == "status_canary_start_blocked_missing_evidence_packet"
        && status_canary_start_guard.side_effects == StatusCanaryStartGuardSideEffects::none();
    let status_canary_start_request_gate_ready = status_canary_start_request_gate.gate_id
        == STATUS_CANARY_START_REQUEST_GATE_ID
        && status_canary_start_request_gate.source_controlled_canary_ready
        && !status_canary_start_request_gate.source_controlled_canary_activation_ready
        && status_canary_start_request_gate.source_status_canary_start_guard_bound
        && status_canary_start_request_gate.source_status_canary_start_guard_reason_audit_ready
        && status_canary_start_request_gate.source_status_canary_start_guard_blocked
        && !status_canary_start_request_gate.source_status_canary_start_guard_allowed
        && status_canary_start_request_gate.source_runtime_boundaries_closed
        && status_canary_start_request_gate.source_controlled_canary_side_effects_closed
        && !status_canary_start_request_gate.start_request_present
        && status_canary_start_request_gate.requested_selected_status_canary
        && !status_canary_start_request_gate.requested_preflight_only_connector
        && status_canary_start_request_gate.start_request_blocked
        && !status_canary_start_request_gate.start_request_allowed
        && status_canary_start_request_gate.gate_route
            == "status_canary_start_request_blocked_no_request"
        && status_canary_start_request_gate.side_effects
            == StatusCanaryStartRequestGateSideEffects::none();
    let status_canary_runner_adapter_ready = status_canary_runner_adapter.adapter_id
        == STATUS_CANARY_RUNNER_ADAPTER_ID
        && status_canary_runner_adapter.source_start_request_gate_id
            == status_canary_start_request_gate.gate_id
        && status_canary_runner_adapter.source_start_request_gate_bound
        && status_canary_runner_adapter.source_start_request_present
            == status_canary_start_request_gate.start_request_present
        && status_canary_runner_adapter.source_requested_tool_id
            == status_canary_start_request_gate.requested_tool_id
        && status_canary_runner_adapter.source_requested_selected_status_canary
        && !status_canary_runner_adapter.source_requested_preflight_only_connector
        && status_canary_runner_adapter.source_start_request_gate_reason_audit_ready
        && status_canary_runner_adapter.source_start_request_blocked
            == status_canary_start_request_gate.start_request_blocked
        && !status_canary_runner_adapter.source_start_request_allowed
        && status_canary_runner_adapter.source_runtime_boundaries_closed
        && status_canary_runner_adapter.source_side_effects_closed
        && !status_canary_runner_adapter.runner_adapter_request_present
        && status_canary_runner_adapter.runner_adapter_plan_blocked
        && !status_canary_runner_adapter.runner_adapter_plan_allowed
        && status_canary_runner_adapter.adapter_route
            == "status_canary_runner_adapter_blocked_no_runner_request"
        && status_canary_runner_adapter.side_effects
            == StatusCanaryRunnerAdapterSideEffects::none();
    let status_canary_runner_start_surface_ready = status_canary_runner_start_surface.surface_id
        == STATUS_CANARY_RUNNER_START_SURFACE_ID
        && status_canary_runner_start_surface.source_runner_adapter_id
            == status_canary_runner_adapter.adapter_id
        && status_canary_runner_start_surface.source_runner_adapter_bound
        && !status_canary_runner_start_surface.source_runner_adapter_allowed
        && status_canary_runner_start_surface.source_runner_adapter_blocked
        && status_canary_runner_start_surface.source_start_request_gate_bound
        && status_canary_runner_start_surface.source_start_request_gate_reason_audit_ready
        && status_canary_runner_start_surface.source_requested_selected_status_canary
        && !status_canary_runner_start_surface.source_requested_preflight_only_connector
        && status_canary_runner_start_surface.source_runtime_boundaries_closed
        && status_canary_runner_start_surface.source_side_effects_closed
        && !status_canary_runner_start_surface.runner_start_request_present
        && status_canary_runner_start_surface.runner_start_surface_blocked
        && !status_canary_runner_start_surface.runner_start_surface_allowed
        && status_canary_runner_start_surface.surface_route
            == "status_canary_runner_start_surface_blocked_no_start_request"
        && status_canary_runner_start_surface.side_effects
            == StatusCanaryRunnerStartSurfaceSideEffects::none();
    let status_canary_runner_entry_boundary_ready = status_canary_runner_entry_boundary.boundary_id
        == STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID
        && status_canary_runner_entry_boundary.source_start_surface_id
            == status_canary_runner_start_surface.surface_id
        && status_canary_runner_entry_boundary.source_start_surface_bound
        && !status_canary_runner_entry_boundary.source_runner_start_surface_allowed
        && status_canary_runner_entry_boundary.source_runner_start_surface_blocked
        && status_canary_runner_entry_boundary.source_runner_adapter_bound
        && status_canary_runner_entry_boundary.source_start_request_gate_bound
        && status_canary_runner_entry_boundary.source_start_request_gate_reason_audit_ready
        && status_canary_runner_entry_boundary.source_requested_selected_status_canary
        && !status_canary_runner_entry_boundary.source_requested_preflight_only_connector
        && status_canary_runner_entry_boundary.source_runtime_boundaries_closed
        && status_canary_runner_entry_boundary.source_side_effects_closed
        && !status_canary_runner_entry_boundary.runner_entry_request_present
        && status_canary_runner_entry_boundary.runner_entry_boundary_blocked
        && !status_canary_runner_entry_boundary.runner_entry_boundary_allowed
        && status_canary_runner_entry_boundary.boundary_route
            == "status_canary_runner_entry_boundary_blocked_no_entry_request"
        && status_canary_runner_entry_boundary.side_effects
            == StatusCanaryRunnerEntryBoundarySideEffects::none();
    let status_canary_runner_entry_adapter_ready = status_canary_runner_entry_adapter.adapter_id
        == STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID
        && status_canary_runner_entry_adapter.source_entry_boundary_id
            == status_canary_runner_entry_boundary.boundary_id
        && status_canary_runner_entry_adapter.source_entry_boundary_bound
        && status_canary_runner_entry_adapter.source_start_request_gate_reason_audit_ready
        && !status_canary_runner_entry_adapter.source_runner_entry_boundary_allowed
        && status_canary_runner_entry_adapter.source_runner_entry_boundary_blocked
        && status_canary_runner_entry_adapter.source_start_surface_bound
        && !status_canary_runner_entry_adapter.source_start_surface_allowed
        && status_canary_runner_entry_adapter.source_requested_selected_status_canary
        && !status_canary_runner_entry_adapter.source_requested_preflight_only_connector
        && status_canary_runner_entry_adapter.source_runtime_boundaries_closed
        && status_canary_runner_entry_adapter.source_side_effects_closed
        && !status_canary_runner_entry_adapter.runner_entry_adapter_request_present
        && status_canary_runner_entry_adapter.runner_entry_adapter_plan_blocked
        && !status_canary_runner_entry_adapter.runner_entry_adapter_plan_allowed
        && status_canary_runner_entry_adapter.adapter_route
            == "status_canary_runner_entry_adapter_blocked_no_adapter_request"
        && status_canary_runner_entry_adapter.side_effects
            == StatusCanaryRunnerEntryAdapterSideEffects::none();
    let status_canary_runner_binding_guard_ready = status_canary_runner_binding_guard.guard_id
        == STATUS_CANARY_RUNNER_BINDING_GUARD_ID
        && status_canary_runner_binding_guard.source_entry_adapter_id
            == status_canary_runner_entry_adapter.adapter_id
        && status_canary_runner_binding_guard.source_entry_adapter_bound
        && !status_canary_runner_binding_guard.source_runner_entry_adapter_allowed
        && status_canary_runner_binding_guard.source_runner_entry_adapter_blocked
        && status_canary_runner_binding_guard.source_entry_boundary_bound
        && !status_canary_runner_binding_guard.source_entry_boundary_allowed
        && status_canary_runner_binding_guard.source_start_surface_bound
        && !status_canary_runner_binding_guard.source_start_surface_allowed
        && status_canary_runner_binding_guard.source_start_request_gate_reason_audit_ready
        && status_canary_runner_binding_guard.source_requested_selected_status_canary
        && !status_canary_runner_binding_guard.source_requested_preflight_only_connector
        && status_canary_runner_binding_guard.source_runtime_boundaries_closed
        && status_canary_runner_binding_guard.source_side_effects_closed
        && !status_canary_runner_binding_guard.runner_binding_request_present
        && status_canary_runner_binding_guard.runner_binding_guard_blocked
        && !status_canary_runner_binding_guard.runner_binding_guard_allowed
        && status_canary_runner_binding_guard.guard_route
            == "status_canary_runner_binding_guard_blocked_no_binding_request"
        && status_canary_runner_binding_guard.side_effects
            == StatusCanaryRunnerBindingGuardSideEffects::none();
    let status_canary_runner_dry_run_selector_ready = status_canary_runner_dry_run_selector
        .selector_id
        == STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID
        && status_canary_runner_dry_run_selector.source_binding_guard_id
            == status_canary_runner_binding_guard.guard_id
        && status_canary_runner_dry_run_selector.source_binding_guard_bound
        && !status_canary_runner_dry_run_selector.source_runner_binding_guard_allowed
        && status_canary_runner_dry_run_selector.source_runner_binding_guard_blocked
        && status_canary_runner_dry_run_selector.source_entry_adapter_bound
        && !status_canary_runner_dry_run_selector.source_entry_adapter_allowed
        && status_canary_runner_dry_run_selector.source_start_request_gate_reason_audit_ready
        && status_canary_runner_dry_run_selector.source_requested_selected_status_canary
        && !status_canary_runner_dry_run_selector.source_requested_preflight_only_connector
        && status_canary_runner_dry_run_selector.source_runtime_boundaries_closed
        && status_canary_runner_dry_run_selector.source_side_effects_closed
        && !status_canary_runner_dry_run_selector.runner_dry_run_selector_request_present
        && status_canary_runner_dry_run_selector.runner_dry_run_selector_blocked
        && !status_canary_runner_dry_run_selector.runner_dry_run_selector_allowed
        && status_canary_runner_dry_run_selector.selector_route
            == "status_canary_runner_dry_run_selector_blocked_no_selector_request"
        && status_canary_runner_dry_run_selector.side_effects
            == StatusCanaryRunnerDryRunSelectorSideEffects::none();
    let dashboard_ready = source.kill_switch_rehearsal_boundary_readback_ready
        && evidence_plan.evidence_collection_plan_ready
        && source.kill_switch_rehearsal_boundary_entry_count == 7
        && source.kill_switch_rehearsal_boundary_ready_count == 7
        && source.kill_switch_rehearsal_boundary_closed_count == 7
        && source.kill_switch_mutation_blocked_count == 7
        && entries.len() == 7
        && operator_visible_blocker_count == 7
        && missing_evidence_blocker_count == 7
        && accepted_blocker_count == 0
        && waived_blocker_count == 0
        && evidence_recorded_count == 0
        && status_canary_evidence_closure_entries.len() == 7
        && status_canary_evidence_closure_ready_count == 7
        && status_canary_evidence_closure_missing_count == 7
        && status_canary_evidence_closure_recorded_count == 0
        && status_canary_evidence_closure_waived_count == 0
        && status_canary_evidence_closure_actionable_precondition_count == 7
        && evidence_packet.packet_ready
        && evidence_packet.packet_id == STATUS_CANARY_EVIDENCE_PACKET_ID
        && evidence_packet.checklist_item_count == 7
        && evidence_packet.missing_item_count == 7
        && evidence_packet.recorded_item_count == 0
        && evidence_packet.waived_item_count == 0
        && evidence_packet.expired_item_count == 0
        && evidence_packet.invalid_item_count == 0
        && evidence_packet.decision_reason_audit_count == 0
        && evidence_packet.decision_reason_audit_ready_count == 0
        && evidence_packet.decision_reason_audit_rejected_count == 0
        && !evidence_packet.evidence_complete
        && evidence_packet.canary_start_blocked
        && !evidence_packet.canary_start_allowed
        && status_canary_evidence_acceptance_packet_ready
        && status_canary_evidence_source_adapter_ready
        && status_canary_evidence_source_reason_packet_ready
        && status_canary_evidence_source_readback_ready
        && status_canary_evidence_source_validator_ready
        && status_canary_start_guard_ready
        && status_canary_start_request_gate_ready
        && status_canary_runner_adapter_ready
        && status_canary_runner_start_surface_ready
        && status_canary_runner_entry_boundary_ready
        && status_canary_runner_entry_adapter_ready
        && status_canary_runner_binding_guard_ready
        && status_canary_runner_dry_run_selector_ready
        && !source.approval_request_sent
        && !source.approval_accepted
        && !source.credential_read_allowed
        && !source.transport_mutation_allowed
        && !source.packet_persisted
        && !source.attachment_persisted
        && !source.readback_persisted
        && !source.live_execution_allowed
        && entries.iter().all(|entry| {
            entry.queryable
                && entry.diffable
                && !entry.acceptance_allowed
                && !entry.waiver_allowed
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.transport_mutation_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveOperatorReadinessDashboardReport {
        runtime: "hepta",
        surface: "controlled_live_operator_readiness_dashboard",
        status: if dashboard_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_GATE,
        schema_version: CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_kill_switch_boundary_readback_ready: source
            .kill_switch_rehearsal_boundary_readback_ready,
        source_required_evidence_collection_plan_ready: evidence_plan
            .evidence_collection_plan_ready,
        status_canary_final_guard_present: true,
        status_canary_tool_id: SELECTED_STATUS_CANARY_TOOL_ID,
        preflight_only_connector_tool_id: PREFLIGHT_ONLY_CONNECTOR_TOOL_ID,
        status_canary_candidate_count: 2,
        selected_status_canary_count: 1,
        preflight_only_non_selected_count: 1,
        status_canary_final_gate_ready_count: 2,
        status_canary_final_guard_live_blocked_count: 1,
        status_canary_final_guard_approval_missing_count: 1,
        status_canary_final_guard_live_enabled: false,
        status_canary_final_guard_tool_invocation_enabled: false,
        status_canary_final_guard_ledger_write_enabled: false,
        status_canary_evidence_packet_ready: evidence_packet.packet_ready,
        status_canary_evidence_packet_id: evidence_packet.packet_id,
        status_canary_evidence_packet_item_count: evidence_packet.checklist_item_count,
        status_canary_evidence_packet_missing_count: evidence_packet.missing_item_count,
        status_canary_evidence_packet_recorded_count: evidence_packet.recorded_item_count,
        status_canary_evidence_packet_waived_count: evidence_packet.waived_item_count,
        status_canary_evidence_packet_expired_count: evidence_packet.expired_item_count,
        status_canary_evidence_packet_invalid_count: evidence_packet.invalid_item_count,
        status_canary_evidence_packet_decision_reason_audit_count: evidence_packet
            .decision_reason_audit_count,
        status_canary_evidence_packet_decision_reason_audit_ready_count: evidence_packet
            .decision_reason_audit_ready_count,
        status_canary_evidence_packet_decision_reason_audit_rejected_count: evidence_packet
            .decision_reason_audit_rejected_count,
        status_canary_evidence_packet_complete: evidence_packet.evidence_complete,
        status_canary_start_blocked_by_evidence_packet: evidence_packet.canary_start_blocked,
        status_canary_start_allowed_by_evidence_packet: evidence_packet.canary_start_allowed,
        status_canary_evidence_packet_guard_route: evidence_packet.guard_route,
        status_canary_evidence_acceptance_packet_ready,
        status_canary_evidence_acceptance_packet_id: evidence_acceptance_packet.packet_id,
        status_canary_evidence_acceptance_packet_route: evidence_acceptance_packet
            .acceptance_packet_route,
        status_canary_evidence_acceptance_request_count: evidence_acceptance_packet.request_count,
        status_canary_evidence_acceptance_known_request_count: evidence_acceptance_packet
            .known_request_count,
        status_canary_evidence_acceptance_unknown_request_count: evidence_acceptance_packet
            .unknown_request_count,
        status_canary_evidence_acceptance_duplicate_request_count: evidence_acceptance_packet
            .duplicate_request_count,
        status_canary_evidence_acceptance_request_source_validator_bound_count:
            evidence_acceptance_packet.request_source_validator_bound_count,
        status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count:
            evidence_acceptance_packet.request_source_validator_contract_audit_ready_count,
        status_canary_evidence_acceptance_request_reason_audit_count: evidence_acceptance_packet
            .request_reason_audit_count,
        status_canary_evidence_acceptance_request_reason_audit_ready_count:
            evidence_acceptance_packet.request_reason_audit_ready_count,
        status_canary_evidence_acceptance_request_reason_audit_rejected_count:
            evidence_acceptance_packet.request_reason_audit_rejected_count,
        status_canary_evidence_acceptance_accepted_decision_count: evidence_acceptance_packet
            .accepted_decision_count,
        status_canary_evidence_acceptance_rejected_decision_count: evidence_acceptance_packet
            .rejected_decision_count,
        status_canary_evidence_acceptance_generated_override_count: evidence_acceptance_packet
            .generated_override_count,
        status_canary_evidence_acceptance_generated_override_reason_audit_ready_count:
            evidence_acceptance_packet.generated_override_reason_audit_ready_count,
        status_canary_evidence_source_adapter_ready,
        status_canary_evidence_source_adapter_id: evidence_source_adapter.adapter_id,
        status_canary_evidence_source_adapter_route: evidence_source_adapter.source_adapter_route,
        status_canary_evidence_source_adapter_count: evidence_source_adapter.source_adapter_count,
        status_canary_evidence_source_adapter_input_count: evidence_source_adapter
            .adapter_input_count,
        status_canary_evidence_source_adapter_generated_fixture_count: evidence_source_adapter
            .generated_fixture_count,
        status_canary_evidence_source_adapter_missing_input_count: evidence_source_adapter
            .missing_adapter_input_count,
        status_canary_evidence_source_adapter_metadata_contract_count: evidence_source_adapter
            .metadata_contract_count,
        status_canary_evidence_source_adapter_metadata_contract_ready_count:
            evidence_source_adapter.metadata_contract_ready_count,
        status_canary_evidence_source_adapter_input_contract_field_count: evidence_source_adapter
            .input_contract_field_count,
        status_canary_evidence_source_adapter_readback_fixture_contract_field_count:
            evidence_source_adapter.readback_fixture_contract_field_count,
        status_canary_evidence_source_adapter_required_field_validator_count:
            evidence_source_adapter.required_field_validator_count,
        status_canary_evidence_source_adapter_required_field_validator_ready_count:
            evidence_source_adapter.required_field_validator_ready_count,
        status_canary_evidence_source_adapter_required_field_rejected_count:
            evidence_source_adapter.required_field_rejected_count,
        status_canary_evidence_source_adapter_missing_required_field_count: evidence_source_adapter
            .missing_required_field_count,
        status_canary_evidence_source_reason_packet_ready,
        status_canary_evidence_source_reason_packet_id: evidence_source_reason_packet
            .reason_packet_id,
        status_canary_evidence_source_reason_packet_route: evidence_source_reason_packet
            .reason_packet_route,
        status_canary_evidence_source_reason_packet_source_count: evidence_source_reason_packet
            .source_count,
        status_canary_evidence_source_decision_reason_count: evidence_source_reason_packet
            .source_decision_reason_count,
        status_canary_evidence_source_decision_reason_ready_count: evidence_source_reason_packet
            .source_decision_reason_ready_count,
        status_canary_evidence_source_decision_required_field_count: evidence_source_reason_packet
            .decision_required_field_count,
        status_canary_evidence_source_missing_required_field_reason_count:
            evidence_source_reason_packet.missing_required_field_reason_count,
        status_canary_evidence_source_adapter_input_missing_reason_count:
            evidence_source_reason_packet.source_adapter_input_missing_reason_count,
        status_canary_evidence_source_adapter_input_other_decision_reason_count:
            evidence_source_reason_packet.source_adapter_input_other_decision_reason_count,
        status_canary_evidence_source_adapter_rejection_reason_count: evidence_source_reason_packet
            .source_adapter_rejection_reason_count,
        status_canary_evidence_source_fixture_generation_allowed_count:
            evidence_source_reason_packet.fixture_generation_allowed_count,
        status_canary_evidence_source_fixture_generation_blocked_count:
            evidence_source_reason_packet.fixture_generation_blocked_count,
        status_canary_evidence_source_readback_ready,
        status_canary_evidence_source_readback_id: evidence_source_readback.readback_id,
        status_canary_evidence_source_readback_route: evidence_source_readback
            .source_readback_route,
        status_canary_evidence_source_readback_fixture_count: evidence_source_readback
            .fixture_input_count,
        status_canary_evidence_source_readback_observation_count: evidence_source_readback
            .observation_count,
        status_canary_evidence_source_readback_missing_observation_count: evidence_source_readback
            .missing_observation_count,
        status_canary_evidence_source_readback_contract_audit_count: evidence_source_readback
            .source_contract_audit_count,
        status_canary_evidence_source_readback_contract_audit_ready_count: evidence_source_readback
            .source_contract_audit_ready_count,
        status_canary_evidence_source_readback_fixture_contract_audit_ready_count:
            evidence_source_readback.fixture_contract_audit_ready_count,
        status_canary_evidence_source_readback_reason_packet_bound: evidence_source_readback
            .source_reason_packet_bound,
        status_canary_evidence_source_readback_reason_packet_ready: evidence_source_readback
            .source_reason_packet_ready,
        status_canary_evidence_source_readback_reason_packet_route: evidence_source_readback
            .source_reason_packet_route,
        status_canary_evidence_source_readback_fixture_reason_audit_count: evidence_source_readback
            .fixture_reason_audit_count,
        status_canary_evidence_source_readback_fixture_reason_audit_ready_count:
            evidence_source_readback.fixture_reason_audit_ready_count,
        status_canary_evidence_source_readback_fixture_reason_audit_rejected_count:
            evidence_source_readback.fixture_reason_audit_rejected_count,
        status_canary_evidence_source_validator_ready,
        status_canary_evidence_source_validator_id: evidence_source_validator.validator_id,
        status_canary_evidence_source_validator_route: evidence_source_validator
            .source_validator_route,
        status_canary_evidence_source_validator_contract_audit_count: evidence_source_validator
            .observation_contract_audit_count,
        status_canary_evidence_source_validator_contract_audit_ready_count:
            evidence_source_validator.observation_contract_audit_ready_count,
        status_canary_evidence_source_validator_contract_audit_rejected_count:
            evidence_source_validator.observation_contract_audit_rejected_count,
        status_canary_evidence_source_validator_reason_audit_count: evidence_source_validator
            .observation_reason_audit_count,
        status_canary_evidence_source_validator_reason_audit_ready_count: evidence_source_validator
            .observation_reason_audit_ready_count,
        status_canary_evidence_source_validator_reason_audit_rejected_count:
            evidence_source_validator.observation_reason_audit_rejected_count,
        status_canary_evidence_source_observation_count: evidence_source_validator
            .observation_count,
        status_canary_evidence_source_missing_count: evidence_source_validator.source_missing_count,
        status_canary_evidence_source_validated_count: evidence_source_validator
            .source_validated_count,
        status_canary_evidence_source_rejected_count: evidence_source_validator
            .source_rejected_count,
        status_canary_evidence_source_generated_request_count: evidence_source_validator
            .generated_request_count,
        status_canary_start_guard_ready,
        status_canary_start_guard_id: status_canary_start_guard.guard_id,
        status_canary_start_guard_route: status_canary_start_guard.guard_route,
        status_canary_start_guard_switch_enabled: status_canary_start_guard
            .canary_start_switch_enabled,
        status_canary_start_guard_evidence_packet_reason_audit_count: status_canary_start_guard
            .source_evidence_packet_decision_reason_audit_count,
        status_canary_start_guard_evidence_packet_reason_audit_ready_count:
            status_canary_start_guard.source_evidence_packet_decision_reason_audit_ready_count,
        status_canary_start_guard_evidence_packet_reason_audit_rejected_count:
            status_canary_start_guard.source_evidence_packet_decision_reason_audit_rejected_count,
        status_canary_start_guard_evidence_packet_reason_audit_ready: status_canary_start_guard
            .source_evidence_packet_reason_audit_ready,
        status_canary_start_guard_blocked: status_canary_start_guard.canary_start_blocked,
        status_canary_start_guard_allowed: status_canary_start_guard.canary_start_allowed,
        status_canary_start_request_gate_ready,
        status_canary_start_request_gate_id: status_canary_start_request_gate.gate_id,
        status_canary_start_request_gate_route: status_canary_start_request_gate.gate_route,
        status_canary_start_request_present: status_canary_start_request_gate.start_request_present,
        status_canary_start_request_requested_tool_id: status_canary_start_request_gate
            .requested_tool_id,
        status_canary_start_request_selected_status_canary: status_canary_start_request_gate
            .requested_selected_status_canary,
        status_canary_start_request_preflight_only_connector: status_canary_start_request_gate
            .requested_preflight_only_connector,
        status_canary_start_request_source_start_guard_reason_audit_ready:
            status_canary_start_request_gate.source_status_canary_start_guard_reason_audit_ready,
        status_canary_start_request_blocked: status_canary_start_request_gate.start_request_blocked,
        status_canary_start_request_allowed: status_canary_start_request_gate.start_request_allowed,
        status_canary_runner_adapter_ready,
        status_canary_runner_adapter_id: status_canary_runner_adapter.adapter_id,
        status_canary_runner_adapter_route: status_canary_runner_adapter.adapter_route,
        status_canary_runner_adapter_request_present: status_canary_runner_adapter
            .runner_adapter_request_present,
        status_canary_runner_adapter_source_gate_bound: status_canary_runner_adapter
            .source_start_request_gate_bound,
        status_canary_runner_adapter_source_start_guard_reason_audit_ready:
            status_canary_runner_adapter.source_start_request_gate_reason_audit_ready,
        status_canary_runner_adapter_source_start_request_allowed: status_canary_runner_adapter
            .source_start_request_allowed,
        status_canary_runner_adapter_blocked: status_canary_runner_adapter
            .runner_adapter_plan_blocked,
        status_canary_runner_adapter_allowed: status_canary_runner_adapter
            .runner_adapter_plan_allowed,
        status_canary_runner_start_surface_ready,
        status_canary_runner_start_surface_id: status_canary_runner_start_surface.surface_id,
        status_canary_runner_start_surface_route: status_canary_runner_start_surface.surface_route,
        status_canary_runner_start_request_present: status_canary_runner_start_surface
            .runner_start_request_present,
        status_canary_runner_start_surface_source_adapter_bound: status_canary_runner_start_surface
            .source_runner_adapter_bound,
        status_canary_runner_start_surface_source_start_guard_reason_audit_ready:
            status_canary_runner_start_surface.source_start_request_gate_reason_audit_ready,
        status_canary_runner_start_surface_source_adapter_allowed:
            status_canary_runner_start_surface.source_runner_adapter_allowed,
        status_canary_runner_start_surface_blocked: status_canary_runner_start_surface
            .runner_start_surface_blocked,
        status_canary_runner_start_surface_allowed: status_canary_runner_start_surface
            .runner_start_surface_allowed,
        status_canary_runner_entry_boundary_ready,
        status_canary_runner_entry_boundary_id: status_canary_runner_entry_boundary.boundary_id,
        status_canary_runner_entry_boundary_route: status_canary_runner_entry_boundary
            .boundary_route,
        status_canary_runner_entry_request_present: status_canary_runner_entry_boundary
            .runner_entry_request_present,
        status_canary_runner_entry_boundary_source_start_surface_bound:
            status_canary_runner_entry_boundary.source_start_surface_bound,
        status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready:
            status_canary_runner_entry_boundary.source_start_request_gate_reason_audit_ready,
        status_canary_runner_entry_boundary_source_start_surface_allowed:
            status_canary_runner_entry_boundary.source_runner_start_surface_allowed,
        status_canary_runner_entry_boundary_blocked: status_canary_runner_entry_boundary
            .runner_entry_boundary_blocked,
        status_canary_runner_entry_boundary_allowed: status_canary_runner_entry_boundary
            .runner_entry_boundary_allowed,
        status_canary_runner_entry_adapter_ready,
        status_canary_runner_entry_adapter_id: status_canary_runner_entry_adapter.adapter_id,
        status_canary_runner_entry_adapter_route: status_canary_runner_entry_adapter.adapter_route,
        status_canary_runner_entry_adapter_request_present: status_canary_runner_entry_adapter
            .runner_entry_adapter_request_present,
        status_canary_runner_entry_adapter_source_boundary_bound:
            status_canary_runner_entry_adapter.source_entry_boundary_bound,
        status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready:
            status_canary_runner_entry_adapter.source_start_request_gate_reason_audit_ready,
        status_canary_runner_entry_adapter_source_boundary_allowed:
            status_canary_runner_entry_adapter.source_runner_entry_boundary_allowed,
        status_canary_runner_entry_adapter_blocked: status_canary_runner_entry_adapter
            .runner_entry_adapter_plan_blocked,
        status_canary_runner_entry_adapter_allowed: status_canary_runner_entry_adapter
            .runner_entry_adapter_plan_allowed,
        status_canary_runner_binding_guard_ready,
        status_canary_runner_binding_guard_id: status_canary_runner_binding_guard.guard_id,
        status_canary_runner_binding_guard_route: status_canary_runner_binding_guard.guard_route,
        status_canary_runner_binding_request_present: status_canary_runner_binding_guard
            .runner_binding_request_present,
        status_canary_runner_binding_guard_source_entry_adapter_bound:
            status_canary_runner_binding_guard.source_entry_adapter_bound,
        status_canary_runner_binding_guard_source_start_guard_reason_audit_ready:
            status_canary_runner_binding_guard.source_start_request_gate_reason_audit_ready,
        status_canary_runner_binding_guard_source_entry_adapter_allowed:
            status_canary_runner_binding_guard.source_runner_entry_adapter_allowed,
        status_canary_runner_binding_guard_blocked: status_canary_runner_binding_guard
            .runner_binding_guard_blocked,
        status_canary_runner_binding_guard_allowed: status_canary_runner_binding_guard
            .runner_binding_guard_allowed,
        status_canary_runner_dry_run_selector_ready,
        status_canary_runner_dry_run_selector_id: status_canary_runner_dry_run_selector.selector_id,
        status_canary_runner_dry_run_selector_route: status_canary_runner_dry_run_selector
            .selector_route,
        status_canary_runner_dry_run_selector_request_present:
            status_canary_runner_dry_run_selector.runner_dry_run_selector_request_present,
        status_canary_runner_dry_run_selector_source_binding_guard_bound:
            status_canary_runner_dry_run_selector.source_binding_guard_bound,
        status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready:
            status_canary_runner_dry_run_selector.source_start_request_gate_reason_audit_ready,
        status_canary_runner_dry_run_selector_source_binding_guard_allowed:
            status_canary_runner_dry_run_selector.source_runner_binding_guard_allowed,
        status_canary_runner_dry_run_selector_blocked: status_canary_runner_dry_run_selector
            .runner_dry_run_selector_blocked,
        status_canary_runner_dry_run_selector_allowed: status_canary_runner_dry_run_selector
            .runner_dry_run_selector_allowed,
        status_canary_evidence_closure_entry_count: status_canary_evidence_closure_entries.len(),
        status_canary_evidence_closure_ready_count,
        status_canary_evidence_closure_missing_count,
        status_canary_evidence_closure_recorded_count,
        status_canary_evidence_closure_waived_count,
        status_canary_evidence_closure_actionable_precondition_count,
        capability_row_count: current_reality_capability_registry_count(),
        live_enabled_count: 0,
        all_live_paths_blocked: true,
        blocker_entry_count: entries.len(),
        operator_visible_blocker_count,
        missing_evidence_blocker_count,
        accepted_blocker_count,
        waived_blocker_count,
        evidence_recorded_count,
        approval_request_sent: false,
        approval_accepted: false,
        credential_read_allowed: false,
        transport_mutation_allowed: false,
        persistence_allowed,
        live_execution_allowed: false,
        dashboard_ready,
        entries,
        status_canary_evidence_closure_entries,
        recommended_next_gate: CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveOperatorReadinessDashboardSideEffects::none(),
    }
}

pub fn controlled_live_status_canary_evidence_closure_entries()
-> Vec<ControlledLiveStatusCanaryEvidenceClosureEntry> {
    let evidence_packet = status_canary_evidence_packet();
    controlled_live_status_canary_evidence_closure_entries_from_packet(&evidence_packet)
}

fn controlled_live_status_canary_evidence_closure_entries_from_packet(
    evidence_packet: &StatusCanaryEvidencePacket,
) -> Vec<ControlledLiveStatusCanaryEvidenceClosureEntry> {
    evidence_packet
        .entries
        .iter()
        .map(|entry| ControlledLiveStatusCanaryEvidenceClosureEntry {
            source_blocker_id: entry.source_blocker_id,
            selected_status_canary_tool_id: evidence_packet.selected_status_canary_tool_id,
            preflight_only_connector_tool_id: evidence_packet.preflight_only_connector_tool_id,
            closure_key: status_canary_evidence_closure_key(entry.source_blocker_id),
            closure_route: status_canary_evidence_closure_route(entry.source_blocker_id),
            action_kind: entry.action_kind,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            evidence_state: entry.evidence_state,
            operator_visible: entry.operator_visible,
            action_required: entry.action_required,
            canary_start_blocked: entry.canary_start_blocked,
            evidence_recorded: entry.evidence_recorded,
            evidence_waived: entry.evidence_waived,
            evidence_expired: entry.evidence_expired,
            evidence_invalid: entry.evidence_invalid,
            evidence_recording_allowed: false,
            waiver_allowed: false,
            credential_read_allowed: false,
            transport_mutation_allowed: false,
            persistence_allowed: false,
            live_mutation_allowed: false,
        })
        .collect()
}

pub fn controlled_live_operator_readiness_dashboard_entries()
-> Vec<ControlledLiveOperatorReadinessDashboardEntry> {
    controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveOperatorReadinessDashboardEntry {
            source_blocker_id: entry.source_blocker_id,
            dashboard_key: dashboard_key(entry.source_blocker_id),
            dashboard_route: dashboard_route(entry.source_blocker_id),
            source_readback_route: entry.kill_switch_rehearsal_boundary_route,
            operator_display_order: entry.operator_display_order,
            operator_status: entry.operator_status,
            evidence_state: entry.kill_switch_rehearsal_evidence_state,
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            operator_visible: true,
            queryable: true,
            diffable: true,
            acceptance_allowed: false,
            waiver_allowed: false,
            evidence_recording_allowed: false,
            credential_read_allowed: false,
            transport_mutation_allowed: false,
            persistence_allowed: false,
            live_mutation_allowed: false,
        })
        .collect()
}

fn dashboard_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "controlled_live.operator_dashboard.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "controlled_live.operator_dashboard.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.operator_dashboard.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.operator_dashboard.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.operator_dashboard.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.operator_dashboard.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.operator_dashboard.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.operator_dashboard.unknown",
    }
}

fn dashboard_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/operator-dashboard/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/operator-dashboard/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/operator-dashboard/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/operator-dashboard/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/operator-dashboard/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/operator-dashboard/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/operator-dashboard/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/operator-dashboard/unknown",
    }
}

fn status_canary_evidence_closure_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.status_canary.evidence_closure.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.status_canary.evidence_closure.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.status_canary.evidence_closure.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.status_canary.evidence_closure.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.status_canary.evidence_closure.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.status_canary.evidence_closure.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.status_canary.evidence_closure.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.status_canary.evidence_closure.unknown",
    }
}

fn status_canary_evidence_closure_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/status-canary/evidence-closure/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/status-canary/evidence-closure/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/status-canary/evidence-closure/unknown",
    }
}

impl ControlledLiveOperatorReadinessDashboardSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
            transport_mutated: false,
            packet_persisted: false,
            attachment_persisted: false,
            readback_persisted: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            provider_invoked: false,
            model_invoked: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_collapses_current_readiness_without_live_paths() {
        let report = controlled_live_operator_readiness_dashboard_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_kill_switch_boundary_readback_ready);
        assert!(report.status_canary_final_guard_present);
        assert_eq!(
            report.status_canary_tool_id,
            "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
        );
        assert_eq!(
            report.preflight_only_connector_tool_id,
            "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
        );
        assert_eq!(report.status_canary_candidate_count, 2);
        assert_eq!(report.selected_status_canary_count, 1);
        assert_eq!(report.preflight_only_non_selected_count, 1);
        assert_eq!(report.status_canary_final_gate_ready_count, 2);
        assert_eq!(report.status_canary_final_guard_live_blocked_count, 1);
        assert_eq!(report.status_canary_final_guard_approval_missing_count, 1);
        assert!(!report.status_canary_final_guard_live_enabled);
        assert!(!report.status_canary_final_guard_tool_invocation_enabled);
        assert!(!report.status_canary_final_guard_ledger_write_enabled);
        assert!(report.source_required_evidence_collection_plan_ready);
        assert!(report.status_canary_evidence_packet_ready);
        assert_eq!(
            report.status_canary_evidence_packet_id,
            STATUS_CANARY_EVIDENCE_PACKET_ID
        );
        assert_eq!(report.status_canary_evidence_packet_item_count, 7);
        assert_eq!(report.status_canary_evidence_packet_missing_count, 7);
        assert_eq!(report.status_canary_evidence_packet_recorded_count, 0);
        assert_eq!(report.status_canary_evidence_packet_waived_count, 0);
        assert_eq!(report.status_canary_evidence_packet_expired_count, 0);
        assert_eq!(report.status_canary_evidence_packet_invalid_count, 0);
        assert_eq!(
            report.status_canary_evidence_packet_decision_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_packet_decision_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_packet_decision_reason_audit_rejected_count,
            0
        );
        assert!(!report.status_canary_evidence_packet_complete);
        assert!(report.status_canary_start_blocked_by_evidence_packet);
        assert!(!report.status_canary_start_allowed_by_evidence_packet);
        assert_eq!(
            report.status_canary_evidence_packet_guard_route,
            "status_canary_evidence_packet_blocked_missing_evidence"
        );
        assert!(report.status_canary_evidence_acceptance_packet_ready);
        assert_eq!(
            report.status_canary_evidence_acceptance_packet_id,
            STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_packet_route,
            "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
        );
        assert_eq!(report.status_canary_evidence_acceptance_request_count, 0);
        assert_eq!(
            report.status_canary_evidence_acceptance_known_request_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_unknown_request_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_duplicate_request_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_request_source_validator_bound_count,
            0
        );
        assert_eq!(
            report
                .status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_request_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_request_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_request_reason_audit_rejected_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_accepted_decision_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_rejected_decision_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_generated_override_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_acceptance_generated_override_reason_audit_ready_count,
            0
        );
        assert!(report.status_canary_evidence_source_adapter_ready);
        assert_eq!(
            report.status_canary_evidence_source_adapter_id,
            STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_route,
            "status_canary_evidence_source_adapter_ready_no_inputs"
        );
        assert_eq!(report.status_canary_evidence_source_adapter_count, 7);
        assert_eq!(report.status_canary_evidence_source_adapter_input_count, 0);
        assert_eq!(
            report.status_canary_evidence_source_adapter_generated_fixture_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_missing_input_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_metadata_contract_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_metadata_contract_ready_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_input_contract_field_count,
            21
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_readback_fixture_contract_field_count,
            70
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_required_field_validator_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_required_field_validator_ready_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_required_field_rejected_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_missing_required_field_count,
            0
        );
        assert!(report.status_canary_evidence_source_reason_packet_ready);
        assert_eq!(
            report.status_canary_evidence_source_reason_packet_id,
            STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID
        );
        assert_eq!(
            report.status_canary_evidence_source_reason_packet_route,
            "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
        );
        assert_eq!(
            report.status_canary_evidence_source_reason_packet_source_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_decision_reason_count,
            28
        );
        assert_eq!(
            report.status_canary_evidence_source_decision_reason_ready_count,
            28
        );
        assert_eq!(
            report.status_canary_evidence_source_decision_required_field_count,
            84
        );
        assert_eq!(
            report.status_canary_evidence_source_missing_required_field_reason_count,
            84
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_input_missing_reason_count,
            28
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_input_other_decision_reason_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_adapter_rejection_reason_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_fixture_generation_allowed_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_fixture_generation_blocked_count,
            28
        );
        assert!(report.status_canary_evidence_source_readback_ready);
        assert_eq!(
            report.status_canary_evidence_source_readback_id,
            STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_route,
            "status_canary_evidence_source_readback_ready_no_fixtures"
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_fixture_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_observation_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_missing_observation_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_contract_audit_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_contract_audit_ready_count,
            7
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_fixture_contract_audit_ready_count,
            0
        );
        assert!(report.status_canary_evidence_source_readback_reason_packet_bound);
        assert!(report.status_canary_evidence_source_readback_reason_packet_ready);
        assert_eq!(
            report.status_canary_evidence_source_readback_reason_packet_route,
            "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_fixture_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_fixture_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_readback_fixture_reason_audit_rejected_count,
            0
        );
        assert!(report.status_canary_evidence_source_validator_ready);
        assert_eq!(
            report.status_canary_evidence_source_validator_id,
            STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_route,
            "status_canary_evidence_source_validator_ready_no_observations"
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_contract_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_contract_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_contract_audit_rejected_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_evidence_source_validator_reason_audit_rejected_count,
            0
        );
        assert_eq!(report.status_canary_evidence_source_observation_count, 0);
        assert_eq!(report.status_canary_evidence_source_missing_count, 7);
        assert_eq!(report.status_canary_evidence_source_validated_count, 0);
        assert_eq!(report.status_canary_evidence_source_rejected_count, 0);
        assert_eq!(
            report.status_canary_evidence_source_generated_request_count,
            0
        );
        assert!(report.status_canary_start_guard_ready);
        assert_eq!(
            report.status_canary_start_guard_id,
            "status-canary-start-guard/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_start_guard_route,
            "status_canary_start_blocked_missing_evidence_packet"
        );
        assert!(!report.status_canary_start_guard_switch_enabled);
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_count,
            0
        );
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_ready_count,
            0
        );
        assert_eq!(
            report.status_canary_start_guard_evidence_packet_reason_audit_rejected_count,
            0
        );
        assert!(report.status_canary_start_guard_evidence_packet_reason_audit_ready);
        assert!(report.status_canary_start_guard_blocked);
        assert!(!report.status_canary_start_guard_allowed);
        assert!(report.status_canary_start_request_gate_ready);
        assert_eq!(
            report.status_canary_start_request_gate_id,
            "status-canary-start-request-gate/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_start_request_gate_route,
            "status_canary_start_request_blocked_no_request"
        );
        assert!(!report.status_canary_start_request_present);
        assert_eq!(
            report.status_canary_start_request_requested_tool_id,
            "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
        );
        assert!(report.status_canary_start_request_selected_status_canary);
        assert!(!report.status_canary_start_request_preflight_only_connector);
        assert!(report.status_canary_start_request_source_start_guard_reason_audit_ready);
        assert!(report.status_canary_start_request_blocked);
        assert!(!report.status_canary_start_request_allowed);
        assert!(report.status_canary_runner_adapter_ready);
        assert_eq!(
            report.status_canary_runner_adapter_id,
            "status-canary-runner-adapter/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_adapter_route,
            "status_canary_runner_adapter_blocked_no_runner_request"
        );
        assert!(!report.status_canary_runner_adapter_request_present);
        assert!(report.status_canary_runner_adapter_source_gate_bound);
        assert!(report.status_canary_runner_adapter_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_adapter_source_start_request_allowed);
        assert!(report.status_canary_runner_adapter_blocked);
        assert!(!report.status_canary_runner_adapter_allowed);
        assert!(report.status_canary_runner_start_surface_ready);
        assert_eq!(
            report.status_canary_runner_start_surface_id,
            "status-canary-runner-start-surface/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_start_surface_route,
            "status_canary_runner_start_surface_blocked_no_start_request"
        );
        assert!(!report.status_canary_runner_start_request_present);
        assert!(report.status_canary_runner_start_surface_source_adapter_bound);
        assert!(report.status_canary_runner_start_surface_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_start_surface_source_adapter_allowed);
        assert!(report.status_canary_runner_start_surface_blocked);
        assert!(!report.status_canary_runner_start_surface_allowed);
        assert!(report.status_canary_runner_entry_boundary_ready);
        assert_eq!(
            report.status_canary_runner_entry_boundary_id,
            "status-canary-runner-entry-boundary/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_entry_boundary_route,
            "status_canary_runner_entry_boundary_blocked_no_entry_request"
        );
        assert!(!report.status_canary_runner_entry_request_present);
        assert!(report.status_canary_runner_entry_boundary_source_start_surface_bound);
        assert!(report.status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_entry_boundary_source_start_surface_allowed);
        assert!(report.status_canary_runner_entry_boundary_blocked);
        assert!(!report.status_canary_runner_entry_boundary_allowed);
        assert!(report.status_canary_runner_entry_adapter_ready);
        assert_eq!(
            report.status_canary_runner_entry_adapter_id,
            "status-canary-runner-entry-adapter/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_entry_adapter_route,
            "status_canary_runner_entry_adapter_blocked_no_adapter_request"
        );
        assert!(!report.status_canary_runner_entry_adapter_request_present);
        assert!(report.status_canary_runner_entry_adapter_source_boundary_bound);
        assert!(report.status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_entry_adapter_source_boundary_allowed);
        assert!(report.status_canary_runner_entry_adapter_blocked);
        assert!(!report.status_canary_runner_entry_adapter_allowed);
        assert!(report.status_canary_runner_binding_guard_ready);
        assert_eq!(
            report.status_canary_runner_binding_guard_id,
            "status-canary-runner-binding-guard/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_binding_guard_route,
            "status_canary_runner_binding_guard_blocked_no_binding_request"
        );
        assert!(!report.status_canary_runner_binding_request_present);
        assert!(report.status_canary_runner_binding_guard_source_entry_adapter_bound);
        assert!(report.status_canary_runner_binding_guard_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_binding_guard_source_entry_adapter_allowed);
        assert!(report.status_canary_runner_binding_guard_blocked);
        assert!(!report.status_canary_runner_binding_guard_allowed);
        assert!(report.status_canary_runner_dry_run_selector_ready);
        assert_eq!(
            report.status_canary_runner_dry_run_selector_id,
            "status-canary-runner-dry-run-selector/hepta-system-status/v1"
        );
        assert_eq!(
            report.status_canary_runner_dry_run_selector_route,
            "status_canary_runner_dry_run_selector_blocked_no_selector_request"
        );
        assert!(!report.status_canary_runner_dry_run_selector_request_present);
        assert!(report.status_canary_runner_dry_run_selector_source_binding_guard_bound);
        assert!(report.status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready);
        assert!(!report.status_canary_runner_dry_run_selector_source_binding_guard_allowed);
        assert!(report.status_canary_runner_dry_run_selector_blocked);
        assert!(!report.status_canary_runner_dry_run_selector_allowed);
        assert_eq!(report.status_canary_evidence_closure_entry_count, 7);
        assert_eq!(report.status_canary_evidence_closure_ready_count, 7);
        assert_eq!(report.status_canary_evidence_closure_missing_count, 7);
        assert_eq!(report.status_canary_evidence_closure_recorded_count, 0);
        assert_eq!(report.status_canary_evidence_closure_waived_count, 0);
        assert_eq!(
            report.status_canary_evidence_closure_actionable_precondition_count,
            7
        );
        assert_eq!(
            report.capability_row_count,
            current_reality_capability_registry_count()
        );
        assert_eq!(report.live_enabled_count, 0);
        assert!(report.all_live_paths_blocked);
        assert_eq!(report.blocker_entry_count, 7);
        assert_eq!(report.operator_visible_blocker_count, 7);
        assert_eq!(report.missing_evidence_blocker_count, 7);
        assert_eq!(report.accepted_blocker_count, 0);
        assert_eq!(report.waived_blocker_count, 0);
        assert!(report.dashboard_ready);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn dashboard_keeps_every_blocker_queryable_without_acceptance() {
        let report = controlled_live_operator_readiness_dashboard_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "kill_switch_rehearsal_missing"
            && entry.dashboard_route
                == "readback://controlled-live/operator-dashboard/kill-switch-rehearsal-missing"));
        assert!(report.entries.iter().all(|entry| entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.evidence_state == "missing"
            && entry.operator_status == "blocked_missing_evidence"
            && !entry.acceptance_allowed
            && !entry.waiver_allowed
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.transport_mutation_allowed
            && !entry.persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn dashboard_projects_status_canary_evidence_closure_actions() {
        let report = controlled_live_operator_readiness_dashboard_report();

        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .all(|entry| entry.selected_status_canary_tool_id
                    == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
                    && entry.preflight_only_connector_tool_id
                        == "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
                    && entry.operator_visible
                    && entry.action_required
                    && entry.canary_start_blocked
                    && entry.evidence_state == "missing"
                    && !entry.evidence_recorded
                    && !entry.evidence_waived
                    && !entry.evidence_expired
                    && !entry.evidence_invalid
                    && !entry.evidence_recording_allowed
                    && !entry.waiver_allowed
                    && !entry.credential_read_allowed
                    && !entry.transport_mutation_allowed
                    && !entry.persistence_allowed
                    && !entry.live_mutation_allowed)
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(|entry| entry.source_blocker_id == "dirty_worktree_boundary"
                    && entry.action_kind == "clean_worktree_snapshot_required")
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(
                    |entry| entry.source_blocker_id == "operator_live_approval_missing"
                        && entry.action_kind == "operator_live_approval_packet_required"
                )
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(
                    |entry| entry.source_blocker_id == "fresh_soak_readback_missing"
                        && entry.action_kind == "fresh_status_canary_soak_readback_required"
                )
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(
                    |entry| entry.source_blocker_id == "credential_boundary_attestation_missing"
                        && entry.action_kind == "credential_boundary_attestation_required"
                )
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(|entry| entry.source_blocker_id
                    == "gateway_native_telegram_post_boundary_approval_missing"
                    && entry.action_kind == "transport_boundary_approval_required")
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(
                    |entry| entry.source_blocker_id == "rollback_rehearsal_missing"
                        && entry.action_kind == "rollback_rehearsal_packet_required"
                )
        );
        assert!(
            report
                .status_canary_evidence_closure_entries
                .iter()
                .any(
                    |entry| entry.source_blocker_id == "kill_switch_rehearsal_missing"
                        && entry.action_kind == "kill_switch_rehearsal_packet_required"
                )
        );
    }

    #[test]
    fn dashboard_keeps_side_effects_closed() {
        let report = controlled_live_operator_readiness_dashboard_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveOperatorReadinessDashboardSideEffects::none()
        );
    }
}
