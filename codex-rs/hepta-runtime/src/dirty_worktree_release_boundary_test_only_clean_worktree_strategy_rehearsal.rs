use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport;
use crate::dirty_worktree_release_boundary_release_risk_snapshot_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_GATE: &str =
    "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_RECOMMENDED_NEXT_GATE: &str = "phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_release_risk_snapshot_gate: &'static str,
    pub source_release_risk_snapshot_ready: bool,
    pub source_release_risk_snapshot_visible: bool,
    pub source_release_risk_snapshot_persisted: bool,
    pub source_evidence_recorded: bool,
    pub source_evidence_recording_persisted: bool,
    pub source_evidence_receipt_persisted: bool,
    pub source_risk_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub rehearsal_scope: DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalScope,
    pub rehearsal_entry_count: usize,
    pub stable_rehearsal_key_count: usize,
    pub rehearsal_route_count: usize,
    pub rehearsal_ready_count: usize,
    pub convergence_candidate_count: usize,
    pub owner_attribution_required_count: usize,
    pub runtime_gate_required_count: usize,
    pub plugin_gate_required_count: usize,
    pub script_gate_required_count: usize,
    pub owned_lane_freeze_required_count: usize,
    pub artifact_classification_required_count: usize,
    pub doc_evidence_required_count: usize,
    pub release_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
    pub test_only_rehearsal_visible: bool,
    pub test_only_rehearsal_persisted: bool,
    pub test_probe_executed: bool,
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
    pub test_only_clean_worktree_strategy_rehearsal_ready: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalScope {
    pub rehearsal_id: &'static str,
    pub rehearsal_route: &'static str,
    pub source_release_risk_snapshot_route: &'static str,
    pub rehearsal_mode: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub approval_boundary: &'static str,
    pub decision_boundary: &'static str,
    pub live_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalEntry {
    pub source_snapshot_key: String,
    pub source_snapshot_route: String,
    pub rehearsal_key: String,
    pub rehearsal_route: String,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub recommended_strategy: &'static str,
    pub source_release_risk_tier: &'static str,
    pub source_release_blocker: &'static str,
    pub source_release_blocker_state: &'static str,
    pub source_rehearsal_action: &'static str,
    pub rehearsal_probe: &'static str,
    pub required_local_gate: &'static str,
    pub convergence_state: &'static str,
    pub operator_action: &'static str,
    pub decision_state: &'static str,
    pub evidence_recording_state: &'static str,
    pub evidence_persistence_state: &'static str,
    pub evidence_receipt_state: &'static str,
    pub approval_request_state: &'static str,
    pub approval_acceptance_state: &'static str,
    pub approval_recording_state: &'static str,
    pub approval_receipt_state: &'static str,
    pub source_snapshot_attached: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub test_only_rehearsal_candidate: bool,
    pub test_only_rehearsal_visible: bool,
    pub test_only_rehearsal_executed: bool,
    pub mutation_free: bool,
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
pub struct DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalSideEffects {
    pub rehearsal_persisted: bool,
    pub test_probe_executed: bool,
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

pub fn dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report()
-> DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport {
    let source = dirty_worktree_release_boundary_release_risk_snapshot_report();
    dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report_from_release_risk_snapshot(&source)
}

pub fn dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report_from_release_risk_snapshot(
    source: &DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport,
) -> DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport {
    let entries =
        dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_entries(source);
    let stable_rehearsal_key_count = entries
        .iter()
        .map(|entry| entry.rehearsal_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let rehearsal_route_count = entries
        .iter()
        .map(|entry| entry.rehearsal_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let rehearsal_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.source_snapshot_attached
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.test_only_rehearsal_candidate
                && entry.test_only_rehearsal_visible
                && !entry.test_only_rehearsal_executed
                && entry.mutation_free
                && entry.source_release_blocker_state == "blocked_dirty_worktree"
                && entry.convergence_state != "unknown"
                && !entry.required_local_gate.is_empty()
                && entry.decision_state == "pending_operator_decision"
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
    let convergence_candidate_count = entries
        .iter()
        .filter(|entry| entry.test_only_rehearsal_candidate)
        .count();
    let owner_attribution_required_count = count_gate(&entries, "owner_attribution_freeze_gate");
    let runtime_gate_required_count = count_gate(&entries, "targeted_rust_gate");
    let plugin_gate_required_count = count_gate(&entries, "plugin_surface_gate");
    let script_gate_required_count = count_gate(&entries, "script_syntax_gate");
    let owned_lane_freeze_required_count = count_gate(&entries, "owned_lane_freeze_gate");
    let artifact_classification_required_count =
        count_gate(&entries, "artifact_classification_gate");
    let doc_evidence_required_count = count_gate(&entries, "doc_evidence_consistency_gate");
    let release_blocked_count = entries
        .iter()
        .filter(|entry| entry.source_release_blocker_state == "blocked_dirty_worktree")
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
    let evidence_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.evidence_recording_allowed)
        .count();
    let approval_acceptance_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_acceptance_allowed)
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_recording_allowed)
        .count();

    let test_only_clean_worktree_strategy_rehearsal_ready = source.release_risk_snapshot_ready
        && source.risk_snapshot_visible
        && !source.risk_snapshot_persisted
        && !source.evidence_recorded
        && !source.evidence_recording_persisted
        && !source.evidence_receipt_persisted
        && !entries.is_empty()
        && entries.len() == source.risk_entry_count
        && stable_rehearsal_key_count == entries.len()
        && rehearsal_route_count == entries.len()
        && rehearsal_ready_count == entries.len()
        && convergence_candidate_count == entries.len()
        && release_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal",
        status: if test_only_clean_worktree_strategy_rehearsal_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_release_risk_snapshot_gate: source.gate,
        source_release_risk_snapshot_ready: source.release_risk_snapshot_ready,
        source_release_risk_snapshot_visible: source.risk_snapshot_visible,
        source_release_risk_snapshot_persisted: source.risk_snapshot_persisted,
        source_evidence_recorded: source.evidence_recorded,
        source_evidence_recording_persisted: source.evidence_recording_persisted,
        source_evidence_receipt_persisted: source.evidence_receipt_persisted,
        source_risk_entry_count: source.risk_entry_count,
        inventory_entry_count: source.inventory_entry_count,
        tracked_change_count: source.tracked_change_count,
        untracked_change_count: source.untracked_change_count,
        rehearsal_scope:
            dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_scope(),
        rehearsal_entry_count: entries.len(),
        stable_rehearsal_key_count,
        rehearsal_route_count,
        rehearsal_ready_count,
        convergence_candidate_count,
        owner_attribution_required_count,
        runtime_gate_required_count,
        plugin_gate_required_count,
        script_gate_required_count,
        owned_lane_freeze_required_count,
        artifact_classification_required_count,
        doc_evidence_required_count,
        release_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        evidence_recording_blocked_count,
        approval_acceptance_blocked_count,
        decision_recording_blocked_count,
        test_only_rehearsal_visible: test_only_clean_worktree_strategy_rehearsal_ready,
        test_only_rehearsal_persisted: false,
        test_probe_executed: false,
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
        test_only_clean_worktree_strategy_rehearsal_ready,
        entries,
        blockers: vec![
            "test_only_rehearsal_visible_only",
            "test_probe_execution_blocked",
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
            DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_scope()
-> DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalScope {
    DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalScope {
        rehearsal_id: "dirty-worktree.release-boundary.test-only-clean-worktree-strategy-rehearsal.v1",
        rehearsal_route: "readback://release-boundary/dirty-worktree/test-only-clean-worktree-strategy-rehearsal/v1",
        source_release_risk_snapshot_route: "readback://release-boundary/dirty-worktree/release-risk-snapshot/v1",
        rehearsal_mode: "test_only_no_git_mutation_no_cleanup_no_evidence_recording",
        git_mutation_boundary: "blocked",
        cleanup_boundary: "blocked",
        evidence_boundary: "blocked",
        approval_boundary: "blocked",
        decision_boundary: "blocked",
        live_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_entries(
    source: &DirtyWorktreeReleaseBoundaryReleaseRiskSnapshotReport,
) -> Vec<DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalEntry {
                source_snapshot_key: entry.snapshot_key.clone(),
                source_snapshot_route: entry.snapshot_route.clone(),
                rehearsal_key: rehearsal_key(entry.group_type, entry.source_bucket),
                rehearsal_route: rehearsal_route(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                recommended_strategy: entry.recommended_strategy,
                source_release_risk_tier: entry.release_risk_tier,
                source_release_blocker: entry.release_blocker,
                source_release_blocker_state: entry.release_blocker_state,
                source_rehearsal_action: entry.rehearsal_action,
                rehearsal_probe: rehearsal_probe(entry.source_bucket),
                required_local_gate: required_local_gate(entry.source_bucket),
                convergence_state: convergence_state(entry.source_bucket),
                operator_action: "review_test_only_rehearsal_before_clean_worktree_strategy",
                decision_state: entry.decision_state,
                evidence_recording_state: entry.evidence_recording_state,
                evidence_persistence_state: entry.evidence_persistence_state,
                evidence_receipt_state: entry.evidence_receipt_state,
                approval_request_state: entry.approval_request_state,
                approval_acceptance_state: entry.approval_acceptance_state,
                approval_recording_state: entry.approval_recording_state,
                approval_receipt_state: entry.approval_receipt_state,
                source_snapshot_attached: !entry.snapshot_key.is_empty()
                    && !entry.snapshot_route.is_empty(),
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                test_only_rehearsal_candidate: entry.clean_worktree_rehearsal_candidate,
                test_only_rehearsal_visible: true,
                test_only_rehearsal_executed: false,
                mutation_free: true,
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

fn count_gate(
    entries: &[DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalEntry],
    gate: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.required_local_gate == gate)
        .count()
}

fn rehearsal_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.test_only_clean_worktree_strategy_rehearsal.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn rehearsal_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/test-only-clean-worktree-strategy-rehearsal/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn rehearsal_probe(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "owner_attribution_and_freeze_probe",
        "codex-rs" => "targeted_rust_gate_probe",
        "plugins" => "plugin_surface_gate_probe",
        "scripts" => "script_syntax_and_gate_probe",
        "hepta_systems_owned" => "owned_lane_freeze_probe",
        "artifacts" => "artifact_classification_probe",
        "docs" => "doc_evidence_consistency_probe",
        _ => "general_dirty_worktree_review_probe",
    }
}

fn required_local_gate(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "owner_attribution_freeze_gate",
        "codex-rs" => "targeted_rust_gate",
        "plugins" => "plugin_surface_gate",
        "scripts" => "script_syntax_gate",
        "hepta_systems_owned" => "owned_lane_freeze_gate",
        "artifacts" => "artifact_classification_gate",
        "docs" => "doc_evidence_consistency_gate",
        _ => "general_dirty_worktree_review_gate",
    }
}

fn convergence_state(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "codex-rs" => "candidate_after_targeted_rust_gate",
        "plugins" => "candidate_after_plugin_surface_gate",
        "scripts" => "candidate_after_script_gate",
        "hepta_systems_owned" => "candidate_after_owned_lane_freeze",
        "artifacts" => "candidate_after_artifact_classification",
        "docs" => "candidate_after_doc_evidence_check",
        _ => "candidate_after_general_dirty_worktree_review",
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

impl DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalSideEffects {
    pub const fn none() -> Self {
        Self {
            rehearsal_persisted: false,
            test_probe_executed: false,
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
    fn test_only_rehearsal_collapses_snapshot_without_mutation() {
        let report =
            dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_release_risk_snapshot_ready);
        assert!(report.source_release_risk_snapshot_visible);
        assert!(!report.source_release_risk_snapshot_persisted);
        assert_eq!(report.rehearsal_entry_count, report.source_risk_entry_count);
        assert_eq!(report.rehearsal_ready_count, report.rehearsal_entry_count);
        assert_eq!(
            report.convergence_candidate_count,
            report.rehearsal_entry_count
        );
        assert_eq!(report.release_blocked_count, report.rehearsal_entry_count);
        assert_eq!(
            report.git_mutation_blocked_count,
            report.rehearsal_entry_count
        );
        assert_eq!(
            report.cleanup_delete_blocked_count,
            report.rehearsal_entry_count
        );
        assert!(report.test_only_rehearsal_visible);
        assert!(!report.test_only_rehearsal_persisted);
        assert!(!report.test_probe_executed);
        assert!(report.test_only_clean_worktree_strategy_rehearsal_ready);
        assert_eq!(
            report.recommended_next_gate,
            "phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation"
        );
    }

    #[test]
    fn rehearsal_entries_are_queryable_test_only_candidates() {
        let report =
            dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report();

        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.source_bucket == "cross_lane_or_unowned"
                    && entry.required_local_gate == "owner_attribution_freeze_gate"
                    && entry.convergence_state == "blocked_until_owner_attribution")
        );
        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.required_local_gate == "targeted_rust_gate"
            && entry.rehearsal_key
                == "dirty_worktree.test_only_clean_worktree_strategy_rehearsal.top_level.codex_rs"
            && entry.rehearsal_route
                == "readback://release-boundary/dirty-worktree/test-only-clean-worktree-strategy-rehearsal/top-level/codex-rs"));
        assert!(
            report
                .entries
                .iter()
                .all(|entry| entry.source_snapshot_attached
                    && entry.operator_visible
                    && entry.queryable
                    && entry.diffable
                    && entry.test_only_rehearsal_candidate
                    && entry.test_only_rehearsal_visible
                    && !entry.test_only_rehearsal_executed
                    && entry.mutation_free
                    && entry.source_release_blocker_state == "blocked_dirty_worktree"
                    && entry.decision_state == "pending_operator_decision"
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
                    && !entry.live_execution_allowed)
        );
    }

    #[test]
    fn rehearsal_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report();

        assert!(!report.test_probe_executed);
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
            DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalSideEffects::none()
        );
    }
}
