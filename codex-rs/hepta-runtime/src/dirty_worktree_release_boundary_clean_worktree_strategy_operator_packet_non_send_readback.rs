use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport;
use crate::dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_GATE:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "phase17_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_packet_gate: &'static str,
    pub source_operator_packet_ready: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_strategy_applied: bool,
    pub source_packet_entry_count: usize,
    pub source_packet_section_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub readback_scope:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackScope,
    pub readback_entry_count: usize,
    pub stable_readback_key_count: usize,
    pub readback_route_count: usize,
    pub readback_ready_count: usize,
    pub visible_unsent_unpersisted_count: usize,
    pub attached_packet_count: usize,
    pub operator_decision_required_count: usize,
    pub no_git_mutation_readback_count: usize,
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
    pub non_send_readback_ready: bool,
    pub entries:
        Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackScope {
    pub readback_id: &'static str,
    pub readback_route: &'static str,
    pub source_packet_route: &'static str,
    pub readback_mode: &'static str,
    pub send_boundary: &'static str,
    pub persistence_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackEntry {
    pub source_packet_key: &'static str,
    pub source_packet_route: &'static str,
    pub non_send_readback_key: &'static str,
    pub non_send_readback_route: &'static str,
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
    pub observed_state: &'static str,
    pub previous_send_state: &'static str,
    pub current_send_state: &'static str,
    pub send_state_delta: &'static str,
    pub previous_persistence_state: &'static str,
    pub current_persistence_state: &'static str,
    pub persistence_state_delta: &'static str,
    pub packet_visible: bool,
    pub non_send_confirmed: bool,
    pub non_persistence_confirmed: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub packet_send_blocked: bool,
    pub packet_persistence_blocked: bool,
    pub approval_request_blocked: bool,
    pub strategy_application_blocked: bool,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackSideEffects
{
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub readback_persisted: bool,
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

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport {
    let packet = dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report();
    dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report_from_packet(&packet)
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report_from_packet(
    packet: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport,
) -> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport {
    let entries =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_entries(packet);
    let stable_readback_key_count = entries
        .iter()
        .map(|entry| entry.non_send_readback_key)
        .collect::<BTreeSet<_>>()
        .len();
    let readback_route_count = entries
        .iter()
        .map(|entry| entry.non_send_readback_route)
        .collect::<BTreeSet<_>>()
        .len();
    let readback_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.packet_visible
                && entry.non_send_confirmed
                && entry.non_persistence_confirmed
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.approval_request_blocked
                && entry.strategy_application_blocked
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.delete_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        })
        .count();
    let visible_unsent_unpersisted_count = entries
        .iter()
        .filter(|entry| {
            entry.observed_state == "operator_packet_visible_unsent_unpersisted"
                && entry.current_send_state == "unsent"
                && entry.current_persistence_state == "unpersisted"
        })
        .count();
    let attached_packet_count = entries
        .iter()
        .filter(|entry| !entry.source_packet_key.is_empty())
        .count();
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let no_git_mutation_readback_count = entries
        .iter()
        .filter(|entry| !entry.git_mutation_allowed)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let non_send_readback_ready = packet.operator_packet_ready
        && !packet.operator_packet_sent
        && !packet.operator_packet_persisted
        && !packet.strategy_applied
        && entries.len() == packet.packet_entry_count
        && stable_readback_key_count == entries.len()
        && readback_route_count == entries.len()
        && readback_ready_count == entries.len()
        && visible_unsent_unpersisted_count == entries.len()
        && attached_packet_count == entries.len()
        && operator_decision_required_count == entries.len()
        && no_git_mutation_readback_count == entries.len()
        && evidence_recorded_count == 0;

    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback",
        status: if non_send_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION,
        plugin_id: packet.plugin_id,
        source_operator_packet_gate: packet.gate,
        source_operator_packet_ready: packet.operator_packet_ready,
        source_operator_packet_sent: packet.operator_packet_sent,
        source_operator_packet_persisted: packet.operator_packet_persisted,
        source_strategy_applied: packet.strategy_applied,
        source_packet_entry_count: packet.packet_entry_count,
        source_packet_section_count: packet.packet_section_count,
        inventory_entry_count: packet.inventory_entry_count,
        tracked_change_count: packet.tracked_change_count,
        untracked_change_count: packet.untracked_change_count,
        readback_scope:
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_scope(),
        readback_entry_count: entries.len(),
        stable_readback_key_count,
        readback_route_count,
        readback_ready_count,
        visible_unsent_unpersisted_count,
        attached_packet_count,
        operator_decision_required_count,
        no_git_mutation_readback_count,
        evidence_recorded_count,
        operator_packet_visible: non_send_readback_ready,
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
        non_send_readback_ready,
        entries,
        blockers: vec![
            "operator_packet_send_blocked",
            "operator_packet_persistence_blocked",
            "operator_packet_readback_persistence_blocked",
            "strategy_application_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_scope()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackScope {
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackScope {
        readback_id: "dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.non-send-readback.v1",
        readback_route: "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/v1",
        source_packet_route: "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/v1",
        readback_mode: "operator_packet_non_send_readback_only",
        send_boundary: "blocked",
        persistence_boundary: "blocked",
        git_mutation_boundary: "closed",
    }
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_entries(
    packet: &DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport,
) -> Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackEntry> {
    packet
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackEntry {
                source_packet_key: entry.packet_key,
                source_packet_route: entry.packet_route,
                non_send_readback_key: non_send_readback_key(entry.group_type, entry.source_bucket),
                non_send_readback_route: non_send_readback_route(
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
                observed_state: "operator_packet_visible_unsent_unpersisted",
                previous_send_state: "unsent",
                current_send_state: "unsent",
                send_state_delta: "unchanged_unsent",
                previous_persistence_state: "unpersisted",
                current_persistence_state: "unpersisted",
                persistence_state_delta: "unchanged_unpersisted",
                packet_visible: true,
                non_send_confirmed: true,
                non_persistence_confirmed: true,
                operator_visible: true,
                queryable: true,
                diffable: true,
                operator_decision_required: true,
                packet_send_blocked: true,
                packet_persistence_blocked: true,
                approval_request_blocked: true,
                strategy_application_blocked: true,
                git_mutation_allowed: false,
                cleanup_allowed: false,
                delete_allowed: false,
                evidence_recording_allowed: false,
                release_cutover_allowed: false,
                live_execution_allowed: false,
            }
        })
        .collect()
}

fn non_send_readback_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.packet.non_send.top_level.artifacts",
        ("top_level", "scripts") => "dirty_worktree.packet.non_send.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.packet.non_send.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.packet.non_send.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.packet.non_send.top_level.plugins",
        ("scope", "hepta_systems_owned") => {
            "dirty_worktree.packet.non_send.scope.hepta_systems_owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "dirty_worktree.packet.non_send.scope.cross_lane_or_unowned"
        }
        _ => "dirty_worktree.packet.non_send.unknown",
    }
}

fn non_send_readback_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/artifacts"
        }
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/docs"
        }
        ("top_level", "plugins") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/plugins"
        }
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/scope/cross-lane-or-unowned"
        }
        _ => {
            "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/unknown"
        }
    }
}

impl DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            packet_sent: false,
            packet_persisted: false,
            readback_persisted: false,
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
    fn operator_packet_non_send_readback_is_ready_but_unsent_and_unpersisted() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_ready);
        assert!(!report.source_operator_packet_sent);
        assert!(!report.source_operator_packet_persisted);
        assert!(!report.source_strategy_applied);
        assert_eq!(
            report.readback_entry_count,
            report.source_packet_entry_count
        );
        assert_eq!(
            report.stable_readback_key_count,
            report.readback_entry_count
        );
        assert_eq!(report.readback_route_count, report.readback_entry_count);
        assert_eq!(report.readback_ready_count, report.readback_entry_count);
        assert_eq!(
            report.visible_unsent_unpersisted_count,
            report.readback_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.operator_packet_visible);
        assert!(report.non_send_readback_ready);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
        assert!(!report.readback_persisted);
    }

    #[test]
    fn operator_packet_non_send_entries_are_queryable_without_git_mutation() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.non_send_readback_route
                == "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/top-level/codex-rs"));
        assert!(report.entries.iter().all(|entry| entry.packet_visible
            && entry.non_send_confirmed
            && entry.non_persistence_confirmed
            && entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.operator_decision_required
            && entry.packet_send_blocked
            && entry.packet_persistence_blocked
            && entry.approval_request_blocked
            && entry.strategy_application_blocked
            && entry.observed_state == "operator_packet_visible_unsent_unpersisted"
            && entry.previous_send_state == "unsent"
            && entry.current_send_state == "unsent"
            && entry.send_state_delta == "unchanged_unsent"
            && entry.previous_persistence_state == "unpersisted"
            && entry.current_persistence_state == "unpersisted"
            && entry.persistence_state_delta == "unchanged_unpersisted"
            && !entry.git_mutation_allowed
            && !entry.cleanup_allowed
            && !entry.delete_allowed
            && !entry.evidence_recording_allowed
            && !entry.release_cutover_allowed
            && !entry.live_execution_allowed));
    }

    #[test]
    fn operator_packet_non_send_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report();

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
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketNonSendReadbackSideEffects::none()
        );
    }
}
