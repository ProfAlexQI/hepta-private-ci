#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
HEPTA_SYSTEM_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"
SOURCE="$ROOT/codex-rs/core-plugins/src/manifest.rs"
SOURCE_OF_TRUTH_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh"
GATE="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PARSER_FIELDS_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-manifest-parser-fields-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -f "$SOURCE" ]] || fail "missing core plugin manifest parser source: $SOURCE"
[[ -x "$SOURCE_OF_TRUTH_REPORT" ]] || fail "missing executable source-of-truth dry-run report: $SOURCE_OF_TRUTH_REPORT"
[[ -f "$DOC" ]] || fail "missing plugin tool manifest parser fields architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool manifest parser fields report"
fi

hepta_system_manifest_present=false
manifest_json='{}'
if [[ -f "$HEPTA_SYSTEM_MANIFEST" ]]; then
  hepta_system_manifest_present=true
  manifest_json="$(jq '.' "$HEPTA_SYSTEM_MANIFEST")"
fi

parser_source_has_tool_declarations=false
if grep -q 'tool_declarations' "$SOURCE" \
  && grep -q 'tool_schemas' "$SOURCE" \
  && grep -q 'activation_events' "$SOURCE" \
  && grep -q 'tool_policies' "$SOURCE"; then
  parser_source_has_tool_declarations=true
fi

jq -n \
  --slurpfile source <("$SOURCE_OF_TRUTH_REPORT") \
  --argjson manifest "$manifest_json" \
  --argjson hepta_system_manifest_present "$hepta_system_manifest_present" \
  --argjson parser_source_has_tool_declarations "$parser_source_has_tool_declarations" \
  --arg gate "scripts/hepta-systems-plugin-tool-manifest-parser-fields-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PARSER_FIELDS_2026-06-21.md" \
  '
  def object_or_empty($value):
    if ($value | type) == "object" then $value else {} end;
  def keys_of($value):
    object_or_empty($value) | keys;
  def has_id($ids; $id):
    ($ids | index($id)) != null;
  def has_field($object; $id; $field):
    (object_or_empty($object)[$id]? // null) as $entry
    | (($entry | type) == "object" and ($entry[$field]? != null));
  def schema_complete_ids($schemas):
    keys_of($schemas)
    | map(
        select(
          . as $id
          | has_field($schemas; $id; "inputSchema")
            and has_field($schemas; $id; "outputSchema")
        )
      );
  def policy_complete_ids($permissions; $activation_events; $tool_policies):
    keys_of($tool_policies)
    | map(
        select(
          . as $id
          | has_id(keys_of($permissions); $id)
            and has_id(keys_of($activation_events); $id)
            and has_field($tool_policies; $id; "approval")
            and has_field($tool_policies; $id; "ledger")
            and has_field($tool_policies; $id; "timeoutMs")
        )
      );
  def parser_entry($tool_schemas; $permissions; $activation_events; $tool_policies; $schema_complete_ids; $policy_complete_ids; $candidate_tool_id):
    {
      candidate_tool_id:$candidate_tool_id,
      tool_schema_declared:has_id(keys_of($tool_schemas); $candidate_tool_id),
      input_schema_declared:has_field($tool_schemas; $candidate_tool_id; "inputSchema"),
      output_schema_declared:has_field($tool_schemas; $candidate_tool_id; "outputSchema"),
      permission_policy_declared:has_id(keys_of($permissions); $candidate_tool_id),
      activation_policy_declared:has_id(keys_of($activation_events); $candidate_tool_id),
      approval_policy_declared:has_field($tool_policies; $candidate_tool_id; "approval"),
      ledger_policy_declared:has_field($tool_policies; $candidate_tool_id; "ledger"),
      timeout_policy_declared:has_field($tool_policies; $candidate_tool_id; "timeoutMs"),
      manifest_schema_complete:has_id($schema_complete_ids; $candidate_tool_id),
      manifest_policy_complete:has_id($policy_complete_ids; $candidate_tool_id)
    };

  ($source[0]) as $source |
  ($manifest.toolSchemas? // null) as $tool_schemas |
  ($manifest.permissions? // null) as $permissions |
  ($manifest.activationEvents? // null) as $activation_events |
  ($manifest.toolPolicies? // null) as $tool_policies |
  keys_of($tool_schemas) as $tool_schema_ids |
  keys_of($permissions) as $permission_ids |
  keys_of($activation_events) as $activation_event_ids |
  keys_of($tool_policies) as $tool_policy_ids |
  schema_complete_ids($tool_schemas) as $schema_complete_ids |
  policy_complete_ids($permissions; $activation_events; $tool_policies) as $policy_complete_ids |
  (
    $tool_schema_ids
    + $permission_ids
    + $activation_event_ids
    + $tool_policy_ids
    | unique
  ) as $declared_candidate_ids |
  {
    runtime:"hepta",
    surface:"plugin_tool_manifest_parser_fields",
    plugin_id:$source.plugin_id,
    status:"ready",
    parser_source:"codex-rs/core-plugins/src/manifest.rs",
    parser_source_has_tool_declarations:$parser_source_has_tool_declarations,
    source_registry_dry_run_surface:$source.surface,
    source_registry_dry_run_ready:$source.registry_source_of_truth_dry_run_ready,
    parser_supported_fields:[
      "toolSchemas",
      "permissions",
      "activationEvents",
      "toolPolicies"
    ],
    parser_supported_field_count:4,
    current_fixture_manifest:"plugins/hepta-system/.codex-plugin/plugin.json",
    hepta_system_manifest_present:$hepta_system_manifest_present,
    current_fixture_tool_schema_candidate_ids:$tool_schema_ids,
    current_fixture_permission_candidate_ids:$permission_ids,
    current_fixture_activation_event_candidate_ids:$activation_event_ids,
    current_fixture_tool_policy_candidate_ids:$tool_policy_ids,
    current_fixture_tool_schema_count:($tool_schema_ids | length),
    current_fixture_permission_count:($permission_ids | length),
    current_fixture_activation_event_count:($activation_event_ids | length),
    current_fixture_tool_policy_count:($tool_policy_ids | length),
    current_fixture_declared_candidate_ids:$declared_candidate_ids,
    current_fixture_declared_candidate_count:($declared_candidate_ids | length),
    current_fixture_schema_complete_candidate_ids:$schema_complete_ids,
    current_fixture_schema_complete_count:($schema_complete_ids | length),
    current_fixture_policy_complete_candidate_ids:$policy_complete_ids,
    current_fixture_policy_complete_count:($policy_complete_ids | length),
    current_fixture_entries:(
      $declared_candidate_ids
      | map(parser_entry(
          $tool_schemas;
          $permissions;
          $activation_events;
          $tool_policies;
          $schema_complete_ids;
          $policy_complete_ids;
          .
        ))
    ),
    parser_fields_ready:true,
    parsed_declarations_feed_preflight:true,
    manifest_declarations_present:(($declared_candidate_ids | length) > 0),
    current_fixture_registration_preconditions_satisfied:(
      (($declared_candidate_ids | length) > 0)
      and (($declared_candidate_ids | length) == ($schema_complete_ids | length))
      and (($declared_candidate_ids | length) == ($policy_complete_ids | length))
    ),
    registration_cutover_allowed:false,
    registration_execution_enabled:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    mcp_server_started:false,
    app_connector_started:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_registry_invocation_source_of_truth_without_execution",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      parser_source:"codex-rs/core-plugins/src/manifest.rs",
      source_registry_dry_run_report:"scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh"
    },
    blockers:[
      "registration_cutover_disallowed",
      "tool_registry_registration_disabled",
      "tool_invocation_disabled"
    ],
    next_actions:[
      "restore_tool_registry_invocation_source_of_truth_without_execution",
      "keep_parser_output_read_only_until_router_preflight_binding_is_restored",
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
