#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-plugin-tool-invocation-router-preflight-binding-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_INVOCATION_SOURCE_OF_TRUTH_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-registry-invocation-source-of-truth-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable tool registry invocation source-of-truth report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable invocation router preflight binding gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing tool registry invocation source-of-truth architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the tool registry invocation source-of-truth report"
fi

grep -q 'Tool Registry Invocation Source Of Truth' "$DOC" \
  || fail "architecture note must document Tool Registry Invocation Source Of Truth"
grep -q 'approval-ledger dry-run' "$DOC" \
  || fail "architecture note must document approval-ledger dry-run source"
grep -q 'without execution' "$DOC" \
  || fail "architecture note must document without execution"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_registry_invocation_source_of_truth"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_router_preflight_surface == "plugin_tool_invocation_router_preflight_binding"
  and .source_router_preflight_ready == true
  and .source_router_forward_count == 2
  and .candidate_count == 2
  and .invocation_source_ready_count == 2
  and .invocation_source_blocked_count == 0
  and .approval_ledger_dry_run_source_count == 2
  and .all_forwarded_candidates_bound_to_invocation_source == true
  and .all_invocation_sources_keep_approval_ledger_guard == true
  and .invocation_source_of_truth_plan_ready == true
  and .router_registration_lookup_enabled == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .live_mutation_ready == false
  and .next_migration_step == "restore_tool_registry_registration_lookup_cutover_preflight_without_execution"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .registry_guard_route == "require_approval_ledger" and .router_decision_route == "forward_require_approval_ledger_dry_run" and .invocation_source_route == "approval_ledger_dry_run_source_only" and .invocation_source_ready == true and .router_registration_lookup_enabled == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .registry_guard_route == "require_approval_ledger" and .router_decision_route == "forward_require_approval_ledger_dry_run" and .invocation_source_route == "approval_ledger_dry_run_source_only" and .invocation_source_ready == true and .router_registration_lookup_enabled == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false)
  and (.blockers | index("router_registration_lookup_disabled")) != null
  and (.blockers | index("registry_source_of_truth_enablement_disabled")) != null
  and (.blockers | index("tool_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.next_actions | index("restore_tool_registry_registration_lookup_cutover_preflight_without_execution")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_registry_invocation_source_of_truth --quiet
)

printf 'hepta-systems-tool-registry-invocation-source-of-truth-gate: PASS: router dry-run candidates are bound to a read-only invocation source of truth while registration, invocation, ledger, and approval paths remain disabled\n'
