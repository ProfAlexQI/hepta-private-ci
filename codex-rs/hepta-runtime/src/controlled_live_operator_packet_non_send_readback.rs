use crate::controlled_live_operator_packet_preview::controlled_live_operator_packet_preview_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_GATE: &str =
    "controlled_live_operator_packet_non_send_readback_gate";
pub const CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION: &str =
    "controlled_live_operator_packet_non_send_readback_v1";
pub const CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "phase5d_controlled_live_required_evidence_collection_plan_without_recording";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketNonSendReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_packet_preview_ready: bool,
    pub source_packet_id: &'static str,
    pub source_payload_hash: &'static str,
    pub source_blocker_readback_count: usize,
    pub readback_entry_count: usize,
    pub readback_ready_count: usize,
    pub packet_visible_to_operator: bool,
    pub packet_send_attempted: bool,
    pub approval_request_ready: bool,
    pub approval_request_sent: bool,
    pub approval_recorded: bool,
    pub packet_persisted: bool,
    pub readback_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub non_send_readback_ready: bool,
    pub entries: Vec<ControlledLiveOperatorPacketNonSendReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveOperatorPacketNonSendReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketNonSendReadbackEntry {
    pub id: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub source: &'static str,
    pub observed_state: &'static str,
    pub operator_visible: bool,
    pub non_send_confirmed: bool,
    pub persistence_blocked: bool,
    pub approval_request_blocked: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketNonSendReadbackSideEffects {
    pub approval_requested: bool,
    pub approval_recorded: bool,
    pub packet_sent: bool,
    pub packet_persisted: bool,
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

pub fn controlled_live_operator_packet_non_send_readback_report()
-> ControlledLiveOperatorPacketNonSendReadbackReport {
    let packet_preview = controlled_live_operator_packet_preview_report();
    let entries = controlled_live_operator_packet_non_send_readback_entries();
    let readback_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.non_send_confirmed
                && entry.persistence_blocked
                && entry.approval_request_blocked
                && !entry.live_mutation_allowed
        })
        .count();
    let non_send_readback_ready = packet_preview.operator_packet_preview_ready
        && !packet_preview.approval_request_sent
        && !packet_preview.approval_recorded
        && !packet_preview.packet_persisted
        && !packet_preview.controlled_live_cutover_ready
        && !packet_preview.live_execution_allowed
        && packet_preview.blocker_readback_count == 7
        && entries.len() == 6
        && readback_ready_count == 6;

    ControlledLiveOperatorPacketNonSendReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_operator_packet_non_send_readback",
        status: if non_send_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
        schema_version: CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_operator_packet_preview_ready: packet_preview.operator_packet_preview_ready,
        source_packet_id: packet_preview.packet_id,
        source_payload_hash: packet_preview.payload_hash,
        source_blocker_readback_count: packet_preview.blocker_readback_count,
        readback_entry_count: entries.len(),
        readback_ready_count,
        packet_visible_to_operator: non_send_readback_ready,
        packet_send_attempted: false,
        approval_request_ready: false,
        approval_request_sent: false,
        approval_recorded: false,
        packet_persisted: false,
        readback_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        non_send_readback_ready,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveOperatorPacketNonSendReadbackSideEffects::none(),
    }
}

pub fn controlled_live_operator_packet_non_send_readback_entries()
-> Vec<ControlledLiveOperatorPacketNonSendReadbackEntry> {
    vec![
        entry(
            "packet_preview_visible",
            "controlled_live.operator_packet.visible",
            "controlled-live/operator-packet/visibility",
            "operator packet preview report",
            "packet_preview_available_from_local_report",
        ),
        entry(
            "approval_request_not_sent",
            "controlled_live.operator_packet.approval_request_not_sent",
            "controlled-live/operator-packet/approval-request/non-send",
            "operator packet preview report",
            "approval_request_sent_false",
        ),
        entry(
            "packet_not_persisted",
            "controlled_live.operator_packet.packet_not_persisted",
            "controlled-live/operator-packet/persistence/non-write",
            "operator packet preview report",
            "packet_persisted_false",
        ),
        entry(
            "transport_not_used",
            "controlled_live.operator_packet.transport_not_used",
            "controlled-live/operator-packet/transport/non-send",
            "operator packet preview report",
            "native_post_and_telegram_transport_false",
        ),
        entry(
            "cutover_not_promoted",
            "controlled_live.operator_packet.cutover_not_promoted",
            "controlled-live/operator-packet/cutover/non-promotion",
            "controlled-live readiness audit",
            "controlled_live_cutover_false",
        ),
        entry(
            "blocker_readback_integrity_retained",
            "controlled_live.operator_packet.blocker_readback_integrity_retained",
            "controlled-live/operator-packet/blocker-readbacks/integrity",
            "controlled-live denial readback index",
            "seven_blocker_readbacks_retained",
        ),
    ]
}

fn entry(
    id: &'static str,
    query_key: &'static str,
    readback_route: &'static str,
    source: &'static str,
    observed_state: &'static str,
) -> ControlledLiveOperatorPacketNonSendReadbackEntry {
    ControlledLiveOperatorPacketNonSendReadbackEntry {
        id,
        query_key,
        readback_route,
        source,
        observed_state,
        operator_visible: true,
        non_send_confirmed: true,
        persistence_blocked: true,
        approval_request_blocked: true,
        live_mutation_allowed: false,
    }
}

impl ControlledLiveOperatorPacketNonSendReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_recorded: false,
            packet_sent: false,
            packet_persisted: false,
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
    fn non_send_readback_is_ready_but_unsent() {
        let report = controlled_live_operator_packet_non_send_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_preview_ready);
        assert_eq!(report.source_blocker_readback_count, 7);
        assert_eq!(report.readback_entry_count, 6);
        assert_eq!(report.readback_ready_count, 6);
        assert!(report.packet_visible_to_operator);
        assert!(report.non_send_readback_ready);
        assert!(!report.packet_send_attempted);
        assert!(!report.approval_request_ready);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_recorded);
        assert!(!report.packet_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn non_send_readback_entries_cover_the_closed_packet_boundary() {
        let report = controlled_live_operator_packet_non_send_readback_report();
        let entry_ids = report
            .entries
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>();

        assert!(entry_ids.contains(&"packet_preview_visible"));
        assert!(entry_ids.contains(&"approval_request_not_sent"));
        assert!(entry_ids.contains(&"packet_not_persisted"));
        assert!(entry_ids.contains(&"transport_not_used"));
        assert!(entry_ids.contains(&"cutover_not_promoted"));
        assert!(entry_ids.contains(&"blocker_readback_integrity_retained"));
        assert!(report.entries.iter().all(|entry| entry.operator_visible
            && entry.non_send_confirmed
            && entry.persistence_blocked
            && entry.approval_request_blocked
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn non_send_readback_keeps_side_effects_closed() {
        let report = controlled_live_operator_packet_non_send_readback_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveOperatorPacketNonSendReadbackSideEffects::none()
        );
    }
}
