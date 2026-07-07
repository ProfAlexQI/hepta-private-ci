use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_positive_preconditions::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_positive_preconditions_readback_without_persistence_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_v1";
const PERSISTENCE_DENIAL_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/persistence-denial";
const PERSISTENCE_DENIAL_REASON: &str = "local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_disabled_positive_preconditions_missing";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_positive_preconditions_readback_ready: bool,
    pub source_precondition_entry_count: usize,
    pub source_positive_precondition_set_projected_count: usize,
    pub source_persistence_authority_required_count: usize,
    pub source_persistence_authority_present_count: usize,
    pub source_operator_persistence_approval_required_count: usize,
    pub source_operator_persistence_approval_present_count: usize,
    pub source_evidence_acceptance_required_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_denial_receipt_persistence_grant_required_count: usize,
    pub source_denial_receipt_persistence_grant_present_count: usize,
    pub source_atomic_append_required_count: usize,
    pub source_atomic_append_enabled_count: usize,
    pub source_post_persist_readback_required_count: usize,
    pub source_post_persist_readback_persisted_count: usize,
    pub source_rollback_anchor_required_count: usize,
    pub source_rollback_anchor_verified_count: usize,
    pub source_retention_policy_commit_required_count: usize,
    pub source_retention_policy_committed_count: usize,
    pub source_replay_idempotency_guard_required_count: usize,
    pub source_replay_idempotency_guard_enabled_count: usize,
    pub source_positive_preconditions_missing_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_authority_decision_persisted_count: usize,
    pub source_denial_receipt_persistence_allowed_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub persistence_denial_route: &'static str,
    pub persistence_denial_entry_count: usize,
    pub persistence_denial_projected_count: usize,
    pub source_positive_preconditions_attached_count: usize,
    pub source_denial_receipt_attached_count: usize,
    pub source_authority_decision_record_id_attached_count: usize,
    pub denial_receipt_persistence_denied_count: usize,
    pub denial_receipt_persistence_disabled_count: usize,
    pub denial_receipt_persistence_allowed_count: usize,
    pub denial_receipt_persistence_attempt_recorded_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub persistence_authority_missing_count: usize,
    pub operator_persistence_approval_missing_count: usize,
    pub evidence_acceptance_missing_count: usize,
    pub denial_receipt_persistence_grant_missing_count: usize,
    pub atomic_append_disabled_count: usize,
    pub post_persist_readback_missing_count: usize,
    pub rollback_anchor_missing_count: usize,
    pub retention_policy_not_committed_count: usize,
    pub replay_idempotency_guard_disabled_count: usize,
    pub authority_decision_recorded_count: usize,
    pub authority_decision_persisted_count: usize,
    pub evidence_acceptance_recorded_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub persistence_denial_readback_ready: bool,
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
    pub entries:
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_positive_precondition_set_id: String,
    pub source_positive_precondition_route: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_authority_decision_record_id: String,
    pub source_retention_policy_id: String,
    pub source_replay_idempotency_key: String,
    pub source_denial_receipt_persistence_grant_precondition_id: String,
    pub persistence_denial_id: String,
    pub persistence_denial_route: String,
    pub persistence_denial_reason: &'static str,
    pub observed_state: &'static str,
    pub source_positive_precondition_set_projected: bool,
    pub source_positive_preconditions_missing: bool,
    pub source_denial_receipt_attached: bool,
    pub source_authority_decision_record_id_attached: bool,
    pub persistence_denial_projected: bool,
    pub denial_receipt_persistence_denied: bool,
    pub denial_receipt_persistence_disabled: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persistence_attempt_recorded: bool,
    pub denial_receipt_persisted: bool,
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
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_store_write_attempt_recorded: bool,
    pub receipt_store_written: bool,
    pub receipt_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub credential_read_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialSideEffects
{
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persistence_attempt_recorded: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report(
) -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialReport
{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_positive_preconditions_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_entries();

    let persistence_denial_projected_count =
        count(&entries, |entry| entry.persistence_denial_projected);
    let source_positive_preconditions_attached_count = count(&entries, |entry| {
        entry.source_positive_precondition_set_projected
    });
    let source_denial_receipt_attached_count =
        count(&entries, |entry| entry.source_denial_receipt_attached);
    let source_authority_decision_record_id_attached_count = count(&entries, |entry| {
        entry.source_authority_decision_record_id_attached
    });
    let denial_receipt_persistence_denied_count =
        count(&entries, |entry| entry.denial_receipt_persistence_denied);
    let denial_receipt_persistence_disabled_count =
        count(&entries, |entry| entry.denial_receipt_persistence_disabled);
    let denial_receipt_persistence_allowed_count =
        count(&entries, |entry| entry.denial_receipt_persistence_allowed);
    let denial_receipt_persistence_attempt_recorded_count = count(&entries, |entry| {
        entry.denial_receipt_persistence_attempt_recorded
    });
    let denial_receipt_persisted_count = count(&entries, |entry| entry.denial_receipt_persisted);
    let persistence_authority_missing_count = count(&entries, |entry| {
        entry.persistence_authority_required && !entry.persistence_authority_present
    });
    let operator_persistence_approval_missing_count = count(&entries, |entry| {
        entry.operator_persistence_approval_required && !entry.operator_persistence_approval_present
    });
    let evidence_acceptance_missing_count = count(&entries, |entry| {
        entry.evidence_acceptance_required && !entry.evidence_acceptance_present
    });
    let denial_receipt_persistence_grant_missing_count = count(&entries, |entry| {
        entry.denial_receipt_persistence_grant_required
            && !entry.denial_receipt_persistence_grant_present
    });
    let atomic_append_disabled_count = count(&entries, |entry| {
        entry.atomic_append_required && !entry.atomic_append_enabled
    });
    let post_persist_readback_missing_count = count(&entries, |entry| {
        entry.post_persist_readback_required && !entry.post_persist_readback_persisted
    });
    let rollback_anchor_missing_count = count(&entries, |entry| {
        entry.rollback_anchor_required && !entry.rollback_anchor_verified
    });
    let retention_policy_not_committed_count = count(&entries, |entry| {
        entry.retention_policy_commit_required && !entry.retention_policy_committed
    });
    let replay_idempotency_guard_disabled_count = count(&entries, |entry| {
        entry.replay_idempotency_guard_required && !entry.replay_idempotency_guard_enabled
    });
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let authority_decision_persisted_count =
        count(&entries, |entry| entry.authority_decision_persisted);
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

    let persistence_denial_readback_ready = source.positive_preconditions_readback_ready
        && source.precondition_entry_count == 7
        && source.positive_precondition_set_projected_count == 7
        && source.persistence_authority_required_count == 7
        && source.persistence_authority_present_count == 0
        && source.operator_persistence_approval_required_count == 7
        && source.operator_persistence_approval_present_count == 0
        && source.evidence_acceptance_required_count == 7
        && source.evidence_acceptance_present_count == 0
        && source.denial_receipt_persistence_grant_required_count == 7
        && source.denial_receipt_persistence_grant_present_count == 0
        && source.atomic_append_required_count == 7
        && source.atomic_append_enabled_count == 0
        && source.post_persist_readback_required_count == 7
        && source.post_persist_readback_persisted_count == 0
        && source.rollback_anchor_required_count == 7
        && source.rollback_anchor_verified_count == 0
        && source.retention_policy_commit_required_count == 7
        && source.retention_policy_committed_count == 0
        && source.replay_idempotency_guard_required_count == 7
        && source.replay_idempotency_guard_enabled_count == 0
        && source.positive_preconditions_missing_count == 7
        && source.denial_receipt_persistence_allowed_count == 0
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
        && persistence_denial_projected_count == 7
        && source_positive_preconditions_attached_count == 7
        && source_denial_receipt_attached_count == 7
        && source_authority_decision_record_id_attached_count == 7
        && denial_receipt_persistence_denied_count == 7
        && denial_receipt_persistence_disabled_count == 7
        && denial_receipt_persistence_allowed_count == 0
        && denial_receipt_persistence_attempt_recorded_count == 0
        && denial_receipt_persisted_count == 0
        && persistence_authority_missing_count == 7
        && operator_persistence_approval_missing_count == 7
        && evidence_acceptance_missing_count == 7
        && denial_receipt_persistence_grant_missing_count == 7
        && atomic_append_disabled_count == 7
        && post_persist_readback_missing_count == 7
        && rollback_anchor_missing_count == 7
        && retention_policy_not_committed_count == 7
        && replay_idempotency_guard_disabled_count == 7
        && authority_decision_recorded_count == 0
        && authority_decision_persisted_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if persistence_denial_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_positive_preconditions_readback_ready:
            source.positive_preconditions_readback_ready,
        source_precondition_entry_count: source.precondition_entry_count,
        source_positive_precondition_set_projected_count:
            source.positive_precondition_set_projected_count,
        source_persistence_authority_required_count: source.persistence_authority_required_count,
        source_persistence_authority_present_count: source.persistence_authority_present_count,
        source_operator_persistence_approval_required_count:
            source.operator_persistence_approval_required_count,
        source_operator_persistence_approval_present_count:
            source.operator_persistence_approval_present_count,
        source_evidence_acceptance_required_count: source.evidence_acceptance_required_count,
        source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
        source_denial_receipt_persistence_grant_required_count:
            source.denial_receipt_persistence_grant_required_count,
        source_denial_receipt_persistence_grant_present_count:
            source.denial_receipt_persistence_grant_present_count,
        source_atomic_append_required_count: source.atomic_append_required_count,
        source_atomic_append_enabled_count: source.atomic_append_enabled_count,
        source_post_persist_readback_required_count: source.post_persist_readback_required_count,
        source_post_persist_readback_persisted_count: source
            .post_persist_readback_persisted_count,
        source_rollback_anchor_required_count: source.rollback_anchor_required_count,
        source_rollback_anchor_verified_count: source.rollback_anchor_verified_count,
        source_retention_policy_commit_required_count:
            source.retention_policy_commit_required_count,
        source_retention_policy_committed_count: source.retention_policy_committed_count,
        source_replay_idempotency_guard_required_count:
            source.replay_idempotency_guard_required_count,
        source_replay_idempotency_guard_enabled_count:
            source.replay_idempotency_guard_enabled_count,
        source_positive_preconditions_missing_count: source.positive_preconditions_missing_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_authority_decision_persisted_count: source.authority_decision_persisted_count,
        source_denial_receipt_persistence_allowed_count:
            source.denial_receipt_persistence_allowed_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count:
            source.receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        persistence_denial_route: PERSISTENCE_DENIAL_ROUTE,
        persistence_denial_entry_count: entries.len(),
        persistence_denial_projected_count,
        source_positive_preconditions_attached_count,
        source_denial_receipt_attached_count,
        source_authority_decision_record_id_attached_count,
        denial_receipt_persistence_denied_count,
        denial_receipt_persistence_disabled_count,
        denial_receipt_persistence_allowed_count,
        denial_receipt_persistence_attempt_recorded_count,
        denial_receipt_persisted_count,
        persistence_authority_missing_count,
        operator_persistence_approval_missing_count,
        evidence_acceptance_missing_count,
        denial_receipt_persistence_grant_missing_count,
        atomic_append_disabled_count,
        post_persist_readback_missing_count,
        rollback_anchor_missing_count,
        retention_policy_not_committed_count,
        replay_idempotency_guard_disabled_count,
        authority_decision_recorded_count,
        authority_decision_persisted_count,
        evidence_acceptance_recorded_count,
        evidence_recorded_count,
        receipt_store_write_attempt_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        persistence_denial_readback_ready,
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
            "persistence_authority_missing",
            "operator_persistence_approval_missing",
            "evidence_acceptance_missing",
            "denial_receipt_persistence_grant_missing",
            "atomic_append_not_enabled",
            "post_persist_readback_missing",
            "rollback_anchor_missing",
            "retention_policy_not_committed",
            "replay_idempotency_guard_disabled",
            "denial_receipt_persistence_disabled",
            "authority_decision_recording_disabled",
            "receipt_store_write_attempt_recording_disabled",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_positive_preconditions_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_positive_precondition_set_id: entry.positive_precondition_set_id,
                source_positive_precondition_route: entry.positive_precondition_route,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_authority_decision_record_id: entry.source_authority_decision_record_id,
                source_retention_policy_id: entry.source_retention_policy_id,
                source_replay_idempotency_key: entry.source_replay_idempotency_key,
                source_denial_receipt_persistence_grant_precondition_id: entry
                    .denial_receipt_persistence_grant_precondition_id,
                persistence_denial_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                persistence_denial_route: format!("{PERSISTENCE_DENIAL_ROUTE}/{hyphenated}"),
                persistence_denial_reason: PERSISTENCE_DENIAL_REASON,
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denied_without_persistence",
                source_positive_precondition_set_projected: entry
                    .positive_precondition_set_projected,
                source_positive_preconditions_missing: entry.positive_preconditions_missing,
                source_denial_receipt_attached: entry.source_denial_receipt_attached,
                source_authority_decision_record_id_attached: entry
                    .source_authority_decision_record_id_attached,
                persistence_denial_projected: true,
                denial_receipt_persistence_denied: true,
                denial_receipt_persistence_disabled: true,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persistence_attempt_recorded: false,
                denial_receipt_persisted: false,
                persistence_authority_required: entry.persistence_authority_required,
                persistence_authority_present: entry.persistence_authority_present,
                operator_persistence_approval_required: entry.operator_persistence_approval_required,
                operator_persistence_approval_present: entry.operator_persistence_approval_present,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: entry.evidence_acceptance_present,
                denial_receipt_persistence_grant_required: entry
                    .denial_receipt_persistence_grant_required,
                denial_receipt_persistence_grant_present: entry
                    .denial_receipt_persistence_grant_present,
                atomic_append_required: entry.atomic_append_required,
                atomic_append_enabled: entry.atomic_append_enabled,
                post_persist_readback_required: entry.post_persist_readback_required,
                post_persist_readback_persisted: entry.post_persist_readback_persisted,
                rollback_anchor_required: entry.rollback_anchor_required,
                rollback_anchor_verified: entry.rollback_anchor_verified,
                retention_policy_commit_required: entry.retention_policy_commit_required,
                retention_policy_committed: entry.retention_policy_committed,
                replay_idempotency_guard_required: entry.replay_idempotency_guard_required,
                replay_idempotency_guard_enabled: entry.replay_idempotency_guard_enabled,
                authority_decision_recorded: false,
                authority_decision_persisted: false,
                evidence_acceptance_recorded: false,
                evidence_recorded: false,
                receipt_store_write_attempt_recorded: false,
                receipt_store_written: false,
                receipt_persisted: false,
                ledger_written: false,
                workflow_event_log_written: false,
                sqlite_written: false,
                credential_read_allowed: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

fn count(
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denied_without_persistence"
        && entry.source_positive_precondition_set_projected
        && entry.source_positive_preconditions_missing
        && entry.source_denial_receipt_attached
        && entry.source_authority_decision_record_id_attached
        && entry.persistence_denial_projected
        && entry.denial_receipt_persistence_denied
        && entry.denial_receipt_persistence_disabled
        && !entry.denial_receipt_persistence_allowed
        && !entry.denial_receipt_persistence_attempt_recorded
        && !entry.denial_receipt_persisted
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
        && !entry.authority_decision_recorded
        && !entry.authority_decision_persisted
        && !entry.evidence_acceptance_recorded
        && !entry.evidence_recorded
        && !entry.receipt_store_write_attempt_recorded
        && !entry.receipt_store_written
        && !entry.receipt_persisted
        && !entry.ledger_written
        && !entry.workflow_event_log_written
        && !entry.sqlite_written
        && !entry.credential_read_allowed
        && !entry.live_mutation_allowed
}

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialSideEffects {
    pub const fn none() -> Self {
        Self {
            authority_decision_recorded: false,
            authority_decision_persisted: false,
            denial_receipt_persistence_attempt_recorded: false,
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
    fn local_denial_receipt_persistence_denial_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_positive_preconditions_readback_ready);
        assert_eq!(report.source_precondition_entry_count, 7);
        assert_eq!(report.persistence_denial_entry_count, 7);
        assert_eq!(report.persistence_denial_projected_count, 7);
        assert_eq!(report.source_positive_preconditions_attached_count, 7);
        assert_eq!(report.source_denial_receipt_attached_count, 7);
        assert_eq!(report.source_authority_decision_record_id_attached_count, 7);
        assert_eq!(report.denial_receipt_persistence_denied_count, 7);
        assert_eq!(report.denial_receipt_persistence_disabled_count, 7);
        assert!(report.persistence_denial_readback_ready);
    }

    #[test]
    fn local_denial_receipt_persistence_denial_keeps_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_report();

        assert_eq!(report.denial_receipt_persistence_allowed_count, 0);
        assert_eq!(report.denial_receipt_persistence_attempt_recorded_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
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
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptPersistenceDenialSideEffects::none()
        );
    }

    #[test]
    fn local_denial_receipt_persistence_denial_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_denial_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.persistence_denial_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/persistence-denial/dirty-worktree-boundary"
            && entry.persistence_denial_reason
                == "local_evidence_acceptance_authority_decision_recording_denial_receipt_persistence_disabled_positive_preconditions_missing"));
        assert!(report.entries.iter().all(|entry| {
            entry.source_positive_precondition_set_id.starts_with(
                "local-evidence-acceptance-authority-decision-recording-denial-receipt-positive-preconditions:",
            ) && entry
                .source_denial_receipt_id
                .starts_with("local-evidence-acceptance-authority-decision-recording-denial-receipt:")
                && entry.persistence_denial_id.starts_with(
                    "local-evidence-acceptance-authority-decision-recording-denial-receipt-persistence-denial:",
                )
                && entry_is_ready_blocked(entry)
        }));
    }
}
