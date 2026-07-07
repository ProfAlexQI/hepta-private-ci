use std::sync::OnceLock;

use crate::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback::controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_terminal_no_persistence_readback_ready: bool,
    pub source_terminal_entry_count: usize,
    pub source_terminal_no_persistence_confirmed_count: usize,
    pub source_terminal_closeout_recorded_count: usize,
    pub source_terminal_closeout_persisted_count: usize,
    pub source_terminal_closeout_accepted_count: usize,
    pub source_terminal_closeout_authoritative_count: usize,
    pub source_denial_receipt_persisted_count: usize,
    pub source_write_attempt_recorded_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_receipt_persisted_count: usize,
    pub source_live_execution_allowed: bool,
    pub open_precondition_entry_count: usize,
    pub open_precondition_catalog_ready_count: usize,
    pub source_terminal_closeout_attached_count: usize,
    pub operator_local_store_approval_required_count: usize,
    pub operator_local_store_approval_present_count: usize,
    pub dev_evidence_acceptance_source_required_count: usize,
    pub dev_evidence_acceptance_source_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub local_receipt_store_feature_gate_required_count: usize,
    pub local_receipt_store_feature_gate_enabled_count: usize,
    pub append_only_store_path_grant_required_count: usize,
    pub append_only_store_path_grant_present_count: usize,
    pub atomic_append_required_count: usize,
    pub atomic_append_enabled_count: usize,
    pub post_append_readback_required_count: usize,
    pub post_append_readback_persisted_count: usize,
    pub rollback_anchor_required_count: usize,
    pub rollback_anchor_verified_count: usize,
    pub retention_policy_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub replay_idempotency_guard_required_count: usize,
    pub replay_idempotency_guard_enabled_count: usize,
    pub local_store_open_allowed_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_write_attempt_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub local_open_preconditions_readback_ready: bool,
    pub local_evidence_receipt_store_open_allowed: bool,
    pub operator_approval_request_allowed: bool,
    pub evidence_acceptance_recording_allowed: bool,
    pub receipt_store_feature_gate_open_allowed: bool,
    pub append_only_store_write_allowed: bool,
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
        Vec<ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_terminal_closeout_id: String,
    pub source_terminal_closeout_key: String,
    pub source_terminal_closeout_route: String,
    pub source_terminal_reason: &'static str,
    pub source_persistence_denial_id: String,
    pub local_open_precondition_set_id: String,
    pub local_open_precondition_route: String,
    pub operator_local_store_approval_id: String,
    pub dev_evidence_acceptance_source_id: String,
    pub evidence_acceptance_key: String,
    pub local_receipt_store_feature_gate: String,
    pub append_only_store_path_grant_key: String,
    pub atomic_append_plan_id: String,
    pub post_append_readback_route: String,
    pub rollback_anchor_route: String,
    pub retention_policy_id: String,
    pub replay_idempotency_guard_key: String,
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
    pub source_terminal_closeout_attached: bool,
    pub terminal_no_persistence_confirmed: bool,
    pub operator_local_store_approval_required: bool,
    pub operator_local_store_approval_present: bool,
    pub dev_evidence_acceptance_source_required: bool,
    pub dev_evidence_acceptance_source_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub local_receipt_store_feature_gate_required: bool,
    pub local_receipt_store_feature_gate_enabled: bool,
    pub append_only_store_path_grant_required: bool,
    pub append_only_store_path_grant_present: bool,
    pub atomic_append_required: bool,
    pub atomic_append_enabled: bool,
    pub post_append_readback_required: bool,
    pub post_append_readback_persisted: bool,
    pub rollback_anchor_required: bool,
    pub rollback_anchor_verified: bool,
    pub retention_policy_required: bool,
    pub retention_policy_committed: bool,
    pub replay_idempotency_guard_required: bool,
    pub replay_idempotency_guard_enabled: bool,
    pub local_store_open_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteSideEffects
{
    pub operator_approval_requested: bool,
    pub evidence_acceptance_recorded: bool,
    pub local_receipt_store_feature_gate_opened: bool,
    pub append_only_store_path_granted: bool,
    pub atomic_append_enabled: bool,
    pub post_append_readback_persisted: bool,
    pub rollback_anchor_verified: bool,
    pub retention_policy_committed: bool,
    pub replay_idempotency_guard_enabled: bool,
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

pub fn controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report()
-> ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteReport{
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteReport> = OnceLock::new();
    REPORT
        .get_or_init(|| {
            let source =
                controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report();
            let entries =
                controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_entries();

            let open_precondition_catalog_ready_count = entries
                .iter()
                .filter(|entry| {
                    entry.source_terminal_closeout_attached
                        && entry.terminal_no_persistence_confirmed
                        && entry.operator_local_store_approval_required
                        && entry.dev_evidence_acceptance_source_required
                        && entry.evidence_acceptance_required
                        && entry.local_receipt_store_feature_gate_required
                        && entry.append_only_store_path_grant_required
                        && entry.atomic_append_required
                        && entry.post_append_readback_required
                        && entry.rollback_anchor_required
                        && entry.retention_policy_required
                        && entry.replay_idempotency_guard_required
                        && !entry.operator_local_store_approval_present
                        && !entry.dev_evidence_acceptance_source_present
                        && !entry.evidence_acceptance_present
                        && !entry.local_receipt_store_feature_gate_enabled
                        && !entry.append_only_store_path_grant_present
                        && !entry.atomic_append_enabled
                        && !entry.post_append_readback_persisted
                        && !entry.rollback_anchor_verified
                        && !entry.retention_policy_committed
                        && !entry.replay_idempotency_guard_enabled
                        && !entry.local_store_open_allowed
                        && !entry.receipt_store_write_allowed
                        && !entry.receipt_store_written
                        && !entry.receipt_persisted
                        && !entry.live_mutation_allowed
                })
                .count();
            let source_terminal_closeout_attached_count = entries
                .iter()
                .filter(|entry| entry.source_terminal_closeout_attached)
                .count();
            let operator_local_store_approval_required_count = entries
                .iter()
                .filter(|entry| entry.operator_local_store_approval_required)
                .count();
            let operator_local_store_approval_present_count = entries
                .iter()
                .filter(|entry| entry.operator_local_store_approval_present)
                .count();
            let dev_evidence_acceptance_source_required_count = entries
                .iter()
                .filter(|entry| entry.dev_evidence_acceptance_source_required)
                .count();
            let dev_evidence_acceptance_source_present_count = entries
                .iter()
                .filter(|entry| entry.dev_evidence_acceptance_source_present)
                .count();
            let evidence_acceptance_required_count = entries
                .iter()
                .filter(|entry| entry.evidence_acceptance_required)
                .count();
            let evidence_acceptance_present_count = entries
                .iter()
                .filter(|entry| entry.evidence_acceptance_present)
                .count();
            let local_receipt_store_feature_gate_required_count = entries
                .iter()
                .filter(|entry| entry.local_receipt_store_feature_gate_required)
                .count();
            let local_receipt_store_feature_gate_enabled_count = entries
                .iter()
                .filter(|entry| entry.local_receipt_store_feature_gate_enabled)
                .count();
            let append_only_store_path_grant_required_count = entries
                .iter()
                .filter(|entry| entry.append_only_store_path_grant_required)
                .count();
            let append_only_store_path_grant_present_count = entries
                .iter()
                .filter(|entry| entry.append_only_store_path_grant_present)
                .count();
            let atomic_append_required_count = entries
                .iter()
                .filter(|entry| entry.atomic_append_required)
                .count();
            let atomic_append_enabled_count = entries
                .iter()
                .filter(|entry| entry.atomic_append_enabled)
                .count();
            let post_append_readback_required_count = entries
                .iter()
                .filter(|entry| entry.post_append_readback_required)
                .count();
            let post_append_readback_persisted_count = entries
                .iter()
                .filter(|entry| entry.post_append_readback_persisted)
                .count();
            let rollback_anchor_required_count = entries
                .iter()
                .filter(|entry| entry.rollback_anchor_required)
                .count();
            let rollback_anchor_verified_count = entries
                .iter()
                .filter(|entry| entry.rollback_anchor_verified)
                .count();
            let retention_policy_required_count = entries
                .iter()
                .filter(|entry| entry.retention_policy_required)
                .count();
            let retention_policy_committed_count = entries
                .iter()
                .filter(|entry| entry.retention_policy_committed)
                .count();
            let replay_idempotency_guard_required_count = entries
                .iter()
                .filter(|entry| entry.replay_idempotency_guard_required)
                .count();
            let replay_idempotency_guard_enabled_count = entries
                .iter()
                .filter(|entry| entry.replay_idempotency_guard_enabled)
                .count();
            let local_store_open_allowed_count = entries
                .iter()
                .filter(|entry| entry.local_store_open_allowed)
                .count();
            let evidence_recorded_count = entries
                .iter()
                .filter(|entry| entry.evidence_recorded)
                .count();
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

            let local_open_preconditions_readback_ready =
                source.terminal_no_persistence_readback_ready
                    && source.terminal_entry_count == 7
                    && source.terminal_no_persistence_confirmed_count == 7
                    && source.terminal_closeout_recorded_count == 0
                    && source.terminal_closeout_persisted_count == 0
                    && source.terminal_closeout_accepted_count == 0
                    && source.terminal_closeout_authoritative_count == 0
                    && source.denial_receipt_persisted_count == 0
                    && source.write_attempt_recorded_count == 0
                    && source.receipt_store_written_count == 0
                    && source.receipt_persisted_count == 0
                    && !source.live_execution_allowed
                    && entries.len() == 7
                    && open_precondition_catalog_ready_count == 7
                    && source_terminal_closeout_attached_count == 7
                    && operator_local_store_approval_required_count == 7
                    && operator_local_store_approval_present_count == 0
                    && dev_evidence_acceptance_source_required_count == 7
                    && dev_evidence_acceptance_source_present_count == 0
                    && evidence_acceptance_required_count == 7
                    && evidence_acceptance_present_count == 0
                    && local_receipt_store_feature_gate_required_count == 7
                    && local_receipt_store_feature_gate_enabled_count == 0
                    && append_only_store_path_grant_required_count == 7
                    && append_only_store_path_grant_present_count == 0
                    && atomic_append_required_count == 7
                    && atomic_append_enabled_count == 0
                    && post_append_readback_required_count == 7
                    && post_append_readback_persisted_count == 0
                    && rollback_anchor_required_count == 7
                    && rollback_anchor_verified_count == 0
                    && retention_policy_required_count == 7
                    && retention_policy_committed_count == 0
                    && replay_idempotency_guard_required_count == 7
                    && replay_idempotency_guard_enabled_count == 0
                    && local_store_open_allowed_count == 0
                    && evidence_recorded_count == 0
                    && receipt_store_write_attempt_recorded_count == 0
                    && receipt_store_written_count == 0
                    && receipt_persisted_count == 0
                    && ledger_written_count == 0
                    && workflow_event_log_written_count == 0
                    && sqlite_written_count == 0
                    && live_mutation_allowed_count == 0;

            ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteReport {
                runtime: "hepta",
                surface:
                    "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write",
                status: if local_open_preconditions_readback_ready {
                    "ready_blocked"
                } else {
                    "blocked"
                },
                gate:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_GATE,
                schema_version:
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_SCHEMA_VERSION,
                plugin_id: "hepta-system@hepta-local",
                source_terminal_no_persistence_readback_ready:
                    source.terminal_no_persistence_readback_ready,
                source_terminal_entry_count: source.terminal_entry_count,
                source_terminal_no_persistence_confirmed_count:
                    source.terminal_no_persistence_confirmed_count,
                source_terminal_closeout_recorded_count: source.terminal_closeout_recorded_count,
                source_terminal_closeout_persisted_count: source.terminal_closeout_persisted_count,
                source_terminal_closeout_accepted_count: source.terminal_closeout_accepted_count,
                source_terminal_closeout_authoritative_count:
                    source.terminal_closeout_authoritative_count,
                source_denial_receipt_persisted_count: source.denial_receipt_persisted_count,
                source_write_attempt_recorded_count: source.write_attempt_recorded_count,
                source_receipt_store_written_count: source.receipt_store_written_count,
                source_receipt_persisted_count: source.receipt_persisted_count,
                source_live_execution_allowed: source.live_execution_allowed,
                open_precondition_entry_count: entries.len(),
                open_precondition_catalog_ready_count,
                source_terminal_closeout_attached_count,
                operator_local_store_approval_required_count,
                operator_local_store_approval_present_count,
                dev_evidence_acceptance_source_required_count,
                dev_evidence_acceptance_source_present_count,
                evidence_acceptance_required_count,
                evidence_acceptance_present_count,
                local_receipt_store_feature_gate_required_count,
                local_receipt_store_feature_gate_enabled_count,
                append_only_store_path_grant_required_count,
                append_only_store_path_grant_present_count,
                atomic_append_required_count,
                atomic_append_enabled_count,
                post_append_readback_required_count,
                post_append_readback_persisted_count,
                rollback_anchor_required_count,
                rollback_anchor_verified_count,
                retention_policy_required_count,
                retention_policy_committed_count,
                replay_idempotency_guard_required_count,
                replay_idempotency_guard_enabled_count,
                local_store_open_allowed_count,
                evidence_recorded_count,
                receipt_store_write_attempt_recorded_count,
                receipt_store_written_count,
                receipt_persisted_count,
                ledger_written_count,
                workflow_event_log_written_count,
                sqlite_written_count,
                live_mutation_allowed_count,
                local_open_preconditions_readback_ready,
                local_evidence_receipt_store_open_allowed: false,
                operator_approval_request_allowed: false,
                evidence_acceptance_recording_allowed: false,
                receipt_store_feature_gate_open_allowed: false,
                append_only_store_write_allowed: false,
                receipt_store_write_attempt_recording_allowed: false,
                receipt_store_write_allowed: false,
                receipt_persistence_allowed: false,
                ledger_write_allowed: false,
                workflow_event_log_write_allowed: false,
                sqlite_write_allowed: false,
                credential_read_allowed: false,
                live_execution_allowed: false,
                blockers: vec![
                    "operator_local_store_approval_missing",
                    "dev_evidence_acceptance_source_missing",
                    "evidence_acceptance_missing",
                    "local_receipt_store_feature_gate_closed",
                    "append_only_store_path_grant_missing",
                    "atomic_append_not_enabled",
                    "post_append_readback_missing",
                    "rollback_anchor_missing",
                    "retention_policy_not_committed",
                    "replay_idempotency_guard_disabled",
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
                    CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_RECOMMENDED_NEXT_GATE,
                side_effects:
                    ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteSideEffects::none(),
            }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_entries()
-> Vec<
    ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteEntry,
>{
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteEntry {
                id: format!(
                    "evidence_receipt_store_local_evidence_receipt_store_open_preconditions_without_write_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_terminal_closeout_id: entry.terminal_closeout_id,
                source_terminal_closeout_key: entry.terminal_closeout_key,
                source_terminal_closeout_route: entry.terminal_closeout_route,
                source_terminal_reason: entry.terminal_reason,
                source_persistence_denial_id: entry.source_persistence_denial_id,
                local_open_precondition_set_id: format!(
                    "local-evidence-receipt-store-open-preconditions:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                local_open_precondition_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-open-preconditions/{hyphenated}"
                ),
                operator_local_store_approval_id: format!(
                    "operator-local-store-approval:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                dev_evidence_acceptance_source_id: format!(
                    "dev-evidence-acceptance-source:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                evidence_acceptance_key: format!(
                    "controlled_live.local_evidence_acceptance.required.{}",
                    entry.source_blocker_id
                ),
                local_receipt_store_feature_gate: format!(
                    "feature-gate:controlled-live-local-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                append_only_store_path_grant_key: format!(
                    "append-only-store-path-grant:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                atomic_append_plan_id: format!(
                    "atomic-append-plan:controlled-live-local-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_append_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-open-preconditions/post-append/{hyphenated}"
                ),
                rollback_anchor_route: format!(
                    "readback://controlled-live/evidence-receipt-store/local-open-preconditions/rollback-anchor/{hyphenated}"
                ),
                retention_policy_id: format!(
                    "retention-policy:controlled-live-local-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                replay_idempotency_guard_key: format!(
                    "replay-idempotency-guard:controlled-live-local-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state:
                    "local_evidence_receipt_store_open_preconditions_listed_without_write",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                source_terminal_closeout_attached: entry.terminal_closeout_projected,
                terminal_no_persistence_confirmed: entry.terminal_no_persistence_confirmed,
                operator_local_store_approval_required: true,
                operator_local_store_approval_present: false,
                dev_evidence_acceptance_source_required: true,
                dev_evidence_acceptance_source_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                local_receipt_store_feature_gate_required: true,
                local_receipt_store_feature_gate_enabled: false,
                append_only_store_path_grant_required: true,
                append_only_store_path_grant_present: false,
                atomic_append_required: true,
                atomic_append_enabled: false,
                post_append_readback_required: true,
                post_append_readback_persisted: false,
                rollback_anchor_required: true,
                rollback_anchor_verified: false,
                retention_policy_required: true,
                retention_policy_committed: false,
                replay_idempotency_guard_required: true,
                replay_idempotency_guard_enabled: false,
                local_store_open_allowed: false,
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

impl ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteSideEffects {
    pub const fn none() -> Self {
        Self {
            operator_approval_requested: false,
            evidence_acceptance_recorded: false,
            local_receipt_store_feature_gate_opened: false,
            append_only_store_path_granted: false,
            atomic_append_enabled: false,
            post_append_readback_persisted: false,
            rollback_anchor_verified: false,
            retention_policy_committed: false,
            replay_idempotency_guard_enabled: false,
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
    fn local_open_preconditions_projects_all_entries() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.local_open_preconditions_readback_ready);
        assert_eq!(report.source_terminal_entry_count, 7);
        assert_eq!(report.open_precondition_entry_count, 7);
        assert_eq!(report.open_precondition_catalog_ready_count, 7);
        assert_eq!(report.source_terminal_closeout_attached_count, 7);
        assert_eq!(report.operator_local_store_approval_required_count, 7);
        assert_eq!(report.dev_evidence_acceptance_source_required_count, 7);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.local_receipt_store_feature_gate_required_count, 7);
        assert_eq!(report.append_only_store_path_grant_required_count, 7);
        assert_eq!(report.atomic_append_required_count, 7);
        assert_eq!(report.post_append_readback_required_count, 7);
        assert_eq!(report.rollback_anchor_required_count, 7);
        assert_eq!(report.retention_policy_required_count, 7);
        assert_eq!(report.replay_idempotency_guard_required_count, 7);
    }

    #[test]
    fn local_open_preconditions_keep_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_report();

        assert_eq!(report.operator_local_store_approval_present_count, 0);
        assert_eq!(report.dev_evidence_acceptance_source_present_count, 0);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.local_receipt_store_feature_gate_enabled_count, 0);
        assert_eq!(report.append_only_store_path_grant_present_count, 0);
        assert_eq!(report.atomic_append_enabled_count, 0);
        assert_eq!(report.post_append_readback_persisted_count, 0);
        assert_eq!(report.rollback_anchor_verified_count, 0);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert_eq!(report.replay_idempotency_guard_enabled_count, 0);
        assert_eq!(report.local_store_open_allowed_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_write_attempt_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.workflow_event_log_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.local_evidence_receipt_store_open_allowed);
        assert!(!report.operator_approval_request_allowed);
        assert!(!report.evidence_acceptance_recording_allowed);
        assert!(!report.receipt_store_feature_gate_open_allowed);
        assert!(!report.append_only_store_write_allowed);
        assert!(!report.receipt_store_write_attempt_recording_allowed);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.ledger_write_allowed);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreLocalEvidenceReceiptStoreOpenPreconditionsReadbackWithoutWriteSideEffects::none()
        );
    }

    #[test]
    fn local_open_precondition_entries_are_stable_and_missing() {
        let entries =
            controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_entries();

        assert!(entries.iter().any(|entry| {
            entry.source_blocker_id == "dirty_worktree_boundary"
                && entry.local_open_precondition_route
                    == "readback://controlled-live/evidence-receipt-store/local-open-preconditions/dirty-worktree-boundary"
                && entry.operator_local_store_approval_id
                    == "operator-local-store-approval:controlled-live-evidence-receipt-store:dirty_worktree_boundary"
        }));
        assert!(entries.iter().all(|entry| {
            entry.observed_state
                == "local_evidence_receipt_store_open_preconditions_listed_without_write"
                && entry.source_terminal_closeout_attached
                && entry.terminal_no_persistence_confirmed
                && entry.operator_local_store_approval_required
                && !entry.operator_local_store_approval_present
                && entry.dev_evidence_acceptance_source_required
                && !entry.dev_evidence_acceptance_source_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.local_receipt_store_feature_gate_required
                && !entry.local_receipt_store_feature_gate_enabled
                && entry.append_only_store_path_grant_required
                && !entry.append_only_store_path_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_append_readback_required
                && !entry.post_append_readback_persisted
                && entry.rollback_anchor_required
                && !entry.rollback_anchor_verified
                && entry.retention_policy_required
                && !entry.retention_policy_committed
                && entry.replay_idempotency_guard_required
                && !entry.replay_idempotency_guard_enabled
                && !entry.local_store_open_allowed
                && !entry.evidence_recorded
                && !entry.receipt_store_write_attempt_recorded
                && !entry.receipt_store_written
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        }));
    }
}
