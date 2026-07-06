#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-git-mutation-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_2026-06-30.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-decision-checklist-without-git-mutation-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification operator packet git-mutation boundary report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing owner/freeze/classification operator decision checklist Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification operator decision checklist architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the owner/freeze/classification operator decision checklist report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render owner/freeze/classification operator packet git-mutation boundary report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by owner/freeze/classification operator packet git-mutation boundary report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-decision-checklist-without-git-mutation-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_DECISION_CHECKLIST_2026-06-30.md" \
  '
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_") | gsub("-"; "_"));
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def operator_action($category):
    if $category == "artifact_classification_outcome_required" then "classify_artifacts_without_cleanup_or_git_mutation"
    elif $category == "owner_attribution_outcome_required" then "confirm_owner_route_without_owner_assignment_persistence"
    elif $category == "owned_lane_freeze_outcome_required" then "confirm_owned_lane_freeze_without_freeze_application"
    elif $category == "targeted_gate_outcome_required" then "review_targeted_gate_without_test_probe_execution"
    else "keep_operator_decision_pending_without_mutation"
    end;
  def evidence_requirement($category):
    if $category == "artifact_classification_outcome_required" then "artifact_classification_evidence_required_but_not_recorded"
    elif $category == "owner_attribution_outcome_required" then "owner_attribution_evidence_required_but_not_recorded"
    elif $category == "owned_lane_freeze_outcome_required" then "owned_lane_freeze_evidence_required_but_not_recorded"
    elif $category == "targeted_gate_outcome_required" then "targeted_gate_evidence_required_but_not_recorded"
    else "operator_decision_evidence_required_but_not_recorded"
    end;
  def checklist_entry:
    . as $entry
    | {
      source_git_boundary_readback_key:$entry.git_boundary_readback_key,
      source_git_boundary_readback_route:$entry.git_boundary_readback_route,
      checklist_key:("dirty_worktree.owner_freeze_classification_operator_decision_checklist." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      checklist_route:("checklist://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      decision_checkpoint:("operator_decision_checkpoint.owner_freeze_classification." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      owner_route:$entry.owner_route,
      outcome_category:$entry.outcome_category,
      packet_section:$entry.packet_section,
      required_local_gate:$entry.required_local_gate,
      operator_action:operator_action($entry.outcome_category),
      evidence_requirement:evidence_requirement($entry.outcome_category),
      decision_state:"pending_operator_decision",
      checklist_state:"ready_blocked_pending_operator_decision",
      packet_visible:$entry.packet_visible,
      packet_unsent:$entry.packet_unsent,
      packet_unpersisted:$entry.packet_unpersisted,
      readback_unpersisted:$entry.readback_unpersisted,
      operator_visible:$entry.operator_visible,
      queryable:$entry.queryable,
      diffable:$entry.diffable,
      operator_decision_required:$entry.operator_decision_required,
      decision_recording_allowed:false,
      approval_request_blocked:$entry.approval_request_blocked,
      approval_acceptance_allowed:false,
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
      owner_assignment_blocked:$entry.owner_assignment_blocked,
      freeze_application_blocked:$entry.freeze_application_blocked,
      classification_persistence_blocked:$entry.classification_persistence_blocked,
      test_probe_blocked:$entry.test_probe_blocked,
      packet_send_blocked:$entry.packet_send_blocked,
      packet_persistence_blocked:$entry.packet_persistence_blocked,
      readback_persistence_blocked:$entry.readback_persistence_blocked,
      release_cutover_allowed:false,
      canary_activation_allowed:false,
      live_execution_allowed:false
    };
  ($source[0]) as $source_report |
  ($source_report.entries | map(checklist_entry)) as $entries |
  ($entries | length) as $checklist_entry_count |
  ($entries | map(.checklist_key) | unique | length) as $stable_checklist_key_count |
  ($entries | map(.checklist_route) | unique | length) as $checklist_route_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true and .operator_visible == true and .queryable == true and .diffable == true and .operator_decision_required == true and .decision_recording_allowed == false and .approval_request_blocked == true and .approval_acceptance_allowed == false and .evidence_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .owner_assignment_blocked == true and .freeze_application_blocked == true and .classification_persistence_blocked == true and .test_probe_blocked == true and .packet_send_blocked == true and .packet_persistence_blocked == true and .readback_persistence_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $checklist_ready_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_unpersisted == true)) | length) as $packet_visible_unsent_unpersisted_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.owner_assignment_blocked == true)) | length) as $owner_assignment_blocked_count |
  ($entries | map(select(.freeze_application_blocked == true)) | length) as $freeze_application_blocked_count |
  ($entries | map(select(.classification_persistence_blocked == true)) | length) as $classification_persistence_blocked_count |
  ($entries | map(select(.test_probe_blocked == true)) | length) as $test_probe_blocked_count |
  ($entries | map(select(.operator_decision_required == true)) | length) as $operator_decision_required_count |
  ($entries | map(select(.decision_state == "pending_operator_decision")) | length) as $pending_operator_decision_count |
  ($entries | map(select((.evidence_requirement | length) > 0)) | length) as $evidence_requirement_count |
  ($entries | map(select(.approval_request_blocked == true)) | length) as $approval_request_blocked_count |
  ($entries | map(select(.approval_acceptance_allowed == false)) | length) as $approval_acceptance_blocked_count |
  ($entries | map(select(.decision_recording_allowed == false)) | length) as $decision_recording_blocked_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($source_report.git_mutation_boundary_readback_ready == true
    and $source_report.operator_packet_visible == true
    and $source_report.operator_packet_sent == false
    and $source_report.operator_packet_persisted == false
    and $source_report.packet_payload_persisted == false
    and $source_report.readback_persisted == false
    and $source_report.git_index_mutated == false
    and $source_report.cleanup_allowed == false
    and $source_report.delete_allowed == false
    and $source_report.side_effect_free == true
    and ($source_report.side_effects | to_entries | all(.value == false))
    and $lib_export_present == true
    and $checklist_entry_count == $source_report.readback_entry_count
    and $stable_checklist_key_count == $checklist_entry_count
    and $checklist_route_count == $checklist_entry_count
    and $checklist_ready_count == $checklist_entry_count
    and $packet_visible_unsent_unpersisted_count == $checklist_entry_count
    and $git_mutation_blocked_count == $checklist_entry_count
    and $cleanup_delete_blocked_count == $checklist_entry_count
    and $owner_assignment_blocked_count == $checklist_entry_count
    and $freeze_application_blocked_count == $checklist_entry_count
    and $classification_persistence_blocked_count == $checklist_entry_count
    and $test_probe_blocked_count == $checklist_entry_count
    and $operator_decision_required_count == $checklist_entry_count
    and $pending_operator_decision_count == $checklist_entry_count
    and $evidence_requirement_count == $checklist_entry_count
    and $approval_request_blocked_count == $checklist_entry_count
    and $approval_acceptance_blocked_count == $checklist_entry_count
    and $decision_recording_blocked_count == $checklist_entry_count
    and $evidence_recorded_count == 0) as $ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation_gate",
    schema_version:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation_v1",
    plugin_id:$source_report.plugin_id,
    source_git_boundary_readback_gate:$source_report.gate,
    source_git_boundary_readback_ready:$source_report.git_mutation_boundary_readback_ready,
    source_operator_packet_visible:$source_report.operator_packet_visible,
    source_operator_packet_sent:$source_report.operator_packet_sent,
    source_operator_packet_persisted:$source_report.operator_packet_persisted,
    source_packet_payload_persisted:$source_report.packet_payload_persisted,
    source_readback_persisted:$source_report.readback_persisted,
    source_git_index_mutated:$source_report.git_index_mutated,
    source_cleanup_allowed:$source_report.cleanup_allowed,
    source_delete_allowed:$source_report.delete_allowed,
    source_readback_entry_count:$source_report.readback_entry_count,
    source_tracked_change_count:$source_report.source_tracked_change_count,
    source_untracked_change_count:$source_report.source_untracked_change_count,
    lib_export_present:$lib_export_present,
    checklist_scope:{
      checklist_id:"dirty-worktree.release-boundary.owner-freeze-classification.operator-decision-checklist.v1",
      checklist_route:"checklist://release-boundary/dirty-worktree/owner-freeze-classification/operator-decision/v1",
      source_git_boundary_readback_route:$source_report.readback_scope.readback_route,
      checklist_mode:"operator_decision_checklist_only",
      decision_recording_boundary:"blocked",
      owner_assignment_boundary:"blocked",
      freeze_application_boundary:"blocked",
      classification_persistence_boundary:"blocked",
      test_probe_boundary:"blocked",
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
    owner_assignment_blocked_count:$owner_assignment_blocked_count,
    freeze_application_blocked_count:$freeze_application_blocked_count,
    classification_persistence_blocked_count:$classification_persistence_blocked_count,
    test_probe_blocked_count:$test_probe_blocked_count,
    operator_decision_required_count:$operator_decision_required_count,
    pending_operator_decision_count:$pending_operator_decision_count,
    evidence_requirement_count:$evidence_requirement_count,
    approval_request_blocked_count:$approval_request_blocked_count,
    approval_acceptance_blocked_count:$approval_acceptance_blocked_count,
    decision_recording_blocked_count:$decision_recording_blocked_count,
    evidence_recorded_count:$evidence_recorded_count,
    decision_checklist_visible:$ready,
    decision_checklist_persisted:false,
    decision_recorded:false,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    packet_payload_persisted:false,
    readback_persisted:false,
    owner_assignment_persisted:false,
    freeze_applied:false,
    classification_persisted:false,
    test_probe_executed:false,
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
    operator_decision_checklist_ready:$ready,
    entries:$entries,
    blockers:[
      "operator_decision_recording_blocked",
      "approval_request_blocked",
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
      "owner_assignment_persistence_blocked",
      "freeze_application_blocked",
      "classification_persistence_blocked",
      "test_probe_execution_blocked",
      "operator_packet_send_blocked",
      "operator_packet_persistence_blocked",
      "readback_persistence_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
      "keep_owner_freeze_classification_decision_checklist_pending_without_recording_or_git_mutation"
    ],
    recommended_next_gate:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      decision_checklist_persisted:false,
      decision_recorded:false,
      packet_sent:false,
      packet_persisted:false,
      packet_payload_persisted:false,
      readback_persisted:false,
      owner_assignment_persisted:false,
      freeze_applied:false,
      classification_persisted:false,
      test_probe_executed:false,
      git_add_performed:false,
      git_index_mutated:false,
      git_commit_created:false,
      git_push_performed:false,
      git_reset_performed:false,
      git_checkout_performed:false,
      git_revert_performed:false,
      cleanup_performed:false,
      unrelated_file_deleted:false,
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
