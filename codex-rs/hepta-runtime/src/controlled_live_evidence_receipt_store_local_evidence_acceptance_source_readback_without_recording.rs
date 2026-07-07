use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write::controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingReport
{
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_open_preconditions_readback_ready: bool,
    pub source_open_precondition_entry_count: usize,
    pub source_open_precondition_catalog_ready_count: usize,
    pub source_dev_evidence_acceptance_source_required_count: usize,
    pub source_dev_evidence_acceptance_source_present_count: usize,
    pub source_evidence_acceptance_required_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_local_store_open_allowed_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub acceptance_source_entry_count: usize,
    pub acceptance_source_projected_count: usize,
    pub acceptance_source_schema_projected_count: usize,
    pub acceptance_source_policy_projected_count: usize,
    pub acceptance_source_readback_route_projected_count: usize,
    pub acceptance_source_idempotency_key_projected_count: usize,
    pub acceptance_source_idempotency_key_unique_count: usize,
    pub source_open_preconditions_attached_count: usize,
    pub acceptance_source_recording_required_count: usize,
    pub acceptance_source_recording_allowed_count: usize,
    pub acceptance_source_recorded_count: usize,
    pub acceptance_source_persisted_count: usize,
    pub evidence_acceptance_recording_allowed_count: usize,
    pub evidence_acceptance_recorded_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub local_evidence_acceptance_source_readback_ready: bool,
    pub acceptance_source_recording_allowed: bool,
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
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_open_precondition_set_id: String,
    pub source_open_precondition_route: String,
    pub source_dev_evidence_acceptance_source_id: String,
    pub source_evidence_acceptance_key: String,
    pub source_operator_local_store_approval_id: String,
    pub source_local_receipt_store_feature_gate: String,
    pub source_append_only_store_path_grant_key: String,
    pub local_evidence_acceptance_source_id: String,
    pub local_evidence_acceptance_source_route: String,
    pub local_evidence_acceptance_source_schema: &'static str,
    pub local_evidence_acceptance_source_kind: &'static str,
    pub local_evidence_acceptance_source_scope: &'static str,
    pub local_evidence_acceptance_source_policy_id: String,
    pub local_evidence_acceptance_source_redaction_policy: &'static str,
    pub local_evidence_acceptance_source_idempotency_key: String,
    pub local_evidence_acceptance_source_readback_route: String,
    pub local_evidence_acceptance_recording_boundary_route: String,
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
    pub source_open_preconditions_attached: bool,
    pub dev_evidence_acceptance_source_required: bool,
    pub dev_evidence_acceptance_source_projected: bool,
    pub acceptance_source_schema_projected: bool,
    pub acceptance_source_policy_projected: bool,
    pub acceptance_source_readback_route_projected: bool,
    pub acceptance_source_idempotency_key_projected: bool,
    pub acceptance_source_recording_required: bool,
    pub acceptance_source_recording_allowed: bool,
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub evidence_acceptance_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
    pub local_store_open_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingSideEffects
{
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingReport {
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
            let source =
                controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report();
            let entries =
                controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_entries();

            let acceptance_source_projected_count = entries
                .iter()
                .filter(|entry| entry.dev_evidence_acceptance_source_projected)
                .count();
            let acceptance_source_schema_projected_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_schema_projected)
                .count();
            let acceptance_source_policy_projected_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_policy_projected)
                .count();
            let acceptance_source_readback_route_projected_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_readback_route_projected)
                .count();
            let acceptance_source_idempotency_key_projected_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_idempotency_key_projected)
                .count();
            let mut acceptance_source_idempotency_keys = entries
                .iter()
                .map(|entry| entry.local_evidence_acceptance_source_idempotency_key.clone())
                .collect::<Vec<_>>();
            acceptance_source_idempotency_keys.sort();
            acceptance_source_idempotency_keys.dedup();
            let acceptance_source_idempotency_key_unique_count =
                acceptance_source_idempotency_keys.len();
            let source_open_preconditions_attached_count = entries
                .iter()
                .filter(|entry| entry.source_open_preconditions_attached)
                .count();
            let acceptance_source_recording_required_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_recording_required)
                .count();
            let acceptance_source_recording_allowed_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_recording_allowed)
                .count();
            let acceptance_source_recorded_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_recorded)
                .count();
            let acceptance_source_persisted_count = entries
                .iter()
                .filter(|entry| entry.acceptance_source_persisted)
                .count();
            let evidence_acceptance_recording_allowed_count = entries
                .iter()
                .filter(|entry| entry.evidence_acceptance_recording_allowed)
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

            let local_evidence_acceptance_source_readback_ready =
                source.local_open_preconditions_readback_ready
                    && source.open_precondition_entry_count == 7
                    && source.open_precondition_catalog_ready_count == 7
                    && source.dev_evidence_acceptance_source_required_count == 7
                    && source.dev_evidence_acceptance_source_present_count == 0
                    && source.evidence_acceptance_required_count == 7
                    && source.evidence_acceptance_present_count == 0
                    && source.local_store_open_allowed_count == 0
                    && source.evidence_recorded_count == 0
                    && source.receipt_store_write_attempt_recorded_count == 0
                    && source.receipt_store_written_count == 0
                    && source.receipt_persisted_count == 0
                    && !source.live_execution_allowed
                    && entries.len() == 7
                    && acceptance_source_projected_count == 7
                    && acceptance_source_schema_projected_count == 7
                    && acceptance_source_policy_projected_count == 7
                    && acceptance_source_readback_route_projected_count == 7
                    && acceptance_source_idempotency_key_projected_count == 7
                    && acceptance_source_idempotency_key_unique_count == 7
                    && source_open_preconditions_attached_count == 7
                    && acceptance_source_recording_required_count == 7
                    && acceptance_source_recording_allowed_count == 0
                    && acceptance_source_recorded_count == 0
                    && acceptance_source_persisted_count == 0
                    && evidence_acceptance_recording_allowed_count == 0
                    && evidence_acceptance_recorded_count == 0
                    && evidence_recorded_count == 0
                    && receipt_store_write_attempt_recorded_count == 0
                    && receipt_store_written_count == 0
                    && receipt_persisted_count == 0
                    && ledger_written_count == 0
                    && workflow_event_log_written_count == 0
                    && sqlite_written_count == 0
                    && live_mutation_allowed_count == 0;

            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingReport {
                runtime: "hepta",
                surface:
                    "controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording",
                status: if local_evidence_acceptance_source_readback_ready {
                    "ready_blocked"
                } else {
                    "blocked"
                },
                gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_GATE,
                schema_version:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION,
                plugin_id: "hepta-system@hepta-local",
                source_open_preconditions_readback_ready:
                    source.local_open_preconditions_readback_ready,
                source_open_precondition_entry_count: source.open_precondition_entry_count,
                source_open_precondition_catalog_ready_count:
                    source.open_precondition_catalog_ready_count,
                source_dev_evidence_acceptance_source_required_count:
                    source.dev_evidence_acceptance_source_required_count,
                source_dev_evidence_acceptance_source_present_count:
                    source.dev_evidence_acceptance_source_present_count,
                source_evidence_acceptance_required_count:
                    source.evidence_acceptance_required_count,
                source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
                source_local_store_open_allowed_count: source.local_store_open_allowed_count,
                source_evidence_recorded_count: source.evidence_recorded_count,
                source_receipt_store_write_attempt_recorded_count:
                    source.receipt_store_write_attempt_recorded_count,
                source_receipt_store_written_count: source.receipt_store_written_count,
                source_receipt_persisted_count: source.receipt_persisted_count,
                source_live_execution_allowed: source.live_execution_allowed,
                acceptance_source_entry_count: entries.len(),
                acceptance_source_projected_count,
                acceptance_source_schema_projected_count,
                acceptance_source_policy_projected_count,
                acceptance_source_readback_route_projected_count,
                acceptance_source_idempotency_key_projected_count,
                acceptance_source_idempotency_key_unique_count,
                source_open_preconditions_attached_count,
                acceptance_source_recording_required_count,
                acceptance_source_recording_allowed_count,
                acceptance_source_recorded_count,
                acceptance_source_persisted_count,
                evidence_acceptance_recording_allowed_count,
                evidence_acceptance_recorded_count,
                evidence_recorded_count,
                receipt_store_write_attempt_recorded_count,
                receipt_store_written_count,
                receipt_persisted_count,
                ledger_written_count,
                workflow_event_log_written_count,
                sqlite_written_count,
                live_mutation_allowed_count,
                local_evidence_acceptance_source_readback_ready,
                acceptance_source_recording_allowed: false,
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
                    "acceptance_source_recording_disabled",
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
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_SOURCE_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE,
                side_effects:
                    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingSideEffects::none(),
            }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingEntry>
{
    controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_source_without_recording_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_open_precondition_set_id: entry.local_open_precondition_set_id,
                source_open_precondition_route: entry.local_open_precondition_route,
                source_dev_evidence_acceptance_source_id: entry.dev_evidence_acceptance_source_id,
                source_evidence_acceptance_key: entry.evidence_acceptance_key,
                source_operator_local_store_approval_id: entry.operator_local_store_approval_id,
                source_local_receipt_store_feature_gate: entry.local_receipt_store_feature_gate,
                source_append_only_store_path_grant_key: entry.append_only_store_path_grant_key,
                local_evidence_acceptance_source_id: format!(
                    "local-evidence-acceptance-source:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_evidence_acceptance_source_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance-source/{hyphenated}"
                ),
                local_evidence_acceptance_source_schema:
                    "controlled_live_local_evidence_acceptance_source_v1",
                local_evidence_acceptance_source_kind: "dev_only",
                local_evidence_acceptance_source_scope:
                    "controlled_live_local_evidence_receipt_store",
                local_evidence_acceptance_source_policy_id: format!(
                    "local-evidence-acceptance-source-policy:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_evidence_acceptance_source_redaction_policy:
                    "metadata_only_no_secret_material",
                local_evidence_acceptance_source_idempotency_key: format!(
                    "local-evidence-acceptance-source-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_evidence_acceptance_source_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance-source/{hyphenated}/readback"
                ),
                local_evidence_acceptance_recording_boundary_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance-source/{hyphenated}/recording-boundary"
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "local_evidence_acceptance_source_projected_without_recording",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_open_preconditions_attached: entry.source_terminal_closeout_attached
                    && entry.terminal_no_persistence_confirmed,
                dev_evidence_acceptance_source_required: entry.dev_evidence_acceptance_source_required,
                dev_evidence_acceptance_source_projected: true,
                acceptance_source_schema_projected: true,
                acceptance_source_policy_projected: true,
                acceptance_source_readback_route_projected: true,
                acceptance_source_idempotency_key_projected: true,
                acceptance_source_recording_required: true,
                acceptance_source_recording_allowed: false,
                acceptance_source_recorded: false,
                acceptance_source_persisted: false,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: false,
                evidence_acceptance_recording_allowed: false,
                evidence_acceptance_recorded: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
                local_store_open_allowed: false,
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingSideEffects
{
    pub const fn none() -> Self {
        Self {
            acceptance_source_recorded: false,
            acceptance_source_persisted: false,
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
    fn local_evidence_acceptance_source_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.local_evidence_acceptance_source_readback_ready);
        assert_eq!(report.source_open_precondition_entry_count, 7);
        assert_eq!(report.source_open_precondition_catalog_ready_count, 7);
        assert_eq!(report.acceptance_source_entry_count, 7);
        assert_eq!(report.acceptance_source_projected_count, 7);
        assert_eq!(report.acceptance_source_schema_projected_count, 7);
        assert_eq!(report.acceptance_source_policy_projected_count, 7);
        assert_eq!(report.acceptance_source_readback_route_projected_count, 7);
        assert_eq!(report.acceptance_source_idempotency_key_projected_count, 7);
        assert_eq!(report.acceptance_source_idempotency_key_unique_count, 7);
        assert_eq!(report.source_open_preconditions_attached_count, 7);
        assert_eq!(report.acceptance_source_recording_required_count, 7);
    }

    #[test]
    fn local_evidence_acceptance_source_keeps_recording_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report();

        assert_eq!(
            report.source_dev_evidence_acceptance_source_present_count,
            0
        );
        assert_eq!(report.source_evidence_acceptance_present_count, 0);
        assert_eq!(report.source_local_store_open_allowed_count, 0);
        assert_eq!(report.acceptance_source_recording_allowed_count, 0);
        assert_eq!(report.acceptance_source_recorded_count, 0);
        assert_eq!(report.acceptance_source_persisted_count, 0);
        assert_eq!(report.evidence_acceptance_recording_allowed_count, 0);
        assert_eq!(report.evidence_acceptance_recorded_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_write_attempt_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.acceptance_source_recording_allowed);
        assert!(!report.evidence_acceptance_recording_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceSourceReadbackWithoutRecordingSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_source_entries_are_stable_and_unrecorded() {
        let entries =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_entries();

        assert!(entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.local_evidence_acceptance_source_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance-source/dirty-worktree-boundary"
                && entry.local_evidence_acceptance_source_idempotency_key
                    == "local-evidence-acceptance-source-idempotency:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(entries.iter().all(|entry| {
            entry.observed_state == "local_evidence_acceptance_source_projected_without_recording"
                && entry.source_open_preconditions_attached
                && entry.dev_evidence_acceptance_source_required
                && entry.dev_evidence_acceptance_source_projected
                && entry.acceptance_source_schema_projected
                && entry.acceptance_source_policy_projected
                && entry.acceptance_source_readback_route_projected
                && entry.acceptance_source_idempotency_key_projected
                && entry.acceptance_source_recording_required
                && !entry.acceptance_source_recording_allowed
                && !entry.acceptance_source_recorded
                && !entry.acceptance_source_persisted
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && !entry.evidence_acceptance_recording_allowed
                && !entry.evidence_acceptance_recorded
                && !entry.evidence_recording_allowed
                && !entry.evidence_recorded
                && !entry.local_store_open_allowed
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        }));
    }
}
