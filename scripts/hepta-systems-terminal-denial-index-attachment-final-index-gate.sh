#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-denial-index-attachment-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-terminal-denial-index-attachment-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_INDEX_ATTACHMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-denial-index-attachment-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal denial index attachment final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable terminal denial index attachment readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing terminal denial index attachment final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal denial index attachment final index report"
fi

grep -q 'Terminal Denial Index Attachment Final Index' "$DOC" \
  || fail "architecture note must document Terminal Denial Index Attachment Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke terminal denial or live gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_denial_index_attachment_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_denial_index_attachment_readback_surface == "terminal_denial_index_attachment_readback"
  and .source_terminal_denial_index_attachment_readback_ready == true
  and .source_terminal_denial_index_attachment_readback_blocked == true
  and .terminal_denial_index_attachment_final_index_ready == true
  and .terminal_denial_index_attachment_final_index_blocked == true
  and .terminal_denial_index_attachment_readback_attached == true
  and .terminal_release_governance_safe_chain_closure_final_index_attached == true
  and .terminal_denial_index_gate_present == true
  and .terminal_denial_index_doc_present == true
  and .terminal_denial_index_gate_invoked == false
  and .terminal_denial_index_recorded == false
  and .terminal_denial_index_persisted == false
  and .terminal_denial_index_materialized == false
  and .terminal_denial_index_filesystem_written == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .final_blocker_count == 30
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .public_release_claim_allowed == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_terminal_denial_index_attachment_final_index_to_terminal_publication_evidence_non_persistence_summary_without_terminal_denial_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-terminal-denial-index-attachment-final-index-gate: PASS: terminal denial index attachment final index is ready but blocked without terminal denial/live invocation\n'
