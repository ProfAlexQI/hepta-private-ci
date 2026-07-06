#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-public-ga-non-promotion-summary-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-terminal-public-ga-non-promotion-summary-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_GA_NON_PROMOTION_SUMMARY_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-ga-non-promotion-summary-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal Public GA non-promotion summary final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable terminal Public GA non-promotion summary readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing terminal Public GA non-promotion summary final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal Public GA non-promotion summary final index report"
fi

grep -q 'Terminal Public GA Non-Promotion Summary Final Index' "$DOC" \
  || fail "architecture note must document Terminal Public GA Non-Promotion Summary Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke Public GA readiness"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_public_ga_non_promotion_summary_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_public_ga_non_promotion_summary_readback_surface == "terminal_public_ga_non_promotion_summary_readback"
  and .source_terminal_public_ga_non_promotion_summary_readback_ready == true
  and .source_terminal_public_ga_non_promotion_summary_readback_blocked == true
  and .source_terminal_denial_index_attachment_final_index_surface == "terminal_denial_index_attachment_final_index"
  and .source_terminal_denial_index_attachment_final_index_ready == true
  and .source_terminal_denial_index_attachment_final_index_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .terminal_public_ga_non_promotion_summary_final_index_ready == true
  and .terminal_public_ga_non_promotion_summary_final_index_blocked == true
  and .terminal_public_ga_non_promotion_summary_readback_attached == true
  and .public_ga_readiness_non_live_attachment_final_index_attached == true
  and .public_claim_non_promotion_denial_gate_present == true
  and .public_claim_non_promotion_denial_gate_invoked == false
  and .public_ga_operator_approval_packet_present == true
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_live_endpoint_read_performed == false
  and .public_ga_readiness_script_invoked == false
  and .public_ga_readiness_live_endpoint_read_performed == false
  and .public_ga_readiness_endpoint_curl_performed == false
  and .public_ga_readiness_report_materialized == false
  and .public_ga_readiness_attachment_recorded == false
  and .public_ga_readiness_attachment_allowed == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .final_blocker_count == 22
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_terminal_public_ga_non_promotion_summary_final_index_to_public_ga_operator_packet_non_send_readback_without_packet_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-terminal-public-ga-non-promotion-summary-final-index-gate: PASS: terminal Public GA non-promotion summary final index is ready but blocked\n'
