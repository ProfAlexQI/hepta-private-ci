#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_CHECKPOINT_CONSISTENCY_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local replay-alignment checkpoint consistency report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite local WorkGraph projection replay-alignment gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local replay-alignment checkpoint consistency architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local replay-alignment checkpoint consistency report"
fi

grep -q 'Temporal-Lite Replay Alignment Checkpoint Consistency Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Replay Alignment Checkpoint Consistency Local Persistence Readback"
grep -q 'local persistence replay alignment checkpoint consistency readback' "$DOC" \
  || fail "architecture note must document local persistence replay alignment checkpoint consistency readback"
grep -q 'no replay execution, checkpoint write, rollback anchor write, checkpoint consistency persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed checkpoint-consistency/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_v1"
  and .source_replay_alignment_ready == true
  and .source_replay_alignment_entry_count == 9
  and .lib_export_present == true
  and .reopened_sqlite_checkpoint_consistency_test_present == true
  and .consistency_scope == "local_persistence_replay_alignment_checkpoint_consistency_readback_no_execution"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .checkpoint_consistency_projection_count == 9
  and .checkpoint_consistency_key_count == 9
  and .checkpoint_digest_count == 9
  and .replay_alignment_checkpoint_match_count == 9
  and .sqlite_readback_validated_count == 9
  and .checkpoint_mismatch_count == 0
  and .replay_executed_count == 0
  and .checkpoint_written_count == 0
  and .rollback_anchor_written_count == 0
  and .consistency_persisted_count == 0
  and .work_graph_store_write_count == 0
  and .event_log_write_count == 0
  and .sqlite_write_count == 0
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .checkpoint_consistency_contract_readback_materialized == true
  and .replay_execution_allowed == false
  and .checkpoint_write_allowed == false
  and .rollback_anchor_write_allowed == false
  and .checkpoint_consistency_persistence_allowed == false
  and .work_graph_projection_write_allowed == false
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .checkpoint_consistency_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.replay_alignment_key | startswith("temporal-lite.local-work-graph.replay-alignment-readback.")) and (.projection_replay_key | startswith("temporal-lite.local-work-graph.projection-replay-readback.")) and (.checkpoint_consistency_key | startswith("temporal-lite.local-replay-alignment.checkpoint-consistency-readback.")) and (.checkpoint_readback_key | startswith("temporal-lite.local-checkpoint.readback.")) and (.checkpoint_consistency_digest | startswith("temporal-lite.local-replay-alignment-checkpoint-consistency-digest.v1.")) and .expected_checkpoint_projection_key == .checkpoint_readback_key and .consistency_state == "checkpoint_consistent_from_local_persistence_not_written" and .readback_state == "projected_from_sqlite_wal_local_persistence_readback_without_checkpoint_writes" and .replay_alignment_projected == true and .checkpoint_consistency_projected == true and .checkpoint_consistency_key_projected == true and .checkpoint_digest_projected == true and .replay_alignment_checkpoint_matches == true and .sqlite_readback_validated == true and .checkpoint_mismatch_detected == false and .replay_executed == false and .checkpoint_written == false and .rollback_anchor_written == false and .consistency_persisted == false and .work_graph_store_written == false and .event_log_record_written == false and .sqlite_row_written == false and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and .checkpoint_consistency_projected == true)
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and .replay_alignment_checkpoint_matches == true)
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("checkpoint_write_disabled")) != null
  and (.blockers | index("rollback_anchor_write_disabled")) != null
  and (.blockers | index("checkpoint_consistency_persistence_disabled")) != null
  and (.blockers | index("work_graph_projection_write_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("rollback_execution_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-gate: PASS: Temporal-lite local replay-alignment checkpoint consistency reads back from SQLite/WAL history, stays mismatch-free, and keeps runtime writes/live closed\n'
