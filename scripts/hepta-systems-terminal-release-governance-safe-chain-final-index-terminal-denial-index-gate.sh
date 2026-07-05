#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-release-governance-safe-chain-final-index-terminal-denial-index-report.sh"
SAFE_CHAIN_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-release-governance-safe-chain-closure-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_GOVERNANCE_SAFE_CHAIN_FINAL_INDEX_TERMINAL_DENIAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-release-governance-safe-chain-final-index-terminal-denial-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal denial index attachment report: $REPORT"
[[ -x "$SAFE_CHAIN_FINAL_INDEX_GATE" ]] || fail "missing executable safe chain closure final index gate: $SAFE_CHAIN_FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing terminal denial index attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal denial index attachment report"
fi

grep -q 'Terminal Release Governance Safe Chain Final Index To Terminal Denial Index' "$DOC" \
  || fail "architecture note must document Terminal Release Governance Safe Chain Final Index To Terminal Denial Index"
grep -q 'source-probe' "$DOC" \
  || fail "architecture note must document source-probe mode"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke terminal denial or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_release_governance_safe_chain_final_index_terminal_denial_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_release_governance_safe_chain_closure_final_index_surface == "terminal_release_governance_safe_chain_closure_final_index"
  and .source_terminal_release_governance_safe_chain_closure_final_index_ready == true
  and .source_terminal_release_governance_safe_chain_closure_final_index_blocked == true
  and .terminal_denial_index_attachment_ready == true
  and .terminal_denial_index_attachment_blocked == true
  and .terminal_release_governance_safe_chain_closure_final_index_attached == true
  and .terminal_denial_index_gate_present == true
  and .terminal_denial_index_doc_present == true
  and .terminal_denial_index_gate_invoked == false
  and .terminal_denial_index_recorded == false
  and .terminal_denial_index_persisted == false
  and .terminal_denial_index_materialized == false
  and .terminal_denial_index_filesystem_written == false
  and .terminal_release_governance_final_audit_gate_invoked == false
  and .terminal_release_artifact_non_write_lock_gate_invoked == false
  and .terminal_public_distribution_non_publication_lock_gate_invoked == false
  and .terminal_non_activation_release_claim_index_gate_invoked == false
  and .terminal_operator_readiness_non_approval_index_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .attachment_blocker_count == 30
  and (.attachment_blockers | index("terminal_denial_index_not_invoked")) != null
  and (.attachment_blockers | index("terminal_denial_index_filesystem_write_disabled")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .public_release_claim_allowed == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_terminal_denial_index_attachment_readback_without_terminal_denial_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SAFE_CHAIN_FINAL_INDEX_GATE" >/dev/null

printf 'hepta-systems-terminal-release-governance-safe-chain-final-index-terminal-denial-index-gate: PASS: terminal denial index is source-probed without denial/live invocation\n'
