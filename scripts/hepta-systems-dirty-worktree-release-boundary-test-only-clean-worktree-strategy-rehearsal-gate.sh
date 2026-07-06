#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_2026-06-28.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 24 test-only rehearsal report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 23 release risk snapshot report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 24 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 24 test-only clean-worktree strategy rehearsal report"
fi

grep -q 'Dirty Worktree Release Boundary Test-Only Clean Worktree Strategy Rehearsal' "$DOC" \
  || fail "architecture note must document the Phase 24 test-only rehearsal"
grep -q 'test_only_no_git_mutation_no_cleanup_no_evidence_recording' "$DOC" \
  || fail "architecture note must document the test-only rehearsal mode"
grep -q 'owner attribution, targeted Rust, plugin, script, owned-lane, artifact, and doc gates' "$DOC" \
  || fail "architecture note must document required local gates"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, test probe execution, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed Phase 24 boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal"
  and .status == "blocked"
  and .gate == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_gate"
  and .schema_version == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_release_risk_snapshot_ready == false
  and .source_release_risk_snapshot_visible == false
  and .source_release_risk_snapshot_persisted == false
  and .source_evidence_recorded == false
  and .source_evidence_recording_persisted == false
  and .source_evidence_receipt_persisted == false
  and .source_risk_entry_count == 4
  and .lib_export_present == true
  and .inventory_entry_count == (.tracked_change_count + .untracked_change_count)
  and .rehearsal_scope.rehearsal_mode == "test_only_no_git_mutation_no_cleanup_no_evidence_recording"
  and .rehearsal_scope.git_mutation_boundary == "blocked"
  and .rehearsal_scope.cleanup_boundary == "blocked"
  and .rehearsal_scope.evidence_boundary == "blocked"
  and .rehearsal_scope.approval_boundary == "blocked"
  and .rehearsal_scope.decision_boundary == "blocked"
  and .rehearsal_scope.live_boundary == "blocked"
  and .rehearsal_entry_count == .source_risk_entry_count
  and .stable_rehearsal_key_count == .rehearsal_entry_count
  and .rehearsal_route_count == .rehearsal_entry_count
  and .rehearsal_ready_count == .rehearsal_entry_count
  and .convergence_candidate_count == .rehearsal_entry_count
  and .owner_attribution_required_count == 1
  and .runtime_gate_required_count == 1
  and .plugin_gate_required_count == 0
  and .script_gate_required_count == 1
  and .owned_lane_freeze_required_count == 1
  and .artifact_classification_required_count == 0
  and .doc_evidence_required_count == 0
  and .release_blocked_count == .rehearsal_entry_count
  and .git_mutation_blocked_count == .rehearsal_entry_count
  and .cleanup_delete_blocked_count == .rehearsal_entry_count
  and .evidence_recording_blocked_count == .rehearsal_entry_count
  and .approval_acceptance_blocked_count == .rehearsal_entry_count
  and .decision_recording_blocked_count == .rehearsal_entry_count
  and .test_only_rehearsal_visible == false
  and .test_only_rehearsal_persisted == false
  and .test_probe_executed == false
  and .test_only_clean_worktree_strategy_rehearsal_ready == false
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
  and (.entries | length) == .rehearsal_entry_count
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .required_local_gate == "owner_attribution_freeze_gate" and .convergence_state == "blocked_until_owner_attribution")
  and any(.entries[]; .source_bucket == "codex-rs" and .required_local_gate == "targeted_rust_gate")
  and any(.entries[]; .source_bucket == "scripts" and .required_local_gate == "script_syntax_gate")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .required_local_gate == "owned_lane_freeze_gate")
  and (.entries | all(
    .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .source_release_blocker_state == "blocked_dirty_worktree"
    and .source_snapshot_attached == true
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .test_only_rehearsal_candidate == true
    and .test_only_rehearsal_visible == true
    and .test_only_rehearsal_executed == false
    and .mutation_free == true
    and .decision_state == "pending_operator_decision"
    and .evidence_recording_state == "evidence_recording_blocked"
    and .evidence_persistence_state == "evidence_persistence_blocked"
    and .evidence_receipt_state == "evidence_receipt_blocked"
    and .evidence_recording_allowed == false
    and .approval_acceptance_allowed == false
    and .decision_recording_allowed == false
    and .git_add_blocked == true
    and .git_index_mutation_blocked == true
    and .git_commit_blocked == true
    and .git_push_blocked == true
    and .git_reset_blocked == true
    and .git_checkout_blocked == true
    and .git_revert_blocked == true
    and .cleanup_blocked == true
    and .delete_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false))
  and (.blockers | index("test_only_rehearsal_visible_only")) != null
  and (.blockers | index("test_probe_execution_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation")) != null
  and .recommended_next_gate == "phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_release_risk_snapshot"
  and .status == "blocked"
  and .release_risk_snapshot_ready == false
  and .risk_snapshot_visible == false
  and .risk_snapshot_persisted == false
  and .risk_entry_count == 4
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .live_execution_allowed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-gate: PASS: clean-worktree strategy rehearsal exposes four dirty buckets with test probes and live blocked\n'
