#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-current-reality-capability-matrix-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_REALITY_CAPABILITY_MATRIX_2026-06-27.md"

fail() {
  printf 'hepta-systems-current-reality-capability-matrix-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable current reality capability matrix report: $REPORT"
[[ -f "$DOC" ]] || fail "missing current reality capability matrix architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the current reality capability matrix report"
fi

grep -q 'Current Reality Capability Matrix' "$DOC" \
  || fail "architecture note must document Current Reality Capability Matrix"
grep -q 'memory/filesystem drift' "$DOC" \
  || fail "architecture note must document memory/filesystem drift"
grep -q 'does not open live execution' "$DOC" \
  || fail "architecture note must document no live execution"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "current_reality_capability_matrix"
  and .status == "blocked"
  and .matrix_date == "2026-06-27"
  and .local_capability_count == 104
  and .local_capability_ready_count == 102
  and .local_capability_blocked_count == 2
  and .current_reality_capability_matrix_ready == false
  and .live_enabled_count == 0
  and .all_live_paths_blocked == true
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
  and .plugin_fixture_shape_ready == true
  and .plugin_manifest_present == true
  and .plugin_manifest_summary.skill_path_present == true
  and .plugin_manifest_summary.mcp_servers_path_present == true
  and .plugin_manifest_summary.apps_path_present == true
  and .plugin_manifest_summary.skill_count == 1
  and .plugin_manifest_summary.mcp_server_count == 1
  and .plugin_manifest_summary.app_count == 1
  and .plugin_manifest_summary.tool_schema_count == 2
  and .plugin_manifest_summary.permission_count == 2
  and .plugin_manifest_summary.activation_event_count == 2
  and .plugin_manifest_summary.tool_policy_count == 2
  and .memory_drift_entry_count == 5
  and .missing_memory_checkpoint_count == 0
  and .resolved_memory_checkpoint_count == 5
  and .memory_filesystem_drift_tracked == true
  and .dirty_worktree_boundary_tracked == true
  and .git_status_entry_count >= 11
  and .git_tracked_change_count >= 7
  and .git_untracked_count >= 4
  and (.capabilities | length) == .local_capability_count
  and (.capabilities | all(.live_enabled == false))
  and ([.capabilities[] | select(.ready == true)] | length) == .local_capability_ready_count
  and ([.capabilities[] | select(.ready == false)] | length) == .local_capability_blocked_count
  and all(.capabilities[] | select(.ready == false); .layer == "release_boundary" and .status == "blocked")
  and any(.capabilities[]; .id == "dirty_worktree_release_boundary_release_risk_snapshot" and .ready == false and .status == "blocked" and .live_enabled == false)
  and any(.capabilities[]; .id == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal" and .ready == false and .status == "blocked" and .live_enabled == false)
  and any(.capabilities[]; .id == "plugins_contribution_point_abi" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "plugins_lifecycle_state_machine" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "tools_invocation_source_of_truth" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "tools_read_only_dispatch_preflight" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "workflow_workgraph_durable_identity" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "workflow_current_readback_receipt_tail" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "workflow_temporal_lite_durable_store_adapter" and .ready == true and .live_enabled == false)
  and any(.capabilities[]; .id == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence" and .ready == true and .live_enabled == false)
  and (.blockers | index("dirty_worktree_release_boundary_release_risk_snapshot_blocks_release_cutover_git_mutation_cleanup_evidence_approval_decision_recording_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_blocks_test_probe_git_mutation_cleanup_evidence_approval_decision_recording_and_live")) != null
  and (.blockers | index("controlled_live_cutover_blocked_by_operator_approval_and_evidence")) != null
  and (.blockers | index("dirty_worktree_boundary")) != null
  and (.next_actions | index("close_controlled_live_evidence_before_status_canary_start")) != null
  and .next_migration_step == "close_controlled_live_evidence_before_status_canary_start"
' >/dev/null

printf 'hepta-systems-current-reality-capability-matrix-gate: PASS: current plugins/tools/workflow reality is blocked by dirty release boundary with live paths closed\n'
