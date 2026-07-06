use crate::controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback::controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_GATE:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "phase5l_controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_without_credential_read";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_non_send_readback_ready: bool,
    pub source_readback_entry_count: usize,
    pub source_unchanged_missing_readback_count: usize,
    pub transport_boundary_entry_count: usize,
    pub transport_boundary_ready_count: usize,
    pub gateway_auth_boundary_closed_count: usize,
    pub native_post_boundary_closed_count: usize,
    pub telegram_transport_boundary_closed_count: usize,
    pub channel_send_boundary_closed_count: usize,
    pub evidence_recorded_count: usize,
    pub blocker_waived_count: usize,
    pub packet_send_attempted: bool,
    pub attachment_send_attempted: bool,
    pub approval_request_ready: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub gateway_or_auth_mutation_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub telegram_transport_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub transport_boundary_readback_ready: bool,
    pub entries:
        Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub attachment_key: &'static str,
    pub attachment_route: &'static str,
    pub non_send_readback_key: &'static str,
    pub non_send_readback_route: &'static str,
    pub transport_boundary_key: &'static str,
    pub transport_boundary_route: &'static str,
    pub operator_display_order: usize,
    pub operator_status: &'static str,
    pub observed_state: &'static str,
    pub previous_state: &'static str,
    pub current_state: &'static str,
    pub state_delta: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub operator_visible: bool,
    pub attachment_visible: bool,
    pub non_send_confirmed: bool,
    pub gateway_auth_boundary: &'static str,
    pub native_post_boundary: &'static str,
    pub telegram_transport_boundary: &'static str,
    pub channel_send_boundary: &'static str,
    pub gateway_or_auth_mutation_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub telegram_transport_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub transport_mutation_allowed: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub packet_persistence_allowed: bool,
    pub attachment_persistence_allowed: bool,
    pub readback_persistence_allowed: bool,
    pub live_mutation_allowed: bool,
    pub evidence_recorded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackSideEffects
{
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub packet_sent: bool,
    pub attachment_sent: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_report()
-> ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackReport {
    let source =
        controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report();
    let entries =
        controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_entries();
    let transport_boundary_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.attachment_visible
                && entry.non_send_confirmed
                && entry.gateway_auth_boundary == "closed"
                && entry.native_post_boundary == "closed"
                && entry.telegram_transport_boundary == "closed"
                && entry.channel_send_boundary == "closed"
                && !entry.gateway_or_auth_mutation_allowed
                && !entry.native_post_mutation_allowed
                && !entry.telegram_transport_mutation_allowed
                && !entry.channel_send_allowed
                && !entry.transport_mutation_allowed
                && !entry.live_mutation_allowed
        })
        .count();
    let gateway_auth_boundary_closed_count = entries
        .iter()
        .filter(|entry| entry.gateway_auth_boundary == "closed")
        .count();
    let native_post_boundary_closed_count = entries
        .iter()
        .filter(|entry| entry.native_post_boundary == "closed")
        .count();
    let telegram_transport_boundary_closed_count = entries
        .iter()
        .filter(|entry| entry.telegram_transport_boundary == "closed")
        .count();
    let channel_send_boundary_closed_count = entries
        .iter()
        .filter(|entry| entry.channel_send_boundary == "closed")
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let transport_boundary_readback_ready = source.non_send_readback_ready
        && source.readback_entry_count == 7
        && source.unchanged_missing_readback_count == 7
        && !source.attachment_send_attempted
        && !source.packet_send_attempted
        && !source.approval_request_sent
        && !source.approval_accepted
        && !source.packet_persisted
        && !source.attachment_persisted
        && !source.readback_persisted
        && entries.len() == 7
        && transport_boundary_ready_count == 7
        && gateway_auth_boundary_closed_count == 7
        && native_post_boundary_closed_count == 7
        && telegram_transport_boundary_closed_count == 7
        && channel_send_boundary_closed_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "transport_boundary_closed_no_send"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.credential_read_allowed
                && !entry.evidence_recording_allowed
                && !entry.packet_persistence_allowed
                && !entry.attachment_persistence_allowed
                && !entry.readback_persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback",
        status: if transport_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_GATE,
        schema_version:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_non_send_readback_ready: source.non_send_readback_ready,
        source_readback_entry_count: source.readback_entry_count,
        source_unchanged_missing_readback_count: source.unchanged_missing_readback_count,
        transport_boundary_entry_count: entries.len(),
        transport_boundary_ready_count,
        gateway_auth_boundary_closed_count,
        native_post_boundary_closed_count,
        telegram_transport_boundary_closed_count,
        channel_send_boundary_closed_count,
        evidence_recorded_count,
        blocker_waived_count,
        packet_send_attempted: false,
        attachment_send_attempted: false,
        approval_request_ready: false,
        approval_request_sent: false,
        approval_acceptance_ready: false,
        approval_accepted: false,
        approval_recorded: false,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        packet_persisted: false,
        attachment_persisted: false,
        readback_persisted: false,
        gateway_or_auth_mutation_allowed: false,
        native_post_mutation_allowed: false,
        telegram_transport_mutation_allowed: false,
        channel_send_allowed: false,
        transport_mutation_allowed: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        transport_boundary_readback_ready,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_entries()
-> Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackEntry> {
    controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackEntry {
            id: entry.id,
            source_blocker_id: entry.source_blocker_id,
            packet_id: entry.packet_id,
            packet_payload_hash: entry.packet_payload_hash,
            attachment_key: entry.attachment_key,
            attachment_route: entry.attachment_route,
            non_send_readback_key: entry.non_send_readback_key,
            non_send_readback_route: entry.non_send_readback_route,
            transport_boundary_key: transport_boundary_key(entry.source_blocker_id),
            transport_boundary_route: transport_boundary_route(entry.source_blocker_id),
            operator_display_order: entry.operator_display_order,
            operator_status: entry.operator_status,
            observed_state: "transport_boundary_closed_no_send",
            previous_state: entry.previous_state,
            current_state: entry.current_state,
            state_delta: entry.state_delta,
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            operator_visible: true,
            attachment_visible: true,
            non_send_confirmed: true,
            gateway_auth_boundary: "closed",
            native_post_boundary: "closed",
            telegram_transport_boundary: "closed",
            channel_send_boundary: "closed",
            gateway_or_auth_mutation_allowed: false,
            native_post_mutation_allowed: false,
            telegram_transport_mutation_allowed: false,
            channel_send_allowed: false,
            transport_mutation_allowed: false,
            approval_request_allowed: false,
            approval_acceptance_allowed: false,
            blocker_waiver_allowed: false,
            credential_read_allowed: false,
            evidence_recording_allowed: false,
            packet_persistence_allowed: false,
            attachment_persistence_allowed: false,
            readback_persistence_allowed: false,
            live_mutation_allowed: false,
            evidence_recorded: false,
        })
        .collect()
}

fn transport_boundary_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.kill_switch_rehearsal_missing"
        }
        _ => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.transport_boundary.unknown"
        }
    }
}

fn transport_boundary_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/transport-boundary/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/operator-packet/attachment/transport-boundary/unknown",
    }
}

impl ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
            packet_sent: false,
            attachment_sent: false,
            packet_persisted: false,
            attachment_persisted: false,
            readback_persisted: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            replay_executed: false,
            rollback_executed: false,
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
    fn transport_boundary_readback_is_ready_blocked_without_transport_mutation() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_non_send_readback_ready);
        assert_eq!(report.source_readback_entry_count, 7);
        assert_eq!(report.transport_boundary_entry_count, 7);
        assert_eq!(report.transport_boundary_ready_count, 7);
        assert_eq!(report.gateway_auth_boundary_closed_count, 7);
        assert_eq!(report.native_post_boundary_closed_count, 7);
        assert_eq!(report.telegram_transport_boundary_closed_count, 7);
        assert_eq!(report.channel_send_boundary_closed_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.transport_boundary_readback_ready);
        assert!(!report.gateway_or_auth_mutation_allowed);
        assert!(!report.native_post_mutation_allowed);
        assert!(!report.telegram_transport_mutation_allowed);
        assert!(!report.channel_send_allowed);
        assert!(!report.transport_mutation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn transport_boundary_readback_keeps_all_transport_paths_closed() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "gateway_native_telegram_post_boundary_approval_missing"
            && entry.transport_boundary_route
                == "readback://controlled-live/operator-packet/attachment/transport-boundary/gateway-native-telegram-post-boundary-approval-missing"
            && entry.observed_state == "transport_boundary_closed_no_send"));
        assert!(
            report
                .entries
                .iter()
                .all(|entry| entry.gateway_auth_boundary == "closed"
                    && entry.native_post_boundary == "closed"
                    && entry.telegram_transport_boundary == "closed"
                    && entry.channel_send_boundary == "closed"
                    && !entry.transport_boundary_key.is_empty()
                    && !entry.transport_boundary_route.is_empty()
                    && !entry.gateway_or_auth_mutation_allowed
                    && !entry.native_post_mutation_allowed
                    && !entry.telegram_transport_mutation_allowed
                    && !entry.channel_send_allowed
                    && !entry.transport_mutation_allowed
                    && !entry.approval_request_allowed
                    && !entry.approval_acceptance_allowed
                    && !entry.credential_read_allowed
                    && !entry.evidence_recording_allowed
                    && !entry.packet_persistence_allowed
                    && !entry.attachment_persistence_allowed
                    && !entry.readback_persistence_allowed
                    && !entry.live_mutation_allowed)
        );
    }

    #[test]
    fn transport_boundary_readback_keeps_side_effects_closed() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentTransportBoundaryReadbackSideEffects::none()
        );
    }
}
