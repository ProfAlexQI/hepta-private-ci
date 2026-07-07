#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-retention-replay-readback-without-persistence-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-readback-without-persistence-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-retention-replay-readback-without-persistence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable local evidence acceptance recording denial receipt retention replay report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local evidence acceptance recording denial receipt source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the local evidence acceptance recording denial receipt retention replay report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/local-evidence-acceptance-recording-denial-receipt.json"
target_json="$tmpdir/local-evidence-acceptance-recording-denial-receipt-retention-replay.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render local evidence acceptance recording denial receipt source report"
jq -e . "$source_json" >/dev/null || fail "local evidence acceptance recording denial receipt source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render local evidence acceptance recording denial receipt retention replay report from cached source"
jq -e . "$target_json" >/dev/null || fail "local evidence acceptance recording denial receipt retention replay report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Recording Denial Receipt Retention Replay Readback Without Persistence' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Local Evidence Acceptance Recording Denial Receipt Retention Replay Readback Without Persistence"
rg -q 'controlled live evidence receipt store local evidence acceptance recording denial receipt retention replay readback without persistence' "$DOC" \
  || fail "architecture note must document local evidence acceptance recording denial receipt retention replay readback without persistence"
rg -q 'no retention policy persistence, replay index write, expiry enforcement, garbage collection, local evidence acceptance source recording, acceptance source persistence, denial receipt persistence, evidence acceptance recording, evidence recording, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed retention replay boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_recording_denial_receipt_readback_ready == true
  and .source_denial_receipt_entry_count == 7
  and .source_denial_receipt_projected_count == 7
  and .source_denial_receipt_persisted_count == 0
  and .source_acceptance_source_recorded_count == 0
  and .source_acceptance_source_persisted_count == 0
  and .source_evidence_acceptance_recorded_count == 0
  and .source_evidence_recorded_count == 0
  and .source_receipt_store_write_attempt_recorded_count == 0
  and .source_receipt_store_written_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .retention_replay_collection_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-retention-replay"
  and .retention_replay_collection_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay"
  and .retention_replay_entry_count == 7
  and .retention_policy_projected_count == 7
  and .expiry_guard_projected_count == 7
  and .replay_key_projected_count == 7
  and .replay_idempotency_key_projected_count == 7
  and .replay_idempotency_key_unique_count == 7
  and .retention_readback_route_projected_count == 7
  and .replay_readback_route_projected_count == 7
  and .garbage_collection_denial_projected_count == 7
  and .supersession_guard_projected_count == 7
  and .zero_effect_digest_projected_count == 7
  and .source_denial_receipt_attached_count == 7
  and .source_acceptance_source_record_attached_count == 7
  and .retention_policy_persisted_count == 0
  and .replay_index_written_count == 0
  and .expiry_enforced_count == 0
  and .garbage_collection_performed_count == 0
  and .acceptance_source_recorded_count == 0
  and .acceptance_source_persisted_count == 0
  and .denial_receipt_persisted_count == 0
  and .evidence_acceptance_recorded_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .retention_replay_readback_ready == true
  and .retention_policy_persistence_allowed == false
  and .replay_index_write_allowed == false
  and .expiry_enforcement_allowed == false
  and .garbage_collection_allowed == false
  and .acceptance_source_recording_allowed == false
  and .acceptance_source_persistence_allowed == false
  and .denial_receipt_persistence_allowed == false
  and .evidence_acceptance_recording_allowed == false
  and .evidence_recording_allowed == false
  and .receipt_store_write_attempt_recording_allowed == false
  and .receipt_persistence_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_store_written == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("retention_policy_persistence_disabled")) != null
  and (.blockers | index("replay_index_write_disabled")) != null
  and (.blockers | index("expiry_enforcement_disabled")) != null
  and (.blockers | index("garbage_collection_disabled")) != null
  and (.blockers | index("denial_receipt_persistence_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_without_persistence_"))
    and (.source_denial_receipt_id | startswith("local-evidence-acceptance-recording-denial-receipt:controlled-live-evidence-receipt-store:"))
    and (.source_denial_receipt_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/"))
    and (.source_denial_receipt_digest | startswith("sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.source_denial_receipt_idempotency_key | startswith("controlled-live-evidence-receipt-store.local-evidence-acceptance-recording-denial-receipt.idempotency."))
    and (.source_acceptance_source_record_id | startswith("local-evidence-acceptance-source-record:controlled-live-evidence-receipt-store:"))
    and (.source_acceptance_source_record_idempotency_key | startswith("local-evidence-acceptance-source-record-idempotency:controlled-live-evidence-receipt-store:"))
    and .source_recording_denial_reason == "local_evidence_acceptance_source_recording_disabled_open_preconditions_missing"
    and (.retention_policy_id | startswith("retention-policy:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.retention_policy_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/retention/"))
    and (.expiry_guard_id | startswith("expiry-guard:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.replay_key | startswith("replay-key:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.replay_idempotency_key | startswith("controlled-live-evidence-receipt-store.local-evidence-acceptance-recording-denial-retention-replay.idempotency."))
    and (.replay_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/replay/"))
    and (.retention_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/readback/"))
    and (.garbage_collection_denial_id | startswith("garbage-collection-denial:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.supersession_guard_id | startswith("supersession-guard:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.zero_effect_digest | startswith("sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-retention-replay-zero-effect:"))
    and .retention_state == "projected_not_persisted"
    and .replay_state == "projected_not_written"
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "local_evidence_acceptance_recording_denial_receipt_retention_replay_projected_without_persistence"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .retention_policy_projected == true
    and .expiry_guard_projected == true
    and .replay_key_projected == true
    and .replay_idempotency_key_projected == true
    and .retention_readback_route_projected == true
    and .replay_readback_route_projected == true
    and .garbage_collection_denial_projected == true
    and .supersession_guard_projected == true
    and .zero_effect_digest_projected == true
    and .source_denial_receipt_attached == true
    and .source_acceptance_source_record_attached == true
    and .retention_policy_persistence_allowed == false
    and .retention_policy_persisted == false
    and .replay_index_write_allowed == false
    and .replay_index_written == false
    and .expiry_enforcement_allowed == false
    and .expiry_enforced == false
    and .garbage_collection_allowed == false
    and .garbage_collection_performed == false
    and .acceptance_source_recording_allowed == false
    and .acceptance_source_recorded == false
    and .acceptance_source_persistence_allowed == false
    and .acceptance_source_persisted == false
    and .denial_receipt_persistence_allowed == false
    and .denial_receipt_persisted == false
    and .evidence_acceptance_recording_allowed == false
    and .evidence_acceptance_recorded == false
    and .evidence_recording_allowed == false
    and .evidence_recorded == false
    and .receipt_store_write_attempt_recording_allowed == false
    and .receipt_store_write_attempt_recorded == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .receipt_store_write_allowed == false
    and .receipt_store_written == false
    and .ledger_write_allowed == false
    and .ledger_written == false
    and .workflow_event_log_write_allowed == false
    and .workflow_event_log_written == false
    and .sqlite_write_allowed == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .retention_policy_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/retention/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .replay_readback_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay/replay/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence"
  and .status == "ready_blocked"
  and .local_evidence_acceptance_recording_denial_receipt_readback_ready == true
  and .denial_receipt_entry_count == 7
  and .denial_receipt_projected_count == 7
  and .denial_receipt_persisted_count == 0
  and .acceptance_source_recorded_count == 0
  and .acceptance_source_persisted_count == 0
  and .evidence_acceptance_recorded_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-retention-replay-readback-without-persistence-gate: PASS: local evidence acceptance recording denial receipt retention replay is read back without persistence, replay writes, cleanup, receipt-store writes, or live execution\n'
