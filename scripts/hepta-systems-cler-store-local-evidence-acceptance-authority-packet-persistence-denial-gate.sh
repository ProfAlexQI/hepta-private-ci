#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-non-send-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable local evidence acceptance authority packet persistence-denial report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local evidence acceptance authority packet non-send source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Authority Packet Persistence Denial Readback Without Persistence' "$DOC" \
  || fail "architecture note must document the local authority packet persistence-denial surface"
rg -q 'controlled live evidence receipt store local evidence acceptance authority packet persistence denial readback without persistence' "$DOC" \
  || fail "architecture note must document the plain-language local persistence-denial boundary"
rg -q 'no operator packet send, send attempt record, operator packet persistence, packet persistence attempt record, packet persistence denial receipt persistence, local evidence acceptance authority, authority decision recording, non-authority receipt persistence, local evidence acceptance, local evidence acceptance recording, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed local persistence-denial boundary"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/local-evidence-acceptance-authority-packet-non-send.json"
target_json="$tmpdir/local-evidence-acceptance-authority-packet-persistence-denial.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render local evidence acceptance authority packet non-send source report"
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_NON_SEND_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render local evidence acceptance authority packet persistence-denial report from cached source"
jq -e . "$target_json" >/dev/null || fail "target report did not render valid JSON"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_non_send_readback_ready == true
  and .source_non_send_entry_count == 7
  and .source_unsent_packet_count == 7
  and .source_send_disabled_count == 7
  and .source_send_allowed_count == 0
  and .source_send_attempt_recorded_count == 0
  and .source_packet_persistence_disabled_count == 7
  and .source_operator_packet_sent_count == 0
  and .source_operator_packet_persisted_count == 0
  and .source_local_evidence_acceptance_authority_present_count == 0
  and .source_local_evidence_acceptance_allowed_count == 0
  and .source_local_evidence_acceptance_recorded_count == 0
  and .source_authority_decision_recorded_count == 0
  and .source_non_authority_receipt_projected_count == 7
  and .source_non_authority_receipt_persisted_count == 0
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
  and .persistence_denial_entry_count == 7
  and .persistence_denial_projected_count == 7
  and .packet_persistence_denied_count == 7
  and .packet_persistence_disabled_count == 7
  and .packet_persistence_allowed_count == 0
  and .packet_persistence_attempt_recorded_count == 0
  and .packet_persisted_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .non_send_projection_count == 7
  and .send_attempt_recorded_count == 0
  and .local_evidence_acceptance_authority_present_count == 0
  and .local_evidence_acceptance_allowed_count == 0
  and .local_evidence_acceptance_recorded_count == 0
  and .authority_decision_recorded_count == 0
  and .non_authority_receipt_projected_count == 7
  and .non_authority_receipt_persisted_count == 0
  and .evidence_acceptance_recorded_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .local_evidence_acceptance_authority_packet_persistence_denial_readback_ready == true
  and .operator_packet_send_allowed == false
  and .operator_packet_sent == false
  and .operator_packet_persistence_allowed == false
  and .operator_packet_persisted == false
  and .packet_persistence_attempt_recording_allowed == false
  and .packet_persistence_attempt_recorded == false
  and .packet_persistence_denial_receipt_persistence_allowed == false
  and .packet_persistence_denial_receipt_persisted == false
  and .local_evidence_acceptance_authority_allowed == false
  and .authority_decision_recording_allowed == false
  and .non_authority_receipt_persistence_allowed == false
  and .local_evidence_acceptance_allowed == false
  and .local_evidence_acceptance_recording_allowed == false
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
  and (.blockers | index("operator_packet_persistence_disabled")) != null
  and (.blockers | index("packet_persistence_attempt_recording_disabled")) != null
  and (.blockers | index("packet_persistence_denial_receipt_persistence_disabled")) != null
  and (.blockers | index("local_evidence_acceptance_authority_missing")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_without_persistence_"))
    and .source_authority_packet_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"
    and .source_authority_packet_route == "operator-packet://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority"
    and (.source_authority_packet_key | startswith("local-evidence-acceptance-authority-packet:controlled-live-evidence-receipt-store:"))
    and (.source_packet_non_send_readback_id | startswith("local-evidence-acceptance-authority-packet-non-send:controlled-live-evidence-receipt-store:"))
    and (.source_packet_non_send_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/non-send/"))
    and (.source_authority_decision_request_id | startswith("local-evidence-acceptance-authority-decision-request:controlled-live-evidence-receipt-store:"))
    and (.source_non_authority_receipt_id | startswith("local-evidence-acceptance-non-authority-receipt:controlled-live-evidence-receipt-store:"))
    and (.packet_persistence_denial_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial:controlled-live-evidence-receipt-store:"))
    and (.packet_persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/"))
    and (.packet_persistence_denial_receipt_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-receipt:controlled-live-evidence-receipt-store:"))
    and .packet_persistence_denial_reason == "operator_packet_persistence_disabled_local_evidence_acceptance_authority_missing_local_receipt_store_write_disabled"
    and .observed_state == "local_evidence_acceptance_authority_packet_persistence_denied_without_persistence"
    and .source_packet_unsent == true
    and .source_send_disabled == true
    and .non_send_projected == true
    and .packet_persistence_denial_projected == true
    and .packet_persistence_denied == true
    and .packet_persistence_disabled == true
    and .packet_persistence_allowed == false
    and .packet_persistence_attempt_recorded == false
    and .packet_persisted == false
    and .operator_packet_sent == false
    and .operator_packet_persisted == false
    and .local_evidence_acceptance_authority_required == true
    and .local_evidence_acceptance_authority_present == false
    and .authority_decision_request_projected == true
    and .authority_decision_recorded == false
    and .non_authority_receipt_projected == true
    and .non_authority_receipt_persisted == false
    and .local_evidence_acceptance_authority_allowed == false
    and .local_evidence_acceptance_allowed == false
    and .local_evidence_acceptance_recording_allowed == false
    and .local_evidence_acceptance_recorded == false
    and .evidence_acceptance_recording_allowed == false
    and .evidence_acceptance_recorded == false
    and .evidence_recording_allowed == false
    and .evidence_recorded == false
    and .receipt_store_write_attempt_recording_allowed == false
    and .receipt_store_write_attempt_recorded == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .receipt_store_written == false
    and .ledger_written == false
    and .workflow_event_log_written == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .packet_persistence_denial_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/operator-live-approval-missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback"
  and .status == "ready_blocked"
  and .local_evidence_acceptance_authority_packet_non_send_readback_ready == true
  and .non_send_entry_count == 7
  and .unsent_packet_count == 7
  and .send_disabled_count == 7
  and .send_allowed_count == 0
  and .send_attempt_recorded_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .local_evidence_acceptance_allowed_count == 0
  and .receipt_store_written_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime local_evidence_acceptance_authority_packet_persistence_denial --lib
)

printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-gate: PASS: local evidence acceptance authority packet persistence denial is read back without persistence or live execution\n'
