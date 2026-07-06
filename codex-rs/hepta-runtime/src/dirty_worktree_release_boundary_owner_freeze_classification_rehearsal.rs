use std::collections::BTreeSet;

use serde::Serialize;

use crate::DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport;
use crate::dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report;

pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_GATE: &str = "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation_gate";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_SCHEMA_VERSION:
    &str = "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_v1";
pub const DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_lease_idempotency_index_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_outcome_gate: &'static str,
    pub source_outcome_ready: bool,
    pub source_outcome_readback_visible: bool,
    pub source_outcome_readback_persisted: bool,
    pub source_test_probe_executed: bool,
    pub source_outcome_entry_count: usize,
    pub source_inventory_entry_count: usize,
    pub source_tracked_change_count: usize,
    pub source_untracked_change_count: usize,
    pub rehearsal_scope: DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalScope,
    pub classification_entry_count: usize,
    pub stable_classification_key_count: usize,
    pub classification_route_count: usize,
    pub classification_ready_count: usize,
    pub owner_attribution_required_count: usize,
    pub owner_hint_projected_count: usize,
    pub hepta_systems_owner_route_count: usize,
    pub cross_lane_owner_route_count: usize,
    pub owned_lane_freeze_candidate_count: usize,
    pub artifact_classification_required_count: usize,
    pub local_gate_required_count: usize,
    pub release_blocked_count: usize,
    pub test_probe_execution_blocked_count: usize,
    pub git_mutation_blocked_count: usize,
    pub cleanup_delete_blocked_count: usize,
    pub evidence_recording_blocked_count: usize,
    pub approval_acceptance_blocked_count: usize,
    pub decision_recording_blocked_count: usize,
    pub owner_freeze_classification_readback_visible: bool,
    pub owner_freeze_classification_readback_persisted: bool,
    pub owner_assignment_persisted: bool,
    pub freeze_applied: bool,
    pub classification_persisted: bool,
    pub test_probe_executed: bool,
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
    pub blocker_waiver_allowed: bool,
    pub package_or_release_allowed: bool,
    pub public_ga_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_activation_allowed: bool,
    pub live_execution_allowed: bool,
    pub owner_freeze_classification_rehearsal_ready: bool,
    pub entries: Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalScope {
    pub rehearsal_id: &'static str,
    pub rehearsal_route: &'static str,
    pub source_outcome_route: &'static str,
    pub owner_boundary: &'static str,
    pub freeze_boundary: &'static str,
    pub classification_boundary: &'static str,
    pub test_probe_boundary: &'static str,
    pub git_mutation_boundary: &'static str,
    pub cleanup_boundary: &'static str,
    pub evidence_boundary: &'static str,
    pub approval_boundary: &'static str,
    pub decision_boundary: &'static str,
    pub live_boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry {
    pub source_outcome_key: String,
    pub source_outcome_route: String,
    pub classification_key: String,
    pub classification_route: String,
    pub group_type: &'static str,
    pub source_bucket: &'static str,
    pub source_entry_count: usize,
    pub tracked_count: usize,
    pub untracked_count: usize,
    pub owner_hint: &'static str,
    pub review_lane: &'static str,
    pub owner_route: String,
    pub owner_state: &'static str,
    pub freeze_state: &'static str,
    pub classification_state: &'static str,
    pub release_disposition: &'static str,
    pub local_gate: &'static str,
    pub recommended_strategy: &'static str,
    pub source_outcome_state: &'static str,
    pub source_outcome_action: &'static str,
    pub rehearsal_action: &'static str,
    pub operator_action: &'static str,
    pub decision_state: &'static str,
    pub evidence_recording_state: &'static str,
    pub evidence_persistence_state: &'static str,
    pub evidence_receipt_state: &'static str,
    pub approval_request_state: &'static str,
    pub approval_acceptance_state: &'static str,
    pub approval_recording_state: &'static str,
    pub approval_receipt_state: &'static str,
    pub source_outcome_attached: bool,
    pub owner_freeze_classification_readback_visible: bool,
    pub owner_freeze_classification_readback_persisted: bool,
    pub owner_projection_visible: bool,
    pub freeze_projection_visible: bool,
    pub classification_projection_visible: bool,
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
    pub git_add_blocked: bool,
    pub git_index_mutation_blocked: bool,
    pub git_commit_blocked: bool,
    pub git_push_blocked: bool,
    pub git_reset_blocked: bool,
    pub git_checkout_blocked: bool,
    pub git_revert_blocked: bool,
    pub cleanup_blocked: bool,
    pub delete_blocked: bool,
    pub release_cutover_allowed: bool,
    pub canary_activation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalSideEffects {
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
    pub blocker_waived: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub canary_activation_started: bool,
    pub live_activation_started: bool,
    pub live_execution_started: bool,
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport {
    let source = dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report();
    dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report_from_outcome(
        &source,
    )
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report_from_outcome(
    source: &DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport,
) -> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport {
    let entries =
        dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_entries(source);
    let stable_classification_key_count = entries
        .iter()
        .map(|entry| entry.classification_key.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let classification_route_count = entries
        .iter()
        .map(|entry| entry.classification_route.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let classification_ready_count = entries
        .iter()
        .filter(|entry| {
            entry.source_outcome_attached
                && entry.owner_freeze_classification_readback_visible
                && !entry.owner_freeze_classification_readback_persisted
                && entry.owner_projection_visible
                && entry.freeze_projection_visible
                && entry.classification_projection_visible
                && entry.queryable
                && entry.diffable
                && !entry.test_probe_executed
                && entry.mutation_free
                && !entry.owner_assignment_persisted
                && !entry.freeze_applied
                && !entry.classification_persisted
                && entry.source_outcome_state != "unknown"
                && entry.owner_state != "unknown"
                && entry.freeze_state != "unknown"
                && entry.classification_state != "unknown"
                && !entry.local_gate.is_empty()
                && entry.decision_state == "pending_operator_decision"
                && entry.evidence_recording_state == "evidence_recording_blocked"
                && entry.evidence_persistence_state == "evidence_persistence_blocked"
                && entry.evidence_receipt_state == "evidence_receipt_blocked"
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
                && entry.cleanup_blocked
                && entry.delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        })
        .count();
    let owner_attribution_required_count =
        count_owner_state(&entries, "owner_attribution_required");
    let owner_hint_projected_count = entries
        .iter()
        .filter(|entry| !entry.owner_hint.is_empty())
        .count();
    let hepta_systems_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/hepta-systems");
    let cross_lane_owner_route_count =
        count_owner_route(&entries, "owner://release-boundary/cross-lane-review");
    let owned_lane_freeze_candidate_count =
        count_freeze_state(&entries, "owned_lane_freeze_candidate");
    let artifact_classification_required_count =
        count_classification_state(&entries, "artifact_classification_required");
    let artifact_bucket_count = entries
        .iter()
        .filter(|entry| entry.source_bucket == "artifacts")
        .count();
    let local_gate_required_count = entries
        .iter()
        .filter(|entry| !entry.local_gate.is_empty())
        .count();
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

    let owner_freeze_classification_rehearsal_ready = source
        .test_only_rehearsal_outcome_readback_ready
        && source.outcome_readback_visible
        && !source.outcome_readback_persisted
        && !source.test_probe_executed
        && !source.evidence_recorded
        && !source.evidence_recording_persisted
        && !source.evidence_receipt_persisted
        && !entries.is_empty()
        && entries.len() == source.outcome_entry_count
        && stable_classification_key_count == entries.len()
        && classification_route_count == entries.len()
        && classification_ready_count == entries.len()
        && owner_attribution_required_count == 1
        && owner_hint_projected_count == entries.len()
        && hepta_systems_owner_route_count + cross_lane_owner_route_count == entries.len()
        && owned_lane_freeze_candidate_count == 1
        && artifact_classification_required_count == artifact_bucket_count
        && local_gate_required_count == entries.len()
        && release_blocked_count == entries.len()
        && test_probe_execution_blocked_count == entries.len()
        && git_mutation_blocked_count == entries.len()
        && cleanup_delete_blocked_count == entries.len()
        && evidence_recording_blocked_count == entries.len()
        && approval_acceptance_blocked_count == entries.len()
        && decision_recording_blocked_count == entries.len();

    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalReport {
        runtime: "hepta",
        surface: "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation",
        status: if owner_freeze_classification_rehearsal_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_GATE,
        schema_version: DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_SCHEMA_VERSION,
        plugin_id: source.plugin_id,
        source_outcome_gate: source.gate,
        source_outcome_ready: source.test_only_rehearsal_outcome_readback_ready,
        source_outcome_readback_visible: source.outcome_readback_visible,
        source_outcome_readback_persisted: source.outcome_readback_persisted,
        source_test_probe_executed: source.test_probe_executed,
        source_outcome_entry_count: source.outcome_entry_count,
        source_inventory_entry_count: source.inventory_entry_count,
        source_tracked_change_count: source.tracked_change_count,
        source_untracked_change_count: source.untracked_change_count,
        rehearsal_scope:
            dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_scope(),
        classification_entry_count: entries.len(),
        stable_classification_key_count,
        classification_route_count,
        classification_ready_count,
        owner_attribution_required_count,
        owner_hint_projected_count,
        hepta_systems_owner_route_count,
        cross_lane_owner_route_count,
        owned_lane_freeze_candidate_count,
        artifact_classification_required_count,
        local_gate_required_count,
        release_blocked_count,
        test_probe_execution_blocked_count,
        git_mutation_blocked_count,
        cleanup_delete_blocked_count,
        evidence_recording_blocked_count,
        approval_acceptance_blocked_count,
        decision_recording_blocked_count,
        owner_freeze_classification_readback_visible: owner_freeze_classification_rehearsal_ready,
        owner_freeze_classification_readback_persisted: false,
        owner_assignment_persisted: false,
        freeze_applied: false,
        classification_persisted: false,
        test_probe_executed: false,
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
        blocker_waiver_allowed: false,
        package_or_release_allowed: false,
        public_ga_allowed: false,
        canary_activation_allowed: false,
        live_activation_allowed: false,
        live_execution_allowed: false,
        owner_freeze_classification_rehearsal_ready,
        entries,
        blockers: vec![
            "owner_freeze_classification_rehearsal_visible_only",
            "owner_assignment_persistence_blocked",
            "freeze_application_blocked",
            "classification_persistence_blocked",
            "test_probe_execution_still_blocked",
            "release_cutover_blocked",
            "git_mutation_blocked",
            "cleanup_and_delete_blocked",
            "evidence_recording_blocked",
            "approval_acceptance_blocked",
            "decision_recording_blocked",
            "canary_activation_blocked",
            "live_activation_blocked",
        ],
        recommended_next_gate:
            DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_RECOMMENDED_NEXT_GATE,
        side_effects: DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalSideEffects::none(),
    }
}

pub const fn dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_scope()
-> DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalScope {
    DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalScope {
        rehearsal_id: "dirty-worktree.release-boundary.owner-freeze-classification-rehearsal.v1",
        rehearsal_route: "readback://release-boundary/dirty-worktree/owner-freeze-classification-rehearsal/v1",
        source_outcome_route: "readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/v1",
        owner_boundary: "visible_only_not_persisted",
        freeze_boundary: "planned_not_applied",
        classification_boundary: "projected_not_persisted",
        test_probe_boundary: "blocked",
        git_mutation_boundary: "blocked",
        cleanup_boundary: "blocked",
        evidence_boundary: "blocked",
        approval_boundary: "blocked",
        decision_boundary: "blocked",
        live_boundary: "blocked",
    }
}

pub fn dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_entries(
    source: &DirtyWorktreeReleaseBoundaryTestOnlyRehearsalOutcomeReadbackReport,
) -> Vec<DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry {
                source_outcome_key: entry.outcome_key.clone(),
                source_outcome_route: entry.outcome_route.clone(),
                classification_key: classification_key(entry.group_type, entry.source_bucket),
                classification_route: classification_route(entry.group_type, entry.source_bucket),
                group_type: entry.group_type,
                source_bucket: entry.source_bucket,
                source_entry_count: entry.source_entry_count,
                tracked_count: entry.tracked_count,
                untracked_count: entry.untracked_count,
                owner_hint: entry.owner_hint,
                review_lane: entry.review_lane,
                owner_route: owner_route(entry.owner_hint),
                owner_state: owner_state(entry.source_bucket, entry.owner_hint),
                freeze_state: freeze_state(entry.source_bucket),
                classification_state: classification_state(entry.source_bucket),
                release_disposition: release_disposition(entry.source_bucket),
                local_gate: entry.source_required_local_gate,
                recommended_strategy: entry.recommended_strategy,
                source_outcome_state: entry.outcome_state,
                source_outcome_action: entry.outcome_action,
                rehearsal_action: rehearsal_action(entry.source_bucket),
                operator_action:
                    "review_owner_freeze_classification_before_any_probe_or_git_mutation",
                decision_state: entry.decision_state,
                evidence_recording_state: entry.evidence_recording_state,
                evidence_persistence_state: entry.evidence_persistence_state,
                evidence_receipt_state: entry.evidence_receipt_state,
                approval_request_state: entry.approval_request_state,
                approval_acceptance_state: entry.approval_acceptance_state,
                approval_recording_state: entry.approval_recording_state,
                approval_receipt_state: entry.approval_receipt_state,
                source_outcome_attached: !entry.outcome_key.is_empty()
                    && !entry.outcome_route.is_empty(),
                owner_freeze_classification_readback_visible: true,
                owner_freeze_classification_readback_persisted: false,
                owner_projection_visible: true,
                freeze_projection_visible: true,
                classification_projection_visible: true,
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
                git_add_blocked: entry.git_add_blocked,
                git_index_mutation_blocked: entry.git_index_mutation_blocked,
                git_commit_blocked: entry.git_commit_blocked,
                git_push_blocked: entry.git_push_blocked,
                git_reset_blocked: entry.git_reset_blocked,
                git_checkout_blocked: entry.git_checkout_blocked,
                git_revert_blocked: entry.git_revert_blocked,
                cleanup_blocked: entry.cleanup_blocked,
                delete_blocked: entry.delete_blocked,
                release_cutover_allowed: false,
                canary_activation_allowed: false,
                live_execution_allowed: false,
            },
        )
        .collect()
}

fn count_owner_state(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry],
    owner_state: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.owner_state == owner_state)
        .count()
}

fn count_owner_route(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry],
    owner_route: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.owner_route == owner_route)
        .count()
}

fn count_freeze_state(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry],
    freeze_state: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.freeze_state == freeze_state)
        .count()
}

fn count_classification_state(
    entries: &[DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalEntry],
    classification_state: &str,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.classification_state == classification_state)
        .count()
}

fn classification_key(group_type: &str, source_bucket: &str) -> String {
    format!(
        "dirty_worktree.owner_freeze_classification_rehearsal.{}.{}",
        key_safe(group_type),
        key_safe(source_bucket)
    )
}

fn classification_route(group_type: &str, source_bucket: &str) -> String {
    format!(
        "readback://release-boundary/dirty-worktree/owner-freeze-classification-rehearsal/{}/{}",
        route_group_type(group_type),
        route_safe(source_bucket)
    )
}

fn owner_route(owner_hint: &str) -> String {
    format!("owner://release-boundary/{}", route_safe(owner_hint))
}

fn owner_state(source_bucket: &str, owner_hint: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "owner_attribution_required",
        _ if owner_hint == "hepta-systems" => "owner_hint_hepta_systems_projected",
        _ if owner_hint == "cross-lane-review" => "cross_lane_owner_review_required",
        _ => "unknown",
    }
}

fn freeze_state(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "freeze_blocked_until_owner_attribution",
        "hepta_systems_owned" => "owned_lane_freeze_candidate",
        "artifacts" => "freeze_deferred_until_artifact_classification",
        "codex-rs" | "plugins" | "scripts" | "docs" => "freeze_deferred_until_targeted_gate",
        _ => "unknown",
    }
}

fn classification_state(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "owner_attribution_required",
        "codex-rs" => "targeted_rust_gate_required",
        "plugins" => "plugin_surface_gate_required",
        "scripts" => "script_syntax_gate_required",
        "hepta_systems_owned" => "owned_lane_freeze_required",
        "artifacts" => "artifact_classification_required",
        "docs" => "doc_evidence_consistency_required",
        _ => "unknown",
    }
}

fn release_disposition(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "artifacts" => "blocked_until_artifact_classification",
        "hepta_systems_owned" => "blocked_until_owned_lane_freeze",
        "codex-rs" | "plugins" | "scripts" | "docs" => "blocked_until_targeted_gate",
        _ => "blocked_until_bucket_review",
    }
}

fn rehearsal_action(source_bucket: &str) -> &'static str {
    match source_bucket {
        "cross_lane_or_unowned" => "project_owner_attribution_without_git_mutation_or_persistence",
        "codex-rs" => "project_targeted_rust_gate_without_execution_or_git_mutation",
        "plugins" => "project_plugin_surface_gate_without_execution_or_git_mutation",
        "scripts" => "project_script_syntax_gate_without_execution_or_git_mutation",
        "hepta_systems_owned" => "project_owned_lane_freeze_without_applying_freeze",
        "artifacts" => "project_artifact_classification_without_delete_or_relocation",
        "docs" => "project_doc_evidence_consistency_without_evidence_persistence",
        _ => "project_bucket_review_without_git_mutation",
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

impl DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalSideEffects {
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
    fn owner_freeze_classification_projects_all_outcome_buckets_without_mutation() {
        let report = dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_outcome_ready);
        assert!(report.source_outcome_readback_visible);
        assert!(!report.source_outcome_readback_persisted);
        assert!(!report.source_test_probe_executed);
        assert_eq!(
            report.classification_entry_count,
            report.source_outcome_entry_count
        );
        assert_eq!(
            report.classification_ready_count,
            report.classification_entry_count
        );
        assert_eq!(
            report.stable_classification_key_count,
            report.classification_entry_count
        );
        assert_eq!(
            report.classification_route_count,
            report.classification_entry_count
        );
        assert_eq!(report.owner_attribution_required_count, 1);
        assert_eq!(
            report.owner_hint_projected_count,
            report.classification_entry_count
        );
        assert_eq!(
            report.hepta_systems_owner_route_count + report.cross_lane_owner_route_count,
            report.classification_entry_count
        );
        assert_eq!(report.owned_lane_freeze_candidate_count, 1);
        assert_eq!(
            report.artifact_classification_required_count,
            report
                .entries
                .iter()
                .filter(|entry| entry.source_bucket == "artifacts")
                .count()
        );
        assert_eq!(
            report.local_gate_required_count,
            report.classification_entry_count
        );
        assert!(report.owner_freeze_classification_readback_visible);
        assert!(!report.owner_freeze_classification_readback_persisted);
        assert!(report.owner_freeze_classification_rehearsal_ready);
        assert_eq!(
            report.recommended_next_gate,
            "temporal_lite_lease_idempotency_index_feature_gated_readback"
        );
    }

    #[test]
    fn owner_freeze_classification_entries_are_stable_queryable_and_blocked() {
        let report = dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report();

        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "cross_lane_or_unowned"
                && entry.owner_state == "owner_attribution_required"
                && entry.freeze_state == "freeze_blocked_until_owner_attribution"
                && entry.classification_state == "owner_attribution_required"
                && entry.local_gate == "owner_attribution_freeze_gate"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.source_bucket == "hepta_systems_owned"
                && entry.owner_route == "owner://release-boundary/hepta-systems"
                && entry.freeze_state == "owned_lane_freeze_candidate"
                && entry.classification_state == "owned_lane_freeze_required"
        }));
        if let Some(artifact) = report
            .entries
            .iter()
            .find(|entry| entry.source_bucket == "artifacts")
        {
            assert_eq!(
                artifact.classification_state,
                "artifact_classification_required"
            );
            assert_eq!(
                artifact.release_disposition,
                "blocked_until_artifact_classification"
            );
        }
        assert!(report.entries.iter().all(|entry| {
            entry.source_outcome_attached
                && entry.owner_freeze_classification_readback_visible
                && !entry.owner_freeze_classification_readback_persisted
                && entry.owner_projection_visible
                && entry.freeze_projection_visible
                && entry.classification_projection_visible
                && entry.queryable
                && entry.diffable
                && !entry.test_probe_executed
                && entry.mutation_free
                && !entry.owner_assignment_persisted
                && !entry.freeze_applied
                && !entry.classification_persisted
                && entry.decision_state == "pending_operator_decision"
                && entry.evidence_recording_state == "evidence_recording_blocked"
                && entry.evidence_persistence_state == "evidence_persistence_blocked"
                && entry.evidence_receipt_state == "evidence_receipt_blocked"
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && entry.git_add_blocked
                && entry.git_index_mutation_blocked
                && entry.git_commit_blocked
                && entry.git_push_blocked
                && entry.git_reset_blocked
                && entry.git_checkout_blocked
                && entry.git_revert_blocked
                && entry.cleanup_blocked
                && entry.delete_blocked
                && !entry.release_cutover_allowed
                && !entry.canary_activation_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn owner_freeze_classification_side_effects_remain_closed() {
        let report = dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report();

        assert!(!report.owner_assignment_persisted);
        assert!(!report.freeze_applied);
        assert!(!report.classification_persisted);
        assert!(!report.test_probe_executed);
        assert!(!report.evidence_recorded);
        assert!(!report.approval_accepted);
        assert!(!report.decision_recorded);
        assert!(!report.operator_packet_sent);
        assert!(!report.readback_persisted);
        assert!(!report.strategy_applied);
        assert!(!report.git_add_allowed);
        assert!(!report.git_index_mutated);
        assert!(!report.cleanup_allowed);
        assert!(!report.delete_allowed);
        assert!(!report.package_or_release_allowed);
        assert!(!report.canary_activation_allowed);
        assert!(!report.live_activation_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            DirtyWorktreeReleaseBoundaryOwnerFreezeClassificationRehearsalSideEffects::none()
        );
    }
}
