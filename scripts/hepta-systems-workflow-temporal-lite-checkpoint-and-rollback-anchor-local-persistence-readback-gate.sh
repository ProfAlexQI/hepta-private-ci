#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local checkpoint/rollback anchor report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite local replay validator gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local checkpoint/rollback anchor architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local checkpoint/rollback anchor report"
fi

grep -q 'Temporal-Lite Checkpoint And Rollback Anchor Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Checkpoint And Rollback Anchor Local Persistence Readback"
grep -q 'local persistence checkpoint and rollback anchor readback' "$DOC" \
  || fail "architecture note must document local persistence checkpoint and rollback anchor readback"
grep -q 'single append-only event store interface' "$DOC" \
  || fail "architecture note must document the single append-only event store interface source"
grep -q 'no runtime event-log write, runtime SQLite write, runtime store persistence, checkpoint write, rollback anchor write, anchor persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed checkpoint/rollback/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_v1"
  and .source_replay_validator_ready == true
  and .source_replay_projection_count == 9
  and .source_append_only_event_store_interface_ready == true
  and .source_replay_validator_derived_from_event_store_interface == true
  and .lib_export_present == true
  and .reopened_sqlite_anchor_test_present == true
  and .anchor_scope == "local_persistence_checkpoint_and_rollback_anchor_readback_no_writes"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .replay_readback_projection_count == 9
  and .checkpoint_anchor_readback_count == 9
  and .rollback_anchor_readback_count == 9
  and .durable_anchor_pair_count == 9
  and .checkpoint_digest_count == 9
  and .rollback_digest_count == 9
  and .anchor_mismatch_count == 0
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .anchor_readback_materialized == true
  and .checkpoint_anchors_derived_from_event_store_interface == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .checkpoint_write_allowed == false
  and .rollback_anchor_write_allowed == false
  and .anchor_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .checkpoint_and_rollback_anchor_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.replay_projection_key | startswith("temporal-lite.local-persistence-replay.")) and (.checkpoint_anchor_key | startswith("temporal-lite.local-checkpoint-anchor.")) and (.rollback_anchor_key | startswith("temporal-lite.local-rollback-anchor.")) and (.checkpoint_source_key | startswith("test-only.checkpoint.")) and (.rollback_source_anchor | length > 0) and (.replay_batch_digest | startswith("temporal-lite.local-replay.v1.9.")) and (.checkpoint_readback_digest | startswith("temporal-lite.local-checkpoint-anchor-digest.v1.")) and (.rollback_readback_digest | startswith("temporal-lite.local-rollback-anchor-digest.v1.")) and .anchor_pair_state == "projected_from_local_persistence_readback_without_anchor_writes" and .checkpoint_anchor_projected == true and .rollback_anchor_projected == true and .durable_anchor_pair_projected == true and .checkpoint_digest_validated == true and .rollback_digest_validated == true and .anchor_mismatch_detected == false and .sqlite_readback_validated == true and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .checkpoint_write_allowed == false and .rollback_anchor_write_allowed == false and .anchor_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.checkpoint_readback_digest | startswith("temporal-lite.local-checkpoint-anchor-digest.v1.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and (.rollback_anchor_key | startswith("temporal-lite.local-rollback-anchor.")))
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("checkpoint_write_disabled")) != null
  and (.blockers | index("rollback_anchor_write_disabled")) != null
  and (.blockers | index("anchor_persistence_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_lease_idempotency_index_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-gate: PASS: Temporal-lite local checkpoint and rollback anchors read from SQLite/WAL history, remain paired and mismatch-free, and keep runtime writes/live closed\n'
