#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-boundary-readback-without-recording-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-positive-preconditions-readback-without-write-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-boundary-readback-without-recording-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable write-attempt recording boundary report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable write-positive-preconditions source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the write-attempt recording boundary report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/write-positive-preconditions.json"
target_json="$tmpdir/write-attempt-recording-boundary.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render write-positive-preconditions source report"
jq -e . "$source_json" >/dev/null || fail "write-positive-preconditions source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_WRITE_POSITIVE_PRECONDITIONS_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render write-attempt recording boundary report from cached source"
jq -e . "$target_json" >/dev/null || fail "write-attempt recording boundary report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Attempt Recording Boundary Readback Without Recording' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Attempt Recording Boundary Readback Without Recording"
rg -q 'controlled live evidence receipt store acceptance authority packet receipt-store write attempt recording boundary readback without recording' "$DOC" \
  || fail "architecture note must document write attempt recording boundary readback without recording"
rg -q 'no write-attempt record, write-attempt persistence, denial receipt persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed write-attempt recording boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_write_positive_preconditions_ready == true
  and .source_precondition_entry_count == 7
  and .source_write_preconditions_missing_count == 7
  and .source_receipt_store_write_allowed_count == 0
  and .source_write_attempt_recorded_count == 0
  and .source_receipt_store_written_count == 0
  and .source_receipt_persisted_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .recording_boundary_id == "controlled-live-evidence-receipt-store-write-attempt-recording-boundary"
  and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary"
  and .write_attempt_record_schema_version == "controlled_live_evidence_receipt_store_write_attempt_record_v1"
  and .boundary_entry_count == 7
  and .boundary_projected_count == 7
  and .boundary_ready_count == 7
  and .write_attempt_record_schema_projected_count == 7
  and .source_write_preconditions_attached_count == 7
  and .acceptance_authority_required_count == 7
  and .acceptance_authority_present_count == 0
  and .operator_write_approval_required_count == 7
  and .operator_write_approval_present_count == 0
  and .evidence_acceptance_required_count == 7
  and .evidence_acceptance_present_count == 0
  and .receipt_store_write_grant_required_count == 7
  and .receipt_store_write_grant_present_count == 0
  and .write_attempt_recording_precondition_missing_count == 7
  and .write_attempt_recording_allowed_count == 0
  and .write_attempt_recorded_count == 0
  and .write_attempt_persisted_count == 0
  and .write_attempt_idempotency_key_projected_count == 7
  and .write_attempt_idempotency_key_unique_count == 7
  and .post_record_readback_route_projected_count == 7
  and .rollback_anchor_projected_count == 7
  and .denial_receipt_projected_count == 7
  and .denial_receipt_persisted_count == 0
  and .denial_receipt_digest_projected_count == 7
  and .receipt_store_write_allowed_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .write_attempt_recording_boundary_readback_ready == true
  and .write_attempt_recording_allowed == false
  and .write_attempt_recorded == false
  and .write_attempt_persisted == false
  and .denial_receipt_persistence_allowed == false
  and .denial_receipt_persisted == false
  and .receipt_store_write_allowed == false
  and .receipt_store_written == false
  and .receipt_persistence_allowed == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("acceptance_authority_missing")) != null
  and (.blockers | index("operator_write_approval_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("receipt_store_write_grant_missing")) != null
  and (.blockers | index("write_attempt_recording_disabled")) != null
  and (.blockers | index("write_attempt_record_persistence_disabled")) != null
  and (.blockers | index("denial_receipt_persistence_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_without_recording_"))
    and (.source_precondition_entry_id | startswith("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_without_write_"))
    and (.source_write_precondition_set_id | startswith("receipt-store-write-positive-preconditions:controlled-live-evidence-receipt-store:"))
    and (.source_write_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-positive-preconditions/"))
    and (.source_write_attempt_recording_precondition_id | startswith("write-attempt-recording-required:controlled-live-evidence-receipt-store:"))
    and (.source_receipt_store_write_denial_id | startswith("receipt-store-write-denial:controlled-live-evidence-receipt-store:"))
    and (.source_replay_idempotency_key | startswith("receipt-store-write-denial-replay-idempotency:controlled-live-evidence-receipt-store:"))
    and (.source_zero_effect_digest | startswith("sha256:receipt-store-write-denial-zero-effect:"))
    and .recording_boundary_id == "controlled-live-evidence-receipt-store-write-attempt-recording-boundary"
    and (.recording_boundary_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary/"))
    and (.write_attempt_record_id | startswith("write-attempt-record:controlled-live-evidence-receipt-store:"))
    and .write_attempt_record_schema_version == "controlled_live_evidence_receipt_store_write_attempt_record_v1"
    and (.write_attempt_idempotency_key | startswith("write-attempt-recording-idempotency:controlled-live-evidence-receipt-store:"))
    and (.post_record_readback_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary/post-record/"))
    and (.rollback_anchor | startswith("rollback-anchor:controlled-live-evidence-receipt-store-write-attempt:"))
    and (.denial_receipt_id | startswith("write-attempt-recording-denial-receipt:controlled-live-evidence-receipt-store:"))
    and (.denial_receipt_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary/denial-receipts/"))
    and (.denial_receipt_digest | startswith("sha256:receipt-store-write-attempt-recording-denial:"))
    and .denial_reason == "write_attempt_recording_disabled_acceptance_authority_missing_evidence_acceptance_missing_write_grant_missing"
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "receipt_store_write_attempt_recording_boundary_projected_without_recording"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .source_packet_unsent == true
    and .source_write_denial_attached == true
    and .source_write_preconditions_attached == true
    and .boundary_projected == true
    and .boundary_ready == true
    and .write_attempt_record_schema_projected == true
    and .acceptance_authority_required == true
    and .acceptance_authority_present == false
    and .operator_write_approval_required == true
    and .operator_write_approval_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .receipt_store_write_grant_required == true
    and .receipt_store_write_grant_present == false
    and .write_attempt_recording_precondition_missing == true
    and .write_attempt_recording_allowed == false
    and .write_attempt_recorded == false
    and .write_attempt_persisted == false
    and .write_attempt_idempotency_key_projected == true
    and .post_record_readback_route_projected == true
    and .rollback_anchor_projected == true
    and .denial_receipt_projected == true
    and .denial_receipt_persistence_allowed == false
    and .denial_receipt_persisted == false
    and .denial_receipt_digest_projected == true
    and .receipt_store_write_allowed == false
    and .receipt_store_written == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .ledger_write_allowed == false
    and .ledger_written == false
    and .workflow_event_log_write_allowed == false
    and .workflow_event_log_written == false
    and .sqlite_write_allowed == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-boundary/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .write_attempt_record_id == "write-attempt-record:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .denial_receipt_id == "write-attempt-recording-denial-receipt:controlled-live-evidence-receipt-store:fresh_soak_readback_missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_retention_replay_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write"
  and .status == "ready_blocked"
  and .write_positive_preconditions_readback_ready == true
  and .precondition_entry_count == 7
  and .write_preconditions_missing_count == 7
  and .receipt_store_write_allowed_count == 0
  and .write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-boundary-readback-without-recording-gate: PASS: receipt-store write-attempt recording boundary is read back without recording, persistence, receipt-store write, or live execution\n'
