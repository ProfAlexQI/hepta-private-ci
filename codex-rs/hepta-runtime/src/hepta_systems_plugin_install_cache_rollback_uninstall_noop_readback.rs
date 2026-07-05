use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport;
use crate::hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_GATE: &str =
    "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_idempotency_denial_receipt_ready: bool,
    pub candidate_count: usize,
    pub rollback_entry_count: usize,
    pub stable_rollback_uninstall_plan_count: usize,
    pub unique_rollback_uninstall_plan_count: usize,
    pub rollback_noop_route_projected_count: usize,
    pub uninstall_noop_route_projected_count: usize,
    pub rollback_guard_projected_count: usize,
    pub uninstall_guard_projected_count: usize,
    pub cache_restore_block_projected_count: usize,
    pub denial_receipt_anchor_projected_count: usize,
    pub rollback_uninstall_plan_mismatch_count: usize,
    pub duplicate_rollback_uninstall_plan_count: usize,
    pub rollback_uninstall_noop_ready_count: usize,
    pub rollback_uninstall_executed_count: usize,
    pub rollback_plan_persisted_count: usize,
    pub uninstall_plan_persisted_count: usize,
    pub idempotency_index_written_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub cache_materialized_count: usize,
    pub cache_mutated_count: usize,
    pub plugin_installed_count: usize,
    pub dynamic_activation_started_count: usize,
    pub install_cache_rollback_uninstall_noop_readback_ready: bool,
    pub rollback_uninstall_execution_allowed: bool,
    pub rollback_plan_persistence_allowed: bool,
    pub uninstall_plan_persistence_allowed: bool,
    pub idempotency_index_write_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub permission_grant_allowed: bool,
    pub mcp_server_start_allowed: bool,
    pub app_connector_start_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub ledger_write_allowed: bool,
    pub approval_request_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: &'static str,
    pub install_cache_path: &'static str,
    pub artifact_digest: &'static str,
    pub first_rollback_uninstall_plan_id: &'static str,
    pub second_rollback_uninstall_plan_id: &'static str,
    pub stable_rollback_uninstall_plan: bool,
    pub unique_rollback_uninstall_plan: bool,
    pub rollback_noop_route: &'static str,
    pub uninstall_noop_route: &'static str,
    pub rollback_guard_key: &'static str,
    pub uninstall_guard_key: &'static str,
    pub cache_restore_block_key: &'static str,
    pub denial_receipt_anchor: &'static str,
    pub idempotency_denial_anchor: &'static str,
    pub rollback_noop_route_projected: bool,
    pub uninstall_noop_route_projected: bool,
    pub rollback_guard_projected: bool,
    pub uninstall_guard_projected: bool,
    pub cache_restore_block_projected: bool,
    pub denial_receipt_anchor_projected: bool,
    pub rollback_uninstall_noop_ready: bool,
    pub rollback_uninstall_executed: bool,
    pub rollback_plan_persisted: bool,
    pub uninstall_plan_persisted: bool,
    pub idempotency_index_written: bool,
    pub denial_receipt_persisted: bool,
    pub cache_materialized: bool,
    pub cache_mutated: bool,
    pub plugin_installed: bool,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackSideEffects {
    pub filesystem_written: bool,
    pub rollback_uninstall_executed: bool,
    pub rollback_plan_persisted: bool,
    pub uninstall_plan_persisted: bool,
    pub idempotency_index_written: bool,
    pub denial_receipt_persisted: bool,
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

pub fn hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report()
-> HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport {
    let source = hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report();
    hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report_from_source(
    source: &HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport,
) -> HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport {
    let entries =
        hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_entries(source);
    let rollback_entry_count = entries.len();
    let stable_rollback_uninstall_plan_count = entries
        .iter()
        .filter(|entry| entry.stable_rollback_uninstall_plan)
        .count();
    let unique_rollback_uninstall_plan_count = entries
        .iter()
        .map(|entry| entry.first_rollback_uninstall_plan_id)
        .collect::<HashSet<_>>()
        .len();
    let rollback_noop_route_projected_count = entries
        .iter()
        .filter(|entry| entry.rollback_noop_route_projected)
        .count();
    let uninstall_noop_route_projected_count = entries
        .iter()
        .filter(|entry| entry.uninstall_noop_route_projected)
        .count();
    let rollback_guard_projected_count = entries
        .iter()
        .filter(|entry| entry.rollback_guard_projected)
        .count();
    let uninstall_guard_projected_count = entries
        .iter()
        .filter(|entry| entry.uninstall_guard_projected)
        .count();
    let cache_restore_block_projected_count = entries
        .iter()
        .filter(|entry| entry.cache_restore_block_projected)
        .count();
    let denial_receipt_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_anchor_projected)
        .count();
    let rollback_uninstall_plan_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_rollback_uninstall_plan)
        .count();
    let duplicate_rollback_uninstall_plan_count =
        rollback_entry_count.saturating_sub(unique_rollback_uninstall_plan_count);
    let rollback_uninstall_noop_ready_count = entries
        .iter()
        .filter(|entry| entry.rollback_uninstall_noop_ready)
        .count();
    let rollback_uninstall_executed_count = entries
        .iter()
        .filter(|entry| entry.rollback_uninstall_executed)
        .count();
    let rollback_plan_persisted_count = entries
        .iter()
        .filter(|entry| entry.rollback_plan_persisted)
        .count();
    let uninstall_plan_persisted_count = entries
        .iter()
        .filter(|entry| entry.uninstall_plan_persisted)
        .count();
    let idempotency_index_written_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_written)
        .count();
    let denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persisted)
        .count();
    let cache_materialized_count = entries
        .iter()
        .filter(|entry| entry.cache_materialized)
        .count();
    let cache_mutated_count = entries.iter().filter(|entry| entry.cache_mutated).count();
    let plugin_installed_count = entries
        .iter()
        .filter(|entry| entry.plugin_installed)
        .count();
    let dynamic_activation_started_count = entries
        .iter()
        .filter(|entry| entry.dynamic_activation_started)
        .count();

    let install_cache_rollback_uninstall_noop_readback_ready = source
        .install_cache_idempotency_denial_receipt_readback_ready
        && source.candidate_count == 2
        && source.stable_idempotency_key_count == 2
        && source.stable_denial_receipt_id_count == 2
        && source.idempotency_index_written_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.cache_materialized_count == 0
        && source.cache_mutated_count == 0
        && source.plugin_installed_count == 0
        && source.dynamic_activation_started_count == 0
        && rollback_entry_count == 2
        && stable_rollback_uninstall_plan_count == 2
        && unique_rollback_uninstall_plan_count == 2
        && rollback_noop_route_projected_count == 2
        && uninstall_noop_route_projected_count == 2
        && rollback_guard_projected_count == 2
        && uninstall_guard_projected_count == 2
        && cache_restore_block_projected_count == 2
        && denial_receipt_anchor_projected_count == 2
        && rollback_uninstall_plan_mismatch_count == 0
        && duplicate_rollback_uninstall_plan_count == 0
        && rollback_uninstall_noop_ready_count == 2
        && rollback_uninstall_executed_count == 0
        && rollback_plan_persisted_count == 0
        && uninstall_plan_persisted_count == 0
        && idempotency_index_written_count == 0
        && denial_receipt_persisted_count == 0
        && cache_materialized_count == 0
        && cache_mutated_count == 0
        && plugin_installed_count == 0
        && dynamic_activation_started_count == 0
        && entries.iter().all(|entry| {
            entry.stable_rollback_uninstall_plan
                && entry.unique_rollback_uninstall_plan
                && entry.rollback_noop_route_projected
                && entry.uninstall_noop_route_projected
                && entry.rollback_guard_projected
                && entry.uninstall_guard_projected
                && entry.cache_restore_block_projected
                && entry.denial_receipt_anchor_projected
                && entry.rollback_uninstall_noop_ready
                && !entry.rollback_uninstall_executed
                && !entry.rollback_plan_persisted
                && !entry.uninstall_plan_persisted
                && !entry.idempotency_index_written
                && !entry.denial_receipt_persisted
                && !entry.cache_materialized
                && !entry.cache_mutated
                && !entry.plugin_installed
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
        });

    HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback",
        status: if install_cache_rollback_uninstall_noop_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_idempotency_denial_receipt_ready: source
            .install_cache_idempotency_denial_receipt_readback_ready,
        candidate_count: source.candidate_count,
        rollback_entry_count,
        stable_rollback_uninstall_plan_count,
        unique_rollback_uninstall_plan_count,
        rollback_noop_route_projected_count,
        uninstall_noop_route_projected_count,
        rollback_guard_projected_count,
        uninstall_guard_projected_count,
        cache_restore_block_projected_count,
        denial_receipt_anchor_projected_count,
        rollback_uninstall_plan_mismatch_count,
        duplicate_rollback_uninstall_plan_count,
        rollback_uninstall_noop_ready_count,
        rollback_uninstall_executed_count,
        rollback_plan_persisted_count,
        uninstall_plan_persisted_count,
        idempotency_index_written_count,
        denial_receipt_persisted_count,
        cache_materialized_count,
        cache_mutated_count,
        plugin_installed_count,
        dynamic_activation_started_count,
        install_cache_rollback_uninstall_noop_readback_ready,
        rollback_uninstall_execution_allowed: false,
        rollback_plan_persistence_allowed: false,
        uninstall_plan_persistence_allowed: false,
        idempotency_index_write_allowed: false,
        denial_receipt_persistence_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        install_cache_materialization_allowed: false,
        dynamic_activation_allowed: false,
        permission_grant_allowed: false,
        mcp_server_start_allowed: false,
        app_connector_start_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        ledger_write_allowed: false,
        approval_request_allowed: false,
        receipt_persistence_allowed: false,
        runtime_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "rollback_uninstall_execution_disabled",
            "rollback_plan_persistence_disabled",
            "uninstall_plan_persistence_disabled",
            "idempotency_index_write_disabled",
            "denial_receipt_persistence_disabled",
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "install_cache_materialization_disabled",
            "dynamic_activation_disabled",
            "permission_grant_disabled",
            "mcp_server_start_disabled",
            "app_connector_start_disabled",
            "tool_registry_registration_disabled",
            "tool_invocation_disabled",
            "ledger_write_disabled",
            "approval_request_disabled",
            "receipt_persistence_disabled",
            "runtime_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_entries(
    source: &HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport,
) -> Vec<HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.source_preflight_route,
                install_cache_path: entry.install_cache_path,
                artifact_digest: entry.artifact_digest,
                first_rollback_uninstall_plan_id: entry.rollback_uninstall_plan_id,
                second_rollback_uninstall_plan_id: entry.rollback_uninstall_plan_id,
                stable_rollback_uninstall_plan: true,
                unique_rollback_uninstall_plan: true,
                rollback_noop_route: rollback_noop_route(entry.contribution_kind),
                uninstall_noop_route: uninstall_noop_route(entry.contribution_kind),
                rollback_guard_key: rollback_guard_key(entry.contribution_kind),
                uninstall_guard_key: uninstall_guard_key(entry.contribution_kind),
                cache_restore_block_key: cache_restore_block_key(entry.contribution_kind),
                denial_receipt_anchor: denial_receipt_anchor(entry.contribution_kind),
                idempotency_denial_anchor: entry.idempotency_denial_anchor,
                rollback_noop_route_projected: true,
                uninstall_noop_route_projected: true,
                rollback_guard_projected: true,
                uninstall_guard_projected: true,
                cache_restore_block_projected: true,
                denial_receipt_anchor_projected: true,
                rollback_uninstall_noop_ready: true,
                rollback_uninstall_executed: entry.rollback_uninstall_executed,
                rollback_plan_persisted: false,
                uninstall_plan_persisted: false,
                idempotency_index_written: entry.idempotency_index_written,
                denial_receipt_persisted: entry.denial_receipt_persisted,
                cache_materialized: entry.cache_materialized,
                cache_mutated: entry.cache_mutated,
                plugin_installed: entry.plugin_installed,
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
            },
        )
        .collect()
}

fn rollback_noop_route(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "plugin-rollback-noop://hepta-system/mcp",
        "app_connector" => "plugin-rollback-noop://hepta-system/app",
        _ => "plugin-rollback-noop://hepta-system/unknown",
    }
}

fn uninstall_noop_route(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "plugin-uninstall-noop://hepta-system/mcp",
        "app_connector" => "plugin-uninstall-noop://hepta-system/app",
        _ => "plugin-uninstall-noop://hepta-system/unknown",
    }
}

fn rollback_guard_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "rollback-guard:hepta-system:local-mcp:no-exec",
        "app_connector" => "rollback-guard:hepta-system:local-app:no-exec",
        _ => "rollback-guard:hepta-system:unknown:no-exec",
    }
}

fn uninstall_guard_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "uninstall-guard:hepta-system:local-mcp:no-exec",
        "app_connector" => "uninstall-guard:hepta-system:local-app:no-exec",
        _ => "uninstall-guard:hepta-system:unknown:no-exec",
    }
}

fn cache_restore_block_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "cache-restore-block:hepta-system:local-mcp:no-cache-write",
        "app_connector" => "cache-restore-block:hepta-system:local-app:no-cache-write",
        _ => "cache-restore-block:hepta-system:unknown:no-cache-write",
    }
}

fn denial_receipt_anchor(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "denial-anchor:hepta-system:local-mcp:rollback-uninstall-noop",
        "app_connector" => "denial-anchor:hepta-system:local-app:rollback-uninstall-noop",
        _ => "denial-anchor:hepta-system:unknown:rollback-uninstall-noop",
    }
}

impl HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            rollback_uninstall_executed: false,
            rollback_plan_persisted: false,
            uninstall_plan_persisted: false,
            idempotency_index_written: false,
            denial_receipt_persisted: false,
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
    fn rollback_uninstall_noop_projects_stable_guarded_routes() {
        let report = hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_idempotency_denial_receipt_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.rollback_entry_count, 2);
        assert_eq!(report.stable_rollback_uninstall_plan_count, 2);
        assert_eq!(report.unique_rollback_uninstall_plan_count, 2);
        assert_eq!(report.rollback_noop_route_projected_count, 2);
        assert_eq!(report.uninstall_noop_route_projected_count, 2);
        assert_eq!(report.rollback_guard_projected_count, 2);
        assert_eq!(report.uninstall_guard_projected_count, 2);
        assert_eq!(report.cache_restore_block_projected_count, 2);
        assert_eq!(report.denial_receipt_anchor_projected_count, 2);
        assert!(report.install_cache_rollback_uninstall_noop_readback_ready);
    }

    #[test]
    fn rollback_uninstall_noop_rejects_execution_persistence_and_duplicates() {
        let report = hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report();

        assert_eq!(report.rollback_uninstall_plan_mismatch_count, 0);
        assert_eq!(report.duplicate_rollback_uninstall_plan_count, 0);
        assert_eq!(report.rollback_uninstall_noop_ready_count, 2);
        assert_eq!(report.rollback_uninstall_executed_count, 0);
        assert_eq!(report.rollback_plan_persisted_count, 0);
        assert_eq!(report.uninstall_plan_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert!(!report.rollback_uninstall_execution_allowed);
        assert!(!report.rollback_plan_persistence_allowed);
        assert!(!report.uninstall_plan_persistence_allowed);
    }

    #[test]
    fn rollback_uninstall_noop_keeps_install_cache_activation_and_live_closed() {
        let report = hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report();

        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.cache_mutated_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.dynamic_activation_started_count, 0);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.mcp_server_start_allowed);
        assert!(!report.app_connector_start_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.approval_request_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginInstallCacheRollbackUninstallNoopReadbackSideEffects::none()
        );
    }
}
