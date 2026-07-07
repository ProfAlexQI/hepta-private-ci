use crate::controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence::controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_GATE:
    &str = "controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance";

const POSITIVE_PRECONDITIONS_ROUTE: &str =
    "readback://controlled-live/evidence-receipt-store/positive-acceptance-preconditions";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_retention_replay_readback_ready: bool,
    pub source_retention_replay_entry_count: usize,
    pub source_retention_policy_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_live_execution_allowed: bool,
    pub positive_preconditions_route: &'static str,
    pub precondition_entry_count: usize,
    pub positive_precondition_set_projected_count: usize,
    pub source_retention_replay_attached_count: usize,
    pub operator_acceptance_required_count: usize,
    pub operator_acceptance_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub receipt_persistence_grant_required_count: usize,
    pub receipt_persistence_grant_present_count: usize,
    pub atomic_append_required_count: usize,
    pub atomic_append_enabled_count: usize,
    pub post_write_readback_required_count: usize,
    pub post_write_readback_persisted_count: usize,
    pub rollback_rehearsal_required_count: usize,
    pub rollback_rehearsal_verified_count: usize,
    pub retention_policy_commit_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub live_cutover_approval_required_count: usize,
    pub live_cutover_approval_present_count: usize,
    pub acceptance_preconditions_missing_count: usize,
    pub acceptance_allowed_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub positive_acceptance_preconditions_readback_ready: bool,
    pub acceptance_allowed: bool,
    pub acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
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
        ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_retention_replay_entry_id: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_retention_policy_id: String,
    pub source_retention_policy_route: String,
    pub source_replay_key: String,
    pub positive_precondition_set_id: String,
    pub positive_precondition_route: String,
    pub operator_acceptance_precondition_id: String,
    pub evidence_acceptance_precondition_id: String,
    pub receipt_persistence_grant_precondition_id: String,
    pub atomic_append_precondition_id: String,
    pub post_write_readback_precondition_id: String,
    pub rollback_rehearsal_precondition_id: String,
    pub retention_commit_precondition_id: String,
    pub live_cutover_approval_precondition_id: String,
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
    pub positive_precondition_set_projected: bool,
    pub source_retention_replay_attached: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub receipt_persistence_grant_required: bool,
    pub receipt_persistence_grant_present: bool,
    pub atomic_append_required: bool,
    pub atomic_append_enabled: bool,
    pub post_write_readback_required: bool,
    pub post_write_readback_persisted: bool,
    pub rollback_rehearsal_required: bool,
    pub rollback_rehearsal_verified: bool,
    pub retention_policy_commit_required: bool,
    pub retention_policy_committed: bool,
    pub live_cutover_approval_required: bool,
    pub live_cutover_approval_present: bool,
    pub acceptance_preconditions_missing: bool,
    pub acceptance_allowed: bool,
    pub acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
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
pub struct ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceSideEffects
{
    pub acceptance_recorded: bool,
    pub evidence_recorded: bool,
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

pub fn controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report()
-> ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceReport
{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_report();
    let entries =
        controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_entries();

    let positive_precondition_set_projected_count = entries
        .iter()
        .filter(|entry| entry.positive_precondition_set_projected)
        .count();
    let source_retention_replay_attached_count = entries
        .iter()
        .filter(|entry| entry.source_retention_replay_attached)
        .count();
    let operator_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_required)
        .count();
    let operator_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_present)
        .count();
    let evidence_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required)
        .count();
    let evidence_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_present)
        .count();
    let receipt_persistence_grant_required_count = entries
        .iter()
        .filter(|entry| entry.receipt_persistence_grant_required)
        .count();
    let receipt_persistence_grant_present_count = entries
        .iter()
        .filter(|entry| entry.receipt_persistence_grant_present)
        .count();
    let atomic_append_required_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_required)
        .count();
    let atomic_append_enabled_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_enabled)
        .count();
    let post_write_readback_required_count = entries
        .iter()
        .filter(|entry| entry.post_write_readback_required)
        .count();
    let post_write_readback_persisted_count = entries
        .iter()
        .filter(|entry| entry.post_write_readback_persisted)
        .count();
    let rollback_rehearsal_required_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_required)
        .count();
    let rollback_rehearsal_verified_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_verified)
        .count();
    let retention_policy_commit_required_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_commit_required)
        .count();
    let retention_policy_committed_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_committed)
        .count();
    let live_cutover_approval_required_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_approval_required)
        .count();
    let live_cutover_approval_present_count = entries
        .iter()
        .filter(|entry| entry.live_cutover_approval_present)
        .count();
    let acceptance_preconditions_missing_count = entries
        .iter()
        .filter(|entry| entry.acceptance_preconditions_missing)
        .count();
    let acceptance_allowed_count = entries
        .iter()
        .filter(|entry| entry.acceptance_allowed)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
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

    let positive_acceptance_preconditions_readback_ready = source.retention_replay_readback_ready
        && source.retention_replay_entry_count == 7
        && source.retention_policy_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && positive_precondition_set_projected_count == 7
        && source_retention_replay_attached_count == 7
        && operator_acceptance_required_count == 7
        && operator_acceptance_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && receipt_persistence_grant_required_count == 7
        && receipt_persistence_grant_present_count == 0
        && atomic_append_required_count == 7
        && atomic_append_enabled_count == 0
        && post_write_readback_required_count == 7
        && post_write_readback_persisted_count == 0
        && rollback_rehearsal_required_count == 7
        && rollback_rehearsal_verified_count == 0
        && retention_policy_commit_required_count == 7
        && retention_policy_committed_count == 0
        && live_cutover_approval_required_count == 7
        && live_cutover_approval_present_count == 0
        && acceptance_preconditions_missing_count == 7
        && acceptance_allowed_count == 0
        && evidence_recorded_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "positive_acceptance_preconditions_projected_without_acceptance"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.positive_precondition_set_projected
                && entry.source_retention_replay_attached
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.receipt_persistence_grant_required
                && !entry.receipt_persistence_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_write_readback_required
                && !entry.post_write_readback_persisted
                && entry.rollback_rehearsal_required
                && !entry.rollback_rehearsal_verified
                && entry.retention_policy_commit_required
                && !entry.retention_policy_committed
                && entry.live_cutover_approval_required
                && !entry.live_cutover_approval_present
                && entry.acceptance_preconditions_missing
                && !entry.acceptance_allowed
                && !entry.acceptance_recording_allowed
                && !entry.evidence_recording_allowed
                && !entry.evidence_recorded
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

    ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance",
        status: if positive_acceptance_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_retention_replay_readback_ready: source.retention_replay_readback_ready,
        source_retention_replay_entry_count: source.retention_replay_entry_count,
        source_retention_policy_persisted_count: source.retention_policy_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_live_execution_allowed: source.live_execution_allowed,
        positive_preconditions_route: POSITIVE_PRECONDITIONS_ROUTE,
        precondition_entry_count: entries.len(),
        positive_precondition_set_projected_count,
        source_retention_replay_attached_count,
        operator_acceptance_required_count,
        operator_acceptance_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        receipt_persistence_grant_required_count,
        receipt_persistence_grant_present_count,
        atomic_append_required_count,
        atomic_append_enabled_count,
        post_write_readback_required_count,
        post_write_readback_persisted_count,
        rollback_rehearsal_required_count,
        rollback_rehearsal_verified_count,
        retention_policy_commit_required_count,
        retention_policy_committed_count,
        live_cutover_approval_required_count,
        live_cutover_approval_present_count,
        acceptance_preconditions_missing_count,
        acceptance_allowed_count,
        evidence_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        positive_acceptance_preconditions_readback_ready,
        acceptance_allowed: false,
        acceptance_recording_allowed: false,
        evidence_recording_allowed: false,
        receipt_persistence_allowed: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_acceptance_missing",
            "evidence_acceptance_missing",
            "receipt_persistence_grant_missing",
            "atomic_append_not_enabled",
            "post_write_readback_missing",
            "rollback_rehearsal_missing",
            "retention_policy_not_committed",
            "live_cutover_approval_missing",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_POSITIVE_ACCEPTANCE_PRECONDITIONS_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceEntry,
> {
    controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceEntry {
                id: format!(
                    "evidence_receipt_store_positive_acceptance_preconditions_without_acceptance_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_retention_replay_entry_id: entry.id,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_retention_policy_id: entry.retention_policy_id,
                source_retention_policy_route: entry.retention_policy_route,
                source_replay_key: entry.replay_key,
                positive_precondition_set_id: format!(
                    "positive-acceptance-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_route: format!("{POSITIVE_PRECONDITIONS_ROUTE}/{hyphenated}"),
                operator_acceptance_precondition_id: format!(
                    "operator-acceptance-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                evidence_acceptance_precondition_id: format!(
                    "evidence-acceptance-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                receipt_persistence_grant_precondition_id: format!(
                    "receipt-persistence-grant-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                atomic_append_precondition_id: format!(
                    "atomic-append-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_write_readback_precondition_id: format!(
                    "post-write-readback-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                rollback_rehearsal_precondition_id: format!(
                    "rollback-rehearsal-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_commit_precondition_id: format!(
                    "retention-policy-commit-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                live_cutover_approval_precondition_id: format!(
                    "live-cutover-approval-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "positive_acceptance_preconditions_projected_without_acceptance",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                positive_precondition_set_projected: true,
                source_retention_replay_attached: true,
                operator_acceptance_required: true,
                operator_acceptance_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                receipt_persistence_grant_required: true,
                receipt_persistence_grant_present: false,
                atomic_append_required: true,
                atomic_append_enabled: false,
                post_write_readback_required: true,
                post_write_readback_persisted: false,
                rollback_rehearsal_required: true,
                rollback_rehearsal_verified: false,
                retention_policy_commit_required: true,
                retention_policy_committed: false,
                live_cutover_approval_required: true,
                live_cutover_approval_present: false,
                acceptance_preconditions_missing: true,
                acceptance_allowed: false,
                acceptance_recording_allowed: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
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

impl ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceSideEffects {
    pub const fn none() -> Self {
        Self {
            acceptance_recorded: false,
            evidence_recorded: false,
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
    fn positive_acceptance_preconditions_project_all_entries_without_acceptance() {
        let report =
            controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_retention_replay_readback_ready);
        assert_eq!(report.source_retention_replay_entry_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.positive_precondition_set_projected_count, 7);
        assert_eq!(report.source_retention_replay_attached_count, 7);
        assert_eq!(report.operator_acceptance_required_count, 7);
        assert_eq!(report.operator_acceptance_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.receipt_persistence_grant_required_count, 7);
        assert_eq!(report.receipt_persistence_grant_present_count, 0);
        assert_eq!(report.atomic_append_required_count, 7);
        assert_eq!(report.atomic_append_enabled_count, 0);
        assert_eq!(report.acceptance_preconditions_missing_count, 7);
        assert!(report.positive_acceptance_preconditions_readback_ready);
    }

    #[test]
    fn positive_acceptance_preconditions_keep_acceptance_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report();

        assert_eq!(report.post_write_readback_required_count, 7);
        assert_eq!(report.post_write_readback_persisted_count, 0);
        assert_eq!(report.rollback_rehearsal_required_count, 7);
        assert_eq!(report.rollback_rehearsal_verified_count, 0);
        assert_eq!(report.retention_policy_commit_required_count, 7);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert_eq!(report.live_cutover_approval_required_count, 7);
        assert_eq!(report.live_cutover_approval_present_count, 0);
        assert_eq!(report.acceptance_allowed_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.acceptance_allowed);
        assert!(!report.receipt_store_written);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStorePositiveAcceptancePreconditionsReadbackWithoutAcceptanceSideEffects::none()
        );
    }

    #[test]
    fn positive_acceptance_precondition_entries_are_stable_and_missing() {
        let report =
            controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.positive_precondition_route
                == "readback://controlled-live/evidence-receipt-store/positive-acceptance-preconditions/dirty-worktree-boundary"
            && entry.operator_acceptance_precondition_id
                == "operator-acceptance-required:controlled-live-evidence-receipt-store:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.positive_precondition_set_projected
                && entry.source_retention_replay_attached
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.receipt_persistence_grant_required
                && !entry.receipt_persistence_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_write_readback_required
                && !entry.post_write_readback_persisted
                && entry.rollback_rehearsal_required
                && !entry.rollback_rehearsal_verified
                && entry.retention_policy_commit_required
                && !entry.retention_policy_committed
                && entry.live_cutover_approval_required
                && !entry.live_cutover_approval_present
                && entry.acceptance_preconditions_missing
                && !entry.acceptance_allowed
                && !entry.evidence_recorded
                && !entry.receipt_persisted
                && !entry.receipt_store_written
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
