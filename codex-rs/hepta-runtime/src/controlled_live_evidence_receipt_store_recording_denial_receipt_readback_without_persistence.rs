use std::collections::BTreeSet;

use crate::controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording::controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_GATE:
    &str = "controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence";

const DENIAL_RECEIPT_COLLECTION_ID: &str =
    "controlled-live-evidence-receipt-store-recording-denial-receipts";
const DENIAL_RECEIPT_COLLECTION_ROUTE: &str =
    "readback://controlled-live/evidence-receipt-store/recording-denial-receipts";
const DENIAL_RECEIPT_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_recording_denial_receipt_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_acceptance_decision_recording_boundary_ready: bool,
    pub source_boundary_entry_count: usize,
    pub source_operator_acceptance_present_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_decision_recording_allowed_count: usize,
    pub source_acceptance_decision_recorded_count: usize,
    pub source_acceptance_decision_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_receipt_store_written_count: usize,
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
    pub source_recording_boundary_attached_count: usize,
    pub source_decision_record_id_attached_count: usize,
    pub source_denial_receipt_id_attached_count: usize,
    pub recording_denial_reason_projected_count: usize,
    pub operator_acceptance_missing_count: usize,
    pub evidence_acceptance_missing_count: usize,
    pub decision_recording_disabled_count: usize,
    pub acceptance_decision_recorded_count: usize,
    pub acceptance_decision_persisted_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub recording_denial_receipt_readback_ready: bool,
    pub acceptance_decision_recording_allowed: bool,
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceEntry {
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_boundary_entry_id: String,
    pub source_recording_boundary_id: &'static str,
    pub source_recording_boundary_route: String,
    pub source_acceptance_decision_record_id: String,
    pub source_acceptance_decision_idempotency_key: String,
    pub source_denial_receipt_id: String,
    pub source_receipt_id: &'static str,
    pub source_receipt_path: &'static str,
    pub denial_receipt_id: String,
    pub denial_receipt_route: String,
    pub denial_receipt_digest: String,
    pub denial_receipt_schema_version: &'static str,
    pub denial_receipt_idempotency_key: String,
    pub recording_denial_reason: &'static str,
    pub recording_denial_state: &'static str,
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
    pub denial_receipt_projected: bool,
    pub denial_receipt_digest_projected: bool,
    pub denial_receipt_readback_route_projected: bool,
    pub denial_receipt_idempotency_key_projected: bool,
    pub source_recording_boundary_attached: bool,
    pub source_decision_record_id_attached: bool,
    pub source_denial_receipt_id_attached: bool,
    pub recording_denial_reason_projected: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub recording_precondition_missing: bool,
    pub decision_recording_disabled: bool,
    pub acceptance_decision_recording_allowed: bool,
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceSideEffects
{
    pub denial_receipt_persisted: bool,
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub receipt_persisted: bool,
    pub receipt_store_written: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
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

pub fn controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceReport,
    > = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report();
    let entries =
        controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_entries();

    let denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_projected)
        .count();
    let denial_receipt_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_digest_projected)
        .count();
    let denial_receipt_readback_route_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_readback_route_projected)
        .count();
    let denial_receipt_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_idempotency_key_projected)
        .count();
    let denial_receipt_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.denial_receipt_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let source_recording_boundary_attached_count = entries
        .iter()
        .filter(|entry| entry.source_recording_boundary_attached)
        .count();
    let source_decision_record_id_attached_count = entries
        .iter()
        .filter(|entry| entry.source_decision_record_id_attached)
        .count();
    let source_denial_receipt_id_attached_count = entries
        .iter()
        .filter(|entry| entry.source_denial_receipt_id_attached)
        .count();
    let recording_denial_reason_projected_count = entries
        .iter()
        .filter(|entry| entry.recording_denial_reason_projected)
        .count();
    let operator_acceptance_missing_count = entries
        .iter()
        .filter(|entry| entry.operator_acceptance_required && !entry.operator_acceptance_present)
        .count();
    let evidence_acceptance_missing_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required && !entry.evidence_acceptance_present)
        .count();
    let decision_recording_disabled_count = entries
        .iter()
        .filter(|entry| entry.decision_recording_disabled)
        .count();
    let acceptance_decision_recorded_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_recorded)
        .count();
    let acceptance_decision_persisted_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_persisted)
        .count();
    let denial_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_persisted)
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

    let recording_denial_receipt_readback_ready = source
        .acceptance_decision_recording_boundary_readback_ready
        && source.boundary_entry_count == 7
        && source.operator_acceptance_present_count == 0
        && source.evidence_acceptance_present_count == 0
        && source.decision_recording_allowed_count == 0
        && source.acceptance_decision_recorded_count == 0
        && source.acceptance_decision_persisted_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && denial_receipt_projected_count == 7
        && denial_receipt_digest_projected_count == 7
        && denial_receipt_readback_route_projected_count == 7
        && denial_receipt_idempotency_key_projected_count == 7
        && denial_receipt_idempotency_key_unique_count == 7
        && source_recording_boundary_attached_count == 7
        && source_decision_record_id_attached_count == 7
        && source_denial_receipt_id_attached_count == 7
        && recording_denial_reason_projected_count == 7
        && operator_acceptance_missing_count == 7
        && evidence_acceptance_missing_count == 7
        && decision_recording_disabled_count == 7
        && acceptance_decision_recorded_count == 0
        && acceptance_decision_persisted_count == 0
        && denial_receipt_persisted_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "recording_denial_receipt_projected_without_persistence"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.recording_denial_state == "denied_not_persisted"
                && entry.denial_receipt_projected
                && entry.denial_receipt_digest_projected
                && entry.denial_receipt_readback_route_projected
                && entry.denial_receipt_idempotency_key_projected
                && entry.source_recording_boundary_attached
                && entry.source_decision_record_id_attached
                && entry.source_denial_receipt_id_attached
                && entry.recording_denial_reason_projected
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.recording_precondition_missing
                && entry.decision_recording_disabled
                && !entry.acceptance_decision_recording_allowed
                && !entry.acceptance_decision_recorded
                && !entry.acceptance_decision_persisted
                && !entry.denial_receipt_persistence_allowed
                && !entry.denial_receipt_persisted
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

    ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence",
        status: if recording_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_acceptance_decision_recording_boundary_ready:
            source.acceptance_decision_recording_boundary_readback_ready,
        source_boundary_entry_count: source.boundary_entry_count,
        source_operator_acceptance_present_count: source.operator_acceptance_present_count,
        source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
        source_decision_recording_allowed_count: source.decision_recording_allowed_count,
        source_acceptance_decision_recorded_count: source.acceptance_decision_recorded_count,
        source_acceptance_decision_persisted_count: source.acceptance_decision_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
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
        source_recording_boundary_attached_count,
        source_decision_record_id_attached_count,
        source_denial_receipt_id_attached_count,
        recording_denial_reason_projected_count,
        operator_acceptance_missing_count,
        evidence_acceptance_missing_count,
        decision_recording_disabled_count,
        acceptance_decision_recorded_count,
        acceptance_decision_persisted_count,
        denial_receipt_persisted_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        recording_denial_receipt_readback_ready,
        acceptance_decision_recording_allowed: false,
        acceptance_decision_recorded: false,
        acceptance_decision_persisted: false,
        denial_receipt_persistence_allowed: false,
        denial_receipt_persisted: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
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
            "acceptance_decision_recording_disabled",
            "denial_receipt_persistence_disabled",
            "receipt_persistence_disabled",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_READBACK_WITHOUT_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceEntry> {
    controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceEntry {
                id: format!(
                    "evidence_receipt_store_recording_denial_receipt_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_boundary_entry_id: entry.id,
                source_recording_boundary_id: entry.recording_boundary_id,
                source_recording_boundary_route: entry.recording_boundary_route,
                source_acceptance_decision_record_id: entry.acceptance_decision_record_id,
                source_acceptance_decision_idempotency_key: entry.acceptance_decision_idempotency_key,
                source_denial_receipt_id: entry.denial_receipt_id,
                source_receipt_id: entry.receipt_id,
                source_receipt_path: entry.receipt_path,
                denial_receipt_id: format!(
                    "recording-denial-receipt:controlled-live-evidence-receipt-store:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_route: format!("{DENIAL_RECEIPT_COLLECTION_ROUTE}/{hyphenated}"),
                denial_receipt_digest: format!(
                    "sha256:controlled-live-evidence-receipt-store-recording-denial:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_schema_version: DENIAL_RECEIPT_SCHEMA_VERSION,
                denial_receipt_idempotency_key: format!(
                    "controlled-live-evidence-receipt-store.recording-denial-receipt.idempotency.{}",
                    entry.source_blocker_id
                ),
                recording_denial_reason:
                    "operator_acceptance_missing_evidence_acceptance_missing_recording_disabled",
                recording_denial_state: "denied_not_persisted",
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "recording_denial_receipt_projected_without_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                denial_receipt_projected: true,
                denial_receipt_digest_projected: true,
                denial_receipt_readback_route_projected: true,
                denial_receipt_idempotency_key_projected: true,
                source_recording_boundary_attached: true,
                source_decision_record_id_attached: true,
                source_denial_receipt_id_attached: true,
                recording_denial_reason_projected: true,
                operator_acceptance_required: entry.operator_acceptance_required,
                operator_acceptance_present: entry.operator_acceptance_present,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: entry.evidence_acceptance_present,
                recording_precondition_missing: entry.recording_precondition_missing,
                decision_recording_disabled: !entry.acceptance_decision_recording_allowed,
                acceptance_decision_recording_allowed: false,
                acceptance_decision_recorded: false,
                acceptance_decision_persisted: false,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
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

impl ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceSideEffects {
    pub const fn none() -> Self {
        Self {
            denial_receipt_persisted: false,
            acceptance_decision_recorded: false,
            acceptance_decision_persisted: false,
            evidence_recorded: false,
            evidence_persisted: false,
            receipt_persisted: false,
            receipt_store_written: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            operator_packet_sent: false,
            operator_packet_persisted: false,
            approval_requested: false,
            approval_accepted: false,
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
    fn recording_denial_receipt_projects_all_entries_without_persistence() {
        let report =
            controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_acceptance_decision_recording_boundary_ready);
        assert_eq!(report.source_boundary_entry_count, 7);
        assert_eq!(report.source_operator_acceptance_present_count, 0);
        assert_eq!(report.source_evidence_acceptance_present_count, 0);
        assert_eq!(report.source_decision_recording_allowed_count, 0);
        assert_eq!(report.denial_receipt_entry_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_digest_projected_count, 7);
        assert_eq!(report.denial_receipt_readback_route_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_unique_count, 7);
        assert_eq!(report.source_recording_boundary_attached_count, 7);
        assert_eq!(report.source_decision_record_id_attached_count, 7);
        assert_eq!(report.source_denial_receipt_id_attached_count, 7);
        assert_eq!(report.recording_denial_reason_projected_count, 7);
        assert!(report.recording_denial_receipt_readback_ready);
    }

    #[test]
    fn recording_denial_receipt_keeps_persistence_and_live_closed() {
        let report =
            controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_report();

        assert_eq!(report.operator_acceptance_missing_count, 7);
        assert_eq!(report.evidence_acceptance_missing_count, 7);
        assert_eq!(report.decision_recording_disabled_count, 7);
        assert_eq!(report.acceptance_decision_recorded_count, 0);
        assert_eq!(report.acceptance_decision_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.acceptance_decision_recording_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.receipt_store_written);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreRecordingDenialReceiptReadbackWithoutPersistenceSideEffects::none()
        );
    }

    #[test]
    fn recording_denial_receipt_entries_are_stable_and_unpersisted() {
        let report =
            controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.denial_receipt_route
                == "readback://controlled-live/evidence-receipt-store/recording-denial-receipts/dirty-worktree-boundary"
            && entry.denial_receipt_id
                == "recording-denial-receipt:controlled-live-evidence-receipt-store:dirty_worktree_boundary:not-persisted"
            && entry.denial_receipt_idempotency_key
                == "controlled-live-evidence-receipt-store.recording-denial-receipt.idempotency.dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.denial_receipt_schema_version == DENIAL_RECEIPT_SCHEMA_VERSION
                && entry.recording_denial_state == "denied_not_persisted"
                && entry.denial_receipt_projected
                && entry.denial_receipt_digest_projected
                && entry.denial_receipt_readback_route_projected
                && entry.denial_receipt_idempotency_key_projected
                && entry.source_recording_boundary_attached
                && entry.source_decision_record_id_attached
                && entry.source_denial_receipt_id_attached
                && entry.recording_denial_reason_projected
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.recording_precondition_missing
                && entry.decision_recording_disabled
                && !entry.acceptance_decision_recording_allowed
                && !entry.acceptance_decision_recorded
                && !entry.acceptance_decision_persisted
                && !entry.denial_receipt_persistence_allowed
                && !entry.denial_receipt_persisted
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
