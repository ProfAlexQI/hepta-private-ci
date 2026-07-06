#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite deterministic replay validator report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite append-only event store gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite deterministic replay validator architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite deterministic replay validator report"
fi

grep -q 'Temporal-Lite Deterministic Replay Validator Feature-Gated Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Deterministic Replay Validator Feature-Gated Readback"
grep -q 'test-only deterministic projection' "$DOC" \
  || fail "architecture note must document the test-only deterministic projection"
grep -q 'no runtime event-log write, SQLite write, replay projection persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed replay/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_gate"
  and .schema_version == "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_v1"
  and .source_append_only_ready == true
  and .source_test_event_count == 9
  and .lib_export_present == true
  and .replay_scope == "test_only_deterministic_projection_no_replay_execution"
  and .test_event_count == 9
  and .replay_projection_count == 9
  and .deterministic_order_count == 9
  and .replay_digest_count == 9
  and .replay_checksum_count == 9
  and .replay_mismatch_count == 0
  and .idempotency_projection_count == 9
  and .checkpoint_projection_count == 9
  and .rollback_anchor_projection_count == 9
  and .feature_gate_required == true
  and .runtime_feature_gate_enabled == false
  and .replay_validator_materialized == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .replay_projection_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .deterministic_replay_validator_ready == true
  and (.entries | length) == 9
  and (.entries | all(.sequence >= 1 and .sequence <= 9 and (.event_id | startswith("temporal-lite.test-event.")) and (.replay_projection_key | startswith("temporal-lite.replay-projection.")) and (.replay_source_digest | startswith("replay-digest.v1.")) and (.replay_observed_digest | startswith("replay-digest.v1.")) and (.replay_checksum | startswith("replay-checksum.v1.")) and .projection_state == "projected_in_memory_readback_only" and .deterministic_order_validated == true and .replay_digest_validated == true and .replay_checksum_validated == true and .replay_mismatch_detected == false and .replay_source_digest == .replay_observed_digest and .idempotency_key_replayed == true and .checkpoint_key_replayed == true and .rollback_anchor_replayed == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .replay_projection_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .sequence == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.replay_checksum | startswith("replay-checksum.v1.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and .rollback_anchor_replayed == true)
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("replay_projection_persistence_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback")) != null
  and .recommended_next_gate == "temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-gate: PASS: Temporal-lite deterministic replay validator is projection-only, mismatch-free, and runtime-write/live blocked\n'
