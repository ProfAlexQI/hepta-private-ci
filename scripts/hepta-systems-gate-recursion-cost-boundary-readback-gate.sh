#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-gate-recursion-cost-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-gate-recursion-cost-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable gate recursion cost report: $REPORT"
[[ -f "$DOC" ]] || fail "missing gate recursion cost architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the gate recursion cost boundary report"
fi

grep -q 'Hepta Systems Gate Recursion Cost Boundary Readback' "$DOC" \
  || fail "architecture note must document Hepta Systems Gate Recursion Cost Boundary Readback"
grep -q 'readback-only gate recursion cost boundary' "$DOC" \
  || fail "architecture note must document readback-only gate recursion cost boundary"
grep -q 'no matrix cache write, compact cache persistence, source report semantic change, full upstream gate-chain invocation, workflow execution, replay execution, event-log write, SQLite write, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed gate recursion cost boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_gate_recursion_cost_boundary_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_gate_recursion_cost_boundary_readback_gate"
  and .schema_version == "hepta_systems_gate_recursion_cost_boundary_readback_v1"
  and .source_matrix_capability_count == 58
  and .source_matrix_live_enabled_count == 0
  and .controlled_live_blocker_count == 7
  and .lib_export_present == true
  and .recovery_receipt_uses_source_report == true
  and .upstream_recovery_window_uses_source_gate == true
  and .cost_scope == "readback_only_gate_recursion_cost_boundary_no_cache_write"
  and .boundary_projection_count == 4
  and .source_gate_recursion_boundary_count == 2
  and .bounded_source_gate_count == 1
  and .full_matrix_render_boundary_count == 1
  and .lane_lock_boundary_count == 1
  and .full_upstream_gate_chain_invocation_allowed == false
  and .matrix_report_cache_write_allowed == false
  and .compact_cache_persistence_allowed == false
  and .source_report_semantics_change_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and .gate_recursion_cost_boundary_readback_ready == true
  and (.entries | length) == 4
  and (.entries | all(.projected_in_memory == true and .matrix_cache_written == false and .compact_cache_persisted == false and .source_report_semantics_changed == false and .cargo_test_executed_by_report == false and .workflow_execution_started == false and .replay_executed == false and .event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and any(.entries[]; .entry_id == "recovery_receipt_source_report_invariant_boundary" and .source_gate_recursion_bounded == true and .full_upstream_gate_chain_invoked == false)
  and any(.entries[]; .entry_id == "upstream_recovery_window_source_gate_chain_boundary" and .source_gate_recursion_bounded == false and .full_upstream_gate_chain_invoked == true)
  and any(.entries[]; .entry_id == "current_reality_matrix_full_render_boundary" and .full_matrix_render_required == true)
  and any(.entries[]; .entry_id == "hepta_systems_lane_lock_serialization_boundary" and .lane_lock_serialization_required == true)
  and (.blockers | index("full_upstream_gate_chain_invocation_disabled")) != null
  and (.blockers | index("matrix_report_cache_write_disabled")) != null
  and (.blockers | index("compact_cache_persistence_disabled")) != null
  and (.blockers | index("source_report_semantics_change_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_matrix_report_single_render_cache_boundary_readback")) != null
  and .recommended_next_gate == "hepta_systems_matrix_report_single_render_cache_boundary_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_gate_recursion_cost_boundary_readback --lib
)

printf 'hepta-systems-gate-recursion-cost-boundary-readback-gate: PASS: gate recursion cost boundaries are queryable, source recursion is bounded for new gates, matrix cache writes remain disabled, and live remains blocked\n'
