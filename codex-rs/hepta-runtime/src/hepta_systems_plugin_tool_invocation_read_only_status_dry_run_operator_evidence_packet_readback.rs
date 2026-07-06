use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunExecutionOpenPreconditionsReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback";

const OPERATOR_EVIDENCE_ITEMS: [&str; 10] = [
    "status_payload_snapshot_required",
    "tool_schema_digest_required",
    "policy_denial_anchor_required",
    "approval_denial_anchor_required",
    "ledger_persistence_prerequisite_required",
    "receipt_persistence_prerequisite_required",
    "tool_registry_registration_prerequisite_required",
    "registry_lookup_invocation_prerequisite_required",
    "connector_runtime_boundary_required",
    "operator_identity_acceptance_recording_required",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_execution_open_preconditions_ready: bool,
    pub candidate_count: usize,
    pub evidence_packet_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub operator_evidence_packet_id_projected_count: usize,
    pub operator_evidence_artifact_ref_projected_count: usize,
    pub operator_evidence_item_count: usize,
    pub acceptance_recording_prerequisite_link_projected_count: usize,
    pub ledger_persistence_prerequisite_link_projected_count: usize,
    pub receipt_persistence_prerequisite_link_projected_count: usize,
    pub tool_registry_registration_prerequisite_link_projected_count: usize,
    pub registry_lookup_prerequisite_link_projected_count: usize,
    pub tool_invocation_prerequisite_link_projected_count: usize,
    pub connector_start_prerequisite_link_projected_count: usize,
    pub runtime_write_prerequisite_link_projected_count: usize,
    pub live_execution_prerequisite_link_projected_count: usize,
    pub evidence_packet_denial_receipt_projected_count: usize,
    pub evidence_packet_idempotency_key_projected_count: usize,
    pub stable_operator_evidence_packet_count: usize,
    pub unique_operator_evidence_packet_count: usize,
    pub stable_evidence_packet_denial_receipt_count: usize,
    pub unique_evidence_packet_denial_receipt_count: usize,
    pub stable_evidence_packet_idempotency_key_count: usize,
    pub unique_evidence_packet_idempotency_key_count: usize,
    pub operator_evidence_packet_mismatch_count: usize,
    pub duplicate_operator_evidence_packet_count: usize,
    pub evidence_packet_denial_receipt_mismatch_count: usize,
    pub duplicate_evidence_packet_denial_receipt_count: usize,
    pub evidence_packet_idempotency_mismatch_count: usize,
    pub duplicate_evidence_packet_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_evidence_packet_sent_count: usize,
    pub operator_evidence_packet_persisted_count: usize,
    pub operator_evidence_recorded_count: usize,
    pub operator_acceptance_recorded_count: usize,
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
    pub operator_evidence_packet_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_evidence_packet_send_allowed: bool,
    pub operator_evidence_packet_persistence_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub ledger_persistence_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub connector_start_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries:
        Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_execution_open_precondition_set_id: &'static str,
    pub source_operator_evidence_precondition_id: &'static str,
    pub source_operator_acceptance_recording_precondition_id: &'static str,
    pub source_ledger_persistence_precondition_id: &'static str,
    pub source_receipt_persistence_precondition_id: &'static str,
    pub source_tool_registry_registration_precondition_id: &'static str,
    pub source_registry_lookup_precondition_id: &'static str,
    pub source_tool_invocation_precondition_id: &'static str,
    pub source_connector_start_precondition_id: &'static str,
    pub source_runtime_write_precondition_id: &'static str,
    pub source_live_execution_precondition_id: &'static str,
    pub source_execution_open_denial_receipt_id: &'static str,
    pub source_execution_open_idempotency_key: &'static str,
    pub operator_evidence_packet_id: &'static str,
    pub operator_evidence_artifact_ref_id: &'static str,
    pub operator_evidence_items: Vec<&'static str>,
    pub acceptance_recording_prerequisite_link_id: &'static str,
    pub ledger_persistence_prerequisite_link_id: &'static str,
    pub receipt_persistence_prerequisite_link_id: &'static str,
    pub tool_registry_registration_prerequisite_link_id: &'static str,
    pub registry_lookup_prerequisite_link_id: &'static str,
    pub tool_invocation_prerequisite_link_id: &'static str,
    pub connector_start_prerequisite_link_id: &'static str,
    pub runtime_write_prerequisite_link_id: &'static str,
    pub live_execution_prerequisite_link_id: &'static str,
    pub evidence_packet_denial_receipt_id: &'static str,
    pub evidence_packet_idempotency_key: &'static str,
    pub first_operator_evidence_packet_id: &'static str,
    pub second_operator_evidence_packet_id: &'static str,
    pub first_evidence_packet_denial_receipt_id: &'static str,
    pub second_evidence_packet_denial_receipt_id: &'static str,
    pub first_evidence_packet_idempotency_key: &'static str,
    pub second_evidence_packet_idempotency_key: &'static str,
    pub operator_evidence_packet_id_projected: bool,
    pub operator_evidence_artifact_ref_projected: bool,
    pub acceptance_recording_prerequisite_link_projected: bool,
    pub ledger_persistence_prerequisite_link_projected: bool,
    pub receipt_persistence_prerequisite_link_projected: bool,
    pub tool_registry_registration_prerequisite_link_projected: bool,
    pub registry_lookup_prerequisite_link_projected: bool,
    pub tool_invocation_prerequisite_link_projected: bool,
    pub connector_start_prerequisite_link_projected: bool,
    pub runtime_write_prerequisite_link_projected: bool,
    pub live_execution_prerequisite_link_projected: bool,
    pub evidence_packet_denial_receipt_projected: bool,
    pub evidence_packet_idempotency_key_projected: bool,
    pub stable_operator_evidence_packet: bool,
    pub unique_operator_evidence_packet: bool,
    pub stable_evidence_packet_denial_receipt: bool,
    pub unique_evidence_packet_denial_receipt: bool,
    pub stable_evidence_packet_idempotency_key: bool,
    pub unique_evidence_packet_idempotency_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
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
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
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

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report()
-> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackReport {
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunExecutionOpenPreconditionsReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackReport {
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_entries(source);
    let evidence_packet_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let operator_evidence_packet_id_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_packet_id_projected)
        .count();
    let operator_evidence_artifact_ref_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_artifact_ref_projected)
        .count();
    let operator_evidence_item_count = entries
        .iter()
        .map(|entry| entry.operator_evidence_items.len())
        .sum();
    let acceptance_recording_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_prerequisite_link_projected)
        .count();
    let ledger_persistence_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_persistence_prerequisite_link_projected)
        .count();
    let receipt_persistence_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_persistence_prerequisite_link_projected)
        .count();
    let tool_registry_registration_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_registry_registration_prerequisite_link_projected)
        .count();
    let registry_lookup_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_prerequisite_link_projected)
        .count();
    let tool_invocation_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_invocation_prerequisite_link_projected)
        .count();
    let connector_start_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.connector_start_prerequisite_link_projected)
        .count();
    let runtime_write_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.runtime_write_prerequisite_link_projected)
        .count();
    let live_execution_prerequisite_link_projected_count = entries
        .iter()
        .filter(|entry| entry.live_execution_prerequisite_link_projected)
        .count();
    let evidence_packet_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.evidence_packet_denial_receipt_projected)
        .count();
    let evidence_packet_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.evidence_packet_idempotency_key_projected)
        .count();
    let stable_operator_evidence_packet_count = entries
        .iter()
        .filter(|entry| entry.stable_operator_evidence_packet)
        .count();
    let unique_operator_evidence_packet_count = entries
        .iter()
        .map(|entry| entry.first_operator_evidence_packet_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_evidence_packet_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_evidence_packet_denial_receipt)
        .count();
    let unique_evidence_packet_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_evidence_packet_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_evidence_packet_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_evidence_packet_idempotency_key)
        .count();
    let unique_evidence_packet_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_evidence_packet_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let operator_evidence_packet_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_operator_evidence_packet)
        .count();
    let duplicate_operator_evidence_packet_count =
        evidence_packet_entry_count.saturating_sub(unique_operator_evidence_packet_count);
    let evidence_packet_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_evidence_packet_denial_receipt)
        .count();
    let duplicate_evidence_packet_denial_receipt_count =
        evidence_packet_entry_count.saturating_sub(unique_evidence_packet_denial_receipt_count);
    let evidence_packet_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_evidence_packet_idempotency_key)
        .count();
    let duplicate_evidence_packet_idempotency_key_count =
        evidence_packet_entry_count.saturating_sub(unique_evidence_packet_idempotency_key_count);
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

    let operator_evidence_packet_readback_ready = source
        .execution_open_preconditions_readback_ready
        && source.candidate_count == 2
        && source.precondition_entry_count == 2
        && source.selected_read_only_status_tool_count == 1
        && source.non_selected_preflight_boundary_count == 1
        && source.execution_open_precondition_set_projected_count == 2
        && source.operator_evidence_precondition_projected_count == 2
        && source.operator_acceptance_recording_precondition_projected_count == 2
        && source.ledger_persistence_precondition_projected_count == 2
        && source.receipt_persistence_precondition_projected_count == 2
        && source.tool_registry_registration_precondition_projected_count == 2
        && source.registry_lookup_precondition_projected_count == 2
        && source.tool_invocation_precondition_projected_count == 2
        && source.connector_start_precondition_projected_count == 2
        && source.runtime_write_precondition_projected_count == 2
        && source.live_execution_precondition_projected_count == 2
        && source.execution_open_denial_receipt_projected_count == 2
        && source.execution_open_idempotency_key_projected_count == 2
        && source.feature_gate_opened_count == 0
        && source.dry_run_executed_count == 0
        && source.operator_acceptance_recorded_count == 0
        && source.ledger_written_count == 0
        && source.receipt_persisted_count == 0
        && source.tool_registered_count == 0
        && source.registry_lookup_executed_count == 0
        && source.tool_invoked_count == 0
        && source.live_execution_started_count == 0
        && evidence_packet_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && operator_evidence_packet_id_projected_count == 2
        && operator_evidence_artifact_ref_projected_count == 2
        && operator_evidence_item_count == 20
        && acceptance_recording_prerequisite_link_projected_count == 2
        && ledger_persistence_prerequisite_link_projected_count == 2
        && receipt_persistence_prerequisite_link_projected_count == 2
        && tool_registry_registration_prerequisite_link_projected_count == 2
        && registry_lookup_prerequisite_link_projected_count == 2
        && tool_invocation_prerequisite_link_projected_count == 2
        && connector_start_prerequisite_link_projected_count == 2
        && runtime_write_prerequisite_link_projected_count == 2
        && live_execution_prerequisite_link_projected_count == 2
        && evidence_packet_denial_receipt_projected_count == 2
        && evidence_packet_idempotency_key_projected_count == 2
        && stable_operator_evidence_packet_count == 2
        && unique_operator_evidence_packet_count == 2
        && stable_evidence_packet_denial_receipt_count == 2
        && unique_evidence_packet_denial_receipt_count == 2
        && stable_evidence_packet_idempotency_key_count == 2
        && unique_evidence_packet_idempotency_key_count == 2
        && operator_evidence_packet_mismatch_count == 0
        && duplicate_operator_evidence_packet_count == 0
        && evidence_packet_denial_receipt_mismatch_count == 0
        && duplicate_evidence_packet_denial_receipt_count == 0
        && evidence_packet_idempotency_mismatch_count == 0
        && duplicate_evidence_packet_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_evidence_packet_sent_count == 0
        && operator_evidence_packet_persisted_count == 0
        && operator_evidence_recorded_count == 0
        && operator_acceptance_recorded_count == 0
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

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback",
        status: if operator_evidence_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_execution_open_preconditions_ready: source
            .execution_open_preconditions_readback_ready,
        candidate_count: source.candidate_count,
        evidence_packet_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        operator_evidence_packet_id_projected_count,
        operator_evidence_artifact_ref_projected_count,
        operator_evidence_item_count,
        acceptance_recording_prerequisite_link_projected_count,
        ledger_persistence_prerequisite_link_projected_count,
        receipt_persistence_prerequisite_link_projected_count,
        tool_registry_registration_prerequisite_link_projected_count,
        registry_lookup_prerequisite_link_projected_count,
        tool_invocation_prerequisite_link_projected_count,
        connector_start_prerequisite_link_projected_count,
        runtime_write_prerequisite_link_projected_count,
        live_execution_prerequisite_link_projected_count,
        evidence_packet_denial_receipt_projected_count,
        evidence_packet_idempotency_key_projected_count,
        stable_operator_evidence_packet_count,
        unique_operator_evidence_packet_count,
        stable_evidence_packet_denial_receipt_count,
        unique_evidence_packet_denial_receipt_count,
        stable_evidence_packet_idempotency_key_count,
        unique_evidence_packet_idempotency_key_count,
        operator_evidence_packet_mismatch_count,
        duplicate_operator_evidence_packet_count,
        evidence_packet_denial_receipt_mismatch_count,
        duplicate_evidence_packet_denial_receipt_count,
        evidence_packet_idempotency_mismatch_count,
        duplicate_evidence_packet_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_evidence_packet_sent_count,
        operator_evidence_packet_persisted_count,
        operator_evidence_recorded_count,
        operator_acceptance_recorded_count,
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
        operator_evidence_packet_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_evidence_packet_send_allowed: false,
        operator_evidence_packet_persistence_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
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
            "operator_evidence_packet_send_disabled",
            "operator_evidence_packet_persistence_disabled",
            "operator_evidence_recording_disabled",
            "operator_acceptance_recording_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunExecutionOpenPreconditionsReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackEntry> {
    source
        .entries
        .iter()
        .map(|entry| {
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_execution_open_precondition_set_id: entry.execution_open_precondition_set_id,
                source_operator_evidence_precondition_id: entry.operator_evidence_precondition_id,
                source_operator_acceptance_recording_precondition_id: entry
                    .operator_acceptance_recording_precondition_id,
                source_ledger_persistence_precondition_id: entry.ledger_persistence_precondition_id,
                source_receipt_persistence_precondition_id: entry
                    .receipt_persistence_precondition_id,
                source_tool_registry_registration_precondition_id: entry
                    .tool_registry_registration_precondition_id,
                source_registry_lookup_precondition_id: entry.registry_lookup_precondition_id,
                source_tool_invocation_precondition_id: entry.tool_invocation_precondition_id,
                source_connector_start_precondition_id: entry.connector_start_precondition_id,
                source_runtime_write_precondition_id: entry.runtime_write_precondition_id,
                source_live_execution_precondition_id: entry.live_execution_precondition_id,
                source_execution_open_denial_receipt_id: entry.execution_open_denial_receipt_id,
                source_execution_open_idempotency_key: entry.execution_open_idempotency_key,
                operator_evidence_packet_id: operator_evidence_packet_id(entry.contribution_kind),
                operator_evidence_artifact_ref_id: operator_evidence_artifact_ref_id(
                    entry.contribution_kind,
                ),
                operator_evidence_items: OPERATOR_EVIDENCE_ITEMS.to_vec(),
                acceptance_recording_prerequisite_link_id:
                    acceptance_recording_prerequisite_link_id(entry.contribution_kind),
                ledger_persistence_prerequisite_link_id: ledger_persistence_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                receipt_persistence_prerequisite_link_id: receipt_persistence_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                tool_registry_registration_prerequisite_link_id:
                    tool_registry_registration_prerequisite_link_id(entry.contribution_kind),
                registry_lookup_prerequisite_link_id: registry_lookup_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                tool_invocation_prerequisite_link_id: tool_invocation_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                connector_start_prerequisite_link_id: connector_start_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                runtime_write_prerequisite_link_id: runtime_write_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                live_execution_prerequisite_link_id: live_execution_prerequisite_link_id(
                    entry.contribution_kind,
                ),
                evidence_packet_denial_receipt_id: evidence_packet_denial_receipt_id(
                    entry.contribution_kind,
                ),
                evidence_packet_idempotency_key: evidence_packet_idempotency_key(
                    entry.contribution_kind,
                ),
                first_operator_evidence_packet_id: operator_evidence_packet_id(
                    entry.contribution_kind,
                ),
                second_operator_evidence_packet_id: operator_evidence_packet_id(
                    entry.contribution_kind,
                ),
                first_evidence_packet_denial_receipt_id: evidence_packet_denial_receipt_id(
                    entry.contribution_kind,
                ),
                second_evidence_packet_denial_receipt_id: evidence_packet_denial_receipt_id(
                    entry.contribution_kind,
                ),
                first_evidence_packet_idempotency_key: evidence_packet_idempotency_key(
                    entry.contribution_kind,
                ),
                second_evidence_packet_idempotency_key: evidence_packet_idempotency_key(
                    entry.contribution_kind,
                ),
                operator_evidence_packet_id_projected: true,
                operator_evidence_artifact_ref_projected: true,
                acceptance_recording_prerequisite_link_projected: true,
                ledger_persistence_prerequisite_link_projected: true,
                receipt_persistence_prerequisite_link_projected: true,
                tool_registry_registration_prerequisite_link_projected: true,
                registry_lookup_prerequisite_link_projected: true,
                tool_invocation_prerequisite_link_projected: true,
                connector_start_prerequisite_link_projected: true,
                runtime_write_prerequisite_link_projected: true,
                live_execution_prerequisite_link_projected: true,
                evidence_packet_denial_receipt_projected: true,
                evidence_packet_idempotency_key_projected: true,
                stable_operator_evidence_packet: true,
                unique_operator_evidence_packet: true,
                stable_evidence_packet_denial_receipt: true,
                unique_evidence_packet_denial_receipt: true,
                stable_evidence_packet_idempotency_key: true,
                unique_evidence_packet_idempotency_key: true,
                feature_gate_opened: entry.feature_gate_opened,
                dry_run_executed: entry.dry_run_executed,
                operator_evidence_packet_sent: false,
                operator_evidence_packet_persisted: false,
                operator_evidence_recorded: false,
                operator_acceptance_recorded: entry.operator_acceptance_recorded,
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

fn operator_evidence_packet_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "operator-evidence-packet:hepta-system:local-mcp:read-only-status-dry-run",
        "app_connector" => "operator-evidence-packet:hepta-system:local-app:not-selected",
        _ => "operator-evidence-packet:hepta-system:unknown:not-selected",
    }
}

fn operator_evidence_artifact_ref_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-artifact-ref:hepta-system:local-mcp:read-only-status-dry-run:missing"
        }
        "app_connector" => {
            "operator-evidence-artifact-ref:hepta-system:local-app:not-selected:missing"
        }
        _ => "operator-evidence-artifact-ref:hepta-system:unknown:not-selected:missing",
    }
}

fn acceptance_recording_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:acceptance-recording:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:acceptance-recording:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-prerequisite:acceptance-recording:hepta-system:unknown:not-selected"
        }
    }
}

fn ledger_persistence_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:ledger-persistence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:ledger-persistence:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:ledger-persistence:hepta-system:unknown:not-selected",
    }
}

fn receipt_persistence_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:receipt-persistence:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:receipt-persistence:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:receipt-persistence:hepta-system:unknown:not-selected",
    }
}

fn tool_registry_registration_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:tool-registry-registration:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:tool-registry-registration:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-prerequisite:tool-registry-registration:hepta-system:unknown:not-selected"
        }
    }
}

fn registry_lookup_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:registry-lookup:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:registry-lookup:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:registry-lookup:hepta-system:unknown:not-selected",
    }
}

fn tool_invocation_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:tool-invocation:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:tool-invocation:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:tool-invocation:hepta-system:unknown:not-selected",
    }
}

fn connector_start_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:connector-start:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:connector-start:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:connector-start:hepta-system:unknown:not-selected",
    }
}

fn runtime_write_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:runtime-write:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:runtime-write:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:runtime-write:hepta-system:unknown:not-selected",
    }
}

fn live_execution_prerequisite_link_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-prerequisite:live-execution:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-prerequisite:live-execution:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-prerequisite:live-execution:hepta-system:unknown:not-selected",
    }
}

fn evidence_packet_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-packet-denial:hepta-system:local-mcp:read-only-status-dry-run:not-sent-not-recorded"
        }
        "app_connector" => {
            "operator-evidence-packet-denial:hepta-system:local-app:not-selected:not-sent-not-recorded"
        }
        _ => {
            "operator-evidence-packet-denial:hepta-system:unknown:not-selected:not-sent-not-recorded"
        }
    }
}

fn evidence_packet_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-packet-idempotency:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-packet-idempotency:hepta-system:local-app:not-selected"
        }
        _ => "operator-evidence-packet-idempotency:hepta-system:unknown:not-selected",
    }
}

impl HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_gate_opened: false,
            dry_run_executed: false,
            operator_evidence_packet_sent: false,
            operator_evidence_packet_persisted: false,
            operator_evidence_recorded: false,
            operator_acceptance_recorded: false,
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
    fn operator_evidence_packet_projects_required_evidence_and_prerequisites() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_execution_open_preconditions_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.evidence_packet_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(report.operator_evidence_packet_id_projected_count, 2);
        assert_eq!(report.operator_evidence_artifact_ref_projected_count, 2);
        assert_eq!(report.operator_evidence_item_count, 20);
        assert_eq!(
            report.acceptance_recording_prerequisite_link_projected_count,
            2
        );
        assert_eq!(
            report.ledger_persistence_prerequisite_link_projected_count,
            2
        );
        assert_eq!(
            report.receipt_persistence_prerequisite_link_projected_count,
            2
        );
        assert_eq!(
            report.tool_registry_registration_prerequisite_link_projected_count,
            2
        );
        assert_eq!(report.registry_lookup_prerequisite_link_projected_count, 2);
        assert_eq!(report.tool_invocation_prerequisite_link_projected_count, 2);
        assert_eq!(report.connector_start_prerequisite_link_projected_count, 2);
        assert_eq!(report.runtime_write_prerequisite_link_projected_count, 2);
        assert_eq!(report.live_execution_prerequisite_link_projected_count, 2);
        assert!(report.operator_evidence_packet_readback_ready);
    }

    #[test]
    fn operator_evidence_packet_stays_stable_unique_and_denied() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report();

        assert_eq!(report.evidence_packet_denial_receipt_projected_count, 2);
        assert_eq!(report.evidence_packet_idempotency_key_projected_count, 2);
        assert_eq!(report.stable_operator_evidence_packet_count, 2);
        assert_eq!(report.unique_operator_evidence_packet_count, 2);
        assert_eq!(report.stable_evidence_packet_denial_receipt_count, 2);
        assert_eq!(report.unique_evidence_packet_denial_receipt_count, 2);
        assert_eq!(report.stable_evidence_packet_idempotency_key_count, 2);
        assert_eq!(report.unique_evidence_packet_idempotency_key_count, 2);
        assert_eq!(report.operator_evidence_packet_mismatch_count, 0);
        assert_eq!(report.duplicate_operator_evidence_packet_count, 0);
        assert_eq!(report.evidence_packet_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_evidence_packet_denial_receipt_count, 0);
        assert_eq!(report.evidence_packet_idempotency_mismatch_count, 0);
        assert_eq!(report.duplicate_evidence_packet_idempotency_key_count, 0);
    }

    #[test]
    fn operator_evidence_packet_keeps_recording_execution_and_live_closed() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
        assert_eq!(report.operator_evidence_packet_sent_count, 0);
        assert_eq!(report.operator_evidence_packet_persisted_count, 0);
        assert_eq!(report.operator_evidence_recorded_count, 0);
        assert_eq!(report.operator_acceptance_recorded_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.mcp_server_started_count, 0);
        assert_eq!(report.app_connector_started_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.feature_gate_open_allowed);
        assert!(!report.dry_run_execution_allowed);
        assert!(!report.operator_evidence_packet_send_allowed);
        assert!(!report.operator_evidence_packet_persistence_allowed);
        assert!(!report.operator_evidence_recording_allowed);
        assert!(!report.operator_acceptance_recording_allowed);
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
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidencePacketReadbackSideEffects::none()
        );
    }
}
