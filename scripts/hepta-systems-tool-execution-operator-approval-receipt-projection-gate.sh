#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-execution-operator-approval-packet-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_OPERATOR_APPROVAL_RECEIPT_PROJECTION_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-operator-approval-receipt-projection-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable execution operator approval receipt projection report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable execution operator approval packet gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing execution operator approval receipt projection architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the execution operator approval receipt projection report"
fi

grep -q 'Operator Approval Receipt Projection' "$DOC" \
  || fail "architecture note must document Operator Approval Receipt Projection"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'readback evidence' "$DOC" \
  || fail "architecture note must document readback evidence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_operator_approval_receipt_projection"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_operator_approval_packet_surface == "tool_execution_operator_approval_packet"
  and .source_operator_approval_packet_ready == true
  and .source_approval_request_allowed == false
  and .source_live_cutover_allowed == false
  and .operator_decision_receipt_projection_present == true
  and .operator_decision_readback_evidence_slot_present == true
  and .operator_decision_record_written == false
  and .operator_decision_receipt_written == false
  and .operator_acceptance_present == false
  and .approval_request_sent == false
  and .live_cutover_switch_enabled == false
  and .candidate_count == 2
  and .operator_approval_receipt_projection_ready_count == 2
  and .operator_approval_receipt_projection_blocked_count == 0
  and .operator_decision_receipt_required_count == 2
  and .operator_decision_readback_evidence_required_count == 2
  and .operator_decision_receipt_write_blocked_count == 2
  and .all_operator_packets_bound_to_receipt_projection == true
  and .all_operator_receipt_projections_keep_approval_guard == true
  and .tool_execution_operator_approval_receipt_projection_ready == true
  and .tool_execution_operator_decision_write_allowed == false
  and .tool_execution_live_cutover_allowed == false
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .execution_adapter_dispatched == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .operator_decision_record_written_flag == false
  and .operator_decision_receipt_written_flag == false
  and .result_receipt_written == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_execution_canary_cutover_plan_without_invocation"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .execution_adapter_kind == "mcp_tool_call_adapter" and .source_operator_approval_packet_route == "operator_approval_packet_ready_for_review" and .registry_guard_route == "require_approval_ledger" and .operator_approval_receipt_projection_route == "operator_approval_receipt_projection_ready" and .operator_approval_receipt_projection_ready == true and .operator_decision_receipt_required == true and .operator_decision_readback_evidence_required == true and .operator_decision_receipt_write_blocked == true and .operator_decision_record_written == false and .operator_decision_receipt_written == false and .operator_acceptance_present == false and .approval_request_sent == false and .live_cutover_switch_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .execution_adapter_kind == "app_connector_invocation_adapter" and .source_operator_approval_packet_route == "operator_approval_packet_ready_for_review" and .registry_guard_route == "require_approval_ledger" and .operator_approval_receipt_projection_route == "operator_approval_receipt_projection_ready" and .operator_approval_receipt_projection_ready == true and .operator_decision_receipt_required == true and .operator_decision_readback_evidence_required == true and .operator_decision_receipt_write_blocked == true and .operator_decision_record_written == false and .operator_decision_receipt_written == false and .operator_acceptance_present == false and .approval_request_sent == false and .live_cutover_switch_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and (.blockers | index("operator_decision_record_not_written")) != null
  and (.blockers | index("operator_decision_receipt_not_written")) != null
  and (.next_actions | index("restore_tool_execution_canary_cutover_plan_without_invocation")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_execution_operator_approval_receipt_projection --quiet
)

printf 'hepta-systems-tool-execution-operator-approval-receipt-projection-gate: PASS: operator approval decision receipts are projected while decision writes and invocation remain disabled\n'
