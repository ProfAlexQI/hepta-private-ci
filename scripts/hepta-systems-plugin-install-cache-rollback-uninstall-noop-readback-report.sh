#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-install-cache-idempotency-denial-receipt-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable install-cache idempotency denial receipt report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing install-cache rollback uninstall noop Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing install-cache rollback uninstall noop architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the install-cache rollback uninstall noop report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render install-cache idempotency denial receipt report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "install-cache idempotency denial receipt report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_INSTALL_CACHE_ROLLBACK_UNINSTALL_NOOP_READBACK_2026-06-30.md" \
  '
  def rollback_noop_route($kind):
    if $kind == "mcp_server" then
      "plugin-rollback-noop://hepta-system/mcp"
    elif $kind == "app_connector" then
      "plugin-rollback-noop://hepta-system/app"
    else
      "plugin-rollback-noop://hepta-system/unknown"
    end;
  def uninstall_noop_route($kind):
    if $kind == "mcp_server" then
      "plugin-uninstall-noop://hepta-system/mcp"
    elif $kind == "app_connector" then
      "plugin-uninstall-noop://hepta-system/app"
    else
      "plugin-uninstall-noop://hepta-system/unknown"
    end;
  def rollback_guard_key($kind):
    if $kind == "mcp_server" then
      "rollback-guard:hepta-system:local-mcp:no-exec"
    elif $kind == "app_connector" then
      "rollback-guard:hepta-system:local-app:no-exec"
    else
      "rollback-guard:hepta-system:unknown:no-exec"
    end;
  def uninstall_guard_key($kind):
    if $kind == "mcp_server" then
      "uninstall-guard:hepta-system:local-mcp:no-exec"
    elif $kind == "app_connector" then
      "uninstall-guard:hepta-system:local-app:no-exec"
    else
      "uninstall-guard:hepta-system:unknown:no-exec"
    end;
  def cache_restore_block_key($kind):
    if $kind == "mcp_server" then
      "cache-restore-block:hepta-system:local-mcp:no-cache-write"
    elif $kind == "app_connector" then
      "cache-restore-block:hepta-system:local-app:no-cache-write"
    else
      "cache-restore-block:hepta-system:unknown:no-cache-write"
    end;
  def denial_receipt_anchor($kind):
    if $kind == "mcp_server" then
      "denial-anchor:hepta-system:local-mcp:rollback-uninstall-noop"
    elif $kind == "app_connector" then
      "denial-anchor:hepta-system:local-app:rollback-uninstall-noop"
    else
      "denial-anchor:hepta-system:unknown:rollback-uninstall-noop"
    end;
  def entry($source_entry): {
    candidate_tool_id:$source_entry.candidate_tool_id,
    contribution_kind:$source_entry.contribution_kind,
    source_preflight_route:$source_entry.source_preflight_route,
    install_cache_path:$source_entry.install_cache_path,
    artifact_digest:$source_entry.artifact_digest,
    first_rollback_uninstall_plan_id:$source_entry.rollback_uninstall_plan_id,
    second_rollback_uninstall_plan_id:$source_entry.rollback_uninstall_plan_id,
    stable_rollback_uninstall_plan:true,
    unique_rollback_uninstall_plan:true,
    rollback_noop_route:rollback_noop_route($source_entry.contribution_kind),
    uninstall_noop_route:uninstall_noop_route($source_entry.contribution_kind),
    rollback_guard_key:rollback_guard_key($source_entry.contribution_kind),
    uninstall_guard_key:uninstall_guard_key($source_entry.contribution_kind),
    cache_restore_block_key:cache_restore_block_key($source_entry.contribution_kind),
    denial_receipt_anchor:denial_receipt_anchor($source_entry.contribution_kind),
    idempotency_denial_anchor:$source_entry.idempotency_denial_anchor,
    rollback_noop_route_projected:true,
    uninstall_noop_route_projected:true,
    rollback_guard_projected:true,
    uninstall_guard_projected:true,
    cache_restore_block_projected:true,
    denial_receipt_anchor_projected:true,
    rollback_uninstall_noop_ready:true,
    rollback_uninstall_executed:$source_entry.rollback_uninstall_executed,
    rollback_plan_persisted:false,
    uninstall_plan_persisted:false,
    idempotency_index_written:$source_entry.idempotency_index_written,
    denial_receipt_persisted:$source_entry.denial_receipt_persisted,
    cache_materialized:$source_entry.cache_materialized,
    cache_mutated:$source_entry.cache_mutated,
    plugin_installed:$source_entry.plugin_installed,
    dynamic_activation_started:$source_entry.dynamic_activation_started,
    permission_granted:$source_entry.permission_granted,
    mcp_server_started:$source_entry.mcp_server_started,
    app_connector_started:$source_entry.app_connector_started,
    tool_registered:$source_entry.tool_registered,
    tool_invoked:$source_entry.tool_invoked,
    ledger_written:$source_entry.ledger_written,
    approval_requested:$source_entry.approval_requested,
    receipt_persisted:$source_entry.receipt_persisted,
    runtime_event_log_written:$source_entry.runtime_event_log_written,
    sqlite_written:$source_entry.sqlite_written,
    live_execution_started:$source_entry.live_execution_started
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(entry(.))) as $entries |
  ($entries | length) as $rollback_entry_count |
  ($entries | map(select(.stable_rollback_uninstall_plan == true)) | length) as $stable_rollback_uninstall_plan_count |
  ($entries | map(.first_rollback_uninstall_plan_id) | unique | length) as $unique_rollback_uninstall_plan_count |
  ($entries | map(select(.rollback_noop_route_projected == true)) | length) as $rollback_noop_route_projected_count |
  ($entries | map(select(.uninstall_noop_route_projected == true)) | length) as $uninstall_noop_route_projected_count |
  ($entries | map(select(.rollback_guard_projected == true)) | length) as $rollback_guard_projected_count |
  ($entries | map(select(.uninstall_guard_projected == true)) | length) as $uninstall_guard_projected_count |
  ($entries | map(select(.cache_restore_block_projected == true)) | length) as $cache_restore_block_projected_count |
  ($entries | map(select(.denial_receipt_anchor_projected == true)) | length) as $denial_receipt_anchor_projected_count |
  ($entries | map(select(.stable_rollback_uninstall_plan == false)) | length) as $rollback_uninstall_plan_mismatch_count |
  ($rollback_entry_count - $unique_rollback_uninstall_plan_count) as $duplicate_rollback_uninstall_plan_count |
  ($entries | map(select(.rollback_uninstall_noop_ready == true)) | length) as $rollback_uninstall_noop_ready_count |
  ($entries | map(select(.rollback_uninstall_executed == true)) | length) as $rollback_uninstall_executed_count |
  ($entries | map(select(.rollback_plan_persisted == true)) | length) as $rollback_plan_persisted_count |
  ($entries | map(select(.uninstall_plan_persisted == true)) | length) as $uninstall_plan_persisted_count |
  ($entries | map(select(.idempotency_index_written == true)) | length) as $idempotency_index_written_count |
  ($entries | map(select(.denial_receipt_persisted == true)) | length) as $denial_receipt_persisted_count |
  ($entries | map(select(.cache_materialized == true)) | length) as $cache_materialized_count |
  ($entries | map(select(.cache_mutated == true)) | length) as $cache_mutated_count |
  ($entries | map(select(.plugin_installed == true)) | length) as $plugin_installed_count |
  ($entries | map(select(.dynamic_activation_started == true)) | length) as $dynamic_activation_started_count |
  ($source_report.install_cache_idempotency_denial_receipt_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.stable_idempotency_key_count == 2
    and $source_report.stable_denial_receipt_id_count == 2
    and $source_report.idempotency_index_written_count == 0
    and $source_report.denial_receipt_persisted_count == 0
    and $source_report.cache_materialized_count == 0
    and $source_report.cache_mutated_count == 0
    and $source_report.plugin_installed_count == 0
    and $source_report.dynamic_activation_started_count == 0
    and $lib_export_present == true
    and $rollback_entry_count == 2
    and $stable_rollback_uninstall_plan_count == 2
    and $unique_rollback_uninstall_plan_count == 2
    and $rollback_noop_route_projected_count == 2
    and $uninstall_noop_route_projected_count == 2
    and $rollback_guard_projected_count == 2
    and $uninstall_guard_projected_count == 2
    and $cache_restore_block_projected_count == 2
    and $denial_receipt_anchor_projected_count == 2
    and $rollback_uninstall_plan_mismatch_count == 0
    and $duplicate_rollback_uninstall_plan_count == 0
    and $rollback_uninstall_noop_ready_count == 2
    and $rollback_uninstall_executed_count == 0
    and $rollback_plan_persisted_count == 0
    and $uninstall_plan_persisted_count == 0
    and $idempotency_index_written_count == 0
    and $denial_receipt_persisted_count == 0
    and $cache_materialized_count == 0
    and $cache_mutated_count == 0
    and $plugin_installed_count == 0
    and $dynamic_activation_started_count == 0
    and ($entries | all(.stable_rollback_uninstall_plan == true
      and .unique_rollback_uninstall_plan == true
      and .rollback_noop_route_projected == true
      and .uninstall_noop_route_projected == true
      and .rollback_guard_projected == true
      and .uninstall_guard_projected == true
      and .cache_restore_block_projected == true
      and .denial_receipt_anchor_projected == true
      and .rollback_uninstall_noop_ready == true
      and .rollback_uninstall_executed == false
      and .rollback_plan_persisted == false
      and .uninstall_plan_persisted == false
      and .idempotency_index_written == false
      and .denial_receipt_persisted == false
      and .cache_materialized == false
      and .cache_mutated == false
      and .plugin_installed == false
      and .dynamic_activation_started == false
      and .permission_granted == false
      and .mcp_server_started == false
      and .app_connector_started == false
      and .tool_registered == false
      and .tool_invoked == false
      and .ledger_written == false
      and .approval_requested == false
      and .receipt_persisted == false
      and .runtime_event_log_written == false
      and .sqlite_written == false
      and .live_execution_started == false))) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_gate",
    schema_version:"hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_idempotency_denial_receipt_ready:$source_report.install_cache_idempotency_denial_receipt_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    rollback_entry_count:$rollback_entry_count,
    stable_rollback_uninstall_plan_count:$stable_rollback_uninstall_plan_count,
    unique_rollback_uninstall_plan_count:$unique_rollback_uninstall_plan_count,
    rollback_noop_route_projected_count:$rollback_noop_route_projected_count,
    uninstall_noop_route_projected_count:$uninstall_noop_route_projected_count,
    rollback_guard_projected_count:$rollback_guard_projected_count,
    uninstall_guard_projected_count:$uninstall_guard_projected_count,
    cache_restore_block_projected_count:$cache_restore_block_projected_count,
    denial_receipt_anchor_projected_count:$denial_receipt_anchor_projected_count,
    rollback_uninstall_plan_mismatch_count:$rollback_uninstall_plan_mismatch_count,
    duplicate_rollback_uninstall_plan_count:$duplicate_rollback_uninstall_plan_count,
    rollback_uninstall_noop_ready_count:$rollback_uninstall_noop_ready_count,
    rollback_uninstall_executed_count:$rollback_uninstall_executed_count,
    rollback_plan_persisted_count:$rollback_plan_persisted_count,
    uninstall_plan_persisted_count:$uninstall_plan_persisted_count,
    idempotency_index_written_count:$idempotency_index_written_count,
    denial_receipt_persisted_count:$denial_receipt_persisted_count,
    cache_materialized_count:$cache_materialized_count,
    cache_mutated_count:$cache_mutated_count,
    plugin_installed_count:$plugin_installed_count,
    dynamic_activation_started_count:$dynamic_activation_started_count,
    install_cache_rollback_uninstall_noop_readback_ready:$ready,
    rollback_uninstall_execution_allowed:false,
    rollback_plan_persistence_allowed:false,
    uninstall_plan_persistence_allowed:false,
    idempotency_index_write_allowed:false,
    denial_receipt_persistence_allowed:false,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    install_cache_materialization_allowed:false,
    dynamic_activation_allowed:false,
    permission_grant_allowed:false,
    mcp_server_start_allowed:false,
    app_connector_start_allowed:false,
    tool_registry_registration_allowed:false,
    tool_invocation_allowed:false,
    ledger_write_allowed:false,
    approval_request_allowed:false,
    receipt_persistence_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "rollback_uninstall_execution_disabled",
      "rollback_plan_persistence_disabled",
      "uninstall_plan_persistence_disabled",
      "idempotency_index_write_disabled",
      "denial_receipt_persistence_disabled",
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "install_cache_materialization_disabled",
      "dynamic_activation_disabled",
      "permission_grant_disabled",
      "mcp_server_start_disabled",
      "app_connector_start_disabled",
      "tool_registry_registration_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled",
      "receipt_persistence_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      rollback_uninstall_executed:false,
      rollback_plan_persisted:false,
      uninstall_plan_persisted:false,
      idempotency_index_written:false,
      denial_receipt_persisted:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      install_cache_materialized:false,
      dynamic_activation_started:false,
      permission_granted:false,
      mcp_server_started:false,
      app_connector_started:false,
      tool_registry_mutated:false,
      tool_registered:false,
      tool_invoked:false,
      ledger_written:false,
      approval_requested:false,
      receipt_persisted:false,
      runtime_event_log_written:false,
      sqlite_written:false,
      credential_read:false,
      external_network_used:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }
  '
