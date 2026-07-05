#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-read-only-e2e-report.sh"
PLUGIN_LIFECYCLE_GATE="$ROOT/scripts/hepta-systems-plugin-lifecycle-state-machine-gate.sh"
TOOL_DISPATCH_GATE="$ROOT/scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-gate.sh"
WORKFLOW_ADAPTER_GATE="$ROOT/scripts/hepta-systems-workflow-durable-store-adapter-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_2026-06-27.md"

fail() {
  printf 'hepta-systems-hepta-system-status-read-only-e2e-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 4 report: $REPORT"
[[ -x "$PLUGIN_LIFECYCLE_GATE" ]] || fail "missing executable plugin lifecycle gate: $PLUGIN_LIFECYCLE_GATE"
[[ -x "$TOOL_DISPATCH_GATE" ]] || fail "missing executable tool dispatch gate: $TOOL_DISPATCH_GATE"
[[ -x "$WORKFLOW_ADAPTER_GATE" ]] || fail "missing executable workflow adapter gate: $WORKFLOW_ADAPTER_GATE"
[[ -f "$DOC" ]] || fail "missing Phase 4 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 4 read-only E2E report"
fi

grep -q 'Thin Read-Only E2E' "$DOC" \
  || fail "architecture note must document Thin Read-Only E2E"
grep -q 'Native read-only console' "$DOC" \
  || fail "architecture note must document Native read-only console"
grep -q 'no registration, invocation, ledger writes, approval requests, receipt persistence, event-log writes, SQLite writes, replay, rollback, or live execution' "$DOC" \
  || fail "architecture note must document the closed mutation boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_system_status_read_only_e2e"
  and .status == "ready"
  and .gate == "hepta_system_status_read_only_e2e_gate"
  and .schema_version == "hepta_system_status_read_only_e2e_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_plugin_lifecycle_ready == true
  and .source_tool_dispatch_ready == true
  and .source_workflow_adapter_ready == true
  and .native_read_only_console_ready == true
  and .plugin_status_skill_present == true
  and .native_runtime_status_present == true
  and .native_home_runtime_status_present == true
  and .native_action_bridge_present == true
  and .lib_export_present == true
  and .chain_link_count == 4
  and .chain_ready_count == 4
  and .read_only_e2e_ready == true
  and .ready_for_registration == false
  and .ready_for_invocation == false
  and .ready_for_ledger_write == false
  and .ready_for_approval_request == false
  and .ready_for_receipt_persistence == false
  and .ready_for_event_log_write == false
  and .ready_for_sqlite_write == false
  and .ready_for_workflow_execution == false
  and .ready_for_replay_execution == false
  and .ready_for_rollback_execution == false
  and .ready_for_native_post_mutation == false
  and .ready_for_channel_send == false
  and .ready_for_live_execution == false
  and (.chain_links | length) == 4
  and (.chain_links | all(.ready == true and .mutation_enabled == false))
  and any(.chain_links[]; .id == "hepta_system_status_plugin_fixture" and .route == "status_plugin_fixture_ready")
  and any(.chain_links[]; .id == "tool_registry_read_only_dispatch_preflight" and .route == "tool_registry_dispatch_preflight_ready")
  and any(.chain_links[]; .id == "workflow_durable_store_adapter_noop_receipt" and .route == "workflow_adapter_noop_receipt_ready")
  and any(.chain_links[]; .id == "native_read_only_console_projection" and .route == "native_read_only_console_projection_ready")
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("native_post_mutation_disabled")) != null
  and (.next_actions | index("phase5_keep_controlled_live_blocked_until_explicit_operator_live_approval")) != null
  and .next_migration_step == "phase5_keep_controlled_live_blocked_until_explicit_operator_live_approval"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PLUGIN_LIFECYCLE_GATE" >/dev/null
"$TOOL_DISPATCH_GATE" >/dev/null
"$WORKFLOW_ADAPTER_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_system_status_read_only_e2e --lib
)

printf 'hepta-systems-hepta-system-status-read-only-e2e-gate: PASS: hepta-system status plugin, ToolRegistry preflight, workflow adapter receipt, and Native read-only console are threaded without invocation, writes, or live execution\n'
