#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
LOADER_BINDING_REPORT="$ROOT/scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/plugin_contribution_inventory_preview.rs"
INVENTORY_SOURCE="$ROOT/codex-rs/tools/src/tool_registry_inventory.rs"
LIB_SOURCE="$ROOT/codex-rs/tools/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_CONTRIBUTION_INVENTORY_PREVIEW_2026-06-21.md"
HEPTA_SYSTEM_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"

fail() {
  printf 'hepta-systems-plugin-tool-contribution-inventory-preview-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$LOADER_BINDING_REPORT" ]] || fail "missing executable loader binding report: $LOADER_BINDING_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing plugin tool preview source: $RUST_SOURCE"
[[ -f "$INVENTORY_SOURCE" ]] || fail "missing ToolRegistry inventory source: $INVENTORY_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing codex-tools lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing plugin tool contribution inventory preview architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool contribution inventory preview report"
fi

lib_export_present=false
if grep -q 'pub use plugin_contribution_inventory_preview::hepta_system_plugin_tool_contribution_inventory_preview_plan;' "$LIB_SOURCE"; then
  lib_export_present=true
fi

tool_registry_inventory_export_present=false
if grep -q 'pub use tool_registry_inventory::ToolRegistryInventory;' "$LIB_SOURCE"; then
  tool_registry_inventory_export_present=true
fi

hepta_system_manifest_present=false
if [[ -f "$HEPTA_SYSTEM_MANIFEST" ]]; then
  hepta_system_manifest_present=true
fi

jq -n \
  --slurpfile loader <("$LOADER_BINDING_REPORT") \
  --arg gate "scripts/hepta-systems-plugin-tool-contribution-inventory-preview-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_CONTRIBUTION_INVENTORY_PREVIEW_2026-06-21.md" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson tool_registry_inventory_export_present "$tool_registry_inventory_export_present" \
  --argjson hepta_system_manifest_present "$hepta_system_manifest_present" \
  '
    ($loader[0]) as $loader
    | {
      runtime:"hepta",
      surface:"plugin_tool_contribution_inventory_preview",
      status:"ready",
      plugin_id:"hepta-system@hepta-local",
      source_loader_binding_surface:$loader.surface,
      source_loader_binding_ready:$loader.binding_ready,
      source_loader_contract_ready:$loader.loader_contract_ready,
      hepta_system_manifest_present:$hepta_system_manifest_present,
      lib_export_present:$lib_export_present,
      tool_registry_inventory_export_present:$tool_registry_inventory_export_present,
      candidate_source:"manifest_fixture_readback_without_registration",
      candidate_count:2,
      current_fixture_candidate_count:2,
      planned_candidate_count:2,
      candidate_tool_ids:[
        "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
        "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
      ],
      candidate_names:[
        "hepta_system_local_mcp",
        "hepta_system_local_app"
      ],
      candidate_kinds:["mcp_server","app_connector"],
      skipped_loader_bound_non_tool_kinds:["skill","hook"],
      candidate_inventory_sources:["mcp","connector"],
      candidate_loader_output_fields:["mcp_servers","apps"],
      candidate_side_effect_levels:["local_mutation","external_mutation"],
      candidate_approval_kinds:["on_use","install"],
      candidate_auth_required:[false,true],
      candidate_timeout_ms:[30000,30000],
      candidate_ledger_required:[true,true],
      candidate_guard_routes:["require_approval_ledger","require_approval_ledger"],
      tool_contribution_schema_complete_count:2,
      tool_contribution_risk_metadata_complete_count:2,
      tool_contribution_ledger_required_count:2,
      tool_contribution_approval_required_count:2,
      all_candidates_have_schema:true,
      all_candidates_have_risk_metadata:true,
      all_candidates_require_ledger:true,
      mutating_candidates_require_approval:true,
      all_candidates_have_guard_route:true,
      inventory_registration_enabled:false,
      tool_invocation_enabled:false,
      ledger_written:false,
      approval_requested:false,
      mcp_server_started:false,
      app_connector_started:false,
      preview_ready:true,
      live_mutation_ready:false,
      next_migration_step:"restore_tool_registry_invocation_source_of_truth_without_execution",
      local_gate:$gate,
      architecture_note:$doc,
      source_files:{
        rust_contract:"codex-rs/tools/src/plugin_contribution_inventory_preview.rs",
        tool_registry_inventory:"codex-rs/tools/src/tool_registry_inventory.rs",
        loader_binding_report:"scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh"
      },
      blockers:[
        "plugin_tool_invocation_router_preflight_binding_not_restored",
        "tool_registry_registration_disabled",
        "tool_invocation_disabled",
        "approval_ledger_execution_disabled"
      ],
      next_actions:[
        "restore_tool_registry_invocation_source_of_truth_without_execution",
        "keep_parser_output_read_only_until_preflight_adapter_is_restored",
        "keep_manifest_fixture_readback_side_effect_free",
        "keep_tool_registration_invocation_ledger_and_approval_disabled_until_operator_approval"
      ],
      side_effect_free:true,
      side_effects:{
        report_written:false,
        git_index_mutated:false,
        plugin_cache_mutated:false,
        plugin_installed:false,
        package_lock_written:false,
        remote_sync_started:false,
        manifest_rewritten:false,
        loader_invoked:false,
        tool_registered:false,
        tool_invoked:false,
        tool_ledger_written:false,
        approval_requested:false,
        mcp_server_started:false,
        app_connector_started:false,
        local_storage_created:false,
        workflow_event_log_mutated:false,
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
