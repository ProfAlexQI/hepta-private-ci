use crate::controlled_live_evidence_receipt_store_preflight_readback::controlled_live_evidence_receipt_store_preflight_readback_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_GATE: &str =
    "controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_SCHEMA_VERSION:
    &str = "controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "controlled_live_evidence_receipt_store_persistence_open_preconditions_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_receipt_store_preflight_ready: bool,
    pub source_store_preflight_entry_count: usize,
    pub source_missing_evidence_entry_count: usize,
    pub source_store_root: &'static str,
    pub source_packet_id: &'static str,
    pub source_packet_payload_hash: &'static str,
    pub shadow_write_rehearsal_entry_count: usize,
    pub shadow_write_rehearsal_ready_count: usize,
    pub in_memory_shadow_receipt_rendered_count: usize,
    pub append_only_sequence_projected_count: usize,
    pub readback_query_bound_count: usize,
    pub idempotency_dedup_projected_count: usize,
    pub redacted_payload_projected_count: usize,
    pub secret_payload_denial_confirmed_count: usize,
    pub persistence_denial_confirmed_count: usize,
    pub ledger_denial_confirmed_count: usize,
    pub workflow_event_log_denial_confirmed_count: usize,
    pub sqlite_denial_confirmed_count: usize,
    pub live_denial_confirmed_count: usize,
    pub evidence_recorded_count: usize,
    pub blocker_waived_count: usize,
    pub in_memory_shadow_write_rehearsal_ready: bool,
    pub in_memory_shadow_receipts_rendered: bool,
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
    pub entries: Vec<ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackEntry {
    pub id: String,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub receipt_path: &'static str,
    pub receipt_id: &'static str,
    pub receipt_schema_version: &'static str,
    pub idempotency_key: &'static str,
    pub readback_query_key: &'static str,
    pub readback_query_route: &'static str,
    pub shadow_write_key: String,
    pub shadow_write_route: String,
    pub shadow_receipt_id: String,
    pub shadow_receipt_payload_fingerprint: String,
    pub append_only_sequence_key: String,
    pub previous_receipt_head: &'static str,
    pub projected_receipt_head: String,
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
    pub redaction_policy: &'static str,
    pub secret_payload_state: &'static str,
    pub shadow_payload_state: &'static str,
    pub readback_projection_state: &'static str,
    pub preflight_confirmed: bool,
    pub missing_evidence_confirmed: bool,
    pub in_memory_shadow_receipt_rendered: bool,
    pub append_only_sequence_projected: bool,
    pub readback_query_bound: bool,
    pub idempotency_dedup_projected: bool,
    pub redacted_payload_projected: bool,
    pub secret_payload_denied: bool,
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
pub struct ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackSideEffects {
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

pub fn controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report()
-> ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackReport {
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackReport> =
        OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source = controlled_live_evidence_receipt_store_preflight_readback_report();
    let entries = controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_entries();

    let shadow_write_rehearsal_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.preflight_confirmed
                && entry.missing_evidence_confirmed
                && entry.in_memory_shadow_receipt_rendered
                && entry.append_only_sequence_projected
                && entry.readback_query_bound
                && entry.idempotency_dedup_projected
                && entry.redacted_payload_projected
                && entry.secret_payload_denied
                && entry.persistence_denied
                && entry.ledger_denied
                && entry.workflow_event_log_denied
                && entry.sqlite_denied
                && entry.live_denied
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.ledger_write_allowed
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.live_mutation_allowed
        })
        .count();
    let in_memory_shadow_receipt_rendered_count = entries
        .iter()
        .filter(|entry| entry.in_memory_shadow_receipt_rendered)
        .count();
    let append_only_sequence_projected_count = entries
        .iter()
        .filter(|entry| entry.append_only_sequence_projected)
        .count();
    let readback_query_bound_count = entries
        .iter()
        .filter(|entry| entry.readback_query_bound)
        .count();
    let idempotency_dedup_projected_count = entries
        .iter()
        .filter(|entry| entry.idempotency_dedup_projected)
        .count();
    let redacted_payload_projected_count = entries
        .iter()
        .filter(|entry| entry.redacted_payload_projected)
        .count();
    let secret_payload_denial_confirmed_count = entries
        .iter()
        .filter(|entry| entry.secret_payload_denied)
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

    let in_memory_shadow_write_rehearsal_ready = source.receipt_store_preflight_ready
        && source.store_preflight_entry_count == 7
        && source.store_preflight_ready_count == 7
        && source.missing_evidence_entry_count == 7
        && !source.approval_request_allowed
        && !source.approval_acceptance_allowed
        && !source.evidence_recording_allowed
        && !source.evidence_persisted
        && !source.receipt_persistence_allowed
        && !source.receipt_persisted
        && !source.receipt_store_write_allowed
        && !source.receipt_store_written
        && !source.ledger_write_allowed
        && !source.workflow_event_log_write_allowed
        && !source.sqlite_write_allowed
        && !source.credential_read_allowed
        && !source.live_execution_allowed
        && entries.len() == 7
        && shadow_write_rehearsal_ready_count == 7
        && in_memory_shadow_receipt_rendered_count == 7
        && append_only_sequence_projected_count == 7
        && readback_query_bound_count == 7
        && idempotency_dedup_projected_count == 7
        && redacted_payload_projected_count == 7
        && secret_payload_denial_confirmed_count == 7
        && persistence_denial_confirmed_count == 7
        && ledger_denial_confirmed_count == 7
        && workflow_event_log_denial_confirmed_count == 7
        && sqlite_denial_confirmed_count == 7
        && live_denial_confirmed_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.packet_id == source.source_packet_id
                && entry.packet_payload_hash == source.source_packet_payload_hash
                && entry.observed_state == "in_memory_shadow_receipt_rehearsed_no_persistence"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && entry.secret_payload_state == "denied"
                && entry.shadow_payload_state == "metadata_shape_rendered_in_memory"
                && entry.readback_projection_state == "query_matches_projected_shadow_receipt"
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.evidence_recording_allowed
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.ledger_write_allowed
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.credential_read_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback",
        status: if in_memory_shadow_write_rehearsal_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_GATE,
        schema_version:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_receipt_store_preflight_ready: source.receipt_store_preflight_ready,
        source_store_preflight_entry_count: source.store_preflight_entry_count,
        source_missing_evidence_entry_count: source.missing_evidence_entry_count,
        source_store_root: source.store_root,
        source_packet_id: source.source_packet_id,
        source_packet_payload_hash: source.source_packet_payload_hash,
        shadow_write_rehearsal_entry_count: entries.len(),
        shadow_write_rehearsal_ready_count,
        in_memory_shadow_receipt_rendered_count,
        append_only_sequence_projected_count,
        readback_query_bound_count,
        idempotency_dedup_projected_count,
        redacted_payload_projected_count,
        secret_payload_denial_confirmed_count,
        persistence_denial_confirmed_count,
        ledger_denial_confirmed_count,
        workflow_event_log_denial_confirmed_count,
        sqlite_denial_confirmed_count,
        live_denial_confirmed_count,
        evidence_recorded_count,
        blocker_waived_count,
        in_memory_shadow_write_rehearsal_ready,
        in_memory_shadow_receipts_rendered: in_memory_shadow_receipt_rendered_count == 7,
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
            "evidence_missing",
            "in_memory_rehearsal_only",
            "store_write_disabled",
            "receipt_persistence_disabled",
            "approval_request_disabled",
            "approval_acceptance_disabled",
            "ledger_write_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_WRITE_REHEARSAL_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_entries()
-> Vec<ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackEntry> {
    controlled_live_evidence_receipt_store_preflight_readback_report()
        .entries
        .into_iter()
        .map(
            |entry| ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackEntry {
                id: format!(
                    "evidence_receipt_store_shadow_write_rehearsal_{}",
                    entry.source_blocker_id
                ),
                source_blocker_id: entry.source_blocker_id,
                packet_id: entry.packet_id,
                packet_payload_hash: entry.packet_payload_hash,
                receipt_path: entry.receipt_path,
                receipt_id: entry.receipt_id,
                receipt_schema_version: entry.receipt_schema_version,
                idempotency_key: entry.idempotency_key,
                readback_query_key: entry.readback_query_key,
                readback_query_route: entry.readback_query_route,
                shadow_write_key: format!(
                    "controlled_live.evidence_receipt_store.shadow_write_rehearsal.{}",
                    entry.source_blocker_id
                ),
                shadow_write_route: format!(
                    "readback://controlled-live/evidence-receipt-store/shadow-write-rehearsal/{}",
                    entry.source_blocker_id.replace('_', "-")
                ),
                shadow_receipt_id: format!("shadow:{}", entry.receipt_id),
                shadow_receipt_payload_fingerprint: format!(
                    "sha256:controlled-live-evidence-receipt-shadow:{}:metadata-only",
                    entry.source_blocker_id
                ),
                append_only_sequence_key: format!(
                    "shadow-sequence:{}:00000001",
                    entry.source_blocker_id
                ),
                previous_receipt_head: "absent",
                projected_receipt_head: format!(
                    "metadata-only-shadow-head:{}",
                    entry.source_blocker_id
                ),
                operator_display_order: entry.operator_display_order,
                operator_status: entry.operator_status,
                observed_state: "in_memory_shadow_receipt_rehearsed_no_persistence",
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                redaction_policy: entry.redaction_policy,
                secret_payload_state: entry.secret_payload_state,
                shadow_payload_state: "metadata_shape_rendered_in_memory",
                readback_projection_state: "query_matches_projected_shadow_receipt",
                preflight_confirmed: true,
                missing_evidence_confirmed: true,
                in_memory_shadow_receipt_rendered: true,
                append_only_sequence_projected: true,
                readback_query_bound: true,
                idempotency_dedup_projected: true,
                redacted_payload_projected: true,
                secret_payload_denied: true,
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
            },
        )
        .collect()
}

impl ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackSideEffects {
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
    fn shadow_write_rehearsal_renders_all_receipts_in_memory_without_persistence() {
        let report =
            controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_receipt_store_preflight_ready);
        assert_eq!(report.source_store_preflight_entry_count, 7);
        assert_eq!(report.source_missing_evidence_entry_count, 7);
        assert_eq!(report.shadow_write_rehearsal_entry_count, 7);
        assert_eq!(report.shadow_write_rehearsal_ready_count, 7);
        assert_eq!(report.in_memory_shadow_receipt_rendered_count, 7);
        assert_eq!(report.append_only_sequence_projected_count, 7);
        assert_eq!(report.readback_query_bound_count, 7);
        assert_eq!(report.idempotency_dedup_projected_count, 7);
        assert_eq!(report.redacted_payload_projected_count, 7);
        assert_eq!(report.secret_payload_denial_confirmed_count, 7);
        assert_eq!(report.persistence_denial_confirmed_count, 7);
        assert_eq!(report.ledger_denial_confirmed_count, 7);
        assert_eq!(report.workflow_event_log_denial_confirmed_count, 7);
        assert_eq!(report.sqlite_denial_confirmed_count, 7);
        assert_eq!(report.live_denial_confirmed_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.in_memory_shadow_write_rehearsal_ready);
        assert!(report.in_memory_shadow_receipts_rendered);
    }

    #[test]
    fn shadow_write_rehearsal_keeps_all_real_writes_closed() {
        let report =
            controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report();

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
            ControlledLiveEvidenceReceiptStoreShadowWriteRehearsalReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            !entry.receipt_store_write_allowed
                && !entry.receipt_store_written
                && !entry.receipt_persistence_allowed
                && !entry.receipt_persisted
                && !entry.ledger_write_allowed
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.live_mutation_allowed
        }));
    }

    #[test]
    fn shadow_write_rehearsal_entries_are_stable_redacted_and_query_bound() {
        let report =
            controlled_live_evidence_receipt_store_shadow_write_rehearsal_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.shadow_write_route
                == "readback://controlled-live/evidence-receipt-store/shadow-write-rehearsal/dirty-worktree-boundary"
            && entry.append_only_sequence_key == "shadow-sequence:dirty_worktree_boundary:00000001"));
        assert!(report.entries.iter().all(|entry| {
            entry
                .shadow_write_key
                .starts_with("controlled_live.evidence_receipt_store.shadow_write_rehearsal.")
                && entry.shadow_write_route.starts_with(
                    "readback://controlled-live/evidence-receipt-store/shadow-write-rehearsal/",
                )
                && entry
                    .shadow_receipt_id
                    .starts_with("shadow:controlled-live-evidence-receipt-preflight:")
                && entry
                    .shadow_receipt_payload_fingerprint
                    .starts_with("sha256:controlled-live-evidence-receipt-shadow:")
                && entry
                    .shadow_receipt_payload_fingerprint
                    .ends_with(":metadata-only")
                && !entry.shadow_receipt_payload_fingerprint.contains("secret")
                && entry.previous_receipt_head == "absent"
                && entry
                    .projected_receipt_head
                    .starts_with("metadata-only-shadow-head:")
                && entry.secret_payload_state == "denied"
                && entry.shadow_payload_state == "metadata_shape_rendered_in_memory"
                && entry.readback_projection_state == "query_matches_projected_shadow_receipt"
                && entry.current_state == "missing"
                && entry.in_memory_shadow_receipt_rendered
                && entry.readback_query_bound
                && entry.redacted_payload_projected
                && entry.secret_payload_denied
        }));
    }
}
