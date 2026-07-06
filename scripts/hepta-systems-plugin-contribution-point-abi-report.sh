#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
RUST_SOURCE="$ROOT/codex-rs/core-plugins/src/contribution_point_abi.rs"
LOADER_BINDING_SOURCE="$ROOT/codex-rs/core-plugins/src/contribution_point_loader_binding.rs"
LIB_SOURCE="$ROOT/codex-rs/core-plugins/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_CONTRIBUTION_POINT_ABI_2026-06-21.md"
HEPTA_SYSTEM_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"

fail() {
  printf 'hepta-systems-plugin-contribution-point-abi-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -f "$RUST_SOURCE" ]] || fail "missing Rust ABI source: $RUST_SOURCE"
[[ -f "$LOADER_BINDING_SOURCE" ]] || fail "missing loader binding source: $LOADER_BINDING_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing core-plugins lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the contribution-point ABI report"
fi

lib_export_present=false
if grep -q '^pub mod contribution_point_abi;' "$LIB_SOURCE"; then
  lib_export_present=true
fi

loader_binding_export_present=false
if grep -q '^pub mod contribution_point_loader_binding;' "$LIB_SOURCE"; then
  loader_binding_export_present=true
fi

hepta_system_manifest_present=false
if [[ -f "$HEPTA_SYSTEM_MANIFEST" ]]; then
  hepta_system_manifest_present=true
fi

jq -n \
  --arg gate "scripts/hepta-systems-plugin-contribution-point-abi-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_CONTRIBUTION_POINT_ABI_2026-06-21.md" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson loader_binding_export_present "$loader_binding_export_present" \
  --argjson hepta_system_manifest_present "$hepta_system_manifest_present" \
  '{
    runtime:"hepta",
    surface:"plugin_contribution_point_abi",
    status:"ready",
    registry_api:"hepta.systems.pluginRegistry/v1",
    lib_export_present:$lib_export_present,
    loader_binding_export_present:$loader_binding_export_present,
    hepta_system_manifest_present:$hepta_system_manifest_present,
    contribution_point_kinds:[
      "skill",
      "mcp_server",
      "tool",
      "app_connector",
      "hook",
      "permission",
      "activation_event",
      "local_storage"
    ],
    contribution_point_count:8,
    manifest_path_supported_kinds:["skill","mcp_server","app_connector","hook"],
    manifest_path_supported_count:4,
    loader_path_supported_count:4,
    current_fixture_declared_count:0,
    future_bridge_kinds:["tool","permission","activation_event","local_storage"],
    future_bridge_required_count:4,
    policy:{
      all_entries_policy_bound:true,
      permission_policy_required:true,
      activation_policy_required:true,
      ledger_required_count:4,
      mutating_entries_require_approval:true
    },
    bridges:{
      manifest_loader_bridge_ready:true,
      tool_registry_bridge_required:true,
      tool_registry_bridge_enabled:false,
      permission_manifest_field_pending:true,
      activation_manifest_field_pending:true,
      local_storage_manifest_field_pending:true,
      local_storage_scoped_to_plugin_data_root:true
    },
    abi_ready:true,
    runtime_execution_enabled:false,
    all_runtime_execution_disabled:true,
    all_live_paths_blocked:true,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_registry_invocation_source_of_truth_without_execution",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/core-plugins/src/contribution_point_abi.rs",
      loader_binding_contract:"codex-rs/core-plugins/src/contribution_point_loader_binding.rs",
      lib_export:"codex-rs/core-plugins/src/lib.rs"
    },
    blockers:[
      "hepta_system_fixture_not_restored",
      "plugin_tool_invocation_router_preflight_binding_not_restored",
      "tool_contribution_bridge_to_tool_registry_pending",
      "permission_activation_storage_manifest_fields_pending",
      "runtime_manager_binding_to_contribution_abi_pending"
    ],
    next_actions:[
      "restore_tool_registry_invocation_source_of_truth_without_execution",
      "restore_local_hepta_system_fixture_or_replace_with_current_repo_fixture",
      "map_plugin_tool_contributions_into_tool_registry_entries",
      "keep_execution_and_live_mutation_disabled_until_operator_approval"
    ],
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_cache_mutated:false,
      plugin_installed:false,
      package_lock_written:false,
      remote_sync_started:false,
      tool_registered:false,
      tool_invoked:false,
      tool_ledger_written:false,
      approval_requested:false,
      workflow_event_log_mutated:false,
      local_storage_created:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
