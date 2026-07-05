#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_2026-06-28.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 20 decision recording boundary readback report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 21 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 21 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 21 approval acceptance boundary readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render Phase 20 decision recording boundary readback report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 20 decision recording boundary readback report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_2026-06-28.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_") | gsub("-"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def approval_entry:
    . as $entry
    | {
      source_boundary_key:$entry.boundary_key,
      source_boundary_route:$entry.boundary_route,
      source_packet_key:$entry.source_packet_key,
      source_packet_route:$entry.source_packet_route,
      source_packet_readback_key:$entry.source_packet_readback_key,
      source_packet_readback_route:$entry.source_packet_readback_route,
      approval_boundary_key:("dirty_worktree.approval_acceptance_boundary." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      approval_boundary_route:("readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-approval-acceptance-boundary/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      approval_checkpoint:("approval_acceptance_boundary." + $entry.decision_checkpoint),
      decision_checkpoint:$entry.decision_checkpoint,
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
      approval_request_state:"approval_request_blocked",
      approval_acceptance_state:"approval_acceptance_blocked",
      approval_recording_state:"approval_recording_blocked",
      approval_receipt_state:"approval_receipt_blocked",
      decision_recording_state:$entry.recording_state,
      decision_persistence_state:$entry.persistence_state,
      source_boundary_state:"decision_recording_boundary_visible_unpersisted",
      source_packet_state:$entry.source_packet_state,
      source_readback_state:$entry.source_readback_state,
      packet_visible:$entry.packet_visible,
      packet_unsent:$entry.packet_unsent,
      packet_unpersisted:$entry.packet_unpersisted,
      readback_visible:$entry.readback_visible,
      readback_unpersisted:$entry.readback_unpersisted,
      operator_visible:$entry.operator_visible,
      queryable:$entry.queryable,
      diffable:$entry.diffable,
      operator_decision_required:$entry.operator_decision_required,
      approval_request_allowed:false,
      approval_acceptance_allowed:false,
      approval_recording_allowed:false,
      approval_receipt_persistence_allowed:false,
      decision_recording_allowed:false,
      decision_persistence_allowed:false,
      decision_receipt_persistence_allowed:false,
      evidence_recording_allowed:false,
      git_add_blocked:$entry.git_add_blocked,
      git_index_mutation_blocked:$entry.git_index_mutation_blocked,
      git_commit_blocked:$entry.git_commit_blocked,
      git_push_blocked:$entry.git_push_blocked,
      git_reset_blocked:$entry.git_reset_blocked,
      git_checkout_blocked:$entry.git_checkout_blocked,
      git_revert_blocked:$entry.git_revert_blocked,
      cleanup_blocked:$entry.cleanup_blocked,
      delete_blocked:$entry.delete_blocked,
      strategy_application_blocked:$entry.strategy_application_blocked,
      release_cutover_allowed:false,
      canary_activation_allowed:false,
      live_execution_allowed:false
    };
  ($source[0]) as $source_report |
  ($source_report.entries | map(approval_entry)) as $entries |
  ($entries | length) as $boundary_entry_count |
  ($entries | map(.approval_boundary_key) | unique | length) as $stable_boundary_key_count |
  ($entries | map(.approval_boundary_route) | unique | length) as $boundary_route_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_visible == true and .readback_unpersisted == true and .operator_visible == true and .queryable == true and .diffable == true and .operator_decision_required == true and .decision_state == "pending_operator_decision" and .approval_request_state == "approval_request_blocked" and .approval_acceptance_state == "approval_acceptance_blocked" and .approval_recording_state == "approval_recording_blocked" and .approval_receipt_state == "approval_receipt_blocked" and .decision_recording_state == "decision_recording_blocked" and .decision_persistence_state == "decision_persistence_blocked" and .source_boundary_state == "decision_recording_boundary_visible_unpersisted" and .approval_request_allowed == false and .approval_acceptance_allowed == false and .approval_recording_allowed == false and .approval_receipt_persistence_allowed == false and .decision_recording_allowed == false and .decision_persistence_allowed == false and .decision_receipt_persistence_allowed == false and .evidence_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .strategy_application_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $boundary_ready_count |
  ($entries | map(select((.source_boundary_key | length) > 0 and (.source_boundary_route | length) > 0 and (.source_packet_key | length) > 0 and (.source_packet_route | length) > 0 and (.source_packet_readback_key | length) > 0 and (.source_packet_readback_route | length) > 0)) | length) as $source_boundary_attached_count |
  ($entries | map(select(.decision_state == "pending_operator_decision")) | length) as $pending_operator_decision_count |
  ($entries | map(select(.approval_request_allowed == false)) | length) as $approval_request_blocked_count |
  ($entries | map(select(.approval_acceptance_allowed == false)) | length) as $approval_acceptance_blocked_count |
  ($entries | map(select(.approval_recording_allowed == false)) | length) as $approval_recording_blocked_count |
  ($entries | map(select(.approval_receipt_persistence_allowed == false)) | length) as $approval_receipt_blocked_count |
  ($entries | map(select(.decision_recording_allowed == false)) | length) as $decision_recording_blocked_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true)) | length) as $packet_visible_unsent_unpersisted_count |
  ($entries | map(select(.readback_visible == true and .readback_unpersisted == true)) | length) as $readback_unpersisted_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.strategy_application_blocked == true)) | length) as $strategy_application_blocked_count |
  ($source_report.operator_decision_recording_boundary_readback_ready == true
    and $source_report.decision_recording_boundary_readback_visible == true
    and $source_report.decision_recording_boundary_readback_persisted == false
    and $source_report.decision_recorded == false
    and $source_report.decision_recording_persisted == false
    and $source_report.decision_receipt_persisted == false
    and $source_report.operator_packet_sent == false
    and $source_report.operator_packet_persisted == false
    and $source_report.readback_persisted == false
    and $source_report.strategy_applied == false
    and $lib_export_present == true
    and $boundary_entry_count == $source_report.boundary_entry_count
    and $stable_boundary_key_count == $boundary_entry_count
    and $boundary_route_count == $boundary_entry_count
    and $boundary_ready_count == $boundary_entry_count
    and $source_boundary_attached_count == $boundary_entry_count
    and $pending_operator_decision_count == $boundary_entry_count
    and $approval_request_blocked_count == $boundary_entry_count
    and $approval_acceptance_blocked_count == $boundary_entry_count
    and $approval_recording_blocked_count == $boundary_entry_count
    and $approval_receipt_blocked_count == $boundary_entry_count
    and $decision_recording_blocked_count == $boundary_entry_count
    and $evidence_recorded_count == 0
    and $packet_visible_unsent_unpersisted_count == $boundary_entry_count
    and $readback_unpersisted_count == $boundary_entry_count
    and $git_mutation_blocked_count == $boundary_entry_count
    and $cleanup_delete_blocked_count == $boundary_entry_count
    and $strategy_application_blocked_count == $boundary_entry_count) as $boundary_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback",
    status:(if $boundary_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_gate",
    schema_version:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_v1",
    plugin_id:$source_report.plugin_id,
    source_decision_recording_boundary_gate:$source_report.gate,
    source_decision_recording_boundary_ready:$source_report.operator_decision_recording_boundary_readback_ready,
    source_decision_recording_boundary_visible:$source_report.decision_recording_boundary_readback_visible,
    source_decision_recording_boundary_persisted:$source_report.decision_recording_boundary_readback_persisted,
    source_decision_recorded:$source_report.decision_recorded,
    source_decision_recording_persisted:$source_report.decision_recording_persisted,
    source_decision_receipt_persisted:$source_report.decision_receipt_persisted,
    source_operator_packet_sent:$source_report.operator_packet_sent,
    source_operator_packet_persisted:$source_report.operator_packet_persisted,
    source_readback_persisted:$source_report.readback_persisted,
    source_strategy_applied:$source_report.strategy_applied,
    source_boundary_entry_count:$source_report.boundary_entry_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$source_report.inventory_entry_count,
    tracked_change_count:$source_report.tracked_change_count,
    untracked_change_count:$source_report.untracked_change_count,
    approval_acceptance_boundary_scope:{
      boundary_readback_id:"dirty-worktree.release-boundary.clean-worktree-strategy.operator-approval-acceptance-boundary-readback.v1",
      boundary_readback_route:"readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-approval-acceptance-boundary/v1",
      source_decision_recording_boundary_route:$source_report.decision_recording_boundary_scope.boundary_readback_route,
      readback_mode:"operator_approval_acceptance_boundary_readback_only",
      approval_request_boundary:"blocked",
      approval_acceptance_boundary:"blocked",
      approval_recording_boundary:"blocked",
      approval_receipt_boundary:"blocked",
      decision_recording_boundary:"blocked",
      evidence_recording_boundary:"blocked"
    },
    boundary_entry_count:$boundary_entry_count,
    stable_boundary_key_count:$stable_boundary_key_count,
    boundary_route_count:$boundary_route_count,
    boundary_ready_count:$boundary_ready_count,
    source_boundary_attached_count:$source_boundary_attached_count,
    pending_operator_decision_count:$pending_operator_decision_count,
    approval_request_blocked_count:$approval_request_blocked_count,
    approval_acceptance_blocked_count:$approval_acceptance_blocked_count,
    approval_recording_blocked_count:$approval_recording_blocked_count,
    approval_receipt_blocked_count:$approval_receipt_blocked_count,
    decision_recording_blocked_count:$decision_recording_blocked_count,
    evidence_recorded_count:$evidence_recorded_count,
    packet_visible_unsent_unpersisted_count:$packet_visible_unsent_unpersisted_count,
    readback_unpersisted_count:$readback_unpersisted_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    strategy_application_blocked_count:$strategy_application_blocked_count,
    approval_acceptance_boundary_readback_visible:$boundary_ready,
    approval_acceptance_boundary_readback_persisted:false,
    approval_request_sent:false,
    approval_accepted:false,
    approval_recorded:false,
    approval_receipt_persisted:false,
    decision_recorded:false,
    decision_recording_persisted:false,
    decision_receipt_persisted:false,
    decision_checklist_persisted:false,
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
    blocker_waiver_allowed:false,
    package_or_release_allowed:false,
    public_ga_allowed:false,
    canary_activation_allowed:false,
    live_activation_allowed:false,
    live_execution_allowed:false,
    operator_approval_acceptance_boundary_readback_ready:$boundary_ready,
    entries:$entries,
    blockers:[
      "approval_request_blocked",
      "approval_acceptance_blocked",
      "approval_recording_blocked",
      "approval_receipt_persistence_blocked",
      "operator_decision_recording_blocked",
      "operator_decision_recording_persistence_blocked",
      "operator_decision_receipt_persistence_blocked",
      "evidence_recording_blocked",
      "operator_approval_acceptance_boundary_readback_persistence_blocked",
      "git_add_blocked",
      "git_index_mutation_blocked",
      "git_commit_blocked",
      "git_push_blocked",
      "git_reset_blocked",
      "git_checkout_blocked",
      "git_revert_blocked",
      "cleanup_and_delete_blocked",
      "strategy_application_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    recommended_next_gate:"phase22_dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_without_recording",
    next_actions:[
      "phase22_dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_without_recording"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      boundary_readback_persisted:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      approval_receipt_persisted:false,
      decision_recorded:false,
      decision_recording_persisted:false,
      decision_receipt_persisted:false,
      decision_checklist_persisted:false,
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
      blocker_waived:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      canary_activation_started:false,
      live_activation_started:false,
      live_execution_started:false
    }
  }'
