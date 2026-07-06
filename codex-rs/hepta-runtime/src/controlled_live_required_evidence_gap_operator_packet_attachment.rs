use crate::controlled_live_operator_packet_preview::controlled_live_operator_packet_preview_report;
use crate::controlled_live_required_evidence_gap_operator_readback::controlled_live_required_evidence_gap_operator_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_GATE: &str =
    "controlled_live_required_evidence_gap_operator_packet_attachment_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_gap_operator_packet_attachment_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_RECOMMENDED_NEXT_GATE:
    &str = "phase5j_controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_packet_preview_ready: bool,
    pub source_packet_id: &'static str,
    pub source_scope_id: &'static str,
    pub source_payload_hash: &'static str,
    pub source_rollback_owner: &'static str,
    pub source_operator_readback_ready: bool,
    pub source_operator_readback_entry_count: usize,
    pub source_unchanged_missing_count: usize,
    pub attachment_id: &'static str,
    pub attachment_key: &'static str,
    pub attached_packet_id: &'static str,
    pub attached_payload_hash: &'static str,
    pub attachment_entry_count: usize,
    pub operator_readback_attachment_count: usize,
    pub unchanged_missing_attachment_count: usize,
    pub attachment_route_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_packet_attachment_ready: bool,
    pub approval_request_ready: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub attachment_key: &'static str,
    pub attachment_route: &'static str,
    pub operator_readback_key: &'static str,
    pub operator_readback_route: &'static str,
    pub operator_display_order: usize,
    pub operator_status: &'static str,
    pub operator_action: &'static str,
    pub previous_state: &'static str,
    pub current_state: &'static str,
    pub state_delta: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub included_in_packet_attachment: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub comparable: bool,
    pub evidence_recorded: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub persistence_allowed: bool,
    pub attachment_persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
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

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_report()
-> ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentReport {
    let packet = controlled_live_operator_packet_preview_report();
    let readback = controlled_live_required_evidence_gap_operator_readback_report();
    let entries = controlled_live_required_evidence_gap_operator_packet_attachment_entries();
    let operator_readback_attachment_count = entries
        .iter()
        .filter(|entry| entry.included_in_packet_attachment)
        .count();
    let unchanged_missing_attachment_count = entries
        .iter()
        .filter(|entry| {
            entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
        })
        .count();
    let attachment_route_count = entries
        .iter()
        .filter(|entry| !entry.attachment_route.is_empty() && !entry.attachment_key.is_empty())
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let operator_packet_attachment_ready = packet.operator_packet_preview_ready
        && !packet.approval_request_sent
        && !packet.packet_persisted
        && readback.operator_readback_ready
        && readback.operator_readback_entry_count == 7
        && readback.unchanged_missing_count == 7
        && entries.len() == 7
        && operator_readback_attachment_count == 7
        && unchanged_missing_attachment_count == 7
        && attachment_route_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.included_in_packet_attachment
                && entry.operator_visible
                && entry.queryable
                && entry.comparable
                && entry.packet_id == packet.packet_id
                && entry.packet_payload_hash == packet.payload_hash
                && entry.operator_status == "blocked_missing_evidence"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.persistence_allowed
                && !entry.attachment_persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_operator_packet_attachment",
        status: if operator_packet_attachment_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_GATE,
        schema_version:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_operator_packet_preview_ready: packet.operator_packet_preview_ready,
        source_packet_id: packet.packet_id,
        source_scope_id: packet.scope_id,
        source_payload_hash: packet.payload_hash,
        source_rollback_owner: packet.rollback_owner,
        source_operator_readback_ready: readback.operator_readback_ready,
        source_operator_readback_entry_count: readback.operator_readback_entry_count,
        source_unchanged_missing_count: readback.unchanged_missing_count,
        attachment_id: "controlled-live-required-evidence-gap-operator-packet-attachment",
        attachment_key: "controlled_live.required_evidence.gap.operator_packet_attachment",
        attached_packet_id: packet.packet_id,
        attached_payload_hash: packet.payload_hash,
        attachment_entry_count: entries.len(),
        operator_readback_attachment_count,
        unchanged_missing_attachment_count,
        attachment_route_count,
        evidence_recorded_count,
        operator_packet_attachment_ready,
        approval_request_ready: false,
        approval_request_sent: false,
        approval_acceptance_ready: false,
        approval_accepted: false,
        approval_recorded: false,
        packet_persisted: false,
        attachment_persisted: false,
        readback_persisted: false,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_entries()
-> Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentEntry> {
    let packet = controlled_live_operator_packet_preview_report();
    controlled_live_required_evidence_gap_operator_readback_report()
        .entries
        .into_iter()
        .map(
            |entry| ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentEntry {
                id: entry.id,
                source_blocker_id: entry.source_blocker_id,
                packet_id: packet.packet_id,
                packet_payload_hash: packet.payload_hash,
                attachment_key: attachment_key(entry.source_blocker_id),
                attachment_route: attachment_route(entry.source_blocker_id),
                operator_readback_key: entry.operator_readback_key,
                operator_readback_route: entry.operator_readback_route,
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                operator_action: entry.operator_action,
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                included_in_packet_attachment: true,
                operator_visible: true,
                queryable: true,
                comparable: true,
                evidence_recorded: false,
                approval_request_allowed: false,
                approval_acceptance_allowed: false,
                blocker_waiver_allowed: false,
                credential_read_allowed: false,
                evidence_recording_allowed: false,
                persistence_allowed: false,
                attachment_persistence_allowed: false,
                live_mutation_allowed: false,
            },
        )
        .collect()
}

fn attachment_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.gap.operator_packet_attachment.unknown",
    }
}

fn attachment_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "attachment://controlled-live/operator-packet/required-evidence-gap/kill-switch-rehearsal-missing"
        }
        _ => "attachment://controlled-live/operator-packet/required-evidence-gap/unknown",
    }
}

impl ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
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
    fn operator_packet_attachment_is_ready_blocked_without_sending_or_persisting() {
        let report = controlled_live_required_evidence_gap_operator_packet_attachment_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_preview_ready);
        assert_eq!(
            report.source_packet_id,
            "controlled-live-operator-packet-preview"
        );
        assert!(report.source_operator_readback_ready);
        assert_eq!(report.source_operator_readback_entry_count, 7);
        assert_eq!(report.source_unchanged_missing_count, 7);
        assert_eq!(report.attachment_entry_count, 7);
        assert_eq!(report.operator_readback_attachment_count, 7);
        assert_eq!(report.unchanged_missing_attachment_count, 7);
        assert_eq!(report.attachment_route_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.operator_packet_attachment_ready);
        assert!(!report.approval_request_ready);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_accepted);
        assert!(!report.packet_persisted);
        assert!(!report.attachment_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn operator_packet_attachment_keeps_all_readbacks_attached_and_blocked() {
        let report = controlled_live_required_evidence_gap_operator_packet_attachment_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "gateway_native_telegram_post_boundary_approval_missing"
            && entry.packet_id == "controlled-live-operator-packet-preview"
            && entry.operator_display_order == 5
            && entry.attachment_route
                == "attachment://controlled-live/operator-packet/required-evidence-gap/gateway-native-telegram-post-boundary-approval-missing"));
        assert!(
            report
                .entries
                .iter()
                .all(|entry| entry.included_in_packet_attachment
                    && entry.operator_visible
                    && entry.queryable
                    && entry.comparable
                    && entry.previous_state == "missing"
                    && entry.current_state == "missing"
                    && entry.state_delta == "unchanged_missing"
                    && !entry.attachment_key.is_empty()
                    && !entry.attachment_route.is_empty()
                    && !entry.operator_readback_key.is_empty()
                    && !entry.operator_readback_route.is_empty()
                    && !entry.evidence_recorded
                    && !entry.approval_request_allowed
                    && !entry.approval_acceptance_allowed
                    && !entry.blocker_waiver_allowed
                    && !entry.credential_read_allowed
                    && !entry.evidence_recording_allowed
                    && !entry.persistence_allowed
                    && !entry.attachment_persistence_allowed
                    && !entry.live_mutation_allowed)
        );
    }

    #[test]
    fn operator_packet_attachment_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_gap_operator_packet_attachment_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentSideEffects::none()
        );
    }
}
