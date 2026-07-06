use serde::Serialize;

use crate::HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE;
use crate::HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE;
use crate::HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport;
use crate::hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_GATE:
    &str = "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_signature_trust_install_cache_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_tool_registry_minimal_readback_ready: bool,
    pub manifest_identity_ready: bool,
    pub manifest_version_declared: bool,
    pub fixture_version_channel: &'static str,
    pub canonical_manifest_contract_ready: bool,
    pub skill_count: usize,
    pub mcp_server_count: usize,
    pub app_connector_count: usize,
    pub tool_schema_count: usize,
    pub permission_count: usize,
    pub activation_event_count: usize,
    pub tool_policy_count: usize,
    pub candidate_count: usize,
    pub canonical_candidate_count: usize,
    pub schema_complete_count: usize,
    pub permission_boundary_count: usize,
    pub network_none_permission_count: usize,
    pub filesystem_read_only_permission_count: usize,
    pub connector_permission_count: usize,
    pub manual_activation_event_count: usize,
    pub approval_policy_count: usize,
    pub ledger_required_count: usize,
    pub timeout_policy_count: usize,
    pub version_bound_count: usize,
    pub signature_boundary_checked_count: usize,
    pub trust_boundary_checked_count: usize,
    pub install_blocked_count: usize,
    pub activation_blocked_count: usize,
    pub signature_present_count: usize,
    pub trust_root_present_count: usize,
    pub signature_accepted_count: usize,
    pub trust_root_accepted_count: usize,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub permission_grant_allowed: bool,
    pub signature_acceptance_allowed: bool,
    pub trust_root_acceptance_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub canonical_contract_route: &'static str,
    pub manifest_identity_bound: bool,
    pub manifest_version_bound: bool,
    pub tool_schema_declared: bool,
    pub input_schema_declared: bool,
    pub output_schema_declared: bool,
    pub permission_declared: bool,
    pub network_none_permission_declared: bool,
    pub filesystem_read_only_permission_declared: bool,
    pub connector_permission_declared: bool,
    pub activation_event_declared: bool,
    pub manual_activation_declared: bool,
    pub tool_policy_declared: bool,
    pub approval_policy_declared: bool,
    pub approval_kind: &'static str,
    pub ledger_required: bool,
    pub timeout_ms: usize,
    pub signature_boundary_checked: bool,
    pub signature_present: bool,
    pub signature_required_before_install: bool,
    pub trust_boundary_checked: bool,
    pub trust_root_present: bool,
    pub trust_required_before_install: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutated: bool,
    pub dynamic_activation_allowed: bool,
    pub permission_granted: bool,
    pub signature_accepted: bool,
    pub trust_root_accepted: bool,
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
pub struct HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackSideEffects {
    pub filesystem_written: bool,
    pub manifest_rewritten: bool,
    pub manifest_schema_written: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub package_lock_written: bool,
    pub remote_sync_started: bool,
    pub loader_invoked: bool,
    pub dynamic_activation_started: bool,
    pub permission_granted: bool,
    pub signature_accepted: bool,
    pub trust_root_accepted: bool,
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

pub fn hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report()
-> HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport {
    let source =
        hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report();
    hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report_from_source(
        &source,
    )
}

pub fn hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report_from_source(
    source: &HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport,
) -> HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport {
    let entries =
        hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_entries();
    let canonical_candidate_count = entries
        .iter()
        .filter(|entry| entry.manifest_identity_bound && entry.manifest_version_bound)
        .count();
    let schema_complete_count = entries
        .iter()
        .filter(|entry| {
            entry.tool_schema_declared
                && entry.input_schema_declared
                && entry.output_schema_declared
        })
        .count();
    let permission_boundary_count = entries
        .iter()
        .filter(|entry| entry.permission_declared)
        .count();
    let network_none_permission_count = entries
        .iter()
        .filter(|entry| entry.network_none_permission_declared)
        .count();
    let filesystem_read_only_permission_count = entries
        .iter()
        .filter(|entry| entry.filesystem_read_only_permission_declared)
        .count();
    let connector_permission_count = entries
        .iter()
        .filter(|entry| entry.connector_permission_declared)
        .count();
    let manual_activation_event_count = entries
        .iter()
        .filter(|entry| entry.manual_activation_declared)
        .count();
    let approval_policy_count = entries
        .iter()
        .filter(|entry| entry.approval_policy_declared)
        .count();
    let ledger_required_count = entries.iter().filter(|entry| entry.ledger_required).count();
    let timeout_policy_count = entries
        .iter()
        .filter(|entry| entry.timeout_ms == 30_000)
        .count();
    let version_bound_count = entries
        .iter()
        .filter(|entry| entry.manifest_version_bound)
        .count();
    let signature_boundary_checked_count = entries
        .iter()
        .filter(|entry| entry.signature_boundary_checked)
        .count();
    let trust_boundary_checked_count = entries
        .iter()
        .filter(|entry| entry.trust_boundary_checked)
        .count();
    let install_blocked_count = entries
        .iter()
        .filter(|entry| !entry.plugin_install_allowed)
        .count();
    let activation_blocked_count = entries
        .iter()
        .filter(|entry| !entry.dynamic_activation_allowed)
        .count();
    let signature_present_count = entries
        .iter()
        .filter(|entry| entry.signature_present)
        .count();
    let trust_root_present_count = entries
        .iter()
        .filter(|entry| entry.trust_root_present)
        .count();
    let signature_accepted_count = entries
        .iter()
        .filter(|entry| entry.signature_accepted)
        .count();
    let trust_root_accepted_count = entries
        .iter()
        .filter(|entry| entry.trust_root_accepted)
        .count();

    let canonical_manifest_contract_ready = source
        .minimal_read_only_invocation_ledger_receipt_readback_ready
        && source.candidate_count == 2
        && !source.tool_invoked
        && !source.ledger_written
        && !source.approval_requested
        && !source.receipt_persisted
        && entries.len() == 2
        && canonical_candidate_count == 2
        && schema_complete_count == 2
        && permission_boundary_count == 2
        && network_none_permission_count == 2
        && filesystem_read_only_permission_count == 1
        && connector_permission_count == 1
        && manual_activation_event_count == 2
        && approval_policy_count == 2
        && ledger_required_count == 2
        && timeout_policy_count == 2
        && version_bound_count == 2
        && signature_boundary_checked_count == 2
        && trust_boundary_checked_count == 2
        && install_blocked_count == 2
        && activation_blocked_count == 2
        && signature_present_count == 0
        && trust_root_present_count == 0
        && signature_accepted_count == 0
        && trust_root_accepted_count == 0
        && entries.iter().all(|entry| {
            !entry.plugin_cache_mutated
                && !entry.permission_granted
                && !entry.signature_accepted
                && !entry.trust_root_accepted
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

    HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackReport {
        runtime: "hepta",
        surface:
            "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
        status: if canonical_manifest_contract_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: "hepta-system",
        manifest_version: "0.0.0-fixture",
        source_tool_registry_minimal_readback_ready: source
            .minimal_read_only_invocation_ledger_receipt_readback_ready,
        manifest_identity_ready: true,
        manifest_version_declared: true,
        fixture_version_channel: "fixture",
        canonical_manifest_contract_ready,
        skill_count: 1,
        mcp_server_count: 1,
        app_connector_count: 1,
        tool_schema_count: 2,
        permission_count: 2,
        activation_event_count: 2,
        tool_policy_count: 2,
        candidate_count: entries.len(),
        canonical_candidate_count,
        schema_complete_count,
        permission_boundary_count,
        network_none_permission_count,
        filesystem_read_only_permission_count,
        connector_permission_count,
        manual_activation_event_count,
        approval_policy_count,
        ledger_required_count,
        timeout_policy_count,
        version_bound_count,
        signature_boundary_checked_count,
        trust_boundary_checked_count,
        install_blocked_count,
        activation_blocked_count,
        signature_present_count,
        trust_root_present_count,
        signature_accepted_count,
        trust_root_accepted_count,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        dynamic_activation_allowed: false,
        permission_grant_allowed: false,
        signature_acceptance_allowed: false,
        trust_root_acceptance_allowed: false,
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
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "dynamic_activation_disabled",
            "permission_grant_disabled",
            "signature_trust_acceptance_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_entries()
-> Vec<HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackEntry> {
    vec![
        HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackEntry {
            candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
            contribution_kind: "mcp_server",
            canonical_contract_route: "plugin-canonical://hepta-system/mcp/status-read-only",
            manifest_identity_bound: true,
            manifest_version_bound: true,
            tool_schema_declared: true,
            input_schema_declared: true,
            output_schema_declared: true,
            permission_declared: true,
            network_none_permission_declared: true,
            filesystem_read_only_permission_declared: true,
            connector_permission_declared: false,
            activation_event_declared: true,
            manual_activation_declared: true,
            tool_policy_declared: true,
            approval_policy_declared: true,
            approval_kind: "onUse",
            ledger_required: true,
            timeout_ms: 30_000,
            signature_boundary_checked: true,
            signature_present: false,
            signature_required_before_install: true,
            trust_boundary_checked: true,
            trust_root_present: false,
            trust_required_before_install: true,
            plugin_install_allowed: false,
            plugin_cache_mutated: false,
            dynamic_activation_allowed: false,
            permission_granted: false,
            signature_accepted: false,
            trust_root_accepted: false,
            mcp_server_started: false,
            app_connector_started: false,
            tool_registered: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            receipt_persisted: false,
            runtime_event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
        HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackEntry {
            candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE,
            contribution_kind: "app_connector",
            canonical_contract_route: "plugin-canonical://hepta-system/app/status-read-only",
            manifest_identity_bound: true,
            manifest_version_bound: true,
            tool_schema_declared: true,
            input_schema_declared: true,
            output_schema_declared: true,
            permission_declared: true,
            network_none_permission_declared: true,
            filesystem_read_only_permission_declared: false,
            connector_permission_declared: true,
            activation_event_declared: true,
            manual_activation_declared: true,
            tool_policy_declared: true,
            approval_policy_declared: true,
            approval_kind: "install",
            ledger_required: true,
            timeout_ms: 30_000,
            signature_boundary_checked: true,
            signature_present: false,
            signature_required_before_install: true,
            trust_boundary_checked: true,
            trust_root_present: false,
            trust_required_before_install: true,
            plugin_install_allowed: false,
            plugin_cache_mutated: false,
            dynamic_activation_allowed: false,
            permission_granted: false,
            signature_accepted: false,
            trust_root_accepted: false,
            mcp_server_started: false,
            app_connector_started: false,
            tool_registered: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            receipt_persisted: false,
            runtime_event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
    ]
}

impl HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            manifest_rewritten: false,
            manifest_schema_written: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            package_lock_written: false,
            remote_sync_started: false,
            loader_invoked: false,
            dynamic_activation_started: false,
            permission_granted: false,
            signature_accepted: false,
            trust_root_accepted: false,
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
    fn canonical_contract_covers_manifest_contributions_permissions_activation_and_policy() {
        let report =
            hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report(
            );

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_tool_registry_minimal_readback_ready);
        assert!(report.manifest_identity_ready);
        assert!(report.manifest_version_declared);
        assert_eq!(report.fixture_version_channel, "fixture");
        assert_eq!(report.skill_count, 1);
        assert_eq!(report.mcp_server_count, 1);
        assert_eq!(report.app_connector_count, 1);
        assert_eq!(report.tool_schema_count, 2);
        assert_eq!(report.permission_count, 2);
        assert_eq!(report.activation_event_count, 2);
        assert_eq!(report.tool_policy_count, 2);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.canonical_candidate_count, 2);
        assert_eq!(report.schema_complete_count, 2);
        assert_eq!(report.permission_boundary_count, 2);
        assert_eq!(report.manual_activation_event_count, 2);
        assert_eq!(report.approval_policy_count, 2);
        assert_eq!(report.ledger_required_count, 2);
        assert_eq!(report.timeout_policy_count, 2);
        assert!(report.canonical_manifest_contract_ready);
    }

    #[test]
    fn canonical_contract_marks_signature_trust_and_install_boundaries_without_acceptance() {
        let report =
            hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report(
            );

        assert_eq!(report.network_none_permission_count, 2);
        assert_eq!(report.filesystem_read_only_permission_count, 1);
        assert_eq!(report.connector_permission_count, 1);
        assert_eq!(report.version_bound_count, 2);
        assert_eq!(report.signature_boundary_checked_count, 2);
        assert_eq!(report.trust_boundary_checked_count, 2);
        assert_eq!(report.install_blocked_count, 2);
        assert_eq!(report.activation_blocked_count, 2);
        assert_eq!(report.signature_present_count, 0);
        assert_eq!(report.trust_root_present_count, 0);
        assert_eq!(report.signature_accepted_count, 0);
        assert_eq!(report.trust_root_accepted_count, 0);
        assert!(report.entries.iter().all(|entry| {
            entry.signature_boundary_checked
                && entry.signature_required_before_install
                && entry.trust_boundary_checked
                && entry.trust_required_before_install
                && !entry.signature_accepted
                && !entry.trust_root_accepted
                && !entry.plugin_install_allowed
                && !entry.dynamic_activation_allowed
        }));
    }

    #[test]
    fn canonical_contract_keeps_plugin_runtime_and_live_paths_closed() {
        let report =
            hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report(
            );

        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.permission_grant_allowed);
        assert!(!report.signature_acceptance_allowed);
        assert!(!report.trust_root_acceptance_allowed);
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
            HeptaSystemsPluginCanonicalManifestPermissionActivationContractReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            !entry.plugin_cache_mutated
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
        }));
    }
}
