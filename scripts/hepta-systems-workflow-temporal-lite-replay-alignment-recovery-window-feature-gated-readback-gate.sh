#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-replay-alignment-rollback-consistency-feature-gated-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_WINDOW_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite replay-alignment recovery window report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite replay-alignment rollback consistency gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite replay-alignment recovery window architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite replay-alignment recovery window report"
fi

grep -q 'Temporal-Lite Replay Alignment Recovery Window Feature-Gated Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Replay Alignment Recovery Window Feature-Gated Readback"
grep -q 'test-only replay alignment recovery window readback' "$DOC" \
  || fail "architecture note must document test-only replay alignment recovery window readback"
grep -q 'no replay execution, checkpoint write, rollback anchor write, recovery window persistence, WorkGraph projection write, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed recovery-window/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_gate"
  and .schema_version == "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_v1"
  and .source_rollback_consistency_ready == true
  and .source_rollback_consistency_entry_count == 9
  and .lib_export_present == true
  and .recovery_scope == "test_only_replay_alignment_recovery_window_readback_no_execution"
  and .recovery_window_projection_count == 9
  and .recovery_window_key_count == 9
  and .recovery_window_digest_count == 9
  and .replay_alignment_recovery_match_count == 9
  and .recovery_window_mismatch_count == 0
  and .replay_executed_count == 0
  and .checkpoint_written_count == 0
  and .rollback_anchor_written_count == 0
  and .recovery_window_persisted_count == 0
  and .work_graph_store_write_count == 0
  and .event_log_write_count == 0
  and .sqlite_write_count == 0
  and .feature_gate_required == true
  and .runtime_feature_gate_enabled == false
  and .recovery_window_contract_readback_materialized == true
  and .replay_execution_allowed == false
  and .checkpoint_write_allowed == false
  and .rollback_anchor_write_allowed == false
  and .recovery_window_persistence_allowed == false
  and .work_graph_projection_write_allowed == false
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .workflow_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .recovery_window_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.sequence >= 1 and .sequence <= 9 and (.event_id | startswith("temporal-lite.test-event.")) and (.replay_alignment_key | startswith("temporal-lite.work-graph.replay-alignment.readback.")) and (.projection_replay_key | startswith("temporal-lite.work-graph.projection-replay.readback.")) and (.rollback_consistency_key | startswith("temporal-lite.replay-alignment.rollback-consistency.readback.")) and (.rollback_readback_key | startswith("temporal-lite.rollback.readback.")) and (.recovery_window_key | startswith("temporal-lite.replay-alignment.recovery-window.readback.")) and (.recovery_window_start_key | startswith("temporal-lite.recovery-window.start.readback.")) and (.recovery_window_end_key | startswith("temporal-lite.recovery-window.end.readback.")) and (.recovery_window_digest | startswith("replay-alignment-recovery-window-digest.v1.")) and .expected_recovery_projection_key == .recovery_window_end_key and .window_state == "recovery_window_projected_not_written" and .readback_state == "replay_alignment_recovery_window_contract_projected_in_memory_only" and .rollback_consistency_projected == true and .recovery_window_projected == true and .recovery_window_key_projected == true and .recovery_window_digest_projected == true and .replay_alignment_recovery_matches == true and .recovery_window_mismatch_detected == false and .replay_executed == false and .checkpoint_written == false and .rollback_anchor_written == false and .recovery_window_persisted == false and .work_graph_store_written == false and .event_log_record_written == false and .sqlite_row_written == false and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .sequence == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and .recovery_window_projected == true)
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and .replay_alignment_recovery_matches == true)
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("checkpoint_write_disabled")) != null
  and (.blockers | index("rollback_anchor_write_disabled")) != null
  and (.blockers | index("recovery_window_persistence_disabled")) != null
  and (.blockers | index("work_graph_projection_write_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("rollback_execution_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback")) != null
  and .recommended_next_gate == "temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-gate: PASS: Temporal-lite replay-alignment recovery window is projected, feature-gated, not executed, not persisted, and runtime-write/live blocked\n'
