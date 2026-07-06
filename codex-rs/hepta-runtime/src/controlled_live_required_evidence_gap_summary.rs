use crate::controlled_live_required_evidence_readback_index::controlled_live_required_evidence_readback_index_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_GATE: &str =
    "controlled_live_required_evidence_gap_summary_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_gap_summary_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_RECOMMENDED_NEXT_GATE: &str =
    "phase5g_controlled_live_required_evidence_gap_diff_view_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapSummaryReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_readback_index_ready: bool,
    pub source_index_entry_count: usize,
    pub source_missing_evidence_count: usize,
    pub gap_entry_count: usize,
    pub missing_evidence_count: usize,
    pub owner_count: usize,
    pub risk_bucket_count: usize,
    pub high_risk_gap_count: usize,
    pub operator_visible_gap_count: usize,
    pub queryable_gap_count: usize,
    pub evidence_recorded_count: usize,
    pub gap_summary_ready: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceGapSummaryEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceGapSummarySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapSummaryEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub gap_key: &'static str,
    pub owner: &'static str,
    pub risk_bucket: &'static str,
    pub cutover_risk: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub diff_key: &'static str,
    pub fingerprint: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub evidence_state: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub evidence_missing: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceGapSummarySideEffects {
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

pub fn controlled_live_required_evidence_gap_summary_report()
-> ControlledLiveRequiredEvidenceGapSummaryReport {
    let source = controlled_live_required_evidence_readback_index_report();
    let entries = controlled_live_required_evidence_gap_summary_entries();
    let missing_evidence_count = entries
        .iter()
        .filter(|entry| entry.evidence_missing)
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
    let high_risk_gap_count = entries
        .iter()
        .filter(|entry| matches!(entry.risk_bucket, "critical" | "high"))
        .count();
    let operator_visible_gap_count = entries
        .iter()
        .filter(|entry| entry.operator_visible)
        .count();
    let queryable_gap_count = entries.iter().filter(|entry| entry.queryable).count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let source_missing_evidence_count = source
        .entries
        .iter()
        .filter(|entry| entry.evidence_state == "missing")
        .count();
    let gap_summary_ready = source.readback_index_ready
        && source.index_entry_count == 7
        && source_missing_evidence_count == 7
        && entries.len() == 7
        && missing_evidence_count == 7
        && owner_count == 7
        && risk_bucket_count == 3
        && high_risk_gap_count == 6
        && operator_visible_gap_count == 7
        && queryable_gap_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.evidence_state == "missing"
                && entry.evidence_missing
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.approval_acceptance_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceGapSummaryReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_gap_summary",
        status: if gap_summary_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_GATE,
        schema_version: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_readback_index_ready: source.readback_index_ready,
        source_index_entry_count: source.index_entry_count,
        source_missing_evidence_count,
        gap_entry_count: entries.len(),
        missing_evidence_count,
        owner_count,
        risk_bucket_count,
        high_risk_gap_count,
        operator_visible_gap_count,
        queryable_gap_count,
        evidence_recorded_count,
        gap_summary_ready,
        approval_acceptance_ready: false,
        approval_accepted: false,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        entries,
        recommended_next_gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceGapSummarySideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_gap_summary_entries()
-> Vec<ControlledLiveRequiredEvidenceGapSummaryEntry> {
    controlled_live_required_evidence_readback_index_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceGapSummaryEntry {
            id: entry.id,
            source_blocker_id: entry.source_blocker_id,
            gap_key: gap_key(entry.source_blocker_id),
            owner: owner_for(entry.source_blocker_id),
            risk_bucket: risk_bucket_for(entry.source_blocker_id),
            cutover_risk: cutover_risk_for(entry.source_blocker_id),
            query_key: entry.query_key,
            readback_route: entry.readback_route,
            diff_key: entry.diff_key,
            fingerprint: entry.fingerprint,
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            evidence_state: entry.evidence_state,
            operator_visible: true,
            queryable: true,
            evidence_missing: entry.evidence_state == "missing",
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

fn gap_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.gap.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.gap.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.gap.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.gap.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gap.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.gap.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.gap.unknown",
    }
}

fn owner_for(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "hepta_systems_lane_owner",
        "operator_live_approval_missing" => "operator",
        "fresh_soak_readback_missing" => "runtime_soak_owner",
        "credential_boundary_attestation_missing" => "credential_boundary_owner",
        "gateway_native_telegram_post_boundary_approval_missing" => "transport_boundary_owner",
        "rollback_rehearsal_missing" => "rollback_rehearsal_owner",
        "kill_switch_rehearsal_missing" => "kill_switch_owner",
        _ => "unknown_owner",
    }
}

fn risk_bucket_for(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "medium",
        "operator_live_approval_missing" => "critical",
        "fresh_soak_readback_missing" => "high",
        "credential_boundary_attestation_missing" => "critical",
        "gateway_native_telegram_post_boundary_approval_missing" => "critical",
        "rollback_rehearsal_missing" => "high",
        "kill_switch_rehearsal_missing" => "high",
        _ => "unknown",
    }
}

fn cutover_risk_for(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "dirty checkout can hide unrelated live-path drift",
        "operator_live_approval_missing" => "no explicit human authorization for live execution",
        "fresh_soak_readback_missing" => "read-only chain lacks fresh soak/readback evidence",
        "credential_boundary_attestation_missing" => {
            "credential access boundary has not been attested"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "external POST and transport mutation boundaries are not approved"
        }
        "rollback_rehearsal_missing" => "rollback path has not been rehearsed",
        "kill_switch_rehearsal_missing" => "kill-switch path has not been rehearsed",
        _ => "unknown cutover risk",
    }
}

impl ControlledLiveRequiredEvidenceGapSummarySideEffects {
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
    fn gap_summary_is_ready_blocked_without_accepting_evidence() {
        let report = controlled_live_required_evidence_gap_summary_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_readback_index_ready);
        assert_eq!(report.source_index_entry_count, 7);
        assert_eq!(report.source_missing_evidence_count, 7);
        assert_eq!(report.gap_entry_count, 7);
        assert_eq!(report.missing_evidence_count, 7);
        assert_eq!(report.owner_count, 7);
        assert_eq!(report.risk_bucket_count, 3);
        assert_eq!(report.high_risk_gap_count, 6);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.gap_summary_ready);
        assert!(!report.approval_acceptance_ready);
        assert!(!report.approval_accepted);
        assert_eq!(report.blocker_waived_count, 0);
        assert!(!report.credential_read_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn gap_summary_groups_all_missing_evidence_by_owner_and_risk() {
        let report = controlled_live_required_evidence_gap_summary_report();

        assert!(report.entries.iter().any(|entry| entry.owner == "operator"
            && entry.risk_bucket == "critical"
            && entry.source_blocker_id == "operator_live_approval_missing"));
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.owner == "transport_boundary_owner"
                    && entry.risk_bucket == "critical"
                    && entry.source_blocker_id
                        == "gateway_native_telegram_post_boundary_approval_missing")
        );
        assert!(report.entries.iter().all(|entry| entry.queryable
            && entry.operator_visible
            && entry.evidence_missing
            && entry.evidence_state == "missing"
            && !entry.gap_key.is_empty()
            && !entry.cutover_risk.is_empty()
            && !entry.evidence_recorded
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn gap_summary_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_gap_summary_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceGapSummarySideEffects::none()
        );
    }
}
