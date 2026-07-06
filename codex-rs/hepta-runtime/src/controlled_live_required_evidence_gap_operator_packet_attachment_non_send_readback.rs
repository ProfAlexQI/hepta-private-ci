use crate::controlled_live_required_evidence_gap_operator_packet_attachment::controlled_live_required_evidence_gap_operator_packet_attachment_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_GATE:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_SCHEMA_VERSION:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "phase5k_controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_packet_attachment_ready: bool,
    pub source_attachment_id: &'static str,
    pub source_attachment_key: &'static str,
    pub source_attached_packet_id: &'static str,
    pub source_attached_payload_hash: &'static str,
    pub source_attachment_entry_count: usize,
    pub source_unchanged_missing_attachment_count: usize,
    pub readback_entry_count: usize,
    pub readback_ready_count: usize,
    pub unchanged_missing_readback_count: usize,
    pub attachment_visible_to_operator: bool,
    pub attachment_send_attempted: bool,
    pub packet_send_attempted: bool,
    pub approval_request_ready: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub evidence_recorded_count: usize,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub non_send_readback_ready: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub attachment_key: &'static str,
    pub attachment_route: &'static str,
    pub non_send_readback_key: &'static str,
    pub non_send_readback_route: &'static str,
    pub operator_readback_key: &'static str,
    pub operator_readback_route: &'static str,
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
    pub packet_send_blocked: bool,
    pub attachment_send_blocked: bool,
    pub approval_request_blocked: bool,
    pub persistence_blocked: bool,
    pub evidence_recorded: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub packet_persistence_allowed: bool,
    pub attachment_persistence_allowed: bool,
    pub readback_persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackSideEffects {
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

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report()
-> ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackReport {
    let attachment = controlled_live_required_evidence_gap_operator_packet_attachment_report();
    let entries =
        controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_entries(
        );
    let readback_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.attachment_visible
                && entry.non_send_confirmed
                && entry.packet_send_blocked
                && entry.attachment_send_blocked
                && entry.approval_request_blocked
                && entry.persistence_blocked
                && !entry.live_mutation_allowed
        })
        .count();
    let unchanged_missing_readback_count = entries
        .iter()
        .filter(|entry| {
            entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
        })
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let non_send_readback_ready = attachment.operator_packet_attachment_ready
        && attachment.attachment_entry_count == 7
        && attachment.unchanged_missing_attachment_count == 7
        && !attachment.approval_request_sent
        && !attachment.approval_accepted
        && !attachment.packet_persisted
        && !attachment.attachment_persisted
        && !attachment.readback_persisted
        && entries.len() == 7
        && readback_ready_count == 7
        && unchanged_missing_readback_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.packet_id == attachment.attached_packet_id
                && entry.packet_payload_hash == attachment.attached_payload_hash
                && entry.operator_status == "blocked_missing_evidence"
                && entry.observed_state == "attachment_visible_unsent_unpersisted"
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.packet_persistence_allowed
                && !entry.attachment_persistence_allowed
                && !entry.readback_persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback",
        status: if non_send_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_GATE,
        schema_version:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_operator_packet_attachment_ready: attachment.operator_packet_attachment_ready,
        source_attachment_id: attachment.attachment_id,
        source_attachment_key: attachment.attachment_key,
        source_attached_packet_id: attachment.attached_packet_id,
        source_attached_payload_hash: attachment.attached_payload_hash,
        source_attachment_entry_count: attachment.attachment_entry_count,
        source_unchanged_missing_attachment_count: attachment.unchanged_missing_attachment_count,
        readback_entry_count: entries.len(),
        readback_ready_count,
        unchanged_missing_readback_count,
        attachment_visible_to_operator: non_send_readback_ready,
        attachment_send_attempted: false,
        packet_send_attempted: false,
        approval_request_ready: false,
        approval_request_sent: false,
        approval_acceptance_ready: false,
        approval_accepted: false,
        approval_recorded: false,
        packet_persisted: false,
        attachment_persisted: false,
        readback_persisted: false,
        evidence_recorded_count,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        non_send_readback_ready,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_entries()
-> Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackEntry> {
    controlled_live_required_evidence_gap_operator_packet_attachment_report()
        .entries
        .into_iter()
        .map(|entry| {
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackEntry {
                id: entry.id,
                source_blocker_id: entry.source_blocker_id,
                packet_id: entry.packet_id,
                packet_payload_hash: entry.packet_payload_hash,
                attachment_key: entry.attachment_key,
                attachment_route: entry.attachment_route,
                non_send_readback_key: non_send_readback_key(entry.source_blocker_id),
                non_send_readback_route: non_send_readback_route(entry.source_blocker_id),
                operator_readback_key: entry.operator_readback_key,
                operator_readback_route: entry.operator_readback_route,
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "attachment_visible_unsent_unpersisted",
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
                packet_send_blocked: true,
                attachment_send_blocked: true,
                approval_request_blocked: true,
                persistence_blocked: true,
                evidence_recorded: false,
                approval_request_allowed: false,
                approval_acceptance_allowed: false,
                blocker_waiver_allowed: false,
                credential_read_allowed: false,
                evidence_recording_allowed: false,
                packet_persistence_allowed: false,
                attachment_persistence_allowed: false,
                readback_persistence_allowed: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

fn non_send_readback_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.gap.operator_packet_attachment.non_send.unknown",
    }
}

fn non_send_readback_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/operator-packet/attachment/non-send/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/non-send/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/operator-packet/attachment/non-send/unknown",
    }
}

impl ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackSideEffects {
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
    fn attachment_non_send_readback_is_ready_blocked_without_send_or_persistence() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_attachment_ready);
        assert_eq!(
            report.source_attachment_id,
            "controlled-live-required-evidence-gap-operator-packet-attachment"
        );
        assert_eq!(report.source_attachment_entry_count, 7);
        assert_eq!(report.source_unchanged_missing_attachment_count, 7);
        assert_eq!(report.readback_entry_count, 7);
        assert_eq!(report.readback_ready_count, 7);
        assert_eq!(report.unchanged_missing_readback_count, 7);
        assert!(report.attachment_visible_to_operator);
        assert!(report.non_send_readback_ready);
        assert!(!report.attachment_send_attempted);
        assert!(!report.packet_send_attempted);
        assert!(!report.approval_request_ready);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_accepted);
        assert!(!report.packet_persisted);
        assert!(!report.attachment_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn attachment_non_send_readback_keeps_all_gap_entries_unsent_and_blocked() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "gateway_native_telegram_post_boundary_approval_missing"
            && entry.non_send_readback_route
                == "readback://controlled-live/operator-packet/attachment/non-send/gateway-native-telegram-post-boundary-approval-missing"
            && entry.observed_state == "attachment_visible_unsent_unpersisted"));
        assert!(report.entries.iter().all(|entry| entry.operator_visible
            && entry.attachment_visible
            && entry.non_send_confirmed
            && entry.packet_send_blocked
            && entry.attachment_send_blocked
            && entry.approval_request_blocked
            && entry.persistence_blocked
            && entry.previous_state == "missing"
            && entry.current_state == "missing"
            && entry.state_delta == "unchanged_missing"
            && !entry.non_send_readback_key.is_empty()
            && !entry.non_send_readback_route.is_empty()
            && !entry.evidence_recorded
            && !entry.approval_request_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.credential_read_allowed
            && !entry.evidence_recording_allowed
            && !entry.packet_persistence_allowed
            && !entry.attachment_persistence_allowed
            && !entry.readback_persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn attachment_non_send_readback_keeps_side_effects_closed() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentNonSendReadbackSideEffects::none()
        );
    }
}
