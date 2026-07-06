use serde::Serialize;

use crate::HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport;
use crate::hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_GATE: &str =
    "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_rollback_uninstall_noop_ready: bool,
    pub candidate_count: usize,
    pub activation_entry_count: usize,
    pub manual_activation_event_projected_count: usize,
    pub permission_gate_projected_count: usize,
    pub connector_start_plan_projected_count: usize,
    pub mcp_server_start_plan_projected_count: usize,
    pub app_connector_start_plan_projected_count: usize,
    pub tool_registry_registration_denial_projected_count: usize,
    pub ledger_denial_projected_count: usize,
    pub receipt_denial_projected_count: usize,
    pub activation_denial_receipt_projected_count: usize,
    pub dynamic_activation_started_count: usize,
    pub permission_granted_count: usize,
    pub mcp_server_started_count: usize,
    pub app_connector_started_count: usize,
    pub tool_registered_count: usize,
    pub tool_invoked_count: usize,
    pub ledger_written_count: usize,
    pub approval_requested_count: usize,
    pub receipt_persisted_count: usize,
    pub runtime_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_execution_started_count: usize,
    pub plugin_installed_count: usize,
    pub cache_materialized_count: usize,
    pub cache_mutated_count: usize,
    pub dynamic_activation_connector_start_boundary_ready: bool,
    pub dynamic_activation_allowed: bool,
    pub permission_grant_allowed: bool,
    pub mcp_server_start_allowed: bool,
    pub app_connector_start_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub ledger_write_allowed: bool,
    pub approval_request_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: &'static str,
    pub install_cache_path: &'static str,
    pub artifact_digest: &'static str,
    pub rollback_uninstall_plan_id: &'static str,
    pub activation_event_type: &'static str,
    pub permission_gate_key: &'static str,
    pub connector_start_plan_id: &'static str,
    pub connector_start_route: &'static str,
    pub tool_registry_registration_denial_id: &'static str,
    pub ledger_denial_id: &'static str,
    pub receipt_denial_id: &'static str,
    pub activation_denial_receipt_id: &'static str,
    pub manual_activation_event_projected: bool,
    pub manual_activation_required: bool,
    pub permission_gate_projected: bool,
    pub connector_start_plan_projected: bool,
    pub mcp_server_start_plan_projected: bool,
    pub app_connector_start_plan_projected: bool,
    pub tool_registry_registration_denial_projected: bool,
    pub ledger_denial_projected: bool,
    pub receipt_denial_projected: bool,
    pub activation_denial_receipt_projected: bool,
    pub dynamic_activation_boundary_ready: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub tool_registered: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub receipt_persisted: bool,
    pub runtime_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
    pub plugin_installed: bool,
    pub cache_materialized: bool,
    pub cache_mutated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackSideEffects {
    pub filesystem_written: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub install_cache_materialized: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub tool_registry_mutated: bool,
    pub tool_registered: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub receipt_persisted: bool,
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

pub fn hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report()
-> HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport {
    let source = hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report();
    hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report_from_source(
        &source,
    )
}

pub fn hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report_from_source(
    source: &HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport,
) -> HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport {
    let entries =
        hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_entries(source);
    let activation_entry_count = entries.len();
    let manual_activation_event_projected_count = entries
        .iter()
        .filter(|entry| entry.manual_activation_event_projected)
        .count();
    let permission_gate_projected_count = entries
        .iter()
        .filter(|entry| entry.permission_gate_projected)
        .count();
    let connector_start_plan_projected_count = entries
        .iter()
        .filter(|entry| entry.connector_start_plan_projected)
        .count();
    let mcp_server_start_plan_projected_count = entries
        .iter()
        .filter(|entry| entry.mcp_server_start_plan_projected)
        .count();
    let app_connector_start_plan_projected_count = entries
        .iter()
        .filter(|entry| entry.app_connector_start_plan_projected)
        .count();
    let tool_registry_registration_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.tool_registry_registration_denial_projected)
        .count();
    let ledger_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.ledger_denial_projected)
        .count();
    let receipt_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_denial_projected)
        .count();
    let activation_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.activation_denial_receipt_projected)
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
    let tool_registered_count = entries.iter().filter(|entry| entry.tool_registered).count();
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
    let runtime_event_log_written_count = entries
        .iter()
        .filter(|entry| entry.runtime_event_log_written)
        .count();
    let sqlite_written_count = entries.iter().filter(|entry| entry.sqlite_written).count();
    let live_execution_started_count = entries
        .iter()
        .filter(|entry| entry.live_execution_started)
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

    let dynamic_activation_connector_start_boundary_ready = source
        .install_cache_rollback_uninstall_noop_readback_ready
        && source.candidate_count == 2
        && source.rollback_uninstall_executed_count == 0
        && source.rollback_plan_persisted_count == 0
        && source.uninstall_plan_persisted_count == 0
        && source.idempotency_index_written_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.cache_materialized_count == 0
        && source.cache_mutated_count == 0
        && source.plugin_installed_count == 0
        && source.dynamic_activation_started_count == 0
        && activation_entry_count == 2
        && manual_activation_event_projected_count == 2
        && permission_gate_projected_count == 2
        && connector_start_plan_projected_count == 2
        && mcp_server_start_plan_projected_count == 1
        && app_connector_start_plan_projected_count == 1
        && tool_registry_registration_denial_projected_count == 2
        && ledger_denial_projected_count == 2
        && receipt_denial_projected_count == 2
        && activation_denial_receipt_projected_count == 2
        && dynamic_activation_started_count == 0
        && permission_granted_count == 0
        && mcp_server_started_count == 0
        && app_connector_started_count == 0
        && tool_registered_count == 0
        && tool_invoked_count == 0
        && ledger_written_count == 0
        && approval_requested_count == 0
        && receipt_persisted_count == 0
        && runtime_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_execution_started_count == 0
        && plugin_installed_count == 0
        && cache_materialized_count == 0
        && cache_mutated_count == 0
        && entries.iter().all(|entry| {
            entry.manual_activation_event_projected
                && entry.manual_activation_required
                && entry.permission_gate_projected
                && entry.connector_start_plan_projected
                && entry.tool_registry_registration_denial_projected
                && entry.ledger_denial_projected
                && entry.receipt_denial_projected
                && entry.activation_denial_receipt_projected
                && entry.dynamic_activation_boundary_ready
                && !entry.dynamic_activation_started
                && !entry.permission_granted
                && !entry.mcp_server_started
                && !entry.app_connector_started
                && !entry.tool_registered
                && !entry.tool_invoked
                && !entry.ledger_written
                && !entry.approval_requested
                && !entry.receipt_persisted
                && !entry.runtime_event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
                && !entry.plugin_installed
                && !entry.cache_materialized
                && !entry.cache_mutated
        });

    HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
        status: if dynamic_activation_connector_start_boundary_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_rollback_uninstall_noop_ready: source
            .install_cache_rollback_uninstall_noop_readback_ready,
        candidate_count: source.candidate_count,
        activation_entry_count,
        manual_activation_event_projected_count,
        permission_gate_projected_count,
        connector_start_plan_projected_count,
        mcp_server_start_plan_projected_count,
        app_connector_start_plan_projected_count,
        tool_registry_registration_denial_projected_count,
        ledger_denial_projected_count,
        receipt_denial_projected_count,
        activation_denial_receipt_projected_count,
        dynamic_activation_started_count,
        permission_granted_count,
        mcp_server_started_count,
        app_connector_started_count,
        tool_registered_count,
        tool_invoked_count,
        ledger_written_count,
        approval_requested_count,
        receipt_persisted_count,
        runtime_event_log_written_count,
        sqlite_written_count,
        live_execution_started_count,
        plugin_installed_count,
        cache_materialized_count,
        cache_mutated_count,
        dynamic_activation_connector_start_boundary_ready,
        dynamic_activation_allowed: false,
        permission_grant_allowed: false,
        mcp_server_start_allowed: false,
        app_connector_start_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        ledger_write_allowed: false,
        approval_request_allowed: false,
        receipt_persistence_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        install_cache_materialization_allowed: false,
        runtime_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "dynamic_activation_disabled",
            "permission_grant_disabled",
            "mcp_server_start_disabled",
            "app_connector_start_disabled",
            "tool_registry_registration_disabled",
            "tool_invocation_disabled",
            "ledger_write_disabled",
            "approval_request_disabled",
            "receipt_persistence_disabled",
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "install_cache_materialization_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_entries(
    source: &HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport,
) -> Vec<HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.source_preflight_route,
                install_cache_path: entry.install_cache_path,
                artifact_digest: entry.artifact_digest,
                rollback_uninstall_plan_id: entry.first_rollback_uninstall_plan_id,
                activation_event_type: "manual",
                permission_gate_key: permission_gate_key(entry.contribution_kind),
                connector_start_plan_id: connector_start_plan_id(entry.contribution_kind),
                connector_start_route: connector_start_route(entry.contribution_kind),
                tool_registry_registration_denial_id: tool_registry_registration_denial_id(
                    entry.contribution_kind,
                ),
                ledger_denial_id: ledger_denial_id(entry.contribution_kind),
                receipt_denial_id: receipt_denial_id(entry.contribution_kind),
                activation_denial_receipt_id: activation_denial_receipt_id(entry.contribution_kind),
                manual_activation_event_projected: true,
                manual_activation_required: true,
                permission_gate_projected: true,
                connector_start_plan_projected: true,
                mcp_server_start_plan_projected: entry.contribution_kind == "mcp_server",
                app_connector_start_plan_projected: entry.contribution_kind == "app_connector",
                tool_registry_registration_denial_projected: true,
                ledger_denial_projected: true,
                receipt_denial_projected: true,
                activation_denial_receipt_projected: true,
                dynamic_activation_boundary_ready: true,
                dynamic_activation_started: entry.dynamic_activation_started,
                permission_granted: entry.permission_granted,
                mcp_server_started: entry.mcp_server_started,
                app_connector_started: entry.app_connector_started,
                tool_registered: entry.tool_registered,
                tool_invoked: entry.tool_invoked,
                ledger_written: entry.ledger_written,
                approval_requested: entry.approval_requested,
                receipt_persisted: entry.receipt_persisted,
                runtime_event_log_written: entry.runtime_event_log_written,
                sqlite_written: entry.sqlite_written,
                live_execution_started: entry.live_execution_started,
                plugin_installed: entry.plugin_installed,
                cache_materialized: entry.cache_materialized,
                cache_mutated: entry.cache_mutated,
            },
        )
        .collect()
}

fn permission_gate_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "permission-gate:hepta-system:local-mcp:read-only-network-none",
        "app_connector" => {
            "permission-gate:hepta-system:local-app:connector-hepta-local-network-none"
        }
        _ => "permission-gate:hepta-system:unknown",
    }
}

fn connector_start_plan_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "connector-start-plan:hepta-system:local-mcp:blocked",
        "app_connector" => "connector-start-plan:hepta-system:local-app:blocked",
        _ => "connector-start-plan:hepta-system:unknown:blocked",
    }
}

fn connector_start_route(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "mcp-start://hepta-system/local-mcp/blocked",
        "app_connector" => "app-connector-start://hepta-system/local-app/blocked",
        _ => "connector-start://hepta-system/unknown/blocked",
    }
}

fn tool_registry_registration_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "tool-registry-denial:hepta-system:local-mcp:no-registration",
        "app_connector" => "tool-registry-denial:hepta-system:local-app:no-registration",
        _ => "tool-registry-denial:hepta-system:unknown:no-registration",
    }
}

fn ledger_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "ledger-denial:hepta-system:local-mcp:no-write",
        "app_connector" => "ledger-denial:hepta-system:local-app:no-write",
        _ => "ledger-denial:hepta-system:unknown:no-write",
    }
}

fn receipt_denial_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "receipt-denial:hepta-system:local-mcp:no-persistence",
        "app_connector" => "receipt-denial:hepta-system:local-app:no-persistence",
        _ => "receipt-denial:hepta-system:unknown:no-persistence",
    }
}

fn activation_denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "activation-denial-receipt:hepta-system:local-mcp:no-activation",
        "app_connector" => "activation-denial-receipt:hepta-system:local-app:no-activation",
        _ => "activation-denial-receipt:hepta-system:unknown:no-activation",
    }
}

impl HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            install_cache_materialized: false,
            dynamic_activation_started: false,
            permission_granted: false,
            mcp_server_started: false,
            app_connector_started: false,
            tool_registry_mutated: false,
            tool_registered: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            receipt_persisted: false,
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
    fn dynamic_activation_boundary_projects_manual_activation_and_connector_plans() {
        let report =
            hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_rollback_uninstall_noop_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.activation_entry_count, 2);
        assert_eq!(report.manual_activation_event_projected_count, 2);
        assert_eq!(report.permission_gate_projected_count, 2);
        assert_eq!(report.connector_start_plan_projected_count, 2);
        assert_eq!(report.mcp_server_start_plan_projected_count, 1);
        assert_eq!(report.app_connector_start_plan_projected_count, 1);
        assert!(report.dynamic_activation_connector_start_boundary_ready);
    }

    #[test]
    fn dynamic_activation_boundary_projects_denials_without_runtime_actions() {
        let report =
            hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report();

        assert_eq!(report.tool_registry_registration_denial_projected_count, 2);
        assert_eq!(report.ledger_denial_projected_count, 2);
        assert_eq!(report.receipt_denial_projected_count, 2);
        assert_eq!(report.activation_denial_receipt_projected_count, 2);
        assert_eq!(report.dynamic_activation_started_count, 0);
        assert_eq!(report.permission_granted_count, 0);
        assert_eq!(report.mcp_server_started_count, 0);
        assert_eq!(report.app_connector_started_count, 0);
        assert_eq!(report.tool_registered_count, 0);
        assert_eq!(report.tool_invoked_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
    }

    #[test]
    fn dynamic_activation_boundary_keeps_install_cache_and_live_closed() {
        let report =
            hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report();

        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.cache_mutated_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_execution_started_count, 0);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.permission_grant_allowed);
        assert!(!report.mcp_server_start_allowed);
        assert!(!report.app_connector_start_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginDynamicActivationConnectorStartBoundaryReadbackSideEffects::none()
        );
    }
}
