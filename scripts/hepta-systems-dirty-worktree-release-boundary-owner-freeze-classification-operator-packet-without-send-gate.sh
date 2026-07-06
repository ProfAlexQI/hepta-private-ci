#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-without-send-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-outcome-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OPERATOR_PACKET_WITHOUT_SEND_2026-06-29.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-without-send-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable owner/freeze/classification operator packet report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification outcome readback report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification operator packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the owner/freeze/classification operator packet report"
fi

grep -q 'Owner Freeze Classification Operator Packet Without Send' "$DOC" \
  || fail "architecture note must document Owner Freeze Classification Operator Packet Without Send"
grep -q 'visible, unsent, and unpersisted' "$DOC" \
  || fail "architecture note must document visible, unsent, and unpersisted packet state"
grep -q 'no owner assignment persistence, freeze application, classification persistence, test probe execution, packet send, packet persistence, packet payload persistence, readback persistence, git mutation, cleanup, delete, evidence recording, approval request, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed packet boundary"
grep -q 'dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation' "$DOC" \
  || fail "architecture note must document the git-mutation boundary next gate"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send"
  and .status == "ready_blocked"
  and .gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_gate"
  and .schema_version == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_outcome_ready == true
  and .source_outcome_visible == true
  and .source_outcome_persisted == false
  and .source_outcome_entry_count == 7
  and .lib_export_present == true
  and .packet_id == "dirty-worktree-owner-freeze-classification-operator-packet"
  and .packet_route == "operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/v1"
  and .packet_payload_hash == "sha256:dirty-worktree-owner-freeze-classification-operator-packet-no-send-no-live"
  and .packet_entry_count == .source_outcome_entry_count
  and .stable_packet_key_count == .packet_entry_count
  and .packet_route_count == .packet_entry_count
  and .packet_ready_count == .packet_entry_count
  and .visible_unsent_unpersisted_count == .packet_entry_count
  and .attached_outcome_count == .packet_entry_count
  and .owner_attribution_packet_count == 1
  and .targeted_gate_packet_count == 4
  and .owned_lane_freeze_packet_count == 1
  and .artifact_classification_packet_count == 1
  and .hepta_systems_owner_route_count == 4
  and .cross_lane_owner_route_count == 3
  and .operator_decision_required_count == .packet_entry_count
  and .packet_send_blocked_count == .packet_entry_count
  and .packet_persistence_blocked_count == .packet_entry_count
  and .git_mutation_blocked_count == .packet_entry_count
  and .cleanup_delete_blocked_count == .packet_entry_count
  and .evidence_recording_blocked_count == .packet_entry_count
  and .approval_request_blocked_count == .packet_entry_count
  and .approval_acceptance_blocked_count == .packet_entry_count
  and .decision_recording_blocked_count == .packet_entry_count
  and .operator_packet_visible == true
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .packet_payload_persisted == false
  and .readback_persisted == false
  and .owner_assignment_persisted == false
  and .freeze_applied == false
  and .classification_persisted == false
  and .test_probe_executed == false
  and .evidence_recorded == false
  and .approval_requested == false
  and .approval_accepted == false
  and .decision_recorded == false
  and .strategy_applied == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .release_cutover_allowed == false
  and .package_or_release_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and .operator_packet_without_send_ready == true
  and (.entries | length) == 7
  and (.entries | all(
    (.source_outcome_key | startswith("dirty_worktree.owner_freeze_classification_outcome."))
    and (.source_outcome_route | startswith("readback://release-boundary/dirty-worktree/owner-freeze-classification-outcome/"))
    and (.packet_key | startswith("dirty_worktree.owner_freeze_classification_operator_packet."))
    and (.packet_route | startswith("operator-packet://release-boundary/dirty-worktree/owner-freeze-classification/"))
    and (.non_send_readback_key | startswith("dirty_worktree.owner_freeze_classification_operator_packet.non_send."))
    and (.non_send_readback_route | startswith("readback://release-boundary/dirty-worktree/owner-freeze-classification/operator-packet/non-send/"))
    and .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .observed_state == "operator_packet_visible_unsent_unpersisted"
    and .previous_send_state == "unsent"
    and .current_send_state == "unsent"
    and .send_state_delta == "unchanged_unsent"
    and .previous_persistence_state == "unpersisted"
    and .current_persistence_state == "unpersisted"
    and .persistence_state_delta == "unchanged_unpersisted"
    and .source_outcome_attached == true
    and .packet_visible == true
    and .packet_payload_visible == true
    and .non_send_confirmed == true
    and .non_persistence_confirmed == true
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .operator_decision_required == true
    and .owner_assignment_blocked == true
    and .freeze_application_blocked == true
    and .classification_persistence_blocked == true
    and .test_probe_blocked == true
    and .packet_send_blocked == true
    and .packet_persistence_blocked == true
    and .readback_persistence_blocked == true
    and .approval_request_blocked == true
    and .approval_acceptance_blocked == true
    and .decision_recording_blocked == true
    and .evidence_recording_blocked == true
    and .git_mutation_blocked == true
    and .cleanup_delete_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false))
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .packet_section == "owner_attribution_packet_section" and .packet_action == "include_owner_attribution_request_without_assignment")
  and any(.entries[]; .source_bucket == "codex-rs" and .packet_section == "targeted_gate_packet_section" and .packet_action == "include_targeted_gate_request_without_probe_execution")
  and any(.entries[]; .source_bucket == "plugins" and .packet_section == "targeted_gate_packet_section")
  and any(.entries[]; .source_bucket == "scripts" and .packet_section == "targeted_gate_packet_section")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .packet_section == "owned_lane_freeze_packet_section" and .packet_action == "include_owned_lane_freeze_request_without_applying_freeze")
  and any(.entries[]; .source_bucket == "artifacts" and .packet_section == "artifact_classification_packet_section" and .packet_action == "include_artifact_classification_request_without_delete_or_relocation")
  and any(.entries[]; .source_bucket == "docs" and .packet_section == "targeted_gate_packet_section")
  and (.blockers | index("operator_packet_send_blocked")) != null
  and (.blockers | index("operator_packet_persistence_blocked")) != null
  and (.blockers | index("operator_packet_payload_persistence_blocked")) != null
  and (.blockers | index("operator_packet_readback_persistence_blocked")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("approval_request_blocked")) != null
  and (.blockers | index("approval_acceptance_blocked")) != null
  and (.blockers | index("decision_recording_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation")) != null
  and .recommended_next_gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation"
  and .status == "ready_blocked"
  and .owner_freeze_classification_outcome_readback_ready == true
  and .outcome_entry_count == 7
  and .outcome_readback_visible == true
  and .outcome_readback_persisted == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-packet-without-send-gate: PASS: owner/freeze/classification operator packet is visible, unsent, unpersisted, and keeps git, cleanup, evidence, approval, decision, release, canary, and live paths blocked\n'
