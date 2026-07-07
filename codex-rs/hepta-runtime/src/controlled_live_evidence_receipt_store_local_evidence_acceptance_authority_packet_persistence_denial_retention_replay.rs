use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RETENTION_REPLAY_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_v1";
const RETENTION_REPLAY_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay";
const RETENTION_REPLAY_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_persistence_denial_readback_ready: bool,
    pub source_persistence_denial_entry_count: usize,
    pub source_persistence_denial_projected_count: usize,
    pub source_packet_persistence_denied_count: usize,
    pub source_packet_persistence_allowed_count: usize,
    pub source_packet_persistence_attempt_recorded_count: usize,
    pub source_packet_persisted_count: usize,
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
    pub retention_replay_collection_id: &'static str,
    pub retention_replay_collection_route: &'static str,
    pub retention_replay_entry_count: usize,
    pub retention_policy_projected_count: usize,
    pub expiry_guard_projected_count: usize,
    pub replay_key_projected_count: usize,
    pub replay_idempotency_key_projected_count: usize,
    pub replay_idempotency_key_unique_count: usize,
    pub retention_readback_route_projected_count: usize,
    pub replay_readback_route_projected_count: usize,
    pub garbage_collection_denial_projected_count: usize,
    pub supersession_guard_projected_count: usize,
    pub zero_effect_digest_projected_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_packet_persistence_denial_receipt_attached_count: usize,
    pub source_non_send_readback_attached_count: usize,
    pub source_authority_packet_attached_count: usize,
    pub retention_policy_persisted_count: usize,
    pub replay_index_written_count: usize,
    pub expiry_enforced_count: usize,
    pub garbage_collection_performed_count: usize,
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
    pub retention_replay_readback_ready: bool,
    pub retention_policy_persistence_allowed: bool,
    pub replay_index_write_allowed: bool,
    pub expiry_enforcement_allowed: bool,
    pub garbage_collection_allowed: bool,
    pub packet_persistence_attempt_recording_allowed: bool,
    pub packet_persistence_denial_receipt_persistence_allowed: bool,
    pub operator_packet_persistence_allowed: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub authority_decision_recording_allowed: bool,
    pub non_authority_receipt_persistence_allowed: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub receipt_store_write_attempt_recording_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_persistence_denial_entry_id: String,
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
    pub retention_policy_id: String,
    pub retention_policy_route: String,
    pub expiry_guard_id: String,
    pub replay_key: String,
    pub replay_idempotency_key: String,
    pub replay_readback_route: String,
    pub retention_readback_route: String,
    pub garbage_collection_denial_id: String,
    pub supersession_guard_id: String,
    pub zero_effect_digest: String,
    pub retention_state: &'static str,
    pub replay_state: &'static str,
    pub observed_state: &'static str,
    pub source_persistence_denial_attached: bool,
    pub source_packet_persistence_denial_receipt_attached: bool,
    pub source_non_send_readback_attached: bool,
    pub source_authority_packet_attached: bool,
    pub source_persistence_denial_projected: bool,
    pub source_packet_persistence_denied: bool,
    pub retention_policy_projected: bool,
    pub expiry_guard_projected: bool,
    pub replay_key_projected: bool,
    pub replay_idempotency_key_projected: bool,
    pub retention_readback_route_projected: bool,
    pub replay_readback_route_projected: bool,
    pub garbage_collection_denial_projected: bool,
    pub supersession_guard_projected: bool,
    pub zero_effect_digest_projected: bool,
    pub retention_policy_persistence_allowed: bool,
    pub retention_policy_persisted: bool,
    pub replay_index_write_allowed: bool,
    pub replay_index_written: bool,
    pub expiry_enforcement_allowed: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_allowed: bool,
    pub garbage_collection_performed: bool,
    pub packet_persistence_attempt_recording_allowed: bool,
    pub packet_persistence_attempt_recorded: bool,
    pub packet_persistence_denial_receipt_persistence_allowed: bool,
    pub packet_persistence_denial_receipt_persisted: bool,
    pub operator_packet_sent: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplaySideEffects
{
    pub retention_policy_persisted: bool,
    pub replay_index_written: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_performed: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayReport{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayReport{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_entries();

    let retention_policy_projected_count =
        count(&entries, |entry| entry.retention_policy_projected);
    let expiry_guard_projected_count = count(&entries, |entry| entry.expiry_guard_projected);
    let replay_key_projected_count = count(&entries, |entry| entry.replay_key_projected);
    let replay_idempotency_key_projected_count =
        count(&entries, |entry| entry.replay_idempotency_key_projected);
    let replay_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.replay_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let retention_readback_route_projected_count =
        count(&entries, |entry| entry.retention_readback_route_projected);
    let replay_readback_route_projected_count =
        count(&entries, |entry| entry.replay_readback_route_projected);
    let garbage_collection_denial_projected_count =
        count(&entries, |entry| entry.garbage_collection_denial_projected);
    let supersession_guard_projected_count =
        count(&entries, |entry| entry.supersession_guard_projected);
    let zero_effect_digest_projected_count =
        count(&entries, |entry| entry.zero_effect_digest_projected);
    let source_persistence_denial_attached_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_packet_persistence_denial_receipt_attached_count = count(&entries, |entry| {
        entry.source_packet_persistence_denial_receipt_attached
    });
    let source_non_send_readback_attached_count =
        count(&entries, |entry| entry.source_non_send_readback_attached);
    let source_authority_packet_attached_count =
        count(&entries, |entry| entry.source_authority_packet_attached);
    let retention_policy_persisted_count =
        count(&entries, |entry| entry.retention_policy_persisted);
    let replay_index_written_count = count(&entries, |entry| entry.replay_index_written);
    let expiry_enforced_count = count(&entries, |entry| entry.expiry_enforced);
    let garbage_collection_performed_count =
        count(&entries, |entry| entry.garbage_collection_performed);
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
        entry.receipt_persisted
            || entry.non_authority_receipt_persisted
            || entry.packet_persistence_denial_receipt_persisted
    });
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let retention_replay_readback_ready = source
        .local_evidence_acceptance_authority_packet_persistence_denial_readback_ready
        && source.persistence_denial_entry_count == 7
        && source.persistence_denial_projected_count == 7
        && source.packet_persistence_denied_count == 7
        && source.packet_persistence_allowed_count == 0
        && source.packet_persistence_attempt_recorded_count == 0
        && source.packet_persisted_count == 0
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
        && retention_policy_projected_count == 7
        && expiry_guard_projected_count == 7
        && replay_key_projected_count == 7
        && replay_idempotency_key_projected_count == 7
        && replay_idempotency_key_unique_count == 7
        && retention_readback_route_projected_count == 7
        && replay_readback_route_projected_count == 7
        && garbage_collection_denial_projected_count == 7
        && supersession_guard_projected_count == 7
        && zero_effect_digest_projected_count == 7
        && source_persistence_denial_attached_count == 7
        && source_packet_persistence_denial_receipt_attached_count == 7
        && source_non_send_readback_attached_count == 7
        && source_authority_packet_attached_count == 7
        && retention_policy_persisted_count == 0
        && replay_index_written_count == 0
        && expiry_enforced_count == 0
        && garbage_collection_performed_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if retention_replay_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_persistence_denial_readback_ready: source
            .local_evidence_acceptance_authority_packet_persistence_denial_readback_ready,
        source_persistence_denial_entry_count: source.persistence_denial_entry_count,
        source_persistence_denial_projected_count: source.persistence_denial_projected_count,
        source_packet_persistence_denied_count: source.packet_persistence_denied_count,
        source_packet_persistence_allowed_count: source.packet_persistence_allowed_count,
        source_packet_persistence_attempt_recorded_count: source
            .packet_persistence_attempt_recorded_count,
        source_packet_persisted_count: source.packet_persisted_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_local_evidence_acceptance_authority_present_count: source
            .local_evidence_acceptance_authority_present_count,
        source_local_evidence_acceptance_allowed_count: source
            .local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count: source
            .local_evidence_acceptance_recorded_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_non_authority_receipt_persisted_count: source.non_authority_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source
            .receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        retention_replay_collection_id: RETENTION_REPLAY_COLLECTION_ID,
        retention_replay_collection_route: RETENTION_REPLAY_COLLECTION_ROUTE,
        retention_replay_entry_count: entries.len(),
        retention_policy_projected_count,
        expiry_guard_projected_count,
        replay_key_projected_count,
        replay_idempotency_key_projected_count,
        replay_idempotency_key_unique_count,
        retention_readback_route_projected_count,
        replay_readback_route_projected_count,
        garbage_collection_denial_projected_count,
        supersession_guard_projected_count,
        zero_effect_digest_projected_count,
        source_persistence_denial_attached_count,
        source_packet_persistence_denial_receipt_attached_count,
        source_non_send_readback_attached_count,
        source_authority_packet_attached_count,
        retention_policy_persisted_count,
        replay_index_written_count,
        expiry_enforced_count,
        garbage_collection_performed_count,
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
        retention_replay_readback_ready,
        retention_policy_persistence_allowed: false,
        replay_index_write_allowed: false,
        expiry_enforcement_allowed: false,
        garbage_collection_allowed: false,
        packet_persistence_attempt_recording_allowed: false,
        packet_persistence_denial_receipt_persistence_allowed: false,
        operator_packet_persistence_allowed: false,
        local_evidence_acceptance_authority_allowed: false,
        authority_decision_recording_allowed: false,
        non_authority_receipt_persistence_allowed: false,
        local_evidence_acceptance_allowed: false,
        local_evidence_acceptance_recording_allowed: false,
        evidence_acceptance_recording_allowed: false,
        evidence_recording_allowed: false,
        receipt_store_write_attempt_recording_allowed: false,
        receipt_store_write_allowed: false,
        receipt_persistence_allowed: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "retention_policy_persistence_disabled",
            "replay_index_write_disabled",
            "expiry_enforcement_disabled",
            "garbage_collection_disabled",
            "packet_persistence_attempt_recording_disabled",
            "packet_persistence_denial_receipt_persistence_disabled",
            "operator_packet_persistence_disabled",
            "local_evidence_acceptance_authority_missing",
            "authority_decision_recording_disabled",
            "non_authority_receipt_persistence_disabled",
            "local_evidence_acceptance_disabled",
            "local_evidence_acceptance_recording_disabled",
            "evidence_acceptance_recording_disabled",
            "evidence_recording_disabled",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RETENTION_REPLAY_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplaySideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry>
{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_persistence_denial_entry_id: entry.id,
                source_persistence_denial_id: entry.packet_persistence_denial_id,
                source_persistence_denial_route: entry.packet_persistence_denial_route,
                source_persistence_denial_reason: entry.packet_persistence_denial_reason,
                source_packet_persistence_denial_receipt_id: entry
                    .packet_persistence_denial_receipt_id,
                source_authority_packet_id: entry.source_authority_packet_id,
                source_authority_packet_route: entry.source_authority_packet_route,
                source_authority_packet_key: entry.source_authority_packet_key,
                source_packet_non_send_readback_id: entry.source_packet_non_send_readback_id,
                source_packet_non_send_readback_route: entry.source_packet_non_send_readback_route,
                source_authority_decision_request_id: entry
                    .source_authority_decision_request_id,
                source_authority_decision_request_route: entry
                    .source_authority_decision_request_route,
                source_non_authority_receipt_id: entry.source_non_authority_receipt_id,
                source_non_authority_receipt_route: entry.source_non_authority_receipt_route,
                retention_policy_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-retention-policy:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_policy_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/retention/{hyphenated}"
                ),
                expiry_guard_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-expiry-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_key: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-replay-key:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_key: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-replay-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/replay/{hyphenated}"
                ),
                retention_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/readback/{hyphenated}"
                ),
                garbage_collection_denial_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-gc-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                supersession_guard_id: format!(
                    "local-evidence-acceptance-authority-packet-persistence-denial-supersession-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                zero_effect_digest: format!(
                    "sha256:local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-zero-effect:{}",
                    entry.source_blocker_id
                ),
                retention_state: "projected_not_persisted",
                replay_state: "projected_not_written",
                observed_state:
                    "local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_projected_without_persistence",
                source_persistence_denial_attached: entry.packet_persistence_denial_projected,
                source_packet_persistence_denial_receipt_attached: true,
                source_non_send_readback_attached: entry.non_send_projected,
                source_authority_packet_attached: true,
                source_persistence_denial_projected: entry.packet_persistence_denial_projected,
                source_packet_persistence_denied: entry.packet_persistence_denied,
                retention_policy_projected: true,
                expiry_guard_projected: true,
                replay_key_projected: true,
                replay_idempotency_key_projected: true,
                retention_readback_route_projected: true,
                replay_readback_route_projected: true,
                garbage_collection_denial_projected: true,
                supersession_guard_projected: true,
                zero_effect_digest_projected: true,
                retention_policy_persistence_allowed: false,
                retention_policy_persisted: false,
                replay_index_write_allowed: false,
                replay_index_written: false,
                expiry_enforcement_allowed: false,
                expiry_enforced: false,
                garbage_collection_allowed: false,
                garbage_collection_performed: false,
                packet_persistence_attempt_recording_allowed: false,
                packet_persistence_attempt_recorded: false,
                packet_persistence_denial_receipt_persistence_allowed: false,
                packet_persistence_denial_receipt_persisted: false,
                operator_packet_sent: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry],
    predicate: impl Fn(
        &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry,
    ) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplayEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_projected_without_persistence"
        && entry.retention_state == "projected_not_persisted"
        && entry.replay_state == "projected_not_written"
        && entry.source_persistence_denial_attached
        && entry.source_packet_persistence_denial_receipt_attached
        && entry.source_non_send_readback_attached
        && entry.source_authority_packet_attached
        && entry.source_persistence_denial_projected
        && entry.source_packet_persistence_denied
        && entry.retention_policy_projected
        && entry.expiry_guard_projected
        && entry.replay_key_projected
        && entry.replay_idempotency_key_projected
        && entry.retention_readback_route_projected
        && entry.replay_readback_route_projected
        && entry.garbage_collection_denial_projected
        && entry.supersession_guard_projected
        && entry.zero_effect_digest_projected
        && !entry.retention_policy_persistence_allowed
        && !entry.retention_policy_persisted
        && !entry.replay_index_write_allowed
        && !entry.replay_index_written
        && !entry.expiry_enforcement_allowed
        && !entry.expiry_enforced
        && !entry.garbage_collection_allowed
        && !entry.garbage_collection_performed
        && !entry.packet_persistence_attempt_recording_allowed
        && !entry.packet_persistence_attempt_recorded
        && !entry.packet_persistence_denial_receipt_persistence_allowed
        && !entry.packet_persistence_denial_receipt_persisted
        && !entry.operator_packet_sent
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplaySideEffects {
    pub const fn none() -> Self {
        Self {
            retention_policy_persisted: false,
            replay_index_written: false,
            expiry_enforced: false,
            garbage_collection_performed: false,
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
    fn local_authority_packet_persistence_denial_retention_replay_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_persistence_denial_readback_ready);
        assert_eq!(report.source_persistence_denial_entry_count, 7);
        assert_eq!(report.source_persistence_denial_projected_count, 7);
        assert_eq!(report.source_packet_persistence_denied_count, 7);
        assert_eq!(report.retention_replay_entry_count, 7);
        assert_eq!(report.retention_policy_projected_count, 7);
        assert_eq!(report.expiry_guard_projected_count, 7);
        assert_eq!(report.replay_key_projected_count, 7);
        assert_eq!(report.replay_idempotency_key_projected_count, 7);
        assert_eq!(report.replay_idempotency_key_unique_count, 7);
        assert_eq!(report.retention_readback_route_projected_count, 7);
        assert_eq!(report.replay_readback_route_projected_count, 7);
        assert_eq!(report.garbage_collection_denial_projected_count, 7);
        assert_eq!(report.supersession_guard_projected_count, 7);
        assert_eq!(report.zero_effect_digest_projected_count, 7);
        assert_eq!(report.source_persistence_denial_attached_count, 7);
        assert_eq!(
            report.source_packet_persistence_denial_receipt_attached_count,
            7
        );
        assert_eq!(report.source_non_send_readback_attached_count, 7);
        assert_eq!(report.source_authority_packet_attached_count, 7);
        assert!(report.retention_replay_readback_ready);
    }

    #[test]
    fn local_authority_packet_persistence_denial_retention_replay_keeps_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report();

        assert_eq!(report.retention_policy_persisted_count, 0);
        assert_eq!(report.replay_index_written_count, 0);
        assert_eq!(report.expiry_enforced_count, 0);
        assert_eq!(report.garbage_collection_performed_count, 0);
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
        assert!(!report.retention_policy_persistence_allowed);
        assert!(!report.replay_index_write_allowed);
        assert!(!report.packet_persistence_denial_receipt_persistence_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketPersistenceDenialRetentionReplaySideEffects::none()
        );
    }

    #[test]
    fn local_authority_packet_persistence_denial_retention_replay_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.retention_policy_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay/retention/operator-live-approval-missing"
            && entry.replay_idempotency_key
                == "local-evidence-acceptance-authority-packet-persistence-denial-replay-idempotency:controlled-live-evidence-receipt-store:operator_live_approval_missing"));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_without_persistence_")
                && entry.source_persistence_denial_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_without_persistence_")
                && entry.source_persistence_denial_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial:")
                && entry.source_packet_persistence_denial_receipt_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial-receipt:")
                && entry.retention_policy_id.starts_with("local-evidence-acceptance-authority-packet-persistence-denial-retention-policy:")
                && entry.replay_key.starts_with("local-evidence-acceptance-authority-packet-persistence-denial-replay-key:")
                && entry.zero_effect_digest.starts_with("sha256:local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-zero-effect:")
                && entry_is_ready_blocked(entry)
        }));
    }
}
