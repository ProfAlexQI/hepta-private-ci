#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-positive-preconditions-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-recording-denial-persistence-denial-terminal-no-persistence-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_POSITIVE_PRECONDITIONS_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-positive-preconditions-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable local evidence acceptance positive preconditions report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable terminal no-persistence source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Positive Preconditions Readback Without Acceptance' "$DOC" \
  || fail "architecture note must document the local evidence acceptance positive preconditions surface"
rg -q 'controlled live evidence receipt store local evidence acceptance positive preconditions readback without acceptance' "$DOC" \
  || fail "architecture note must document the plain-language local evidence acceptance boundary"
rg -q 'no local evidence acceptance, local evidence acceptance recording, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed local evidence acceptance boundary"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/terminal-no-persistence.json"
target_json="$tmpdir/local-evidence-acceptance-positive-preconditions.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render terminal no-persistence source report"
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render local evidence acceptance positive preconditions report from cached source"
jq -e . "$target_json" >/dev/null || fail "target report did not render valid JSON"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_positive_preconditions_readback_without_acceptance_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_terminal_no_persistence_readback_ready == true
  and .source_terminal_entry_count == 7
  and .source_terminal_closeout_projected_count == 7
  and .source_terminal_no_persistence_confirmed_count == 7
  and .source_terminal_closeout_key_unique_count == 7
  and .source_terminal_closeout_recorded_count == 0
  and .source_terminal_closeout_persisted_count == 0
  and .source_terminal_closeout_accepted_count == 0
  and .source_terminal_closeout_authoritative_count == 0
  and .source_denial_receipt_persistence_attempt_recorded_count == 0
  and .source_denial_receipt_persisted_count == 0
  and .source_acceptance_source_recorded_count == 0
  and .source_acceptance_source_persisted_count == 0
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
  and .positive_preconditions_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/positive-preconditions"
  and .precondition_entry_count == 7
  and .positive_precondition_set_projected_count == 7
  and .positive_precondition_key_projected_count == 7
  and .positive_precondition_key_unique_count == 7
  and .source_terminal_closeout_attached_count == 7
  and .source_persistence_denial_attached_count == 7
  and .source_denial_receipt_attached_count == 7
  and .source_acceptance_source_record_attached_count == 7
  and .local_acceptance_authority_required_count == 7
  and .local_acceptance_authority_present_count == 0
  and .operator_local_acceptance_approval_required_count == 7
  and .operator_local_acceptance_approval_present_count == 0
  and .dev_evidence_acceptance_source_required_count == 7
  and .dev_evidence_acceptance_source_present_count == 0
  and .evidence_payload_source_binding_required_count == 7
  and .evidence_payload_source_binding_present_count == 0
  and .local_evidence_store_feature_gate_required_count == 7
  and .local_evidence_store_feature_gate_enabled_count == 0
  and .local_receipt_store_feature_gate_required_count == 7
  and .local_receipt_store_feature_gate_enabled_count == 0
  and .atomic_acceptance_append_required_count == 7
  and .atomic_acceptance_append_enabled_count == 0
  and .post_acceptance_readback_required_count == 7
  and .post_acceptance_readback_persisted_count == 0
  and .rollback_anchor_required_count == 7
  and .rollback_anchor_verified_count == 0
  and .retention_policy_commit_required_count == 7
  and .retention_policy_committed_count == 0
  and .replay_idempotency_guard_required_count == 7
  and .replay_idempotency_guard_enabled_count == 0
  and .positive_preconditions_missing_count == 7
  and .local_evidence_acceptance_allowed_count == 0
  and .local_evidence_acceptance_recorded_count == 0
  and .evidence_acceptance_recorded_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .local_evidence_acceptance_positive_preconditions_readback_ready == true
  and .local_evidence_acceptance_allowed == false
  and .local_evidence_acceptance_recording_allowed == false
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
  and (.blockers | index("local_acceptance_authority_missing")) != null
  and (.blockers | index("operator_local_acceptance_approval_missing")) != null
  and (.blockers | index("dev_evidence_acceptance_source_missing")) != null
  and (.blockers | index("evidence_payload_source_binding_missing")) != null
  and (.blockers | index("local_evidence_store_feature_gate_closed")) != null
  and (.blockers | index("local_receipt_store_feature_gate_closed")) != null
  and (.blockers | index("atomic_acceptance_append_not_enabled")) != null
  and (.blockers | index("post_acceptance_readback_missing")) != null
  and (.blockers | index("rollback_anchor_missing")) != null
  and (.blockers | index("retention_policy_not_committed")) != null
  and (.blockers | index("replay_idempotency_guard_disabled")) != null
  and (.blockers | index("local_evidence_acceptance_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_positive_preconditions_without_acceptance_"))
    and (.source_terminal_no_persistence_entry_id | startswith("evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_"))
    and (.source_terminal_closeout_id | startswith("local-evidence-acceptance-recording-denial-receipt-persistence-denial-terminal-no-persistence:"))
    and (.source_terminal_closeout_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/persistence-denial/terminal-no-persistence/"))
    and .source_terminal_reason == "local_evidence_acceptance_recording_denial_receipt_persistence_denied_retention_replay_projected_no_persistence_authority"
    and (.source_persistence_denial_id | startswith("local-evidence-acceptance-recording-denial-receipt-persistence-denial:"))
    and (.source_persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/persistence-denial/"))
    and (.source_denial_receipt_id | startswith("local-evidence-acceptance-recording-denial-receipt:"))
    and (.source_denial_receipt_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/"))
    and (.source_denial_receipt_digest | startswith("sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial:"))
    and (.source_acceptance_source_record_id | startswith("local-evidence-acceptance-source-record:"))
    and (.positive_precondition_set_id | startswith("local-evidence-acceptance-positive-preconditions:controlled-live-evidence-receipt-store:"))
    and (.positive_precondition_key | startswith("local-evidence-acceptance-positive-preconditions:controlled-live-evidence-receipt-store:"))
    and (.positive_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/positive-preconditions/"))
    and (.local_acceptance_authority_precondition_id | startswith("local-evidence-acceptance-authority-required:controlled-live-evidence-receipt-store:"))
    and (.operator_local_acceptance_approval_precondition_id | startswith("operator-local-evidence-acceptance-approval-required:controlled-live-evidence-receipt-store:"))
    and (.dev_evidence_acceptance_source_precondition_id | startswith("dev-evidence-acceptance-source-required:controlled-live-evidence-receipt-store:"))
    and (.evidence_payload_source_binding_precondition_id | startswith("evidence-payload-source-binding-required:controlled-live-evidence-receipt-store:"))
    and (.local_evidence_store_feature_gate_precondition_id | startswith("local-evidence-store-feature-gate-required:controlled-live-evidence-receipt-store:"))
    and (.local_receipt_store_feature_gate_precondition_id | startswith("local-receipt-store-feature-gate-required:controlled-live-evidence-receipt-store:"))
    and (.atomic_acceptance_append_precondition_id | startswith("atomic-local-evidence-acceptance-append-required:controlled-live-evidence-receipt-store:"))
    and (.post_acceptance_readback_precondition_id | startswith("post-local-evidence-acceptance-readback-required:controlled-live-evidence-receipt-store:"))
    and (.rollback_anchor_precondition_id | startswith("local-evidence-acceptance-rollback-anchor-required:controlled-live-evidence-receipt-store:"))
    and (.retention_policy_commit_precondition_id | startswith("local-evidence-acceptance-retention-policy-commit-required:controlled-live-evidence-receipt-store:"))
    and (.replay_idempotency_guard_precondition_id | startswith("local-evidence-acceptance-replay-idempotency-guard-required:controlled-live-evidence-receipt-store:"))
    and .observed_state == "local_evidence_acceptance_positive_preconditions_projected_without_acceptance"
    and .positive_precondition_set_projected == true
    and .positive_precondition_key_projected == true
    and .source_terminal_closeout_attached == true
    and .source_persistence_denial_attached == true
    and .source_denial_receipt_binding_attached == true
    and .source_acceptance_source_record_attached == true
    and .local_acceptance_authority_required == true
    and .local_acceptance_authority_present == false
    and .operator_local_acceptance_approval_required == true
    and .operator_local_acceptance_approval_present == false
    and .dev_evidence_acceptance_source_required == true
    and .dev_evidence_acceptance_source_present == false
    and .evidence_payload_source_binding_required == true
    and .evidence_payload_source_binding_present == false
    and .local_evidence_store_feature_gate_required == true
    and .local_evidence_store_feature_gate_enabled == false
    and .local_receipt_store_feature_gate_required == true
    and .local_receipt_store_feature_gate_enabled == false
    and .atomic_acceptance_append_required == true
    and .atomic_acceptance_append_enabled == false
    and .post_acceptance_readback_required == true
    and .post_acceptance_readback_persisted == false
    and .rollback_anchor_required == true
    and .rollback_anchor_verified == false
    and .retention_policy_commit_required == true
    and .retention_policy_committed == false
    and .replay_idempotency_guard_required == true
    and .replay_idempotency_guard_enabled == false
    and .positive_preconditions_missing == true
    and .local_evidence_acceptance_allowed == false
    and .local_evidence_acceptance_recording_allowed == false
    and .local_evidence_acceptance_recorded == false
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
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .positive_precondition_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/positive-preconditions/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .operator_local_acceptance_approval_precondition_id == "operator-local-evidence-acceptance-approval-required:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback"
  and .status == "ready_blocked"
  and .terminal_no_persistence_readback_ready == true
  and .terminal_entry_count == 7
  and .terminal_no_persistence_confirmed_count == 7
  and .terminal_closeout_recorded_count == 0
  and .terminal_closeout_persisted_count == 0
  and .denial_receipt_persisted_count == 0
  and .receipt_store_written_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    local_evidence_acceptance_positive_preconditions --lib
)

printf 'hepta-systems-cler-store-local-evidence-acceptance-positive-preconditions-gate: PASS: local evidence acceptance positive preconditions are read back without acceptance, persistence, or live execution\n'
