use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistReport;
use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistSideEffects;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_operator_decision_checklist_gate: &'static str,
    pub source_operator_decision_checklist_ready: bool,
    pub source_decision_checklist_visible: bool,
    pub source_decision_checklist_persisted: bool,
    pub source_decision_recorded: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_packet_payload_persisted: bool,
    pub source_readback_persisted: bool,
    pub source_owner_assignment_persisted: bool,
    pub source_freeze_applied: bool,
    pub source_classification_persisted: bool,
    pub source_test_probe_executed: bool,
    pub source_checklist_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub packet_readback_scope:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackScope,
    pub packet_readback_entry_count: usize,
    pub stable_packet_key_count: usize,
    pub stable_readback_key_count: usize,
    pub packet_route_count: usize,
    pub readback_route_count: usize,
    pub packet_readback_ready_count: usize,
    pub checklist_attached_count: usize,
    pub packet_visible_unsent_unpersisted_count: usize,
    pub readback_visible_unpersisted_count: usize,
    pub pending_operator_decision_count: usize,
    pub decision_recording_blocked_count: usize,
    pub approval_request_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub evidence_recorded_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub owner_assignment_blocked_count: usize,
    pub freeze_application_blocked_count: usize,
    pub classification_persistence_blocked_count: usize,
    pub test_probe_blocked_count: usize,
    pub packet_readback_visible: bool,
    pub packet_readback_persisted: bool,
    pub decision_checklist_persisted: bool,
    pub decision_recorded: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub packet_payload_persisted: bool,
    pub readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
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
    pub operator_decision_checklist_packet_readback_ready: bool,
    pub entries: Vec<
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackEntry,
    >,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackScope
{
    pub packet_readback_id: &'static str,
    pub packet_readback_route: &'static str,
    pub source_checklist_route: &'static str,
    pub readback_mode: &'static str,
    pub packet_send_boundary: &'static str,
    pub packet_persistence_boundary: &'static str,
    pub readback_persistence_boundary: &'static str,
    pub decision_recording_boundary: &'static str,
    pub approval_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub owner_assignment_boundary: &'static str,
    pub freeze_application_boundary: &'static str,
    pub classification_persistence_boundary: &'static str,
    pub test_probe_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackEntry
{
    pub source_checklist_key: String,
    pub source_checklist_route: String,
    pub packet_key: String,
    pub packet_route: String,
    pub readback_key: String,
    pub readback_route: String,
    pub decision_checkpoint: String,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_route: String,
    pub outcome_category: &'static str,
    pub packet_section: &'static str,
    pub required_local_gate: &'static str,
    pub operator_action: &'static str,
    pub evidence_requirement: &'static str,
    pub decision_state: &'static str,
    pub packet_state: &'static str,
    pub readback_state: &'static str,
    pub checklist_attached: bool,
    pub packet_visible: bool,
    pub packet_unsent: bool,
    pub packet_unpersisted: bool,
    pub readback_visible: bool,
    pub readback_unpersisted: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub decision_recording_allowed: bool,
    pub approval_request_blocked: bool,
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
    pub owner_assignment_blocked: bool,
    pub freeze_application_blocked: bool,
    pub classification_persistence_blocked: bool,
    pub test_probe_blocked: bool,
    pub packet_send_blocked: bool,
    pub packet_persistence_blocked: bool,
    pub readback_persistence_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackSideEffects
{
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub packet_payload_persisted: bool,
    pub packet_readback_persisted: bool,
    pub decision_checklist_persisted: bool,
    pub decision_recorded: bool,
    pub readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
    pub git_add_performed: bool,
    pub git_index_mutated: bool,
    pub git_commit_created: bool,
    pub git_push_performed: bool,
    pub git_reset_performed: bool,
    pub git_checkout_performed: bool,
    pub git_revert_performed: bool,
    pub cleanup_performed: bool,
    pub unrelated_file_deleted: bool,
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

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport{
    let checklist =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_report();
    dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report_from_checklist(&checklist)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report_from_checklist(
    checklist: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport
{
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_entries(checklist);
    let stable_packet_key_count = entries
        .iter()
        .map(|entry| entry.packet_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let stable_readback_key_count = entries
        .iter()
        .map(|entry| entry.readback_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let packet_route_count = entries
        .iter()
        .map(|entry| entry.packet_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let readback_route_count = entries
        .iter()
        .map(|entry| entry.readback_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let packet_readback_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.checklist_attached
                && entry.packet_visible
                && entry.packet_unsent
                && entry.packet_unpersisted
                && entry.readback_visible
                && entry.readback_unpersisted
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && !entry.decision_recording_allowed
                && entry.approval_request_blocked
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
                && entry.owner_assignment_blocked
                && entry.freeze_application_blocked
                && entry.classification_persistence_blocked
                && entry.test_probe_blocked
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.readback_persistence_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        })
        .count();
    let checklist_attached_count = entries
        .iter()
        .filter(|entry| entry.checklist_attached)
        .count();
    let packet_visible_unsent_unpersisted_count = entries
        .iter()
        .filter(|entry| entry.packet_visible && entry.packet_unsent && entry.packet_unpersisted)
        .count();
    let readback_visible_unpersisted_count = entries
        .iter()
        .filter(|entry| entry.readback_visible && entry.readback_unpersisted)
        .count();
    let pending_operator_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_state == "pending_operator_decision")
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_recording_allowed)
        .count();
    let approval_request_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_request_blocked)
        .count();
    let approval_acceptance_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_acceptance_allowed)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
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

    let operator_decision_checklist_packet_readback_ready = checklist
        .operator_decision_checklist_ready
        && checklist.decision_checklist_visible
        && !checklist.decision_checklist_persisted
        && !checklist.decision_recorded
        && !checklist.operator_packet_sent
        && !checklist.operator_packet_persisted
        && !checklist.packet_payload_persisted
        && !checklist.readback_persisted
        && !checklist.owner_assignment_persisted
        && !checklist.freeze_applied
        && !checklist.classification_persisted
        && !checklist.test_probe_executed
        && checklist.side_effects
            == DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistSideEffects::none()
        && entries.len() == checklist.checklist_entry_count
        && stable_packet_key_count == entries.len()
        && stable_readback_key_count == entries.len()
        && packet_route_count == entries.len()
        && readback_route_count == entries.len()
        && packet_readback_ready_count == entries.len()
        && checklist_attached_count == entries.len()
        && packet_visible_unsent_unpersisted_count == entries.len()
        && readback_visible_unpersisted_count == entries.len()
        && pending_operator_decision_count == entries.len()
        && decision_recording_blocked_count == entries.len()
        && approval_request_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && evidence_recorded_count == 0
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && owner_assignment_blocked_count == entries.len()
        && freeze_application_blocked_count == entries.len()
        && classification_persistence_blocked_count == entries.len()
        && test_probe_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
        status: if operator_decision_checklist_packet_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_SCHEMA_VERSION,
        plugin_id: checklist.plugin_id,
        source_operator_decision_checklist_gate: checklist.gate,
        source_operator_decision_checklist_ready: checklist.operator_decision_checklist_ready,
        source_decision_checklist_visible: checklist.decision_checklist_visible,
        source_decision_checklist_persisted: checklist.decision_checklist_persisted,
        source_decision_recorded: checklist.decision_recorded,
        source_operator_packet_sent: checklist.operator_packet_sent,
        source_operator_packet_persisted: checklist.operator_packet_persisted,
        source_packet_payload_persisted: checklist.packet_payload_persisted,
        source_readback_persisted: checklist.readback_persisted,
        source_owner_assignment_persisted: checklist.owner_assignment_persisted,
        source_freeze_applied: checklist.freeze_applied,
        source_classification_persisted: checklist.classification_persisted,
        source_test_probe_executed: checklist.test_probe_executed,
        source_checklist_entry_count: checklist.checklist_entry_count,
        source_tracked_change_count: checklist.source_tracked_change_count,
        source_untracked_change_count: checklist.source_untracked_change_count,
        packet_readback_scope:
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_scope(),
        packet_readback_entry_count: entries.len(),
        stable_packet_key_count,
        stable_readback_key_count,
        packet_route_count,
        readback_route_count,
        packet_readback_ready_count,
        checklist_attached_count,
        packet_visible_unsent_unpersisted_count,
        readback_visible_unpersisted_count,
        pending_operator_decision_count,
        decision_recording_blocked_count,
        approval_request_blocked_count,
        approval_acceptance_blocked_count,
        evidence_recorded_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        owner_assignment_blocked_count,
        freeze_application_blocked_count,
        classification_persistence_blocked_count,
        test_probe_blocked_count,
        packet_readback_visible: operator_decision_checklist_packet_readback_ready,
        packet_readback_persisted: false,
        decision_checklist_persisted: false,
        decision_recorded: false,
        operator_packet_sent: false,
        operator_packet_persisted: false,
        packet_payload_persisted: false,
        readback_persisted: false,
        owner_assignment_persisted: false,
        freeze_applied: false,
        classification_persisted: false,
        test_probe_executed: false,
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
        operator_decision_checklist_packet_readback_ready,
        entries,
        blockers: vec![
            "operator_decision_checklist_packet_send_blocked",
            "operator_decision_checklist_packet_persistence_blocked",
            "operator_decision_checklist_packet_payload_persistence_blocked",
            "operator_decision_checklist_packet_readback_persistence_blocked",
            "operator_decision_recording_blocked",
            "approval_request_blocked",
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
            "owner_assignment_persistence_blocked",
            "freeze_application_blocked",
            "classification_persistence_blocked",
            "test_probe_execution_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_scope()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackScope
{
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackScope {
        packet_readback_id: "dirty-worktree.release-boundary.owner-freeze-classification.operator-decision-checklist.packet-readback.v1",
        packet_readback_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist-packet/v1",
        source_checklist_route: "checklist://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision/v1",
        readback_mode: "operator_decision_checklist_packet_readback_only",
        packet_send_boundary: "blocked",
        packet_persistence_boundary: "blocked",
        readback_persistence_boundary: "blocked",
        decision_recording_boundary: "blocked",
        approval_boundary: "blocked",
        evidence_boundary: "blocked",
        owner_assignment_boundary: "blocked",
        freeze_application_boundary: "blocked",
        classification_persistence_boundary: "blocked",
        test_probe_boundary: "blocked",
        git_mutation_boundary: "closed",
        cleanup_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_entries(
    checklist: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistReport,
) -> Vec<
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackEntry,
>{
    checklist
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackEntry {
                source_checklist_key: entry.checklist_key.clone(),
                source_checklist_route: entry.checklist_route.clone(),
                packet_key: packet_key(entry.group_type, entry.source_bucket),
                packet_route: packet_route(entry.group_type, entry.source_bucket),
                readback_key: readback_key(entry.group_type, entry.source_bucket),
                readback_route: readback_route(entry.group_type, entry.source_bucket),
                decision_checkpoint: entry.decision_checkpoint.clone(),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_route: entry.owner_route.clone(),
                outcome_category: entry.outcome_category,
                packet_section: entry.packet_section,
                required_local_gate: entry.required_local_gate,
                operator_action: entry.operator_action,
                evidence_requirement: entry.evidence_requirement,
                decision_state: entry.decision_state,
                packet_state: "operator_decision_checklist_packet_visible_unsent_unpersisted",
                readback_state: "operator_decision_checklist_packet_readback_visible_unpersisted",
                checklist_attached: true,
                packet_visible: entry.operator_visible,
                packet_unsent: true,
                packet_unpersisted: true,
                readback_visible: true,
                readback_unpersisted: true,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                operator_decision_required: entry.operator_decision_required,
                decision_recording_allowed: false,
                approval_request_blocked: entry.approval_request_blocked,
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
                owner_assignment_blocked: entry.owner_assignment_blocked,
                freeze_application_blocked: entry.freeze_application_blocked,
                classification_persistence_blocked: entry.classification_persistence_blocked,
                test_probe_blocked: entry.test_probe_blocked,
                packet_send_blocked: entry.packet_send_blocked,
                packet_persistence_blocked: entry.packet_persistence_blocked,
                readback_persistence_blocked: entry.readback_persistence_blocked,
                release_cutover_allowed: false,
                canary_activation_allowed: false,
                live_execution_allowed: false,
            }
        })
        .collect()
}

fn packet_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.owner_freeze_classification_operator_decision_checklist_packet.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn packet_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn readback_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.owner_freeze_classification_operator_decision_checklist_packet_readback.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn readback_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist-packet/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            packet_sent: false,
            packet_persisted: false,
            packet_payload_persisted: false,
            packet_readback_persisted: false,
            decision_checklist_persisted: false,
            decision_recorded: false,
            readback_persisted: false,
            owner_assignment_persisted: false,
            freeze_applied: false,
            classification_persisted: false,
            test_probe_executed: false,
            git_add_performed: false,
            git_index_mutated: false,
            git_commit_created: false,
            git_push_performed: false,
            git_reset_performed: false,
            git_checkout_performed: false,
            git_revert_performed: false,
            cleanup_performed: false,
            unrelated_file_deleted: false,
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
    fn checklist_packet_readback_is_visible_but_unpersisted() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_operator_decision_checklist_ready);
        assert!(report.source_decision_checklist_visible);
        assert!(!report.source_decision_checklist_persisted);
        assert!(!report.source_decision_recorded);
        assert_eq!(
            report.packet_readback_entry_count,
            report.source_checklist_entry_count
        );
        assert_eq!(
            report.packet_readback_ready_count,
            report.packet_readback_entry_count
        );
        assert_eq!(
            report.packet_visible_unsent_unpersisted_count,
            report.packet_readback_entry_count
        );
        assert_eq!(
            report.readback_visible_unpersisted_count,
            report.packet_readback_entry_count
        );
        assert!(report.operator_decision_checklist_packet_readback_ready);
        assert!(report.packet_readback_visible);
        assert!(!report.packet_readback_persisted);
    }

    #[test]
    fn packet_readback_entries_preserve_pending_decisions_and_routes() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.packet_route
                == "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist/top-level/codex-rs"
            && entry.readback_route
                == "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist-packet/top-level/codex-rs"));
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.source_bucket == "hepta_systems_owned"
                    && entry.owner_route == "owner://release-boundary/hepta-systems")
        );
        assert!(report.entries.iter().all(|entry| entry.checklist_attached
            && entry.packet_visible
            && entry.packet_unsent
            && entry.packet_unpersisted
            && entry.readback_visible
            && entry.readback_unpersisted
            && entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.operator_decision_required
            && entry.decision_state == "pending_operator_decision"
            && entry.packet_state
                == "operator_decision_checklist_packet_visible_unsent_unpersisted"
            && entry.readback_state
                == "operator_decision_checklist_packet_readback_visible_unpersisted"
            && !entry.decision_recording_allowed
            && entry.approval_request_blocked
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
            && entry.owner_assignment_blocked
            && entry.freeze_application_blocked
            && entry.classification_persistence_blocked
            && entry.test_probe_blocked
            && entry.packet_send_blocked
            && entry.packet_persistence_blocked
            && entry.readback_persistence_blocked
            && !entry.release_cutover_allowed
            && !entry.canary_activation_allowed
            && !entry.live_execution_allowed));
    }

    #[test]
    fn checklist_packet_readback_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report();

        assert!(!report.decision_checklist_persisted);
        assert!(!report.decision_recorded);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
        assert!(!report.packet_payload_persisted);
        assert!(!report.readback_persisted);
        assert!(!report.owner_assignment_persisted);
        assert!(!report.freeze_applied);
        assert!(!report.classification_persisted);
        assert!(!report.test_probe_executed);
        assert!(!report.git_add_allowed);
        assert!(!report.git_index_mutated);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persistence_allowed);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert!(
            report
                .side_effects
                .eq(&DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackSideEffects::none())
        );
    }
}
