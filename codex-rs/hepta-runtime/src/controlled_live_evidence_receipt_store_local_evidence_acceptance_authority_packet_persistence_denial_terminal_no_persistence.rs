use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_v1";
const TERMINAL_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-persistence-denial-terminal-no-persistence";
const TERMINAL_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/terminal-no-persistence";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_retention_replay_readback_ready: bool,
    pub source_retention_replay_entry_count: usize,
    pub source_retention_policy_projected_count: usize,
    pub source_expiry_guard_projected_count: usize,
    pub source_replay_key_projected_count: usize,
    pub source_replay_idempotency_key_unique_count: usize,
    pub source_retention_readback_route_projected_count: usize,
    pub source_replay_readback_route_projected_count: usize,
    pub source_garbage_collection_denial_projected_count: usize,
    pub source_supersession_guard_projected_count: usize,
    pub source_zero_effect_digest_projected_count: usize,
    pub source_packet_persistence_attempt_recorded_count: usize,
    pub source_packet_persistence_denial_receipt_persisted_count: usize,
    pub source_operator_packet_sent_count: usize,
    pub source_operator_packet_persisted_count: usize,
    pub source_local_evidence_acceptance_authority_present_count: usize,
    pub source_local_evidence_acceptance_allowed_count: usize,
    pub source_local_evidence_acceptance_recorded_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_non_authority_receipt_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub terminal_collection_id: &'static str,
    pub terminal_collection_route: &'static str,
    pub terminal_entry_count: usize,
    pub terminal_closeout_projected_count: usize,
    pub terminal_no_persistence_confirmed_count: usize,
    pub terminal_closeout_key_projected_count: usize,
    pub terminal_closeout_key_unique_count: usize,
    pub terminal_readback_route_projected_count: usize,
    pub source_retention_replay_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_packet_persistence_denial_receipt_attached_count: usize,
    pub source_non_send_readback_attached_count: usize,
    pub source_authority_packet_attached_count: usize,
    pub terminal_closeout_recorded_count: usize,
    pub terminal_closeout_persisted_count: usize,
    pub terminal_closeout_accepted_count: usize,
    pub terminal_closeout_authoritative_count: usize,
    pub packet_persistence_attempt_recorded_count: usize,
    pub packet_persistence_denial_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub local_evidence_acceptance_authority_present_count: usize,
    pub local_evidence_acceptance_allowed_count: usize,
    pub local_evidence_acceptance_recorded_count: usize,
    pub authority_decision_recorded_count: usize,
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
    pub terminal_no_persistence_readback_ready: bool,
    pub terminal_closeout_recording_allowed: bool,
    pub terminal_closeout_persistence_allowed: bool,
    pub terminal_closeout_acceptance_allowed: bool,
    pub packet_persistence_denial_receipt_persistence_allowed: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_persistence_allowed: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub authority_decision_recording_allowed: bool,
    pub non_authority_receipt_persistence_allowed: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_retention_replay_entry_id: String,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_persistence_denial_reason: &'static str,
    pub source_packet_persistence_denial_receipt_id: String,
    pub source_authority_packet_id: &'static str,
    pub source_authority_packet_route: &'static str,
    pub source_authority_packet_key: String,
    pub source_packet_non_send_readback_id: String,
    pub source_packet_non_send_readback_route: String,
    pub source_authority_decision_request_id: String,
    pub source_authority_decision_request_route: String,
    pub source_non_authority_receipt_id: String,
    pub source_non_authority_receipt_route: String,
    pub source_retention_policy_id: String,
    pub source_replay_idempotency_key: String,
    pub source_zero_effect_digest: String,
    pub terminal_closeout_id: String,
    pub terminal_closeout_key: String,
    pub terminal_closeout_route: String,
    pub terminal_reason: &'static str,
    pub terminal_state: &'static str,
    pub observed_state: &'static str,
    pub source_retention_replay_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_packet_persistence_denial_receipt_attached: bool,
    pub source_non_send_readback_attached: bool,
    pub source_authority_packet_attached: bool,
    pub terminal_closeout_projected: bool,
    pub terminal_no_persistence_confirmed: bool,
    pub terminal_closeout_key_projected: bool,
    pub terminal_readback_route_projected: bool,
    pub terminal_closeout_recording_allowed: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persistence_allowed: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_acceptance_allowed: bool,
    pub terminal_closeout_accepted: bool,
    pub terminal_closeout_authoritative: bool,
    pub packet_persistence_denial_receipt_persistence_allowed: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persistence_denial_receipt_persisted: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub local_evidence_acceptance_authority_present: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recorded: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_persisted: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recording_allowed: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceSideEffects
{
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_accepted: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persistence_denial_receipt_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report(
) -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceReport
{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_entries();

    let terminal_closeout_projected_count =
        count(&entries, |entry| entry.terminal_closeout_projected);
    let terminal_no_persistence_confirmed_count =
        count(&entries, |entry| entry.terminal_no_persistence_confirmed);
    let terminal_closeout_key_projected_count =
        count(&entries, |entry| entry.terminal_closeout_key_projected);
    let terminal_closeout_key_unique_count = entries
        .iter()
        .map(|entry| entry.terminal_closeout_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let terminal_readback_route_projected_count =
        count(&entries, |entry| entry.terminal_readback_route_projected);
    let source_retention_replay_attached_count =
        count(&entries, |entry| entry.source_retention_replay_attached);
    let source_persistence_denial_attached_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_packet_persistence_denial_receipt_attached_count = count(&entries, |entry| {
        entry.source_packet_persistence_denial_receipt_attached
    });
    let source_non_send_readback_attached_count =
        count(&entries, |entry| entry.source_non_send_readback_attached);
    let source_authority_packet_attached_count =
        count(&entries, |entry| entry.source_authority_packet_attached);
    let terminal_closeout_recorded_count =
        count(&entries, |entry| entry.terminal_closeout_recorded);
    let terminal_closeout_persisted_count =
        count(&entries, |entry| entry.terminal_closeout_persisted);
    let terminal_closeout_accepted_count =
        count(&entries, |entry| entry.terminal_closeout_accepted);
    let terminal_closeout_authoritative_count =
        count(&entries, |entry| entry.terminal_closeout_authoritative);
    let packet_persistence_attempt_recorded_count =
        count(&entries, |entry| entry.packet_persistence_attempt_recorded);
    let packet_persistence_denial_receipt_persisted_count = count(&entries, |entry| {
        entry.packet_persistence_denial_receipt_persisted
    });
    let operator_packet_sent_count = count(&entries, |entry| entry.operator_packet_sent);
    let operator_packet_persisted_count = count(&entries, |entry| entry.operator_packet_persisted);
    let local_evidence_acceptance_authority_present_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_present
    });
    let local_evidence_acceptance_allowed_count =
        count(&entries, |entry| entry.local_evidence_acceptance_allowed);
    let local_evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.local_evidence_acceptance_recorded);
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let non_authority_receipt_persisted_count =
        count(&entries, |entry| entry.non_authority_receipt_persisted);
    let evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.evidence_acceptance_recorded);
    let evidence_recorded_count = count(&entries, |entry| entry.evidence_recorded);
    let receipt_store_write_attempt_recorded_count =
        count(&entries, |entry| entry.receipt_store_write_attempt_recorded);
    let receipt_store_written_count = count(&entries, |entry| entry.receipt_store_written);
    let receipt_persisted_count = count(&entries, |entry| {
        entry.receipt_persisted || entry.packet_persistence_denial_receipt_persisted
    });
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let terminal_no_persistence_readback_ready = source.retention_replay_readback_ready
        && source.retention_replay_entry_count == 7
        && source.retention_policy_projected_count == 7
        && source.expiry_guard_projected_count == 7
        && source.replay_key_projected_count == 7
        && source.replay_idempotency_key_unique_count == 7
        && source.retention_readback_route_projected_count == 7
        && source.replay_readback_route_projected_count == 7
        && source.garbage_collection_denial_projected_count == 7
        && source.supersession_guard_projected_count == 7
        && source.zero_effect_digest_projected_count == 7
        && source.source_persistence_denial_attached_count == 7
        && source.source_packet_persistence_denial_receipt_attached_count == 7
        && source.source_non_send_readback_attached_count == 7
        && source.source_authority_packet_attached_count == 7
        && source.packet_persistence_attempt_recorded_count == 0
        && source.packet_persistence_denial_receipt_persisted_count == 0
        && source.operator_packet_sent_count == 0
        && source.operator_packet_persisted_count == 0
        && source.local_evidence_acceptance_authority_present_count == 0
        && source.local_evidence_acceptance_allowed_count == 0
        && source.local_evidence_acceptance_recorded_count == 0
        && source.authority_decision_recorded_count == 0
        && source.non_authority_receipt_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && terminal_closeout_projected_count == 7
        && terminal_no_persistence_confirmed_count == 7
        && terminal_closeout_key_projected_count == 7
        && terminal_closeout_key_unique_count == 7
        && terminal_readback_route_projected_count == 7
        && source_retention_replay_attached_count == 7
        && source_persistence_denial_attached_count == 7
        && source_packet_persistence_denial_receipt_attached_count == 7
        && source_non_send_readback_attached_count == 7
        && source_authority_packet_attached_count == 7
        && terminal_closeout_recorded_count == 0
        && terminal_closeout_persisted_count == 0
        && terminal_closeout_accepted_count == 0
        && terminal_closeout_authoritative_count == 0
        && packet_persistence_attempt_recorded_count == 0
        && packet_persistence_denial_receipt_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && local_evidence_acceptance_authority_present_count == 0
        && local_evidence_acceptance_allowed_count == 0
        && local_evidence_acceptance_recorded_count == 0
        && authority_decision_recorded_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if terminal_no_persistence_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_retention_replay_readback_ready: source.retention_replay_readback_ready,
        source_retention_replay_entry_count: source.retention_replay_entry_count,
        source_retention_policy_projected_count: source.retention_policy_projected_count,
        source_expiry_guard_projected_count: source.expiry_guard_projected_count,
        source_replay_key_projected_count: source.replay_key_projected_count,
        source_replay_idempotency_key_unique_count:
            source.replay_idempotency_key_unique_count,
        source_retention_readback_route_projected_count:
            source.retention_readback_route_projected_count,
        source_replay_readback_route_projected_count: source.replay_readback_route_projected_count,
        source_garbage_collection_denial_projected_count:
            source.garbage_collection_denial_projected_count,
        source_supersession_guard_projected_count: source.supersession_guard_projected_count,
        source_zero_effect_digest_projected_count: source.zero_effect_digest_projected_count,
        source_packet_persistence_attempt_recorded_count:
            source.packet_persistence_attempt_recorded_count,
        source_packet_persistence_denial_receipt_persisted_count: source.packet_persistence_denial_receipt_persisted_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_local_evidence_acceptance_authority_present_count:
            source.local_evidence_acceptance_authority_present_count,
        source_local_evidence_acceptance_allowed_count:
            source.local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count:
            source.local_evidence_acceptance_recorded_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_non_authority_receipt_persisted_count:
            source.non_authority_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count:
            source.receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        terminal_collection_id: TERMINAL_COLLECTION_ID,
        terminal_collection_route: TERMINAL_COLLECTION_ROUTE,
        terminal_entry_count: entries.len(),
        terminal_closeout_projected_count,
        terminal_no_persistence_confirmed_count,
        terminal_closeout_key_projected_count,
        terminal_closeout_key_unique_count,
        terminal_readback_route_projected_count,
        source_retention_replay_attached_count,
        source_persistence_denial_attached_count,
        source_packet_persistence_denial_receipt_attached_count,
        source_non_send_readback_attached_count,
        source_authority_packet_attached_count,
        terminal_closeout_recorded_count,
        terminal_closeout_persisted_count,
        terminal_closeout_accepted_count,
        terminal_closeout_authoritative_count,
        packet_persistence_attempt_recorded_count,
        packet_persistence_denial_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        local_evidence_acceptance_authority_present_count,
        local_evidence_acceptance_allowed_count,
        local_evidence_acceptance_recorded_count,
        authority_decision_recorded_count,
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
        terminal_no_persistence_readback_ready,
        terminal_closeout_recording_allowed: false,
        terminal_closeout_persistence_allowed: false,
        terminal_closeout_acceptance_allowed: false,
        packet_persistence_denial_receipt_persistence_allowed: false,
        operator_packet_send_allowed: false,
        operator_packet_persistence_allowed: false,
        local_evidence_acceptance_authority_allowed: false,
        authority_decision_recording_allowed: false,
        non_authority_receipt_persistence_allowed: false,
        local_evidence_acceptance_allowed: false,
        local_evidence_acceptance_recording_allowed: false,
        receipt_store_write_allowed: false,
        receipt_persistence_allowed: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "terminal_closeout_recording_disabled",
            "terminal_closeout_persistence_disabled",
            "terminal_closeout_acceptance_disabled",
            "packet_persistence_denial_receipt_persistence_disabled",
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "local_evidence_acceptance_authority_missing",
            "authority_decision_recording_disabled",
            "non_authority_receipt_persistence_disabled",
            "local_evidence_acceptance_disabled",
            "local_evidence_acceptance_recording_disabled",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_retention_replay_entry_id: entry.id,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_persistence_denial_reason: entry.source_persistence_denial_reason,
                source_packet_persistence_denial_receipt_id: entry.source_packet_persistence_denial_receipt_id,
                source_authority_packet_id: entry.source_authority_packet_id,
                source_authority_packet_route: entry.source_authority_packet_route,
                source_authority_packet_key: entry.source_authority_packet_key,
                source_packet_non_send_readback_id: entry.source_packet_non_send_readback_id,
                source_packet_non_send_readback_route: entry.source_packet_non_send_readback_route,
                source_authority_decision_request_id: entry.source_authority_decision_request_id,
                source_authority_decision_request_route: entry.source_authority_decision_request_route,
                source_non_authority_receipt_id: entry.source_non_authority_receipt_id,
                source_non_authority_receipt_route: entry.source_non_authority_receipt_route,
                source_retention_policy_id: entry.retention_policy_id,
                source_replay_idempotency_key: entry.replay_idempotency_key,
                source_zero_effect_digest: entry.zero_effect_digest,
                terminal_closeout_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-terminal-no-persistence:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                terminal_closeout_key: format!(
                    "terminal-no-persistence:local-evidence-acceptance-authority-packet-persistence-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                terminal_closeout_route: format!("{TERMINAL_COLLECTION_ROUTE}/{hyphenated}"),
                terminal_reason:
                    "local_evidence_acceptance_authority_packet_persistence_denied_retention_replay_projected_no_persistence_authority",
                terminal_state: "terminal_no_persistence",
                observed_state:
                    "local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_closed",
                source_retention_replay_attached: true,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_packet_persistence_denial_receipt_attached: entry
                    .source_packet_persistence_denial_receipt_attached,
                source_non_send_readback_attached: entry
                    .source_non_send_readback_attached,
                source_authority_packet_attached: entry.source_authority_packet_attached,
                terminal_closeout_projected: true,
                terminal_no_persistence_confirmed: true,
                terminal_closeout_key_projected: true,
                terminal_readback_route_projected: true,
                terminal_closeout_recording_allowed: false,
                terminal_closeout_recorded: false,
                terminal_closeout_persistence_allowed: false,
                terminal_closeout_persisted: false,
                terminal_closeout_acceptance_allowed: false,
                terminal_closeout_accepted: false,
                terminal_closeout_authoritative: false,
                packet_persistence_denial_receipt_persistence_allowed: false,
                packet_persistence_attempt_recorded: false,
                packet_persistence_denial_receipt_persisted: false,
                operator_packet_send_allowed: false,
                operator_packet_sent: false,
                operator_packet_persistence_allowed: false,
                operator_packet_persisted: false,
                local_evidence_acceptance_authority_present: false,
                local_evidence_acceptance_allowed: false,
                local_evidence_acceptance_recorded: false,
                authority_decision_recorded: false,
                non_authority_receipt_persisted: false,
                evidence_acceptance_recording_allowed: false,
                evidence_acceptance_recorded: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
                receipt_store_write_attempt_recording_allowed: false,
                receipt_store_write_attempt_recorded: false,
                receipt_store_write_allowed: false,
                receipt_store_written: false,
                receipt_persistence_allowed: false,
                receipt_persisted: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_closed"
        && entry.terminal_state == "terminal_no_persistence"
        && entry.source_retention_replay_attached
        && entry.source_persistence_denial_attached
        && entry.source_packet_persistence_denial_receipt_attached
        && entry.source_non_send_readback_attached
        && entry.source_authority_packet_attached
        && entry.terminal_closeout_projected
        && entry.terminal_no_persistence_confirmed
        && entry.terminal_closeout_key_projected
        && entry.terminal_readback_route_projected
        && !entry.terminal_closeout_recording_allowed
        && !entry.terminal_closeout_recorded
        && !entry.terminal_closeout_persistence_allowed
        && !entry.terminal_closeout_persisted
        && !entry.terminal_closeout_acceptance_allowed
        && !entry.terminal_closeout_accepted
        && !entry.terminal_closeout_authoritative
        && !entry.packet_persistence_denial_receipt_persistence_allowed
        && !entry.packet_persistence_attempt_recorded
        && !entry.packet_persistence_denial_receipt_persisted
        && !entry.operator_packet_send_allowed
        && !entry.operator_packet_sent
        && !entry.operator_packet_persistence_allowed
        && !entry.operator_packet_persisted
        && !entry.local_evidence_acceptance_authority_present
        && !entry.local_evidence_acceptance_allowed
        && !entry.local_evidence_acceptance_recorded
        && !entry.authority_decision_recorded
        && !entry.non_authority_receipt_persisted
        && !entry.evidence_acceptance_recording_allowed
        && !entry.evidence_acceptance_recorded
        && !entry.evidence_recording_allowed
        && !entry.evidence_recorded
        && !entry.receipt_store_write_attempt_recording_allowed
        && !entry.receipt_store_write_attempt_recorded
        && !entry.receipt_store_write_allowed
        && !entry.receipt_store_written
        && !entry.receipt_persistence_allowed
        && !entry.receipt_persisted
        && !entry.ledger_write_allowed
        && !entry.ledger_written
        && !entry.workflow_event_log_write_allowed
        && !entry.workflow_event_log_written
        && !entry.sqlite_write_allowed
        && !entry.sqlite_written
        && !entry.credential_read_allowed
        && !entry.live_mutation_allowed
}

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceSideEffects {
    pub const fn none() -> Self {
        Self {
            terminal_closeout_recorded: false,
            terminal_closeout_persisted: false,
            terminal_closeout_accepted: false,
            packet_persistence_attempt_recorded: false,
            packet_persistence_denial_receipt_persisted: false,
            operator_packet_sent: false,
            operator_packet_persisted: false,
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
    fn local_authority_packet_persistence_denial_terminal_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.terminal_no_persistence_readback_ready);
        assert_eq!(report.source_retention_replay_entry_count, 7);
        assert_eq!(report.terminal_entry_count, 7);
        assert_eq!(report.terminal_closeout_projected_count, 7);
        assert_eq!(report.terminal_no_persistence_confirmed_count, 7);
        assert_eq!(report.terminal_closeout_key_projected_count, 7);
        assert_eq!(report.terminal_closeout_key_unique_count, 7);
        assert_eq!(report.terminal_readback_route_projected_count, 7);
        assert_eq!(report.source_retention_replay_attached_count, 7);
        assert_eq!(report.source_persistence_denial_attached_count, 7);
        assert_eq!(
            report.source_packet_persistence_denial_receipt_attached_count,
            7
        );
        assert_eq!(report.source_non_send_readback_attached_count, 7);
        assert_eq!(report.source_authority_packet_attached_count, 7);
    }

    #[test]
    fn local_authority_packet_persistence_denial_terminal_keeps_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report();

        assert_eq!(report.terminal_closeout_recorded_count, 0);
        assert_eq!(report.terminal_closeout_persisted_count, 0);
        assert_eq!(report.terminal_closeout_accepted_count, 0);
        assert_eq!(report.terminal_closeout_authoritative_count, 0);
        assert_eq!(report.packet_persistence_attempt_recorded_count, 0);
        assert_eq!(report.packet_persistence_denial_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.local_evidence_acceptance_authority_present_count, 0);
        assert_eq!(report.local_evidence_acceptance_allowed_count, 0);
        assert_eq!(report.local_evidence_acceptance_recorded_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
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
        assert!(!report.terminal_closeout_recording_allowed);
        assert!(!report.terminal_closeout_persistence_allowed);
        assert!(!report.terminal_closeout_acceptance_allowed);
        assert!(!report.packet_persistence_denial_receipt_persistence_allowed);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_persistence_allowed);
        assert!(!report.local_evidence_acceptance_allowed);
        assert!(!report.authority_decision_recording_allowed);
        assert!(!report.non_authority_receipt_persistence_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialTerminalNoPersistenceSideEffects::none()
        );
    }

    #[test]
    fn local_authority_packet_persistence_denial_terminal_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.terminal_closeout_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/terminal-no-persistence/dirty-worktree-boundary"
                && entry.terminal_closeout_key
                    == "terminal-no-persistence:local-evidence-acceptance-authority-packet-persistence-denial:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_")
                && entry.source_retention_replay_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_without_persistence_")
                && entry.source_persistence_denial_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial:")
                && entry.source_packet_persistence_denial_receipt_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial-receipt:")
                && entry.source_authority_packet_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"
                && entry.source_packet_non_send_readback_id.starts_with("local-evidence-acceptance-authority-packet-non-send:")
                && entry.terminal_closeout_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial-terminal-no-persistence:")
                && entry_is_ready_blocked(entry)
        }));
    }
}
