use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport;
use crate::dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_GATE: &str =
    "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_append_only_event_store_feature_gated_test_implementation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_rehearsal_gate: &'static str,
    pub source_rehearsal_ready: bool,
    pub source_test_only_rehearsal_visible: bool,
    pub source_test_only_rehearsal_persisted: bool,
    pub source_test_probe_executed: bool,
    pub source_rehearsal_entry_count: usize,
    pub source_rehearsal_ready_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub outcome_scope: DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackScope,
    pub outcome_entry_count: usize,
    pub stable_outcome_key_count: usize,
    pub outcome_route_count: usize,
    pub outcome_ready_count: usize,
    pub blocked_until_owner_attribution_count: usize,
    pub ready_for_targeted_rust_gate_rehearsal_count: usize,
    pub ready_for_plugin_surface_gate_rehearsal_count: usize,
    pub ready_for_script_syntax_gate_rehearsal_count: usize,
    pub ready_for_owned_lane_freeze_rehearsal_count: usize,
    pub ready_for_artifact_classification_rehearsal_count: usize,
    pub ready_for_doc_evidence_consistency_rehearsal_count: usize,
    pub release_blocked_count: usize,
    pub test_probe_execution_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
    pub outcome_readback_visible: bool,
    pub outcome_readback_persisted: bool,
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
    pub test_only_rehearsal_outcome_readback_ready: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackScope {
    pub outcome_readback_id: &'static str,
    pub outcome_readback_route: &'static str,
    pub source_rehearsal_route: &'static str,
    pub outcome_mode: &'static str,
    pub test_probe_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub approval_boundary: &'static str,
    pub decision_boundary: &'static str,
    pub live_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackEntry {
    pub source_rehearsal_key: String,
    pub source_rehearsal_route: String,
    pub outcome_key: String,
    pub outcome_route: String,
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
    pub source_required_local_gate: &'static str,
    pub source_rehearsal_probe: &'static str,
    pub source_convergence_state: &'static str,
    pub outcome_state: &'static str,
    pub outcome_action: &'static str,
    pub operator_action: &'static str,
    pub decision_state: &'static str,
    pub evidence_recording_state: &'static str,
    pub evidence_persistence_state: &'static str,
    pub evidence_receipt_state: &'static str,
    pub approval_request_state: &'static str,
    pub approval_acceptance_state: &'static str,
    pub approval_recording_state: &'static str,
    pub approval_receipt_state: &'static str,
    pub source_rehearsal_attached: bool,
    pub outcome_readback_visible: bool,
    pub outcome_readback_persisted: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub test_probe_executed: bool,
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
pub struct DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackSideEffects {
    pub outcome_readback_persisted: bool,
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

pub fn dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report()
-> DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport {
    let source =
        dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report();
    dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report_from_rehearsal(
        &source,
    )
}

pub fn dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report_from_rehearsal(
    source: &DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport,
) -> DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport {
    let entries =
        dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_entries(source);
    let stable_outcome_key_count = entries
        .iter()
        .map(|entry| entry.outcome_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let outcome_route_count = entries
        .iter()
        .map(|entry| entry.outcome_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let outcome_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.source_rehearsal_attached
                && entry.outcome_readback_visible
                && !entry.outcome_readback_persisted
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && !entry.test_probe_executed
                && entry.mutation_free
                && entry.source_release_blocker_state == "blocked_dirty_worktree"
                && !entry.source_required_local_gate.is_empty()
                && entry.source_convergence_state != "unknown"
                && entry.outcome_state != "unknown"
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
    let blocked_until_owner_attribution_count =
        count_outcome_state(&entries, "blocked_until_owner_attribution");
    let ready_for_targeted_rust_gate_rehearsal_count =
        count_outcome_state(&entries, "ready_for_targeted_rust_gate_rehearsal");
    let ready_for_plugin_surface_gate_rehearsal_count =
        count_outcome_state(&entries, "ready_for_plugin_surface_gate_rehearsal");
    let ready_for_script_syntax_gate_rehearsal_count =
        count_outcome_state(&entries, "ready_for_script_syntax_gate_rehearsal");
    let ready_for_owned_lane_freeze_rehearsal_count =
        count_outcome_state(&entries, "ready_for_owned_lane_freeze_rehearsal");
    let ready_for_artifact_classification_rehearsal_count =
        count_outcome_state(&entries, "ready_for_artifact_classification_rehearsal");
    let ready_for_doc_evidence_consistency_rehearsal_count =
        count_outcome_state(&entries, "ready_for_doc_evidence_consistency_rehearsal");
    let categorized_outcome_count = blocked_until_owner_attribution_count
        + ready_for_targeted_rust_gate_rehearsal_count
        + ready_for_plugin_surface_gate_rehearsal_count
        + ready_for_script_syntax_gate_rehearsal_count
        + ready_for_owned_lane_freeze_rehearsal_count
        + ready_for_artifact_classification_rehearsal_count
        + ready_for_doc_evidence_consistency_rehearsal_count;
    let release_blocked_count = entries
        .iter()
        .filter(|entry| entry.source_release_blocker_state == "blocked_dirty_worktree")
        .count();
    let test_probe_execution_blocked_count = entries
        .iter()
        .filter(|entry| !entry.test_probe_executed)
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

    let test_only_rehearsal_outcome_readback_ready = source
        .test_only_clean_worktree_strategy_rehearsal_ready
        && source.test_only_rehearsal_visible
        && !source.test_only_rehearsal_persisted
        && !source.test_probe_executed
        && !source.evidence_recorded
        && !source.evidence_recording_persisted
        && !source.evidence_receipt_persisted
        && !entries.is_empty()
        && entries.len() == source.rehearsal_entry_count
        && stable_outcome_key_count == entries.len()
        && outcome_route_count == entries.len()
        && outcome_ready_count == entries.len()
        && blocked_until_owner_attribution_count == 1
        && categorized_outcome_count == entries.len()
        && release_blocked_count == entries.len()
        && test_probe_execution_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback",
        status: if test_only_rehearsal_outcome_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_rehearsal_gate: source.gate,
        source_rehearsal_ready: source.test_only_clean_worktree_strategy_rehearsal_ready,
        source_test_only_rehearsal_visible: source.test_only_rehearsal_visible,
        source_test_only_rehearsal_persisted: source.test_only_rehearsal_persisted,
        source_test_probe_executed: source.test_probe_executed,
        source_rehearsal_entry_count: source.rehearsal_entry_count,
        source_rehearsal_ready_count: source.rehearsal_ready_count,
        inventory_entry_count: source.inventory_entry_count,
        tracked_change_count: source.tracked_change_count,
        untracked_change_count: source.untracked_change_count,
        outcome_scope: dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_scope(),
        outcome_entry_count: entries.len(),
        stable_outcome_key_count,
        outcome_route_count,
        outcome_ready_count,
        blocked_until_owner_attribution_count,
        ready_for_targeted_rust_gate_rehearsal_count,
        ready_for_plugin_surface_gate_rehearsal_count,
        ready_for_script_syntax_gate_rehearsal_count,
        ready_for_owned_lane_freeze_rehearsal_count,
        ready_for_artifact_classification_rehearsal_count,
        ready_for_doc_evidence_consistency_rehearsal_count,
        release_blocked_count,
        test_probe_execution_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        evidence_recording_blocked_count,
        approval_acceptance_blocked_count,
        decision_recording_blocked_count,
        outcome_readback_visible: test_only_rehearsal_outcome_readback_ready,
        outcome_readback_persisted: false,
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
        test_only_rehearsal_outcome_readback_ready,
        entries,
        blockers: vec![
            "outcome_readback_visible_only",
            "test_probe_execution_still_blocked",
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
            DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackSideEffects::none(
        ),
    }
}

pub const fn dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_scope()
-> DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackScope {
    DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackScope {
        outcome_readback_id: "dirty-worktree.release-boundary.test-only-rehearsal-outcome-readback.v1",
        outcome_readback_route: "readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/v1",
        source_rehearsal_route: "readback://release-boundary/dirty-worktree/test-only-clean-worktree-strategy-rehearsal/v1",
        outcome_mode: "visible_only_no_probe_no_git_mutation_no_cleanup_no_evidence_recording",
        test_probe_boundary: "blocked",
        git_mutation_boundary: "blocked",
        cleanup_boundary: "blocked",
        evidence_boundary: "blocked",
        approval_boundary: "blocked",
        decision_boundary: "blocked",
        live_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_entries(
    source: &DirtyWorktreeReleaseBoundaryTestOnlyCleanWorktreeStrategyRehearsalReport,
) -> Vec<DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackEntry {
                source_rehearsal_key: entry.rehearsal_key.clone(),
                source_rehearsal_route: entry.rehearsal_route.clone(),
                outcome_key: outcome_key(entry.group_type, entry.source_bucket),
                outcome_route: outcome_route(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                recommended_strategy: entry.recommended_strategy,
                source_release_risk_tier: entry.source_release_risk_tier,
                source_release_blocker: entry.source_release_blocker,
                source_release_blocker_state: entry.source_release_blocker_state,
                source_required_local_gate: entry.required_local_gate,
                source_rehearsal_probe: entry.rehearsal_probe,
                source_convergence_state: entry.convergence_state,
                outcome_state: outcome_state(entry.source_bucket),
                outcome_action: outcome_action(entry.source_bucket),
                operator_action: "review_outcome_readback_before_any_probe_or_git_mutation",
                decision_state: entry.decision_state,
                evidence_recording_state: entry.evidence_recording_state,
                evidence_persistence_state: entry.evidence_persistence_state,
                evidence_receipt_state: entry.evidence_receipt_state,
                approval_request_state: entry.approval_request_state,
                approval_acceptance_state: entry.approval_acceptance_state,
                approval_recording_state: entry.approval_recording_state,
                approval_receipt_state: entry.approval_receipt_state,
                source_rehearsal_attached: !entry.rehearsal_key.is_empty()
                    && !entry.rehearsal_route.is_empty(),
                outcome_readback_visible: true,
                outcome_readback_persisted: false,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                test_probe_executed: false,
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

fn count_outcome_state(
    entries: &[DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackEntry],
    outcome_state: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.outcome_state == outcome_state)
        .count()
}

fn outcome_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.test_only_rehearsal_outcome_readback.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn outcome_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn outcome_state(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "codex-rs" => "ready_for_targeted_rust_gate_rehearsal",
        "plugins" => "ready_for_plugin_surface_gate_rehearsal",
        "scripts" => "ready_for_script_syntax_gate_rehearsal",
        "hepta_systems_owned" => "ready_for_owned_lane_freeze_rehearsal",
        "artifacts" => "ready_for_artifact_classification_rehearsal",
        "docs" => "ready_for_doc_evidence_consistency_rehearsal",
        _ => "unknown",
    }
}

fn outcome_action(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "attribute_owner_before_any_clean_worktree_action",
        "codex-rs" => "run_targeted_rust_gate_probe_later_without_git_mutation",
        "plugins" => "run_plugin_surface_gate_probe_later_without_git_mutation",
        "scripts" => "run_script_syntax_gate_probe_later_without_git_mutation",
        "hepta_systems_owned" => "freeze_owned_lane_changes_later_without_git_mutation",
        "artifacts" => "classify_artifacts_later_without_delete",
        "docs" => "check_doc_evidence_consistency_later_without_persistence",
        _ => "review_dirty_worktree_bucket_later_without_git_mutation",
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

impl DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            outcome_readback_persisted: false,
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
    fn outcome_readback_summarizes_rehearsal_without_execution() {
        let report = dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_rehearsal_ready);
        assert!(report.source_test_only_rehearsal_visible);
        assert!(!report.source_test_only_rehearsal_persisted);
        assert!(!report.source_test_probe_executed);
        assert_eq!(
            report.outcome_entry_count,
            report.source_rehearsal_entry_count
        );
        assert_eq!(report.outcome_ready_count, report.outcome_entry_count);
        assert_eq!(report.stable_outcome_key_count, report.outcome_entry_count);
        assert_eq!(report.outcome_route_count, report.outcome_entry_count);
        assert_eq!(
            report.test_probe_execution_blocked_count,
            report.outcome_entry_count
        );
        assert_eq!(
            report.git_mutation_blocked_count,
            report.outcome_entry_count
        );
        assert_eq!(
            report.cleanup_delete_blocked_count,
            report.outcome_entry_count
        );
        assert!(report.outcome_readback_visible);
        assert!(!report.outcome_readback_persisted);
        assert!(!report.test_probe_executed);
        assert!(report.test_only_rehearsal_outcome_readback_ready);
        assert_eq!(
            report.recommended_next_gate,
            "temporal_lite_append_only_event_store_feature_gated_test_implementation"
        );
    }

    #[test]
    fn outcome_entries_preserve_bucket_specific_next_actions() {
        let report = dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report();

        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.source_bucket == "cross_lane_or_unowned"
                    && entry.outcome_state == "blocked_until_owner_attribution"
                    && entry.outcome_action == "attribute_owner_before_any_clean_worktree_action")
        );
        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.outcome_state == "ready_for_targeted_rust_gate_rehearsal"
            && entry.outcome_key
                == "dirty_worktree.test_only_rehearsal_outcome_readback.top_level.codex_rs"
            && entry.outcome_route
                == "readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/top-level/codex-rs"));
        assert!(
            report
                .entries
                .iter()
                .all(|entry| entry.source_rehearsal_attached
                    && entry.outcome_readback_visible
                    && !entry.outcome_readback_persisted
                    && entry.operator_visible
                    && entry.queryable
                    && entry.diffable
                    && !entry.test_probe_executed
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
    fn outcome_readback_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report();

        assert!(!report.outcome_readback_persisted);
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
            DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackSideEffects::none()
        );
    }
}
