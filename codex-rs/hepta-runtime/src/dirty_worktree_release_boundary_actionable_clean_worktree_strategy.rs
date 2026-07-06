use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport;
use crate::dirty_worktree_release_boundary_grouping_freeze_operator_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_GATE: &str =
    "dirty_worktree_release_boundary_actionable_clean_worktree_strategy_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_actionable_clean_worktree_strategy_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_RECOMMENDED_NEXT_GATE: &str =
    "phase15_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_readback_gate: &'static str,
    pub source_operator_readback_ready: bool,
    pub source_freeze_applied: bool,
    pub source_readback_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub strategy_scope: DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyScope,
    pub strategy_entry_count: usize,
    pub stable_strategy_key_count: usize,
    pub strategy_route_count: usize,
    pub ready_strategy_count: usize,
    pub operator_decision_required_count: usize,
    pub no_git_mutation_strategy_count: usize,
    pub hepta_systems_strategy_count: usize,
    pub cross_lane_strategy_count: usize,
    pub mixed_lane_strategy_count: usize,
    pub evidence_recorded_count: usize,
    pub strategy_ready: bool,
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
    pub evidence_recording_allowed: bool,
    pub evidence_persistence_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyScope {
    pub strategy_id: &'static str,
    pub strategy_route: &'static str,
    pub source_readback_route: &'static str,
    pub strategy_mode: &'static str,
    pub action_mode: &'static str,
    pub mutation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyEntry {
    pub source_readback_key: &'static str,
    pub source_readback_route: &'static str,
    pub source_diff_key: &'static str,
    pub strategy_key: &'static str,
    pub strategy_route: &'static str,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub recommended_strategy: &'static str,
    pub operator_action: &'static str,
    pub evidence_requirement: &'static str,
    pub execution_mode: &'static str,
    pub decision_state: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub strategy_ready: bool,
    pub operator_decision_required: bool,
    pub strategy_applied: bool,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategySideEffects {
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub cleanup_performed: bool,
    pub unrelated_file_deleted: bool,
    pub strategy_applied: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub approval_accepted: bool,
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report()
-> DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport {
    let readback = dirty_worktree_release_boundary_grouping_freeze_operator_readback_report();
    dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report_from_readback(
        &readback,
    )
}

pub fn dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report_from_readback(
    readback: &DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport,
) -> DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport {
    let entries =
        dirty_worktree_release_boundary_actionable_clean_worktree_strategy_entries(readback);
    let stable_strategy_key_count = entries
        .iter()
        .map(|entry| entry.strategy_key)
        .collect::<BTreeSet<_>>()
        .len();
    let strategy_route_count = entries
        .iter()
        .map(|entry| entry.strategy_route)
        .collect::<BTreeSet<_>>()
        .len();
    let ready_strategy_count = entries.iter().filter(|entry| entry.strategy_ready).count();
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let no_git_mutation_strategy_count = entries
        .iter()
        .filter(|entry| !entry.git_mutation_allowed)
        .count();
    let hepta_systems_strategy_count = entries
        .iter()
        .filter(|entry| entry.review_lane == "hepta-systems")
        .count();
    let cross_lane_strategy_count = entries
        .iter()
        .filter(|entry| {
            entry.review_lane == "cross-lane-review"
                || entry.review_lane == "external-or-cross-lane"
        })
        .count();
    let mixed_lane_strategy_count = entries
        .iter()
        .filter(|entry| {
            entry.review_lane == "mixed" || entry.review_lane == "mixed-hepta-and-cross-lane"
        })
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let strategy_ready = readback.operator_readback_ready
        && !readback.freeze_applied
        && entries.len() == readback.readback_entry_count
        && stable_strategy_key_count == entries.len()
        && strategy_route_count == entries.len()
        && ready_strategy_count == entries.len()
        && operator_decision_required_count == entries.len()
        && no_git_mutation_strategy_count == entries.len()
        && evidence_recorded_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.strategy_ready
                && entry.operator_decision_required
                && !entry.strategy_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.delete_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        });

    DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_actionable_clean_worktree_strategy",
        status: if strategy_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_SCHEMA_VERSION,
        plugin_id: readback.plugin_id,
        source_operator_readback_gate: readback.gate,
        source_operator_readback_ready: readback.operator_readback_ready,
        source_freeze_applied: readback.freeze_applied,
        source_readback_entry_count: readback.readback_entry_count,
        inventory_entry_count: readback.inventory_entry_count,
        tracked_change_count: readback.tracked_change_count,
        untracked_change_count: readback.untracked_change_count,
        strategy_scope: dirty_worktree_release_boundary_actionable_clean_worktree_strategy_scope(),
        strategy_entry_count: entries.len(),
        stable_strategy_key_count,
        strategy_route_count,
        ready_strategy_count,
        operator_decision_required_count,
        no_git_mutation_strategy_count,
        hepta_systems_strategy_count,
        cross_lane_strategy_count,
        mixed_lane_strategy_count,
        evidence_recorded_count,
        strategy_ready,
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
        evidence_recording_allowed: false,
        evidence_persistence_allowed: false,
        approval_acceptance_allowed: false,
        blocker_waiver_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        entries,
        blockers: vec![
            "clean_worktree_strategy_requires_operator_decision",
            "strategy_application_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategySideEffects::none(
        ),
    }
}

pub const fn dirty_worktree_release_boundary_actionable_clean_worktree_strategy_scope()
-> DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyScope {
    DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyScope {
        strategy_id: "dirty-worktree.release-boundary.actionable-clean-worktree-strategy.v1",
        strategy_route: "readback://release-boundary/dirty-worktree/actionable-clean-worktree-strategy/v1",
        source_readback_route: "readback://release-boundary/dirty-worktree/grouping-freeze/operator-readback/v1",
        strategy_mode: "operator_strategy_only",
        action_mode: "no_git_mutation_no_cleanup_no_evidence_recording",
        mutation_boundary: "closed",
    }
}

pub fn dirty_worktree_release_boundary_actionable_clean_worktree_strategy_entries(
    readback: &DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport,
) -> Vec<DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyEntry> {
    readback
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyEntry {
                source_readback_key: entry.readback_key,
                source_readback_route: entry.readback_route,
                source_diff_key: entry.diff_key,
                strategy_key: strategy_key(entry.group_type, entry.source_bucket),
                strategy_route: strategy_route(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                recommended_strategy: recommended_strategy(entry.review_lane),
                operator_action: operator_action(entry.review_lane),
                evidence_requirement: "clean_worktree_decision_record_required_before_release",
                execution_mode: "strategy_only_no_git_mutation",
                decision_state: "pending_operator_decision",
                operator_visible: true,
                queryable: true,
                diffable: true,
                strategy_ready: true,
                operator_decision_required: true,
                strategy_applied: false,
                git_mutation_allowed: false,
                cleanup_allowed: false,
                delete_allowed: false,
                evidence_recording_allowed: false,
                release_cutover_allowed: false,
                live_execution_allowed: false,
            },
        )
        .collect()
}

fn strategy_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.strategy.top_level.artifacts",
        ("top_level", "scripts") => "dirty_worktree.strategy.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.strategy.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.strategy.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.strategy.top_level.plugins",
        ("scope", "hepta_systems_owned") => "dirty_worktree.strategy.scope.hepta_systems_owned",
        ("scope", "cross_lane_or_unowned") => "dirty_worktree.strategy.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.strategy.unknown",
    }
}

fn strategy_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "readback://release-boundary/dirty-worktree/strategy/top-level/artifacts"
        }
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/strategy/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/strategy/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "readback://release-boundary/dirty-worktree/strategy/top-level/docs"
        }
        ("top_level", "plugins") => {
            "readback://release-boundary/dirty-worktree/strategy/top-level/plugins"
        }
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/strategy/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "readback://release-boundary/dirty-worktree/strategy/scope/cross-lane-or-unowned"
        }
        _ => "readback://release-boundary/dirty-worktree/strategy/unknown",
    }
}

fn recommended_strategy(review_lane: &str) -> &'static str {
    match review_lane {
        "hepta-systems" => "hepta_systems_owned_batch_review",
        "cross-lane-review" | "external-or-cross-lane" => "cross_lane_owner_review_required",
        "mixed" | "mixed-hepta-and-cross-lane" => "split_owned_and_cross_lane_review",
        _ => "operator_classification_required",
    }
}

fn operator_action(review_lane: &str) -> &'static str {
    match review_lane {
        "hepta-systems" => "prepare_hepta_systems_clean_plan_for_operator_review",
        "cross-lane-review" | "external-or-cross-lane" => {
            "request_owner_classification_before_cleanup"
        }
        "mixed" | "mixed-hepta-and-cross-lane" => "split_group_into_owned_and_cross_lane_subsets",
        _ => "classify_group_before_any_cleanup",
    }
}

impl DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategySideEffects {
    pub const fn none() -> Self {
        Self {
            git_index_mutated: false,
            git_commit_created: false,
            git_push_performed: false,
            git_reset_performed: false,
            git_checkout_performed: false,
            git_revert_performed: false,
            cleanup_performed: false,
            unrelated_file_deleted: false,
            strategy_applied: false,
            evidence_recorded: false,
            evidence_persisted: false,
            approval_accepted: false,
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
    fn clean_worktree_strategy_is_ready_but_not_applied() {
        let report = dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_readback_ready);
        assert!(!report.source_freeze_applied);
        assert!(report.strategy_ready);
        assert_eq!(
            report.strategy_entry_count,
            report.source_readback_entry_count
        );
        assert_eq!(
            report.stable_strategy_key_count,
            report.strategy_entry_count
        );
        assert_eq!(report.strategy_route_count, report.strategy_entry_count);
        assert_eq!(report.ready_strategy_count, report.strategy_entry_count);
        assert_eq!(
            report.operator_decision_required_count,
            report.strategy_entry_count
        );
        assert_eq!(
            report.no_git_mutation_strategy_count,
            report.strategy_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(!report.strategy_applied);
    }

    #[test]
    fn clean_worktree_strategy_entries_are_actionable_without_mutation() {
        let report = dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report();

        assert!(report.entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.strategy_ready
                && entry.operator_decision_required
                && entry.decision_state == "pending_operator_decision"
                && entry.execution_mode == "strategy_only_no_git_mutation"
                && !entry.strategy_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.delete_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn clean_worktree_strategy_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report();

        assert!(!report.release_cutover_allowed);
        assert!(!report.git_add_allowed);
        assert!(!report.git_index_mutated);
        assert!(!report.git_commit_allowed);
        assert!(!report.git_push_allowed);
        assert!(!report.git_reset_allowed);
        assert!(!report.git_checkout_allowed);
        assert!(!report.git_revert_allowed);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persistence_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.blocker_waiver_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.public_ga_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategySideEffects::none()
        );
    }
}
