use crate::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report;
use std::sync::OnceLock;

use serde::Serialize;

pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_GATE: &str =
    "controlled_live_evidence_receipt_store_preflight_readback_gate";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_SCHEMA_VERSION: &str =
    "controlled_live_evidence_receipt_store_preflight_readback_v1";
pub const CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "controlled_live_evidence_receipt_store_shadow_write_rehearsal_without_persistence";

const STORE_ROOT: &str = ".hepta/controlled-live/evidence-receipts/status-canary";
const RECEIPT_SCHEMA_VERSION: &str = "controlled_live_evidence_receipt_v1";
const REDACTION_POLICY: &str = "metadata_only_no_secret_payload";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePreflightReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_kill_switch_rehearsal_boundary_readback_ready: bool,
    pub source_kill_switch_rehearsal_boundary_entry_count: usize,
    pub source_kill_switch_rehearsal_evidence_missing_count: usize,
    pub source_packet_id: &'static str,
    pub source_packet_payload_hash: &'static str,
    pub store_root: &'static str,
    pub store_preflight_entry_count: usize,
    pub store_preflight_ready_count: usize,
    pub missing_evidence_entry_count: usize,
    pub path_allowlist_projected_count: usize,
    pub receipt_schema_projected_count: usize,
    pub redaction_policy_projected_count: usize,
    pub secret_payload_denial_projected_count: usize,
    pub idempotency_key_projected_count: usize,
    pub append_only_contract_projected_count: usize,
    pub retention_policy_projected_count: usize,
    pub readback_query_projected_count: usize,
    pub replay_guard_projected_count: usize,
    pub evidence_recorded_count: usize,
    pub blocker_waived_count: usize,
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
    pub receipt_store_preflight_ready: bool,
    pub blockers: Vec<&'static str>,
    pub entries: Vec<ControlledLiveEvidenceReceiptStorePreflightReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveEvidenceReceiptStorePreflightReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveEvidenceReceiptStorePreflightReadbackEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub packet_id: &'static str,
    pub packet_payload_hash: &'static str,
    pub attachment_key: &'static str,
    pub attachment_route: &'static str,
    pub kill_switch_rehearsal_boundary_key: &'static str,
    pub kill_switch_rehearsal_boundary_route: &'static str,
    pub store_root: &'static str,
    pub receipt_path: &'static str,
    pub receipt_id: &'static str,
    pub receipt_schema_version: &'static str,
    pub receipt_status: &'static str,
    pub idempotency_key: &'static str,
    pub readback_query_key: &'static str,
    pub readback_query_route: &'static str,
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
    pub path_allowlist_state: &'static str,
    pub append_only_contract: &'static str,
    pub retention_policy: &'static str,
    pub replay_guard_state: &'static str,
    pub kill_switch_rehearsal_boundary_confirmed: bool,
    pub missing_evidence_confirmed: bool,
    pub path_allowlist_projected: bool,
    pub receipt_schema_projected: bool,
    pub redaction_policy_projected: bool,
    pub secret_payload_denied: bool,
    pub idempotency_key_projected: bool,
    pub append_only_contract_projected: bool,
    pub retention_policy_projected: bool,
    pub readback_query_projected: bool,
    pub replay_guard_projected: bool,
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
pub struct ControlledLiveEvidenceReceiptStorePreflightReadbackSideEffects {
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

pub fn controlled_live_evidence_receipt_store_preflight_readback_report()
-> ControlledLiveEvidenceReceiptStorePreflightReadbackReport {
    static REPORT: OnceLock<ControlledLiveEvidenceReceiptStorePreflightReadbackReport> =
        OnceLock::new();
    REPORT
        .get_or_init(|| {
    let source =
        controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report();
    let source_packet_id = source
        .entries
        .first()
        .map(|entry| entry.packet_id)
        .unwrap_or("unknown");
    let source_packet_payload_hash = source
        .entries
        .first()
        .map(|entry| entry.packet_payload_hash)
        .unwrap_or("unknown");
    let entries = controlled_live_evidence_receipt_store_preflight_readback_entries();

    let store_preflight_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.kill_switch_rehearsal_boundary_confirmed
                && entry.missing_evidence_confirmed
                && entry.path_allowlist_projected
                && entry.receipt_schema_projected
                && entry.redaction_policy_projected
                && entry.secret_payload_denied
                && entry.idempotency_key_projected
                && entry.append_only_contract_projected
                && entry.retention_policy_projected
                && entry.readback_query_projected
                && entry.replay_guard_projected
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
    let missing_evidence_entry_count = entries
        .iter()
        .filter(|entry| entry.current_state == "missing" && entry.missing_evidence_confirmed)
        .count();
    let path_allowlist_projected_count = entries
        .iter()
        .filter(|entry| entry.path_allowlist_projected)
        .count();
    let receipt_schema_projected_count = entries
        .iter()
        .filter(|entry| entry.receipt_schema_projected)
        .count();
    let redaction_policy_projected_count = entries
        .iter()
        .filter(|entry| entry.redaction_policy_projected)
        .count();
    let secret_payload_denial_projected_count = entries
        .iter()
        .filter(|entry| entry.secret_payload_denied)
        .count();
    let idempotency_key_projected_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_projected)
        .count();
    let append_only_contract_projected_count = entries
        .iter()
        .filter(|entry| entry.append_only_contract_projected)
        .count();
    let retention_policy_projected_count = entries
        .iter()
        .filter(|entry| entry.retention_policy_projected)
        .count();
    let readback_query_projected_count = entries
        .iter()
        .filter(|entry| entry.readback_query_projected)
        .count();
    let replay_guard_projected_count = entries
        .iter()
        .filter(|entry| entry.replay_guard_projected)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();

    let receipt_store_preflight_ready = source.kill_switch_rehearsal_boundary_readback_ready
        && source.kill_switch_rehearsal_boundary_entry_count == 7
        && source.kill_switch_rehearsal_boundary_ready_count == 7
        && source.kill_switch_rehearsal_evidence_missing_count == 7
        && source.kill_switch_rehearsal_receipt_persistence_blocked_count == 7
        && !source.packet_send_attempted
        && !source.attachment_send_attempted
        && !source.approval_request_sent
        && !source.approval_accepted
        && !source.credential_read_allowed
        && !source.kill_switch_rehearsal_allowed
        && !source.kill_switch_mutation_allowed
        && !source.evidence_recording_allowed
        && !source.evidence_persisted
        && !source.packet_persisted
        && !source.attachment_persisted
        && !source.readback_persisted
        && !source.live_execution_allowed
        && entries.len() == 7
        && store_preflight_ready_count == 7
        && missing_evidence_entry_count == 7
        && path_allowlist_projected_count == 7
        && receipt_schema_projected_count == 7
        && redaction_policy_projected_count == 7
        && secret_payload_denial_projected_count == 7
        && idempotency_key_projected_count == 7
        && append_only_contract_projected_count == 7
        && retention_policy_projected_count == 7
        && readback_query_projected_count == 7
        && replay_guard_projected_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.packet_id == source_packet_id
                && entry.packet_payload_hash == source_packet_payload_hash
                && entry.receipt_schema_version == RECEIPT_SCHEMA_VERSION
                && entry.redaction_policy == REDACTION_POLICY
                && entry.secret_payload_state == "denied"
                && entry.receipt_status == "projected_missing_evidence_no_write"
                && entry.observed_state == "receipt_store_preflight_projected_no_write"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
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

    ControlledLiveEvidenceReceiptStorePreflightReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_evidence_receipt_store_preflight_readback",
        status: if receipt_store_preflight_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_GATE,
        schema_version: CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_kill_switch_rehearsal_boundary_readback_ready: source
            .kill_switch_rehearsal_boundary_readback_ready,
        source_kill_switch_rehearsal_boundary_entry_count: source
            .kill_switch_rehearsal_boundary_entry_count,
        source_kill_switch_rehearsal_evidence_missing_count: source
            .kill_switch_rehearsal_evidence_missing_count,
        source_packet_id,
        source_packet_payload_hash,
        store_root: STORE_ROOT,
        store_preflight_entry_count: entries.len(),
        store_preflight_ready_count,
        missing_evidence_entry_count,
        path_allowlist_projected_count,
        receipt_schema_projected_count,
        redaction_policy_projected_count,
        secret_payload_denial_projected_count,
        idempotency_key_projected_count,
        append_only_contract_projected_count,
        retention_policy_projected_count,
        readback_query_projected_count,
        replay_guard_projected_count,
        evidence_recorded_count,
        blocker_waived_count,
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
        receipt_store_preflight_ready,
        blockers: vec![
            "evidence_missing",
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
            CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PREFLIGHT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveEvidenceReceiptStorePreflightReadbackSideEffects::none(),
    }
        })
        .clone()
}

pub fn controlled_live_evidence_receipt_store_preflight_readback_entries()
-> Vec<ControlledLiveEvidenceReceiptStorePreflightReadbackEntry> {
    controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveEvidenceReceiptStorePreflightReadbackEntry {
            id: receipt_store_preflight_id(entry.source_blocker_id),
            source_blocker_id: entry.source_blocker_id,
            packet_id: entry.packet_id,
            packet_payload_hash: entry.packet_payload_hash,
            attachment_key: entry.attachment_key,
            attachment_route: entry.attachment_route,
            kill_switch_rehearsal_boundary_key: entry.kill_switch_rehearsal_boundary_key,
            kill_switch_rehearsal_boundary_route: entry.kill_switch_rehearsal_boundary_route,
            store_root: STORE_ROOT,
            receipt_path: receipt_path(entry.source_blocker_id),
            receipt_id: receipt_id(entry.source_blocker_id),
            receipt_schema_version: RECEIPT_SCHEMA_VERSION,
            receipt_status: "projected_missing_evidence_no_write",
            idempotency_key: idempotency_key(entry.source_blocker_id),
            readback_query_key: readback_query_key(entry.source_blocker_id),
            readback_query_route: readback_query_route(entry.source_blocker_id),
            operator_display_order: entry.operator_display_order,
            operator_status: entry.operator_status,
            observed_state: "receipt_store_preflight_projected_no_write",
            previous_state: entry.previous_state,
            current_state: entry.current_state,
            state_delta: entry.state_delta,
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            redaction_policy: REDACTION_POLICY,
            secret_payload_state: "denied",
            path_allowlist_state: "projected",
            append_only_contract: "projected_append_only_metadata_receipt",
            retention_policy: "projected_local_receipt_metadata_only",
            replay_guard_state: "projected_no_replay_execution",
            kill_switch_rehearsal_boundary_confirmed: true,
            missing_evidence_confirmed: true,
            path_allowlist_projected: true,
            receipt_schema_projected: true,
            redaction_policy_projected: true,
            secret_payload_denied: true,
            idempotency_key_projected: true,
            append_only_contract_projected: true,
            retention_policy_projected: true,
            readback_query_projected: true,
            replay_guard_projected: true,
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
        })
        .collect()
}

fn receipt_store_preflight_id(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "evidence_receipt_store_preflight_dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "evidence_receipt_store_preflight_operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "evidence_receipt_store_preflight_fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "evidence_receipt_store_preflight_credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "evidence_receipt_store_preflight_gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "evidence_receipt_store_preflight_rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "evidence_receipt_store_preflight_kill_switch_rehearsal_missing"
        }
        _ => "evidence_receipt_store_preflight_unknown",
    }
}

fn receipt_path(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/dirty-worktree-boundary.receipt.json"
        }
        "operator_live_approval_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/operator-live-approval-missing.receipt.json"
        }
        "fresh_soak_readback_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/fresh-soak-readback-missing.receipt.json"
        }
        "credential_boundary_attestation_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/credential-boundary-attestation-missing.receipt.json"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/gateway-native-telegram-post-boundary-approval-missing.receipt.json"
        }
        "rollback_rehearsal_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/rollback-rehearsal-missing.receipt.json"
        }
        "kill_switch_rehearsal_missing" => {
            ".hepta/controlled-live/evidence-receipts/status-canary/kill-switch-rehearsal-missing.receipt.json"
        }
        _ => ".hepta/controlled-live/evidence-receipts/status-canary/unknown.receipt.json",
    }
}

fn receipt_id(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled-live-evidence-receipt-preflight:dirty_worktree_boundary:missing"
        }
        "operator_live_approval_missing" => {
            "controlled-live-evidence-receipt-preflight:operator_live_approval_missing:missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled-live-evidence-receipt-preflight:fresh_soak_readback_missing:missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled-live-evidence-receipt-preflight:credential_boundary_attestation_missing:missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled-live-evidence-receipt-preflight:gateway_native_telegram_post_boundary_approval_missing:missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled-live-evidence-receipt-preflight:rollback_rehearsal_missing:missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled-live-evidence-receipt-preflight:kill_switch_rehearsal_missing:missing"
        }
        _ => "controlled-live-evidence-receipt-preflight:unknown:missing",
    }
}

fn idempotency_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled-live-evidence-receipt-preflight:idempotency:dirty_worktree_boundary:controlled-live-operator-packet-preview-no-live-payload"
        }
        "operator_live_approval_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:operator_live_approval_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        "fresh_soak_readback_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:fresh_soak_readback_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        "credential_boundary_attestation_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:credential_boundary_attestation_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:gateway_native_telegram_post_boundary_approval_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        "rollback_rehearsal_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:rollback_rehearsal_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled-live-evidence-receipt-preflight:idempotency:kill_switch_rehearsal_missing:controlled-live-operator-packet-preview-no-live-payload"
        }
        _ => {
            "controlled-live-evidence-receipt-preflight:idempotency:unknown:controlled-live-operator-packet-preview-no-live-payload"
        }
    }
}

fn readback_query_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.evidence_receipt_store.preflight.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.evidence_receipt_store.preflight.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.evidence_receipt_store.preflight.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.evidence_receipt_store.preflight.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.evidence_receipt_store.preflight.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.evidence_receipt_store.preflight.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.evidence_receipt_store.preflight.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.evidence_receipt_store.preflight.unknown",
    }
}

fn readback_query_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/evidence-receipt-store/preflight/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/evidence-receipt-store/preflight/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/evidence-receipt-store/preflight/unknown",
    }
}

impl ControlledLiveEvidenceReceiptStorePreflightReadbackSideEffects {
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
    fn receipt_store_preflight_projects_all_missing_evidence_receipts_without_writes() {
        let report = controlled_live_evidence_receipt_store_preflight_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_kill_switch_rehearsal_boundary_readback_ready);
        assert_eq!(report.source_kill_switch_rehearsal_boundary_entry_count, 7);
        assert_eq!(
            report.source_kill_switch_rehearsal_evidence_missing_count,
            7
        );
        assert_eq!(report.store_preflight_entry_count, 7);
        assert_eq!(report.store_preflight_ready_count, 7);
        assert_eq!(report.missing_evidence_entry_count, 7);
        assert_eq!(report.path_allowlist_projected_count, 7);
        assert_eq!(report.receipt_schema_projected_count, 7);
        assert_eq!(report.redaction_policy_projected_count, 7);
        assert_eq!(report.secret_payload_denial_projected_count, 7);
        assert_eq!(report.idempotency_key_projected_count, 7);
        assert_eq!(report.append_only_contract_projected_count, 7);
        assert_eq!(report.retention_policy_projected_count, 7);
        assert_eq!(report.readback_query_projected_count, 7);
        assert_eq!(report.replay_guard_projected_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.receipt_store_preflight_ready);
    }

    #[test]
    fn receipt_store_preflight_keeps_persistence_and_live_closed() {
        let report = controlled_live_evidence_receipt_store_preflight_readback_report();

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
            ControlledLiveEvidenceReceiptStorePreflightReadbackSideEffects::none()
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
    fn receipt_store_preflight_entries_are_stable_and_redacted() {
        let report = controlled_live_evidence_receipt_store_preflight_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "dirty_worktree_boundary"
            && entry.receipt_path
                == ".hepta/controlled-live/evidence-receipts/status-canary/dirty-worktree-boundary.receipt.json"
            && entry.readback_query_route
                == "readback://controlled-live/evidence-receipt-store/preflight/dirty-worktree-boundary"));
        assert!(report.entries.iter().all(|entry| {
            entry.receipt_path.starts_with(STORE_ROOT)
                && entry
                    .receipt_id
                    .starts_with("controlled-live-evidence-receipt-preflight:")
                && entry
                    .idempotency_key
                    .starts_with("controlled-live-evidence-receipt-preflight:idempotency:")
                && entry
                    .readback_query_route
                    .starts_with("readback://controlled-live/evidence-receipt-store/preflight/")
                && entry.receipt_schema_version == RECEIPT_SCHEMA_VERSION
                && entry.redaction_policy == REDACTION_POLICY
                && entry.secret_payload_state == "denied"
                && !entry.receipt_id.contains("secret")
                && !entry.idempotency_key.contains("secret")
                && entry.packet_payload_hash
                    == "sha256:controlled-live-operator-packet-preview-no-live-payload"
                && entry.current_state == "missing"
                && entry.receipt_status == "projected_missing_evidence_no_write"
        }));
    }
}
