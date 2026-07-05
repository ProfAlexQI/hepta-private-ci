#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-gate-recursion-lean-contract-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-gate-recursion-lean-contract-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable gate recursion lean contract report: $REPORT"
[[ -f "$DOC" ]] || fail "missing gate recursion lean contract architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the gate recursion lean contract report"
fi

rg -q 'Hepta Systems Gate Recursion Lean Contract Readback' "$DOC" \
  || fail "architecture note must document Hepta Systems Gate Recursion Lean Contract Readback"
rg -q 'source-report smoke plus targeted Rust test contract' "$DOC" \
  || fail "architecture note must document the source-report smoke plus targeted Rust test contract"
rg -q 'no matrix cache write, compact cache persistence, source report semantic change, recursive source-gate invocation for new gates, workflow execution, replay execution, event-log write, SQLite write, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed lean contract boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_gate_recursion_lean_contract_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_gate_recursion_lean_contract_readback_gate"
  and .schema_version == "hepta_systems_gate_recursion_lean_contract_readback_v1"
  and .source_cost_boundary_ready == true
  and .source_cost_boundary_projection_count == 4
  and .lib_export_present == true
  and .recovery_receipt_local_uses_source_report == true
  and .recovery_window_feature_uses_source_gate == true
  and .workgraph_legacy_gate_present == true
  and .single_render_matrix_once == true
  and .dashboard_uses_single_render == true
  and .contract_scope == "source_report_smoke_plus_targeted_test_no_recursive_source_gate_chain"
  and .contract_entry_count == 5
  and .source_report_smoke_contract_count == 3
  and .targeted_rust_test_contract_count == 3
  and .legacy_recursion_inventory_count == 2
  and .current_full_upstream_gate_chain_count == 2
  and .contract_full_upstream_gate_chain_allowed_count == 0
  and .matrix_cache_write_allowed == false
  and .compact_cache_persistence_allowed == false
  and .source_report_semantics_change_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and .lean_contract_readback_ready == true
  and (.entries | length) == 5
  and (.entries | all(.projected_in_memory == true and .observed_contract_ready == true and .contract_full_upstream_gate_chain_allowed == false and .downstream_direct_matrix_render_required == false and .matrix_cache_written == false and .compact_cache_persisted == false and .source_report_semantics_changed == false and .cargo_test_executed_by_report == false and .workflow_execution_started == false and .replay_executed == false and .event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and any(.entries[]; .entry_id == "recovery_receipt_local_source_report_contract" and .source_report_smoke_required == true and .targeted_rust_test_required == true and .current_full_upstream_gate_chain_invoked == false)
  and any(.entries[]; .entry_id == "legacy_recovery_window_feature_gate_inventory" and .legacy_recursion_inventory_required == true and .current_full_upstream_gate_chain_invoked == true)
  and any(.entries[]; .entry_id == "legacy_workgraph_closeout_receipt_chain_inventory" and .legacy_recursion_inventory_required == true and .current_full_upstream_gate_chain_invoked == true)
  and any(.entries[]; .entry_id == "matrix_single_render_contract" and .source_report_smoke_required == true and .targeted_rust_test_required == true)
  and any(.entries[]; .entry_id == "controlled_live_dashboard_single_render_contract" and .source_report_smoke_required == true and .targeted_rust_test_required == true)
  and (.blockers | index("recursive_source_gate_chain_disabled_for_new_gates")) != null
  and (.blockers | index("legacy_recursive_source_gate_inventory_required")) != null
  and (.blockers | index("matrix_cache_write_disabled")) != null
  and (.blockers | index("compact_cache_persistence_disabled")) != null
  and (.blockers | index("source_report_semantics_change_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_workgraph_legacy_gate_recursion_inventory_readback")) != null
  and .recommended_next_gate == "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_gate_recursion_lean_contract_readback --lib
)

printf 'hepta-systems-gate-recursion-lean-contract-readback-gate: PASS: gate recursion is contracted to source-report smoke plus targeted tests, legacy recursion is inventoried, and live remains blocked\n'
