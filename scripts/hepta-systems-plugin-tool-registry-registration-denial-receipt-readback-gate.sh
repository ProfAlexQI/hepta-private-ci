#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-registry-registration-denial-receipt-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_REGISTRATION_DENIAL_RECEIPT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-registry-registration-denial-receipt-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable ToolRegistry registration denial receipt report: $REPORT"
[[ -f "$DOC" ]] || fail "missing ToolRegistry registration denial receipt architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the ToolRegistry registration denial receipt readback report"
fi

rg -q 'Hepta Systems Plugin ToolRegistry Registration Denial Receipt Readback' "$DOC" \
  || fail "architecture note must document the ToolRegistry registration denial receipt readback"
rg -q 'tool schema digests, registration denial ids, stable registration denial receipts, router lookup blocks, registry source-of-truth blocks, and invocation denials' "$DOC" \
  || fail "architecture note must document schema, denial, router, registry, and invocation projections"
rg -q 'no ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed side-effect boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_dynamic_activation_boundary_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .registration_entry_count == 2
  and .tool_schema_bound_count == 2
  and .tool_schema_digest_projected_count == 2
  and .registration_denial_id_projected_count == 2
  and .stable_registration_denial_receipt_count == 2
  and .unique_registration_denial_receipt_count == 2
  and .registration_denial_receipt_projected_count == 2
  and .router_lookup_block_projected_count == 2
  and .registry_source_of_truth_block_projected_count == 2
  and .invocation_denial_projected_count == 2
  and .registration_denial_receipt_mismatch_count == 0
  and .duplicate_registration_denial_receipt_count == 0
  and .tool_registered_count == 0
  and .tool_registry_mutated_count == 0
  and .registry_lookup_executed_count == 0
  and .tool_invoked_count == 0
  and .ledger_written_count == 0
  and .approval_requested_count == 0
  and .receipt_persisted_count == 0
  and .dynamic_activation_started_count == 0
  and .permission_granted_count == 0
  and .mcp_server_started_count == 0
  and .app_connector_started_count == 0
  and .plugin_installed_count == 0
  and .cache_materialized_count == 0
  and .cache_mutated_count == 0
  and .runtime_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_execution_started_count == 0
  and .tool_registry_registration_denial_receipt_readback_ready == true
  and .tool_registry_registration_allowed == false
  and .tool_registry_mutation_allowed == false
  and .registry_lookup_execution_allowed == false
  and .tool_invocation_allowed == false
  and .ledger_write_allowed == false
  and .approval_request_allowed == false
  and .receipt_persistence_allowed == false
  and .dynamic_activation_allowed == false
  and .permission_grant_allowed == false
  and .mcp_server_start_allowed == false
  and .app_connector_start_allowed == false
  and .plugin_install_allowed == false
  and .plugin_cache_mutation_allowed == false
  and .install_cache_materialization_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .tool_schema_digest == "tool-schema-digest:hepta-system:local-mcp:readiness-v0" and .tool_registry_registration_denial_id == "tool-registry-denial:hepta-system:local-mcp:no-registration" and .first_registration_denial_receipt_id == "tool-registry-registration-denial-receipt:hepta-system:local-mcp:no-registration" and .second_registration_denial_receipt_id == "tool-registry-registration-denial-receipt:hepta-system:local-mcp:no-registration" and .router_lookup_block_key == "router-lookup-block:hepta-system:local-mcp:no-registered-route" and .registry_source_of_truth_block_key == "registry-sot-block:hepta-system:local-mcp:no-mutation" and .invocation_denial_id == "tool-invocation-denial:hepta-system:local-mcp:no-invocation")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .tool_schema_digest == "tool-schema-digest:hepta-system:local-app:readiness-v0" and .tool_registry_registration_denial_id == "tool-registry-denial:hepta-system:local-app:no-registration" and .first_registration_denial_receipt_id == "tool-registry-registration-denial-receipt:hepta-system:local-app:no-registration" and .second_registration_denial_receipt_id == "tool-registry-registration-denial-receipt:hepta-system:local-app:no-registration" and .router_lookup_block_key == "router-lookup-block:hepta-system:local-app:no-registered-route" and .registry_source_of_truth_block_key == "registry-sot-block:hepta-system:local-app:no-mutation" and .invocation_denial_id == "tool-invocation-denial:hepta-system:local-app:no-invocation")
  and (.entries | all(.tool_schema_bound == true and .tool_schema_digest_projected == true and .registration_denial_id_projected == true and .registration_denial_receipt_projected == true and .stable_registration_denial_receipt == true and .unique_registration_denial_receipt == true and .router_lookup_block_projected == true and .registry_source_of_truth_block_projected == true and .invocation_denial_projected == true and .tool_registered == false and .tool_registry_mutated == false and .registry_lookup_executed == false and .tool_invoked == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .plugin_installed == false and .cache_materialized == false and .cache_mutated == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_registry_mutation_disabled")) != null
  and (.blockers | index("registry_lookup_execution_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("dynamic_activation_disabled")) != null
  and (.blockers | index("permission_grant_disabled")) != null
  and (.blockers | index("mcp_server_start_disabled")) != null
  and (.blockers | index("app_connector_start_disabled")) != null
  and (.blockers | index("plugin_install_disabled")) != null
  and (.blockers | index("plugin_cache_mutation_disabled")) != null
  and (.blockers | index("install_cache_materialization_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_registry_registration_denial_receipt_readback --lib
)

printf 'hepta-systems-plugin-tool-registry-registration-denial-receipt-readback-gate: PASS: ToolRegistry registration denial receipts are stable without registration, lookup, invocation, ledger, receipt, runtime, or live mutation\n'
