use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_acceptance_recording_open_preconditions_readback_ready: bool,
    pub candidate_count: usize,
    pub persistence_denial_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub persistence_denial_receipt_projected_count: usize,
    pub persistence_denial_receipt_digest_projected_count: usize,
    pub persistence_write_denial_projected_count: usize,
    pub non_recording_denial_receipt_anchor_projected_count: usize,
    pub acceptance_recording_open_denial_receipt_anchor_projected_count: usize,
    pub ledger_persistence_denial_anchor_projected_count: usize,
    pub receipt_persistence_denial_anchor_projected_count: usize,
    pub tool_invocation_denial_anchor_projected_count: usize,
    pub runtime_write_denial_anchor_projected_count: usize,
    pub live_execution_denial_anchor_projected_count: usize,
    pub persistence_idempotency_key_projected_count: usize,
    pub stable_persistence_denial_receipt_count: usize,
    pub unique_persistence_denial_receipt_count: usize,
    pub stable_persistence_denial_receipt_digest_count: usize,
    pub unique_persistence_denial_receipt_digest_count: usize,
    pub stable_persistence_idempotency_key_count: usize,
    pub unique_persistence_idempotency_key_count: usize,
    pub persistence_denial_receipt_mismatch_count: usize,
    pub duplicate_persistence_denial_receipt_count: usize,
    pub persistence_denial_receipt_digest_mismatch_count: usize,
    pub duplicate_persistence_denial_receipt_digest_count: usize,
    pub persistence_idempotency_mismatch_count: usize,
    pub duplicate_persistence_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_evidence_packet_sent_count: usize,
    pub operator_evidence_packet_persisted_count: usize,
    pub operator_evidence_recorded_count: usize,
    pub operator_acceptance_recorded_count: usize,
    pub acceptance_record_persisted_count: usize,
    pub persistence_denial_receipt_persisted_count: usize,
    pub non_recording_denial_receipt_persisted_count: usize,
    pub idempotency_index_written_count: usize,
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
    pub persistence_denial_receipt_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_evidence_packet_send_allowed: bool,
    pub operator_evidence_packet_persistence_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub acceptance_record_persistence_allowed: bool,
    pub persistence_denial_receipt_persistence_allowed: bool,
    pub non_recording_denial_receipt_persistence_allowed: bool,
    pub idempotency_index_write_allowed: bool,
    pub ledger_persistence_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub connector_start_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackEntry
{
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_acceptance_recording_open_precondition_set_id: &'static str,
    pub source_evidence_artifact_presence_precondition_id: &'static str,
    pub source_operator_identity_precondition_id: &'static str,
    pub source_acceptance_recording_persistence_precondition_id: &'static str,
    pub source_ledger_persistence_precondition_id: &'static str,
    pub source_receipt_persistence_precondition_id: &'static str,
    pub source_tool_registry_registration_precondition_id: &'static str,
    pub source_registry_lookup_precondition_id: &'static str,
    pub source_tool_invocation_precondition_id: &'static str,
    pub source_runtime_write_precondition_id: &'static str,
    pub source_live_execution_precondition_id: &'static str,
    pub source_acceptance_recording_open_denial_receipt_id: &'static str,
    pub source_acceptance_recording_open_idempotency_key: &'static str,
    pub persistence_denial_receipt_id: &'static str,
    pub persistence_denial_receipt_digest: &'static str,
    pub persistence_write_denial_id: &'static str,
    pub non_recording_denial_receipt_anchor_id: &'static str,
    pub acceptance_recording_open_denial_receipt_anchor_id: &'static str,
    pub ledger_persistence_denial_anchor_id: &'static str,
    pub receipt_persistence_denial_anchor_id: &'static str,
    pub tool_invocation_denial_anchor_id: &'static str,
    pub runtime_write_denial_anchor_id: &'static str,
    pub live_execution_denial_anchor_id: &'static str,
    pub persistence_idempotency_key: &'static str,
    pub first_persistence_denial_receipt_id: &'static str,
    pub second_persistence_denial_receipt_id: &'static str,
    pub first_persistence_denial_receipt_digest: &'static str,
    pub second_persistence_denial_receipt_digest: &'static str,
    pub first_persistence_idempotency_key: &'static str,
    pub second_persistence_idempotency_key: &'static str,
    pub persistence_denial_receipt_projected: bool,
    pub persistence_denial_receipt_digest_projected: bool,
    pub persistence_write_denial_projected: bool,
    pub non_recording_denial_receipt_anchor_projected: bool,
    pub acceptance_recording_open_denial_receipt_anchor_projected: bool,
    pub ledger_persistence_denial_anchor_projected: bool,
    pub receipt_persistence_denial_anchor_projected: bool,
    pub tool_invocation_denial_anchor_projected: bool,
    pub runtime_write_denial_anchor_projected: bool,
    pub live_execution_denial_anchor_projected: bool,
    pub persistence_idempotency_key_projected: bool,
    pub stable_persistence_denial_receipt: bool,
    pub unique_persistence_denial_receipt: bool,
    pub stable_persistence_denial_receipt_digest: bool,
    pub unique_persistence_denial_receipt_digest: bool,
    pub stable_persistence_idempotency_key: bool,
    pub unique_persistence_idempotency_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub persistence_denial_receipt_persisted: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub idempotency_index_written: bool,
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
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub persistence_denial_receipt_persisted: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub idempotency_index_written: bool,
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

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report()
-> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport{
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport{
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_entries(source);
    let persistence_denial_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let persistence_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_receipt_projected)
        .count();
    let persistence_denial_receipt_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_receipt_digest_projected)
        .count();
    let persistence_write_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_write_denial_projected)
        .count();
    let non_recording_denial_receipt_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.non_recording_denial_receipt_anchor_projected)
        .count();
    let acceptance_recording_open_denial_receipt_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_recording_open_denial_receipt_anchor_projected)
        .count();
    let ledger_persistence_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_persistence_denial_anchor_projected)
        .count();
    let receipt_persistence_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_persistence_denial_anchor_projected)
        .count();
    let tool_invocation_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_invocation_denial_anchor_projected)
        .count();
    let runtime_write_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.runtime_write_denial_anchor_projected)
        .count();
    let live_execution_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.live_execution_denial_anchor_projected)
        .count();
    let persistence_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_idempotency_key_projected)
        .count();
    let stable_persistence_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_denial_receipt)
        .count();
    let unique_persistence_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_persistence_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_persistence_denial_receipt_digest_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_denial_receipt_digest)
        .count();
    let unique_persistence_denial_receipt_digest_count = entries
        .iter()
        .map(|entry| entry.first_persistence_denial_receipt_digest)
        .collect::<HashSet<_>>()
        .len();
    let stable_persistence_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_idempotency_key)
        .count();
    let unique_persistence_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_persistence_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let persistence_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_denial_receipt)
        .count();
    let duplicate_persistence_denial_receipt_count =
        persistence_denial_entry_count.saturating_sub(unique_persistence_denial_receipt_count);
    let persistence_denial_receipt_digest_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_denial_receipt_digest)
        .count();
    let duplicate_persistence_denial_receipt_digest_count = persistence_denial_entry_count
        .saturating_sub(unique_persistence_denial_receipt_digest_count);
    let persistence_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_idempotency_key)
        .count();
    let duplicate_persistence_idempotency_key_count =
        persistence_denial_entry_count.saturating_sub(unique_persistence_idempotency_key_count);
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
    let persistence_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_receipt_persisted)
        .count();
    let non_recording_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_recording_denial_receipt_persisted)
        .count();
    let idempotency_index_written_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_written)
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

    let persistence_denial_receipt_readback_ready = source
        .acceptance_recording_open_preconditions_readback_ready
        && source.candidate_count == 2
        && source.precondition_entry_count == 2
        && source.selected_read_only_status_tool_count == 1
        && source.non_selected_preflight_boundary_count == 1
        && source.acceptance_recording_persistence_precondition_projected_count == 2
        && source.ledger_persistence_precondition_projected_count == 2
        && source.receipt_persistence_precondition_projected_count == 2
        && source.tool_registry_registration_precondition_projected_count == 2
        && source.registry_lookup_precondition_projected_count == 2
        && source.tool_invocation_precondition_projected_count == 2
        && source.runtime_write_precondition_projected_count == 2
        && source.live_execution_precondition_projected_count == 2
        && source.acceptance_recording_open_denial_receipt_projected_count == 2
        && source.acceptance_recording_open_idempotency_key_projected_count == 2
        && source.operator_acceptance_recorded_count == 0
        && source.acceptance_record_persisted_count == 0
        && source.non_recording_denial_receipt_persisted_count == 0
        && source.ledger_written_count == 0
        && source.receipt_persisted_count == 0
        && source.tool_registered_count == 0
        && source.registry_lookup_executed_count == 0
        && source.tool_invoked_count == 0
        && persistence_denial_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && persistence_denial_receipt_projected_count == 2
        && persistence_denial_receipt_digest_projected_count == 2
        && persistence_write_denial_projected_count == 2
        && non_recording_denial_receipt_anchor_projected_count == 2
        && acceptance_recording_open_denial_receipt_anchor_projected_count == 2
        && ledger_persistence_denial_anchor_projected_count == 2
        && receipt_persistence_denial_anchor_projected_count == 2
        && tool_invocation_denial_anchor_projected_count == 2
        && runtime_write_denial_anchor_projected_count == 2
        && live_execution_denial_anchor_projected_count == 2
        && persistence_idempotency_key_projected_count == 2
        && stable_persistence_denial_receipt_count == 2
        && unique_persistence_denial_receipt_count == 2
        && stable_persistence_denial_receipt_digest_count == 2
        && unique_persistence_denial_receipt_digest_count == 2
        && stable_persistence_idempotency_key_count == 2
        && unique_persistence_idempotency_key_count == 2
        && persistence_denial_receipt_mismatch_count == 0
        && duplicate_persistence_denial_receipt_count == 0
        && persistence_denial_receipt_digest_mismatch_count == 0
        && duplicate_persistence_denial_receipt_digest_count == 0
        && persistence_idempotency_mismatch_count == 0
        && duplicate_persistence_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_evidence_packet_sent_count == 0
        && operator_evidence_packet_persisted_count == 0
        && operator_evidence_recorded_count == 0
        && operator_acceptance_recorded_count == 0
        && acceptance_record_persisted_count == 0
        && persistence_denial_receipt_persisted_count == 0
        && non_recording_denial_receipt_persisted_count == 0
        && idempotency_index_written_count == 0
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

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
        status: if persistence_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_acceptance_recording_open_preconditions_readback_ready: source
            .acceptance_recording_open_preconditions_readback_ready,
        candidate_count: source.candidate_count,
        persistence_denial_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        persistence_denial_receipt_projected_count,
        persistence_denial_receipt_digest_projected_count,
        persistence_write_denial_projected_count,
        non_recording_denial_receipt_anchor_projected_count,
        acceptance_recording_open_denial_receipt_anchor_projected_count,
        ledger_persistence_denial_anchor_projected_count,
        receipt_persistence_denial_anchor_projected_count,
        tool_invocation_denial_anchor_projected_count,
        runtime_write_denial_anchor_projected_count,
        live_execution_denial_anchor_projected_count,
        persistence_idempotency_key_projected_count,
        stable_persistence_denial_receipt_count,
        unique_persistence_denial_receipt_count,
        stable_persistence_denial_receipt_digest_count,
        unique_persistence_denial_receipt_digest_count,
        stable_persistence_idempotency_key_count,
        unique_persistence_idempotency_key_count,
        persistence_denial_receipt_mismatch_count,
        duplicate_persistence_denial_receipt_count,
        persistence_denial_receipt_digest_mismatch_count,
        duplicate_persistence_denial_receipt_digest_count,
        persistence_idempotency_mismatch_count,
        duplicate_persistence_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_evidence_packet_sent_count,
        operator_evidence_packet_persisted_count,
        operator_evidence_recorded_count,
        operator_acceptance_recorded_count,
        acceptance_record_persisted_count,
        persistence_denial_receipt_persisted_count,
        non_recording_denial_receipt_persisted_count,
        idempotency_index_written_count,
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
        persistence_denial_receipt_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_evidence_packet_send_allowed: false,
        operator_evidence_packet_persistence_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
        acceptance_record_persistence_allowed: false,
        persistence_denial_receipt_persistence_allowed: false,
        non_recording_denial_receipt_persistence_allowed: false,
        idempotency_index_write_allowed: false,
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
            "acceptance_recording_persistence_disabled",
            "acceptance_record_persistence_disabled",
            "persistence_denial_receipt_persistence_disabled",
            "non_recording_denial_receipt_persistence_disabled",
            "idempotency_index_write_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingOpenPreconditionsReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackEntry>{
    source
        .entries
        .iter()
        .map(|entry| HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackEntry {
            candidate_tool_id: entry.candidate_tool_id,
            contribution_kind: entry.contribution_kind,
            dry_run_path_selected: entry.dry_run_path_selected,
            source_acceptance_recording_open_precondition_set_id: entry
                .acceptance_recording_open_precondition_set_id,
            source_evidence_artifact_presence_precondition_id: entry
                .evidence_artifact_presence_precondition_id,
            source_operator_identity_precondition_id: entry.operator_identity_precondition_id,
            source_acceptance_recording_persistence_precondition_id: entry
                .acceptance_recording_persistence_precondition_id,
            source_ledger_persistence_precondition_id: entry
                .ledger_persistence_precondition_id,
            source_receipt_persistence_precondition_id: entry
                .receipt_persistence_precondition_id,
            source_tool_registry_registration_precondition_id: entry
                .tool_registry_registration_precondition_id,
            source_registry_lookup_precondition_id: entry.registry_lookup_precondition_id,
            source_tool_invocation_precondition_id: entry.tool_invocation_precondition_id,
            source_runtime_write_precondition_id: entry.runtime_write_precondition_id,
            source_live_execution_precondition_id: entry.live_execution_precondition_id,
            source_acceptance_recording_open_denial_receipt_id: entry
                .acceptance_recording_open_denial_receipt_id,
            source_acceptance_recording_open_idempotency_key: entry
                .acceptance_recording_open_idempotency_key,
            persistence_denial_receipt_id: persistence_denial_receipt_id(entry.contribution_kind),
            persistence_denial_receipt_digest: persistence_denial_receipt_digest(entry.contribution_kind),
            persistence_write_denial_id: persistence_write_denial_id(entry.contribution_kind),
            non_recording_denial_receipt_anchor_id: non_recording_denial_receipt_anchor_id(
                entry.contribution_kind,
            ),
            acceptance_recording_open_denial_receipt_anchor_id:
                acceptance_recording_open_denial_receipt_anchor_id(entry.contribution_kind),
            ledger_persistence_denial_anchor_id: ledger_persistence_denial_anchor_id(
                entry.contribution_kind,
            ),
            receipt_persistence_denial_anchor_id: receipt_persistence_denial_anchor_id(
                entry.contribution_kind,
            ),
            tool_invocation_denial_anchor_id: tool_invocation_denial_anchor_id(
                entry.contribution_kind,
            ),
            runtime_write_denial_anchor_id: runtime_write_denial_anchor_id(
                entry.contribution_kind,
            ),
            live_execution_denial_anchor_id: live_execution_denial_anchor_id(
                entry.contribution_kind,
            ),
            persistence_idempotency_key: persistence_idempotency_key(entry.contribution_kind),
            first_persistence_denial_receipt_id: persistence_denial_receipt_id(
                entry.contribution_kind,
            ),
            second_persistence_denial_receipt_id: persistence_denial_receipt_id(
                entry.contribution_kind,
            ),
            first_persistence_denial_receipt_digest: persistence_denial_receipt_digest(
                entry.contribution_kind,
            ),
            second_persistence_denial_receipt_digest: persistence_denial_receipt_digest(
                entry.contribution_kind,
            ),
            first_persistence_idempotency_key: persistence_idempotency_key(entry.contribution_kind),
            second_persistence_idempotency_key: persistence_idempotency_key(entry.contribution_kind),
            persistence_denial_receipt_projected: true,
            persistence_denial_receipt_digest_projected: true,
            persistence_write_denial_projected: true,
            non_recording_denial_receipt_anchor_projected: true,
            acceptance_recording_open_denial_receipt_anchor_projected: true,
            ledger_persistence_denial_anchor_projected: true,
            receipt_persistence_denial_anchor_projected: true,
            tool_invocation_denial_anchor_projected: true,
            runtime_write_denial_anchor_projected: true,
            live_execution_denial_anchor_projected: true,
            persistence_idempotency_key_projected: true,
            stable_persistence_denial_receipt: true,
            unique_persistence_denial_receipt: true,
            stable_persistence_denial_receipt_digest: true,
            unique_persistence_denial_receipt_digest: true,
            stable_persistence_idempotency_key: true,
            unique_persistence_idempotency_key: true,
            feature_gate_opened: entry.feature_gate_opened,
            dry_run_executed: entry.dry_run_executed,
            operator_evidence_packet_sent: entry.operator_evidence_packet_sent,
            operator_evidence_packet_persisted: entry.operator_evidence_packet_persisted,
            operator_evidence_recorded: entry.operator_evidence_recorded,
            operator_acceptance_recorded: entry.operator_acceptance_recorded,
            acceptance_record_persisted: entry.acceptance_record_persisted,
            persistence_denial_receipt_persisted: false,
            non_recording_denial_receipt_persisted: entry
                .non_recording_denial_receipt_persisted,
            idempotency_index_written: false,
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

fn persistence_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-persistence-denial-receipt:hepta-system:local-mcp:read-only-status-dry-run:acceptance-record-persistence-disabled"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-persistence-denial-receipt:hepta-system:local-app:not-selected:acceptance-record-persistence-disabled"
        }
        _ => {
            "operator-evidence-acceptance-recording-persistence-denial-receipt:hepta-system:unknown:not-selected:acceptance-record-persistence-disabled"
        }
    }
}

fn persistence_denial_receipt_digest(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "sha256:operator-evidence-acceptance-recording-persistence-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "sha256:operator-evidence-acceptance-recording-persistence-denial:hepta-system:local-app:not-selected"
        }
        _ => {
            "sha256:operator-evidence-acceptance-recording-persistence-denial:hepta-system:unknown:not-selected"
        }
    }
}

fn persistence_write_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-persistence-write-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-persistence-write-denial:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-persistence-write-denial:hepta-system:unknown:not-selected"
        }
    }
}

fn anchor_id(name: &str, contribution_kind: &str) -> &'static str {
    match (name, suffix(contribution_kind)) {
        ("non-recording-denial-receipt", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:non-recording-denial-receipt:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("non-recording-denial-receipt", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:non-recording-denial-receipt:hepta-system:local-app:not-selected"
        }
        ("acceptance-recording-open-denial-receipt", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:open-denial-receipt:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("acceptance-recording-open-denial-receipt", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:open-denial-receipt:hepta-system:local-app:not-selected"
        }
        ("ledger-persistence-denial", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:ledger-persistence-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("ledger-persistence-denial", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:ledger-persistence-denial:hepta-system:local-app:not-selected"
        }
        ("receipt-persistence-denial", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:receipt-persistence-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("receipt-persistence-denial", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:receipt-persistence-denial:hepta-system:local-app:not-selected"
        }
        ("tool-invocation-denial", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:tool-invocation-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("tool-invocation-denial", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:tool-invocation-denial:hepta-system:local-app:not-selected"
        }
        ("runtime-write-denial", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:runtime-write-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("runtime-write-denial", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:runtime-write-denial:hepta-system:local-app:not-selected"
        }
        ("live-execution-denial", "local-mcp:read-only-status-dry-run") => {
            "operator-evidence-acceptance-recording-persistence-anchor:live-execution-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        ("live-execution-denial", "local-app:not-selected") => {
            "operator-evidence-acceptance-recording-persistence-anchor:live-execution-denial:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-persistence-anchor:unknown:hepta-system:unknown:not-selected"
        }
    }
}

fn non_recording_denial_receipt_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("non-recording-denial-receipt", contribution_kind)
}

fn acceptance_recording_open_denial_receipt_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id(
        "acceptance-recording-open-denial-receipt",
        contribution_kind,
    )
}

fn ledger_persistence_denial_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("ledger-persistence-denial", contribution_kind)
}

fn receipt_persistence_denial_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("receipt-persistence-denial", contribution_kind)
}

fn tool_invocation_denial_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("tool-invocation-denial", contribution_kind)
}

fn runtime_write_denial_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("runtime-write-denial", contribution_kind)
}

fn live_execution_denial_anchor_id(contribution_kind: &str) -> &'static str {
    anchor_id("live-execution-denial", contribution_kind)
}

fn persistence_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-evidence-acceptance-recording-persistence-idempotency:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-evidence-acceptance-recording-persistence-idempotency:hepta-system:local-app:not-selected"
        }
        _ => {
            "operator-evidence-acceptance-recording-persistence-idempotency:hepta-system:unknown:not-selected"
        }
    }
}

impl HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackSideEffects {
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
            persistence_denial_receipt_persisted: false,
            non_recording_denial_receipt_persisted: false,
            idempotency_index_written: false,
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
    fn persistence_denial_receipt_projects_stable_query_only_receipts() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_acceptance_recording_open_preconditions_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.persistence_denial_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(report.persistence_denial_receipt_projected_count, 2);
        assert_eq!(report.persistence_denial_receipt_digest_projected_count, 2);
        assert_eq!(report.persistence_write_denial_projected_count, 2);
        assert_eq!(
            report.non_recording_denial_receipt_anchor_projected_count,
            2
        );
        assert_eq!(
            report.acceptance_recording_open_denial_receipt_anchor_projected_count,
            2
        );
        assert_eq!(report.ledger_persistence_denial_anchor_projected_count, 2);
        assert_eq!(report.receipt_persistence_denial_anchor_projected_count, 2);
        assert_eq!(report.tool_invocation_denial_anchor_projected_count, 2);
        assert_eq!(report.runtime_write_denial_anchor_projected_count, 2);
        assert_eq!(report.live_execution_denial_anchor_projected_count, 2);
        assert_eq!(report.persistence_idempotency_key_projected_count, 2);
        assert!(report.persistence_denial_receipt_readback_ready);
    }

    #[test]
    fn persistence_denial_receipt_stays_stable_unique_and_unpersisted() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report();

        assert_eq!(report.stable_persistence_denial_receipt_count, 2);
        assert_eq!(report.unique_persistence_denial_receipt_count, 2);
        assert_eq!(report.stable_persistence_denial_receipt_digest_count, 2);
        assert_eq!(report.unique_persistence_denial_receipt_digest_count, 2);
        assert_eq!(report.stable_persistence_idempotency_key_count, 2);
        assert_eq!(report.unique_persistence_idempotency_key_count, 2);
        assert_eq!(report.persistence_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_denial_receipt_count, 0);
        assert_eq!(report.persistence_denial_receipt_digest_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_denial_receipt_digest_count, 0);
        assert_eq!(report.persistence_idempotency_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_idempotency_key_count, 0);
        assert_eq!(report.persistence_denial_receipt_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
    }

    #[test]
    fn persistence_denial_receipt_keeps_all_execution_and_runtime_paths_closed() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report();

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
        assert!(!report.acceptance_record_persistence_allowed);
        assert!(!report.persistence_denial_receipt_persistence_allowed);
        assert!(!report.idempotency_index_write_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackSideEffects::none()
        );
        assert_eq!(
            report.recommended_next_gate,
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
