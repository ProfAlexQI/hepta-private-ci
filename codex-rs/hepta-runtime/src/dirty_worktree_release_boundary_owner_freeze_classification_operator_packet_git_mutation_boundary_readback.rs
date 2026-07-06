use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport;
use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_GATE: &str =
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_packet_gate: &'static str,
    pub source_operator_packet_ready: bool,
    pub source_operator_packet_visible: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_packet_payload_persisted: bool,
    pub source_readback_persisted: bool,
    pub source_packet_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub readback_scope:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackScope,
    pub readback_entry_count: usize,
    pub stable_readback_key_count: usize,
    pub readback_route_count: usize,
    pub readback_ready_count: usize,
    pub packet_visible_unsent_unpersisted_count: usize,
    pub git_mutation_blocked_count: usize,
    pub git_operation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub owner_assignment_blocked_count: usize,
    pub freeze_application_blocked_count: usize,
    pub classification_persistence_blocked_count: usize,
    pub test_probe_blocked_count: usize,
    pub operator_decision_required_count: usize,
    pub evidence_recorded_count: usize,
    pub operator_packet_visible: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub packet_payload_persisted: bool,
    pub readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persistence_allowed: bool,
    pub approval_request_sent: bool,
    pub approval_acceptance_allowed: bool,
    pub decision_recording_allowed: bool,
    pub blocker_waiver_allowed: bool,
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
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub git_mutation_boundary_readback_ready: bool,
    pub entries: Vec<
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackEntry,
    >,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackScope
{
    pub readback_id: &'static str,
    pub readback_route: &'static str,
    pub source_operator_packet_route: &'static str,
    pub source_non_send_readback_route: &'static str,
    pub readback_mode: &'static str,
    pub git_mutation_boundary: &'static str,
    pub git_index_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub deletion_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackEntry
{
    pub source_packet_key: String,
    pub source_packet_route: String,
    pub source_non_send_readback_key: String,
    pub source_non_send_readback_route: String,
    pub git_boundary_readback_key: String,
    pub git_boundary_readback_route: String,
    pub source_bucket: &'static str,
    pub group_type: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_route: String,
    pub outcome_category: &'static str,
    pub packet_section: &'static str,
    pub required_local_gate: &'static str,
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
    pub owner_assignment_blocked: bool,
    pub freeze_application_blocked: bool,
    pub classification_persistence_blocked: bool,
    pub test_probe_blocked: bool,
    pub packet_send_blocked: bool,
    pub packet_persistence_blocked: bool,
    pub readback_persistence_blocked: bool,
    pub evidence_recording_allowed: bool,
    pub approval_request_blocked: bool,
    pub approval_acceptance_blocked: bool,
    pub decision_recording_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackSideEffects
{
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub packet_payload_persisted: bool,
    pub readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub decision_recorded: bool,
    pub decision_recording_persisted: bool,
    pub git_add_performed: bool,
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub cleanup_performed: bool,
    pub unrelated_file_deleted: bool,
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackReport{
    let source =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report();
    dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report_from_operator_packet(&source)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report_from_operator_packet(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackReport{
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_entries(source);
    let stable_readback_key_count = entries
        .iter()
        .map(|entry| entry.git_boundary_readback_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let readback_route_count = entries
        .iter()
        .map(|entry| entry.git_boundary_readback_route.as_str())
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
                && entry.owner_assignment_blocked
                && entry.freeze_application_blocked
                && entry.classification_persistence_blocked
                && entry.test_probe_blocked
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.readback_persistence_blocked
                && !entry.evidence_recording_allowed
                && entry.approval_request_blocked
                && entry.approval_acceptance_blocked
                && entry.decision_recording_blocked
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
    let owner_assignment_blocked_count = entries
        .iter()
        .filter(|entry| entry.owner_assignment_blocked)
        .count();
    let freeze_application_blocked_count = entries
        .iter()
        .filter(|entry| entry.freeze_application_blocked)
        .count();
    let classification_persistence_blocked_count = entries
        .iter()
        .filter(|entry| entry.classification_persistence_blocked)
        .count();
    let test_probe_blocked_count = entries
        .iter()
        .filter(|entry| entry.test_probe_blocked)
        .count();
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let git_mutation_boundary_readback_ready = source.operator_packet_without_send_ready
        && source.operator_packet_visible
        && !source.operator_packet_sent
        && !source.operator_packet_persisted
        && !source.packet_payload_persisted
        && !source.readback_persisted
        && !source.git_index_mutated
        && !source.cleanup_allowed
        && !source.delete_allowed
        && source.side_effects
            == DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects::none()
        && entries.len() == source.packet_entry_count
        && stable_readback_key_count == entries.len()
        && readback_route_count == entries.len()
        && readback_ready_count == entries.len()
        && packet_visible_unsent_unpersisted_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && git_operation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && owner_assignment_blocked_count == entries.len()
        && freeze_application_blocked_count == entries.len()
        && classification_persistence_blocked_count == entries.len()
        && test_probe_blocked_count == entries.len()
        && operator_decision_required_count == entries.len()
        && evidence_recorded_count == 0;

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation",
        status: if git_mutation_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_operator_packet_gate: source.gate,
        source_operator_packet_ready: source.operator_packet_without_send_ready,
        source_operator_packet_visible: source.operator_packet_visible,
        source_operator_packet_sent: source.operator_packet_sent,
        source_operator_packet_persisted: source.operator_packet_persisted,
        source_packet_payload_persisted: source.packet_payload_persisted,
        source_readback_persisted: source.readback_persisted,
        source_packet_entry_count: source.packet_entry_count,
        source_tracked_change_count: source.source_tracked_change_count,
        source_untracked_change_count: source.source_untracked_change_count,
        readback_scope:
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_scope(),
        readback_entry_count: entries.len(),
        stable_readback_key_count,
        readback_route_count,
        readback_ready_count,
        packet_visible_unsent_unpersisted_count,
        git_mutation_blocked_count,
        git_operation_blocked_count,
        cleanup_delete_blocked_count,
        owner_assignment_blocked_count,
        freeze_application_blocked_count,
        classification_persistence_blocked_count,
        test_probe_blocked_count,
        operator_decision_required_count,
        evidence_recorded_count,
        operator_packet_visible: git_mutation_boundary_readback_ready,
        operator_packet_sent: false,
        operator_packet_persisted: false,
        packet_payload_persisted: false,
        readback_persisted: false,
        owner_assignment_persisted: false,
        freeze_applied: false,
        classification_persisted: false,
        test_probe_executed: false,
        evidence_recording_allowed: false,
        evidence_persistence_allowed: false,
        approval_request_sent: false,
        approval_acceptance_allowed: false,
        decision_recording_allowed: false,
        blocker_waiver_allowed: false,
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
            "owner_assignment_persistence_blocked",
            "freeze_application_blocked",
            "classification_persistence_blocked",
            "test_probe_execution_blocked",
            "operator_packet_send_blocked",
            "operator_packet_persistence_blocked",
            "operator_packet_readback_persistence_blocked",
            "evidence_recording_blocked",
            "approval_request_blocked",
            "approval_acceptance_blocked",
            "decision_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_scope()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackScope{
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackScope {
        readback_id: "dirty-worktree.release-boundary.owner-freeze-classification.operator-packet.git-mutation-boundary-readback.v1",
        readback_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/git-mutation-boundary/v1",
        source_operator_packet_route:
            "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/v1",
        source_non_send_readback_route:
            "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/non-send/v1",
        readback_mode: "git_mutation_boundary_readback_only",
        git_mutation_boundary: "closed",
        git_index_boundary: "blocked",
        cleanup_boundary: "blocked",
        deletion_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_entries(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport,
) -> Vec<
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackEntry,
>{
    source
        .entries
        .iter()
        .map(|entry| DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackEntry {
            source_packet_key: entry.packet_key.clone(),
            source_packet_route: entry.packet_route.clone(),
            source_non_send_readback_key: entry.non_send_readback_key.clone(),
            source_non_send_readback_route: entry.non_send_readback_route.clone(),
            git_boundary_readback_key: format!(
                "dirty_worktree.owner_freeze_classification_operator_packet.git_boundary.{}.{}",
                key_safe(entry.group_type),
                key_safe(entry.source_bucket)
            ),
            git_boundary_readback_route: format!(
                "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/git-mutation-boundary/{}/{}",
                route_group_type(entry.group_type),
                route_safe(entry.source_bucket)
            ),
            source_bucket: entry.source_bucket,
            group_type: entry.group_type,
            source_entry_count: entry.source_entry_count,
            tracked_count: entry.tracked_count,
            untracked_count: entry.untracked_count,
            owner_route: entry.owner_route.clone(),
            outcome_category: entry.outcome_category,
            packet_section: entry.packet_section,
            required_local_gate: entry.required_local_gate,
            previous_git_mutation_state: "blocked",
            current_git_mutation_state: "blocked",
            git_mutation_state_delta: "unchanged_blocked",
            packet_visible: entry.packet_visible && entry.packet_payload_visible,
            packet_unsent: entry.non_send_confirmed,
            packet_unpersisted: entry.non_persistence_confirmed,
            readback_unpersisted: true,
            operator_visible: true,
            queryable: entry.queryable,
            diffable: entry.diffable,
            operator_decision_required: entry.operator_decision_required,
            git_add_blocked: true,
            git_index_mutation_blocked: entry.git_mutation_blocked,
            git_commit_blocked: true,
            git_push_blocked: true,
            git_reset_blocked: true,
            git_checkout_blocked: true,
            git_revert_blocked: true,
            cleanup_blocked: entry.cleanup_delete_blocked,
            delete_blocked: entry.cleanup_delete_blocked,
            owner_assignment_blocked: entry.owner_assignment_blocked,
            freeze_application_blocked: entry.freeze_application_blocked,
            classification_persistence_blocked: entry.classification_persistence_blocked,
            test_probe_blocked: entry.test_probe_blocked,
            packet_send_blocked: entry.packet_send_blocked,
            packet_persistence_blocked: entry.packet_persistence_blocked,
            readback_persistence_blocked: entry.readback_persistence_blocked,
            evidence_recording_allowed: false,
            approval_request_blocked: entry.approval_request_blocked,
            approval_acceptance_blocked: entry.approval_acceptance_blocked,
            decision_recording_blocked: entry.decision_recording_blocked,
            release_cutover_allowed: false,
            canary_activation_allowed: false,
            live_execution_allowed: false,
        })
        .collect()
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            packet_sent: false,
            packet_persisted: false,
            packet_payload_persisted: false,
            readback_persisted: false,
            owner_assignment_persisted: false,
            freeze_applied: false,
            classification_persisted: false,
            test_probe_executed: false,
            evidence_recorded: false,
            evidence_persisted: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            decision_recorded: false,
            decision_recording_persisted: false,
            git_add_performed: false,
            git_index_mutated: false,
            git_commit_created: false,
            git_push_performed: false,
            git_reset_performed: false,
            git_checkout_performed: false,
            git_revert_performed: false,
            cleanup_performed: false,
            unrelated_file_deleted: false,
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
    fn owner_freeze_classification_git_boundary_is_ready_but_blocked() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_packet_ready);
        assert!(report.source_operator_packet_visible);
        assert!(!report.source_operator_packet_sent);
        assert!(!report.source_operator_packet_persisted);
        assert!(!report.source_packet_payload_persisted);
        assert!(!report.source_readback_persisted);
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
            report.git_mutation_blocked_count,
            report.readback_entry_count
        );
        assert_eq!(
            report.git_operation_blocked_count,
            report.readback_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.git_mutation_boundary_readback_ready);
    }

    #[test]
    fn owner_freeze_classification_git_boundary_entries_block_git_operations() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "codex-rs"
                && entry.git_boundary_readback_route
                    == "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/git-mutation-boundary/top-level/codex-rs"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "cross_lane_or_unowned"
                && entry.git_boundary_readback_route
                    == "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/git-mutation-boundary/scope/cross-lane-or-unowned"
        }));
        assert!(report.entries.iter().all(|entry| {
            entry.packet_visible
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
                && entry.owner_assignment_blocked
                && entry.freeze_application_blocked
                && entry.classification_persistence_blocked
                && entry.test_probe_blocked
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.readback_persistence_blocked
                && !entry.evidence_recording_allowed
                && entry.approval_request_blocked
                && entry.approval_acceptance_blocked
                && entry.decision_recording_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn owner_freeze_classification_git_boundary_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report();

        assert!(!report.git_add_allowed);
        assert!(!report.git_index_mutated);
        assert!(!report.git_commit_allowed);
        assert!(!report.git_push_allowed);
        assert!(!report.git_reset_allowed);
        assert!(!report.git_checkout_allowed);
        assert!(!report.git_revert_allowed);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.owner_assignment_persisted);
        assert!(!report.freeze_applied);
        assert!(!report.classification_persisted);
        assert!(!report.test_probe_executed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.decision_recording_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketGitMutationBoundaryReadbackSideEffects::none()
        );
    }
}
