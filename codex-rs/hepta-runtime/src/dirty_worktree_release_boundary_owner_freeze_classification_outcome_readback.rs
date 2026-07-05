use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport;
use crate::dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_GATE: &str = "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_rehearsal_gate: &'static str,
    pub source_rehearsal_ready: bool,
    pub source_rehearsal_visible: bool,
    pub source_rehearsal_persisted: bool,
    pub source_classification_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub outcome_entry_count: usize,
    pub stable_outcome_key_count: usize,
    pub outcome_route_count: usize,
    pub outcome_ready_count: usize,
    pub owner_attribution_outcome_required_count: usize,
    pub targeted_gate_outcome_required_count: usize,
    pub owned_lane_freeze_outcome_required_count: usize,
    pub artifact_classification_outcome_required_count: usize,
    pub hepta_systems_owner_route_count: usize,
    pub cross_lane_owner_route_count: usize,
    pub release_blocked_count: usize,
    pub test_probe_execution_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
    pub outcome_readback_visible: bool,
    pub outcome_readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
    pub evidence_recorded: bool,
    pub approval_accepted: bool,
    pub decision_recorded: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
    pub strategy_applied: bool,
    pub git_index_mutated: bool,
    pub cleanup_allowed: bool,
    pub delete_allowed: bool,
    pub release_cutover_allowed: bool,
    pub package_or_release_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub owner_freeze_classification_outcome_readback_ready: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry {
    pub source_classification_key: String,
    pub source_classification_route: String,
    pub outcome_key: String,
    pub outcome_route: String,
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
    pub release_disposition: &'static str,
    pub required_local_gate: &'static str,
    pub operator_packet_candidate: bool,
    pub source_rehearsal_attached: bool,
    pub outcome_readback_visible: bool,
    pub outcome_readback_persisted: bool,
    pub queryable: bool,
    pub diffable: bool,
    pub test_probe_executed: bool,
    pub mutation_free: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub evidence_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub decision_recording_allowed: bool,
    pub git_mutation_blocked: bool,
    pub cleanup_delete_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackSideEffects {
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
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
    pub operator_packet_sent: bool,
    pub operator_packet_persisted: bool,
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
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport {
    let source = dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report();
    dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report_from_rehearsal(&source)
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report_from_rehearsal(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport {
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_entries(
            source,
        );
    let stable_outcome_key_count = entries
        .iter()
        .map(|entry| entry.outcome_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let outcome_route_count = entries
        .iter()
        .map(|entry| entry.outcome_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let outcome_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.source_rehearsal_attached
                && entry.outcome_readback_visible
                && !entry.outcome_readback_persisted
                && entry.queryable
                && entry.diffable
                && !entry.test_probe_executed
                && entry.mutation_free
                && !entry.owner_assignment_persisted
                && !entry.freeze_applied
                && !entry.classification_persisted
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && entry.git_mutation_blocked
                && entry.cleanup_delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
                && !entry.outcome_category.is_empty()
                && !entry.outcome_action.is_empty()
                && !entry.required_local_gate.is_empty()
        })
        .count();
    let owner_attribution_outcome_required_count =
        count_outcome_category(&entries, "owner_attribution_outcome_required");
    let targeted_gate_outcome_required_count =
        count_outcome_category(&entries, "targeted_gate_outcome_required");
    let owned_lane_freeze_outcome_required_count =
        count_outcome_category(&entries, "owned_lane_freeze_outcome_required");
    let artifact_classification_outcome_required_count =
        count_outcome_category(&entries, "artifact_classification_outcome_required");
    let hepta_systems_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/hepta-systems");
    let cross_lane_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/cross-lane-review");
    let release_blocked_count = entries
        .iter()
        .filter(|entry| entry.release_disposition != "release_cutover_ready")
        .count();
    let test_probe_execution_blocked_count = entries
        .iter()
        .filter(|entry| !entry.test_probe_executed)
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
        .filter(|entry| !entry.evidence_recording_allowed)
        .count();
    let approval_acceptance_blocked_count = entries
        .iter()
        .filter(|entry| !entry.approval_acceptance_allowed)
        .count();
    let decision_recording_blocked_count = entries
        .iter()
        .filter(|entry| !entry.decision_recording_allowed)
        .count();
    let targeted_gate_source_bucket_count = entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.source_bucket,
                "codex-rs" | "plugins" | "scripts" | "docs"
            )
        })
        .count();
    let artifact_source_bucket_count = entries
        .iter()
        .filter(|entry| entry.source_bucket == "artifacts")
        .count();
    let owner_freeze_classification_outcome_readback_ready = source
        .owner_freeze_classification_rehearsal_ready
        && source.owner_freeze_classification_readback_visible
        && !source.owner_freeze_classification_readback_persisted
        && !source.owner_assignment_persisted
        && !source.freeze_applied
        && !source.classification_persisted
        && !source.test_probe_executed
        && !source.evidence_recorded
        && !source.approval_accepted
        && !source.decision_recorded
        && entries.len() == source.classification_entry_count
        && stable_outcome_key_count == entries.len()
        && outcome_route_count == entries.len()
        && outcome_ready_count == entries.len()
        && owner_attribution_outcome_required_count == source.owner_attribution_required_count
        && targeted_gate_outcome_required_count == targeted_gate_source_bucket_count
        && owned_lane_freeze_outcome_required_count == source.owned_lane_freeze_candidate_count
        && artifact_classification_outcome_required_count == artifact_source_bucket_count
        && hepta_systems_owner_route_count == source.hepta_systems_owner_route_count
        && cross_lane_owner_route_count == source.cross_lane_owner_route_count
        && release_blocked_count == entries.len()
        && test_probe_execution_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation",
        status: if owner_freeze_classification_outcome_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_GATE,
        schema_version: DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_rehearsal_gate: source.gate,
        source_rehearsal_ready: source.owner_freeze_classification_rehearsal_ready,
        source_rehearsal_visible: source.owner_freeze_classification_readback_visible,
        source_rehearsal_persisted: source.owner_freeze_classification_readback_persisted,
        source_classification_entry_count: source.classification_entry_count,
        source_tracked_change_count: source.source_tracked_change_count,
        source_untracked_change_count: source.source_untracked_change_count,
        outcome_entry_count: entries.len(),
        stable_outcome_key_count,
        outcome_route_count,
        outcome_ready_count,
        owner_attribution_outcome_required_count,
        targeted_gate_outcome_required_count,
        owned_lane_freeze_outcome_required_count,
        artifact_classification_outcome_required_count,
        hepta_systems_owner_route_count,
        cross_lane_owner_route_count,
        release_blocked_count,
        test_probe_execution_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        evidence_recording_blocked_count,
        approval_acceptance_blocked_count,
        decision_recording_blocked_count,
        outcome_readback_visible: owner_freeze_classification_outcome_readback_ready,
        outcome_readback_persisted: false,
        owner_assignment_persisted: false,
        freeze_applied: false,
        classification_persisted: false,
        test_probe_executed: false,
        evidence_recorded: false,
        approval_accepted: false,
        decision_recorded: false,
        operator_packet_sent: false,
        operator_packet_persisted: false,
        strategy_applied: false,
        git_index_mutated: false,
        cleanup_allowed: false,
        delete_allowed: false,
        release_cutover_allowed: false,
        package_or_release_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        owner_freeze_classification_outcome_readback_ready,
        entries,
        blockers: vec![
            "owner_freeze_classification_outcome_readback_visible_only",
            "owner_assignment_persistence_blocked",
            "freeze_application_blocked",
            "classification_persistence_blocked",
            "test_probe_execution_still_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "approval_acceptance_blocked",
            "decision_recording_blocked",
            "operator_packet_send_blocked",
            "release_cutover_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackSideEffects::none(),
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_entries(
    source: &DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport,
) -> Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry> {
    source
        .entries
        .iter()
        .map(|entry| DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry {
            source_classification_key: entry.classification_key.clone(),
            source_classification_route: entry.classification_route.clone(),
            outcome_key: format!(
                "dirty_worktree.owner_freeze_classification_outcome.{}.{}",
                key_safe(entry.group_type),
                key_safe(entry.source_bucket)
            ),
            outcome_route: format!(
                "readback://release-boundary/dirty-worktree/owner-freeze-classification-outcome/{}/{}",
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
            outcome_category: outcome_category(entry.source_bucket),
            outcome_action: outcome_action(entry.source_bucket),
            release_disposition: release_disposition(entry.release_disposition),
            required_local_gate: entry.local_gate,
            operator_packet_candidate: true,
            source_rehearsal_attached: !entry.classification_key.is_empty()
                && !entry.classification_route.is_empty(),
            outcome_readback_visible: true,
            outcome_readback_persisted: false,
            queryable: entry.queryable,
            diffable: entry.diffable,
            test_probe_executed: false,
            mutation_free: true,
            owner_assignment_persisted: false,
            freeze_applied: false,
            classification_persisted: false,
            evidence_recording_allowed: false,
            approval_acceptance_allowed: false,
            decision_recording_allowed: false,
            git_mutation_blocked: entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked,
            cleanup_delete_blocked: entry.cleanup_blocked && entry.delete_blocked,
            release_cutover_allowed: false,
            canary_activation_allowed: false,
            live_execution_allowed: false,
        })
        .collect()
}

fn count_outcome_category(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry],
    outcome_category: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.outcome_category == outcome_category)
        .count()
}

fn count_owner_route(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackEntry],
    owner_route: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.owner_route == owner_route)
        .count()
}

fn outcome_category(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "owner_attribution_outcome_required",
        "hepta_systems_owned" => "owned_lane_freeze_outcome_required",
        "artifacts" => "artifact_classification_outcome_required",
        "codex-rs" | "plugins" | "scripts" | "docs" => "targeted_gate_outcome_required",
        _ => "bucket_review_outcome_required",
    }
}

fn outcome_action(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "readback_owner_attribution_gap_without_persistence",
        "codex-rs" => "readback_targeted_rust_gate_required_without_execution",
        "plugins" => "readback_plugin_surface_gate_required_without_execution",
        "scripts" => "readback_script_syntax_gate_required_without_execution",
        "hepta_systems_owned" => "readback_owned_lane_freeze_candidate_without_applying_freeze",
        "artifacts" => "readback_artifact_classification_required_without_delete_or_relocation",
        "docs" => "readback_doc_evidence_consistency_required_without_evidence_persistence",
        _ => "readback_bucket_review_required_without_git_mutation",
    }
}

fn release_disposition(source_release_disposition: &str) -> &'static str {
    match source_release_disposition {
        "blocked_until_owner_attribution" => "release_blocked_until_owner_attribution_outcome",
        "blocked_until_artifact_classification" => {
            "release_blocked_until_artifact_classification_outcome"
        }
        "blocked_until_owned_lane_freeze" => "release_blocked_until_owned_lane_freeze_outcome",
        "blocked_until_targeted_gate" => "release_blocked_until_targeted_gate_outcome",
        _ => "release_blocked_until_bucket_review_outcome",
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            owner_assignment_persisted: false,
            freeze_applied: false,
            classification_persisted: false,
            test_probe_executed: false,
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
            operator_packet_sent: false,
            operator_packet_persisted: false,
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
    fn owner_freeze_classification_outcome_projects_all_buckets_without_mutation() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_rehearsal_ready);
        assert!(report.source_rehearsal_visible);
        assert!(!report.source_rehearsal_persisted);
        assert_eq!(
            report.outcome_entry_count,
            report.source_classification_entry_count
        );
        assert_eq!(report.outcome_ready_count, report.outcome_entry_count);
        assert_eq!(report.stable_outcome_key_count, report.outcome_entry_count);
        assert_eq!(report.outcome_route_count, report.outcome_entry_count);
        assert_eq!(report.owner_attribution_outcome_required_count, 1);
        assert_eq!(
            report.targeted_gate_outcome_required_count,
            report
                .entries
                .iter()
                .filter(|entry| matches!(
                    entry.source_bucket,
                    "codex-rs" | "plugins" | "scripts" | "docs"
                ))
                .count()
        );
        assert_eq!(report.owned_lane_freeze_outcome_required_count, 1);
        assert_eq!(
            report.artifact_classification_outcome_required_count,
            report
                .entries
                .iter()
                .filter(|entry| entry.source_bucket == "artifacts")
                .count()
        );
        assert_eq!(
            report.hepta_systems_owner_route_count + report.cross_lane_owner_route_count,
            report.outcome_entry_count
        );
        assert_eq!(report.release_blocked_count, report.outcome_entry_count);
        assert!(report.outcome_readback_visible);
        assert!(!report.outcome_readback_persisted);
        assert!(report.owner_freeze_classification_outcome_readback_ready);
    }

    #[test]
    fn owner_freeze_classification_outcome_entries_are_actionable_readbacks() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "cross_lane_or_unowned"
                && entry.outcome_category == "owner_attribution_outcome_required"
                && entry.outcome_action == "readback_owner_attribution_gap_without_persistence"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "hepta_systems_owned"
                && entry.outcome_category == "owned_lane_freeze_outcome_required"
                && entry.outcome_action
                    == "readback_owned_lane_freeze_candidate_without_applying_freeze"
        }));
        if let Some(artifact) = report
            .entries
            .iter()
            .find(|entry| entry.source_bucket == "artifacts")
        {
            assert_eq!(
                artifact.outcome_category,
                "artifact_classification_outcome_required"
            );
            assert_eq!(
                artifact.outcome_action,
                "readback_artifact_classification_required_without_delete_or_relocation"
            );
        }
        assert!(report.entries.iter().all(|entry| {
            entry.source_rehearsal_attached
                && entry.outcome_readback_visible
                && !entry.outcome_readback_persisted
                && entry.queryable
                && entry.diffable
                && !entry.test_probe_executed
                && entry.mutation_free
                && !entry.owner_assignment_persisted
                && !entry.freeze_applied
                && !entry.classification_persisted
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && entry.git_mutation_blocked
                && entry.cleanup_delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
                && entry.operator_packet_candidate
        }));
    }

    #[test]
    fn owner_freeze_classification_outcome_side_effects_remain_closed() {
        let report =
            dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report();

        assert!(!report.owner_assignment_persisted);
        assert!(!report.freeze_applied);
        assert!(!report.classification_persisted);
        assert!(!report.test_probe_executed);
        assert!(!report.evidence_recorded);
        assert!(!report.approval_accepted);
        assert!(!report.decision_recorded);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_persisted);
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
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationOutcomeReadbackSideEffects::none()
        );
    }
}
