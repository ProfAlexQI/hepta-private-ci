use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryInventoryReport;
use crate::dirty_worktree_release_boundary_inventory_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_GATE: &str =
    "dirty_worktree_release_boundary_grouping_freeze_plan_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_SCHEMA_VERSION: &str =
    "dirty_worktree_release_boundary_grouping_freeze_plan_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_RECOMMENDED_NEXT_GATE: &str = "phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_inventory_gate: &'static str,
    pub source_inventory_ready: bool,
    pub source_dirty_worktree_release_boundary_open: bool,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub grouping_scope: DirtyWorktreeReleaseBoundaryGroupingFreezePlanScope,
    pub top_level_group_count: usize,
    pub scope_group_count: usize,
    pub group_entry_count: usize,
    pub freeze_plan_ready_count: usize,
    pub planned_not_applied_count: usize,
    pub release_evidence_bucket_count: usize,
    pub evidence_recorded_count: usize,
    pub grouping_freeze_plan_ready: bool,
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
    pub entries: Vec<DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryGroupingFreezePlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezePlanScope {
    pub plan_id: &'static str,
    pub plan_route: &'static str,
    pub source_inventory_route: &'static str,
    pub grouping_mode: &'static str,
    pub freeze_mode: &'static str,
    pub mutation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry {
    pub group_type: &'static str,
    pub group_key: &'static str,
    pub group_route: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub hepta_systems_owned_count: usize,
    pub cross_lane_or_unowned_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub freeze_state: &'static str,
    pub evidence_state: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub freeze_plan_ready: bool,
    pub freeze_applied: bool,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryGroupingFreezePlanSideEffects {
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

pub fn dirty_worktree_release_boundary_grouping_freeze_plan_report()
-> DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport {
    let inventory = dirty_worktree_release_boundary_inventory_report();
    dirty_worktree_release_boundary_grouping_freeze_plan_report_from_inventory(&inventory)
}

pub fn dirty_worktree_release_boundary_grouping_freeze_plan_report_from_inventory(
    inventory: &DirtyWorktreeReleaseBoundaryInventoryReport,
) -> DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport {
    let entries = dirty_worktree_release_boundary_grouping_freeze_plan_fixture_entries();
    dirty_worktree_release_boundary_grouping_freeze_plan_report_from_inventory_and_entries(
        inventory, entries,
    )
}

pub fn dirty_worktree_release_boundary_grouping_freeze_plan_report_from_inventory_and_entries(
    inventory: &DirtyWorktreeReleaseBoundaryInventoryReport,
    entries: Vec<DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry>,
) -> DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport {
    let freeze_plan_ready_count = entries
        .iter()
        .filter(|entry| entry.freeze_plan_ready)
        .count();
    let planned_not_applied_count = entries
        .iter()
        .filter(|entry| entry.freeze_state == "planned_not_applied")
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let top_level_group_count = entries
        .iter()
        .filter(|entry| entry.group_type == "top_level")
        .count();
    let scope_group_count = entries
        .iter()
        .filter(|entry| entry.group_type == "scope")
        .count();
    let grouping_freeze_plan_ready = inventory.release_boundary_inventory_ready
        && inventory.dirty_worktree_release_boundary_open
        && inventory.inventory_counts.inventory_entry_count > 0
        && entries.len() == top_level_group_count + scope_group_count
        && freeze_plan_ready_count == entries.len()
        && planned_not_applied_count == entries.len()
        && evidence_recorded_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && !entry.freeze_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
        });

    DirtyWorktreeReleaseBoundaryGroupingFreezePlanReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_grouping_freeze_plan",
        status: if grouping_freeze_plan_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_GATE,
        schema_version: DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_SCHEMA_VERSION,
        plugin_id: inventory.plugin_id,
        source_inventory_gate: inventory.gate,
        source_inventory_ready: inventory.release_boundary_inventory_ready,
        source_dirty_worktree_release_boundary_open: inventory.dirty_worktree_release_boundary_open,
        inventory_entry_count: inventory.inventory_counts.inventory_entry_count,
        tracked_change_count: inventory.inventory_counts.tracked_change_count,
        untracked_change_count: inventory.inventory_counts.untracked_change_count,
        grouping_scope: dirty_worktree_release_boundary_grouping_freeze_plan_scope(),
        top_level_group_count,
        scope_group_count,
        group_entry_count: entries.len(),
        freeze_plan_ready_count,
        planned_not_applied_count,
        release_evidence_bucket_count: entries.len(),
        evidence_recorded_count,
        grouping_freeze_plan_ready,
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
            "dirty_worktree_release_boundary_grouping_freeze_plan_not_applied",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryGroupingFreezePlanSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_grouping_freeze_plan_scope()
-> DirtyWorktreeReleaseBoundaryGroupingFreezePlanScope {
    DirtyWorktreeReleaseBoundaryGroupingFreezePlanScope {
        plan_id: "dirty-worktree.release-boundary.grouping-freeze-plan.v1",
        plan_route: "readback://release-boundary/dirty-worktree/grouping-freeze-plan/v1",
        source_inventory_route: "readback://release-boundary/dirty-worktree/inventory/v1",
        grouping_mode: "top_level_and_scope_bucket",
        freeze_mode: "plan_only_not_applied",
        mutation_boundary: "closed",
    }
}

pub fn dirty_worktree_release_boundary_grouping_freeze_plan_fixture_entries()
-> Vec<DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry> {
    vec![
        fixture_entry("top_level", "scripts", 3, 1, 2, 3, 0),
        fixture_entry("top_level", "codex-rs", 2, 2, 0, 1, 1),
        fixture_entry("top_level", "docs", 1, 1, 0, 1, 0),
        fixture_entry("scope", "hepta_systems_owned", 4, 3, 1, 4, 0),
        fixture_entry("scope", "cross_lane_or_unowned", 2, 1, 1, 0, 2),
    ]
}

fn fixture_entry(
    group_type: &'static str,
    source_bucket: &'static str,
    source_entry_count: usize,
    tracked_count: usize,
    untracked_count: usize,
    hepta_systems_owned_count: usize,
    cross_lane_or_unowned_count: usize,
) -> DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry {
    DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry {
        group_type,
        group_key: group_key(group_type, source_bucket),
        group_route: group_route(group_type, source_bucket),
        source_bucket,
        source_entry_count,
        tracked_count,
        untracked_count,
        hepta_systems_owned_count,
        cross_lane_or_unowned_count,
        owner_hint: owner_hint(hepta_systems_owned_count, cross_lane_or_unowned_count),
        review_lane: review_lane(hepta_systems_owned_count, cross_lane_or_unowned_count),
        freeze_state: "planned_not_applied",
        evidence_state: "not_recorded",
        operator_visible: true,
        queryable: true,
        diffable: true,
        freeze_plan_ready: true,
        freeze_applied: false,
        git_mutation_allowed: false,
        cleanup_allowed: false,
        evidence_recording_allowed: false,
        release_cutover_allowed: false,
    }
}

fn group_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => "dirty_worktree.group.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.group.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.group.top_level.docs",
        ("scope", "hepta_systems_owned") => "dirty_worktree.group.scope.hepta_systems_owned",
        ("scope", "cross_lane_or_unowned") => "dirty_worktree.group.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.group.unknown",
    }
}

fn group_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/group/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/group/top-level/codex-rs"
        }
        ("top_level", "docs") => "readback://release-boundary/dirty-worktree/group/top-level/docs",
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/group/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "readback://release-boundary/dirty-worktree/group/scope/cross-lane-or-unowned"
        }
        _ => "readback://release-boundary/dirty-worktree/group/unknown",
    }
}

fn owner_hint(
    hepta_systems_owned_count: usize,
    cross_lane_or_unowned_count: usize,
) -> &'static str {
    if hepta_systems_owned_count >= cross_lane_or_unowned_count {
        "hepta-systems"
    } else {
        "cross-lane-review"
    }
}

fn review_lane(
    hepta_systems_owned_count: usize,
    cross_lane_or_unowned_count: usize,
) -> &'static str {
    if cross_lane_or_unowned_count > 0 && hepta_systems_owned_count == 0 {
        "external-or-cross-lane"
    } else if cross_lane_or_unowned_count > 0 {
        "mixed"
    } else {
        "hepta-systems"
    }
}

impl DirtyWorktreeReleaseBoundaryGroupingFreezePlanSideEffects {
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
    fn grouping_freeze_plan_is_ready_but_not_applied() {
        let report = dirty_worktree_release_boundary_grouping_freeze_plan_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_inventory_ready);
        assert!(report.source_dirty_worktree_release_boundary_open);
        assert!(report.grouping_freeze_plan_ready);
        assert_eq!(report.group_entry_count, 5);
        assert_eq!(report.top_level_group_count, 3);
        assert_eq!(report.scope_group_count, 2);
        assert_eq!(report.freeze_plan_ready_count, 5);
        assert_eq!(report.planned_not_applied_count, 5);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(!report.freeze_applied);
    }

    #[test]
    fn grouping_freeze_entries_keep_release_actions_closed() {
        let report = dirty_worktree_release_boundary_grouping_freeze_plan_report();

        assert!(report.entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.freeze_plan_ready
                && entry.freeze_state == "planned_not_applied"
                && entry.evidence_state == "not_recorded"
                && !entry.freeze_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
        }));
    }

    #[test]
    fn grouping_freeze_plan_side_effects_are_closed() {
        let report = dirty_worktree_release_boundary_grouping_freeze_plan_report();

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
            DirtyWorktreeReleaseBoundaryGroupingFreezePlanSideEffects::none()
        );
    }
}
