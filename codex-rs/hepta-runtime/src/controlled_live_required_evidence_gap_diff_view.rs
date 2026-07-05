use crate::controlled_live_required_evidence_gap_summary::controlled_live_required_evidence_gap_summary_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_GATE: &str =
    "controlled_live_required_evidence_gap_diff_view_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_gap_diff_view_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_RECOMMENDED_NEXT_GATE: &str =
    "phase5h_controlled_live_required_evidence_gap_operator_readback_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapDiffViewReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_gap_summary_ready: bool,
    pub source_gap_entry_count: usize,
    pub source_missing_evidence_count: usize,
    pub diff_entry_count: usize,
    pub stable_diff_key_count: usize,
    pub comparable_entry_count: usize,
    pub unchanged_missing_count: usize,
    pub owner_count: usize,
    pub risk_bucket_count: usize,
    pub evidence_recorded_count: usize,
    pub diff_view_ready: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceGapDiffViewEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceGapDiffViewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapDiffViewEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
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
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapDiffViewSideEffects {
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

pub fn controlled_live_required_evidence_gap_diff_view_report()
-> ControlledLiveRequiredEvidenceGapDiffViewReport {
    let source = controlled_live_required_evidence_gap_summary_report();
    let entries = controlled_live_required_evidence_gap_diff_view_entries();
    let stable_diff_key_count = entries
        .iter()
        .filter(|entry| !entry.diff_view_key.is_empty() && !entry.comparison_anchor.is_empty())
        .count();
    let comparable_entry_count = entries.iter().filter(|entry| entry.comparable).count();
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
    let diff_view_ready = source.gap_summary_ready
        && source.gap_entry_count == 7
        && source.missing_evidence_count == 7
        && entries.len() == 7
        && stable_diff_key_count == 7
        && comparable_entry_count == 7
        && unchanged_missing_count == 7
        && owner_count == 7
        && risk_bucket_count == 3
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.comparable
                && entry.previous_state == "missing"
                && entry.current_state == "missing"
                && entry.state_delta == "unchanged_missing"
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.approval_acceptance_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapDiffViewReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_diff_view",
        status: if diff_view_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_GATE,
        schema_version: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_gap_summary_ready: source.gap_summary_ready,
        source_gap_entry_count: source.gap_entry_count,
        source_missing_evidence_count: source.missing_evidence_count,
        diff_entry_count: entries.len(),
        stable_diff_key_count,
        comparable_entry_count,
        unchanged_missing_count,
        owner_count,
        risk_bucket_count,
        evidence_recorded_count,
        diff_view_ready,
        approval_acceptance_ready: false,
        approval_accepted: false,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceGapDiffViewSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_diff_view_entries()
-> Vec<ControlledLiveRequiredEvidenceGapDiffViewEntry> {
    controlled_live_required_evidence_gap_summary_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceGapDiffViewEntry {
            id: entry.id,
            source_blocker_id: entry.source_blocker_id,
            gap_key: entry.gap_key,
            diff_view_key: diff_view_key(entry.source_blocker_id),
            comparison_anchor: comparison_anchor(entry.source_blocker_id),
            owner: entry.owner,
            risk_bucket: entry.risk_bucket,
            previous_state: "missing",
            current_state: entry.evidence_state,
            state_delta: "unchanged_missing",
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
            live_mutation_allowed: false,
        })
        .collect()
}

fn diff_view_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.diff_view.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.diff_view.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.diff_view.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.diff_view.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.diff_view.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.diff_view.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.diff_view.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.gap.diff_view.unknown",
    }
}

fn comparison_anchor(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "gap-summary-owner-risk:dirty-worktree-boundary",
        "operator_live_approval_missing" => "gap-summary-owner-risk:operator-live-approval-missing",
        "fresh_soak_readback_missing" => "gap-summary-owner-risk:fresh-soak-readback-missing",
        "credential_boundary_attestation_missing" => {
            "gap-summary-owner-risk:credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "gap-summary-owner-risk:gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => "gap-summary-owner-risk:rollback-rehearsal-missing",
        "kill_switch_rehearsal_missing" => "gap-summary-owner-risk:kill-switch-rehearsal-missing",
        _ => "gap-summary-owner-risk:unknown",
    }
}

impl ControlledLiveRequiredEvidenceGapDiffViewSideEffects {
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
    fn gap_diff_view_is_ready_blocked_without_acceptance_or_recording() {
        let report = controlled_live_required_evidence_gap_diff_view_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_gap_summary_ready);
        assert_eq!(report.source_gap_entry_count, 7);
        assert_eq!(report.source_missing_evidence_count, 7);
        assert_eq!(report.diff_entry_count, 7);
        assert_eq!(report.stable_diff_key_count, 7);
        assert_eq!(report.comparable_entry_count, 7);
        assert_eq!(report.unchanged_missing_count, 7);
        assert_eq!(report.owner_count, 7);
        assert_eq!(report.risk_bucket_count, 3);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.diff_view_ready);
        assert!(!report.approval_acceptance_ready);
        assert!(!report.approval_accepted);
        assert_eq!(report.blocker_waived_count, 0);
        assert!(!report.credential_read_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn gap_diff_view_keeps_all_gaps_comparable_and_unchanged_missing() {
        let report = controlled_live_required_evidence_gap_diff_view_report();

        assert!(report.entries.iter().any(|entry| entry.source_blocker_id
            == "gateway_native_telegram_post_boundary_approval_missing"
            && entry.owner == "transport_boundary_owner"
            && entry.risk_bucket == "critical"
            && entry.comparison_anchor
                == "gap-summary-owner-risk:gateway-native-telegram-post-boundary-approval-missing"));
        assert!(report.entries.iter().all(|entry| entry.queryable
            && entry.operator_visible
            && entry.comparable
            && entry.previous_state == "missing"
            && entry.current_state == "missing"
            && entry.state_delta == "unchanged_missing"
            && !entry.diff_view_key.is_empty()
            && !entry.comparison_anchor.is_empty()
            && !entry.evidence_recorded
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn gap_diff_view_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_gap_diff_view_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapDiffViewSideEffects::none()
        );
    }
}
