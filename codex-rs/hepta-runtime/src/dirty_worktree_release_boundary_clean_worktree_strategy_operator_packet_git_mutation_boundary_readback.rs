use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport;
use crate::dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_GATE:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "phase18_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_non_send_readback_gate: &'static str,
    pub source_non_send_readback_ready: bool,
    pub source_operator_packet_visible: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_readback_persisted: bool,
    pub source_strategy_applied: bool,
    pub source_readback_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub readback_scope:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackScope,
    pub readback_entry_count: usize,
    pub stable_readback_key_count: usize,
    pub readback_route_count: usize,
    pub readback_ready_count: usize,
    pub packet_visible_unsent_unpersisted_count: usize,
    pub git_mutation_blocked_count: usize,
    pub git_operation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub operator_decision_required_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_packet_visible: bool,
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
    pub evidence_recording_allowed: bool,
    pub evidence_persistence_allowed: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub git_mutation_boundary_readback_ready: bool,
    pub entries:
        Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackScope
{
    pub readback_id: &'static str,
    pub readback_route: &'static str,
    pub source_non_send_readback_route: &'static str,
    pub readback_mode: &'static str,
    pub git_mutation_boundary: &'static str,
    pub git_index_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub deletion_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackEntry
{
    pub source_non_send_readback_key: &'static str,
    pub source_non_send_readback_route: &'static str,
    pub git_boundary_readback_key: &'static str,
    pub git_boundary_readback_route: &'static str,
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
    pub decision_state: &'static str,
    pub previous_git_mutation_state: &'static str,
    pub current_git_mutation_state: &'static str,
    pub git_mutation_state_delta: &'static str,
    pub packet_visible: bool,
    pub packet_unsent: bool,
    pub packet_unpersisted: bool,
    pub readback_unpersisted: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub git_add_blocked: bool,
    pub git_index_mutation_blocked: bool,
    pub git_commit_blocked: bool,
    pub git_push_blocked: bool,
    pub git_reset_blocked: bool,
    pub git_checkout_blocked: bool,
    pub git_revert_blocked: bool,
    pub cleanup_blocked: bool,
    pub delete_blocked: bool,
    pub strategy_application_blocked: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackSideEffects
{
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
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport
{
    let non_send =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report();
    dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report_from_non_send_readback(&non_send)
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report_from_non_send_readback(
    non_send: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport,
) -> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport
{
    let entries =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_entries(non_send);
    let stable_readback_key_count = entries
        .iter()
        .map(|entry| entry.git_boundary_readback_key)
        .collect::<BTreeSet<_>>()
        .len();
    let readback_route_count = entries
        .iter()
        .map(|entry| entry.git_boundary_readback_route)
        .collect::<BTreeSet<_>>()
        .len();
    let readback_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.packet_visible
                && entry.packet_unsent
                && entry.packet_unpersisted
                && entry.readback_unpersisted
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
                && entry.cleanup_blocked
                && entry.delete_blocked
                && entry.strategy_application_blocked
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        })
        .count();
    let packet_visible_unsent_unpersisted_count = entries
        .iter()
        .filter(|entry| {
            entry.packet_visible
                && entry.packet_unsent
                && entry.packet_unpersisted
                && entry.readback_unpersisted
        })
        .count();
    let git_mutation_blocked_count = entries
        .iter()
        .filter(|entry| {
            entry.previous_git_mutation_state == "blocked"
                && entry.current_git_mutation_state == "blocked"
                && entry.git_mutation_state_delta == "unchanged_blocked"
                && entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
        })
        .count();
    let git_operation_blocked_count = entries
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
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let git_mutation_boundary_readback_ready = non_send.non_send_readback_ready
        && non_send.operator_packet_visible
        && !non_send.operator_packet_sent
        && !non_send.operator_packet_persisted
        && !non_send.readback_persisted
        && !non_send.strategy_applied
        && entries.len() == non_send.readback_entry_count
        && stable_readback_key_count == entries.len()
        && readback_route_count == entries.len()
        && readback_ready_count == entries.len()
        && packet_visible_unsent_unpersisted_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && git_operation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && operator_decision_required_count == entries.len()
        && evidence_recorded_count == 0;

    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback",
        status: if git_mutation_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: non_send.plugin_id,
        source_non_send_readback_gate: non_send.gate,
        source_non_send_readback_ready: non_send.non_send_readback_ready,
        source_operator_packet_visible: non_send.operator_packet_visible,
        source_operator_packet_sent: non_send.operator_packet_sent,
        source_operator_packet_persisted: non_send.operator_packet_persisted,
        source_readback_persisted: non_send.readback_persisted,
        source_strategy_applied: non_send.strategy_applied,
        source_readback_entry_count: non_send.readback_entry_count,
        inventory_entry_count: non_send.inventory_entry_count,
        tracked_change_count: non_send.tracked_change_count,
        untracked_change_count: non_send.untracked_change_count,
        readback_scope:
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_scope(),
        readback_entry_count: entries.len(),
        stable_readback_key_count,
        readback_route_count,
        readback_ready_count,
        packet_visible_unsent_unpersisted_count,
        git_mutation_blocked_count,
        git_operation_blocked_count,
        cleanup_delete_blocked_count,
        operator_decision_required_count,
        evidence_recorded_count,
        operator_packet_visible: git_mutation_boundary_readback_ready,
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
        evidence_recording_allowed: false,
        evidence_persistence_allowed: false,
        approval_request_sent: false,
        approval_acceptance_allowed: false,
        blocker_waiver_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        git_mutation_boundary_readback_ready,
        entries,
        blockers: vec![
            "git_add_blocked",
            "git_index_mutation_blocked",
            "git_commit_blocked",
            "git_push_blocked",
            "git_reset_blocked",
            "git_checkout_blocked",
            "git_revert_blocked",
            "cleanup_and_delete_blocked",
            "strategy_application_blocked",
            "operator_packet_send_blocked",
            "operator_packet_persistence_blocked",
            "operator_packet_readback_persistence_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_scope()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackScope {
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackScope {
        readback_id: "dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.git-mutation-boundary-readback.v1",
        readback_route: "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1",
        source_non_send_readback_route: "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/v1",
        readback_mode: "git_mutation_boundary_readback_only",
        git_mutation_boundary: "closed",
        git_index_boundary: "blocked",
        cleanup_boundary: "blocked",
        deletion_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_entries(
    non_send: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport,
) -> Vec<
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackEntry,
> {
    non_send
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackEntry {
                source_non_send_readback_key: entry.non_send_readback_key,
                source_non_send_readback_route: entry.non_send_readback_route,
                git_boundary_readback_key: git_boundary_readback_key(
                    entry.group_type,
                    entry.source_bucket,
                ),
                git_boundary_readback_route: git_boundary_readback_route(
                    entry.group_type,
                    entry.source_bucket,
                ),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                recommended_strategy: entry.recommended_strategy,
                operator_action: entry.operator_action,
                evidence_requirement: entry.evidence_requirement,
                decision_state: entry.decision_state,
                previous_git_mutation_state: "blocked",
                current_git_mutation_state: "blocked",
                git_mutation_state_delta: "unchanged_blocked",
                packet_visible: entry.packet_visible,
                packet_unsent: entry.non_send_confirmed,
                packet_unpersisted: entry.non_persistence_confirmed,
                readback_unpersisted: true,
                operator_visible: true,
                queryable: true,
                diffable: true,
                operator_decision_required: true,
                git_add_blocked: true,
                git_index_mutation_blocked: true,
                git_commit_blocked: true,
                git_push_blocked: true,
                git_reset_blocked: true,
                git_checkout_blocked: true,
                git_revert_blocked: true,
                cleanup_blocked: true,
                delete_blocked: true,
                strategy_application_blocked: true,
                evidence_recording_allowed: false,
                release_cutover_allowed: false,
                live_execution_allowed: false,
            }
        })
        .collect()
}

fn git_boundary_readback_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.packet.git_boundary.top_level.artifacts",
        ("top_level", "scripts") => "dirty_worktree.packet.git_boundary.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.packet.git_boundary.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.packet.git_boundary.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.packet.git_boundary.top_level.plugins",
        ("scope", "hepta_systems_owned") => {
            "dirty_worktree.packet.git_boundary.scope.hepta_systems_owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "dirty_worktree.packet.git_boundary.scope.cross_lane_or_unowned"
        }
        _ => "dirty_worktree.packet.git_boundary.unknown",
    }
}

fn git_boundary_readback_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/artifacts"
        }
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/docs"
        }
        ("top_level", "plugins") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/plugins"
        }
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/scope/cross-lane-or-unowned"
        }
        _ => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/unknown"
        }
    }
}

impl DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
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
            evidence_recorded: false,
            evidence_persisted: false,
            approval_requested: false,
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
    fn git_mutation_boundary_readback_is_ready_but_blocked() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_non_send_readback_ready);
        assert!(report.source_operator_packet_visible);
        assert!(!report.source_operator_packet_sent);
        assert!(!report.source_operator_packet_persisted);
        assert!(!report.source_readback_persisted);
        assert_eq!(
            report.readback_entry_count,
            report.source_readback_entry_count
        );
        assert_eq!(
            report.stable_readback_key_count,
            report.readback_entry_count
        );
        assert_eq!(report.readback_route_count, report.readback_entry_count);
        assert_eq!(report.readback_ready_count, report.readback_entry_count);
        assert_eq!(
            report.git_mutation_blocked_count,
            report.readback_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.git_mutation_boundary_readback_ready);
    }

    #[test]
    fn git_mutation_boundary_entries_are_queryable_without_git_operations() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.git_boundary_readback_route
                == "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/top-level/codex-rs"));
        assert!(report.entries.iter().all(|entry| entry.packet_visible
            && entry.packet_unsent
            && entry.packet_unpersisted
            && entry.readback_unpersisted
            && entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.operator_decision_required
            && entry.previous_git_mutation_state == "blocked"
            && entry.current_git_mutation_state == "blocked"
            && entry.git_mutation_state_delta == "unchanged_blocked"
            && entry.git_add_blocked
            && entry.git_index_mutation_blocked
            && entry.git_commit_blocked
            && entry.git_push_blocked
            && entry.git_reset_blocked
            && entry.git_checkout_blocked
            && entry.git_revert_blocked
            && entry.cleanup_blocked
            && entry.delete_blocked
            && entry.strategy_application_blocked
            && !entry.evidence_recording_allowed
            && !entry.release_cutover_allowed
            && !entry.live_execution_allowed));
    }

    #[test]
    fn git_mutation_boundary_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report();

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
        assert!(!report.approval_request_sent);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.blocker_waiver_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.public_ga_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackSideEffects::none()
        );
    }
}
