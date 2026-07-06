#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_2026-06-28.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 23 release risk snapshot report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 22 evidence recording boundary readback report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 23 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 23 release risk snapshot report"
fi

grep -q 'Dirty Worktree Release Boundary Release Risk Snapshot' "$DOC" \
  || fail "architecture note must document Dirty Worktree Release Boundary Release Risk Snapshot"
grep -q 'fast_local_release_risk_snapshot_only' "$DOC" \
  || fail "architecture note must document fast_local_release_risk_snapshot_only"
grep -q 'critical, high, and medium release-risk tiers' "$DOC" \
  || fail "architecture note must document release-risk tiers"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed Phase 23 boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_release_risk_snapshot"
  and .status == "blocked"
  and .gate == "dirty_worktree_release_boundary_release_risk_snapshot_gate"
  and .schema_version == "dirty_worktree_release_boundary_release_risk_snapshot_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_evidence_recording_boundary_ready == true
  and .source_evidence_recording_boundary_visible == true
  and .source_evidence_recording_boundary_persisted == false
  and .source_evidence_recorded == false
  and .source_evidence_recording_persisted == false
  and .source_evidence_receipt_persisted == false
  and .source_boundary_entry_count == 4
  and .lib_export_present == true
  and .inventory_entry_count == (.tracked_change_count + .untracked_change_count)
  and .release_risk_snapshot_scope.snapshot_mode == "fast_local_release_risk_snapshot_only"
  and .release_risk_snapshot_scope.release_cutover_boundary == "blocked_dirty_worktree"
  and .release_risk_snapshot_scope.git_mutation_boundary == "blocked"
  and .release_risk_snapshot_scope.cleanup_boundary == "blocked"
  and .release_risk_snapshot_scope.evidence_boundary == "blocked"
  and .release_risk_snapshot_scope.live_boundary == "blocked"
  and .risk_entry_count == .source_boundary_entry_count
  and .stable_snapshot_key_count == .risk_entry_count
  and .snapshot_route_count == .risk_entry_count
  and .snapshot_ready_count == .risk_entry_count
  and .critical_risk_count == 1
  and .high_risk_count == 3
  and .medium_risk_count == 0
  and .high_or_critical_risk_count == 4
  and .release_blocked_count == .risk_entry_count
  and .rehearsal_candidate_count == .risk_entry_count
  and .pending_operator_decision_count == .risk_entry_count
  and .evidence_recording_blocked_count == .risk_entry_count
  and .git_mutation_blocked_count == .risk_entry_count
  and .cleanup_delete_blocked_count == .risk_entry_count
  and .risk_snapshot_visible == false
  and .risk_snapshot_persisted == false
  and .release_risk_snapshot_ready == false
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
  and (.entries | length) == .risk_entry_count
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .release_risk_tier == "critical" and .rehearsal_action == "test_only_owner_attribution_and_freeze_rehearsal")
  and any(.entries[]; .source_bucket == "codex-rs" and .release_risk_tier == "high")
  and any(.entries[]; .source_bucket == "scripts" and .release_risk_tier == "high")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .release_risk_tier == "high")
  and (.entries | all(
    .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .release_blocker_state == "blocked_dirty_worktree"
    and .clean_worktree_rehearsal_candidate == true
    and .decision_state == "pending_operator_decision"
    and .evidence_recording_state == "evidence_recording_blocked"
    and .evidence_persistence_state == "evidence_persistence_blocked"
    and .evidence_receipt_state == "evidence_receipt_blocked"
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
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
  and (.blockers | index("dirty_worktree_release_risk_snapshot_visible_only")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("git_mutation_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation")) != null
  and .recommended_next_gate == "phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback"
  and .status == "ready_blocked"
  and .operator_evidence_recording_boundary_readback_ready == true
  and .evidence_recording_boundary_readback_visible == true
  and .evidence_recording_boundary_readback_persisted == false
  and .boundary_entry_count == 4
  and .evidence_recorded == false
  and .evidence_recording_persisted == false
  and .evidence_receipt_persisted == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .live_execution_allowed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_release_risk_snapshot --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-gate: PASS: release risk snapshot exposes four dirty buckets with release and live blocked\n'
