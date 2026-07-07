use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_v1";
const POSITIVE_PRECONDITIONS_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/positive-preconditions";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsReport {
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
    pub source_acceptance_source_recorded_count: usize,
    pub source_acceptance_source_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_live_execution_allowed: bool,
    pub positive_preconditions_route: &'static str,
    pub precondition_entry_count: usize,
    pub positive_precondition_set_projected_count: usize,
    pub source_retention_replay_attached_count: usize,
    pub source_denial_receipt_attached_count: usize,
    pub source_acceptance_source_record_attached_count: usize,
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
    pub acceptance_source_recorded_count: usize,
    pub acceptance_source_persisted_count: usize,
    pub denial_receipt_persistence_allowed_count: usize,
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
    pub acceptance_source_recording_allowed: bool,
    pub acceptance_source_persistence_allowed: bool,
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
    pub entries:
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_retention_replay_entry_id: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_acceptance_source_record_id: String,
    pub source_retention_policy_id: String,
    pub source_replay_idempotency_key: String,
    pub positive_precondition_set_id: String,
    pub positive_precondition_route: String,
    pub denial_receipt_persistence_grant_precondition_id: String,
    pub observed_state: &'static str,
    pub positive_precondition_set_projected: bool,
    pub source_retention_replay_attached: bool,
    pub source_denial_receipt_attached: bool,
    pub source_acceptance_source_record_attached: bool,
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
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_store_written: bool,
    pub receipt_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsSideEffects
{
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report() -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsReport{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_entries();

    let positive_precondition_set_projected_count =
        count(&entries, |entry| entry.positive_precondition_set_projected);
    let source_retention_replay_attached_count =
        count(&entries, |entry| entry.source_retention_replay_attached);
    let source_denial_receipt_attached_count =
        count(&entries, |entry| entry.source_denial_receipt_attached);
    let source_acceptance_source_record_attached_count = count(&entries, |entry| {
        entry.source_acceptance_source_record_attached
    });
    let persistence_authority_required_count =
        count(&entries, |entry| entry.persistence_authority_required);
    let persistence_authority_present_count =
        count(&entries, |entry| entry.persistence_authority_present);
    let operator_persistence_approval_required_count = count(&entries, |entry| {
        entry.operator_persistence_approval_required
    });
    let operator_persistence_approval_present_count = count(&entries, |entry| {
        entry.operator_persistence_approval_present
    });
    let evidence_acceptance_required_count =
        count(&entries, |entry| entry.evidence_acceptance_required);
    let evidence_acceptance_present_count =
        count(&entries, |entry| entry.evidence_acceptance_present);
    let denial_receipt_persistence_grant_required_count = count(&entries, |entry| {
        entry.denial_receipt_persistence_grant_required
    });
    let denial_receipt_persistence_grant_present_count = count(&entries, |entry| {
        entry.denial_receipt_persistence_grant_present
    });
    let atomic_append_required_count = count(&entries, |entry| entry.atomic_append_required);
    let atomic_append_enabled_count = count(&entries, |entry| entry.atomic_append_enabled);
    let post_persist_readback_required_count =
        count(&entries, |entry| entry.post_persist_readback_required);
    let post_persist_readback_persisted_count =
        count(&entries, |entry| entry.post_persist_readback_persisted);
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
    let acceptance_source_recorded_count =
        count(&entries, |entry| entry.acceptance_source_recorded);
    let acceptance_source_persisted_count =
        count(&entries, |entry| entry.acceptance_source_persisted);
    let denial_receipt_persistence_allowed_count =
        count(&entries, |entry| entry.denial_receipt_persistence_allowed);
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

    let positive_preconditions_readback_ready = source.retention_replay_readback_ready
        && source.retention_replay_entry_count == 7
        && source.retention_policy_persisted_count == 0
        && source.replay_index_written_count == 0
        && source.acceptance_source_recorded_count == 0
        && source.acceptance_source_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && positive_precondition_set_projected_count == 7
        && source_retention_replay_attached_count == 7
        && source_denial_receipt_attached_count == 7
        && source_acceptance_source_record_attached_count == 7
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
        && acceptance_source_recorded_count == 0
        && acceptance_source_persisted_count == 0
        && denial_receipt_persistence_allowed_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if positive_preconditions_readback_ready { "ready_blocked" } else { "blocked" },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_retention_replay_readback_ready: source.retention_replay_readback_ready,
        source_retention_replay_entry_count: source.retention_replay_entry_count,
        source_retention_policy_persisted_count: source.retention_policy_persisted_count,
        source_replay_index_written_count: source.replay_index_written_count,
        source_acceptance_source_recorded_count: source.acceptance_source_recorded_count,
        source_acceptance_source_persisted_count: source.acceptance_source_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count:
            source.receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_live_execution_allowed: source.live_execution_allowed,
        positive_preconditions_route: POSITIVE_PRECONDITIONS_ROUTE,
        precondition_entry_count: entries.len(),
        positive_precondition_set_projected_count,
        source_retention_replay_attached_count,
        source_denial_receipt_attached_count,
        source_acceptance_source_record_attached_count,
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
        acceptance_source_recorded_count,
        acceptance_source_persisted_count,
        denial_receipt_persistence_allowed_count,
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
        acceptance_source_recording_allowed: false,
        acceptance_source_persistence_allowed: false,
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
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry>
{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_retention_replay_entry_id: entry.id,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_acceptance_source_record_id: entry.source_acceptance_source_record_id,
                source_retention_policy_id: entry.retention_policy_id,
                source_replay_idempotency_key: entry.replay_idempotency_key,
                positive_precondition_set_id: format!(
                    "local-evidence-acceptance-recording-denial-receipt-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_route: format!("{POSITIVE_PRECONDITIONS_ROUTE}/{hyphenated}"),
                denial_receipt_persistence_grant_precondition_id: format!(
                    "denial-receipt-persistence-grant-required:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:{}",
                    entry.source_blocker_id
                ),
                observed_state:
                    "local_evidence_acceptance_recording_denial_receipt_positive_preconditions_projected_without_persistence",
                positive_precondition_set_projected: true,
                source_retention_replay_attached: true,
                source_denial_receipt_attached: true,
                source_acceptance_source_record_attached: true,
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
                acceptance_source_recorded: false,
                acceptance_source_persisted: false,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
                evidence_acceptance_recorded: false,
                evidence_recorded: false,
                receipt_store_write_attempt_recorded: false,
                receipt_store_written: false,
                receipt_persisted: false,
                ledger_written: false,
                workflow_event_log_written: false,
                sqlite_written: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

fn count(
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_recording_denial_receipt_positive_preconditions_projected_without_persistence"
        && entry.positive_precondition_set_projected
        && entry.source_retention_replay_attached
        && entry.source_denial_receipt_attached
        && entry.source_acceptance_source_record_attached
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
        && !entry.acceptance_source_recorded
        && !entry.acceptance_source_persisted
        && !entry.denial_receipt_persistence_allowed
        && !entry.denial_receipt_persisted
        && !entry.evidence_acceptance_recorded
        && !entry.evidence_recorded
        && !entry.receipt_store_write_attempt_recorded
        && !entry.receipt_store_written
        && !entry.receipt_persisted
        && !entry.ledger_written
        && !entry.workflow_event_log_written
        && !entry.sqlite_written
        && !entry.live_mutation_allowed
}

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsSideEffects {
    pub const fn none() -> Self {
        Self {
            acceptance_source_recorded: false,
            acceptance_source_persisted: false,
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
    fn local_denial_receipt_positive_preconditions_project_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_retention_replay_readback_ready);
        assert_eq!(report.source_retention_replay_entry_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.positive_precondition_set_projected_count, 7);
        assert_eq!(report.source_denial_receipt_attached_count, 7);
        assert_eq!(report.source_acceptance_source_record_attached_count, 7);
        assert_eq!(report.persistence_authority_required_count, 7);
        assert_eq!(report.persistence_authority_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert!(report.positive_preconditions_readback_ready);
    }

    #[test]
    fn local_denial_receipt_positive_preconditions_keep_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert_eq!(report.positive_preconditions_missing_count, 7);
        assert_eq!(report.acceptance_source_recorded_count, 0);
        assert_eq!(report.acceptance_source_persisted_count, 0);
        assert_eq!(report.denial_receipt_persistence_allowed_count, 0);
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
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingDenialReceiptPositivePreconditionsSideEffects::none()
        );
    }

    #[test]
    fn local_denial_receipt_positive_precondition_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.positive_precondition_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/positive-preconditions/dirty-worktree-boundary"
            && entry.denial_receipt_persistence_grant_precondition_id
                == "denial-receipt-persistence-grant-required:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry
                .source_retention_replay_entry_id
                .starts_with("evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_without_persistence_")
                && entry.source_denial_receipt_id.starts_with(
                    "local-evidence-acceptance-recording-denial-receipt:",
                )
                && entry_is_ready_blocked(entry)
        }));
    }
}
