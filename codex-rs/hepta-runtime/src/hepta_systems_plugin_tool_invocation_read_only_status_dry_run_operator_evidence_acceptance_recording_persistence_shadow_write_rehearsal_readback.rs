use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_tool_registry_shadow_registration_lookup_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_persistence_open_preconditions_readback_ready: bool,
    pub candidate_count: usize,
    pub shadow_write_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub source_persistence_open_precondition_set_linked_count: usize,
    pub source_persistence_open_denial_receipt_linked_count: usize,
    pub source_persistence_open_idempotency_key_linked_count: usize,
    pub source_acceptance_record_store_binding_precondition_linked_count: usize,
    pub source_acceptance_record_idempotency_index_precondition_linked_count: usize,
    pub source_ledger_store_binding_precondition_linked_count: usize,
    pub source_receipt_store_binding_precondition_linked_count: usize,
    pub source_runtime_event_log_store_binding_precondition_linked_count: usize,
    pub source_rollback_anchor_precondition_linked_count: usize,
    pub source_kill_switch_precondition_linked_count: usize,
    pub shadow_acceptance_record_envelope_projected_count: usize,
    pub shadow_write_intent_projected_count: usize,
    pub shadow_write_payload_digest_projected_count: usize,
    pub shadow_idempotency_replay_key_projected_count: usize,
    pub shadow_receipt_preview_projected_count: usize,
    pub shadow_store_target_projected_count: usize,
    pub shadow_replay_result_projected_count: usize,
    pub shadow_write_rehearsal_item_count: usize,
    pub stable_shadow_acceptance_record_envelope_count: usize,
    pub unique_shadow_acceptance_record_envelope_count: usize,
    pub stable_shadow_write_payload_digest_count: usize,
    pub unique_shadow_write_payload_digest_count: usize,
    pub stable_shadow_idempotency_replay_key_count: usize,
    pub unique_shadow_idempotency_replay_key_count: usize,
    pub shadow_acceptance_record_envelope_mismatch_count: usize,
    pub duplicate_shadow_acceptance_record_envelope_count: usize,
    pub shadow_write_payload_digest_mismatch_count: usize,
    pub duplicate_shadow_write_payload_digest_count: usize,
    pub shadow_idempotency_replay_mismatch_count: usize,
    pub duplicate_shadow_idempotency_replay_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_evidence_packet_sent_count: usize,
    pub operator_evidence_packet_persisted_count: usize,
    pub operator_evidence_recorded_count: usize,
    pub operator_acceptance_recorded_count: usize,
    pub acceptance_record_persisted_count: usize,
    pub shadow_write_executed_count: usize,
    pub shadow_write_materialized_count: usize,
    pub shadow_store_written_count: usize,
    pub test_tmp_written_count: usize,
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
    pub persistence_shadow_write_rehearsal_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_evidence_packet_send_allowed: bool,
    pub operator_evidence_packet_persistence_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub acceptance_record_persistence_allowed: bool,
    pub shadow_write_execution_allowed: bool,
    pub shadow_store_write_allowed: bool,
    pub test_tmp_write_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackEntry
{
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_persistence_open_precondition_set_id: String,
    pub source_persistence_open_denial_receipt_id: String,
    pub source_persistence_open_idempotency_key: String,
    pub source_acceptance_record_store_binding_precondition_id: String,
    pub source_acceptance_record_idempotency_index_precondition_id: String,
    pub source_ledger_store_binding_precondition_id: String,
    pub source_receipt_store_binding_precondition_id: String,
    pub source_runtime_event_log_store_binding_precondition_id: String,
    pub source_rollback_anchor_precondition_id: String,
    pub source_kill_switch_precondition_id: String,
    pub shadow_acceptance_record_envelope_id: String,
    pub shadow_write_intent_id: String,
    pub shadow_write_payload_digest: String,
    pub shadow_idempotency_replay_key: String,
    pub shadow_receipt_preview_id: String,
    pub shadow_store_target_id: String,
    pub shadow_replay_result_id: String,
    pub first_shadow_acceptance_record_envelope_id: String,
    pub second_shadow_acceptance_record_envelope_id: String,
    pub first_shadow_write_payload_digest: String,
    pub second_shadow_write_payload_digest: String,
    pub first_shadow_idempotency_replay_key: String,
    pub second_shadow_idempotency_replay_key: String,
    pub source_persistence_open_precondition_set_linked: bool,
    pub source_persistence_open_denial_receipt_linked: bool,
    pub source_persistence_open_idempotency_key_linked: bool,
    pub source_acceptance_record_store_binding_precondition_linked: bool,
    pub source_acceptance_record_idempotency_index_precondition_linked: bool,
    pub source_ledger_store_binding_precondition_linked: bool,
    pub source_receipt_store_binding_precondition_linked: bool,
    pub source_runtime_event_log_store_binding_precondition_linked: bool,
    pub source_rollback_anchor_precondition_linked: bool,
    pub source_kill_switch_precondition_linked: bool,
    pub shadow_acceptance_record_envelope_projected: bool,
    pub shadow_write_intent_projected: bool,
    pub shadow_write_payload_digest_projected: bool,
    pub shadow_idempotency_replay_key_projected: bool,
    pub shadow_receipt_preview_projected: bool,
    pub shadow_store_target_projected: bool,
    pub shadow_replay_result_projected: bool,
    pub stable_shadow_acceptance_record_envelope: bool,
    pub unique_shadow_acceptance_record_envelope: bool,
    pub stable_shadow_write_payload_digest: bool,
    pub unique_shadow_write_payload_digest: bool,
    pub stable_shadow_idempotency_replay_key: bool,
    pub unique_shadow_idempotency_replay_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub shadow_write_executed: bool,
    pub shadow_write_materialized: bool,
    pub shadow_store_written: bool,
    pub test_tmp_written: bool,
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
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_evidence_packet_sent: bool,
    pub operator_evidence_packet_persisted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub acceptance_record_persisted: bool,
    pub shadow_write_executed: bool,
    pub shadow_write_materialized: bool,
    pub shadow_store_written: bool,
    pub test_tmp_written: bool,
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

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report(
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport{
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport{
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_entries(source);
    let shadow_write_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let source_persistence_open_precondition_set_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_open_precondition_set_linked)
        .count();
    let source_persistence_open_denial_receipt_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_open_denial_receipt_linked)
        .count();
    let source_persistence_open_idempotency_key_linked_count = entries
        .iter()
        .filter(|entry| entry.source_persistence_open_idempotency_key_linked)
        .count();
    let source_acceptance_record_store_binding_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_acceptance_record_store_binding_precondition_linked)
        .count();
    let source_acceptance_record_idempotency_index_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_acceptance_record_idempotency_index_precondition_linked)
        .count();
    let source_ledger_store_binding_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_ledger_store_binding_precondition_linked)
        .count();
    let source_receipt_store_binding_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_receipt_store_binding_precondition_linked)
        .count();
    let source_runtime_event_log_store_binding_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_runtime_event_log_store_binding_precondition_linked)
        .count();
    let source_rollback_anchor_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_rollback_anchor_precondition_linked)
        .count();
    let source_kill_switch_precondition_linked_count = entries
        .iter()
        .filter(|entry| entry.source_kill_switch_precondition_linked)
        .count();
    let shadow_acceptance_record_envelope_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_acceptance_record_envelope_projected)
        .count();
    let shadow_write_intent_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_write_intent_projected)
        .count();
    let shadow_write_payload_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_write_payload_digest_projected)
        .count();
    let shadow_idempotency_replay_key_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_idempotency_replay_key_projected)
        .count();
    let shadow_receipt_preview_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_receipt_preview_projected)
        .count();
    let shadow_store_target_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_store_target_projected)
        .count();
    let shadow_replay_result_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_replay_result_projected)
        .count();
    let shadow_write_rehearsal_item_count = shadow_acceptance_record_envelope_projected_count
        + shadow_write_intent_projected_count
        + shadow_write_payload_digest_projected_count
        + shadow_idempotency_replay_key_projected_count
        + shadow_receipt_preview_projected_count
        + shadow_store_target_projected_count
        + shadow_replay_result_projected_count;
    let stable_shadow_acceptance_record_envelope_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_acceptance_record_envelope)
        .count();
    let unique_shadow_acceptance_record_envelope_count = entries
        .iter()
        .map(|entry| entry.first_shadow_acceptance_record_envelope_id.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_shadow_write_payload_digest_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_write_payload_digest)
        .count();
    let unique_shadow_write_payload_digest_count = entries
        .iter()
        .map(|entry| entry.first_shadow_write_payload_digest.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_shadow_idempotency_replay_key_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_idempotency_replay_key)
        .count();
    let unique_shadow_idempotency_replay_key_count = entries
        .iter()
        .map(|entry| entry.first_shadow_idempotency_replay_key.as_str())
        .collect::<HashSet<_>>()
        .len();
    let shadow_acceptance_record_envelope_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_acceptance_record_envelope)
        .count();
    let duplicate_shadow_acceptance_record_envelope_count =
        shadow_write_entry_count.saturating_sub(unique_shadow_acceptance_record_envelope_count);
    let shadow_write_payload_digest_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_write_payload_digest)
        .count();
    let duplicate_shadow_write_payload_digest_count =
        shadow_write_entry_count.saturating_sub(unique_shadow_write_payload_digest_count);
    let shadow_idempotency_replay_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_idempotency_replay_key)
        .count();
    let duplicate_shadow_idempotency_replay_key_count =
        shadow_write_entry_count.saturating_sub(unique_shadow_idempotency_replay_key_count);
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
    let shadow_write_executed_count = entries
        .iter()
        .filter(|entry| entry.shadow_write_executed)
        .count();
    let shadow_write_materialized_count = entries
        .iter()
        .filter(|entry| entry.shadow_write_materialized)
        .count();
    let shadow_store_written_count = entries
        .iter()
        .filter(|entry| entry.shadow_store_written)
        .count();
    let test_tmp_written_count = entries
        .iter()
        .filter(|entry| entry.test_tmp_written)
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

    let persistence_shadow_write_rehearsal_readback_ready = source
        .persistence_open_preconditions_readback_ready
        && source.candidate_count == 2
        && source.precondition_entry_count == 2
        && source.persistence_open_precondition_set_projected_count == 2
        && source.acceptance_record_store_binding_precondition_projected_count == 2
        && source.acceptance_record_idempotency_index_precondition_projected_count == 2
        && source.ledger_store_binding_precondition_projected_count == 2
        && source.receipt_store_binding_precondition_projected_count == 2
        && source.runtime_event_log_store_binding_precondition_projected_count == 2
        && source.rollback_anchor_precondition_projected_count == 2
        && source.kill_switch_precondition_projected_count == 2
        && source.acceptance_record_persisted_count == 0
        && source.idempotency_index_written_count == 0
        && source.runtime_event_log_written_count == 0
        && shadow_write_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && source_persistence_open_precondition_set_linked_count == 2
        && source_persistence_open_denial_receipt_linked_count == 2
        && source_persistence_open_idempotency_key_linked_count == 2
        && source_acceptance_record_store_binding_precondition_linked_count == 2
        && source_acceptance_record_idempotency_index_precondition_linked_count == 2
        && source_ledger_store_binding_precondition_linked_count == 2
        && source_receipt_store_binding_precondition_linked_count == 2
        && source_runtime_event_log_store_binding_precondition_linked_count == 2
        && source_rollback_anchor_precondition_linked_count == 2
        && source_kill_switch_precondition_linked_count == 2
        && shadow_acceptance_record_envelope_projected_count == 2
        && shadow_write_intent_projected_count == 2
        && shadow_write_payload_digest_projected_count == 2
        && shadow_idempotency_replay_key_projected_count == 2
        && shadow_receipt_preview_projected_count == 2
        && shadow_store_target_projected_count == 2
        && shadow_replay_result_projected_count == 2
        && shadow_write_rehearsal_item_count == 14
        && stable_shadow_acceptance_record_envelope_count == 2
        && unique_shadow_acceptance_record_envelope_count == 2
        && stable_shadow_write_payload_digest_count == 2
        && unique_shadow_write_payload_digest_count == 2
        && stable_shadow_idempotency_replay_key_count == 2
        && unique_shadow_idempotency_replay_key_count == 2
        && shadow_acceptance_record_envelope_mismatch_count == 0
        && duplicate_shadow_acceptance_record_envelope_count == 0
        && shadow_write_payload_digest_mismatch_count == 0
        && duplicate_shadow_write_payload_digest_count == 0
        && shadow_idempotency_replay_mismatch_count == 0
        && duplicate_shadow_idempotency_replay_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_evidence_packet_sent_count == 0
        && operator_evidence_packet_persisted_count == 0
        && operator_evidence_recorded_count == 0
        && operator_acceptance_recorded_count == 0
        && acceptance_record_persisted_count == 0
        && shadow_write_executed_count == 0
        && shadow_write_materialized_count == 0
        && shadow_store_written_count == 0
        && test_tmp_written_count == 0
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

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback",
        status: if persistence_shadow_write_rehearsal_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_persistence_open_preconditions_readback_ready: source
            .persistence_open_preconditions_readback_ready,
        candidate_count: source.candidate_count,
        shadow_write_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        source_persistence_open_precondition_set_linked_count,
        source_persistence_open_denial_receipt_linked_count,
        source_persistence_open_idempotency_key_linked_count,
        source_acceptance_record_store_binding_precondition_linked_count,
        source_acceptance_record_idempotency_index_precondition_linked_count,
        source_ledger_store_binding_precondition_linked_count,
        source_receipt_store_binding_precondition_linked_count,
        source_runtime_event_log_store_binding_precondition_linked_count,
        source_rollback_anchor_precondition_linked_count,
        source_kill_switch_precondition_linked_count,
        shadow_acceptance_record_envelope_projected_count,
        shadow_write_intent_projected_count,
        shadow_write_payload_digest_projected_count,
        shadow_idempotency_replay_key_projected_count,
        shadow_receipt_preview_projected_count,
        shadow_store_target_projected_count,
        shadow_replay_result_projected_count,
        shadow_write_rehearsal_item_count,
        stable_shadow_acceptance_record_envelope_count,
        unique_shadow_acceptance_record_envelope_count,
        stable_shadow_write_payload_digest_count,
        unique_shadow_write_payload_digest_count,
        stable_shadow_idempotency_replay_key_count,
        unique_shadow_idempotency_replay_key_count,
        shadow_acceptance_record_envelope_mismatch_count,
        duplicate_shadow_acceptance_record_envelope_count,
        shadow_write_payload_digest_mismatch_count,
        duplicate_shadow_write_payload_digest_count,
        shadow_idempotency_replay_mismatch_count,
        duplicate_shadow_idempotency_replay_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_evidence_packet_sent_count,
        operator_evidence_packet_persisted_count,
        operator_evidence_recorded_count,
        operator_acceptance_recorded_count,
        acceptance_record_persisted_count,
        shadow_write_executed_count,
        shadow_write_materialized_count,
        shadow_store_written_count,
        test_tmp_written_count,
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
        persistence_shadow_write_rehearsal_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_evidence_packet_send_allowed: false,
        operator_evidence_packet_persistence_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
        acceptance_record_persistence_allowed: false,
        shadow_write_execution_allowed: false,
        shadow_store_write_allowed: false,
        test_tmp_write_allowed: false,
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
            "operator_acceptance_unrecorded",
            "acceptance_record_store_binding_absent",
            "acceptance_record_idempotency_index_absent",
            "shadow_write_execution_disabled",
            "shadow_store_write_disabled",
            "test_tmp_write_disabled",
            "ledger_store_binding_absent",
            "receipt_store_binding_absent",
            "runtime_event_log_store_binding_absent",
            "rollback_anchor_absent",
            "kill_switch_unrehearsed",
            "tool_registry_registration_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "connector_start_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceOpenPreconditionsReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackEntry>{
    source
        .entries
        .iter()
        .map(|entry| {
            let kind = entry.contribution_kind;
            let envelope_id = shadow_acceptance_record_envelope_id(kind);
            let digest = shadow_write_payload_digest(kind);
            let replay_key = shadow_idempotency_replay_key(kind);
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_persistence_open_precondition_set_id: entry.persistence_open_precondition_set_id.clone(),
                source_persistence_open_denial_receipt_id: entry.persistence_open_denial_receipt_id.clone(),
                source_persistence_open_idempotency_key: entry.persistence_open_idempotency_key.clone(),
                source_acceptance_record_store_binding_precondition_id: entry.acceptance_record_store_binding_precondition_id.clone(),
                source_acceptance_record_idempotency_index_precondition_id: entry.acceptance_record_idempotency_index_precondition_id.clone(),
                source_ledger_store_binding_precondition_id: entry.ledger_store_binding_precondition_id.clone(),
                source_receipt_store_binding_precondition_id: entry.receipt_store_binding_precondition_id.clone(),
                source_runtime_event_log_store_binding_precondition_id: entry.runtime_event_log_store_binding_precondition_id.clone(),
                source_rollback_anchor_precondition_id: entry.rollback_anchor_precondition_id.clone(),
                source_kill_switch_precondition_id: entry.kill_switch_precondition_id.clone(),
                shadow_acceptance_record_envelope_id: envelope_id.clone(),
                shadow_write_intent_id: shadow_write_intent_id(kind),
                shadow_write_payload_digest: digest.clone(),
                shadow_idempotency_replay_key: replay_key.clone(),
                shadow_receipt_preview_id: shadow_receipt_preview_id(kind),
                shadow_store_target_id: shadow_store_target_id(kind),
                shadow_replay_result_id: shadow_replay_result_id(kind),
                first_shadow_acceptance_record_envelope_id: envelope_id.clone(),
                second_shadow_acceptance_record_envelope_id: envelope_id,
                first_shadow_write_payload_digest: digest.clone(),
                second_shadow_write_payload_digest: digest,
                first_shadow_idempotency_replay_key: replay_key.clone(),
                second_shadow_idempotency_replay_key: replay_key,
                source_persistence_open_precondition_set_linked: true,
                source_persistence_open_denial_receipt_linked: true,
                source_persistence_open_idempotency_key_linked: true,
                source_acceptance_record_store_binding_precondition_linked: true,
                source_acceptance_record_idempotency_index_precondition_linked: true,
                source_ledger_store_binding_precondition_linked: true,
                source_receipt_store_binding_precondition_linked: true,
                source_runtime_event_log_store_binding_precondition_linked: true,
                source_rollback_anchor_precondition_linked: true,
                source_kill_switch_precondition_linked: true,
                shadow_acceptance_record_envelope_projected: true,
                shadow_write_intent_projected: true,
                shadow_write_payload_digest_projected: true,
                shadow_idempotency_replay_key_projected: true,
                shadow_receipt_preview_projected: true,
                shadow_store_target_projected: true,
                shadow_replay_result_projected: true,
                stable_shadow_acceptance_record_envelope: true,
                unique_shadow_acceptance_record_envelope: true,
                stable_shadow_write_payload_digest: true,
                unique_shadow_write_payload_digest: true,
                stable_shadow_idempotency_replay_key: true,
                unique_shadow_idempotency_replay_key: true,
                feature_gate_opened: entry.feature_gate_opened,
                dry_run_executed: entry.dry_run_executed,
                operator_evidence_packet_sent: entry.operator_evidence_packet_sent,
                operator_evidence_packet_persisted: entry.operator_evidence_packet_persisted,
                operator_evidence_recorded: entry.operator_evidence_recorded,
                operator_acceptance_recorded: entry.operator_acceptance_recorded,
                acceptance_record_persisted: entry.acceptance_record_persisted,
                shadow_write_executed: false,
                shadow_write_materialized: false,
                shadow_store_written: false,
                test_tmp_written: false,
                persistence_open_denial_receipt_persisted: entry.persistence_open_denial_receipt_persisted,
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

fn shadow_acceptance_record_envelope_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-envelope:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_write_intent_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-write-intent:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_write_payload_digest(contribution_kind: &str) -> String {
    format!(
        "sha256:operator-evidence-acceptance-recording-persistence-shadow-payload:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_idempotency_replay_key(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-idempotency-replay:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_receipt_preview_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-receipt-preview:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_store_target_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-store-target:hepta-system:{}:no-store-bound",
        suffix(contribution_kind)
    )
}

fn shadow_replay_result_id(contribution_kind: &str) -> String {
    format!(
        "operator-evidence-acceptance-recording-persistence-shadow-replay-result:hepta-system:{}:not-executed",
        suffix(contribution_kind)
    )
}

impl HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackSideEffects {
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
            shadow_write_executed: false,
            shadow_write_materialized: false,
            shadow_store_written: false,
            test_tmp_written: false,
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
    fn shadow_write_rehearsal_projects_stable_write_envelopes() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_persistence_open_preconditions_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.shadow_write_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(
            report.source_persistence_open_precondition_set_linked_count,
            2
        );
        assert_eq!(
            report.source_persistence_open_denial_receipt_linked_count,
            2
        );
        assert_eq!(
            report.source_persistence_open_idempotency_key_linked_count,
            2
        );
        assert_eq!(report.shadow_acceptance_record_envelope_projected_count, 2);
        assert_eq!(report.shadow_write_intent_projected_count, 2);
        assert_eq!(report.shadow_write_payload_digest_projected_count, 2);
        assert_eq!(report.shadow_idempotency_replay_key_projected_count, 2);
        assert_eq!(report.shadow_receipt_preview_projected_count, 2);
        assert_eq!(report.shadow_store_target_projected_count, 2);
        assert_eq!(report.shadow_replay_result_projected_count, 2);
        assert_eq!(report.shadow_write_rehearsal_item_count, 14);
        assert!(report.persistence_shadow_write_rehearsal_readback_ready);
    }

    #[test]
    fn shadow_write_rehearsal_stays_stable_unique_and_unexecuted() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report();

        assert_eq!(report.stable_shadow_acceptance_record_envelope_count, 2);
        assert_eq!(report.unique_shadow_acceptance_record_envelope_count, 2);
        assert_eq!(report.stable_shadow_write_payload_digest_count, 2);
        assert_eq!(report.unique_shadow_write_payload_digest_count, 2);
        assert_eq!(report.stable_shadow_idempotency_replay_key_count, 2);
        assert_eq!(report.unique_shadow_idempotency_replay_key_count, 2);
        assert_eq!(report.shadow_acceptance_record_envelope_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_acceptance_record_envelope_count, 0);
        assert_eq!(report.shadow_write_payload_digest_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_write_payload_digest_count, 0);
        assert_eq!(report.shadow_idempotency_replay_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_idempotency_replay_key_count, 0);
        assert_eq!(report.shadow_write_executed_count, 0);
        assert_eq!(report.shadow_write_materialized_count, 0);
        assert_eq!(report.shadow_store_written_count, 0);
        assert_eq!(report.test_tmp_written_count, 0);
    }

    #[test]
    fn shadow_write_rehearsal_keeps_all_persistence_registry_and_live_paths_closed() {
        let report = hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
        assert_eq!(report.operator_evidence_packet_sent_count, 0);
        assert_eq!(report.operator_evidence_packet_persisted_count, 0);
        assert_eq!(report.operator_evidence_recorded_count, 0);
        assert_eq!(report.operator_acceptance_recorded_count, 0);
        assert_eq!(report.acceptance_record_persisted_count, 0);
        assert_eq!(report.persistence_open_denial_receipt_persisted_count, 0);
        assert_eq!(report.persistence_denial_receipt_persisted_count, 0);
        assert_eq!(report.non_recording_denial_receipt_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.acceptance_record_persistence_allowed);
        assert!(!report.shadow_write_execution_allowed);
        assert!(!report.shadow_store_write_allowed);
        assert!(!report.test_tmp_write_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackSideEffects::none()
        );
        assert_eq!(
            report.recommended_next_gate,
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_SHADOW_WRITE_REHEARSAL_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
