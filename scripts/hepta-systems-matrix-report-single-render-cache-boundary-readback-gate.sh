#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-matrix-report-single-render-cache-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-matrix-report-single-render-cache-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable single-render cache boundary report: $REPORT"
[[ -f "$DOC" ]] || fail "missing single-render cache boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the single-render cache boundary report"
fi

grep -q 'Hepta Systems Matrix Report Single Render Cache Boundary Readback' "$DOC" \
  || fail "architecture note must document Hepta Systems Matrix Report Single Render Cache Boundary Readback"
grep -q 'single-render readback boundary' "$DOC" \
  || fail "architecture note must document single-render readback boundary"
grep -q 'no matrix cache write, matrix cache persistence, compact cache persistence, source report semantic change, downstream direct matrix render, workflow execution, replay execution, event-log write, SQLite write, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed single-render cache boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_matrix_report_single_render_cache_boundary_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_matrix_report_single_render_cache_boundary_readback_gate"
  and .schema_version == "hepta_systems_matrix_report_single_render_cache_boundary_readback_v1"
  and .source_matrix_ready == false
  and .source_matrix_capability_count > 0
  and .source_matrix_ready_count > 0
  and .source_matrix_ready_count < .source_matrix_capability_count
  and .source_live_enabled_count == 0
  and .source_all_live_paths_blocked == true
  and .source_dirty_worktree_entry_count >= 0
  and .controlled_live_blocker_count == 7
  and .matrix_report_render_count == 1
  and .single_render_projection_count == 4
  and .downstream_consumer_count == 2
  and .lib_export_present == true
  and .compact_cache_consumer_rewired == true
  and .dashboard_consumer_rewired == true
  and .matrix_cache_write_allowed == false
  and .matrix_cache_persisted == false
  and .compact_cache_persisted == false
  and .source_report_semantics_change_allowed == false
  and .downstream_direct_matrix_render_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and .single_render_cache_boundary_readback_ready == true
  and (.entries | length) == 4
  and (.entries | all(.projected_in_memory == true and .matrix_report_render_consumed == true and .downstream_direct_matrix_render_required == false and .matrix_cache_written == false and .matrix_cache_persisted == false and .compact_cache_persisted == false and .source_report_semantics_changed == false and .workflow_execution_started == false and .replay_executed == false and .event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and any(.entries[]; .entry_id == "matrix_capability_summary_projection")
  and any(.entries[]; .entry_id == "matrix_live_blocker_summary_projection")
  and any(.entries[]; .entry_id == "compact_cache_boundary_single_render_consumer" and .consumer_route == "scripts/hepta-systems-current-reality-matrix-compact-cache-boundary-readback-report.sh")
  and any(.entries[]; .entry_id == "controlled_live_dashboard_single_render_consumer" and .consumer_route == "scripts/hepta-systems-controlled-live-operator-readiness-dashboard-report.sh")
  and (.blockers | index("matrix_cache_write_disabled")) != null
  and (.blockers | index("matrix_cache_persistence_disabled")) != null
  and (.blockers | index("compact_cache_persistence_disabled")) != null
  and (.blockers | index("source_report_semantics_change_disabled")) != null
  and (.blockers | index("downstream_direct_matrix_render_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_read_only_status_tool_registration_denial_readback_without_registration")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_read_only_status_tool_registration_denial_readback_without_registration"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_matrix_report_single_render_cache_boundary_readback --lib
)

printf 'hepta-systems-matrix-report-single-render-cache-boundary-readback-gate: PASS: matrix report single-render cache boundary is queryable, downstream consumers are rewired, cache writes remain disabled, and live remains blocked\n'
