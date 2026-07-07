use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence";

const WRITE_ATTEMPT_RECORDING_BOUNDARY_ID: &str =
    "controlled-live-evidence-receipt-store-write-attempt-recording-boundary";
const WRITE_ATTEMPT_RECORDING_BOUNDARY_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary";
const WRITE_ATTEMPT_RECORD_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_write_attempt_record_v1";
const WRITE_ATTEMPT_RECORDING_DENIAL_REASON: &str = "write_attempt_recording_disabled_acceptance_authority_missing_evidence_acceptance_missing_write_grant_missing";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_write_positive_preconditions_ready: bool,
    pub source_precondition_entry_count: usize,
    pub source_write_preconditions_missing_count: usize,
    pub source_receipt_store_write_allowed_count: usize,
    pub source_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: &'static str,
    pub write_attempt_record_schema_version: &'static str,
    pub boundary_entry_count: usize,
    pub boundary_projected_count: usize,
    pub boundary_ready_count: usize,
    pub write_attempt_record_schema_projected_count: usize,
    pub source_write_preconditions_attached_count: usize,
    pub acceptance_authority_required_count: usize,
    pub acceptance_authority_present_count: usize,
    pub operator_write_approval_required_count: usize,
    pub operator_write_approval_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub receipt_store_write_grant_required_count: usize,
    pub receipt_store_write_grant_present_count: usize,
    pub write_attempt_recording_precondition_missing_count: usize,
    pub write_attempt_recording_allowed_count: usize,
    pub write_attempt_recorded_count: usize,
    pub write_attempt_persisted_count: usize,
    pub write_attempt_idempotency_key_projected_count: usize,
    pub write_attempt_idempotency_key_unique_count: usize,
    pub post_record_readback_route_projected_count: usize,
    pub rollback_anchor_projected_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub denial_receipt_digest_projected_count: usize,
    pub receipt_store_write_allowed_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub write_attempt_recording_boundary_readback_ready: bool,
    pub write_attempt_recording_allowed: bool,
    pub write_attempt_recorded: bool,
    pub write_attempt_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub receipt_persistence_allowed: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingEntry,
    >,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_precondition_entry_id: String,
    pub source_write_precondition_set_id: String,
    pub source_write_precondition_route: String,
    pub source_write_attempt_recording_precondition_id: String,
    pub source_receipt_store_write_denial_id: String,
    pub source_replay_idempotency_key: String,
    pub source_zero_effect_digest: String,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: String,
    pub write_attempt_record_id: String,
    pub write_attempt_record_schema_version: &'static str,
    pub write_attempt_idempotency_key: String,
    pub post_record_readback_route: String,
    pub rollback_anchor: String,
    pub denial_receipt_id: String,
    pub denial_receipt_route: String,
    pub denial_receipt_digest: String,
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
    pub source_packet_unsent: bool,
    pub source_write_denial_attached: bool,
    pub source_write_preconditions_attached: bool,
    pub boundary_projected: bool,
    pub boundary_ready: bool,
    pub write_attempt_record_schema_projected: bool,
    pub acceptance_authority_required: bool,
    pub acceptance_authority_present: bool,
    pub operator_write_approval_required: bool,
    pub operator_write_approval_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub receipt_store_write_grant_required: bool,
    pub receipt_store_write_grant_present: bool,
    pub write_attempt_recording_precondition_missing: bool,
    pub write_attempt_recording_allowed: bool,
    pub write_attempt_recorded: bool,
    pub write_attempt_persisted: bool,
    pub write_attempt_idempotency_key_projected: bool,
    pub post_record_readback_route_projected: bool,
    pub rollback_anchor_projected: bool,
    pub denial_receipt_projected: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub denial_receipt_digest_projected: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingSideEffects
{
    pub write_attempt_recorded: bool,
    pub write_attempt_persisted: bool,
    pub denial_receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingReport
{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_report();
    let entries =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_entries();

    let boundary_projected_count = entries
        .iter()
        .filter(|entry| entry.boundary_projected)
        .count();
    let boundary_ready_count = entries.iter().filter(|entry| entry.boundary_ready).count();
    let write_attempt_record_schema_projected_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_record_schema_projected)
        .count();
    let source_write_preconditions_attached_count = entries
        .iter()
        .filter(|entry| entry.source_write_preconditions_attached)
        .count();
    let acceptance_authority_required_count = entries
        .iter()
        .filter(|entry| entry.acceptance_authority_required)
        .count();
    let acceptance_authority_present_count = entries
        .iter()
        .filter(|entry| entry.acceptance_authority_present)
        .count();
    let operator_write_approval_required_count = entries
        .iter()
        .filter(|entry| entry.operator_write_approval_required)
        .count();
    let operator_write_approval_present_count = entries
        .iter()
        .filter(|entry| entry.operator_write_approval_present)
        .count();
    let evidence_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required)
        .count();
    let evidence_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_present)
        .count();
    let receipt_store_write_grant_required_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_write_grant_required)
        .count();
    let receipt_store_write_grant_present_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_write_grant_present)
        .count();
    let write_attempt_recording_precondition_missing_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_recording_precondition_missing)
        .count();
    let write_attempt_recording_allowed_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_recording_allowed)
        .count();
    let write_attempt_recorded_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_recorded)
        .count();
    let write_attempt_persisted_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_persisted)
        .count();
    let write_attempt_idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.write_attempt_idempotency_key_projected)
        .count();
    let write_attempt_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.write_attempt_idempotency_key.as_str())
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
    let denial_receipt_digest_projected_count = entries
        .iter()
        .filter(|entry| entry.denial_receipt_digest_projected)
        .count();
    let receipt_store_write_allowed_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_write_allowed)
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

    let write_attempt_recording_boundary_readback_ready = source
        .write_positive_preconditions_readback_ready
        && source.precondition_entry_count == 7
        && source.write_preconditions_missing_count == 7
        && source.receipt_store_write_allowed_count == 0
        && source.write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && boundary_projected_count == 7
        && boundary_ready_count == 7
        && write_attempt_record_schema_projected_count == 7
        && source_write_preconditions_attached_count == 7
        && acceptance_authority_required_count == 7
        && acceptance_authority_present_count == 0
        && operator_write_approval_required_count == 7
        && operator_write_approval_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && receipt_store_write_grant_required_count == 7
        && receipt_store_write_grant_present_count == 0
        && write_attempt_recording_precondition_missing_count == 7
        && write_attempt_recording_allowed_count == 0
        && write_attempt_recorded_count == 0
        && write_attempt_persisted_count == 0
        && write_attempt_idempotency_key_projected_count == 7
        && write_attempt_idempotency_key_unique_count == 7
        && post_record_readback_route_projected_count == 7
        && rollback_anchor_projected_count == 7
        && denial_receipt_projected_count == 7
        && denial_receipt_persisted_count == 0
        && denial_receipt_digest_projected_count == 7
        && receipt_store_write_allowed_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state
                == "receipt_store_write_attempt_recording_boundary_projected_without_recording"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.source_packet_unsent
                && entry.source_write_denial_attached
                && entry.source_write_preconditions_attached
                && entry.boundary_projected
                && entry.boundary_ready
                && entry.write_attempt_record_schema_projected
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && entry.operator_write_approval_required
                && !entry.operator_write_approval_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.receipt_store_write_grant_required
                && !entry.receipt_store_write_grant_present
                && entry.write_attempt_recording_precondition_missing
                && !entry.write_attempt_recording_allowed
                && !entry.write_attempt_recorded
                && !entry.write_attempt_persisted
                && entry.write_attempt_idempotency_key_projected
                && entry.post_record_readback_route_projected
                && entry.rollback_anchor_projected
                && entry.denial_receipt_projected
                && !entry.denial_receipt_persistence_allowed
                && !entry.denial_receipt_persisted
                && entry.denial_receipt_digest_projected
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
        });

    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording",
        status: if write_attempt_recording_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_write_positive_preconditions_ready: source.write_positive_preconditions_readback_ready,
        source_precondition_entry_count: source.precondition_entry_count,
        source_write_preconditions_missing_count: source.write_preconditions_missing_count,
        source_receipt_store_write_allowed_count: source.receipt_store_write_allowed_count,
        source_write_attempt_recorded_count: source.write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        recording_boundary_id: WRITE_ATTEMPT_RECORDING_BOUNDARY_ID,
        recording_boundary_route: WRITE_ATTEMPT_RECORDING_BOUNDARY_ROUTE,
        write_attempt_record_schema_version: WRITE_ATTEMPT_RECORD_SCHEMA_VERSION,
        boundary_entry_count: entries.len(),
        boundary_projected_count,
        boundary_ready_count,
        write_attempt_record_schema_projected_count,
        source_write_preconditions_attached_count,
        acceptance_authority_required_count,
        acceptance_authority_present_count,
        operator_write_approval_required_count,
        operator_write_approval_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        receipt_store_write_grant_required_count,
        receipt_store_write_grant_present_count,
        write_attempt_recording_precondition_missing_count,
        write_attempt_recording_allowed_count,
        write_attempt_recorded_count,
        write_attempt_persisted_count,
        write_attempt_idempotency_key_projected_count,
        write_attempt_idempotency_key_unique_count,
        post_record_readback_route_projected_count,
        rollback_anchor_projected_count,
        denial_receipt_projected_count,
        denial_receipt_persisted_count,
        denial_receipt_digest_projected_count,
        receipt_store_write_allowed_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        write_attempt_recording_boundary_readback_ready,
        write_attempt_recording_allowed: false,
        write_attempt_recorded: false,
        write_attempt_persisted: false,
        denial_receipt_persistence_allowed: false,
        denial_receipt_persisted: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        receipt_persistence_allowed: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "acceptance_authority_missing",
            "operator_write_approval_missing",
            "evidence_acceptance_missing",
            "receipt_store_write_grant_missing",
            "write_attempt_recording_disabled",
            "write_attempt_record_persistence_disabled",
            "denial_receipt_persistence_disabled",
            "receipt_store_write_disabled",
            "receipt_persistence_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_without_recording_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_precondition_entry_id: entry.id,
                source_write_precondition_set_id: entry.write_precondition_set_id,
                source_write_precondition_route: entry.write_precondition_route,
                source_write_attempt_recording_precondition_id: entry.write_attempt_recording_precondition_id,
                source_receipt_store_write_denial_id: entry.source_receipt_store_write_denial_id,
                source_replay_idempotency_key: entry.source_replay_idempotency_key,
                source_zero_effect_digest: entry.source_zero_effect_digest,
                recording_boundary_id: WRITE_ATTEMPT_RECORDING_BOUNDARY_ID,
                recording_boundary_route: format!("{WRITE_ATTEMPT_RECORDING_BOUNDARY_ROUTE}/{hyphenated}"),
                write_attempt_record_id: format!(
                    "write-attempt-record:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                write_attempt_record_schema_version: WRITE_ATTEMPT_RECORD_SCHEMA_VERSION,
                write_attempt_idempotency_key: format!(
                    "write-attempt-recording-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_record_readback_route: format!(
                    "{WRITE_ATTEMPT_RECORDING_BOUNDARY_ROUTE}/post-record/{hyphenated}"
                ),
                rollback_anchor: format!(
                    "rollback-anchor:controlled-live-evidence-receipt-store-write-attempt:{}",
                    entry.source_blocker_id
                ),
                denial_receipt_id: format!(
                    "write-attempt-recording-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                denial_receipt_route: format!(
                    "{WRITE_ATTEMPT_RECORDING_BOUNDARY_ROUTE}/denial-receipts/{hyphenated}"
                ),
                denial_receipt_digest: format!(
                    "sha256:receipt-store-write-attempt-recording-denial:{}",
                    entry.source_blocker_id
                ),
                denial_reason: WRITE_ATTEMPT_RECORDING_DENIAL_REASON,
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "receipt_store_write_attempt_recording_boundary_projected_without_recording",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_packet_unsent: entry.source_packet_unsent,
                source_write_denial_attached: entry.source_write_denial_attached,
                source_write_preconditions_attached: entry.write_precondition_set_projected,
                boundary_projected: true,
                boundary_ready: true,
                write_attempt_record_schema_projected: true,
                acceptance_authority_required: entry.acceptance_authority_required,
                acceptance_authority_present: entry.acceptance_authority_present,
                operator_write_approval_required: entry.operator_write_approval_required,
                operator_write_approval_present: entry.operator_write_approval_present,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: entry.evidence_acceptance_present,
                receipt_store_write_grant_required: entry.receipt_store_write_grant_required,
                receipt_store_write_grant_present: entry.receipt_store_write_grant_present,
                write_attempt_recording_precondition_missing: entry.write_attempt_recording_required
                    && !entry.write_attempt_recording_enabled,
                write_attempt_recording_allowed: false,
                write_attempt_recorded: false,
                write_attempt_persisted: false,
                write_attempt_idempotency_key_projected: true,
                post_record_readback_route_projected: true,
                rollback_anchor_projected: true,
                denial_receipt_projected: true,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
                denial_receipt_digest_projected: true,
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

impl
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingSideEffects
{
    pub const fn none() -> Self {
        Self {
            write_attempt_recorded: false,
            write_attempt_persisted: false,
            denial_receipt_persisted: false,
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
    fn write_attempt_recording_boundary_projects_all_entries_without_recording() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_write_positive_preconditions_ready);
        assert_eq!(report.source_precondition_entry_count, 7);
        assert_eq!(report.source_write_preconditions_missing_count, 7);
        assert_eq!(report.boundary_entry_count, 7);
        assert_eq!(report.boundary_projected_count, 7);
        assert_eq!(report.boundary_ready_count, 7);
        assert_eq!(report.write_attempt_record_schema_projected_count, 7);
        assert_eq!(report.source_write_preconditions_attached_count, 7);
        assert_eq!(report.acceptance_authority_required_count, 7);
        assert_eq!(report.acceptance_authority_present_count, 0);
        assert_eq!(report.operator_write_approval_required_count, 7);
        assert_eq!(report.operator_write_approval_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.receipt_store_write_grant_required_count, 7);
        assert_eq!(report.receipt_store_write_grant_present_count, 0);
        assert!(report.write_attempt_recording_boundary_readback_ready);
    }

    #[test]
    fn write_attempt_recording_boundary_keeps_recording_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_report();

        assert_eq!(report.write_attempt_recording_precondition_missing_count, 7);
        assert_eq!(report.write_attempt_recording_allowed_count, 0);
        assert_eq!(report.write_attempt_recorded_count, 0);
        assert_eq!(report.write_attempt_persisted_count, 0);
        assert_eq!(report.write_attempt_idempotency_key_projected_count, 7);
        assert_eq!(report.write_attempt_idempotency_key_unique_count, 7);
        assert_eq!(report.post_record_readback_route_projected_count, 7);
        assert_eq!(report.rollback_anchor_projected_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.denial_receipt_digest_projected_count, 7);
        assert_eq!(report.receipt_store_write_allowed_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.write_attempt_recording_allowed);
        assert!(!report.denial_receipt_persisted);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReceiptStoreWriteAttemptRecordingBoundaryReadbackWithoutRecordingSideEffects::none()
        );
    }

    #[test]
    fn write_attempt_recording_boundary_entries_are_stable_and_unrecorded() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.recording_boundary_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary/dirty-worktree-boundary"
            && entry.write_attempt_record_id
                == "write-attempt-record:controlled-live-evidence-receipt-store:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry
                .source_precondition_entry_id
                .starts_with("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_without_write_")
                && entry
                    .write_attempt_idempotency_key
                    .starts_with("write-attempt-recording-idempotency:")
                && entry
                    .denial_receipt_id
                    .starts_with("write-attempt-recording-denial-receipt:")
                && entry.denial_reason == WRITE_ATTEMPT_RECORDING_DENIAL_REASON
                && entry.source_packet_unsent
                && entry.source_write_denial_attached
                && entry.source_write_preconditions_attached
                && entry.boundary_projected
                && entry.boundary_ready
                && entry.write_attempt_record_schema_projected
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && entry.operator_write_approval_required
                && !entry.operator_write_approval_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.receipt_store_write_grant_required
                && !entry.receipt_store_write_grant_present
                && entry.write_attempt_recording_precondition_missing
                && !entry.write_attempt_recorded
                && !entry.write_attempt_persisted
                && entry.write_attempt_idempotency_key_projected
                && entry.post_record_readback_route_projected
                && entry.rollback_anchor_projected
                && entry.denial_receipt_projected
                && !entry.denial_receipt_persisted
                && entry.denial_receipt_digest_projected
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
