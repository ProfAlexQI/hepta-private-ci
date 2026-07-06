#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PREVIEW_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/plugin_tool_registry_source_of_truth_dry_run.rs"
PREVIEW_SOURCE="$ROOT/codex-rs/tools/src/plugin_contribution_inventory_preview.rs"
INVENTORY_SOURCE="$ROOT/codex-rs/tools/src/tool_registry_inventory.rs"
LIB_SOURCE="$ROOT/codex-rs/tools/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_SOURCE_OF_TRUTH_DRY_RUN_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PREVIEW_REPORT" ]] || fail "missing executable plugin tool contribution preview report: $PREVIEW_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing plugin tool registry source-of-truth dry-run source: $RUST_SOURCE"
[[ -f "$PREVIEW_SOURCE" ]] || fail "missing plugin tool contribution preview source: $PREVIEW_SOURCE"
[[ -f "$INVENTORY_SOURCE" ]] || fail "missing ToolRegistry inventory source: $INVENTORY_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing codex-tools lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing plugin tool registry source-of-truth dry-run architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool registry source-of-truth dry-run report"
fi

lib_export_present=false
if grep -q 'pub use plugin_tool_registry_source_of_truth_dry_run::hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan;' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile preview <("$PREVIEW_REPORT") \
  --arg gate "scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_REGISTRY_SOURCE_OF_TRUTH_DRY_RUN_2026-06-21.md" \
  --argjson lib_export_present "$lib_export_present" \
  '
    ($preview[0]) as $preview
    | {
      runtime:"hepta",
      surface:"plugin_tool_registry_source_of_truth_dry_run",
      status:"ready",
      plugin_id:"hepta-system@hepta-local",
      source_preview_surface:$preview.surface,
      source_preview_ready:$preview.preview_ready,
      planned_source_of_truth_surface:"tool_registry_inventory",
      hepta_system_manifest_present:$preview.hepta_system_manifest_present,
      lib_export_present:$lib_export_present,
      preview_candidate_count:$preview.candidate_count,
      planned_registry_entry_count:$preview.planned_candidate_count,
      planned_mcp_entry_count:1,
      planned_connector_entry_count:1,
      duplicate_candidate_ids:[],
      duplicate_registry_ids:[],
      unbound_candidate_ids:[],
      all_candidate_ids_unique:true,
      all_preview_candidates_bound_to_registry:true,
      all_candidates_have_schema:true,
      all_candidates_have_risk_metadata:true,
      all_candidates_require_ledger:true,
      mutating_candidates_require_approval:true,
      all_candidates_have_guard_route:true,
      registry_invocation_guard_ready:true,
      registry_source_of_truth_dry_run_ready:true,
      registry_source_of_truth_enabled:false,
      tool_registry_registration_enabled:false,
      tool_invocation_enabled:false,
      ledger_written:false,
      approval_requested:false,
      mcp_server_started:false,
      app_connector_started:false,
      live_mutation_ready:false,
      side_effect_free:true,
      next_migration_step:"restore_tool_registry_invocation_source_of_truth_without_execution",
      entries:[
        {
          plugin_id:"hepta-system@hepta-local",
          contribution_kind:"mcp_server",
          candidate_tool_id:"preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
          planned_registry_source:"mcp",
          planned_registry_name:"hepta_system_local_mcp",
          owner:"hepta-system@hepta-local",
          has_input_schema:true,
          has_output_schema:true,
          side_effect_level:"local_mutation",
          approval_kind:"on_use",
          auth_required:false,
          timeout_ms:30000,
          ledger_required:true,
          registry_entry_found:true,
          duplicate_id:false,
          guard_route:"require_approval_ledger",
          approval_required:true,
          guard_blocked:false,
          guard_blocked_reason:null,
          source_of_truth_registration_enabled:false,
          tool_invocation_enabled:false,
          ledger_write_enabled:false,
          approval_request_enabled:false,
          dry_run_ready:true
        },
        {
          plugin_id:"hepta-system@hepta-local",
          contribution_kind:"app_connector",
          candidate_tool_id:"preview:connector:hepta-system@hepta-local:hepta_system_local_app",
          planned_registry_source:"connector",
          planned_registry_name:"hepta_system_local_app",
          owner:"hepta-system@hepta-local",
          has_input_schema:true,
          has_output_schema:true,
          side_effect_level:"external_mutation",
          approval_kind:"install",
          auth_required:true,
          timeout_ms:30000,
          ledger_required:true,
          registry_entry_found:true,
          duplicate_id:false,
          guard_route:"require_approval_ledger",
          approval_required:true,
          guard_blocked:false,
          guard_blocked_reason:null,
          source_of_truth_registration_enabled:false,
          tool_invocation_enabled:false,
          ledger_write_enabled:false,
          approval_request_enabled:false,
          dry_run_ready:true
        }
      ],
      local_gate:$gate,
      architecture_note:$doc,
      source_files:{
        rust_contract:"codex-rs/tools/src/plugin_tool_registry_source_of_truth_dry_run.rs",
        preview_contract:"codex-rs/tools/src/plugin_contribution_inventory_preview.rs",
        tool_registry_inventory:"codex-rs/tools/src/tool_registry_inventory.rs",
        preview_report:"scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh"
      },
      blockers:[
        "plugin_tool_invocation_router_preflight_binding_not_restored",
        "registry_source_of_truth_enablement_disabled",
        "tool_registry_registration_disabled",
        "tool_invocation_disabled",
        "approval_ledger_execution_disabled"
      ],
      next_actions:[
        "restore_tool_registry_invocation_source_of_truth_without_execution",
        "keep_parser_output_read_only_until_preflight_adapter_is_restored",
        "keep_registry_source_of_truth_read_only_until_operator_cutover",
        "keep_tool_registration_invocation_ledger_and_approval_disabled_until_operator_approval"
      ],
      side_effects:{
        report_written:false,
        git_index_mutated:false,
        plugin_cache_mutated:false,
        plugin_installed:false,
        package_lock_written:false,
        remote_sync_started:false,
        manifest_rewritten:false,
        loader_invoked:false,
        registry_source_of_truth_enabled:false,
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
