use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_persistence_denial_receipt_readback_ready: bool,
    pub candidate_count: usize,
    pub precondition_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub persistence_open_precondition_set_projected_count: usize,
    pub source_persistence_denial_receipt_linked_count: usize,
    pub source_persistence_denial_receipt_digest_linked_count: usize,
    pub source_persistence_idempotency_key_linked_count: usize,
    pub evidence_artifact_presence_precondition_projected_count: usize,
    pub operator_identity_precondition_projected_count: usize,
    pub operator_acceptance_precondition_projected_count: usize,
    pub operator_evidence_record_store_binding_precondition_projected_count: usize,
    pub acceptance_record_schema_precondition_projected_count: usize,
    pub acceptance_record_store_binding_precondition_projected_count: usize,
    pub acceptance_record_idempotency_index_precondition_projected_count: usize,
    pub ledger_store_binding_precondition_projected_count: usize,
    pub receipt_store_binding_precondition_projected_count: usize,
    pub runtime_event_log_store_binding_precondition_projected_count: usize,
    pub rollback_anchor_precondition_projected_count: usize,
    pub kill_switch_precondition_projected_count: usize,
    pub retention_policy_precondition_projected_count: usize,
    pub readback_query_precondition_projected_count: usize,
    pub controlled_live_evidence_precondition_projected_count: usize,
    pub feature_gate_precondition_projected_count: usize,
    pub persistence_open_precondition_item_count: usize,
    pub stable_persistence_open_precondition_set_count: usize,
    pub unique_persistence_open_precondition_set_count: usize,
    pub stable_persistence_open_denial_receipt_count: usize,
    pub unique_persistence_open_denial_receipt_count: usize,
    pub stable_persistence_open_idempotency_key_count: usize,
    pub unique_persistence_open_idempotency_key_count: usize,
    pub persistence_open_precondition_set_mismatch_count: usize,
    pub duplicate_persistence_open_precondition_set_count: usize,
    pub persistence_open_denial_receipt_mismatch_count: usize,
    pub duplicate_persistence_open_denial_receipt_count: usize,
    pub persistence_open_idempotency_mismatch_count: usize,
    pub duplicate_persistence_open_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_evidence_packet_sent_count: usize,
    pub operator_evidence_packet_persisted_count: usize,
    pub operator_evidence_recorded_count: usize,
    pub operator_acceptance_recorded_count: usize,
    pub acceptance_record_persisted_count: usize,
    pub persistence_open_denial_receipt_persisted_count: usize,
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
    pub persistence_open_preconditions_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_evidence_packet_send_allowed: bool,
    pub operator_evidence_packet_persistence_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub acceptance_record_persistence_allowed: bool,
    pub persistence_open_denial_receipt_persistence_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackEntry
{
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_persistence_denial_receipt_id: &'static str,
    pub source_persistence_denial_receipt_digest: &'static str,
    pub source_persistence_idempotency_key: &'static str,
    pub source_non_recording_denial_receipt_anchor_id: &'static str,
    pub source_acceptance_recording_open_denial_receipt_anchor_id: &'static str,
    pub source_ledger_persistence_denial_anchor_id: &'static str,
    pub source_receipt_persistence_denial_anchor_id: &'static str,
    pub source_tool_invocation_denial_anchor_id: &'static str,
    pub source_runtime_write_denial_anchor_id: &'static str,
    pub source_live_execution_denial_anchor_id: &'static str,
    pub persistence_open_precondition_set_id: String,
    pub persistence_open_denial_receipt_id: String,
    pub persistence_open_idempotency_key: String,
    pub first_persistence_open_precondition_set_id: String,
    pub second_persistence_open_precondition_set_id: String,
    pub first_persistence_open_denial_receipt_id: String,
    pub second_persistence_open_denial_receipt_id: String,
    pub first_persistence_open_idempotency_key: String,
    pub second_persistence_open_idempotency_key: String,
    pub evidence_artifact_presence_precondition_id: String,
    pub operator_identity_precondition_id: String,
    pub operator_acceptance_precondition_id: String,
    pub operator_evidence_record_store_binding_precondition_id: String,
    pub acceptance_record_schema_precondition_id: String,
    pub acceptance_record_store_binding_precondition_id: String,
    pub acceptance_record_idempotency_index_precondition_id: String,
    pub ledger_store_binding_precondition_id: String,
    pub receipt_store_binding_precondition_id: String,
    pub runtime_event_log_store_binding_precondition_id: String,
    pub rollback_anchor_precondition_id: String,
    pub kill_switch_precondition_id: String,
    pub retention_policy_precondition_id: String,
    pub readback_query_precondition_id: String,
    pub controlled_live_evidence_precondition_id: String,
    pub feature_gate_precondition_id: String,
    pub persistence_open_precondition_set_projected: bool,
    pub source_persistence_denial_receipt_linked: bool,
    pub source_persistence_denial_receipt_digest_linked: bool,
    pub source_persistence_idempotency_key_linked: bool,
    pub evidence_artifact_presence_precondition_projected: bool,
    pub operator_identity_precondition_projected: bool,
    pub operator_acceptance_precondition_projected: bool,
    pub operator_evidence_record_store_binding_precondition_projected: bool,
    pub acceptance_record_schema_precondition_projected: bool,
    pub acceptance_record_store_binding_precondition_projected: bool,
    pub acceptance_record_idempotency_index_precondition_projected: bool,
    pub ledger_store_binding_precondition_projected: bool,
    pub receipt_store_binding_precondition_projected: bool,
    pub runtime_event_log_store_binding_precondition_projected: bool,
    pub rollback_anchor_precondition_projected: bool,
    pub kill_switch_precondition_projected: bool,
    pub retention_policy_precondition_projected: bool,
    pub readback_query_precondition_projected: bool,
    pub controlled_live_evidence_precondition_projected: bool,
    pub feature_gate_precondition_projected: bool,
    pub stable_persistence_open_precondition_set: bool,
    pub unique_persistence_open_precondition_set: bool,
    pub stable_persistence_open_denial_receipt: bool,
    pub unique_persistence_open_denial_receipt: bool,
    pub stable_persistence_open_idempotency_key: bool,
    pub unique_persistence_open_idempotency_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub persistence_open_denial_receipt_persisted: bool,
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
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub persistence_open_denial_receipt_persisted: bool,
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

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report(
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport{
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport{
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_entries(source);
    let precondition_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let persistence_open_precondition_set_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_open_precondition_set_projected)
        .count();
    let source_persistence_denial_receipt_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_denial_receipt_linked)
        .count();
    let source_persistence_denial_receipt_digest_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_denial_receipt_digest_linked)
        .count();
    let source_persistence_idempotency_key_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_idempotency_key_linked)
        .count();
    let evidence_artifact_presence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.evidence_artifact_presence_precondition_projected)
        .count();
    let operator_identity_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_identity_precondition_projected)
        .count();
    let operator_acceptance_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_precondition_projected)
        .count();
    let operator_evidence_record_store_binding_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_record_store_binding_precondition_projected)
        .count();
    let acceptance_record_schema_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_schema_precondition_projected)
        .count();
    let acceptance_record_store_binding_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_store_binding_precondition_projected)
        .count();
    let acceptance_record_idempotency_index_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_idempotency_index_precondition_projected)
        .count();
    let ledger_store_binding_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_store_binding_precondition_projected)
        .count();
    let receipt_store_binding_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_binding_precondition_projected)
        .count();
    let runtime_event_log_store_binding_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.runtime_event_log_store_binding_precondition_projected)
        .count();
    let rollback_anchor_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_precondition_projected)
        .count();
    let kill_switch_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.kill_switch_precondition_projected)
        .count();
    let retention_policy_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_precondition_projected)
        .count();
    let readback_query_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.readback_query_precondition_projected)
        .count();
    let controlled_live_evidence_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.controlled_live_evidence_precondition_projected)
        .count();
    let feature_gate_precondition_projected_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_precondition_projected)
        .count();
    let persistence_open_precondition_item_count =
        evidence_artifact_presence_precondition_projected_count
            + operator_identity_precondition_projected_count
            + operator_acceptance_precondition_projected_count
            + operator_evidence_record_store_binding_precondition_projected_count
            + acceptance_record_schema_precondition_projected_count
            + acceptance_record_store_binding_precondition_projected_count
            + acceptance_record_idempotency_index_precondition_projected_count
            + ledger_store_binding_precondition_projected_count
            + receipt_store_binding_precondition_projected_count
            + runtime_event_log_store_binding_precondition_projected_count
            + rollback_anchor_precondition_projected_count
            + kill_switch_precondition_projected_count
            + retention_policy_precondition_projected_count
            + readback_query_precondition_projected_count
            + controlled_live_evidence_precondition_projected_count
            + feature_gate_precondition_projected_count;
    let stable_persistence_open_precondition_set_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_open_precondition_set)
        .count();
    let unique_persistence_open_precondition_set_count = entries
        .iter()
        .map(|entry| entry.first_persistence_open_precondition_set_id.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_persistence_open_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_open_denial_receipt)
        .count();
    let unique_persistence_open_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_persistence_open_denial_receipt_id.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_persistence_open_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_persistence_open_idempotency_key)
        .count();
    let unique_persistence_open_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_persistence_open_idempotency_key.as_str())
        .collect::<HashSet<_>>()
        .len();
    let persistence_open_precondition_set_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_open_precondition_set)
        .count();
    let duplicate_persistence_open_precondition_set_count =
        precondition_entry_count.saturating_sub(unique_persistence_open_precondition_set_count);
    let persistence_open_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_open_denial_receipt)
        .count();
    let duplicate_persistence_open_denial_receipt_count =
        precondition_entry_count.saturating_sub(unique_persistence_open_denial_receipt_count);
    let persistence_open_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_persistence_open_idempotency_key)
        .count();
    let duplicate_persistence_open_idempotency_key_count =
        precondition_entry_count.saturating_sub(unique_persistence_open_idempotency_key_count);
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
    let persistence_open_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.persistence_open_denial_receipt_persisted)
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

    let persistence_open_preconditions_readback_ready = source
        .persistence_denial_receipt_readback_ready
        && source.candidate_count == 2
        && source.persistence_denial_entry_count == 2
        && source.stable_persistence_denial_receipt_count == 2
        && source.unique_persistence_denial_receipt_count == 2
        && source.stable_persistence_idempotency_key_count == 2
        && source.unique_persistence_idempotency_key_count == 2
        && source.acceptance_record_persisted_count == 0
        && source.persistence_denial_receipt_persisted_count == 0
        && source.idempotency_index_written_count == 0
        && precondition_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && persistence_open_precondition_set_projected_count == 2
        && source_persistence_denial_receipt_linked_count == 2
        && source_persistence_denial_receipt_digest_linked_count == 2
        && source_persistence_idempotency_key_linked_count == 2
        && evidence_artifact_presence_precondition_projected_count == 2
        && operator_identity_precondition_projected_count == 2
        && operator_acceptance_precondition_projected_count == 2
        && operator_evidence_record_store_binding_precondition_projected_count == 2
        && acceptance_record_schema_precondition_projected_count == 2
        && acceptance_record_store_binding_precondition_projected_count == 2
        && acceptance_record_idempotency_index_precondition_projected_count == 2
        && ledger_store_binding_precondition_projected_count == 2
        && receipt_store_binding_precondition_projected_count == 2
        && runtime_event_log_store_binding_precondition_projected_count == 2
        && rollback_anchor_precondition_projected_count == 2
        && kill_switch_precondition_projected_count == 2
        && retention_policy_precondition_projected_count == 2
        && readback_query_precondition_projected_count == 2
        && controlled_live_evidence_precondition_projected_count == 2
        && feature_gate_precondition_projected_count == 2
        && persistence_open_precondition_item_count == 32
        && stable_persistence_open_precondition_set_count == 2
        && unique_persistence_open_precondition_set_count == 2
        && stable_persistence_open_denial_receipt_count == 2
        && unique_persistence_open_denial_receipt_count == 2
        && stable_persistence_open_idempotency_key_count == 2
        && unique_persistence_open_idempotency_key_count == 2
        && persistence_open_precondition_set_mismatch_count == 0
        && duplicate_persistence_open_precondition_set_count == 0
        && persistence_open_denial_receipt_mismatch_count == 0
        && duplicate_persistence_open_denial_receipt_count == 0
        && persistence_open_idempotency_mismatch_count == 0
        && duplicate_persistence_open_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_evidence_packet_sent_count == 0
        && operator_evidence_packet_persisted_count == 0
        && operator_evidence_recorded_count == 0
        && operator_acceptance_recorded_count == 0
        && acceptance_record_persisted_count == 0
        && persistence_open_denial_receipt_persisted_count == 0
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

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback",
        status: if persistence_open_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_persistence_denial_receipt_readback_ready: source
            .persistence_denial_receipt_readback_ready,
        candidate_count: source.candidate_count,
        precondition_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        persistence_open_precondition_set_projected_count,
        source_persistence_denial_receipt_linked_count,
        source_persistence_denial_receipt_digest_linked_count,
        source_persistence_idempotency_key_linked_count,
        evidence_artifact_presence_precondition_projected_count,
        operator_identity_precondition_projected_count,
        operator_acceptance_precondition_projected_count,
        operator_evidence_record_store_binding_precondition_projected_count,
        acceptance_record_schema_precondition_projected_count,
        acceptance_record_store_binding_precondition_projected_count,
        acceptance_record_idempotency_index_precondition_projected_count,
        ledger_store_binding_precondition_projected_count,
        receipt_store_binding_precondition_projected_count,
        runtime_event_log_store_binding_precondition_projected_count,
        rollback_anchor_precondition_projected_count,
        kill_switch_precondition_projected_count,
        retention_policy_precondition_projected_count,
        readback_query_precondition_projected_count,
        controlled_live_evidence_precondition_projected_count,
        feature_gate_precondition_projected_count,
        persistence_open_precondition_item_count,
        stable_persistence_open_precondition_set_count,
        unique_persistence_open_precondition_set_count,
        stable_persistence_open_denial_receipt_count,
        unique_persistence_open_denial_receipt_count,
        stable_persistence_open_idempotency_key_count,
        unique_persistence_open_idempotency_key_count,
        persistence_open_precondition_set_mismatch_count,
        duplicate_persistence_open_precondition_set_count,
        persistence_open_denial_receipt_mismatch_count,
        duplicate_persistence_open_denial_receipt_count,
        persistence_open_idempotency_mismatch_count,
        duplicate_persistence_open_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_evidence_packet_sent_count,
        operator_evidence_packet_persisted_count,
        operator_evidence_recorded_count,
        operator_acceptance_recorded_count,
        acceptance_record_persisted_count,
        persistence_open_denial_receipt_persisted_count,
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
        persistence_open_preconditions_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_evidence_packet_send_allowed: false,
        operator_evidence_packet_persistence_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
        acceptance_record_persistence_allowed: false,
        persistence_open_denial_receipt_persistence_allowed: false,
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
            "feature_gate_closed",
            "operator_evidence_artifact_absent",
            "operator_identity_unverified",
            "operator_acceptance_unrecorded",
            "operator_evidence_record_store_binding_absent",
            "acceptance_record_schema_unaccepted",
            "acceptance_record_store_binding_absent",
            "acceptance_record_idempotency_index_absent",
            "ledger_store_binding_absent",
            "receipt_store_binding_absent",
            "runtime_event_log_store_binding_absent",
            "rollback_anchor_absent",
            "kill_switch_unrehearsed",
            "retention_policy_unaccepted",
            "readback_query_unverified",
            "controlled_live_evidence_absent",
            "tool_registry_registration_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "connector_start_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceDenialReceiptReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackEntry>{
    source
        .entries
        .iter()
        .map(|entry| {
            let kind = entry.contribution_kind;
            let precondition_set_id = open_precondition_set_id(kind);
            let open_denial_receipt_id = open_denial_receipt_id(kind);
            let open_idempotency_key = open_idempotency_key(kind);
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_persistence_denial_receipt_id: entry.persistence_denial_receipt_id,
                source_persistence_denial_receipt_digest: entry.persistence_denial_receipt_digest,
                source_persistence_idempotency_key: entry.persistence_idempotency_key,
                source_non_recording_denial_receipt_anchor_id: entry.non_recording_denial_receipt_anchor_id,
                source_acceptance_recording_open_denial_receipt_anchor_id: entry.acceptance_recording_open_denial_receipt_anchor_id,
                source_ledger_persistence_denial_anchor_id: entry.ledger_persistence_denial_anchor_id,
                source_receipt_persistence_denial_anchor_id: entry.receipt_persistence_denial_anchor_id,
                source_tool_invocation_denial_anchor_id: entry.tool_invocation_denial_anchor_id,
                source_runtime_write_denial_anchor_id: entry.runtime_write_denial_anchor_id,
                source_live_execution_denial_anchor_id: entry.live_execution_denial_anchor_id,
                persistence_open_precondition_set_id: precondition_set_id.clone(),
                persistence_open_denial_receipt_id: open_denial_receipt_id.clone(),
                persistence_open_idempotency_key: open_idempotency_key.clone(),
                first_persistence_open_precondition_set_id: precondition_set_id.clone(),
                second_persistence_open_precondition_set_id: precondition_set_id,
                first_persistence_open_denial_receipt_id: open_denial_receipt_id.clone(),
                second_persistence_open_denial_receipt_id: open_denial_receipt_id,
                first_persistence_open_idempotency_key: open_idempotency_key.clone(),
                second_persistence_open_idempotency_key: open_idempotency_key,
                evidence_artifact_presence_precondition_id: open_precondition_id("evidence-artifact-presence", kind),
                operator_identity_precondition_id: open_precondition_id("operator-identity", kind),
                operator_acceptance_precondition_id: open_precondition_id("operator-acceptance", kind),
                operator_evidence_record_store_binding_precondition_id: open_precondition_id("operator-evidence-record-store-binding", kind),
                acceptance_record_schema_precondition_id: open_precondition_id("acceptance-record-schema", kind),
                acceptance_record_store_binding_precondition_id: open_precondition_id("acceptance-record-store-binding", kind),
                acceptance_record_idempotency_index_precondition_id: open_precondition_id("acceptance-record-idempotency-index", kind),
                ledger_store_binding_precondition_id: open_precondition_id("ledger-store-binding", kind),
                receipt_store_binding_precondition_id: open_precondition_id("receipt-store-binding", kind),
                runtime_event_log_store_binding_precondition_id: open_precondition_id("runtime-event-log-store-binding", kind),
                rollback_anchor_precondition_id: open_precondition_id("rollback-anchor", kind),
                kill_switch_precondition_id: open_precondition_id("kill-switch", kind),
                retention_policy_precondition_id: open_precondition_id("retention-policy", kind),
                readback_query_precondition_id: open_precondition_id("readback-query", kind),
                controlled_live_evidence_precondition_id: open_precondition_id("controlled-live-evidence", kind),
                feature_gate_precondition_id: open_precondition_id("feature-gate", kind),
                persistence_open_precondition_set_projected: true,
                source_persistence_denial_receipt_linked: true,
                source_persistence_denial_receipt_digest_linked: true,
                source_persistence_idempotency_key_linked: true,
                evidence_artifact_presence_precondition_projected: true,
                operator_identity_precondition_projected: true,
                operator_acceptance_precondition_projected: true,
                operator_evidence_record_store_binding_precondition_projected: true,
                acceptance_record_schema_precondition_projected: true,
                acceptance_record_store_binding_precondition_projected: true,
                acceptance_record_idempotency_index_precondition_projected: true,
                ledger_store_binding_precondition_projected: true,
                receipt_store_binding_precondition_projected: true,
                runtime_event_log_store_binding_precondition_projected: true,
                rollback_anchor_precondition_projected: true,
                kill_switch_precondition_projected: true,
                retention_policy_precondition_projected: true,
                readback_query_precondition_projected: true,
                controlled_live_evidence_precondition_projected: true,
                feature_gate_precondition_projected: true,
                stable_persistence_open_precondition_set: true,
                unique_persistence_open_precondition_set: true,
                stable_persistence_open_denial_receipt: true,
                unique_persistence_open_denial_receipt: true,
                stable_persistence_open_idempotency_key: true,
                unique_persistence_open_idempotency_key: true,
                feature_gate_opened: entry.feature_gate_opened,
                dry_run_executed: entry.dry_run_executed,
                operator_evidence_packet_sent: entry.operator_evidence_packet_sent,
                operator_evidence_packet_persisted: entry.operator_evidence_packet_persisted,
                operator_evidence_recorded: entry.operator_evidence_recorded,
                operator_acceptance_recorded: entry.operator_acceptance_recorded,
                acceptance_record_persisted: entry.acceptance_record_persisted,
                persistence_open_denial_receipt_persisted: false,
                persistence_denial_receipt_persisted: entry.persistence_denial_receipt_persisted,
                non_recording_denial_receipt_persisted: entry.non_recording_denial_receipt_persisted,
                idempotency_index_written: entry.idempotency_index_written,
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

fn open_precondition_set_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-open-preconditions:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn open_denial_receipt_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-open-denial-receipt:hepta-system:{}:open-preconditions-unsatisfied",
        suffix(contribution_kind)
    )
}

fn open_idempotency_key(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-open-idempotency:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn open_precondition_id(name: &str, contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-open-precondition:{}:hepta-system:{}",
        name,
        suffix(contribution_kind)
    )
}

impl HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackSideEffects {
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
            persistence_open_denial_receipt_persisted: false,
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
    fn persistence_open_preconditions_project_required_readiness_gates() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_persistence_denial_receipt_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.precondition_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(report.persistence_open_precondition_set_projected_count, 2);
        assert_eq!(report.source_persistence_denial_receipt_linked_count, 2);
        assert_eq!(
            report.source_persistence_denial_receipt_digest_linked_count,
            2
        );
        assert_eq!(report.source_persistence_idempotency_key_linked_count, 2);
        assert_eq!(
            report.evidence_artifact_presence_precondition_projected_count,
            2
        );
        assert_eq!(report.operator_identity_precondition_projected_count, 2);
        assert_eq!(report.operator_acceptance_precondition_projected_count, 2);
        assert_eq!(
            report.operator_evidence_record_store_binding_precondition_projected_count,
            2
        );
        assert_eq!(
            report.acceptance_record_schema_precondition_projected_count,
            2
        );
        assert_eq!(
            report.acceptance_record_store_binding_precondition_projected_count,
            2
        );
        assert_eq!(
            report.acceptance_record_idempotency_index_precondition_projected_count,
            2
        );
        assert_eq!(report.ledger_store_binding_precondition_projected_count, 2);
        assert_eq!(report.receipt_store_binding_precondition_projected_count, 2);
        assert_eq!(
            report.runtime_event_log_store_binding_precondition_projected_count,
            2
        );
        assert_eq!(report.rollback_anchor_precondition_projected_count, 2);
        assert_eq!(report.kill_switch_precondition_projected_count, 2);
        assert_eq!(report.retention_policy_precondition_projected_count, 2);
        assert_eq!(report.readback_query_precondition_projected_count, 2);
        assert_eq!(
            report.controlled_live_evidence_precondition_projected_count,
            2
        );
        assert_eq!(report.feature_gate_precondition_projected_count, 2);
        assert_eq!(report.persistence_open_precondition_item_count, 32);
        assert!(report.persistence_open_preconditions_readback_ready);
    }

    #[test]
    fn persistence_open_preconditions_stay_stable_unique_and_unpersisted() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report();

        assert_eq!(report.stable_persistence_open_precondition_set_count, 2);
        assert_eq!(report.unique_persistence_open_precondition_set_count, 2);
        assert_eq!(report.stable_persistence_open_denial_receipt_count, 2);
        assert_eq!(report.unique_persistence_open_denial_receipt_count, 2);
        assert_eq!(report.stable_persistence_open_idempotency_key_count, 2);
        assert_eq!(report.unique_persistence_open_idempotency_key_count, 2);
        assert_eq!(report.persistence_open_precondition_set_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_open_precondition_set_count, 0);
        assert_eq!(report.persistence_open_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_open_denial_receipt_count, 0);
        assert_eq!(report.persistence_open_idempotency_mismatch_count, 0);
        assert_eq!(report.duplicate_persistence_open_idempotency_key_count, 0);
        assert_eq!(report.acceptance_record_persisted_count, 0);
        assert_eq!(report.persistence_open_denial_receipt_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
    }

    #[test]
    fn persistence_open_preconditions_keep_all_writes_and_live_closed() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
        assert_eq!(report.operator_evidence_packet_sent_count, 0);
        assert_eq!(report.operator_evidence_packet_persisted_count, 0);
        assert_eq!(report.operator_evidence_recorded_count, 0);
        assert_eq!(report.operator_acceptance_recorded_count, 0);
        assert_eq!(report.acceptance_record_persisted_count, 0);
        assert_eq!(report.persistence_denial_receipt_persisted_count, 0);
        assert_eq!(report.non_recording_denial_receipt_persisted_count, 0);
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
        assert!(!report.acceptance_record_persistence_allowed);
        assert!(!report.persistence_open_denial_receipt_persistence_allowed);
        assert!(!report.idempotency_index_write_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackSideEffects::none()
        );
        assert_eq!(
            report.recommended_next_gate,
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
