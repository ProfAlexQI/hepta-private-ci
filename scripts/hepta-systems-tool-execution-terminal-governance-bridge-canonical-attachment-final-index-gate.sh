#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal governance bridge canonical attachment final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable terminal governance bridge canonical attachment readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing terminal governance bridge canonical attachment final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal governance bridge canonical attachment final index report"
fi

grep -q 'Terminal Governance Bridge Canonical Attachment Final Index' "$DOC" \
  || fail "architecture note must document Terminal Governance Bridge Canonical Attachment Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document the ready-but-blocked final index"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the final index does not invoke live gates or aliases"
grep -q '17-blocker closure categories' "$DOC" \
  || fail "architecture note must document 17-blocker closure categories"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_terminal_governance_bridge_canonical_attachment_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_bridge_canonical_attachment_readback_surface == "tool_execution_terminal_governance_bridge_canonical_attachment_readback"
  and .source_bridge_canonical_attachment_readback_ready == true
  and .terminal_governance_bridge_canonical_attachment_final_index_ready == true
  and .terminal_governance_bridge_canonical_attachment_final_index_blocked == true
  and .bridge_source_count == 2
  and .tool_execution_closure_attached == true
  and .source_closure_blocker_count == 17
  and .source_closure_blocker_category_count == 4
  and .source_closure_blocker_category_ready_count == 4
  and .source_closure_blocker_category_blocker_count == 17
  and .source_closure_blocker_categorization_ready == true
  and (.source_closure_blocker_categories | length) == 4
  and any(.source_closure_blocker_categories[]; .id == "approval_control" and .blocker_count == 4)
  and any(.source_closure_blocker_categories[]; .id == "execution_and_receipts" and .blocker_count == 9)
  and any(.source_closure_blocker_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_closure_blocker_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .current_canonical_governance_terminal_index_attached == true
  and .terminal_source_probe_count == 4
  and .terminal_source_probe_ready_count == 4
  and .source_active_current_canonical_consumer_surface == "current_canonical_consumer"
  and .source_successor_cutover_final_gate_attached == true
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .final_blocker_count == 10
  and (.final_blockers | index("manual_operator_live_cutover_approval_required")) != null
  and (.final_blockers | index("canonical_successor_consumer_cutover_disallowed")) != null
  and (.final_blockers | index("terminal_live_gates_not_invoked")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "attach_terminal_governance_bridge_canonical_attachment_final_index_to_next_terminal_denial_summary_without_live_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-gate: PASS: terminal governance bridge canonical attachment final index is ready but blocked without live invocation\n'
