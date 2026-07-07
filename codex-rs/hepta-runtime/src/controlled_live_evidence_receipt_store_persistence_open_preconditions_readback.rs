use crate::controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback::controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_GATE:
    &str = "controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_shadow_write_rehearsal_ready: bool,
    pub source_shadow_write_rehearsal_entry_count: usize,
    pub source_in_memory_shadow_receipt_rendered_count: usize,
    pub source_receipt_store_written: bool,
    pub source_receipt_persisted: bool,
    pub precondition_entry_count: usize,
    pub precondition_catalog_ready_count: usize,
    pub operator_approval_required_count: usize,
    pub operator_approval_present_count: usize,
    pub evidence_acceptance_required_count: usize,
    pub evidence_acceptance_present_count: usize,
    pub store_path_write_grant_required_count: usize,
    pub store_path_write_grant_present_count: usize,
    pub atomic_append_required_count: usize,
    pub atomic_append_enabled_count: usize,
    pub post_write_readback_required_count: usize,
    pub post_write_readback_persisted_count: usize,
    pub rollback_rehearsal_required_count: usize,
    pub rollback_rehearsal_verified_count: usize,
    pub retention_policy_required_count: usize,
    pub retention_policy_committed_count: usize,
    pub persistence_denial_confirmed_count: usize,
    pub ledger_denial_confirmed_count: usize,
    pub workflow_event_log_denial_confirmed_count: usize,
    pub sqlite_denial_confirmed_count: usize,
    pub live_denial_confirmed_count: usize,
    pub evidence_recorded_count: usize,
    pub blocker_waived_count: usize,
    pub persistence_open_preconditions_readback_ready: bool,
    pub persistence_open_allowed: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_persisted: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackEntry {
    pub id: String,
    pub source_blocker_id: &'static str,
    pub receipt_path: &'static str,
    pub receipt_id: &'static str,
    pub idempotency_key: &'static str,
    pub shadow_write_route: String,
    pub persistence_precondition_key: String,
    pub persistence_precondition_route: String,
    pub operator_approval_id: String,
    pub evidence_acceptance_key: String,
    pub store_path_write_grant_key: String,
    pub atomic_append_plan_id: String,
    pub post_write_readback_route: String,
    pub rollback_rehearsal_route: String,
    pub retention_policy_id: String,
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
    pub precondition_state: &'static str,
    pub shadow_rehearsal_confirmed: bool,
    pub operator_approval_required: bool,
    pub operator_approval_present: bool,
    pub evidence_acceptance_required: bool,
    pub evidence_acceptance_present: bool,
    pub store_path_write_grant_required: bool,
    pub store_path_write_grant_present: bool,
    pub atomic_append_required: bool,
    pub atomic_append_enabled: bool,
    pub post_write_readback_required: bool,
    pub post_write_readback_persisted: bool,
    pub rollback_rehearsal_required: bool,
    pub rollback_rehearsal_verified: bool,
    pub retention_policy_required: bool,
    pub retention_policy_committed: bool,
    pub persistence_denied: bool,
    pub ledger_denied: bool,
    pub workflow_event_log_denied: bool,
    pub sqlite_denied: bool,
    pub live_denied: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_recorded: bool,
    pub blocker_waiver_allowed: bool,
    pub receipt_persistence_allowed: bool,
    pub receipt_persisted: bool,
    pub receipt_store_write_allowed: bool,
    pub receipt_store_written: bool,
    pub ledger_write_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub credential_read_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub receipt_persisted: bool,
    pub receipt_store_written: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub packet_sent: bool,
    pub attachment_sent: bool,
    pub packet_persisted: bool,
    pub attachment_persisted: bool,
    pub readback_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
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

pub fn controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report()
-> ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackReport {
    static REPORT: OnceLock<
        ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackReport,
    > = OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source = controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report();
    let entries =
        controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_entries();

    let precondition_catalog_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.shadow_rehearsal_confirmed
                && entry.operator_approval_required
                && entry.evidence_acceptance_required
                && entry.store_path_write_grant_required
                && entry.atomic_append_required
                && entry.post_write_readback_required
                && entry.rollback_rehearsal_required
                && entry.retention_policy_required
                && entry.persistence_denied
                && entry.ledger_denied
                && entry.workflow_event_log_denied
                && entry.sqlite_denied
                && entry.live_denied
                && !entry.operator_approval_present
                && !entry.evidence_acceptance_present
                && !entry.store_path_write_grant_present
                && !entry.atomic_append_enabled
                && !entry.post_write_readback_persisted
                && !entry.rollback_rehearsal_verified
                && !entry.retention_policy_committed
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        })
        .count();
    let operator_approval_required_count = entries
        .iter()
        .filter(|entry| entry.operator_approval_required)
        .count();
    let operator_approval_present_count = entries
        .iter()
        .filter(|entry| entry.operator_approval_present)
        .count();
    let evidence_acceptance_required_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_required)
        .count();
    let evidence_acceptance_present_count = entries
        .iter()
        .filter(|entry| entry.evidence_acceptance_present)
        .count();
    let store_path_write_grant_required_count = entries
        .iter()
        .filter(|entry| entry.store_path_write_grant_required)
        .count();
    let store_path_write_grant_present_count = entries
        .iter()
        .filter(|entry| entry.store_path_write_grant_present)
        .count();
    let atomic_append_required_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_required)
        .count();
    let atomic_append_enabled_count = entries
        .iter()
        .filter(|entry| entry.atomic_append_enabled)
        .count();
    let post_write_readback_required_count = entries
        .iter()
        .filter(|entry| entry.post_write_readback_required)
        .count();
    let post_write_readback_persisted_count = entries
        .iter()
        .filter(|entry| entry.post_write_readback_persisted)
        .count();
    let rollback_rehearsal_required_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_required)
        .count();
    let rollback_rehearsal_verified_count = entries
        .iter()
        .filter(|entry| entry.rollback_rehearsal_verified)
        .count();
    let retention_policy_required_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_required)
        .count();
    let retention_policy_committed_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_committed)
        .count();
    let persistence_denial_confirmed_count = entries
        .iter()
        .filter(|entry| entry.persistence_denied)
        .count();
    let ledger_denial_confirmed_count = entries.iter().filter(|entry| entry.ledger_denied).count();
    let workflow_event_log_denial_confirmed_count = entries
        .iter()
        .filter(|entry| entry.workflow_event_log_denied)
        .count();
    let sqlite_denial_confirmed_count = entries.iter().filter(|entry| entry.sqlite_denied).count();
    let live_denial_confirmed_count = entries.iter().filter(|entry| entry.live_denied).count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();

    let persistence_open_preconditions_readback_ready = source
        .in_memory_shadow_write_rehearsal_ready
        && source.shadow_write_rehearsal_entry_count == 7
        && source.shadow_write_rehearsal_ready_count == 7
        && source.in_memory_shadow_receipt_rendered_count == 7
        && !source.receipt_store_write_allowed
        && !source.receipt_store_written
        && !source.receipt_persistence_allowed
        && !source.receipt_persisted
        && !source.ledger_write_allowed
        && !source.workflow_event_log_write_allowed
        && !source.sqlite_write_allowed
        && !source.live_execution_allowed
        && entries.len() == 7
        && precondition_catalog_ready_count == 7
        && operator_approval_required_count == 7
        && operator_approval_present_count == 0
        && evidence_acceptance_required_count == 7
        && evidence_acceptance_present_count == 0
        && store_path_write_grant_required_count == 7
        && store_path_write_grant_present_count == 0
        && atomic_append_required_count == 7
        && atomic_append_enabled_count == 0
        && post_write_readback_required_count == 7
        && post_write_readback_persisted_count == 0
        && rollback_rehearsal_required_count == 7
        && rollback_rehearsal_verified_count == 0
        && retention_policy_required_count == 7
        && retention_policy_committed_count == 0
        && persistence_denial_confirmed_count == 7
        && ledger_denial_confirmed_count == 7
        && workflow_event_log_denial_confirmed_count == 7
        && sqlite_denial_confirmed_count == 7
        && live_denial_confirmed_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0;

    ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_evidence_receipt_store_persistence_open_preconditions_readback",
        status: if persistence_open_preconditions_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_shadow_write_rehearsal_ready: source.in_memory_shadow_write_rehearsal_ready,
        source_shadow_write_rehearsal_entry_count: source.shadow_write_rehearsal_entry_count,
        source_in_memory_shadow_receipt_rendered_count:
            source.in_memory_shadow_receipt_rendered_count,
        source_receipt_store_written: source.receipt_store_written,
        source_receipt_persisted: source.receipt_persisted,
        precondition_entry_count: entries.len(),
        precondition_catalog_ready_count,
        operator_approval_required_count,
        operator_approval_present_count,
        evidence_acceptance_required_count,
        evidence_acceptance_present_count,
        store_path_write_grant_required_count,
        store_path_write_grant_present_count,
        atomic_append_required_count,
        atomic_append_enabled_count,
        post_write_readback_required_count,
        post_write_readback_persisted_count,
        rollback_rehearsal_required_count,
        rollback_rehearsal_verified_count,
        retention_policy_required_count,
        retention_policy_committed_count,
        persistence_denial_confirmed_count,
        ledger_denial_confirmed_count,
        workflow_event_log_denial_confirmed_count,
        sqlite_denial_confirmed_count,
        live_denial_confirmed_count,
        evidence_recorded_count,
        blocker_waived_count,
        persistence_open_preconditions_readback_ready,
        persistence_open_allowed: false,
        approval_request_allowed: false,
        approval_acceptance_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        receipt_persistence_allowed: false,
        receipt_persisted: false,
        receipt_store_write_allowed: false,
        receipt_store_written: false,
        ledger_write_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        credential_read_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "operator_approval_missing",
            "evidence_acceptance_missing",
            "store_path_write_grant_missing",
            "atomic_append_not_enabled",
            "post_write_readback_missing",
            "rollback_rehearsal_missing",
            "retention_policy_not_committed",
            "receipt_persistence_disabled",
            "ledger_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_entries()
-> Vec<ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackEntry> {
    controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report()
        .entries
        .into_iter()
        .map(|entry| {
            let hyphenated = entry.source_blocker_id.replace('_', "-");
            ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackEntry {
                id: format!(
                    "evidence_receipt_store_persistence_open_preconditions_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                receipt_path: entry.receipt_path,
                receipt_id: entry.receipt_id,
                idempotency_key: entry.idempotency_key,
                shadow_write_route: entry.shadow_write_route,
                persistence_precondition_key: format!(
                    "controlled_live.evidence_receipt_store.persistence_open_preconditions.{}",
                    entry.source_blocker_id
                ),
                persistence_precondition_route: format!(
                    "readback://controlled-live/evidence-receipt-store/persistence-open-preconditions/{hyphenated}"
                ),
                operator_approval_id: format!(
                    "operator-approval:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                evidence_acceptance_key: format!(
                    "controlled_live.evidence_acceptance.required.{}",
                    entry.source_blocker_id
                ),
                store_path_write_grant_key: format!(
                    "controlled_live.receipt_store.write_grant.required.{}",
                    entry.source_blocker_id
                ),
                atomic_append_plan_id: format!(
                    "atomic-append-plan:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                post_write_readback_route: format!(
                    "readback://controlled-live/evidence-receipt-store/post-write/{hyphenated}"
                ),
                rollback_rehearsal_route: format!(
                    "readback://controlled-live/evidence-receipt-store/rollback-rehearsal/{hyphenated}"
                ),
                retention_policy_id: format!(
                    "retention-policy:controlled-live-evidence-receipt-store:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "persistence_open_preconditions_listed_no_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                precondition_state: "required_missing",
                shadow_rehearsal_confirmed: true,
                operator_approval_required: true,
                operator_approval_present: false,
                evidence_acceptance_required: true,
                evidence_acceptance_present: false,
                store_path_write_grant_required: true,
                store_path_write_grant_present: false,
                atomic_append_required: true,
                atomic_append_enabled: false,
                post_write_readback_required: true,
                post_write_readback_persisted: false,
                rollback_rehearsal_required: true,
                rollback_rehearsal_verified: false,
                retention_policy_required: true,
                retention_policy_committed: false,
                persistence_denied: true,
                ledger_denied: true,
                workflow_event_log_denied: true,
                sqlite_denied: true,
                live_denied: true,
                approval_request_allowed: false,
                approval_acceptance_allowed: false,
                evidence_recording_allowed: false,
                evidence_recorded: false,
                blocker_waiver_allowed: false,
                receipt_persistence_allowed: false,
                receipt_persisted: false,
                receipt_store_write_allowed: false,
                receipt_store_written: false,
                ledger_write_allowed: false,
                workflow_event_log_write_allowed: false,
                sqlite_write_allowed: false,
                credential_read_allowed: false,
                live_mutation_allowed: false,
            }
        })
        .collect()
}

impl ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            receipt_persisted: false,
            receipt_store_written: false,
            blocker_waived: false,
            credential_read: false,
            packet_sent: false,
            attachment_sent: false,
            packet_persisted: false,
            attachment_persisted: false,
            readback_persisted: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
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
    fn persistence_open_preconditions_lists_required_missing_conditions() {
        let report =
            controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_shadow_write_rehearsal_ready);
        assert_eq!(report.source_shadow_write_rehearsal_entry_count, 7);
        assert_eq!(report.source_in_memory_shadow_receipt_rendered_count, 7);
        assert_eq!(report.precondition_entry_count, 7);
        assert_eq!(report.precondition_catalog_ready_count, 7);
        assert_eq!(report.operator_approval_required_count, 7);
        assert_eq!(report.operator_approval_present_count, 0);
        assert_eq!(report.evidence_acceptance_required_count, 7);
        assert_eq!(report.evidence_acceptance_present_count, 0);
        assert_eq!(report.store_path_write_grant_required_count, 7);
        assert_eq!(report.store_path_write_grant_present_count, 0);
        assert_eq!(report.atomic_append_required_count, 7);
        assert_eq!(report.atomic_append_enabled_count, 0);
        assert_eq!(report.post_write_readback_required_count, 7);
        assert_eq!(report.post_write_readback_persisted_count, 0);
        assert_eq!(report.rollback_rehearsal_required_count, 7);
        assert_eq!(report.rollback_rehearsal_verified_count, 0);
        assert_eq!(report.retention_policy_required_count, 7);
        assert_eq!(report.retention_policy_committed_count, 0);
        assert!(report.persistence_open_preconditions_readback_ready);
        assert!(!report.persistence_open_allowed);
    }

    #[test]
    fn persistence_open_preconditions_keep_all_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report();

        assert!(!report.approval_request_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.receipt_persistence_allowed);
        assert!(!report.receipt_persisted);
        assert!(!report.receipt_store_write_allowed);
        assert!(!report.receipt_store_written);
        assert!(!report.ledger_write_allowed);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            ControlledLiveEvidenceReceiptStorePersistenceOpenPreconditionsReadbackSideEffects::none(
            )
        );
    }

    #[test]
    fn persistence_open_precondition_entries_are_stable_and_missing() {
        let report =
            controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.persistence_precondition_route
                == "readback://controlled-live/evidence-receipt-store/persistence-open-preconditions/dirty-worktree-boundary"
            && entry.operator_approval_id
                == "operator-approval:controlled-live-evidence-receipt-store:dirty_worktree_boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.shadow_rehearsal_confirmed
                && entry.precondition_state == "required_missing"
                && entry.operator_approval_required
                && !entry.operator_approval_present
                && entry.evidence_acceptance_required
                && !entry.evidence_acceptance_present
                && entry.store_path_write_grant_required
                && !entry.store_path_write_grant_present
                && entry.atomic_append_required
                && !entry.atomic_append_enabled
                && entry.post_write_readback_required
                && !entry.post_write_readback_persisted
                && entry.rollback_rehearsal_required
                && !entry.rollback_rehearsal_verified
                && entry.retention_policy_required
                && !entry.retention_policy_committed
                && entry.persistence_denied
                && entry.ledger_denied
                && entry.workflow_event_log_denied
                && entry.sqlite_denied
                && entry.live_denied
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.live_mutation_allowed
        }));
    }
}
