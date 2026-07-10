#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local lease/idempotency report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite local checkpoint/rollback anchor gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local lease/idempotency architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local lease/idempotency report"
fi

grep -q 'Temporal-Lite Lease Idempotency Index Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Lease Idempotency Index Local Persistence Readback"
grep -q 'local persistence lease and idempotency readback' "$DOC" \
  || fail "architecture note must document local persistence lease and idempotency readback"
grep -q 'single append-only event store interface' "$DOC" \
  || fail "architecture note must document the single append-only event store interface source"
grep -q 'no runtime event-log write, runtime SQLite write, runtime store persistence, lease acquisition, lease persistence, idempotency index write, idempotency index persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed lease/idempotency/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_v1"
  and .source_checkpoint_rollback_ready == true
  and .source_anchor_pair_count == 9
  and .source_append_only_event_store_interface_ready == true
  and .source_checkpoint_anchors_derived_from_event_store_interface == true
  and .lib_export_present == true
  and .reopened_sqlite_lease_test_present == true
  and .lease_scope == "local_persistence_lease_idempotency_readback_no_acquire_no_persistence"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .lease_readback_count == 9
  and .idempotency_index_readback_count == 9
  and .duplicate_guard_readback_count == 9
  and .lease_digest_count == 9
  and .idempotency_digest_count == 9
  and .lease_idempotency_pair_count == 9
  and .lease_acquired_count == 0
  and .lease_persisted_count == 0
  and .idempotency_index_written_count == 0
  and .idempotency_index_persisted_count == 0
  and .lease_idempotency_mismatch_count == 0
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .lease_idempotency_readback_materialized == true
  and .lease_idempotency_derived_from_event_store_interface == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .lease_acquire_allowed == false
  and .lease_persistence_allowed == false
  and .idempotency_index_write_allowed == false
  and .idempotency_index_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .lease_idempotency_index_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.replay_projection_key | startswith("temporal-lite.local-persistence-replay.")) and (.checkpoint_anchor_key | startswith("temporal-lite.local-checkpoint-anchor.")) and (.rollback_anchor_key | startswith("temporal-lite.local-rollback-anchor.")) and (.lease_readback_key | startswith("temporal-lite.local-lease-readback.")) and (.lease_scope_key | startswith("temporal-lite.local-lease-scope.")) and .lease_owner == "hepta-temporal-lite-local-test-worker" and .lease_ttl_ms == 30000 and (.lease_digest | startswith("temporal-lite.local-lease-digest.v1.")) and .lease_state == "projected_from_local_persistence_not_acquired" and (.idempotency_index_readback_key | startswith("temporal-lite.local-idempotency-index-readback.")) and (.idempotency_key | startswith("idempotency-key.local.v1.")) and (.idempotency_digest | startswith("temporal-lite.local-idempotency-digest.v1.")) and (.duplicate_guard_key | startswith("temporal-lite.local-duplicate-guard-readback.")) and .duplicate_guard_state == "projected_duplicate_denial_boundary" and .readback_state == "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes" and .lease_readback_projected == true and .lease_digest_validated == true and .idempotency_index_projected == true and .idempotency_digest_validated == true and .duplicate_guard_projected == true and .lease_idempotency_pair_projected == true and .lease_idempotency_mismatch_detected == false and .sqlite_readback_validated == true and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .lease_acquired == false and .lease_persisted == false and .idempotency_index_written == false and .idempotency_index_persisted == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.idempotency_index_readback_key | startswith("temporal-lite.local-idempotency-index-readback.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and (.duplicate_guard_key | startswith("temporal-lite.local-duplicate-guard-readback.")))
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("lease_acquire_disabled")) != null
  and (.blockers | index("lease_persistence_disabled")) != null
  and (.blockers | index("idempotency_index_write_disabled")) != null
  and (.blockers | index("idempotency_index_persistence_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_lease_idempotency_index_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-gate: PASS: Temporal-lite local lease/idempotency indexes read back from SQLite/WAL history, stay mismatch-free, and keep runtime writes/live closed\n'
