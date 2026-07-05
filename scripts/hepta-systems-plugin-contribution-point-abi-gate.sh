#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-contribution-point-abi-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_CONTRIBUTION_POINT_ABI_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-contribution-point-abi-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable contribution-point ABI report: $REPORT"
[[ -f "$DOC" ]] || fail "missing contribution-point ABI architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the contribution-point ABI report"
fi

grep -q 'Contribution Point ABI' "$DOC" \
  || fail "architecture note must document Contribution Point ABI"
grep -q 'ToolRegistry' "$DOC" \
  || fail "architecture note must document ToolRegistry bridge"
grep -q 'live mutation disabled' "$DOC" \
  || fail "architecture note must document live mutation disabled boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "plugin_contribution_point_abi"
  and .status == "ready"
  and .registry_api == "hepta.systems.pluginRegistry/v1"
  and .lib_export_present == true
  and .loader_binding_export_present == true
  and .contribution_point_count == 8
  and .manifest_path_supported_kinds == ["skill","mcp_server","app_connector","hook"]
  and .manifest_path_supported_count == 4
  and .loader_path_supported_count == 4
  and .current_fixture_declared_count == 0
  and .future_bridge_kinds == ["tool","permission","activation_event","local_storage"]
  and .future_bridge_required_count == 4
  and .policy.all_entries_policy_bound == true
  and .policy.permission_policy_required == true
  and .policy.activation_policy_required == true
  and .policy.ledger_required_count == 4
  and .policy.mutating_entries_require_approval == true
  and .bridges.manifest_loader_bridge_ready == true
  and .bridges.tool_registry_bridge_required == true
  and .bridges.tool_registry_bridge_enabled == false
  and .bridges.permission_manifest_field_pending == true
  and .bridges.activation_manifest_field_pending == true
  and .bridges.local_storage_manifest_field_pending == true
  and .bridges.local_storage_scoped_to_plugin_data_root == true
  and .abi_ready == true
  and .runtime_execution_enabled == false
  and .all_runtime_execution_disabled == true
  and .all_live_paths_blocked == true
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_invocation_source_of_truth_without_execution"
  and (.blockers | index("hepta_system_fixture_not_restored")) != null
  and (.blockers | index("plugin_tool_invocation_router_preflight_binding_not_restored")) != null
  and (.blockers | index("tool_contribution_bridge_to_tool_registry_pending")) != null
  and (.next_actions | index("restore_tool_registry_invocation_source_of_truth_without_execution")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-core-plugins contribution_point_abi --quiet
)

printf 'hepta-systems-plugin-contribution-point-abi-gate: PASS: plugin contribution-point ABI is restored locally and execution remains disabled\n'
