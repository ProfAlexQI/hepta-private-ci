#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-durable-store-test-only-append-fixture-report.sh"
ADAPTER_GATE="$ROOT/scripts/hepta-systems-workflow-durable-store-adapter-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_2026-06-27.md"

fail() {
  printf 'hepta-systems-workflow-durable-store-test-only-append-fixture-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 7 report: $REPORT"
[[ -x "$ADAPTER_GATE" ]] || fail "missing executable workflow durable store adapter gate: $ADAPTER_GATE"
[[ -f "$DOC" ]] || fail "missing Phase 7 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the workflow durable store test-only append fixture report"
fi

grep -q 'Workflow Durable Store Test-Only Append Fixture' "$DOC" \
  || fail "architecture note must document Workflow Durable Store Test-Only Append Fixture"
grep -q 'test-only in-memory fixture' "$DOC" \
  || fail "architecture note must document test-only in-memory fixture"
grep -q 'no runtime event-log write, SQLite write, fixture file write, lease acquisition, idempotency index mutation, checkpoint write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document closed runtime write/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_durable_store_test_only_append_fixture"
  and .status == "ready"
  and .gate == "hepta_workflow_durable_store_test_only_append_fixture_gate"
  and .schema_version == "workflow_durable_store_test_only_append_fixture_v1"
  and .source_adapter_ready == true
  and .source_adapter_event_contract_count == 9
  and .source_adapter_entry_count == 9
  and .lib_export_present == true
  and .test_fixture_scope == "test_only_in_memory_fixture_no_runtime_store_write"
  and .event_contract_count == 9
  and .fixture_entry_count == 9
  and .append_only_sequence_count == 9
  and .idempotency_fixture_count == 9
  and .checkpoint_fixture_count == 9
  and .replay_validation_fixture_count == 9
  and .rollback_fixture_count == 9
  and .duplicate_append_denial_count == 9
  and .feature_gate_required == true
  and .runtime_feature_gate_enabled == false
  and .test_fixture_materialized == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .fixture_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .test_only_append_fixture_ready == true
  and (.entries | length) == 9
  and (.entries | all(.fixture_sequence >= 1 and .fixture_sequence <= 9 and (.fixture_append_key | startswith("test-only.append-fixture.")) and (.fixture_idempotency_key | startswith("test-only.idempotency.")) and (.fixture_checkpoint_key | startswith("test-only.checkpoint.")) and (.fixture_replay_validation_key | startswith("test-only.replay-validation.")) and .append_only_order_validated == true and .idempotency_key_validated == true and .duplicate_append_denied == true and .checkpoint_metadata_validated == true and .replay_validation_metadata_validated == true and .rollback_metadata_validated == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .fixture_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .fixture_sequence == 1)
  and any(.entries[]; .event_contract_id == "worker_task_event_intake" and .fixture_rollback_anchor == "rollback_to_prior_worker_task_attempt_anchor")
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.fixture_checkpoint_key | contains("checkpoint_metadata_only_no_checkpoint_write")))
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation")) != null
  and .next_migration_step == "phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ADAPTER_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_durable_store_test_only_append_fixture --lib
)

printf 'hepta-systems-workflow-durable-store-test-only-append-fixture-gate: PASS: test-only durable store append fixture validates append-only metadata with runtime writes and live execution closed\n'
