use serde::Serialize;

use crate::HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport;
use crate::hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report;

pub const HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_GATE: &str =
    "hepta_systems_plugin_operator_evidence_acceptance_packet_readback_gate";
pub const HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_plugin_operator_evidence_acceptance_packet_readback_v1";
pub const HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_install_cache_noop_preflight_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub manifest_name: &'static str,
    pub manifest_version: &'static str,
    pub source_signature_trust_install_cache_boundary_ready: bool,
    pub candidate_count: usize,
    pub packet_entry_count: usize,
    pub packet_projected_count: usize,
    pub checklist_projected_count: usize,
    pub evidence_item_required_count: usize,
    pub evidence_item_recorded_count: usize,
    pub acceptance_check_required_count: usize,
    pub acceptance_check_recorded_count: usize,
    pub signature_artifact_evidence_required_count: usize,
    pub trust_root_evidence_required_count: usize,
    pub install_cache_plan_evidence_required_count: usize,
    pub rollback_uninstall_plan_evidence_required_count: usize,
    pub non_acceptance_receipt_projected_count: usize,
    pub packet_persisted_count: usize,
    pub checklist_persisted_count: usize,
    pub receipt_persisted_count: usize,
    pub plugin_install_allowed_count: usize,
    pub dynamic_activation_allowed_count: usize,
    pub operator_evidence_acceptance_packet_readback_ready: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_persistence_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub acceptance_recording_allowed: bool,
    pub signature_acceptance_allowed: bool,
    pub trust_root_acceptance_allowed: bool,
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
    pub entries: Vec<HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub packet_route: &'static str,
    pub source_signature_boundary_ready: bool,
    pub source_trust_boundary_ready: bool,
    pub source_install_cache_boundary_ready: bool,
    pub operator_packet_projected: bool,
    pub operator_packet_persisted: bool,
    pub checklist_projected: bool,
    pub checklist_persisted: bool,
    pub signature_artifact_evidence_required: bool,
    pub trust_root_evidence_required: bool,
    pub install_cache_plan_evidence_required: bool,
    pub rollback_uninstall_plan_evidence_required: bool,
    pub evidence_item_required_count: usize,
    pub evidence_item_recorded_count: usize,
    pub acceptance_check_required_count: usize,
    pub acceptance_check_recorded_count: usize,
    pub signature_artifact_evidence_recorded: bool,
    pub trust_root_evidence_recorded: bool,
    pub install_cache_plan_evidence_recorded: bool,
    pub rollback_uninstall_plan_evidence_recorded: bool,
    pub signature_acceptance_recorded: bool,
    pub trust_root_acceptance_recorded: bool,
    pub install_cache_acceptance_recorded: bool,
    pub rollback_uninstall_acceptance_recorded: bool,
    pub dynamic_activation_acceptance_recorded: bool,
    pub non_acceptance_receipt_projected: bool,
    pub non_acceptance_receipt_persisted: bool,
    pub plugin_install_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub rollback_uninstall_execution_allowed: bool,
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
pub struct HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackSideEffects {
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub checklist_persisted: bool,
    pub evidence_recorded: bool,
    pub acceptance_recorded: bool,
    pub signature_accepted: bool,
    pub trust_root_accepted: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub install_cache_materialized: bool,
    pub rollback_uninstall_executed: bool,
    pub manifest_rewritten: bool,
    pub manifest_schema_written: bool,
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

pub fn hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report()
-> HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport {
    let source = hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report();
    hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report_from_source(&source)
}

pub fn hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report_from_source(
    source: &HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport,
) -> HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport {
    let entries = hepta_systems_plugin_operator_evidence_acceptance_packet_readback_entries(source);
    let packet_entry_count = entries.len();
    let packet_projected_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_projected)
        .count();
    let checklist_projected_count = entries
        .iter()
        .filter(|entry| entry.checklist_projected)
        .count();
    let evidence_item_required_count = entries
        .iter()
        .map(|entry| entry.evidence_item_required_count)
        .sum();
    let evidence_item_recorded_count = entries
        .iter()
        .map(|entry| entry.evidence_item_recorded_count)
        .sum();
    let acceptance_check_required_count = entries
        .iter()
        .map(|entry| entry.acceptance_check_required_count)
        .sum();
    let acceptance_check_recorded_count = entries
        .iter()
        .map(|entry| entry.acceptance_check_recorded_count)
        .sum();
    let signature_artifact_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.signature_artifact_evidence_required)
        .count();
    let trust_root_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.trust_root_evidence_required)
        .count();
    let install_cache_plan_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.install_cache_plan_evidence_required)
        .count();
    let rollback_uninstall_plan_evidence_required_count = entries
        .iter()
        .filter(|entry| entry.rollback_uninstall_plan_evidence_required)
        .count();
    let non_acceptance_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.non_acceptance_receipt_projected)
        .count();
    let packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_persisted)
        .count();
    let checklist_persisted_count = entries
        .iter()
        .filter(|entry| entry.checklist_persisted)
        .count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted || entry.non_acceptance_receipt_persisted)
        .count();
    let plugin_install_allowed_count = entries
        .iter()
        .filter(|entry| entry.plugin_install_allowed)
        .count();
    let dynamic_activation_allowed_count = entries
        .iter()
        .filter(|entry| entry.dynamic_activation_allowed)
        .count();

    let operator_evidence_acceptance_packet_readback_ready = source
        .signature_trust_install_cache_boundary_readback_ready
        && source.candidate_count == 2
        && source.operator_evidence_required_count == 2
        && source.operator_acceptance_required_count == 2
        && source.evidence_recorded_count == 0
        && source.acceptance_recorded_count == 0
        && !source.plugin_install_allowed
        && !source.plugin_cache_mutation_allowed
        && !source.dynamic_activation_allowed
        && packet_entry_count == 2
        && packet_projected_count == 2
        && checklist_projected_count == 2
        && evidence_item_required_count == 8
        && evidence_item_recorded_count == 0
        && acceptance_check_required_count == 10
        && acceptance_check_recorded_count == 0
        && signature_artifact_evidence_required_count == 2
        && trust_root_evidence_required_count == 2
        && install_cache_plan_evidence_required_count == 2
        && rollback_uninstall_plan_evidence_required_count == 2
        && non_acceptance_receipt_projected_count == 2
        && packet_persisted_count == 0
        && checklist_persisted_count == 0
        && receipt_persisted_count == 0
        && plugin_install_allowed_count == 0
        && dynamic_activation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.source_signature_boundary_ready
                && entry.source_trust_boundary_ready
                && entry.source_install_cache_boundary_ready
                && !entry.operator_packet_persisted
                && !entry.checklist_persisted
                && !entry.signature_artifact_evidence_recorded
                && !entry.trust_root_evidence_recorded
                && !entry.install_cache_plan_evidence_recorded
                && !entry.rollback_uninstall_plan_evidence_recorded
                && !entry.signature_acceptance_recorded
                && !entry.trust_root_acceptance_recorded
                && !entry.install_cache_acceptance_recorded
                && !entry.rollback_uninstall_acceptance_recorded
                && !entry.dynamic_activation_acceptance_recorded
                && !entry.non_acceptance_receipt_persisted
                && !entry.install_cache_materialization_allowed
                && !entry.rollback_uninstall_execution_allowed
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

    HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
        status: if operator_evidence_acceptance_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        manifest_name: source.manifest_name,
        manifest_version: source.manifest_version,
        source_signature_trust_install_cache_boundary_ready: source
            .signature_trust_install_cache_boundary_readback_ready,
        candidate_count: source.candidate_count,
        packet_entry_count,
        packet_projected_count,
        checklist_projected_count,
        evidence_item_required_count,
        evidence_item_recorded_count,
        acceptance_check_required_count,
        acceptance_check_recorded_count,
        signature_artifact_evidence_required_count,
        trust_root_evidence_required_count,
        install_cache_plan_evidence_required_count,
        rollback_uninstall_plan_evidence_required_count,
        non_acceptance_receipt_projected_count,
        packet_persisted_count,
        checklist_persisted_count,
        receipt_persisted_count,
        plugin_install_allowed_count,
        dynamic_activation_allowed_count,
        operator_evidence_acceptance_packet_readback_ready,
        operator_packet_send_allowed: false,
        operator_packet_persistence_allowed: false,
        evidence_recording_allowed: false,
        acceptance_recording_allowed: false,
        signature_acceptance_allowed: false,
        trust_root_acceptance_allowed: false,
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
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "signature_artifact_evidence_missing",
            "trust_root_evidence_missing",
            "install_cache_plan_evidence_missing",
            "rollback_uninstall_plan_evidence_missing",
            "operator_acceptance_missing",
            "evidence_recording_disabled",
            "acceptance_recording_disabled",
            "plugin_install_disabled",
            "plugin_cache_mutation_disabled",
            "install_cache_materialization_disabled",
            "dynamic_activation_disabled",
            "rollback_uninstall_execution_disabled",
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
            HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_plugin_operator_evidence_acceptance_packet_readback_entries(
    source: &HeptaSystemsPluginSignatureTrustInstallCacheBoundaryReadbackReport,
) -> Vec<HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackEntry {
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                packet_route: match entry.contribution_kind {
                    "mcp_server" => "plugin-operator-packet://hepta-system/mcp/evidence-acceptance",
                    "app_connector" => {
                        "plugin-operator-packet://hepta-system/app/evidence-acceptance"
                    }
                    _ => "plugin-operator-packet://hepta-system/unknown/evidence-acceptance",
                },
                source_signature_boundary_ready: entry.signature_boundary_ready,
                source_trust_boundary_ready: entry.trust_boundary_ready,
                source_install_cache_boundary_ready: entry.install_cache_boundary_ready,
                operator_packet_projected: true,
                operator_packet_persisted: false,
                checklist_projected: true,
                checklist_persisted: false,
                signature_artifact_evidence_required: true,
                trust_root_evidence_required: true,
                install_cache_plan_evidence_required: true,
                rollback_uninstall_plan_evidence_required: true,
                evidence_item_required_count: 4,
                evidence_item_recorded_count: 0,
                acceptance_check_required_count: 5,
                acceptance_check_recorded_count: 0,
                signature_artifact_evidence_recorded: false,
                trust_root_evidence_recorded: false,
                install_cache_plan_evidence_recorded: false,
                rollback_uninstall_plan_evidence_recorded: false,
                signature_acceptance_recorded: false,
                trust_root_acceptance_recorded: false,
                install_cache_acceptance_recorded: false,
                rollback_uninstall_acceptance_recorded: false,
                dynamic_activation_acceptance_recorded: false,
                non_acceptance_receipt_projected: true,
                non_acceptance_receipt_persisted: false,
                plugin_install_allowed: entry.plugin_install_allowed,
                install_cache_materialization_allowed: entry.install_cache_materialization_allowed,
                dynamic_activation_allowed: entry.dynamic_activation_allowed,
                rollback_uninstall_execution_allowed: false,
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

impl HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_packet_sent: false,
            operator_packet_persisted: false,
            checklist_persisted: false,
            evidence_recorded: false,
            acceptance_recorded: false,
            signature_accepted: false,
            trust_root_accepted: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            install_cache_materialized: false,
            rollback_uninstall_executed: false,
            manifest_rewritten: false,
            manifest_schema_written: false,
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
    fn operator_packet_projects_evidence_and_acceptance_requirements() {
        let report = hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_signature_trust_install_cache_boundary_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.packet_entry_count, 2);
        assert_eq!(report.packet_projected_count, 2);
        assert_eq!(report.checklist_projected_count, 2);
        assert_eq!(report.evidence_item_required_count, 8);
        assert_eq!(report.acceptance_check_required_count, 10);
        assert_eq!(report.signature_artifact_evidence_required_count, 2);
        assert_eq!(report.trust_root_evidence_required_count, 2);
        assert_eq!(report.install_cache_plan_evidence_required_count, 2);
        assert_eq!(report.rollback_uninstall_plan_evidence_required_count, 2);
        assert!(report.operator_evidence_acceptance_packet_readback_ready);
    }

    #[test]
    fn operator_packet_keeps_evidence_acceptance_and_receipts_unpersisted() {
        let report = hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report();

        assert_eq!(report.evidence_item_recorded_count, 0);
        assert_eq!(report.acceptance_check_recorded_count, 0);
        assert_eq!(report.non_acceptance_receipt_projected_count, 2);
        assert_eq!(report.packet_persisted_count, 0);
        assert_eq!(report.checklist_persisted_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_persistence_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.acceptance_recording_allowed);
        assert!(!report.receipt_persistence_allowed);
    }

    #[test]
    fn operator_packet_keeps_install_cache_activation_and_live_closed() {
        let report = hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report();

        assert_eq!(report.plugin_install_allowed_count, 0);
        assert_eq!(report.dynamic_activation_allowed_count, 0);
        assert!(!report.signature_acceptance_allowed);
        assert!(!report.trust_root_acceptance_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.rollback_uninstall_execution_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.approval_request_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsPluginOperatorEvidenceAcceptancePacketReadbackSideEffects::none()
        );
    }
}
