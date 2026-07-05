#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local WorkGraph replay-alignment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite local WorkGraph projection gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local WorkGraph replay-alignment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local WorkGraph replay-alignment report"
fi

grep -q 'Temporal-Lite WorkGraph Projection Replay Alignment Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite WorkGraph Projection Replay Alignment Local Persistence Readback"
grep -q 'local persistence WorkGraph projection replay alignment readback' "$DOC" \
  || fail "architecture note must document local persistence WorkGraph projection replay alignment readback"
grep -q 'no replay execution, projection alignment persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed replay-alignment/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_v1"
  and .source_projection_ready == true
  and .source_projection_entry_count == 9
  and .lib_export_present == true
  and .reopened_sqlite_replay_alignment_test_present == true
  and .alignment_scope == "local_persistence_work_graph_projection_replay_alignment_readback_no_execution"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .replay_alignment_projection_count == 9
  and .projection_replay_key_count == 9
  and .replay_alignment_checksum_count == 9
  and .deterministic_alignment_count == 9
  and .sqlite_readback_validated_count == 9
  and .replay_alignment_mismatch_count == 0
  and .replay_executed_count == 0
  and .projection_alignment_persisted_count == 0
  and .work_graph_store_write_count == 0
  and .event_log_write_count == 0
  and .sqlite_write_count == 0
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .replay_alignment_contract_readback_materialized == true
  and .replay_execution_allowed == false
  and .projection_alignment_persistence_allowed == false
  and .work_graph_projection_write_allowed == false
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .replay_alignment_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.work_graph_node_key | startswith("temporal-lite.local-work-graph.node-readback.")) and (.work_graph_event_edge_key | startswith("temporal-lite.local-work-graph.event-edge-readback.")) and (.work_graph_state_edge_key | startswith("temporal-lite.local-work-graph.state-edge-readback.")) and (.projection_key | startswith("temporal-lite.local-work-graph.projection-readback.")) and (.projection_checksum | startswith("temporal-lite.local-work-graph-projection-checksum.v1.")) and (.replay_alignment_key | startswith("temporal-lite.local-work-graph.replay-alignment-readback.")) and (.projection_replay_key | startswith("temporal-lite.local-work-graph.projection-replay-readback.")) and (.replay_alignment_checksum | startswith("temporal-lite.local-work-graph-replay-alignment-checksum.v1.")) and .expected_replay_projection_key == .projection_key and .alignment_state == "aligned_from_local_persistence_not_replayed" and .readback_state == "projected_from_sqlite_wal_local_persistence_readback_without_replay_execution" and .work_graph_projection_projected == true and .replay_alignment_projected == true and .projection_replay_key_projected == true and .replay_alignment_checksum_projected == true and .deterministic_alignment_projected == true and .sqlite_readback_validated == true and .replay_alignment_mismatch_detected == false and .replay_executed == false and .projection_alignment_persisted == false and .work_graph_store_written == false and .event_log_record_written == false and .sqlite_row_written == false and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and .replay_alignment_projected == true)
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and .deterministic_alignment_projected == true)
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("projection_alignment_persistence_disabled")) != null
  and (.blockers | index("work_graph_projection_write_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("rollback_execution_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-gate: PASS: Temporal-lite local WorkGraph projection replay alignment reads back from SQLite/WAL history, stays mismatch-free, and keeps runtime writes/live closed\n'
