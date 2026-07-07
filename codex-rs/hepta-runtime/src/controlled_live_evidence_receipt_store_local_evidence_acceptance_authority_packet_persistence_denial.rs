use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_v1";
const PACKET_PERSISTENCE_DENIAL_REASON: &str = "operator_packet_persistence_disabled_local_evidence_acceptance_authority_missing_local_receipt_store_write_disabled";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialReport {
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
    pub source_send_allowed_count: usize,
    pub source_send_attempt_recorded_count: usize,
    pub source_packet_persistence_disabled_count: usize,
    pub source_operator_packet_sent_count: usize,
    pub source_operator_packet_persisted_count: usize,
    pub source_local_evidence_acceptance_authority_present_count: usize,
    pub source_local_evidence_acceptance_allowed_count: usize,
    pub source_local_evidence_acceptance_recorded_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_non_authority_receipt_projected_count: usize,
    pub source_non_authority_receipt_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub persistence_denial_entry_count: usize,
    pub persistence_denial_projected_count: usize,
    pub packet_persistence_denied_count: usize,
    pub packet_persistence_disabled_count: usize,
    pub packet_persistence_allowed_count: usize,
    pub packet_persistence_attempt_recorded_count: usize,
    pub packet_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub non_send_projection_count: usize,
    pub send_attempt_recorded_count: usize,
    pub local_evidence_acceptance_authority_present_count: usize,
    pub local_evidence_acceptance_allowed_count: usize,
    pub local_evidence_acceptance_recorded_count: usize,
    pub authority_decision_recorded_count: usize,
    pub non_authority_receipt_projected_count: usize,
    pub non_authority_receipt_persisted_count: usize,
    pub evidence_acceptance_recorded_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub local_evidence_acceptance_authority_packet_persistence_denial_readback_ready: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub packet_persistence_attempt_recording_allowed: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persistence_denial_receipt_persistence_allowed: bool,
    pub packet_persistence_denial_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub authority_decision_recording_allowed: bool,
    pub non_authority_receipt_persistence_allowed: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub receipt_store_write_attempt_recording_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_authority_packet_id: &'static str,
    pub source_authority_packet_route: &'static str,
    pub source_authority_packet_key: String,
    pub source_packet_non_send_readback_id: String,
    pub source_packet_non_send_readback_route: String,
    pub source_authority_decision_request_id: String,
    pub source_authority_decision_request_route: String,
    pub source_non_authority_receipt_id: String,
    pub source_non_authority_receipt_route: String,
    pub packet_persistence_denial_id: String,
    pub packet_persistence_denial_route: String,
    pub packet_persistence_denial_receipt_id: String,
    pub packet_persistence_denial_reason: &'static str,
    pub observed_state: &'static str,
    pub source_packet_unsent: bool,
    pub source_send_disabled: bool,
    pub non_send_projected: bool,
    pub packet_persistence_denial_projected: bool,
    pub packet_persistence_denied: bool,
    pub packet_persistence_disabled: bool,
    pub packet_persistence_allowed: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub local_evidence_acceptance_authority_required: bool,
    pub local_evidence_acceptance_authority_present: bool,
    pub authority_decision_request_projected: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_projected: bool,
    pub non_authority_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
    pub local_evidence_acceptance_recorded: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recording_allowed: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialSideEffects
{
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persistence_denial_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_accepted: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_persisted: bool,
    pub local_evidence_acceptance_recorded: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_store_written: bool,
    pub receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialReport {
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_entries();

    let persistence_denial_projected_count =
        count(&entries, |entry| entry.packet_persistence_denial_projected);
    let packet_persistence_denied_count = count(&entries, |entry| entry.packet_persistence_denied);
    let packet_persistence_disabled_count =
        count(&entries, |entry| entry.packet_persistence_disabled);
    let packet_persistence_allowed_count =
        count(&entries, |entry| entry.packet_persistence_allowed);
    let packet_persistence_attempt_recorded_count =
        count(&entries, |entry| entry.packet_persistence_attempt_recorded);
    let packet_persisted_count = count(&entries, |entry| entry.packet_persisted);
    let operator_packet_sent_count = count(&entries, |entry| entry.operator_packet_sent);
    let operator_packet_persisted_count = count(&entries, |entry| entry.operator_packet_persisted);
    let non_send_projection_count = count(&entries, |entry| entry.non_send_projected);
    let send_attempt_recorded_count = count(&entries, |entry| {
        entry.source_packet_unsent && entry.packet_persistence_attempt_recorded
    });
    let local_evidence_acceptance_authority_present_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_present
    });
    let local_evidence_acceptance_allowed_count =
        count(&entries, |entry| entry.local_evidence_acceptance_allowed);
    let local_evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.local_evidence_acceptance_recorded);
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let non_authority_receipt_projected_count =
        count(&entries, |entry| entry.non_authority_receipt_projected);
    let non_authority_receipt_persisted_count =
        count(&entries, |entry| entry.non_authority_receipt_persisted);
    let evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.evidence_acceptance_recorded);
    let evidence_recorded_count = count(&entries, |entry| entry.evidence_recorded);
    let receipt_store_write_attempt_recorded_count =
        count(&entries, |entry| entry.receipt_store_write_attempt_recorded);
    let receipt_store_written_count = count(&entries, |entry| entry.receipt_store_written);
    let receipt_persisted_count = count(&entries, |entry| {
        entry.receipt_persisted || entry.non_authority_receipt_persisted
    });
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let local_evidence_acceptance_authority_packet_persistence_denial_readback_ready = source
        .local_evidence_acceptance_authority_packet_non_send_readback_ready
        && source.non_send_entry_count == 7
        && source.unsent_packet_count == 7
        && source.send_disabled_count == 7
        && source.send_allowed_count == 0
        && source.send_attempt_recorded_count == 0
        && source.packet_persistence_disabled_count == 7
        && source.operator_packet_sent_count == 0
        && source.operator_packet_persisted_count == 0
        && source.local_evidence_acceptance_authority_present_count == 0
        && source.local_evidence_acceptance_allowed_count == 0
        && source.local_evidence_acceptance_recorded_count == 0
        && source.authority_decision_recorded_count == 0
        && source.non_authority_receipt_projected_count == 7
        && source.non_authority_receipt_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && persistence_denial_projected_count == 7
        && packet_persistence_denied_count == 7
        && packet_persistence_disabled_count == 7
        && packet_persistence_allowed_count == 0
        && packet_persistence_attempt_recorded_count == 0
        && packet_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && non_send_projection_count == 7
        && send_attempt_recorded_count == 0
        && local_evidence_acceptance_authority_present_count == 0
        && local_evidence_acceptance_allowed_count == 0
        && local_evidence_acceptance_recorded_count == 0
        && authority_decision_recorded_count == 0
        && non_authority_receipt_projected_count == 7
        && non_authority_receipt_persisted_count == 0
        && evidence_acceptance_recorded_count == 0
        && evidence_recorded_count == 0
        && receipt_store_write_attempt_recorded_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(entry_is_ready_blocked);

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if local_evidence_acceptance_authority_packet_persistence_denial_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_non_send_readback_ready: source
            .local_evidence_acceptance_authority_packet_non_send_readback_ready,
        source_non_send_entry_count: source.non_send_entry_count,
        source_unsent_packet_count: source.unsent_packet_count,
        source_send_disabled_count: source.send_disabled_count,
        source_send_allowed_count: source.send_allowed_count,
        source_send_attempt_recorded_count: source.send_attempt_recorded_count,
        source_packet_persistence_disabled_count: source.packet_persistence_disabled_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_local_evidence_acceptance_authority_present_count: source
            .local_evidence_acceptance_authority_present_count,
        source_local_evidence_acceptance_allowed_count: source
            .local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count: source
            .local_evidence_acceptance_recorded_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_non_authority_receipt_projected_count: source.non_authority_receipt_projected_count,
        source_non_authority_receipt_persisted_count: source.non_authority_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source
            .receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        persistence_denial_entry_count: entries.len(),
        persistence_denial_projected_count,
        packet_persistence_denied_count,
        packet_persistence_disabled_count,
        packet_persistence_allowed_count,
        packet_persistence_attempt_recorded_count,
        packet_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        non_send_projection_count,
        send_attempt_recorded_count,
        local_evidence_acceptance_authority_present_count,
        local_evidence_acceptance_allowed_count,
        local_evidence_acceptance_recorded_count,
        authority_decision_recorded_count,
        non_authority_receipt_projected_count,
        non_authority_receipt_persisted_count,
        evidence_acceptance_recorded_count,
        evidence_recorded_count,
        receipt_store_write_attempt_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        local_evidence_acceptance_authority_packet_persistence_denial_readback_ready,
        operator_packet_send_allowed: false,
        operator_packet_sent: false,
        operator_packet_persistence_allowed: false,
        operator_packet_persisted: false,
        packet_persistence_attempt_recording_allowed: false,
        packet_persistence_attempt_recorded: false,
        packet_persistence_denial_receipt_persistence_allowed: false,
        packet_persistence_denial_receipt_persisted: false,
        local_evidence_acceptance_authority_allowed: false,
        authority_decision_recording_allowed: false,
        non_authority_receipt_persistence_allowed: false,
        local_evidence_acceptance_allowed: false,
        local_evidence_acceptance_recording_allowed: false,
        evidence_acceptance_recording_allowed: false,
        evidence_recording_allowed: false,
        receipt_store_write_attempt_recording_allowed: false,
        receipt_persistence_allowed: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_packet_persistence_disabled",
            "packet_persistence_attempt_recording_disabled",
            "packet_persistence_denial_receipt_persistence_disabled",
            "local_evidence_acceptance_authority_missing",
            "authority_decision_recording_disabled",
            "non_authority_receipt_persistence_disabled",
            "local_evidence_acceptance_disabled",
            "local_evidence_acceptance_recording_disabled",
            "evidence_acceptance_recording_disabled",
            "receipt_store_write_attempt_recording_disabled",
            "receipt_store_write_disabled",
            "receipt_persistence_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry,
> {
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_authority_packet_id: entry.source_authority_packet_id,
                source_authority_packet_route: entry.source_authority_packet_route,
                source_authority_packet_key: entry.source_authority_packet_key,
                source_packet_non_send_readback_id: entry.packet_non_send_readback_id,
                source_packet_non_send_readback_route: entry.packet_non_send_readback_route,
                source_authority_decision_request_id: entry.source_authority_decision_request_id,
                source_authority_decision_request_route: entry.source_authority_decision_request_route,
                source_non_authority_receipt_id: entry.source_non_authority_receipt_id,
                source_non_authority_receipt_route: entry.source_non_authority_receipt_route,
                packet_persistence_denial_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                packet_persistence_denial_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/{hyphenated}"
                ),
                packet_persistence_denial_receipt_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                packet_persistence_denial_reason: PACKET_PERSISTENCE_DENIAL_REASON,
                observed_state:
                    "local_evidence_acceptance_authority_packet_persistence_denied_without_persistence",
                source_packet_unsent: entry.packet_unsent,
                source_send_disabled: entry.send_disabled,
                non_send_projected: entry.non_send_projected,
                packet_persistence_denial_projected: true,
                packet_persistence_denied: true,
                packet_persistence_disabled: true,
                packet_persistence_allowed: false,
                packet_persistence_attempt_recorded: false,
                packet_persisted: false,
                operator_packet_sent: false,
                operator_packet_persisted: false,
                local_evidence_acceptance_authority_required: entry
                    .local_evidence_acceptance_authority_required,
                local_evidence_acceptance_authority_present: entry
                    .local_evidence_acceptance_authority_present,
                authority_decision_request_projected: entry.authority_decision_request_projected,
                authority_decision_recorded: false,
                non_authority_receipt_projected: entry.non_authority_receipt_projected,
                non_authority_receipt_persisted: false,
                local_evidence_acceptance_authority_allowed: false,
                local_evidence_acceptance_allowed: false,
                local_evidence_acceptance_recording_allowed: false,
                local_evidence_acceptance_recorded: false,
                evidence_acceptance_recording_allowed: false,
                evidence_acceptance_recorded: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
                receipt_store_write_attempt_recording_allowed: false,
                receipt_store_write_attempt_recorded: false,
                receipt_persistence_allowed: false,
                receipt_persisted: false,
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

fn count(
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry],
    predicate: impl Fn(
        &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry,
    ) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_packet_persistence_denied_without_persistence"
        && entry.source_packet_unsent
        && entry.source_send_disabled
        && entry.non_send_projected
        && entry.packet_persistence_denial_projected
        && entry.packet_persistence_denied
        && entry.packet_persistence_disabled
        && !entry.packet_persistence_allowed
        && !entry.packet_persistence_attempt_recorded
        && !entry.packet_persisted
        && !entry.operator_packet_sent
        && !entry.operator_packet_persisted
        && entry.local_evidence_acceptance_authority_required
        && !entry.local_evidence_acceptance_authority_present
        && entry.authority_decision_request_projected
        && !entry.authority_decision_recorded
        && entry.non_authority_receipt_projected
        && !entry.non_authority_receipt_persisted
        && !entry.local_evidence_acceptance_authority_allowed
        && !entry.local_evidence_acceptance_allowed
        && !entry.local_evidence_acceptance_recording_allowed
        && !entry.local_evidence_acceptance_recorded
        && !entry.evidence_acceptance_recording_allowed
        && !entry.evidence_acceptance_recorded
        && !entry.evidence_recording_allowed
        && !entry.evidence_recorded
        && !entry.receipt_store_write_attempt_recording_allowed
        && !entry.receipt_store_write_attempt_recorded
        && !entry.receipt_persistence_allowed
        && !entry.receipt_persisted
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
}

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialSideEffects {
    pub const fn none() -> Self {
        Self {
            operator_packet_sent: false,
            operator_packet_persisted: false,
            packet_persistence_attempt_recorded: false,
            packet_persistence_denial_receipt_persisted: false,
            local_evidence_acceptance_authority_accepted: false,
            authority_decision_recorded: false,
            non_authority_receipt_persisted: false,
            local_evidence_acceptance_recorded: false,
            evidence_acceptance_recorded: false,
            evidence_recorded: false,
            receipt_store_write_attempt_recorded: false,
            receipt_store_written: false,
            receipt_persisted: false,
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
    fn local_evidence_acceptance_authority_packet_persistence_denial_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_non_send_readback_ready);
        assert_eq!(report.source_non_send_entry_count, 7);
        assert_eq!(report.source_unsent_packet_count, 7);
        assert_eq!(report.source_send_disabled_count, 7);
        assert_eq!(report.source_send_allowed_count, 0);
        assert_eq!(report.persistence_denial_entry_count, 7);
        assert_eq!(report.persistence_denial_projected_count, 7);
        assert_eq!(report.packet_persistence_denied_count, 7);
        assert_eq!(report.packet_persistence_disabled_count, 7);
        assert!(
            report.local_evidence_acceptance_authority_packet_persistence_denial_readback_ready
        );
    }

    #[test]
    fn local_evidence_acceptance_authority_packet_persistence_denial_keeps_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.packet_persistence_allowed_count, 0);
        assert_eq!(report.packet_persistence_attempt_recorded_count, 0);
        assert_eq!(report.packet_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.local_evidence_acceptance_authority_present_count, 0);
        assert_eq!(report.local_evidence_acceptance_allowed_count, 0);
        assert_eq!(report.local_evidence_acceptance_recorded_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.non_authority_receipt_projected_count, 7);
        assert_eq!(report.non_authority_receipt_persisted_count, 0);
        assert_eq!(report.evidence_acceptance_recorded_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_write_attempt_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.operator_packet_persistence_allowed);
        assert!(!report.packet_persistence_denial_receipt_persistence_allowed);
        assert!(!report.local_evidence_acceptance_authority_allowed);
        assert!(!report.local_evidence_acceptance_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_authority_packet_persistence_denial_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.packet_persistence_denial_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/operator-live-approval-missing"));
        assert!(report.entries.iter().all(|entry| {
            entry.packet_persistence_denial_id.starts_with(
                "local-evidence-acceptance-authority-packet-persistence-denial:controlled-live-evidence-receipt-store:",
            ) && entry.packet_persistence_denial_receipt_id.starts_with(
                "local-evidence-acceptance-authority-packet-persistence-denial-receipt:controlled-live-evidence-receipt-store:",
            ) && entry.packet_persistence_denial_reason == PACKET_PERSISTENCE_DENIAL_REASON
                && entry.source_packet_unsent
                && entry.source_send_disabled
                && entry.non_send_projected
                && entry.packet_persistence_denial_projected
                && entry.packet_persistence_denied
                && entry.packet_persistence_disabled
                && !entry.packet_persistence_allowed
                && !entry.packet_persistence_attempt_recorded
                && !entry.packet_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && entry.local_evidence_acceptance_authority_required
                && !entry.local_evidence_acceptance_authority_present
                && entry.authority_decision_request_projected
                && !entry.authority_decision_recorded
                && entry.non_authority_receipt_projected
                && !entry.non_authority_receipt_persisted
                && !entry.local_evidence_acceptance_allowed
                && !entry.local_evidence_acceptance_recorded
                && !entry.evidence_acceptance_recorded
                && !entry.evidence_recorded
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
