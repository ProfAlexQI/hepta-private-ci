#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-operator-approval-packet-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-execution-cutover-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_OPERATOR_APPROVAL_PACKET_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-operator-approval-packet-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable execution operator approval packet report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable execution cutover preflight gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing execution operator approval packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the execution operator approval packet report"
fi

grep -q 'Operator Approval Packet' "$DOC" \
  || fail "architecture note must document Operator Approval Packet"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'approval request' "$DOC" \
  || fail "architecture note must document blocked approval request"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_operator_approval_packet"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_cutover_preflight_surface == "tool_execution_cutover_preflight"
  and .source_cutover_preflight_ready == true
  and .source_live_cutover_allowed == false
  and .operator_packet_template_present == true
  and .operator_session_binding_present == true
  and .approval_request_sent == false
  and .operator_approval_record_written == false
  and .operator_acceptance_present == false
  and .live_cutover_switch_enabled == false
  and .candidate_count == 2
  and .operator_approval_packet_ready_count == 2
  and .operator_approval_packet_blocked_count == 0
  and .operator_review_required_count == 2
  and .approval_request_blocked_count == 2
  and .all_cutover_preflight_entries_bound_to_operator_packet == true
  and .all_operator_packets_keep_approval_guard == true
  and .tool_execution_operator_approval_packet_ready == true
  and .tool_execution_operator_approval_request_allowed == false
  and .tool_execution_live_cutover_allowed == false
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .execution_adapter_dispatched == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .result_receipt_written == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_execution_operator_approval_decision_preflight_without_invocation"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .execution_adapter_kind == "mcp_tool_call_adapter" and .source_cutover_preflight_route == "cutover_preflight_blocked_until_explicit_approval" and .registry_guard_route == "require_approval_ledger" and .operator_approval_packet_route == "operator_approval_packet_ready_for_review" and .operator_approval_packet_ready == true and .operator_review_required == true and .approval_request_blocked == true and .approval_request_sent == false and .operator_approval_record_written == false and .operator_acceptance_present == false and .live_cutover_switch_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .execution_adapter_kind == "app_connector_invocation_adapter" and .source_cutover_preflight_route == "cutover_preflight_blocked_until_explicit_approval" and .registry_guard_route == "require_approval_ledger" and .operator_approval_packet_route == "operator_approval_packet_ready_for_review" and .operator_approval_packet_ready == true and .operator_review_required == true and .approval_request_blocked == true and .approval_request_sent == false and .operator_approval_record_written == false and .operator_acceptance_present == false and .live_cutover_switch_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and (.blockers | index("approval_request_not_sent")) != null
  and (.blockers | index("operator_approval_record_not_written")) != null
  and (.next_actions | index("restore_tool_execution_operator_approval_decision_preflight_without_invocation")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_execution_operator_approval_packet --quiet
)

printf 'hepta-systems-tool-execution-operator-approval-packet-gate: PASS: operator approval packets are ready for review while approval requests and invocation remain disabled\n'
