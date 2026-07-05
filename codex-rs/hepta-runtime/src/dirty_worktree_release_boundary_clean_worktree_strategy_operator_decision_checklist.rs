use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport;
use crate::dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_GATE:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_RECOMMENDED_NEXT_GATE:
    &str = "phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_git_boundary_readback_gate: &'static str,
    pub source_git_boundary_readback_ready: bool,
    pub source_operator_packet_visible: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_readback_persisted: bool,
    pub source_strategy_applied: bool,
    pub source_readback_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub checklist_scope:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistScope,
    pub checklist_entry_count: usize,
    pub stable_checklist_key_count: usize,
    pub checklist_route_count: usize,
    pub checklist_ready_count: usize,
    pub packet_visible_unsent_unpersisted_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub strategy_application_blocked_count: usize,
    pub operator_decision_required_count: usize,
    pub pending_operator_decision_count: usize,
    pub evidence_requirement_count: usize,
    pub evidence_recorded_count: usize,
    pub decision_checklist_visible: bool,
    pub decision_checklist_persisted: bool,
    pub decision_recorded: bool,
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
    pub operator_decision_checklist_ready: bool,
    pub entries:
        Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistScope {
    pub checklist_id: &'static str,
    pub checklist_route: &'static str,
    pub source_git_boundary_readback_route: &'static str,
    pub checklist_mode: &'static str,
    pub decision_recording_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub evidence_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistEntry {
    pub source_git_boundary_readback_key: &'static str,
    pub source_git_boundary_readback_route: &'static str,
    pub checklist_key: &'static str,
    pub checklist_route: &'static str,
    pub decision_checkpoint: &'static str,
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
    pub checklist_state: &'static str,
    pub packet_visible: bool,
    pub packet_unsent: bool,
    pub packet_unpersisted: bool,
    pub readback_unpersisted: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub decision_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub evidence_recording_allowed: bool,
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
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistSideEffects {
    pub decision_checklist_persisted: bool,
    pub decision_recorded: bool,
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

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistReport {
    let git_boundary =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report();
    dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report_from_git_boundary(&git_boundary)
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report_from_git_boundary(
    git_boundary: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport,
) -> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistReport {
    let entries =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_entries(
            git_boundary,
        );
    let stable_checklist_key_count = entries
        .iter()
        .map(|entry| entry.checklist_key)
        .collect::<BTreeSet<_>>()
        .len();
    let checklist_route_count = entries
        .iter()
        .map(|entry| entry.checklist_route)
        .collect::<BTreeSet<_>>()
        .len();
    let checklist_ready_count = entries
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
                && !entry.decision_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.evidence_recording_allowed
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
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
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
    let strategy_application_blocked_count = entries
        .iter()
        .filter(|entry| entry.strategy_application_blocked)
        .count();
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let pending_operator_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_state == "pending_operator_decision")
        .count();
    let evidence_requirement_count = entries
        .iter()
        .filter(|entry| !entry.evidence_requirement.is_empty())
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();

    let operator_decision_checklist_ready = git_boundary.git_mutation_boundary_readback_ready
        && git_boundary.operator_packet_visible
        && !git_boundary.operator_packet_sent
        && !git_boundary.operator_packet_persisted
        && !git_boundary.readback_persisted
        && !git_boundary.strategy_applied
        && entries.len() == git_boundary.readback_entry_count
        && stable_checklist_key_count == entries.len()
        && checklist_route_count == entries.len()
        && checklist_ready_count == entries.len()
        && packet_visible_unsent_unpersisted_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && strategy_application_blocked_count == entries.len()
        && operator_decision_required_count == entries.len()
        && pending_operator_decision_count == entries.len()
        && evidence_requirement_count == entries.len()
        && evidence_recorded_count == 0;

    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist",
        status: if operator_decision_checklist_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_SCHEMA_VERSION,
        plugin_id: git_boundary.plugin_id,
        source_git_boundary_readback_gate: git_boundary.gate,
        source_git_boundary_readback_ready: git_boundary.git_mutation_boundary_readback_ready,
        source_operator_packet_visible: git_boundary.operator_packet_visible,
        source_operator_packet_sent: git_boundary.operator_packet_sent,
        source_operator_packet_persisted: git_boundary.operator_packet_persisted,
        source_readback_persisted: git_boundary.readback_persisted,
        source_strategy_applied: git_boundary.strategy_applied,
        source_readback_entry_count: git_boundary.readback_entry_count,
        inventory_entry_count: git_boundary.inventory_entry_count,
        tracked_change_count: git_boundary.tracked_change_count,
        untracked_change_count: git_boundary.untracked_change_count,
        checklist_scope:
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_scope(),
        checklist_entry_count: entries.len(),
        stable_checklist_key_count,
        checklist_route_count,
        checklist_ready_count,
        packet_visible_unsent_unpersisted_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        strategy_application_blocked_count,
        operator_decision_required_count,
        pending_operator_decision_count,
        evidence_requirement_count,
        evidence_recorded_count,
        decision_checklist_visible: operator_decision_checklist_ready,
        decision_checklist_persisted: false,
        decision_recorded: false,
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
        operator_decision_checklist_ready,
        entries,
        blockers: vec![
            "operator_decision_recording_blocked",
            "approval_acceptance_blocked",
            "evidence_recording_blocked",
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
            "readback_persistence_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_scope()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistScope {
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistScope {
        checklist_id: "dirty-worktree.release-boundary.clean-worktree-strategy.operator-decision-checklist.v1",
        checklist_route: "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/v1",
        source_git_boundary_readback_route: "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1",
        checklist_mode: "operator_decision_checklist_only",
        decision_recording_boundary: "blocked",
        git_mutation_boundary: "closed",
        cleanup_boundary: "blocked",
        evidence_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_entries(
    git_boundary: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketGitMutationBoundaryReadbackReport,
) -> Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistEntry> {
    git_boundary
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistEntry {
                source_git_boundary_readback_key: entry.git_boundary_readback_key,
                source_git_boundary_readback_route: entry.git_boundary_readback_route,
                checklist_key: decision_checklist_key(entry.group_type, entry.source_bucket),
                checklist_route: decision_checklist_route(entry.group_type, entry.source_bucket),
                decision_checkpoint: decision_checkpoint(entry.group_type, entry.source_bucket),
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
                checklist_state: "ready_blocked_pending_operator_decision",
                packet_visible: entry.packet_visible,
                packet_unsent: entry.packet_unsent,
                packet_unpersisted: entry.packet_unpersisted,
                readback_unpersisted: entry.readback_unpersisted,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                operator_decision_required: entry.operator_decision_required,
                decision_recording_allowed: false,
                approval_acceptance_allowed: false,
                evidence_recording_allowed: false,
                git_add_blocked: entry.git_add_blocked,
                git_index_mutation_blocked: entry.git_index_mutation_blocked,
                git_commit_blocked: entry.git_commit_blocked,
                git_push_blocked: entry.git_push_blocked,
                git_reset_blocked: entry.git_reset_blocked,
                git_checkout_blocked: entry.git_checkout_blocked,
                git_revert_blocked: entry.git_revert_blocked,
                cleanup_blocked: entry.cleanup_blocked,
                delete_blocked: entry.delete_blocked,
                strategy_application_blocked: entry.strategy_application_blocked,
                release_cutover_allowed: false,
                canary_activation_allowed: false,
                live_execution_allowed: false,
            }
        })
        .collect()
}

fn decision_checklist_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.decision_checklist.top_level.artifacts",
        ("top_level", "scripts") => "dirty_worktree.decision_checklist.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.decision_checklist.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.decision_checklist.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.decision_checklist.top_level.plugins",
        ("scope", "hepta_systems_owned") => {
            "dirty_worktree.decision_checklist.scope.hepta_systems_owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "dirty_worktree.decision_checklist.scope.cross_lane_or_unowned"
        }
        _ => "dirty_worktree.decision_checklist.unknown",
    }
}

fn decision_checklist_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/artifacts"
        }
        ("top_level", "scripts") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/docs"
        }
        ("top_level", "plugins") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/plugins"
        }
        ("scope", "hepta_systems_owned") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/scope/cross-lane-or-unowned"
        }
        _ => {
            "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/unknown"
        }
    }
}

fn decision_checkpoint(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "operator_decision_checkpoint.top_level.artifacts",
        ("top_level", "scripts") => "operator_decision_checkpoint.top_level.scripts",
        ("top_level", "codex-rs") => "operator_decision_checkpoint.top_level.codex_rs",
        ("top_level", "docs") => "operator_decision_checkpoint.top_level.docs",
        ("top_level", "plugins") => "operator_decision_checkpoint.top_level.plugins",
        ("scope", "hepta_systems_owned") => {
            "operator_decision_checkpoint.scope.hepta_systems_owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "operator_decision_checkpoint.scope.cross_lane_or_unowned"
        }
        _ => "operator_decision_checkpoint.unknown",
    }
}

impl DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistSideEffects {
    pub const fn none() -> Self {
        Self {
            decision_checklist_persisted: false,
            decision_recorded: false,
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
    fn operator_decision_checklist_is_ready_but_blocked() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_git_boundary_readback_ready);
        assert!(report.source_operator_packet_visible);
        assert!(!report.source_operator_packet_sent);
        assert!(!report.source_operator_packet_persisted);
        assert!(!report.source_readback_persisted);
        assert_eq!(
            report.checklist_entry_count,
            report.source_readback_entry_count
        );
        assert_eq!(
            report.stable_checklist_key_count,
            report.checklist_entry_count
        );
        assert_eq!(report.checklist_route_count, report.checklist_entry_count);
        assert_eq!(report.checklist_ready_count, report.checklist_entry_count);
        assert_eq!(
            report.pending_operator_decision_count,
            report.checklist_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.operator_decision_checklist_ready);
    }

    #[test]
    fn checklist_entries_preserve_pending_decisions_without_git_operations() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.checklist_route
                == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/codex-rs"));
        assert!(report.entries.iter().all(|entry| entry.packet_visible
            && entry.packet_unsent
            && entry.packet_unpersisted
            && entry.readback_unpersisted
            && entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.operator_decision_required
            && entry.decision_state == "pending_operator_decision"
            && entry.checklist_state == "ready_blocked_pending_operator_decision"
            && !entry.decision_recording_allowed
            && !entry.approval_acceptance_allowed
            && !entry.evidence_recording_allowed
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
            && !entry.release_cutover_allowed
            && !entry.canary_activation_allowed
            && !entry.live_execution_allowed));
    }

    #[test]
    fn checklist_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report();

        assert!(report.decision_checklist_visible);
        assert!(!report.decision_checklist_persisted);
        assert!(!report.decision_recorded);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.strategy_applied);
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
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorDecisionChecklistSideEffects::none()
        );
    }
}
