#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-report.sh"
PARSER_GATE="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_MANIFEST_PREFLIGHT_ADAPTER_BRIDGE_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable manifest preflight adapter bridge report: $REPORT"
[[ -x "$PARSER_GATE" ]] || fail "missing executable plugin tool manifest parser fields gate: $PARSER_GATE"
[[ -f "$DOC" ]] || fail "missing manifest preflight adapter bridge architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool manifest preflight adapter bridge report"
fi

grep -q 'Manifest Preflight Adapter Bridge' "$DOC" \
  || fail "architecture note must document Manifest Preflight Adapter Bridge"
grep -q 'parser-shaped input' "$DOC" \
  || fail "architecture note must document parser-shaped input"
grep -q 'without registration' "$DOC" \
  || fail "architecture note must document without registration"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "plugin_tool_manifest_preflight_adapter_bridge"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_registry_dry_run_surface == "plugin_tool_registry_source_of_truth_dry_run"
  and .source_registry_dry_run_ready == true
  and .source_manifest_parser_fields_surface == "plugin_tool_manifest_parser_fields"
  and .source_manifest_parser_fields_ready == true
  and .parsed_manifest_declarations_feed_preflight == true
  and .lib_export_present == true
  and .parser_input_fields == ["contribution_candidate_ids","tool_schemas","permissions","activation_events","tool_policies","schema_complete_candidate_ids","policy_complete_candidate_ids"]
  and .parser_input_field_count == 7
  and .planned_candidate_count == 2
  and .parsed_manifest_declared_candidate_count == 2
  and .parsed_manifest_schema_complete_count == 2
  and .parsed_manifest_policy_complete_count == 2
  and .preflight_adapter_bridge_ready == true
  and .registration_cutover_allowed == false
  and .registration_execution_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .mcp_server_started == false
  and .app_connector_started == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_plugin_tool_manifest_schema_cutover_preflight_without_registration"
  and (.next_actions | index("restore_plugin_tool_manifest_schema_cutover_preflight_without_registration")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PARSER_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools plugin_tool_manifest_schema_cutover_preflight --quiet
)

printf 'hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-gate: PASS: parsed manifest declarations are bridged into the tool preflight adapter and execution remains disabled\n'
