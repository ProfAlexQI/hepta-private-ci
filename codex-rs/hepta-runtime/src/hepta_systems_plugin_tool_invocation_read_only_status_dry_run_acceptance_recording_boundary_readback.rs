use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorPacketReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback";

const OPERATOR_CHECKLIST_CLOSURE_ITEMS: [&str; 5] = [
    "confirm_operator_packet_is_query_only",
    "confirm_acceptance_recording_is_disabled",
    "confirm_ledger_and_receipt_persistence_remain_disabled",
    "confirm_tool_registration_and_invocation_remain_disabled",
    "confirm_runtime_and_live_remain_disabled",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_operator_packet_readback_ready: bool,
    pub candidate_count: usize,
    pub boundary_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub acceptance_record_id_projected_count: usize,
    pub non_recording_denial_receipt_projected_count: usize,
    pub ledger_preview_anchor_projected_count: usize,
    pub receipt_preview_anchor_projected_count: usize,
    pub operator_checklist_closure_projected_count: usize,
    pub operator_checklist_closure_item_count: usize,
    pub acceptance_idempotency_key_projected_count: usize,
    pub stable_acceptance_record_count: usize,
    pub unique_acceptance_record_count: usize,
    pub stable_non_recording_denial_receipt_count: usize,
    pub unique_non_recording_denial_receipt_count: usize,
    pub stable_acceptance_idempotency_key_count: usize,
    pub unique_acceptance_idempotency_key_count: usize,
    pub acceptance_record_mismatch_count: usize,
    pub duplicate_acceptance_record_count: usize,
    pub non_recording_denial_receipt_mismatch_count: usize,
    pub duplicate_non_recording_denial_receipt_count: usize,
    pub acceptance_idempotency_mismatch_count: usize,
    pub duplicate_acceptance_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub operator_checklist_persisted_count: usize,
    pub non_acceptance_receipt_persisted_count: usize,
    pub acceptance_record_persisted_count: usize,
    pub operator_acceptance_recorded_count: usize,
    pub non_recording_denial_receipt_persisted_count: usize,
    pub operator_checklist_closure_persisted_count: usize,
    pub dry_run_receipt_preview_persisted_count: usize,
    pub ledger_preview_persisted_count: usize,
    pub policy_decision_persisted_count: usize,
    pub approval_preflight_executed_count: usize,
    pub ledger_write_attempted_count: usize,
    pub receipt_projection_persisted_count: usize,
    pub tool_registered_count: usize,
    pub tool_registry_mutated_count: usize,
    pub registry_lookup_executed_count: usize,
    pub tool_invoked_count: usize,
    pub noop_result_persisted_count: usize,
    pub ledger_written_count: usize,
    pub approval_requested_count: usize,
    pub receipt_persisted_count: usize,
    pub dynamic_activation_started_count: usize,
    pub permission_granted_count: usize,
    pub mcp_server_started_count: usize,
    pub app_connector_started_count: usize,
    pub plugin_installed_count: usize,
    pub cache_materialized_count: usize,
    pub cache_mutated_count: usize,
    pub runtime_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_execution_started_count: usize,
    pub acceptance_recording_boundary_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_checklist_persistence_allowed: bool,
    pub non_acceptance_receipt_persistence_allowed: bool,
    pub acceptance_record_persistence_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub non_recording_denial_receipt_persistence_allowed: bool,
    pub operator_checklist_closure_persistence_allowed: bool,
    pub dry_run_receipt_preview_persistence_allowed: bool,
    pub ledger_preview_persistence_allowed: bool,
    pub policy_decision_persistence_allowed: bool,
    pub approval_preflight_execution_allowed: bool,
    pub ledger_write_allowed: bool,
    pub receipt_projection_persistence_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_registry_mutation_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub noop_result_persistence_allowed: bool,
    pub approval_request_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub permission_grant_allowed: bool,
    pub mcp_server_start_allowed: bool,
    pub app_connector_start_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<
        HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackEntry,
    >,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackEntry
{
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_operator_packet_id: &'static str,
    pub source_operator_checklist_id: &'static str,
    pub source_non_acceptance_receipt_id: &'static str,
    pub source_ledger_preview_link_id: &'static str,
    pub source_receipt_preview_link_id: &'static str,
    pub source_policy_denial_anchor_link_id: &'static str,
    pub source_approval_denial_anchor_link_id: &'static str,
    pub source_operator_packet_idempotency_key: &'static str,
    pub acceptance_record_id: &'static str,
    pub non_recording_denial_receipt_id: &'static str,
    pub ledger_preview_anchor_id: &'static str,
    pub receipt_preview_anchor_id: &'static str,
    pub operator_checklist_closure_id: &'static str,
    pub operator_checklist_closure_items: Vec<&'static str>,
    pub acceptance_idempotency_key: &'static str,
    pub first_acceptance_record_id: &'static str,
    pub second_acceptance_record_id: &'static str,
    pub first_non_recording_denial_receipt_id: &'static str,
    pub second_non_recording_denial_receipt_id: &'static str,
    pub first_acceptance_idempotency_key: &'static str,
    pub second_acceptance_idempotency_key: &'static str,
    pub acceptance_record_id_projected: bool,
    pub non_recording_denial_receipt_projected: bool,
    pub ledger_preview_anchor_projected: bool,
    pub receipt_preview_anchor_projected: bool,
    pub operator_checklist_closure_projected: bool,
    pub acceptance_idempotency_key_projected: bool,
    pub stable_acceptance_record: bool,
    pub unique_acceptance_record: bool,
    pub stable_non_recording_denial_receipt: bool,
    pub unique_non_recording_denial_receipt: bool,
    pub stable_acceptance_idempotency_key: bool,
    pub unique_acceptance_idempotency_key: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub operator_checklist_persisted: bool,
    pub non_acceptance_receipt_persisted: bool,
    pub acceptance_record_persisted: bool,
    pub operator_acceptance_recorded: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub operator_checklist_closure_persisted: bool,
    pub dry_run_receipt_preview_persisted: bool,
    pub ledger_preview_persisted: bool,
    pub policy_decision_persisted: bool,
    pub approval_preflight_executed: bool,
    pub ledger_write_attempted: bool,
    pub receipt_projection_persisted: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub noop_result_persisted: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub receipt_persisted: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub plugin_installed: bool,
    pub cache_materialized: bool,
    pub cache_mutated: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackSideEffects
{
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub operator_checklist_persisted: bool,
    pub non_acceptance_receipt_persisted: bool,
    pub acceptance_record_persisted: bool,
    pub operator_acceptance_recorded: bool,
    pub non_recording_denial_receipt_persisted: bool,
    pub operator_checklist_closure_persisted: bool,
    pub dry_run_receipt_preview_persisted: bool,
    pub ledger_preview_persisted: bool,
    pub policy_decision_persisted: bool,
    pub approval_preflight_executed: bool,
    pub ledger_write_attempted: bool,
    pub receipt_projection_persisted: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub noop_result_persisted: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub receipt_persisted: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub install_cache_materialized: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub credential_read: bool,
    pub external_network_used: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report()
-> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackReport {
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_report();
    hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorPacketReadbackReport,
) -> HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackReport {
    let entries =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_entries(source);
    let boundary_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let acceptance_record_id_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_id_projected)
        .count();
    let non_recording_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.non_recording_denial_receipt_projected)
        .count();
    let ledger_preview_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_preview_anchor_projected)
        .count();
    let receipt_preview_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_preview_anchor_projected)
        .count();
    let operator_checklist_closure_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_checklist_closure_projected)
        .count();
    let operator_checklist_closure_item_count = entries
        .iter()
        .map(|entry| entry.operator_checklist_closure_items.len())
        .sum();
    let acceptance_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_idempotency_key_projected)
        .count();
    let stable_acceptance_record_count = entries
        .iter()
        .filter(|entry| entry.stable_acceptance_record)
        .count();
    let unique_acceptance_record_count = entries
        .iter()
        .map(|entry| entry.first_acceptance_record_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_non_recording_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_non_recording_denial_receipt)
        .count();
    let unique_non_recording_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_non_recording_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let stable_acceptance_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_acceptance_idempotency_key)
        .count();
    let unique_acceptance_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_acceptance_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let acceptance_record_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_acceptance_record)
        .count();
    let duplicate_acceptance_record_count =
        boundary_entry_count.saturating_sub(unique_acceptance_record_count);
    let non_recording_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_non_recording_denial_receipt)
        .count();
    let duplicate_non_recording_denial_receipt_count =
        boundary_entry_count.saturating_sub(unique_non_recording_denial_receipt_count);
    let acceptance_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_acceptance_idempotency_key)
        .count();
    let duplicate_acceptance_idempotency_key_count =
        boundary_entry_count.saturating_sub(unique_acceptance_idempotency_key_count);
    let feature_gate_opened_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_opened)
        .count();
    let dry_run_executed_count = entries
        .iter()
        .filter(|entry| entry.dry_run_executed)
        .count();
    let operator_packet_sent_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_sent)
        .count();
    let operator_packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_persisted)
        .count();
    let operator_checklist_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_checklist_persisted)
        .count();
    let non_acceptance_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_acceptance_receipt_persisted)
        .count();
    let acceptance_record_persisted_count = entries
        .iter()
        .filter(|entry| entry.acceptance_record_persisted)
        .count();
    let operator_acceptance_recorded_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_recorded)
        .count();
    let non_recording_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_recording_denial_receipt_persisted)
        .count();
    let operator_checklist_closure_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_checklist_closure_persisted)
        .count();
    let dry_run_receipt_preview_persisted_count = entries
        .iter()
        .filter(|entry| entry.dry_run_receipt_preview_persisted)
        .count();
    let ledger_preview_persisted_count = entries
        .iter()
        .filter(|entry| entry.ledger_preview_persisted)
        .count();
    let policy_decision_persisted_count = entries
        .iter()
        .filter(|entry| entry.policy_decision_persisted)
        .count();
    let approval_preflight_executed_count = entries
        .iter()
        .filter(|entry| entry.approval_preflight_executed)
        .count();
    let ledger_write_attempted_count = entries
        .iter()
        .filter(|entry| entry.ledger_write_attempted)
        .count();
    let receipt_projection_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_projection_persisted)
        .count();
    let tool_registered_count = entries.iter().filter(|entry| entry.tool_registered).count();
    let tool_registry_mutated_count = entries
        .iter()
        .filter(|entry| entry.tool_registry_mutated)
        .count();
    let registry_lookup_executed_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_executed)
        .count();
    let tool_invoked_count = entries.iter().filter(|entry| entry.tool_invoked).count();
    let noop_result_persisted_count = entries
        .iter()
        .filter(|entry| entry.noop_result_persisted)
        .count();
    let ledger_written_count = entries.iter().filter(|entry| entry.ledger_written).count();
    let approval_requested_count = entries
        .iter()
        .filter(|entry| entry.approval_requested)
        .count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted)
        .count();
    let dynamic_activation_started_count = entries
        .iter()
        .filter(|entry| entry.dynamic_activation_started)
        .count();
    let permission_granted_count = entries
        .iter()
        .filter(|entry| entry.permission_granted)
        .count();
    let mcp_server_started_count = entries
        .iter()
        .filter(|entry| entry.mcp_server_started)
        .count();
    let app_connector_started_count = entries
        .iter()
        .filter(|entry| entry.app_connector_started)
        .count();
    let plugin_installed_count = entries
        .iter()
        .filter(|entry| entry.plugin_installed)
        .count();
    let cache_materialized_count = entries
        .iter()
        .filter(|entry| entry.cache_materialized)
        .count();
    let cache_mutated_count = entries.iter().filter(|entry| entry.cache_mutated).count();
    let runtime_event_log_written_count = entries
        .iter()
        .filter(|entry| entry.runtime_event_log_written)
        .count();
    let sqlite_written_count = entries.iter().filter(|entry| entry.sqlite_written).count();
    let live_execution_started_count = entries
        .iter()
        .filter(|entry| entry.live_execution_started)
        .count();

    let acceptance_recording_boundary_readback_ready = source.operator_packet_readback_ready
        && source.candidate_count == 2
        && source.packet_entry_count == 2
        && source.selected_read_only_status_tool_count == 1
        && source.non_selected_preflight_boundary_count == 1
        && source.operator_packet_id_projected_count == 2
        && source.operator_checklist_projected_count == 2
        && source.non_acceptance_receipt_projected_count == 2
        && source.ledger_preview_link_projected_count == 2
        && source.receipt_preview_link_projected_count == 2
        && source.stable_operator_packet_count == 2
        && source.unique_operator_packet_count == 2
        && source.operator_acceptance_recorded_count == 0
        && source.operator_packet_sent_count == 0
        && source.operator_packet_persisted_count == 0
        && source.feature_gate_opened_count == 0
        && source.dry_run_executed_count == 0
        && source.tool_invoked_count == 0
        && source.ledger_written_count == 0
        && source.receipt_persisted_count == 0
        && source.live_execution_started_count == 0
        && boundary_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && acceptance_record_id_projected_count == 2
        && non_recording_denial_receipt_projected_count == 2
        && ledger_preview_anchor_projected_count == 2
        && receipt_preview_anchor_projected_count == 2
        && operator_checklist_closure_projected_count == 2
        && operator_checklist_closure_item_count == 10
        && acceptance_idempotency_key_projected_count == 2
        && stable_acceptance_record_count == 2
        && unique_acceptance_record_count == 2
        && stable_non_recording_denial_receipt_count == 2
        && unique_non_recording_denial_receipt_count == 2
        && stable_acceptance_idempotency_key_count == 2
        && unique_acceptance_idempotency_key_count == 2
        && acceptance_record_mismatch_count == 0
        && duplicate_acceptance_record_count == 0
        && non_recording_denial_receipt_mismatch_count == 0
        && duplicate_non_recording_denial_receipt_count == 0
        && acceptance_idempotency_mismatch_count == 0
        && duplicate_acceptance_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && operator_checklist_persisted_count == 0
        && non_acceptance_receipt_persisted_count == 0
        && acceptance_record_persisted_count == 0
        && operator_acceptance_recorded_count == 0
        && non_recording_denial_receipt_persisted_count == 0
        && operator_checklist_closure_persisted_count == 0
        && dry_run_receipt_preview_persisted_count == 0
        && ledger_preview_persisted_count == 0
        && policy_decision_persisted_count == 0
        && approval_preflight_executed_count == 0
        && ledger_write_attempted_count == 0
        && receipt_projection_persisted_count == 0
        && tool_registered_count == 0
        && tool_registry_mutated_count == 0
        && registry_lookup_executed_count == 0
        && tool_invoked_count == 0
        && noop_result_persisted_count == 0
        && ledger_written_count == 0
        && approval_requested_count == 0
        && receipt_persisted_count == 0
        && dynamic_activation_started_count == 0
        && permission_granted_count == 0
        && mcp_server_started_count == 0
        && app_connector_started_count == 0
        && plugin_installed_count == 0
        && cache_materialized_count == 0
        && cache_mutated_count == 0
        && runtime_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_execution_started_count == 0;

    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback",
        status: if acceptance_recording_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_GATE,
        schema_version: HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_operator_packet_readback_ready: source.operator_packet_readback_ready,
        candidate_count: source.candidate_count,
        boundary_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        acceptance_record_id_projected_count,
        non_recording_denial_receipt_projected_count,
        ledger_preview_anchor_projected_count,
        receipt_preview_anchor_projected_count,
        operator_checklist_closure_projected_count,
        operator_checklist_closure_item_count,
        acceptance_idempotency_key_projected_count,
        stable_acceptance_record_count,
        unique_acceptance_record_count,
        stable_non_recording_denial_receipt_count,
        unique_non_recording_denial_receipt_count,
        stable_acceptance_idempotency_key_count,
        unique_acceptance_idempotency_key_count,
        acceptance_record_mismatch_count,
        duplicate_acceptance_record_count,
        non_recording_denial_receipt_mismatch_count,
        duplicate_non_recording_denial_receipt_count,
        acceptance_idempotency_mismatch_count,
        duplicate_acceptance_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        operator_checklist_persisted_count,
        non_acceptance_receipt_persisted_count,
        acceptance_record_persisted_count,
        operator_acceptance_recorded_count,
        non_recording_denial_receipt_persisted_count,
        operator_checklist_closure_persisted_count,
        dry_run_receipt_preview_persisted_count,
        ledger_preview_persisted_count,
        policy_decision_persisted_count,
        approval_preflight_executed_count,
        ledger_write_attempted_count,
        receipt_projection_persisted_count,
        tool_registered_count,
        tool_registry_mutated_count,
        registry_lookup_executed_count,
        tool_invoked_count,
        noop_result_persisted_count,
        ledger_written_count,
        approval_requested_count,
        receipt_persisted_count,
        dynamic_activation_started_count,
        permission_granted_count,
        mcp_server_started_count,
        app_connector_started_count,
        plugin_installed_count,
        cache_materialized_count,
        cache_mutated_count,
        runtime_event_log_written_count,
        sqlite_written_count,
        live_execution_started_count,
        acceptance_recording_boundary_readback_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        operator_packet_send_allowed: false,
        operator_packet_persistence_allowed: false,
        operator_checklist_persistence_allowed: false,
        non_acceptance_receipt_persistence_allowed: false,
        acceptance_record_persistence_allowed: false,
        operator_acceptance_recording_allowed: false,
        non_recording_denial_receipt_persistence_allowed: false,
        operator_checklist_closure_persistence_allowed: false,
        dry_run_receipt_preview_persistence_allowed: false,
        ledger_preview_persistence_allowed: false,
        policy_decision_persistence_allowed: false,
        approval_preflight_execution_allowed: false,
        ledger_write_allowed: false,
        receipt_projection_persistence_allowed: false,
        tool_registry_registration_allowed: false,
        tool_registry_mutation_allowed: false,
        registry_lookup_execution_allowed: false,
        tool_invocation_allowed: false,
        noop_result_persistence_allowed: false,
        approval_request_allowed: false,
        receipt_persistence_allowed: false,
        dynamic_activation_allowed: false,
        permission_grant_allowed: false,
        mcp_server_start_allowed: false,
        app_connector_start_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        install_cache_materialization_allowed: false,
        runtime_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "feature_gate_open_disabled",
            "dry_run_execution_disabled",
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "operator_checklist_persistence_disabled",
            "non_acceptance_receipt_persistence_disabled",
            "acceptance_record_persistence_disabled",
            "operator_acceptance_recording_disabled",
            "non_recording_denial_receipt_persistence_disabled",
            "operator_checklist_closure_persistence_disabled",
            "dry_run_receipt_preview_persistence_disabled",
            "ledger_preview_persistence_disabled",
            "policy_decision_persistence_disabled",
            "approval_preflight_execution_disabled",
            "ledger_write_attempt_disabled",
            "receipt_projection_persistence_disabled",
            "tool_registry_registration_disabled",
            "tool_registry_mutation_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "noop_result_persistence_disabled",
            "approval_request_disabled",
            "receipt_persistence_disabled",
            "dynamic_activation_disabled",
            "permission_grant_disabled",
            "mcp_server_start_disabled",
            "app_connector_start_disabled",
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "install_cache_materialization_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorPacketReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackEntry>
{
    source
        .entries
        .iter()
        .map(|entry| {
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_operator_packet_id: entry.operator_packet_id,
                source_operator_checklist_id: entry.operator_checklist_id,
                source_non_acceptance_receipt_id: entry.non_acceptance_receipt_id,
                source_ledger_preview_link_id: entry.ledger_preview_link_id,
                source_receipt_preview_link_id: entry.receipt_preview_link_id,
                source_policy_denial_anchor_link_id: entry.policy_denial_anchor_link_id,
                source_approval_denial_anchor_link_id: entry.approval_denial_anchor_link_id,
                source_operator_packet_idempotency_key: entry.operator_packet_idempotency_key,
                acceptance_record_id: acceptance_record_id(entry.contribution_kind),
                non_recording_denial_receipt_id: non_recording_denial_receipt_id(
                    entry.contribution_kind,
                ),
                ledger_preview_anchor_id: ledger_preview_anchor_id(entry.contribution_kind),
                receipt_preview_anchor_id: receipt_preview_anchor_id(entry.contribution_kind),
                operator_checklist_closure_id: operator_checklist_closure_id(
                    entry.contribution_kind,
                ),
                operator_checklist_closure_items: OPERATOR_CHECKLIST_CLOSURE_ITEMS.to_vec(),
                acceptance_idempotency_key: acceptance_idempotency_key(entry.contribution_kind),
                first_acceptance_record_id: acceptance_record_id(entry.contribution_kind),
                second_acceptance_record_id: acceptance_record_id(entry.contribution_kind),
                first_non_recording_denial_receipt_id: non_recording_denial_receipt_id(
                    entry.contribution_kind,
                ),
                second_non_recording_denial_receipt_id: non_recording_denial_receipt_id(
                    entry.contribution_kind,
                ),
                first_acceptance_idempotency_key: acceptance_idempotency_key(
                    entry.contribution_kind,
                ),
                second_acceptance_idempotency_key: acceptance_idempotency_key(
                    entry.contribution_kind,
                ),
                acceptance_record_id_projected: true,
                non_recording_denial_receipt_projected: true,
                ledger_preview_anchor_projected: true,
                receipt_preview_anchor_projected: true,
                operator_checklist_closure_projected: true,
                acceptance_idempotency_key_projected: true,
                stable_acceptance_record: true,
                unique_acceptance_record: true,
                stable_non_recording_denial_receipt: true,
                unique_non_recording_denial_receipt: true,
                stable_acceptance_idempotency_key: true,
                unique_acceptance_idempotency_key: true,
                feature_gate_opened: entry.feature_gate_opened,
                dry_run_executed: entry.dry_run_executed,
                operator_packet_sent: entry.operator_packet_sent,
                operator_packet_persisted: entry.operator_packet_persisted,
                operator_checklist_persisted: entry.operator_checklist_persisted,
                non_acceptance_receipt_persisted: entry.non_acceptance_receipt_persisted,
                acceptance_record_persisted: false,
                operator_acceptance_recorded: entry.operator_acceptance_recorded,
                non_recording_denial_receipt_persisted: false,
                operator_checklist_closure_persisted: false,
                dry_run_receipt_preview_persisted: entry.dry_run_receipt_preview_persisted,
                ledger_preview_persisted: entry.ledger_preview_persisted,
                policy_decision_persisted: entry.policy_decision_persisted,
                approval_preflight_executed: entry.approval_preflight_executed,
                ledger_write_attempted: entry.ledger_write_attempted,
                receipt_projection_persisted: entry.receipt_projection_persisted,
                tool_registered: entry.tool_registered,
                tool_registry_mutated: entry.tool_registry_mutated,
                registry_lookup_executed: entry.registry_lookup_executed,
                tool_invoked: entry.tool_invoked,
                noop_result_persisted: entry.noop_result_persisted,
                ledger_written: entry.ledger_written,
                approval_requested: entry.approval_requested,
                receipt_persisted: entry.receipt_persisted,
                dynamic_activation_started: entry.dynamic_activation_started,
                permission_granted: entry.permission_granted,
                mcp_server_started: entry.mcp_server_started,
                app_connector_started: entry.app_connector_started,
                plugin_installed: entry.plugin_installed,
                cache_materialized: entry.cache_materialized,
                cache_mutated: entry.cache_mutated,
                runtime_event_log_written: entry.runtime_event_log_written,
                sqlite_written: entry.sqlite_written,
                live_execution_started: entry.live_execution_started,
            }
        })
        .collect()
}

fn acceptance_record_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-acceptance-record:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => "operator-acceptance-record:hepta-system:local-app:not-selected",
        _ => "operator-acceptance-record:hepta-system:unknown:not-selected",
    }
}

fn non_recording_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-acceptance-non-recording-denial:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-acceptance-non-recording-denial:hepta-system:local-app:not-selected"
        }
        _ => "operator-acceptance-non-recording-denial:hepta-system:unknown:not-selected",
    }
}

fn ledger_preview_anchor_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "acceptance-ledger-preview-anchor:hepta-system:local-mcp:dry-run-read-only-denied"
        }
        "app_connector" => "acceptance-ledger-preview-anchor:hepta-system:local-app:not-selected",
        _ => "acceptance-ledger-preview-anchor:hepta-system:unknown:not-selected",
    }
}

fn receipt_preview_anchor_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "acceptance-receipt-preview-anchor:hepta-system:local-mcp:read-only-denied",
        "app_connector" => "acceptance-receipt-preview-anchor:hepta-system:local-app:not-selected",
        _ => "acceptance-receipt-preview-anchor:hepta-system:unknown:not-selected",
    }
}

fn operator_checklist_closure_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-checklist-closure:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => "operator-checklist-closure:hepta-system:local-app:not-selected",
        _ => "operator-checklist-closure:hepta-system:unknown:not-selected",
    }
}

fn acceptance_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "operator-acceptance-recording-idempotency:hepta-system:local-mcp:read-only-status-dry-run"
        }
        "app_connector" => {
            "operator-acceptance-recording-idempotency:hepta-system:local-app:not-selected"
        }
        _ => "operator-acceptance-recording-idempotency:hepta-system:unknown:not-selected",
    }
}

impl
    HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_gate_opened: false,
            dry_run_executed: false,
            operator_packet_sent: false,
            operator_packet_persisted: false,
            operator_checklist_persisted: false,
            non_acceptance_receipt_persisted: false,
            acceptance_record_persisted: false,
            operator_acceptance_recorded: false,
            non_recording_denial_receipt_persisted: false,
            operator_checklist_closure_persisted: false,
            dry_run_receipt_preview_persisted: false,
            ledger_preview_persisted: false,
            policy_decision_persisted: false,
            approval_preflight_executed: false,
            ledger_write_attempted: false,
            receipt_projection_persisted: false,
            tool_registered: false,
            tool_registry_mutated: false,
            registry_lookup_executed: false,
            tool_invoked: false,
            noop_result_persisted: false,
            ledger_written: false,
            approval_requested: false,
            receipt_persisted: false,
            dynamic_activation_started: false,
            permission_granted: false,
            mcp_server_started: false,
            app_connector_started: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            install_cache_materialized: false,
            runtime_event_log_written: false,
            sqlite_written: false,
            credential_read: false,
            external_network_used: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_boundary_projects_recording_denial_and_preview_anchors() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.boundary_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(report.acceptance_record_id_projected_count, 2);
        assert_eq!(report.non_recording_denial_receipt_projected_count, 2);
        assert_eq!(report.ledger_preview_anchor_projected_count, 2);
        assert_eq!(report.receipt_preview_anchor_projected_count, 2);
        assert_eq!(report.operator_checklist_closure_projected_count, 2);
        assert_eq!(report.operator_checklist_closure_item_count, 10);
        assert_eq!(report.acceptance_idempotency_key_projected_count, 2);
        assert!(report.acceptance_recording_boundary_readback_ready);
    }

    #[test]
    fn acceptance_boundary_stays_stable_unique_and_not_recorded() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report();

        assert_eq!(report.stable_acceptance_record_count, 2);
        assert_eq!(report.unique_acceptance_record_count, 2);
        assert_eq!(report.stable_non_recording_denial_receipt_count, 2);
        assert_eq!(report.unique_non_recording_denial_receipt_count, 2);
        assert_eq!(report.stable_acceptance_idempotency_key_count, 2);
        assert_eq!(report.unique_acceptance_idempotency_key_count, 2);
        assert_eq!(report.acceptance_record_mismatch_count, 0);
        assert_eq!(report.duplicate_acceptance_record_count, 0);
        assert_eq!(report.non_recording_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_non_recording_denial_receipt_count, 0);
        assert_eq!(report.acceptance_idempotency_mismatch_count, 0);
        assert_eq!(report.duplicate_acceptance_idempotency_key_count, 0);
        assert_eq!(report.acceptance_record_persisted_count, 0);
        assert_eq!(report.operator_acceptance_recorded_count, 0);
    }

    #[test]
    fn acceptance_boundary_keeps_execution_persistence_and_live_closed() {
        let report =
            hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.non_recording_denial_receipt_persisted_count, 0);
        assert_eq!(report.operator_checklist_closure_persisted_count, 0);
        assert_eq!(report.policy_decision_persisted_count, 0);
        assert_eq!(report.approval_preflight_executed_count, 0);
        assert_eq!(report.ledger_write_attempted_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.approval_requested_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.acceptance_record_persistence_allowed);
        assert!(!report.operator_acceptance_recording_allowed);
        assert!(!report.non_recording_denial_receipt_persistence_allowed);
        assert!(!report.operator_checklist_closure_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunAcceptanceRecordingBoundaryReadbackSideEffects::none()
        );
    }
}
