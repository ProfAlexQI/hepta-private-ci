#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PARSER_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh"
SOURCE_OF_TRUTH_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/plugin_tool_manifest_schema_cutover_preflight.rs"
LIB_SOURCE="$ROOT/codex-rs/tools/src/lib.rs"
GATE="$ROOT/scripts/hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PREFLIGHT_ADAPTER_BRIDGE_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PARSER_REPORT" ]] || fail "missing executable plugin tool manifest parser fields report: $PARSER_REPORT"
[[ -x "$SOURCE_OF_TRUTH_REPORT" ]] || fail "missing executable source-of-truth dry-run report: $SOURCE_OF_TRUTH_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing manifest schema cutover preflight source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing codex-tools lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing manifest preflight adapter bridge architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool manifest preflight adapter bridge report"
fi

lib_export_present=false
if grep -q 'pub use plugin_tool_manifest_schema_cutover_preflight::plugin_tool_manifest_schema_cutover_preflight_plan;' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile parser <("$PARSER_REPORT") \
  --slurpfile source <("$SOURCE_OF_TRUTH_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PREFLIGHT_ADAPTER_BRIDGE_2026-06-21.md" \
  '
  ($parser[0]) as $parser |
  ($source[0]) as $source |
  {
    runtime:"hepta",
    surface:"plugin_tool_manifest_preflight_adapter_bridge",
    plugin_id:$source.plugin_id,
    status:"ready",
    source_registry_dry_run_surface:$source.surface,
    source_registry_dry_run_ready:$source.registry_source_of_truth_dry_run_ready,
    source_manifest_parser_fields_surface:$parser.surface,
    source_manifest_parser_fields_ready:$parser.parser_fields_ready,
    parsed_manifest_declarations_feed_preflight:$parser.parsed_declarations_feed_preflight,
    rust_adapter_source:"codex-rs/tools/src/plugin_tool_manifest_schema_cutover_preflight.rs",
    lib_export_present:$lib_export_present,
    parser_input_fields:[
      "contribution_candidate_ids",
      "tool_schemas",
      "permissions",
      "activation_events",
      "tool_policies",
      "schema_complete_candidate_ids",
      "policy_complete_candidate_ids"
    ],
    parser_input_field_count:7,
    planned_candidate_count:$source.planned_registry_entry_count,
    parsed_manifest_declared_candidate_count:$parser.current_fixture_declared_candidate_count,
    parsed_manifest_schema_complete_count:$parser.current_fixture_schema_complete_count,
    parsed_manifest_policy_complete_count:$parser.current_fixture_policy_complete_count,
    preflight_adapter_bridge_ready:true,
    registration_cutover_allowed:false,
    registration_execution_enabled:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    mcp_server_started:false,
    app_connector_started:false,
    live_mutation_ready:false,
    next_migration_step:"restore_plugin_tool_manifest_schema_cutover_preflight_without_registration",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_adapter:"codex-rs/tools/src/plugin_tool_manifest_schema_cutover_preflight.rs",
      parser_report:"scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh",
      source_registry_dry_run_report:"scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh"
    },
    blockers:[
      "registration_cutover_disallowed",
      "tool_registry_registration_disabled",
      "tool_invocation_disabled"
    ],
    next_actions:[
      "restore_plugin_tool_manifest_schema_cutover_preflight_without_registration",
      "keep_preflight_adapter_read_only_until_manifest_fixture_declarations_exist",
      "keep_registration_invocation_ledger_and_approval_disabled_until_operator_approval"
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
      manifest_schema_written:false,
      loader_invoked:false,
      registry_source_of_truth_enabled:false,
      registration_cutover_executed:false,
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
