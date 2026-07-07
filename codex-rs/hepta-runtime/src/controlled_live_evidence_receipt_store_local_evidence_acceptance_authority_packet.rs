use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions::controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_v1";
const PACKET_ID: &str =
    "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet";
const PACKET_ROUTE: &str =
    "operator-packet://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority";
const PACKET_FINGERPRINT: &str = "sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-no-acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_positive_preconditions_ready: bool,
    pub source_precondition_entry_count: usize,
    pub source_positive_preconditions_missing_count: usize,
    pub source_local_evidence_acceptance_allowed_count: usize,
    pub source_local_evidence_acceptance_recorded_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub authority_packet_id: &'static str,
    pub authority_packet_route: &'static str,
    pub authority_packet_payload_fingerprint: &'static str,
    pub packet_entry_count: usize,
    pub packet_projected_count: usize,
    pub packet_ready_count: usize,
    pub authority_packet_key_projected_count: usize,
    pub authority_packet_key_unique_count: usize,
    pub authority_checklist_projected_count: usize,
    pub authority_item_required_count: usize,
    pub authority_item_present_count: usize,
    pub source_positive_preconditions_attached_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub source_persistence_denial_attached_count: usize,
    pub source_denial_receipt_attached_count: usize,
    pub source_acceptance_source_record_attached_count: usize,
    pub local_acceptance_authority_required_count: usize,
    pub local_acceptance_authority_present_count: usize,
    pub operator_local_acceptance_approval_required_count: usize,
    pub operator_local_acceptance_approval_present_count: usize,
    pub dev_evidence_acceptance_source_required_count: usize,
    pub dev_evidence_acceptance_source_present_count: usize,
    pub evidence_payload_source_binding_required_count: usize,
    pub evidence_payload_source_binding_present_count: usize,
    pub local_evidence_store_feature_gate_required_count: usize,
    pub local_evidence_store_feature_gate_enabled_count: usize,
    pub local_receipt_store_feature_gate_required_count: usize,
    pub local_receipt_store_feature_gate_enabled_count: usize,
    pub atomic_acceptance_append_required_count: usize,
    pub atomic_acceptance_append_enabled_count: usize,
    pub post_acceptance_readback_required_count: usize,
    pub post_acceptance_readback_persisted_count: usize,
    pub rollback_anchor_required_count: usize,
    pub rollback_anchor_verified_count: usize,
    pub retention_policy_commit_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub replay_idempotency_guard_required_count: usize,
    pub replay_idempotency_guard_enabled_count: usize,
    pub authority_decision_request_projected_count: usize,
    pub authority_decision_recorded_count: usize,
    pub non_authority_receipt_projected_count: usize,
    pub non_authority_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
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
    pub local_evidence_acceptance_authority_packet_readback_ready: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
    pub authority_decision_recording_allowed: bool,
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
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry {
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_positive_precondition_set_id: String,
    pub source_positive_precondition_key: String,
    pub source_positive_precondition_route: String,
    pub source_terminal_no_persistence_entry_id: String,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_route: String,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_acceptance_source_record_id: String,
    pub source_local_acceptance_authority_precondition_id: String,
    pub source_operator_local_acceptance_approval_precondition_id: String,
    pub source_dev_evidence_acceptance_source_precondition_id: String,
    pub source_evidence_payload_source_binding_precondition_id: String,
    pub source_local_evidence_store_feature_gate_precondition_id: String,
    pub source_local_receipt_store_feature_gate_precondition_id: String,
    pub source_atomic_acceptance_append_precondition_id: String,
    pub source_post_acceptance_readback_precondition_id: String,
    pub source_rollback_anchor_precondition_id: String,
    pub source_retention_policy_commit_precondition_id: String,
    pub source_replay_idempotency_guard_precondition_id: String,
    pub authority_packet_id: &'static str,
    pub authority_packet_route: &'static str,
    pub authority_packet_payload_fingerprint: &'static str,
    pub authority_packet_key: String,
    pub authority_decision_request_id: String,
    pub authority_decision_request_route: String,
    pub non_authority_receipt_id: String,
    pub non_authority_receipt_route: String,
    pub observed_state: &'static str,
    pub packet_projected: bool,
    pub packet_ready: bool,
    pub authority_packet_key_projected: bool,
    pub authority_checklist_projected: bool,
    pub authority_item_required_count: usize,
    pub authority_item_present_count: usize,
    pub source_positive_preconditions_attached: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_denial_receipt_binding_attached: bool,
    pub source_acceptance_source_record_attached: bool,
    pub local_acceptance_authority_required: bool,
    pub local_acceptance_authority_present: bool,
    pub operator_local_acceptance_approval_required: bool,
    pub operator_local_acceptance_approval_present: bool,
    pub dev_evidence_acceptance_source_required: bool,
    pub dev_evidence_acceptance_source_present: bool,
    pub evidence_payload_source_binding_required: bool,
    pub evidence_payload_source_binding_present: bool,
    pub local_evidence_store_feature_gate_required: bool,
    pub local_evidence_store_feature_gate_enabled: bool,
    pub local_receipt_store_feature_gate_required: bool,
    pub local_receipt_store_feature_gate_enabled: bool,
    pub atomic_acceptance_append_required: bool,
    pub atomic_acceptance_append_enabled: bool,
    pub post_acceptance_readback_required: bool,
    pub post_acceptance_readback_persisted: bool,
    pub rollback_anchor_required: bool,
    pub rollback_anchor_verified: bool,
    pub retention_policy_commit_required: bool,
    pub retention_policy_committed: bool,
    pub replay_idempotency_guard_required: bool,
    pub replay_idempotency_guard_enabled: bool,
    pub authority_decision_request_projected: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_projected: bool,
    pub non_authority_receipt_persisted: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub local_evidence_acceptance_authority_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketSideEffects {
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub local_evidence_acceptance_authority_accepted: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_persisted: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report() -> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketReport
{
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_entries();

    let packet_projected_count = count(&entries, |entry| entry.packet_projected);
    let packet_ready_count = count(&entries, |entry| entry.packet_ready);
    let authority_packet_key_projected_count =
        count(&entries, |entry| entry.authority_packet_key_projected);
    let authority_packet_key_unique_count = entries
        .iter()
        .map(|entry| entry.authority_packet_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let authority_checklist_projected_count =
        count(&entries, |entry| entry.authority_checklist_projected);
    let authority_item_required_count = entries
        .iter()
        .map(|entry| entry.authority_item_required_count)
        .sum();
    let authority_item_present_count = entries
        .iter()
        .map(|entry| entry.authority_item_present_count)
        .sum();
    let source_positive_preconditions_attached_count = count(&entries, |entry| {
        entry.source_positive_preconditions_attached
    });
    let source_terminal_closeout_attached_count =
        count(&entries, |entry| entry.source_terminal_closeout_attached);
    let source_persistence_denial_attached_count =
        count(&entries, |entry| entry.source_persistence_denial_attached);
    let source_denial_receipt_attached_count = count(&entries, |entry| {
        entry.source_denial_receipt_binding_attached
    });
    let source_acceptance_source_record_attached_count = count(&entries, |entry| {
        entry.source_acceptance_source_record_attached
    });
    let local_acceptance_authority_required_count =
        count(&entries, |entry| entry.local_acceptance_authority_required);
    let local_acceptance_authority_present_count =
        count(&entries, |entry| entry.local_acceptance_authority_present);
    let operator_local_acceptance_approval_required_count = count(&entries, |entry| {
        entry.operator_local_acceptance_approval_required
    });
    let operator_local_acceptance_approval_present_count = count(&entries, |entry| {
        entry.operator_local_acceptance_approval_present
    });
    let dev_evidence_acceptance_source_required_count = count(&entries, |entry| {
        entry.dev_evidence_acceptance_source_required
    });
    let dev_evidence_acceptance_source_present_count = count(&entries, |entry| {
        entry.dev_evidence_acceptance_source_present
    });
    let evidence_payload_source_binding_required_count = count(&entries, |entry| {
        entry.evidence_payload_source_binding_required
    });
    let evidence_payload_source_binding_present_count = count(&entries, |entry| {
        entry.evidence_payload_source_binding_present
    });
    let local_evidence_store_feature_gate_required_count = count(&entries, |entry| {
        entry.local_evidence_store_feature_gate_required
    });
    let local_evidence_store_feature_gate_enabled_count = count(&entries, |entry| {
        entry.local_evidence_store_feature_gate_enabled
    });
    let local_receipt_store_feature_gate_required_count = count(&entries, |entry| {
        entry.local_receipt_store_feature_gate_required
    });
    let local_receipt_store_feature_gate_enabled_count = count(&entries, |entry| {
        entry.local_receipt_store_feature_gate_enabled
    });
    let atomic_acceptance_append_required_count =
        count(&entries, |entry| entry.atomic_acceptance_append_required);
    let atomic_acceptance_append_enabled_count =
        count(&entries, |entry| entry.atomic_acceptance_append_enabled);
    let post_acceptance_readback_required_count =
        count(&entries, |entry| entry.post_acceptance_readback_required);
    let post_acceptance_readback_persisted_count =
        count(&entries, |entry| entry.post_acceptance_readback_persisted);
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
    let authority_decision_request_projected_count =
        count(&entries, |entry| entry.authority_decision_request_projected);
    let authority_decision_recorded_count =
        count(&entries, |entry| entry.authority_decision_recorded);
    let non_authority_receipt_projected_count =
        count(&entries, |entry| entry.non_authority_receipt_projected);
    let non_authority_receipt_persisted_count =
        count(&entries, |entry| entry.non_authority_receipt_persisted);
    let operator_packet_sent_count = count(&entries, |entry| entry.operator_packet_sent);
    let operator_packet_persisted_count = count(&entries, |entry| entry.operator_packet_persisted);
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
        entry.receipt_persisted || entry.non_authority_receipt_persisted
    });
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let local_evidence_acceptance_authority_packet_readback_ready = source
        .local_evidence_acceptance_positive_preconditions_readback_ready
        && source.precondition_entry_count == 7
        && source.positive_preconditions_missing_count == 7
        && source.local_evidence_acceptance_allowed_count == 0
        && source.local_evidence_acceptance_recorded_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && packet_projected_count == 7
        && packet_ready_count == 7
        && authority_packet_key_projected_count == 7
        && authority_packet_key_unique_count == 7
        && authority_checklist_projected_count == 7
        && authority_item_required_count == 77
        && authority_item_present_count == 0
        && source_positive_preconditions_attached_count == 7
        && source_terminal_closeout_attached_count == 7
        && source_persistence_denial_attached_count == 7
        && source_denial_receipt_attached_count == 7
        && source_acceptance_source_record_attached_count == 7
        && local_acceptance_authority_required_count == 7
        && local_acceptance_authority_present_count == 0
        && operator_local_acceptance_approval_required_count == 7
        && operator_local_acceptance_approval_present_count == 0
        && dev_evidence_acceptance_source_required_count == 7
        && dev_evidence_acceptance_source_present_count == 0
        && evidence_payload_source_binding_required_count == 7
        && evidence_payload_source_binding_present_count == 0
        && local_evidence_store_feature_gate_required_count == 7
        && local_evidence_store_feature_gate_enabled_count == 0
        && local_receipt_store_feature_gate_required_count == 7
        && local_receipt_store_feature_gate_enabled_count == 0
        && atomic_acceptance_append_required_count == 7
        && atomic_acceptance_append_enabled_count == 0
        && post_acceptance_readback_required_count == 7
        && post_acceptance_readback_persisted_count == 0
        && rollback_anchor_required_count == 7
        && rollback_anchor_verified_count == 0
        && retention_policy_commit_required_count == 7
        && retention_policy_committed_count == 0
        && replay_idempotency_guard_required_count == 7
        && replay_idempotency_guard_enabled_count == 0
        && authority_decision_request_projected_count == 7
        && authority_decision_recorded_count == 0
        && non_authority_receipt_projected_count == 7
        && non_authority_receipt_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if local_evidence_acceptance_authority_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: GATE,
        schema_version: SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_positive_preconditions_ready: source
            .local_evidence_acceptance_positive_preconditions_readback_ready,
        source_precondition_entry_count: source.precondition_entry_count,
        source_positive_preconditions_missing_count: source.positive_preconditions_missing_count,
        source_local_evidence_acceptance_allowed_count: source
            .local_evidence_acceptance_allowed_count,
        source_local_evidence_acceptance_recorded_count: source
            .local_evidence_acceptance_recorded_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source
            .receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        authority_packet_id: PACKET_ID,
        authority_packet_route: PACKET_ROUTE,
        authority_packet_payload_fingerprint: PACKET_FINGERPRINT,
        packet_entry_count: entries.len(),
        packet_projected_count,
        packet_ready_count,
        authority_packet_key_projected_count,
        authority_packet_key_unique_count,
        authority_checklist_projected_count,
        authority_item_required_count,
        authority_item_present_count,
        source_positive_preconditions_attached_count,
        source_terminal_closeout_attached_count,
        source_persistence_denial_attached_count,
        source_denial_receipt_attached_count,
        source_acceptance_source_record_attached_count,
        local_acceptance_authority_required_count,
        local_acceptance_authority_present_count,
        operator_local_acceptance_approval_required_count,
        operator_local_acceptance_approval_present_count,
        dev_evidence_acceptance_source_required_count,
        dev_evidence_acceptance_source_present_count,
        evidence_payload_source_binding_required_count,
        evidence_payload_source_binding_present_count,
        local_evidence_store_feature_gate_required_count,
        local_evidence_store_feature_gate_enabled_count,
        local_receipt_store_feature_gate_required_count,
        local_receipt_store_feature_gate_enabled_count,
        atomic_acceptance_append_required_count,
        atomic_acceptance_append_enabled_count,
        post_acceptance_readback_required_count,
        post_acceptance_readback_persisted_count,
        rollback_anchor_required_count,
        rollback_anchor_verified_count,
        retention_policy_commit_required_count,
        retention_policy_committed_count,
        replay_idempotency_guard_required_count,
        replay_idempotency_guard_enabled_count,
        authority_decision_request_projected_count,
        authority_decision_recorded_count,
        non_authority_receipt_projected_count,
        non_authority_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
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
        local_evidence_acceptance_authority_packet_readback_ready,
        operator_packet_send_allowed: false,
        operator_packet_sent: false,
        operator_packet_persistence_allowed: false,
        operator_packet_persisted: false,
        local_evidence_acceptance_authority_allowed: false,
        authority_decision_recording_allowed: false,
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
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "local_evidence_acceptance_authority_missing",
            "operator_local_acceptance_approval_missing",
            "dev_evidence_acceptance_source_missing",
            "evidence_payload_source_binding_missing",
            "local_evidence_store_feature_gate_closed",
            "local_receipt_store_feature_gate_closed",
            "atomic_acceptance_append_not_enabled",
            "post_acceptance_readback_missing",
            "rollback_anchor_missing",
            "retention_policy_not_committed",
            "replay_idempotency_guard_disabled",
            "authority_decision_recording_disabled",
            "non_authority_receipt_persistence_disabled",
            "local_evidence_acceptance_disabled",
            "evidence_acceptance_recording_disabled",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry> {
    controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_authority_packet_without_acceptance_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_positive_precondition_set_id: entry.positive_precondition_set_id,
                source_positive_precondition_key: entry.positive_precondition_key,
                source_positive_precondition_route: entry.positive_precondition_route,
                source_terminal_no_persistence_entry_id: entry.source_terminal_no_persistence_entry_id,
                source_terminal_closeout_id: entry.source_terminal_closeout_id,
                source_terminal_closeout_route: entry.source_terminal_closeout_route,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_acceptance_source_record_id: entry.source_acceptance_source_record_id,
                source_local_acceptance_authority_precondition_id: entry
                    .local_acceptance_authority_precondition_id,
                source_operator_local_acceptance_approval_precondition_id: entry
                    .operator_local_acceptance_approval_precondition_id,
                source_dev_evidence_acceptance_source_precondition_id: entry
                    .dev_evidence_acceptance_source_precondition_id,
                source_evidence_payload_source_binding_precondition_id: entry
                    .evidence_payload_source_binding_precondition_id,
                source_local_evidence_store_feature_gate_precondition_id: entry
                    .local_evidence_store_feature_gate_precondition_id,
                source_local_receipt_store_feature_gate_precondition_id: entry
                    .local_receipt_store_feature_gate_precondition_id,
                source_atomic_acceptance_append_precondition_id: entry
                    .atomic_acceptance_append_precondition_id,
                source_post_acceptance_readback_precondition_id: entry
                    .post_acceptance_readback_precondition_id,
                source_rollback_anchor_precondition_id: entry.rollback_anchor_precondition_id,
                source_retention_policy_commit_precondition_id: entry
                    .retention_policy_commit_precondition_id,
                source_replay_idempotency_guard_precondition_id: entry
                    .replay_idempotency_guard_precondition_id,
                authority_packet_id: PACKET_ID,
                authority_packet_route: PACKET_ROUTE,
                authority_packet_payload_fingerprint: PACKET_FINGERPRINT,
                authority_packet_key: format!(
                    "local-evidence-acceptance-authority-packet:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                authority_decision_request_id: format!(
                    "local-evidence-acceptance-authority-decision-request:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                authority_decision_request_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/decision-request/{hyphenated}"
                ),
                non_authority_receipt_id: format!(
                    "local-evidence-acceptance-non-authority-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                non_authority_receipt_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/non-authority-receipts/{hyphenated}"
                ),
                observed_state:
                    "local_evidence_acceptance_authority_packet_projected_without_acceptance",
                packet_projected: true,
                packet_ready: true,
                authority_packet_key_projected: true,
                authority_checklist_projected: true,
                authority_item_required_count: 11,
                authority_item_present_count: 0,
                source_positive_preconditions_attached: entry.positive_precondition_set_projected,
                source_terminal_closeout_attached: entry.source_terminal_closeout_attached,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_denial_receipt_binding_attached: entry.source_denial_receipt_binding_attached,
                source_acceptance_source_record_attached: entry
                    .source_acceptance_source_record_attached,
                local_acceptance_authority_required: entry.local_acceptance_authority_required,
                local_acceptance_authority_present: entry.local_acceptance_authority_present,
                operator_local_acceptance_approval_required: entry
                    .operator_local_acceptance_approval_required,
                operator_local_acceptance_approval_present: entry
                    .operator_local_acceptance_approval_present,
                dev_evidence_acceptance_source_required: entry
                    .dev_evidence_acceptance_source_required,
                dev_evidence_acceptance_source_present: entry.dev_evidence_acceptance_source_present,
                evidence_payload_source_binding_required: entry
                    .evidence_payload_source_binding_required,
                evidence_payload_source_binding_present: entry
                    .evidence_payload_source_binding_present,
                local_evidence_store_feature_gate_required: entry
                    .local_evidence_store_feature_gate_required,
                local_evidence_store_feature_gate_enabled: entry
                    .local_evidence_store_feature_gate_enabled,
                local_receipt_store_feature_gate_required: entry
                    .local_receipt_store_feature_gate_required,
                local_receipt_store_feature_gate_enabled: entry
                    .local_receipt_store_feature_gate_enabled,
                atomic_acceptance_append_required: entry.atomic_acceptance_append_required,
                atomic_acceptance_append_enabled: entry.atomic_acceptance_append_enabled,
                post_acceptance_readback_required: entry.post_acceptance_readback_required,
                post_acceptance_readback_persisted: entry.post_acceptance_readback_persisted,
                rollback_anchor_required: entry.rollback_anchor_required,
                rollback_anchor_verified: entry.rollback_anchor_verified,
                retention_policy_commit_required: entry.retention_policy_commit_required,
                retention_policy_committed: entry.retention_policy_committed,
                replay_idempotency_guard_required: entry.replay_idempotency_guard_required,
                replay_idempotency_guard_enabled: entry.replay_idempotency_guard_enabled,
                authority_decision_request_projected: true,
                authority_decision_recorded: false,
                non_authority_receipt_projected: true,
                non_authority_receipt_persisted: false,
                operator_packet_send_allowed: false,
                operator_packet_sent: false,
                operator_packet_persistence_allowed: false,
                operator_packet_persisted: false,
                local_evidence_acceptance_authority_allowed: false,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry],
    predicate: impl Fn(
        &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry,
    ) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_authority_packet_projected_without_acceptance"
        && entry.packet_projected
        && entry.packet_ready
        && entry.authority_packet_key_projected
        && entry.authority_checklist_projected
        && entry.authority_item_required_count == 11
        && entry.authority_item_present_count == 0
        && entry.source_positive_preconditions_attached
        && entry.source_terminal_closeout_attached
        && entry.source_persistence_denial_attached
        && entry.source_denial_receipt_binding_attached
        && entry.source_acceptance_source_record_attached
        && entry.local_acceptance_authority_required
        && !entry.local_acceptance_authority_present
        && entry.operator_local_acceptance_approval_required
        && !entry.operator_local_acceptance_approval_present
        && entry.dev_evidence_acceptance_source_required
        && !entry.dev_evidence_acceptance_source_present
        && entry.evidence_payload_source_binding_required
        && !entry.evidence_payload_source_binding_present
        && entry.local_evidence_store_feature_gate_required
        && !entry.local_evidence_store_feature_gate_enabled
        && entry.local_receipt_store_feature_gate_required
        && !entry.local_receipt_store_feature_gate_enabled
        && entry.atomic_acceptance_append_required
        && !entry.atomic_acceptance_append_enabled
        && entry.post_acceptance_readback_required
        && !entry.post_acceptance_readback_persisted
        && entry.rollback_anchor_required
        && !entry.rollback_anchor_verified
        && entry.retention_policy_commit_required
        && !entry.retention_policy_committed
        && entry.replay_idempotency_guard_required
        && !entry.replay_idempotency_guard_enabled
        && entry.authority_decision_request_projected
        && !entry.authority_decision_recorded
        && entry.non_authority_receipt_projected
        && !entry.non_authority_receipt_persisted
        && !entry.operator_packet_send_allowed
        && !entry.operator_packet_sent
        && !entry.operator_packet_persistence_allowed
        && !entry.operator_packet_persisted
        && !entry.local_evidence_acceptance_authority_allowed
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketSideEffects {
    pub const fn none() -> Self {
        Self {
            operator_packet_sent: false,
            operator_packet_persisted: false,
            local_evidence_acceptance_authority_accepted: false,
            authority_decision_recorded: false,
            non_authority_receipt_persisted: false,
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
    fn local_evidence_acceptance_authority_packet_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_positive_preconditions_ready);
        assert_eq!(report.source_precondition_entry_count, 7);
        assert_eq!(report.source_positive_preconditions_missing_count, 7);
        assert_eq!(report.packet_entry_count, 7);
        assert_eq!(report.packet_projected_count, 7);
        assert_eq!(report.packet_ready_count, 7);
        assert_eq!(report.authority_packet_key_projected_count, 7);
        assert_eq!(report.authority_packet_key_unique_count, 7);
        assert_eq!(report.authority_checklist_projected_count, 7);
        assert_eq!(report.authority_item_required_count, 77);
        assert_eq!(report.authority_item_present_count, 0);
        assert_eq!(report.local_acceptance_authority_required_count, 7);
        assert_eq!(report.local_acceptance_authority_present_count, 0);
        assert_eq!(report.operator_local_acceptance_approval_required_count, 7);
        assert_eq!(report.operator_local_acceptance_approval_present_count, 0);
        assert_eq!(report.dev_evidence_acceptance_source_required_count, 7);
        assert_eq!(report.dev_evidence_acceptance_source_present_count, 0);
        assert_eq!(report.authority_decision_request_projected_count, 7);
        assert!(report.local_evidence_acceptance_authority_packet_readback_ready);
    }

    #[test]
    fn local_evidence_acceptance_authority_packet_keeps_acceptance_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_report();

        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.non_authority_receipt_projected_count, 7);
        assert_eq!(report.non_authority_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
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
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.local_evidence_acceptance_authority_allowed);
        assert!(!report.local_evidence_acceptance_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptanceAuthorityPacketSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_authority_packet_entries_are_stable_and_unsent() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.authority_decision_request_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/decision-request/dirty-worktree-boundary"
                && entry.non_authority_receipt_id == "local-evidence-acceptance-non-authority-receipt:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "operator_live_approval_missing"
                && entry.source_operator_local_acceptance_approval_precondition_id
                    == "operator-local-evidence-acceptance-approval-required:controlled-live-evidence-receipt-store:operator_live_approval_missing"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.authority_packet_id == PACKET_ID
                && entry.authority_packet_route == PACKET_ROUTE
                && entry.authority_packet_payload_fingerprint == PACKET_FINGERPRINT
                && entry.packet_projected
                && entry.packet_ready
                && entry.authority_packet_key_projected
                && entry.authority_checklist_projected
                && entry.authority_item_required_count == 11
                && entry.authority_item_present_count == 0
                && entry.source_positive_preconditions_attached
                && entry.authority_decision_request_projected
                && !entry.authority_decision_recorded
                && entry.non_authority_receipt_projected
                && !entry.non_authority_receipt_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && !entry.local_evidence_acceptance_authority_allowed
                && !entry.local_evidence_acceptance_allowed
                && !entry.local_evidence_acceptance_recorded
                && !entry.evidence_acceptance_recorded
                && !entry.evidence_recorded
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.ledger_written
                && !entry.workflow_event_log_written
                && !entry.sqlite_written
                && !entry.live_mutation_allowed
        }));
    }
}
