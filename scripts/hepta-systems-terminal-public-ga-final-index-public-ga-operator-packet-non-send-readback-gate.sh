#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-public-ga-final-index-public-ga-operator-packet-non-send-readback-report.sh"
TERMINAL_PUBLIC_GA_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-public-ga-non-promotion-summary-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_GA_FINAL_INDEX_PUBLIC_GA_OPERATOR_PACKET_NON_SEND_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-ga-final-index-public-ga-operator-packet-non-send-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator packet non-send readback report: $REPORT"
[[ -x "$TERMINAL_PUBLIC_GA_FINAL_INDEX_GATE" ]] || fail "missing executable terminal Public GA non-promotion summary final index gate: $TERMINAL_PUBLIC_GA_FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator packet non-send readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator packet non-send readback report"
fi

grep -q 'Public GA Operator Packet Non-Send Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Packet Non-Send Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke the Public GA operator packet"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_packet_non_send_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_public_ga_non_promotion_summary_final_index_surface == "terminal_public_ga_non_promotion_summary_final_index"
  and .source_terminal_public_ga_non_promotion_summary_final_index_ready == true
  and .source_terminal_public_ga_non_promotion_summary_final_index_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_packet_non_send_readback_ready == true
  and .public_ga_operator_packet_non_send_readback_blocked == true
  and .terminal_public_ga_non_promotion_summary_final_index_attached == true
  and .public_ga_operator_approval_packet_present == true
  and .public_ga_operator_compat_wrapper_present == true
  and .public_ga_operator_approval_packet_doc_present == true
  and .public_ga_operator_packet_target_curl_count == 2
  and .public_ga_operator_packet_target_endpoint_count == 2
  and .public_ga_operator_packet_required_approval_static_count == 8
  and .public_ga_operator_compat_wrapper_exec_count == 1
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_compat_wrapper_invoked == false
  and .public_ga_operator_packet_live_endpoint_read_performed == false
  and .public_ga_operator_packet_endpoint_curl_performed == false
  and .public_ga_operator_packet_report_materialized == false
  and .public_ga_operator_packet_sent == false
  and .public_ga_operator_packet_recorded == false
  and .public_ga_operator_packet_accepted == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .public_ga_readiness_script_invoked == false
  and .public_ga_readiness_live_endpoint_read_performed == false
  and .public_claim_non_promotion_denial_gate_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .readback_blocker_count == 18
  and (.readback_blockers | index("public_ga_operator_packet_send_blocked")) != null
  and (.readback_blockers | index("operator_approval_request_send_blocked")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_packet_non_send_static_readback_without_packet_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$TERMINAL_PUBLIC_GA_FINAL_INDEX_GATE" >/dev/null

printf 'hepta-systems-terminal-public-ga-final-index-public-ga-operator-packet-non-send-readback-gate: PASS: Public GA operator packet non-send readback is ready but blocked without packet invocation\n'
