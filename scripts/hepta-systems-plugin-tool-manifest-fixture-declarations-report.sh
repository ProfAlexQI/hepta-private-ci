#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PARSER_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh"
GATE="$ROOT/scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_FIXTURE_DECLARATIONS_2026-06-21.md"
HEPTA_SYSTEM_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"

fail() {
  printf 'hepta-systems-plugin-tool-manifest-fixture-declarations-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PARSER_REPORT" ]] || fail "missing executable plugin tool manifest parser fields report: $PARSER_REPORT"
[[ -f "$DOC" ]] || fail "missing plugin tool manifest fixture declarations architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool manifest fixture declarations report"
fi

hepta_system_manifest_present=false
if [[ -f "$HEPTA_SYSTEM_MANIFEST" ]]; then
  hepta_system_manifest_present=true
fi

jq -n \
  --slurpfile parser <("$PARSER_REPORT") \
  --argjson hepta_system_manifest_present "$hepta_system_manifest_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_FIXTURE_DECLARATIONS_2026-06-21.md" \
  '
  ($parser[0]) as $parser |
  {
    runtime:"hepta",
    surface:"plugin_tool_manifest_fixture_declarations",
    plugin_id:$parser.plugin_id,
    status:"ready",
    source_manifest_parser_fields_surface:$parser.surface,
    source_manifest_parser_fields_ready:$parser.parser_fields_ready,
    current_fixture_manifest:"plugins/hepta-system/.codex-plugin/plugin.json",
    hepta_system_manifest_present:$hepta_system_manifest_present,
    declaration_source:"hepta_system_manifest_fixture_readback",
    current_fixture_tool_schema_candidate_ids:$parser.current_fixture_tool_schema_candidate_ids,
    current_fixture_permission_candidate_ids:$parser.current_fixture_permission_candidate_ids,
    current_fixture_activation_event_candidate_ids:$parser.current_fixture_activation_event_candidate_ids,
    current_fixture_tool_policy_candidate_ids:$parser.current_fixture_tool_policy_candidate_ids,
    current_fixture_tool_schema_count:$parser.current_fixture_tool_schema_count,
    current_fixture_permission_count:$parser.current_fixture_permission_count,
    current_fixture_activation_event_count:$parser.current_fixture_activation_event_count,
    current_fixture_tool_policy_count:$parser.current_fixture_tool_policy_count,
    current_fixture_declared_candidate_ids:$parser.current_fixture_declared_candidate_ids,
    current_fixture_declared_candidate_count:$parser.current_fixture_declared_candidate_count,
    current_fixture_schema_complete_candidate_ids:$parser.current_fixture_schema_complete_candidate_ids,
    current_fixture_schema_complete_count:$parser.current_fixture_schema_complete_count,
    current_fixture_policy_complete_candidate_ids:$parser.current_fixture_policy_complete_candidate_ids,
    current_fixture_policy_complete_count:$parser.current_fixture_policy_complete_count,
    current_fixture_registration_preconditions_satisfied:$parser.current_fixture_registration_preconditions_satisfied,
    manifest_fixture_declarations_ready:true,
    registration_cutover_allowed:false,
    registration_execution_enabled:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    mcp_server_started:false,
    app_connector_started:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_registry_invocation_source_of_truth_without_execution",
    entries:$parser.current_fixture_entries,
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      parser_report:"scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh"
    },
    blockers:[
      "registration_execution_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled"
    ],
    next_actions:[
      "restore_tool_registry_invocation_source_of_truth_without_execution",
      "keep_manifest_fixture_readback_side_effect_free",
      "keep_registration_invocation_ledger_and_approval_disabled_until_explicit_cutover"
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
