#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report.sh"
INVENTORY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 12 report: $REPORT"
[[ -x "$INVENTORY_REPORT" ]] || fail "missing executable Phase 11 inventory report: $INVENTORY_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 12 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 12 dirty-worktree grouping freeze-plan report"
fi

grep -q 'Dirty Worktree Release Boundary Grouping Freeze Plan' "$DOC" \
  || fail "architecture note must document Dirty Worktree Release Boundary Grouping Freeze Plan"
grep -q 'top_level_and_scope_bucket' "$DOC" \
  || fail "architecture note must document top_level_and_scope_bucket"
grep -q 'plan_only_not_applied' "$DOC" \
  || fail "architecture note must document plan_only_not_applied"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, evidence recording, evidence persistence, approval acceptance, blocker waiver, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed grouping freeze-plan boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_grouping_freeze_plan"
  and .status == "ready_blocked"
  and .gate == "dirty_worktree_release_boundary_grouping_freeze_plan_gate"
  and .schema_version == "dirty_worktree_release_boundary_grouping_freeze_plan_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_inventory_ready == true
  and .source_dirty_worktree_release_boundary_open == true
  and .source_dirty_worktree_release_boundary_resolved == false
  and .lib_export_present == true
  and .inventory_entry_count > 0
  and .inventory_entry_count == (.tracked_change_count + .untracked_change_count)
  and .inventory_entry_count == (.hepta_systems_owned_count + .cross_lane_or_unowned_count)
  and .grouping_scope.plan_id == "dirty-worktree.release-boundary.grouping-freeze-plan.v1"
  and .grouping_scope.plan_route == "readback://release-boundary/dirty-worktree/grouping-freeze-plan/v1"
  and .grouping_scope.grouping_mode == "top_level_and_scope_bucket"
  and .grouping_scope.freeze_mode == "plan_only_not_applied"
  and .grouping_scope.mutation_boundary == "closed"
  and .top_level_group_count > 0
  and .scope_group_count > 0
  and .group_entry_count == (.top_level_group_count + .scope_group_count)
  and .freeze_plan_ready_count == .group_entry_count
  and .planned_not_applied_count == .group_entry_count
  and .release_evidence_bucket_count == .group_entry_count
  and .evidence_recorded_count == 0
  and .grouping_freeze_plan_ready == true
  and .freeze_applied == false
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
  and (.entries | length) == .group_entry_count
  and (.entries | any(.group_type == "top_level"))
  and (.entries | any(.group_type == "scope" and .source_bucket == "hepta_systems_owned"))
  and (.entries | any(.group_type == "scope" and .source_bucket == "cross_lane_or_unowned"))
  and (.entries | all(
    (.group_key | length) > 0
    and (.group_route | startswith("readback://release-boundary/dirty-worktree/group/"))
    and .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .source_entry_count == (.hepta_systems_owned_count + .cross_lane_or_unowned_count)
    and (.owner_hint == "hepta-systems" or .owner_hint == "cross-lane-review")
    and (.review_lane == "hepta-systems" or .review_lane == "cross-lane-review" or .review_lane == "mixed-hepta-and-cross-lane")
    and .freeze_state == "planned_not_applied"
    and .evidence_state == "not_recorded"
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .freeze_plan_ready == true
    and .freeze_applied == false
    and .git_mutation_allowed == false
    and .cleanup_allowed == false
    and .evidence_recording_allowed == false
    and .release_cutover_allowed == false
  ))
  and (.blockers | index("dirty_worktree_release_boundary_grouping_freeze_plan_not_applied")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("canary_activation_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation")) != null
  and .next_migration_step == "phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$INVENTORY_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_inventory"
  and .release_boundary_inventory_ready == true
  and .dirty_worktree_release_boundary_open == true
  and .release_cutover_allowed == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_grouping_freeze_plan --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-gate: PASS: dirty worktree release boundary is grouped into a freeze plan without git mutation, evidence recording, release, canary activation, or live execution\n'
