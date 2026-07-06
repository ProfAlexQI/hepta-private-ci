#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-governance-closure-summary-final-index-terminal-release-governance-report.sh"
CLOSURE_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-governance-closure-summary-attachment-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_FINAL_INDEX_TERMINAL_RELEASE_GOVERNANCE_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-governance-closure-summary-final-index-terminal-release-governance-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal release governance attachment report: $REPORT"
[[ -x "$CLOSURE_FINAL_INDEX_GATE" ]] || fail "missing executable terminal governance closure summary attachment final index gate: $CLOSURE_FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing terminal release governance attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal release governance attachment report"
fi

grep -q 'Terminal Release Governance Attachment' "$DOC" \
  || fail "architecture note must document Terminal Release Governance Attachment"
grep -q 'source-probes the terminal release governance final audit index gate' "$DOC" \
  || fail "architecture note must document source-probing the terminal release governance final audit index gate"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke release or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_governance_closure_summary_final_index_terminal_release_governance"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_governance_closure_summary_attachment_final_index_surface == "terminal_governance_closure_summary_attachment_final_index"
  and .source_terminal_governance_closure_summary_attachment_final_index_ready == true
  and .source_terminal_governance_closure_summary_attachment_final_index_blocked == true
  and .terminal_release_governance_attachment_ready == true
  and .terminal_release_governance_attachment_blocked == true
  and .terminal_governance_closure_summary_attachment_final_index_attached == true
  and .terminal_release_governance_final_audit_gate_present == true
  and .terminal_release_governance_final_audit_doc_present == true
  and .terminal_release_governance_final_audit_gate_invoked == false
  and .terminal_governance_closure_summary_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .attachment_blocker_count == 13
  and (.attachment_blockers | index("terminal_release_governance_final_audit_source_probed_not_invoked")) != null
  and (.attachment_blockers | index("release_publication_disabled")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .release_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .next_migration_step == "derive_terminal_release_governance_attachment_readback_without_release_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$CLOSURE_FINAL_INDEX_GATE" >/dev/null

printf 'hepta-systems-terminal-governance-closure-summary-final-index-terminal-release-governance-gate: PASS: terminal release governance attachment source-probes release final audit without live invocation\n'
