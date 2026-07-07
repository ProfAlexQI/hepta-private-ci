#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-terminal-no-persistence-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-retention-replay-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_2026-07-08.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-terminal-no-persistence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable persistence-denial terminal no-persistence report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable persistence-denial retention/replay source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/persistence-denial-retention-replay.json"
target_json="$tmpdir/persistence-denial-terminal-no-persistence.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render persistence-denial retention/replay source report"
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_RETENTION_REPLAY_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render terminal no-persistence report from cached source"
jq -e . "$target_json" >/dev/null || fail "target report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Authority Decision Recording Positive Preconditions Recording Denial Receipt Persistence Denial Terminal No Persistence Readback' "$DOC" \
  || fail "architecture note must document the terminal no-persistence readback surface"
rg -q 'controlled live evidence receipt store local evidence acceptance authority decision recording positive-preconditions recording-denial receipt persistence denial terminal no-persistence readback' "$DOC" \
  || fail "architecture note must document the plain-language local terminal no-persistence boundary"
rg -q 'no terminal closeout recording, terminal closeout persistence, terminal closeout acceptance, terminal closeout authority, recording-denial receipt persistence attempt recording, recording-denial receipt persistence, authority decision recording, authority decision persistence, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed terminal no-persistence boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_retention_replay_readback_ready == true
  and .source_retention_replay_entry_count == 7
  and .source_retention_policy_projected_count == 7
  and .source_expiry_guard_projected_count == 7
  and .source_replay_key_projected_count == 7
  and .source_replay_idempotency_key_unique_count == 7
  and .source_retention_readback_route_projected_count == 7
  and .source_replay_readback_route_projected_count == 7
  and .source_garbage_collection_denial_projected_count == 7
  and .source_supersession_guard_projected_count == 7
  and .source_zero_effect_digest_projected_count == 7
  and .source_persistence_denial_attached_count == 7
  and .source_denial_receipt_binding_attached_count == 7
  and .source_authority_decision_record_id_attached_count == 7
  and .source_recording_denial_receipt_persistence_attempt_recorded_count == 0
  and .source_denial_receipt_persisted_count == 0
  and .source_authority_decision_recorded_count == 0
  and .source_authority_decision_persisted_count == 0
  and .source_evidence_acceptance_recorded_count == 0
  and .source_evidence_recorded_count == 0
  and .source_receipt_store_write_attempt_recorded_count == 0
  and .source_receipt_store_written_count == 0
  and .source_receipt_persisted_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .terminal_collection_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-terminal-no-persistence"
  and .terminal_collection_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/persistence-denial/terminal-no-persistence"
  and .terminal_entry_count == 7
  and .terminal_closeout_projected_count == 7
  and .terminal_no_persistence_confirmed_count == 7
  and .terminal_closeout_key_projected_count == 7
  and .terminal_closeout_key_unique_count == 7
  and .terminal_readback_route_projected_count == 7
  and .source_retention_replay_attached_count == 7
  and .source_persistence_denial_attached_count == 7
  and .source_denial_receipt_attached_count == 7
  and .source_authority_decision_record_id_attached_count == 7
  and .terminal_closeout_recorded_count == 0
  and .terminal_closeout_persisted_count == 0
  and .terminal_closeout_accepted_count == 0
  and .terminal_closeout_authoritative_count == 0
  and .recording_denial_receipt_persistence_attempt_recorded_count == 0
  and .denial_receipt_persisted_count == 0
  and .authority_decision_recorded_count == 0
  and .authority_decision_persisted_count == 0
  and .evidence_acceptance_recorded_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .terminal_no_persistence_readback_ready == true
  and .terminal_closeout_recording_allowed == false
  and .terminal_closeout_persistence_allowed == false
  and .terminal_closeout_acceptance_allowed == false
  and .recording_denial_receipt_persistence_allowed == false
  and .authority_decision_recording_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_persistence_allowed == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("terminal_closeout_recording_disabled")) != null
  and (.blockers | index("terminal_closeout_persistence_disabled")) != null
  and (.blockers | index("terminal_closeout_acceptance_disabled")) != null
  and (.blockers | index("recording_denial_receipt_persistence_disabled")) != null
  and (.blockers | index("authority_decision_recording_disabled")) != null
  and (.blockers | index("receipt_store_write_attempt_recording_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence_"))
    and (.source_retention_replay_entry_id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_retention_replay_without_persistence_"))
    and (.source_persistence_denial_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial:"))
    and (.source_persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/persistence-denial/"))
    and .source_persistence_denial_reason == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_disabled_positive_preconditions_missing"
    and (.source_denial_receipt_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:"))
    and (.source_denial_receipt_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/"))
    and (.source_denial_receipt_digest | startswith("sha256:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt:"))
    and (.source_positive_precondition_set_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-positive-preconditions:"))
    and (.source_authority_decision_record_id | startswith("local-evidence-acceptance-authority-decision-record:"))
    and (.source_retention_policy_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-retention-policy:"))
    and (.source_replay_idempotency_key | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-replay-idempotency:"))
    and (.source_zero_effect_digest | startswith("sha256:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-retention-replay-zero-effect:"))
    and (.terminal_closeout_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-terminal-no-persistence:"))
    and (.terminal_closeout_key | startswith("terminal-no-persistence:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial:"))
    and (.terminal_closeout_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/persistence-denial/terminal-no-persistence/"))
    and .terminal_reason == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denied_retention_replay_projected_no_persistence_authority"
    and .terminal_state == "terminal_no_persistence"
    and .observed_state == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence_closed"
    and .source_retention_replay_attached == true
    and .source_persistence_denial_attached == true
    and .source_denial_receipt_binding_attached == true
    and .source_authority_decision_record_id_attached == true
    and .terminal_closeout_projected == true
    and .terminal_no_persistence_confirmed == true
    and .terminal_closeout_key_projected == true
    and .terminal_readback_route_projected == true
    and .terminal_closeout_recording_allowed == false
    and .terminal_closeout_recorded == false
    and .terminal_closeout_persistence_allowed == false
    and .terminal_closeout_persisted == false
    and .terminal_closeout_acceptance_allowed == false
    and .terminal_closeout_accepted == false
    and .terminal_closeout_authoritative == false
    and .recording_denial_receipt_persistence_allowed == false
    and .recording_denial_receipt_persistence_attempt_recorded == false
    and .denial_receipt_persisted == false
    and .authority_decision_recording_allowed == false
    and .authority_decision_recorded == false
    and .authority_decision_persistence_allowed == false
    and .authority_decision_persisted == false
    and .evidence_acceptance_recording_allowed == false
    and .evidence_acceptance_recorded == false
    and .evidence_recording_allowed == false
    and .evidence_recorded == false
    and .receipt_store_write_attempt_recording_allowed == false
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
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .terminal_closeout_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denial-receipts/persistence-denial/terminal-no-persistence/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .terminal_closeout_key == "terminal-no-persistence:local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence"
  and .status == "ready_blocked"
  and .retention_replay_readback_ready == true
  and .retention_replay_entry_count == 7
  and .retention_policy_projected_count == 7
  and .replay_idempotency_key_unique_count == 7
  and .recording_denial_receipt_persistence_attempt_recorded_count == 0
  and .denial_receipt_persisted_count == 0
  and .authority_decision_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_persistence_denial_terminal_no_persistence::tests::local_recording_denial_receipt_persistence_denial_terminal --lib
)

printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-receipt-persistence-denial-terminal-no-persistence-gate: PASS: local evidence acceptance authority decision recording positive-preconditions recording-denial receipt persistence-denial branch is terminally read back without persistence or live execution\n'
