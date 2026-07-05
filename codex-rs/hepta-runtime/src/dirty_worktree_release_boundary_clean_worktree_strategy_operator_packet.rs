use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport;
use crate::dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GATE: &str =
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_RECOMMENDED_NEXT_GATE: &str =
    "phase16_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_strategy_gate: &'static str,
    pub source_strategy_ready: bool,
    pub source_strategy_applied: bool,
    pub source_strategy_entry_count: usize,
    pub inventory_entry_count: usize,
    pub tracked_change_count: usize,
    pub untracked_change_count: usize,
    pub packet_scope: DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketScope,
    pub packet_section_count: usize,
    pub packet_entry_count: usize,
    pub stable_packet_key_count: usize,
    pub packet_route_count: usize,
    pub attached_strategy_count: usize,
    pub operator_decision_required_count: usize,
    pub no_git_mutation_packet_count: usize,
    pub hepta_systems_packet_count: usize,
    pub cross_lane_packet_count: usize,
    pub mixed_lane_packet_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_packet_ready: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
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
    pub sections: Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSection>,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketScope {
    pub packet_id: &'static str,
    pub packet_route: &'static str,
    pub source_strategy_route: &'static str,
    pub packet_mode: &'static str,
    pub send_mode: &'static str,
    pub mutation_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSection {
    pub id: &'static str,
    pub title: &'static str,
    pub source: &'static str,
    pub preview_ready: bool,
    pub mutation_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketEntry {
    pub source_strategy_key: &'static str,
    pub source_strategy_route: &'static str,
    pub packet_key: &'static str,
    pub packet_route: &'static str,
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
    pub packet_section: &'static str,
    pub decision_state: &'static str,
    pub attached_to_packet: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub strategy_applied: bool,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub release_cutover_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSideEffects {
    pub packet_sent: bool,
    pub packet_persisted: bool,
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

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport {
    let strategy = dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report();
    dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report_from_strategy(
        &strategy,
    )
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report_from_strategy(
    strategy: &DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport,
) -> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport {
    let sections =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_sections();
    let entries =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_entries(strategy);
    let stable_packet_key_count = entries
        .iter()
        .map(|entry| entry.packet_key)
        .collect::<BTreeSet<_>>()
        .len();
    let packet_route_count = entries
        .iter()
        .map(|entry| entry.packet_route)
        .collect::<BTreeSet<_>>()
        .len();
    let attached_strategy_count = entries
        .iter()
        .filter(|entry| entry.attached_to_packet)
        .count();
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let no_git_mutation_packet_count = entries
        .iter()
        .filter(|entry| !entry.git_mutation_allowed)
        .count();
    let hepta_systems_packet_count = entries
        .iter()
        .filter(|entry| entry.review_lane == "hepta-systems")
        .count();
    let cross_lane_packet_count = entries
        .iter()
        .filter(|entry| {
            entry.review_lane == "cross-lane-review"
                || entry.review_lane == "external-or-cross-lane"
        })
        .count();
    let mixed_lane_packet_count = entries
        .iter()
        .filter(|entry| {
            entry.review_lane == "mixed" || entry.review_lane == "mixed-hepta-and-cross-lane"
        })
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let operator_packet_ready = strategy.strategy_ready
        && !strategy.strategy_applied
        && sections.len() == 6
        && sections
            .iter()
            .all(|section| section.preview_ready && !section.mutation_enabled)
        && entries.len() == strategy.strategy_entry_count
        && stable_packet_key_count == entries.len()
        && packet_route_count == entries.len()
        && attached_strategy_count == entries.len()
        && operator_decision_required_count == entries.len()
        && no_git_mutation_packet_count == entries.len()
        && evidence_recorded_count == 0
        && entries.iter().all(|entry| {
            entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.attached_to_packet
                && entry.operator_decision_required
                && !entry.packet_sent
                && !entry.packet_persisted
                && !entry.strategy_applied
                && !entry.git_mutation_allowed
                && !entry.cleanup_allowed
                && !entry.delete_allowed
                && !entry.evidence_recording_allowed
                && !entry.release_cutover_allowed
                && !entry.live_execution_allowed
        });

    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet",
        status: if operator_packet_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_SCHEMA_VERSION,
        plugin_id: strategy.plugin_id,
        source_strategy_gate: strategy.gate,
        source_strategy_ready: strategy.strategy_ready,
        source_strategy_applied: strategy.strategy_applied,
        source_strategy_entry_count: strategy.strategy_entry_count,
        inventory_entry_count: strategy.inventory_entry_count,
        tracked_change_count: strategy.tracked_change_count,
        untracked_change_count: strategy.untracked_change_count,
        packet_scope: dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_scope(
        ),
        packet_section_count: sections.len(),
        packet_entry_count: entries.len(),
        stable_packet_key_count,
        packet_route_count,
        attached_strategy_count,
        operator_decision_required_count,
        no_git_mutation_packet_count,
        hepta_systems_packet_count,
        cross_lane_packet_count,
        mixed_lane_packet_count,
        evidence_recorded_count,
        operator_packet_ready,
        operator_packet_sent: false,
        operator_packet_persisted: false,
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
        sections,
        entries,
        blockers: vec![
            "operator_packet_not_sent",
            "operator_packet_not_persisted",
            "strategy_application_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_scope()
-> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketScope {
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketScope {
        packet_id: "dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.v1",
        packet_route: "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/v1",
        source_strategy_route: "readback://release-boundary/dirty-worktree/actionable-clean-worktree-strategy/v1",
        packet_mode: "operator_packet_preview_only",
        send_mode: "not_sent_not_persisted",
        mutation_boundary: "closed",
    }
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_sections()
-> Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSection> {
    vec![
        section("scope", "Scope", "Phase 14 clean-worktree strategy"),
        section(
            "inventory_summary",
            "Inventory Summary",
            "dirty-worktree inventory",
        ),
        section(
            "strategy_entries",
            "Strategy Entries",
            "Phase 14 clean-worktree strategy",
        ),
        section(
            "operator_decisions",
            "Operator Decisions",
            "pending operator decisions",
        ),
        section(
            "evidence_requirements",
            "Evidence Requirements",
            "clean-worktree decision record",
        ),
        section("closed_boundary", "Closed Boundary", "local packet preview"),
    ]
}

pub fn dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_entries(
    strategy: &DirtyWorktreeReleaseBoundaryActionableCleanWorktreeStrategyReport,
) -> Vec<DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketEntry> {
    strategy
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketEntry {
                source_strategy_key: entry.strategy_key,
                source_strategy_route: entry.strategy_route,
                packet_key: packet_key(entry.group_type, entry.source_bucket),
                packet_route: packet_route(entry.group_type, entry.source_bucket),
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
                packet_section: "strategy_entries",
                decision_state: entry.decision_state,
                attached_to_packet: true,
                operator_visible: true,
                queryable: true,
                diffable: true,
                operator_decision_required: true,
                packet_sent: false,
                packet_persisted: false,
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

fn section(
    id: &'static str,
    title: &'static str,
    source: &'static str,
) -> DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSection {
    DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSection {
        id,
        title,
        source,
        preview_ready: true,
        mutation_enabled: false,
    }
}

fn packet_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.packet.top_level.artifacts",
        ("top_level", "scripts") => "dirty_worktree.packet.top_level.scripts",
        ("top_level", "codex-rs") => "dirty_worktree.packet.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.packet.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.packet.top_level.plugins",
        ("scope", "hepta_systems_owned") => "dirty_worktree.packet.scope.hepta_systems_owned",
        ("scope", "cross_lane_or_unowned") => "dirty_worktree.packet.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.packet.unknown",
    }
}

fn packet_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/top-level/artifacts"
        }
        ("top_level", "scripts") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/top-level/scripts"
        }
        ("top_level", "codex-rs") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/top-level/codex-rs"
        }
        ("top_level", "docs") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/top-level/docs"
        }
        ("top_level", "plugins") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/top-level/plugins"
        }
        ("scope", "hepta_systems_owned") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/scope/hepta-systems-owned"
        }
        ("scope", "cross_lane_or_unowned") => {
            "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/scope/cross-lane-or-unowned"
        }
        _ => "operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/unknown",
    }
}

impl DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSideEffects {
    pub const fn none() -> Self {
        Self {
            packet_sent: false,
            packet_persisted: false,
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
    fn operator_packet_is_ready_but_unsent_and_unpersisted() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_strategy_ready);
        assert!(!report.source_strategy_applied);
        assert!(report.operator_packet_ready);
        assert_eq!(report.packet_section_count, 6);
        assert_eq!(
            report.packet_entry_count,
            report.source_strategy_entry_count
        );
        assert_eq!(report.stable_packet_key_count, report.packet_entry_count);
        assert_eq!(report.packet_route_count, report.packet_entry_count);
        assert_eq!(report.attached_strategy_count, report.packet_entry_count);
        assert_eq!(
            report.operator_decision_required_count,
            report.packet_entry_count
        );
        assert_eq!(
            report.no_git_mutation_packet_count,
            report.packet_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
        assert!(!report.strategy_applied);
    }

    #[test]
    fn operator_packet_entries_attach_strategy_without_mutation() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report();

        assert!(report.entries.iter().all(|entry| {
            entry.attached_to_packet
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && entry.packet_section == "strategy_entries"
                && entry.decision_state == "pending_operator_decision"
                && !entry.packet_sent
                && !entry.packet_persisted
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
    fn operator_packet_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report();

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
            DirtyWorktreeReleaseBoundaryCleanWorktreeStrategyOperatorPacketSideEffects::none()
        );
    }
}
