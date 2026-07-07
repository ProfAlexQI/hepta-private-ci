use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_positive_preconditions_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_v1";
const RETENTION_REPLAY_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-retention-replay";
const RETENTION_REPLAY_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/retention-replay";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_recording_denial_receipt_readback_ready: bool,
    pub source_denial_receipt_entry_count: usize,
    pub source_denial_receipt_projected_count: usize,
    pub source_denial_receipt_digest_projected_count: usize,
    pub source_denial_receipt_readback_route_projected_count: usize,
    pub source_denial_receipt_idempotency_key_projected_count: usize,
    pub source_denial_receipt_idempotency_key_unique_count: usize,
    pub source_recording_denial_attached_count: usize,
    pub source_positive_preconditions_attached_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_prior_denial_receipt_attached_count: usize,
    pub source_authority_decision_record_id_attached_count: usize,
    pub source_authority_decision_recording_allowed_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_authority_decision_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
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
    pub source_denial_receipt_attached_count: usize,
    pub source_recording_denial_attached_entry_count: usize,
    pub source_positive_preconditions_attached_entry_count: usize,
    pub source_terminal_closeout_attached_entry_count: usize,
    pub source_persistence_denial_attached_entry_count: usize,
    pub source_prior_denial_receipt_attached_entry_count: usize,
    pub source_authority_decision_record_id_attached_entry_count: usize,
    pub local_evidence_acceptance_authority_missing_count: usize,
    pub authority_decision_request_missing_count: usize,
    pub operator_authority_decision_approval_missing_count: usize,
    pub evidence_acceptance_missing_count: usize,
    pub authority_decision_recording_grant_missing_count: usize,
    pub decision_record_schema_commit_missing_count: usize,
    pub atomic_decision_record_append_missing_count: usize,
    pub post_record_readback_missing_count: usize,
    pub rollback_anchor_missing_count: usize,
    pub retention_policy_commit_missing_count: usize,
    pub replay_idempotency_guard_missing_count: usize,
    pub retention_policy_persisted_count: usize,
    pub replay_index_written_count: usize,
    pub expiry_enforced_count: usize,
    pub garbage_collection_performed_count: usize,
    pub authority_decision_recording_allowed_count: usize,
    pub authority_decision_recorded_count: usize,
    pub authority_decision_persisted_count: usize,
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
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_persistence_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
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
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_denial_receipt_entry_id: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_denial_receipt_schema_version: &'static str,
    pub source_denial_receipt_idempotency_key: String,
    pub source_recording_denial_id: String,
    pub source_recording_denial_key: String,
    pub source_recording_denial_route: String,
    pub source_recording_denial_reason: &'static str,
    pub source_recording_denial_state: &'static str,
    pub source_recording_denial_digest: String,
    pub source_positive_preconditions_entry_id: String,
    pub source_positive_precondition_set_id: String,
    pub source_positive_precondition_key: String,
    pub source_positive_precondition_route: String,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_key: String,
    pub source_terminal_closeout_route: String,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_prior_denial_receipt_id: String,
    pub source_prior_denial_receipt_route: String,
    pub source_prior_denial_receipt_digest: String,
    pub source_authority_decision_record_id: String,
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
    pub source_denial_receipt_attached: bool,
    pub source_recording_denial_attached: bool,
    pub source_positive_preconditions_attached: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_prior_denial_receipt_attached: bool,
    pub source_authority_decision_record_id_attached: bool,
    pub local_evidence_acceptance_authority_missing: bool,
    pub authority_decision_request_missing: bool,
    pub operator_authority_decision_approval_missing: bool,
    pub evidence_acceptance_missing: bool,
    pub authority_decision_recording_grant_missing: bool,
    pub decision_record_schema_commit_missing: bool,
    pub atomic_decision_record_append_missing: bool,
    pub post_record_readback_missing: bool,
    pub rollback_anchor_missing: bool,
    pub retention_policy_commit_missing: bool,
    pub replay_idempotency_guard_missing: bool,
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
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persistence_allowed: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplaySideEffects
{
    pub retention_policy_persisted: bool,
    pub replay_index_written: bool,
    pub expiry_enforced: bool,
    pub garbage_collection_performed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report() -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayReport{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_entries();

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
    let source_denial_receipt_attached_count =
        count(&entries, |entry| entry.source_denial_receipt_attached);
    let source_recording_denial_attached_entry_count =
        count(&entries, |entry| entry.source_recording_denial_attached);
    let source_positive_preconditions_attached_entry_count = count(&entries, |entry| {
        entry.source_positive_preconditions_attached
    });
    let source_terminal_closeout_attached_entry_count =
        count(&entries, |entry| entry.source_terminal_closeout_attached);
    let source_persistence_denial_attached_entry_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_prior_denial_receipt_attached_entry_count =
        count(&entries, |entry| entry.source_prior_denial_receipt_attached);
    let source_authority_decision_record_id_attached_entry_count = count(&entries, |entry| {
        entry.source_authority_decision_record_id_attached
    });
    let local_evidence_acceptance_authority_missing_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_missing
    });
    let authority_decision_request_missing_count =
        count(&entries, |entry| entry.authority_decision_request_missing);
    let operator_authority_decision_approval_missing_count = count(&entries, |entry| {
        entry.operator_authority_decision_approval_missing
    });
    let evidence_acceptance_missing_count =
        count(&entries, |entry| entry.evidence_acceptance_missing);
    let authority_decision_recording_grant_missing_count = count(&entries, |entry| {
        entry.authority_decision_recording_grant_missing
    });
    let decision_record_schema_commit_missing_count = count(&entries, |entry| {
        entry.decision_record_schema_commit_missing
    });
    let atomic_decision_record_append_missing_count = count(&entries, |entry| {
        entry.atomic_decision_record_append_missing
    });
    let post_record_readback_missing_count =
        count(&entries, |entry| entry.post_record_readback_missing);
    let rollback_anchor_missing_count = count(&entries, |entry| entry.rollback_anchor_missing);
    let retention_policy_commit_missing_count =
        count(&entries, |entry| entry.retention_policy_commit_missing);
    let replay_idempotency_guard_missing_count =
        count(&entries, |entry| entry.replay_idempotency_guard_missing);
    let retention_policy_persisted_count =
        count(&entries, |entry| entry.retention_policy_persisted);
    let replay_index_written_count = count(&entries, |entry| entry.replay_index_written);
    let expiry_enforced_count = count(&entries, |entry| entry.expiry_enforced);
    let garbage_collection_performed_count =
        count(&entries, |entry| entry.garbage_collection_performed);
    let authority_decision_recording_allowed_count =
        count(&entries, |entry| entry.authority_decision_recording_allowed);
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let authority_decision_persisted_count =
        count(&entries, |entry| entry.authority_decision_persisted);
    let denial_receipt_persisted_count = count(&entries, |entry| entry.denial_receipt_persisted);
    let evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.evidence_acceptance_recorded);
    let evidence_recorded_count = count(&entries, |entry| entry.evidence_recorded);
    let receipt_store_write_attempt_recorded_count =
        count(&entries, |entry| entry.receipt_store_write_attempt_recorded);
    let receipt_store_written_count = count(&entries, |entry| entry.receipt_store_written);
    let receipt_persisted_count = count(&entries, |entry| entry.receipt_persisted);
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let retention_replay_readback_ready = source.recording_denial_receipt_readback_ready
        && source.denial_receipt_entry_count == 7
        && source.denial_receipt_projected_count == 7
        && source.denial_receipt_idempotency_key_unique_count == 7
        && source.authority_decision_recording_allowed_count == 0
        && source.authority_decision_recorded_count == 0
        && source.authority_decision_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
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
        && source_denial_receipt_attached_count == 7
        && source_recording_denial_attached_entry_count == 7
        && source_positive_preconditions_attached_entry_count == 7
        && source_terminal_closeout_attached_entry_count == 7
        && source_persistence_denial_attached_entry_count == 7
        && source_prior_denial_receipt_attached_entry_count == 7
        && source_authority_decision_record_id_attached_entry_count == 7
        && local_evidence_acceptance_authority_missing_count == 7
        && authority_decision_request_missing_count == 7
        && operator_authority_decision_approval_missing_count == 7
        && evidence_acceptance_missing_count == 7
        && authority_decision_recording_grant_missing_count == 7
        && decision_record_schema_commit_missing_count == 7
        && atomic_decision_record_append_missing_count == 7
        && post_record_readback_missing_count == 7
        && rollback_anchor_missing_count == 7
        && retention_policy_commit_missing_count == 7
        && replay_idempotency_guard_missing_count == 7
        && retention_policy_persisted_count == 0
        && replay_index_written_count == 0
        && expiry_enforced_count == 0
        && garbage_collection_performed_count == 0
        && authority_decision_recording_allowed_count == 0
        && authority_decision_recorded_count == 0
        && authority_decision_persisted_count == 0
        && denial_receipt_persisted_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(entry_is_ready_blocked);

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayReport {
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
        source_recording_denial_receipt_readback_ready: source
            .recording_denial_receipt_readback_ready,
        source_denial_receipt_entry_count: source.denial_receipt_entry_count,
        source_denial_receipt_projected_count: source.denial_receipt_projected_count,
        source_denial_receipt_digest_projected_count: source.denial_receipt_digest_projected_count,
        source_denial_receipt_readback_route_projected_count: source
            .denial_receipt_readback_route_projected_count,
        source_denial_receipt_idempotency_key_projected_count: source
            .denial_receipt_idempotency_key_projected_count,
        source_denial_receipt_idempotency_key_unique_count: source
            .denial_receipt_idempotency_key_unique_count,
        source_recording_denial_attached_count: source.source_recording_denial_attached_count,
        source_positive_preconditions_attached_count: source
            .source_positive_preconditions_attached_entry_count,
        source_terminal_closeout_attached_count: source.source_terminal_closeout_attached_entry_count,
        source_persistence_denial_attached_count: source.source_persistence_denial_attached_entry_count,
        source_prior_denial_receipt_attached_count: source
            .source_prior_denial_receipt_attached_entry_count,
        source_authority_decision_record_id_attached_count: source
            .source_authority_decision_record_id_attached_entry_count,
        source_authority_decision_recording_allowed_count: source
            .authority_decision_recording_allowed_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_authority_decision_persisted_count: source.authority_decision_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
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
        source_denial_receipt_attached_count,
        source_recording_denial_attached_entry_count,
        source_positive_preconditions_attached_entry_count,
        source_terminal_closeout_attached_entry_count,
        source_persistence_denial_attached_entry_count,
        source_prior_denial_receipt_attached_entry_count,
        source_authority_decision_record_id_attached_entry_count,
        local_evidence_acceptance_authority_missing_count,
        authority_decision_request_missing_count,
        operator_authority_decision_approval_missing_count,
        evidence_acceptance_missing_count,
        authority_decision_recording_grant_missing_count,
        decision_record_schema_commit_missing_count,
        atomic_decision_record_append_missing_count,
        post_record_readback_missing_count,
        rollback_anchor_missing_count,
        retention_policy_commit_missing_count,
        replay_idempotency_guard_missing_count,
        retention_policy_persisted_count,
        replay_index_written_count,
        expiry_enforced_count,
        garbage_collection_performed_count,
        authority_decision_recording_allowed_count,
        authority_decision_recorded_count,
        authority_decision_persisted_count,
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
        authority_decision_recording_allowed: false,
        authority_decision_persistence_allowed: false,
        denial_receipt_persistence_allowed: false,
        denial_receipt_persisted: false,
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
            "local_evidence_acceptance_authority_missing",
            "authority_decision_request_missing",
            "operator_authority_decision_approval_missing",
            "evidence_acceptance_missing",
            "authority_decision_recording_disabled",
            "denial_receipt_persistence_disabled",
            "retention_policy_persistence_disabled",
            "replay_index_write_disabled",
            "expiry_enforcement_disabled",
            "garbage_collection_disabled",
            "receipt_store_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplaySideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_entries(
) -> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_denial_receipt_entry_id: entry.id,
                source_denial_receipt_id: entry.denial_receipt_id,
                source_denial_receipt_route: entry.denial_receipt_route,
                source_denial_receipt_digest: entry.denial_receipt_digest,
                source_denial_receipt_schema_version: entry.denial_receipt_schema_version,
                source_denial_receipt_idempotency_key: entry.denial_receipt_idempotency_key,
                source_recording_denial_id: entry.source_recording_denial_id,
                source_recording_denial_key: entry.source_recording_denial_key,
                source_recording_denial_route: entry.source_recording_denial_route,
                source_recording_denial_reason: entry.source_recording_denial_reason,
                source_recording_denial_state: entry.source_recording_denial_state,
                source_recording_denial_digest: entry.source_recording_denial_digest,
                source_positive_preconditions_entry_id: entry.source_positive_preconditions_entry_id,
                source_positive_precondition_set_id: entry.source_positive_precondition_set_id,
                source_positive_precondition_key: entry.source_positive_precondition_key,
                source_positive_precondition_route: entry.source_positive_precondition_route,
                source_terminal_closeout_id: entry.source_terminal_closeout_id,
                source_terminal_closeout_key: entry.source_terminal_closeout_key,
                source_terminal_closeout_route: entry.source_terminal_closeout_route,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_prior_denial_receipt_id: entry.source_prior_denial_receipt_id,
                source_prior_denial_receipt_route: entry.source_prior_denial_receipt_route,
                source_prior_denial_receipt_digest: entry.source_prior_denial_receipt_digest,
                source_authority_decision_record_id: entry.source_authority_decision_record_id,
                retention_policy_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-retention-policy:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_policy_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/retention/{hyphenated}"
                ),
                expiry_guard_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-expiry-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_key: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-replay-key:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_key: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-replay-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/replay/{hyphenated}"
                ),
                retention_readback_route: format!(
                    "{RETENTION_REPLAY_COLLECTION_ROUTE}/readback/{hyphenated}"
                ),
                garbage_collection_denial_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-gc-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                supersession_guard_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-supersession-guard:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                zero_effect_digest: format!(
                    "sha256:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-retention-replay-zero-effect:{}",
                    entry.source_blocker_id
                ),
                retention_state: "projected_not_persisted",
                replay_state: "projected_not_written",
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_projected_without_persistence",
                source_denial_receipt_attached: entry.denial_receipt_projected,
                source_recording_denial_attached: entry.source_recording_denial_attached,
                source_positive_preconditions_attached: entry.source_positive_preconditions_attached,
                source_terminal_closeout_attached: entry.source_terminal_closeout_attached,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_prior_denial_receipt_attached: entry.source_prior_denial_receipt_attached,
                source_authority_decision_record_id_attached: entry
                    .source_authority_decision_record_id_attached,
                local_evidence_acceptance_authority_missing: entry
                    .local_evidence_acceptance_authority_missing,
                authority_decision_request_missing: entry.authority_decision_request_missing,
                operator_authority_decision_approval_missing: entry
                    .operator_authority_decision_approval_missing,
                evidence_acceptance_missing: entry.evidence_acceptance_missing,
                authority_decision_recording_grant_missing: entry
                    .authority_decision_recording_grant_missing,
                decision_record_schema_commit_missing: entry.decision_record_schema_commit_missing,
                atomic_decision_record_append_missing: entry.atomic_decision_record_append_missing,
                post_record_readback_missing: entry.post_record_readback_missing,
                rollback_anchor_missing: entry.rollback_anchor_missing,
                retention_policy_commit_missing: entry.retention_policy_commit_missing,
                replay_idempotency_guard_missing: entry.replay_idempotency_guard_missing,
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
                authority_decision_recording_allowed: false,
                authority_decision_recorded: false,
                authority_decision_persistence_allowed: false,
                authority_decision_persisted: false,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplayEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_projected_without_persistence"
        && entry.source_denial_receipt_attached
        && entry.source_recording_denial_attached
        && entry.source_positive_preconditions_attached
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_prior_denial_receipt_attached
        && entry.source_authority_decision_record_id_attached
        && entry.local_evidence_acceptance_authority_missing
        && entry.authority_decision_request_missing
        && entry.operator_authority_decision_approval_missing
        && entry.evidence_acceptance_missing
        && entry.authority_decision_recording_grant_missing
        && entry.decision_record_schema_commit_missing
        && entry.atomic_decision_record_append_missing
        && entry.post_record_readback_missing
        && entry.rollback_anchor_missing
        && entry.retention_policy_commit_missing
        && entry.replay_idempotency_guard_missing
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
        && !entry.authority_decision_recording_allowed
        && !entry.authority_decision_recorded
        && !entry.authority_decision_persistence_allowed
        && !entry.authority_decision_persisted
        && !entry.denial_receipt_persistence_allowed
        && !entry.denial_receipt_persisted
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplaySideEffects {
    pub const fn none() -> Self {
        Self {
            retention_policy_persisted: false,
            replay_index_written: false,
            expiry_enforced: false,
            garbage_collection_performed: false,
            authority_decision_recorded: false,
            authority_decision_persisted: false,
            denial_receipt_persisted: false,
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
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_projects_all_entries()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.retention_replay_readback_ready);
        assert_eq!(report.source_denial_receipt_entry_count, 7);
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
        assert_eq!(report.source_recording_denial_attached_entry_count, 7);
        assert_eq!(report.source_positive_preconditions_attached_entry_count, 7);
        assert_eq!(report.source_terminal_closeout_attached_entry_count, 7);
        assert_eq!(report.source_persistence_denial_attached_entry_count, 7);
        assert_eq!(report.source_prior_denial_receipt_attached_entry_count, 7);
        assert_eq!(
            report.source_authority_decision_record_id_attached_entry_count,
            7
        );
    }

    #[test]
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_keeps_persistence_closed()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert_eq!(report.local_evidence_acceptance_authority_missing_count, 7);
        assert_eq!(report.authority_decision_request_missing_count, 7);
        assert_eq!(report.operator_authority_decision_approval_missing_count, 7);
        assert_eq!(report.evidence_acceptance_missing_count, 7);
        assert_eq!(report.authority_decision_recording_grant_missing_count, 7);
        assert_eq!(report.decision_record_schema_commit_missing_count, 7);
        assert_eq!(report.atomic_decision_record_append_missing_count, 7);
        assert_eq!(report.post_record_readback_missing_count, 7);
        assert_eq!(report.rollback_anchor_missing_count, 7);
        assert_eq!(report.retention_policy_commit_missing_count, 7);
        assert_eq!(report.replay_idempotency_guard_missing_count, 7);
        assert_eq!(report.retention_policy_persisted_count, 0);
        assert_eq!(report.replay_index_written_count, 0);
        assert_eq!(report.expiry_enforced_count, 0);
        assert_eq!(report.garbage_collection_performed_count, 0);
        assert_eq!(report.authority_decision_recording_allowed_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.retention_policy_persistence_allowed);
        assert!(!report.replay_index_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptRetentionReplaySideEffects::none()
        );
    }

    #[test]
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_entries_are_stable()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.retention_readback_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/retention-replay/readback/dirty-worktree-boundary"
                && entry.replay_idempotency_key
                    == "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-replay-idempotency:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_without_persistence_")
                && entry.source_denial_receipt_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_without_persistence_")
                && entry.source_denial_receipt_id.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:")
                && entry.source_recording_denial_id.starts_with("local-evidence-acceptance-authority-decision-recording-denial:")
                && entry.source_positive_precondition_set_id.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions:")
                && entry.retention_policy_id.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-retention-policy:")
                && entry.replay_key.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-replay-key:")
                && entry.zero_effect_digest.starts_with("sha256:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-retention-replay-zero-effect:")
                && entry.retention_state == "projected_not_persisted"
                && entry.replay_state == "projected_not_written"
                && entry_is_ready_blocked(entry)
        }));
    }
}
