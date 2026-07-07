use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_retention_replay_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_v1";
const DENIAL_RECEIPT_COLLECTION_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-denial-receipts";
const DENIAL_RECEIPT_COLLECTION_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts";
const DENIAL_RECEIPT_SCHEMA_VERSION: &str =
    "controlled_live_local_evidence_acceptance_authority_decision_recording_denial_receipt_v1";
const RECORDING_DENIAL_REASON: &str = "local_evidence_acceptance_authority_decision_recording_disabled_authority_missing_no_local_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_recording_boundary_ready: bool,
    pub source_boundary_entry_count: usize,
    pub source_boundary_projected_count: usize,
    pub source_boundary_ready_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_packet_persistence_denial_receipt_attached_count: usize,
    pub source_non_send_readback_attached_count: usize,
    pub source_authority_packet_attached_count: usize,
    pub source_authority_decision_request_attached_count: usize,
    pub source_non_authority_receipt_attached_count: usize,
    pub source_decision_record_schema_projected_count: usize,
    pub source_local_evidence_acceptance_authority_required_count: usize,
    pub source_local_evidence_acceptance_authority_present_count: usize,
    pub source_recording_precondition_missing_count: usize,
    pub source_decision_recording_allowed_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_authority_decision_persisted_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_operator_packet_sent_count: usize,
    pub source_operator_packet_persisted_count: usize,
    pub source_non_authority_receipt_persisted_count: usize,
    pub source_local_evidence_acceptance_allowed_count: usize,
    pub source_local_evidence_acceptance_recorded_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
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
    pub source_recording_boundary_attached_count: usize,
    pub source_terminal_closeout_attached_entry_count: usize,
    pub source_persistence_denial_attached_entry_count: usize,
    pub source_packet_persistence_denial_receipt_attached_entry_count: usize,
    pub source_non_send_readback_attached_entry_count: usize,
    pub source_authority_packet_attached_entry_count: usize,
    pub source_authority_decision_request_attached_entry_count: usize,
    pub source_non_authority_receipt_attached_entry_count: usize,
    pub source_authority_decision_record_id_attached_count: usize,
    pub recording_denial_reason_projected_count: usize,
    pub recording_precondition_missing_count: usize,
    pub authority_decision_recording_disabled_count: usize,
    pub authority_decision_recorded_count: usize,
    pub authority_decision_persisted_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub non_authority_receipt_persisted_count: usize,
    pub local_evidence_acceptance_authority_present_count: usize,
    pub local_evidence_acceptance_allowed_count: usize,
    pub local_evidence_acceptance_recorded_count: usize,
    pub evidence_acceptance_recorded_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready:
        bool,
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub non_authority_receipt_persistence_allowed: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
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
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_boundary_entry_id: String,
    pub source_recording_boundary_id: &'static str,
    pub source_recording_boundary_route: String,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_key: String,
    pub source_terminal_closeout_route: String,
    pub source_terminal_reason: &'static str,
    pub source_terminal_state: &'static str,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_persistence_denial_reason: &'static str,
    pub source_packet_persistence_denial_receipt_id: String,
    pub source_non_send_readback_id: String,
    pub source_non_send_readback_route: String,
    pub source_authority_packet_id: &'static str,
    pub source_authority_packet_route: &'static str,
    pub source_authority_packet_key: String,
    pub source_authority_decision_request_id: String,
    pub source_authority_decision_request_route: String,
    pub source_non_authority_receipt_id: String,
    pub source_non_authority_receipt_route: String,
    pub source_authority_decision_record_id: String,
    pub source_authority_decision_record_schema_version: &'static str,
    pub source_authority_decision_idempotency_key: String,
    pub source_post_record_readback_route: String,
    pub source_rollback_anchor: String,
    pub source_projected_denial_receipt_id: String,
    pub denial_receipt_id: String,
    pub denial_receipt_route: String,
    pub denial_receipt_digest: String,
    pub denial_receipt_schema_version: &'static str,
    pub denial_receipt_idempotency_key: String,
    pub recording_denial_reason: &'static str,
    pub recording_denial_state: &'static str,
    pub observed_state: &'static str,
    pub source_recording_boundary_attached: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_packet_persistence_denial_receipt_attached: bool,
    pub source_non_send_readback_attached: bool,
    pub source_authority_packet_attached: bool,
    pub source_authority_decision_request_attached: bool,
    pub source_non_authority_receipt_attached: bool,
    pub source_authority_decision_record_id_attached: bool,
    pub denial_receipt_projected: bool,
    pub denial_receipt_digest_projected: bool,
    pub denial_receipt_readback_route_projected: bool,
    pub denial_receipt_idempotency_key_projected: bool,
    pub recording_denial_reason_projected: bool,
    pub recording_precondition_missing: bool,
    pub authority_decision_recording_disabled: bool,
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub denial_receipt_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub non_authority_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_required: bool,
    pub local_evidence_acceptance_authority_present: bool,
    pub local_evidence_acceptance_allowed: bool,
    pub local_evidence_acceptance_recording_allowed: bool,
    pub local_evidence_acceptance_recorded: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptSideEffects
{
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub non_authority_receipt_persisted: bool,
    pub local_evidence_acceptance_authority_accepted: bool,
    pub local_evidence_acceptance_recorded: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptReport
{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_entries();

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
    let source_recording_boundary_attached_count =
        count(&entries, |entry| entry.source_recording_boundary_attached);
    let source_terminal_closeout_attached_entry_count =
        count(&entries, |entry| entry.source_terminal_closeout_attached);
    let source_persistence_denial_attached_entry_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_packet_persistence_denial_receipt_attached_entry_count = count(&entries, |entry| {
        entry.source_packet_persistence_denial_receipt_attached
    });
    let source_non_send_readback_attached_entry_count =
        count(&entries, |entry| entry.source_non_send_readback_attached);
    let source_authority_packet_attached_entry_count =
        count(&entries, |entry| entry.source_authority_packet_attached);
    let source_authority_decision_request_attached_entry_count = count(&entries, |entry| {
        entry.source_authority_decision_request_attached
    });
    let source_non_authority_receipt_attached_entry_count = count(&entries, |entry| {
        entry.source_non_authority_receipt_attached
    });
    let source_authority_decision_record_id_attached_count = count(&entries, |entry| {
        entry.source_authority_decision_record_id_attached
    });
    let recording_denial_reason_projected_count =
        count(&entries, |entry| entry.recording_denial_reason_projected);
    let recording_precondition_missing_count =
        count(&entries, |entry| entry.recording_precondition_missing);
    let authority_decision_recording_disabled_count = count(&entries, |entry| {
        entry.authority_decision_recording_disabled
    });
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let authority_decision_persisted_count =
        count(&entries, |entry| entry.authority_decision_persisted);
    let denial_receipt_persisted_count = count(&entries, |entry| entry.denial_receipt_persisted);
    let operator_packet_sent_count = count(&entries, |entry| entry.operator_packet_sent);
    let operator_packet_persisted_count = count(&entries, |entry| entry.operator_packet_persisted);
    let non_authority_receipt_persisted_count =
        count(&entries, |entry| entry.non_authority_receipt_persisted);
    let local_evidence_acceptance_authority_present_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_present
    });
    let local_evidence_acceptance_allowed_count =
        count(&entries, |entry| entry.local_evidence_acceptance_allowed);
    let local_evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.local_evidence_acceptance_recorded);
    let evidence_acceptance_recorded_count =
        count(&entries, |entry| entry.evidence_acceptance_recorded);
    let evidence_recorded_count = count(&entries, |entry| entry.evidence_recorded);
    let receipt_store_write_attempt_recorded_count =
        count(&entries, |entry| entry.receipt_store_write_attempt_recorded);
    let receipt_store_written_count = count(&entries, |entry| entry.receipt_store_written);
    let receipt_persisted_count = count(&entries, |entry| {
        entry.receipt_persisted || entry.denial_receipt_persisted
    });
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready =
        source.local_evidence_acceptance_authority_decision_recording_boundary_readback_ready
            && source.boundary_entry_count == 7
            && source.boundary_projected_count == 7
            && source.boundary_ready_count == 7
            && source.source_terminal_closeout_attached_count == 7
            && source.source_persistence_denial_attached_count == 7
            && source.source_packet_persistence_denial_receipt_attached_count == 7
            && source.source_non_send_readback_attached_count == 7
            && source.source_authority_packet_attached_count == 7
            && source.source_authority_decision_request_attached_count == 7
            && source.source_non_authority_receipt_attached_count == 7
            && source.decision_record_schema_projected_count == 7
            && source.local_evidence_acceptance_authority_required_count == 7
            && source.local_evidence_acceptance_authority_present_count == 0
            && source.recording_precondition_missing_count == 7
            && source.decision_recording_allowed_count == 0
            && source.authority_decision_recorded_count == 0
            && source.authority_decision_persisted_count == 0
            && source.denial_receipt_persisted_count == 0
            && source.operator_packet_sent_count == 0
            && source.operator_packet_persisted_count == 0
            && source.non_authority_receipt_persisted_count == 0
            && source.local_evidence_acceptance_allowed_count == 0
            && source.local_evidence_acceptance_recorded_count == 0
            && source.evidence_acceptance_recorded_count == 0
            && source.evidence_recorded_count == 0
            && source.receipt_store_write_attempt_recorded_count == 0
            && source.receipt_store_written_count == 0
            && source.receipt_persisted_count == 0
            && !source.live_execution_allowed
            && entries.len() == 7
            && denial_receipt_projected_count == 7
            && denial_receipt_digest_projected_count == 7
            && denial_receipt_readback_route_projected_count == 7
            && denial_receipt_idempotency_key_projected_count == 7
            && denial_receipt_idempotency_key_unique_count == 7
            && source_recording_boundary_attached_count == 7
            && source_terminal_closeout_attached_entry_count == 7
            && source_persistence_denial_attached_entry_count == 7
            && source_packet_persistence_denial_receipt_attached_entry_count == 7
            && source_non_send_readback_attached_entry_count == 7
            && source_authority_packet_attached_entry_count == 7
            && source_authority_decision_request_attached_entry_count == 7
            && source_non_authority_receipt_attached_entry_count == 7
            && source_authority_decision_record_id_attached_count == 7
            && recording_denial_reason_projected_count == 7
            && recording_precondition_missing_count == 7
            && authority_decision_recording_disabled_count == 7
            && authority_decision_recorded_count == 0
            && authority_decision_persisted_count == 0
            && denial_receipt_persisted_count == 0
            && operator_packet_sent_count == 0
            && operator_packet_persisted_count == 0
            && non_authority_receipt_persisted_count == 0
            && local_evidence_acceptance_authority_present_count == 0
            && local_evidence_acceptance_allowed_count == 0
            && local_evidence_acceptance_recorded_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_recording_boundary_ready: source
            .local_evidence_acceptance_authority_decision_recording_boundary_readback_ready,
        source_boundary_entry_count: source.boundary_entry_count,
        source_boundary_projected_count: source.boundary_projected_count,
        source_boundary_ready_count: source.boundary_ready_count,
        source_terminal_closeout_attached_count: source.source_terminal_closeout_attached_count,
        source_persistence_denial_attached_count: source.source_persistence_denial_attached_count,
        source_packet_persistence_denial_receipt_attached_count: source.source_packet_persistence_denial_receipt_attached_count,
        source_non_send_readback_attached_count: source.source_non_send_readback_attached_count,
        source_authority_packet_attached_count: source.source_authority_packet_attached_count,
        source_authority_decision_request_attached_count: source.source_authority_decision_request_attached_count,
        source_non_authority_receipt_attached_count: source.source_non_authority_receipt_attached_count,
        source_decision_record_schema_projected_count: source.decision_record_schema_projected_count,
        source_local_evidence_acceptance_authority_required_count: source.local_evidence_acceptance_authority_required_count,
        source_local_evidence_acceptance_authority_present_count: source.local_evidence_acceptance_authority_present_count,
        source_recording_precondition_missing_count: source.recording_precondition_missing_count,
        source_decision_recording_allowed_count: source.decision_recording_allowed_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_authority_decision_persisted_count: source.authority_decision_persisted_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_non_authority_receipt_persisted_count: source.non_authority_receipt_persisted_count,
        source_local_evidence_acceptance_allowed_count: source.local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count: source.local_evidence_acceptance_recorded_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source.receipt_store_write_attempt_recorded_count,
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
        source_recording_boundary_attached_count,
        source_terminal_closeout_attached_entry_count,
        source_persistence_denial_attached_entry_count,
        source_packet_persistence_denial_receipt_attached_entry_count,
        source_non_send_readback_attached_entry_count,
        source_authority_packet_attached_entry_count,
        source_authority_decision_request_attached_entry_count,
        source_non_authority_receipt_attached_entry_count,
        source_authority_decision_record_id_attached_count,
        recording_denial_reason_projected_count,
        recording_precondition_missing_count,
        authority_decision_recording_disabled_count,
        authority_decision_recorded_count,
        authority_decision_persisted_count,
        denial_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        non_authority_receipt_persisted_count,
        local_evidence_acceptance_authority_present_count,
        local_evidence_acceptance_allowed_count,
        local_evidence_acceptance_recorded_count,
        evidence_acceptance_recorded_count,
        evidence_recorded_count,
        receipt_store_write_attempt_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready,
        authority_decision_recording_allowed: false,
        authority_decision_recorded: false,
        authority_decision_persisted: false,
        denial_receipt_persistence_allowed: false,
        denial_receipt_persisted: false,
        local_evidence_acceptance_authority_allowed: false,
        non_authority_receipt_persistence_allowed: false,
        local_evidence_acceptance_allowed: false,
        local_evidence_acceptance_recording_allowed: false,
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
            "authority_decision_recording_disabled",
            "authority_decision_persistence_disabled",
            "authority_decision_denial_receipt_persistence_disabled",
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "non_authority_receipt_persistence_disabled",
            "local_evidence_acceptance_disabled",
            "local_evidence_acceptance_recording_disabled",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_entries(
) -> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_without_persistence_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_boundary_entry_id: entry.id,
                source_recording_boundary_id: entry.recording_boundary_id,
                source_recording_boundary_route: entry.recording_boundary_route,
                source_terminal_closeout_id: entry.source_terminal_closeout_id,
                source_terminal_closeout_key: entry.source_terminal_closeout_key,
                source_terminal_closeout_route: entry.source_terminal_closeout_route,
                source_terminal_reason: entry.source_terminal_reason,
                source_terminal_state: entry.source_terminal_state,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_persistence_denial_reason: entry.source_persistence_denial_reason,
                source_packet_persistence_denial_receipt_id: entry
                    .source_packet_persistence_denial_receipt_id,
                source_non_send_readback_id: entry.source_packet_non_send_readback_id,
                source_non_send_readback_route: entry.source_packet_non_send_readback_route,
                source_authority_packet_id: entry.source_authority_packet_id,
                source_authority_packet_route: entry.source_authority_packet_route,
                source_authority_packet_key: entry.source_authority_packet_key,
                source_authority_decision_request_id: entry.source_authority_decision_request_id,
                source_authority_decision_request_route: entry
                    .source_authority_decision_request_route,
                source_non_authority_receipt_id: entry.source_non_authority_receipt_id,
                source_non_authority_receipt_route: entry.source_non_authority_receipt_route,
                source_authority_decision_record_id: entry.authority_decision_record_id,
                source_authority_decision_record_schema_version: entry
                    .authority_decision_record_schema_version,
                source_authority_decision_idempotency_key: entry.authority_decision_idempotency_key,
                source_post_record_readback_route: entry.post_record_readback_route,
                source_rollback_anchor: entry.rollback_anchor,
                source_projected_denial_receipt_id: entry.denial_receipt_id,
                denial_receipt_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_route: format!("{DENIAL_RECEIPT_COLLECTION_ROUTE}/{hyphenated}"),
                denial_receipt_digest: format!(
                    "sha256:local-evidence-acceptance-authority-decision-recording-denial-receipt:{}:not-persisted",
                    entry.source_blocker_id
                ),
                denial_receipt_schema_version: DENIAL_RECEIPT_SCHEMA_VERSION,
                denial_receipt_idempotency_key: format!(
                    "local-evidence-acceptance-authority-decision-recording-denial-receipt-idempotency:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                recording_denial_reason: RECORDING_DENIAL_REASON,
                recording_denial_state: "authority_decision_recording_denied_without_persistence",
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_denial_receipt_projected_without_persistence",
                source_recording_boundary_attached: entry.boundary_projected,
                source_terminal_closeout_attached: entry.source_terminal_closeout_attached,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_packet_persistence_denial_receipt_attached: entry
                    .source_packet_persistence_denial_receipt_attached,
                source_non_send_readback_attached: entry.source_non_send_readback_attached,
                source_authority_packet_attached: entry.source_authority_packet_attached,
                source_authority_decision_request_attached: entry
                    .source_authority_decision_request_attached,
                source_non_authority_receipt_attached: entry
                    .source_non_authority_receipt_attached,
                source_authority_decision_record_id_attached: true,
                denial_receipt_projected: true,
                denial_receipt_digest_projected: true,
                denial_receipt_readback_route_projected: true,
                denial_receipt_idempotency_key_projected: true,
                recording_denial_reason_projected: true,
                recording_precondition_missing: entry.recording_precondition_missing,
                authority_decision_recording_disabled: !entry.authority_decision_recording_allowed,
                authority_decision_recording_allowed: false,
                authority_decision_recorded: false,
                authority_decision_persisted: false,
                denial_receipt_persistence_allowed: false,
                denial_receipt_persisted: false,
                operator_packet_sent: false,
                operator_packet_persisted: false,
                non_authority_receipt_persisted: false,
                local_evidence_acceptance_authority_required: entry
                    .local_evidence_acceptance_authority_required,
                local_evidence_acceptance_authority_present: false,
                local_evidence_acceptance_allowed: false,
                local_evidence_acceptance_recording_allowed: false,
                local_evidence_acceptance_recorded: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_denial_receipt_projected_without_persistence"
        && entry.source_recording_boundary_attached
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_packet_persistence_denial_receipt_attached
        && entry.source_non_send_readback_attached
        && entry.source_authority_packet_attached
        && entry.source_authority_decision_request_attached
        && entry.source_non_authority_receipt_attached
        && entry.source_authority_decision_record_id_attached
        && entry.denial_receipt_projected
        && entry.denial_receipt_digest_projected
        && entry.denial_receipt_readback_route_projected
        && entry.denial_receipt_idempotency_key_projected
        && entry.recording_denial_reason_projected
        && entry.recording_precondition_missing
        && entry.authority_decision_recording_disabled
        && !entry.authority_decision_recording_allowed
        && !entry.authority_decision_recorded
        && !entry.authority_decision_persisted
        && !entry.denial_receipt_persistence_allowed
        && !entry.denial_receipt_persisted
        && !entry.operator_packet_sent
        && !entry.operator_packet_persisted
        && !entry.non_authority_receipt_persisted
        && entry.local_evidence_acceptance_authority_required
        && !entry.local_evidence_acceptance_authority_present
        && !entry.local_evidence_acceptance_allowed
        && !entry.local_evidence_acceptance_recording_allowed
        && !entry.local_evidence_acceptance_recorded
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptSideEffects {
    pub const fn none() -> Self {
        Self {
            authority_decision_recorded: false,
            authority_decision_persisted: false,
            denial_receipt_persisted: false,
            operator_packet_sent: false,
            operator_packet_persisted: false,
            non_authority_receipt_persisted: false,
            local_evidence_acceptance_authority_accepted: false,
            local_evidence_acceptance_recorded: false,
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
    fn local_authority_decision_recording_denial_receipt_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_recording_boundary_ready);
        assert_eq!(report.source_boundary_entry_count, 7);
        assert_eq!(report.source_boundary_projected_count, 7);
        assert_eq!(report.source_boundary_ready_count, 7);
        assert_eq!(report.source_terminal_closeout_attached_count, 7);
        assert_eq!(report.source_persistence_denial_attached_count, 7);
        assert_eq!(
            report.source_packet_persistence_denial_receipt_attached_count,
            7
        );
        assert_eq!(report.source_non_send_readback_attached_count, 7);
        assert_eq!(report.source_authority_packet_attached_count, 7);
        assert_eq!(report.source_authority_decision_request_attached_count, 7);
        assert_eq!(report.source_non_authority_receipt_attached_count, 7);
        assert_eq!(report.denial_receipt_entry_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_digest_projected_count, 7);
        assert_eq!(report.denial_receipt_readback_route_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_projected_count, 7);
        assert_eq!(report.denial_receipt_idempotency_key_unique_count, 7);
        assert_eq!(report.recording_denial_reason_projected_count, 7);
        assert_eq!(report.recording_precondition_missing_count, 7);
        assert_eq!(report.authority_decision_recording_disabled_count, 7);
        assert!(
            report
                .local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready
        );
    }

    #[test]
    fn local_authority_decision_recording_denial_receipt_keeps_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_report();

        assert_eq!(report.source_authority_decision_recorded_count, 0);
        assert_eq!(report.source_authority_decision_persisted_count, 0);
        assert_eq!(report.source_denial_receipt_persisted_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.non_authority_receipt_persisted_count, 0);
        assert_eq!(report.local_evidence_acceptance_authority_present_count, 0);
        assert_eq!(report.local_evidence_acceptance_allowed_count, 0);
        assert_eq!(report.local_evidence_acceptance_recorded_count, 0);
        assert_eq!(report.evidence_acceptance_recorded_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_write_attempt_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.authority_decision_recording_allowed);
        assert!(!report.authority_decision_recorded);
        assert!(!report.authority_decision_persisted);
        assert!(!report.denial_receipt_persisted);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingDenialReceiptSideEffects::none()
        );
    }

    #[test]
    fn local_authority_decision_recording_denial_receipt_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.denial_receipt_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/operator-live-approval-missing"
            && entry.denial_receipt_id
                == "local-evidence-acceptance-authority-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:operator_live_approval_missing:not-persisted"
            && entry.source_authority_decision_record_id
                == "local-evidence-acceptance-authority-decision-record:controlled-live-evidence-receipt-store:operator_live_approval_missing:not-recorded"));
        assert!(report.entries.iter().all(entry_is_ready_blocked));
        assert!(report.entries.iter().all(|entry| {
            entry.denial_receipt_schema_version == DENIAL_RECEIPT_SCHEMA_VERSION
                && entry.recording_denial_reason == RECORDING_DENIAL_REASON
                && entry.recording_denial_state
                    == "authority_decision_recording_denied_without_persistence"
                && entry.source_recording_boundary_id
                    == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-boundary"
                && entry.source_terminal_state == "terminal_no_persistence"
                && entry.source_authority_packet_id
                    == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"
        }));
    }
}
