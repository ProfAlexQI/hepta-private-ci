use crate::controlled_live_evidence_receipt_store_persistence_open_preconditions_readback::controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_GATE:
    &str = "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording";

const OPERATOR_ACCEPTANCE_PACKET_ID: &str =
    "controlled-live-evidence-receipt-store-operator-acceptance-packet";
const OPERATOR_ACCEPTANCE_PACKET_ROUTE: &str =
    "operator-packet://controlled-live/evidence-receipt-store/acceptance";
const OPERATOR_ACCEPTANCE_PACKET_PAYLOAD_FINGERPRINT: &str =
    "sha256:controlled-live-evidence-receipt-store-operator-acceptance-packet-no-acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_persistence_open_preconditions_ready: bool,
    pub source_precondition_entry_count: usize,
    pub source_operator_approval_present_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_persistence_open_allowed: bool,
    pub operator_acceptance_packet_id: &'static str,
    pub operator_acceptance_packet_route: &'static str,
    pub operator_acceptance_packet_payload_fingerprint: &'static str,
    pub packet_entry_count: usize,
    pub packet_projected_count: usize,
    pub packet_ready_count: usize,
    pub checklist_projected_count: usize,
    pub operator_acceptance_required_count: usize,
    pub operator_acceptance_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub persistence_precondition_catalog_present_count: usize,
    pub persistence_open_allowed_count: usize,
    pub acceptance_decision_request_projected_count: usize,
    pub acceptance_decision_recorded_count: usize,
    pub non_acceptance_receipt_projected_count: usize,
    pub non_acceptance_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_persisted_count: usize,
    pub operator_acceptance_packet_readback_ready: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub approval_request_allowed: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_accepted: bool,
    pub acceptance_recording_allowed: bool,
    pub acceptance_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries:
        Vec<ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub receipt_id: &'static str,
    pub receipt_path: &'static str,
    pub persistence_precondition_route: String,
    pub operator_acceptance_packet_id: &'static str,
    pub operator_acceptance_packet_route: &'static str,
    pub acceptance_decision_request_id: String,
    pub acceptance_decision_request_route: String,
    pub non_acceptance_receipt_id: String,
    pub operator_approval_id: String,
    pub evidence_acceptance_key: String,
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
    pub packet_projected: bool,
    pub packet_ready: bool,
    pub checklist_projected: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub persistence_precondition_catalog_present: bool,
    pub persistence_open_allowed: bool,
    pub acceptance_decision_request_projected: bool,
    pub acceptance_decision_recorded: bool,
    pub non_acceptance_receipt_projected: bool,
    pub non_acceptance_receipt_persisted: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub approval_request_allowed: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_accepted: bool,
    pub acceptance_recording_allowed: bool,
    pub acceptance_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_persisted: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceSideEffects
{
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub receipt_persisted: bool,
    pub receipt_store_written: bool,
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
    pub kill_switch_rehearsal_executed: bool,
    pub kill_switch_mutated: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report()
-> ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceReport,
    > = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_entries();

    let packet_projected_count = entries
        .iter()
        .filter(|entry| entry.packet_projected)
        .count();
    let packet_ready_count = entries.iter().filter(|entry| entry.packet_ready).count();
    let checklist_projected_count = entries
        .iter()
        .filter(|entry| entry.checklist_projected)
        .count();
    let operator_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_required)
        .count();
    let operator_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_present)
        .count();
    let evidence_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required)
        .count();
    let evidence_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_present)
        .count();
    let persistence_precondition_catalog_present_count = entries
        .iter()
        .filter(|entry| entry.persistence_precondition_catalog_present)
        .count();
    let persistence_open_allowed_count = entries
        .iter()
        .filter(|entry| entry.persistence_open_allowed)
        .count();
    let acceptance_decision_request_projected_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_request_projected)
        .count();
    let acceptance_decision_recorded_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_recorded)
        .count();
    let non_acceptance_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.non_acceptance_receipt_projected)
        .count();
    let non_acceptance_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_acceptance_receipt_persisted)
        .count();
    let operator_packet_sent_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_sent)
        .count();
    let operator_packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_persisted)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted || entry.non_acceptance_receipt_persisted)
        .count();

    let operator_acceptance_packet_readback_ready = source
        .persistence_open_preconditions_readback_ready
        && source.precondition_entry_count == 7
        && source.precondition_catalog_ready_count == 7
        && source.operator_approval_present_count == 0
        && source.evidence_acceptance_present_count == 0
        && !source.persistence_open_allowed
        && !source.receipt_store_written
        && !source.receipt_persisted
        && entries.len() == 7
        && packet_projected_count == 7
        && packet_ready_count == 7
        && checklist_projected_count == 7
        && operator_acceptance_required_count == 7
        && operator_acceptance_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && persistence_precondition_catalog_present_count == 7
        && persistence_open_allowed_count == 0
        && acceptance_decision_request_projected_count == 7
        && acceptance_decision_recorded_count == 0
        && non_acceptance_receipt_projected_count == 7
        && non_acceptance_receipt_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && evidence_recorded_count == 0
        && receipt_persisted_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "operator_acceptance_packet_projected_without_acceptance"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.packet_projected
                && entry.packet_ready
                && entry.checklist_projected
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.persistence_precondition_catalog_present
                && !entry.persistence_open_allowed
                && entry.acceptance_decision_request_projected
                && !entry.acceptance_decision_recorded
                && entry.non_acceptance_receipt_projected
                && !entry.non_acceptance_receipt_persisted
                && !entry.operator_packet_send_allowed
                && !entry.operator_packet_sent
                && !entry.operator_packet_persistence_allowed
                && !entry.operator_packet_persisted
                && !entry.approval_request_allowed
                && !entry.approval_request_sent
                && !entry.approval_acceptance_allowed
                && !entry.approval_accepted
                && !entry.acceptance_recording_allowed
                && !entry.acceptance_recorded
                && !entry.evidence_recording_allowed
                && !entry.evidence_recorded
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.ledger_write_allowed
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.credential_read_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance",
        status: if operator_acceptance_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_persistence_open_preconditions_ready:
            source.persistence_open_preconditions_readback_ready,
        source_precondition_entry_count: source.precondition_entry_count,
        source_operator_approval_present_count: source.operator_approval_present_count,
        source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
        source_persistence_open_allowed: source.persistence_open_allowed,
        operator_acceptance_packet_id: OPERATOR_ACCEPTANCE_PACKET_ID,
        operator_acceptance_packet_route: OPERATOR_ACCEPTANCE_PACKET_ROUTE,
        operator_acceptance_packet_payload_fingerprint:
            OPERATOR_ACCEPTANCE_PACKET_PAYLOAD_FINGERPRINT,
        packet_entry_count: entries.len(),
        packet_projected_count,
        packet_ready_count,
        checklist_projected_count,
        operator_acceptance_required_count,
        operator_acceptance_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        persistence_precondition_catalog_present_count,
        persistence_open_allowed_count,
        acceptance_decision_request_projected_count,
        acceptance_decision_recorded_count,
        non_acceptance_receipt_projected_count,
        non_acceptance_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        evidence_recorded_count,
        receipt_persisted_count,
        operator_acceptance_packet_readback_ready,
        operator_packet_send_allowed: false,
        operator_packet_sent: false,
        operator_packet_persistence_allowed: false,
        operator_packet_persisted: false,
        approval_request_allowed: false,
        approval_request_sent: false,
        approval_acceptance_allowed: false,
        approval_accepted: false,
        acceptance_recording_allowed: false,
        acceptance_recorded: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        receipt_persistence_allowed: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "operator_acceptance_missing",
            "evidence_acceptance_missing",
            "persistence_open_preconditions_missing",
            "acceptance_recording_disabled",
            "evidence_recording_disabled",
            "receipt_persistence_disabled",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceEntry> {
    controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceEntry {
                id: format!(
                    "evidence_receipt_store_operator_acceptance_packet_without_acceptance_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                receipt_id: entry.receipt_id,
                receipt_path: entry.receipt_path,
                persistence_precondition_route: entry.persistence_precondition_route,
                operator_acceptance_packet_id: OPERATOR_ACCEPTANCE_PACKET_ID,
                operator_acceptance_packet_route: OPERATOR_ACCEPTANCE_PACKET_ROUTE,
                acceptance_decision_request_id: format!(
                    "acceptance-decision-request:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                acceptance_decision_request_route: format!(
                    "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/{hyphenated}"
                ),
                non_acceptance_receipt_id: format!(
                    "non-acceptance-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_approval_id: entry.operator_approval_id,
                evidence_acceptance_key: entry.evidence_acceptance_key,
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "operator_acceptance_packet_projected_without_acceptance",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                packet_projected: true,
                packet_ready: true,
                checklist_projected: true,
                operator_acceptance_required: true,
                operator_acceptance_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                persistence_precondition_catalog_present: true,
                persistence_open_allowed: false,
                acceptance_decision_request_projected: true,
                acceptance_decision_recorded: false,
                non_acceptance_receipt_projected: true,
                non_acceptance_receipt_persisted: false,
                operator_packet_send_allowed: false,
                operator_packet_sent: false,
                operator_packet_persistence_allowed: false,
                operator_packet_persisted: false,
                approval_request_allowed: false,
                approval_request_sent: false,
                approval_acceptance_allowed: false,
                approval_accepted: false,
                acceptance_recording_allowed: false,
                acceptance_recorded: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
                receipt_persistence_allowed: false,
                receipt_persisted: false,
                receipt_store_write_allowed: false,
                receipt_store_written: false,
                ledger_write_allowed: false,
                workflow_event_log_write_allowed: false,
                sqlite_write_allowed: false,
                credential_read_allowed: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

impl
    ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceSideEffects
{
    pub const fn none() -> Self {
        Self {
            operator_packet_sent: false,
            operator_packet_persisted: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            acceptance_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            receipt_persisted: false,
            receipt_store_written: false,
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
            kill_switch_rehearsal_executed: false,
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
    fn operator_acceptance_packet_projects_all_entries_without_acceptance() {
        let report =
            controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_persistence_open_preconditions_ready);
        assert_eq!(report.source_precondition_entry_count, 7);
        assert_eq!(report.source_operator_approval_present_count, 0);
        assert_eq!(report.source_evidence_acceptance_present_count, 0);
        assert!(!report.source_persistence_open_allowed);
        assert_eq!(report.packet_entry_count, 7);
        assert_eq!(report.packet_projected_count, 7);
        assert_eq!(report.packet_ready_count, 7);
        assert_eq!(report.checklist_projected_count, 7);
        assert_eq!(report.operator_acceptance_required_count, 7);
        assert_eq!(report.operator_acceptance_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.persistence_precondition_catalog_present_count, 7);
        assert_eq!(report.persistence_open_allowed_count, 0);
        assert!(report.operator_acceptance_packet_readback_ready);
    }

    #[test]
    fn operator_acceptance_packet_keeps_send_recording_and_persistence_closed() {
        let report =
            controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report();

        assert_eq!(report.acceptance_decision_request_projected_count, 7);
        assert_eq!(report.acceptance_decision_recorded_count, 0);
        assert_eq!(report.non_acceptance_receipt_projected_count, 7);
        assert_eq!(report.non_acceptance_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_persistence_allowed);
        assert!(!report.approval_request_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.acceptance_recording_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreOperatorAcceptancePacketReadbackWithoutAcceptanceSideEffects::none()
        );
    }

    #[test]
    fn operator_acceptance_packet_entries_are_stable_and_unaccepted() {
        let report =
            controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.acceptance_decision_request_route
                == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/dirty-worktree-boundary"
            && entry.non_acceptance_receipt_id
                == "non-acceptance-receipt:controlled-live-evidence-receipt-store:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.operator_acceptance_packet_id == OPERATOR_ACCEPTANCE_PACKET_ID
                && entry.operator_acceptance_packet_route == OPERATOR_ACCEPTANCE_PACKET_ROUTE
                && entry.packet_projected
                && entry.packet_ready
                && entry.checklist_projected
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.persistence_precondition_catalog_present
                && !entry.persistence_open_allowed
                && entry.acceptance_decision_request_projected
                && !entry.acceptance_decision_recorded
                && entry.non_acceptance_receipt_projected
                && !entry.non_acceptance_receipt_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && !entry.acceptance_recorded
                && !entry.evidence_recorded
                && !entry.receipt_persisted
                && !entry.receipt_store_written
                && !entry.live_mutation_allowed
        }));
    }
}
