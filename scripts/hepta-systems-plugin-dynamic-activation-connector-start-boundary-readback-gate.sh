#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable dynamic activation connector boundary report: $REPORT"
[[ -f "$DOC" ]] || fail "missing dynamic activation connector boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the dynamic activation connector boundary report"
fi

rg -q 'Hepta Systems Plugin Dynamic Activation Connector Start Boundary Readback' "$DOC" \
  || fail "architecture note must document the dynamic activation connector boundary readback"
rg -q 'manual activation events, permission gates, connector start plans, ToolRegistry registration denials, ledger denials, receipt denials, and activation denial receipts' "$DOC" \
  || fail "architecture note must document activation and denial projections"
rg -q 'no dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed side-effect boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_gate"
  and .schema_version == "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_rollback_uninstall_noop_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .activation_entry_count == 2
  and .manual_activation_event_projected_count == 2
  and .permission_gate_projected_count == 2
  and .connector_start_plan_projected_count == 2
  and .mcp_server_start_plan_projected_count == 1
  and .app_connector_start_plan_projected_count == 1
  and .tool_registry_registration_denial_projected_count == 2
  and .ledger_denial_projected_count == 2
  and .receipt_denial_projected_count == 2
  and .activation_denial_receipt_projected_count == 2
  and .dynamic_activation_started_count == 0
  and .permission_granted_count == 0
  and .mcp_server_started_count == 0
  and .app_connector_started_count == 0
  and .tool_registered_count == 0
  and .tool_invoked_count == 0
  and .ledger_written_count == 0
  and .approval_requested_count == 0
  and .receipt_persisted_count == 0
  and .runtime_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_execution_started_count == 0
  and .plugin_installed_count == 0
  and .cache_materialized_count == 0
  and .cache_mutated_count == 0
  and .dynamic_activation_connector_start_boundary_ready == true
  and .dynamic_activation_allowed == false
  and .permission_grant_allowed == false
  and .mcp_server_start_allowed == false
  and .app_connector_start_allowed == false
  and .tool_registry_registration_allowed == false
  and .tool_invocation_allowed == false
  and .ledger_write_allowed == false
  and .approval_request_allowed == false
  and .receipt_persistence_allowed == false
  and .plugin_install_allowed == false
  and .plugin_cache_mutation_allowed == false
  and .install_cache_materialization_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .activation_event_type == "manual" and .permission_gate_key == "permission-gate:hepta-system:local-mcp:read-only-network-none" and .connector_start_plan_id == "connector-start-plan:hepta-system:local-mcp:blocked" and .connector_start_route == "mcp-start://hepta-system/local-mcp/blocked" and .tool_registry_registration_denial_id == "tool-registry-denial:hepta-system:local-mcp:no-registration" and .ledger_denial_id == "ledger-denial:hepta-system:local-mcp:no-write" and .receipt_denial_id == "receipt-denial:hepta-system:local-mcp:no-persistence" and .activation_denial_receipt_id == "activation-denial-receipt:hepta-system:local-mcp:no-activation")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .activation_event_type == "manual" and .permission_gate_key == "permission-gate:hepta-system:local-app:connector-hepta-local-network-none" and .connector_start_plan_id == "connector-start-plan:hepta-system:local-app:blocked" and .connector_start_route == "app-connector-start://hepta-system/local-app/blocked" and .tool_registry_registration_denial_id == "tool-registry-denial:hepta-system:local-app:no-registration" and .ledger_denial_id == "ledger-denial:hepta-system:local-app:no-write" and .receipt_denial_id == "receipt-denial:hepta-system:local-app:no-persistence" and .activation_denial_receipt_id == "activation-denial-receipt:hepta-system:local-app:no-activation")
  and (.entries | all(.manual_activation_event_projected == true and .manual_activation_required == true and .permission_gate_projected == true and .connector_start_plan_projected == true and .tool_registry_registration_denial_projected == true and .ledger_denial_projected == true and .receipt_denial_projected == true and .activation_denial_receipt_projected == true and .dynamic_activation_boundary_ready == true and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .tool_registered == false and .tool_invoked == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false and .plugin_installed == false and .cache_materialized == false and .cache_mutated == false))
  and (.blockers | index("dynamic_activation_disabled")) != null
  and (.blockers | index("permission_grant_disabled")) != null
  and (.blockers | index("mcp_server_start_disabled")) != null
  and (.blockers | index("app_connector_start_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("plugin_install_disabled")) != null
  and (.blockers | index("plugin_cache_mutation_disabled")) != null
  and (.blockers | index("install_cache_materialization_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_registry_registration_denial_receipt_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback --lib
)

printf 'hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-gate: PASS: dynamic activation and connector start boundaries are projected without activation, registration, ledger, receipt, runtime, or live mutation\n'
