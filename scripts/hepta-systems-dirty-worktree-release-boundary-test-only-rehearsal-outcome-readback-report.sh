#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 24 test-only rehearsal report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 25 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 25 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 25 test-only rehearsal outcome readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render Phase 24 test-only rehearsal report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 24 test-only rehearsal report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_2026-06-29.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_") | gsub("-"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def outcome_state($bucket):
    if $bucket == "cross_lane_or_unowned" then "blocked_until_owner_attribution"
    elif $bucket == "codex-rs" then "ready_for_targeted_rust_gate_rehearsal"
    elif $bucket == "plugins" then "ready_for_plugin_surface_gate_rehearsal"
    elif $bucket == "scripts" then "ready_for_script_syntax_gate_rehearsal"
    elif $bucket == "hepta_systems_owned" then "ready_for_owned_lane_freeze_rehearsal"
    elif $bucket == "artifacts" then "ready_for_artifact_classification_rehearsal"
    elif $bucket == "docs" then "ready_for_doc_evidence_consistency_rehearsal"
    else "unknown"
    end;
  def outcome_action($bucket):
    if $bucket == "cross_lane_or_unowned" then "attribute_owner_before_any_clean_worktree_action"
    elif $bucket == "codex-rs" then "run_targeted_rust_gate_probe_later_without_git_mutation"
    elif $bucket == "plugins" then "run_plugin_surface_gate_probe_later_without_git_mutation"
    elif $bucket == "scripts" then "run_script_syntax_gate_probe_later_without_git_mutation"
    elif $bucket == "hepta_systems_owned" then "freeze_owned_lane_changes_later_without_git_mutation"
    elif $bucket == "artifacts" then "classify_artifacts_later_without_delete"
    elif $bucket == "docs" then "check_doc_evidence_consistency_later_without_persistence"
    else "review_dirty_worktree_bucket_later_without_git_mutation"
    end;
  def outcome_entry:
    . as $entry
    | {
      source_rehearsal_key:$entry.rehearsal_key,
      source_rehearsal_route:$entry.rehearsal_route,
      outcome_key:("dirty_worktree.test_only_rehearsal_outcome_readback." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      outcome_route:("readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      owner_hint:$entry.owner_hint,
      review_lane:$entry.review_lane,
      recommended_strategy:$entry.recommended_strategy,
      source_release_risk_tier:$entry.source_release_risk_tier,
      source_release_blocker:$entry.source_release_blocker,
      source_release_blocker_state:$entry.source_release_blocker_state,
      source_required_local_gate:$entry.required_local_gate,
      source_rehearsal_probe:$entry.rehearsal_probe,
      source_convergence_state:$entry.convergence_state,
      outcome_state:outcome_state($entry.source_bucket),
      outcome_action:outcome_action($entry.source_bucket),
      operator_action:"review_outcome_readback_before_any_probe_or_git_mutation",
      decision_state:$entry.decision_state,
      evidence_recording_state:$entry.evidence_recording_state,
      evidence_persistence_state:$entry.evidence_persistence_state,
      evidence_receipt_state:$entry.evidence_receipt_state,
      approval_request_state:$entry.approval_request_state,
      approval_acceptance_state:$entry.approval_acceptance_state,
      approval_recording_state:$entry.approval_recording_state,
      approval_receipt_state:$entry.approval_receipt_state,
      source_rehearsal_attached:(($entry.rehearsal_key | length) > 0 and ($entry.rehearsal_route | length) > 0),
      outcome_readback_visible:true,
      outcome_readback_persisted:false,
      operator_visible:$entry.operator_visible,
      queryable:$entry.queryable,
      diffable:$entry.diffable,
      test_probe_executed:false,
      mutation_free:true,
      evidence_recording_allowed:false,
      approval_acceptance_allowed:false,
      decision_recording_allowed:false,
      git_add_blocked:$entry.git_add_blocked,
      git_index_mutation_blocked:$entry.git_index_mutation_blocked,
      git_commit_blocked:$entry.git_commit_blocked,
      git_push_blocked:$entry.git_push_blocked,
      git_reset_blocked:$entry.git_reset_blocked,
      git_checkout_blocked:$entry.git_checkout_blocked,
      git_revert_blocked:$entry.git_revert_blocked,
      cleanup_blocked:$entry.cleanup_blocked,
      delete_blocked:$entry.delete_blocked,
      release_cutover_allowed:false,
      canary_activation_allowed:false,
      live_execution_allowed:false
    };
  ($source[0]) as $source_report |
  ($source_report.entries | map(outcome_entry)) as $entries |
  ($entries | length) as $outcome_entry_count |
  ($entries | map(.outcome_key) | unique | length) as $stable_outcome_key_count |
  ($entries | map(.outcome_route) | unique | length) as $outcome_route_count |
  ($entries | map(select(.source_rehearsal_attached == true and .outcome_readback_visible == true and .outcome_readback_persisted == false and .operator_visible == true and .queryable == true and .diffable == true and .test_probe_executed == false and .mutation_free == true and .source_release_blocker_state == "blocked_dirty_worktree" and (.source_required_local_gate | length) > 0 and .source_convergence_state != "unknown" and .outcome_state != "unknown" and .decision_state == "pending_operator_decision" and .evidence_recording_state == "evidence_recording_blocked" and .evidence_persistence_state == "evidence_persistence_blocked" and .evidence_receipt_state == "evidence_receipt_blocked" and .evidence_recording_allowed == false and .approval_acceptance_allowed == false and .decision_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $outcome_ready_count |
  ($entries | map(select(.outcome_state == "blocked_until_owner_attribution")) | length) as $blocked_until_owner_attribution_count |
  ($entries | map(select(.outcome_state == "ready_for_targeted_rust_gate_rehearsal")) | length) as $ready_for_targeted_rust_gate_rehearsal_count |
  ($entries | map(select(.outcome_state == "ready_for_plugin_surface_gate_rehearsal")) | length) as $ready_for_plugin_surface_gate_rehearsal_count |
  ($entries | map(select(.outcome_state == "ready_for_script_syntax_gate_rehearsal")) | length) as $ready_for_script_syntax_gate_rehearsal_count |
  ($entries | map(select(.outcome_state == "ready_for_owned_lane_freeze_rehearsal")) | length) as $ready_for_owned_lane_freeze_rehearsal_count |
  ($entries | map(select(.outcome_state == "ready_for_artifact_classification_rehearsal")) | length) as $ready_for_artifact_classification_rehearsal_count |
  ($entries | map(select(.outcome_state == "ready_for_doc_evidence_consistency_rehearsal")) | length) as $ready_for_doc_evidence_consistency_rehearsal_count |
  ($entries | map(select(.source_release_blocker_state == "blocked_dirty_worktree")) | length) as $release_blocked_count |
  ($entries | map(select(.test_probe_executed == false)) | length) as $test_probe_execution_blocked_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.evidence_recording_allowed == false)) | length) as $evidence_recording_blocked_count |
  ($entries | map(select(.approval_acceptance_allowed == false)) | length) as $approval_acceptance_blocked_count |
  ($entries | map(select(.decision_recording_allowed == false)) | length) as $decision_recording_blocked_count |
  ($source_report.test_only_clean_worktree_strategy_rehearsal_ready == true
    and $source_report.test_only_rehearsal_visible == true
    and $source_report.test_only_rehearsal_persisted == false
    and $source_report.test_probe_executed == false
    and $source_report.evidence_recorded == false
    and $source_report.evidence_recording_persisted == false
    and $source_report.evidence_receipt_persisted == false
    and $lib_export_present == true
    and $outcome_entry_count == $source_report.rehearsal_entry_count
    and $stable_outcome_key_count == $outcome_entry_count
    and $outcome_route_count == $outcome_entry_count
    and $outcome_ready_count == $outcome_entry_count
    and $blocked_until_owner_attribution_count == 1
    and $ready_for_targeted_rust_gate_rehearsal_count == 1
    and $ready_for_plugin_surface_gate_rehearsal_count == 1
    and $ready_for_script_syntax_gate_rehearsal_count == 1
    and $ready_for_owned_lane_freeze_rehearsal_count == 1
    and $ready_for_artifact_classification_rehearsal_count == 1
    and $ready_for_doc_evidence_consistency_rehearsal_count == 1
    and $release_blocked_count == $outcome_entry_count
    and $test_probe_execution_blocked_count == $outcome_entry_count
    and $git_mutation_blocked_count == $outcome_entry_count
    and $cleanup_delete_blocked_count == $outcome_entry_count
    and $evidence_recording_blocked_count == $outcome_entry_count
    and $approval_acceptance_blocked_count == $outcome_entry_count
    and $decision_recording_blocked_count == $outcome_entry_count) as $outcome_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback",
    status:(if $outcome_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_gate",
    schema_version:"dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_v1",
    plugin_id:$source_report.plugin_id,
    source_rehearsal_gate:$source_report.gate,
    source_rehearsal_ready:$source_report.test_only_clean_worktree_strategy_rehearsal_ready,
    source_test_only_rehearsal_visible:$source_report.test_only_rehearsal_visible,
    source_test_only_rehearsal_persisted:$source_report.test_only_rehearsal_persisted,
    source_test_probe_executed:$source_report.test_probe_executed,
    source_rehearsal_entry_count:$source_report.rehearsal_entry_count,
    source_rehearsal_ready_count:$source_report.rehearsal_ready_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$source_report.inventory_entry_count,
    tracked_change_count:$source_report.tracked_change_count,
    untracked_change_count:$source_report.untracked_change_count,
    outcome_scope:{
      outcome_readback_id:"dirty-worktree.release-boundary.test-only-rehearsal-outcome-readback.v1",
      outcome_readback_route:"readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/v1",
      source_rehearsal_route:$source_report.rehearsal_scope.rehearsal_route,
      outcome_mode:"visible_only_no_probe_no_git_mutation_no_cleanup_no_evidence_recording",
      test_probe_boundary:"blocked",
      git_mutation_boundary:"blocked",
      cleanup_boundary:"blocked",
      evidence_boundary:"blocked",
      approval_boundary:"blocked",
      decision_boundary:"blocked",
      live_boundary:"blocked"
    },
    outcome_entry_count:$outcome_entry_count,
    stable_outcome_key_count:$stable_outcome_key_count,
    outcome_route_count:$outcome_route_count,
    outcome_ready_count:$outcome_ready_count,
    blocked_until_owner_attribution_count:$blocked_until_owner_attribution_count,
    ready_for_targeted_rust_gate_rehearsal_count:$ready_for_targeted_rust_gate_rehearsal_count,
    ready_for_plugin_surface_gate_rehearsal_count:$ready_for_plugin_surface_gate_rehearsal_count,
    ready_for_script_syntax_gate_rehearsal_count:$ready_for_script_syntax_gate_rehearsal_count,
    ready_for_owned_lane_freeze_rehearsal_count:$ready_for_owned_lane_freeze_rehearsal_count,
    ready_for_artifact_classification_rehearsal_count:$ready_for_artifact_classification_rehearsal_count,
    ready_for_doc_evidence_consistency_rehearsal_count:$ready_for_doc_evidence_consistency_rehearsal_count,
    release_blocked_count:$release_blocked_count,
    test_probe_execution_blocked_count:$test_probe_execution_blocked_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    evidence_recording_blocked_count:$evidence_recording_blocked_count,
    approval_acceptance_blocked_count:$approval_acceptance_blocked_count,
    decision_recording_blocked_count:$decision_recording_blocked_count,
    outcome_readback_visible:$outcome_ready,
    outcome_readback_persisted:false,
    test_probe_executed:false,
    evidence_recorded:false,
    evidence_recording_persisted:false,
    evidence_receipt_persisted:false,
    approval_request_sent:false,
    approval_accepted:false,
    approval_recorded:false,
    approval_receipt_persisted:false,
    decision_recorded:false,
    decision_recording_persisted:false,
    decision_receipt_persisted:false,
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
    blocker_waiver_allowed:false,
    package_or_release_allowed:false,
    public_ga_allowed:false,
    canary_activation_allowed:false,
    live_activation_allowed:false,
    live_execution_allowed:false,
    test_only_rehearsal_outcome_readback_ready:$outcome_ready,
    entries:$entries,
    blockers:[
      "outcome_readback_visible_only",
      "test_probe_execution_still_blocked",
      "release_cutover_blocked",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "approval_acceptance_blocked",
      "decision_recording_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "temporal_lite_append_only_event_store_feature_gated_test_implementation"
    ],
    recommended_next_gate:"temporal_lite_append_only_event_store_feature_gated_test_implementation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      outcome_readback_persisted:false,
      test_probe_executed:false,
      evidence_recorded:false,
      evidence_persisted:false,
      evidence_receipt_persisted:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      approval_receipt_persisted:false,
      decision_recorded:false,
      decision_recording_persisted:false,
      decision_receipt_persisted:false,
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
      blocker_waived:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      canary_activation_started:false,
      live_activation_started:false,
      live_execution_started:false
    }
  }'
