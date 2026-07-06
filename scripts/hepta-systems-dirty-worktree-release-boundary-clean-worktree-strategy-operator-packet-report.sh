#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
STRATEGY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$STRATEGY_REPORT" ]] || fail "missing executable Phase 14 clean-worktree strategy report: $STRATEGY_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 15 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 15 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 15 dirty-worktree strategy operator packet report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$STRATEGY_REPORT" >"$tmpdir/strategy.json" \
  || fail "failed to render Phase 14 dirty-worktree clean-worktree strategy report"
jq -e . "$tmpdir/strategy.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 14 dirty-worktree clean-worktree strategy report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile strategy "$tmpdir/strategy.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_2026-06-27.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_"));
  def route_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "-"));
  def section($id; $title; $source): {
    id:$id,
    title:$title,
    source:$source,
    preview_ready:true,
    mutation_enabled:false
  };
  def packet_entry:
    . as $entry
    | {
      source_strategy_key:$entry.strategy_key,
      source_strategy_route:$entry.strategy_route,
      packet_key:("dirty_worktree.packet." + $entry.group_type + "." + key_safe($entry.source_bucket)),
      packet_route:("operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
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
      packet_section:"strategy_entries",
      decision_state:$entry.decision_state,
      attached_to_packet:true,
      operator_visible:true,
      queryable:true,
      diffable:true,
      operator_decision_required:true,
      packet_sent:false,
      packet_persisted:false,
      strategy_applied:false,
      git_mutation_allowed:false,
      cleanup_allowed:false,
      delete_allowed:false,
      evidence_recording_allowed:false,
      release_cutover_allowed:false,
      live_execution_allowed:false
    };
  ($strategy[0]) as $strategy |
  [
    section("scope"; "Scope"; "Phase 14 clean-worktree strategy"),
    section("inventory_summary"; "Inventory Summary"; "dirty-worktree inventory"),
    section("strategy_entries"; "Strategy Entries"; "Phase 14 clean-worktree strategy"),
    section("operator_decisions"; "Operator Decisions"; "pending operator decisions"),
    section("evidence_requirements"; "Evidence Requirements"; "clean-worktree decision record"),
    section("closed_boundary"; "Closed Boundary"; "local packet preview")
  ] as $sections |
  ($strategy.entries | map(packet_entry)) as $entries |
  ($entries | length) as $packet_entry_count |
  ($entries | map(.packet_key) | unique | length) as $stable_packet_key_count |
  ($entries | map(.packet_route) | unique | length) as $packet_route_count |
  ($entries | map(select(.attached_to_packet == true)) | length) as $attached_strategy_count |
  ($entries | map(select(.operator_decision_required == true)) | length) as $operator_decision_required_count |
  ($entries | map(select(.git_mutation_allowed == false)) | length) as $no_git_mutation_packet_count |
  ($entries | map(select(.review_lane == "hepta-systems")) | length) as $hepta_systems_packet_count |
  ($entries | map(select(.review_lane == "cross-lane-review" or .review_lane == "external-or-cross-lane")) | length) as $cross_lane_packet_count |
  ($entries | map(select(.review_lane == "mixed" or .review_lane == "mixed-hepta-and-cross-lane")) | length) as $mixed_lane_packet_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($strategy.strategy_ready == true
    and $strategy.strategy_applied == false
    and $lib_export_present == true
    and ($sections | length) == 6
    and ($sections | all(.preview_ready == true and .mutation_enabled == false))
    and $packet_entry_count == $strategy.strategy_entry_count
    and $stable_packet_key_count == $packet_entry_count
    and $packet_route_count == $packet_entry_count
    and $attached_strategy_count == $packet_entry_count
    and $operator_decision_required_count == $packet_entry_count
    and $no_git_mutation_packet_count == $packet_entry_count
    and $evidence_recorded_count == 0
    and ($entries | all(.operator_visible == true
      and .queryable == true
      and .diffable == true
      and .attached_to_packet == true
      and .operator_decision_required == true
      and .packet_sent == false
      and .packet_persisted == false
      and .strategy_applied == false
      and .git_mutation_allowed == false
      and .cleanup_allowed == false
      and .delete_allowed == false
      and .evidence_recording_allowed == false
      and .release_cutover_allowed == false
      and .live_execution_allowed == false))) as $operator_packet_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet",
    status:(if $operator_packet_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_gate",
    schema_version:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_v1",
    plugin_id:$strategy.plugin_id,
    source_strategy_gate:$strategy.gate,
    source_strategy_ready:$strategy.strategy_ready,
    source_strategy_applied:$strategy.strategy_applied,
    source_strategy_entry_count:$strategy.strategy_entry_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$strategy.inventory_entry_count,
    tracked_change_count:$strategy.tracked_change_count,
    untracked_change_count:$strategy.untracked_change_count,
    packet_scope:{
      packet_id:"dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.v1",
      packet_route:"operator-packet://release-boundary/dirty-worktree/clean-worktree-strategy/v1",
      source_strategy_route:$strategy.strategy_scope.strategy_route,
      packet_mode:"operator_packet_preview_only",
      send_mode:"not_sent_not_persisted",
      mutation_boundary:"closed"
    },
    packet_section_count:($sections | length),
    packet_entry_count:$packet_entry_count,
    stable_packet_key_count:$stable_packet_key_count,
    packet_route_count:$packet_route_count,
    attached_strategy_count:$attached_strategy_count,
    operator_decision_required_count:$operator_decision_required_count,
    no_git_mutation_packet_count:$no_git_mutation_packet_count,
    hepta_systems_packet_count:$hepta_systems_packet_count,
    cross_lane_packet_count:$cross_lane_packet_count,
    mixed_lane_packet_count:$mixed_lane_packet_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_packet_ready:$operator_packet_ready,
    operator_packet_sent:false,
    operator_packet_persisted:false,
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
    sections:$sections,
    entries:$entries,
    blockers:[
      "operator_packet_not_sent",
      "operator_packet_not_persisted",
      "strategy_application_blocked",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "phase16_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_without_git_mutation"
    ],
    next_migration_step:"phase16_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      packet_sent:false,
      packet_persisted:false,
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
