use std::collections::BTreeSet;

use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write";

const RETENTION_REPLAY_COLLECTION_ID: &str =
    "controlled-live-evidence-receipt-store-write-denial-retention-replay";
const RETENTION_REPLAY_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention-replay";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_write_denial_ready: bool,
    pub source_write_denial_entry_count: usize,
    pub source_receipt_store_write_denied_count: usize,
    pub source_receipt_store_write_allowed_count: usize,
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
    pub source_write_denial_attached_count: usize,
    pub retention_policy_persisted_count: usize,
    pub replay_index_written_count: usize,
    pub expiry_enforced_count: usize,
    pub garbage_collection_performed_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub write_denial_retention_replay_readback_ready: bool,
    pub retention_policy_persistence_allowed: bool,
    pub replay_index_write_allowed: bool,
    pub expiry_enforcement_allowed: bool,
    pub garbage_collection_allowed: bool,
    pub receipt_store_write_attempt_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_receipt_store_write_denial_id: String,
    pub source_receipt_store_write_denial_route: String,
    pub source_receipt_store_write_denial_reason: &'static str,
    pub retention_policy_id: String,
    pub retention_policy_route: String,
    pub expiry_guard_id: String,
    pub replay_key: String,
    pub replay_idempotency_key: String,
    pub retention_readback_route: String,
    pub replay_readback_route: String,
    pub garbage_collection_denial_id: String,
    pub supersession_guard_id: String,
    pub zero_effect_digest: String,
    pub retention_state: &'static str,
    pub replay_state: &'static str,
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
    pub source_write_denial_attached: bool,
    pub receipt_store_write_denied: bool,
    pub receipt_store_write_disabled: bool,
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
    pub receipt_store_write_attempt_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteSideEffects
{
    pub retention_policy_persisted: bool,
    pub replay_index_written: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_performed: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write_report();
    let entries =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_entries();

    let retention_policy_projected_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_projected)
        .count();
    let expiry_guard_projected_count = entries
        .iter()
        .filter(|entry| entry.expiry_guard_projected)
        .count();
    let replay_key_projected_count = entries
        .iter()
        .filter(|entry| entry.replay_key_projected)
        .count();
    let replay_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.replay_idempotency_key_projected)
        .count();
    let replay_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.replay_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let retention_readback_route_projected_count = entries
        .iter()
        .filter(|entry| entry.retention_readback_route_projected)
        .count();
    let replay_readback_route_projected_count = entries
        .iter()
        .filter(|entry| entry.replay_readback_route_projected)
        .count();
    let garbage_collection_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.garbage_collection_denial_projected)
        .count();
    let supersession_guard_projected_count = entries
        .iter()
        .filter(|entry| entry.supersession_guard_projected)
        .count();
    let zero_effect_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.zero_effect_digest_projected)
        .count();
    let source_write_denial_attached_count = entries
        .iter()
        .filter(|entry| entry.source_write_denial_attached)
        .count();
    let retention_policy_persisted_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_persisted)
        .count();
    let replay_index_written_count = entries
        .iter()
        .filter(|entry| entry.replay_index_written)
        .count();
    let expiry_enforced_count = entries.iter().filter(|entry| entry.expiry_enforced).count();
    let garbage_collection_performed_count = entries
        .iter()
        .filter(|entry| entry.garbage_collection_performed)
        .count();
    let receipt_store_write_attempt_recorded_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_write_attempt_recorded)
        .count();
    let receipt_store_written_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_written)
        .count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted)
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

    let write_denial_retention_replay_readback_ready = source
        .receipt_store_write_denial_readback_ready
        && source.write_denial_entry_count == 7
        && source.receipt_store_write_denied_count == 7
        && source.receipt_store_write_allowed_count == 0
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
        && source_write_denial_attached_count == 7
        && retention_policy_persisted_count == 0
        && replay_index_written_count == 0
        && expiry_enforced_count == 0
        && garbage_collection_performed_count == 0
        && receipt_store_write_attempt_recorded_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state
                == "receipt_store_write_denial_retention_replay_projected_without_write"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.source_packet_unsent
                && entry.source_write_denial_attached
                && entry.receipt_store_write_denied
                && entry.receipt_store_write_disabled
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
                && !entry.receipt_store_write_attempt_allowed
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
        });

    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write",
        status: if write_denial_retention_replay_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_write_denial_ready: source.receipt_store_write_denial_readback_ready,
        source_write_denial_entry_count: source.write_denial_entry_count,
        source_receipt_store_write_denied_count: source.receipt_store_write_denied_count,
        source_receipt_store_write_allowed_count: source.receipt_store_write_allowed_count,
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
        source_write_denial_attached_count,
        retention_policy_persisted_count,
        replay_index_written_count,
        expiry_enforced_count,
        garbage_collection_performed_count,
        receipt_store_write_attempt_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        write_denial_retention_replay_readback_ready,
        retention_policy_persistence_allowed: false,
        replay_index_write_allowed: false,
        expiry_enforcement_allowed: false,
        garbage_collection_allowed: false,
        receipt_store_write_attempt_allowed: false,
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_without_write_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_receipt_store_write_denial_id: entry.receipt_store_write_denial_id,
                source_receipt_store_write_denial_route: entry.receipt_store_write_denial_route,
                source_receipt_store_write_denial_reason: entry.receipt_store_write_denial_reason,
                retention_policy_id: format!(
                    "receipt-store-write-denial-retention-policy:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_policy_route: format!(
                    "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention/{hyphenated}"
                ),
                expiry_guard_id: format!(
                    "receipt-store-write-denial-expiry-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_key: format!(
                    "receipt-store-write-denial-replay-key:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_key: format!(
                    "receipt-store-write-denial-replay-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention/{hyphenated}"
                ),
                replay_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/replay/{hyphenated}"
                ),
                garbage_collection_denial_id: format!(
                    "receipt-store-write-denial-gc-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                supersession_guard_id: format!(
                    "receipt-store-write-denial-supersession-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                zero_effect_digest: format!(
                    "sha256:receipt-store-write-denial-zero-effect:{}",
                    entry.source_blocker_id
                ),
                retention_state: "projected_not_persisted",
                replay_state: "projected_not_executed",
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "receipt_store_write_denial_retention_replay_projected_without_write",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_packet_unsent: entry.source_packet_unsent,
                source_write_denial_attached: entry.receipt_store_write_denial_projected,
                receipt_store_write_denied: entry.receipt_store_write_denied,
                receipt_store_write_disabled: entry.receipt_store_write_disabled,
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
                receipt_store_write_attempt_allowed: false,
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

impl
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteSideEffects
{
    pub const fn none() -> Self {
        Self {
            retention_policy_persisted: false,
            replay_index_written: false,
            expiry_enforced: false,
            garbage_collection_performed: false,
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
    fn write_denial_retention_replay_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_write_denial_ready);
        assert_eq!(report.source_write_denial_entry_count, 7);
        assert_eq!(report.source_receipt_store_write_denied_count, 7);
        assert_eq!(report.retention_replay_entry_count, 7);
        assert_eq!(report.retention_policy_projected_count, 7);
        assert_eq!(report.expiry_guard_projected_count, 7);
        assert_eq!(report.replay_key_projected_count, 7);
        assert_eq!(report.replay_idempotency_key_projected_count, 7);
        assert_eq!(report.replay_idempotency_key_unique_count, 7);
        assert!(report.write_denial_retention_replay_readback_ready);
    }

    #[test]
    fn write_denial_retention_replay_keeps_writes_and_replay_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_report();

        assert_eq!(report.retention_policy_persisted_count, 0);
        assert_eq!(report.replay_index_written_count, 0);
        assert_eq!(report.expiry_enforced_count, 0);
        assert_eq!(report.garbage_collection_performed_count, 0);
        assert_eq!(report.receipt_store_write_attempt_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.retention_policy_persistence_allowed);
        assert!(!report.replay_index_write_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteDenialRetentionReplayReadbackWithoutWriteSideEffects::none()
        );
    }

    #[test]
    fn write_denial_retention_replay_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.replay_readback_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/replay/dirty-worktree-boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry
                .retention_policy_id
                .starts_with("receipt-store-write-denial-retention-policy:")
                && entry
                    .replay_key
                    .starts_with("receipt-store-write-denial-replay-key:")
                && entry
                    .replay_idempotency_key
                    .starts_with("receipt-store-write-denial-replay-idempotency:")
                && entry
                    .zero_effect_digest
                    .starts_with("sha256:receipt-store-write-denial-zero-effect:")
                && entry.source_write_denial_attached
                && entry.receipt_store_write_denied
                && entry.receipt_store_write_disabled
                && entry.retention_policy_projected
                && entry.expiry_guard_projected
                && entry.replay_key_projected
                && entry.replay_idempotency_key_projected
                && entry.garbage_collection_denial_projected
                && entry.supersession_guard_projected
                && entry.zero_effect_digest_projected
                && !entry.retention_policy_persisted
                && !entry.replay_index_written
                && !entry.expiry_enforced
                && !entry.garbage_collection_performed
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
