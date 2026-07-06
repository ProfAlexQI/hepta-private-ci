use serde::Serialize;

use crate::ControlledCanaryReadinessPlanReport;
use crate::controlled_canary_readiness_plan_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_GATE: &str =
    "dirty_worktree_release_boundary_inventory_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_inventory_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_RECOMMENDED_NEXT_GATE: &str =
    "phase12_dirty_worktree_release_boundary_grouping_freeze_plan_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryInventoryReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_canary_gate: &'static str,
    pub source_canary_ready: bool,
    pub source_canary_activation_ready: bool,
    pub inventory_scope: DirtyWorktreeReleaseBoundaryInventoryScope,
    pub inventory_counts: DirtyWorktreeReleaseBoundaryInventoryCounts,
    pub dirty_worktree_release_boundary_open: bool,
    pub dirty_worktree_release_boundary_resolved: bool,
    pub release_boundary_inventory_ready: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
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
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryInventorySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryInventoryScope {
    pub inventory_id: &'static str,
    pub inventory_route: &'static str,
    pub source_command: &'static str,
    pub collection_mode: &'static str,
    pub mutation_boundary: &'static str,
    pub release_boundary_state: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryInventoryCounts {
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub staged_change_count: usize,
    pub unstaged_change_count: usize,
    pub modified_change_count: usize,
    pub deleted_change_count: usize,
    pub added_change_count: usize,
    pub renamed_change_count: usize,
    pub unmerged_change_count: usize,
    pub hepta_systems_owned_count: usize,
    pub cross_lane_or_unowned_count: usize,
    pub top_level_bucket_count: usize,
    pub sample_entry_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryInventorySideEffects {
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub unrelated_file_deleted: bool,
    pub cleanup_performed: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_inventory_report()
-> DirtyWorktreeReleaseBoundaryInventoryReport {
    let canary = controlled_canary_readiness_plan_report();
    let counts = dirty_worktree_release_boundary_inventory_fixture_counts();
    dirty_worktree_release_boundary_inventory_report_from_counts(&canary, counts)
}

pub fn dirty_worktree_release_boundary_inventory_report_from_counts(
    canary: &ControlledCanaryReadinessPlanReport,
    counts: DirtyWorktreeReleaseBoundaryInventoryCounts,
) -> DirtyWorktreeReleaseBoundaryInventoryReport {
    let dirty_worktree_release_boundary_open = counts.inventory_entry_count > 0;
    let count_invariants_ready = counts.inventory_entry_count
        == counts.tracked_change_count + counts.untracked_change_count
        && counts.inventory_entry_count
            == counts.hepta_systems_owned_count + counts.cross_lane_or_unowned_count
        && counts.sample_entry_count <= counts.inventory_entry_count;
    let release_boundary_inventory_ready = canary.controlled_canary_readiness_plan_ready
        && !canary.controlled_canary_activation_ready
        && dirty_worktree_release_boundary_open
        && count_invariants_ready
        && counts.top_level_bucket_count > 0;

    DirtyWorktreeReleaseBoundaryInventoryReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_inventory",
        status: if release_boundary_inventory_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_GATE,
        schema_version: DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_SCHEMA_VERSION,
        plugin_id: canary.plugin_id,
        source_canary_gate: canary.gate,
        source_canary_ready: canary.controlled_canary_readiness_plan_ready,
        source_canary_activation_ready: canary.controlled_canary_activation_ready,
        inventory_scope: dirty_worktree_release_boundary_inventory_scope(),
        inventory_counts: counts,
        dirty_worktree_release_boundary_open,
        dirty_worktree_release_boundary_resolved: !dirty_worktree_release_boundary_open,
        release_boundary_inventory_ready,
        operator_visible: true,
        queryable: true,
        diffable: true,
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
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "dirty_worktree_release_boundary_open",
            "release_cutover_blocked_until_inventory_grouping_freeze",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryInventorySideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_inventory_scope()
-> DirtyWorktreeReleaseBoundaryInventoryScope {
    DirtyWorktreeReleaseBoundaryInventoryScope {
        inventory_id: "dirty-worktree.release-boundary.inventory.v1",
        inventory_route: "readback://release-boundary/dirty-worktree/inventory/v1",
        source_command: "git status --porcelain",
        collection_mode: "read_only_inventory_no_git_mutation",
        mutation_boundary: "closed",
        release_boundary_state: "blocked_dirty_worktree",
    }
}

pub const fn dirty_worktree_release_boundary_inventory_fixture_counts()
-> DirtyWorktreeReleaseBoundaryInventoryCounts {
    DirtyWorktreeReleaseBoundaryInventoryCounts {
        inventory_entry_count: 6,
        tracked_change_count: 4,
        untracked_change_count: 2,
        staged_change_count: 1,
        unstaged_change_count: 3,
        modified_change_count: 3,
        deleted_change_count: 1,
        added_change_count: 1,
        renamed_change_count: 0,
        unmerged_change_count: 0,
        hepta_systems_owned_count: 4,
        cross_lane_or_unowned_count: 2,
        top_level_bucket_count: 3,
        sample_entry_count: 6,
    }
}

impl DirtyWorktreeReleaseBoundaryInventorySideEffects {
    pub const fn none() -> Self {
        Self {
            git_index_mutated: false,
            git_commit_created: false,
            git_push_performed: false,
            git_reset_performed: false,
            git_checkout_performed: false,
            git_revert_performed: false,
            unrelated_file_deleted: false,
            cleanup_performed: false,
            evidence_recorded: false,
            evidence_persisted: false,
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
    fn inventory_report_keeps_dirty_boundary_open_without_git_mutation() {
        let report = dirty_worktree_release_boundary_inventory_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_canary_ready);
        assert!(!report.source_canary_activation_ready);
        assert!(report.dirty_worktree_release_boundary_open);
        assert!(!report.dirty_worktree_release_boundary_resolved);
        assert!(report.release_boundary_inventory_ready);
        assert!(report.operator_visible);
        assert!(report.queryable);
        assert!(report.diffable);
        assert_eq!(report.inventory_counts.inventory_entry_count, 6);
        assert_eq!(
            report.inventory_counts.inventory_entry_count,
            report.inventory_counts.tracked_change_count
                + report.inventory_counts.untracked_change_count
        );
        assert_eq!(
            report.inventory_counts.inventory_entry_count,
            report.inventory_counts.hepta_systems_owned_count
                + report.inventory_counts.cross_lane_or_unowned_count
        );
    }

    #[test]
    fn inventory_blocks_release_and_cleanup_actions() {
        let report = dirty_worktree_release_boundary_inventory_report();

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
        assert!(!report.package_or_release_allowed);
        assert!(!report.public_ga_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn inventory_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_inventory_report();

        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryInventorySideEffects::none()
        );
    }
}
