#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-outcome-readback-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_OUTCOME_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-outcome-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable owner/freeze/classification outcome readback report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable owner/freeze/classification rehearsal report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing owner/freeze/classification outcome architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the owner/freeze/classification outcome readback report"
fi

grep -q 'Owner Freeze Classification Outcome Readback' "$DOC" \
  || fail "architecture note must document Owner Freeze Classification Outcome Readback"
grep -q 'visible-only outcome read model' "$DOC" \
  || fail "architecture note must document the visible-only outcome read model"
grep -q 'no owner assignment persistence, freeze application, classification persistence, test probe execution, git mutation, cleanup, delete, evidence recording, approval acceptance, decision recording, operator packet send, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed boundary"
grep -q 'dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send' "$DOC" \
  || fail "architecture note must document the operator-packet next gate"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation"
  and .status == "blocked"
  and .gate == "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation_gate"
  and .schema_version == "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_rehearsal_ready == false
  and .source_rehearsal_visible == false
  and .source_rehearsal_persisted == false
  and .source_classification_entry_count == 4
  and .lib_export_present == true
  and .outcome_entry_count == .source_classification_entry_count
  and .stable_outcome_key_count == .outcome_entry_count
  and .outcome_route_count == .outcome_entry_count
  and .outcome_ready_count == .outcome_entry_count
  and .owner_attribution_outcome_required_count == 1
  and .targeted_gate_outcome_required_count == 2
  and .owned_lane_freeze_outcome_required_count == 1
  and .artifact_classification_outcome_required_count == 0
  and .hepta_systems_owner_route_count == 3
  and .cross_lane_owner_route_count == 1
  and .release_blocked_count == .outcome_entry_count
  and .test_probe_execution_blocked_count == .outcome_entry_count
  and .git_mutation_blocked_count == .outcome_entry_count
  and .cleanup_delete_blocked_count == .outcome_entry_count
  and .evidence_recording_blocked_count == .outcome_entry_count
  and .approval_acceptance_blocked_count == .outcome_entry_count
  and .decision_recording_blocked_count == .outcome_entry_count
  and .outcome_readback_visible == false
  and .outcome_readback_persisted == false
  and .owner_freeze_classification_outcome_readback_ready == false
  and .owner_assignment_persisted == false
  and .freeze_applied == false
  and .classification_persisted == false
  and .test_probe_executed == false
  and .evidence_recorded == false
  and .approval_accepted == false
  and .decision_recorded == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .strategy_applied == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .release_cutover_allowed == false
  and .package_or_release_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == .outcome_entry_count
  and (.entries | all(
    .source_rehearsal_attached == true
    and .outcome_readback_visible == true
    and .outcome_readback_persisted == false
    and .queryable == true
    and .diffable == true
    and .test_probe_executed == false
    and .mutation_free == true
    and .owner_assignment_persisted == false
    and .freeze_applied == false
    and .classification_persisted == false
    and .evidence_recording_allowed == false
    and .approval_acceptance_allowed == false
    and .decision_recording_allowed == false
    and .git_mutation_blocked == true
    and .cleanup_delete_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false
    and .operator_packet_candidate == true))
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .outcome_category == "owner_attribution_outcome_required")
  and any(.entries[]; .source_bucket == "codex-rs" and .outcome_category == "targeted_gate_outcome_required")
  and any(.entries[]; .source_bucket == "scripts" and .outcome_category == "targeted_gate_outcome_required")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .outcome_category == "owned_lane_freeze_outcome_required")
  and (.blockers | index("owner_freeze_classification_outcome_readback_visible_only")) != null
  and (.blockers | index("owner_assignment_persistence_blocked")) != null
  and (.blockers | index("freeze_application_blocked")) != null
  and (.blockers | index("classification_persistence_blocked")) != null
  and (.blockers | index("operator_packet_send_blocked")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send")) != null
  and .recommended_next_gate == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation"
  and .status == "blocked"
  and .owner_freeze_classification_rehearsal_ready == false
  and .owner_freeze_classification_readback_visible == false
  and .owner_freeze_classification_readback_persisted == false
  and .classification_entry_count == 4
  and .test_probe_executed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-outcome-readback-gate: PASS: owner/freeze/classification outcome exposes four dirty buckets with owner assignment, freeze, classification, git, cleanup, evidence, approval, decision, operator packet, release, canary, and live paths blocked\n'
