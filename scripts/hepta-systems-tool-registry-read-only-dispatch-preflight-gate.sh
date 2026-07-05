#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh"
PLUGIN_LIFECYCLE_GATE="$ROOT/scripts/hepta-systems-plugin-lifecycle-state-machine-gate.sh"
INVOCATION_SOURCE_GATE="$ROOT/scripts/hepta-systems-tool-registry-invocation-source-of-truth-gate.sh"
LOOKUP_SHADOW_GATE="$ROOT/scripts/hepta-systems-tool-registry-router-lookup-shadow-gate.sh"
LEDGER_APPROVAL_GATE="$ROOT/scripts/hepta-systems-tool-invocation-ledger-approval-preflight-gate.sh"
RECEIPT_PROJECTION_GATE="$ROOT/scripts/hepta-systems-tool-invocation-receipt-projection-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_READ_ONLY_DISPATCH_PREFLIGHT_2026-06-27.md"

fail() {
  printf 'hepta-systems-tool-registry-read-only-dispatch-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable read-only dispatch preflight report: $REPORT"
[[ -x "$PLUGIN_LIFECYCLE_GATE" ]] || fail "missing executable plugin lifecycle gate: $PLUGIN_LIFECYCLE_GATE"
[[ -x "$INVOCATION_SOURCE_GATE" ]] || fail "missing executable invocation source gate: $INVOCATION_SOURCE_GATE"
[[ -x "$LOOKUP_SHADOW_GATE" ]] || fail "missing executable lookup shadow gate: $LOOKUP_SHADOW_GATE"
[[ -x "$LEDGER_APPROVAL_GATE" ]] || fail "missing executable ledger approval gate: $LEDGER_APPROVAL_GATE"
[[ -x "$RECEIPT_PROJECTION_GATE" ]] || fail "missing executable receipt projection gate: $RECEIPT_PROJECTION_GATE"
[[ -f "$DOC" ]] || fail "missing read-only dispatch preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the read-only dispatch preflight report"
fi

grep -q 'Read-Only Dispatch Preflight' "$DOC" \
  || fail "architecture note must document Read-Only Dispatch Preflight"
grep -q 'plugin lifecycle' "$DOC" \
  || fail "architecture note must document plugin lifecycle source of truth"
grep -q 'without invocation' "$DOC" \
  || fail "architecture note must document without invocation"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "tool_registry_read_only_dispatch_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_plugin_lifecycle_surface == "plugin_lifecycle_state_machine"
  and .source_plugin_lifecycle_ready == true
  and .source_plugin_lifecycle_phase_count == 6
  and .source_invocation_surface == "tool_registry_invocation_source_of_truth"
  and .source_invocation_ready == true
  and .source_lookup_shadow_surface == "tool_registry_router_lookup_shadow"
  and .source_lookup_shadow_ready == true
  and .source_ledger_approval_preflight_surface == "tool_invocation_ledger_approval_preflight"
  and .source_ledger_approval_preflight_ready == true
  and .source_receipt_projection_surface == "tool_invocation_receipt_projection"
  and .source_receipt_projection_ready == true
  and .lib_export_present == true
  and .dispatch_preflight_binding_present == true
  and .candidate_count == 2
  and .dispatch_preflight_ready_count == 2
  and .dispatch_preflight_blocked_count == 0
  and .registry_lookup_preview_required_count == 2
  and .ledger_preview_required_count == 2
  and .approval_preflight_required_count == 2
  and .receipt_projection_required_count == 2
  and .all_entries_bound_to_plugin_lifecycle == true
  and .all_entries_bound_to_read_only_dispatch_preflight == true
  and .all_dispatch_entries_keep_no_invocation_guard == true
  and .read_only_dispatch_preflight_ready == true
  and .read_only_dispatch_preflight_allowed == true
  and .registry_dispatch_switch_enabled == false
  and .router_registration_lookup_enabled == false
  and .registry_lookup_executed == false
  and .registry_source_of_truth_enabled == false
  and .tool_registration_enabled == false
  and .tool_invocation_enabled == false
  and .ledger_written == false
  and .approval_requested == false
  and .result_receipt_written == false
  and .live_mutation_ready == false
  and .next_migration_step == "phase3_rebuild_temporal_lite_event_log_adapter_behind_feature_gate"
  and (.entries | length) == 2
  and any(.entries[]; .contribution_kind == "mcp_server" and .source_invocation_route == "approval_ledger_dry_run_source_only" and .lookup_shadow_route == "disabled_approval_ledger_lookup_shadow" and .ledger_preflight_route == "approval_ledger_preflight_required" and .receipt_projection_route == "result_receipt_projection_required" and .dispatch_preflight_route == "read_only_dispatch_receipt_projection_ready" and .dispatch_preflight_ready == true and .registry_lookup_preview_required == true and .ledger_preview_required == true and .approval_preflight_required == true and .receipt_projection_required == true and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and any(.entries[]; .contribution_kind == "app_connector" and .source_invocation_route == "approval_ledger_dry_run_source_only" and .lookup_shadow_route == "disabled_approval_ledger_lookup_shadow" and .ledger_preflight_route == "approval_ledger_preflight_required" and .receipt_projection_route == "result_receipt_projection_required" and .dispatch_preflight_route == "read_only_dispatch_receipt_projection_ready" and .dispatch_preflight_ready == true and .registry_lookup_preview_required == true and .ledger_preview_required == true and .approval_preflight_required == true and .receipt_projection_required == true and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false)
  and (.blockers | index("registry_dispatch_switch_disabled")) != null
  and (.blockers | index("workflow_durable_event_log_adapter_pending")) != null
  and (.next_actions | index("phase3_rebuild_temporal_lite_event_log_adapter_behind_feature_gate")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PLUGIN_LIFECYCLE_GATE" >/dev/null
"$INVOCATION_SOURCE_GATE" >/dev/null
"$LOOKUP_SHADOW_GATE" >/dev/null
"$LEDGER_APPROVAL_GATE" >/dev/null
"$RECEIPT_PROJECTION_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p codex-tools tool_registry_read_only_dispatch_preflight --quiet
)

printf 'hepta-systems-tool-registry-read-only-dispatch-preflight-gate: PASS: plugin lifecycle-backed ToolRegistry dispatch preflight is ready while invocation, writes, approvals, and live mutation remain disabled\n'
