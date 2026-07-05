#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-report.sh"
SOURCE_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 14 clean-worktree strategy report: $REPORT"
[[ -x "$SOURCE_READBACK_REPORT" ]] || fail "missing executable Phase 13 grouping-freeze operator readback report: $SOURCE_READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 14 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 14 dirty-worktree clean-worktree strategy report"
fi

grep -q 'Dirty Worktree Release Boundary Actionable Clean Worktree Strategy' "$DOC" \
  || fail "architecture note must document Dirty Worktree Release Boundary Actionable Clean Worktree Strategy"
grep -q 'operator_strategy_only' "$DOC" \
  || fail "architecture note must document operator_strategy_only"
grep -q 'no_git_mutation_no_cleanup_no_evidence_recording' "$DOC" \
  || fail "architecture note must document the no-mutation action mode"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, evidence recording, evidence persistence, approval acceptance, blocker waiver, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed Phase 14 boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_actionable_clean_worktree_strategy"
  and .status == "ready_blocked"
  and .gate == "dirty_worktree_release_boundary_actionable_clean_worktree_strategy_gate"
  and .schema_version == "dirty_worktree_release_boundary_actionable_clean_worktree_strategy_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_operator_readback_ready == true
  and .source_freeze_applied == false
  and .source_readback_entry_count > 0
  and .lib_export_present == true
  and .inventory_entry_count == (.tracked_change_count + .untracked_change_count)
  and .strategy_scope.strategy_id == "dirty-worktree.release-boundary.actionable-clean-worktree-strategy.v1"
  and .strategy_scope.strategy_route == "readback://release-boundary/dirty-worktree/actionable-clean-worktree-strategy/v1"
  and .strategy_scope.source_readback_route == "readback://release-boundary/dirty-worktree/grouping-freeze/operator-readback/v1"
  and .strategy_scope.strategy_mode == "operator_strategy_only"
  and .strategy_scope.action_mode == "no_git_mutation_no_cleanup_no_evidence_recording"
  and .strategy_scope.mutation_boundary == "closed"
  and .strategy_entry_count == .source_readback_entry_count
  and .stable_strategy_key_count == .strategy_entry_count
  and .strategy_route_count == .strategy_entry_count
  and .ready_strategy_count == .strategy_entry_count
  and .operator_decision_required_count == .strategy_entry_count
  and .no_git_mutation_strategy_count == .strategy_entry_count
  and .hepta_systems_strategy_count > 0
  and .cross_lane_strategy_count > 0
  and .mixed_lane_strategy_count > 0
  and .evidence_recorded_count == 0
  and .strategy_ready == true
  and .strategy_applied == false
  and .release_cutover_allowed == false
  and .git_add_allowed == false
  and .git_index_mutated == false
  and .git_commit_allowed == false
  and .git_push_allowed == false
  and .git_reset_allowed == false
  and .git_checkout_allowed == false
  and .git_revert_allowed == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persistence_allowed == false
  and .approval_acceptance_allowed == false
  and .blocker_waiver_allowed == false
  and .package_or_release_allowed == false
  and .public_ga_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == .strategy_entry_count
  and any(.entries[]; .group_type == "top_level")
  and any(.entries[]; .group_type == "scope")
  and any(.entries[]; .review_lane == "hepta-systems")
  and any(.entries[]; .review_lane == "cross-lane-review" or .review_lane == "external-or-cross-lane")
  and any(.entries[]; .review_lane == "mixed" or .review_lane == "mixed-hepta-and-cross-lane")
  and (.entries | all(
    (.source_readback_key | startswith("dirty_worktree.readback."))
    and (.source_readback_route | startswith("readback://release-boundary/dirty-worktree/grouping-freeze/operator/"))
    and (.source_diff_key | startswith("dirty_worktree.diff."))
    and (.strategy_key | startswith("dirty_worktree.strategy."))
    and (.strategy_route | startswith("readback://release-boundary/dirty-worktree/strategy/"))
    and .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .source_entry_count == (.hepta_systems_owned_count + .cross_lane_or_unowned_count)
    and (.recommended_strategy == "hepta_systems_owned_batch_review"
      or .recommended_strategy == "cross_lane_owner_review_required"
      or .recommended_strategy == "split_owned_and_cross_lane_review"
      or .recommended_strategy == "operator_classification_required")
    and (.operator_action == "prepare_hepta_systems_clean_plan_for_operator_review"
      or .operator_action == "request_owner_classification_before_cleanup"
      or .operator_action == "split_group_into_owned_and_cross_lane_subsets"
      or .operator_action == "classify_group_before_any_cleanup")
    and .evidence_requirement == "clean_worktree_decision_record_required_before_release"
    and .execution_mode == "strategy_only_no_git_mutation"
    and .decision_state == "pending_operator_decision"
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .strategy_ready == true
    and .operator_decision_required == true
    and .strategy_applied == false
    and .git_mutation_allowed == false
    and .cleanup_allowed == false
    and .delete_allowed == false
    and .evidence_recording_allowed == false
    and .release_cutover_allowed == false
    and .live_execution_allowed == false))
  and (.blockers | index("clean_worktree_strategy_requires_operator_decision")) != null
  and (.blockers | index("strategy_application_blocked")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("canary_activation_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("phase15_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_without_git_mutation")) != null
  and .next_migration_step == "phase15_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_READBACK_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_grouping_freeze_operator_readback"
  and .status == "ready_blocked"
  and .operator_readback_ready == true
  and .freeze_applied == false
  and .readback_entry_count > 0
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .release_cutover_allowed == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_actionable_clean_worktree_strategy --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-gate: PASS: dirty-worktree clean-worktree strategy is operator-visible without git mutation or live activation\n'
