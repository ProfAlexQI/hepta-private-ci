#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable install-cache rollback uninstall noop report: $REPORT"
[[ -f "$DOC" ]] || fail "missing install-cache rollback uninstall noop architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the install-cache rollback uninstall noop report"
fi

rg -q 'Hepta Systems Plugin Install Cache Rollback Uninstall Noop Readback' "$DOC" \
  || fail "architecture note must document the install-cache rollback uninstall noop readback"
rg -q 'stable rollback/uninstall plan ids, rollback noop routes, uninstall noop routes, guard keys, cache-restore blocks, and denial receipt anchors' "$DOC" \
  || fail "architecture note must document rollback/uninstall route and guard checks"
rg -q 'no rollback/uninstall execution, rollback plan persistence, uninstall plan persistence, idempotency index write, denial receipt persistence, noop preflight execution, plugin install, cache mutation, install-cache materialization, rollback cache restore, uninstall execution, manifest rewrite, manifest schema write, dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed side-effect boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_gate"
  and .schema_version == "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_idempotency_denial_receipt_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .rollback_entry_count == 2
  and .stable_rollback_uninstall_plan_count == 2
  and .unique_rollback_uninstall_plan_count == 2
  and .rollback_noop_route_projected_count == 2
  and .uninstall_noop_route_projected_count == 2
  and .rollback_guard_projected_count == 2
  and .uninstall_guard_projected_count == 2
  and .cache_restore_block_projected_count == 2
  and .denial_receipt_anchor_projected_count == 2
  and .rollback_uninstall_plan_mismatch_count == 0
  and .duplicate_rollback_uninstall_plan_count == 0
  and .rollback_uninstall_noop_ready_count == 2
  and .rollback_uninstall_executed_count == 0
  and .rollback_plan_persisted_count == 0
  and .uninstall_plan_persisted_count == 0
  and .idempotency_index_written_count == 0
  and .denial_receipt_persisted_count == 0
  and .cache_materialized_count == 0
  and .cache_mutated_count == 0
  and .plugin_installed_count == 0
  and .dynamic_activation_started_count == 0
  and .install_cache_rollback_uninstall_noop_readback_ready == true
  and .rollback_uninstall_execution_allowed == false
  and .rollback_plan_persistence_allowed == false
  and .uninstall_plan_persistence_allowed == false
  and .idempotency_index_write_allowed == false
  and .denial_receipt_persistence_allowed == false
  and .plugin_install_allowed == false
  and .plugin_cache_mutation_allowed == false
  and .install_cache_materialization_allowed == false
  and .dynamic_activation_allowed == false
  and .permission_grant_allowed == false
  and .mcp_server_start_allowed == false
  and .app_connector_start_allowed == false
  and .tool_registry_registration_allowed == false
  and .tool_invocation_allowed == false
  and .ledger_write_allowed == false
  and .approval_request_allowed == false
  and .receipt_persistence_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .rollback_noop_route == "plugin-rollback-noop://hepta-system/mcp" and .uninstall_noop_route == "plugin-uninstall-noop://hepta-system/mcp" and .rollback_guard_key == "rollback-guard:hepta-system:local-mcp:no-exec" and .uninstall_guard_key == "uninstall-guard:hepta-system:local-mcp:no-exec" and .cache_restore_block_key == "cache-restore-block:hepta-system:local-mcp:no-cache-write" and .denial_receipt_anchor == "denial-anchor:hepta-system:local-mcp:rollback-uninstall-noop")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .rollback_noop_route == "plugin-rollback-noop://hepta-system/app" and .uninstall_noop_route == "plugin-uninstall-noop://hepta-system/app" and .rollback_guard_key == "rollback-guard:hepta-system:local-app:no-exec" and .uninstall_guard_key == "uninstall-guard:hepta-system:local-app:no-exec" and .cache_restore_block_key == "cache-restore-block:hepta-system:local-app:no-cache-write" and .denial_receipt_anchor == "denial-anchor:hepta-system:local-app:rollback-uninstall-noop")
  and (.entries | all(.first_rollback_uninstall_plan_id == .second_rollback_uninstall_plan_id and .stable_rollback_uninstall_plan == true and .unique_rollback_uninstall_plan == true and .rollback_noop_route_projected == true and .uninstall_noop_route_projected == true and .rollback_guard_projected == true and .uninstall_guard_projected == true and .cache_restore_block_projected == true and .denial_receipt_anchor_projected == true and .rollback_uninstall_noop_ready == true and .rollback_uninstall_executed == false and .rollback_plan_persisted == false and .uninstall_plan_persisted == false and .idempotency_index_written == false and .denial_receipt_persisted == false and .cache_materialized == false and .cache_mutated == false and .plugin_installed == false and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .tool_registered == false and .tool_invoked == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("rollback_uninstall_execution_disabled")) != null
  and (.blockers | index("rollback_plan_persistence_disabled")) != null
  and (.blockers | index("uninstall_plan_persistence_disabled")) != null
  and (.blockers | index("idempotency_index_write_disabled")) != null
  and (.blockers | index("denial_receipt_persistence_disabled")) != null
  and (.blockers | index("plugin_install_disabled")) != null
  and (.blockers | index("plugin_cache_mutation_disabled")) != null
  and (.blockers | index("install_cache_materialization_disabled")) != null
  and (.blockers | index("dynamic_activation_disabled")) != null
  and (.blockers | index("mcp_server_start_disabled")) != null
  and (.blockers | index("app_connector_start_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback --lib
)

printf 'hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-gate: PASS: rollback/uninstall noop plans are stable and guarded without execution, persistence, cache mutation, plugin install, or live activation\n'
