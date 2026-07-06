use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport;
use crate::dirty_worktree_release_boundary_grouping_freeze_plan_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_GATE: &str =
    "dirty_worktree_release_boundary_grouping_freeze_operator_readback_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_grouping_freeze_operator_readback_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_RECOMMENDED_NEXT_GATE: &str = "phase14_dirty_worktree_release_boundary_actionable_clean_worktree_strategy_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_grouping_freeze_plan_gate: &'static str,
    pub source_grouping_freeze_plan_ready: bool,
    pub source_freeze_applied: bool,
    pub source_group_entry_count: usize,
    pub source_planned_not_applied_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub readback_scope: DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackScope,
    pub readback_entry_count: usize,
    pub stable_readback_key_count: usize,
    pub diff_key_count: usize,
    pub comparison_anchor_count: usize,
    pub planned_not_applied_readback_count: usize,
    pub unchanged_freeze_state_count: usize,
    pub unchanged_evidence_state_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_readback_ready: bool,
    pub freeze_applied: bool,
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
    pub entries: Vec<DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackScope {
    pub readback_id: &'static str,
    pub readback_route: &'static str,
    pub source_plan_route: &'static str,
    pub readback_mode: &'static str,
    pub diff_mode: &'static str,
    pub mutation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackEntry {
    pub source_group_key: &'static str,
    pub source_group_route: &'static str,
    pub readback_key: &'static str,
    pub readback_route: &'static str,
    pub diff_key: &'static str,
    pub comparison_anchor: &'static str,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub hepta_systems_owned_count: usize,
    pub cross_lane_or_unowned_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub operator_status: &'static str,
    pub previous_freeze_state: &'static str,
    pub current_freeze_state: &'static str,
    pub freeze_state_delta: &'static str,
    pub previous_evidence_state: &'static str,
    pub current_evidence_state: &'static str,
    pub evidence_state_delta: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub readback_ready: bool,
    pub freeze_applied: bool,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackSideEffects {
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub cleanup_performed: bool,
    pub unrelated_file_deleted: bool,
    pub freeze_applied: bool,
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

pub fn dirty_worktree_release_boundary_grouping_freeze_operator_readback_report()
-> DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport {
    let plan = dirty_worktree_release_boundary_grouping_freeze_plan_report();
    dirty_worktree_release_boundary_grouping_freeze_operator_readback_report_from_plan(&plan)
}

pub fn dirty_worktree_release_boundary_grouping_freeze_operator_readback_report_from_plan(
    plan: &DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport,
) -> DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport {
    let entries = dirty_worktree_release_boundary_grouping_freeze_operator_readback_entries(plan);
    let stable_readback_key_count = entries
        .iter()
        .map(|entry| entry.readback_key)
        .collect::<BTreeSet<_>>()
        .len();
    let diff_key_count = entries
        .iter()
        .map(|entry| entry.diff_key)
        .collect::<BTreeSet<_>>()
        .len();
    let comparison_anchor_count = entries
        .iter()
        .map(|entry| entry.comparison_anchor)
        .collect::<BTreeSet<_>>()
        .len();
    let planned_not_applied_readback_count = entries
        .iter()
        .filter(|entry| entry.current_freeze_state == "planned_not_applied")
        .count();
    let unchanged_freeze_state_count = entries
        .iter()
        .filter(|entry| entry.freeze_state_delta == "unchanged_planned_not_applied")
        .count();
    let unchanged_evidence_state_count = entries
        .iter()
        .filter(|entry| entry.evidence_state_delta == "unchanged_not_recorded")
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let operator_readback_ready = plan.grouping_freeze_plan_ready
        && !plan.freeze_applied
        && plan.group_entry_count == entries.len()
        && stable_readback_key_count == entries.len()
        && diff_key_count == entries.len()
        && comparison_anchor_count == entries.len()
        && planned_not_applied_readback_count == entries.len()
        && unchanged_freeze_state_count == entries.len()
        && unchanged_evidence_state_count == entries.len()
        && evidence_recorded_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.readback_ready
                && !entry.freeze_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        });

    DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_grouping_freeze_operator_readback",
        status: if operator_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_SCHEMA_VERSION,
        plugin_id: plan.plugin_id,
        source_grouping_freeze_plan_gate: plan.gate,
        source_grouping_freeze_plan_ready: plan.grouping_freeze_plan_ready,
        source_freeze_applied: plan.freeze_applied,
        source_group_entry_count: plan.group_entry_count,
        source_planned_not_applied_count: plan.planned_not_applied_count,
        inventory_entry_count: plan.inventory_entry_count,
        tracked_change_count: plan.tracked_change_count,
        untracked_change_count: plan.untracked_change_count,
        readback_scope: dirty_worktree_release_boundary_grouping_freeze_operator_readback_scope(),
        readback_entry_count: entries.len(),
        stable_readback_key_count,
        diff_key_count,
        comparison_anchor_count,
        planned_not_applied_readback_count,
        unchanged_freeze_state_count,
        unchanged_evidence_state_count,
        evidence_recorded_count,
        operator_readback_ready,
        freeze_applied: false,
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
            "dirty_worktree_release_boundary_operator_readback_not_clean",
            "freeze_application_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_grouping_freeze_operator_readback_scope()
-> DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackScope {
    DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackScope {
        readback_id: "dirty-worktree.release-boundary.grouping-freeze.operator-readback.v1",
        readback_route: "readback://release-boundary/dirty-worktree/grouping-freeze/operator-readback/v1",
        source_plan_route: "readback://release-boundary/dirty-worktree/grouping-freeze-plan/v1",
        readback_mode: "operator_readback_diff_only",
        diff_mode: "stable_key_state_delta",
        mutation_boundary: "closed",
    }
}

pub fn dirty_worktree_release_boundary_grouping_freeze_operator_readback_entries(
    plan: &DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport,
) -> Vec<DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackEntry> {
    plan.entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackEntry {
                source_group_key: entry.group_key,
                source_group_route: entry.group_route,
                readback_key: readback_key(entry.group_type, entry.source_bucket),
                readback_route: readback_route(entry.group_type, entry.source_bucket),
                diff_key: diff_key(entry.group_type, entry.source_bucket),
                comparison_anchor: comparison_anchor(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                hepta_systems_owned_count: entry.hepta_systems_owned_count,
                cross_lane_or_unowned_count: entry.cross_lane_or_unowned_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                operator_status: "blocked_pending_clean_worktree_strategy",
                previous_freeze_state: entry.freeze_state,
                current_freeze_state: entry.freeze_state,
                freeze_state_delta: "unchanged_planned_not_applied",
                previous_evidence_state: entry.evidence_state,
                current_evidence_state: entry.evidence_state,
                evidence_state_delta: "unchanged_not_recorded",
                operator_visible: true,
                queryable: true,
                diffable: true,
                readback_ready: true,
                freeze_applied: false,
                git_mutation_allowed: false,
                cleanup_allowed: false,
                evidence_recording_allowed: false,
                release_cutover_allowed: false,
                live_execution_allowed: false,
            },
        )
        .collect()
}

fn readback_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => "dirty_worktree.readback.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.readback.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.readback.top_level.docs",
        ("scope", "hepta_systems_owned") => "dirty_worktree.readback.scope.hepta_systems_owned",
        ("scope", "cross_lane_or_unowned") => "dirty_worktree.readback.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.readback.unknown",
    }
}

fn readback_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/top-level/docs"
        }
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/scope/cross-lane-or-unowned"
        }
        _ => "readback://release-boundary/dirty-worktree/grouping-freeze/operator/unknown",
    }
}

fn diff_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => "dirty_worktree.diff.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.diff.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.diff.top_level.docs",
        ("scope", "hepta_systems_owned") => "dirty_worktree.diff.scope.hepta_systems_owned",
        ("scope", "cross_lane_or_unowned") => "dirty_worktree.diff.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.diff.unknown",
    }
}

fn comparison_anchor(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => "dirty_worktree.anchor.top_level.scripts.v1",
        ("top_level", "codex-rs") => "dirty_worktree.anchor.top_level.codex_rs.v1",
        ("top_level", "docs") => "dirty_worktree.anchor.top_level.docs.v1",
        ("scope", "hepta_systems_owned") => "dirty_worktree.anchor.scope.hepta_systems_owned.v1",
        ("scope", "cross_lane_or_unowned") => {
            "dirty_worktree.anchor.scope.cross_lane_or_unowned.v1"
        }
        _ => "dirty_worktree.anchor.unknown.v1",
    }
}

impl DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackSideEffects {
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
            freeze_applied: false,
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
    fn operator_readback_is_ready_but_keeps_freeze_unapplied() {
        let report = dirty_worktree_release_boundary_grouping_freeze_operator_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_grouping_freeze_plan_ready);
        assert!(!report.source_freeze_applied);
        assert!(report.operator_readback_ready);
        assert_eq!(report.readback_entry_count, report.source_group_entry_count);
        assert_eq!(
            report.stable_readback_key_count,
            report.readback_entry_count
        );
        assert_eq!(report.diff_key_count, report.readback_entry_count);
        assert_eq!(report.comparison_anchor_count, report.readback_entry_count);
        assert_eq!(
            report.planned_not_applied_readback_count,
            report.readback_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(!report.freeze_applied);
    }

    #[test]
    fn operator_readback_entries_are_diffable_without_mutation() {
        let report = dirty_worktree_release_boundary_grouping_freeze_operator_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.readback_ready
                && entry.current_freeze_state == "planned_not_applied"
                && entry.freeze_state_delta == "unchanged_planned_not_applied"
                && entry.current_evidence_state == "not_recorded"
                && entry.evidence_state_delta == "unchanged_not_recorded"
                && !entry.freeze_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn operator_readback_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_grouping_freeze_operator_readback_report();

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
            DirtyWorktreeReleaseBoundaryGroupingFreezeOperatorReadbackSideEffects::none()
        );
    }
}
