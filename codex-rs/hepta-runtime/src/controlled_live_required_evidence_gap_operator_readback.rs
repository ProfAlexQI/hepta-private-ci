use crate::controlled_live_required_evidence_gap_diff_view::controlled_live_required_evidence_gap_diff_view_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_GATE: &str =
    "controlled_live_required_evidence_gap_operator_readback_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_gap_operator_readback_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "phase5i_controlled_live_required_evidence_gap_operator_packet_attachment_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_diff_view_ready: bool,
    pub source_diff_entry_count: usize,
    pub source_unchanged_missing_count: usize,
    pub operator_readback_entry_count: usize,
    pub operator_visible_entry_count: usize,
    pub stable_readback_key_count: usize,
    pub unchanged_missing_count: usize,
    pub owner_count: usize,
    pub risk_bucket_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_readback_ready: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub readback_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceGapOperatorReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceGapOperatorReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorReadbackEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub operator_readback_key: &'static str,
    pub operator_readback_route: &'static str,
    pub operator_display_order: usize,
    pub operator_status: &'static str,
    pub operator_action: &'static str,
    pub gap_key: &'static str,
    pub diff_view_key: &'static str,
    pub comparison_anchor: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub previous_state: &'static str,
    pub current_state: &'static str,
    pub state_delta: &'static str,
    pub cutover_risk: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub diff_key: &'static str,
    pub fingerprint: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub comparable: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub persistence_allowed: bool,
    pub readback_persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapOperatorReadbackSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
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
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_required_evidence_gap_operator_readback_report()
-> ControlledLiveRequiredEvidenceGapOperatorReadbackReport {
    let source = controlled_live_required_evidence_gap_diff_view_report();
    let entries = controlled_live_required_evidence_gap_operator_readback_entries();
    let operator_visible_entry_count = entries
        .iter()
        .filter(|entry| entry.operator_visible)
        .count();
    let stable_readback_key_count = entries
        .iter()
        .filter(|entry| {
            !entry.operator_readback_key.is_empty()
                && !entry.operator_readback_route.is_empty()
                && entry.operator_display_order > 0
        })
        .count();
    let unchanged_missing_count = entries
        .iter()
        .filter(|entry| {
            entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
        })
        .count();
    let owner_count = entries
        .iter()
        .map(|entry| entry.owner)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let risk_bucket_count = entries
        .iter()
        .map(|entry| entry.risk_bucket)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let operator_readback_ready = source.diff_view_ready
        && source.diff_entry_count == 7
        && source.unchanged_missing_count == 7
        && entries.len() == 7
        && operator_visible_entry_count == 7
        && stable_readback_key_count == 7
        && unchanged_missing_count == 7
        && owner_count == 7
        && risk_bucket_count == 3
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.comparable
                && entry.operator_status == "blocked_missing_evidence"
                && entry.operator_action == "collect_required_evidence_before_live_cutover"
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.approval_acceptance_allowed
                && !entry.persistence_allowed
                && !entry.readback_persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapOperatorReadbackReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_operator_readback",
        status: if operator_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_GATE,
        schema_version: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_diff_view_ready: source.diff_view_ready,
        source_diff_entry_count: source.diff_entry_count,
        source_unchanged_missing_count: source.unchanged_missing_count,
        operator_readback_entry_count: entries.len(),
        operator_visible_entry_count,
        stable_readback_key_count,
        unchanged_missing_count,
        owner_count,
        risk_bucket_count,
        evidence_recorded_count,
        operator_readback_ready,
        approval_acceptance_ready: false,
        approval_accepted: false,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        readback_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceGapOperatorReadbackSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_operator_readback_entries()
-> Vec<ControlledLiveRequiredEvidenceGapOperatorReadbackEntry> {
    controlled_live_required_evidence_gap_diff_view_report()
        .entries
        .into_iter()
        .map(
            |entry| ControlledLiveRequiredEvidenceGapOperatorReadbackEntry {
                id: entry.id,
                source_blocker_id: entry.source_blocker_id,
                operator_readback_key: operator_readback_key(entry.source_blocker_id),
                operator_readback_route: operator_readback_route(entry.source_blocker_id),
                operator_display_order: operator_display_order(entry.source_blocker_id),
                operator_status: "blocked_missing_evidence",
                operator_action: "collect_required_evidence_before_live_cutover",
                gap_key: entry.gap_key,
                diff_view_key: entry.diff_view_key,
                comparison_anchor: entry.comparison_anchor,
                owner: entry.owner,
                risk_bucket: entry.risk_bucket,
                previous_state: entry.previous_state,
                current_state: entry.current_state,
                state_delta: entry.state_delta,
                cutover_risk: entry.cutover_risk,
                query_key: entry.query_key,
                readback_route: entry.readback_route,
                diff_key: entry.diff_key,
                fingerprint: entry.fingerprint,
                operator_label: entry.operator_label,
                required_evidence: entry.required_evidence,
                operator_visible: true,
                queryable: true,
                comparable: true,
                evidence_recorded: false,
                evidence_recording_allowed: false,
                credential_read_allowed: false,
                approval_acceptance_allowed: false,
                blocker_waiver_allowed: false,
                persistence_allowed: false,
                readback_persistence_allowed: false,
                live_mutation_allowed: false,
            },
        )
        .collect()
}

fn operator_readback_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.operator_readback.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.operator_readback.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.gap.operator_readback.unknown",
    }
}

fn operator_readback_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "readback://controlled-live/required-evidence/gap/operator/dirty-worktree-boundary"
        }
        "operator_live_approval_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/rollback-rehearsal-missing"
        }
        "kill_switch_rehearsal_missing" => {
            "readback://controlled-live/required-evidence/gap/operator/kill-switch-rehearsal-missing"
        }
        _ => "readback://controlled-live/required-evidence/gap/operator/unknown",
    }
}

fn operator_display_order(source_blocker_id: &str) -> usize {
    match source_blocker_id {
        "dirty_worktree_boundary" => 1,
        "operator_live_approval_missing" => 2,
        "fresh_soak_readback_missing" => 3,
        "credential_boundary_attestation_missing" => 4,
        "gateway_native_telegram_post_boundary_approval_missing" => 5,
        "rollback_rehearsal_missing" => 6,
        "kill_switch_rehearsal_missing" => 7,
        _ => 0,
    }
}

impl ControlledLiveRequiredEvidenceGapOperatorReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
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
    fn operator_readback_is_ready_blocked_without_acceptance_or_recording() {
        let report = controlled_live_required_evidence_gap_operator_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_diff_view_ready);
        assert_eq!(report.source_diff_entry_count, 7);
        assert_eq!(report.source_unchanged_missing_count, 7);
        assert_eq!(report.operator_readback_entry_count, 7);
        assert_eq!(report.operator_visible_entry_count, 7);
        assert_eq!(report.stable_readback_key_count, 7);
        assert_eq!(report.unchanged_missing_count, 7);
        assert_eq!(report.owner_count, 7);
        assert_eq!(report.risk_bucket_count, 3);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.operator_readback_ready);
        assert!(!report.approval_acceptance_ready);
        assert!(!report.approval_accepted);
        assert_eq!(report.blocker_waived_count, 0);
        assert!(!report.credential_read_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn operator_readback_keeps_all_gaps_visible_and_ordered() {
        let report = controlled_live_required_evidence_gap_operator_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "operator_live_approval_missing"
            && entry.operator_display_order == 2
            && entry.operator_status == "blocked_missing_evidence"
            && entry.operator_readback_route
                == "readback://controlled-live/required-evidence/gap/operator/operator-live-approval-missing"));
        assert!(report.entries.iter().all(|entry| entry.queryable
            && entry.operator_visible
            && entry.comparable
            && entry.operator_status == "blocked_missing_evidence"
            && entry.operator_action == "collect_required_evidence_before_live_cutover"
            && entry.previous_state == "missing"
            && entry.current_state == "missing"
            && entry.state_delta == "unchanged_missing"
            && !entry.operator_readback_key.is_empty()
            && !entry.operator_readback_route.is_empty()
            && entry.operator_display_order > 0
            && !entry.evidence_recorded
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.persistence_allowed
            && !entry.readback_persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn operator_readback_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_gap_operator_readback_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapOperatorReadbackSideEffects::none()
        );
    }
}
