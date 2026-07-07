use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence::controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_BOUNDARY_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_v1";
const RECORDING_BOUNDARY_ID: &str = "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-boundary";
const RECORDING_BOUNDARY_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-boundary";
const AUTHORITY_DECISION_RECORD_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_record_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryReport {
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
    pub source_retention_replay_attached_count: usize,
    pub source_terminal_source_persistence_denial_attached_count: usize,
    pub source_terminal_source_packet_persistence_denial_receipt_attached_count: usize,
    pub source_terminal_source_non_send_readback_attached_count: usize,
    pub source_terminal_source_authority_packet_attached_count: usize,
    pub source_terminal_closeout_recorded_count: usize,
    pub source_terminal_closeout_persisted_count: usize,
    pub source_terminal_closeout_accepted_count: usize,
    pub source_terminal_closeout_authoritative_count: usize,
    pub source_packet_persistence_attempt_recorded_count: usize,
    pub source_packet_persistence_denial_receipt_persisted_count: usize,
    pub source_operator_packet_sent_count: usize,
    pub source_operator_packet_persisted_count: usize,
    pub source_local_evidence_acceptance_authority_present_count: usize,
    pub source_local_evidence_acceptance_allowed_count: usize,
    pub source_local_evidence_acceptance_recorded_count: usize,
    pub source_authority_decision_recorded_count: usize,
    pub source_non_authority_receipt_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: &'static str,
    pub authority_decision_record_schema_version: &'static str,
    pub boundary_entry_count: usize,
    pub boundary_projected_count: usize,
    pub boundary_ready_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_packet_persistence_denial_receipt_attached_count: usize,
    pub source_non_send_readback_attached_count: usize,
    pub source_authority_packet_attached_count: usize,
    pub source_authority_decision_request_attached_count: usize,
    pub source_non_authority_receipt_attached_count: usize,
    pub decision_record_schema_projected_count: usize,
    pub local_evidence_acceptance_authority_required_count: usize,
    pub local_evidence_acceptance_authority_present_count: usize,
    pub recording_precondition_missing_count: usize,
    pub decision_recording_allowed_count: usize,
    pub authority_decision_recorded_count: usize,
    pub authority_decision_persisted_count: usize,
    pub decision_idempotency_key_projected_count: usize,
    pub decision_idempotency_key_unique_count: usize,
    pub post_record_readback_route_projected_count: usize,
    pub rollback_anchor_projected_count: usize,
    pub denial_receipt_projected_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub non_authority_receipt_persisted_count: usize,
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
    pub local_evidence_acceptance_authority_decision_recording_boundary_readback_ready: bool,
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub denial_receipt_persistence_allowed: bool,
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
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundarySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry
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
    pub source_packet_persistence_denial_receipt_id: String,
    pub source_authority_packet_id: &'static str,
    pub source_authority_packet_route: &'static str,
    pub source_authority_packet_key: String,
    pub source_packet_non_send_readback_id: String,
    pub source_packet_non_send_readback_route: String,
    pub source_authority_decision_request_id: String,
    pub source_authority_decision_request_route: String,
    pub source_non_authority_receipt_id: String,
    pub source_non_authority_receipt_route: String,
    pub recording_boundary_id: &'static str,
    pub recording_boundary_route: String,
    pub authority_decision_record_id: String,
    pub authority_decision_record_schema_version: &'static str,
    pub authority_decision_idempotency_key: String,
    pub post_record_readback_route: String,
    pub rollback_anchor: String,
    pub denial_receipt_id: String,
    pub observed_state: &'static str,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_packet_persistence_denial_receipt_attached: bool,
    pub source_non_send_readback_attached: bool,
    pub source_authority_packet_attached: bool,
    pub source_authority_decision_request_attached: bool,
    pub source_non_authority_receipt_attached: bool,
    pub boundary_projected: bool,
    pub boundary_ready: bool,
    pub decision_record_schema_projected: bool,
    pub local_evidence_acceptance_authority_required: bool,
    pub local_evidence_acceptance_authority_present: bool,
    pub recording_precondition_missing: bool,
    pub authority_decision_recording_allowed: bool,
    pub authority_decision_recorded: bool,
    pub authority_decision_persisted: bool,
    pub decision_idempotency_key_projected: bool,
    pub post_record_readback_route_projected: bool,
    pub rollback_anchor_projected: bool,
    pub denial_receipt_projected: bool,
    pub denial_receipt_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub non_authority_receipt_persisted: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundarySideEffects
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryReport
{
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryReport
{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_entries();

    let boundary_projected_count = count(&entries, |entry| entry.boundary_projected);
    let boundary_ready_count = count(&entries, |entry| entry.boundary_ready);
    let source_terminal_closeout_attached_count =
        count(&entries, |entry| entry.source_terminal_closeout_attached);
    let source_persistence_denial_attached_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_packet_persistence_denial_receipt_attached_count = count(&entries, |entry| {
        entry.source_packet_persistence_denial_receipt_attached
    });
    let source_non_send_readback_attached_count =
        count(&entries, |entry| entry.source_non_send_readback_attached);
    let source_authority_packet_attached_count =
        count(&entries, |entry| entry.source_authority_packet_attached);
    let source_authority_decision_request_attached_count = count(&entries, |entry| {
        entry.source_authority_decision_request_attached
    });
    let source_non_authority_receipt_attached_count = count(&entries, |entry| {
        entry.source_non_authority_receipt_attached
    });
    let decision_record_schema_projected_count =
        count(&entries, |entry| entry.decision_record_schema_projected);
    let local_evidence_acceptance_authority_required_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_required
    });
    let local_evidence_acceptance_authority_present_count = count(&entries, |entry| {
        entry.local_evidence_acceptance_authority_present
    });
    let recording_precondition_missing_count =
        count(&entries, |entry| entry.recording_precondition_missing);
    let decision_recording_allowed_count =
        count(&entries, |entry| entry.authority_decision_recording_allowed);
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let authority_decision_persisted_count =
        count(&entries, |entry| entry.authority_decision_persisted);
    let decision_idempotency_key_projected_count =
        count(&entries, |entry| entry.decision_idempotency_key_projected);
    let decision_idempotency_key_unique_count = entries
        .iter()
        .map(|entry| entry.authority_decision_idempotency_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let post_record_readback_route_projected_count =
        count(&entries, |entry| entry.post_record_readback_route_projected);
    let rollback_anchor_projected_count = count(&entries, |entry| entry.rollback_anchor_projected);
    let denial_receipt_projected_count = count(&entries, |entry| entry.denial_receipt_projected);
    let denial_receipt_persisted_count = count(&entries, |entry| entry.denial_receipt_persisted);
    let operator_packet_sent_count = count(&entries, |entry| entry.operator_packet_sent);
    let operator_packet_persisted_count = count(&entries, |entry| entry.operator_packet_persisted);
    let non_authority_receipt_persisted_count =
        count(&entries, |entry| entry.non_authority_receipt_persisted);
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

    let local_evidence_acceptance_authority_decision_recording_boundary_readback_ready = source
        .terminal_no_persistence_readback_ready
        && source.terminal_entry_count == 7
        && source.terminal_closeout_projected_count == 7
        && source.terminal_no_persistence_confirmed_count == 7
        && source.source_retention_replay_attached_count == 7
        && source.source_persistence_denial_attached_count == 7
        && source.source_packet_persistence_denial_receipt_attached_count == 7
        && source.source_non_send_readback_attached_count == 7
        && source.source_authority_packet_attached_count == 7
        && source.terminal_closeout_recorded_count == 0
        && source.terminal_closeout_persisted_count == 0
        && source.terminal_closeout_accepted_count == 0
        && source.terminal_closeout_authoritative_count == 0
        && source.packet_persistence_attempt_recorded_count == 0
        && source.packet_persistence_denial_receipt_persisted_count == 0
        && source.operator_packet_sent_count == 0
        && source.operator_packet_persisted_count == 0
        && source.local_evidence_acceptance_authority_present_count == 0
        && source.local_evidence_acceptance_allowed_count == 0
        && source.local_evidence_acceptance_recorded_count == 0
        && source.authority_decision_recorded_count == 0
        && source.non_authority_receipt_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && boundary_projected_count == 7
        && boundary_ready_count == 7
        && source_terminal_closeout_attached_count == 7
        && source_persistence_denial_attached_count == 7
        && source_packet_persistence_denial_receipt_attached_count == 7
        && source_non_send_readback_attached_count == 7
        && source_authority_packet_attached_count == 7
        && source_authority_decision_request_attached_count == 7
        && source_non_authority_receipt_attached_count == 7
        && decision_record_schema_projected_count == 7
        && local_evidence_acceptance_authority_required_count == 7
        && local_evidence_acceptance_authority_present_count == 0
        && recording_precondition_missing_count == 7
        && decision_recording_allowed_count == 0
        && authority_decision_recorded_count == 0
        && authority_decision_persisted_count == 0
        && decision_idempotency_key_projected_count == 7
        && decision_idempotency_key_unique_count == 7
        && post_record_readback_route_projected_count == 7
        && rollback_anchor_projected_count == 7
        && denial_receipt_projected_count == 7
        && denial_receipt_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && non_authority_receipt_persisted_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if local_evidence_acceptance_authority_decision_recording_boundary_readback_ready {
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
        source_terminal_no_persistence_confirmed_count: source.terminal_no_persistence_confirmed_count,
        source_retention_replay_attached_count: source.source_retention_replay_attached_count,
        source_terminal_source_persistence_denial_attached_count: source.source_persistence_denial_attached_count,
        source_terminal_source_packet_persistence_denial_receipt_attached_count: source.source_packet_persistence_denial_receipt_attached_count,
        source_terminal_source_non_send_readback_attached_count: source.source_non_send_readback_attached_count,
        source_terminal_source_authority_packet_attached_count: source.source_authority_packet_attached_count,
        source_terminal_closeout_recorded_count: source.terminal_closeout_recorded_count,
        source_terminal_closeout_persisted_count: source.terminal_closeout_persisted_count,
        source_terminal_closeout_accepted_count: source.terminal_closeout_accepted_count,
        source_terminal_closeout_authoritative_count: source.terminal_closeout_authoritative_count,
        source_packet_persistence_attempt_recorded_count: source.packet_persistence_attempt_recorded_count,
        source_packet_persistence_denial_receipt_persisted_count: source.packet_persistence_denial_receipt_persisted_count,
        source_operator_packet_sent_count: source.operator_packet_sent_count,
        source_operator_packet_persisted_count: source.operator_packet_persisted_count,
        source_local_evidence_acceptance_authority_present_count: source.local_evidence_acceptance_authority_present_count,
        source_local_evidence_acceptance_allowed_count: source.local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count: source.local_evidence_acceptance_recorded_count,
        source_authority_decision_recorded_count: source.authority_decision_recorded_count,
        source_non_authority_receipt_persisted_count: source.non_authority_receipt_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source.receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        recording_boundary_id: RECORDING_BOUNDARY_ID,
        recording_boundary_route: RECORDING_BOUNDARY_ROUTE,
        authority_decision_record_schema_version: AUTHORITY_DECISION_RECORD_SCHEMA_VERSION,
        boundary_entry_count: entries.len(),
        boundary_projected_count,
        boundary_ready_count,
        source_terminal_closeout_attached_count,
        source_persistence_denial_attached_count,
        source_packet_persistence_denial_receipt_attached_count,
        source_non_send_readback_attached_count,
        source_authority_packet_attached_count,
        source_authority_decision_request_attached_count,
        source_non_authority_receipt_attached_count,
        decision_record_schema_projected_count,
        local_evidence_acceptance_authority_required_count,
        local_evidence_acceptance_authority_present_count,
        recording_precondition_missing_count,
        decision_recording_allowed_count,
        authority_decision_recorded_count,
        authority_decision_persisted_count,
        decision_idempotency_key_projected_count,
        decision_idempotency_key_unique_count,
        post_record_readback_route_projected_count,
        rollback_anchor_projected_count,
        denial_receipt_projected_count,
        denial_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        non_authority_receipt_persisted_count,
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
        local_evidence_acceptance_authority_decision_recording_boundary_readback_ready,
        authority_decision_recording_allowed: false,
        authority_decision_recorded: false,
        authority_decision_persisted: false,
        denial_receipt_persistence_allowed: false,
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_BOUNDARY_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundarySideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_entries(
) -> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry,
>{
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_without_recording_{}",
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
                source_packet_persistence_denial_receipt_id: entry
                    .source_packet_persistence_denial_receipt_id,
                source_authority_packet_id: entry.source_authority_packet_id,
                source_authority_packet_route: entry.source_authority_packet_route,
                source_authority_packet_key: entry.source_authority_packet_key,
                source_packet_non_send_readback_id: entry.source_packet_non_send_readback_id,
                source_packet_non_send_readback_route: entry
                    .source_packet_non_send_readback_route,
                source_authority_decision_request_id: entry.source_authority_decision_request_id,
                source_authority_decision_request_route: entry
                    .source_authority_decision_request_route,
                source_non_authority_receipt_id: entry.source_non_authority_receipt_id,
                source_non_authority_receipt_route: entry.source_non_authority_receipt_route,
                recording_boundary_id: RECORDING_BOUNDARY_ID,
                recording_boundary_route: format!("{RECORDING_BOUNDARY_ROUTE}/{hyphenated}"),
                authority_decision_record_id: format!(
                    "local-evidence-acceptance-authority-decision-record:controlled-live-evidence-receipt-store:{}:not-recorded",
                    entry.source_blocker_id
                ),
                authority_decision_record_schema_version: AUTHORITY_DECISION_RECORD_SCHEMA_VERSION,
                authority_decision_idempotency_key: format!(
                    "controlled-live-evidence-receipt-store.local-evidence-acceptance-authority-decision-recording.idempotency.{}",
                    entry.source_blocker_id
                ),
                post_record_readback_route: format!(
                    "{RECORDING_BOUNDARY_ROUTE}/post-record/{hyphenated}"
                ),
                rollback_anchor: format!(
                    "rollback-anchor://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-boundary/{hyphenated}"
                ),
                denial_receipt_id: format!(
                    "local-evidence-acceptance-authority-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                observed_state:
                    "local_evidence_acceptance_authority_decision_recording_boundary_projected_without_recording",
                source_terminal_closeout_attached: entry.terminal_no_persistence_confirmed,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_packet_persistence_denial_receipt_attached: entry
                    .source_packet_persistence_denial_receipt_attached,
                source_non_send_readback_attached: entry.source_non_send_readback_attached,
                source_authority_packet_attached: entry.source_authority_packet_attached,
                source_authority_decision_request_attached: true,
                source_non_authority_receipt_attached: true,
                boundary_projected: true,
                boundary_ready: true,
                decision_record_schema_projected: true,
                local_evidence_acceptance_authority_required: true,
                local_evidence_acceptance_authority_present: false,
                recording_precondition_missing: true,
                authority_decision_recording_allowed: false,
                authority_decision_recorded: false,
                authority_decision_persisted: false,
                decision_idempotency_key_projected: true,
                post_record_readback_route_projected: true,
                rollback_anchor_projected: true,
                denial_receipt_projected: true,
                denial_receipt_persisted: false,
                operator_packet_sent: false,
                operator_packet_persisted: false,
                non_authority_receipt_persisted: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry],
    predicate: impl Fn(&ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundaryEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_decision_recording_boundary_projected_without_recording"
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_packet_persistence_denial_receipt_attached
        && entry.source_non_send_readback_attached
        && entry.source_authority_packet_attached
        && entry.source_authority_decision_request_attached
        && entry.source_non_authority_receipt_attached
        && entry.boundary_projected
        && entry.boundary_ready
        && entry.decision_record_schema_projected
        && entry.local_evidence_acceptance_authority_required
        && !entry.local_evidence_acceptance_authority_present
        && entry.recording_precondition_missing
        && !entry.authority_decision_recording_allowed
        && !entry.authority_decision_recorded
        && !entry.authority_decision_persisted
        && entry.decision_idempotency_key_projected
        && entry.post_record_readback_route_projected
        && entry.rollback_anchor_projected
        && entry.denial_receipt_projected
        && !entry.denial_receipt_persisted
        && !entry.operator_packet_sent
        && !entry.operator_packet_persisted
        && !entry.non_authority_receipt_persisted
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundarySideEffects {
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
    fn local_authority_decision_recording_boundary_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_terminal_no_persistence_readback_ready);
        assert_eq!(report.source_terminal_entry_count, 7);
        assert_eq!(report.source_terminal_closeout_projected_count, 7);
        assert_eq!(report.source_terminal_no_persistence_confirmed_count, 7);
        assert_eq!(report.boundary_entry_count, 7);
        assert_eq!(report.boundary_projected_count, 7);
        assert_eq!(report.boundary_ready_count, 7);
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
        assert_eq!(report.decision_record_schema_projected_count, 7);
        assert_eq!(report.local_evidence_acceptance_authority_required_count, 7);
        assert_eq!(report.recording_precondition_missing_count, 7);
        assert!(
            report.local_evidence_acceptance_authority_decision_recording_boundary_readback_ready
        );
    }

    #[test]
    fn local_authority_decision_recording_boundary_keeps_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report();

        assert_eq!(report.local_evidence_acceptance_authority_present_count, 0);
        assert_eq!(report.decision_recording_allowed_count, 0);
        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.authority_decision_persisted_count, 0);
        assert_eq!(report.decision_idempotency_key_projected_count, 7);
        assert_eq!(report.decision_idempotency_key_unique_count, 7);
        assert_eq!(report.post_record_readback_route_projected_count, 7);
        assert_eq!(report.rollback_anchor_projected_count, 7);
        assert_eq!(report.denial_receipt_projected_count, 7);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.non_authority_receipt_persisted_count, 0);
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
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityDecisionRecordingBoundarySideEffects::none()
        );
    }

    #[test]
    fn local_authority_decision_recording_boundary_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.recording_boundary_route
                == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-boundary/operator-live-approval-missing"
            && entry.authority_decision_record_id
                == "local-evidence-acceptance-authority-decision-record:controlled-live-evidence-receipt-store:operator_live_approval_missing:not-recorded"
            && entry.authority_decision_idempotency_key
                == "controlled-live-evidence-receipt-store.local-evidence-acceptance-authority-decision-recording.idempotency.operator_live_approval_missing"));
        assert!(report.entries.iter().all(entry_is_ready_blocked));
        assert!(report.entries.iter().all(|entry| {
            entry.recording_boundary_id == RECORDING_BOUNDARY_ID
                && entry.authority_decision_record_schema_version
                    == AUTHORITY_DECISION_RECORD_SCHEMA_VERSION
                && entry.source_terminal_state == "terminal_no_persistence"
                && entry.source_authority_packet_id
                    == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"
                && entry.source_authority_packet_route
                    == "operator-packet://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority"
        }));
    }
}
