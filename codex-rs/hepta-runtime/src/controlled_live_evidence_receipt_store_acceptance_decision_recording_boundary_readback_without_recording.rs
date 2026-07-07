use std::collections::BTreeSet;

use crate::controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance::controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE:
    &str = "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence";

const RECORDING_BOUNDARY_ID: &str =
    "controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary";
const RECORDING_BOUNDARY_ROUTE: &str =
    "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary";
const ACCEPTANCE_DECISION_RECORD_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_decision_record_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_acceptance_packet_ready: bool,
    pub source_packet_entry_count: usize,
    pub source_operator_acceptance_present_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_acceptance_decision_recorded_count: usize,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_live_execution_allowed: bool,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: &'static str,
    pub acceptance_decision_record_schema_version: &'static str,
    pub boundary_entry_count: usize,
    pub boundary_projected_count: usize,
    pub boundary_ready_count: usize,
    pub decision_record_schema_projected_count: usize,
    pub acceptance_decision_request_attached_count: usize,
    pub operator_acceptance_required_count: usize,
    pub operator_acceptance_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub recording_precondition_missing_count: usize,
    pub decision_recording_allowed_count: usize,
    pub acceptance_decision_recorded_count: usize,
    pub acceptance_decision_persisted_count: usize,
    pub decision_idempotency_key_projected_count: usize,
    pub decision_idempotency_key_unique_count: usize,
    pub post_record_readback_route_projected_count: usize,
    pub rollback_anchor_projected_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub acceptance_decision_recording_boundary_readback_ready: bool,
    pub acceptance_decision_recording_allowed: bool,
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
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
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_packet_entry_id: String,
    pub source_acceptance_decision_request_id: String,
    pub source_acceptance_decision_request_route: String,
    pub source_non_acceptance_receipt_id: String,
    pub source_operator_acceptance_packet_id: &'static str,
    pub source_operator_acceptance_packet_route: &'static str,
    pub receipt_id: &'static str,
    pub receipt_path: &'static str,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: String,
    pub acceptance_decision_record_id: String,
    pub acceptance_decision_record_schema_version: &'static str,
    pub acceptance_decision_idempotency_key: String,
    pub post_record_readback_route: String,
    pub rollback_anchor: String,
    pub denial_receipt_id: String,
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
    pub boundary_projected: bool,
    pub boundary_ready: bool,
    pub source_packet_ready: bool,
    pub source_packet_sent: bool,
    pub source_packet_persisted: bool,
    pub decision_record_schema_projected: bool,
    pub acceptance_decision_request_attached: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub recording_precondition_missing: bool,
    pub acceptance_decision_recording_allowed: bool,
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
    pub decision_idempotency_key_projected: bool,
    pub post_record_readback_route_projected: bool,
    pub rollback_anchor_projected: bool,
    pub denial_receipt_projected: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingSideEffects
{
    pub acceptance_decision_recorded: bool,
    pub acceptance_decision_persisted: bool,
    pub denial_receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report();
    let entries =
        controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_entries();

    let boundary_projected_count = entries
        .iter()
        .filter(|entry| entry.boundary_projected)
        .count();
    let boundary_ready_count = entries.iter().filter(|entry| entry.boundary_ready).count();
    let decision_record_schema_projected_count = entries
        .iter()
        .filter(|entry| entry.decision_record_schema_projected)
        .count();
    let acceptance_decision_request_attached_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_request_attached)
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
    let recording_precondition_missing_count = entries
        .iter()
        .filter(|entry| entry.recording_precondition_missing)
        .count();
    let decision_recording_allowed_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_recording_allowed)
        .count();
    let acceptance_decision_recorded_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_recorded)
        .count();
    let acceptance_decision_persisted_count = entries
        .iter()
        .filter(|entry| entry.acceptance_decision_persisted)
        .count();
    let decision_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.decision_idempotency_key_projected)
        .count();
    let decision_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.acceptance_decision_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let post_record_readback_route_projected_count = entries
        .iter()
        .filter(|entry| entry.post_record_readback_route_projected)
        .count();
    let rollback_anchor_projected_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_projected)
        .count();
    let denial_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_projected)
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

    let acceptance_decision_recording_boundary_readback_ready = source
        .operator_acceptance_packet_readback_ready
        && source.packet_entry_count == 7
        && source.operator_acceptance_present_count == 0
        && source.evidence_acceptance_present_count == 0
        && source.acceptance_decision_recorded_count == 0
        && !source.operator_packet_sent
        && !source.operator_packet_persisted
        && !source.live_execution_allowed
        && entries.len() == 7
        && boundary_projected_count == 7
        && boundary_ready_count == 7
        && decision_record_schema_projected_count == 7
        && acceptance_decision_request_attached_count == 7
        && operator_acceptance_required_count == 7
        && operator_acceptance_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && recording_precondition_missing_count == 7
        && decision_recording_allowed_count == 0
        && acceptance_decision_recorded_count == 0
        && acceptance_decision_persisted_count == 0
        && decision_idempotency_key_projected_count == 7
        && decision_idempotency_key_unique_count == 7
        && post_record_readback_route_projected_count == 7
        && rollback_anchor_projected_count == 7
        && denial_receipt_projected_count == 7
        && denial_receipt_persisted_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state
                == "acceptance_decision_recording_boundary_projected_without_recording"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.boundary_projected
                && entry.boundary_ready
                && entry.source_packet_ready
                && !entry.source_packet_sent
                && !entry.source_packet_persisted
                && entry.decision_record_schema_projected
                && entry.acceptance_decision_request_attached
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.recording_precondition_missing
                && !entry.acceptance_decision_recording_allowed
                && !entry.acceptance_decision_recorded
                && !entry.acceptance_decision_persisted
                && entry.decision_idempotency_key_projected
                && entry.post_record_readback_route_projected
                && entry.rollback_anchor_projected
                && entry.denial_receipt_projected
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

    ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording",
        status: if acceptance_decision_recording_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_operator_acceptance_packet_ready: source.operator_acceptance_packet_readback_ready,
        source_packet_entry_count: source.packet_entry_count,
        source_operator_acceptance_present_count: source.operator_acceptance_present_count,
        source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
        source_acceptance_decision_recorded_count: source.acceptance_decision_recorded_count,
        source_operator_packet_sent: source.operator_packet_sent,
        source_operator_packet_persisted: source.operator_packet_persisted,
        source_live_execution_allowed: source.live_execution_allowed,
        recording_boundary_id: RECORDING_BOUNDARY_ID,
        recording_boundary_route: RECORDING_BOUNDARY_ROUTE,
        acceptance_decision_record_schema_version: ACCEPTANCE_DECISION_RECORD_SCHEMA_VERSION,
        boundary_entry_count: entries.len(),
        boundary_projected_count,
        boundary_ready_count,
        decision_record_schema_projected_count,
        acceptance_decision_request_attached_count,
        operator_acceptance_required_count,
        operator_acceptance_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        recording_precondition_missing_count,
        decision_recording_allowed_count,
        acceptance_decision_recorded_count,
        acceptance_decision_persisted_count,
        decision_idempotency_key_projected_count,
        decision_idempotency_key_unique_count,
        post_record_readback_route_projected_count,
        rollback_anchor_projected_count,
        denial_receipt_projected_count,
        denial_receipt_persisted_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        acceptance_decision_recording_boundary_readback_ready,
        acceptance_decision_recording_allowed: false,
        acceptance_decision_recorded: false,
        acceptance_decision_persisted: false,
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
            "acceptance_decision_persistence_disabled",
            "denial_receipt_persistence_disabled",
            "evidence_recording_disabled",
            "receipt_persistence_disabled",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingEntry,
>{
    controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_decision_recording_boundary_without_recording_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_packet_entry_id: entry.id,
                source_acceptance_decision_request_id: entry.acceptance_decision_request_id,
                source_acceptance_decision_request_route: entry.acceptance_decision_request_route,
                source_non_acceptance_receipt_id: entry.non_acceptance_receipt_id,
                source_operator_acceptance_packet_id: entry.operator_acceptance_packet_id,
                source_operator_acceptance_packet_route: entry.operator_acceptance_packet_route,
                receipt_id: entry.receipt_id,
                receipt_path: entry.receipt_path,
                recording_boundary_id: RECORDING_BOUNDARY_ID,
                recording_boundary_route: format!("{RECORDING_BOUNDARY_ROUTE}/{hyphenated}"),
                acceptance_decision_record_id: format!(
                    "acceptance-decision-record:controlled-live-evidence-receipt-store:{}:not-recorded",
                    entry.source_blocker_id
                ),
                acceptance_decision_record_schema_version: ACCEPTANCE_DECISION_RECORD_SCHEMA_VERSION,
                acceptance_decision_idempotency_key: format!(
                    "controlled-live-evidence-receipt-store.acceptance-decision-recording.idempotency.{}",
                    entry.source_blocker_id
                ),
                post_record_readback_route: format!(
                    "{RECORDING_BOUNDARY_ROUTE}/post-record/{hyphenated}"
                ),
                rollback_anchor: format!(
                    "rollback-anchor://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/{hyphenated}"
                ),
                denial_receipt_id: format!(
                    "acceptance-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "acceptance_decision_recording_boundary_projected_without_recording",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                boundary_projected: true,
                boundary_ready: true,
                source_packet_ready: entry.packet_ready,
                source_packet_sent: entry.operator_packet_sent,
                source_packet_persisted: entry.operator_packet_persisted,
                decision_record_schema_projected: true,
                acceptance_decision_request_attached: entry.acceptance_decision_request_projected,
                operator_acceptance_required: entry.operator_acceptance_required,
                operator_acceptance_present: entry.operator_acceptance_present,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: entry.evidence_acceptance_present,
                recording_precondition_missing: true,
                acceptance_decision_recording_allowed: false,
                acceptance_decision_recorded: false,
                acceptance_decision_persisted: false,
                decision_idempotency_key_projected: true,
                post_record_readback_route_projected: true,
                rollback_anchor_projected: true,
                denial_receipt_projected: true,
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

impl ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingSideEffects {
    pub const fn none() -> Self {
        Self {
            acceptance_decision_recorded: false,
            acceptance_decision_persisted: false,
            denial_receipt_persisted: false,
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
    fn acceptance_decision_recording_boundary_projects_all_entries_without_recording() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_acceptance_packet_ready);
        assert_eq!(report.source_packet_entry_count, 7);
        assert_eq!(report.source_operator_acceptance_present_count, 0);
        assert_eq!(report.source_evidence_acceptance_present_count, 0);
        assert_eq!(report.source_acceptance_decision_recorded_count, 0);
        assert_eq!(report.boundary_entry_count, 7);
        assert_eq!(report.boundary_projected_count, 7);
        assert_eq!(report.boundary_ready_count, 7);
        assert_eq!(report.decision_record_schema_projected_count, 7);
        assert_eq!(report.acceptance_decision_request_attached_count, 7);
        assert_eq!(report.operator_acceptance_required_count, 7);
        assert_eq!(report.operator_acceptance_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.recording_precondition_missing_count, 7);
        assert!(report.acceptance_decision_recording_boundary_readback_ready);
    }

    #[test]
    fn acceptance_decision_recording_boundary_keeps_recording_and_persistence_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report();

        assert_eq!(report.decision_recording_allowed_count, 0);
        assert_eq!(report.acceptance_decision_recorded_count, 0);
        assert_eq!(report.acceptance_decision_persisted_count, 0);
        assert_eq!(report.decision_idempotency_key_projected_count, 7);
        assert_eq!(report.decision_idempotency_key_unique_count, 7);
        assert_eq!(report.post_record_readback_route_projected_count, 7);
        assert_eq!(report.rollback_anchor_projected_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.acceptance_decision_recording_allowed);
        assert!(!report.acceptance_decision_recorded);
        assert!(!report.acceptance_decision_persisted);
        assert!(!report.receipt_store_written);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceDecisionRecordingBoundaryReadbackWithoutRecordingSideEffects::none()
        );
    }

    #[test]
    fn acceptance_decision_recording_boundary_entries_are_stable_and_unrecorded() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.recording_boundary_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/dirty-worktree-boundary"
            && entry.acceptance_decision_record_id
                == "acceptance-decision-record:controlled-live-evidence-receipt-store:dirty_worktree_boundary:not-recorded"
            && entry.acceptance_decision_idempotency_key
                == "controlled-live-evidence-receipt-store.acceptance-decision-recording.idempotency.dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.recording_boundary_id == RECORDING_BOUNDARY_ID
                && entry.acceptance_decision_record_schema_version
                    == ACCEPTANCE_DECISION_RECORD_SCHEMA_VERSION
                && entry.boundary_projected
                && entry.boundary_ready
                && entry.source_packet_ready
                && !entry.source_packet_sent
                && !entry.source_packet_persisted
                && entry.decision_record_schema_projected
                && entry.acceptance_decision_request_attached
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.recording_precondition_missing
                && !entry.acceptance_decision_recording_allowed
                && !entry.acceptance_decision_recorded
                && !entry.acceptance_decision_persisted
                && entry.decision_idempotency_key_projected
                && entry.post_record_readback_route_projected
                && entry.rollback_anchor_projected
                && entry.denial_receipt_projected
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
