use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport;
use crate::hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_GATE: &str =
    "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_tool_registry_registration_denial_receipt_ready: bool,
    pub candidate_count: usize,
    pub invocation_entry_count: usize,
    pub invocation_denial_id_projected_count: usize,
    pub noop_result_projected_count: usize,
    pub noop_result_digest_projected_count: usize,
    pub stable_invocation_denial_receipt_count: usize,
    pub unique_invocation_denial_receipt_count: usize,
    pub invocation_denial_receipt_projected_count: usize,
    pub ledger_denial_anchor_projected_count: usize,
    pub approval_denial_anchor_projected_count: usize,
    pub receipt_denial_anchor_projected_count: usize,
    pub idempotency_key_projected_count: usize,
    pub stable_idempotency_key_count: usize,
    pub unique_idempotency_key_count: usize,
    pub invocation_denial_receipt_mismatch_count: usize,
    pub duplicate_invocation_denial_receipt_count: usize,
    pub idempotency_key_mismatch_count: usize,
    pub duplicate_idempotency_key_count: usize,
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
    pub tool_invocation_noop_denial_receipt_readback_ready: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_registry_mutation_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub noop_result_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: &'static str,
    pub tool_schema_digest: &'static str,
    pub tool_registry_registration_denial_id: &'static str,
    pub source_registration_denial_receipt_id: &'static str,
    pub tool_invocation_denial_id: &'static str,
    pub noop_result_projection_id: &'static str,
    pub noop_result_digest: &'static str,
    pub first_invocation_denial_receipt_id: &'static str,
    pub second_invocation_denial_receipt_id: &'static str,
    pub stable_invocation_denial_receipt: bool,
    pub unique_invocation_denial_receipt: bool,
    pub ledger_denial_anchor_id: &'static str,
    pub approval_denial_anchor_id: &'static str,
    pub receipt_denial_anchor_id: &'static str,
    pub first_invocation_idempotency_key: &'static str,
    pub second_invocation_idempotency_key: &'static str,
    pub stable_idempotency_key: bool,
    pub unique_idempotency_key: bool,
    pub invocation_denial_id_projected: bool,
    pub noop_result_projected: bool,
    pub noop_result_digest_projected: bool,
    pub invocation_denial_receipt_projected: bool,
    pub ledger_denial_anchor_projected: bool,
    pub approval_denial_anchor_projected: bool,
    pub receipt_denial_anchor_projected: bool,
    pub idempotency_key_projected: bool,
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
pub struct HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackSideEffects {
    pub filesystem_written: bool,
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

pub fn hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report()
-> HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackReport {
    let source = hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report();
    hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report_from_source(
    source: &HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport,
) -> HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackReport {
    let entries = hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_entries(source);
    let invocation_entry_count = entries.len();
    let invocation_denial_id_projected_count = entries
        .iter()
        .filter(|entry| entry.invocation_denial_id_projected)
        .count();
    let noop_result_projected_count = entries
        .iter()
        .filter(|entry| entry.noop_result_projected)
        .count();
    let noop_result_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.noop_result_digest_projected)
        .count();
    let stable_invocation_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_invocation_denial_receipt)
        .count();
    let unique_invocation_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_invocation_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let invocation_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.invocation_denial_receipt_projected)
        .count();
    let ledger_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_denial_anchor_projected)
        .count();
    let approval_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.approval_denial_anchor_projected)
        .count();
    let receipt_denial_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_denial_anchor_projected)
        .count();
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
        .map(|entry| entry.first_invocation_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let invocation_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_invocation_denial_receipt)
        .count();
    let duplicate_invocation_denial_receipt_count =
        invocation_entry_count.saturating_sub(unique_invocation_denial_receipt_count);
    let idempotency_key_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_idempotency_key)
        .count();
    let duplicate_idempotency_key_count =
        invocation_entry_count.saturating_sub(unique_idempotency_key_count);
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

    let tool_invocation_noop_denial_receipt_readback_ready = source
        .tool_registry_registration_denial_receipt_readback_ready
        && source.candidate_count == 2
        && source.invocation_denial_projected_count == 2
        && source.tool_registered_count == 0
        && source.tool_registry_mutated_count == 0
        && source.registry_lookup_executed_count == 0
        && source.tool_invoked_count == 0
        && source.ledger_written_count == 0
        && source.approval_requested_count == 0
        && source.receipt_persisted_count == 0
        && source.dynamic_activation_started_count == 0
        && source.permission_granted_count == 0
        && source.mcp_server_started_count == 0
        && source.app_connector_started_count == 0
        && source.plugin_installed_count == 0
        && source.cache_materialized_count == 0
        && source.cache_mutated_count == 0
        && source.runtime_event_log_written_count == 0
        && source.sqlite_written_count == 0
        && source.live_execution_started_count == 0
        && invocation_entry_count == 2
        && invocation_denial_id_projected_count == 2
        && noop_result_projected_count == 2
        && noop_result_digest_projected_count == 2
        && stable_invocation_denial_receipt_count == 2
        && unique_invocation_denial_receipt_count == 2
        && invocation_denial_receipt_projected_count == 2
        && ledger_denial_anchor_projected_count == 2
        && approval_denial_anchor_projected_count == 2
        && receipt_denial_anchor_projected_count == 2
        && idempotency_key_projected_count == 2
        && stable_idempotency_key_count == 2
        && unique_idempotency_key_count == 2
        && invocation_denial_receipt_mismatch_count == 0
        && duplicate_invocation_denial_receipt_count == 0
        && idempotency_key_mismatch_count == 0
        && duplicate_idempotency_key_count == 0
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
            entry.invocation_denial_id_projected
                && entry.noop_result_projected
                && entry.noop_result_digest_projected
                && entry.invocation_denial_receipt_projected
                && entry.stable_invocation_denial_receipt
                && entry.unique_invocation_denial_receipt
                && entry.ledger_denial_anchor_projected
                && entry.approval_denial_anchor_projected
                && entry.receipt_denial_anchor_projected
                && entry.idempotency_key_projected
                && entry.stable_idempotency_key
                && entry.unique_idempotency_key
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
        });

    HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback",
        status: if tool_invocation_noop_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_tool_registry_registration_denial_receipt_ready: source
            .tool_registry_registration_denial_receipt_readback_ready,
        candidate_count: source.candidate_count,
        invocation_entry_count,
        invocation_denial_id_projected_count,
        noop_result_projected_count,
        noop_result_digest_projected_count,
        stable_invocation_denial_receipt_count,
        unique_invocation_denial_receipt_count,
        invocation_denial_receipt_projected_count,
        ledger_denial_anchor_projected_count,
        approval_denial_anchor_projected_count,
        receipt_denial_anchor_projected_count,
        idempotency_key_projected_count,
        stable_idempotency_key_count,
        unique_idempotency_key_count,
        invocation_denial_receipt_mismatch_count,
        duplicate_invocation_denial_receipt_count,
        idempotency_key_mismatch_count,
        duplicate_idempotency_key_count,
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
        tool_invocation_noop_denial_receipt_readback_ready,
        tool_registry_registration_allowed: false,
        tool_registry_mutation_allowed: false,
        registry_lookup_execution_allowed: false,
        tool_invocation_allowed: false,
        noop_result_persistence_allowed: false,
        ledger_write_allowed: false,
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
            "tool_registry_registration_disabled",
            "tool_registry_mutation_disabled",
            "registry_lookup_execution_disabled",
            "tool_invocation_disabled",
            "noop_result_persistence_disabled",
            "ledger_write_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_NOOP_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_entries(
    source: &HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport,
) -> Vec<HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.source_preflight_route,
                tool_schema_digest: entry.tool_schema_digest,
                tool_registry_registration_denial_id: entry.tool_registry_registration_denial_id,
                source_registration_denial_receipt_id: entry.first_registration_denial_receipt_id,
                tool_invocation_denial_id: entry.invocation_denial_id,
                noop_result_projection_id: noop_result_projection_id(entry.contribution_kind),
                noop_result_digest: noop_result_digest(entry.contribution_kind),
                first_invocation_denial_receipt_id: invocation_denial_receipt_id(
                    entry.contribution_kind,
                ),
                second_invocation_denial_receipt_id: invocation_denial_receipt_id(
                    entry.contribution_kind,
                ),
                stable_invocation_denial_receipt: true,
                unique_invocation_denial_receipt: true,
                ledger_denial_anchor_id: ledger_denial_anchor_id(entry.contribution_kind),
                approval_denial_anchor_id: approval_denial_anchor_id(entry.contribution_kind),
                receipt_denial_anchor_id: receipt_denial_anchor_id(entry.contribution_kind),
                first_invocation_idempotency_key: invocation_idempotency_key(
                    entry.contribution_kind,
                ),
                second_invocation_idempotency_key: invocation_idempotency_key(
                    entry.contribution_kind,
                ),
                stable_idempotency_key: true,
                unique_idempotency_key: true,
                invocation_denial_id_projected: true,
                noop_result_projected: true,
                noop_result_digest_projected: true,
                invocation_denial_receipt_projected: true,
                ledger_denial_anchor_projected: true,
                approval_denial_anchor_projected: true,
                receipt_denial_anchor_projected: true,
                idempotency_key_projected: true,
                tool_registered: entry.tool_registered,
                tool_registry_mutated: entry.tool_registry_mutated,
                registry_lookup_executed: entry.registry_lookup_executed,
                tool_invoked: entry.tool_invoked,
                noop_result_persisted: false,
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
            },
        )
        .collect()
}

fn noop_result_projection_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "noop-result:hepta-system:local-mcp:denied-no-invocation",
        "app_connector" => "noop-result:hepta-system:local-app:denied-no-invocation",
        _ => "noop-result:hepta-system:unknown:denied-no-invocation",
    }
}

fn noop_result_digest(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "noop-result-digest:hepta-system:local-mcp:denied-v0",
        "app_connector" => "noop-result-digest:hepta-system:local-app:denied-v0",
        _ => "noop-result-digest:hepta-system:unknown:denied-v0",
    }
}

fn invocation_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "tool-invocation-denial-receipt:hepta-system:local-mcp:no-invocation",
        "app_connector" => "tool-invocation-denial-receipt:hepta-system:local-app:no-invocation",
        _ => "tool-invocation-denial-receipt:hepta-system:unknown:no-invocation",
    }
}

fn ledger_denial_anchor_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "ledger-denial-anchor:hepta-system:local-mcp:no-write",
        "app_connector" => "ledger-denial-anchor:hepta-system:local-app:no-write",
        _ => "ledger-denial-anchor:hepta-system:unknown:no-write",
    }
}

fn approval_denial_anchor_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "approval-denial-anchor:hepta-system:local-mcp:no-request",
        "app_connector" => "approval-denial-anchor:hepta-system:local-app:no-request",
        _ => "approval-denial-anchor:hepta-system:unknown:no-request",
    }
}

fn receipt_denial_anchor_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "receipt-denial-anchor:hepta-system:local-mcp:no-persistence",
        "app_connector" => "receipt-denial-anchor:hepta-system:local-app:no-persistence",
        _ => "receipt-denial-anchor:hepta-system:unknown:no-persistence",
    }
}

fn invocation_idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "tool-invocation-idempotency:hepta-system:local-mcp:no-invocation",
        "app_connector" => "tool-invocation-idempotency:hepta-system:local-app:no-invocation",
        _ => "tool-invocation-idempotency:hepta-system:unknown:no-invocation",
    }
}

impl HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
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
    fn invocation_noop_denial_receipt_projects_invocation_contract() {
        let report = hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_tool_registry_registration_denial_receipt_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.invocation_entry_count, 2);
        assert_eq!(report.invocation_denial_id_projected_count, 2);
        assert_eq!(report.noop_result_projected_count, 2);
        assert_eq!(report.noop_result_digest_projected_count, 2);
        assert_eq!(report.invocation_denial_receipt_projected_count, 2);
        assert_eq!(report.ledger_denial_anchor_projected_count, 2);
        assert_eq!(report.approval_denial_anchor_projected_count, 2);
        assert_eq!(report.receipt_denial_anchor_projected_count, 2);
        assert_eq!(report.idempotency_key_projected_count, 2);
        assert!(report.tool_invocation_noop_denial_receipt_readback_ready);
    }

    #[test]
    fn invocation_noop_denial_receipt_stays_stable_unique_and_unpersisted() {
        let report = hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report();

        assert_eq!(report.stable_invocation_denial_receipt_count, 2);
        assert_eq!(report.unique_invocation_denial_receipt_count, 2);
        assert_eq!(report.stable_idempotency_key_count, 2);
        assert_eq!(report.unique_idempotency_key_count, 2);
        assert_eq!(report.invocation_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_invocation_denial_receipt_count, 0);
        assert_eq!(report.idempotency_key_mismatch_count, 0);
        assert_eq!(report.duplicate_idempotency_key_count, 0);
        assert_eq!(report.noop_result_persisted_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
    }

    #[test]
    fn invocation_noop_denial_receipt_keeps_tooling_runtime_and_live_closed() {
        let report = hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_report();

        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.tool_registry_mutated_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.approval_requested_count, 0);
        assert_eq!(report.dynamic_activation_started_count, 0);
        assert_eq!(report.mcp_server_started_count, 0);
        assert_eq!(report.app_connector_started_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.noop_result_persistence_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.approval_request_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolInvocationNoopDenialReceiptReadbackSideEffects::none()
        );
    }
}
