#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
NON_SEND_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$NON_SEND_READBACK_REPORT" ]] || fail "missing executable Phase 16 non-send readback report: $NON_SEND_READBACK_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 17 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 17 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 17 clean-worktree strategy operator packet git-mutation boundary readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$NON_SEND_READBACK_REPORT" >"$tmpdir/non_send_readback.json" \
  || fail "failed to render Phase 16 clean-worktree strategy operator packet non-send readback report"
jq -e . "$tmpdir/non_send_readback.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 16 clean-worktree strategy operator packet non-send readback report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile non_send "$tmpdir/non_send_readback.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_2026-06-27.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def git_boundary_entry:
    . as $entry
    | {
      source_non_send_readback_key:$entry.non_send_readback_key,
      source_non_send_readback_route:$entry.non_send_readback_route,
      git_boundary_readback_key:("dirty_worktree.packet.git_boundary." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      git_boundary_readback_route:("readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      hepta_systems_owned_count:$entry.hepta_systems_owned_count,
      cross_lane_or_unowned_count:$entry.cross_lane_or_unowned_count,
      owner_hint:$entry.owner_hint,
      review_lane:$entry.review_lane,
      recommended_strategy:$entry.recommended_strategy,
      operator_action:$entry.operator_action,
      evidence_requirement:$entry.evidence_requirement,
      decision_state:$entry.decision_state,
      previous_git_mutation_state:"blocked",
      current_git_mutation_state:"blocked",
      git_mutation_state_delta:"unchanged_blocked",
      packet_visible:$entry.packet_visible,
      packet_unsent:$entry.non_send_confirmed,
      packet_unpersisted:$entry.non_persistence_confirmed,
      readback_unpersisted:true,
      operator_visible:true,
      queryable:true,
      diffable:true,
      operator_decision_required:true,
      git_add_blocked:true,
      git_index_mutation_blocked:true,
      git_commit_blocked:true,
      git_push_blocked:true,
      git_reset_blocked:true,
      git_checkout_blocked:true,
      git_revert_blocked:true,
      cleanup_blocked:true,
      delete_blocked:true,
      strategy_application_blocked:true,
      evidence_recording_allowed:false,
      release_cutover_allowed:false,
      live_execution_allowed:false
    };
  ($non_send[0]) as $non_send |
  ($non_send.entries | map(git_boundary_entry)) as $entries |
  ($entries | length) as $readback_entry_count |
  ($entries | map(.git_boundary_readback_key) | unique | length) as $stable_readback_key_count |
  ($entries | map(.git_boundary_readback_route) | unique | length) as $readback_route_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true and .operator_visible == true and .queryable == true and .diffable == true and .operator_decision_required == true and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .strategy_application_blocked == true and .evidence_recording_allowed == false and .release_cutover_allowed == false and .live_execution_allowed == false)) | length) as $readback_ready_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true)) | length) as $packet_visible_unsent_unpersisted_count |
  ($entries | map(select(.previous_git_mutation_state == "blocked" and .current_git_mutation_state == "blocked" and .git_mutation_state_delta == "unchanged_blocked" and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_operation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.operator_decision_required == true)) | length) as $operator_decision_required_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($non_send.non_send_readback_ready == true
    and $non_send.operator_packet_visible == true
    and $non_send.operator_packet_sent == false
    and $non_send.operator_packet_persisted == false
    and $non_send.readback_persisted == false
    and $non_send.strategy_applied == false
    and $lib_export_present == true
    and $readback_entry_count == $non_send.readback_entry_count
    and $stable_readback_key_count == $readback_entry_count
    and $readback_route_count == $readback_entry_count
    and $readback_ready_count == $readback_entry_count
    and $packet_visible_unsent_unpersisted_count == $readback_entry_count
    and $git_mutation_blocked_count == $readback_entry_count
    and $git_operation_blocked_count == $readback_entry_count
    and $cleanup_delete_blocked_count == $readback_entry_count
    and $operator_decision_required_count == $readback_entry_count
    and $evidence_recorded_count == 0) as $git_mutation_boundary_readback_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback",
    status:(if $git_mutation_boundary_readback_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_gate",
    schema_version:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_v1",
    plugin_id:$non_send.plugin_id,
    source_non_send_readback_gate:$non_send.gate,
    source_non_send_readback_ready:$non_send.non_send_readback_ready,
    source_operator_packet_visible:$non_send.operator_packet_visible,
    source_operator_packet_sent:$non_send.operator_packet_sent,
    source_operator_packet_persisted:$non_send.operator_packet_persisted,
    source_readback_persisted:$non_send.readback_persisted,
    source_strategy_applied:$non_send.strategy_applied,
    source_readback_entry_count:$non_send.readback_entry_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$non_send.inventory_entry_count,
    tracked_change_count:$non_send.tracked_change_count,
    untracked_change_count:$non_send.untracked_change_count,
    readback_scope:{
      readback_id:"dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.git-mutation-boundary-readback.v1",
      readback_route:"readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1",
      source_non_send_readback_route:$non_send.readback_scope.readback_route,
      readback_mode:"git_mutation_boundary_readback_only",
      git_mutation_boundary:"closed",
      git_index_boundary:"blocked",
      cleanup_boundary:"blocked",
      deletion_boundary:"blocked"
    },
    readback_entry_count:$readback_entry_count,
    stable_readback_key_count:$stable_readback_key_count,
    readback_route_count:$readback_route_count,
    readback_ready_count:$readback_ready_count,
    packet_visible_unsent_unpersisted_count:$packet_visible_unsent_unpersisted_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    git_operation_blocked_count:$git_operation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    operator_decision_required_count:$operator_decision_required_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_packet_visible:$git_mutation_boundary_readback_ready,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    readback_persisted:false,
    strategy_applied:false,
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
    approval_request_sent:false,
    approval_acceptance_allowed:false,
    blocker_waiver_allowed:false,
    package_or_release_allowed:false,
    public_ga_allowed:false,
    canary_activation_allowed:false,
    live_activation_allowed:false,
    live_execution_allowed:false,
    git_mutation_boundary_readback_ready:$git_mutation_boundary_readback_ready,
    entries:$entries,
    blockers:[
      "git_add_blocked",
      "git_index_mutation_blocked",
      "git_commit_blocked",
      "git_push_blocked",
      "git_reset_blocked",
      "git_checkout_blocked",
      "git_revert_blocked",
      "cleanup_and_delete_blocked",
      "strategy_application_blocked",
      "operator_packet_send_blocked",
      "operator_packet_persistence_blocked",
      "operator_packet_readback_persistence_blocked",
      "evidence_recording_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase18_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_without_git_mutation",
      "keep_git_mutation_cleanup_delete_release_and_live_blocked"
    ],
    next_migration_step:"phase18_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      packet_sent:false,
      packet_persisted:false,
      readback_persisted:false,
      git_add_performed:false,
      git_index_mutated:false,
      git_commit_created:false,
      git_push_performed:false,
      git_reset_performed:false,
      git_checkout_performed:false,
      git_revert_performed:false,
      cleanup_performed:false,
      unrelated_file_deleted:false,
      strategy_applied:false,
      evidence_recorded:false,
      evidence_persisted:false,
      approval_requested:false,
      approval_accepted:false,
      blocker_waived:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      canary_activation_started:false,
      live_activation_started:false,
      live_execution_started:false
    }
  }'
