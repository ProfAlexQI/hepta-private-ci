use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback::controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write";

const PACKET_PERSISTENCE_DENIAL_REASON: &str = "operator_packet_persistence_disabled_acceptance_authority_missing_receipt_store_write_disabled";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_non_send_readback_ready: bool,
    pub source_non_send_entry_count: usize,
    pub source_unsent_packet_count: usize,
    pub source_send_disabled_count: usize,
    pub source_operator_packet_sent_count: usize,
    pub source_operator_packet_persisted_count: usize,
    pub source_acceptance_allowed_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_live_execution_allowed: bool,
    pub persistence_denial_entry_count: usize,
    pub persistence_denial_projection_count: usize,
    pub persistence_denial_receipt_projected_count: usize,
    pub persistence_denial_receipt_persisted_count: usize,
    pub packet_persistence_allowed_count: usize,
    pub packet_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub acceptance_authority_present_count: usize,
    pub acceptance_allowed_count: usize,
    pub authority_decision_recorded_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub acceptance_authority_packet_persistence_denial_readback_ready: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub persistence_denial_receipt_persistence_allowed: bool,
    pub persistence_denial_receipt_persisted: bool,
    pub acceptance_authority_allowed: bool,
    pub acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_acceptance_authority_packet_id: &'static str,
    pub source_acceptance_authority_packet_route: &'static str,
    pub source_packet_non_send_readback_id: String,
    pub source_packet_non_send_readback_route: String,
    pub persistence_denial_readback_id: String,
    pub persistence_denial_readback_route: String,
    pub persistence_denial_receipt_id: String,
    pub persistence_denial_reason: &'static str,
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
    pub source_packet_unsent: bool,
    pub source_send_disabled: bool,
    pub persistence_denial_projected: bool,
    pub persistence_denial_receipt_projected: bool,
    pub persistence_denial_receipt_persisted: bool,
    pub packet_persistence_allowed: bool,
    pub packet_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub acceptance_authority_required: bool,
    pub acceptance_authority_present: bool,
    pub authority_decision_recorded: bool,
    pub acceptance_allowed: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub ledger_written: bool,
    pub workflow_event_log_write_allowed: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_write_allowed: bool,
    pub sqlite_written: bool,
    pub credential_read_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceSideEffects
{
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub persistence_denial_receipt_persisted: bool,
    pub acceptance_authority_accepted: bool,
    pub acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_store_written: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub credential_read: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback_report(
        );
    let entries =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_entries();

    let persistence_denial_projection_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_projected)
        .count();
    let persistence_denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_receipt_projected)
        .count();
    let persistence_denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.persistence_denial_receipt_persisted)
        .count();
    let packet_persistence_allowed_count = entries
        .iter()
        .filter(|entry| entry.packet_persistence_allowed)
        .count();
    let packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.packet_persisted)
        .count();
    let operator_packet_sent_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_sent)
        .count();
    let operator_packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_persisted)
        .count();
    let acceptance_authority_present_count = entries
        .iter()
        .filter(|entry| entry.acceptance_authority_present)
        .count();
    let acceptance_allowed_count = entries
        .iter()
        .filter(|entry| entry.acceptance_allowed)
        .count();
    let authority_decision_recorded_count = entries
        .iter()
        .filter(|entry| entry.authority_decision_recorded)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let receipt_store_written_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_written)
        .count();
    let ledger_written_count = entries.iter().filter(|entry| entry.ledger_written).count();
    let workflow_event_log_written_count = entries
        .iter()
        .filter(|entry| entry.workflow_event_log_written)
        .count();
    let sqlite_written_count = entries.iter().filter(|entry| entry.sqlite_written).count();
    let live_mutation_allowed_count = entries
        .iter()
        .filter(|entry| entry.live_mutation_allowed)
        .count();

    let acceptance_authority_packet_persistence_denial_readback_ready = source
        .acceptance_authority_packet_non_send_readback_ready
        && source.non_send_entry_count == 7
        && source.unsent_packet_count == 7
        && source.send_disabled_count == 7
        && source.operator_packet_sent_count == 0
        && source.operator_packet_persisted_count == 0
        && source.acceptance_allowed_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && persistence_denial_projection_count == 7
        && persistence_denial_receipt_projected_count == 7
        && persistence_denial_receipt_persisted_count == 0
        && packet_persistence_allowed_count == 0
        && packet_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && acceptance_authority_present_count == 0
        && acceptance_allowed_count == 0
        && authority_decision_recorded_count == 0
        && evidence_recorded_count == 0
        && receipt_store_written_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state
                == "acceptance_authority_packet_persistence_denied_without_persistence"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.source_packet_unsent
                && entry.source_send_disabled
                && entry.persistence_denial_projected
                && entry.persistence_denial_receipt_projected
                && !entry.persistence_denial_receipt_persisted
                && !entry.packet_persistence_allowed
                && !entry.packet_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && !entry.authority_decision_recorded
                && !entry.acceptance_allowed
                && !entry.evidence_recorded
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.ledger_write_allowed
                && !entry.ledger_written
                && !entry.workflow_event_log_write_allowed
                && !entry.workflow_event_log_written
                && !entry.sqlite_write_allowed
                && !entry.sqlite_written
                && !entry.credential_read_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence",
        status: if acceptance_authority_packet_persistence_denial_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_non_send_readback_ready: source
            .acceptance_authority_packet_non_send_readback_ready,
        source_non_send_entry_count: source.non_send_entry_count,
        source_unsent_packet_count: source.unsent_packet_count,
        source_send_disabled_count: source.send_disabled_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_acceptance_allowed_count: source.acceptance_allowed_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_live_execution_allowed: source.live_execution_allowed,
        persistence_denial_entry_count: entries.len(),
        persistence_denial_projection_count,
        persistence_denial_receipt_projected_count,
        persistence_denial_receipt_persisted_count,
        packet_persistence_allowed_count,
        packet_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        acceptance_authority_present_count,
        acceptance_allowed_count,
        authority_decision_recorded_count,
        evidence_recorded_count,
        receipt_store_written_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        acceptance_authority_packet_persistence_denial_readback_ready,
        operator_packet_persistence_allowed: false,
        operator_packet_persisted: false,
        persistence_denial_receipt_persistence_allowed: false,
        persistence_denial_receipt_persisted: false,
        acceptance_authority_allowed: false,
        acceptance_recording_allowed: false,
        evidence_recording_allowed: false,
        receipt_store_write_allowed: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_packet_persistence_disabled",
            "persistence_denial_receipt_persistence_disabled",
            "acceptance_authority_missing",
            "operator_acceptance_missing",
            "evidence_acceptance_missing",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_persistence_denial_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_acceptance_authority_packet_id: entry.source_acceptance_authority_packet_id,
                source_acceptance_authority_packet_route: entry.source_acceptance_authority_packet_route,
                source_packet_non_send_readback_id: entry.packet_non_send_readback_id,
                source_packet_non_send_readback_route: entry.packet_non_send_readback_route,
                persistence_denial_readback_id: format!(
                    "packet-persistence-denial-readback:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                persistence_denial_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/persistence-denial/{hyphenated}"
                ),
                persistence_denial_receipt_id: format!(
                    "packet-persistence-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                persistence_denial_reason: PACKET_PERSISTENCE_DENIAL_REASON,
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "acceptance_authority_packet_persistence_denied_without_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_packet_unsent: entry.packet_unsent,
                source_send_disabled: entry.send_disabled,
                persistence_denial_projected: true,
                persistence_denial_receipt_projected: true,
                persistence_denial_receipt_persisted: false,
                packet_persistence_allowed: false,
                packet_persisted: false,
                operator_packet_sent: false,
                operator_packet_persisted: false,
                acceptance_authority_required: entry.acceptance_authority_required,
                acceptance_authority_present: entry.acceptance_authority_present,
                authority_decision_recorded: entry.authority_decision_recorded,
                acceptance_allowed: false,
                evidence_recorded: false,
                receipt_store_write_allowed: false,
                receipt_store_written: false,
                ledger_write_allowed: false,
                ledger_written: false,
                workflow_event_log_write_allowed: false,
                workflow_event_log_written: false,
                sqlite_write_allowed: false,
                sqlite_written: false,
                credential_read_allowed: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

impl ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceSideEffects {
    pub const fn none() -> Self {
        Self {
            operator_packet_sent: false,
            operator_packet_persisted: false,
            persistence_denial_receipt_persisted: false,
            acceptance_authority_accepted: false,
            acceptance_recorded: false,
            evidence_recorded: false,
            receipt_store_written: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            credential_read: false,
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
    fn acceptance_authority_packet_persistence_denial_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_non_send_readback_ready);
        assert_eq!(report.source_non_send_entry_count, 7);
        assert_eq!(report.source_unsent_packet_count, 7);
        assert_eq!(report.source_send_disabled_count, 7);
        assert_eq!(report.persistence_denial_entry_count, 7);
        assert_eq!(report.persistence_denial_projection_count, 7);
        assert_eq!(report.persistence_denial_receipt_projected_count, 7);
        assert!(report.acceptance_authority_packet_persistence_denial_readback_ready);
    }

    #[test]
    fn acceptance_authority_packet_persistence_denial_keeps_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.packet_persistence_allowed_count, 0);
        assert_eq!(report.packet_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.persistence_denial_receipt_persisted_count, 0);
        assert_eq!(report.acceptance_authority_present_count, 0);
        assert_eq!(report.acceptance_allowed_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.operator_packet_persistence_allowed);
        assert!(!report.persistence_denial_receipt_persistence_allowed);
        assert!(!report.acceptance_authority_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketPersistenceDenialReadbackWithoutPersistenceSideEffects::none()
        );
    }

    #[test]
    fn acceptance_authority_packet_persistence_denial_entries_are_stable_and_unpersisted() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.persistence_denial_readback_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/persistence-denial/operator-live-approval-missing"));
        assert!(report.entries.iter().all(|entry| {
            entry.persistence_denial_readback_id.starts_with(
                "packet-persistence-denial-readback:controlled-live-evidence-receipt-store:",
            ) && entry.persistence_denial_receipt_id.starts_with(
                "packet-persistence-denial-receipt:controlled-live-evidence-receipt-store:",
            ) && entry.persistence_denial_reason == PACKET_PERSISTENCE_DENIAL_REASON
                && entry.source_packet_unsent
                && entry.source_send_disabled
                && entry.persistence_denial_projected
                && entry.persistence_denial_receipt_projected
                && !entry.persistence_denial_receipt_persisted
                && !entry.packet_persistence_allowed
                && !entry.packet_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && !entry.authority_decision_recorded
                && !entry.acceptance_allowed
                && !entry.evidence_recorded
                && !entry.receipt_store_written
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
