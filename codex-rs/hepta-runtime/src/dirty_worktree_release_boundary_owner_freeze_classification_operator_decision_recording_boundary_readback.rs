use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_packet_readback_gate: &'static str,
    pub source_packet_readback_ready: bool,
    pub source_packet_readback_visible: bool,
    pub source_packet_readback_persisted: bool,
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
    pub source_packet_readback_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub decision_recording_boundary_scope:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackScope,
    pub boundary_entry_count: usize,
    pub stable_boundary_key_count: usize,
    pub boundary_route_count: usize,
    pub boundary_ready_count: usize,
    pub source_packet_attached_count: usize,
    pub pending_operator_decision_count: usize,
    pub decision_recording_blocked_count: usize,
    pub decision_recording_persistence_blocked_count: usize,
    pub decision_receipt_blocked_count: usize,
    pub approval_request_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub evidence_recorded_count: usize,
    pub packet_visible_unsent_unpersisted_count: usize,
    pub readback_unpersisted_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub owner_assignment_blocked_count: usize,
    pub freeze_application_blocked_count: usize,
    pub classification_persistence_blocked_count: usize,
    pub test_probe_blocked_count: usize,
    pub packet_send_blocked_count: usize,
    pub packet_persistence_blocked_count: usize,
    pub readback_persistence_blocked_count: usize,
    pub decision_recording_boundary_readback_visible: bool,
    pub decision_recording_boundary_readback_persisted: bool,
    pub decision_recorded: bool,
    pub decision_recording_persisted: bool,
    pub decision_receipt_persisted: bool,
    pub decision_checklist_persisted: bool,
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
    pub operator_decision_recording_boundary_readback_ready: bool,
    pub entries: Vec<
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackEntry,
    >,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackScope
{
    pub boundary_readback_id: &'static str,
    pub boundary_readback_route: &'static str,
    pub source_packet_readback_route: &'static str,
    pub readback_mode: &'static str,
    pub decision_recording_boundary: &'static str,
    pub decision_persistence_boundary: &'static str,
    pub decision_receipt_boundary: &'static str,
    pub approval_request_boundary: &'static str,
    pub approval_acceptance_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub owner_assignment_boundary: &'static str,
    pub freeze_application_boundary: &'static str,
    pub classification_persistence_boundary: &'static str,
    pub test_probe_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackEntry
{
    pub source_packet_key: String,
    pub source_packet_route: String,
    pub source_packet_readback_key: String,
    pub source_packet_readback_route: String,
    pub boundary_key: String,
    pub boundary_route: String,
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
    pub recording_state: &'static str,
    pub persistence_state: &'static str,
    pub receipt_state: &'static str,
    pub source_packet_state: &'static str,
    pub source_readback_state: &'static str,
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
    pub decision_persistence_allowed: bool,
    pub decision_receipt_persistence_allowed: bool,
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
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackSideEffects
{
    pub boundary_readback_persisted: bool,
    pub decision_recorded: bool,
    pub decision_recording_persisted: bool,
    pub decision_receipt_persisted: bool,
    pub decision_checklist_persisted: bool,
    pub packet_sent: bool,
    pub packet_persisted: bool,
    pub packet_payload_persisted: bool,
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

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackReport
{
    let packet_readback =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report();
    dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report_from_packet_readback(&packet_readback)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report_from_packet_readback(
    packet_readback: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackReport
{
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_entries(packet_readback);
    let stable_boundary_key_count = entries
        .iter()
        .map(|entry| entry.boundary_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let boundary_route_count = entries
        .iter()
        .map(|entry| entry.boundary_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let boundary_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.packet_visible
                && entry.packet_unsent
                && entry.packet_unpersisted
                && entry.readback_visible
                && entry.readback_unpersisted
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && entry.decision_state == "pending_operator_decision"
                && entry.recording_state == "decision_recording_blocked"
                && entry.persistence_state == "decision_persistence_blocked"
                && entry.receipt_state == "decision_receipt_blocked"
                && !entry.decision_recording_allowed
                && !entry.decision_persistence_allowed
                && !entry.decision_receipt_persistence_allowed
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
    let source_packet_attached_count = entries
        .iter()
        .filter(|entry| {
            !entry.source_packet_key.is_empty()
                && !entry.source_packet_route.is_empty()
                && !entry.source_packet_readback_key.is_empty()
                && !entry.source_packet_readback_route.is_empty()
        })
        .count();
    let pending_operator_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_state == "pending_operator_decision")
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_recording_allowed)
        .count();
    let decision_recording_persistence_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_persistence_allowed)
        .count();
    let decision_receipt_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_receipt_persistence_allowed)
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
    let packet_visible_unsent_unpersisted_count = entries
        .iter()
        .filter(|entry| entry.packet_visible && entry.packet_unsent && entry.packet_unpersisted)
        .count();
    let readback_unpersisted_count = entries
        .iter()
        .filter(|entry| entry.readback_visible && entry.readback_unpersisted)
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
    let packet_send_blocked_count = entries
        .iter()
        .filter(|entry| entry.packet_send_blocked)
        .count();
    let packet_persistence_blocked_count = entries
        .iter()
        .filter(|entry| entry.packet_persistence_blocked)
        .count();
    let readback_persistence_blocked_count = entries
        .iter()
        .filter(|entry| entry.readback_persistence_blocked)
        .count();

    let operator_decision_recording_boundary_readback_ready = packet_readback
        .operator_decision_checklist_packet_readback_ready
        && packet_readback.packet_readback_visible
        && !packet_readback.packet_readback_persisted
        && !packet_readback.decision_checklist_persisted
        && !packet_readback.decision_recorded
        && !packet_readback.operator_packet_sent
        && !packet_readback.operator_packet_persisted
        && !packet_readback.packet_payload_persisted
        && !packet_readback.readback_persisted
        && !packet_readback.owner_assignment_persisted
        && !packet_readback.freeze_applied
        && !packet_readback.classification_persisted
        && !packet_readback.test_probe_executed
        && entries.len() == packet_readback.packet_readback_entry_count
        && stable_boundary_key_count == entries.len()
        && boundary_route_count == entries.len()
        && boundary_ready_count == entries.len()
        && source_packet_attached_count == entries.len()
        && pending_operator_decision_count == entries.len()
        && decision_recording_blocked_count == entries.len()
        && decision_recording_persistence_blocked_count == entries.len()
        && decision_receipt_blocked_count == entries.len()
        && approval_request_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && evidence_recorded_count == 0
        && packet_visible_unsent_unpersisted_count == entries.len()
        && readback_unpersisted_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && owner_assignment_blocked_count == entries.len()
        && freeze_application_blocked_count == entries.len()
        && classification_persistence_blocked_count == entries.len()
        && test_probe_blocked_count == entries.len()
        && packet_send_blocked_count == entries.len()
        && packet_persistence_blocked_count == entries.len()
        && readback_persistence_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording",
        status: if operator_decision_recording_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: packet_readback.plugin_id,
        source_packet_readback_gate: packet_readback.gate,
        source_packet_readback_ready: packet_readback
            .operator_decision_checklist_packet_readback_ready,
        source_packet_readback_visible: packet_readback.packet_readback_visible,
        source_packet_readback_persisted: packet_readback.packet_readback_persisted,
        source_decision_checklist_persisted: packet_readback.decision_checklist_persisted,
        source_decision_recorded: packet_readback.decision_recorded,
        source_operator_packet_sent: packet_readback.operator_packet_sent,
        source_operator_packet_persisted: packet_readback.operator_packet_persisted,
        source_packet_payload_persisted: packet_readback.packet_payload_persisted,
        source_readback_persisted: packet_readback.readback_persisted,
        source_owner_assignment_persisted: packet_readback.owner_assignment_persisted,
        source_freeze_applied: packet_readback.freeze_applied,
        source_classification_persisted: packet_readback.classification_persisted,
        source_test_probe_executed: packet_readback.test_probe_executed,
        source_packet_readback_entry_count: packet_readback.packet_readback_entry_count,
        source_tracked_change_count: packet_readback.source_tracked_change_count,
        source_untracked_change_count: packet_readback.source_untracked_change_count,
        decision_recording_boundary_scope:
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_scope(),
        boundary_entry_count: entries.len(),
        stable_boundary_key_count,
        boundary_route_count,
        boundary_ready_count,
        source_packet_attached_count,
        pending_operator_decision_count,
        decision_recording_blocked_count,
        decision_recording_persistence_blocked_count,
        decision_receipt_blocked_count,
        approval_request_blocked_count,
        approval_acceptance_blocked_count,
        evidence_recorded_count,
        packet_visible_unsent_unpersisted_count,
        readback_unpersisted_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        owner_assignment_blocked_count,
        freeze_application_blocked_count,
        classification_persistence_blocked_count,
        test_probe_blocked_count,
        packet_send_blocked_count,
        packet_persistence_blocked_count,
        readback_persistence_blocked_count,
        decision_recording_boundary_readback_visible:
            operator_decision_recording_boundary_readback_ready,
        decision_recording_boundary_readback_persisted: false,
        decision_recorded: false,
        decision_recording_persisted: false,
        decision_receipt_persisted: false,
        decision_checklist_persisted: false,
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
        operator_decision_recording_boundary_readback_ready,
        entries,
        blockers: vec![
            "operator_decision_recording_blocked",
            "operator_decision_recording_persistence_blocked",
            "operator_decision_receipt_persistence_blocked",
            "approval_request_blocked",
            "approval_acceptance_blocked",
            "evidence_recording_blocked",
            "operator_decision_checklist_packet_send_blocked",
            "operator_decision_checklist_packet_persistence_blocked",
            "operator_decision_checklist_packet_payload_persistence_blocked",
            "operator_decision_recording_boundary_readback_persistence_blocked",
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
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_scope()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackScope
{
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackScope {
        boundary_readback_id: "dirty-worktree.release-boundary.owner-freeze-classification.operator-decision-recording-boundary-readback.v1",
        boundary_readback_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-recording-boundary/v1",
        source_packet_readback_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-checklist-packet/v1",
        readback_mode: "operator_decision_recording_boundary_readback_only",
        decision_recording_boundary: "blocked",
        decision_persistence_boundary: "blocked",
        decision_receipt_boundary: "blocked",
        approval_request_boundary: "blocked",
        approval_acceptance_boundary: "blocked",
        evidence_boundary: "blocked",
        owner_assignment_boundary: "blocked",
        freeze_application_boundary: "blocked",
        classification_persistence_boundary: "blocked",
        test_probe_boundary: "blocked",
        git_mutation_boundary: "closed",
        cleanup_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_entries(
    packet_readback: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionChecklistPacketReadbackReport,
) -> Vec<
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackEntry,
>{
    packet_readback
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackEntry {
                source_packet_key: entry.packet_key.clone(),
                source_packet_route: entry.packet_route.clone(),
                source_packet_readback_key: entry.readback_key.clone(),
                source_packet_readback_route: entry.readback_route.clone(),
                boundary_key: boundary_key(entry.group_type, entry.source_bucket),
                boundary_route: boundary_route(entry.group_type, entry.source_bucket),
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
                recording_state: "decision_recording_blocked",
                persistence_state: "decision_persistence_blocked",
                receipt_state: "decision_receipt_blocked",
                source_packet_state: entry.packet_state,
                source_readback_state: entry.readback_state,
                packet_visible: entry.packet_visible,
                packet_unsent: entry.packet_unsent,
                packet_unpersisted: entry.packet_unpersisted,
                readback_visible: entry.readback_visible,
                readback_unpersisted: entry.readback_unpersisted,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                operator_decision_required: entry.operator_decision_required,
                decision_recording_allowed: false,
                decision_persistence_allowed: false,
                decision_receipt_persistence_allowed: false,
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

fn boundary_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.owner_freeze_classification_operator_decision_recording_boundary.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn boundary_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-recording-boundary/{}/{}",
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            boundary_readback_persisted: false,
            decision_recorded: false,
            decision_recording_persisted: false,
            decision_receipt_persisted: false,
            decision_checklist_persisted: false,
            packet_sent: false,
            packet_persisted: false,
            packet_payload_persisted: false,
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
    fn owner_freeze_classification_decision_recording_boundary_is_ready_blocked() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_packet_readback_ready);
        assert!(report.source_packet_readback_visible);
        assert!(!report.source_packet_readback_persisted);
        assert!(!report.source_decision_recorded);
        assert_eq!(
            report.boundary_entry_count,
            report.source_packet_readback_entry_count
        );
        assert_eq!(report.boundary_ready_count, report.boundary_entry_count);
        assert_eq!(
            report.decision_recording_blocked_count,
            report.boundary_entry_count
        );
        assert_eq!(
            report.decision_recording_persistence_blocked_count,
            report.boundary_entry_count
        );
        assert_eq!(
            report.decision_receipt_blocked_count,
            report.boundary_entry_count
        );
        assert!(report.operator_decision_recording_boundary_readback_ready);
        assert!(report.decision_recording_boundary_readback_visible);
        assert!(!report.decision_recording_boundary_readback_persisted);
    }

    #[test]
    fn owner_freeze_classification_decision_recording_boundary_entries_are_stable_and_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.boundary_key
                == "dirty_worktree.owner_freeze_classification_operator_decision_recording_boundary.top_level.codex_rs"
            && entry.boundary_route
                == "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision-recording-boundary/top-level/codex-rs"));
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.source_bucket == "hepta_systems_owned"
                    && entry.owner_route == "owner://release-boundary/hepta-systems")
        );
        assert!(report.entries.iter().all(|entry| entry.packet_visible
            && entry.packet_unsent
            && entry.packet_unpersisted
            && entry.readback_visible
            && entry.readback_unpersisted
            && entry.operator_visible
            && entry.queryable
            && entry.diffable
            && entry.operator_decision_required
            && entry.decision_state == "pending_operator_decision"
            && entry.recording_state == "decision_recording_blocked"
            && entry.persistence_state == "decision_persistence_blocked"
            && entry.receipt_state == "decision_receipt_blocked"
            && entry.source_packet_state
                == "operator_decision_checklist_packet_visible_unsent_unpersisted"
            && entry.source_readback_state
                == "operator_decision_checklist_packet_readback_visible_unpersisted"
            && !entry.decision_recording_allowed
            && !entry.decision_persistence_allowed
            && !entry.decision_receipt_persistence_allowed
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
    fn owner_freeze_classification_decision_recording_boundary_has_no_side_effects() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report();

        assert!(!report.decision_recorded);
        assert!(!report.decision_recording_persisted);
        assert!(!report.decision_receipt_persisted);
        assert!(!report.decision_checklist_persisted);
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
            report.side_effects
                == DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorDecisionRecordingBoundaryReadbackSideEffects::none()
        );
    }
}
