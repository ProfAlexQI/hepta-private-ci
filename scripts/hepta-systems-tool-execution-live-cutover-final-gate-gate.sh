#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-final-gate-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_FINAL_GATE_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-final-gate-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable execution live cutover final gate report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable execution live cutover receipt rollback packet gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing execution live cutover final gate architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the execution live cutover final gate report"
fi

grep -q 'Live Cutover Final Gate' "$DOC" \
  || fail "architecture note must document Live Cutover Final Gate"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'explicit live cutover approval' "$DOC" \
  || fail "architecture note must document explicit live cutover approval"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_live_cutover_final_gate"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_live_cutover_receipt_rollback_packet_surface == "tool_execution_live_cutover_receipt_rollback_packet"
  and .source_live_cutover_receipt_rollback_packet_ready == true
  and .source_live_cutover_start_allowed == false
  and .source_live_cutover_rollback_allowed == false
  and .source_live_cutover_result_receipt_write_allowed == false
  and .source_live_cutover_allowed == false
  and .final_gate_policy_present == true
  and .final_cutover_ticket_present == true
  and .final_operator_readback_required == true
  and .explicit_live_cutover_approval_present == false
  and .approval_request_sent == false
  and .operator_cutover_decision_receipt_written == false
  and .operator_cutover_readback_evidence_written == false
  and .operator_cutover_acceptance_recorded == false
  and .live_cutover_switch_enabled == false
  and .adapter_dispatch_switch_enabled == false
  and .tool_invocation_execution_switch_enabled == false
  and .live_cutover_started == false
  and .rollback_executed == false
  and .rollback_receipt_written == false
  and .result_receipt_written == false
  and .candidate_count == 2
  and .live_cutover_final_gate_ready_count == 2
  and .live_cutover_final_gate_blocked_count == 0
  and .explicit_live_cutover_approval_required_count == 1
  and .explicit_live_cutover_approval_missing_count == 1
  and .final_operator_readback_required_count == 1
  and .live_cutover_blocked_count == 1
  and .approval_request_blocked_count == 1
  and .operator_acceptance_blocked_count == 1
  and .execution_switch_blocked_count == 1
  and .rollback_execution_blocked_count == 1
  and .result_receipt_write_blocked_count == 1
  and .selected_status_canary_count == 1
  and .preflight_only_non_selected_count == 1
  and .all_receipt_rollback_packets_bound_to_final_gate == true
  and .all_live_cutover_final_gate_entries_keep_no_invocation_guard == true
  and .tool_execution_live_cutover_final_gate_ready == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .execution_adapter_dispatched == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .live_mutation_ready == false
  and .next_migration_step == "manual_operator_live_cutover_approval_required"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .execution_adapter_kind == "mcp_tool_call_adapter" and .selected_for_status_canary == true and .preflight_only_non_selected_candidate == false and .source_live_cutover_receipt_rollback_packet_route == "live_cutover_receipt_rollback_packet_ready" and .registry_guard_route == "require_approval_ledger" and .live_cutover_final_gate_route == "live_cutover_final_gate_ready_blocked" and .live_cutover_final_gate_ready == true and .final_operator_readback_required == true and .explicit_live_cutover_approval_required == true and .explicit_live_cutover_approval_present == false and .live_cutover_blocked == true and .approval_request_blocked == true and .operator_acceptance_blocked == true and .execution_switch_blocked == true and .rollback_execution_blocked == true and .result_receipt_write_blocked == true and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .execution_adapter_kind == "app_connector_invocation_adapter" and .selected_for_status_canary == false and .preflight_only_non_selected_candidate == true and .source_live_cutover_receipt_rollback_packet_route == "preflight_only_non_selected_candidate" and .registry_guard_route == "require_approval_ledger" and .live_cutover_final_gate_route == "preflight_only_non_selected_candidate" and .live_cutover_final_gate_ready == true and .final_operator_readback_required == false and .explicit_live_cutover_approval_required == false and .explicit_live_cutover_approval_present == false and .live_cutover_blocked == false and .approval_request_blocked == false and .operator_acceptance_blocked == false and .execution_switch_blocked == false and .rollback_execution_blocked == false and .result_receipt_write_blocked == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and (.blockers | index("explicit_live_cutover_approval_missing")) != null
  and (.blockers | index("live_cutover_blocked")) != null
  and (.next_actions | index("manual_operator_live_cutover_approval_required")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_execution_live_cutover_final_gate --quiet
)

printf 'hepta-systems-tool-execution-live-cutover-final-gate-gate: PASS: final live cutover gate is ready while explicit approval, dispatch, invocation, and mutation stay disabled\n'
