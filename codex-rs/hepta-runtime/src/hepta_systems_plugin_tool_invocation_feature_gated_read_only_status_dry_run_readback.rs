use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationPolicyApprovalLedgerBoundaryReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_GATE:
    &str = "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_policy_approval_ledger_boundary_ready: bool,
    pub source_registration_denial_query_api_ready: bool,
    pub source_tool_registry_shadow_lookup_ready: bool,
    pub source_internal_read_only_invocation_ready: bool,
    pub source_minimal_ledger_receipt_ready: bool,
    pub candidate_count: usize,
    pub dry_run_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub registration_denial_query_hit_count: usize,
    pub shadow_lookup_projection_attached_count: usize,
    pub internal_status_payload_projection_attached_count: usize,
    pub internal_call_dry_run_projected_count: usize,
    pub structured_result_projection_attached_count: usize,
    pub approval_ledger_receipt_projection_attached_count: usize,
    pub local_append_only_store_projection_attached_count: usize,
    pub selected_dry_run_path_proof_count: usize,
    pub feature_gate_id_projected_count: usize,
    pub feature_gate_closed_count: usize,
    pub dry_run_payload_projected_count: usize,
    pub dry_run_payload_digest_projected_count: usize,
    pub dry_run_result_projection_count: usize,
    pub policy_denial_projected_count: usize,
    pub receipt_projection_count: usize,
    pub stable_dry_run_receipt_count: usize,
    pub unique_dry_run_receipt_count: usize,
    pub idempotency_key_projected_count: usize,
    pub stable_idempotency_key_count: usize,
    pub unique_idempotency_key_count: usize,
    pub dry_run_receipt_mismatch_count: usize,
    pub duplicate_dry_run_receipt_count: usize,
    pub idempotency_key_mismatch_count: usize,
    pub duplicate_idempotency_key_count: usize,
    pub feature_gate_opened_count: usize,
    pub dry_run_executed_count: usize,
    pub dry_run_payload_persisted_count: usize,
    pub dry_run_result_persisted_count: usize,
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
    pub feature_gated_read_only_status_dry_run_readback_ready: bool,
    pub feature_gated_read_only_status_dry_run_path_proof_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub dry_run_execution_allowed: bool,
    pub dry_run_payload_persistence_allowed: bool,
    pub dry_run_result_persistence_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub dry_run_selection_reason: &'static str,
    pub source_policy_decision_id: &'static str,
    pub source_policy_boundary_receipt_id: &'static str,
    pub source_policy_idempotency_key: &'static str,
    pub feature_gate_id: &'static str,
    pub feature_gate_state: &'static str,
    pub source_registration_denial_id: String,
    pub source_shadow_lookup_result_id: String,
    pub source_internal_status_request_id: &'static str,
    pub source_status_payload_fingerprint: &'static str,
    pub source_minimal_receipt_stage_id: &'static str,
    pub dry_run_request_id: &'static str,
    pub dry_run_payload_id: &'static str,
    pub dry_run_payload_digest: &'static str,
    pub dry_run_result_projection_id: &'static str,
    pub policy_denial_id: &'static str,
    pub receipt_projection_id: &'static str,
    pub first_dry_run_receipt_id: &'static str,
    pub second_dry_run_receipt_id: &'static str,
    pub stable_dry_run_receipt: bool,
    pub unique_dry_run_receipt: bool,
    pub first_dry_run_idempotency_key: &'static str,
    pub second_dry_run_idempotency_key: &'static str,
    pub stable_idempotency_key: bool,
    pub unique_idempotency_key: bool,
    pub feature_gate_id_projected: bool,
    pub feature_gate_closed: bool,
    pub registration_denial_query_hit: bool,
    pub shadow_lookup_projection_attached: bool,
    pub internal_status_payload_projection_attached: bool,
    pub internal_call_dry_run_projected: bool,
    pub structured_result_projection_attached: bool,
    pub approval_ledger_receipt_projection_attached: bool,
    pub local_append_only_store_projection_attached: bool,
    pub selected_dry_run_path_proof: bool,
    pub dry_run_payload_projected: bool,
    pub dry_run_payload_digest_projected: bool,
    pub dry_run_result_projected: bool,
    pub policy_denial_projected: bool,
    pub receipt_projection_projected: bool,
    pub dry_run_receipt_projected: bool,
    pub idempotency_key_projected: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub dry_run_payload_persisted: bool,
    pub dry_run_result_persisted: bool,
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
pub struct HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackSideEffects {
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub dry_run_executed: bool,
    pub dry_run_payload_persisted: bool,
    pub dry_run_result_persisted: bool,
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

pub fn hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report()
-> HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackReport {
    let source =
        hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_report();
    hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationPolicyApprovalLedgerBoundaryReadbackReport,
) -> HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackReport {
    let entries =
        hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_entries(
            source,
        );
    let dry_run_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let registration_denial_query_hit_count = entries
        .iter()
        .filter(|entry| entry.registration_denial_query_hit)
        .count();
    let shadow_lookup_projection_attached_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_projection_attached)
        .count();
    let internal_status_payload_projection_attached_count = entries
        .iter()
        .filter(|entry| entry.internal_status_payload_projection_attached)
        .count();
    let internal_call_dry_run_projected_count = entries
        .iter()
        .filter(|entry| entry.internal_call_dry_run_projected)
        .count();
    let structured_result_projection_attached_count = entries
        .iter()
        .filter(|entry| entry.structured_result_projection_attached)
        .count();
    let approval_ledger_receipt_projection_attached_count = entries
        .iter()
        .filter(|entry| entry.approval_ledger_receipt_projection_attached)
        .count();
    let local_append_only_store_projection_attached_count = entries
        .iter()
        .filter(|entry| entry.local_append_only_store_projection_attached)
        .count();
    let selected_dry_run_path_proof_count = entries
        .iter()
        .filter(|entry| entry.selected_dry_run_path_proof)
        .count();
    let feature_gate_id_projected_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_id_projected)
        .count();
    let feature_gate_closed_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_closed)
        .count();
    let dry_run_payload_projected_count = entries
        .iter()
        .filter(|entry| entry.dry_run_payload_projected)
        .count();
    let dry_run_payload_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.dry_run_payload_digest_projected)
        .count();
    let dry_run_result_projection_count = entries
        .iter()
        .filter(|entry| entry.dry_run_result_projected)
        .count();
    let policy_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.policy_denial_projected)
        .count();
    let receipt_projection_count = entries
        .iter()
        .filter(|entry| entry.receipt_projection_projected)
        .count();
    let stable_dry_run_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_dry_run_receipt)
        .count();
    let unique_dry_run_receipt_count = entries
        .iter()
        .map(|entry| entry.first_dry_run_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_projected)
        .count();
    let stable_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_idempotency_key)
        .count();
    let unique_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_dry_run_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let dry_run_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_dry_run_receipt)
        .count();
    let duplicate_dry_run_receipt_count =
        dry_run_entry_count.saturating_sub(unique_dry_run_receipt_count);
    let idempotency_key_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_idempotency_key)
        .count();
    let duplicate_idempotency_key_count =
        dry_run_entry_count.saturating_sub(unique_idempotency_key_count);
    let feature_gate_opened_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_opened)
        .count();
    let dry_run_executed_count = entries
        .iter()
        .filter(|entry| entry.dry_run_executed)
        .count();
    let dry_run_payload_persisted_count = entries
        .iter()
        .filter(|entry| entry.dry_run_payload_persisted)
        .count();
    let dry_run_result_persisted_count = entries
        .iter()
        .filter(|entry| entry.dry_run_result_persisted)
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

    let feature_gated_read_only_status_dry_run_path_proof_ready =
        registration_denial_query_hit_count == 2
            && shadow_lookup_projection_attached_count == 2
            && internal_status_payload_projection_attached_count == 1
            && internal_call_dry_run_projected_count == 1
            && structured_result_projection_attached_count == 1
            && approval_ledger_receipt_projection_attached_count == 1
            && local_append_only_store_projection_attached_count == 1
            && selected_dry_run_path_proof_count == 1
            && registry_lookup_executed_count == 0
            && tool_invoked_count == 0
            && approval_requested_count == 0
            && ledger_written_count == 0
            && receipt_persisted_count == 0
            && runtime_event_log_written_count == 0
            && sqlite_written_count == 0;

    let feature_gated_read_only_status_dry_run_readback_ready = source
        .tool_invocation_policy_approval_ledger_boundary_readback_ready
        && source.candidate_count == 2
        && source.policy_decision_id_projected_count == 2
        && source.approval_preflight_denial_id_projected_count == 2
        && source.ledger_write_denial_id_projected_count == 2
        && source.receipt_anchor_projected_count == 2
        && source.policy_boundary_receipt_projected_count == 2
        && source.policy_decision_persisted_count == 0
        && source.approval_preflight_executed_count == 0
        && source.ledger_write_attempted_count == 0
        && source.receipt_anchor_persisted_count == 0
        && source.tool_registered_count == 0
        && source.tool_registry_mutated_count == 0
        && source.registry_lookup_executed_count == 0
        && source.tool_invoked_count == 0
        && source.ledger_written_count == 0
        && source.approval_requested_count == 0
        && source.receipt_persisted_count == 0
        && source.runtime_event_log_written_count == 0
        && source.sqlite_written_count == 0
        && source.live_execution_started_count == 0
        && feature_gated_read_only_status_dry_run_path_proof_ready
        && dry_run_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && registration_denial_query_hit_count == 2
        && shadow_lookup_projection_attached_count == 2
        && internal_status_payload_projection_attached_count == 1
        && internal_call_dry_run_projected_count == 1
        && structured_result_projection_attached_count == 1
        && approval_ledger_receipt_projection_attached_count == 1
        && local_append_only_store_projection_attached_count == 1
        && selected_dry_run_path_proof_count == 1
        && feature_gate_id_projected_count == 2
        && feature_gate_closed_count == 2
        && dry_run_payload_projected_count == 1
        && dry_run_payload_digest_projected_count == 1
        && dry_run_result_projection_count == 1
        && policy_denial_projected_count == 2
        && receipt_projection_count == 2
        && stable_dry_run_receipt_count == 2
        && unique_dry_run_receipt_count == 2
        && idempotency_key_projected_count == 2
        && stable_idempotency_key_count == 2
        && unique_idempotency_key_count == 2
        && dry_run_receipt_mismatch_count == 0
        && duplicate_dry_run_receipt_count == 0
        && idempotency_key_mismatch_count == 0
        && duplicate_idempotency_key_count == 0
        && feature_gate_opened_count == 0
        && dry_run_executed_count == 0
        && dry_run_payload_persisted_count == 0
        && dry_run_result_persisted_count == 0
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
        && live_execution_started_count == 0
        && entries.iter().all(|entry| {
            entry.feature_gate_id_projected
                && entry.feature_gate_closed
                && entry.registration_denial_query_hit
                && entry.shadow_lookup_projection_attached
                && entry.policy_denial_projected
                && entry.receipt_projection_projected
                && entry.dry_run_receipt_projected
                && entry.stable_dry_run_receipt
                && entry.unique_dry_run_receipt
                && entry.idempotency_key_projected
                && entry.stable_idempotency_key
                && entry.unique_idempotency_key
                && !entry.feature_gate_opened
                && !entry.dry_run_executed
                && !entry.dry_run_payload_persisted
                && !entry.dry_run_result_persisted
                && !entry.policy_decision_persisted
                && !entry.approval_preflight_executed
                && !entry.ledger_write_attempted
                && !entry.receipt_projection_persisted
                && !entry.tool_registered
                && !entry.tool_registry_mutated
                && !entry.registry_lookup_executed
                && !entry.tool_invoked
                && !entry.noop_result_persisted
                && !entry.ledger_written
                && !entry.approval_requested
                && !entry.receipt_persisted
                && !entry.dynamic_activation_started
                && !entry.permission_granted
                && !entry.mcp_server_started
                && !entry.app_connector_started
                && !entry.plugin_installed
                && !entry.cache_materialized
                && !entry.cache_mutated
                && !entry.runtime_event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
                && (!entry.dry_run_path_selected
                    || (entry.internal_status_payload_projection_attached
                        && entry.internal_call_dry_run_projected
                        && entry.structured_result_projection_attached
                        && entry.approval_ledger_receipt_projection_attached
                        && entry.local_append_only_store_projection_attached
                        && entry.selected_dry_run_path_proof))
        });

    HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
        status: if feature_gated_read_only_status_dry_run_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_policy_approval_ledger_boundary_ready: source
            .tool_invocation_policy_approval_ledger_boundary_readback_ready,
        source_registration_denial_query_api_ready: true,
        source_tool_registry_shadow_lookup_ready: true,
        source_internal_read_only_invocation_ready: true,
        source_minimal_ledger_receipt_ready: true,
        candidate_count: source.candidate_count,
        dry_run_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        registration_denial_query_hit_count,
        shadow_lookup_projection_attached_count,
        internal_status_payload_projection_attached_count,
        internal_call_dry_run_projected_count,
        structured_result_projection_attached_count,
        approval_ledger_receipt_projection_attached_count,
        local_append_only_store_projection_attached_count,
        selected_dry_run_path_proof_count,
        feature_gate_id_projected_count,
        feature_gate_closed_count,
        dry_run_payload_projected_count,
        dry_run_payload_digest_projected_count,
        dry_run_result_projection_count,
        policy_denial_projected_count,
        receipt_projection_count,
        stable_dry_run_receipt_count,
        unique_dry_run_receipt_count,
        idempotency_key_projected_count,
        stable_idempotency_key_count,
        unique_idempotency_key_count,
        dry_run_receipt_mismatch_count,
        duplicate_dry_run_receipt_count,
        idempotency_key_mismatch_count,
        duplicate_idempotency_key_count,
        feature_gate_opened_count,
        dry_run_executed_count,
        dry_run_payload_persisted_count,
        dry_run_result_persisted_count,
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
        feature_gated_read_only_status_dry_run_readback_ready,
        feature_gated_read_only_status_dry_run_path_proof_ready,
        feature_gate_open_allowed: false,
        dry_run_execution_allowed: false,
        dry_run_payload_persistence_allowed: false,
        dry_run_result_persistence_allowed: false,
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
            "dry_run_payload_persistence_disabled",
            "dry_run_result_persistence_disabled",
            "policy_decision_persistence_disabled",
            "approval_preflight_execution_disabled",
            "ledger_write_attempt_disabled",
            "ledger_write_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_entries(
    source: &HeptaSystemsPluginToolInvocationPolicyApprovalLedgerBoundaryReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackEntry> {
    source
        .entries
        .iter()
        .map(|entry| {
            let selected = entry.contribution_kind == "mcp_server";
            let registration_denial_query_hit = true;
            let shadow_lookup_projection_attached = true;
            let internal_status_payload_projection_attached = selected;
            let internal_call_dry_run_projected = selected;
            let structured_result_projection_attached = selected;
            let approval_ledger_receipt_projection_attached = selected;
            let local_append_only_store_projection_attached =
                selected && !entry.runtime_event_log_written && !entry.sqlite_written;
            let selected_dry_run_path_proof = selected
                && registration_denial_query_hit
                && shadow_lookup_projection_attached
                && internal_status_payload_projection_attached
                && internal_call_dry_run_projected
                && structured_result_projection_attached
                && approval_ledger_receipt_projection_attached
                && local_append_only_store_projection_attached;
            HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                dry_run_path_selected: selected,
                dry_run_selection_reason: if selected {
                    "selected_mcp_status_read_only_path"
                } else {
                    "non_selected_app_connector_preflight_boundary"
                },
                source_policy_decision_id: entry.policy_decision_id,
                source_policy_boundary_receipt_id: entry.first_policy_boundary_receipt_id,
                source_policy_idempotency_key: entry.first_policy_idempotency_key,
                feature_gate_id: feature_gate_id(entry.contribution_kind),
                feature_gate_state: "closed",
                source_registration_denial_id: registration_denial_id(entry.contribution_kind)
                    .to_string(),
                source_shadow_lookup_result_id: shadow_lookup_result_id(entry.contribution_kind)
                    .to_string(),
                source_internal_status_request_id: internal_status_request_id(
                    entry.contribution_kind,
                ),
                source_status_payload_fingerprint: status_payload_fingerprint(
                    entry.contribution_kind,
                ),
                source_minimal_receipt_stage_id: minimal_receipt_stage_id(entry.contribution_kind),
                dry_run_request_id: dry_run_request_id(entry.contribution_kind),
                dry_run_payload_id: dry_run_payload_id(entry.contribution_kind),
                dry_run_payload_digest: dry_run_payload_digest(entry.contribution_kind),
                dry_run_result_projection_id: dry_run_result_projection_id(entry.contribution_kind),
                policy_denial_id: entry.policy_decision_id,
                receipt_projection_id: receipt_projection_id(entry.contribution_kind),
                first_dry_run_receipt_id: dry_run_receipt_id(entry.contribution_kind),
                second_dry_run_receipt_id: dry_run_receipt_id(entry.contribution_kind),
                stable_dry_run_receipt: true,
                unique_dry_run_receipt: true,
                first_dry_run_idempotency_key: dry_run_idempotency_key(entry.contribution_kind),
                second_dry_run_idempotency_key: dry_run_idempotency_key(entry.contribution_kind),
                stable_idempotency_key: true,
                unique_idempotency_key: true,
                feature_gate_id_projected: true,
                feature_gate_closed: true,
                registration_denial_query_hit,
                shadow_lookup_projection_attached,
                internal_status_payload_projection_attached,
                internal_call_dry_run_projected,
                structured_result_projection_attached,
                approval_ledger_receipt_projection_attached,
                local_append_only_store_projection_attached,
                selected_dry_run_path_proof,
                dry_run_payload_projected: selected,
                dry_run_payload_digest_projected: selected,
                dry_run_result_projected: selected,
                policy_denial_projected: true,
                receipt_projection_projected: true,
                dry_run_receipt_projected: true,
                idempotency_key_projected: true,
                feature_gate_opened: false,
                dry_run_executed: false,
                dry_run_payload_persisted: false,
                dry_run_result_persisted: false,
                policy_decision_persisted: entry.policy_decision_persisted,
                approval_preflight_executed: entry.approval_preflight_executed,
                ledger_write_attempted: entry.ledger_write_attempted,
                receipt_projection_persisted: false,
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

fn feature_gate_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "feature-gate:hepta-system:local-mcp:status-dry-run",
        "app_connector" => "feature-gate:hepta-system:local-app:status-dry-run",
        _ => "feature-gate:hepta-system:unknown:status-dry-run",
    }
}

fn dry_run_request_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-request:hepta-system:local-mcp:status-read-only",
        "app_connector" => "dry-run-request:hepta-system:local-app:not-selected",
        _ => "dry-run-request:hepta-system:unknown:not-selected",
    }
}

fn dry_run_payload_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-payload:hepta-system:local-mcp:status-read-only-v0",
        "app_connector" => "dry-run-payload:hepta-system:local-app:not-selected",
        _ => "dry-run-payload:hepta-system:unknown:not-selected",
    }
}

fn dry_run_payload_digest(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-payload-digest:hepta-system:local-mcp:status-read-only-v0",
        "app_connector" => "dry-run-payload-digest:hepta-system:local-app:not-selected",
        _ => "dry-run-payload-digest:hepta-system:unknown:not-selected",
    }
}

fn dry_run_result_projection_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-result-projection:hepta-system:local-mcp:status-read-only-v0",
        "app_connector" => "dry-run-result-projection:hepta-system:local-app:not-selected",
        _ => "dry-run-result-projection:hepta-system:unknown:not-selected",
    }
}

fn receipt_projection_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-receipt-projection:hepta-system:local-mcp:read-only-denied",
        "app_connector" => "dry-run-receipt-projection:hepta-system:local-app:not-selected",
        _ => "dry-run-receipt-projection:hepta-system:unknown:not-selected",
    }
}

fn dry_run_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-receipt:hepta-system:local-mcp:read-only-denied",
        "app_connector" => "dry-run-receipt:hepta-system:local-app:not-selected",
        _ => "dry-run-receipt:hepta-system:unknown:not-selected",
    }
}

fn dry_run_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "dry-run-idempotency:hepta-system:local-mcp:read-only-denied",
        "app_connector" => "dry-run-idempotency:hepta-system:local-app:not-selected",
        _ => "dry-run-idempotency:hepta-system:unknown:not-selected",
    }
}

fn registration_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "registration-denial:hepta-system:local-mcp:status-read-only",
        "app_connector" => "registration-denial:hepta-system:local-app:not-selected",
        _ => "registration-denial:hepta-system:unknown:not-selected",
    }
}

fn shadow_lookup_result_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "shadow-lookup-result:hepta-system:local-mcp:status-read-only",
        "app_connector" => "shadow-lookup-result:hepta-system:local-app:not-selected",
        _ => "shadow-lookup-result:hepta-system:unknown:not-selected",
    }
}

fn internal_status_request_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "hepta-system.status.internal-read-only.v1",
        "app_connector" => "hepta-system.status.internal-read-only.non-selected-app.v1",
        _ => "none_preflight_only",
    }
}

fn status_payload_fingerprint(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "hepta-system-status.internal-read-only.v1.e2e4.fixture9.live0",
        "app_connector" => "not-selected.preflight-only.no-payload",
        _ => "not-selected.preflight-only.no-payload",
    }
}

fn minimal_receipt_stage_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "selected_result_receipt_projection",
        "app_connector" => "none_preflight_only",
        _ => "none_preflight_only",
    }
}

impl HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_gate_opened: false,
            dry_run_executed: false,
            dry_run_payload_persisted: false,
            dry_run_result_persisted: false,
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
    fn feature_gated_dry_run_projects_selected_status_contract() {
        let report =
            hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_policy_approval_ledger_boundary_ready);
        assert!(report.source_registration_denial_query_api_ready);
        assert!(report.source_tool_registry_shadow_lookup_ready);
        assert!(report.source_internal_read_only_invocation_ready);
        assert!(report.source_minimal_ledger_receipt_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(report.registration_denial_query_hit_count, 2);
        assert_eq!(report.shadow_lookup_projection_attached_count, 2);
        assert_eq!(report.internal_status_payload_projection_attached_count, 1);
        assert_eq!(report.internal_call_dry_run_projected_count, 1);
        assert_eq!(report.structured_result_projection_attached_count, 1);
        assert_eq!(report.approval_ledger_receipt_projection_attached_count, 1);
        assert_eq!(report.local_append_only_store_projection_attached_count, 1);
        assert_eq!(report.selected_dry_run_path_proof_count, 1);
        assert_eq!(report.feature_gate_id_projected_count, 2);
        assert_eq!(report.feature_gate_closed_count, 2);
        assert_eq!(report.dry_run_payload_projected_count, 1);
        assert_eq!(report.dry_run_payload_digest_projected_count, 1);
        assert_eq!(report.dry_run_result_projection_count, 1);
        assert_eq!(report.policy_denial_projected_count, 2);
        assert_eq!(report.receipt_projection_count, 2);
        assert!(report.feature_gated_read_only_status_dry_run_path_proof_ready);
        assert!(report.feature_gated_read_only_status_dry_run_readback_ready);
    }

    #[test]
    fn feature_gated_dry_run_stays_stable_unique_and_unpersisted() {
        let report =
            hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report();

        assert_eq!(report.stable_dry_run_receipt_count, 2);
        assert_eq!(report.unique_dry_run_receipt_count, 2);
        assert_eq!(report.idempotency_key_projected_count, 2);
        assert_eq!(report.stable_idempotency_key_count, 2);
        assert_eq!(report.unique_idempotency_key_count, 2);
        assert_eq!(report.dry_run_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_dry_run_receipt_count, 0);
        assert_eq!(report.idempotency_key_mismatch_count, 0);
        assert_eq!(report.duplicate_idempotency_key_count, 0);
        assert_eq!(report.dry_run_payload_persisted_count, 0);
        assert_eq!(report.dry_run_result_persisted_count, 0);
        assert_eq!(report.receipt_projection_persisted_count, 0);
    }

    #[test]
    fn feature_gated_dry_run_keeps_execution_and_live_closed() {
        let report =
            hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.dry_run_executed_count, 0);
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
        assert!(!report.feature_gate_open_allowed);
        assert!(!report.dry_run_execution_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationFeatureGatedReadOnlyStatusDryRunReadbackSideEffects::none()
        );
    }

    #[test]
    fn feature_gated_dry_run_path_proof_links_registration_lookup_internal_call_and_receipt() {
        let report =
            hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report();

        let selected = report
            .entries
            .iter()
            .find(|entry| entry.dry_run_path_selected)
            .expect("selected read-only status dry-run entry");

        assert!(selected.registration_denial_query_hit);
        assert!(selected.shadow_lookup_projection_attached);
        assert!(selected.internal_status_payload_projection_attached);
        assert!(selected.internal_call_dry_run_projected);
        assert!(selected.structured_result_projection_attached);
        assert!(selected.approval_ledger_receipt_projection_attached);
        assert!(selected.local_append_only_store_projection_attached);
        assert!(selected.selected_dry_run_path_proof);
        assert!(
            selected
                .source_registration_denial_id
                .starts_with("registration-denial:hepta-system:")
        );
        assert!(
            selected
                .source_shadow_lookup_result_id
                .starts_with("shadow-lookup-result:hepta-system:")
        );
        assert_eq!(
            selected.source_internal_status_request_id,
            "hepta-system.status.internal-read-only.v1"
        );
        assert_eq!(
            selected.source_status_payload_fingerprint,
            "hepta-system-status.internal-read-only.v1.e2e4.fixture9.live0"
        );
        assert_eq!(
            selected.source_minimal_receipt_stage_id,
            "selected_result_receipt_projection"
        );

        let non_selected = report
            .entries
            .iter()
            .find(|entry| !entry.dry_run_path_selected)
            .expect("non-selected preflight entry");

        assert!(non_selected.registration_denial_query_hit);
        assert!(non_selected.shadow_lookup_projection_attached);
        assert!(!non_selected.internal_status_payload_projection_attached);
        assert!(!non_selected.internal_call_dry_run_projected);
        assert!(!non_selected.selected_dry_run_path_proof);
        assert_eq!(
            non_selected.source_internal_status_request_id,
            "hepta-system.status.internal-read-only.non-selected-app.v1"
        );
        assert_eq!(
            non_selected.source_minimal_receipt_stage_id,
            "none_preflight_only"
        );
    }
}
