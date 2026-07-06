use serde::Serialize;

use crate::HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE;
use crate::HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE;
use crate::HeptaSystemStatusInternalReadOnlyInvocationReport;
use crate::HeptaSystemStatusOperatorApprovalProtocolReport;
use crate::HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackReport;
use crate::hepta_system_status_internal_read_only_invocation_report;
use crate::hepta_system_status_operator_approval_protocol_report;
use crate::hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report;

pub const HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_GATE:
    &str = "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_gate";
pub const HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_SCHEMA_VERSION:
    &str = "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_v1";
pub const HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_workgraph_inventory_ready: bool,
    pub source_internal_invocation_ready: bool,
    pub source_operator_approval_protocol_ready: bool,
    pub candidate_count: usize,
    pub selected_candidate_tool_id: &'static str,
    pub non_selected_candidate_tool_id: &'static str,
    pub selected_minimal_path_count: usize,
    pub selected_minimal_stage_count: usize,
    pub non_selected_preflight_only_count: usize,
    pub registry_lookup_preview_required_count: usize,
    pub status_payload_projection_count: usize,
    pub ledger_preview_required_count: usize,
    pub approval_preflight_required_count: usize,
    pub approval_packet_preview_count: usize,
    pub receipt_projection_required_count: usize,
    pub result_receipt_projected_in_memory_count: usize,
    pub operator_protocol_step_count: usize,
    pub explicit_accept_required: bool,
    pub non_acceptance_receipt_projected: bool,
    pub output_schema_validated: bool,
    pub minimal_read_only_invocation_ledger_receipt_readback_ready: bool,
    pub tool_invoked: bool,
    pub tool_invocation_switch_enabled: bool,
    pub registry_lookup_executed: bool,
    pub tool_registry_mutated: bool,
    pub ledger_written: bool,
    pub ledger_write_allowed: bool,
    pub approval_requested: bool,
    pub approval_request_allowed: bool,
    pub approval_accepted: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_recorded: bool,
    pub receipt_persisted: bool,
    pub result_receipt_written: bool,
    pub external_network_allowed: bool,
    pub credential_read_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry {
    pub entry_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub contract_stage: HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage,
    pub route: &'static str,
    pub selected_for_minimal_path: bool,
    pub preflight_only: bool,
    pub source_bound: bool,
    pub input_schema_validated: bool,
    pub output_schema_validated: bool,
    pub status_payload_materialized: bool,
    pub registry_lookup_preview_required: bool,
    pub ledger_preview_required: bool,
    pub approval_preflight_required: bool,
    pub approval_packet_preview_ready: bool,
    pub receipt_projection_required: bool,
    pub result_receipt_projected_in_memory: bool,
    pub non_acceptance_receipt_projected: bool,
    pub tool_invoked: bool,
    pub registry_lookup_executed: bool,
    pub tool_registry_mutated: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub receipt_persisted: bool,
    pub result_receipt_written: bool,
    pub external_network_used: bool,
    pub credential_read: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage {
    RegistryLookupPreview,
    InternalStatusPayloadProjection,
    LedgerApprovalPreflight,
    ResultReceiptProjection,
    NonSelectedPreflightOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackSideEffects {
    pub filesystem_written: bool,
    pub tool_registry_mutated: bool,
    pub registry_lookup_executed: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_broker_written: bool,
    pub receipt_persisted: bool,
    pub result_receipt_written: bool,
    pub credential_read: bool,
    pub external_network_used: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report()
-> HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport {
    let workgraph_inventory =
        hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report();
    let internal_invocation = hepta_system_status_internal_read_only_invocation_report();
    let operator_protocol = hepta_system_status_operator_approval_protocol_report();

    hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report_from_sources(
        &workgraph_inventory,
        &internal_invocation,
        &operator_protocol,
    )
}

pub fn hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report_from_sources(
    workgraph_inventory: &HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackReport,
    internal_invocation: &HeptaSystemStatusInternalReadOnlyInvocationReport,
    operator_protocol: &HeptaSystemStatusOperatorApprovalProtocolReport,
) -> HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport {
    let entries =
        hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_entries();
    let selected_minimal_stage_count = entries
        .iter()
        .filter(|entry| entry.selected_for_minimal_path)
        .count();
    let non_selected_preflight_only_count =
        entries.iter().filter(|entry| entry.preflight_only).count();
    let registry_lookup_preview_required_count = entries
        .iter()
        .filter(|entry| entry.registry_lookup_preview_required)
        .count();
    let status_payload_projection_count = entries
        .iter()
        .filter(|entry| entry.status_payload_materialized)
        .count();
    let ledger_preview_required_count = entries
        .iter()
        .filter(|entry| entry.ledger_preview_required)
        .count();
    let approval_preflight_required_count = entries
        .iter()
        .filter(|entry| entry.approval_preflight_required)
        .count();
    let approval_packet_preview_count = entries
        .iter()
        .filter(|entry| entry.approval_packet_preview_ready)
        .count();
    let receipt_projection_required_count = entries
        .iter()
        .filter(|entry| entry.receipt_projection_required)
        .count();
    let result_receipt_projected_in_memory_count = entries
        .iter()
        .filter(|entry| entry.result_receipt_projected_in_memory)
        .count();

    let minimal_read_only_invocation_ledger_receipt_readback_ready = workgraph_inventory
        .workgraph_legacy_gate_recursion_inventory_ready
        && internal_invocation.internal_read_only_invocation_ready
        && internal_invocation.candidate_count == 2
        && internal_invocation.invocation_entry_count == 1
        && internal_invocation.selected_candidate_tool_id
            == HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE
        && internal_invocation.non_selected_candidate_tool_id
            == HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE
        && internal_invocation.status_payload_materialized
        && internal_invocation.output_schema_validated
        && internal_invocation.receipt_projected_in_memory
        && !internal_invocation.receipt_persisted
        && operator_protocol.approval_protocol_ready
        && operator_protocol.approval_packet_count == 1
        && operator_protocol.protocol_step_count == 3
        && operator_protocol.selected_candidate_tool_id
            == HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE
        && operator_protocol.explicit_accept_required
        && operator_protocol.non_acceptance_receipt_projected
        && !operator_protocol.approval_request_sent
        && !operator_protocol.approval_request_allowed
        && !operator_protocol.approval_accepted
        && !operator_protocol.approval_acceptance_allowed
        && !operator_protocol.approval_recorded
        && !operator_protocol.approval_broker_write_allowed
        && !operator_protocol.receipt_persisted
        && entries.len() == 5
        && selected_minimal_stage_count == 4
        && non_selected_preflight_only_count == 1
        && registry_lookup_preview_required_count == 1
        && status_payload_projection_count == 1
        && ledger_preview_required_count == 1
        && approval_preflight_required_count == 1
        && approval_packet_preview_count == 1
        && receipt_projection_required_count == 1
        && result_receipt_projected_in_memory_count == 1
        && entries.iter().all(|entry| {
            entry.source_bound
                && entry.input_schema_validated
                && entry.output_schema_validated
                && !entry.tool_invoked
                && !entry.registry_lookup_executed
                && !entry.tool_registry_mutated
                && !entry.ledger_written
                && !entry.approval_requested
                && !entry.approval_accepted
                && !entry.approval_recorded
                && !entry.receipt_persisted
                && !entry.result_receipt_written
                && !entry.external_network_used
                && !entry.credential_read
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.native_post_mutation_performed
                && !entry.channel_send_performed
                && !entry.live_execution_started
        });

    HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback",
        status: if minimal_read_only_invocation_ledger_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_SCHEMA_VERSION,
        plugin_id: internal_invocation.plugin_id,
        source_workgraph_inventory_ready: workgraph_inventory
            .workgraph_legacy_gate_recursion_inventory_ready,
        source_internal_invocation_ready: internal_invocation.internal_read_only_invocation_ready,
        source_operator_approval_protocol_ready: operator_protocol.approval_protocol_ready,
        candidate_count: internal_invocation.candidate_count,
        selected_candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
        non_selected_candidate_tool_id:
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE,
        selected_minimal_path_count: 1,
        selected_minimal_stage_count,
        non_selected_preflight_only_count,
        registry_lookup_preview_required_count,
        status_payload_projection_count,
        ledger_preview_required_count,
        approval_preflight_required_count,
        approval_packet_preview_count,
        receipt_projection_required_count,
        result_receipt_projected_in_memory_count,
        operator_protocol_step_count: operator_protocol.protocol_step_count,
        explicit_accept_required: operator_protocol.explicit_accept_required,
        non_acceptance_receipt_projected: operator_protocol.non_acceptance_receipt_projected,
        output_schema_validated: internal_invocation.output_schema_validated,
        minimal_read_only_invocation_ledger_receipt_readback_ready,
        tool_invoked: false,
        tool_invocation_switch_enabled: false,
        registry_lookup_executed: false,
        tool_registry_mutated: false,
        ledger_written: false,
        ledger_write_allowed: false,
        approval_requested: false,
        approval_request_allowed: false,
        approval_accepted: false,
        approval_acceptance_allowed: false,
        approval_recorded: false,
        receipt_persisted: false,
        result_receipt_written: false,
        external_network_allowed: false,
        credential_read_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        native_post_mutation_allowed: false,
        channel_send_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "tool_invocation_switch_disabled",
            "registry_lookup_execution_disabled",
            "tool_registry_mutation_disabled",
            "ledger_write_disabled",
            "approval_request_disabled",
            "approval_acceptance_disabled",
            "approval_recording_disabled",
            "receipt_persistence_disabled",
            "external_network_disabled",
            "credential_read_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "native_post_mutation_disabled",
            "channel_send_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_entries()
-> Vec<HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry> {
    vec![
        selected_stage(
            "selected_registry_lookup_preview",
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage::RegistryLookupPreview,
            "tool-registry://hepta-system/status/read-only/lookup-preview",
            true,
            false,
            false,
            false,
            false,
        ),
        selected_stage(
            "selected_internal_status_payload_projection",
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage::InternalStatusPayloadProjection,
            "internal://hepta-system/status/read-only",
            false,
            true,
            false,
            false,
            false,
        ),
        selected_stage(
            "selected_ledger_approval_preflight",
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage::LedgerApprovalPreflight,
            "approval-ledger://hepta-system/status/read-only/preflight",
            false,
            false,
            true,
            true,
            false,
        ),
        selected_stage(
            "selected_result_receipt_projection",
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage::ResultReceiptProjection,
            "receipt://hepta-system/status/read-only/result-projection",
            false,
            false,
            false,
            false,
            true,
        ),
        non_selected_preflight_stage(),
    ]
}

fn selected_stage(
    entry_id: &'static str,
    contract_stage: HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage,
    route: &'static str,
    registry_lookup_preview_required: bool,
    status_payload_materialized: bool,
    ledger_preview_required: bool,
    approval_preflight_required: bool,
    receipt_projection_required: bool,
) -> HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry {
    let approval_packet_preview_ready = approval_preflight_required;
    let result_receipt_projected_in_memory = receipt_projection_required;

    HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry {
        entry_id,
        candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
        contribution_kind: "mcp_server",
        contract_stage,
        route,
        selected_for_minimal_path: true,
        preflight_only: false,
        source_bound: true,
        input_schema_validated: true,
        output_schema_validated: true,
        status_payload_materialized,
        registry_lookup_preview_required,
        ledger_preview_required,
        approval_preflight_required,
        approval_packet_preview_ready,
        receipt_projection_required,
        result_receipt_projected_in_memory,
        non_acceptance_receipt_projected: result_receipt_projected_in_memory,
        tool_invoked: false,
        registry_lookup_executed: false,
        tool_registry_mutated: false,
        ledger_written: false,
        approval_requested: false,
        approval_accepted: false,
        approval_recorded: false,
        receipt_persisted: false,
        result_receipt_written: false,
        external_network_used: false,
        credential_read: false,
        workflow_event_log_written: false,
        sqlite_written: false,
        native_post_mutation_performed: false,
        channel_send_performed: false,
        live_execution_started: false,
    }
}

fn non_selected_preflight_stage()
-> HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry {
    HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackEntry {
        entry_id: "non_selected_app_connector_preflight_only",
        candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE,
        contribution_kind: "app_connector",
        contract_stage:
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackStage::NonSelectedPreflightOnly,
        route: "tool-registry://hepta-system/status/read-only/non-selected-app-preflight",
        selected_for_minimal_path: false,
        preflight_only: true,
        source_bound: true,
        input_schema_validated: true,
        output_schema_validated: true,
        status_payload_materialized: false,
        registry_lookup_preview_required: false,
        ledger_preview_required: false,
        approval_preflight_required: false,
        approval_packet_preview_ready: false,
        receipt_projection_required: false,
        result_receipt_projected_in_memory: false,
        non_acceptance_receipt_projected: false,
        tool_invoked: false,
        registry_lookup_executed: false,
        tool_registry_mutated: false,
        ledger_written: false,
        approval_requested: false,
        approval_accepted: false,
        approval_recorded: false,
        receipt_persisted: false,
        result_receipt_written: false,
        external_network_used: false,
        credential_read: false,
        workflow_event_log_written: false,
        sqlite_written: false,
        native_post_mutation_performed: false,
        channel_send_performed: false,
        live_execution_started: false,
    }
}

impl HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            tool_registry_mutated: false,
            registry_lookup_executed: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            approval_broker_written: false,
            receipt_persisted: false,
            result_receipt_written: false,
            credential_read: false,
            external_network_used: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
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
    fn minimal_read_only_invocation_binds_selected_tool_to_ledger_approval_and_receipt() {
        let report =
            hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report(
            );

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_workgraph_inventory_ready);
        assert!(report.source_internal_invocation_ready);
        assert!(report.source_operator_approval_protocol_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.selected_minimal_path_count, 1);
        assert_eq!(report.selected_minimal_stage_count, 4);
        assert_eq!(report.non_selected_preflight_only_count, 1);
        assert_eq!(report.registry_lookup_preview_required_count, 1);
        assert_eq!(report.status_payload_projection_count, 1);
        assert_eq!(report.ledger_preview_required_count, 1);
        assert_eq!(report.approval_preflight_required_count, 1);
        assert_eq!(report.approval_packet_preview_count, 1);
        assert_eq!(report.receipt_projection_required_count, 1);
        assert_eq!(report.result_receipt_projected_in_memory_count, 1);
        assert_eq!(report.operator_protocol_step_count, 3);
        assert!(report.explicit_accept_required);
        assert!(report.non_acceptance_receipt_projected);
        assert!(report.minimal_read_only_invocation_ledger_receipt_readback_ready);
    }

    #[test]
    fn minimal_read_only_invocation_keeps_non_selected_candidate_preflight_only() {
        let report =
            hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report(
            );

        let non_selected = report
            .entries
            .iter()
            .find(|entry| entry.entry_id == "non_selected_app_connector_preflight_only")
            .expect("non-selected app connector preflight entry");

        assert_eq!(
            non_selected.candidate_tool_id,
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE
        );
        assert!(non_selected.preflight_only);
        assert!(!non_selected.selected_for_minimal_path);
        assert!(non_selected.source_bound);
        assert!(!non_selected.tool_invoked);
        assert!(!non_selected.ledger_written);
        assert!(!non_selected.approval_requested);
        assert!(!non_selected.receipt_persisted);
    }

    #[test]
    fn minimal_read_only_invocation_keeps_all_mutation_and_live_paths_closed() {
        let report =
            hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report(
            );

        assert!(!report.tool_invoked);
        assert!(!report.tool_invocation_switch_enabled);
        assert!(!report.registry_lookup_executed);
        assert!(!report.tool_registry_mutated);
        assert!(!report.ledger_written);
        assert!(!report.ledger_write_allowed);
        assert!(!report.approval_requested);
        assert!(!report.approval_request_allowed);
        assert!(!report.approval_accepted);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.approval_recorded);
        assert!(!report.receipt_persisted);
        assert!(!report.result_receipt_written);
        assert!(!report.external_network_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.native_post_mutation_allowed);
        assert!(!report.channel_send_allowed);
        assert!(!report.live_execution_allowed);
        assert!(report.entries.iter().all(|entry| {
            !entry.tool_invoked
                && !entry.registry_lookup_executed
                && !entry.tool_registry_mutated
                && !entry.ledger_written
                && !entry.approval_requested
                && !entry.approval_accepted
                && !entry.approval_recorded
                && !entry.receipt_persisted
                && !entry.result_receipt_written
                && !entry.external_network_used
                && !entry.credential_read
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.native_post_mutation_performed
                && !entry.channel_send_performed
                && !entry.live_execution_started
        }));
        assert_eq!(
            report.side_effects,
            HeptaSystemsToolRegistryMinimalReadOnlyInvocationLedgerReceiptReadbackSideEffects::none(
            )
        );
    }
}
