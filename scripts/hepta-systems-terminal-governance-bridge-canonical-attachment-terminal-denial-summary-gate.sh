#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-report.sh"
FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_TERMINAL_DENIAL_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal denial summary attachment report: $REPORT"
[[ -x "$FINAL_INDEX_GATE" ]] || fail "missing executable terminal governance bridge canonical attachment final index gate: $FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing terminal denial summary attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal denial summary attachment report"
fi

grep -q 'Terminal Denial Summary Attachment' "$DOC" \
  || fail "architecture note must document Terminal Denial Summary Attachment"
grep -q 'source-probes terminal denial summary entrypoints' "$DOC" \
  || fail "architecture note must document source-probing terminal denial summary entrypoints"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_governance_bridge_canonical_attachment_terminal_denial_summary"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_bridge_canonical_attachment_final_index_surface == "tool_execution_terminal_governance_bridge_canonical_attachment_final_index"
  and .source_bridge_canonical_attachment_final_index_ready == true
  and .source_bridge_canonical_attachment_final_index_blocked == true
  and .terminal_denial_summary_attachment_ready == true
  and .terminal_denial_summary_attachment_blocked == true
  and .terminal_summary_source_probe_count == 4
  and .terminal_summary_source_probe_ready_count == 4
  and (.terminal_summary_sources | length) == 4
  and (.terminal_summary_sources | all(.script_present == true and .doc_present == true and .invoked == false))
  and .bridge_source_count == 2
  and .tool_execution_closure_attached == true
  and .current_canonical_governance_terminal_index_attached == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .source_active_current_canonical_consumer_surface == "current_canonical_consumer"
  and .source_successor_cutover_final_gate_attached == true
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .attachment_blocker_count == 10
  and (.attachment_blockers | index("manual_operator_live_cutover_approval_required")) != null
  and (.attachment_blockers | index("terminal_summary_gates_source_probed_not_invoked")) != null
  and (.attachment_blockers | index("terminal_live_gates_not_invoked")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "derive_terminal_governance_bridge_canonical_attachment_terminal_denial_summary_readback_without_live_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$FINAL_INDEX_GATE" >/dev/null

printf 'hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-gate: PASS: terminal denial summary attachment source-probes terminal gates without live invocation\n'
