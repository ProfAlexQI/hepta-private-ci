#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-report.sh"
GIT_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_2026-06-27.md"

fail() {
  printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 18 operator decision checklist report: $REPORT"
[[ -x "$GIT_BOUNDARY_REPORT" ]] || fail "missing executable Phase 17 git-mutation boundary readback report: $GIT_BOUNDARY_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 18 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 18 operator decision checklist report"
fi

grep -q 'Dirty Worktree Release Boundary Clean Worktree Strategy Operator Decision Checklist' "$DOC" \
  || fail "architecture note must document Dirty Worktree Release Boundary Clean Worktree Strategy Operator Decision Checklist"
grep -q 'operator_decision_checklist_only' "$DOC" \
  || fail "architecture note must document operator_decision_checklist_only"
grep -q 'decision recording, approval acceptance, evidence recording, git mutation, cleanup, delete, strategy application, release, canary activation, and live execution remain blocked' "$DOC" \
  || fail "architecture note must document blocked decision and git boundaries"
grep -q 'no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, decision recording, approval acceptance, evidence recording, evidence persistence, packet send, packet persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed Phase 18 boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist"
  and .status == "ready_blocked"
  and .gate == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_gate"
  and .schema_version == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_git_boundary_readback_gate == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_gate"
  and .source_git_boundary_readback_ready == true
  and .source_operator_packet_visible == true
  and .source_operator_packet_sent == false
  and .source_operator_packet_persisted == false
  and .source_readback_persisted == false
  and .source_strategy_applied == false
  and .source_readback_entry_count > 0
  and .lib_export_present == true
  and .inventory_entry_count == (.tracked_change_count + .untracked_change_count)
  and .checklist_scope.checklist_id == "dirty-worktree.release-boundary.clean-worktree-strategy.operator-decision-checklist.v1"
  and .checklist_scope.checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/v1"
  and .checklist_scope.source_git_boundary_readback_route == "readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1"
  and .checklist_scope.checklist_mode == "operator_decision_checklist_only"
  and .checklist_scope.decision_recording_boundary == "blocked"
  and .checklist_scope.git_mutation_boundary == "closed"
  and .checklist_scope.cleanup_boundary == "blocked"
  and .checklist_scope.evidence_boundary == "blocked"
  and .checklist_entry_count == .source_readback_entry_count
  and .stable_checklist_key_count == .checklist_entry_count
  and .checklist_route_count == .checklist_entry_count
  and .checklist_ready_count == .checklist_entry_count
  and .packet_visible_unsent_unpersisted_count == .checklist_entry_count
  and .git_mutation_blocked_count == .checklist_entry_count
  and .cleanup_delete_blocked_count == .checklist_entry_count
  and .strategy_application_blocked_count == .checklist_entry_count
  and .operator_decision_required_count == .checklist_entry_count
  and .pending_operator_decision_count == .checklist_entry_count
  and .evidence_requirement_count == .checklist_entry_count
  and .evidence_recorded_count == 0
  and .decision_checklist_visible == true
  and .decision_checklist_persisted == false
  and .decision_recorded == false
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
  and .evidence_recording_allowed == false
  and .evidence_persistence_allowed == false
  and .approval_request_sent == false
  and .approval_acceptance_allowed == false
  and .blocker_waiver_allowed == false
  and .package_or_release_allowed == false
  and .public_ga_allowed == false
  and .canary_activation_allowed == false
  and .live_activation_allowed == false
  and .live_execution_allowed == false
  and .operator_decision_checklist_ready == true
  and (.entries | length) == .checklist_entry_count
  and any(.entries[]; .group_type == "top_level")
  and any(.entries[]; .group_type == "scope")
  and (.entries | all(
    (.source_git_boundary_readback_key | startswith("dirty_worktree.packet.git_boundary."))
    and (.source_git_boundary_readback_route | startswith("readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/"))
    and (.checklist_key | startswith("dirty_worktree.decision_checklist."))
    and (.checklist_route | startswith("checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/"))
    and (.decision_checkpoint | startswith("operator_decision_checkpoint."))
    and .source_entry_count > 0
    and .source_entry_count == (.tracked_count + .untracked_count)
    and .decision_state == "pending_operator_decision"
    and .checklist_state == "ready_blocked_pending_operator_decision"
    and .packet_visible == true
    and .packet_unsent == true
    and .packet_unpersisted == true
    and .readback_unpersisted == true
    and .operator_visible == true
    and .queryable == true
    and .diffable == true
    and .operator_decision_required == true
    and .decision_recording_allowed == false
    and .approval_acceptance_allowed == false
    and .evidence_recording_allowed == false
    and .git_add_blocked == true
    and .git_index_mutation_blocked == true
    and .git_commit_blocked == true
    and .git_push_blocked == true
    and .git_reset_blocked == true
    and .git_checkout_blocked == true
    and .git_revert_blocked == true
    and .cleanup_blocked == true
    and .delete_blocked == true
    and .strategy_application_blocked == true
    and .release_cutover_allowed == false
    and .canary_activation_allowed == false
    and .live_execution_allowed == false))
  and any(.entries[]; .source_bucket == "artifacts" and .checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/artifacts")
  and any(.entries[]; .source_bucket == "scripts" and .checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/scripts")
  and any(.entries[]; .source_bucket == "codex-rs" and .checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/top-level/codex-rs")
  and any(.entries[]; .source_bucket == "hepta_systems_owned" and .checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/scope/hepta-systems-owned")
  and any(.entries[]; .source_bucket == "cross_lane_or_unowned" and .checklist_route == "checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/scope/cross-lane-or-unowned")
  and (.blockers | index("operator_decision_recording_blocked")) != null
  and (.blockers | index("approval_acceptance_blocked")) != null
  and (.blockers | index("evidence_recording_blocked")) != null
  and (.blockers | index("git_add_blocked")) != null
  and (.blockers | index("git_index_mutation_blocked")) != null
  and (.blockers | index("git_commit_blocked")) != null
  and (.blockers | index("git_push_blocked")) != null
  and (.blockers | index("git_reset_blocked")) != null
  and (.blockers | index("git_checkout_blocked")) != null
  and (.blockers | index("git_revert_blocked")) != null
  and (.blockers | index("cleanup_and_delete_blocked")) != null
  and (.blockers | index("strategy_application_blocked")) != null
  and (.blockers | index("operator_packet_send_blocked")) != null
  and (.blockers | index("operator_packet_persistence_blocked")) != null
  and (.blockers | index("readback_persistence_blocked")) != null
  and (.blockers | index("release_cutover_blocked")) != null
  and (.blockers | index("canary_activation_blocked")) != null
  and (.blockers | index("live_activation_blocked")) != null
  and (.next_actions | index("phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation")) != null
  and .next_migration_step == "phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$GIT_BOUNDARY_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback"
  and .status == "ready_blocked"
  and .git_mutation_boundary_readback_ready == true
  and .readback_entry_count > 0
  and .operator_packet_visible == true
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .readback_persisted == false
  and .strategy_applied == false
  and .git_index_mutated == false
  and .cleanup_allowed == false
  and .delete_allowed == false
  and .release_cutover_allowed == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist --lib
)

printf 'hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-gate: PASS: clean-worktree strategy operator decisions remain checklist-only with git mutation, cleanup, evidence, approval, release, canary, and live blocked\n'
