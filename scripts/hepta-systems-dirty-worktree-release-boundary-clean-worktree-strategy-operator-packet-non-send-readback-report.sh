#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
OPERATOR_PACKET_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$OPERATOR_PACKET_REPORT" ]] || fail "missing executable Phase 15 operator packet report: $OPERATOR_PACKET_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 16 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 16 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 16 clean-worktree strategy operator packet non-send readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$OPERATOR_PACKET_REPORT" >"$tmpdir/operator_packet.json" \
  || fail "failed to render Phase 15 clean-worktree strategy operator packet report"
jq -e . "$tmpdir/operator_packet.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 15 clean-worktree strategy operator packet report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile packet "$tmpdir/operator_packet.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_2026-06-27.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def readback_entry:
    . as $entry
    | {
      source_packet_key:$entry.packet_key,
      source_packet_route:$entry.packet_route,
      non_send_readback_key:("dirty_worktree.packet.non_send." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      non_send_readback_route:("readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
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
      observed_state:"operator_packet_visible_unsent_unpersisted",
      previous_send_state:"unsent",
      current_send_state:"unsent",
      send_state_delta:"unchanged_unsent",
      previous_persistence_state:"unpersisted",
      current_persistence_state:"unpersisted",
      persistence_state_delta:"unchanged_unpersisted",
      packet_visible:true,
      non_send_confirmed:true,
      non_persistence_confirmed:true,
      operator_visible:true,
      queryable:true,
      diffable:true,
      operator_decision_required:true,
      packet_send_blocked:true,
      packet_persistence_blocked:true,
      approval_request_blocked:true,
      strategy_application_blocked:true,
      git_mutation_allowed:false,
      cleanup_allowed:false,
      delete_allowed:false,
      evidence_recording_allowed:false,
      release_cutover_allowed:false,
      live_execution_allowed:false
    };
  ($packet[0]) as $packet |
  ($packet.entries | map(readback_entry)) as $entries |
  ($entries | length) as $readback_entry_count |
  ($entries | map(.non_send_readback_key) | unique | length) as $stable_readback_key_count |
  ($entries | map(.non_send_readback_route) | unique | length) as $readback_route_count |
  ($entries | map(select(.packet_visible == true and .non_send_confirmed == true and .non_persistence_confirmed == true and .operator_visible == true and .queryable == true and .diffable == true and .packet_send_blocked == true and .packet_persistence_blocked == true and .approval_request_blocked == true and .strategy_application_blocked == true and .git_mutation_allowed == false and .cleanup_allowed == false and .delete_allowed == false and .evidence_recording_allowed == false and .release_cutover_allowed == false and .live_execution_allowed == false)) | length) as $readback_ready_count |
  ($entries | map(select(.observed_state == "operator_packet_visible_unsent_unpersisted" and .current_send_state == "unsent" and .current_persistence_state == "unpersisted")) | length) as $visible_unsent_unpersisted_count |
  ($entries | map(select((.source_packet_key | length) > 0)) | length) as $attached_packet_count |
  ($entries | map(select(.operator_decision_required == true)) | length) as $operator_decision_required_count |
  ($entries | map(select(.git_mutation_allowed == false)) | length) as $no_git_mutation_readback_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($packet.operator_packet_ready == true
    and $packet.operator_packet_sent == false
    and $packet.operator_packet_persisted == false
    and $packet.strategy_applied == false
    and $lib_export_present == true
    and $readback_entry_count == $packet.packet_entry_count
    and $stable_readback_key_count == $readback_entry_count
    and $readback_route_count == $readback_entry_count
    and $readback_ready_count == $readback_entry_count
    and $visible_unsent_unpersisted_count == $readback_entry_count
    and $attached_packet_count == $readback_entry_count
    and $operator_decision_required_count == $readback_entry_count
    and $no_git_mutation_readback_count == $readback_entry_count
    and $evidence_recorded_count == 0) as $non_send_readback_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback",
    status:(if $non_send_readback_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_gate",
    schema_version:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_v1",
    plugin_id:$packet.plugin_id,
    source_operator_packet_gate:$packet.gate,
    source_operator_packet_ready:$packet.operator_packet_ready,
    source_operator_packet_sent:$packet.operator_packet_sent,
    source_operator_packet_persisted:$packet.operator_packet_persisted,
    source_strategy_applied:$packet.strategy_applied,
    source_packet_entry_count:$packet.packet_entry_count,
    source_packet_section_count:$packet.packet_section_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$packet.inventory_entry_count,
    tracked_change_count:$packet.tracked_change_count,
    untracked_change_count:$packet.untracked_change_count,
    readback_scope:{
      readback_id:"dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.non-send-readback.v1",
      readback_route:"readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/v1",
      source_packet_route:$packet.packet_scope.packet_route,
      readback_mode:"operator_packet_non_send_readback_only",
      send_boundary:"blocked",
      persistence_boundary:"blocked",
      git_mutation_boundary:"closed"
    },
    readback_entry_count:$readback_entry_count,
    stable_readback_key_count:$stable_readback_key_count,
    readback_route_count:$readback_route_count,
    readback_ready_count:$readback_ready_count,
    visible_unsent_unpersisted_count:$visible_unsent_unpersisted_count,
    attached_packet_count:$attached_packet_count,
    operator_decision_required_count:$operator_decision_required_count,
    no_git_mutation_readback_count:$no_git_mutation_readback_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_packet_visible:$non_send_readback_ready,
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
    non_send_readback_ready:$non_send_readback_ready,
    entries:$entries,
    blockers:[
      "operator_packet_send_blocked",
      "operator_packet_persistence_blocked",
      "operator_packet_readback_persistence_blocked",
      "strategy_application_blocked",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase17_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_without_git_mutation",
      "keep_clean_worktree_strategy_operator_packet_visible_unsent_unpersisted_without_git_mutation"
    ],
    next_migration_step:"phase17_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      packet_sent:false,
      packet_persisted:false,
      readback_persisted:false,
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
