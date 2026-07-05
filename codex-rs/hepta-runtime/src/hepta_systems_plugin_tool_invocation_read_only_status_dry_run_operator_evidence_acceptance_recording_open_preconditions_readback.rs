use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingBoundaryReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback";

const ACCEPTANCE_RECORDING_OPEN_PRECONDITION_ITEMS: [&str; 11] = [
    "evidence_artifact_presence_required",
    "operator_identity_required",
    "acceptance_recording_persistence_required",
    "ledger_persistence_required",
    "receipt_persistence_required",
    "tool_registry_registration_required",
    "registry_lookup_execution_required",
    "read_only_tool_invocation_required",
    "connector_start_boundary_required",
    "runtime_write_boundary_required",
    "live_execution_boundary_required",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_acceptance_recording_boundary_readback_ready: bool,
    pub candidate_count: usize,
    pub precondition_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub acceptance_recording_open_precondition_set_projected_count: usize,
    pub evidence_artifact_presence_precondition_projected_count: usize,
    pub operator_identity_precondition_projected_count: usize,
    pub acceptance_recording_persistence_precondition_projected_count: usize,
    pub ledger_persistence_precondition_projected_count: usize,
    pub receipt_persistence_precondition_projected_count: usize,
    pub tool_registry_registration_precondition_projected_count: usize,
    pub registry_lookup_precondition_projected_count: usize,
    pub tool_invocation_precondition_projected_count: usize,
    pub connector_start_precondition_projected_count: usize,
    pub runtime_write_precondition_projected_count: usize,
    pub live_execution_precondition_projected_count: usize,
    pub acceptance_recording_open_precondition_item_count: usize,
    pub acceptance_recording_open_denial_receipt_projected_count: usize,
    pub acceptance_recording_open_idempotency_key_projected_count: usize,
    pub stable_acceptance_recording_open_precondition_set_count: usize,
    pub unique_acceptance_recording_open_precondition_set_count: usize,
    pub stable_acceptance_recording_open_denial_receipt_count: usize,
    pub unique_acceptance_recording_open_denial_receipt_count: usize,
    pub stable_acceptance_recording_open_idempotency_key_count: usize,
    pub unique_acceptance_recording_open_idempotency_key_count: usize,
    pub acceptance_recording_open_precondition_mismatch_count: usize,
    pub duplicate_acceptance_recording_open_precondition_count: usize,
    pub acceptance_recording_open_denial_receipt_mismatch_count: usize,
    pub duplicate_acceptance_recording_open_denial_receipt_count: usize,
    pub acceptance_recording_open_idempotency_mismatch_count: usize,
    pub duplicate_acceptance_recording_open_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_evidence_packet_sent_count: usize,
    pub operator_evidence_packet_persisted_count: usize,
    pub operator_evidence_recorded_count: usize,
    pub operator_acceptance_recorded_count: usize,
    pub acceptance_record_persisted_count: usize,
    pub non_recording_denial_receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub receipt_persisted_count: usize,
    pub tool_registered_count: usize,
    pub registry_lookup_executed_count: usize,
    pub tool_invoked_count: usize,
    pub mcp_server_started_count: usize,
    pub app_connector_started_count: usize,
    pub runtime_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_execution_started_count: usize,
    pub acceptance_recording_open_preconditions_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_evidence_packet_send_allowed: bool,
    pub operator_evidence_packet_persistence_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub acceptance_record_persistence_allowed: bool,
    pub non_recording_denial_receipt_persistence_allowed: bool,
    pub ledger_persistence_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub connector_start_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackEntry
{
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_acceptance_recording_boundary_id: &'static str,
    pub source_evidence_artifact_ref_link_id: &'static str,
    pub source_operator_identity_link_id: &'static str,
    pub source_acceptance_record_prerequisite_id: &'static str,
    pub source_non_recording_denial_receipt_id: &'static str,
    pub source_ledger_persistence_closure_anchor_id: &'static str,
    pub source_receipt_persistence_closure_anchor_id: &'static str,
    pub source_tool_invocation_closure_anchor_id: &'static str,
    pub source_runtime_write_closure_anchor_id: &'static str,
    pub source_live_execution_closure_anchor_id: &'static str,
    pub source_acceptance_recording_boundary_idempotency_key: &'static str,
    pub acceptance_recording_open_precondition_set_id: &'static str,
    pub evidence_artifact_presence_precondition_id: &'static str,
    pub operator_identity_precondition_id: &'static str,
    pub acceptance_recording_persistence_precondition_id: &'static str,
    pub ledger_persistence_precondition_id: &'static str,
    pub receipt_persistence_precondition_id: &'static str,
    pub tool_registry_registration_precondition_id: &'static str,
    pub registry_lookup_precondition_id: &'static str,
    pub tool_invocation_precondition_id: &'static str,
    pub connector_start_precondition_id: &'static str,
    pub runtime_write_precondition_id: &'static str,
    pub live_execution_precondition_id: &'static str,
    pub acceptance_recording_open_precondition_items: Vec<&'static str>,
    pub acceptance_recording_open_denial_receipt_id: &'static str,
    pub acceptance_recording_open_idempotency_key: &'static str,
    pub first_acceptance_recording_open_precondition_set_id: &'static str,
    pub second_acceptance_recording_open_precondition_set_id: &'static str,
    pub first_acceptance_recording_open_denial_receipt_id: &'static str,
    pub second_acceptance_recording_open_denial_receipt_id: &'static str,
    pub first_acceptance_recording_open_idempotency_key: &'static str,
    pub second_acceptance_recording_open_idempotency_key: &'static str,
    pub acceptance_recording_open_precondition_set_projected: bool,
    pub evidence_artifact_presence_precondition_projected: bool,
    pub operator_identity_precondition_projected: bool,
    pub acceptance_recording_persistence_precondition_projected: bool,
    pub ledger_persistence_precondition_projected: bool,
    pub receipt_persistence_precondition_projected: bool,
    pub tool_registry_registration_precondition_projected: bool,
    pub registry_lookup_precondition_projected: bool,
    pub tool_invocation_precondition_projected: bool,
    pub connector_start_precondition_projected: bool,
    pub runtime_write_precondition_projected: bool,
    pub live_execution_precondition_projected: bool,
    pub acceptance_recording_open_denial_receipt_projected: bool,
    pub acceptance_recording_open_idempotency_key_projected: bool,
    pub stable_acceptance_recording_open_precondition_set: bool,
    pub unique_acceptance_recording_open_precondition_set: bool,
    pub stable_acceptance_recording_open_denial_receipt: bool,
    pub unique_acceptance_recording_open_denial_receipt: bool,
    pub stable_acceptance_recording_open_idempotency_key: bool,
    pub unique_acceptance_recording_open_idempotency_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub ledger_written: bool,
    pub receipt_persisted: bool,
    pub tool_registered: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub connector_started: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub credential_read: bool,
    pub external_network_used: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub telegram_transport_mutated: bool,
    pub package_or_release_written: bool,
    pub live_execution_started: bool,
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report()
-> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport{
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingBoundaryReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport{
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_entries(source);
    let precondition_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let acceptance_recording_open_precondition_set_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_open_precondition_set_projected)
        .count();
    let evidence_artifact_presence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.evidence_artifact_presence_precondition_projected)
        .count();
    let operator_identity_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_identity_precondition_projected)
        .count();
    let acceptance_recording_persistence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_persistence_precondition_projected)
        .count();
    let ledger_persistence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_persistence_precondition_projected)
        .count();
    let receipt_persistence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_persistence_precondition_projected)
        .count();
    let tool_registry_registration_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_registry_registration_precondition_projected)
        .count();
    let registry_lookup_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_precondition_projected)
        .count();
    let tool_invocation_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_invocation_precondition_projected)
        .count();
    let connector_start_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.connector_start_precondition_projected)
        .count();
    let runtime_write_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.runtime_write_precondition_projected)
        .count();
    let live_execution_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.live_execution_precondition_projected)
        .count();
    let acceptance_recording_open_precondition_item_count = entries
        .iter()
        .map(|entry| entry.acceptance_recording_open_precondition_items.len())
        .sum();
    let acceptance_recording_open_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_open_denial_receipt_projected)
        .count();
    let acceptance_recording_open_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_open_idempotency_key_projected)
        .count();
    let stable_acceptance_recording_open_precondition_set_count = entries
        .iter()
        .filter(|entry| entry.stable_acceptance_recording_open_precondition_set)
        .count();
    let unique_acceptance_recording_open_precondition_set_count = entries
        .iter()
        .map(|entry| entry.first_acceptance_recording_open_precondition_set_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_acceptance_recording_open_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_acceptance_recording_open_denial_receipt)
        .count();
    let unique_acceptance_recording_open_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_acceptance_recording_open_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_acceptance_recording_open_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_acceptance_recording_open_idempotency_key)
        .count();
    let unique_acceptance_recording_open_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_acceptance_recording_open_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let acceptance_recording_open_precondition_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_acceptance_recording_open_precondition_set)
        .count();
    let duplicate_acceptance_recording_open_precondition_count = precondition_entry_count
        .saturating_sub(unique_acceptance_recording_open_precondition_set_count);
    let acceptance_recording_open_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_acceptance_recording_open_denial_receipt)
        .count();
    let duplicate_acceptance_recording_open_denial_receipt_count = precondition_entry_count
        .saturating_sub(unique_acceptance_recording_open_denial_receipt_count);
    let acceptance_recording_open_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_acceptance_recording_open_idempotency_key)
        .count();
    let duplicate_acceptance_recording_open_idempotency_key_count = precondition_entry_count
        .saturating_sub(unique_acceptance_recording_open_idempotency_key_count);
    let feature_gate_opened_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_opened)
        .count();
    let dry_run_executed_count = entries
        .iter()
        .filter(|entry| entry.dry_run_executed)
        .count();
    let operator_evidence_packet_sent_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_packet_sent)
        .count();
    let operator_evidence_packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_packet_persisted)
        .count();
    let operator_evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_recorded)
        .count();
    let operator_acceptance_recorded_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_recorded)
        .count();
    let acceptance_record_persisted_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_persisted)
        .count();
    let non_recording_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_recording_denial_receipt_persisted)
        .count();
    let ledger_written_count = entries.iter().filter(|entry| entry.ledger_written).count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted)
        .count();
    let tool_registered_count = entries.iter().filter(|entry| entry.tool_registered).count();
    let registry_lookup_executed_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_executed)
        .count();
    let tool_invoked_count = entries.iter().filter(|entry| entry.tool_invoked).count();
    let mcp_server_started_count = entries
        .iter()
        .filter(|entry| entry.mcp_server_started)
        .count();
    let app_connector_started_count = entries
        .iter()
        .filter(|entry| entry.app_connector_started)
        .count();
    let runtime_event_log_written_count = entries
        .iter()
        .filter(|entry| entry.runtime_event_log_written)
        .count();
    let sqlite_written_count = entries.iter().filter(|entry| entry.sqlite_written).count();
    let live_execution_started_count = entries
        .iter()
        .filter(|entry| entry.live_execution_started)
        .count();

    let acceptance_recording_open_preconditions_readback_ready = source
        .acceptance_recording_boundary_readback_ready
        && source.candidate_count == 2
        && source.boundary_entry_count == 2
        && source.selected_read_only_status_tool_count == 1
        && source.non_selected_preflight_boundary_count == 1
        && source.acceptance_recording_boundary_id_projected_count == 2
        && source.evidence_artifact_ref_link_projected_count == 2
        && source.operator_identity_link_projected_count == 2
        && source.acceptance_record_prerequisite_projected_count == 2
        && source.non_recording_denial_receipt_projected_count == 2
        && source.ledger_persistence_closure_anchor_projected_count == 2
        && source.receipt_persistence_closure_anchor_projected_count == 2
        && source.tool_invocation_closure_anchor_projected_count == 2
        && source.runtime_write_closure_anchor_projected_count == 2
        && source.live_execution_closure_anchor_projected_count == 2
        && source.acceptance_recording_boundary_idempotency_key_projected_count == 2
        && source.feature_gate_opened_count == 0
        && source.dry_run_executed_count == 0
        && source.operator_evidence_packet_sent_count == 0
        && source.operator_evidence_packet_persisted_count == 0
        && source.operator_evidence_recorded_count == 0
        && source.operator_acceptance_recorded_count == 0
        && source.acceptance_record_persisted_count == 0
        && source.non_recording_denial_receipt_persisted_count == 0
        && source.ledger_written_count == 0
        && source.receipt_persisted_count == 0
        && source.tool_registered_count == 0
        && source.registry_lookup_executed_count == 0
        && source.tool_invoked_count == 0
        && source.live_execution_started_count == 0
        && precondition_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && acceptance_recording_open_precondition_set_projected_count == 2
        && evidence_artifact_presence_precondition_projected_count == 2
        && operator_identity_precondition_projected_count == 2
        && acceptance_recording_persistence_precondition_projected_count == 2
        && ledger_persistence_precondition_projected_count == 2
        && receipt_persistence_precondition_projected_count == 2
        && tool_registry_registration_precondition_projected_count == 2
        && registry_lookup_precondition_projected_count == 2
        && tool_invocation_precondition_projected_count == 2
        && connector_start_precondition_projected_count == 2
        && runtime_write_precondition_projected_count == 2
        && live_execution_precondition_projected_count == 2
        && acceptance_recording_open_precondition_item_count == 22
        && acceptance_recording_open_denial_receipt_projected_count == 2
        && acceptance_recording_open_idempotency_key_projected_count == 2
        && stable_acceptance_recording_open_precondition_set_count == 2
        && unique_acceptance_recording_open_precondition_set_count == 2
        && stable_acceptance_recording_open_denial_receipt_count == 2
        && unique_acceptance_recording_open_denial_receipt_count == 2
        && stable_acceptance_recording_open_idempotency_key_count == 2
        && unique_acceptance_recording_open_idempotency_key_count == 2
        && acceptance_recording_open_precondition_mismatch_count == 0
        && duplicate_acceptance_recording_open_precondition_count == 0
        && acceptance_recording_open_denial_receipt_mismatch_count == 0
        && duplicate_acceptance_recording_open_denial_receipt_count == 0
        && acceptance_recording_open_idempotency_mismatch_count == 0
        && duplicate_acceptance_recording_open_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_evidence_packet_sent_count == 0
        && operator_evidence_packet_persisted_count == 0
        && operator_evidence_recorded_count == 0
        && operator_acceptance_recorded_count == 0
        && acceptance_record_persisted_count == 0
        && non_recording_denial_receipt_persisted_count == 0
        && ledger_written_count == 0
        && receipt_persisted_count == 0
        && tool_registered_count == 0
        && registry_lookup_executed_count == 0
        && tool_invoked_count == 0
        && mcp_server_started_count == 0
        && app_connector_started_count == 0
        && runtime_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_execution_started_count == 0;

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback",
        status: if acceptance_recording_open_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_acceptance_recording_boundary_readback_ready: source.acceptance_recording_boundary_readback_ready,
        candidate_count: source.candidate_count,
        precondition_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        acceptance_recording_open_precondition_set_projected_count,
        evidence_artifact_presence_precondition_projected_count,
        operator_identity_precondition_projected_count,
        acceptance_recording_persistence_precondition_projected_count,
        ledger_persistence_precondition_projected_count,
        receipt_persistence_precondition_projected_count,
        tool_registry_registration_precondition_projected_count,
        registry_lookup_precondition_projected_count,
        tool_invocation_precondition_projected_count,
        connector_start_precondition_projected_count,
        runtime_write_precondition_projected_count,
        live_execution_precondition_projected_count,
        acceptance_recording_open_precondition_item_count,
        acceptance_recording_open_denial_receipt_projected_count,
        acceptance_recording_open_idempotency_key_projected_count,
        stable_acceptance_recording_open_precondition_set_count,
        unique_acceptance_recording_open_precondition_set_count,
        stable_acceptance_recording_open_denial_receipt_count,
        unique_acceptance_recording_open_denial_receipt_count,
        stable_acceptance_recording_open_idempotency_key_count,
        unique_acceptance_recording_open_idempotency_key_count,
        acceptance_recording_open_precondition_mismatch_count,
        duplicate_acceptance_recording_open_precondition_count,
        acceptance_recording_open_denial_receipt_mismatch_count,
        duplicate_acceptance_recording_open_denial_receipt_count,
        acceptance_recording_open_idempotency_mismatch_count,
        duplicate_acceptance_recording_open_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_evidence_packet_sent_count,
        operator_evidence_packet_persisted_count,
        operator_evidence_recorded_count,
        operator_acceptance_recorded_count,
        acceptance_record_persisted_count,
        non_recording_denial_receipt_persisted_count,
        ledger_written_count,
        receipt_persisted_count,
        tool_registered_count,
        registry_lookup_executed_count,
        tool_invoked_count,
        mcp_server_started_count,
        app_connector_started_count,
        runtime_event_log_written_count,
        sqlite_written_count,
        live_execution_started_count,
        acceptance_recording_open_preconditions_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_evidence_packet_send_allowed: false,
        operator_evidence_packet_persistence_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
        acceptance_record_persistence_allowed: false,
        non_recording_denial_receipt_persistence_allowed: false,
        ledger_persistence_allowed: false,
        receipt_persistence_allowed: false,
        tool_registry_registration_allowed: false,
        registry_lookup_execution_allowed: false,
        tool_invocation_allowed: false,
        connector_start_allowed: false,
        runtime_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "evidence_artifact_missing",
            "operator_identity_missing",
            "operator_acceptance_recording_disabled",
            "acceptance_record_persistence_disabled",
            "ledger_persistence_disabled",
            "receipt_persistence_disabled",
            "tool_registry_registration_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "connector_start_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingBoundaryReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackEntry>{
    source
        .entries
        .iter()
        .map(|entry| {
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_acceptance_recording_boundary_id: entry.acceptance_recording_boundary_id,
                source_evidence_artifact_ref_link_id: entry.evidence_artifact_ref_link_id,
                source_operator_identity_link_id: entry.operator_identity_link_id,
                source_acceptance_record_prerequisite_id: entry.acceptance_record_prerequisite_id,
                source_non_recording_denial_receipt_id: entry.non_recording_denial_receipt_id,
                source_ledger_persistence_closure_anchor_id: entry
                    .ledger_persistence_closure_anchor_id,
                source_receipt_persistence_closure_anchor_id: entry
                    .receipt_persistence_closure_anchor_id,
                source_tool_invocation_closure_anchor_id: entry
                    .tool_invocation_closure_anchor_id,
                source_runtime_write_closure_anchor_id: entry.runtime_write_closure_anchor_id,
                source_live_execution_closure_anchor_id: entry.live_execution_closure_anchor_id,
                source_acceptance_recording_boundary_idempotency_key: entry
                    .acceptance_recording_boundary_idempotency_key,
                acceptance_recording_open_precondition_set_id:
                    acceptance_recording_open_precondition_set_id(entry.contribution_kind),
                evidence_artifact_presence_precondition_id:
                    acceptance_recording_open_precondition_id(
                        "evidence-artifact-presence",
                        entry.contribution_kind,
                    ),
                operator_identity_precondition_id: acceptance_recording_open_precondition_id(
                    "operator-identity",
                    entry.contribution_kind,
                ),
                acceptance_recording_persistence_precondition_id:
                    acceptance_recording_open_precondition_id(
                        "acceptance-recording-persistence",
                        entry.contribution_kind,
                    ),
                ledger_persistence_precondition_id: acceptance_recording_open_precondition_id(
                    "ledger-persistence",
                    entry.contribution_kind,
                ),
                receipt_persistence_precondition_id: acceptance_recording_open_precondition_id(
                    "receipt-persistence",
                    entry.contribution_kind,
                ),
                tool_registry_registration_precondition_id:
                    acceptance_recording_open_precondition_id(
                        "tool-registry-registration",
                        entry.contribution_kind,
                    ),
                registry_lookup_precondition_id: acceptance_recording_open_precondition_id(
                    "registry-lookup",
                    entry.contribution_kind,
                ),
                tool_invocation_precondition_id: acceptance_recording_open_precondition_id(
                    "tool-invocation",
                    entry.contribution_kind,
                ),
                connector_start_precondition_id: acceptance_recording_open_precondition_id(
                    "connector-start",
                    entry.contribution_kind,
                ),
                runtime_write_precondition_id: acceptance_recording_open_precondition_id(
                    "runtime-write-boundary",
                    entry.contribution_kind,
                ),
                live_execution_precondition_id: acceptance_recording_open_precondition_id(
                    "live-execution-boundary",
                    entry.contribution_kind,
                ),
                acceptance_recording_open_precondition_items:
                    ACCEPTANCE_RECORDING_OPEN_PRECONDITION_ITEMS.to_vec(),
                acceptance_recording_open_denial_receipt_id:
                    acceptance_recording_open_denial_receipt_id(entry.contribution_kind),
                acceptance_recording_open_idempotency_key:
                    acceptance_recording_open_idempotency_key(entry.contribution_kind),
                first_acceptance_recording_open_precondition_set_id:
                    acceptance_recording_open_precondition_set_id(entry.contribution_kind),
                second_acceptance_recording_open_precondition_set_id:
                    acceptance_recording_open_precondition_set_id(entry.contribution_kind),
                first_acceptance_recording_open_denial_receipt_id:
                    acceptance_recording_open_denial_receipt_id(entry.contribution_kind),
                second_acceptance_recording_open_denial_receipt_id:
                    acceptance_recording_open_denial_receipt_id(entry.contribution_kind),
                first_acceptance_recording_open_idempotency_key:
                    acceptance_recording_open_idempotency_key(entry.contribution_kind),
                second_acceptance_recording_open_idempotency_key:
                    acceptance_recording_open_idempotency_key(entry.contribution_kind),
                acceptance_recording_open_precondition_set_projected: true,
                evidence_artifact_presence_precondition_projected: true,
                operator_identity_precondition_projected: true,
                acceptance_recording_persistence_precondition_projected: true,
                ledger_persistence_precondition_projected: true,
                receipt_persistence_precondition_projected: true,
                tool_registry_registration_precondition_projected: true,
                registry_lookup_precondition_projected: true,
                tool_invocation_precondition_projected: true,
                connector_start_precondition_projected: true,
                runtime_write_precondition_projected: true,
                live_execution_precondition_projected: true,
                acceptance_recording_open_denial_receipt_projected: true,
                acceptance_recording_open_idempotency_key_projected: true,
                stable_acceptance_recording_open_precondition_set: true,
                unique_acceptance_recording_open_precondition_set: true,
                stable_acceptance_recording_open_denial_receipt: true,
                unique_acceptance_recording_open_denial_receipt: true,
                stable_acceptance_recording_open_idempotency_key: true,
                unique_acceptance_recording_open_idempotency_key: true,
                feature_gate_opened: entry.feature_gate_opened,
                dry_run_executed: entry.dry_run_executed,
                operator_evidence_packet_sent: entry.operator_evidence_packet_sent,
                operator_evidence_packet_persisted: entry.operator_evidence_packet_persisted,
                operator_evidence_recorded: entry.operator_evidence_recorded,
                operator_acceptance_recorded: entry.operator_acceptance_recorded,
                acceptance_record_persisted: entry.acceptance_record_persisted,
                non_recording_denial_receipt_persisted: entry
                    .non_recording_denial_receipt_persisted,
                ledger_written: entry.ledger_written,
                receipt_persisted: entry.receipt_persisted,
                tool_registered: entry.tool_registered,
                registry_lookup_executed: entry.registry_lookup_executed,
                tool_invoked: entry.tool_invoked,
                mcp_server_started: entry.mcp_server_started,
                app_connector_started: entry.app_connector_started,
                runtime_event_log_written: entry.runtime_event_log_written,
                sqlite_written: entry.sqlite_written,
                live_execution_started: entry.live_execution_started,
            }
        })
        .collect()
}

fn suffix(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "local-mcp:read-only-status-dry-run",
        "app_connector" => "local-app:not-selected",
        _ => "unknown:not-selected",
    }
}

fn acceptance_recording_open_precondition_set_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-open-preconditions:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-open-preconditions:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-open-preconditions:hepta-system:unknown:not-selected"
        }
    }
}

fn acceptance_recording_open_precondition_id(name: &str, contribution_kind: &str) -> &'static str {
    match (name, suffix(contribution_kind)) {
        ("evidence-artifact-presence", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:evidence-artifact-presence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("evidence-artifact-presence", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:evidence-artifact-presence:hepta-system:local-app:not-selected"
        }
        ("operator-identity", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:operator-identity:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("operator-identity", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:operator-identity:hepta-system:local-app:not-selected"
        }
        ("acceptance-recording-persistence", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:acceptance-recording-persistence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("acceptance-recording-persistence", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:acceptance-recording-persistence:hepta-system:local-app:not-selected"
        }
        ("ledger-persistence", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:ledger-persistence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("ledger-persistence", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:ledger-persistence:hepta-system:local-app:not-selected"
        }
        ("receipt-persistence", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:receipt-persistence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("receipt-persistence", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:receipt-persistence:hepta-system:local-app:not-selected"
        }
        ("tool-registry-registration", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:tool-registry-registration:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("tool-registry-registration", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:tool-registry-registration:hepta-system:local-app:not-selected"
        }
        ("registry-lookup", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:registry-lookup:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("registry-lookup", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:registry-lookup:hepta-system:local-app:not-selected"
        }
        ("tool-invocation", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:tool-invocation:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("tool-invocation", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:tool-invocation:hepta-system:local-app:not-selected"
        }
        ("connector-start", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:connector-start:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("connector-start", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:connector-start:hepta-system:local-app:not-selected"
        }
        ("runtime-write-boundary", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:runtime-write-boundary:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("runtime-write-boundary", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:runtime-write-boundary:hepta-system:local-app:not-selected"
        }
        ("live-execution-boundary", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-open-precondition:live-execution-boundary:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("live-execution-boundary", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-open-precondition:live-execution-boundary:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-open-precondition:unknown:hepta-system:unknown:not-selected"
        }
    }
}

fn acceptance_recording_open_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-open-denial:hepta-system:local-mcp:read-only-status-dry-run:missing-evidence-identity-acceptance-ledger-receipt-registration"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-open-denial:hepta-system:local-app:not-selected:missing-evidence-identity-acceptance-ledger-receipt-registration"
        }
        _ => {
            "operator-evidence-acceptance-recording-open-denial:hepta-system:unknown:not-selected:missing-evidence-identity-acceptance-ledger-receipt-registration"
        }
    }
}

fn acceptance_recording_open_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-open-idempotency:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-open-idempotency:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-open-idempotency:hepta-system:unknown:not-selected"
        }
    }
}

impl HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_gate_opened: false,
            dry_run_executed: false,
            operator_evidence_packet_sent: false,
            operator_evidence_packet_persisted: false,
            operator_evidence_recorded: false,
            operator_acceptance_recorded: false,
            acceptance_record_persisted: false,
            non_recording_denial_receipt_persisted: false,
            ledger_persisted: false,
            receipt_persisted: false,
            tool_registered: false,
            tool_registry_mutated: false,
            registry_lookup_executed: false,
            tool_invoked: false,
            connector_started: false,
            runtime_event_log_written: false,
            sqlite_written: false,
            credential_read: false,
            external_network_used: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            telegram_transport_mutated: false,
            package_or_release_written: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_evidence_acceptance_open_preconditions_project_required_items() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_acceptance_recording_boundary_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.precondition_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(
            report.acceptance_recording_open_precondition_set_projected_count,
            2
        );
        assert_eq!(
            report.evidence_artifact_presence_precondition_projected_count,
            2
        );
        assert_eq!(report.operator_identity_precondition_projected_count, 2);
        assert_eq!(
            report.acceptance_recording_persistence_precondition_projected_count,
            2
        );
        assert_eq!(report.ledger_persistence_precondition_projected_count, 2);
        assert_eq!(report.receipt_persistence_precondition_projected_count, 2);
        assert_eq!(
            report.tool_registry_registration_precondition_projected_count,
            2
        );
        assert_eq!(report.registry_lookup_precondition_projected_count, 2);
        assert_eq!(report.tool_invocation_precondition_projected_count, 2);
        assert_eq!(report.connector_start_precondition_projected_count, 2);
        assert_eq!(report.runtime_write_precondition_projected_count, 2);
        assert_eq!(report.live_execution_precondition_projected_count, 2);
        assert_eq!(report.acceptance_recording_open_precondition_item_count, 22);
        assert!(report.acceptance_recording_open_preconditions_readback_ready);
    }

    #[test]
    fn operator_evidence_acceptance_open_preconditions_stay_stable_unique_and_denied() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report();

        assert_eq!(
            report.acceptance_recording_open_denial_receipt_projected_count,
            2
        );
        assert_eq!(
            report.acceptance_recording_open_idempotency_key_projected_count,
            2
        );
        assert_eq!(
            report.stable_acceptance_recording_open_precondition_set_count,
            2
        );
        assert_eq!(
            report.unique_acceptance_recording_open_precondition_set_count,
            2
        );
        assert_eq!(
            report.stable_acceptance_recording_open_denial_receipt_count,
            2
        );
        assert_eq!(
            report.unique_acceptance_recording_open_denial_receipt_count,
            2
        );
        assert_eq!(
            report.stable_acceptance_recording_open_idempotency_key_count,
            2
        );
        assert_eq!(
            report.unique_acceptance_recording_open_idempotency_key_count,
            2
        );
        assert_eq!(
            report.acceptance_recording_open_precondition_mismatch_count,
            0
        );
        assert_eq!(
            report.duplicate_acceptance_recording_open_precondition_count,
            0
        );
        assert_eq!(
            report.acceptance_recording_open_denial_receipt_mismatch_count,
            0
        );
        assert_eq!(
            report.duplicate_acceptance_recording_open_denial_receipt_count,
            0
        );
        assert_eq!(
            report.acceptance_recording_open_idempotency_mismatch_count,
            0
        );
        assert_eq!(
            report.duplicate_acceptance_recording_open_idempotency_key_count,
            0
        );
    }

    #[test]
    fn operator_evidence_acceptance_open_preconditions_keep_every_execution_path_closed() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
        assert_eq!(report.operator_evidence_packet_sent_count, 0);
        assert_eq!(report.operator_evidence_packet_persisted_count, 0);
        assert_eq!(report.operator_evidence_recorded_count, 0);
        assert_eq!(report.operator_acceptance_recorded_count, 0);
        assert_eq!(report.acceptance_record_persisted_count, 0);
        assert_eq!(report.non_recording_denial_receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.feature_gate_open_allowed);
        assert!(!report.dry_run_execution_allowed);
        assert!(!report.operator_evidence_packet_send_allowed);
        assert!(!report.operator_evidence_packet_persistence_allowed);
        assert!(!report.operator_evidence_recording_allowed);
        assert!(!report.operator_acceptance_recording_allowed);
        assert!(!report.acceptance_record_persistence_allowed);
        assert!(!report.non_recording_denial_receipt_persistence_allowed);
        assert!(!report.ledger_persistence_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.registry_lookup_execution_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.connector_start_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackSideEffects::none()
        );
    }
}
