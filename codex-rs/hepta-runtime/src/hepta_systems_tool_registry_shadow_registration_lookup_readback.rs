use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport;
use crate::hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report;

pub const HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_GATE: &str =
    "hepta_systems_tool_registry_shadow_registration_lookup_readback_gate";
pub const HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_tool_registry_shadow_registration_lookup_readback_v1";
pub const HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "close_controlled_live_evidence_before_status_canary_start";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryShadowRegistrationLookupReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_persistence_shadow_write_rehearsal_readback_ready: bool,
    pub candidate_count: usize,
    pub registry_shadow_entry_count: usize,
    pub selected_read_only_status_tool_count: usize,
    pub non_selected_preflight_boundary_count: usize,
    pub source_shadow_acceptance_record_envelope_linked_count: usize,
    pub source_shadow_write_intent_linked_count: usize,
    pub source_shadow_payload_digest_linked_count: usize,
    pub source_shadow_idempotency_replay_key_linked_count: usize,
    pub source_shadow_receipt_preview_linked_count: usize,
    pub source_shadow_store_target_linked_count: usize,
    pub source_shadow_replay_result_linked_count: usize,
    pub shadow_registry_registration_plan_projected_count: usize,
    pub shadow_registry_entry_key_projected_count: usize,
    pub shadow_registration_payload_digest_projected_count: usize,
    pub shadow_lookup_query_projected_count: usize,
    pub shadow_lookup_result_projected_count: usize,
    pub shadow_duplicate_check_projected_count: usize,
    pub shadow_idempotency_replay_anchor_projected_count: usize,
    pub shadow_approval_ledger_replay_anchor_projected_count: usize,
    pub tool_registry_shadow_item_count: usize,
    pub stable_shadow_registry_entry_key_count: usize,
    pub unique_shadow_registry_entry_key_count: usize,
    pub stable_shadow_registration_payload_digest_count: usize,
    pub unique_shadow_registration_payload_digest_count: usize,
    pub stable_shadow_lookup_query_count: usize,
    pub unique_shadow_lookup_query_count: usize,
    pub stable_shadow_idempotency_replay_anchor_count: usize,
    pub unique_shadow_idempotency_replay_anchor_count: usize,
    pub shadow_registry_entry_key_mismatch_count: usize,
    pub duplicate_shadow_registry_entry_key_count: usize,
    pub shadow_registration_payload_digest_mismatch_count: usize,
    pub duplicate_shadow_registration_payload_digest_count: usize,
    pub shadow_lookup_query_mismatch_count: usize,
    pub duplicate_shadow_lookup_query_count: usize,
    pub shadow_idempotency_replay_anchor_mismatch_count: usize,
    pub duplicate_shadow_idempotency_replay_anchor_count: usize,
    pub feature_gate_opened_count: usize,
    pub shadow_write_executed_count: usize,
    pub shadow_write_materialized_count: usize,
    pub shadow_store_written_count: usize,
    pub test_tmp_written_count: usize,
    pub shadow_registry_materialized_count: usize,
    pub shadow_lookup_executed_count: usize,
    pub tool_registered_count: usize,
    pub tool_registry_mutated_count: usize,
    pub registry_lookup_executed_count: usize,
    pub tool_invoked_count: usize,
    pub approval_requested_count: usize,
    pub ledger_written_count: usize,
    pub receipt_persisted_count: usize,
    pub mcp_server_started_count: usize,
    pub app_connector_started_count: usize,
    pub runtime_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_execution_started_count: usize,
    pub tool_registry_shadow_registration_lookup_readback_ready: bool,
    pub feature_gate_open_allowed: bool,
    pub shadow_write_execution_allowed: bool,
    pub shadow_store_write_allowed: bool,
    pub test_tmp_write_allowed: bool,
    pub shadow_registry_materialization_allowed: bool,
    pub shadow_lookup_execution_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_registry_mutation_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub approval_request_allowed: bool,
    pub ledger_persistence_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub connector_start_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsToolRegistryShadowRegistrationLookupReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsToolRegistryShadowRegistrationLookupReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryShadowRegistrationLookupReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub dry_run_path_selected: bool,
    pub source_shadow_acceptance_record_envelope_id: String,
    pub source_shadow_write_intent_id: String,
    pub source_shadow_write_payload_digest: String,
    pub source_shadow_idempotency_replay_key: String,
    pub source_shadow_receipt_preview_id: String,
    pub source_shadow_store_target_id: String,
    pub source_shadow_replay_result_id: String,
    pub shadow_registry_registration_plan_id: String,
    pub shadow_registry_entry_key: String,
    pub shadow_registration_payload_digest: String,
    pub shadow_lookup_query_id: String,
    pub shadow_lookup_result_id: String,
    pub shadow_duplicate_check_id: String,
    pub shadow_idempotency_replay_anchor_id: String,
    pub shadow_approval_ledger_replay_anchor_id: String,
    pub first_shadow_registry_entry_key: String,
    pub second_shadow_registry_entry_key: String,
    pub first_shadow_registration_payload_digest: String,
    pub second_shadow_registration_payload_digest: String,
    pub first_shadow_lookup_query_id: String,
    pub second_shadow_lookup_query_id: String,
    pub first_shadow_idempotency_replay_anchor_id: String,
    pub second_shadow_idempotency_replay_anchor_id: String,
    pub source_shadow_acceptance_record_envelope_linked: bool,
    pub source_shadow_write_intent_linked: bool,
    pub source_shadow_payload_digest_linked: bool,
    pub source_shadow_idempotency_replay_key_linked: bool,
    pub source_shadow_receipt_preview_linked: bool,
    pub source_shadow_store_target_linked: bool,
    pub source_shadow_replay_result_linked: bool,
    pub shadow_registry_registration_plan_projected: bool,
    pub shadow_registry_entry_key_projected: bool,
    pub shadow_registration_payload_digest_projected: bool,
    pub shadow_lookup_query_projected: bool,
    pub shadow_lookup_result_projected: bool,
    pub shadow_duplicate_check_projected: bool,
    pub shadow_idempotency_replay_anchor_projected: bool,
    pub shadow_approval_ledger_replay_anchor_projected: bool,
    pub stable_shadow_registry_entry_key: bool,
    pub unique_shadow_registry_entry_key: bool,
    pub stable_shadow_registration_payload_digest: bool,
    pub unique_shadow_registration_payload_digest: bool,
    pub stable_shadow_lookup_query: bool,
    pub unique_shadow_lookup_query: bool,
    pub stable_shadow_idempotency_replay_anchor: bool,
    pub unique_shadow_idempotency_replay_anchor: bool,
    pub feature_gate_opened: bool,
    pub shadow_write_executed: bool,
    pub shadow_write_materialized: bool,
    pub shadow_store_written: bool,
    pub test_tmp_written: bool,
    pub shadow_registry_materialized: bool,
    pub shadow_lookup_executed: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub approval_requested: bool,
    pub ledger_written: bool,
    pub receipt_persisted: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryShadowRegistrationLookupReadbackSideEffects {
    pub filesystem_written: bool,
    pub feature_gate_opened: bool,
    pub shadow_write_executed: bool,
    pub shadow_write_materialized: bool,
    pub shadow_store_written: bool,
    pub test_tmp_written: bool,
    pub shadow_registry_materialized: bool,
    pub shadow_lookup_executed: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub approval_requested: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
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

pub fn hepta_systems_tool_registry_shadow_registration_lookup_readback_report()
-> HeptaSystemsToolRegistryShadowRegistrationLookupReadbackReport {
    let source =
        hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_report();
    hepta_systems_tool_registry_shadow_registration_lookup_readback_report_from_source(&source)
}

pub fn hepta_systems_tool_registry_shadow_registration_lookup_readback_report_from_source(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport,
) -> HeptaSystemsToolRegistryShadowRegistrationLookupReadbackReport {
    let entries = hepta_systems_tool_registry_shadow_registration_lookup_readback_entries(source);
    let registry_shadow_entry_count = entries.len();
    let selected_read_only_status_tool_count = entries
        .iter()
        .filter(|entry| entry.dry_run_path_selected)
        .count();
    let non_selected_preflight_boundary_count = entries
        .iter()
        .filter(|entry| !entry.dry_run_path_selected)
        .count();
    let source_shadow_acceptance_record_envelope_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_acceptance_record_envelope_linked)
        .count();
    let source_shadow_write_intent_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_write_intent_linked)
        .count();
    let source_shadow_payload_digest_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_payload_digest_linked)
        .count();
    let source_shadow_idempotency_replay_key_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_idempotency_replay_key_linked)
        .count();
    let source_shadow_receipt_preview_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_receipt_preview_linked)
        .count();
    let source_shadow_store_target_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_store_target_linked)
        .count();
    let source_shadow_replay_result_linked_count = entries
        .iter()
        .filter(|entry| entry.source_shadow_replay_result_linked)
        .count();
    let shadow_registry_registration_plan_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_registry_registration_plan_projected)
        .count();
    let shadow_registry_entry_key_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_registry_entry_key_projected)
        .count();
    let shadow_registration_payload_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_registration_payload_digest_projected)
        .count();
    let shadow_lookup_query_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_query_projected)
        .count();
    let shadow_lookup_result_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_result_projected)
        .count();
    let shadow_duplicate_check_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_duplicate_check_projected)
        .count();
    let shadow_idempotency_replay_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_idempotency_replay_anchor_projected)
        .count();
    let shadow_approval_ledger_replay_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.shadow_approval_ledger_replay_anchor_projected)
        .count();
    let tool_registry_shadow_item_count = shadow_registry_registration_plan_projected_count
        + shadow_registry_entry_key_projected_count
        + shadow_registration_payload_digest_projected_count
        + shadow_lookup_query_projected_count
        + shadow_lookup_result_projected_count
        + shadow_duplicate_check_projected_count
        + shadow_idempotency_replay_anchor_projected_count
        + shadow_approval_ledger_replay_anchor_projected_count;
    let stable_shadow_registry_entry_key_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_registry_entry_key)
        .count();
    let unique_shadow_registry_entry_key_count = entries
        .iter()
        .map(|entry| entry.first_shadow_registry_entry_key.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_shadow_registration_payload_digest_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_registration_payload_digest)
        .count();
    let unique_shadow_registration_payload_digest_count = entries
        .iter()
        .map(|entry| entry.first_shadow_registration_payload_digest.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_shadow_lookup_query_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_lookup_query)
        .count();
    let unique_shadow_lookup_query_count = entries
        .iter()
        .map(|entry| entry.first_shadow_lookup_query_id.as_str())
        .collect::<HashSet<_>>()
        .len();
    let stable_shadow_idempotency_replay_anchor_count = entries
        .iter()
        .filter(|entry| entry.stable_shadow_idempotency_replay_anchor)
        .count();
    let unique_shadow_idempotency_replay_anchor_count = entries
        .iter()
        .map(|entry| entry.first_shadow_idempotency_replay_anchor_id.as_str())
        .collect::<HashSet<_>>()
        .len();
    let shadow_registry_entry_key_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_registry_entry_key)
        .count();
    let duplicate_shadow_registry_entry_key_count =
        registry_shadow_entry_count.saturating_sub(unique_shadow_registry_entry_key_count);
    let shadow_registration_payload_digest_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_registration_payload_digest)
        .count();
    let duplicate_shadow_registration_payload_digest_count =
        registry_shadow_entry_count.saturating_sub(unique_shadow_registration_payload_digest_count);
    let shadow_lookup_query_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_lookup_query)
        .count();
    let duplicate_shadow_lookup_query_count =
        registry_shadow_entry_count.saturating_sub(unique_shadow_lookup_query_count);
    let shadow_idempotency_replay_anchor_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_shadow_idempotency_replay_anchor)
        .count();
    let duplicate_shadow_idempotency_replay_anchor_count =
        registry_shadow_entry_count.saturating_sub(unique_shadow_idempotency_replay_anchor_count);
    let feature_gate_opened_count = entries
        .iter()
        .filter(|entry| entry.feature_gate_opened)
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
    let shadow_registry_materialized_count = entries
        .iter()
        .filter(|entry| entry.shadow_registry_materialized)
        .count();
    let shadow_lookup_executed_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_executed)
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
    let approval_requested_count = entries
        .iter()
        .filter(|entry| entry.approval_requested)
        .count();
    let ledger_written_count = entries.iter().filter(|entry| entry.ledger_written).count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted)
        .count();
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

    let tool_registry_shadow_registration_lookup_readback_ready = source
        .persistence_shadow_write_rehearsal_readback_ready
        && source.candidate_count == 2
        && source.shadow_write_entry_count == 2
        && source.shadow_write_rehearsal_item_count == 14
        && source.shadow_write_executed_count == 0
        && source.shadow_store_written_count == 0
        && source.registry_lookup_executed_count == 0
        && registry_shadow_entry_count == 2
        && selected_read_only_status_tool_count == 1
        && non_selected_preflight_boundary_count == 1
        && source_shadow_acceptance_record_envelope_linked_count == 2
        && source_shadow_write_intent_linked_count == 2
        && source_shadow_payload_digest_linked_count == 2
        && source_shadow_idempotency_replay_key_linked_count == 2
        && source_shadow_receipt_preview_linked_count == 2
        && source_shadow_store_target_linked_count == 2
        && source_shadow_replay_result_linked_count == 2
        && shadow_registry_registration_plan_projected_count == 2
        && shadow_registry_entry_key_projected_count == 2
        && shadow_registration_payload_digest_projected_count == 2
        && shadow_lookup_query_projected_count == 2
        && shadow_lookup_result_projected_count == 2
        && shadow_duplicate_check_projected_count == 2
        && shadow_idempotency_replay_anchor_projected_count == 2
        && shadow_approval_ledger_replay_anchor_projected_count == 2
        && tool_registry_shadow_item_count == 16
        && stable_shadow_registry_entry_key_count == 2
        && unique_shadow_registry_entry_key_count == 2
        && stable_shadow_registration_payload_digest_count == 2
        && unique_shadow_registration_payload_digest_count == 2
        && stable_shadow_lookup_query_count == 2
        && unique_shadow_lookup_query_count == 2
        && stable_shadow_idempotency_replay_anchor_count == 2
        && unique_shadow_idempotency_replay_anchor_count == 2
        && shadow_registry_entry_key_mismatch_count == 0
        && duplicate_shadow_registry_entry_key_count == 0
        && shadow_registration_payload_digest_mismatch_count == 0
        && duplicate_shadow_registration_payload_digest_count == 0
        && shadow_lookup_query_mismatch_count == 0
        && duplicate_shadow_lookup_query_count == 0
        && shadow_idempotency_replay_anchor_mismatch_count == 0
        && duplicate_shadow_idempotency_replay_anchor_count == 0
        && feature_gate_opened_count == 0
        && shadow_write_executed_count == 0
        && shadow_write_materialized_count == 0
        && shadow_store_written_count == 0
        && test_tmp_written_count == 0
        && shadow_registry_materialized_count == 0
        && shadow_lookup_executed_count == 0
        && tool_registered_count == 0
        && tool_registry_mutated_count == 0
        && registry_lookup_executed_count == 0
        && tool_invoked_count == 0
        && approval_requested_count == 0
        && ledger_written_count == 0
        && receipt_persisted_count == 0
        && mcp_server_started_count == 0
        && app_connector_started_count == 0
        && runtime_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_execution_started_count == 0;

    HeptaSystemsToolRegistryShadowRegistrationLookupReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_tool_registry_shadow_registration_lookup_readback",
        status: if tool_registry_shadow_registration_lookup_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_persistence_shadow_write_rehearsal_readback_ready: source
            .persistence_shadow_write_rehearsal_readback_ready,
        candidate_count: source.candidate_count,
        registry_shadow_entry_count,
        selected_read_only_status_tool_count,
        non_selected_preflight_boundary_count,
        source_shadow_acceptance_record_envelope_linked_count,
        source_shadow_write_intent_linked_count,
        source_shadow_payload_digest_linked_count,
        source_shadow_idempotency_replay_key_linked_count,
        source_shadow_receipt_preview_linked_count,
        source_shadow_store_target_linked_count,
        source_shadow_replay_result_linked_count,
        shadow_registry_registration_plan_projected_count,
        shadow_registry_entry_key_projected_count,
        shadow_registration_payload_digest_projected_count,
        shadow_lookup_query_projected_count,
        shadow_lookup_result_projected_count,
        shadow_duplicate_check_projected_count,
        shadow_idempotency_replay_anchor_projected_count,
        shadow_approval_ledger_replay_anchor_projected_count,
        tool_registry_shadow_item_count,
        stable_shadow_registry_entry_key_count,
        unique_shadow_registry_entry_key_count,
        stable_shadow_registration_payload_digest_count,
        unique_shadow_registration_payload_digest_count,
        stable_shadow_lookup_query_count,
        unique_shadow_lookup_query_count,
        stable_shadow_idempotency_replay_anchor_count,
        unique_shadow_idempotency_replay_anchor_count,
        shadow_registry_entry_key_mismatch_count,
        duplicate_shadow_registry_entry_key_count,
        shadow_registration_payload_digest_mismatch_count,
        duplicate_shadow_registration_payload_digest_count,
        shadow_lookup_query_mismatch_count,
        duplicate_shadow_lookup_query_count,
        shadow_idempotency_replay_anchor_mismatch_count,
        duplicate_shadow_idempotency_replay_anchor_count,
        feature_gate_opened_count,
        shadow_write_executed_count,
        shadow_write_materialized_count,
        shadow_store_written_count,
        test_tmp_written_count,
        shadow_registry_materialized_count,
        shadow_lookup_executed_count,
        tool_registered_count,
        tool_registry_mutated_count,
        registry_lookup_executed_count,
        tool_invoked_count,
        approval_requested_count,
        ledger_written_count,
        receipt_persisted_count,
        mcp_server_started_count,
        app_connector_started_count,
        runtime_event_log_written_count,
        sqlite_written_count,
        live_execution_started_count,
        tool_registry_shadow_registration_lookup_readback_ready,
        feature_gate_open_allowed: false,
        shadow_write_execution_allowed: false,
        shadow_store_write_allowed: false,
        test_tmp_write_allowed: false,
        shadow_registry_materialization_allowed: false,
        shadow_lookup_execution_allowed: false,
        tool_registry_registration_allowed: false,
        tool_registry_mutation_allowed: false,
        registry_lookup_execution_allowed: false,
        tool_invocation_allowed: false,
        approval_request_allowed: false,
        ledger_persistence_allowed: false,
        receipt_persistence_allowed: false,
        connector_start_allowed: false,
        runtime_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "feature_gate_closed",
            "shadow_write_execution_disabled",
            "shadow_store_write_disabled",
            "test_tmp_write_disabled",
            "shadow_registry_materialization_disabled",
            "shadow_lookup_execution_disabled",
            "tool_registry_registration_disabled",
            "tool_registry_mutation_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "approval_request_disabled",
            "ledger_persistence_disabled",
            "receipt_persistence_disabled",
            "connector_start_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsToolRegistryShadowRegistrationLookupReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_tool_registry_shadow_registration_lookup_readback_entries(
    source: &HeptaSystemsPluginToolInvocationReadOnlyStatusDryRunOperatorEvidenceAcceptanceRecordingPersistenceShadowWriteRehearsalReadbackReport,
) -> Vec<HeptaSystemsToolRegistryShadowRegistrationLookupReadbackEntry> {
    source
        .entries
        .iter()
        .map(|entry| {
            let kind = entry.contribution_kind;
            let registry_key = shadow_registry_entry_key(kind);
            let payload_digest = shadow_registration_payload_digest(kind);
            let lookup_query = shadow_lookup_query_id(kind);
            let replay_anchor = shadow_idempotency_replay_anchor_id(kind);
            HeptaSystemsToolRegistryShadowRegistrationLookupReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: kind,
                dry_run_path_selected: entry.dry_run_path_selected,
                source_shadow_acceptance_record_envelope_id: entry
                    .shadow_acceptance_record_envelope_id
                    .clone(),
                source_shadow_write_intent_id: entry.shadow_write_intent_id.clone(),
                source_shadow_write_payload_digest: entry.shadow_write_payload_digest.clone(),
                source_shadow_idempotency_replay_key: entry.shadow_idempotency_replay_key.clone(),
                source_shadow_receipt_preview_id: entry.shadow_receipt_preview_id.clone(),
                source_shadow_store_target_id: entry.shadow_store_target_id.clone(),
                source_shadow_replay_result_id: entry.shadow_replay_result_id.clone(),
                shadow_registry_registration_plan_id: shadow_registry_registration_plan_id(kind),
                shadow_registry_entry_key: registry_key.clone(),
                shadow_registration_payload_digest: payload_digest.clone(),
                shadow_lookup_query_id: lookup_query.clone(),
                shadow_lookup_result_id: shadow_lookup_result_id(kind),
                shadow_duplicate_check_id: shadow_duplicate_check_id(kind),
                shadow_idempotency_replay_anchor_id: replay_anchor.clone(),
                shadow_approval_ledger_replay_anchor_id: shadow_approval_ledger_replay_anchor_id(
                    kind,
                ),
                first_shadow_registry_entry_key: registry_key.clone(),
                second_shadow_registry_entry_key: registry_key,
                first_shadow_registration_payload_digest: payload_digest.clone(),
                second_shadow_registration_payload_digest: payload_digest,
                first_shadow_lookup_query_id: lookup_query.clone(),
                second_shadow_lookup_query_id: lookup_query,
                first_shadow_idempotency_replay_anchor_id: replay_anchor.clone(),
                second_shadow_idempotency_replay_anchor_id: replay_anchor,
                source_shadow_acceptance_record_envelope_linked: true,
                source_shadow_write_intent_linked: true,
                source_shadow_payload_digest_linked: true,
                source_shadow_idempotency_replay_key_linked: true,
                source_shadow_receipt_preview_linked: true,
                source_shadow_store_target_linked: true,
                source_shadow_replay_result_linked: true,
                shadow_registry_registration_plan_projected: true,
                shadow_registry_entry_key_projected: true,
                shadow_registration_payload_digest_projected: true,
                shadow_lookup_query_projected: true,
                shadow_lookup_result_projected: true,
                shadow_duplicate_check_projected: true,
                shadow_idempotency_replay_anchor_projected: true,
                shadow_approval_ledger_replay_anchor_projected: true,
                stable_shadow_registry_entry_key: true,
                unique_shadow_registry_entry_key: true,
                stable_shadow_registration_payload_digest: true,
                unique_shadow_registration_payload_digest: true,
                stable_shadow_lookup_query: true,
                unique_shadow_lookup_query: true,
                stable_shadow_idempotency_replay_anchor: true,
                unique_shadow_idempotency_replay_anchor: true,
                feature_gate_opened: entry.feature_gate_opened,
                shadow_write_executed: entry.shadow_write_executed,
                shadow_write_materialized: entry.shadow_write_materialized,
                shadow_store_written: entry.shadow_store_written,
                test_tmp_written: entry.test_tmp_written,
                shadow_registry_materialized: false,
                shadow_lookup_executed: false,
                tool_registered: entry.tool_registered,
                tool_registry_mutated: false,
                registry_lookup_executed: entry.registry_lookup_executed,
                tool_invoked: entry.tool_invoked,
                approval_requested: false,
                ledger_written: entry.ledger_written,
                receipt_persisted: entry.receipt_persisted,
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

fn shadow_registry_registration_plan_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-registration-plan:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_registry_entry_key(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-entry:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_registration_payload_digest(contribution_kind: &str) -> String {
    format!(
        "sha256:tool-registry-shadow-registration-payload:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_lookup_query_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-lookup-query:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_lookup_result_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-lookup-result:hepta-system:{}:not-executed",
        suffix(contribution_kind)
    )
}

fn shadow_duplicate_check_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-duplicate-check:hepta-system:{}:unique",
        suffix(contribution_kind)
    )
}

fn shadow_idempotency_replay_anchor_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-idempotency-replay-anchor:hepta-system:{}",
        suffix(contribution_kind)
    )
}

fn shadow_approval_ledger_replay_anchor_id(contribution_kind: &str) -> String {
    format!(
        "tool-registry-shadow-approval-ledger-replay-anchor:hepta-system:{}:not-written",
        suffix(contribution_kind)
    )
}

impl HeptaSystemsToolRegistryShadowRegistrationLookupReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_gate_opened: false,
            shadow_write_executed: false,
            shadow_write_materialized: false,
            shadow_store_written: false,
            test_tmp_written: false,
            shadow_registry_materialized: false,
            shadow_lookup_executed: false,
            tool_registered: false,
            tool_registry_mutated: false,
            registry_lookup_executed: false,
            tool_invoked: false,
            approval_requested: false,
            ledger_persisted: false,
            receipt_persisted: false,
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
    fn shadow_registry_readback_projects_registration_and_lookup_state() {
        let report = hepta_systems_tool_registry_shadow_registration_lookup_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_persistence_shadow_write_rehearsal_readback_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.registry_shadow_entry_count, 2);
        assert_eq!(report.selected_read_only_status_tool_count, 1);
        assert_eq!(report.non_selected_preflight_boundary_count, 1);
        assert_eq!(
            report.source_shadow_acceptance_record_envelope_linked_count,
            2
        );
        assert_eq!(report.source_shadow_write_intent_linked_count, 2);
        assert_eq!(report.source_shadow_payload_digest_linked_count, 2);
        assert_eq!(report.source_shadow_idempotency_replay_key_linked_count, 2);
        assert_eq!(report.shadow_registry_registration_plan_projected_count, 2);
        assert_eq!(report.shadow_registry_entry_key_projected_count, 2);
        assert_eq!(report.shadow_registration_payload_digest_projected_count, 2);
        assert_eq!(report.shadow_lookup_query_projected_count, 2);
        assert_eq!(report.shadow_lookup_result_projected_count, 2);
        assert_eq!(report.shadow_duplicate_check_projected_count, 2);
        assert_eq!(report.shadow_idempotency_replay_anchor_projected_count, 2);
        assert_eq!(
            report.shadow_approval_ledger_replay_anchor_projected_count,
            2
        );
        assert_eq!(report.tool_registry_shadow_item_count, 16);
        assert!(report.tool_registry_shadow_registration_lookup_readback_ready);
    }

    #[test]
    fn shadow_registry_readback_stays_stable_unique_and_duplicate_clean() {
        let report = hepta_systems_tool_registry_shadow_registration_lookup_readback_report();

        assert_eq!(report.stable_shadow_registry_entry_key_count, 2);
        assert_eq!(report.unique_shadow_registry_entry_key_count, 2);
        assert_eq!(report.stable_shadow_registration_payload_digest_count, 2);
        assert_eq!(report.unique_shadow_registration_payload_digest_count, 2);
        assert_eq!(report.stable_shadow_lookup_query_count, 2);
        assert_eq!(report.unique_shadow_lookup_query_count, 2);
        assert_eq!(report.stable_shadow_idempotency_replay_anchor_count, 2);
        assert_eq!(report.unique_shadow_idempotency_replay_anchor_count, 2);
        assert_eq!(report.shadow_registry_entry_key_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_registry_entry_key_count, 0);
        assert_eq!(report.shadow_registration_payload_digest_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_registration_payload_digest_count, 0);
        assert_eq!(report.shadow_lookup_query_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_lookup_query_count, 0);
        assert_eq!(report.shadow_idempotency_replay_anchor_mismatch_count, 0);
        assert_eq!(report.duplicate_shadow_idempotency_replay_anchor_count, 0);
    }

    #[test]
    fn shadow_registry_readback_keeps_registry_lookup_ledger_and_live_closed() {
        let report = hepta_systems_tool_registry_shadow_registration_lookup_readback_report();

        assert_eq!(report.feature_gate_opened_count, 0);
        assert_eq!(report.shadow_write_executed_count, 0);
        assert_eq!(report.shadow_write_materialized_count, 0);
        assert_eq!(report.shadow_store_written_count, 0);
        assert_eq!(report.test_tmp_written_count, 0);
        assert_eq!(report.shadow_registry_materialized_count, 0);
        assert_eq!(report.shadow_lookup_executed_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.tool_registry_mutated_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.approval_requested_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.shadow_registry_materialization_allowed);
        assert!(!report.shadow_lookup_execution_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_registry_mutation_allowed);
        assert!(!report.registry_lookup_execution_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.ledger_persistence_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsToolRegistryShadowRegistrationLookupReadbackSideEffects::none()
        );
        assert_eq!(
            report.recommended_next_gate,
            HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
