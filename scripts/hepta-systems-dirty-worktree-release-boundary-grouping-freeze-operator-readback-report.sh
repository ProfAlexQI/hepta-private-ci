#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
GROUPING_FREEZE_PLAN_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_grouping_freeze_operator_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$GROUPING_FREEZE_PLAN_REPORT" ]] || fail "missing executable Phase 12 grouping freeze-plan report: $GROUPING_FREEZE_PLAN_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 13 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 13 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 13 dirty-worktree grouping freeze operator readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$GROUPING_FREEZE_PLAN_REPORT" >"$tmpdir/grouping_freeze_plan.json" \
  || fail "failed to render Phase 12 dirty-worktree grouping freeze-plan report"
jq -e . "$tmpdir/grouping_freeze_plan.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 12 dirty-worktree grouping freeze-plan report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_grouping_freeze_operator_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile plan "$tmpdir/grouping_freeze_plan.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_2026-06-27.md" \
  '
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_"));
  def route_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "-"));
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def readback_entry:
    . as $entry
    | {
      source_group_key:$entry.group_key,
      source_group_route:$entry.group_route,
      readback_key:("dirty_worktree.readback." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      readback_route:("readback://release-boundary/dirty-worktree/grouping-freeze/operator/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      diff_key:("dirty_worktree.diff." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      comparison_anchor:("dirty_worktree.anchor." + $entry.group_type + "." + key_safe($entry.source_bucket) + ".v1"),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      hepta_systems_owned_count:$entry.hepta_systems_owned_count,
      cross_lane_or_unowned_count:$entry.cross_lane_or_unowned_count,
      owner_hint:$entry.owner_hint,
      review_lane:$entry.review_lane,
      operator_status:"blocked_pending_clean_worktree_strategy",
      previous_freeze_state:$entry.freeze_state,
      current_freeze_state:$entry.freeze_state,
      freeze_state_delta:"unchanged_planned_not_applied",
      previous_evidence_state:$entry.evidence_state,
      current_evidence_state:$entry.evidence_state,
      evidence_state_delta:"unchanged_not_recorded",
      operator_visible:true,
      queryable:true,
      diffable:true,
      readback_ready:true,
      freeze_applied:false,
      git_mutation_allowed:false,
      cleanup_allowed:false,
      evidence_recording_allowed:false,
      release_cutover_allowed:false,
      live_execution_allowed:false
    };
  ($plan[0]) as $plan |
  ($plan.entries | map(readback_entry)) as $entries |
  ($entries | length) as $readback_entry_count |
  ($entries | map(.readback_key) | unique | length) as $stable_readback_key_count |
  ($entries | map(.diff_key) | unique | length) as $diff_key_count |
  ($entries | map(.comparison_anchor) | unique | length) as $comparison_anchor_count |
  ($entries | map(select(.current_freeze_state == "planned_not_applied")) | length) as $planned_not_applied_readback_count |
  ($entries | map(select(.freeze_state_delta == "unchanged_planned_not_applied")) | length) as $unchanged_freeze_state_count |
  ($entries | map(select(.evidence_state_delta == "unchanged_not_recorded")) | length) as $unchanged_evidence_state_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($plan.grouping_freeze_plan_ready == true
    and $plan.freeze_applied == false
    and $plan.group_entry_count == $readback_entry_count
    and $stable_readback_key_count == $readback_entry_count
    and $diff_key_count == $readback_entry_count
    and $comparison_anchor_count == $readback_entry_count
    and $planned_not_applied_readback_count == $readback_entry_count
    and $unchanged_freeze_state_count == $readback_entry_count
    and $unchanged_evidence_state_count == $readback_entry_count
    and $evidence_recorded_count == 0
    and $lib_export_present == true
    and ($entries | all(.operator_visible == true
      and .queryable == true
      and .diffable == true
      and .readback_ready == true
      and .freeze_applied == false
      and .git_mutation_allowed == false
      and .cleanup_allowed == false
      and .evidence_recording_allowed == false
      and .release_cutover_allowed == false
      and .live_execution_allowed == false))) as $operator_readback_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_grouping_freeze_operator_readback",
    status:(if $operator_readback_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_grouping_freeze_operator_readback_gate",
    schema_version:"dirty_worktree_release_boundary_grouping_freeze_operator_readback_v1",
    plugin_id:$plan.plugin_id,
    source_grouping_freeze_plan_gate:$plan.gate,
    source_grouping_freeze_plan_ready:$plan.grouping_freeze_plan_ready,
    source_freeze_applied:$plan.freeze_applied,
    source_group_entry_count:$plan.group_entry_count,
    source_planned_not_applied_count:$plan.planned_not_applied_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$plan.inventory_entry_count,
    tracked_change_count:$plan.tracked_change_count,
    untracked_change_count:$plan.untracked_change_count,
    readback_scope:{
      readback_id:"dirty-worktree.release-boundary.grouping-freeze.operator-readback.v1",
      readback_route:"readback://release-boundary/dirty-worktree/grouping-freeze/operator-readback/v1",
      source_plan_route:$plan.grouping_scope.plan_route,
      readback_mode:"operator_readback_diff_only",
      diff_mode:"stable_key_state_delta",
      mutation_boundary:"closed"
    },
    readback_entry_count:$readback_entry_count,
    stable_readback_key_count:$stable_readback_key_count,
    diff_key_count:$diff_key_count,
    comparison_anchor_count:$comparison_anchor_count,
    planned_not_applied_readback_count:$planned_not_applied_readback_count,
    unchanged_freeze_state_count:$unchanged_freeze_state_count,
    unchanged_evidence_state_count:$unchanged_evidence_state_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_readback_ready:$operator_readback_ready,
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
      "dirty_worktree_release_boundary_operator_readback_not_clean",
      "freeze_application_blocked",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase14_dirty_worktree_release_boundary_actionable_clean_worktree_strategy_without_git_mutation"
    ],
    next_migration_step:"phase14_dirty_worktree_release_boundary_actionable_clean_worktree_strategy_without_git_mutation",
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
      freeze_applied:false,
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
