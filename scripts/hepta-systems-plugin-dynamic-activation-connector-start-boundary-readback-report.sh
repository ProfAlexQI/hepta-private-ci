#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable rollback/uninstall noop report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing dynamic activation connector boundary Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing dynamic activation connector boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the dynamic activation connector boundary report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render rollback/uninstall noop report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "rollback/uninstall noop report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_DYNAMIC_ACTIVATION_CONNECTOR_START_BOUNDARY_READBACK_2026-06-30.md" \
  '
  def permission_gate_key($kind):
    if $kind == "mcp_server" then
      "permission-gate:hepta-system:local-mcp:read-only-network-none"
    elif $kind == "app_connector" then
      "permission-gate:hepta-system:local-app:connector-hepta-local-network-none"
    else
      "permission-gate:hepta-system:unknown"
    end;
  def connector_start_plan_id($kind):
    if $kind == "mcp_server" then
      "connector-start-plan:hepta-system:local-mcp:blocked"
    elif $kind == "app_connector" then
      "connector-start-plan:hepta-system:local-app:blocked"
    else
      "connector-start-plan:hepta-system:unknown:blocked"
    end;
  def connector_start_route($kind):
    if $kind == "mcp_server" then
      "mcp-start://hepta-system/local-mcp/blocked"
    elif $kind == "app_connector" then
      "app-connector-start://hepta-system/local-app/blocked"
    else
      "connector-start://hepta-system/unknown/blocked"
    end;
  def tool_registry_registration_denial_id($kind):
    if $kind == "mcp_server" then
      "tool-registry-denial:hepta-system:local-mcp:no-registration"
    elif $kind == "app_connector" then
      "tool-registry-denial:hepta-system:local-app:no-registration"
    else
      "tool-registry-denial:hepta-system:unknown:no-registration"
    end;
  def ledger_denial_id($kind):
    if $kind == "mcp_server" then
      "ledger-denial:hepta-system:local-mcp:no-write"
    elif $kind == "app_connector" then
      "ledger-denial:hepta-system:local-app:no-write"
    else
      "ledger-denial:hepta-system:unknown:no-write"
    end;
  def receipt_denial_id($kind):
    if $kind == "mcp_server" then
      "receipt-denial:hepta-system:local-mcp:no-persistence"
    elif $kind == "app_connector" then
      "receipt-denial:hepta-system:local-app:no-persistence"
    else
      "receipt-denial:hepta-system:unknown:no-persistence"
    end;
  def activation_denial_receipt_id($kind):
    if $kind == "mcp_server" then
      "activation-denial-receipt:hepta-system:local-mcp:no-activation"
    elif $kind == "app_connector" then
      "activation-denial-receipt:hepta-system:local-app:no-activation"
    else
      "activation-denial-receipt:hepta-system:unknown:no-activation"
    end;
  def entry($source_entry): {
    candidate_tool_id:$source_entry.candidate_tool_id,
    contribution_kind:$source_entry.contribution_kind,
    source_preflight_route:$source_entry.source_preflight_route,
    install_cache_path:$source_entry.install_cache_path,
    artifact_digest:$source_entry.artifact_digest,
    rollback_uninstall_plan_id:$source_entry.first_rollback_uninstall_plan_id,
    activation_event_type:"manual",
    permission_gate_key:permission_gate_key($source_entry.contribution_kind),
    connector_start_plan_id:connector_start_plan_id($source_entry.contribution_kind),
    connector_start_route:connector_start_route($source_entry.contribution_kind),
    tool_registry_registration_denial_id:tool_registry_registration_denial_id($source_entry.contribution_kind),
    ledger_denial_id:ledger_denial_id($source_entry.contribution_kind),
    receipt_denial_id:receipt_denial_id($source_entry.contribution_kind),
    activation_denial_receipt_id:activation_denial_receipt_id($source_entry.contribution_kind),
    manual_activation_event_projected:true,
    manual_activation_required:true,
    permission_gate_projected:true,
    connector_start_plan_projected:true,
    mcp_server_start_plan_projected:($source_entry.contribution_kind == "mcp_server"),
    app_connector_start_plan_projected:($source_entry.contribution_kind == "app_connector"),
    tool_registry_registration_denial_projected:true,
    ledger_denial_projected:true,
    receipt_denial_projected:true,
    activation_denial_receipt_projected:true,
    dynamic_activation_boundary_ready:true,
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
    live_execution_started:$source_entry.live_execution_started,
    plugin_installed:$source_entry.plugin_installed,
    cache_materialized:$source_entry.cache_materialized,
    cache_mutated:$source_entry.cache_mutated
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(entry(.))) as $entries |
  ($entries | length) as $activation_entry_count |
  ($entries | map(select(.manual_activation_event_projected == true)) | length) as $manual_activation_event_projected_count |
  ($entries | map(select(.permission_gate_projected == true)) | length) as $permission_gate_projected_count |
  ($entries | map(select(.connector_start_plan_projected == true)) | length) as $connector_start_plan_projected_count |
  ($entries | map(select(.mcp_server_start_plan_projected == true)) | length) as $mcp_server_start_plan_projected_count |
  ($entries | map(select(.app_connector_start_plan_projected == true)) | length) as $app_connector_start_plan_projected_count |
  ($entries | map(select(.tool_registry_registration_denial_projected == true)) | length) as $tool_registry_registration_denial_projected_count |
  ($entries | map(select(.ledger_denial_projected == true)) | length) as $ledger_denial_projected_count |
  ($entries | map(select(.receipt_denial_projected == true)) | length) as $receipt_denial_projected_count |
  ($entries | map(select(.activation_denial_receipt_projected == true)) | length) as $activation_denial_receipt_projected_count |
  ($entries | map(select(.dynamic_activation_started == true)) | length) as $dynamic_activation_started_count |
  ($entries | map(select(.permission_granted == true)) | length) as $permission_granted_count |
  ($entries | map(select(.mcp_server_started == true)) | length) as $mcp_server_started_count |
  ($entries | map(select(.app_connector_started == true)) | length) as $app_connector_started_count |
  ($entries | map(select(.tool_registered == true)) | length) as $tool_registered_count |
  ($entries | map(select(.tool_invoked == true)) | length) as $tool_invoked_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.approval_requested == true)) | length) as $approval_requested_count |
  ($entries | map(select(.receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.runtime_event_log_written == true)) | length) as $runtime_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_execution_started == true)) | length) as $live_execution_started_count |
  ($entries | map(select(.plugin_installed == true)) | length) as $plugin_installed_count |
  ($entries | map(select(.cache_materialized == true)) | length) as $cache_materialized_count |
  ($entries | map(select(.cache_mutated == true)) | length) as $cache_mutated_count |
  ($source_report.install_cache_rollback_uninstall_noop_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.rollback_uninstall_executed_count == 0
    and $source_report.rollback_plan_persisted_count == 0
    and $source_report.uninstall_plan_persisted_count == 0
    and $source_report.idempotency_index_written_count == 0
    and $source_report.denial_receipt_persisted_count == 0
    and $source_report.cache_materialized_count == 0
    and $source_report.cache_mutated_count == 0
    and $source_report.plugin_installed_count == 0
    and $source_report.dynamic_activation_started_count == 0
    and $lib_export_present == true
    and $activation_entry_count == 2
    and $manual_activation_event_projected_count == 2
    and $permission_gate_projected_count == 2
    and $connector_start_plan_projected_count == 2
    and $mcp_server_start_plan_projected_count == 1
    and $app_connector_start_plan_projected_count == 1
    and $tool_registry_registration_denial_projected_count == 2
    and $ledger_denial_projected_count == 2
    and $receipt_denial_projected_count == 2
    and $activation_denial_receipt_projected_count == 2
    and $dynamic_activation_started_count == 0
    and $permission_granted_count == 0
    and $mcp_server_started_count == 0
    and $app_connector_started_count == 0
    and $tool_registered_count == 0
    and $tool_invoked_count == 0
    and $ledger_written_count == 0
    and $approval_requested_count == 0
    and $receipt_persisted_count == 0
    and $runtime_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_execution_started_count == 0
    and $plugin_installed_count == 0
    and $cache_materialized_count == 0
    and $cache_mutated_count == 0
    and ($entries | all(.manual_activation_event_projected == true
      and .manual_activation_required == true
      and .permission_gate_projected == true
      and .connector_start_plan_projected == true
      and .tool_registry_registration_denial_projected == true
      and .ledger_denial_projected == true
      and .receipt_denial_projected == true
      and .activation_denial_receipt_projected == true
      and .dynamic_activation_boundary_ready == true
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
      and .live_execution_started == false
      and .plugin_installed == false
      and .cache_materialized == false
      and .cache_mutated == false))) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_gate",
    schema_version:"hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_rollback_uninstall_noop_ready:$source_report.install_cache_rollback_uninstall_noop_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    activation_entry_count:$activation_entry_count,
    manual_activation_event_projected_count:$manual_activation_event_projected_count,
    permission_gate_projected_count:$permission_gate_projected_count,
    connector_start_plan_projected_count:$connector_start_plan_projected_count,
    mcp_server_start_plan_projected_count:$mcp_server_start_plan_projected_count,
    app_connector_start_plan_projected_count:$app_connector_start_plan_projected_count,
    tool_registry_registration_denial_projected_count:$tool_registry_registration_denial_projected_count,
    ledger_denial_projected_count:$ledger_denial_projected_count,
    receipt_denial_projected_count:$receipt_denial_projected_count,
    activation_denial_receipt_projected_count:$activation_denial_receipt_projected_count,
    dynamic_activation_started_count:$dynamic_activation_started_count,
    permission_granted_count:$permission_granted_count,
    mcp_server_started_count:$mcp_server_started_count,
    app_connector_started_count:$app_connector_started_count,
    tool_registered_count:$tool_registered_count,
    tool_invoked_count:$tool_invoked_count,
    ledger_written_count:$ledger_written_count,
    approval_requested_count:$approval_requested_count,
    receipt_persisted_count:$receipt_persisted_count,
    runtime_event_log_written_count:$runtime_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_execution_started_count:$live_execution_started_count,
    plugin_installed_count:$plugin_installed_count,
    cache_materialized_count:$cache_materialized_count,
    cache_mutated_count:$cache_mutated_count,
    dynamic_activation_connector_start_boundary_ready:$ready,
    dynamic_activation_allowed:false,
    permission_grant_allowed:false,
    mcp_server_start_allowed:false,
    app_connector_start_allowed:false,
    tool_registry_registration_allowed:false,
    tool_invocation_allowed:false,
    ledger_write_allowed:false,
    approval_request_allowed:false,
    receipt_persistence_allowed:false,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    install_cache_materialization_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "dynamic_activation_disabled",
      "permission_grant_disabled",
      "mcp_server_start_disabled",
      "app_connector_start_disabled",
      "tool_registry_registration_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled",
      "receipt_persistence_disabled",
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "install_cache_materialization_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_registry_registration_denial_receipt_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
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
