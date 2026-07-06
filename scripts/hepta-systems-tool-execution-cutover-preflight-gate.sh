#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-execution-cutover-preflight-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-execution-dispatch-shadow-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-cutover-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable execution cutover preflight report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable execution dispatch shadow gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing execution cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the execution cutover preflight report"
fi

grep -q 'Execution Cutover Preflight' "$DOC" \
  || fail "architecture note must document Execution Cutover Preflight"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"
grep -q 'explicit cutover approval' "$DOC" \
  || fail "architecture note must document explicit cutover approval"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_execution_cutover_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_execution_dispatch_shadow_surface == "tool_execution_dispatch_shadow"
  and .source_execution_dispatch_shadow_ready == true
  and .cutover_matrix_binding_present == true
  and .explicit_cutover_approval_present == false
  and .tool_invocation_execution_switch_enabled == false
  and .adapter_dispatch_switch_enabled == false
  and .live_cutover_switch_enabled == false
  and .candidate_count == 2
  and .cutover_preflight_ready_count == 2
  and .cutover_preflight_blocked_count == 0
  and .explicit_cutover_approval_required_count == 2
  and .live_cutover_blocked_count == 2
  and .all_dispatch_shadow_entries_bound_to_cutover_preflight == true
  and .all_cutover_entries_keep_approval_guard == true
  and .tool_execution_cutover_preflight_ready == true
  and .tool_execution_live_cutover_allowed == false
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .result_receipt_written == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_execution_operator_approval_receipt_projection_without_invocation"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .source_dispatch_shadow_route == "disabled_execution_dispatch_shadow" and .registry_guard_route == "require_approval_ledger" and .cutover_preflight_route == "cutover_preflight_blocked_until_explicit_approval" and .cutover_preflight_ready == true and .explicit_cutover_approval_required == true and .live_cutover_blocked == true and .explicit_cutover_approval_present == false and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .live_cutover_switch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .source_dispatch_shadow_route == "disabled_execution_dispatch_shadow" and .registry_guard_route == "require_approval_ledger" and .cutover_preflight_route == "cutover_preflight_blocked_until_explicit_approval" and .cutover_preflight_ready == true and .explicit_cutover_approval_required == true and .live_cutover_blocked == true and .explicit_cutover_approval_present == false and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .live_cutover_switch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and (.blockers | index("explicit_cutover_approval_missing")) != null
  and (.blockers | index("live_cutover_switch_disabled")) != null
  and (.next_actions | index("restore_tool_execution_operator_approval_receipt_projection_without_invocation")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_execution_cutover_preflight --quiet
)

printf 'hepta-systems-tool-execution-cutover-preflight-gate: PASS: execution dispatch shadows are collected into a cutover blocker matrix while live invocation remains disabled\n'
