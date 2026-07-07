use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_retention_replay_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_v1";
const DENIAL_RECEIPT_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipts";
const DENIAL_RECEIPT_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts";
const DENIAL_RECEIPT_SCHEMA_VERSION: &str = "controlled_live_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_v1";
const RECORDING_DENIAL_STATE: &str = "denied_not_recorded";
const DENIAL_RECEIPT_STATE: &str = "denial_receipt_projected_not_persisted";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_recording_denial_readback_ready: bool,
    pub source_recording_denial_entry_count: usize,
    pub source_recording_denial_projected_count: usize,
    pub source_recording_denial_key_unique_count: usize,
    pub source_positive_preconditions_attached_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_denial_receipt_attached_count: usize,
    pub source_authority_decision_record_id_attached_count: usize,
    pub source_authority_decision_recording_allowed_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_authority_decision_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub denial_receipt_collection_id: &'static str,
    pub denial_receipt_collection_route: &'static str,
    pub denial_receipt_schema_version: &'static str,
    pub denial_receipt_entry_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_digest_projected_count: usize,
    pub denial_receipt_readback_route_projected_count: usize,
    pub denial_receipt_idempotency_key_projected_count: usize,
    pub denial_receipt_idempotency_key_unique_count: usize,
    pub source_recording_denial_attached_count: usize,
    pub source_positive_preconditions_attached_entry_count: usize,
    pub source_terminal_closeout_attached_entry_count: usize,
    pub source_persistence_denial_attached_entry_count: usize,
    pub source_prior_denial_receipt_attached_entry_count: usize,
    pub source_authority_decision_record_id_attached_entry_count: usize,
    pub recording_denial_reason_projected_count: usize,
    pub recording_denial_state_projected_count: usize,
    pub denial_receipt_state_projected_count: usize,
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
    pub recording_denial_receipt_readback_ready: bool,
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
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_recording_denial_entry_id: String,
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
    pub denial_receipt_id: String,
    pub denial_receipt_route: String,
    pub denial_receipt_digest: String,
    pub denial_receipt_schema_version: &'static str,
    pub denial_receipt_idempotency_key: String,
    pub recording_denial_reason: &'static str,
    pub recording_denial_state: &'static str,
    pub denial_receipt_state: &'static str,
    pub observed_state: &'static str,
    pub denial_receipt_projected: bool,
    pub denial_receipt_digest_projected: bool,
    pub denial_receipt_readback_route_projected: bool,
    pub denial_receipt_idempotency_key_projected: bool,
    pub source_recording_denial_attached: bool,
    pub source_positive_preconditions_attached: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_prior_denial_receipt_attached: bool,
    pub source_authority_decision_record_id_attached: bool,
    pub recording_denial_reason_projected: bool,
    pub recording_denial_state_projected: bool,
    pub denial_receipt_state_projected: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptSideEffects
{
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report() -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptReport{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_entries();

    let denial_receipt_projected_count = count(&entries, |entry| entry.denial_receipt_projected);
    let denial_receipt_digest_projected_count =
        count(&entries, |entry| entry.denial_receipt_digest_projected);
    let denial_receipt_readback_route_projected_count = count(&entries, |entry| {
        entry.denial_receipt_readback_route_projected
    });
    let denial_receipt_idempotency_key_projected_count = count(&entries, |entry| {
        entry.denial_receipt_idempotency_key_projected
    });
    let denial_receipt_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.denial_receipt_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let source_recording_denial_attached_count =
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
    let recording_denial_reason_projected_count =
        count(&entries, |entry| entry.recording_denial_reason_projected);
    let recording_denial_state_projected_count =
        count(&entries, |entry| entry.recording_denial_state_projected);
    let denial_receipt_state_projected_count =
        count(&entries, |entry| entry.denial_receipt_state_projected);
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

    let recording_denial_receipt_readback_ready = source.recording_denial_readback_ready
        && source.recording_denial_entry_count == 7
        && source.recording_denial_projected_count == 7
        && source.recording_denial_key_unique_count == 7
        && source.source_positive_preconditions_attached_count == 7
        && source.source_terminal_closeout_attached_entry_count == 7
        && source.source_persistence_denial_attached_entry_count == 7
        && source.source_denial_receipt_attached_entry_count == 7
        && source.source_authority_decision_record_id_attached_entry_count == 7
        && source.authority_decision_recording_allowed_count == 0
        && source.authority_decision_recorded_count == 0
        && source.authority_decision_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && denial_receipt_projected_count == 7
        && denial_receipt_digest_projected_count == 7
        && denial_receipt_readback_route_projected_count == 7
        && denial_receipt_idempotency_key_projected_count == 7
        && denial_receipt_idempotency_key_unique_count == 7
        && source_recording_denial_attached_count == 7
        && source_positive_preconditions_attached_entry_count == 7
        && source_terminal_closeout_attached_entry_count == 7
        && source_persistence_denial_attached_entry_count == 7
        && source_prior_denial_receipt_attached_entry_count == 7
        && source_authority_decision_record_id_attached_entry_count == 7
        && recording_denial_reason_projected_count == 7
        && recording_denial_state_projected_count == 7
        && denial_receipt_state_projected_count == 7
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
        && authority_decision_recording_allowed_count == 0
        && authority_decision_recorded_count == 0
        && authority_decision_persisted_count == 0
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
        && entries.iter().all(entry_is_ready_blocked);

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if recording_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_recording_denial_readback_ready: source.recording_denial_readback_ready,
        source_recording_denial_entry_count: source.recording_denial_entry_count,
        source_recording_denial_projected_count: source.recording_denial_projected_count,
        source_recording_denial_key_unique_count: source.recording_denial_key_unique_count,
        source_positive_preconditions_attached_count: source.source_positive_preconditions_attached_count,
        source_terminal_closeout_attached_count: source.source_terminal_closeout_attached_entry_count,
        source_persistence_denial_attached_count: source.source_persistence_denial_attached_entry_count,
        source_denial_receipt_attached_count: source.source_denial_receipt_attached_entry_count,
        source_authority_decision_record_id_attached_count: source.source_authority_decision_record_id_attached_entry_count,
        source_authority_decision_recording_allowed_count: source.authority_decision_recording_allowed_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_authority_decision_persisted_count: source.authority_decision_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        denial_receipt_collection_id: DENIAL_RECEIPT_COLLECTION_ID,
        denial_receipt_collection_route: DENIAL_RECEIPT_COLLECTION_ROUTE,
        denial_receipt_schema_version: DENIAL_RECEIPT_SCHEMA_VERSION,
        denial_receipt_entry_count: entries.len(),
        denial_receipt_projected_count,
        denial_receipt_digest_projected_count,
        denial_receipt_readback_route_projected_count,
        denial_receipt_idempotency_key_projected_count,
        denial_receipt_idempotency_key_unique_count,
        source_recording_denial_attached_count,
        source_positive_preconditions_attached_entry_count,
        source_terminal_closeout_attached_entry_count,
        source_persistence_denial_attached_entry_count,
        source_prior_denial_receipt_attached_entry_count,
        source_authority_decision_record_id_attached_entry_count,
        recording_denial_reason_projected_count,
        recording_denial_state_projected_count,
        denial_receipt_state_projected_count,
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
        recording_denial_receipt_readback_ready,
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
            "authority_decision_recording_grant_missing",
            "decision_record_schema_not_committed",
            "atomic_decision_record_append_not_enabled",
            "post_record_readback_missing",
            "rollback_anchor_missing",
            "retention_policy_not_committed",
            "replay_idempotency_guard_disabled",
            "authority_decision_recording_disabled",
            "authority_decision_persistence_disabled",
            "denial_receipt_persistence_disabled",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_entries(
) -> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_recording_denial_entry_id: entry.id,
                source_recording_denial_id: entry.recording_denial_id,
                source_recording_denial_key: entry.recording_denial_key,
                source_recording_denial_route: entry.recording_denial_route,
                source_recording_denial_reason: entry.recording_denial_reason,
                source_recording_denial_state: entry.recording_denial_state,
                source_recording_denial_digest: entry.recording_denial_digest,
                source_positive_preconditions_entry_id: entry.source_positive_preconditions_entry_id,
                source_positive_precondition_set_id: entry.source_positive_precondition_set_id,
                source_positive_precondition_key: entry.source_positive_precondition_key,
                source_positive_precondition_route: entry.source_positive_precondition_route,
                source_terminal_closeout_id: entry.source_terminal_closeout_id,
                source_terminal_closeout_key: entry.source_terminal_closeout_key,
                source_terminal_closeout_route: entry.source_terminal_closeout_route,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_prior_denial_receipt_id: entry.source_denial_receipt_id,
                source_prior_denial_receipt_route: entry.source_denial_receipt_route,
                source_prior_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_authority_decision_record_id: entry.source_authority_decision_record_id,
                denial_receipt_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:controlled-live-evidence-receipt-store:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_route: format!("{DENIAL_RECEIPT_COLLECTION_ROUTE}/{hyphenated}"),
                denial_receipt_digest: format!(
                    "sha256:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_schema_version: DENIAL_RECEIPT_SCHEMA_VERSION,
                denial_receipt_idempotency_key: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                recording_denial_reason: entry.recording_denial_reason,
                recording_denial_state: RECORDING_DENIAL_STATE,
                denial_receipt_state: DENIAL_RECEIPT_STATE,
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_projected_without_persistence",
                denial_receipt_projected: true,
                denial_receipt_digest_projected: true,
                denial_receipt_readback_route_projected: true,
                denial_receipt_idempotency_key_projected: true,
                source_recording_denial_attached: entry.recording_denial_projected,
                source_positive_preconditions_attached: entry.source_positive_preconditions_attached,
                source_terminal_closeout_attached: entry.source_terminal_closeout_attached,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_prior_denial_receipt_attached: entry.source_denial_receipt_binding_attached,
                source_authority_decision_record_id_attached: entry.source_authority_decision_record_id_attached,
                recording_denial_reason_projected: entry.recording_denial_reason_projected,
                recording_denial_state_projected: entry.recording_denial_state_projected,
                denial_receipt_state_projected: true,
                local_evidence_acceptance_authority_missing: entry.local_evidence_acceptance_authority_missing,
                authority_decision_request_missing: entry.authority_decision_request_missing,
                operator_authority_decision_approval_missing: entry.operator_authority_decision_approval_missing,
                evidence_acceptance_missing: entry.evidence_acceptance_missing,
                authority_decision_recording_grant_missing: entry.authority_decision_recording_grant_missing,
                decision_record_schema_commit_missing: entry.decision_record_schema_commit_missing,
                atomic_decision_record_append_missing: entry.atomic_decision_record_append_missing,
                post_record_readback_missing: entry.post_record_readback_missing,
                rollback_anchor_missing: entry.rollback_anchor_missing,
                retention_policy_commit_missing: entry.retention_policy_commit_missing,
                replay_idempotency_guard_missing: entry.replay_idempotency_guard_missing,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_projected_without_persistence"
        && entry.denial_receipt_projected
        && entry.denial_receipt_digest_projected
        && entry.denial_receipt_readback_route_projected
        && entry.denial_receipt_idempotency_key_projected
        && entry.source_recording_denial_attached
        && entry.source_positive_preconditions_attached
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_prior_denial_receipt_attached
        && entry.source_authority_decision_record_id_attached
        && entry.recording_denial_reason_projected
        && entry.recording_denial_state_projected
        && entry.denial_receipt_state_projected
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptSideEffects {
    pub const fn none() -> Self {
        Self {
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
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_projects_all_entries()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.recording_denial_receipt_readback_ready);
        assert_eq!(report.source_recording_denial_entry_count, 7);
        assert_eq!(report.denial_receipt_entry_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_digest_projected_count, 7);
        assert_eq!(report.denial_receipt_readback_route_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_unique_count, 7);
        assert_eq!(report.source_recording_denial_attached_count, 7);
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
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_keeps_persistence_closed()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report();

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
        assert_eq!(report.authority_decision_recording_allowed_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsRecordingDenialReceiptSideEffects::none()
        );
    }

    #[test]
    fn authority_decision_recording_positive_preconditions_recording_denial_receipt_entries_are_stable()
     {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.denial_receipt_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/dirty-worktree-boundary"
                && entry.denial_receipt_id
                    == "local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:controlled-live-evidence-receipt-store:dirty_worktree_boundary:not-persisted"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_without_persistence_")
                && entry.source_recording_denial_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_without_recording_")
                && entry.source_recording_denial_id.starts_with("local-evidence-acceptance-authority-decision-recording-denial:")
                && entry.source_positive_precondition_set_id.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions:")
                && entry.denial_receipt_idempotency_key.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-idempotency:")
                && entry.recording_denial_state == RECORDING_DENIAL_STATE
                && entry.denial_receipt_state == DENIAL_RECEIPT_STATE
                && entry_is_ready_blocked(entry)
        }));
    }
}
