#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-report.sh"
FINAL_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh"
OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_CLOSURE_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-closure-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable live cutover closure index report: $REPORT"
[[ -x "$FINAL_GATE" ]] || fail "missing executable live cutover final gate: $FINAL_GATE"
[[ -x "$OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE" ]] || fail "missing executable owner/freeze release blocker gate: $OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE"
[[ -f "$DOC" ]] || fail "missing live cutover closure index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the live cutover closure index report"
fi

grep -q 'Live Cutover Closure Index' "$DOC" \
  || fail "architecture note must document Live Cutover Closure Index"
grep -q 'manual operator approval' "$DOC" \
  || fail "architecture note must document manual operator approval"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'runner dry-run selector' "$DOC" \
  || fail "architecture note must document runner dry-run selector"
grep -q 'dirty-worktree owner/freeze release blocker' "$DOC" \
  || fail "architecture note must document dirty-worktree owner/freeze release blocker"
grep -q '17-blocker category readback' "$DOC" \
  || fail "architecture note must document 17-blocker category readback"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_live_cutover_closure_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_final_gate_surface == "tool_execution_live_cutover_final_gate"
  and .source_final_gate_ready == true
  and .source_live_cutover_allowed == false
  and .source_public_ga_allowed == false
  and .covered_surface_count == 24
  and (.covered_surfaces | length) == .covered_surface_count
  and (.covered_surfaces | index("tool_execution_live_cutover_final_gate")) != null
  and (.covered_surfaces | index("status_canary_runner_dry_run_selector")) != null
  and (.covered_surfaces | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback")) != null
  and .candidate_count == 2
  and .closure_candidate_count == 2
  and .final_gate_ready_count == 2
  and .explicit_live_cutover_approval_required_count == 1
  and .explicit_live_cutover_approval_missing_count == 1
  and .live_cutover_blocked_count == 1
  and .approval_request_blocked_count == 1
  and .operator_acceptance_blocked_count == 1
  and .execution_switch_blocked_count == 1
  and .rollback_execution_blocked_count == 1
  and .result_receipt_write_blocked_count == 1
  and .selected_status_canary_count == 1
  and .preflight_only_non_selected_count == 1
  and .source_runner_dry_run_selector_present == true
  and .source_runner_dry_run_selector_id == "status-canary-runner-dry-run-selector/hepta-system-status/v1"
  and .source_runner_dry_run_selector_route == "status_canary_runner_dry_run_selector_blocked_no_selector_request"
  and .source_runner_dry_run_selector_request_present == false
  and .source_runner_dry_run_selector_binding_guard_bound == true
  and .source_runner_dry_run_selector_start_guard_reason_audit_ready == true
  and .source_runner_dry_run_selector_binding_guard_allowed == false
  and .source_runner_dry_run_selector_blocked == true
  and .source_runner_dry_run_selector_allowed == false
  and .runner_preflight_selector_classification_ready == true
  and .runner_preflight_selector_release_blocker_classification == "blocked_runner_dry_run_selector_no_request"
  and .concrete_runner_preflight_selector_fail_closed == true
  and .source_dirty_worktree_owner_freeze_surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording"
  and .source_dirty_worktree_owner_freeze_ready == true
  and .dirty_worktree_owner_freeze_release_blocker_ready == true
  and .dirty_worktree_owner_freeze_boundary_entry_count == 7
  and .dirty_worktree_owner_freeze_pending_operator_decision_count == 7
  and .dirty_worktree_owner_freeze_evidence_recording_blocked_count == 7
  and .dirty_worktree_owner_freeze_evidence_recorded_count == 0
  and .dirty_worktree_owner_freeze_release_cutover_allowed == false
  and .dirty_worktree_owner_freeze_canary_activation_allowed == false
  and .dirty_worktree_owner_freeze_live_execution_allowed == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_closure_index_ready == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "manual_operator_live_cutover_approval_required"
  and .closure_blocker_count == 17
  and .closure_blocker_category_count == 4
  and .closure_blocker_category_ready_count == 4
  and .closure_blocker_category_blocker_count == 17
  and .closure_blocker_categorization_ready == true
  and (.closure_blockers | index("explicit_live_cutover_approval_missing")) != null
  and (.closure_blockers | index("manual_operator_live_cutover_approval_required") == null)
  and (.closure_blockers | index("public_ga_disabled")) != null
  and (.closure_blockers | index("runner_dry_run_selector_no_request")) != null
  and (.closure_blockers | index("concrete_runner_preflight_selector_fail_closed")) != null
  and (.closure_blockers | index("dirty_worktree_owner_freeze_operator_decision_pending")) != null
  and (.closure_blockers | index("dirty_worktree_owner_freeze_evidence_recording_blocked")) != null
  and (.closure_blocker_categories | length) == 4
  and any(.closure_blocker_categories[]; .id == "approval_control" and .blocker_count == 4 and (.blocker_ids | index("approval_request_not_sent")) != null and .release_cutover_allowed == false and .side_effect_free == true)
  and any(.closure_blocker_categories[]; .id == "execution_and_receipts" and .blocker_count == 9 and (.blocker_ids | index("tool_invocation_ledger_write_disabled")) != null and .live_execution_allowed == false and .side_effect_free == true)
  and any(.closure_blocker_categories[]; .id == "runner_selector" and .blocker_count == 2 and (.blocker_ids | index("runner_dry_run_selector_no_request")) != null and .canary_activation_allowed == false and .side_effect_free == true)
  and any(.closure_blocker_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2 and (.blocker_ids | index("dirty_worktree_owner_freeze_evidence_recording_blocked")) != null and .evidence_recording_allowed == false and .side_effect_free == true)
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .selected_for_status_canary == true and .preflight_only_non_selected_candidate == false and .final_gate_route == "live_cutover_final_gate_ready_blocked" and .final_operator_readback_required == true and .explicit_live_cutover_approval_required == true and .explicit_live_cutover_approval_present == false and .live_cutover_blocked == true and .runner_preflight_selector_classification == "blocked_runner_dry_run_selector_no_request" and .runner_preflight_selector_release_blocker == true and .concrete_runner_preflight_selector_fail_closed == true and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .selected_for_status_canary == false and .preflight_only_non_selected_candidate == true and .final_gate_route == "preflight_only_non_selected_candidate" and .final_operator_readback_required == false and .explicit_live_cutover_approval_required == false and .explicit_live_cutover_approval_present == false and .live_cutover_blocked == false and .runner_preflight_selector_classification == "preflight_only_non_selected_candidate" and .runner_preflight_selector_release_blocker == false and .concrete_runner_preflight_selector_fail_closed == true and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$FINAL_GATE" >/dev/null
"$OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE" >/dev/null

printf 'hepta-systems-tool-execution-live-cutover-closure-index-gate: PASS: live cutover closure index stops at manual operator approval with invocation and mutation disabled\n'
