use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence";

const RETENTION_REPLAY_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-retention-replay";
const RETENTION_REPLAY_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_recording_denial_receipt_readback_ready: bool,
    pub source_denial_receipt_entry_count: usize,
    pub source_denial_receipt_projected_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_acceptance_source_recorded_count: usize,
    pub source_acceptance_source_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
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
    pub source_denial_receipt_attached_count: usize,
    pub source_acceptance_source_record_attached_count: usize,
    pub retention_policy_persisted_count: usize,
    pub replay_index_written_count: usize,
    pub expiry_enforced_count: usize,
    pub garbage_collection_performed_count: usize,
    pub acceptance_source_recorded_count: usize,
    pub acceptance_source_persisted_count: usize,
    pub denial_receipt_persisted_count: usize,
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
    pub acceptance_source_recording_allowed: bool,
    pub acceptance_source_persistence_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
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
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_denial_receipt_idempotency_key: String,
    pub source_acceptance_source_record_id: String,
    pub source_acceptance_source_record_idempotency_key: String,
    pub source_recording_denial_reason: &'static str,
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
    pub retention_policy_projected: bool,
    pub expiry_guard_projected: bool,
    pub replay_key_projected: bool,
    pub replay_idempotency_key_projected: bool,
    pub retention_readback_route_projected: bool,
    pub replay_readback_route_projected: bool,
    pub garbage_collection_denial_projected: bool,
    pub supersession_guard_projected: bool,
    pub zero_effect_digest_projected: bool,
    pub source_denial_receipt_attached: bool,
    pub source_acceptance_source_record_attached: bool,
    pub retention_policy_persistence_allowed: bool,
    pub retention_policy_persisted: bool,
    pub replay_index_write_allowed: bool,
    pub replay_index_written: bool,
    pub expiry_enforcement_allowed: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_allowed: bool,
    pub garbage_collection_performed: bool,
    pub acceptance_source_recording_allowed: bool,
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persistence_allowed: bool,
    pub acceptance_source_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceSideEffects
{
    pub retention_policy_persisted: bool,
    pub replay_index_written: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_performed: bool,
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
    pub denial_receipt_persisted: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceReport{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceReport,
    > = OnceLock::new();
    REPORT
        .get_or_init(|| {
            let source =
                controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence_report();
            let entries =
                controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_entries();

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
            let source_denial_receipt_attached_count = entries
                .iter()
                .filter(|entry| entry.source_denial_receipt_attached)
                .count();
            let source_acceptance_source_record_attached_count = entries
                .iter()
                .filter(|entry| entry.source_acceptance_source_record_attached)
                .count();
            let retention_policy_persisted_count = entries
                .iter()
                .filter(|entry| entry.retention_policy_persisted)
                .count();
            let replay_index_written_count = entries
                .iter()
                .filter(|entry| entry.replay_index_written)
                .count();
            let expiry_enforced_count =
                entries.iter().filter(|entry| entry.expiry_enforced).count();
            let garbage_collection_performed_count = entries
                .iter()
                .filter(|entry| entry.garbage_collection_performed)
                .count();
            let acceptance_source_recorded_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_recorded)
                .count();
            let acceptance_source_persisted_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_persisted)
                .count();
            let denial_receipt_persisted_count = entries
                .iter()
                .filter(|entry| entry.denial_receipt_persisted)
                .count();
            let evidence_acceptance_recorded_count = entries
                .iter()
                .filter(|entry| entry.evidence_acceptance_recorded)
                .count();
            let evidence_recorded_count =
                entries.iter().filter(|entry| entry.evidence_recorded).count();
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

            let retention_replay_readback_ready =
                source.local_evidence_acceptance_recording_denial_receipt_readback_ready
                    && source.denial_receipt_entry_count == 7
                    && source.denial_receipt_projected_count == 7
                    && source.denial_receipt_persisted_count == 0
                    && source.acceptance_source_recorded_count == 0
                    && source.acceptance_source_persisted_count == 0
                    && source.evidence_acceptance_recorded_count == 0
                    && source.evidence_recorded_count == 0
                    && source.receipt_store_write_attempt_recorded_count == 0
                    && source.receipt_store_written_count == 0
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
                    && source_denial_receipt_attached_count == 7
                    && source_acceptance_source_record_attached_count == 7
                    && retention_policy_persisted_count == 0
                    && replay_index_written_count == 0
                    && expiry_enforced_count == 0
                    && garbage_collection_performed_count == 0
                    && acceptance_source_recorded_count == 0
                    && acceptance_source_persisted_count == 0
                    && denial_receipt_persisted_count == 0
                    && evidence_acceptance_recorded_count == 0
                    && evidence_recorded_count == 0
                    && receipt_store_write_attempt_recorded_count == 0
                    && receipt_store_written_count == 0
                    && receipt_persisted_count == 0
                    && ledger_written_count == 0
                    && workflow_event_log_written_count == 0
                    && sqlite_written_count == 0
                    && live_mutation_allowed_count == 0
                    && entries.iter().all(|entry| {
                        entry.observed_state
                            == "local_evidence_acceptance_recording_denial_receipt_retention_replay_projected_without_persistence"
                            && entry.previous_state == "missing"
                            && entry.current_state == "missing"
                            && entry.state_delta == "unchanged_missing"
                            && entry.retention_state == "projected_not_persisted"
                            && entry.replay_state == "projected_not_written"
                            && entry.retention_policy_projected
                            && entry.expiry_guard_projected
                            && entry.replay_key_projected
                            && entry.replay_idempotency_key_projected
                            && entry.retention_readback_route_projected
                            && entry.replay_readback_route_projected
                            && entry.garbage_collection_denial_projected
                            && entry.supersession_guard_projected
                            && entry.zero_effect_digest_projected
                            && entry.source_denial_receipt_attached
                            && entry.source_acceptance_source_record_attached
                            && !entry.retention_policy_persistence_allowed
                            && !entry.retention_policy_persisted
                            && !entry.replay_index_write_allowed
                            && !entry.replay_index_written
                            && !entry.expiry_enforcement_allowed
                            && !entry.expiry_enforced
                            && !entry.garbage_collection_allowed
                            && !entry.garbage_collection_performed
                            && !entry.acceptance_source_recording_allowed
                            && !entry.acceptance_source_recorded
                            && !entry.acceptance_source_persistence_allowed
                            && !entry.acceptance_source_persisted
                            && !entry.denial_receipt_persistence_allowed
                            && !entry.denial_receipt_persisted
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
                    });

            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceReport {
                runtime: "hepta",
                surface:
                    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence",
                status: if retention_replay_readback_ready {
                    "ready_blocked"
                } else {
                    "blocked"
                },
                gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_GATE,
                schema_version:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION,
                plugin_id: "hepta-system@hepta-local",
                source_recording_denial_receipt_readback_ready:
                    source.local_evidence_acceptance_recording_denial_receipt_readback_ready,
                source_denial_receipt_entry_count: source.denial_receipt_entry_count,
                source_denial_receipt_projected_count: source.denial_receipt_projected_count,
                source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
                source_acceptance_source_recorded_count: source.acceptance_source_recorded_count,
                source_acceptance_source_persisted_count:
                    source.acceptance_source_persisted_count,
                source_evidence_acceptance_recorded_count:
                    source.evidence_acceptance_recorded_count,
                source_evidence_recorded_count: source.evidence_recorded_count,
                source_receipt_store_write_attempt_recorded_count:
                    source.receipt_store_write_attempt_recorded_count,
                source_receipt_store_written_count: source.receipt_store_written_count,
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
                source_denial_receipt_attached_count,
                source_acceptance_source_record_attached_count,
                retention_policy_persisted_count,
                replay_index_written_count,
                expiry_enforced_count,
                garbage_collection_performed_count,
                acceptance_source_recorded_count,
                acceptance_source_persisted_count,
                denial_receipt_persisted_count,
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
                acceptance_source_recording_allowed: false,
                acceptance_source_persistence_allowed: false,
                denial_receipt_persistence_allowed: false,
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
                    "retention_policy_persistence_disabled",
                    "replay_index_write_disabled",
                    "expiry_enforcement_disabled",
                    "garbage_collection_disabled",
                    "acceptance_source_recording_disabled",
                    "acceptance_source_persistence_disabled",
                    "denial_receipt_persistence_disabled",
                    "evidence_acceptance_recording_disabled",
                    "evidence_recording_disabled",
                    "receipt_store_write_attempt_recording_disabled",
                    "receipt_persistence_disabled",
                    "receipt_store_write_disabled",
                    "ledger_write_disabled",
                    "workflow_event_log_write_disabled",
                    "sqlite_write_disabled",
                    "live_execution_disabled",
                ],
                entries,
                recommended_next_gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE,
                side_effects:
                    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceSideEffects::none(),
            }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_denial_receipt_id: entry.denial_receipt_id,
                source_denial_receipt_route: entry.denial_receipt_route,
                source_denial_receipt_digest: entry.denial_receipt_digest,
                source_denial_receipt_idempotency_key: entry.denial_receipt_idempotency_key,
                source_acceptance_source_record_id: entry.source_acceptance_source_record_id,
                source_acceptance_source_record_idempotency_key:
                    entry.source_acceptance_source_record_idempotency_key,
                source_recording_denial_reason: entry.recording_denial_reason,
                retention_policy_id: format!(
                    "retention-policy:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                retention_policy_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/retention/{hyphenated}"
                ),
                expiry_guard_id: format!(
                    "expiry-guard:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                replay_key: format!(
                    "replay-key:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_key: format!(
                    "controlled-live-evidence-receipt-store.local-evidence-acceptance-recording-denial-retention-replay.idempotency.{}",
                    entry.source_blocker_id
                ),
                replay_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/replay/{hyphenated}"
                ),
                retention_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/readback/{hyphenated}"
                ),
                garbage_collection_denial_id: format!(
                    "garbage-collection-denial:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                supersession_guard_id: format!(
                    "supersession-guard:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                zero_effect_digest: format!(
                    "sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-retention-replay-zero-effect:{}",
                    entry.source_blocker_id
                ),
                retention_state: "projected_not_persisted",
                replay_state: "projected_not_written",
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "local_evidence_acceptance_recording_denial_receipt_retention_replay_projected_without_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                retention_policy_projected: true,
                expiry_guard_projected: true,
                replay_key_projected: true,
                replay_idempotency_key_projected: true,
                retention_readback_route_projected: true,
                replay_readback_route_projected: true,
                garbage_collection_denial_projected: true,
                supersession_guard_projected: true,
                zero_effect_digest_projected: true,
                source_denial_receipt_attached: true,
                source_acceptance_source_record_attached: true,
                retention_policy_persistence_allowed: false,
                retention_policy_persisted: false,
                replay_index_write_allowed: false,
                replay_index_written: false,
                expiry_enforcement_allowed: false,
                expiry_enforced: false,
                garbage_collection_allowed: false,
                garbage_collection_performed: false,
                acceptance_source_recording_allowed: false,
                acceptance_source_recorded: false,
                acceptance_source_persistence_allowed: false,
                acceptance_source_persisted: false,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceSideEffects {
    pub const fn none() -> Self {
        Self {
            retention_policy_persisted: false,
            replay_index_written: false,
            expiry_enforced: false,
            garbage_collection_performed: false,
            acceptance_source_recorded: false,
            acceptance_source_persisted: false,
            denial_receipt_persisted: false,
            evidence_acceptance_recorded: false,
            evidence_recorded: false,
            receipt_store_write_attempt_recorded: false,
            receipt_persisted: false,
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
    fn local_evidence_acceptance_denial_retention_replay_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_recording_denial_receipt_readback_ready);
        assert_eq!(report.source_denial_receipt_entry_count, 7);
        assert_eq!(report.source_denial_receipt_projected_count, 7);
        assert_eq!(report.source_denial_receipt_persisted_count, 0);
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
        assert_eq!(report.source_denial_receipt_attached_count, 7);
        assert_eq!(report.source_acceptance_source_record_attached_count, 7);
        assert!(report.retention_replay_readback_ready);
    }

    #[test]
    fn local_evidence_acceptance_denial_retention_replay_keeps_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert_eq!(report.retention_policy_persisted_count, 0);
        assert_eq!(report.replay_index_written_count, 0);
        assert_eq!(report.expiry_enforced_count, 0);
        assert_eq!(report.garbage_collection_performed_count, 0);
        assert_eq!(report.acceptance_source_recorded_count, 0);
        assert_eq!(report.acceptance_source_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
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
        assert!(!report.expiry_enforcement_allowed);
        assert!(!report.garbage_collection_allowed);
        assert!(!report.acceptance_source_recording_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptRetentionReplayReadbackWithoutPersistenceSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_denial_retention_replay_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.retention_policy_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/retention/dirty-worktree-boundary"
            && entry.replay_idempotency_key
                == "controlled-live-evidence-receipt-store.local-evidence-acceptance-recording-denial-retention-replay.idempotency.dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.retention_state == "projected_not_persisted"
                && entry.replay_state == "projected_not_written"
                && entry.retention_policy_projected
                && entry.expiry_guard_projected
                && entry.replay_key_projected
                && entry.replay_idempotency_key_projected
                && entry.retention_readback_route_projected
                && entry.replay_readback_route_projected
                && entry.garbage_collection_denial_projected
                && entry.supersession_guard_projected
                && entry.zero_effect_digest_projected
                && entry.source_denial_receipt_attached
                && entry.source_acceptance_source_record_attached
                && !entry.retention_policy_persisted
                && !entry.replay_index_written
                && !entry.expiry_enforced
                && !entry.garbage_collection_performed
                && !entry.acceptance_source_recorded
                && !entry.acceptance_source_persisted
                && !entry.denial_receipt_persisted
                && !entry.evidence_acceptance_recorded
                && !entry.evidence_recorded
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_persisted
                && !entry.receipt_store_written
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
