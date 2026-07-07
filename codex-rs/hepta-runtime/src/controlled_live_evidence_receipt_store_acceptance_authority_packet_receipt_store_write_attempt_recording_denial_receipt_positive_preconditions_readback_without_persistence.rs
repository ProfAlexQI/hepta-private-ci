use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_readback_without_persistence";

const POSITIVE_PRECONDITIONS_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/positive-preconditions";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_retention_replay_readback_ready: bool,
    pub source_retention_replay_entry_count: usize,
    pub source_retention_policy_persisted_count: usize,
    pub source_replay_index_written_count: usize,
    pub source_write_attempt_recorded_count: usize,
    pub source_write_attempt_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_live_execution_allowed: bool,
    pub positive_preconditions_route: &'static str,
    pub precondition_entry_count: usize,
    pub positive_precondition_set_projected_count: usize,
    pub source_retention_replay_attached_count: usize,
    pub persistence_authority_required_count: usize,
    pub persistence_authority_present_count: usize,
    pub operator_persistence_approval_required_count: usize,
    pub operator_persistence_approval_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub denial_receipt_persistence_grant_required_count: usize,
    pub denial_receipt_persistence_grant_present_count: usize,
    pub atomic_append_required_count: usize,
    pub atomic_append_enabled_count: usize,
    pub post_persist_readback_required_count: usize,
    pub post_persist_readback_persisted_count: usize,
    pub rollback_anchor_required_count: usize,
    pub rollback_anchor_verified_count: usize,
    pub retention_policy_commit_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub replay_idempotency_guard_required_count: usize,
    pub replay_idempotency_guard_enabled_count: usize,
    pub positive_preconditions_missing_count: usize,
    pub denial_receipt_persistence_allowed_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub write_attempt_recorded_count: usize,
    pub write_attempt_persisted_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub positive_preconditions_readback_ready: bool,
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
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_retention_replay_entry_id: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_denial_reason: &'static str,
    pub source_retention_policy_id: String,
    pub source_retention_policy_route: String,
    pub source_replay_key: String,
    pub source_replay_idempotency_key: String,
    pub source_zero_effect_digest: String,
    pub positive_precondition_set_id: String,
    pub positive_precondition_route: String,
    pub persistence_authority_precondition_id: String,
    pub operator_persistence_approval_precondition_id: String,
    pub evidence_acceptance_precondition_id: String,
    pub denial_receipt_persistence_grant_precondition_id: String,
    pub atomic_append_precondition_id: String,
    pub post_persist_readback_precondition_id: String,
    pub rollback_anchor_precondition_id: String,
    pub retention_commit_precondition_id: String,
    pub replay_idempotency_guard_precondition_id: String,
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
    pub positive_precondition_set_projected: bool,
    pub source_retention_replay_attached: bool,
    pub persistence_authority_required: bool,
    pub persistence_authority_present: bool,
    pub operator_persistence_approval_required: bool,
    pub operator_persistence_approval_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub denial_receipt_persistence_grant_required: bool,
    pub denial_receipt_persistence_grant_present: bool,
    pub atomic_append_required: bool,
    pub atomic_append_enabled: bool,
    pub post_persist_readback_required: bool,
    pub post_persist_readback_persisted: bool,
    pub rollback_anchor_required: bool,
    pub rollback_anchor_verified: bool,
    pub retention_policy_commit_required: bool,
    pub retention_policy_committed: bool,
    pub replay_idempotency_guard_required: bool,
    pub replay_idempotency_guard_enabled: bool,
    pub positive_preconditions_missing: bool,
    pub denial_receipt_persistence_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceSideEffects
{
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceReport
{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_entries();

    let positive_precondition_set_projected_count = entries
        .iter()
        .filter(|entry| entry.positive_precondition_set_projected)
        .count();
    let source_retention_replay_attached_count = entries
        .iter()
        .filter(|entry| entry.source_retention_replay_attached)
        .count();
    let persistence_authority_required_count = entries
        .iter()
        .filter(|entry| entry.persistence_authority_required)
        .count();
    let persistence_authority_present_count = entries
        .iter()
        .filter(|entry| entry.persistence_authority_present)
        .count();
    let operator_persistence_approval_required_count = entries
        .iter()
        .filter(|entry| entry.operator_persistence_approval_required)
        .count();
    let operator_persistence_approval_present_count = entries
        .iter()
        .filter(|entry| entry.operator_persistence_approval_present)
        .count();
    let evidence_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required)
        .count();
    let evidence_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_present)
        .count();
    let denial_receipt_persistence_grant_required_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persistence_grant_required)
        .count();
    let denial_receipt_persistence_grant_present_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persistence_grant_present)
        .count();
    let atomic_append_required_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_required)
        .count();
    let atomic_append_enabled_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_enabled)
        .count();
    let post_persist_readback_required_count = entries
        .iter()
        .filter(|entry| entry.post_persist_readback_required)
        .count();
    let post_persist_readback_persisted_count = entries
        .iter()
        .filter(|entry| entry.post_persist_readback_persisted)
        .count();
    let rollback_anchor_required_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_required)
        .count();
    let rollback_anchor_verified_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_verified)
        .count();
    let retention_policy_commit_required_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_commit_required)
        .count();
    let retention_policy_committed_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_committed)
        .count();
    let replay_idempotency_guard_required_count = entries
        .iter()
        .filter(|entry| entry.replay_idempotency_guard_required)
        .count();
    let replay_idempotency_guard_enabled_count = entries
        .iter()
        .filter(|entry| entry.replay_idempotency_guard_enabled)
        .count();
    let positive_preconditions_missing_count = entries
        .iter()
        .filter(|entry| entry.positive_preconditions_missing)
        .count();
    let denial_receipt_persistence_allowed_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persistence_allowed)
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

    let positive_preconditions_readback_ready = source.retention_replay_readback_ready
        && source.retention_replay_entry_count == 7
        && source.retention_policy_persisted_count == 0
        && source.replay_index_written_count == 0
        && source.write_attempt_recorded_count == 0
        && source.write_attempt_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && positive_precondition_set_projected_count == 7
        && source_retention_replay_attached_count == 7
        && persistence_authority_required_count == 7
        && persistence_authority_present_count == 0
        && operator_persistence_approval_required_count == 7
        && operator_persistence_approval_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && denial_receipt_persistence_grant_required_count == 7
        && denial_receipt_persistence_grant_present_count == 0
        && atomic_append_required_count == 7
        && atomic_append_enabled_count == 0
        && post_persist_readback_required_count == 7
        && post_persist_readback_persisted_count == 0
        && rollback_anchor_required_count == 7
        && rollback_anchor_verified_count == 0
        && retention_policy_commit_required_count == 7
        && retention_policy_committed_count == 0
        && replay_idempotency_guard_required_count == 7
        && replay_idempotency_guard_enabled_count == 0
        && positive_preconditions_missing_count == 7
        && denial_receipt_persistence_allowed_count == 0
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
                == "write_attempt_recording_denial_receipt_positive_preconditions_projected_without_persistence"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.source_packet_unsent
                && entry.positive_precondition_set_projected
                && entry.source_retention_replay_attached
                && entry.persistence_authority_required
                && !entry.persistence_authority_present
                && entry.operator_persistence_approval_required
                && !entry.operator_persistence_approval_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.denial_receipt_persistence_grant_required
                && !entry.denial_receipt_persistence_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_persist_readback_required
                && !entry.post_persist_readback_persisted
                && entry.rollback_anchor_required
                && !entry.rollback_anchor_verified
                && entry.retention_policy_commit_required
                && !entry.retention_policy_committed
                && entry.replay_idempotency_guard_required
                && !entry.replay_idempotency_guard_enabled
                && entry.positive_preconditions_missing
                && !entry.denial_receipt_persistence_allowed
                && !entry.denial_receipt_persisted
                && !entry.write_attempt_recording_allowed
                && !entry.write_attempt_recorded
                && !entry.write_attempt_persisted
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

    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence",
        status: if positive_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_retention_replay_readback_ready: source.retention_replay_readback_ready,
        source_retention_replay_entry_count: source.retention_replay_entry_count,
        source_retention_policy_persisted_count: source.retention_policy_persisted_count,
        source_replay_index_written_count: source.replay_index_written_count,
        source_write_attempt_recorded_count: source.write_attempt_recorded_count,
        source_write_attempt_persisted_count: source.write_attempt_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_live_execution_allowed: source.live_execution_allowed,
        positive_preconditions_route: POSITIVE_PRECONDITIONS_ROUTE,
        precondition_entry_count: entries.len(),
        positive_precondition_set_projected_count,
        source_retention_replay_attached_count,
        persistence_authority_required_count,
        persistence_authority_present_count,
        operator_persistence_approval_required_count,
        operator_persistence_approval_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        denial_receipt_persistence_grant_required_count,
        denial_receipt_persistence_grant_present_count,
        atomic_append_required_count,
        atomic_append_enabled_count,
        post_persist_readback_required_count,
        post_persist_readback_persisted_count,
        rollback_anchor_required_count,
        rollback_anchor_verified_count,
        retention_policy_commit_required_count,
        retention_policy_committed_count,
        replay_idempotency_guard_required_count,
        replay_idempotency_guard_enabled_count,
        positive_preconditions_missing_count,
        denial_receipt_persistence_allowed_count,
        denial_receipt_persisted_count,
        write_attempt_recorded_count,
        write_attempt_persisted_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        positive_preconditions_readback_ready,
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
            "persistence_authority_missing",
            "operator_persistence_approval_missing",
            "evidence_acceptance_missing",
            "denial_receipt_persistence_grant_missing",
            "atomic_append_not_enabled",
            "post_persist_readback_missing",
            "rollback_anchor_missing",
            "retention_policy_not_committed",
            "replay_idempotency_guard_disabled",
            "write_attempt_recording_disabled",
            "denial_receipt_persistence_disabled",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_retention_replay_entry_id: entry.id,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_denial_reason: entry.source_denial_reason,
                source_retention_policy_id: entry.retention_policy_id,
                source_retention_policy_route: entry.retention_policy_route,
                source_replay_key: entry.replay_key,
                source_replay_idempotency_key: entry.replay_idempotency_key,
                source_zero_effect_digest: entry.zero_effect_digest,
                positive_precondition_set_id: format!(
                    "write-attempt-recording-denial-receipt-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_route: format!("{POSITIVE_PRECONDITIONS_ROUTE}/{hyphenated}"),
                persistence_authority_precondition_id: format!(
                    "persistence-authority-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                operator_persistence_approval_precondition_id: format!(
                    "operator-persistence-approval-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                evidence_acceptance_precondition_id: format!(
                    "evidence-acceptance-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                denial_receipt_persistence_grant_precondition_id: format!(
                    "denial-receipt-persistence-grant-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                atomic_append_precondition_id: format!(
                    "atomic-append-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                post_persist_readback_precondition_id: format!(
                    "post-persist-readback-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                rollback_anchor_precondition_id: format!(
                    "rollback-anchor-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                retention_commit_precondition_id: format!(
                    "retention-policy-commit-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_guard_precondition_id: format!(
                    "replay-idempotency-guard-required:controlled-live-evidence-receipt-store-write-attempt-denial:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "write_attempt_recording_denial_receipt_positive_preconditions_projected_without_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_packet_unsent: entry.source_packet_unsent,
                positive_precondition_set_projected: true,
                source_retention_replay_attached: true,
                persistence_authority_required: true,
                persistence_authority_present: false,
                operator_persistence_approval_required: true,
                operator_persistence_approval_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                denial_receipt_persistence_grant_required: true,
                denial_receipt_persistence_grant_present: false,
                atomic_append_required: true,
                atomic_append_enabled: false,
                post_persist_readback_required: true,
                post_persist_readback_persisted: false,
                rollback_anchor_required: true,
                rollback_anchor_verified: false,
                retention_policy_commit_required: true,
                retention_policy_committed: false,
                replay_idempotency_guard_required: true,
                replay_idempotency_guard_enabled: false,
                positive_preconditions_missing: true,
                denial_receipt_persistence_allowed: false,
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

impl
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceSideEffects
{
    pub const fn none() -> Self {
        Self {
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
    fn positive_preconditions_project_all_entries_without_persistence() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_retention_replay_readback_ready);
        assert_eq!(report.source_retention_replay_entry_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.positive_precondition_set_projected_count, 7);
        assert_eq!(report.source_retention_replay_attached_count, 7);
        assert_eq!(report.persistence_authority_required_count, 7);
        assert_eq!(report.persistence_authority_present_count, 0);
        assert_eq!(report.operator_persistence_approval_required_count, 7);
        assert_eq!(report.operator_persistence_approval_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.denial_receipt_persistence_grant_required_count, 7);
        assert_eq!(report.denial_receipt_persistence_grant_present_count, 0);
        assert_eq!(report.atomic_append_required_count, 7);
        assert_eq!(report.atomic_append_enabled_count, 0);
        assert!(report.positive_preconditions_readback_ready);
    }

    #[test]
    fn positive_preconditions_keep_persistence_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert_eq!(report.post_persist_readback_required_count, 7);
        assert_eq!(report.post_persist_readback_persisted_count, 0);
        assert_eq!(report.rollback_anchor_required_count, 7);
        assert_eq!(report.rollback_anchor_verified_count, 0);
        assert_eq!(report.retention_policy_commit_required_count, 7);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert_eq!(report.replay_idempotency_guard_required_count, 7);
        assert_eq!(report.replay_idempotency_guard_enabled_count, 0);
        assert_eq!(report.positive_preconditions_missing_count, 7);
        assert_eq!(report.denial_receipt_persistence_allowed_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.write_attempt_recorded_count, 0);
        assert_eq!(report.write_attempt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.write_attempt_recording_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingDenialReceiptPositivePreconditionsReadbackWithoutPersistenceSideEffects::none()
        );
    }

    #[test]
    fn positive_precondition_entries_are_stable_and_missing() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.positive_precondition_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/positive-preconditions/dirty-worktree-boundary"
            && entry.denial_receipt_persistence_grant_precondition_id
                == "denial-receipt-persistence-grant-required:controlled-live-evidence-receipt-store-write-attempt-denial:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry
                .source_retention_replay_entry_id
                .starts_with("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_without_persistence_")
                && entry
                    .source_denial_receipt_id
                    .starts_with("write-attempt-recording-denial-receipt:")
                && entry.positive_precondition_set_projected
                && entry.source_retention_replay_attached
                && entry.persistence_authority_required
                && !entry.persistence_authority_present
                && entry.operator_persistence_approval_required
                && !entry.operator_persistence_approval_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.denial_receipt_persistence_grant_required
                && !entry.denial_receipt_persistence_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_persist_readback_required
                && !entry.post_persist_readback_persisted
                && entry.rollback_anchor_required
                && !entry.rollback_anchor_verified
                && entry.retention_policy_commit_required
                && !entry.retention_policy_committed
                && entry.replay_idempotency_guard_required
                && !entry.replay_idempotency_guard_enabled
                && entry.positive_preconditions_missing
                && !entry.denial_receipt_persisted
                && !entry.write_attempt_recorded
                && !entry.write_attempt_persisted
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
