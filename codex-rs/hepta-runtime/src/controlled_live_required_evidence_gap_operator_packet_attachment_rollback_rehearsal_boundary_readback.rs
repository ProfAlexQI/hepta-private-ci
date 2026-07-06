use crate::controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback::controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_GATE:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "phase5n_controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_without_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_credential_boundary_readback_ready: bool,
    pub source_credential_boundary_entry_count: usize,
    pub source_credential_boundary_ready_count: usize,
    pub rollback_rehearsal_boundary_entry_count: usize,
    pub rollback_rehearsal_boundary_ready_count: usize,
    pub rollback_rehearsal_boundary_closed_count: usize,
    pub rollback_rehearsal_execution_blocked_count: usize,
    pub rollback_execution_blocked_count: usize,
    pub rollback_rehearsal_recording_blocked_count: usize,
    pub rollback_rehearsal_receipt_persistence_blocked_count: usize,
    pub rollback_rehearsal_evidence_missing_count: usize,
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
    pub rollback_rehearsal_allowed: bool,
    pub rollback_rehearsal_executed: bool,
    pub rollback_execution_allowed: bool,
    pub rollback_executed: bool,
    pub rollback_rehearsal_recording_allowed: bool,
    pub rollback_rehearsal_receipt_persistence_allowed: bool,
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
    pub rollback_rehearsal_boundary_readback_ready: bool,
    pub entries:
        Vec<ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackEntry
{
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub attachment_key: &'static str,
    pub attachment_route: &'static str,
    pub credential_boundary_key: &'static str,
    pub credential_boundary_route: &'static str,
    pub rollback_rehearsal_boundary_key: &'static str,
    pub rollback_rehearsal_boundary_route: &'static str,
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
    pub credential_boundary_confirmed: bool,
    pub rollback_rehearsal_boundary_readback_visible: bool,
    pub rollback_rehearsal_boundary_status: &'static str,
    pub rollback_rehearsal_evidence_state: &'static str,
    pub rollback_rehearsal_allowed: bool,
    pub rollback_rehearsal_executed: bool,
    pub rollback_execution_allowed: bool,
    pub rollback_executed: bool,
    pub rollback_rehearsal_recording_allowed: bool,
    pub rollback_rehearsal_recorded: bool,
    pub rollback_rehearsal_receipt_persistence_allowed: bool,
    pub rollback_rehearsal_receipt_persisted: bool,
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
pub struct ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackSideEffects
{
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub rollback_rehearsal_executed: bool,
    pub rollback_executed: bool,
    pub rollback_rehearsal_recorded: bool,
    pub rollback_rehearsal_receipt_persisted: bool,
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
    pub rollback_executed_side_effect: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_report()
-> ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackReport
{
    let source =
        controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_report();
    let entries =
        controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_entries();
    let rollback_rehearsal_boundary_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.attachment_visible
                && entry.credential_boundary_confirmed
                && entry.rollback_rehearsal_boundary_readback_visible
                && entry.rollback_rehearsal_boundary_status == "closed_no_execution"
                && entry.rollback_rehearsal_evidence_state == "missing"
                && !entry.rollback_rehearsal_allowed
                && !entry.rollback_rehearsal_executed
                && !entry.rollback_execution_allowed
                && !entry.rollback_executed
                && !entry.rollback_rehearsal_recording_allowed
                && !entry.rollback_rehearsal_recorded
                && !entry.rollback_rehearsal_receipt_persistence_allowed
                && !entry.rollback_rehearsal_receipt_persisted
                && !entry.live_mutation_allowed
        })
        .count();
    let rollback_rehearsal_boundary_closed_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_boundary_status == "closed_no_execution")
        .count();
    let rollback_rehearsal_execution_blocked_count = entries
        .iter()
        .filter(|entry| !entry.rollback_rehearsal_allowed && !entry.rollback_rehearsal_executed)
        .count();
    let rollback_execution_blocked_count = entries
        .iter()
        .filter(|entry| !entry.rollback_execution_allowed && !entry.rollback_executed)
        .count();
    let rollback_rehearsal_recording_blocked_count = entries
        .iter()
        .filter(|entry| {
            !entry.rollback_rehearsal_recording_allowed && !entry.rollback_rehearsal_recorded
        })
        .count();
    let rollback_rehearsal_receipt_persistence_blocked_count = entries
        .iter()
        .filter(|entry| {
            !entry.rollback_rehearsal_receipt_persistence_allowed
                && !entry.rollback_rehearsal_receipt_persisted
        })
        .count();
    let rollback_rehearsal_evidence_missing_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_evidence_state == "missing")
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let rollback_rehearsal_boundary_readback_ready = source.credential_boundary_readback_ready
        && source.credential_boundary_entry_count == 7
        && source.credential_boundary_ready_count == 7
        && source.credential_read_blocked_count == 7
        && source.credential_material_load_blocked_count == 7
        && source.credential_value_exposure_blocked_count == 7
        && source.credential_handle_resolution_blocked_count == 7
        && !source.packet_send_attempted
        && !source.attachment_send_attempted
        && !source.approval_request_sent
        && !source.approval_accepted
        && !source.credential_read_allowed
        && !source.packet_persisted
        && !source.attachment_persisted
        && !source.readback_persisted
        && entries.len() == 7
        && rollback_rehearsal_boundary_ready_count == 7
        && rollback_rehearsal_boundary_closed_count == 7
        && rollback_rehearsal_execution_blocked_count == 7
        && rollback_execution_blocked_count == 7
        && rollback_rehearsal_recording_blocked_count == 7
        && rollback_rehearsal_receipt_persistence_blocked_count == 7
        && rollback_rehearsal_evidence_missing_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "rollback_rehearsal_boundary_closed_no_execution"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.credential_read_allowed
                && !entry.rollback_rehearsal_allowed
                && !entry.rollback_execution_allowed
                && !entry.rollback_rehearsal_recording_allowed
                && !entry.rollback_rehearsal_receipt_persistence_allowed
                && !entry.evidence_recording_allowed
                && !entry.packet_persistence_allowed
                && !entry.attachment_persistence_allowed
                && !entry.readback_persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback",
        status: if rollback_rehearsal_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_GATE,
        schema_version:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_credential_boundary_readback_ready: source.credential_boundary_readback_ready,
        source_credential_boundary_entry_count: source.credential_boundary_entry_count,
        source_credential_boundary_ready_count: source.credential_boundary_ready_count,
        rollback_rehearsal_boundary_entry_count: entries.len(),
        rollback_rehearsal_boundary_ready_count,
        rollback_rehearsal_boundary_closed_count,
        rollback_rehearsal_execution_blocked_count,
        rollback_execution_blocked_count,
        rollback_rehearsal_recording_blocked_count,
        rollback_rehearsal_receipt_persistence_blocked_count,
        rollback_rehearsal_evidence_missing_count,
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
        rollback_rehearsal_allowed: false,
        rollback_rehearsal_executed: false,
        rollback_execution_allowed: false,
        rollback_executed: false,
        rollback_rehearsal_recording_allowed: false,
        rollback_rehearsal_receipt_persistence_allowed: false,
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
        rollback_rehearsal_boundary_readback_ready,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_entries()
-> Vec<
    ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackEntry,
> {
    controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackEntry {
            id: rollback_rehearsal_boundary_id(entry.source_blocker_id),
            source_blocker_id: entry.source_blocker_id,
            packet_id: entry.packet_id,
            packet_payload_hash: entry.packet_payload_hash,
            attachment_key: entry.attachment_key,
            attachment_route: entry.attachment_route,
            credential_boundary_key: entry.credential_boundary_key,
            credential_boundary_route: entry.credential_boundary_route,
            rollback_rehearsal_boundary_key: rollback_rehearsal_boundary_key(entry.source_blocker_id),
            rollback_rehearsal_boundary_route: rollback_rehearsal_boundary_route(entry.source_blocker_id),
            operator_display_order: entry.operator_display_order,
            operator_status: entry.operator_status,
            observed_state: "rollback_rehearsal_boundary_closed_no_execution",
            previous_state: entry.previous_state,
            current_state: entry.current_state,
            state_delta: entry.state_delta,
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            operator_visible: true,
            attachment_visible: true,
            credential_boundary_confirmed: true,
            rollback_rehearsal_boundary_readback_visible: true,
            rollback_rehearsal_boundary_status: "closed_no_execution",
            rollback_rehearsal_evidence_state: "missing",
            rollback_rehearsal_allowed: false,
            rollback_rehearsal_executed: false,
            rollback_execution_allowed: false,
            rollback_executed: false,
            rollback_rehearsal_recording_allowed: false,
            rollback_rehearsal_recorded: false,
            rollback_rehearsal_receipt_persistence_allowed: false,
            rollback_rehearsal_receipt_persisted: false,
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

fn rollback_rehearsal_boundary_id(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "rollback_rehearsal_boundary_readback_dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "rollback_rehearsal_boundary_readback_operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "rollback_rehearsal_boundary_readback_fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "rollback_rehearsal_boundary_readback_credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "rollback_rehearsal_boundary_readback_gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "rollback_rehearsal_boundary_readback_rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "rollback_rehearsal_boundary_readback_kill_switch_rehearsal_missing"
        }
        _ => "rollback_rehearsal_boundary_readback_unknown",
    }
}

fn rollback_rehearsal_boundary_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.kill_switch_rehearsal_missing"
        }
        _ => {
            "controlled_live.required_evidence.gap.operator_packet_attachment.rollback_rehearsal_boundary.unknown"
        }
    }
}

fn rollback_rehearsal_boundary_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/kill-switch-rehearsal-missing"
        }
        _ => {
            "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/unknown"
        }
    }
}

impl ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
            rollback_rehearsal_executed: false,
            rollback_executed: false,
            rollback_rehearsal_recorded: false,
            rollback_rehearsal_receipt_persisted: false,
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
            rollback_executed_side_effect: false,
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
    fn rollback_rehearsal_boundary_readback_is_ready_blocked_without_execution() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_credential_boundary_readback_ready);
        assert_eq!(report.source_credential_boundary_entry_count, 7);
        assert_eq!(report.rollback_rehearsal_boundary_entry_count, 7);
        assert_eq!(report.rollback_rehearsal_boundary_ready_count, 7);
        assert_eq!(report.rollback_rehearsal_boundary_closed_count, 7);
        assert_eq!(report.rollback_rehearsal_execution_blocked_count, 7);
        assert_eq!(report.rollback_execution_blocked_count, 7);
        assert_eq!(report.rollback_rehearsal_recording_blocked_count, 7);
        assert_eq!(
            report.rollback_rehearsal_receipt_persistence_blocked_count,
            7
        );
        assert_eq!(report.rollback_rehearsal_evidence_missing_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.rollback_rehearsal_boundary_readback_ready);
        assert!(!report.rollback_rehearsal_allowed);
        assert!(!report.rollback_rehearsal_executed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.rollback_executed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn rollback_rehearsal_boundary_readback_keeps_rehearsal_closed() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "rollback_rehearsal_missing"
            && entry.rollback_rehearsal_boundary_route
                == "readback://controlled-live/operator-packet/attachment/rollback-rehearsal-boundary/rollback-rehearsal-missing"
            && entry.rollback_rehearsal_boundary_status == "closed_no_execution"));
        assert!(report.entries.iter().all(|entry| entry.operator_visible
            && entry.attachment_visible
            && entry.credential_boundary_confirmed
            && entry.rollback_rehearsal_boundary_readback_visible
            && entry.rollback_rehearsal_evidence_state == "missing"
            && !entry.rollback_rehearsal_boundary_key.is_empty()
            && !entry.rollback_rehearsal_boundary_route.is_empty()
            && !entry.rollback_rehearsal_allowed
            && !entry.rollback_rehearsal_executed
            && !entry.rollback_execution_allowed
            && !entry.rollback_executed
            && !entry.rollback_rehearsal_recording_allowed
            && !entry.rollback_rehearsal_recorded
            && !entry.rollback_rehearsal_receipt_persistence_allowed
            && !entry.rollback_rehearsal_receipt_persisted
            && !entry.approval_request_allowed
            && !entry.approval_acceptance_allowed
            && !entry.credential_read_allowed
            && !entry.evidence_recording_allowed
            && !entry.packet_persistence_allowed
            && !entry.attachment_persistence_allowed
            && !entry.readback_persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn rollback_rehearsal_boundary_readback_keeps_side_effects_closed() {
        let report =
            controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapOperatorPacketAttachmentRollbackRehearsalBoundaryReadbackSideEffects::none()
        );
    }
}
