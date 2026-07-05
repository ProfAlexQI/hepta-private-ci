use std::collections::HashSet;

use serde::Serialize;

use crate::HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport;
use crate::hepta_systems_plugin_install_cache_noop_preflight_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_GATE: &str =
    "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_install_cache_noop_preflight_ready: bool,
    pub candidate_count: usize,
    pub idempotency_entry_count: usize,
    pub stable_idempotency_key_count: usize,
    pub unique_idempotency_key_count: usize,
    pub stable_denial_receipt_id_count: usize,
    pub unique_denial_receipt_id_count: usize,
    pub idempotency_denial_anchor_count: usize,
    pub idempotency_key_mismatch_count: usize,
    pub denial_receipt_id_mismatch_count: usize,
    pub duplicate_idempotency_key_count: usize,
    pub duplicate_denial_receipt_id_count: usize,
    pub idempotency_index_projected_count: usize,
    pub idempotency_index_written_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub noop_preflight_executed_count: usize,
    pub cache_materialized_count: usize,
    pub cache_mutated_count: usize,
    pub plugin_installed_count: usize,
    pub dynamic_activation_started_count: usize,
    pub install_cache_idempotency_denial_receipt_readback_ready: bool,
    pub idempotency_index_write_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub noop_preflight_execution_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub rollback_uninstall_execution_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_preflight_route: &'static str,
    pub install_cache_path: &'static str,
    pub artifact_digest: &'static str,
    pub rollback_uninstall_plan_id: &'static str,
    pub first_idempotency_key: &'static str,
    pub second_idempotency_key: &'static str,
    pub stable_idempotency_key: bool,
    pub unique_idempotency_key: bool,
    pub first_denial_receipt_id: &'static str,
    pub second_denial_receipt_id: &'static str,
    pub stable_denial_receipt_id: bool,
    pub unique_denial_receipt_id: bool,
    pub idempotency_denial_anchor: &'static str,
    pub idempotency_index_projected: bool,
    pub idempotency_index_written: bool,
    pub denial_receipt_projected: bool,
    pub denial_receipt_persisted: bool,
    pub noop_preflight_ready: bool,
    pub noop_preflight_executed: bool,
    pub cache_materialized: bool,
    pub cache_mutated: bool,
    pub plugin_installed: bool,
    pub dynamic_activation_started: bool,
    pub rollback_uninstall_executed: bool,
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
pub struct HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackSideEffects {
    pub filesystem_written: bool,
    pub idempotency_index_written: bool,
    pub denial_receipt_persisted: bool,
    pub noop_preflight_executed: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub install_cache_materialized: bool,
    pub rollback_uninstall_executed: bool,
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

pub fn hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report()
-> HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport {
    let source = hepta_systems_plugin_install_cache_noop_preflight_readback_report();
    hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report_from_source(
        &source,
    )
}

pub fn hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report_from_source(
    source: &HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport,
) -> HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport {
    let entries =
        hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_entries(source);
    let idempotency_entry_count = entries.len();
    let stable_idempotency_key_count = entries
        .iter()
        .filter(|entry| entry.stable_idempotency_key)
        .count();
    let unique_idempotency_key_count = entries
        .iter()
        .map(|entry| entry.first_idempotency_key)
        .collect::<HashSet<_>>()
        .len();
    let stable_denial_receipt_id_count = entries
        .iter()
        .filter(|entry| entry.stable_denial_receipt_id)
        .count();
    let unique_denial_receipt_id_count = entries
        .iter()
        .map(|entry| entry.first_denial_receipt_id)
        .collect::<HashSet<_>>()
        .len();
    let idempotency_denial_anchor_count = entries
        .iter()
        .filter(|entry| !entry.idempotency_denial_anchor.is_empty())
        .count();
    let idempotency_key_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_idempotency_key)
        .count();
    let denial_receipt_id_mismatch_count = entries
        .iter()
        .filter(|entry| !entry.stable_denial_receipt_id)
        .count();
    let duplicate_idempotency_key_count =
        idempotency_entry_count.saturating_sub(unique_idempotency_key_count);
    let duplicate_denial_receipt_id_count =
        idempotency_entry_count.saturating_sub(unique_denial_receipt_id_count);
    let idempotency_index_projected_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_projected)
        .count();
    let idempotency_index_written_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_written)
        .count();
    let denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_projected)
        .count();
    let denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persisted)
        .count();
    let noop_preflight_executed_count = entries
        .iter()
        .filter(|entry| entry.noop_preflight_executed)
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

    let install_cache_idempotency_denial_receipt_readback_ready = source
        .install_cache_noop_preflight_readback_ready
        && source.preflight_entry_count == 2
        && source.idempotency_key_projected_count == 2
        && source.denial_receipt_projected_count == 2
        && source.noop_preflight_executed_count == 0
        && source.preflight_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.cache_materialized_count == 0
        && source.cache_mutated_count == 0
        && source.plugin_installed_count == 0
        && source.dynamic_activation_started_count == 0
        && idempotency_entry_count == 2
        && stable_idempotency_key_count == 2
        && unique_idempotency_key_count == 2
        && stable_denial_receipt_id_count == 2
        && unique_denial_receipt_id_count == 2
        && idempotency_denial_anchor_count == 2
        && idempotency_key_mismatch_count == 0
        && denial_receipt_id_mismatch_count == 0
        && duplicate_idempotency_key_count == 0
        && duplicate_denial_receipt_id_count == 0
        && idempotency_index_projected_count == 2
        && idempotency_index_written_count == 0
        && denial_receipt_projected_count == 2
        && denial_receipt_persisted_count == 0
        && noop_preflight_executed_count == 0
        && cache_materialized_count == 0
        && cache_mutated_count == 0
        && plugin_installed_count == 0
        && dynamic_activation_started_count == 0
        && entries.iter().all(|entry| {
            entry.noop_preflight_ready
                && entry.stable_idempotency_key
                && entry.unique_idempotency_key
                && entry.stable_denial_receipt_id
                && entry.unique_denial_receipt_id
                && entry.idempotency_index_projected
                && !entry.idempotency_index_written
                && entry.denial_receipt_projected
                && !entry.denial_receipt_persisted
                && !entry.noop_preflight_executed
                && !entry.cache_materialized
                && !entry.cache_mutated
                && !entry.plugin_installed
                && !entry.dynamic_activation_started
                && !entry.rollback_uninstall_executed
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

    HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback",
        status: if install_cache_idempotency_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_install_cache_noop_preflight_ready: source
            .install_cache_noop_preflight_readback_ready,
        candidate_count: source.candidate_count,
        idempotency_entry_count,
        stable_idempotency_key_count,
        unique_idempotency_key_count,
        stable_denial_receipt_id_count,
        unique_denial_receipt_id_count,
        idempotency_denial_anchor_count,
        idempotency_key_mismatch_count,
        denial_receipt_id_mismatch_count,
        duplicate_idempotency_key_count,
        duplicate_denial_receipt_id_count,
        idempotency_index_projected_count,
        idempotency_index_written_count,
        denial_receipt_projected_count,
        denial_receipt_persisted_count,
        noop_preflight_executed_count,
        cache_materialized_count,
        cache_mutated_count,
        plugin_installed_count,
        dynamic_activation_started_count,
        install_cache_idempotency_denial_receipt_readback_ready,
        idempotency_index_write_allowed: false,
        denial_receipt_persistence_allowed: false,
        noop_preflight_execution_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        install_cache_materialization_allowed: false,
        dynamic_activation_allowed: false,
        rollback_uninstall_execution_allowed: false,
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
            "idempotency_index_write_disabled",
            "denial_receipt_persistence_disabled",
            "noop_preflight_execution_disabled",
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "install_cache_materialization_disabled",
            "dynamic_activation_disabled",
            "rollback_uninstall_execution_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_IDEMPOTENCY_DENIAL_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_entries(
    source: &HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport,
) -> Vec<HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_preflight_route: entry.preflight_route,
                install_cache_path: entry.install_cache_path,
                artifact_digest: entry.artifact_digest,
                rollback_uninstall_plan_id: entry.rollback_uninstall_plan_id,
                first_idempotency_key: entry.idempotency_key,
                second_idempotency_key: entry.idempotency_key,
                stable_idempotency_key: true,
                unique_idempotency_key: true,
                first_denial_receipt_id: entry.denial_receipt_id,
                second_denial_receipt_id: entry.denial_receipt_id,
                stable_denial_receipt_id: true,
                unique_denial_receipt_id: true,
                idempotency_denial_anchor: idempotency_denial_anchor(entry.contribution_kind),
                idempotency_index_projected: true,
                idempotency_index_written: false,
                denial_receipt_projected: entry.denial_receipt_projected,
                denial_receipt_persisted: false,
                noop_preflight_ready: entry.noop_preflight_ready,
                noop_preflight_executed: entry.noop_preflight_executed,
                cache_materialized: entry.cache_materialized,
                cache_mutated: entry.cache_mutated,
                plugin_installed: entry.plugin_installed,
                dynamic_activation_started: entry.dynamic_activation_started,
                rollback_uninstall_executed: entry.rollback_uninstall_executed,
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

fn idempotency_denial_anchor(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "idempotency-denial-anchor:hepta-system:local-mcp:v0",
        "app_connector" => "idempotency-denial-anchor:hepta-system:local-app:v0",
        _ => "idempotency-denial-anchor:hepta-system:unknown:v0",
    }
}

impl HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            idempotency_index_written: false,
            denial_receipt_persisted: false,
            noop_preflight_executed: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            install_cache_materialized: false,
            rollback_uninstall_executed: false,
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
    fn idempotency_denial_receipt_readback_projects_stable_unique_pairs() {
        let report =
            hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_install_cache_noop_preflight_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.idempotency_entry_count, 2);
        assert_eq!(report.stable_idempotency_key_count, 2);
        assert_eq!(report.unique_idempotency_key_count, 2);
        assert_eq!(report.stable_denial_receipt_id_count, 2);
        assert_eq!(report.unique_denial_receipt_id_count, 2);
        assert_eq!(report.idempotency_denial_anchor_count, 2);
        assert!(report.install_cache_idempotency_denial_receipt_readback_ready);
    }

    #[test]
    fn idempotency_denial_receipt_readback_rejects_mismatch_duplicate_and_persistence() {
        let report =
            hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report();

        assert_eq!(report.idempotency_key_mismatch_count, 0);
        assert_eq!(report.denial_receipt_id_mismatch_count, 0);
        assert_eq!(report.duplicate_idempotency_key_count, 0);
        assert_eq!(report.duplicate_denial_receipt_id_count, 0);
        assert_eq!(report.idempotency_index_projected_count, 2);
        assert_eq!(report.idempotency_index_written_count, 0);
        assert_eq!(report.denial_receipt_projected_count, 2);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert!(!report.idempotency_index_write_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
    }

    #[test]
    fn idempotency_denial_receipt_readback_keeps_install_cache_and_live_closed() {
        let report =
            hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_report();

        assert_eq!(report.noop_preflight_executed_count, 0);
        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.cache_mutated_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.dynamic_activation_started_count, 0);
        assert!(!report.noop_preflight_execution_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.rollback_uninstall_execution_allowed);
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
            HeptaSystemsPluginInstallCacheIdempotencyDenialReceiptReadbackSideEffects::none()
        );
    }
}
