#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-registry-invocation-source-of-truth-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_REGISTRATION_LOOKUP_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable registration lookup cutover preflight report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable invocation source-of-truth gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing registration lookup cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the registration lookup cutover preflight report"
fi

grep -q 'Registration Lookup Cutover Preflight' "$DOC" \
  || fail "architecture note must document Registration Lookup Cutover Preflight"
grep -q 'approval-ledger lookup dry-run' "$DOC" \
  || fail "architecture note must document approval-ledger lookup dry-run"
grep -q 'without execution' "$DOC" \
  || fail "architecture note must document without execution"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_registry_registration_lookup_cutover_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_invocation_surface == "tool_registry_invocation_source_of_truth"
  and .source_invocation_ready == true
  and .source_invocation_ready_count == 2
  and .candidate_count == 2
  and .lookup_precondition_satisfied_count == 2
  and .lookup_blocked_count == 0
  and .approval_ledger_lookup_dry_run_count == 2
  and .all_invocation_sources_bound_to_lookup_preflight == true
  and .all_lookup_entries_keep_approval_ledger_guard == true
  and .registration_lookup_cutover_preflight_ready == true
  and .registration_lookup_cutover_allowed == true
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_router_lookup_shadow_without_registration"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .registry_guard_route == "require_approval_ledger" and .source_invocation_route == "approval_ledger_dry_run_source_only" and .lookup_cutover_route == "approval_ledger_lookup_dry_run" and .lookup_precondition_satisfied == true and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .registry_guard_route == "require_approval_ledger" and .source_invocation_route == "approval_ledger_dry_run_source_only" and .lookup_cutover_route == "approval_ledger_lookup_dry_run" and .lookup_precondition_satisfied == true and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and (.blockers | index("router_registration_lookup_disabled")) != null
  and (.blockers | index("registry_lookup_execution_disabled")) != null
  and (.blockers | index("tool_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.next_actions | index("restore_tool_registry_router_lookup_shadow_without_registration")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_registry_registration_lookup_cutover_preflight --quiet
)

printf 'hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate: PASS: registration lookup cutover preconditions are ready while lookup, registration, invocation, ledger, and approval execution remain disabled\n'
