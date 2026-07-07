use std::collections::BTreeSet;
use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence::controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance";

const SURFACE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance";
const GATE: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_gate";
const SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_v1";
const POSITIVE_PRECONDITIONS_ROUTE: &str = "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/positive-preconditions";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsReport {
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
    pub source_terminal_closeout_key_unique_count: usize,
    pub source_terminal_closeout_recorded_count: usize,
    pub source_terminal_closeout_persisted_count: usize,
    pub source_terminal_closeout_accepted_count: usize,
    pub source_terminal_closeout_authoritative_count: usize,
    pub source_denial_receipt_persistence_attempt_recorded_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_acceptance_source_recorded_count: usize,
    pub source_acceptance_source_persisted_count: usize,
    pub source_evidence_acceptance_recorded_count: usize,
    pub source_evidence_recorded_count: usize,
    pub source_receipt_store_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub positive_preconditions_route: &'static str,
    pub precondition_entry_count: usize,
    pub positive_precondition_set_projected_count: usize,
    pub positive_precondition_key_projected_count: usize,
    pub positive_precondition_key_unique_count: usize,
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
    pub positive_preconditions_missing_count: usize,
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
    pub local_evidence_acceptance_positive_preconditions_readback_ready: bool,
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
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry {
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_terminal_no_persistence_entry_id: String,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_key: String,
    pub source_terminal_closeout_route: String,
    pub source_terminal_reason: &'static str,
    pub source_persistence_denial_id: String,
    pub source_persistence_denial_route: String,
    pub source_denial_receipt_id: String,
    pub source_denial_receipt_route: String,
    pub source_denial_receipt_digest: String,
    pub source_acceptance_source_record_id: String,
    pub positive_precondition_set_id: String,
    pub positive_precondition_key: String,
    pub positive_precondition_route: String,
    pub local_acceptance_authority_precondition_id: String,
    pub operator_local_acceptance_approval_precondition_id: String,
    pub dev_evidence_acceptance_source_precondition_id: String,
    pub evidence_payload_source_binding_precondition_id: String,
    pub local_evidence_store_feature_gate_precondition_id: String,
    pub local_receipt_store_feature_gate_precondition_id: String,
    pub atomic_acceptance_append_precondition_id: String,
    pub post_acceptance_readback_precondition_id: String,
    pub rollback_anchor_precondition_id: String,
    pub retention_policy_commit_precondition_id: String,
    pub replay_idempotency_guard_precondition_id: String,
    pub observed_state: &'static str,
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
    pub positive_precondition_set_projected: bool,
    pub positive_precondition_key_projected: bool,
    pub source_terminal_closeout_attached: bool,
    pub source_persistence_denial_attached: bool,
    pub source_denial_receipt_binding_attached: bool,
    pub source_acceptance_source_record_attached: bool,
    pub positive_preconditions_missing: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsSideEffects
{
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

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsReport,
    > = OnceLock::new();
    REPORT.get_or_init(build_report).clone()
}

fn build_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsReport {
    let source =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_entries();

    let positive_precondition_set_projected_count =
        count(&entries, |entry| entry.positive_precondition_set_projected);
    let positive_precondition_key_projected_count =
        count(&entries, |entry| entry.positive_precondition_key_projected);
    let positive_precondition_key_unique_count = entries
        .iter()
        .map(|entry| entry.positive_precondition_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
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
    let positive_preconditions_missing_count =
        count(&entries, |entry| entry.positive_preconditions_missing);
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
    let receipt_persisted_count = count(&entries, |entry| entry.receipt_persisted);
    let ledger_written_count = count(&entries, |entry| entry.ledger_written);
    let workflow_event_log_written_count =
        count(&entries, |entry| entry.workflow_event_log_written);
    let sqlite_written_count = count(&entries, |entry| entry.sqlite_written);
    let live_mutation_allowed_count = count(&entries, |entry| entry.live_mutation_allowed);

    let local_evidence_acceptance_positive_preconditions_readback_ready = source
        .terminal_no_persistence_readback_ready
        && source.terminal_entry_count == 7
        && source.terminal_closeout_projected_count == 7
        && source.terminal_no_persistence_confirmed_count == 7
        && source.terminal_closeout_key_unique_count == 7
        && source.terminal_closeout_recorded_count == 0
        && source.terminal_closeout_persisted_count == 0
        && source.terminal_closeout_accepted_count == 0
        && source.terminal_closeout_authoritative_count == 0
        && source.denial_receipt_persistence_attempt_recorded_count == 0
        && source.denial_receipt_persisted_count == 0
        && source.acceptance_source_recorded_count == 0
        && source.acceptance_source_persisted_count == 0
        && source.evidence_acceptance_recorded_count == 0
        && source.evidence_recorded_count == 0
        && source.receipt_store_write_attempt_recorded_count == 0
        && source.receipt_store_written_count == 0
        && source.receipt_persisted_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && positive_precondition_set_projected_count == 7
        && positive_precondition_key_projected_count == 7
        && positive_precondition_key_unique_count == 7
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
        && positive_preconditions_missing_count == 7
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

    ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsReport {
        runtime: "hepta",
        surface: SURFACE,
        status: if local_evidence_acceptance_positive_preconditions_readback_ready {
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
        source_terminal_no_persistence_confirmed_count: source
            .terminal_no_persistence_confirmed_count,
        source_terminal_closeout_key_unique_count: source.terminal_closeout_key_unique_count,
        source_terminal_closeout_recorded_count: source.terminal_closeout_recorded_count,
        source_terminal_closeout_persisted_count: source.terminal_closeout_persisted_count,
        source_terminal_closeout_accepted_count: source.terminal_closeout_accepted_count,
        source_terminal_closeout_authoritative_count: source
            .terminal_closeout_authoritative_count,
        source_denial_receipt_persistence_attempt_recorded_count: source
            .denial_receipt_persistence_attempt_recorded_count,
        source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
        source_acceptance_source_recorded_count: source.acceptance_source_recorded_count,
        source_acceptance_source_persisted_count: source.acceptance_source_persisted_count,
        source_evidence_acceptance_recorded_count: source.evidence_acceptance_recorded_count,
        source_evidence_recorded_count: source.evidence_recorded_count,
        source_receipt_store_write_attempt_recorded_count: source
            .receipt_store_write_attempt_recorded_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_receipt_persisted_count: source.receipt_persisted_count,
        source_live_execution_allowed: source.live_execution_allowed,
        positive_preconditions_route: POSITIVE_PRECONDITIONS_ROUTE,
        precondition_entry_count: entries.len(),
        positive_precondition_set_projected_count,
        positive_precondition_key_projected_count,
        positive_precondition_key_unique_count,
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
        positive_preconditions_missing_count,
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
        local_evidence_acceptance_positive_preconditions_readback_ready,
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
            "local_acceptance_authority_missing",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_POSITIVE_PRECONDITIONS_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsSideEffects::none(),
    }
}

pub fn controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry> {
    controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_acceptance_positive_preconditions_without_acceptance_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_terminal_no_persistence_entry_id: entry.id,
                source_terminal_closeout_id: entry.terminal_closeout_id,
                source_terminal_closeout_key: entry.terminal_closeout_key,
                source_terminal_closeout_route: entry.terminal_closeout_route,
                source_terminal_reason: entry.terminal_reason,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                source_persistence_denial_route: entry.source_persistence_denial_route,
                source_denial_receipt_id: entry.source_denial_receipt_id,
                source_denial_receipt_route: entry.source_denial_receipt_route,
                source_denial_receipt_digest: entry.source_denial_receipt_digest,
                source_acceptance_source_record_id: entry.source_acceptance_source_record_id,
                positive_precondition_set_id: format!(
                    "local-evidence-acceptance-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_key: format!(
                    "local-evidence-acceptance-positive-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                positive_precondition_route: format!("{POSITIVE_PRECONDITIONS_ROUTE}/{hyphenated}"),
                local_acceptance_authority_precondition_id: format!(
                    "local-evidence-acceptance-authority-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_local_acceptance_approval_precondition_id: format!(
                    "operator-local-evidence-acceptance-approval-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                dev_evidence_acceptance_source_precondition_id: format!(
                    "dev-evidence-acceptance-source-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                evidence_payload_source_binding_precondition_id: format!(
                    "evidence-payload-source-binding-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_evidence_store_feature_gate_precondition_id: format!(
                    "local-evidence-store-feature-gate-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_receipt_store_feature_gate_precondition_id: format!(
                    "local-receipt-store-feature-gate-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                atomic_acceptance_append_precondition_id: format!(
                    "atomic-local-evidence-acceptance-append-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_acceptance_readback_precondition_id: format!(
                    "post-local-evidence-acceptance-readback-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                rollback_anchor_precondition_id: format!(
                    "local-evidence-acceptance-rollback-anchor-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                retention_policy_commit_precondition_id: format!(
                    "local-evidence-acceptance-retention-policy-commit-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_guard_precondition_id: format!(
                    "local-evidence-acceptance-replay-idempotency-guard-required:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                observed_state:
                    "local_evidence_acceptance_positive_preconditions_projected_without_acceptance",
                local_acceptance_authority_required: true,
                local_acceptance_authority_present: false,
                operator_local_acceptance_approval_required: true,
                operator_local_acceptance_approval_present: false,
                dev_evidence_acceptance_source_required: true,
                dev_evidence_acceptance_source_present: false,
                evidence_payload_source_binding_required: true,
                evidence_payload_source_binding_present: false,
                local_evidence_store_feature_gate_required: true,
                local_evidence_store_feature_gate_enabled: false,
                local_receipt_store_feature_gate_required: true,
                local_receipt_store_feature_gate_enabled: false,
                atomic_acceptance_append_required: true,
                atomic_acceptance_append_enabled: false,
                post_acceptance_readback_required: true,
                post_acceptance_readback_persisted: false,
                rollback_anchor_required: true,
                rollback_anchor_verified: false,
                retention_policy_commit_required: true,
                retention_policy_committed: false,
                replay_idempotency_guard_required: true,
                replay_idempotency_guard_enabled: false,
                positive_precondition_set_projected: true,
                positive_precondition_key_projected: true,
                source_terminal_closeout_attached: entry.terminal_closeout_projected,
                source_persistence_denial_attached: entry.source_persistence_denial_attached,
                source_denial_receipt_binding_attached: entry
                    .source_denial_receipt_binding_attached,
                source_acceptance_source_record_attached: entry
                    .source_acceptance_source_record_attached,
                positive_preconditions_missing: true,
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
    entries: &[ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry],
    predicate: impl Fn(
        &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry,
    ) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn entry_is_ready_blocked(
    entry: &ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsEntry,
) -> bool {
    entry.observed_state
        == "local_evidence_acceptance_positive_preconditions_projected_without_acceptance"
        && entry.positive_precondition_set_projected
        && entry.positive_precondition_key_projected
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
        && entry.positive_preconditions_missing
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsSideEffects {
    pub const fn none() -> Self {
        Self {
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
    fn local_evidence_acceptance_positive_preconditions_project_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.local_evidence_acceptance_positive_preconditions_readback_ready);
        assert_eq!(report.source_terminal_entry_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.positive_precondition_set_projected_count, 7);
        assert_eq!(report.positive_precondition_key_projected_count, 7);
        assert_eq!(report.positive_precondition_key_unique_count, 7);
        assert_eq!(report.source_terminal_closeout_attached_count, 7);
        assert_eq!(report.source_persistence_denial_attached_count, 7);
        assert_eq!(report.source_denial_receipt_attached_count, 7);
        assert_eq!(report.source_acceptance_source_record_attached_count, 7);
        assert_eq!(report.local_acceptance_authority_required_count, 7);
        assert_eq!(report.local_acceptance_authority_present_count, 0);
        assert_eq!(report.operator_local_acceptance_approval_required_count, 7);
        assert_eq!(report.operator_local_acceptance_approval_present_count, 0);
        assert_eq!(report.dev_evidence_acceptance_source_required_count, 7);
        assert_eq!(report.dev_evidence_acceptance_source_present_count, 0);
        assert_eq!(report.evidence_payload_source_binding_required_count, 7);
        assert_eq!(report.evidence_payload_source_binding_present_count, 0);
        assert_eq!(report.positive_preconditions_missing_count, 7);
    }

    #[test]
    fn local_evidence_acceptance_positive_preconditions_keep_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report();

        assert_eq!(report.local_evidence_store_feature_gate_required_count, 7);
        assert_eq!(report.local_evidence_store_feature_gate_enabled_count, 0);
        assert_eq!(report.local_receipt_store_feature_gate_required_count, 7);
        assert_eq!(report.local_receipt_store_feature_gate_enabled_count, 0);
        assert_eq!(report.atomic_acceptance_append_required_count, 7);
        assert_eq!(report.atomic_acceptance_append_enabled_count, 0);
        assert_eq!(report.post_acceptance_readback_required_count, 7);
        assert_eq!(report.post_acceptance_readback_persisted_count, 0);
        assert_eq!(report.rollback_anchor_required_count, 7);
        assert_eq!(report.rollback_anchor_verified_count, 0);
        assert_eq!(report.retention_policy_commit_required_count, 7);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert_eq!(report.replay_idempotency_guard_required_count, 7);
        assert_eq!(report.replay_idempotency_guard_enabled_count, 0);
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
        assert!(!report.local_evidence_acceptance_allowed);
        assert!(!report.local_evidence_acceptance_recording_allowed);
        assert!(!report.evidence_acceptance_recording_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceAcceptancePositivePreconditionsSideEffects::none()
        );
    }

    #[test]
    fn local_evidence_acceptance_positive_precondition_entries_are_stable() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.positive_precondition_route
                    == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/positive-preconditions/dirty-worktree-boundary"
                && entry.local_acceptance_authority_precondition_id
                    == "local-evidence-acceptance-authority-required:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.id.starts_with("evidence_receipt_store_local_evidence_acceptance_positive_preconditions_without_acceptance_")
                && entry.source_terminal_no_persistence_entry_id.starts_with("evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_")
                && entry.source_persistence_denial_id.starts_with("local-evidence-acceptance-recording-denial-receipt-persistence-denial:")
                && entry.source_denial_receipt_id.starts_with("local-evidence-acceptance-recording-denial-receipt:")
                && entry.positive_precondition_set_id.starts_with("local-evidence-acceptance-positive-preconditions:")
                && entry_is_ready_blocked(entry)
        }));
    }
}
