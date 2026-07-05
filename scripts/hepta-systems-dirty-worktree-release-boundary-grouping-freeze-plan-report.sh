#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
INVENTORY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_grouping_freeze_plan.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$INVENTORY_REPORT" ]] || fail "missing executable Phase 11 inventory report: $INVENTORY_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 12 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 12 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 12 dirty-worktree grouping freeze-plan report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$INVENTORY_REPORT" >"$tmpdir/inventory.json" \
  || fail "failed to render Phase 11 dirty-worktree release-boundary inventory report"
jq -e . "$tmpdir/inventory.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 11 dirty-worktree release-boundary inventory report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_grouping_freeze_plan_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile inventory "$tmpdir/inventory.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_2026-06-27.md" \
  '
  def group_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "-"));
  def owner_hint($hepta; $cross):
    if $cross > $hepta then "cross-lane-review" else "hepta-systems" end;
  def review_lane($hepta; $cross):
    if $cross > 0 and $hepta > 0 then "mixed-hepta-and-cross-lane"
    elif $cross > 0 then "cross-lane-review"
    else "hepta-systems"
    end;
  def top_level_entry:
    {
      group_type:"top_level",
      group_key:("dirty_worktree.group.top_level." + group_safe(.top_level)),
      group_route:("readback://release-boundary/dirty-worktree/group/top-level/" + group_safe(.top_level)),
      source_bucket:.top_level,
      source_entry_count:.count,
      tracked_count:.tracked_count,
      untracked_count:.untracked_count,
      hepta_systems_owned_count:.hepta_systems_owned_count,
      cross_lane_or_unowned_count:.cross_lane_or_unowned_count,
      owner_hint:owner_hint(.hepta_systems_owned_count; .cross_lane_or_unowned_count),
      review_lane:review_lane(.hepta_systems_owned_count; .cross_lane_or_unowned_count),
      freeze_state:"planned_not_applied",
      evidence_state:"not_recorded",
      operator_visible:true,
      queryable:true,
      diffable:true,
      freeze_plan_ready:true,
      freeze_applied:false,
      git_mutation_allowed:false,
      cleanup_allowed:false,
      evidence_recording_allowed:false,
      release_cutover_allowed:false
    };
  def scope_entry:
    . as $bucket
    | ($bucket.scope_bucket == "hepta_systems_owned") as $owned
    | {
      group_type:"scope",
      group_key:("dirty_worktree.group.scope." + group_safe($bucket.scope_bucket)),
      group_route:("readback://release-boundary/dirty-worktree/group/scope/" + group_safe($bucket.scope_bucket)),
      source_bucket:$bucket.scope_bucket,
      source_entry_count:$bucket.count,
      tracked_count:$bucket.tracked_count,
      untracked_count:$bucket.untracked_count,
      hepta_systems_owned_count:(if $owned then $bucket.count else 0 end),
      cross_lane_or_unowned_count:(if $owned then 0 else $bucket.count end),
      owner_hint:(if $owned then "hepta-systems" else "cross-lane-review" end),
      review_lane:(if $owned then "hepta-systems" else "cross-lane-review" end),
      freeze_state:"planned_not_applied",
      evidence_state:"not_recorded",
      operator_visible:true,
      queryable:true,
      diffable:true,
      freeze_plan_ready:true,
      freeze_applied:false,
      git_mutation_allowed:false,
      cleanup_allowed:false,
      evidence_recording_allowed:false,
      release_cutover_allowed:false
    };
  ($inventory[0]) as $inventory |
  ($inventory.top_level_buckets | map(top_level_entry)) as $top_entries |
  ($inventory.scope_buckets | map(scope_entry)) as $scope_entries |
  ($top_entries + $scope_entries) as $entries |
  ($entries | length) as $group_entry_count |
  ($entries | map(select(.freeze_plan_ready == true)) | length) as $freeze_plan_ready_count |
  ($entries | map(select(.freeze_state == "planned_not_applied")) | length) as $planned_not_applied_count |
  ($entries | map(select(.evidence_state != "not_recorded")) | length) as $evidence_recorded_count |
  ($inventory.release_boundary_inventory_ready == true
    and $inventory.dirty_worktree_release_boundary_open == true
    and $inventory.dirty_worktree_release_boundary_resolved == false
    and $inventory.inventory_entry_count > 0
    and $inventory.top_level_bucket_count == ($top_entries | length)
    and $inventory.scope_bucket_count == ($scope_entries | length)
    and $group_entry_count == (($top_entries | length) + ($scope_entries | length))
    and $freeze_plan_ready_count == $group_entry_count
    and $planned_not_applied_count == $group_entry_count
    and $evidence_recorded_count == 0
    and $lib_export_present == true
    and ($entries | all(.operator_visible == true
      and .queryable == true
      and .diffable == true
      and .freeze_applied == false
      and .git_mutation_allowed == false
      and .cleanup_allowed == false
      and .evidence_recording_allowed == false
      and .release_cutover_allowed == false))) as $freeze_plan_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_grouping_freeze_plan",
    status:(if $freeze_plan_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_grouping_freeze_plan_gate",
    schema_version:"dirty_worktree_release_boundary_grouping_freeze_plan_v1",
    plugin_id:$inventory.plugin_id,
    source_inventory_gate:$inventory.gate,
    source_inventory_ready:$inventory.release_boundary_inventory_ready,
    source_dirty_worktree_release_boundary_open:$inventory.dirty_worktree_release_boundary_open,
    source_dirty_worktree_release_boundary_resolved:$inventory.dirty_worktree_release_boundary_resolved,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$inventory.inventory_entry_count,
    tracked_change_count:$inventory.tracked_change_count,
    untracked_change_count:$inventory.untracked_change_count,
    hepta_systems_owned_count:$inventory.hepta_systems_owned_count,
    cross_lane_or_unowned_count:$inventory.cross_lane_or_unowned_count,
    grouping_scope:{
      plan_id:"dirty-worktree.release-boundary.grouping-freeze-plan.v1",
      plan_route:"readback://release-boundary/dirty-worktree/grouping-freeze-plan/v1",
      source_inventory_route:$inventory.inventory_scope.inventory_route,
      grouping_mode:"top_level_and_scope_bucket",
      freeze_mode:"plan_only_not_applied",
      mutation_boundary:"closed"
    },
    top_level_group_count:($top_entries | length),
    scope_group_count:($scope_entries | length),
    group_entry_count:$group_entry_count,
    freeze_plan_ready_count:$freeze_plan_ready_count,
    planned_not_applied_count:$planned_not_applied_count,
    release_evidence_bucket_count:$group_entry_count,
    evidence_recorded_count:$evidence_recorded_count,
    grouping_freeze_plan_ready:$freeze_plan_ready,
    freeze_applied:false,
    release_cutover_allowed:false,
    git_add_allowed:false,
    git_index_mutated:false,
    git_commit_allowed:false,
    git_push_allowed:false,
    git_reset_allowed:false,
    git_checkout_allowed:false,
    git_revert_allowed:false,
    cleanup_allowed:false,
    delete_allowed:false,
    evidence_recording_allowed:false,
    evidence_persistence_allowed:false,
    approval_acceptance_allowed:false,
    blocker_waiver_allowed:false,
    package_or_release_allowed:false,
    public_ga_allowed:false,
    canary_activation_allowed:false,
    live_activation_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "dirty_worktree_release_boundary_grouping_freeze_plan_not_applied",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation"
    ],
    next_migration_step:"phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      git_commit_created:false,
      git_push_performed:false,
      git_reset_performed:false,
      git_checkout_performed:false,
      git_revert_performed:false,
      unrelated_file_deleted:false,
      cleanup_performed:false,
      evidence_recorded:false,
      evidence_persisted:false,
      approval_accepted:false,
      blocker_waived:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      canary_activation_started:false,
      live_activation_started:false,
      live_execution_started:false
    }
  }'
