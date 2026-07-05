#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-router-preflight-binding-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_ROUTER_PREFLIGHT_BINDING_2026-06-21.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-router-preflight-binding-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable plugin tool invocation router preflight binding report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable manifest schema preflight gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing plugin tool invocation router preflight binding architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool invocation router preflight binding report"
fi

grep -q 'Invocation Router Preflight Binding' "$DOC" \
  || fail "architecture note must document Invocation Router Preflight Binding"
grep -q 'forward dry-run path' "$DOC" \
  || fail "architecture note must document forward dry-run path"
grep -q 'without registration' "$DOC" \
  || fail "architecture note must document without registration"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "plugin_tool_invocation_router_preflight_binding"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_registry_dry_run_surface == "plugin_tool_registry_source_of_truth_dry_run"
  and .source_registry_dry_run_ready == true
  and .source_manifest_schema_preflight_surface == "plugin_tool_manifest_schema_cutover_preflight"
  and .source_manifest_schema_preflight_ready == true
  and .source_manifest_parser_fields_surface == "plugin_tool_manifest_parser_fields"
  and .source_manifest_parser_fields_ready == true
  and .candidate_count == 2
  and .router_bound_candidate_count == 2
  and .router_unbound_candidate_count == 0
  and .router_blocked_candidate_count == 0
  and .router_blocked_by_source_registry_count == 0
  and .router_blocked_by_manifest_precondition_count == 0
  and .router_forward_require_approval_ledger_count == 2
  and .registration_precondition_satisfied_count == 2
  and .registration_cutover_allowed == true
  and .all_candidates_bound_to_router == true
  and .all_missing_manifest_preconditions_blocked == true
  and .all_forwarded_candidates_keep_approval_ledger == true
  and .invocation_router_preflight_binding_ready == true
  and .router_registration_lookup_enabled == false
  and .registration_execution_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_invocation_source_of_truth_without_execution"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .registry_guard_route == "require_approval_ledger" and .registration_preconditions_satisfied == true and .router_decision_route == "forward_require_approval_ledger_dry_run" and .router_blocked == false and .router_blocked_reason == null and .tool_registration_enabled == false and .tool_invocation_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .registry_guard_route == "require_approval_ledger" and .registration_preconditions_satisfied == true and .router_decision_route == "forward_require_approval_ledger_dry_run" and .router_blocked == false and .router_blocked_reason == null and .tool_registration_enabled == false and .tool_invocation_enabled == false)
  and (.blockers | index("registration_execution_disabled")) != null
  and (.blockers | index("router_registration_lookup_disabled")) != null
  and (.blockers | index("tool_registration_disabled")) != null
  and (.next_actions | index("restore_tool_registry_invocation_source_of_truth_without_execution")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools plugin_tool_invocation_router_preflight_binding --quiet
)

printf 'hepta-systems-plugin-tool-invocation-router-preflight-binding-gate: PASS: manifest schema preflight is bound into invocation router planning while registration and invocation remain disabled\n'
