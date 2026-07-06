use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_GATE: &str =
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_RECOMMENDED_NEXT_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_outcome_gate: &'static str,
    pub source_outcome_ready: bool,
    pub source_outcome_visible: bool,
    pub source_outcome_persisted: bool,
    pub source_outcome_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub packet_id: &'static str,
    pub packet_route: &'static str,
    pub packet_payload_hash: &'static str,
    pub packet_entry_count: usize,
    pub stable_packet_key_count: usize,
    pub packet_route_count: usize,
    pub packet_ready_count: usize,
    pub visible_unsent_unpersisted_count: usize,
    pub attached_outcome_count: usize,
    pub owner_attribution_packet_count: usize,
    pub targeted_gate_packet_count: usize,
    pub owned_lane_freeze_packet_count: usize,
    pub artifact_classification_packet_count: usize,
    pub hepta_systems_owner_route_count: usize,
    pub cross_lane_owner_route_count: usize,
    pub operator_decision_required_count: usize,
    pub packet_send_blocked_count: usize,
    pub packet_persistence_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub approval_request_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
    pub operator_packet_visible: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub packet_payload_persisted: bool,
    pub readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
    pub evidence_recorded: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub decision_recorded: bool,
    pub strategy_applied: bool,
    pub git_index_mutated: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub release_cutover_allowed: bool,
    pub package_or_release_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub operator_packet_without_send_ready: bool,
    pub entries:
        Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry {
    pub source_outcome_key: String,
    pub source_outcome_route: String,
    pub packet_key: String,
    pub packet_route: String,
    pub non_send_readback_key: String,
    pub non_send_readback_route: String,
    pub source_bucket: &'static str,
    pub group_type: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_route: String,
    pub owner_state: &'static str,
    pub freeze_state: &'static str,
    pub classification_state: &'static str,
    pub outcome_category: &'static str,
    pub outcome_action: &'static str,
    pub packet_section: &'static str,
    pub packet_action: &'static str,
    pub required_local_gate: &'static str,
    pub release_disposition: &'static str,
    pub observed_state: &'static str,
    pub previous_send_state: &'static str,
    pub current_send_state: &'static str,
    pub send_state_delta: &'static str,
    pub previous_persistence_state: &'static str,
    pub current_persistence_state: &'static str,
    pub persistence_state_delta: &'static str,
    pub source_outcome_attached: bool,
    pub packet_visible: bool,
    pub packet_payload_visible: bool,
    pub non_send_confirmed: bool,
    pub non_persistence_confirmed: bool,
    pub operator_visible: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub operator_decision_required: bool,
    pub owner_assignment_blocked: bool,
    pub freeze_application_blocked: bool,
    pub classification_persistence_blocked: bool,
    pub test_probe_blocked: bool,
    pub packet_send_blocked: bool,
    pub packet_persistence_blocked: bool,
    pub readback_persistence_blocked: bool,
    pub approval_request_blocked: bool,
    pub approval_acceptance_blocked: bool,
    pub decision_recording_blocked: bool,
    pub evidence_recording_blocked: bool,
    pub git_mutation_blocked: bool,
    pub cleanup_delete_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects
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
    pub strategy_applied: bool,
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport {
    let source =
        dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report();
    dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report_from_outcome(&source)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report_from_outcome(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport {
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_entries(source);
    let stable_packet_key_count = entries
        .iter()
        .map(|entry| entry.packet_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let packet_route_count = entries
        .iter()
        .map(|entry| entry.packet_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let packet_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.source_outcome_attached
                && entry.packet_visible
                && entry.packet_payload_visible
                && entry.non_send_confirmed
                && entry.non_persistence_confirmed
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && entry.owner_assignment_blocked
                && entry.freeze_application_blocked
                && entry.classification_persistence_blocked
                && entry.test_probe_blocked
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.readback_persistence_blocked
                && entry.approval_request_blocked
                && entry.approval_acceptance_blocked
                && entry.decision_recording_blocked
                && entry.evidence_recording_blocked
                && entry.git_mutation_blocked
                && entry.cleanup_delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
                && !entry.packet_section.is_empty()
                && !entry.packet_action.is_empty()
                && !entry.required_local_gate.is_empty()
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
    let attached_outcome_count = entries
        .iter()
        .filter(|entry| {
            !entry.source_outcome_key.is_empty() && !entry.source_outcome_route.is_empty()
        })
        .count();
    let owner_attribution_packet_count =
        count_packet_section(&entries, "owner_attribution_packet_section");
    let targeted_gate_packet_count = count_packet_section(&entries, "targeted_gate_packet_section");
    let owned_lane_freeze_packet_count =
        count_packet_section(&entries, "owned_lane_freeze_packet_section");
    let artifact_classification_packet_count =
        count_packet_section(&entries, "artifact_classification_packet_section");
    let hepta_systems_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/hepta-systems");
    let cross_lane_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/cross-lane-review");
    let operator_decision_required_count = entries
        .iter()
        .filter(|entry| entry.operator_decision_required)
        .count();
    let packet_send_blocked_count = entries
        .iter()
        .filter(|entry| entry.packet_send_blocked)
        .count();
    let packet_persistence_blocked_count = entries
        .iter()
        .filter(|entry| entry.packet_persistence_blocked)
        .count();
    let git_mutation_blocked_count = entries
        .iter()
        .filter(|entry| entry.git_mutation_blocked)
        .count();
    let cleanup_delete_blocked_count = entries
        .iter()
        .filter(|entry| entry.cleanup_delete_blocked)
        .count();
    let evidence_recording_blocked_count = entries
        .iter()
        .filter(|entry| entry.evidence_recording_blocked)
        .count();
    let approval_request_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_request_blocked)
        .count();
    let approval_acceptance_blocked_count = entries
        .iter()
        .filter(|entry| entry.approval_acceptance_blocked)
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| entry.decision_recording_blocked)
        .count();
    let operator_packet_without_send_ready = source
        .owner_freeze_classification_outcome_readback_ready
        && source.outcome_readback_visible
        && !source.outcome_readback_persisted
        && !source.operator_packet_sent
        && !source.operator_packet_persisted
        && !source.git_index_mutated
        && !source.cleanup_allowed
        && !source.delete_allowed
        && !entries.is_empty()
        && entries.len() == source.outcome_entry_count
        && stable_packet_key_count == entries.len()
        && packet_route_count == entries.len()
        && packet_ready_count == entries.len()
        && visible_unsent_unpersisted_count == entries.len()
        && attached_outcome_count == entries.len()
        && owner_attribution_packet_count == source.owner_attribution_outcome_required_count
        && targeted_gate_packet_count == source.targeted_gate_outcome_required_count
        && owned_lane_freeze_packet_count == source.owned_lane_freeze_outcome_required_count
        && artifact_classification_packet_count
            == source.artifact_classification_outcome_required_count
        && hepta_systems_owner_route_count == source.hepta_systems_owner_route_count
        && cross_lane_owner_route_count == source.cross_lane_owner_route_count
        && operator_decision_required_count == entries.len()
        && packet_send_blocked_count == entries.len()
        && packet_persistence_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && approval_request_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send",
        status: if operator_packet_without_send_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_GATE,
        schema_version:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_outcome_gate: source.gate,
        source_outcome_ready: source.owner_freeze_classification_outcome_readback_ready,
        source_outcome_visible: source.outcome_readback_visible,
        source_outcome_persisted: source.outcome_readback_persisted,
        source_outcome_entry_count: source.outcome_entry_count,
        source_tracked_change_count: source.source_tracked_change_count,
        source_untracked_change_count: source.source_untracked_change_count,
        packet_id: "dirty-worktree-owner-freeze-classification-operator-packet",
        packet_route:
            "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/v1",
        packet_payload_hash:
            "sha256:dirty-worktree-owner-freeze-classification-operator-packet-no-send-no-live",
        packet_entry_count: entries.len(),
        stable_packet_key_count,
        packet_route_count,
        packet_ready_count,
        visible_unsent_unpersisted_count,
        attached_outcome_count,
        owner_attribution_packet_count,
        targeted_gate_packet_count,
        owned_lane_freeze_packet_count,
        artifact_classification_packet_count,
        hepta_systems_owner_route_count,
        cross_lane_owner_route_count,
        operator_decision_required_count,
        packet_send_blocked_count,
        packet_persistence_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        evidence_recording_blocked_count,
        approval_request_blocked_count,
        approval_acceptance_blocked_count,
        decision_recording_blocked_count,
        operator_packet_visible: operator_packet_without_send_ready,
        operator_packet_sent: false,
        operator_packet_persisted: false,
        packet_payload_persisted: false,
        readback_persisted: false,
        owner_assignment_persisted: false,
        freeze_applied: false,
        classification_persisted: false,
        test_probe_executed: false,
        evidence_recorded: false,
        approval_requested: false,
        approval_accepted: false,
        decision_recorded: false,
        strategy_applied: false,
        git_index_mutated: false,
        cleanup_allowed: false,
        delete_allowed: false,
        release_cutover_allowed: false,
        package_or_release_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        operator_packet_without_send_ready,
        entries,
        blockers: vec![
            "operator_packet_send_blocked",
            "operator_packet_persistence_blocked",
            "operator_packet_payload_persistence_blocked",
            "operator_packet_readback_persistence_blocked",
            "owner_assignment_persistence_blocked",
            "freeze_application_blocked",
            "classification_persistence_blocked",
            "test_probe_execution_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "approval_request_blocked",
            "approval_acceptance_blocked",
            "decision_recording_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects::none(),
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_entries(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport,
) -> Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry> {
    source
        .entries
        .iter()
        .map(|entry| DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry {
            source_outcome_key: entry.outcome_key.clone(),
            source_outcome_route: entry.outcome_route.clone(),
            packet_key: format!(
                "dirty_worktree.owner_freeze_classification_operator_packet.{}.{}",
                key_safe(entry.group_type),
                key_safe(entry.source_bucket)
            ),
            packet_route: format!(
                "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/{}/{}",
                route_group_type(entry.group_type),
                route_safe(entry.source_bucket)
            ),
            non_send_readback_key: format!(
                "dirty_worktree.owner_freeze_classification_operator_packet.non_send.{}.{}",
                key_safe(entry.group_type),
                key_safe(entry.source_bucket)
            ),
            non_send_readback_route: format!(
                "readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/non-send/{}/{}",
                route_group_type(entry.group_type),
                route_safe(entry.source_bucket)
            ),
            source_bucket: entry.source_bucket,
            group_type: entry.group_type,
            source_entry_count: entry.source_entry_count,
            tracked_count: entry.tracked_count,
            untracked_count: entry.untracked_count,
            owner_route: entry.owner_route.clone(),
            owner_state: entry.owner_state,
            freeze_state: entry.freeze_state,
            classification_state: entry.classification_state,
            outcome_category: entry.outcome_category,
            outcome_action: entry.outcome_action,
            packet_section: packet_section(entry.outcome_category),
            packet_action: packet_action(entry.outcome_category),
            required_local_gate: entry.required_local_gate,
            release_disposition: entry.release_disposition,
            observed_state: "operator_packet_visible_unsent_unpersisted",
            previous_send_state: "unsent",
            current_send_state: "unsent",
            send_state_delta: "unchanged_unsent",
            previous_persistence_state: "unpersisted",
            current_persistence_state: "unpersisted",
            persistence_state_delta: "unchanged_unpersisted",
            source_outcome_attached: !entry.outcome_key.is_empty() && !entry.outcome_route.is_empty(),
            packet_visible: true,
            packet_payload_visible: true,
            non_send_confirmed: true,
            non_persistence_confirmed: true,
            operator_visible: true,
            queryable: entry.queryable,
            diffable: entry.diffable,
            operator_decision_required: true,
            owner_assignment_blocked: true,
            freeze_application_blocked: true,
            classification_persistence_blocked: true,
            test_probe_blocked: !entry.test_probe_executed,
            packet_send_blocked: true,
            packet_persistence_blocked: true,
            readback_persistence_blocked: true,
            approval_request_blocked: true,
            approval_acceptance_blocked: !entry.approval_acceptance_allowed,
            decision_recording_blocked: !entry.decision_recording_allowed,
            evidence_recording_blocked: !entry.evidence_recording_allowed,
            git_mutation_blocked: entry.git_mutation_blocked,
            cleanup_delete_blocked: entry.cleanup_delete_blocked,
            release_cutover_allowed: false,
            canary_activation_allowed: false,
            live_execution_allowed: false,
        })
        .collect()
}

fn count_packet_section(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry],
    packet_section: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.packet_section == packet_section)
        .count()
}

fn count_owner_route(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendEntry],
    owner_route: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.owner_route == owner_route)
        .count()
}

fn packet_section(outcome_category: &str) -> &'static str {
    match outcome_category {
        "owner_attribution_outcome_required" => "owner_attribution_packet_section",
        "targeted_gate_outcome_required" => "targeted_gate_packet_section",
        "owned_lane_freeze_outcome_required" => "owned_lane_freeze_packet_section",
        "artifact_classification_outcome_required" => "artifact_classification_packet_section",
        _ => "bucket_review_packet_section",
    }
}

fn packet_action(outcome_category: &str) -> &'static str {
    match outcome_category {
        "owner_attribution_outcome_required" => {
            "include_owner_attribution_request_without_assignment"
        }
        "targeted_gate_outcome_required" => "include_targeted_gate_request_without_probe_execution",
        "owned_lane_freeze_outcome_required" => {
            "include_owned_lane_freeze_request_without_applying_freeze"
        }
        "artifact_classification_outcome_required" => {
            "include_artifact_classification_request_without_delete_or_relocation"
        }
        _ => "include_bucket_review_request_without_git_mutation",
    }
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects {
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
            strategy_applied: false,
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
    fn owner_freeze_classification_operator_packet_is_visible_unsent_unpersisted() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_outcome_ready);
        assert!(report.source_outcome_visible);
        assert!(!report.source_outcome_persisted);
        assert_eq!(report.packet_entry_count, report.source_outcome_entry_count);
        assert_eq!(report.packet_ready_count, report.packet_entry_count);
        assert_eq!(
            report.visible_unsent_unpersisted_count,
            report.packet_entry_count
        );
        assert_eq!(report.attached_outcome_count, report.packet_entry_count);
        assert_eq!(report.stable_packet_key_count, report.packet_entry_count);
        assert_eq!(report.packet_route_count, report.packet_entry_count);
        assert_eq!(report.owner_attribution_packet_count, 1);
        assert_eq!(report.owned_lane_freeze_packet_count, 1);
        assert_eq!(
            report.hepta_systems_owner_route_count + report.cross_lane_owner_route_count,
            report.packet_entry_count
        );
        assert!(report.operator_packet_visible);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
        assert!(!report.packet_payload_persisted);
        assert!(!report.readback_persisted);
        assert!(report.operator_packet_without_send_ready);
        assert_eq!(
            report.recommended_next_gate,
            "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation"
        );
    }

    #[test]
    fn owner_freeze_classification_operator_packet_entries_cover_outcomes() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "cross_lane_or_unowned"
                && entry.packet_section == "owner_attribution_packet_section"
                && entry.packet_action == "include_owner_attribution_request_without_assignment"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "hepta_systems_owned"
                && entry.packet_section == "owned_lane_freeze_packet_section"
                && entry.packet_action
                    == "include_owned_lane_freeze_request_without_applying_freeze"
        }));
        if let Some(artifact) = report
            .entries
            .iter()
            .find(|entry| entry.source_bucket == "artifacts")
        {
            assert_eq!(
                artifact.packet_section,
                "artifact_classification_packet_section"
            );
            assert_eq!(
                artifact.packet_action,
                "include_artifact_classification_request_without_delete_or_relocation"
            );
        }
        assert!(report.entries.iter().all(|entry| {
            entry.source_outcome_attached
                && entry.packet_visible
                && entry.packet_payload_visible
                && entry.non_send_confirmed
                && entry.non_persistence_confirmed
                && entry.operator_visible
                && entry.queryable
                && entry.diffable
                && entry.operator_decision_required
                && entry.owner_assignment_blocked
                && entry.freeze_application_blocked
                && entry.classification_persistence_blocked
                && entry.test_probe_blocked
                && entry.packet_send_blocked
                && entry.packet_persistence_blocked
                && entry.readback_persistence_blocked
                && entry.approval_request_blocked
                && entry.approval_acceptance_blocked
                && entry.decision_recording_blocked
                && entry.evidence_recording_blocked
                && entry.git_mutation_blocked
                && entry.cleanup_delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn owner_freeze_classification_operator_packet_keeps_side_effects_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report();

        assert!(!report.owner_assignment_persisted);
        assert!(!report.freeze_applied);
        assert!(!report.classification_persisted);
        assert!(!report.test_probe_executed);
        assert!(!report.evidence_recorded);
        assert!(!report.approval_requested);
        assert!(!report.approval_accepted);
        assert!(!report.decision_recorded);
        assert!(!report.strategy_applied);
        assert!(!report.git_index_mutated);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOperatorPacketWithoutSendSideEffects::none()
        );
    }
}
