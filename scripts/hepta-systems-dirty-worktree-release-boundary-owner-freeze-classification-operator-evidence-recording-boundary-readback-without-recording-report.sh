#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-approval-acceptance-boundary-readback-without-acceptance-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification approval acceptance boundary report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing owner/freeze/classification evidence recording Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification evidence recording architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the owner/freeze/classification evidence recording boundary readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render owner/freeze/classification approval acceptance boundary report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by owner/freeze/classification approval acceptance boundary report"

lib_export_present=false
if grep -q 'dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_2026-06-30.md" \
  '
  def route_prefix($group_type):
    if $group_type == "top_level" then "top-level"
    elif $group_type == "scope" then "scope"
    else $group_type
    end;
  def key_safe($value): ($value | gsub("[^A-Za-z0-9._-]"; "_") | gsub("-"; "_"));
  def route_safe($value): ($value | gsub("_"; "-") | gsub("[^A-Za-z0-9.-]"; "-"));
  def evidence_entry:
    . as $entry
    | {
      source_approval_boundary_key:$entry.approval_boundary_key,
      source_approval_boundary_route:$entry.approval_boundary_route,
      source_packet_key:$entry.source_packet_key,
      source_packet_route:$entry.source_packet_route,
      source_packet_readback_key:$entry.source_packet_readback_key,
      source_packet_readback_route:$entry.source_packet_readback_route,
      evidence_boundary_key:("dirty_worktree.owner_freeze_classification_operator_evidence_recording_boundary." + key_safe($entry.group_type) + "." + key_safe($entry.source_bucket)),
      evidence_boundary_route:("readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-evidence-recording-boundary/" + route_prefix($entry.group_type) + "/" + route_safe($entry.source_bucket)),
      evidence_checkpoint:("evidence_recording_boundary." + $entry.approval_checkpoint),
      approval_checkpoint:$entry.approval_checkpoint,
      decision_checkpoint:$entry.decision_checkpoint,
      group_type:$entry.group_type,
      source_bucket:$entry.source_bucket,
      source_entry_count:$entry.source_entry_count,
      tracked_count:$entry.tracked_count,
      untracked_count:$entry.untracked_count,
      owner_route:$entry.owner_route,
      outcome_category:$entry.outcome_category,
      packet_section:$entry.packet_section,
      required_local_gate:$entry.required_local_gate,
      operator_action:$entry.operator_action,
      evidence_requirement:$entry.evidence_requirement,
      decision_state:$entry.decision_state,
      evidence_recording_state:"evidence_recording_blocked",
      evidence_persistence_state:"evidence_persistence_blocked",
      evidence_receipt_state:"evidence_receipt_blocked",
      approval_request_state:$entry.approval_request_state,
      approval_acceptance_state:$entry.approval_acceptance_state,
      approval_recording_state:$entry.approval_recording_state,
      approval_receipt_state:$entry.approval_receipt_state,
      decision_recording_state:$entry.decision_recording_state,
      decision_persistence_state:$entry.decision_persistence_state,
      source_boundary_state:"approval_acceptance_boundary_visible_unpersisted",
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
      evidence_recording_allowed:false,
      evidence_persistence_allowed:false,
      evidence_receipt_persistence_allowed:false,
      approval_request_allowed:false,
      approval_acceptance_allowed:false,
      approval_recording_allowed:false,
      approval_receipt_persistence_allowed:false,
      decision_recording_allowed:false,
      decision_persistence_allowed:false,
      decision_receipt_persistence_allowed:false,
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
  ($source_report.entries | map(evidence_entry)) as $entries |
  ($entries | length) as $boundary_entry_count |
  ($entries | map(.evidence_boundary_key) | unique | length) as $stable_boundary_key_count |
  ($entries | map(.evidence_boundary_route) | unique | length) as $boundary_route_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true and .readback_visible == true and .readback_unpersisted == true and .operator_visible == true and .queryable == true and .diffable == true and .operator_decision_required == true and .decision_state == "pending_operator_decision" and .evidence_recording_state == "evidence_recording_blocked" and .evidence_persistence_state == "evidence_persistence_blocked" and .evidence_receipt_state == "evidence_receipt_blocked" and .approval_request_state == "approval_request_blocked" and .approval_acceptance_state == "approval_acceptance_blocked" and .approval_recording_state == "approval_recording_blocked" and .approval_receipt_state == "approval_receipt_blocked" and .decision_recording_state == "decision_recording_blocked" and .decision_persistence_state == "decision_persistence_blocked" and .source_boundary_state == "approval_acceptance_boundary_visible_unpersisted" and .evidence_recording_allowed == false and .evidence_persistence_allowed == false and .evidence_receipt_persistence_allowed == false and .approval_request_allowed == false and .approval_acceptance_allowed == false and .approval_recording_allowed == false and .approval_receipt_persistence_allowed == false and .decision_recording_allowed == false and .decision_persistence_allowed == false and .decision_receipt_persistence_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .owner_assignment_blocked == true and .freeze_application_blocked == true and .classification_persistence_blocked == true and .test_probe_blocked == true and .packet_send_blocked == true and .packet_persistence_blocked == true and .readback_persistence_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false)) | length) as $boundary_ready_count |
  ($entries | map(select((.source_approval_boundary_key | length) > 0 and (.source_approval_boundary_route | length) > 0 and (.source_packet_key | length) > 0 and (.source_packet_route | length) > 0 and (.source_packet_readback_key | length) > 0 and (.source_packet_readback_route | length) > 0)) | length) as $source_boundary_attached_count |
  ($entries | map(select(.decision_state == "pending_operator_decision")) | length) as $pending_operator_decision_count |
  ($entries | map(select(.evidence_recording_allowed == false)) | length) as $evidence_recording_blocked_count |
  ($entries | map(select(.evidence_persistence_allowed == false)) | length) as $evidence_persistence_blocked_count |
  ($entries | map(select(.evidence_receipt_persistence_allowed == false)) | length) as $evidence_receipt_blocked_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.approval_request_allowed == false)) | length) as $approval_request_blocked_count |
  ($entries | map(select(.approval_acceptance_allowed == false)) | length) as $approval_acceptance_blocked_count |
  ($entries | map(select(.approval_recording_allowed == false)) | length) as $approval_recording_blocked_count |
  ($entries | map(select(.approval_receipt_persistence_allowed == false)) | length) as $approval_receipt_blocked_count |
  ($entries | map(select(.decision_recording_allowed == false)) | length) as $decision_recording_blocked_count |
  ($entries | map(select(.packet_visible == true and .packet_unsent == true and .packet_unpersisted == true)) | length) as $packet_visible_unsent_unpersisted_count |
  ($entries | map(select(.readback_visible == true and .readback_unpersisted == true)) | length) as $readback_unpersisted_count |
  ($entries | map(select(.git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true)) | length) as $git_mutation_blocked_count |
  ($entries | map(select(.cleanup_blocked == true and .delete_blocked == true)) | length) as $cleanup_delete_blocked_count |
  ($entries | map(select(.owner_assignment_blocked == true)) | length) as $owner_assignment_blocked_count |
  ($entries | map(select(.freeze_application_blocked == true)) | length) as $freeze_application_blocked_count |
  ($entries | map(select(.classification_persistence_blocked == true)) | length) as $classification_persistence_blocked_count |
  ($entries | map(select(.test_probe_blocked == true)) | length) as $test_probe_blocked_count |
  ($entries | map(select(.packet_send_blocked == true)) | length) as $packet_send_blocked_count |
  ($entries | map(select(.packet_persistence_blocked == true)) | length) as $packet_persistence_blocked_count |
  ($entries | map(select(.readback_persistence_blocked == true)) | length) as $readback_persistence_blocked_count |
  ($source_report.operator_approval_acceptance_boundary_readback_ready == true
    and $source_report.approval_acceptance_boundary_readback_visible == true
    and $source_report.approval_acceptance_boundary_readback_persisted == false
    and $source_report.approval_request_sent == false
    and $source_report.approval_accepted == false
    and $source_report.approval_recorded == false
    and $source_report.approval_receipt_persisted == false
    and $source_report.decision_recorded == false
    and $source_report.decision_recording_persisted == false
    and $source_report.decision_receipt_persisted == false
    and $source_report.operator_packet_sent == false
    and $source_report.operator_packet_persisted == false
    and $source_report.packet_payload_persisted == false
    and $source_report.readback_persisted == false
    and $source_report.owner_assignment_persisted == false
    and $source_report.freeze_applied == false
    and $source_report.classification_persisted == false
    and $source_report.test_probe_executed == false
    and $lib_export_present == true
    and $boundary_entry_count == $source_report.boundary_entry_count
    and $stable_boundary_key_count == $boundary_entry_count
    and $boundary_route_count == $boundary_entry_count
    and $boundary_ready_count == $boundary_entry_count
    and $source_boundary_attached_count == $boundary_entry_count
    and $pending_operator_decision_count == $boundary_entry_count
    and $evidence_recording_blocked_count == $boundary_entry_count
    and $evidence_persistence_blocked_count == $boundary_entry_count
    and $evidence_receipt_blocked_count == $boundary_entry_count
    and $evidence_recorded_count == 0
    and $approval_request_blocked_count == $boundary_entry_count
    and $approval_acceptance_blocked_count == $boundary_entry_count
    and $approval_recording_blocked_count == $boundary_entry_count
    and $approval_receipt_blocked_count == $boundary_entry_count
    and $decision_recording_blocked_count == $boundary_entry_count
    and $packet_visible_unsent_unpersisted_count == $boundary_entry_count
    and $readback_unpersisted_count == $boundary_entry_count
    and $git_mutation_blocked_count == $boundary_entry_count
    and $cleanup_delete_blocked_count == $boundary_entry_count
    and $owner_assignment_blocked_count == $boundary_entry_count
    and $freeze_application_blocked_count == $boundary_entry_count
    and $classification_persistence_blocked_count == $boundary_entry_count
    and $test_probe_blocked_count == $boundary_entry_count
    and $packet_send_blocked_count == $boundary_entry_count
    and $packet_persistence_blocked_count == $boundary_entry_count
    and $readback_persistence_blocked_count == $boundary_entry_count) as $boundary_ready |
  {
    runtime:"hepta",
    surface:"dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording",
    status:(if $boundary_ready then "ready_blocked" else "blocked" end),
    gate:"dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_gate",
    schema_version:"dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_v1",
    plugin_id:$source_report.plugin_id,
    source_approval_acceptance_boundary_gate:$source_report.gate,
    source_approval_acceptance_boundary_ready:$source_report.operator_approval_acceptance_boundary_readback_ready,
    source_approval_acceptance_boundary_visible:$source_report.approval_acceptance_boundary_readback_visible,
    source_approval_acceptance_boundary_persisted:$source_report.approval_acceptance_boundary_readback_persisted,
    source_approval_request_sent:$source_report.approval_request_sent,
    source_approval_accepted:$source_report.approval_accepted,
    source_approval_recorded:$source_report.approval_recorded,
    source_approval_receipt_persisted:$source_report.approval_receipt_persisted,
    source_decision_recorded:$source_report.decision_recorded,
    source_decision_recording_persisted:$source_report.decision_recording_persisted,
    source_decision_receipt_persisted:$source_report.decision_receipt_persisted,
    source_operator_packet_sent:$source_report.operator_packet_sent,
    source_operator_packet_persisted:$source_report.operator_packet_persisted,
    source_packet_payload_persisted:$source_report.packet_payload_persisted,
    source_readback_persisted:$source_report.readback_persisted,
    source_owner_assignment_persisted:$source_report.owner_assignment_persisted,
    source_freeze_applied:$source_report.freeze_applied,
    source_classification_persisted:$source_report.classification_persisted,
    source_test_probe_executed:$source_report.test_probe_executed,
    source_boundary_entry_count:$source_report.boundary_entry_count,
    source_tracked_change_count:$source_report.source_tracked_change_count,
    source_untracked_change_count:$source_report.source_untracked_change_count,
    lib_export_present:$lib_export_present,
    evidence_recording_boundary_scope:{
      boundary_readback_id:"dirty-worktree.release-boundary.owner-freeze-classification.operator-evidence-recording-boundary-readback.v1",
      boundary_readback_route:"readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-evidence-recording-boundary/v1",
      source_approval_acceptance_boundary_route:$source_report.approval_acceptance_boundary_scope.boundary_readback_route,
      readback_mode:"operator_evidence_recording_boundary_readback_only",
      evidence_recording_boundary:"blocked",
      evidence_persistence_boundary:"blocked",
      evidence_receipt_boundary:"blocked",
      approval_acceptance_boundary:"blocked",
      decision_recording_boundary:"blocked",
      owner_assignment_boundary:"blocked",
      freeze_application_boundary:"blocked",
      classification_persistence_boundary:"blocked",
      test_probe_boundary:"blocked",
      git_mutation_boundary:"blocked",
      cleanup_boundary:"blocked"
    },
    boundary_entry_count:$boundary_entry_count,
    stable_boundary_key_count:$stable_boundary_key_count,
    boundary_route_count:$boundary_route_count,
    boundary_ready_count:$boundary_ready_count,
    source_boundary_attached_count:$source_boundary_attached_count,
    pending_operator_decision_count:$pending_operator_decision_count,
    evidence_recording_blocked_count:$evidence_recording_blocked_count,
    evidence_persistence_blocked_count:$evidence_persistence_blocked_count,
    evidence_receipt_blocked_count:$evidence_receipt_blocked_count,
    evidence_recorded_count:$evidence_recorded_count,
    approval_request_blocked_count:$approval_request_blocked_count,
    approval_acceptance_blocked_count:$approval_acceptance_blocked_count,
    approval_recording_blocked_count:$approval_recording_blocked_count,
    approval_receipt_blocked_count:$approval_receipt_blocked_count,
    decision_recording_blocked_count:$decision_recording_blocked_count,
    packet_visible_unsent_unpersisted_count:$packet_visible_unsent_unpersisted_count,
    readback_unpersisted_count:$readback_unpersisted_count,
    git_mutation_blocked_count:$git_mutation_blocked_count,
    cleanup_delete_blocked_count:$cleanup_delete_blocked_count,
    owner_assignment_blocked_count:$owner_assignment_blocked_count,
    freeze_application_blocked_count:$freeze_application_blocked_count,
    classification_persistence_blocked_count:$classification_persistence_blocked_count,
    test_probe_blocked_count:$test_probe_blocked_count,
    packet_send_blocked_count:$packet_send_blocked_count,
    packet_persistence_blocked_count:$packet_persistence_blocked_count,
    readback_persistence_blocked_count:$readback_persistence_blocked_count,
    evidence_recording_boundary_readback_visible:$boundary_ready,
    evidence_recording_boundary_readback_persisted:false,
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
    decision_checklist_persisted:false,
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
    blocker_waiver_allowed:false,
    package_or_release_allowed:false,
    public_ga_allowed:false,
    canary_activation_allowed:false,
    live_activation_allowed:false,
    live_execution_allowed:false,
    operator_evidence_recording_boundary_readback_ready:$boundary_ready,
    entries:$entries,
    blockers:[
      "evidence_recording_blocked",
      "evidence_persistence_blocked",
      "evidence_receipt_persistence_blocked",
      "approval_request_blocked",
      "approval_acceptance_blocked",
      "approval_recording_blocked",
      "approval_receipt_persistence_blocked",
      "operator_decision_recording_blocked",
      "operator_decision_recording_persistence_blocked",
      "operator_decision_receipt_persistence_blocked",
      "operator_evidence_recording_boundary_readback_persistence_blocked",
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
      "packet_send_blocked",
      "packet_persistence_blocked",
      "readback_persistence_blocked",
      "release_cutover_blocked",
      "canary_activation_blocked",
      "live_activation_blocked"
    ],
    next_actions:[
      "workflow_temporal_lite_append_only_event_store_minimal_local_persistence"
    ],
    recommended_next_gate:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
    next_migration_step:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
    report_script:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      boundary_readback_persisted:false,
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
      decision_checklist_persisted:false,
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
      blocker_waived:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      canary_activation_started:false,
      live_activation_started:false,
      live_execution_started:false
    }
  }'
