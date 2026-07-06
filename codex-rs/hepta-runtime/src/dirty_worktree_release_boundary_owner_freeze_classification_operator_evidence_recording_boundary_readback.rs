use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorApprovalAcceptanceBoundaryReadbackReport;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_append_only_event_store_minimal_local_persistence";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_approval_acceptance_boundary_gate: &'static str,
    pub source_approval_acceptance_boundary_ready: bool,
    pub source_approval_acceptance_boundary_visible: bool,
    pub source_approval_acceptance_boundary_persisted: bool,
    pub source_approval_request_sent: bool,
    pub source_approval_accepted: bool,
    pub source_approval_recorded: bool,
    pub source_approval_receipt_persisted: bool,
    pub source_decision_recorded: bool,
    pub source_decision_recording_persisted: bool,
    pub source_decision_receipt_persisted: bool,
    pub source_operator_packet_sent: bool,
    pub source_operator_packet_persisted: bool,
    pub source_packet_payload_persisted: bool,
    pub source_readback_persisted: bool,
    pub source_owner_assignment_persisted: bool,
    pub source_freeze_applied: bool,
    pub source_classification_persisted: bool,
    pub source_test_probe_executed: bool,
    pub source_boundary_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub evidence_recording_boundary_scope:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackScope,
    pub boundary_entry_count: usize,
    pub stable_boundary_key_count: usize,
    pub boundary_route_count: usize,
    pub boundary_ready_count: usize,
    pub source_boundary_attached_count: usize,
    pub pending_operator_decision_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub evidence_persistence_blocked_count: usize,
    pub evidence_recorded_count: usize,
    pub approval_request_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub approval_recording_blocked_count: usize,
    pub approval_receipt_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
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
    pub evidence_recording_boundary_readback_visible: bool,
    pub evidence_recording_boundary_readback_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_persisted: bool,
    pub evidence_receipt_persisted: bool,
    pub approval_request_sent: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_receipt_persisted: bool,
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
    pub blocker_waiver_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub operator_evidence_recording_boundary_readback_ready: bool,
    pub entries: Vec<
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackEntry,
    >,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackScope
{
    pub boundary_readback_id: &'static str,
    pub boundary_readback_route: &'static str,
    pub source_approval_acceptance_boundary_route: &'static str,
    pub readback_mode: &'static str,
    pub evidence_recording_boundary: &'static str,
    pub evidence_persistence_boundary: &'static str,
    pub evidence_receipt_boundary: &'static str,
    pub approval_acceptance_boundary: &'static str,
    pub decision_recording_boundary: &'static str,
    pub owner_assignment_boundary: &'static str,
    pub freeze_application_boundary: &'static str,
    pub classification_persistence_boundary: &'static str,
    pub test_probe_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackEntry
{
    pub source_approval_boundary_key: String,
    pub source_approval_boundary_route: String,
    pub source_packet_key: String,
    pub source_packet_route: String,
    pub source_packet_readback_key: String,
    pub source_packet_readback_route: String,
    pub evidence_boundary_key: String,
    pub evidence_boundary_route: String,
    pub evidence_checkpoint: String,
    pub approval_checkpoint: String,
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
    pub evidence_recording_state: &'static str,
    pub evidence_persistence_state: &'static str,
    pub evidence_receipt_state: &'static str,
    pub approval_request_state: &'static str,
    pub approval_acceptance_state: &'static str,
    pub approval_recording_state: &'static str,
    pub approval_receipt_state: &'static str,
    pub decision_recording_state: &'static str,
    pub source_boundary_state: &'static str,
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
    pub evidence_recording_allowed: bool,
    pub evidence_persistence_allowed: bool,
    pub evidence_receipt_persistence_allowed: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub approval_recording_allowed: bool,
    pub approval_receipt_persistence_allowed: bool,
    pub decision_recording_allowed: bool,
    pub decision_persistence_allowed: bool,
    pub decision_receipt_persistence_allowed: bool,
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
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackSideEffects
{
    pub boundary_readback_persisted: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub evidence_receipt_persisted: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub approval_receipt_persisted: bool,
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
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackReport
{
    let source =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_report();
    dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report_from_approval_acceptance_boundary(&source)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report_from_approval_acceptance_boundary(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorApprovalAcceptanceBoundaryReadbackReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackReport
{
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_entries(source);
    let stable_boundary_key_count = entries
        .iter()
        .map(|entry| entry.evidence_boundary_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let boundary_route_count = entries
        .iter()
        .map(|entry| entry.evidence_boundary_route.as_str())
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
                && entry.evidence_recording_state == "evidence_recording_blocked"
                && entry.evidence_persistence_state == "evidence_persistence_blocked"
                && entry.evidence_receipt_state == "evidence_receipt_blocked"
                && entry.approval_request_state == "approval_request_blocked"
                && entry.approval_acceptance_state == "approval_acceptance_blocked"
                && entry.approval_recording_state == "approval_recording_blocked"
                && entry.approval_receipt_state == "approval_receipt_blocked"
                && entry.decision_recording_state == "decision_recording_blocked"
                && entry.source_boundary_state == "approval_acceptance_boundary_visible_unpersisted"
                && !entry.evidence_recording_allowed
                && !entry.evidence_persistence_allowed
                && !entry.evidence_receipt_persistence_allowed
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.approval_recording_allowed
                && !entry.approval_receipt_persistence_allowed
                && !entry.decision_recording_allowed
                && !entry.decision_persistence_allowed
                && !entry.decision_receipt_persistence_allowed
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
    let source_boundary_attached_count = entries
        .iter()
        .filter(|entry| {
            !entry.source_approval_boundary_key.is_empty()
                && !entry.source_approval_boundary_route.is_empty()
                && !entry.source_packet_key.is_empty()
                && !entry.source_packet_route.is_empty()
                && !entry.source_packet_readback_key.is_empty()
                && !entry.source_packet_readback_route.is_empty()
        })
        .count();
    let pending_operator_decision_count = entries
        .iter()
        .filter(|entry| entry.decision_state == "pending_operator_decision")
        .count();
    let evidence_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.evidence_recording_allowed)
        .count();
    let evidence_persistence_blocked_count = entries
        .iter()
        .filter(|entry| !entry.evidence_persistence_allowed)
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_allowed)
        .count();
    let approval_request_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_request_allowed)
        .count();
    let approval_acceptance_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_acceptance_allowed)
        .count();
    let approval_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_recording_allowed)
        .count();
    let approval_receipt_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_receipt_persistence_allowed)
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_recording_allowed)
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

    let operator_evidence_recording_boundary_readback_ready = source
        .operator_approval_acceptance_boundary_readback_ready
        && source.approval_acceptance_boundary_readback_visible
        && !source.approval_acceptance_boundary_readback_persisted
        && !source.approval_request_sent
        && !source.approval_accepted
        && !source.approval_recorded
        && !source.approval_receipt_persisted
        && !source.decision_recorded
        && !source.decision_recording_persisted
        && !source.decision_receipt_persisted
        && !source.operator_packet_sent
        && !source.operator_packet_persisted
        && !source.packet_payload_persisted
        && !source.readback_persisted
        && !source.owner_assignment_persisted
        && !source.freeze_applied
        && !source.classification_persisted
        && !source.test_probe_executed
        && entries.len() == source.boundary_entry_count
        && stable_boundary_key_count == entries.len()
        && boundary_route_count == entries.len()
        && boundary_ready_count == entries.len()
        && source_boundary_attached_count == entries.len()
        && pending_operator_decision_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && evidence_persistence_blocked_count == entries.len()
        && evidence_recorded_count == 0
        && approval_request_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && approval_recording_blocked_count == entries.len()
        && approval_receipt_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len()
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

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackReport {
        runtime: "hepta",
        surface:
            "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording",
        status: if operator_evidence_recording_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_approval_acceptance_boundary_gate: source.gate,
        source_approval_acceptance_boundary_ready: source
            .operator_approval_acceptance_boundary_readback_ready,
        source_approval_acceptance_boundary_visible: source
            .approval_acceptance_boundary_readback_visible,
        source_approval_acceptance_boundary_persisted: source
            .approval_acceptance_boundary_readback_persisted,
        source_approval_request_sent: source.approval_request_sent,
        source_approval_accepted: source.approval_accepted,
        source_approval_recorded: source.approval_recorded,
        source_approval_receipt_persisted: source.approval_receipt_persisted,
        source_decision_recorded: source.decision_recorded,
        source_decision_recording_persisted: source.decision_recording_persisted,
        source_decision_receipt_persisted: source.decision_receipt_persisted,
        source_operator_packet_sent: source.operator_packet_sent,
        source_operator_packet_persisted: source.operator_packet_persisted,
        source_packet_payload_persisted: source.packet_payload_persisted,
        source_readback_persisted: source.readback_persisted,
        source_owner_assignment_persisted: source.owner_assignment_persisted,
        source_freeze_applied: source.freeze_applied,
        source_classification_persisted: source.classification_persisted,
        source_test_probe_executed: source.test_probe_executed,
        source_boundary_entry_count: source.boundary_entry_count,
        source_tracked_change_count: source.source_tracked_change_count,
        source_untracked_change_count: source.source_untracked_change_count,
        evidence_recording_boundary_scope:
            dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_scope(),
        boundary_entry_count: entries.len(),
        stable_boundary_key_count,
        boundary_route_count,
        boundary_ready_count,
        source_boundary_attached_count,
        pending_operator_decision_count,
        evidence_recording_blocked_count,
        evidence_persistence_blocked_count,
        evidence_recorded_count,
        approval_request_blocked_count,
        approval_acceptance_blocked_count,
        approval_recording_blocked_count,
        approval_receipt_blocked_count,
        decision_recording_blocked_count,
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
        evidence_recording_boundary_readback_visible:
            operator_evidence_recording_boundary_readback_ready,
        evidence_recording_boundary_readback_persisted: false,
        evidence_recorded: false,
        evidence_recording_persisted: false,
        evidence_receipt_persisted: false,
        approval_request_sent: false,
        approval_accepted: false,
        approval_recorded: false,
        approval_receipt_persisted: false,
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
        blocker_waiver_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        operator_evidence_recording_boundary_readback_ready,
        entries,
        blockers: vec![
            "evidence_recording_blocked",
            "evidence_persistence_blocked",
            "evidence_receipt_persistence_blocked",
            "approval_request_blocked",
            "approval_acceptance_blocked",
            "approval_recording_blocked",
            "approval_receipt_persistence_blocked",
            "operator_decision_recording_blocked",
            "operator_decision_recording_persistence_blocked",
            "operator_decision_receipt_persistence_blocked",
            "operator_evidence_recording_boundary_readback_persistence_blocked",
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
            "packet_send_blocked",
            "packet_persistence_blocked",
            "readback_persistence_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_scope()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackScope{
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackScope {
        boundary_readback_id: "dirty-worktree.release-boundary.owner-freeze-classification.operator-evidence-recording-boundary-readback.v1",
        boundary_readback_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-evidence-recording-boundary/v1",
        source_approval_acceptance_boundary_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-approval-acceptance-boundary/v1",
        readback_mode: "operator_evidence_recording_boundary_readback_only",
        evidence_recording_boundary: "blocked",
        evidence_persistence_boundary: "blocked",
        evidence_receipt_boundary: "blocked",
        approval_acceptance_boundary: "blocked",
        decision_recording_boundary: "blocked",
        owner_assignment_boundary: "blocked",
        freeze_application_boundary: "blocked",
        classification_persistence_boundary: "blocked",
        test_probe_boundary: "blocked",
        git_mutation_boundary: "blocked",
        cleanup_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_entries(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorApprovalAcceptanceBoundaryReadbackReport,
) -> Vec<
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackEntry,
>{
    source
        .entries
        .iter()
        .map(|entry| {
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackEntry {
                source_approval_boundary_key: entry.approval_boundary_key.clone(),
                source_approval_boundary_route: entry.approval_boundary_route.clone(),
                source_packet_key: entry.source_packet_key.clone(),
                source_packet_route: entry.source_packet_route.clone(),
                source_packet_readback_key: entry.source_packet_readback_key.clone(),
                source_packet_readback_route: entry.source_packet_readback_route.clone(),
                evidence_boundary_key: evidence_boundary_key(entry.group_type, entry.source_bucket),
                evidence_boundary_route: evidence_boundary_route(
                    entry.group_type,
                    entry.source_bucket,
                ),
                evidence_checkpoint: evidence_checkpoint(&entry.approval_checkpoint),
                approval_checkpoint: entry.approval_checkpoint.clone(),
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
                evidence_recording_state: "evidence_recording_blocked",
                evidence_persistence_state: "evidence_persistence_blocked",
                evidence_receipt_state: "evidence_receipt_blocked",
                approval_request_state: entry.approval_request_state,
                approval_acceptance_state: entry.approval_acceptance_state,
                approval_recording_state: entry.approval_recording_state,
                approval_receipt_state: entry.approval_receipt_state,
                decision_recording_state: entry.decision_recording_state,
                source_boundary_state: "approval_acceptance_boundary_visible_unpersisted",
                source_packet_state: entry.source_packet_state,
                source_readback_state: entry.source_readback_state,
                packet_visible: entry.packet_visible,
                packet_unsent: entry.packet_unsent,
                packet_unpersisted: entry.packet_unpersisted,
                readback_visible: entry.readback_visible,
                readback_unpersisted: entry.readback_unpersisted,
                operator_visible: entry.operator_visible,
                queryable: entry.queryable,
                diffable: entry.diffable,
                operator_decision_required: entry.operator_decision_required,
                evidence_recording_allowed: false,
                evidence_persistence_allowed: false,
                evidence_receipt_persistence_allowed: false,
                approval_request_allowed: false,
                approval_acceptance_allowed: false,
                approval_recording_allowed: false,
                approval_receipt_persistence_allowed: false,
                decision_recording_allowed: false,
                decision_persistence_allowed: false,
                decision_receipt_persistence_allowed: false,
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

fn evidence_boundary_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.owner_freeze_classification_operator_evidence_recording_boundary.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn evidence_boundary_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-evidence-recording-boundary/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn evidence_checkpoint(approval_checkpoint: &str) -> String {
    format!("evidence_recording_boundary.{approval_checkpoint}")
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            boundary_readback_persisted: false,
            evidence_recorded: false,
            evidence_persisted: false,
            evidence_receipt_persisted: false,
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            approval_receipt_persisted: false,
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
    fn evidence_recording_boundary_readback_is_visible_but_unpersisted() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_approval_acceptance_boundary_ready);
        assert!(report.source_approval_acceptance_boundary_visible);
        assert!(!report.source_approval_acceptance_boundary_persisted);
        assert!(!report.source_approval_accepted);
        assert_eq!(
            report.boundary_entry_count,
            report.source_boundary_entry_count
        );
        assert_eq!(report.boundary_ready_count, report.boundary_entry_count);
        assert_eq!(
            report.evidence_recording_blocked_count,
            report.boundary_entry_count
        );
        assert_eq!(
            report.evidence_persistence_blocked_count,
            report.boundary_entry_count
        );
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.operator_evidence_recording_boundary_readback_ready);
        assert!(report.evidence_recording_boundary_readback_visible);
        assert!(!report.evidence_recording_boundary_readback_persisted);
    }

    #[test]
    fn boundary_entries_preserve_pending_decisions_and_block_evidence_recording() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report();

        assert!(report.entries.iter().any(|entry| entry.source_bucket == "codex-rs"
            && entry.evidence_boundary_key
                == "dirty_worktree.owner_freeze_classification_operator_evidence_recording_boundary.top_level.codex_rs"
            && entry.evidence_boundary_route
                == "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-evidence-recording-boundary/top-level/codex-rs"));
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
            && entry.evidence_recording_state == "evidence_recording_blocked"
            && entry.evidence_persistence_state == "evidence_persistence_blocked"
            && entry.evidence_receipt_state == "evidence_receipt_blocked"
            && entry.approval_acceptance_state == "approval_acceptance_blocked"
            && entry.source_boundary_state == "approval_acceptance_boundary_visible_unpersisted"
            && !entry.evidence_recording_allowed
            && !entry.evidence_persistence_allowed
            && !entry.evidence_receipt_persistence_allowed
            && !entry.approval_request_allowed
            && !entry.approval_acceptance_allowed
            && !entry.approval_recording_allowed
            && !entry.approval_receipt_persistence_allowed
            && !entry.decision_recording_allowed
            && !entry.decision_persistence_allowed
            && !entry.decision_receipt_persistence_allowed
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
    fn evidence_recording_boundary_side_effects_are_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report();

        assert!(!report.evidence_recorded);
        assert!(!report.evidence_recording_persisted);
        assert!(!report.evidence_receipt_persisted);
        assert!(!report.approval_request_sent);
        assert!(!report.approval_accepted);
        assert!(!report.approval_recorded);
        assert!(!report.approval_receipt_persisted);
        assert!(!report.decision_recorded);
        assert!(!report.decision_recording_persisted);
        assert!(!report.decision_receipt_persisted);
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
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert!(report.side_effects.eq(
            &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorEvidenceRecordingBoundaryReadbackSideEffects::none()
        ));
    }
}
