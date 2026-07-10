#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-current-reality-matrix-compact-cache-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-current-reality-matrix-compact-cache-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable compact cache boundary report: $REPORT"
[[ -f "$DOC" ]] || fail "missing compact cache boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the compact cache boundary report"
fi

grep -q 'Current Reality Matrix Compact Cache Boundary Readback' "$DOC" \
  || fail "architecture note must document Current Reality Matrix Compact Cache Boundary Readback"
grep -q 'consumes the single-render matrix summary' "$DOC" \
  || fail "architecture note must document single-render matrix summary consumption"
grep -q 'no cache write, compact cache persistence, evidence recording, approval acceptance, decision recording, event-log write, SQLite write, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed compact cache boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "current_reality_matrix_compact_cache_boundary_readback"
  and .status == "ready_blocked"
  and .gate == "current_reality_matrix_compact_cache_boundary_readback_gate"
  and .schema_version == "current_reality_matrix_compact_cache_boundary_readback_v1"
  and .source_single_render_cache_boundary_ready == true
  and .source_matrix_ready == false
  and .source_matrix_capability_count > 0
  and .source_matrix_ready_count > 0
  and .source_matrix_ready_count < .source_matrix_capability_count
  and .source_live_enabled_count == 0
  and .source_all_live_paths_blocked == true
  and .source_dirty_worktree_entry_count >= 0
  and .controlled_live_blocker_count == 7
  and .compact_projection_count == 4
  and .matrix_report_render_count == 1
  and .single_render_cache_boundary_consumed == true
  and .dashboard_gate_matrix_rerun_removed == true
  and .lib_export_present == true
  and .cache_write_allowed == false
  and .cache_persisted == false
  and .evidence_recording_allowed == false
  and .approval_acceptance_allowed == false
  and .decision_recording_allowed == false
  and .live_execution_allowed == false
  and .compact_cache_boundary_readback_ready == true
  and (.entries | length) == 4
  and (.entries | all((.readback_route | startswith("readback://current-reality-matrix/compact-cache/")) and .projected_in_memory == true and .cache_write_allowed == false and .cache_persisted == false and .evidence_recording_allowed == false and .approval_acceptance_allowed == false and .decision_recording_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .entry_id == "matrix_capability_counts")
  and any(.entries[]; .entry_id == "matrix_live_blockers")
  and any(.entries[]; .entry_id == "dirty_worktree_counts")
  and any(.entries[]; .entry_id == "dashboard_matrix_rerun_boundary")
  and (.blockers | index("matrix_cache_write_disabled")) != null
  and (.blockers | index("compact_cache_persistence_disabled")) != null
  and (.blockers | index("evidence_recording_disabled")) != null
  and (.blockers | index("approval_acceptance_disabled")) != null
  and (.blockers | index("decision_recording_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("close_controlled_live_evidence_before_status_canary_start")) != null
  and .recommended_next_gate == "close_controlled_live_evidence_before_status_canary_start"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime current_reality_matrix_compact_cache_boundary_readback --lib
)

printf 'hepta-systems-current-reality-matrix-compact-cache-boundary-readback-gate: PASS: current reality matrix compact cache boundary is readback-only and dashboard matrix rerun is removed\n'
