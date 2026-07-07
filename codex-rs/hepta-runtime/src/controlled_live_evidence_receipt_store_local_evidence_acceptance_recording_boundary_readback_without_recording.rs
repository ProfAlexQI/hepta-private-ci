use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording::controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence";

const RECORDING_BOUNDARY_ID: &str =
    "controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-boundary";
const RECORDING_BOUNDARY_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-boundary";
const ACCEPTANCE_SOURCE_RECORD_SCHEMA_VERSION: &str =
    "controlled_live_local_evidence_acceptance_source_record_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingReport
{
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_acceptance_source_readback_ready: bool,
    pub source_acceptance_source_entry_count: usize,
    pub source_acceptance_source_projected_count: usize,
    pub source_acceptance_source_schema_projected_count: usize,
    pub source_acceptance_source_policy_projected_count: usize,
    pub source_acceptance_source_idempotency_key_unique_count: usize,
    pub source_open_preconditions_attached_count: usize,
    pub source_acceptance_source_recording_required_count: usize,
    pub source_acceptance_source_recording_allowed_count: usize,
    pub source_acceptance_source_recorded_count: usize,
    pub source_acceptance_source_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: &'static str,
    pub acceptance_source_record_schema_version: &'static str,
    pub boundary_entry_count: usize,
    pub boundary_projected_count: usize,
    pub boundary_ready_count: usize,
    pub source_acceptance_source_attached_count: usize,
    pub record_schema_projected_count: usize,
    pub recording_precondition_missing_count: usize,
    pub acceptance_source_recording_required_count: usize,
    pub acceptance_source_recording_allowed_count: usize,
    pub acceptance_source_recorded_count: usize,
    pub acceptance_source_persisted_count: usize,
    pub recording_idempotency_key_projected_count: usize,
    pub recording_idempotency_key_unique_count: usize,
    pub post_record_readback_route_projected_count: usize,
    pub rollback_anchor_projected_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_persisted_count: usize,
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
    pub local_evidence_acceptance_recording_boundary_readback_ready: bool,
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
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_acceptance_source_id: String,
    pub source_acceptance_source_route: String,
    pub source_acceptance_source_schema: &'static str,
    pub source_acceptance_source_policy_id: String,
    pub source_acceptance_source_idempotency_key: String,
    pub source_acceptance_source_readback_route: String,
    pub source_acceptance_source_recording_boundary_route: String,
    pub recording_boundary_id: String,
    pub recording_boundary_route: String,
    pub acceptance_source_record_id: String,
    pub acceptance_source_record_schema_version: &'static str,
    pub acceptance_source_record_idempotency_key: String,
    pub post_record_readback_route: String,
    pub rollback_anchor: String,
    pub denial_receipt_id: String,
    pub denial_receipt_route: String,
    pub denial_reason: &'static str,
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
    pub source_acceptance_source_attached: bool,
    pub record_schema_projected: bool,
    pub recording_precondition_missing: bool,
    pub acceptance_source_recording_required: bool,
    pub acceptance_source_recording_allowed: bool,
    pub acceptance_source_recorded: bool,
    pub acceptance_source_persisted: bool,
    pub recording_idempotency_key_projected: bool,
    pub post_record_readback_route_projected: bool,
    pub rollback_anchor_projected: bool,
    pub denial_receipt_projected: bool,
    pub denial_receipt_persisted: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingSideEffects
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
            let source =
                controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report();
            let entries =
                controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_entries();

            let boundary_projected_count = entries
                .iter()
                .filter(|entry| entry.boundary_projected)
                .count();
            let boundary_ready_count = entries.iter().filter(|entry| entry.boundary_ready).count();
            let source_acceptance_source_attached_count = entries
                .iter()
                .filter(|entry| entry.source_acceptance_source_attached)
                .count();
            let record_schema_projected_count = entries
                .iter()
                .filter(|entry| entry.record_schema_projected)
                .count();
            let recording_precondition_missing_count = entries
                .iter()
                .filter(|entry| entry.recording_precondition_missing)
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
            let recording_idempotency_key_projected_count = entries
                .iter()
                .filter(|entry| entry.recording_idempotency_key_projected)
                .count();
            let mut recording_idempotency_keys = entries
                .iter()
                .map(|entry| entry.acceptance_source_record_idempotency_key.clone())
                .collect::<Vec<_>>();
            recording_idempotency_keys.sort();
            recording_idempotency_keys.dedup();
            let recording_idempotency_key_unique_count = recording_idempotency_keys.len();
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

            let local_evidence_acceptance_recording_boundary_readback_ready =
                source.local_evidence_acceptance_source_readback_ready
                    && source.acceptance_source_entry_count == 7
                    && source.acceptance_source_projected_count == 7
                    && source.acceptance_source_schema_projected_count == 7
                    && source.acceptance_source_policy_projected_count == 7
                    && source.acceptance_source_idempotency_key_unique_count == 7
                    && source.source_open_preconditions_attached_count == 7
                    && source.acceptance_source_recording_required_count == 7
                    && source.acceptance_source_recording_allowed_count == 0
                    && source.acceptance_source_recorded_count == 0
                    && source.acceptance_source_persisted_count == 0
                    && source.evidence_acceptance_recorded_count == 0
                    && source.evidence_recorded_count == 0
                    && source.receipt_store_written_count == 0
                    && source.receipt_persisted_count == 0
                    && !source.live_execution_allowed
                    && entries.len() == 7
                    && boundary_projected_count == 7
                    && boundary_ready_count == 7
                    && source_acceptance_source_attached_count == 7
                    && record_schema_projected_count == 7
                    && recording_precondition_missing_count == 7
                    && acceptance_source_recording_required_count == 7
                    && acceptance_source_recording_allowed_count == 0
                    && acceptance_source_recorded_count == 0
                    && acceptance_source_persisted_count == 0
                    && recording_idempotency_key_projected_count == 7
                    && recording_idempotency_key_unique_count == 7
                    && post_record_readback_route_projected_count == 7
                    && rollback_anchor_projected_count == 7
                    && denial_receipt_projected_count == 7
                    && denial_receipt_persisted_count == 0
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

            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingReport {
                runtime: "hepta",
                surface:
                    "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording",
                status: if local_evidence_acceptance_recording_boundary_readback_ready {
                    "ready_blocked"
                } else {
                    "blocked"
                },
                gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE,
                schema_version:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION,
                plugin_id: "hepta-system@hepta-local",
                source_acceptance_source_readback_ready:
                    source.local_evidence_acceptance_source_readback_ready,
                source_acceptance_source_entry_count: source.acceptance_source_entry_count,
                source_acceptance_source_projected_count:
                    source.acceptance_source_projected_count,
                source_acceptance_source_schema_projected_count:
                    source.acceptance_source_schema_projected_count,
                source_acceptance_source_policy_projected_count:
                    source.acceptance_source_policy_projected_count,
                source_acceptance_source_idempotency_key_unique_count:
                    source.acceptance_source_idempotency_key_unique_count,
                source_open_preconditions_attached_count:
                    source.source_open_preconditions_attached_count,
                source_acceptance_source_recording_required_count:
                    source.acceptance_source_recording_required_count,
                source_acceptance_source_recording_allowed_count:
                    source.acceptance_source_recording_allowed_count,
                source_acceptance_source_recorded_count: source.acceptance_source_recorded_count,
                source_acceptance_source_persisted_count: source.acceptance_source_persisted_count,
                source_evidence_acceptance_recorded_count:
                    source.evidence_acceptance_recorded_count,
                source_evidence_recorded_count: source.evidence_recorded_count,
                source_receipt_store_written_count: source.receipt_store_written_count,
                source_receipt_persisted_count: source.receipt_persisted_count,
                source_live_execution_allowed: source.live_execution_allowed,
                recording_boundary_id: RECORDING_BOUNDARY_ID,
                recording_boundary_route: RECORDING_BOUNDARY_ROUTE,
                acceptance_source_record_schema_version: ACCEPTANCE_SOURCE_RECORD_SCHEMA_VERSION,
                boundary_entry_count: entries.len(),
                boundary_projected_count,
                boundary_ready_count,
                source_acceptance_source_attached_count,
                record_schema_projected_count,
                recording_precondition_missing_count,
                acceptance_source_recording_required_count,
                acceptance_source_recording_allowed_count,
                acceptance_source_recorded_count,
                acceptance_source_persisted_count,
                recording_idempotency_key_projected_count,
                recording_idempotency_key_unique_count,
                post_record_readback_route_projected_count,
                rollback_anchor_projected_count,
                denial_receipt_projected_count,
                denial_receipt_persisted_count,
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
                local_evidence_acceptance_recording_boundary_readback_ready,
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
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE,
                side_effects:
                    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingSideEffects::none(),
            }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_recording_boundary_without_recording_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_acceptance_source_id: entry.local_evidence_acceptance_source_id,
                source_acceptance_source_route: entry.local_evidence_acceptance_source_route,
                source_acceptance_source_schema: entry.local_evidence_acceptance_source_schema,
                source_acceptance_source_policy_id: entry.local_evidence_acceptance_source_policy_id,
                source_acceptance_source_idempotency_key:
                    entry.local_evidence_acceptance_source_idempotency_key,
                source_acceptance_source_readback_route:
                    entry.local_evidence_acceptance_source_readback_route,
                source_acceptance_source_recording_boundary_route:
                    entry.local_evidence_acceptance_recording_boundary_route,
                recording_boundary_id: format!(
                    "local-evidence-acceptance-recording-boundary:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                recording_boundary_route: format!(
                    "{RECORDING_BOUNDARY_ROUTE}/{hyphenated}"
                ),
                acceptance_source_record_id: format!(
                    "local-evidence-acceptance-source-record:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                acceptance_source_record_schema_version: ACCEPTANCE_SOURCE_RECORD_SCHEMA_VERSION,
                acceptance_source_record_idempotency_key: format!(
                    "local-evidence-acceptance-source-record-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_record_readback_route: format!(
                    "{RECORDING_BOUNDARY_ROUTE}/{hyphenated}/post-record-readback"
                ),
                rollback_anchor: format!(
                    "rollback-anchor:local-evidence-acceptance-recording-boundary:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                denial_receipt_id: format!(
                    "local-evidence-acceptance-source-recording-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                denial_receipt_route: format!(
                    "{RECORDING_BOUNDARY_ROUTE}/{hyphenated}/denial-receipt"
                ),
                denial_reason:
                    "local_evidence_acceptance_source_recording_disabled_open_preconditions_missing",
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "local_evidence_acceptance_recording_boundary_projected_without_recording",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                boundary_projected: true,
                boundary_ready: entry.dev_evidence_acceptance_source_projected
                    && entry.acceptance_source_schema_projected
                    && entry.acceptance_source_idempotency_key_projected
                    && !entry.acceptance_source_recorded
                    && !entry.evidence_recorded
                    && !entry.live_mutation_allowed,
                source_acceptance_source_attached: entry.dev_evidence_acceptance_source_projected,
                record_schema_projected: true,
                recording_precondition_missing: true,
                acceptance_source_recording_required: entry.acceptance_source_recording_required,
                acceptance_source_recording_allowed: false,
                acceptance_source_recorded: false,
                acceptance_source_persisted: false,
                recording_idempotency_key_projected: true,
                post_record_readback_route_projected: true,
                rollback_anchor_projected: true,
                denial_receipt_projected: true,
                denial_receipt_persisted: false,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: false,
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingSideEffects
{
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
    fn local_evidence_acceptance_recording_boundary_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.local_evidence_acceptance_recording_boundary_readback_ready);
        assert_eq!(report.source_acceptance_source_entry_count, 7);
        assert_eq!(report.source_acceptance_source_projected_count, 7);
        assert_eq!(report.boundary_entry_count, 7);
        assert_eq!(report.boundary_projected_count, 7);
        assert_eq!(report.boundary_ready_count, 7);
        assert_eq!(report.source_acceptance_source_attached_count, 7);
        assert_eq!(report.record_schema_projected_count, 7);
        assert_eq!(report.recording_precondition_missing_count, 7);
        assert_eq!(report.recording_idempotency_key_projected_count, 7);
        assert_eq!(report.recording_idempotency_key_unique_count, 7);
        assert_eq!(report.post_record_readback_route_projected_count, 7);
        assert_eq!(report.rollback_anchor_projected_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
    }

    #[test]
    fn local_evidence_acceptance_recording_boundary_keeps_recording_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_report();

        assert_eq!(report.source_acceptance_source_recording_allowed_count, 0);
        assert_eq!(report.source_acceptance_source_recorded_count, 0);
        assert_eq!(report.source_acceptance_source_persisted_count, 0);
        assert_eq!(report.acceptance_source_recording_allowed_count, 0);
        assert_eq!(report.acceptance_source_recorded_count, 0);
        assert_eq!(report.acceptance_source_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
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
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceRecordingBoundaryReadbackWithoutRecordingSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_recording_boundary_entries_are_stable_and_unrecorded() {
        let entries =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording_entries();

        assert!(entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.recording_boundary_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-boundary/dirty-worktree-boundary"
                && entry.acceptance_source_record_idempotency_key
                    == "local-evidence-acceptance-source-record-idempotency:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(entries.iter().all(|entry| {
            entry.observed_state
                == "local_evidence_acceptance_recording_boundary_projected_without_recording"
                && entry.boundary_projected
                && entry.boundary_ready
                && entry.source_acceptance_source_attached
                && entry.record_schema_projected
                && entry.recording_precondition_missing
                && entry.acceptance_source_recording_required
                && !entry.acceptance_source_recording_allowed
                && !entry.acceptance_source_recorded
                && !entry.acceptance_source_persisted
                && entry.recording_idempotency_key_projected
                && entry.post_record_readback_route_projected
                && entry.rollback_anchor_projected
                && entry.denial_receipt_projected
                && !entry.denial_receipt_persisted
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && !entry.evidence_acceptance_recording_allowed
                && !entry.evidence_acceptance_recorded
                && !entry.evidence_recorded
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        }));
    }
}
