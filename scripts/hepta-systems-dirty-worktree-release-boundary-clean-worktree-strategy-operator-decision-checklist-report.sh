#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
GIT_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$GIT_BOUNDARY_REPORT" ]] || fail "missing executable Phase 17 git-mutation boundary readback report: $GIT_BOUNDARY_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 18 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 18 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 18 operator decision checklist report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$GIT_BOUNDARY_REPORT" >"$tmpdir/git_boundary.json" \
  || fail "failed to render Phase 17 git-mutation boundary readback report"
jq -e . "$tmpdir/git_boundary.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 17 git-mutation boundary readback report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile git_boundary "$tmpdir/git_boundary.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_2026-06-27.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def decision_entry:
    . as $entry
    | {
      source_git_boundary_readback_key:$entry.git_boundary_readback_key,
      source_git_boundary_readback_route:$entry.git_boundary_readback_route,
      checklist_key:("dirty_worktree.decision_checklist." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      checklist_route:("checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      decision_checkpoint:("operator_decision_checkpoint." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      owner_hint:$entry.owner_hint,
      review_lane:$entry.review_lane,
      recommended_strategy:$entry.recommended_strategy,
      operator_action:$entry.operator_action,
      evidence_requirement:$entry.evidence_requirement,
      decision_state:$entry.decision_state,
      checklist_state:"ready_blocked_pending_operator_decision",
      packet_visible:$entry.packet_visible,
      packet_unsent:$entry.packet_unsent,
      packet_unpersisted:$entry.packet_unpersisted,
      readback_unpersisted:$entry.readback_unpersisted,
      operator_visible:true,
      queryable:true,
      diffable:true,
      operator_decision_required:true,
      decision_recording_allowed:false,
      approval_acceptance_allowed:false,
      evidence_recording_allowed:false,
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
      release_cutover_allowed:false,
      canary_activation_allowed:false,
      live_execution_allowed:false
    };
  ($git_boundary[0]) as $git_boundary |
  ($git_boundary.entries | map(decision_entry)) as $entries |
  ($entries | length) as $checklist_entry_count |
  ($entries | map(.checklist_key) | unique | length) as $stable_checklist_key_count |
  ($entries | map(.checklist_route) | unique | length) as $checklist_route_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true and .operator_visible == true and .queryable == true and .diffable == true and .operator_decision_required == true and .decision_recording_allowed == false and .approval_acceptance_allowed == false and .evidence_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .strategy_application_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $checklist_ready_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true)) | length) as $packet_visible_unsent_unpersisted_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.strategy_application_blocked == true)) | length) as $strategy_application_blocked_count |
  ($entries | map(select(.operator_decision_required == true)) | length) as $operator_decision_required_count |
  ($entries | map(select(.decision_state == "pending_operator_decision")) | length) as $pending_operator_decision_count |
  ($entries | map(select((.evidence_requirement | length) > 0)) | length) as $evidence_requirement_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($git_boundary.git_mutation_boundary_readback_ready == true
    and $git_boundary.operator_packet_visible == true
    and $git_boundary.operator_packet_sent == false
    and $git_boundary.operator_packet_persisted == false
    and $git_boundary.readback_persisted == false
    and $git_boundary.strategy_applied == false
    and $lib_export_present == true
    and $checklist_entry_count == $git_boundary.readback_entry_count
    and $stable_checklist_key_count == $checklist_entry_count
    and $checklist_route_count == $checklist_entry_count
    and $checklist_ready_count == $checklist_entry_count
    and $packet_visible_unsent_unpersisted_count == $checklist_entry_count
    and $git_mutation_blocked_count == $checklist_entry_count
    and $cleanup_delete_blocked_count == $checklist_entry_count
    and $strategy_application_blocked_count == $checklist_entry_count
    and $operator_decision_required_count == $checklist_entry_count
    and $pending_operator_decision_count == $checklist_entry_count
    and $evidence_requirement_count == $checklist_entry_count
    and $evidence_recorded_count == 0) as $operator_decision_checklist_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist",
    status:(if $operator_decision_checklist_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_gate",
    schema_version:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_v1",
    plugin_id:$git_boundary.plugin_id,
    source_git_boundary_readback_gate:$git_boundary.gate,
    source_git_boundary_readback_ready:$git_boundary.git_mutation_boundary_readback_ready,
    source_operator_packet_visible:$git_boundary.operator_packet_visible,
    source_operator_packet_sent:$git_boundary.operator_packet_sent,
    source_operator_packet_persisted:$git_boundary.operator_packet_persisted,
    source_readback_persisted:$git_boundary.readback_persisted,
    source_strategy_applied:$git_boundary.strategy_applied,
    source_readback_entry_count:$git_boundary.readback_entry_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$git_boundary.inventory_entry_count,
    tracked_change_count:$git_boundary.tracked_change_count,
    untracked_change_count:$git_boundary.untracked_change_count,
    checklist_scope:{
      checklist_id:"dirty-worktree.release-boundary.clean-worktree-strategy.operator-decision-checklist.v1",
      checklist_route:"checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/v1",
      source_git_boundary_readback_route:$git_boundary.readback_scope.readback_route,
      checklist_mode:"operator_decision_checklist_only",
      decision_recording_boundary:"blocked",
      git_mutation_boundary:"closed",
      cleanup_boundary:"blocked",
      evidence_boundary:"blocked"
    },
    checklist_entry_count:$checklist_entry_count,
    stable_checklist_key_count:$stable_checklist_key_count,
    checklist_route_count:$checklist_route_count,
    checklist_ready_count:$checklist_ready_count,
    packet_visible_unsent_unpersisted_count:$packet_visible_unsent_unpersisted_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    strategy_application_blocked_count:$strategy_application_blocked_count,
    operator_decision_required_count:$operator_decision_required_count,
    pending_operator_decision_count:$pending_operator_decision_count,
    evidence_requirement_count:$evidence_requirement_count,
    evidence_recorded_count:$evidence_recorded_count,
    decision_checklist_visible:$operator_decision_checklist_ready,
    decision_checklist_persisted:false,
    decision_recorded:false,
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
    operator_decision_checklist_ready:$operator_decision_checklist_ready,
    entries:$entries,
    blockers:[
      "operator_decision_recording_blocked",
      "approval_acceptance_blocked",
      "evidence_recording_blocked",
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
      "readback_persistence_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation",
      "keep_operator_decision_checklist_pending_without_recording_or_git_mutation"
    ],
    next_migration_step:"phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      decision_checklist_persisted:false,
      decision_recorded:false,
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
