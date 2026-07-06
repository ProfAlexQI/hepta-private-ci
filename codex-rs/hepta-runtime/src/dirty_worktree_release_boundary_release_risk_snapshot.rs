use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorEvidenceRecordingBoundaryReadbackReport;
use crate::dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_GATE: &str =
    "dirty_worktree_release_boundary_release_risk_snapshot_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_release_risk_snapshot_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_RECOMMENDED_NEXT_GATE: &str = "phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_evidence_recording_boundary_gate: &'static str,
    pub source_evidence_recording_boundary_ready: bool,
    pub source_evidence_recording_boundary_visible: bool,
    pub source_evidence_recording_boundary_persisted: bool,
    pub source_evidence_recorded: bool,
    pub source_evidence_recording_persisted: bool,
    pub source_evidence_receipt_persisted: bool,
    pub source_boundary_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub release_risk_snapshot_scope: DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotScope,
    pub risk_entry_count: usize,
    pub stable_snapshot_key_count: usize,
    pub snapshot_route_count: usize,
    pub snapshot_ready_count: usize,
    pub critical_risk_count: usize,
    pub high_risk_count: usize,
    pub medium_risk_count: usize,
    pub high_or_critical_risk_count: usize,
    pub release_blocked_count: usize,
    pub rehearsal_candidate_count: usize,
    pub pending_operator_decision_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub risk_snapshot_visible: bool,
    pub risk_snapshot_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_persisted: bool,
    pub evidence_receipt_persisted: bool,
    pub approval_request_sent: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_receipt_persisted: bool,
    pub decision_recorded: bool,
    pub decision_recording_persisted: bool,
    pub decision_receipt_persisted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub readback_persisted: bool,
    pub strategy_applied: bool,
    pub release_cutover_allowed: bool,
    pub git_add_allowed: bool,
    pub git_index_mutated: bool,
    pub git_commit_allowed: bool,
    pub git_push_allowed: bool,
    pub git_reset_allowed: bool,
    pub git_checkout_allowed: bool,
    pub git_revert_allowed: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub release_risk_snapshot_ready: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotScope {
    pub snapshot_id: &'static str,
    pub snapshot_route: &'static str,
    pub source_evidence_recording_boundary_route: &'static str,
    pub snapshot_mode: &'static str,
    pub release_cutover_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub live_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotEntry {
    pub source_evidence_boundary_key: String,
    pub source_evidence_boundary_route: String,
    pub snapshot_key: String,
    pub snapshot_route: String,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub recommended_strategy: &'static str,
    pub release_risk_tier: &'static str,
    pub release_risk_reason: &'static str,
    pub release_blocker: &'static str,
    pub release_blocker_state: &'static str,
    pub clean_worktree_rehearsal_candidate: bool,
    pub rehearsal_action: &'static str,
    pub decision_state: &'static str,
    pub evidence_recording_state: &'static str,
    pub evidence_persistence_state: &'static str,
    pub evidence_receipt_state: &'static str,
    pub approval_request_state: &'static str,
    pub approval_acceptance_state: &'static str,
    pub approval_recording_state: &'static str,
    pub approval_receipt_state: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub evidence_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub decision_recording_allowed: bool,
    pub git_add_blocked: bool,
    pub git_index_mutation_blocked: bool,
    pub git_commit_blocked: bool,
    pub git_push_blocked: bool,
    pub git_reset_blocked: bool,
    pub git_checkout_blocked: bool,
    pub git_revert_blocked: bool,
    pub cleanup_blocked: bool,
    pub delete_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotSideEffects {
    pub snapshot_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub evidence_receipt_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_receipt_persisted: bool,
    pub decision_recorded: bool,
    pub decision_recording_persisted: bool,
    pub decision_receipt_persisted: bool,
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub readback_persisted: bool,
    pub git_add_performed: bool,
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub cleanup_performed: bool,
    pub unrelated_file_deleted: bool,
    pub strategy_applied: bool,
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_release_risk_snapshot_report()
-> DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport {
    let source =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_report();
    dirty_worktree_release_boundary_release_risk_snapshot_report_from_evidence_recording_boundary(
        &source,
    )
}

pub fn dirty_worktree_release_boundary_release_risk_snapshot_report_from_evidence_recording_boundary(
    source: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorEvidenceRecordingBoundaryReadbackReport,
) -> DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport {
    let entries = dirty_worktree_release_boundary_release_risk_snapshot_entries(source);
    let stable_snapshot_key_count = entries
        .iter()
        .map(|entry| entry.snapshot_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let snapshot_route_count = entries
        .iter()
        .map(|entry| entry.snapshot_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let snapshot_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.decision_state == "pending_operator_decision"
                && entry.release_blocker_state == "blocked_dirty_worktree"
                && entry.clean_worktree_rehearsal_candidate
                && entry.evidence_recording_state == "evidence_recording_blocked"
                && entry.evidence_persistence_state == "evidence_persistence_blocked"
                && entry.evidence_receipt_state == "evidence_receipt_blocked"
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
                && entry.cleanup_blocked
                && entry.delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        })
        .count();
    let critical_risk_count = entries
        .iter()
        .filter(|entry| entry.release_risk_tier == "critical")
        .count();
    let high_risk_count = entries
        .iter()
        .filter(|entry| entry.release_risk_tier == "high")
        .count();
    let medium_risk_count = entries
        .iter()
        .filter(|entry| entry.release_risk_tier == "medium")
        .count();
    let classified_risk_count = critical_risk_count + high_risk_count + medium_risk_count;
    let release_blocked_count = entries
        .iter()
        .filter(|entry| entry.release_blocker_state == "blocked_dirty_worktree")
        .count();
    let rehearsal_candidate_count = entries
        .iter()
        .filter(|entry| entry.clean_worktree_rehearsal_candidate)
        .count();
    let pending_operator_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_state == "pending_operator_decision")
        .count();
    let evidence_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.evidence_recording_allowed)
        .count();
    let git_mutation_blocked_count = entries
        .iter()
        .filter(|entry| {
            entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
        })
        .count();
    let cleanup_delete_blocked_count = entries
        .iter()
        .filter(|entry| entry.cleanup_blocked && entry.delete_blocked)
        .count();

    let release_risk_snapshot_ready = source.operator_evidence_recording_boundary_readback_ready
        && source.evidence_recording_boundary_readback_visible
        && !source.evidence_recording_boundary_readback_persisted
        && !source.evidence_recorded
        && !source.evidence_recording_persisted
        && !source.evidence_receipt_persisted
        && !entries.is_empty()
        && entries.len() == source.boundary_entry_count
        && stable_snapshot_key_count == entries.len()
        && snapshot_route_count == entries.len()
        && snapshot_ready_count == entries.len()
        && classified_risk_count == entries.len()
        && release_blocked_count == entries.len()
        && rehearsal_candidate_count == entries.len()
        && pending_operator_decision_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_release_risk_snapshot",
        status: if release_risk_snapshot_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_GATE,
        schema_version: DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_evidence_recording_boundary_gate: source.gate,
        source_evidence_recording_boundary_ready: source
            .operator_evidence_recording_boundary_readback_ready,
        source_evidence_recording_boundary_visible: source
            .evidence_recording_boundary_readback_visible,
        source_evidence_recording_boundary_persisted: source
            .evidence_recording_boundary_readback_persisted,
        source_evidence_recorded: source.evidence_recorded,
        source_evidence_recording_persisted: source.evidence_recording_persisted,
        source_evidence_receipt_persisted: source.evidence_receipt_persisted,
        source_boundary_entry_count: source.boundary_entry_count,
        inventory_entry_count: source.inventory_entry_count,
        tracked_change_count: source.tracked_change_count,
        untracked_change_count: source.untracked_change_count,
        release_risk_snapshot_scope: dirty_worktree_release_boundary_release_risk_snapshot_scope(),
        risk_entry_count: entries.len(),
        stable_snapshot_key_count,
        snapshot_route_count,
        snapshot_ready_count,
        critical_risk_count,
        high_risk_count,
        medium_risk_count,
        high_or_critical_risk_count: critical_risk_count + high_risk_count,
        release_blocked_count,
        rehearsal_candidate_count,
        pending_operator_decision_count,
        evidence_recording_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        risk_snapshot_visible: release_risk_snapshot_ready,
        risk_snapshot_persisted: false,
        evidence_recorded: false,
        evidence_recording_persisted: false,
        evidence_receipt_persisted: false,
        approval_request_sent: false,
        approval_accepted: false,
        approval_recorded: false,
        approval_receipt_persisted: false,
        decision_recorded: false,
        decision_recording_persisted: false,
        decision_receipt_persisted: false,
        operator_packet_sent: false,
        operator_packet_persisted: false,
        readback_persisted: false,
        strategy_applied: false,
        release_cutover_allowed: false,
        git_add_allowed: false,
        git_index_mutated: false,
        git_commit_allowed: false,
        git_push_allowed: false,
        git_reset_allowed: false,
        git_checkout_allowed: false,
        git_revert_allowed: false,
        cleanup_allowed: false,
        delete_allowed: false,
        blocker_waiver_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        release_risk_snapshot_ready,
        entries,
        blockers: vec![
            "dirty_worktree_release_risk_snapshot_visible_only",
            "release_cutover_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "approval_acceptance_blocked",
            "decision_recording_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_release_risk_snapshot_scope()
-> DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotScope {
    DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotScope {
        snapshot_id: "dirty-worktree.release-boundary.release-risk-snapshot.v1",
        snapshot_route: "readback://release-boundary/dirty-worktree/release-risk-snapshot/v1",
        source_evidence_recording_boundary_route: "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-evidence-recording-boundary/v1",
        snapshot_mode: "fast_local_release_risk_snapshot_only",
        release_cutover_boundary: "blocked_dirty_worktree",
        git_mutation_boundary: "blocked",
        cleanup_boundary: "blocked",
        evidence_boundary: "blocked",
        live_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_release_risk_snapshot_entries(
    source: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorEvidenceRecordingBoundaryReadbackReport,
) -> Vec<DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotEntry {
                source_evidence_boundary_key: entry.evidence_boundary_key.clone(),
                source_evidence_boundary_route: entry.evidence_boundary_route.clone(),
                snapshot_key: snapshot_key(entry.group_type, entry.source_bucket),
                snapshot_route: snapshot_route(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                recommended_strategy: entry.recommended_strategy,
                release_risk_tier: release_risk_tier(entry.source_bucket),
                release_risk_reason: release_risk_reason(entry.source_bucket),
                release_blocker: release_blocker(entry.source_bucket),
                release_blocker_state: "blocked_dirty_worktree",
                clean_worktree_rehearsal_candidate: true,
                rehearsal_action: rehearsal_action(entry.source_bucket),
                decision_state: entry.decision_state,
                evidence_recording_state: entry.evidence_recording_state,
                evidence_persistence_state: entry.evidence_persistence_state,
                evidence_receipt_state: entry.evidence_receipt_state,
                approval_request_state: entry.approval_request_state,
                approval_acceptance_state: entry.approval_acceptance_state,
                approval_recording_state: entry.approval_recording_state,
                approval_receipt_state: entry.approval_receipt_state,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                evidence_recording_allowed: false,
                approval_acceptance_allowed: false,
                decision_recording_allowed: false,
                git_add_blocked: entry.git_add_blocked,
                git_index_mutation_blocked: entry.git_index_mutation_blocked,
                git_commit_blocked: entry.git_commit_blocked,
                git_push_blocked: entry.git_push_blocked,
                git_reset_blocked: entry.git_reset_blocked,
                git_checkout_blocked: entry.git_checkout_blocked,
                git_revert_blocked: entry.git_revert_blocked,
                cleanup_blocked: entry.cleanup_blocked,
                delete_blocked: entry.delete_blocked,
                release_cutover_allowed: false,
                canary_activation_allowed: false,
                live_execution_allowed: false,
            },
        )
        .collect()
}

fn snapshot_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.release_risk_snapshot.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn snapshot_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/release-risk-snapshot/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn release_risk_tier(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "critical",
        "codex-rs" | "plugins" | "scripts" | "hepta_systems_owned" => "high",
        "artifacts" | "docs" => "medium",
        _ => "high",
    }
}

fn release_risk_reason(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => {
            "cross-lane or unowned changes need owner attribution before release"
        }
        "codex-rs" => "runtime and crate changes require targeted Rust gates before release",
        "plugins" => "plugin surface changes can affect runtime/tool contribution boundaries",
        "scripts" => "automation and gate scripts can change release evidence",
        "hepta_systems_owned" => "owned Hepta systems changes still need freeze and rehearsal",
        "artifacts" => "generated or local artifacts need classification before release evidence",
        "docs" => {
            "architecture and evidence notes affect operator readback but not runtime execution"
        }
        _ => "dirty worktree bucket requires release-risk review",
    }
}

fn release_blocker(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "cross_lane_or_unowned_changes",
        "codex-rs" => "runtime_crate_changes",
        "plugins" => "plugin_surface_changes",
        "scripts" => "automation_gate_changes",
        "hepta_systems_owned" => "hepta_systems_owned_changes",
        "artifacts" => "generated_or_local_artifacts",
        "docs" => "documentation_evidence_changes",
        _ => "dirty_worktree_changes",
    }
}

fn rehearsal_action(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "test_only_owner_attribution_and_freeze_rehearsal",
        "codex-rs" => "test_only_targeted_rust_gate_rehearsal",
        "plugins" => "test_only_plugin_surface_rehearsal",
        "scripts" => "test_only_script_gate_rehearsal",
        "hepta_systems_owned" => "test_only_owned_lane_freeze_rehearsal",
        "artifacts" => "test_only_artifact_classification_rehearsal",
        "docs" => "test_only_doc_evidence_rehearsal",
        _ => "test_only_dirty_worktree_rehearsal",
    }
}

fn key_safe(value: &str) -> String {
    value.replace('-', "_")
}

fn route_safe(value: &str) -> String {
    value.replace('_', "-")
}

fn route_group_type(group_type: &str) -> &'static str {
    match group_type {
        "top_level" => "top-level",
        "scope" => "scope",
        _ => "unknown",
    }
}

impl DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotSideEffects {
    pub const fn none() -> Self {
        Self {
            snapshot_persisted: false,
            evidence_recorded: false,
            evidence_persisted: false,
            evidence_receipt_persisted: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            approval_receipt_persisted: false,
            decision_recorded: false,
            decision_recording_persisted: false,
            decision_receipt_persisted: false,
            packet_sent: false,
            packet_persisted: false,
            readback_persisted: false,
            git_add_performed: false,
            git_index_mutated: false,
            git_commit_created: false,
            git_push_performed: false,
            git_reset_performed: false,
            git_checkout_performed: false,
            git_revert_performed: false,
            cleanup_performed: false,
            unrelated_file_deleted: false,
            strategy_applied: false,
            blocker_waived: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            canary_activation_started: false,
            live_activation_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_risk_snapshot_collapses_dirty_groups_without_live_paths() {
        let report = dirty_worktree_release_boundary_release_risk_snapshot_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_evidence_recording_boundary_ready);
        assert!(report.source_evidence_recording_boundary_visible);
        assert!(!report.source_evidence_recording_boundary_persisted);
        assert_eq!(report.risk_entry_count, report.source_boundary_entry_count);
        assert_eq!(report.snapshot_ready_count, report.risk_entry_count);
        assert_eq!(
            report.critical_risk_count + report.high_risk_count + report.medium_risk_count,
            report.risk_entry_count
        );
        assert_eq!(
            report.high_or_critical_risk_count,
            report.critical_risk_count + report.high_risk_count
        );
        assert_eq!(report.release_blocked_count, report.risk_entry_count);
        assert_eq!(report.rehearsal_candidate_count, report.risk_entry_count);
        assert!(report.risk_snapshot_visible);
        assert!(!report.risk_snapshot_persisted);
        assert!(report.release_risk_snapshot_ready);
        assert_eq!(
            report.recommended_next_gate,
            "phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation"
        );
    }

    #[test]
    fn release_risk_snapshot_entries_are_queryable_rehearsal_candidates() {
        let report = dirty_worktree_release_boundary_release_risk_snapshot_report();

        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.source_bucket == "cross_lane_or_unowned"
                    && entry.release_risk_tier == "critical"
                    && entry.rehearsal_action
                        == "test_only_owner_attribution_and_freeze_rehearsal")
        );
        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.release_risk_tier == "high"
            && entry.snapshot_key == "dirty_worktree.release_risk_snapshot.top_level.codex_rs"
            && entry.snapshot_route
                == "readback://release-boundary/dirty-worktree/release-risk-snapshot/top-level/codex-rs"));
        assert!(report.entries.iter().all(|entry| entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.decision_state == "pending_operator_decision"
            && entry.release_blocker_state == "blocked_dirty_worktree"
            && entry.clean_worktree_rehearsal_candidate
            && entry.evidence_recording_state == "evidence_recording_blocked"
            && entry.evidence_persistence_state == "evidence_persistence_blocked"
            && entry.evidence_receipt_state == "evidence_receipt_blocked"
            && !entry.evidence_recording_allowed
            && !entry.approval_acceptance_allowed
            && !entry.decision_recording_allowed
            && entry.git_add_blocked
            && entry.git_index_mutation_blocked
            && entry.git_commit_blocked
            && entry.git_push_blocked
            && entry.git_reset_blocked
            && entry.git_checkout_blocked
            && entry.git_revert_blocked
            && entry.cleanup_blocked
            && entry.delete_blocked
            && !entry.release_cutover_allowed
            && !entry.canary_activation_allowed
            && !entry.live_execution_allowed));
    }

    #[test]
    fn release_risk_snapshot_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_release_risk_snapshot_report();

        assert!(!report.evidence_recorded);
        assert!(!report.approval_accepted);
        assert!(!report.decision_recorded);
        assert!(!report.operator_packet_sent);
        assert!(!report.readback_persisted);
        assert!(!report.strategy_applied);
        assert!(!report.git_add_allowed);
        assert!(!report.git_index_mutated);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotSideEffects::none()
        );
    }
}
