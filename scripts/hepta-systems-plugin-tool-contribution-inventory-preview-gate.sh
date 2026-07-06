#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh"
LOADER_BINDING_GATE="$ROOT/scripts/hepta-systems-plugin-contribution-point-loader-binding-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_CONTRIBUTION_INVENTORY_PREVIEW_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-contribution-inventory-preview-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable plugin tool contribution inventory preview report: $REPORT"
[[ -x "$LOADER_BINDING_GATE" ]] || fail "missing executable loader binding gate: $LOADER_BINDING_GATE"
[[ -f "$DOC" ]] || fail "missing plugin tool contribution inventory preview architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool contribution inventory preview report"
fi

grep -q 'Plugin Tool Contribution Inventory Preview' "$DOC" \
  || fail "architecture note must document Plugin Tool Contribution Inventory Preview"
grep -q 'ToolRegistry' "$DOC" \
  || fail "architecture note must document ToolRegistry candidate mapping"
grep -q 'manifest fixture readback' "$DOC" \
  || fail "architecture note must document manifest fixture readback"
grep -q 'live mutation disabled' "$DOC" \
  || fail "architecture note must document live mutation disabled boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "plugin_tool_contribution_inventory_preview"
  and .status == "ready"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_loader_binding_surface == "plugin_contribution_point_loader_binding"
  and .source_loader_binding_ready == true
  and .source_loader_contract_ready == true
  and .hepta_system_manifest_present == true
  and .lib_export_present == true
  and .tool_registry_inventory_export_present == true
  and .candidate_source == "manifest_fixture_readback_without_registration"
  and .candidate_count == 2
  and .current_fixture_candidate_count == 2
  and .planned_candidate_count == 2
  and .candidate_tool_ids == [
    "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
    "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
  ]
  and .candidate_kinds == ["mcp_server","app_connector"]
  and .skipped_loader_bound_non_tool_kinds == ["skill","hook"]
  and .candidate_inventory_sources == ["mcp","connector"]
  and .candidate_loader_output_fields == ["mcp_servers","apps"]
  and .candidate_side_effect_levels == ["local_mutation","external_mutation"]
  and .candidate_approval_kinds == ["on_use","install"]
  and .candidate_auth_required == [false,true]
  and .candidate_timeout_ms == [30000,30000]
  and .candidate_ledger_required == [true,true]
  and .candidate_guard_routes == ["require_approval_ledger","require_approval_ledger"]
  and .tool_contribution_schema_complete_count == 2
  and .tool_contribution_risk_metadata_complete_count == 2
  and .tool_contribution_ledger_required_count == 2
  and .tool_contribution_approval_required_count == 2
  and .all_candidates_have_schema == true
  and .all_candidates_have_risk_metadata == true
  and .all_candidates_require_ledger == true
  and .mutating_candidates_require_approval == true
  and .all_candidates_have_guard_route == true
  and .inventory_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .mcp_server_started == false
  and .app_connector_started == false
  and .preview_ready == true
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_invocation_source_of_truth_without_execution"
  and (.blockers | index("plugin_tool_invocation_router_preflight_binding_not_restored")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.next_actions | index("restore_tool_registry_invocation_source_of_truth_without_execution")) != null
  and (.next_actions | index("keep_parser_output_read_only_until_preflight_adapter_is_restored")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$LOADER_BINDING_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools plugin_tool_contribution_inventory_preview --quiet
  cargo test -p codex-tools tool_registry_inventory --quiet
)

printf 'hepta-systems-plugin-tool-contribution-inventory-preview-gate: PASS: plugin tool contribution inventory preview is restored locally and execution remains disabled\n'
