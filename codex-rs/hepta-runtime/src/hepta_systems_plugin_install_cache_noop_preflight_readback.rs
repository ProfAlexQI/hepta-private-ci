use serde::Serialize;

use crate::HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport;
use crate::hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_GATE: &str =
    "hepta_systems_plugin_install_cache_noop_preflight_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_plugin_install_cache_noop_preflight_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_operator_evidence_acceptance_packet_ready: bool,
    pub candidate_count: usize,
    pub preflight_entry_count: usize,
    pub cache_path_projected_count: usize,
    pub artifact_digest_projected_count: usize,
    pub rollback_uninstall_plan_projected_count: usize,
    pub idempotency_key_projected_count: usize,
    pub denial_receipt_projected_count: usize,
    pub noop_preflight_ready_count: usize,
    pub noop_preflight_executed_count: usize,
    pub preflight_persisted_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub cache_materialized_count: usize,
    pub cache_mutated_count: usize,
    pub plugin_installed_count: usize,
    pub dynamic_activation_started_count: usize,
    pub install_cache_noop_preflight_readback_ready: bool,
    pub noop_preflight_execution_allowed: bool,
    pub preflight_persistence_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginInstallCacheNoopPreflightReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginInstallCacheNoopPreflightReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginInstallCacheNoopPreflightReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub preflight_route: &'static str,
    pub install_cache_path: &'static str,
    pub artifact_digest: &'static str,
    pub rollback_uninstall_plan_id: &'static str,
    pub idempotency_key: &'static str,
    pub denial_receipt_id: &'static str,
    pub source_operator_packet_projected: bool,
    pub source_checklist_projected: bool,
    pub source_non_acceptance_receipt_projected: bool,
    pub source_evidence_item_required_count: usize,
    pub source_acceptance_check_required_count: usize,
    pub cache_path_projected: bool,
    pub artifact_digest_projected: bool,
    pub rollback_uninstall_plan_projected: bool,
    pub idempotency_key_projected: bool,
    pub denial_receipt_projected: bool,
    pub noop_preflight_ready: bool,
    pub noop_preflight_executed: bool,
    pub preflight_persisted: bool,
    pub denial_receipt_persisted: bool,
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
pub struct HeptaSystemsPluginInstallCacheNoopPreflightReadbackSideEffects {
    pub filesystem_written: bool,
    pub noop_preflight_executed: bool,
    pub preflight_persisted: bool,
    pub denial_receipt_persisted: bool,
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

pub fn hepta_systems_plugin_install_cache_noop_preflight_readback_report()
-> HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport {
    let source = hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report();
    hepta_systems_plugin_install_cache_noop_preflight_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_install_cache_noop_preflight_readback_report_from_source(
    source: &HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport,
) -> HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport {
    let entries = hepta_systems_plugin_install_cache_noop_preflight_readback_entries(source);
    let preflight_entry_count = entries.len();
    let cache_path_projected_count = entries
        .iter()
        .filter(|entry| entry.cache_path_projected)
        .count();
    let artifact_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.artifact_digest_projected)
        .count();
    let rollback_uninstall_plan_projected_count = entries
        .iter()
        .filter(|entry| entry.rollback_uninstall_plan_projected)
        .count();
    let idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_projected)
        .count();
    let denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_projected)
        .count();
    let noop_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.noop_preflight_ready)
        .count();
    let noop_preflight_executed_count = entries
        .iter()
        .filter(|entry| entry.noop_preflight_executed)
        .count();
    let preflight_persisted_count = entries
        .iter()
        .filter(|entry| entry.preflight_persisted)
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

    let install_cache_noop_preflight_readback_ready = source
        .operator_evidence_acceptance_packet_readback_ready
        && source.candidate_count == 2
        && source.packet_projected_count == 2
        && source.checklist_projected_count == 2
        && source.non_acceptance_receipt_projected_count == 2
        && source.evidence_item_recorded_count == 0
        && source.acceptance_check_recorded_count == 0
        && !source.plugin_install_allowed
        && !source.plugin_cache_mutation_allowed
        && !source.install_cache_materialization_allowed
        && !source.dynamic_activation_allowed
        && preflight_entry_count == 2
        && cache_path_projected_count == 2
        && artifact_digest_projected_count == 2
        && rollback_uninstall_plan_projected_count == 2
        && idempotency_key_projected_count == 2
        && denial_receipt_projected_count == 2
        && noop_preflight_ready_count == 2
        && noop_preflight_executed_count == 0
        && preflight_persisted_count == 0
        && denial_receipt_persisted_count == 0
        && cache_materialized_count == 0
        && cache_mutated_count == 0
        && plugin_installed_count == 0
        && dynamic_activation_started_count == 0
        && entries.iter().all(|entry| {
            entry.source_operator_packet_projected
                && entry.source_checklist_projected
                && entry.source_non_acceptance_receipt_projected
                && entry.source_evidence_item_required_count == 4
                && entry.source_acceptance_check_required_count == 5
                && entry.cache_path_projected
                && entry.artifact_digest_projected
                && entry.rollback_uninstall_plan_projected
                && entry.idempotency_key_projected
                && entry.denial_receipt_projected
                && entry.noop_preflight_ready
                && !entry.noop_preflight_executed
                && !entry.preflight_persisted
                && !entry.denial_receipt_persisted
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

    HeptaSystemsPluginInstallCacheNoopPreflightReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_install_cache_noop_preflight_readback",
        status: if install_cache_noop_preflight_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_GATE,
        schema_version: HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_operator_evidence_acceptance_packet_ready: source
            .operator_evidence_acceptance_packet_readback_ready,
        candidate_count: source.candidate_count,
        preflight_entry_count,
        cache_path_projected_count,
        artifact_digest_projected_count,
        rollback_uninstall_plan_projected_count,
        idempotency_key_projected_count,
        denial_receipt_projected_count,
        noop_preflight_ready_count,
        noop_preflight_executed_count,
        preflight_persisted_count,
        denial_receipt_persisted_count,
        cache_materialized_count,
        cache_mutated_count,
        plugin_installed_count,
        dynamic_activation_started_count,
        install_cache_noop_preflight_readback_ready,
        noop_preflight_execution_allowed: false,
        preflight_persistence_allowed: false,
        denial_receipt_persistence_allowed: false,
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
            "noop_preflight_execution_disabled",
            "preflight_persistence_disabled",
            "denial_receipt_persistence_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_NOOP_PREFLIGHT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginInstallCacheNoopPreflightReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_install_cache_noop_preflight_readback_entries(
    source: &HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport,
) -> Vec<HeptaSystemsPluginInstallCacheNoopPreflightReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginInstallCacheNoopPreflightReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                preflight_route: preflight_route(entry.contribution_kind),
                install_cache_path: install_cache_path(entry.contribution_kind),
                artifact_digest: artifact_digest(entry.contribution_kind),
                rollback_uninstall_plan_id: rollback_uninstall_plan_id(entry.contribution_kind),
                idempotency_key: idempotency_key(entry.contribution_kind),
                denial_receipt_id: denial_receipt_id(entry.contribution_kind),
                source_operator_packet_projected: entry.operator_packet_projected,
                source_checklist_projected: entry.checklist_projected,
                source_non_acceptance_receipt_projected: entry.non_acceptance_receipt_projected,
                source_evidence_item_required_count: entry.evidence_item_required_count,
                source_acceptance_check_required_count: entry.acceptance_check_required_count,
                cache_path_projected: true,
                artifact_digest_projected: true,
                rollback_uninstall_plan_projected: true,
                idempotency_key_projected: true,
                denial_receipt_projected: true,
                noop_preflight_ready: true,
                noop_preflight_executed: false,
                preflight_persisted: false,
                denial_receipt_persisted: false,
                cache_materialized: false,
                cache_mutated: false,
                plugin_installed: false,
                dynamic_activation_started: false,
                rollback_uninstall_executed: false,
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

fn preflight_route(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "plugin-install-cache-noop-preflight://hepta-system/mcp",
        "app_connector" => "plugin-install-cache-noop-preflight://hepta-system/app",
        _ => "plugin-install-cache-noop-preflight://hepta-system/unknown",
    }
}

fn install_cache_path(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => ".hepta/plugin-install-cache/noop/hepta-system/mcp/hepta_system_local_mcp",
        "app_connector" => {
            ".hepta/plugin-install-cache/noop/hepta-system/app/hepta_system_local_app"
        }
        _ => ".hepta/plugin-install-cache/noop/hepta-system/unknown",
    }
}

fn artifact_digest(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "sha256:noop-hepta-system-local-mcp-manifest-fixture",
        "app_connector" => "sha256:noop-hepta-system-local-app-manifest-fixture",
        _ => "sha256:noop-hepta-system-unknown-manifest-fixture",
    }
}

fn rollback_uninstall_plan_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "rollback-uninstall-plan:hepta-system:local-mcp:no-exec",
        "app_connector" => "rollback-uninstall-plan:hepta-system:local-app:no-exec",
        _ => "rollback-uninstall-plan:hepta-system:unknown:no-exec",
    }
}

fn idempotency_key(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "install-cache-noop:hepta-system:local-mcp:v0",
        "app_connector" => "install-cache-noop:hepta-system:local-app:v0",
        _ => "install-cache-noop:hepta-system:unknown:v0",
    }
}

fn denial_receipt_id(contribution_kind: &str) -> &'static str {
    match contribution_kind {
        "mcp_server" => "denial-receipt:hepta-system:local-mcp:install-cache-noop",
        "app_connector" => "denial-receipt:hepta-system:local-app:install-cache-noop",
        _ => "denial-receipt:hepta-system:unknown:install-cache-noop",
    }
}

impl HeptaSystemsPluginInstallCacheNoopPreflightReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            noop_preflight_executed: false,
            preflight_persisted: false,
            denial_receipt_persisted: false,
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
    fn noop_preflight_projects_cache_digest_rollback_idempotency_and_denial_receipts() {
        let report = hepta_systems_plugin_install_cache_noop_preflight_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_evidence_acceptance_packet_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.preflight_entry_count, 2);
        assert_eq!(report.cache_path_projected_count, 2);
        assert_eq!(report.artifact_digest_projected_count, 2);
        assert_eq!(report.rollback_uninstall_plan_projected_count, 2);
        assert_eq!(report.idempotency_key_projected_count, 2);
        assert_eq!(report.denial_receipt_projected_count, 2);
        assert_eq!(report.noop_preflight_ready_count, 2);
        assert!(report.install_cache_noop_preflight_readback_ready);
    }

    #[test]
    fn noop_preflight_keeps_preflight_cache_and_receipts_unpersisted() {
        let report = hepta_systems_plugin_install_cache_noop_preflight_readback_report();

        assert_eq!(report.noop_preflight_executed_count, 0);
        assert_eq!(report.preflight_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.cache_materialized_count, 0);
        assert_eq!(report.cache_mutated_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.dynamic_activation_started_count, 0);
        assert!(!report.noop_preflight_execution_allowed);
        assert!(!report.preflight_persistence_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
    }

    #[test]
    fn noop_preflight_keeps_runtime_tooling_and_live_closed() {
        let report = hepta_systems_plugin_install_cache_noop_preflight_readback_report();

        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.rollback_uninstall_execution_allowed);
        assert!(!report.permission_grant_allowed);
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
            HeptaSystemsPluginInstallCacheNoopPreflightReadbackSideEffects::none()
        );
    }
}
