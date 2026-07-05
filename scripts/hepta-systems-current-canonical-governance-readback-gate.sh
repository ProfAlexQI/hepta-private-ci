#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-current-canonical-governance-readback-report.sh"
GOVERNANCE_REPORT="$ROOT/scripts/hepta-systems-current-canonical-governance-report.sh"
CLOSURE_INDEX_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_GOVERNANCE_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-governance-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable current canonical governance readback report: $REPORT"
[[ -x "$GOVERNANCE_REPORT" ]] || fail "missing executable current canonical governance report: $GOVERNANCE_REPORT"
[[ -x "$CLOSURE_INDEX_GATE" ]] || fail "missing executable live cutover closure index gate: $CLOSURE_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing current canonical governance readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the current canonical governance readback report"
fi

grep -q 'Current Canonical Governance Readback' "$DOC" \
  || fail "architecture note must document Current Canonical Governance Readback"
grep -q 'verified governance report snapshot' "$DOC" \
  || fail "architecture note must document the verified governance report snapshot basis"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke the alias"
grep -q 'tool execution closure backfeed' "$DOC" \
  || fail "architecture note must document tool execution closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "current_canonical_governance_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .readback_mode == "static_governance_snapshot_readback_only"
  and .source_current_canonical_governance_surface == "current_canonical_governance"
  and .source_current_canonical_governance_basis == "verified_governance_report_snapshot"
  and .source_current_canonical_governance_report_reexecuted == false
  and .source_current_canonical_governance_ready == true
  and .source_current_canonical_governance_blocked == true
  and .source_tool_execution_closure_surface == "tool_execution_live_cutover_closure_index"
  and .source_tool_execution_closure_ready == true
  and .tool_execution_closure_backfeed_ready == true
  and .tool_execution_closure_backfeed_blocker_count == 17
  and .tool_execution_closure_backfeed_category_count == 4
  and .tool_execution_closure_backfeed_category_ready_count == 4
  and .tool_execution_closure_backfeed_category_blocker_count == 17
  and .tool_execution_closure_backfeed_categorization_ready == true
  and (.tool_execution_closure_backfeed_categories | length) == 4
  and any(.tool_execution_closure_backfeed_categories[]; .id == "approval_control" and .blocker_count == 4)
  and any(.tool_execution_closure_backfeed_categories[]; .id == "execution_and_receipts" and .blocker_count == 9)
  and any(.tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .current_canonical_governance_readback_ready == true
  and .current_canonical_governance_readback_blocked == true
  and .readback_check_count == 8
  and (.readback_checks | all(.observed == .expected))
  and .active_current_canonical_consumer_surface == "current_canonical_consumer"
  and .active_current_canonical_consumer_replaced_in_place == false
  and .successor_canonical_consumer_surface == "promoted_current_canonical_consumer"
  and .successor_cutover_final_gate_attached == true
  and .successor_cutover_final_gate_status == "ready_blocked"
  and .successor_consumer_cutover_allowed == false
  and .rollback_anchor == "current_canonical_consumer"
  and .manual_operator_live_cutover_approval_required == true
  and .explicit_live_cutover_approval_present == false
  and .operator_live_cutover_approval_recorded == false
  and .cutover_packet_recorded == false
  and .cutover_packet_accepted == false
  and .final_blocker_count == 14
  and .governance_blocker_count == 13
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "derive_current_canonical_governance_terminal_index_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$CLOSURE_INDEX_GATE" >/dev/null

printf 'hepta-systems-current-canonical-governance-readback-gate: PASS: current canonical governance readback is static and non-authorizing\n'
