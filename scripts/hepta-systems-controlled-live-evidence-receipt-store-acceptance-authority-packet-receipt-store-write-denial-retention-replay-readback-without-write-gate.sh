#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-readback-without-write-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable write-denial retention/replay report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable receipt-store write denial source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the write-denial retention/replay report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/receipt-store-write-denial.json"
target_json="$tmpdir/write-denial-retention-replay.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render receipt-store write denial source report"
jq -e . "$source_json" >/dev/null || fail "receipt-store write denial source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECEIPT_STORE_WRITE_DENIAL_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render write-denial retention/replay report from cached source"
jq -e . "$target_json" >/dev/null || fail "write-denial retention/replay report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Denial Retention Replay Readback Without Write' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Denial Retention Replay Readback Without Write"
rg -q 'controlled live evidence receipt store acceptance authority packet receipt-store write denial retention replay readback without write' "$DOC" \
  || fail "architecture note must document write-denial retention replay readback without write"
rg -q 'no retention policy persistence, replay index write, expiry enforcement, garbage collection, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed write-denial retention/replay boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_write_denial_ready == true
  and .source_write_denial_entry_count == 7
  and .source_receipt_store_write_denied_count == 7
  and .source_receipt_store_write_allowed_count == 0
  and .source_receipt_store_write_attempt_recorded_count == 0
  and .source_receipt_store_written_count == 0
  and .source_receipt_persisted_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .retention_replay_collection_id == "controlled-live-evidence-receipt-store-write-denial-retention-replay"
  and .retention_replay_collection_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention-replay"
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
  and .source_write_denial_attached_count == 7
  and .retention_policy_persisted_count == 0
  and .replay_index_written_count == 0
  and .expiry_enforced_count == 0
  and .garbage_collection_performed_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .write_denial_retention_replay_readback_ready == true
  and .retention_policy_persistence_allowed == false
  and .replay_index_write_allowed == false
  and .expiry_enforcement_allowed == false
  and .garbage_collection_allowed == false
  and .receipt_store_write_attempt_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_persistence_allowed == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("retention_policy_persistence_disabled")) != null
  and (.blockers | index("replay_index_write_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.source_receipt_store_write_denial_id | startswith("receipt-store-write-denial:controlled-live-evidence-receipt-store:"))
    and (.source_receipt_store_write_denial_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/"))
    and (.retention_policy_id | startswith("receipt-store-write-denial-retention-policy:controlled-live-evidence-receipt-store:"))
    and (.replay_key | startswith("receipt-store-write-denial-replay-key:controlled-live-evidence-receipt-store:"))
    and (.replay_idempotency_key | startswith("receipt-store-write-denial-replay-idempotency:controlled-live-evidence-receipt-store:"))
    and (.zero_effect_digest | startswith("sha256:receipt-store-write-denial-zero-effect:"))
    and .retention_state == "projected_not_persisted"
    and .replay_state == "projected_not_executed"
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "receipt_store_write_denial_retention_replay_projected_without_write"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .source_packet_unsent == true
    and .source_write_denial_attached == true
    and .receipt_store_write_denied == true
    and .receipt_store_write_disabled == true
    and .retention_policy_projected == true
    and .expiry_guard_projected == true
    and .replay_key_projected == true
    and .replay_idempotency_key_projected == true
    and .retention_readback_route_projected == true
    and .replay_readback_route_projected == true
    and .garbage_collection_denial_projected == true
    and .supersession_guard_projected == true
    and .zero_effect_digest_projected == true
    and .retention_policy_persistence_allowed == false
    and .retention_policy_persisted == false
    and .replay_index_write_allowed == false
    and .replay_index_written == false
    and .expiry_enforcement_allowed == false
    and .expiry_enforced == false
    and .garbage_collection_allowed == false
    and .garbage_collection_performed == false
    and .receipt_store_write_attempt_allowed == false
    and .receipt_store_write_attempt_recorded == false
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
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .replay_readback_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/replay/dirty-worktree-boundary")
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write"
  and .status == "ready_blocked"
  and .receipt_store_write_denial_readback_ready == true
  and .write_denial_entry_count == 7
  and .receipt_store_write_denied_count == 7
  and .receipt_store_write_allowed_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-gate: PASS: write-denial retention/replay is read back without write, persistence, replay execution, or live execution\n'
