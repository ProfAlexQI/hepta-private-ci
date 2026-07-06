#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 25 outcome readback report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 24 test-only rehearsal report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 25 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 25 outcome readback report"
fi

grep -q 'Test-Only Rehearsal Outcome Readback' "$DOC" \
  || fail "architecture note must document Test-Only Rehearsal Outcome Readback"
grep -q 'outcome_readback_visible_only' "$DOC" \
  || fail "architecture note must document visible-only outcome readback"
grep -q 'no test probe execution, git mutation, cleanup, delete, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed Phase 25 boundary"
grep -q 'temporal_lite_append_only_event_store_feature_gated_test_implementation' "$DOC" \
  || fail "architecture note must document the Temporal-lite next gate"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback"
  and .status == "ready_blocked"
  and .gate == "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_gate"
  and .schema_version == "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_rehearsal_ready == true
  and .source_test_only_rehearsal_visible == true
  and .source_test_only_rehearsal_persisted == false
  and .source_test_probe_executed == false
  and .source_rehearsal_entry_count == 7
  and .source_rehearsal_ready_count == 7
  and .lib_export_present == true
  and .outcome_entry_count == .source_rehearsal_entry_count
  and .stable_outcome_key_count == .outcome_entry_count
  and .outcome_route_count == .outcome_entry_count
  and .outcome_ready_count == .outcome_entry_count
  and .blocked_until_owner_attribution_count == 1
  and .ready_for_targeted_rust_gate_rehearsal_count == 1
  and .ready_for_plugin_surface_gate_rehearsal_count == 1
  and .ready_for_script_syntax_gate_rehearsal_count == 1
  and .ready_for_owned_lane_freeze_rehearsal_count == 1
  and .ready_for_artifact_classification_rehearsal_count == 1
  and .ready_for_doc_evidence_consistency_rehearsal_count == 1
  and .release_blocked_count == .outcome_entry_count
  and .test_probe_execution_blocked_count == .outcome_entry_count
  and .git_mutation_blocked_count == .outcome_entry_count
  and .cleanup_delete_blocked_count == .outcome_entry_count
  and .evidence_recording_blocked_count == .outcome_entry_count
  and .approval_acceptance_blocked_count == .outcome_entry_count
  and .decision_recording_blocked_count == .outcome_entry_count
  and .outcome_readback_visible == true
  and .outcome_readback_persisted == false
  and .test_probe_executed == false
  and .evidence_recorded == false
  and .evidence_recording_persisted == false
  and .evidence_receipt_persisted == false
  and .approval_request_sent == false
  and .approval_accepted == false
  and .approval_recorded == false
  and .approval_receipt_persisted == false
  and .decision_recorded == false
  and .decision_recording_persisted == false
  and .decision_receipt_persisted == false
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .readback_persisted == false
  and .strategy_applied == false
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
  and .test_only_rehearsal_outcome_readback_ready == true
  and (.entries | length) == 7
  and (.entries | all(.source_rehearsal_attached == true and .outcome_readback_visible == true and .outcome_readback_persisted == false and .operator_visible == true and .queryable == true and .diffable == true and .test_probe_executed == false and .mutation_free == true and .source_release_blocker_state == "blocked_dirty_worktree" and .decision_state == "pending_operator_decision" and .evidence_recording_state == "evidence_recording_blocked" and .evidence_persistence_state == "evidence_persistence_blocked" and .evidence_receipt_state == "evidence_receipt_blocked" and .evidence_recording_allowed == false and .approval_acceptance_allowed == false and .decision_recording_allowed == false and .git_add_blocked == true and .git_index_mutation_blocked == true and .git_commit_blocked == true and .git_push_blocked == true and .git_reset_blocked == true and .git_checkout_blocked == true and .git_revert_blocked == true and .cleanup_blocked == true and .delete_blocked == true and .release_cutover_allowed == false and .canary_activation_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .outcome_state == "blocked_until_owner_attribution" and .outcome_action == "attribute_owner_before_any_clean_worktree_action")
  and any(.entries[]; .source_bucket == "codex-rs" and .outcome_state == "ready_for_targeted_rust_gate_rehearsal" and .outcome_key == "dirty_worktree.test_only_rehearsal_outcome_readback.top_level.codex_rs" and .outcome_route == "readback://release-boundary/dirty-worktree/test-only-rehearsal-outcome-readback/top-level/codex-rs")
  and any(.entries[]; .source_bucket == "plugins" and .outcome_state == "ready_for_plugin_surface_gate_rehearsal")
  and any(.entries[]; .source_bucket == "scripts" and .outcome_state == "ready_for_script_syntax_gate_rehearsal")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .outcome_state == "ready_for_owned_lane_freeze_rehearsal")
  and any(.entries[]; .source_bucket == "artifacts" and .outcome_state == "ready_for_artifact_classification_rehearsal")
  and any(.entries[]; .source_bucket == "docs" and .outcome_state == "ready_for_doc_evidence_consistency_rehearsal")
  and (.blockers | index("outcome_readback_visible_only")) != null
  and (.blockers | index("test_probe_execution_still_blocked")) != null
  and (.next_actions | index("temporal_lite_append_only_event_store_feature_gated_test_implementation")) != null
  and .recommended_next_gate == "temporal_lite_append_only_event_store_feature_gated_test_implementation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal"
  and .status == "ready_blocked"
  and .test_only_clean_worktree_strategy_rehearsal_ready == true
  and .test_only_rehearsal_visible == true
  and .test_only_rehearsal_persisted == false
  and .test_probe_executed == false
  and .rehearsal_entry_count == 7
  and .rehearsal_ready_count == 7
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-gate: PASS: Phase 25 outcome readback is visible-only and keeps probe, git, cleanup, evidence, approval, decision, release, canary, and live paths blocked\n'
