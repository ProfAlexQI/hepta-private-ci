use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_terminal_no_persistence::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_v1";
const POSITIVE_PRECONDITIONS_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/positive-preconditions";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_terminal_no_persistence_readback_ready: bool,
    pub source_terminal_entry_count: usize,
    pub source_terminal_closeout_projected_count: usize,
    pub source_terminal_no_persistence_confirmed_count: usize,
    pub source_terminal_closeout_key_unique_count: usize,
    pub source_terminal_closeout_recorded_count: usize,
    pub source_terminal_closeout_persisted_count: usize,
    pub source_terminal_closeout_accepted_count: usize,
    pub source_terminal_closeout_authoritative_count: usize,
    pub source_denial_receipt_persistence_attempt_recorded_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_authority_decision_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub positive_preconditions_route: &'static str,
    pub precondition_entry_count: usize,
    pub positive_precondition_set_projected_count: usize,
    pub positive_precondition_key_projected_count: usize,
    pub positive_precondition_key_unique_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_denial_receipt_attached_count: usize,
    pub source_authority_decision_record_id_attached_count: usize,
    pub local_evidence_acceptance_authority_required_count: usize,
    pub local_evidence_acceptance_authority_present_count: usize,
    pub authority_decision_request_required_count: usize,
    pub authority_decision_request_present_count: usize,
    pub operator_authority_decision_approval_required_count: usize,
    pub operator_authority_decision_approval_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub authority_decision_recording_grant_required_count: usize,
    pub authority_decision_recording_grant_present_count: usize,
    pub decision_record_schema_commit_required_count: usize,
    pub decision_record_schema_committed_count: usize,
    pub atomic_decision_record_append_required_count: usize,
    pub atomic_decision_record_append_enabled_count: usize,
    pub post_record_readback_required_count: usize,
    pub post_record_readback_persisted_count: usize,
    pub rollback_anchor_required_count: usize,
    pub rollback_anchor_verified_count: usize,
    pub retention_policy_commit_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub replay_idempotency_guard_required_count: usize,
    pub replay_idempotency_guard_enabled_count: usize,
    pub positive_preconditions_missing_count: usize,
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
    pub positive_preconditions_readback_ready: bool,
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_persistence_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
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
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_terminal_no_persistence_entry_id: String,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_key: String,
    pub source_terminal_closeout_route: String,
    pub source_terminal_reason: &'static str,
    pub source_terminal_state: &'static str,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_persistence_denial_reason: &'static str,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_authority_decision_record_id: String,
    pub positive_precondition_set_id: String,
    pub positive_precondition_key: String,
    pub positive_precondition_route: String,
    pub local_evidence_acceptance_authority_precondition_id: String,
    pub authority_decision_request_precondition_id: String,
    pub operator_authority_decision_approval_precondition_id: String,
    pub evidence_acceptance_precondition_id: String,
    pub authority_decision_recording_grant_precondition_id: String,
    pub decision_record_schema_commit_precondition_id: String,
    pub atomic_decision_record_append_precondition_id: String,
    pub post_record_readback_precondition_id: String,
    pub rollback_anchor_precondition_id: String,
    pub retention_policy_commit_precondition_id: String,
    pub replay_idempotency_guard_precondition_id: String,
    pub observed_state: &'static str,
    pub positive_precondition_set_projected: bool,
    pub positive_precondition_key_projected: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_denial_receipt_binding_attached: bool,
    pub source_authority_decision_record_id_attached: bool,
    pub local_evidence_acceptance_authority_required: bool,
    pub local_evidence_acceptance_authority_present: bool,
    pub authority_decision_request_required: bool,
    pub authority_decision_request_present: bool,
    pub operator_authority_decision_approval_required: bool,
    pub operator_authority_decision_approval_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub authority_decision_recording_grant_required: bool,
    pub authority_decision_recording_grant_present: bool,
    pub decision_record_schema_commit_required: bool,
    pub decision_record_schema_committed: bool,
    pub atomic_decision_record_append_required: bool,
    pub atomic_decision_record_append_enabled: bool,
    pub post_record_readback_required: bool,
    pub post_record_readback_persisted: bool,
    pub rollback_anchor_required: bool,
    pub rollback_anchor_verified: bool,
    pub retention_policy_commit_required: bool,
    pub retention_policy_committed: bool,
    pub replay_idempotency_guard_required: bool,
    pub replay_idempotency_guard_enabled: bool,
    pub positive_preconditions_missing: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsSideEffects
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report() -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsReport{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_entries();

    let positive_precondition_set_projected_count =
        count(&entries, |entry| entry.positive_precondition_set_projected);
    let positive_precondition_key_projected_count =
        count(&entries, |entry| entry.positive_precondition_key_projected);
    let positive_precondition_key_unique_count = entries
        .iter()
        .map(|entry| entry.positive_precondition_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let source_terminal_closeout_attached_count =
        count(&entries, |entry| entry.source_terminal_closeout_attached);
    let source_persistence_denial_attached_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_denial_receipt_attached_count = count(&entries, |entry| {
        entry.source_denial_receipt_binding_attached
    });
    let source_authority_decision_record_id_attached_count = count(&entries, |entry| {
        entry.source_authority_decision_record_id_attached
    });
    let local_evidence_acceptance_authority_required_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_required
    });
    let local_evidence_acceptance_authority_present_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_present
    });
    let authority_decision_request_required_count =
        count(&entries, |entry| entry.authority_decision_request_required);
    let authority_decision_request_present_count =
        count(&entries, |entry| entry.authority_decision_request_present);
    let operator_authority_decision_approval_required_count = count(&entries, |entry| {
        entry.operator_authority_decision_approval_required
    });
    let operator_authority_decision_approval_present_count = count(&entries, |entry| {
        entry.operator_authority_decision_approval_present
    });
    let evidence_acceptance_required_count =
        count(&entries, |entry| entry.evidence_acceptance_required);
    let evidence_acceptance_present_count =
        count(&entries, |entry| entry.evidence_acceptance_present);
    let authority_decision_recording_grant_required_count = count(&entries, |entry| {
        entry.authority_decision_recording_grant_required
    });
    let authority_decision_recording_grant_present_count = count(&entries, |entry| {
        entry.authority_decision_recording_grant_present
    });
    let decision_record_schema_commit_required_count = count(&entries, |entry| {
        entry.decision_record_schema_commit_required
    });
    let decision_record_schema_committed_count =
        count(&entries, |entry| entry.decision_record_schema_committed);
    let atomic_decision_record_append_required_count = count(&entries, |entry| {
        entry.atomic_decision_record_append_required
    });
    let atomic_decision_record_append_enabled_count = count(&entries, |entry| {
        entry.atomic_decision_record_append_enabled
    });
    let post_record_readback_required_count =
        count(&entries, |entry| entry.post_record_readback_required);
    let post_record_readback_persisted_count =
        count(&entries, |entry| entry.post_record_readback_persisted);
    let rollback_anchor_required_count = count(&entries, |entry| entry.rollback_anchor_required);
    let rollback_anchor_verified_count = count(&entries, |entry| entry.rollback_anchor_verified);
    let retention_policy_commit_required_count =
        count(&entries, |entry| entry.retention_policy_commit_required);
    let retention_policy_committed_count =
        count(&entries, |entry| entry.retention_policy_committed);
    let replay_idempotency_guard_required_count =
        count(&entries, |entry| entry.replay_idempotency_guard_required);
    let replay_idempotency_guard_enabled_count =
        count(&entries, |entry| entry.replay_idempotency_guard_enabled);
    let positive_preconditions_missing_count =
        count(&entries, |entry| entry.positive_preconditions_missing);
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

    let positive_preconditions_readback_ready = source.terminal_no_persistence_readback_ready
        && source.terminal_entry_count == 7
        && source.terminal_closeout_projected_count == 7
        && source.terminal_no_persistence_confirmed_count == 7
        && source.terminal_closeout_key_unique_count == 7
        && source.terminal_closeout_recorded_count == 0
        && source.terminal_closeout_persisted_count == 0
        && source.terminal_closeout_accepted_count == 0
        && source.terminal_closeout_authoritative_count == 0
        && source.denial_receipt_persistence_attempt_recorded_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.authority_decision_recorded_count == 0
        && source.authority_decision_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && positive_precondition_set_projected_count == 7
        && positive_precondition_key_projected_count == 7
        && positive_precondition_key_unique_count == 7
        && source_terminal_closeout_attached_count == 7
        && source_persistence_denial_attached_count == 7
        && source_denial_receipt_attached_count == 7
        && source_authority_decision_record_id_attached_count == 7
        && local_evidence_acceptance_authority_required_count == 7
        && local_evidence_acceptance_authority_present_count == 0
        && authority_decision_request_required_count == 7
        && authority_decision_request_present_count == 0
        && operator_authority_decision_approval_required_count == 7
        && operator_authority_decision_approval_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && authority_decision_recording_grant_required_count == 7
        && authority_decision_recording_grant_present_count == 0
        && decision_record_schema_commit_required_count == 7
        && decision_record_schema_committed_count == 0
        && atomic_decision_record_append_required_count == 7
        && atomic_decision_record_append_enabled_count == 0
        && post_record_readback_required_count == 7
        && post_record_readback_persisted_count == 0
        && rollback_anchor_required_count == 7
        && rollback_anchor_verified_count == 0
        && retention_policy_commit_required_count == 7
        && retention_policy_committed_count == 0
        && replay_idempotency_guard_required_count == 7
        && replay_idempotency_guard_enabled_count == 0
        && positive_preconditions_missing_count == 7
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if positive_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_terminal_no_persistence_readback_ready: source
            .terminal_no_persistence_readback_ready,
        source_terminal_entry_count: source.terminal_entry_count,
        source_terminal_closeout_projected_count: source.terminal_closeout_projected_count,
        source_terminal_no_persistence_confirmed_count: source
            .terminal_no_persistence_confirmed_count,
        source_terminal_closeout_key_unique_count: source.terminal_closeout_key_unique_count,
        source_terminal_closeout_recorded_count: source.terminal_closeout_recorded_count,
        source_terminal_closeout_persisted_count: source.terminal_closeout_persisted_count,
        source_terminal_closeout_accepted_count: source.terminal_closeout_accepted_count,
        source_terminal_closeout_authoritative_count: source
            .terminal_closeout_authoritative_count,
        source_denial_receipt_persistence_attempt_recorded_count: source
            .denial_receipt_persistence_attempt_recorded_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_authority_decision_persisted_count: source.authority_decision_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count:
            source.receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        positive_preconditions_route: POSITIVE_PRECONDITIONS_ROUTE,
        precondition_entry_count: entries.len(),
        positive_precondition_set_projected_count,
        positive_precondition_key_projected_count,
        positive_precondition_key_unique_count,
        source_terminal_closeout_attached_count,
        source_persistence_denial_attached_count,
        source_denial_receipt_attached_count,
        source_authority_decision_record_id_attached_count,
        local_evidence_acceptance_authority_required_count,
        local_evidence_acceptance_authority_present_count,
        authority_decision_request_required_count,
        authority_decision_request_present_count,
        operator_authority_decision_approval_required_count,
        operator_authority_decision_approval_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        authority_decision_recording_grant_required_count,
        authority_decision_recording_grant_present_count,
        decision_record_schema_commit_required_count,
        decision_record_schema_committed_count,
        atomic_decision_record_append_required_count,
        atomic_decision_record_append_enabled_count,
        post_record_readback_required_count,
        post_record_readback_persisted_count,
        rollback_anchor_required_count,
        rollback_anchor_verified_count,
        retention_policy_commit_required_count,
        retention_policy_committed_count,
        replay_idempotency_guard_required_count,
        replay_idempotency_guard_enabled_count,
        positive_preconditions_missing_count,
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
        positive_preconditions_readback_ready,
        authority_decision_recording_allowed: false,
        authority_decision_persistence_allowed: false,
        denial_receipt_persistence_allowed: false,
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_entries(
) -> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_without_recording_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_terminal_no_persistence_entry_id: entry.id,
                source_terminal_closeout_id: entry.terminal_closeout_id,
                source_terminal_closeout_key: entry.terminal_closeout_key,
                source_terminal_closeout_route: entry.terminal_closeout_route,
                source_terminal_reason: entry.terminal_reason,
                source_terminal_state: entry.terminal_state,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_persistence_denial_reason: entry.source_persistence_denial_reason,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_authority_decision_record_id: entry.source_authority_decision_record_id,
                positive_precondition_set_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_key: format!(
                    "authority-decision-recording-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_route: format!("{POSITIVE_PRECONDITIONS_ROUTE}/{hyphenated}"),
                local_evidence_acceptance_authority_precondition_id: format!(
                    "local-evidence-acceptance-authority-required-for-authority-decision-recording:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                authority_decision_request_precondition_id: format!(
                    "authority-decision-request-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_authority_decision_approval_precondition_id: format!(
                    "operator-authority-decision-approval-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                evidence_acceptance_precondition_id: format!(
                    "evidence-acceptance-required-for-authority-decision-recording:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                authority_decision_recording_grant_precondition_id: format!(
                    "authority-decision-recording-grant-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                decision_record_schema_commit_precondition_id: format!(
                    "authority-decision-record-schema-commit-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                atomic_decision_record_append_precondition_id: format!(
                    "atomic-authority-decision-record-append-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_record_readback_precondition_id: format!(
                    "post-authority-decision-record-readback-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                rollback_anchor_precondition_id: format!(
                    "authority-decision-recording-rollback-anchor-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_policy_commit_precondition_id: format!(
                    "authority-decision-recording-retention-policy-commit-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_guard_precondition_id: format!(
                    "authority-decision-recording-replay-idempotency-guard-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_positive_preconditions_projected_without_recording",
                positive_precondition_set_projected: true,
                positive_precondition_key_projected: true,
                source_terminal_closeout_attached: entry.terminal_closeout_projected,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_denial_receipt_binding_attached: entry
                    .source_denial_receipt_binding_attached,
                source_authority_decision_record_id_attached: entry
                    .source_authority_decision_record_id_attached,
                local_evidence_acceptance_authority_required: true,
                local_evidence_acceptance_authority_present: false,
                authority_decision_request_required: true,
                authority_decision_request_present: false,
                operator_authority_decision_approval_required: true,
                operator_authority_decision_approval_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                authority_decision_recording_grant_required: true,
                authority_decision_recording_grant_present: false,
                decision_record_schema_commit_required: true,
                decision_record_schema_committed: false,
                atomic_decision_record_append_required: true,
                atomic_decision_record_append_enabled: false,
                post_record_readback_required: true,
                post_record_readback_persisted: false,
                rollback_anchor_required: true,
                rollback_anchor_verified: false,
                retention_policy_commit_required: true,
                retention_policy_committed: false,
                replay_idempotency_guard_required: true,
                replay_idempotency_guard_enabled: false,
                positive_preconditions_missing: true,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_projected_without_recording"
        && entry.positive_precondition_set_projected
        && entry.positive_precondition_key_projected
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_denial_receipt_binding_attached
        && entry.source_authority_decision_record_id_attached
        && entry.local_evidence_acceptance_authority_required
        && !entry.local_evidence_acceptance_authority_present
        && entry.authority_decision_request_required
        && !entry.authority_decision_request_present
        && entry.operator_authority_decision_approval_required
        && !entry.operator_authority_decision_approval_present
        && entry.evidence_acceptance_required
        && !entry.evidence_acceptance_present
        && entry.authority_decision_recording_grant_required
        && !entry.authority_decision_recording_grant_present
        && entry.decision_record_schema_commit_required
        && !entry.decision_record_schema_committed
        && entry.atomic_decision_record_append_required
        && !entry.atomic_decision_record_append_enabled
        && entry.post_record_readback_required
        && !entry.post_record_readback_persisted
        && entry.rollback_anchor_required
        && !entry.rollback_anchor_verified
        && entry.retention_policy_commit_required
        && !entry.retention_policy_committed
        && entry.replay_idempotency_guard_required
        && !entry.replay_idempotency_guard_enabled
        && entry.positive_preconditions_missing
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsSideEffects {
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
    fn authority_decision_recording_positive_preconditions_project_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.positive_preconditions_readback_ready);
        assert_eq!(report.source_terminal_entry_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.positive_precondition_set_projected_count, 7);
        assert_eq!(report.positive_precondition_key_projected_count, 7);
        assert_eq!(report.positive_precondition_key_unique_count, 7);
        assert_eq!(report.source_terminal_closeout_attached_count, 7);
        assert_eq!(report.source_persistence_denial_attached_count, 7);
        assert_eq!(report.source_denial_receipt_attached_count, 7);
        assert_eq!(report.source_authority_decision_record_id_attached_count, 7);
        assert_eq!(report.local_evidence_acceptance_authority_required_count, 7);
        assert_eq!(report.authority_decision_request_required_count, 7);
        assert_eq!(
            report.operator_authority_decision_approval_required_count,
            7
        );
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.positive_preconditions_missing_count, 7);
    }

    #[test]
    fn authority_decision_recording_positive_preconditions_keep_recording_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_report();

        assert_eq!(report.local_evidence_acceptance_authority_present_count, 0);
        assert_eq!(report.authority_decision_request_present_count, 0);
        assert_eq!(report.operator_authority_decision_approval_present_count, 0);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.authority_decision_recording_grant_present_count, 0);
        assert_eq!(report.decision_record_schema_committed_count, 0);
        assert_eq!(report.atomic_decision_record_append_enabled_count, 0);
        assert_eq!(report.post_record_readback_persisted_count, 0);
        assert_eq!(report.rollback_anchor_verified_count, 0);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert_eq!(report.replay_idempotency_guard_enabled_count, 0);
        assert_eq!(report.authority_decision_recording_allowed_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.authority_decision_recording_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingPositivePreconditionsSideEffects::none()
        );
    }

    #[test]
    fn authority_decision_recording_positive_precondition_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.positive_precondition_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/positive-preconditions/dirty-worktree-boundary"
                && entry.source_authority_decision_record_id
                    == "local-evidence-acceptance-authority-decision-record:controlled-live-evidence-receipt-store:dirty_worktree_boundary:not-recorded"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_without_recording_")
                && entry.source_terminal_no_persistence_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_terminal_no_persistence_")
                && entry.source_persistence_denial_id.starts_with("local-evidence-acceptance-authority-decision-recording-denial-receipt-persistence-denial:")
                && entry.source_denial_receipt_id.starts_with("local-evidence-acceptance-authority-decision-recording-denial-receipt:")
                && entry.positive_precondition_set_id.starts_with("local-evidence-acceptance-authority-decision-recording-positive-preconditions:")
                && entry_is_ready_blocked(entry)
        }));
    }
}
