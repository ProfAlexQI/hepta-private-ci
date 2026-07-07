#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_RECORDING_DENIAL_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable recording-denial report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable positive-preconditions source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Authority Decision Recording Positive Preconditions Recording Denial Readback Without Recording' "$DOC" \
  || fail "architecture note must document the recording-denial readback surface"
rg -q 'controlled live evidence receipt store local evidence acceptance authority decision recording positive preconditions recording denial readback without recording' "$DOC" \
  || fail "architecture note must document the plain-language recording-denial boundary"
rg -q 'no authority decision recording, authority decision persistence, denial receipt persistence, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed recording boundary"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/positive-preconditions.json"
target_json="$tmpdir/recording-denial.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render positive-preconditions source report"
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_POSITIVE_PRECONDITIONS_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render recording-denial report from cached source"
jq -e . "$target_json" >/dev/null || fail "target report did not render valid JSON"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_readback_without_recording_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_positive_preconditions_readback_ready == true
  and .source_precondition_entry_count == 7
  and .source_positive_precondition_set_projected_count == 7
  and .source_positive_precondition_key_unique_count == 7
  and .source_terminal_closeout_attached_count == 7
  and .source_persistence_denial_attached_count == 7
  and .source_denial_receipt_attached_count == 7
  and .source_authority_decision_record_id_attached_count == 7
  and .source_positive_preconditions_missing_count == 7
  and .source_authority_decision_recording_allowed_count == 0
  and .source_authority_decision_recorded_count == 0
  and .source_authority_decision_persisted_count == 0
  and .source_denial_receipt_persisted_count == 0
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
  and .recording_denial_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denials"
  and .recording_denial_entry_count == 7
  and .recording_denial_projected_count == 7
  and .recording_denial_key_projected_count == 7
  and .recording_denial_key_unique_count == 7
  and .recording_denial_readback_route_projected_count == 7
  and .recording_denial_reason_projected_count == 7
  and .recording_denial_state_projected_count == 7
  and .recording_denial_digest_projected_count == 7
  and .source_positive_preconditions_attached_count == 7
  and .source_terminal_closeout_attached_entry_count == 7
  and .source_persistence_denial_attached_entry_count == 7
  and .source_denial_receipt_attached_entry_count == 7
  and .source_authority_decision_record_id_attached_entry_count == 7
  and .local_evidence_acceptance_authority_missing_count == 7
  and .authority_decision_request_missing_count == 7
  and .operator_authority_decision_approval_missing_count == 7
  and .evidence_acceptance_missing_count == 7
  and .authority_decision_recording_grant_missing_count == 7
  and .decision_record_schema_commit_missing_count == 7
  and .atomic_decision_record_append_missing_count == 7
  and .post_record_readback_missing_count == 7
  and .rollback_anchor_missing_count == 7
  and .retention_policy_commit_missing_count == 7
  and .replay_idempotency_guard_missing_count == 7
  and .authority_decision_recording_allowed_count == 0
  and .authority_decision_recorded_count == 0
  and .authority_decision_persisted_count == 0
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
  and .recording_denial_readback_ready == true
  and .authority_decision_recording_allowed == false
  and .authority_decision_persistence_allowed == false
  and .denial_receipt_persistence_allowed == false
  and .evidence_acceptance_recording_allowed == false
  and .evidence_recording_allowed == false
  and .receipt_store_write_attempt_recording_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_persistence_allowed == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("local_evidence_acceptance_authority_missing")) != null
  and (.blockers | index("authority_decision_request_missing")) != null
  and (.blockers | index("operator_authority_decision_approval_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("authority_decision_recording_grant_missing")) != null
  and (.blockers | index("decision_record_schema_not_committed")) != null
  and (.blockers | index("atomic_decision_record_append_not_enabled")) != null
  and (.blockers | index("post_record_readback_missing")) != null
  and (.blockers | index("rollback_anchor_missing")) != null
  and (.blockers | index("retention_policy_not_committed")) != null
  and (.blockers | index("replay_idempotency_guard_disabled")) != null
  and (.blockers | index("authority_decision_recording_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_without_recording_"))
    and (.source_positive_preconditions_entry_id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_without_recording_"))
    and (.source_positive_precondition_set_id | startswith("local-evidence-acceptance-authority-decision-recording-positive-preconditions:"))
    and (.source_positive_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/positive-preconditions/"))
    and (.source_terminal_closeout_id | startswith("local-evidence-acceptance-authority-decision-recording-denial-receipt-persistence-denial-terminal-no-persistence:"))
    and (.source_terminal_closeout_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/persistence-denial/terminal-no-persistence/"))
    and (.source_persistence_denial_id | startswith("local-evidence-acceptance-authority-decision-recording-denial-receipt-persistence-denial:"))
    and (.source_persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/persistence-denial/"))
    and (.source_denial_receipt_id | startswith("local-evidence-acceptance-authority-decision-recording-denial-receipt:"))
    and (.source_denial_receipt_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts/"))
    and (.source_denial_receipt_digest | startswith("sha256:local-evidence-acceptance-authority-decision-recording-denial-receipt:"))
    and (.source_authority_decision_record_id | startswith("local-evidence-acceptance-authority-decision-record:"))
    and (.recording_denial_id | startswith("local-evidence-acceptance-authority-decision-recording-denial:"))
    and (.recording_denial_key | startswith("authority-decision-recording-denial:"))
    and (.recording_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denials/"))
    and .recording_denial_reason == "local_evidence_acceptance_authority_decision_recording_disabled_positive_preconditions_missing"
    and .recording_denial_state == "denied_not_recorded"
    and (.recording_denial_digest | startswith("sha256:local-evidence-acceptance-authority-decision-recording-denial:"))
    and .observed_state == "local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_projected_without_recording"
    and .recording_denial_projected == true
    and .recording_denial_key_projected == true
    and .recording_denial_readback_route_projected == true
    and .recording_denial_reason_projected == true
    and .recording_denial_state_projected == true
    and .recording_denial_digest_projected == true
    and .source_positive_preconditions_attached == true
    and .source_terminal_closeout_attached == true
    and .source_persistence_denial_attached == true
    and .source_denial_receipt_binding_attached == true
    and .source_authority_decision_record_id_attached == true
    and .local_evidence_acceptance_authority_missing == true
    and .authority_decision_request_missing == true
    and .operator_authority_decision_approval_missing == true
    and .evidence_acceptance_missing == true
    and .authority_decision_recording_grant_missing == true
    and .decision_record_schema_commit_missing == true
    and .atomic_decision_record_append_missing == true
    and .post_record_readback_missing == true
    and .rollback_anchor_missing == true
    and .retention_policy_commit_missing == true
    and .replay_idempotency_guard_missing == true
    and .authority_decision_recording_allowed == false
    and .authority_decision_recorded == false
    and .authority_decision_persistence_allowed == false
    and .authority_decision_persisted == false
    and .denial_receipt_persistence_allowed == false
    and .denial_receipt_persisted == false
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
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .recording_denial_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording/recording-denials/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .recording_denial_key == "authority-decision-recording-denial:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial_receipt_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_readback_without_recording"
  and .status == "ready_blocked"
  and .positive_preconditions_readback_ready == true
  and .precondition_entry_count == 7
  and .positive_precondition_set_projected_count == 7
  and .positive_preconditions_missing_count == 7
  and .authority_decision_recording_allowed_count == 0
  and .authority_decision_recorded_count == 0
  and .authority_decision_persisted_count == 0
  and .denial_receipt_persisted_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_positive_preconditions_recording_denial::tests::authority_decision_recording_positive_preconditions_recording_denial --lib
)

printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-positive-preconditions-recording-denial-gate: PASS: local evidence acceptance authority decision recording positive precondition recording denials are read back without recording, persistence, or live execution\n'
