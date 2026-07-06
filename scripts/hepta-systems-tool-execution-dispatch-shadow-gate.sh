#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-dispatch-shadow-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-execution-adapter-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_DISPATCH_SHADOW_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-dispatch-shadow-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable execution dispatch shadow report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable execution adapter preflight gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing execution dispatch shadow architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the execution dispatch shadow report"
fi

grep -q 'Execution Dispatch Shadow' "$DOC" \
  || fail "architecture note must document Execution Dispatch Shadow"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'disabled execution dispatch shadow' "$DOC" \
  || fail "architecture note must document disabled execution dispatch shadow"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_dispatch_shadow"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_execution_adapter_preflight_surface == "tool_execution_adapter_preflight"
  and .source_execution_adapter_preflight_ready == true
  and .dispatch_shadow_binding_present == true
  and .tool_invocation_execution_switch_enabled == false
  and .adapter_dispatch_switch_enabled == false
  and .candidate_count == 2
  and .dispatch_shadow_ready_count == 2
  and .dispatch_shadow_blocked_count == 0
  and .disabled_execution_dispatch_shadow_count == 2
  and .all_execution_adapter_preflight_entries_shadowed == true
  and .all_dispatch_shadow_entries_keep_approval_guard == true
  and .tool_execution_dispatch_shadow_ready == true
  and .execution_dispatch_shadow_allowed == true
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .result_receipt_written == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_execution_operator_approval_packet_without_invocation"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .execution_adapter_kind == "mcp_tool_call_adapter" and .source_adapter_preflight_route == "disabled_execution_adapter_preflight" and .registry_guard_route == "require_approval_ledger" and .dispatch_shadow_route == "disabled_execution_dispatch_shadow" and .dispatch_shadow_ready == true and .execution_adapter_preflight_ready == true and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .execution_adapter_kind == "app_connector_invocation_adapter" and .source_adapter_preflight_route == "disabled_execution_adapter_preflight" and .registry_guard_route == "require_approval_ledger" and .dispatch_shadow_route == "disabled_execution_dispatch_shadow" and .dispatch_shadow_ready == true and .execution_adapter_preflight_ready == true and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and (.blockers | index("execution_adapter_dispatch_disabled")) != null
  and (.next_actions | index("restore_tool_execution_operator_approval_packet_without_invocation")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_execution_dispatch_shadow --quiet
)

printf 'hepta-systems-tool-execution-dispatch-shadow-gate: PASS: disabled execution adapter routes are shadowed while dispatch and invocation remain disabled\n'
