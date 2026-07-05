use serde::Serialize;

use crate::HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE;
use crate::HeptaSystemStatusInternalReadOnlyInvocationReport;
use crate::hepta_system_status_internal_read_only_invocation_report;

pub const HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_GATE: &str =
    "hepta_system_status_operator_approval_protocol_gate";
pub const HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_SCHEMA_VERSION: &str =
    "hepta_system_status_operator_approval_protocol_v1";
pub const HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_RECOMMENDED_NEXT_GATE: &str =
    "phase10_controlled_canary_readiness_plan_without_gateway_native_telegram_or_live_activation";

pub const HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_SUBJECT: &str =
    "approval-subject:hepta-system/status/internal-read-only";
pub const HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_NONCE: &str =
    "approval-nonce.hepta-system-status.internal-read-only.v1";
pub const HEPTA_SYSTEM_STATUS_OPERATOR_SESSION_BINDING: &str =
    "operator-session-binding.hepta-local.explicit-accept-required.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusOperatorApprovalProtocolReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_invocation_gate: &'static str,
    pub source_invocation_ready: bool,
    pub source_status_payload_materialized: bool,
    pub source_receipt_projected_in_memory: bool,
    pub source_receipt_persisted: bool,
    pub selected_candidate_tool_id: &'static str,
    pub approval_subject: &'static str,
    pub approval_packet_count: usize,
    pub protocol_step_count: usize,
    pub nonce_binding_present: bool,
    pub session_binding_present: bool,
    pub approval_packet_preview_ready: bool,
    pub explicit_accept_required: bool,
    pub non_acceptance_receipt_projected: bool,
    pub approval_protocol_ready: bool,
    pub approval_request_sent: bool,
    pub approval_request_allowed: bool,
    pub approval_accepted: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_recorded: bool,
    pub approval_recording_allowed: bool,
    pub auto_approval_enabled: bool,
    pub evidence_recording_allowed: bool,
    pub approval_broker_write_allowed: bool,
    pub approval_broker_persisted: bool,
    pub receipt_persisted: bool,
    pub credential_read_allowed: bool,
    pub external_network_allowed: bool,
    pub external_tool_invoked: bool,
    pub tool_invocation_switch_enabled: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub live_execution_allowed: bool,
    pub packet: HeptaSystemStatusOperatorApprovalPacket,
    pub steps: Vec<HeptaSystemStatusOperatorApprovalProtocolStep>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemStatusOperatorApprovalProtocolSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusOperatorApprovalPacket {
    pub packet_id: &'static str,
    pub approval_subject: &'static str,
    pub subject_route: &'static str,
    pub source_payload_fingerprint: &'static str,
    pub selected_candidate_tool_id: &'static str,
    pub nonce: &'static str,
    pub nonce_binding_key: &'static str,
    pub operator_session_binding_key: &'static str,
    pub approval_mode: &'static str,
    pub receipt_projection_route: &'static str,
    pub packet_preview_materialized: bool,
    pub explicit_accept_required: bool,
    pub auto_accept_allowed: bool,
    pub approval_request_sent: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_broker_write_allowed: bool,
    pub packet_persisted: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusOperatorApprovalProtocolStep {
    pub step_id: &'static str,
    pub route: HeptaSystemStatusOperatorApprovalProtocolRoute,
    pub ready: bool,
    pub required: bool,
    pub source_payload_bound: bool,
    pub nonce_bound: bool,
    pub operator_session_bound: bool,
    pub explicit_accept_required: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_recording_allowed: bool,
    pub approval_broker_write_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub persistence_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaSystemStatusOperatorApprovalProtocolRoute {
    NonceSessionBindingPreflight,
    ApprovalPacketPreview,
    NonAcceptanceReceiptProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusOperatorApprovalProtocolSideEffects {
    pub filesystem_written: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_broker_written: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub credential_read: bool,
    pub external_network_used: bool,
    pub external_tool_invoked: bool,
    pub tool_registry_switch_enabled: bool,
    pub ledger_written: bool,
    pub receipt_persisted: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub transport_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_system_status_operator_approval_protocol_report()
-> HeptaSystemStatusOperatorApprovalProtocolReport {
    let source = hepta_system_status_internal_read_only_invocation_report();
    hepta_system_status_operator_approval_protocol_report_from_source(&source)
}

pub fn hepta_system_status_operator_approval_protocol_report_from_source(
    source: &HeptaSystemStatusInternalReadOnlyInvocationReport,
) -> HeptaSystemStatusOperatorApprovalProtocolReport {
    let packet = hepta_system_status_operator_approval_packet(source);
    let steps = hepta_system_status_operator_approval_protocol_steps(source);
    let ready_step_count = steps.iter().filter(|step| step.ready).count();
    let approval_protocol_ready = source.internal_read_only_invocation_ready
        && source.status_payload_materialized
        && source.receipt_projected_in_memory
        && !source.receipt_persisted
        && !source.external_network_allowed
        && !source.credential_read_allowed
        && !source.external_tool_invoked
        && !source.tool_invocation_switch_enabled
        && !source.ledger_write_allowed
        && !source.approval_request_allowed
        && !source.approval_acceptance_allowed
        && !source.workflow_event_log_write_allowed
        && !source.sqlite_write_allowed
        && !source.native_post_mutation_allowed
        && !source.channel_send_allowed
        && !source.live_execution_allowed
        && packet.packet_preview_materialized
        && packet.explicit_accept_required
        && !packet.auto_accept_allowed
        && !packet.approval_request_sent
        && !packet.approval_accepted
        && !packet.approval_recorded
        && !packet.approval_broker_write_allowed
        && !packet.packet_persisted
        && !packet.live_execution_allowed
        && steps.len() == 3
        && ready_step_count == 3
        && steps.iter().all(|step| {
            step.required
                && step.source_payload_bound
                && step.nonce_bound
                && step.operator_session_bound
                && step.explicit_accept_required
                && !step.approval_request_allowed
                && !step.approval_acceptance_allowed
                && !step.approval_recording_allowed
                && !step.approval_broker_write_allowed
                && !step.evidence_recording_allowed
                && !step.credential_read_allowed
                && !step.transport_mutation_allowed
                && !step.persistence_allowed
                && !step.live_execution_allowed
        });

    HeptaSystemStatusOperatorApprovalProtocolReport {
        runtime: "hepta",
        surface: "hepta_system_status_operator_approval_protocol",
        status: if approval_protocol_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_GATE,
        schema_version: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_invocation_gate: source.gate,
        source_invocation_ready: source.internal_read_only_invocation_ready,
        source_status_payload_materialized: source.status_payload_materialized,
        source_receipt_projected_in_memory: source.receipt_projected_in_memory,
        source_receipt_persisted: source.receipt_persisted,
        selected_candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
        approval_subject: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_SUBJECT,
        approval_packet_count: 1,
        protocol_step_count: steps.len(),
        nonce_binding_present: true,
        session_binding_present: true,
        approval_packet_preview_ready: packet.packet_preview_materialized,
        explicit_accept_required: packet.explicit_accept_required,
        non_acceptance_receipt_projected: approval_protocol_ready,
        approval_protocol_ready,
        approval_request_sent: false,
        approval_request_allowed: false,
        approval_accepted: false,
        approval_acceptance_allowed: false,
        approval_recorded: false,
        approval_recording_allowed: false,
        auto_approval_enabled: false,
        evidence_recording_allowed: false,
        approval_broker_write_allowed: false,
        approval_broker_persisted: false,
        receipt_persisted: false,
        credential_read_allowed: false,
        external_network_allowed: false,
        external_tool_invoked: false,
        tool_invocation_switch_enabled: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        transport_mutation_allowed: false,
        native_post_mutation_allowed: false,
        channel_send_allowed: false,
        live_execution_allowed: false,
        packet,
        steps,
        blockers: vec![
            "approval_request_not_sent",
            "approval_acceptance_requires_explicit_operator_action",
            "auto_approval_disabled",
            "approval_recording_disabled",
            "approval_broker_write_disabled",
            "evidence_recording_disabled",
            "credential_read_disabled",
            "external_network_disabled",
            "ledger_write_disabled",
            "transport_mutation_disabled",
            "receipt_persistence_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "native_post_mutation_disabled",
            "channel_send_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemStatusOperatorApprovalProtocolSideEffects::none(),
    }
}

pub fn hepta_system_status_operator_approval_packet(
    source: &HeptaSystemStatusInternalReadOnlyInvocationReport,
) -> HeptaSystemStatusOperatorApprovalPacket {
    HeptaSystemStatusOperatorApprovalPacket {
        packet_id: "approval-packet.hepta-system-status.internal-read-only.v1",
        approval_subject: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_SUBJECT,
        subject_route: "approval://hepta-system/status/internal-read-only/v1",
        source_payload_fingerprint: source.status_payload_fingerprint,
        selected_candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
        nonce: HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_NONCE,
        nonce_binding_key: "nonce-binding.hepta-system-status.internal-read-only.v1",
        operator_session_binding_key: HEPTA_SYSTEM_STATUS_OPERATOR_SESSION_BINDING,
        approval_mode: "explicit_operator_accept_required_no_auto_accept",
        receipt_projection_route: "receipt://hepta-system/status/internal-read-only/operator-approval/non-acceptance",
        packet_preview_materialized: source.internal_read_only_invocation_ready,
        explicit_accept_required: true,
        auto_accept_allowed: false,
        approval_request_sent: false,
        approval_accepted: false,
        approval_recorded: false,
        approval_broker_write_allowed: false,
        packet_persisted: false,
        live_execution_allowed: false,
    }
}

pub fn hepta_system_status_operator_approval_protocol_steps(
    source: &HeptaSystemStatusInternalReadOnlyInvocationReport,
) -> Vec<HeptaSystemStatusOperatorApprovalProtocolStep> {
    vec![
        protocol_step(
            "hepta-system.status.operator-approval.nonce-session-binding.v1",
            HeptaSystemStatusOperatorApprovalProtocolRoute::NonceSessionBindingPreflight,
            source.internal_read_only_invocation_ready,
        ),
        protocol_step(
            "hepta-system.status.operator-approval.packet-preview.v1",
            HeptaSystemStatusOperatorApprovalProtocolRoute::ApprovalPacketPreview,
            source.status_payload_materialized,
        ),
        protocol_step(
            "hepta-system.status.operator-approval.non-acceptance-receipt.v1",
            HeptaSystemStatusOperatorApprovalProtocolRoute::NonAcceptanceReceiptProjection,
            source.receipt_projected_in_memory && !source.receipt_persisted,
        ),
    ]
}

fn protocol_step(
    step_id: &'static str,
    route: HeptaSystemStatusOperatorApprovalProtocolRoute,
    source_ready: bool,
) -> HeptaSystemStatusOperatorApprovalProtocolStep {
    HeptaSystemStatusOperatorApprovalProtocolStep {
        step_id,
        route,
        ready: source_ready,
        required: true,
        source_payload_bound: source_ready,
        nonce_bound: true,
        operator_session_bound: true,
        explicit_accept_required: true,
        approval_request_allowed: false,
        approval_acceptance_allowed: false,
        approval_recording_allowed: false,
        approval_broker_write_allowed: false,
        evidence_recording_allowed: false,
        credential_read_allowed: false,
        transport_mutation_allowed: false,
        persistence_allowed: false,
        live_execution_allowed: false,
    }
}

impl HeptaSystemStatusOperatorApprovalProtocolSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            approval_broker_written: false,
            evidence_recorded: false,
            evidence_persisted: false,
            credential_read: false,
            external_network_used: false,
            external_tool_invoked: false,
            tool_registry_switch_enabled: false,
            ledger_written: false,
            receipt_persisted: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            transport_mutated: false,
            native_post_mutation_performed: false,
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
    fn operator_approval_protocol_binds_nonce_session_and_packet() {
        let report = hepta_system_status_operator_approval_protocol_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_invocation_ready);
        assert!(report.source_status_payload_materialized);
        assert_eq!(report.approval_packet_count, 1);
        assert_eq!(report.protocol_step_count, 3);
        assert!(report.nonce_binding_present);
        assert!(report.session_binding_present);
        assert!(report.approval_packet_preview_ready);
        assert_eq!(
            report.packet.nonce,
            HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_NONCE
        );
        assert_eq!(
            report.packet.operator_session_binding_key,
            HEPTA_SYSTEM_STATUS_OPERATOR_SESSION_BINDING
        );
        assert_eq!(
            report.packet.selected_candidate_tool_id,
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE
        );
    }

    #[test]
    fn operator_approval_protocol_requires_explicit_accept_without_auto_acceptance() {
        let report = hepta_system_status_operator_approval_protocol_report();

        assert!(report.explicit_accept_required);
        assert!(report.non_acceptance_receipt_projected);
        assert!(!report.auto_approval_enabled);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_request_allowed);
        assert!(!report.approval_accepted);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.approval_recorded);
        assert!(!report.approval_recording_allowed);
        assert!(report.steps.iter().all(|step| {
            step.explicit_accept_required
                && !step.approval_request_allowed
                && !step.approval_acceptance_allowed
                && !step.approval_recording_allowed
        }));
    }

    #[test]
    fn operator_approval_protocol_keeps_broker_persistence_and_live_closed() {
        let report = hepta_system_status_operator_approval_protocol_report();

        assert!(report.approval_protocol_ready);
        assert!(!report.approval_broker_write_allowed);
        assert!(!report.approval_broker_persisted);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.external_network_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.receipt_persisted);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.transport_mutation_allowed);
        assert!(!report.native_post_mutation_allowed);
        assert!(!report.channel_send_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemStatusOperatorApprovalProtocolSideEffects::none()
        );
    }

    #[test]
    fn operator_approval_protocol_fails_closed_without_source_invocation() {
        let mut source = hepta_system_status_internal_read_only_invocation_report();
        source.internal_read_only_invocation_ready = false;
        source.status_payload_materialized = false;

        let report = hepta_system_status_operator_approval_protocol_report_from_source(&source);

        assert_eq!(report.status, "blocked");
        assert!(!report.approval_protocol_ready);
        assert!(!report.approval_packet_preview_ready);
        assert!(!report.non_acceptance_receipt_projected);
        assert!(report.steps.iter().any(|step| !step.ready));
    }
}
