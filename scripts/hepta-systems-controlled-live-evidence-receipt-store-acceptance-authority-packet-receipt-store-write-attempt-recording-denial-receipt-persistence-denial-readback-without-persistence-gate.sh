#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-denial-receipt-persistence-denial-readback-without-persistence-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-denial-receipt-positive-preconditions-readback-without-persistence-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-denial-receipt-persistence-denial-readback-without-persistence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable write-attempt recording denial receipt persistence denial report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable write-attempt recording denial receipt positive preconditions source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the write-attempt recording denial receipt persistence denial report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/write-attempt-recording-denial-receipt-positive-preconditions.json"
target_json="$tmpdir/write-attempt-recording-denial-receipt-persistence-denial.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render write-attempt recording denial receipt positive preconditions source report"
jq -e . "$source_json" >/dev/null || fail "write-attempt recording denial receipt positive preconditions source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_WRITE_ATTEMPT_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render write-attempt recording denial receipt persistence denial report from cached source"
jq -e . "$target_json" >/dev/null || fail "write-attempt recording denial receipt persistence denial report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Attempt Recording Denial Receipt Persistence Denial Readback Without Persistence' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Attempt Recording Denial Receipt Persistence Denial Readback Without Persistence"
rg -q 'controlled live evidence receipt store acceptance authority packet receipt-store write attempt recording denial receipt persistence denial readback without persistence' "$DOC" \
  || fail "architecture note must document write attempt recording denial receipt persistence denial readback without persistence"
rg -q 'no persistence authority recording, operator persistence approval, evidence acceptance, denial receipt persistence grant, persistence attempt recording, denial receipt persistence, atomic append, post-persist readback persistence, rollback anchor verification, retention policy commit, replay idempotency guard enablement, write-attempt record, write-attempt persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed persistence-denial boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_readback_without_persistence"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_readback_without_persistence_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_readback_without_persistence_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_positive_preconditions_readback_ready == true
  and .source_precondition_entry_count == 7
  and .source_positive_precondition_set_projected_count == 7
  and .source_persistence_authority_required_count == 7
  and .source_persistence_authority_present_count == 0
  and .source_operator_persistence_approval_required_count == 7
  and .source_operator_persistence_approval_present_count == 0
  and .source_evidence_acceptance_required_count == 7
  and .source_evidence_acceptance_present_count == 0
  and .source_denial_receipt_persistence_grant_required_count == 7
  and .source_denial_receipt_persistence_grant_present_count == 0
  and .source_atomic_append_required_count == 7
  and .source_atomic_append_enabled_count == 0
  and .source_post_persist_readback_required_count == 7
  and .source_post_persist_readback_persisted_count == 0
  and .source_rollback_anchor_required_count == 7
  and .source_rollback_anchor_verified_count == 0
  and .source_retention_policy_commit_required_count == 7
  and .source_retention_policy_committed_count == 0
  and .source_replay_idempotency_guard_required_count == 7
  and .source_replay_idempotency_guard_enabled_count == 0
  and .source_denial_receipt_persistence_allowed_count == 0
  and .source_denial_receipt_persisted_count == 0
  and .source_write_attempt_recorded_count == 0
  and .source_write_attempt_persisted_count == 0
  and .source_receipt_store_written_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .persistence_denial_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/persistence-denial"
  and .persistence_denial_entry_count == 7
  and .persistence_denial_projected_count == 7
  and .source_positive_preconditions_attached_count == 7
  and .denial_receipt_persistence_denied_count == 7
  and .denial_receipt_persistence_disabled_count == 7
  and .denial_receipt_persistence_allowed_count == 0
  and .denial_receipt_persistence_attempt_recorded_count == 0
  and .denial_receipt_persisted_count == 0
  and .persistence_authority_missing_count == 7
  and .operator_persistence_approval_missing_count == 7
  and .evidence_acceptance_missing_count == 7
  and .denial_receipt_persistence_grant_missing_count == 7
  and .atomic_append_disabled_count == 7
  and .post_persist_readback_missing_count == 7
  and .rollback_anchor_missing_count == 7
  and .retention_policy_not_committed_count == 7
  and .replay_idempotency_guard_disabled_count == 7
  and .write_attempt_recorded_count == 0
  and .write_attempt_persisted_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .persistence_denial_readback_ready == true
  and .denial_receipt_persistence_allowed == false
  and .denial_receipt_persisted == false
  and .write_attempt_recording_allowed == false
  and .write_attempt_recorded == false
  and .receipt_store_write_allowed == false
  and .receipt_store_written == false
  and .receipt_persistence_allowed == false
  and .receipt_persisted == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("persistence_authority_missing")) != null
  and (.blockers | index("operator_persistence_approval_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("denial_receipt_persistence_grant_missing")) != null
  and (.blockers | index("atomic_append_not_enabled")) != null
  and (.blockers | index("post_persist_readback_missing")) != null
  and (.blockers | index("rollback_anchor_missing")) != null
  and (.blockers | index("retention_policy_not_committed")) != null
  and (.blockers | index("replay_idempotency_guard_disabled")) != null
  and (.blockers | index("denial_receipt_persistence_disabled")) != null
  and (.blockers | index("write_attempt_recording_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_without_persistence_"))
    and (.source_positive_precondition_set_id | startswith("write-attempt-recording-denial-receipt-positive-preconditions:controlled-live-evidence-receipt-store:"))
    and (.source_denial_receipt_id | startswith("write-attempt-recording-denial-receipt:"))
    and (.persistence_denial_id | startswith("write-attempt-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:"))
    and (.persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/persistence-denial/"))
    and .persistence_denial_reason == "write_attempt_recording_denial_receipt_persistence_disabled_positive_preconditions_missing"
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "write_attempt_recording_denial_receipt_persistence_denied_without_persistence"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .source_packet_unsent == true
    and .source_positive_precondition_set_projected == true
    and .source_positive_preconditions_missing == true
    and .persistence_denial_projected == true
    and .denial_receipt_persistence_denied == true
    and .denial_receipt_persistence_disabled == true
    and .denial_receipt_persistence_allowed == false
    and .denial_receipt_persistence_attempt_recorded == false
    and .denial_receipt_persisted == false
    and .persistence_authority_required == true
    and .persistence_authority_present == false
    and .operator_persistence_approval_required == true
    and .operator_persistence_approval_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .denial_receipt_persistence_grant_required == true
    and .denial_receipt_persistence_grant_present == false
    and .atomic_append_required == true
    and .atomic_append_enabled == false
    and .post_persist_readback_required == true
    and .post_persist_readback_persisted == false
    and .rollback_anchor_required == true
    and .rollback_anchor_verified == false
    and .retention_policy_commit_required == true
    and .retention_policy_committed == false
    and .replay_idempotency_guard_required == true
    and .replay_idempotency_guard_enabled == false
    and .write_attempt_recorded == false
    and .write_attempt_persisted == false
    and .receipt_store_written == false
    and .receipt_persisted == false
    and .ledger_written == false
    and .workflow_event_log_written == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .persistence_denial_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/persistence-denial/dirty-worktree-boundary")
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_positive_preconditions_readback_without_persistence"
  and .status == "ready_blocked"
  and .positive_preconditions_readback_ready == true
  and .precondition_entry_count == 7
  and .denial_receipt_persistence_allowed_count == 0
  and .denial_receipt_persisted_count == 0
  and .write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_readback_without_persistence --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-denial-receipt-persistence-denial-readback-without-persistence-gate: PASS: write-attempt recording denial receipt persistence denial is read back without persistence or live execution\n'
