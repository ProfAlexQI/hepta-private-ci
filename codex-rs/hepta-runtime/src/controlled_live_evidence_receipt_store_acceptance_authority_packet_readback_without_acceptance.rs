use crate::controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance::controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_GATE:
    &str = "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback";

const ACCEPTANCE_AUTHORITY_PACKET_ID: &str =
    "controlled-live-evidence-receipt-store-acceptance-authority-packet";
const ACCEPTANCE_AUTHORITY_PACKET_ROUTE: &str =
    "operator-packet://controlled-live/evidence-receipt-store/acceptance-authority";
const ACCEPTANCE_AUTHORITY_PACKET_PAYLOAD_FINGERPRINT: &str =
    "sha256:controlled-live-evidence-receipt-store-acceptance-authority-packet-no-acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_positive_preconditions_ready: bool,
    pub source_precondition_entry_count: usize,
    pub source_acceptance_allowed_count: usize,
    pub source_operator_acceptance_present_count: usize,
    pub source_evidence_acceptance_present_count: usize,
    pub source_receipt_store_written_count: usize,
    pub source_live_execution_allowed: bool,
    pub acceptance_authority_packet_id: &'static str,
    pub acceptance_authority_packet_route: &'static str,
    pub acceptance_authority_packet_payload_fingerprint: &'static str,
    pub packet_entry_count: usize,
    pub packet_projected_count: usize,
    pub packet_ready_count: usize,
    pub authority_checklist_projected_count: usize,
    pub authority_item_required_count: usize,
    pub authority_item_present_count: usize,
    pub acceptance_authority_required_count: usize,
    pub acceptance_authority_present_count: usize,
    pub authority_decision_request_projected_count: usize,
    pub authority_decision_recorded_count: usize,
    pub non_authority_receipt_projected_count: usize,
    pub non_authority_receipt_persisted_count: usize,
    pub operator_packet_sent_count: usize,
    pub operator_packet_persisted_count: usize,
    pub acceptance_allowed_count: usize,
    pub evidence_recorded_count: usize,
    pub receipt_store_written_count: usize,
    pub receipt_persisted_count: usize,
    pub ledger_written_count: usize,
    pub workflow_event_log_written_count: usize,
    pub sqlite_written_count: usize,
    pub live_mutation_allowed_count: usize,
    pub acceptance_authority_packet_readback_ready: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub acceptance_authority_allowed: bool,
    pub acceptance_recording_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries:
        Vec<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceEntry
{
    pub id: String,
    pub source_blocker_id: &'static str,
    pub source_positive_precondition_set_id: String,
    pub source_positive_precondition_route: String,
    pub acceptance_authority_packet_id: &'static str,
    pub acceptance_authority_packet_route: &'static str,
    pub authority_decision_request_id: String,
    pub authority_decision_request_route: String,
    pub non_authority_receipt_id: String,
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
    pub packet_projected: bool,
    pub packet_ready: bool,
    pub authority_checklist_projected: bool,
    pub authority_item_required_count: usize,
    pub authority_item_present_count: usize,
    pub acceptance_authority_required: bool,
    pub acceptance_authority_present: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub receipt_persistence_grant_required: bool,
    pub receipt_persistence_grant_present: bool,
    pub atomic_append_required: bool,
    pub atomic_append_enabled: bool,
    pub post_write_readback_required: bool,
    pub post_write_readback_persisted: bool,
    pub rollback_rehearsal_required: bool,
    pub rollback_rehearsal_verified: bool,
    pub retention_policy_commit_required: bool,
    pub retention_policy_committed: bool,
    pub live_cutover_approval_required: bool,
    pub live_cutover_approval_present: bool,
    pub authority_decision_request_projected: bool,
    pub authority_decision_recorded: bool,
    pub non_authority_receipt_projected: bool,
    pub non_authority_receipt_persisted: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persistence_allowed: bool,
    pub operator_packet_persisted: bool,
    pub acceptance_allowed: bool,
    pub acceptance_recording_allowed: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceSideEffects
{
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub acceptance_authority_accepted: bool,
    pub acceptance_recorded: bool,
    pub evidence_recorded: bool,
    pub receipt_persisted: bool,
    pub receipt_store_written: bool,
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

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_report()
-> ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceReport,
    > = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report();
    let entries =
        controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_entries();

    let packet_projected_count = entries
        .iter()
        .filter(|entry| entry.packet_projected)
        .count();
    let packet_ready_count = entries.iter().filter(|entry| entry.packet_ready).count();
    let authority_checklist_projected_count = entries
        .iter()
        .filter(|entry| entry.authority_checklist_projected)
        .count();
    let authority_item_required_count = entries
        .iter()
        .map(|entry| entry.authority_item_required_count)
        .sum();
    let authority_item_present_count = entries
        .iter()
        .map(|entry| entry.authority_item_present_count)
        .sum();
    let acceptance_authority_required_count = entries
        .iter()
        .filter(|entry| entry.acceptance_authority_required)
        .count();
    let acceptance_authority_present_count = entries
        .iter()
        .filter(|entry| entry.acceptance_authority_present)
        .count();
    let authority_decision_request_projected_count = entries
        .iter()
        .filter(|entry| entry.authority_decision_request_projected)
        .count();
    let authority_decision_recorded_count = entries
        .iter()
        .filter(|entry| entry.authority_decision_recorded)
        .count();
    let non_authority_receipt_projected_count = entries
        .iter()
        .filter(|entry| entry.non_authority_receipt_projected)
        .count();
    let non_authority_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.non_authority_receipt_persisted)
        .count();
    let operator_packet_sent_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_sent)
        .count();
    let operator_packet_persisted_count = entries
        .iter()
        .filter(|entry| entry.operator_packet_persisted)
        .count();
    let acceptance_allowed_count = entries
        .iter()
        .filter(|entry| entry.acceptance_allowed)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let receipt_store_written_count = entries
        .iter()
        .filter(|entry| entry.receipt_store_written)
        .count();
    let receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.receipt_persisted || entry.non_authority_receipt_persisted)
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

    let acceptance_authority_packet_readback_ready = source
        .positive_acceptance_preconditions_readback_ready
        && source.precondition_entry_count == 7
        && source.acceptance_allowed_count == 0
        && source.operator_acceptance_present_count == 0
        && source.evidence_acceptance_present_count == 0
        && source.receipt_store_written_count == 0
        && !source.live_execution_allowed
        && entries.len() == 7
        && packet_projected_count == 7
        && packet_ready_count == 7
        && authority_checklist_projected_count == 7
        && authority_item_required_count == 56
        && authority_item_present_count == 0
        && acceptance_authority_required_count == 7
        && acceptance_authority_present_count == 0
        && authority_decision_request_projected_count == 7
        && authority_decision_recorded_count == 0
        && non_authority_receipt_projected_count == 7
        && non_authority_receipt_persisted_count == 0
        && operator_packet_sent_count == 0
        && operator_packet_persisted_count == 0
        && acceptance_allowed_count == 0
        && evidence_recorded_count == 0
        && receipt_store_written_count == 0
        && receipt_persisted_count == 0
        && ledger_written_count == 0
        && workflow_event_log_written_count == 0
        && sqlite_written_count == 0
        && live_mutation_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.observed_state == "acceptance_authority_packet_projected_without_acceptance"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.packet_projected
                && entry.packet_ready
                && entry.authority_checklist_projected
                && entry.authority_item_required_count == 8
                && entry.authority_item_present_count == 0
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && entry.operator_acceptance_required
                && !entry.operator_acceptance_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.receipt_persistence_grant_required
                && !entry.receipt_persistence_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_write_readback_required
                && !entry.post_write_readback_persisted
                && entry.rollback_rehearsal_required
                && !entry.rollback_rehearsal_verified
                && entry.retention_policy_commit_required
                && !entry.retention_policy_committed
                && entry.live_cutover_approval_required
                && !entry.live_cutover_approval_present
                && entry.authority_decision_request_projected
                && !entry.authority_decision_recorded
                && entry.non_authority_receipt_projected
                && !entry.non_authority_receipt_persisted
                && !entry.operator_packet_send_allowed
                && !entry.operator_packet_sent
                && !entry.operator_packet_persistence_allowed
                && !entry.operator_packet_persisted
                && !entry.acceptance_allowed
                && !entry.acceptance_recording_allowed
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

    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceReport {
        runtime: "hepta",
        surface:
            "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance",
        status: if acceptance_authority_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_positive_preconditions_ready: source.positive_acceptance_preconditions_readback_ready,
        source_precondition_entry_count: source.precondition_entry_count,
        source_acceptance_allowed_count: source.acceptance_allowed_count,
        source_operator_acceptance_present_count: source.operator_acceptance_present_count,
        source_evidence_acceptance_present_count: source.evidence_acceptance_present_count,
        source_receipt_store_written_count: source.receipt_store_written_count,
        source_live_execution_allowed: source.live_execution_allowed,
        acceptance_authority_packet_id: ACCEPTANCE_AUTHORITY_PACKET_ID,
        acceptance_authority_packet_route: ACCEPTANCE_AUTHORITY_PACKET_ROUTE,
        acceptance_authority_packet_payload_fingerprint:
            ACCEPTANCE_AUTHORITY_PACKET_PAYLOAD_FINGERPRINT,
        packet_entry_count: entries.len(),
        packet_projected_count,
        packet_ready_count,
        authority_checklist_projected_count,
        authority_item_required_count,
        authority_item_present_count,
        acceptance_authority_required_count,
        acceptance_authority_present_count,
        authority_decision_request_projected_count,
        authority_decision_recorded_count,
        non_authority_receipt_projected_count,
        non_authority_receipt_persisted_count,
        operator_packet_sent_count,
        operator_packet_persisted_count,
        acceptance_allowed_count,
        evidence_recorded_count,
        receipt_store_written_count,
        receipt_persisted_count,
        ledger_written_count,
        workflow_event_log_written_count,
        sqlite_written_count,
        live_mutation_allowed_count,
        acceptance_authority_packet_readback_ready,
        operator_packet_send_allowed: false,
        operator_packet_sent: false,
        operator_packet_persistence_allowed: false,
        operator_packet_persisted: false,
        acceptance_authority_allowed: false,
        acceptance_recording_allowed: false,
        evidence_recording_allowed: false,
        receipt_persistence_allowed: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_packet_send_disabled",
            "operator_packet_persistence_disabled",
            "acceptance_authority_missing",
            "operator_acceptance_missing",
            "evidence_acceptance_missing",
            "receipt_persistence_grant_missing",
            "atomic_append_not_enabled",
            "post_write_readback_missing",
            "rollback_rehearsal_missing",
            "retention_policy_not_committed",
            "live_cutover_approval_missing",
            "receipt_store_write_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceEntry> {
    controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceEntry {
                id: format!(
                    "evidence_receipt_store_acceptance_authority_packet_without_acceptance_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                source_positive_precondition_set_id: entry.positive_precondition_set_id,
                source_positive_precondition_route: entry.positive_precondition_route,
                acceptance_authority_packet_id: ACCEPTANCE_AUTHORITY_PACKET_ID,
                acceptance_authority_packet_route: ACCEPTANCE_AUTHORITY_PACKET_ROUTE,
                authority_decision_request_id: format!(
                    "acceptance-authority-decision-request:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                authority_decision_request_route: format!(
                    "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/decision-request/{hyphenated}"
                ),
                non_authority_receipt_id: format!(
                    "non-authority-receipt:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "acceptance_authority_packet_projected_without_acceptance",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                packet_projected: true,
                packet_ready: true,
                authority_checklist_projected: true,
                authority_item_required_count: 8,
                authority_item_present_count: 0,
                acceptance_authority_required: true,
                acceptance_authority_present: false,
                operator_acceptance_required: entry.operator_acceptance_required,
                operator_acceptance_present: entry.operator_acceptance_present,
                evidence_acceptance_required: entry.evidence_acceptance_required,
                evidence_acceptance_present: entry.evidence_acceptance_present,
                receipt_persistence_grant_required: entry.receipt_persistence_grant_required,
                receipt_persistence_grant_present: entry.receipt_persistence_grant_present,
                atomic_append_required: entry.atomic_append_required,
                atomic_append_enabled: entry.atomic_append_enabled,
                post_write_readback_required: entry.post_write_readback_required,
                post_write_readback_persisted: entry.post_write_readback_persisted,
                rollback_rehearsal_required: entry.rollback_rehearsal_required,
                rollback_rehearsal_verified: entry.rollback_rehearsal_verified,
                retention_policy_commit_required: entry.retention_policy_commit_required,
                retention_policy_committed: entry.retention_policy_committed,
                live_cutover_approval_required: entry.live_cutover_approval_required,
                live_cutover_approval_present: entry.live_cutover_approval_present,
                authority_decision_request_projected: true,
                authority_decision_recorded: false,
                non_authority_receipt_projected: true,
                non_authority_receipt_persisted: false,
                operator_packet_send_allowed: false,
                operator_packet_sent: false,
                operator_packet_persistence_allowed: false,
                operator_packet_persisted: false,
                acceptance_allowed: false,
                acceptance_recording_allowed: false,
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

impl
    ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceSideEffects
{
    pub const fn none() -> Self {
        Self {
            operator_packet_sent: false,
            operator_packet_persisted: false,
            acceptance_authority_accepted: false,
            acceptance_recorded: false,
            evidence_recorded: false,
            receipt_persisted: false,
            receipt_store_written: false,
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
    fn acceptance_authority_packet_projects_all_entries_without_acceptance() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_positive_preconditions_ready);
        assert_eq!(report.source_precondition_entry_count, 7);
        assert_eq!(report.source_acceptance_allowed_count, 0);
        assert_eq!(report.packet_entry_count, 7);
        assert_eq!(report.packet_projected_count, 7);
        assert_eq!(report.packet_ready_count, 7);
        assert_eq!(report.authority_checklist_projected_count, 7);
        assert_eq!(report.authority_item_required_count, 56);
        assert_eq!(report.authority_item_present_count, 0);
        assert_eq!(report.acceptance_authority_required_count, 7);
        assert_eq!(report.acceptance_authority_present_count, 0);
        assert_eq!(report.authority_decision_request_projected_count, 7);
        assert!(report.acceptance_authority_packet_readback_ready);
    }

    #[test]
    fn acceptance_authority_packet_keeps_send_acceptance_and_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_report();

        assert_eq!(report.authority_decision_recorded_count, 0);
        assert_eq!(report.non_authority_receipt_projected_count, 7);
        assert_eq!(report.non_authority_receipt_persisted_count, 0);
        assert_eq!(report.operator_packet_sent_count, 0);
        assert_eq!(report.operator_packet_persisted_count, 0);
        assert_eq!(report.acceptance_allowed_count, 0);
        assert_eq!(report.evidence_recorded_count, 0);
        assert_eq!(report.receipt_store_written_count, 0);
        assert_eq!(report.receipt_persisted_count, 0);
        assert_eq!(report.ledger_written_count, 0);
        assert_eq!(report.sqlite_written_count, 0);
        assert_eq!(report.live_mutation_allowed_count, 0);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.acceptance_authority_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStoreAcceptanceAuthorityPacketReadbackWithoutAcceptanceSideEffects::none()
        );
    }

    #[test]
    fn acceptance_authority_packet_entries_are_stable_and_missing() {
        let report =
            controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.authority_decision_request_route
                == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/decision-request/dirty-worktree-boundary"
            && entry.non_authority_receipt_id
                == "non-authority-receipt:controlled-live-evidence-receipt-store:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.acceptance_authority_packet_id == ACCEPTANCE_AUTHORITY_PACKET_ID
                && entry.acceptance_authority_packet_route == ACCEPTANCE_AUTHORITY_PACKET_ROUTE
                && entry.packet_projected
                && entry.packet_ready
                && entry.authority_checklist_projected
                && entry.authority_item_required_count == 8
                && entry.authority_item_present_count == 0
                && entry.acceptance_authority_required
                && !entry.acceptance_authority_present
                && entry.authority_decision_request_projected
                && !entry.authority_decision_recorded
                && entry.non_authority_receipt_projected
                && !entry.non_authority_receipt_persisted
                && !entry.operator_packet_sent
                && !entry.operator_packet_persisted
                && !entry.acceptance_allowed
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
