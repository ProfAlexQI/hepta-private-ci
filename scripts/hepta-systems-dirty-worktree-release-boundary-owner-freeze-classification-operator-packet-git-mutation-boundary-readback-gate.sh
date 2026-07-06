#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-git-mutation-boundary-readback-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-without-send-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-git-mutation-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable owner/freeze/classification operator packet git-mutation boundary report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification operator packet without-send report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification operator packet git-mutation boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the owner/freeze/classification operator packet git-mutation boundary report"
fi

grep -q 'Owner Freeze Classification Operator Packet Git-Mutation Boundary Readback' "$DOC" \
  || fail "architecture note must document Owner Freeze Classification Operator Packet Git-Mutation Boundary Readback"
grep -q 'git_mutation_boundary_readback_only' "$DOC" \
  || fail "architecture note must document git_mutation_boundary_readback_only"
grep -q 'git add, commit, push, reset, checkout, revert, cleanup, and delete remain blocked' "$DOC" \
  || fail "architecture note must document blocked git operations"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, owner assignment persistence, freeze application, classification persistence, test probe execution, packet send, packet persistence, packet payload persistence, readback persistence, evidence recording, approval request, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed git-mutation boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation"
  and .status == "blocked"
  and .gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_gate"
  and .schema_version == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_operator_packet_ready == false
  and .source_operator_packet_visible == false
  and .source_operator_packet_sent == false
  and .source_operator_packet_persisted == false
  and .source_packet_payload_persisted == false
  and .source_readback_persisted == false
  and .source_packet_entry_count == 4
  and .lib_export_present == true
  and .readback_scope.readback_mode == "git_mutation_boundary_readback_only"
  and .readback_scope.git_mutation_boundary == "closed"
  and .readback_scope.git_index_boundary == "blocked"
  and .readback_scope.cleanup_boundary == "blocked"
  and .readback_scope.deletion_boundary == "blocked"
  and .readback_entry_count == .source_packet_entry_count
  and .stable_readback_key_count == .readback_entry_count
  and .readback_route_count == .readback_entry_count
  and .readback_ready_count == .readback_entry_count
  and .packet_visible_unsent_unpersisted_count == .readback_entry_count
  and .git_mutation_blocked_count == .readback_entry_count
  and .git_operation_blocked_count == .readback_entry_count
  and .cleanup_delete_blocked_count == .readback_entry_count
  and .owner_assignment_blocked_count == .readback_entry_count
  and .freeze_application_blocked_count == .readback_entry_count
  and .classification_persistence_blocked_count == .readback_entry_count
  and .test_probe_blocked_count == .readback_entry_count
  and .operator_decision_required_count == .readback_entry_count
  and .evidence_recorded_count == 0
  and .operator_packet_visible == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .packet_payload_persisted == false
  and .readback_persisted == false
  and .git_mutation_boundary_readback_ready == false
  and .owner_assignment_persisted == false
  and .freeze_applied == false
  and .classification_persisted == false
  and .test_probe_executed == false
  and .evidence_recording_allowed == false
  and .evidence_persistence_allowed == false
  and .approval_request_sent == false
  and .approval_acceptance_allowed == false
  and .decision_recording_allowed == false
  and .blocker_waiver_allowed == false
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
  and .package_or_release_allowed == false
  and .public_ga_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == .readback_entry_count
  and (.entries | all(
    .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .previous_git_mutation_state == "blocked"
    and .current_git_mutation_state == "blocked"
    and .git_mutation_state_delta == "unchanged_blocked"
    and .packet_visible == true
    and .packet_unsent == true
    and .packet_unpersisted == true
    and .readback_unpersisted == true
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .operator_decision_required == true
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
    and .evidence_recording_allowed == false
    and .approval_request_blocked == true
    and .approval_acceptance_blocked == true
    and .decision_recording_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false))
  and any(.entries[]; .source_bucket == "codex-rs")
  and any(.entries[]; .source_bucket == "hepta_systems_owned")
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned")
  and (.blockers | index("git_add_blocked")) != null
  and (.blockers | index("git_index_mutation_blocked")) != null
  and (.blockers | index("git_commit_blocked")) != null
  and (.blockers | index("git_push_blocked")) != null
  and (.blockers | index("git_reset_blocked")) != null
  and (.blockers | index("git_checkout_blocked")) != null
  and (.blockers | index("git_revert_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("operator_packet_send_blocked")) != null
  and (.blockers | index("operator_packet_persistence_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("approval_request_blocked")) != null
  and (.blockers | index("approval_acceptance_blocked")) != null
  and (.blockers | index("decision_recording_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation")) != null
  and .recommended_next_gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send"
  and .status == "blocked"
  and .operator_packet_without_send_ready == false
  and .packet_entry_count == 4
  and .operator_packet_visible == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .packet_payload_persisted == false
  and .readback_persisted == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .live_execution_allowed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-git-mutation-boundary-readback-gate: PASS: owner/freeze/classification git-mutation boundary exposes four dirty buckets with git, cleanup, delete, release, canary, and live blocked\n'
