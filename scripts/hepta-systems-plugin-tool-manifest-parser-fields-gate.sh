#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh"
SOURCE_OF_TRUTH_GATE="$ROOT/scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PARSER_FIELDS_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-manifest-parser-fields-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable plugin tool manifest parser fields report: $REPORT"
[[ -x "$SOURCE_OF_TRUTH_GATE" ]] || fail "missing executable source-of-truth dry-run gate: $SOURCE_OF_TRUTH_GATE"
[[ -f "$DOC" ]] || fail "missing plugin tool manifest parser fields architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool manifest parser fields report"
fi

grep -q 'Plugin Tool Manifest Parser Fields' "$DOC" \
  || fail "architecture note must document Plugin Tool Manifest Parser Fields"
grep -q 'toolSchemas' "$DOC" \
  || fail "architecture note must document toolSchemas"
grep -q 'Manifest schema preflight' "$DOC" \
  || fail "architecture note must document manifest schema preflight follow-up"
grep -q 'live mutation disabled' "$DOC" \
  || fail "architecture note must document live mutation disabled boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "plugin_tool_manifest_parser_fields"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .parser_source == "codex-rs/core-plugins/src/manifest.rs"
  and .parser_source_has_tool_declarations == true
  and .source_registry_dry_run_surface == "plugin_tool_registry_source_of_truth_dry_run"
  and .source_registry_dry_run_ready == true
  and .parser_supported_fields == ["toolSchemas","permissions","activationEvents","toolPolicies"]
  and .parser_supported_field_count == 4
  and .current_fixture_manifest == "plugins/hepta-system/.codex-plugin/plugin.json"
  and .hepta_system_manifest_present == true
  and .current_fixture_tool_schema_count == 2
  and .current_fixture_permission_count == 2
  and .current_fixture_activation_event_count == 2
  and .current_fixture_tool_policy_count == 2
  and .current_fixture_declared_candidate_count == 2
  and .current_fixture_schema_complete_count == 2
  and .current_fixture_policy_complete_count == 2
  and (.current_fixture_entries | length) == 2
  and .parser_fields_ready == true
  and .parsed_declarations_feed_preflight == true
  and .manifest_declarations_present == true
  and .current_fixture_registration_preconditions_satisfied == true
  and .registration_cutover_allowed == false
  and .registration_execution_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .mcp_server_started == false
  and .app_connector_started == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_invocation_source_of_truth_without_execution"
  and (.blockers | index("registration_cutover_disallowed")) != null
  and (.next_actions | index("restore_tool_registry_invocation_source_of_truth_without_execution")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_OF_TRUTH_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-core-plugins plugin_manifest_tool_declaration_parser --quiet
)

printf 'hepta-systems-plugin-tool-manifest-parser-fields-gate: PASS: plugin manifest parser fields are restored locally and execution remains disabled\n'
