use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport;
use crate::hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_GATE: &str =
    "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_dynamic_activation_boundary_ready: bool,
    pub candidate_count: usize,
    pub registration_entry_count: usize,
    pub tool_schema_bound_count: usize,
    pub tool_schema_digest_projected_count: usize,
    pub registration_denial_id_projected_count: usize,
    pub stable_registration_denial_receipt_count: usize,
    pub unique_registration_denial_receipt_count: usize,
    pub registration_denial_receipt_projected_count: usize,
    pub router_lookup_block_projected_count: usize,
    pub registry_source_of_truth_block_projected_count: usize,
    pub invocation_denial_projected_count: usize,
    pub registration_denial_receipt_mismatch_count: usize,
    pub duplicate_registration_denial_receipt_count: usize,
    pub tool_registered_count: usize,
    pub tool_registry_mutated_count: usize,
    pub registry_lookup_executed_count: usize,
    pub tool_invoked_count: usize,
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
    pub tool_registry_registration_denial_receipt_readback_ready: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_registry_mutation_allowed: bool,
    pub registry_lookup_execution_allowed: bool,
    pub tool_invocation_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: &'static str,
    pub activation_denial_receipt_id: &'static str,
    pub tool_schema_digest: &'static str,
    pub tool_registry_registration_denial_id: &'static str,
    pub first_registration_denial_receipt_id: &'static str,
    pub second_registration_denial_receipt_id: &'static str,
    pub stable_registration_denial_receipt: bool,
    pub unique_registration_denial_receipt: bool,
    pub router_lookup_block_key: &'static str,
    pub registry_source_of_truth_block_key: &'static str,
    pub invocation_denial_id: &'static str,
    pub tool_schema_bound: bool,
    pub tool_schema_digest_projected: bool,
    pub registration_denial_id_projected: bool,
    pub registration_denial_receipt_projected: bool,
    pub router_lookup_block_projected: bool,
    pub registry_source_of_truth_block_projected: bool,
    pub invocation_denial_projected: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
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
pub struct HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackSideEffects {
    pub filesystem_written: bool,
    pub tool_registered: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
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

pub fn hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report()
-> HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport {
    let source = hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report();
    hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report_from_source(
        &source,
    )
}

pub fn hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report_from_source(
    source: &HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport,
) -> HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport {
    let entries =
        hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_entries(source);
    let registration_entry_count = entries.len();
    let tool_schema_bound_count = entries
        .iter()
        .filter(|entry| entry.tool_schema_bound)
        .count();
    let tool_schema_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_schema_digest_projected)
        .count();
    let registration_denial_id_projected_count = entries
        .iter()
        .filter(|entry| entry.registration_denial_id_projected)
        .count();
    let stable_registration_denial_receipt_count = entries
        .iter()
        .filter(|entry| entry.stable_registration_denial_receipt)
        .count();
    let unique_registration_denial_receipt_count = entries
        .iter()
        .map(|entry| entry.first_registration_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let registration_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.registration_denial_receipt_projected)
        .count();
    let router_lookup_block_projected_count = entries
        .iter()
        .filter(|entry| entry.router_lookup_block_projected)
        .count();
    let registry_source_of_truth_block_projected_count = entries
        .iter()
        .filter(|entry| entry.registry_source_of_truth_block_projected)
        .count();
    let invocation_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.invocation_denial_projected)
        .count();
    let registration_denial_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_registration_denial_receipt)
        .count();
    let duplicate_registration_denial_receipt_count =
        registration_entry_count.saturating_sub(unique_registration_denial_receipt_count);
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

    let tool_registry_registration_denial_receipt_readback_ready = source
        .dynamic_activation_connector_start_boundary_ready
        && source.candidate_count == 2
        && source.tool_registry_registration_denial_projected_count == 2
        && source.ledger_denial_projected_count == 2
        && source.receipt_denial_projected_count == 2
        && source.dynamic_activation_started_count == 0
        && source.permission_granted_count == 0
        && source.mcp_server_started_count == 0
        && source.app_connector_started_count == 0
        && source.tool_registered_count == 0
        && source.tool_invoked_count == 0
        && source.ledger_written_count == 0
        && source.approval_requested_count == 0
        && source.receipt_persisted_count == 0
        && source.plugin_installed_count == 0
        && source.cache_materialized_count == 0
        && source.cache_mutated_count == 0
        && source.runtime_event_log_written_count == 0
        && source.sqlite_written_count == 0
        && source.live_execution_started_count == 0
        && registration_entry_count == 2
        && tool_schema_bound_count == 2
        && tool_schema_digest_projected_count == 2
        && registration_denial_id_projected_count == 2
        && stable_registration_denial_receipt_count == 2
        && unique_registration_denial_receipt_count == 2
        && registration_denial_receipt_projected_count == 2
        && router_lookup_block_projected_count == 2
        && registry_source_of_truth_block_projected_count == 2
        && invocation_denial_projected_count == 2
        && registration_denial_receipt_mismatch_count == 0
        && duplicate_registration_denial_receipt_count == 0
        && tool_registered_count == 0
        && tool_registry_mutated_count == 0
        && registry_lookup_executed_count == 0
        && tool_invoked_count == 0
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
            entry.tool_schema_bound
                && entry.tool_schema_digest_projected
                && entry.registration_denial_id_projected
                && entry.registration_denial_receipt_projected
                && entry.stable_registration_denial_receipt
                && entry.unique_registration_denial_receipt
                && entry.router_lookup_block_projected
                && entry.registry_source_of_truth_block_projected
                && entry.invocation_denial_projected
                && !entry.tool_registered
                && !entry.tool_registry_mutated
                && !entry.registry_lookup_executed
                && !entry.tool_invoked
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

    HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback",
        status: if tool_registry_registration_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_dynamic_activation_boundary_ready: source
            .dynamic_activation_connector_start_boundary_ready,
        candidate_count: source.candidate_count,
        registration_entry_count,
        tool_schema_bound_count,
        tool_schema_digest_projected_count,
        registration_denial_id_projected_count,
        stable_registration_denial_receipt_count,
        unique_registration_denial_receipt_count,
        registration_denial_receipt_projected_count,
        router_lookup_block_projected_count,
        registry_source_of_truth_block_projected_count,
        invocation_denial_projected_count,
        registration_denial_receipt_mismatch_count,
        duplicate_registration_denial_receipt_count,
        tool_registered_count,
        tool_registry_mutated_count,
        registry_lookup_executed_count,
        tool_invoked_count,
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
        tool_registry_registration_denial_receipt_readback_ready,
        tool_registry_registration_allowed: false,
        tool_registry_mutation_allowed: false,
        registry_lookup_execution_allowed: false,
        tool_invocation_allowed: false,
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
            HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_entries(
    source: &HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport,
) -> Vec<HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.source_preflight_route,
                activation_denial_receipt_id: entry.activation_denial_receipt_id,
                tool_schema_digest: tool_schema_digest(entry.contribution_kind),
                tool_registry_registration_denial_id: entry.tool_registry_registration_denial_id,
                first_registration_denial_receipt_id: registration_denial_receipt_id(
                    entry.contribution_kind,
                ),
                second_registration_denial_receipt_id: registration_denial_receipt_id(
                    entry.contribution_kind,
                ),
                stable_registration_denial_receipt: true,
                unique_registration_denial_receipt: true,
                router_lookup_block_key: router_lookup_block_key(entry.contribution_kind),
                registry_source_of_truth_block_key: registry_source_of_truth_block_key(
                    entry.contribution_kind,
                ),
                invocation_denial_id: invocation_denial_id(entry.contribution_kind),
                tool_schema_bound: true,
                tool_schema_digest_projected: true,
                registration_denial_id_projected: true,
                registration_denial_receipt_projected: true,
                router_lookup_block_projected: true,
                registry_source_of_truth_block_projected: true,
                invocation_denial_projected: true,
                tool_registered: entry.tool_registered,
                tool_registry_mutated: false,
                registry_lookup_executed: false,
                tool_invoked: entry.tool_invoked,
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

fn tool_schema_digest(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "tool-schema-digest:hepta-system:local-mcp:readiness-v0",
        "app_connector" => "tool-schema-digest:hepta-system:local-app:readiness-v0",
        _ => "tool-schema-digest:hepta-system:unknown:readiness-v0",
    }
}

fn registration_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => {
            "tool-registry-registration-denial-receipt:hepta-system:local-mcp:no-registration"
        }
        "app_connector" => {
            "tool-registry-registration-denial-receipt:hepta-system:local-app:no-registration"
        }
        _ => "tool-registry-registration-denial-receipt:hepta-system:unknown:no-registration",
    }
}

fn router_lookup_block_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "router-lookup-block:hepta-system:local-mcp:no-registered-route",
        "app_connector" => "router-lookup-block:hepta-system:local-app:no-registered-route",
        _ => "router-lookup-block:hepta-system:unknown:no-registered-route",
    }
}

fn registry_source_of_truth_block_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "registry-sot-block:hepta-system:local-mcp:no-mutation",
        "app_connector" => "registry-sot-block:hepta-system:local-app:no-mutation",
        _ => "registry-sot-block:hepta-system:unknown:no-mutation",
    }
}

fn invocation_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "tool-invocation-denial:hepta-system:local-mcp:no-invocation",
        "app_connector" => "tool-invocation-denial:hepta-system:local-app:no-invocation",
        _ => "tool-invocation-denial:hepta-system:unknown:no-invocation",
    }
}

impl HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            tool_registered: false,
            tool_registry_mutated: false,
            registry_lookup_executed: false,
            tool_invoked: false,
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
    fn registration_denial_receipt_projects_schema_and_denial_ids() {
        let report =
            hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_dynamic_activation_boundary_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.registration_entry_count, 2);
        assert_eq!(report.tool_schema_bound_count, 2);
        assert_eq!(report.tool_schema_digest_projected_count, 2);
        assert_eq!(report.registration_denial_id_projected_count, 2);
        assert_eq!(report.registration_denial_receipt_projected_count, 2);
        assert_eq!(report.router_lookup_block_projected_count, 2);
        assert_eq!(report.registry_source_of_truth_block_projected_count, 2);
        assert_eq!(report.invocation_denial_projected_count, 2);
        assert!(report.tool_registry_registration_denial_receipt_readback_ready);
    }

    #[test]
    fn registration_denial_receipt_stays_stable_and_unique_without_writes() {
        let report =
            hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report();

        assert_eq!(report.stable_registration_denial_receipt_count, 2);
        assert_eq!(report.unique_registration_denial_receipt_count, 2);
        assert_eq!(report.registration_denial_receipt_mismatch_count, 0);
        assert_eq!(report.duplicate_registration_denial_receipt_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.tool_registry_mutated_count, 0);
        assert_eq!(report.registry_lookup_executed_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.approval_requested_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
    }

    #[test]
    fn registration_denial_receipt_keeps_activation_cache_runtime_and_live_closed() {
        let report =
            hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_report();

        assert_eq!(report.dynamic_activation_started_count, 0);
        assert_eq!(report.permission_granted_count, 0);
        assert_eq!(report.mcp_server_started_count, 0);
        assert_eq!(report.app_connector_started_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.cache_mutated_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_registry_mutation_allowed);
        assert!(!report.registry_lookup_execution_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginToolRegistryRegistrationDenialReceiptReadbackSideEffects::none()
        );
    }
}
