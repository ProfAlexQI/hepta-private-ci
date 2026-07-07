#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-approval-acceptance-boundary-readback-without-acceptance-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable owner/freeze/classification evidence recording boundary report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification approval acceptance boundary report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification evidence recording architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the owner/freeze/classification evidence recording boundary report"
fi

grep -q 'Owner/Freeze/Classification Operator Evidence Recording Boundary Readback' "$DOC" \
  || fail "architecture note must document Owner/Freeze/Classification Operator Evidence Recording Boundary Readback"
grep -q 'operator_evidence_recording_boundary_readback_only' "$DOC" \
  || fail "architecture note must document operator_evidence_recording_boundary_readback_only"
grep -q 'evidence recording, evidence persistence, evidence receipt persistence, approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, packet send, packet persistence, packet payload persistence, readback persistence, owner assignment persistence, freeze application, classification persistence, test probe execution, git mutation, cleanup, delete, release, canary activation, and live execution remain blocked' "$DOC" \
  || fail "architecture note must document blocked owner/freeze/classification evidence boundaries"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, owner assignment persistence, freeze application, classification persistence, test probe execution, approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, decision persistence, decision receipt persistence, evidence recording, evidence persistence, evidence receipt persistence, packet send, packet persistence, packet payload persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document closed owner/freeze/classification evidence boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording"
  and .status == "blocked"
  and .gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_gate"
  and .schema_version == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_approval_acceptance_boundary_ready == false
  and .source_approval_acceptance_boundary_visible == false
  and .source_approval_acceptance_boundary_persisted == false
  and .source_approval_request_sent == false
  and .source_approval_accepted == false
  and .source_approval_recorded == false
  and .source_approval_receipt_persisted == false
  and .source_decision_recorded == false
  and .source_operator_packet_sent == false
  and .source_operator_packet_persisted == false
  and .source_owner_assignment_persisted == false
  and .source_freeze_applied == false
  and .source_classification_persisted == false
  and .source_test_probe_executed == false
  and .source_boundary_entry_count == .boundary_entry_count
  and .lib_export_present == true
  and .evidence_recording_boundary_scope.readback_mode == "operator_evidence_recording_boundary_readback_only"
  and .evidence_recording_boundary_scope.evidence_recording_boundary == "blocked"
  and .evidence_recording_boundary_scope.evidence_persistence_boundary == "blocked"
  and .evidence_recording_boundary_scope.evidence_receipt_boundary == "blocked"
  and .evidence_recording_boundary_scope.approval_acceptance_boundary == "blocked"
  and .evidence_recording_boundary_scope.git_mutation_boundary == "blocked"
  and .boundary_entry_count > 0
  and .stable_boundary_key_count == .boundary_entry_count
  and .boundary_route_count == .boundary_entry_count
  and .boundary_ready_count == .boundary_entry_count
  and .source_boundary_attached_count == .boundary_entry_count
  and .pending_operator_decision_count == .boundary_entry_count
  and .evidence_recording_blocked_count == .boundary_entry_count
  and .evidence_persistence_blocked_count == .boundary_entry_count
  and .evidence_receipt_blocked_count == .boundary_entry_count
  and .evidence_recorded_count == 0
  and .approval_request_blocked_count == .boundary_entry_count
  and .approval_acceptance_blocked_count == .boundary_entry_count
  and .approval_recording_blocked_count == .boundary_entry_count
  and .approval_receipt_blocked_count == .boundary_entry_count
  and .decision_recording_blocked_count == .boundary_entry_count
  and .packet_visible_unsent_unpersisted_count == .boundary_entry_count
  and .readback_unpersisted_count == .boundary_entry_count
  and .git_mutation_blocked_count == .boundary_entry_count
  and .cleanup_delete_blocked_count == .boundary_entry_count
  and .owner_assignment_blocked_count == .boundary_entry_count
  and .freeze_application_blocked_count == .boundary_entry_count
  and .classification_persistence_blocked_count == .boundary_entry_count
  and .test_probe_blocked_count == .boundary_entry_count
  and .packet_send_blocked_count == .boundary_entry_count
  and .packet_persistence_blocked_count == .boundary_entry_count
  and .readback_persistence_blocked_count == .boundary_entry_count
  and .evidence_recording_boundary_readback_visible == false
  and .evidence_recording_boundary_readback_persisted == false
  and .evidence_recorded == false
  and .evidence_recording_persisted == false
  and .evidence_receipt_persisted == false
  and .approval_request_sent == false
  and .approval_accepted == false
  and .approval_recorded == false
  and .approval_receipt_persisted == false
  and .decision_recorded == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .packet_payload_persisted == false
  and .readback_persisted == false
  and .owner_assignment_persisted == false
  and .freeze_applied == false
  and .classification_persisted == false
  and .test_probe_executed == false
  and .release_cutover_allowed == false
  and .git_add_allowed == false
  and .git_index_mutated == false
  and .git_commit_allowed == false
  and .git_push_allowed == false
  and .git_reset_allowed == false
  and .git_checkout_allowed == false
  and .git_revert_allowed == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .blocker_waiver_allowed == false
  and .package_or_release_allowed == false
  and .public_ga_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and .operator_evidence_recording_boundary_readback_ready == false
  and (.entries | length) == .boundary_entry_count
  and (.entries | all(
    .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .decision_state == "pending_operator_decision"
    and .evidence_recording_state == "evidence_recording_blocked"
    and .evidence_persistence_state == "evidence_persistence_blocked"
    and .evidence_receipt_state == "evidence_receipt_blocked"
    and .approval_acceptance_state == "approval_acceptance_blocked"
    and .source_boundary_state == "approval_acceptance_boundary_visible_unpersisted"
    and .packet_visible == true
    and .packet_unsent == true
    and .packet_unpersisted == true
    and .readback_visible == true
    and .readback_unpersisted == true
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .operator_decision_required == true
    and .evidence_recording_allowed == false
    and .evidence_persistence_allowed == false
    and .evidence_receipt_persistence_allowed == false
    and .approval_request_allowed == false
    and .approval_acceptance_allowed == false
    and .approval_recording_allowed == false
    and .approval_receipt_persistence_allowed == false
    and .decision_recording_allowed == false
    and .decision_persistence_allowed == false
    and .decision_receipt_persistence_allowed == false
    and .git_add_blocked == true
    and .git_index_mutation_blocked == true
    and .git_commit_blocked == true
    and .git_push_blocked == true
    and .git_reset_blocked == true
    and .git_checkout_blocked == true
    and .git_revert_blocked == true
    and .cleanup_blocked == true
    and .delete_blocked == true
    and .owner_assignment_blocked == true
    and .freeze_application_blocked == true
    and .classification_persistence_blocked == true
    and .test_probe_blocked == true
    and .packet_send_blocked == true
    and .packet_persistence_blocked == true
    and .readback_persistence_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false))
  and any(.entries[]; .source_bucket == "codex-rs" and .required_local_gate == "targeted_rust_gate")
  and any(.entries[]; .source_bucket == "docs" and .required_local_gate == "doc_evidence_consistency_gate")
  and any(.entries[]; .source_bucket == "scripts" and .required_local_gate == "script_syntax_gate")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .required_local_gate == "owned_lane_freeze_gate")
  and ((.entries | map(select(.source_bucket == "cross_lane_or_unowned")) | length) == 0
    or any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .required_local_gate == "owner_attribution_freeze_gate"))
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("approval_request_blocked")) != null
  and (.blockers | index("approval_acceptance_blocked")) != null
  and (.blockers | index("git_index_mutation_blocked")) != null
  and (.blockers | index("owner_assignment_persistence_blocked")) != null
  and (.blockers | index("freeze_application_blocked")) != null
  and (.blockers | index("classification_persistence_blocked")) != null
  and (.blockers | index("test_probe_execution_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("workflow_temporal_lite_append_only_event_store_minimal_local_persistence")) != null
  and .recommended_next_gate == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence"
  and .next_migration_step == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance"
  and .status == "blocked"
  and .operator_approval_acceptance_boundary_readback_ready == false
  and .boundary_entry_count > 0
  and .pending_operator_decision_count == .boundary_entry_count
  and .approval_acceptance_blocked_count == .boundary_entry_count
  and .release_cutover_allowed == false
  and .canary_activation_allowed == false
  and .live_execution_allowed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate: PASS: owner/freeze evidence boundary exposes dirty buckets with recording and live execution blocked\n'
