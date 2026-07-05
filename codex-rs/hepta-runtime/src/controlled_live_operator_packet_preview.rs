use crate::controlled_live_readiness_denial_readback_index::controlled_live_readiness_denial_readback_index_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_GATE: &str =
    "controlled_live_operator_packet_preview_gate";
pub const CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_SCHEMA_VERSION: &str =
    "controlled_live_operator_packet_preview_v1";
pub const CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_RECOMMENDED_NEXT_GATE: &str =
    "phase5c_controlled_live_operator_packet_non_send_readback_without_approval_request";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketPreviewReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_readback_index_ready: bool,
    pub source_cutover_blocked: bool,
    pub source_blocker_count: usize,
    pub packet_id: &'static str,
    pub scope_id: &'static str,
    pub payload_hash: &'static str,
    pub rollback_owner: &'static str,
    pub packet_section_count: usize,
    pub blocker_readback_count: usize,
    pub required_evidence_count: usize,
    pub operator_packet_preview_ready: bool,
    pub approval_request_ready: bool,
    pub approval_request_sent: bool,
    pub approval_recorded: bool,
    pub packet_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub sections: Vec<ControlledLiveOperatorPacketSection>,
    pub blocker_readbacks: Vec<ControlledLiveOperatorPacketBlockerReadback>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveOperatorPacketPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketSection {
    pub id: &'static str,
    pub title: &'static str,
    pub source: &'static str,
    pub preview_ready: bool,
    pub mutation_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketBlockerReadback {
    pub source_blocker_id: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub current_state: &'static str,
    pub included_in_packet: bool,
    pub acceptance_allowed: bool,
    pub waiver_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveOperatorPacketPreviewSideEffects {
    pub approval_requested: bool,
    pub approval_recorded: bool,
    pub packet_persisted: bool,
    pub readback_persisted: bool,
    pub blocker_waived: bool,
    pub denial_accepted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub rollback_executed: bool,
    pub kill_switch_mutated: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_operator_packet_preview_report() -> ControlledLiveOperatorPacketPreviewReport
{
    let readback_index = controlled_live_readiness_denial_readback_index_report();
    let sections = controlled_live_operator_packet_sections();
    let blocker_readbacks = controlled_live_operator_packet_blocker_readbacks();
    let required_evidence_count = blocker_readbacks
        .iter()
        .filter(|readback| !readback.required_evidence.is_empty())
        .count();
    let operator_packet_preview_ready = readback_index.readback_index_ready
        && !readback_index.controlled_live_cutover_ready
        && readback_index.index_entry_count == 7
        && sections.len() == 6
        && sections
            .iter()
            .all(|section| section.preview_ready && !section.mutation_enabled)
        && blocker_readbacks.len() == 7
        && required_evidence_count == 7
        && blocker_readbacks.iter().all(|readback| {
            readback.included_in_packet
                && !readback.acceptance_allowed
                && !readback.waiver_allowed
                && !readback.live_mutation_allowed
        });

    ControlledLiveOperatorPacketPreviewReport {
        runtime: "hepta",
        surface: "controlled_live_operator_packet_preview",
        status: if operator_packet_preview_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_GATE,
        schema_version: CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_readback_index_ready: readback_index.readback_index_ready,
        source_cutover_blocked: !readback_index.controlled_live_cutover_ready,
        source_blocker_count: readback_index.source_blocker_count,
        packet_id: "controlled-live-operator-packet-preview",
        scope_id: "hepta-system-controlled-live-read-only-chain",
        payload_hash: "sha256:controlled-live-operator-packet-preview-no-live-payload",
        rollback_owner: "operator-explicit-before-live",
        packet_section_count: sections.len(),
        blocker_readback_count: blocker_readbacks.len(),
        required_evidence_count,
        operator_packet_preview_ready,
        approval_request_ready: false,
        approval_request_sent: false,
        approval_recorded: false,
        packet_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        sections,
        blocker_readbacks,
        recommended_next_gate: CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveOperatorPacketPreviewSideEffects::none(),
    }
}

pub fn controlled_live_operator_packet_sections() -> Vec<ControlledLiveOperatorPacketSection> {
    vec![
        section("scope", "Scope", "controlled-live readiness audit"),
        section("payload_hash", "Payload Hash", "operator packet preview"),
        section(
            "rollback_owner",
            "Rollback Owner",
            "operator packet preview",
        ),
        section(
            "blocker_readbacks",
            "Blocker Readbacks",
            "controlled-live denial readback index",
        ),
        section(
            "required_evidence",
            "Required Evidence",
            "controlled-live denial readback index",
        ),
        section("closed_boundary", "Closed Boundary", "local no-op preview"),
    ]
}

pub fn controlled_live_operator_packet_blocker_readbacks()
-> Vec<ControlledLiveOperatorPacketBlockerReadback> {
    controlled_live_readiness_denial_readback_index_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveOperatorPacketBlockerReadback {
            source_blocker_id: entry.source_blocker_id,
            query_key: entry.query_key,
            readback_route: entry.readback_route,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            current_state: entry.current_state,
            included_in_packet: true,
            acceptance_allowed: false,
            waiver_allowed: false,
            live_mutation_allowed: false,
        })
        .collect()
}

fn section(
    id: &'static str,
    title: &'static str,
    source: &'static str,
) -> ControlledLiveOperatorPacketSection {
    ControlledLiveOperatorPacketSection {
        id,
        title,
        source,
        preview_ready: true,
        mutation_enabled: false,
    }
}

impl ControlledLiveOperatorPacketPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_recorded: false,
            packet_persisted: false,
            readback_persisted: false,
            blocker_waived: false,
            denial_accepted: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            rollback_executed: false,
            kill_switch_mutated: false,
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
    fn operator_packet_preview_is_ready_but_not_sendable() {
        let report = controlled_live_operator_packet_preview_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_readback_index_ready);
        assert!(report.source_cutover_blocked);
        assert_eq!(report.source_blocker_count, 7);
        assert_eq!(report.packet_section_count, 6);
        assert_eq!(report.blocker_readback_count, 7);
        assert_eq!(report.required_evidence_count, 7);
        assert!(report.operator_packet_preview_ready);
        assert!(!report.approval_request_ready);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_recorded);
        assert!(!report.packet_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn operator_packet_preview_includes_all_blocker_readbacks() {
        let report = controlled_live_operator_packet_preview_report();
        let blocker_ids = report
            .blocker_readbacks
            .iter()
            .map(|readback| readback.source_blocker_id)
            .collect::<Vec<_>>();

        assert!(blocker_ids.contains(&"operator_live_approval_missing"));
        assert!(blocker_ids.contains(&"fresh_soak_readback_missing"));
        assert!(blocker_ids.contains(&"credential_boundary_attestation_missing"));
        assert!(blocker_ids.contains(&"gateway_native_telegram_post_boundary_approval_missing"));
        assert!(blocker_ids.contains(&"rollback_rehearsal_missing"));
        assert!(blocker_ids.contains(&"kill_switch_rehearsal_missing"));
        assert!(
            report
                .blocker_readbacks
                .iter()
                .all(|readback| readback.included_in_packet
                    && !readback.acceptance_allowed
                    && !readback.waiver_allowed
                    && !readback.live_mutation_allowed)
        );
    }

    #[test]
    fn operator_packet_preview_keeps_side_effects_closed() {
        let report = controlled_live_operator_packet_preview_report();

        assert!(
            report
                .sections
                .iter()
                .all(|section| section.preview_ready && !section.mutation_enabled)
        );
        assert_eq!(
            report.side_effects,
            ControlledLiveOperatorPacketPreviewSideEffects::none()
        );
    }
}
