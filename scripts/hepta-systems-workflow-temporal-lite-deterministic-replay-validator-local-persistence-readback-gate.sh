#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local persistence replay validator report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite minimal local persistence gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local persistence replay validator architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local persistence replay validator report"
fi

grep -q 'Temporal-Lite Deterministic Replay Validator Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Deterministic Replay Validator Local Persistence Readback"
grep -q 'local persistence readback projection' "$DOC" \
  || fail "architecture note must document the local persistence readback projection"
grep -q 'no runtime event-log write, runtime SQLite write, runtime store persistence, replay projection persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed replay/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_v1"
  and .source_minimal_local_persistence_ready == true
  and .source_local_event_contract_count == 9
  and .lib_export_present == true
  and .stored_events_readback_helper_present == true
  and .tempdb_reopen_test_present == true
  and .replay_scope == "local_persistence_readback_projection_no_replay_execution"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .local_event_count == 9
  and .replay_readback_projection_count == 9
  and .deterministic_order_count == 9
  and .replay_digest_count == 9
  and .replay_checksum_count == 9
  and .replay_batch_digest_count == 9
  and .replay_mismatch_count == 0
  and .idempotency_readback_count == 9
  and .checkpoint_readback_count == 9
  and .rollback_anchor_readback_count == 9
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .replay_validator_materialized == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .replay_projection_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .deterministic_replay_validator_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.aggregate_id | startswith("workflow://hepta/test-only/")) and (.idempotency_key | startswith("test-only.idempotency.")) and (.checkpoint_key | startswith("test-only.checkpoint.")) and (.rollback_anchor | length > 0) and (.replay_projection_key | startswith("temporal-lite.local-persistence-replay.")) and (.replay_source_digest | startswith("replay-digest.v1.")) and .replay_source_digest == .replay_observed_digest and (.replay_batch_digest | startswith("temporal-lite.local-replay.v1.9.")) and (.replay_checksum | startswith("local-replay-checksum.v1.")) and .readback_state == "projected_from_local_persistence_readback_without_replay_execution" and .deterministic_order_validated == true and .replay_digest_validated == true and .replay_checksum_validated == true and .replay_batch_digest_validated == true and .replay_mismatch_detected == false and .sqlite_readback_validated == true and .idempotency_key_replayed == true and .checkpoint_key_replayed == true and .rollback_anchor_replayed == true and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .replay_projection_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.replay_checksum | startswith("local-replay-checksum.v1.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and .rollback_anchor_replayed == true)
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("replay_projection_persistence_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-gate: PASS: Temporal-lite replay validator reads local SQLite/WAL history, remains mismatch-free, and keeps runtime replay/live closed\n'
