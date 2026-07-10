#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_release_risk_snapshot.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_2026-06-28.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 22 evidence recording boundary readback report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 23 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 23 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 23 release risk snapshot report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render Phase 22 evidence recording boundary readback report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 22 evidence recording boundary readback report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_release_risk_snapshot_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_2026-06-28.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_") | gsub("-"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def release_risk_tier($bucket):
    if $bucket == "cross_lane_or_unowned" then "critical"
    elif ($bucket == "codex-rs" or $bucket == "plugins" or $bucket == "scripts" or $bucket == "hepta_systems_owned") then "high"
    elif ($bucket == "artifacts" or $bucket == "docs") then "medium"
    else "high"
    end;
  def release_risk_reason($bucket):
    if $bucket == "cross_lane_or_unowned" then "cross-lane or unowned changes need owner attribution before release"
    elif $bucket == "codex-rs" then "runtime and crate changes require targeted Rust gates before release"
    elif $bucket == "plugins" then "plugin surface changes can affect runtime/tool contribution boundaries"
    elif $bucket == "scripts" then "automation and gate scripts can change release evidence"
    elif $bucket == "hepta_systems_owned" then "owned Hepta systems changes still need freeze and rehearsal"
    elif $bucket == "artifacts" then "generated or local artifacts need classification before release evidence"
    elif $bucket == "docs" then "architecture and evidence notes affect operator readback but not runtime execution"
    else "dirty worktree bucket requires release-risk review"
    end;
  def release_blocker($bucket):
    if $bucket == "cross_lane_or_unowned" then "cross_lane_or_unowned_changes"
    elif $bucket == "codex-rs" then "runtime_crate_changes"
    elif $bucket == "plugins" then "plugin_surface_changes"
    elif $bucket == "scripts" then "automation_gate_changes"
    elif $bucket == "hepta_systems_owned" then "hepta_systems_owned_changes"
    elif $bucket == "artifacts" then "generated_or_local_artifacts"
    elif $bucket == "docs" then "documentation_evidence_changes"
    else "dirty_worktree_changes"
    end;
  def rehearsal_action($bucket):
    if $bucket == "cross_lane_or_unowned" then "test_only_owner_attribution_and_freeze_rehearsal"
    elif $bucket == "codex-rs" then "test_only_targeted_rust_gate_rehearsal"
    elif $bucket == "plugins" then "test_only_plugin_surface_rehearsal"
    elif $bucket == "scripts" then "test_only_script_gate_rehearsal"
    elif $bucket == "hepta_systems_owned" then "test_only_owned_lane_freeze_rehearsal"
    elif $bucket == "artifacts" then "test_only_artifact_classification_rehearsal"
    elif $bucket == "docs" then "test_only_doc_evidence_rehearsal"
    else "test_only_dirty_worktree_rehearsal"
    end;
  def snapshot_entry:
    . as $entry
    | {
      source_evidence_boundary_key:$entry.evidence_boundary_key,
      source_evidence_boundary_route:$entry.evidence_boundary_route,
      snapshot_key:("dirty_worktree.release_risk_snapshot." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      snapshot_route:("readback://release-boundary/dirty-worktree/release-risk-snapshot/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      owner_hint:$entry.owner_hint,
      review_lane:$entry.review_lane,
      recommended_strategy:$entry.recommended_strategy,
      release_risk_tier:release_risk_tier($entry.source_bucket),
      release_risk_reason:release_risk_reason($entry.source_bucket),
      release_blocker:release_blocker($entry.source_bucket),
      release_blocker_state:"blocked_dirty_worktree",
      clean_worktree_rehearsal_candidate:true,
      rehearsal_action:rehearsal_action($entry.source_bucket),
      decision_state:$entry.decision_state,
      evidence_recording_state:$entry.evidence_recording_state,
      evidence_persistence_state:$entry.evidence_persistence_state,
      evidence_receipt_state:$entry.evidence_receipt_state,
      approval_request_state:$entry.approval_request_state,
      approval_acceptance_state:$entry.approval_acceptance_state,
      approval_recording_state:$entry.approval_recording_state,
      approval_receipt_state:$entry.approval_receipt_state,
      operator_visible:$entry.operator_visible,
      queryable:$entry.queryable,
      diffable:$entry.diffable,
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
  ($source_report.entries | map(snapshot_entry)) as $entries |
  ($entries | length) as $risk_entry_count |
  ($entries | map(.snapshot_key) | unique | length) as $stable_snapshot_key_count |
  ($entries | map(.snapshot_route) | unique | length) as $snapshot_route_count |
  ($entries | map(select(.operator_visible == true and .queryable == true and .diffable == true and .decision_state == "pending_operator_decision" and .release_blocker_state == "blocked_dirty_worktree" and .clean_worktree_rehearsal_candidate == true and .evidence_recording_state == "evidence_recording_blocked" and .evidence_persistence_state == "evidence_persistence_blocked" and .evidence_receipt_state == "evidence_receipt_blocked" and .evidence_recording_allowed == false and .approval_acceptance_allowed == false and .decision_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $snapshot_ready_count |
  ($entries | map(select(.release_risk_tier == "critical")) | length) as $critical_risk_count |
  ($entries | map(select(.release_risk_tier == "high")) | length) as $high_risk_count |
  ($entries | map(select(.release_risk_tier == "medium")) | length) as $medium_risk_count |
  ($entries | map(select(.release_blocker_state == "blocked_dirty_worktree")) | length) as $release_blocked_count |
  ($entries | map(select(.clean_worktree_rehearsal_candidate == true)) | length) as $rehearsal_candidate_count |
  ($entries | map(select(.decision_state == "pending_operator_decision")) | length) as $pending_operator_decision_count |
  ($entries | map(select(.evidence_recording_allowed == false)) | length) as $evidence_recording_blocked_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($source_report.operator_evidence_recording_boundary_readback_ready == true
    and $source_report.evidence_recording_boundary_readback_visible == true
    and $source_report.evidence_recording_boundary_readback_persisted == false
    and $source_report.evidence_recorded == false
    and $source_report.evidence_recording_persisted == false
    and $source_report.evidence_receipt_persisted == false
    and $lib_export_present == true
    and $risk_entry_count == $source_report.boundary_entry_count
    and $stable_snapshot_key_count == $risk_entry_count
    and $snapshot_route_count == $risk_entry_count
    and $snapshot_ready_count == $risk_entry_count
    and $critical_risk_count == 1
    and $high_risk_count == 4
    and $medium_risk_count == 2
    and $release_blocked_count == $risk_entry_count
    and $rehearsal_candidate_count == $risk_entry_count
    and $pending_operator_decision_count == $risk_entry_count
    and $evidence_recording_blocked_count == $risk_entry_count
    and $git_mutation_blocked_count == $risk_entry_count
    and $cleanup_delete_blocked_count == $risk_entry_count) as $snapshot_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_release_risk_snapshot",
    status:(if $snapshot_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_release_risk_snapshot_gate",
    schema_version:"dirty_worktree_release_boundary_release_risk_snapshot_v1",
    plugin_id:$source_report.plugin_id,
    source_evidence_recording_boundary_gate:$source_report.gate,
    source_evidence_recording_boundary_ready:$source_report.operator_evidence_recording_boundary_readback_ready,
    source_evidence_recording_boundary_visible:$source_report.evidence_recording_boundary_readback_visible,
    source_evidence_recording_boundary_persisted:$source_report.evidence_recording_boundary_readback_persisted,
    source_evidence_recorded:$source_report.evidence_recorded,
    source_evidence_recording_persisted:$source_report.evidence_recording_persisted,
    source_evidence_receipt_persisted:$source_report.evidence_receipt_persisted,
    source_boundary_entry_count:$source_report.boundary_entry_count,
    lib_export_present:$lib_export_present,
    inventory_entry_count:$source_report.inventory_entry_count,
    tracked_change_count:$source_report.tracked_change_count,
    untracked_change_count:$source_report.untracked_change_count,
    release_risk_snapshot_scope:{
      snapshot_id:"dirty-worktree.release-boundary.release-risk-snapshot.v1",
      snapshot_route:"readback://release-boundary/dirty-worktree/release-risk-snapshot/v1",
      source_evidence_recording_boundary_route:$source_report.evidence_recording_boundary_scope.boundary_readback_route,
      snapshot_mode:"fast_local_release_risk_snapshot_only",
      release_cutover_boundary:"blocked_dirty_worktree",
      git_mutation_boundary:"blocked",
      cleanup_boundary:"blocked",
      evidence_boundary:"blocked",
      live_boundary:"blocked"
    },
    risk_entry_count:$risk_entry_count,
    stable_snapshot_key_count:$stable_snapshot_key_count,
    snapshot_route_count:$snapshot_route_count,
    snapshot_ready_count:$snapshot_ready_count,
    critical_risk_count:$critical_risk_count,
    high_risk_count:$high_risk_count,
    medium_risk_count:$medium_risk_count,
    high_or_critical_risk_count:($critical_risk_count + $high_risk_count),
    release_blocked_count:$release_blocked_count,
    rehearsal_candidate_count:$rehearsal_candidate_count,
    pending_operator_decision_count:$pending_operator_decision_count,
    evidence_recording_blocked_count:$evidence_recording_blocked_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    risk_snapshot_visible:$snapshot_ready,
    risk_snapshot_persisted:false,
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
    release_risk_snapshot_ready:$snapshot_ready,
    entries:$entries,
    blockers:[
      "dirty_worktree_release_risk_snapshot_visible_only",
      "release_cutover_blocked",
      "git_mutation_blocked",
      "cleanup_and_delete_blocked",
      "evidence_recording_blocked",
      "approval_acceptance_blocked",
      "decision_recording_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    recommended_next_gate:"phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation",
    next_actions:[
      "phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      snapshot_persisted:false,
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
