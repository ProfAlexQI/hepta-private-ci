use serde::Serialize;

use crate::HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport;
use crate::hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_GATE: &str =
    "hepta_systems_plugin_signature_trust_install_cache_boundary_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_signature_trust_install_cache_boundary_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_operator_evidence_acceptance_packet_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_canonical_manifest_contract_ready: bool,
    pub source_signature_boundary_checked_count: usize,
    pub source_trust_boundary_checked_count: usize,
    pub source_install_blocked_count: usize,
    pub candidate_count: usize,
    pub signature_boundary_ready_count: usize,
    pub trust_boundary_ready_count: usize,
    pub install_cache_boundary_ready_count: usize,
    pub operator_evidence_required_count: usize,
    pub operator_acceptance_required_count: usize,
    pub explicit_non_acceptance_receipt_projected_count: usize,
    pub signature_artifact_present_count: usize,
    pub signature_verified_count: usize,
    pub trust_root_present_count: usize,
    pub trust_root_accepted_count: usize,
    pub install_cache_materialized_count: usize,
    pub install_cache_mutated_count: usize,
    pub evidence_recorded_count: usize,
    pub acceptance_recorded_count: usize,
    pub plugin_install_allowed_count: usize,
    pub dynamic_activation_allowed_count: usize,
    pub signature_trust_install_cache_boundary_readback_ready: bool,
    pub signature_acceptance_allowed: bool,
    pub trust_root_acceptance_allowed: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_recording_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub boundary_route: &'static str,
    pub signature_boundary_ready: bool,
    pub signature_artifact_present: bool,
    pub signature_digest_projected: bool,
    pub signature_verified: bool,
    pub signature_required_before_install: bool,
    pub signature_acceptance_allowed: bool,
    pub trust_boundary_ready: bool,
    pub trust_root_present: bool,
    pub trust_root_digest_projected: bool,
    pub trust_root_accepted: bool,
    pub trust_required_before_install: bool,
    pub trust_root_acceptance_allowed: bool,
    pub install_cache_boundary_ready: bool,
    pub install_cache_route_projected: bool,
    pub install_cache_materialized: bool,
    pub install_cache_mutated: bool,
    pub install_cache_materialization_allowed: bool,
    pub operator_evidence_required: bool,
    pub operator_evidence_recorded: bool,
    pub operator_evidence_recording_allowed: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_recorded: bool,
    pub operator_acceptance_recording_allowed: bool,
    pub explicit_non_acceptance_receipt_projected: bool,
    pub plugin_install_allowed: bool,
    pub dynamic_activation_allowed: bool,
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
pub struct HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackSideEffects {
    pub filesystem_written: bool,
    pub manifest_rewritten: bool,
    pub manifest_schema_written: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub install_cache_materialized: bool,
    pub package_lock_written: bool,
    pub remote_sync_started: bool,
    pub loader_invoked: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub signature_verified: bool,
    pub signature_accepted: bool,
    pub trust_root_accepted: bool,
    pub operator_evidence_recorded: bool,
    pub operator_acceptance_recorded: bool,
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

pub fn hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report()
-> HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport {
    let source =
        hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report();
    hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report_from_source(
    source: &HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport,
) -> HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport {
    let entries =
        hepta_systems_plugin_signature_trust_install_cache_boundary_readback_entries(source);
    let signature_boundary_ready_count = entries
        .iter()
        .filter(|entry| entry.signature_boundary_ready)
        .count();
    let trust_boundary_ready_count = entries
        .iter()
        .filter(|entry| entry.trust_boundary_ready)
        .count();
    let install_cache_boundary_ready_count = entries
        .iter()
        .filter(|entry| entry.install_cache_boundary_ready)
        .count();
    let operator_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_required)
        .count();
    let operator_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_required)
        .count();
    let explicit_non_acceptance_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.explicit_non_acceptance_receipt_projected)
        .count();
    let signature_artifact_present_count = entries
        .iter()
        .filter(|entry| entry.signature_artifact_present)
        .count();
    let signature_verified_count = entries
        .iter()
        .filter(|entry| entry.signature_verified)
        .count();
    let trust_root_present_count = entries
        .iter()
        .filter(|entry| entry.trust_root_present)
        .count();
    let trust_root_accepted_count = entries
        .iter()
        .filter(|entry| entry.trust_root_accepted)
        .count();
    let install_cache_materialized_count = entries
        .iter()
        .filter(|entry| entry.install_cache_materialized)
        .count();
    let install_cache_mutated_count = entries
        .iter()
        .filter(|entry| entry.install_cache_mutated)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.operator_evidence_recorded)
        .count();
    let acceptance_recorded_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_recorded)
        .count();
    let plugin_install_allowed_count = entries
        .iter()
        .filter(|entry| entry.plugin_install_allowed)
        .count();
    let dynamic_activation_allowed_count = entries
        .iter()
        .filter(|entry| entry.dynamic_activation_allowed)
        .count();

    let candidate_count = entries.len();
    let signature_trust_install_cache_boundary_readback_ready = source
        .canonical_manifest_contract_ready
        && source.candidate_count == 2
        && source.signature_boundary_checked_count == 2
        && source.trust_boundary_checked_count == 2
        && source.install_blocked_count == 2
        && !source.plugin_install_allowed
        && !source.plugin_cache_mutation_allowed
        && !source.dynamic_activation_allowed
        && !source.signature_acceptance_allowed
        && !source.trust_root_acceptance_allowed
        && candidate_count == 2
        && signature_boundary_ready_count == 2
        && trust_boundary_ready_count == 2
        && install_cache_boundary_ready_count == 2
        && operator_evidence_required_count == 2
        && operator_acceptance_required_count == 2
        && explicit_non_acceptance_receipt_projected_count == 2
        && signature_artifact_present_count == 0
        && signature_verified_count == 0
        && trust_root_present_count == 0
        && trust_root_accepted_count == 0
        && install_cache_materialized_count == 0
        && install_cache_mutated_count == 0
        && evidence_recorded_count == 0
        && acceptance_recorded_count == 0
        && plugin_install_allowed_count == 0
        && dynamic_activation_allowed_count == 0
        && entries.iter().all(|entry| {
            !entry.signature_acceptance_allowed
                && !entry.trust_root_acceptance_allowed
                && !entry.install_cache_materialization_allowed
                && !entry.operator_evidence_recording_allowed
                && !entry.operator_acceptance_recording_allowed
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

    HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
        status: if signature_trust_install_cache_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_canonical_manifest_contract_ready: source.canonical_manifest_contract_ready,
        source_signature_boundary_checked_count: source.signature_boundary_checked_count,
        source_trust_boundary_checked_count: source.trust_boundary_checked_count,
        source_install_blocked_count: source.install_blocked_count,
        candidate_count,
        signature_boundary_ready_count,
        trust_boundary_ready_count,
        install_cache_boundary_ready_count,
        operator_evidence_required_count,
        operator_acceptance_required_count,
        explicit_non_acceptance_receipt_projected_count,
        signature_artifact_present_count,
        signature_verified_count,
        trust_root_present_count,
        trust_root_accepted_count,
        install_cache_materialized_count,
        install_cache_mutated_count,
        evidence_recorded_count,
        acceptance_recorded_count,
        plugin_install_allowed_count,
        dynamic_activation_allowed_count,
        signature_trust_install_cache_boundary_readback_ready,
        signature_acceptance_allowed: false,
        trust_root_acceptance_allowed: false,
        operator_evidence_recording_allowed: false,
        operator_acceptance_recording_allowed: false,
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
            "signature_artifact_missing",
            "trust_root_missing",
            "operator_evidence_missing",
            "operator_acceptance_missing",
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
            HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_signature_trust_install_cache_boundary_readback_entries(
    source: &HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport,
) -> Vec<HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                boundary_route: match entry.contribution_kind {
                    "mcp_server" => {
                        "plugin-boundary://hepta-system/mcp/signature-trust-install-cache"
                    }
                    "app_connector" => {
                        "plugin-boundary://hepta-system/app/signature-trust-install-cache"
                    }
                    _ => "plugin-boundary://hepta-system/unknown/signature-trust-install-cache",
                },
                signature_boundary_ready: entry.signature_boundary_checked
                    && entry.signature_required_before_install,
                signature_artifact_present: entry.signature_present,
                signature_digest_projected: true,
                signature_verified: false,
                signature_required_before_install: entry.signature_required_before_install,
                signature_acceptance_allowed: false,
                trust_boundary_ready: entry.trust_boundary_checked
                    && entry.trust_required_before_install,
                trust_root_present: entry.trust_root_present,
                trust_root_digest_projected: true,
                trust_root_accepted: entry.trust_root_accepted,
                trust_required_before_install: entry.trust_required_before_install,
                trust_root_acceptance_allowed: false,
                install_cache_boundary_ready: !entry.plugin_install_allowed
                    && !entry.plugin_cache_mutated
                    && !entry.dynamic_activation_allowed,
                install_cache_route_projected: true,
                install_cache_materialized: false,
                install_cache_mutated: entry.plugin_cache_mutated,
                install_cache_materialization_allowed: false,
                operator_evidence_required: true,
                operator_evidence_recorded: false,
                operator_evidence_recording_allowed: false,
                operator_acceptance_required: true,
                operator_acceptance_recorded: false,
                operator_acceptance_recording_allowed: false,
                explicit_non_acceptance_receipt_projected: true,
                plugin_install_allowed: entry.plugin_install_allowed,
                dynamic_activation_allowed: entry.dynamic_activation_allowed,
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

impl HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            manifest_rewritten: false,
            manifest_schema_written: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            install_cache_materialized: false,
            package_lock_written: false,
            remote_sync_started: false,
            loader_invoked: false,
            dynamic_activation_started: false,
            permission_granted: false,
            signature_verified: false,
            signature_accepted: false,
            trust_root_accepted: false,
            operator_evidence_recorded: false,
            operator_acceptance_recorded: false,
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
    fn signature_trust_install_cache_boundary_projects_all_candidate_gates() {
        let report = hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_canonical_manifest_contract_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.signature_boundary_ready_count, 2);
        assert_eq!(report.trust_boundary_ready_count, 2);
        assert_eq!(report.install_cache_boundary_ready_count, 2);
        assert_eq!(report.operator_evidence_required_count, 2);
        assert_eq!(report.operator_acceptance_required_count, 2);
        assert_eq!(report.explicit_non_acceptance_receipt_projected_count, 2);
        assert!(report.signature_trust_install_cache_boundary_readback_ready);
    }

    #[test]
    fn signature_trust_install_cache_boundary_requires_evidence_before_acceptance() {
        let report = hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report();

        assert_eq!(report.signature_artifact_present_count, 0);
        assert_eq!(report.signature_verified_count, 0);
        assert_eq!(report.trust_root_present_count, 0);
        assert_eq!(report.trust_root_accepted_count, 0);
        assert_eq!(report.install_cache_materialized_count, 0);
        assert_eq!(report.install_cache_mutated_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.acceptance_recorded_count, 0);
        assert!(!report.signature_acceptance_allowed);
        assert!(!report.trust_root_acceptance_allowed);
        assert!(!report.operator_evidence_recording_allowed);
        assert!(!report.operator_acceptance_recording_allowed);
    }

    #[test]
    fn signature_trust_install_cache_boundary_keeps_install_cache_and_live_closed() {
        let report = hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report();

        assert_eq!(report.plugin_install_allowed_count, 0);
        assert_eq!(report.dynamic_activation_allowed_count, 0);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
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
            HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackSideEffects::none()
        );
    }
}
