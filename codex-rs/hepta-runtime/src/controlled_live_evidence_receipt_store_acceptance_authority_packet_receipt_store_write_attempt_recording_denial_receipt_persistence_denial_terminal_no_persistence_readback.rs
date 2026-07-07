use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write";

const TERMINAL_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-write-attempt-recording-denial-receipt-persistence-denial-terminal-no-persistence";
const TERMINAL_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/persistence-denial/terminal-no-persistence";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackReport {
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
    pub source_denial_receipt_persistence_attempt_recorded_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_write_attempt_recorded_count: usize,
    pub source_write_attempt_persisted_count: usize,
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
    pub source_denial_receipt_binding_attached_count: usize,
    pub terminal_closeout_recorded_count: usize,
    pub terminal_closeout_persisted_count: usize,
    pub terminal_closeout_accepted_count: usize,
    pub terminal_closeout_authoritative_count: usize,
    pub denial_receipt_persistence_attempt_recorded_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub write_attempt_recorded_count: usize,
    pub write_attempt_persisted_count: usize,
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
    pub denial_receipt_persistence_allowed: bool,
    pub write_attempt_recording_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_retention_replay_entry_id: String,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_persistence_denial_reason: &'static str,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_retention_policy_id: String,
    pub source_replay_idempotency_key: String,
    pub source_zero_effect_digest: String,
    pub terminal_closeout_id: String,
    pub terminal_closeout_key: String,
    pub terminal_closeout_route: String,
    pub terminal_reason: &'static str,
    pub terminal_state: &'static str,
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
    pub source_retention_replay_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_denial_receipt_binding_attached: bool,
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
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persistence_attempt_recorded: bool,
    pub denial_receipt_persisted: bool,
    pub write_attempt_recording_allowed: bool,
    pub write_attempt_recorded: bool,
    pub write_attempt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackSideEffects
{
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_accepted: bool,
    pub denial_receipt_persistence_attempt_recorded: bool,
    pub denial_receipt_persisted: bool,
    pub write_attempt_recorded: bool,
    pub write_attempt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
            let source =
                controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence_report();
            let entries =
                controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_entries();

            let terminal_closeout_projected_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_projected)
                .count();
            let terminal_no_persistence_confirmed_count = entries
                .iter()
                .filter(|entry| entry.terminal_no_persistence_confirmed)
                .count();
            let terminal_closeout_key_projected_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_key_projected)
                .count();
            let terminal_closeout_key_unique_count = entries
                .iter()
                .map(|entry| entry.terminal_closeout_key.as_str())
                .collect::<BTreeSet<_>>()
                .len();
            let terminal_readback_route_projected_count = entries
                .iter()
                .filter(|entry| entry.terminal_readback_route_projected)
                .count();
            let source_retention_replay_attached_count = entries
                .iter()
                .filter(|entry| entry.source_retention_replay_attached)
                .count();
            let source_persistence_denial_attached_count = entries
                .iter()
                .filter(|entry| entry.source_persistence_denial_attached)
                .count();
            let source_denial_receipt_binding_attached_count = entries
                .iter()
                .filter(|entry| entry.source_denial_receipt_binding_attached)
                .count();
            let terminal_closeout_recorded_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_recorded)
                .count();
            let terminal_closeout_persisted_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_persisted)
                .count();
            let terminal_closeout_accepted_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_accepted)
                .count();
            let terminal_closeout_authoritative_count = entries
                .iter()
                .filter(|entry| entry.terminal_closeout_authoritative)
                .count();
            let denial_receipt_persistence_attempt_recorded_count = entries
                .iter()
                .filter(|entry| entry.denial_receipt_persistence_attempt_recorded)
                .count();
            let denial_receipt_persisted_count = entries
                .iter()
                .filter(|entry| entry.denial_receipt_persisted)
                .count();
            let write_attempt_recorded_count = entries
                .iter()
                .filter(|entry| entry.write_attempt_recorded)
                .count();
            let write_attempt_persisted_count = entries
                .iter()
                .filter(|entry| entry.write_attempt_persisted)
                .count();
            let receipt_store_written_count = entries
                .iter()
                .filter(|entry| entry.receipt_store_written)
                .count();
            let receipt_persisted_count = entries
                .iter()
                .filter(|entry| entry.receipt_persisted || entry.denial_receipt_persisted)
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
                && source.denial_receipt_persistence_attempt_recorded_count == 0
                && source.denial_receipt_persisted_count == 0
                && source.write_attempt_recorded_count == 0
                && source.write_attempt_persisted_count == 0
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
                && source_denial_receipt_binding_attached_count == 7
                && terminal_closeout_recorded_count == 0
                && terminal_closeout_persisted_count == 0
                && terminal_closeout_accepted_count == 0
                && terminal_closeout_authoritative_count == 0
                && denial_receipt_persistence_attempt_recorded_count == 0
                && denial_receipt_persisted_count == 0
                && write_attempt_recorded_count == 0
                && write_attempt_persisted_count == 0
                && receipt_store_written_count == 0
                && receipt_persisted_count == 0
                && ledger_written_count == 0
                && workflow_event_log_written_count == 0
                && sqlite_written_count == 0
                && live_mutation_allowed_count == 0
                && entries.iter().all(|entry| {
                    entry.observed_state
                        == "write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_closed"
                        && entry.previous_state == "missing"
                        && entry.current_state == "missing"
                        && entry.state_delta == "unchanged_missing"
                        && entry.terminal_state == "terminal_no_persistence"
                        && entry.source_packet_unsent
                        && entry.source_retention_replay_attached
                        && entry.source_persistence_denial_attached
                        && entry.source_denial_receipt_binding_attached
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
                        && !entry.denial_receipt_persistence_allowed
                        && !entry.denial_receipt_persistence_attempt_recorded
                        && !entry.denial_receipt_persisted
                        && !entry.write_attempt_recorded
                        && !entry.write_attempt_persisted
                        && !entry.receipt_store_written
                        && !entry.receipt_persisted
                        && !entry.ledger_written
                        && !entry.workflow_event_log_written
                        && !entry.sqlite_written
                        && !entry.credential_read_allowed
                        && !entry.live_mutation_allowed
                });

            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackReport {
                runtime: "hepta",
                surface: "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback",
                status: if terminal_no_persistence_readback_ready {
                    "ready_blocked"
                } else {
                    "blocked"
                },
                gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_GATE,
                schema_version:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_SCHEMA_VERSION,
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
                source_replay_readback_route_projected_count:
                    source.replay_readback_route_projected_count,
                source_garbage_collection_denial_projected_count:
                    source.garbage_collection_denial_projected_count,
                source_supersession_guard_projected_count:
                    source.supersession_guard_projected_count,
                source_zero_effect_digest_projected_count:
                    source.zero_effect_digest_projected_count,
                source_denial_receipt_persistence_attempt_recorded_count:
                    source.denial_receipt_persistence_attempt_recorded_count,
                source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
                source_write_attempt_recorded_count: source.write_attempt_recorded_count,
                source_write_attempt_persisted_count: source.write_attempt_persisted_count,
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
                source_denial_receipt_binding_attached_count,
                terminal_closeout_recorded_count,
                terminal_closeout_persisted_count,
                terminal_closeout_accepted_count,
                terminal_closeout_authoritative_count,
                denial_receipt_persistence_attempt_recorded_count,
                denial_receipt_persisted_count,
                write_attempt_recorded_count,
                write_attempt_persisted_count,
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
                denial_receipt_persistence_allowed: false,
                write_attempt_recording_allowed: false,
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
                    "denial_receipt_persistence_disabled",
                    "write_attempt_recording_disabled",
                    "receipt_store_write_disabled",
                    "receipt_persistence_disabled",
                    "ledger_write_disabled",
                    "workflow_event_log_write_disabled",
                    "sqlite_write_disabled",
                    "live_execution_disabled",
                ],
                entries,
                recommended_next_gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
                side_effects:
                    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackSideEffects::none(),
            }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_retention_replay_entry_id: entry.id,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_persistence_denial_reason: entry.source_persistence_denial_reason,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_retention_policy_id: entry.retention_policy_id,
                source_replay_idempotency_key: entry.replay_idempotency_key,
                source_zero_effect_digest: entry.zero_effect_digest,
                terminal_closeout_id: format!(
                    "write-attempt-recording-denial-receipt-persistence-denial-terminal-no-persistence:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                terminal_closeout_key: format!(
                    "terminal-no-persistence:write-attempt-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                terminal_closeout_route: format!("{TERMINAL_COLLECTION_ROUTE}/{hyphenated}"),
                terminal_reason:
                    "write_attempt_recording_denial_receipt_persistence_denied_retention_replay_projected_no_persistence_authority",
                terminal_state: "terminal_no_persistence",
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_closed",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_packet_unsent: entry.source_packet_unsent,
                source_retention_replay_attached: true,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_denial_receipt_binding_attached: entry.source_denial_receipt_binding_attached,
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
                denial_receipt_persistence_allowed: false,
                denial_receipt_persistence_attempt_recorded: false,
                denial_receipt_persisted: false,
                write_attempt_recording_allowed: false,
                write_attempt_recorded: false,
                write_attempt_persisted: false,
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

impl ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            terminal_closeout_recorded: false,
            terminal_closeout_persisted: false,
            terminal_closeout_accepted: false,
            denial_receipt_persistence_attempt_recorded: false,
            denial_receipt_persisted: false,
            write_attempt_recorded: false,
            write_attempt_persisted: false,
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
    fn terminal_no_persistence_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report();

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
        assert_eq!(report.source_denial_receipt_binding_attached_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn terminal_no_persistence_keeps_all_persistence_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report();

        assert_eq!(report.terminal_closeout_recorded_count, 0);
        assert_eq!(report.terminal_closeout_persisted_count, 0);
        assert_eq!(report.terminal_closeout_accepted_count, 0);
        assert_eq!(report.terminal_closeout_authoritative_count, 0);
        assert_eq!(report.denial_receipt_persistence_attempt_recorded_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.write_attempt_recorded_count, 0);
        assert_eq!(report.write_attempt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.terminal_closeout_recording_allowed);
        assert!(!report.terminal_closeout_persistence_allowed);
        assert!(!report.terminal_closeout_acceptance_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.write_attempt_recording_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPersistenceDenialTerminalNoPersistenceReadbackSideEffects::none()
        );
    }

    #[test]
    fn terminal_no_persistence_entries_are_stable() {
        let entries =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_entries();

        assert!(entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.terminal_closeout_route.ends_with("/dirty-worktree-boundary")
                && entry.terminal_closeout_key
                    == "terminal-no-persistence:write-attempt-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(entries.iter().all(|entry| {
            entry.observed_state
                == "write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_closed"
                && entry.terminal_state == "terminal_no_persistence"
                && entry.source_packet_unsent
                && entry.source_retention_replay_attached
                && entry.source_persistence_denial_attached
                && entry.source_denial_receipt_binding_attached
                && entry.terminal_closeout_projected
                && entry.terminal_no_persistence_confirmed
                && entry.terminal_closeout_key_projected
                && entry.terminal_readback_route_projected
                && !entry.terminal_closeout_recorded
                && !entry.terminal_closeout_persisted
                && !entry.terminal_closeout_accepted
                && !entry.terminal_closeout_authoritative
                && !entry.denial_receipt_persisted
                && !entry.write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        }));
    }
}
