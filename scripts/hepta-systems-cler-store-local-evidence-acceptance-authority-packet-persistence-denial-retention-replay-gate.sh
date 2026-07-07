#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RETENTION_REPLAY_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable local authority packet persistence-denial retention/replay report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local authority packet persistence-denial source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/local-evidence-acceptance-authority-packet-persistence-denial.json"
target_json="$tmpdir/local-evidence-acceptance-authority-packet-persistence-denial-retention-replay.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render local authority packet persistence-denial source report"
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render target report from cached source"
jq -e . "$target_json" >/dev/null || fail "target report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Acceptance Authority Packet Persistence Denial Retention Replay Readback Without Persistence' "$DOC" \
  || fail "architecture note must document the local authority packet persistence-denial retention/replay surface"
rg -q 'controlled live evidence receipt store local evidence acceptance authority packet persistence denial retention replay readback without persistence' "$DOC" \
  || fail "architecture note must document the plain-language local authority packet persistence-denial retention/replay boundary"
rg -q 'no retention policy persistence, replay index write, expiry enforcement, garbage collection, packet persistence attempt recording, packet persistence denial receipt persistence, operator packet send, operator packet persistence, local evidence acceptance authority, authority decision recording, non-authority receipt persistence, local evidence acceptance, local evidence acceptance recording, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed local authority packet persistence-denial retention/replay boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_persistence_denial_readback_ready == true
  and .source_persistence_denial_entry_count == 7
  and .source_persistence_denial_projected_count == 7
  and .source_packet_persistence_denied_count == 7
  and .source_packet_persistence_allowed_count == 0
  and .source_packet_persistence_attempt_recorded_count == 0
  and .source_packet_persisted_count == 0
  and .source_operator_packet_sent_count == 0
  and .source_operator_packet_persisted_count == 0
  and .source_local_evidence_acceptance_authority_present_count == 0
  and .source_local_evidence_acceptance_allowed_count == 0
  and .source_local_evidence_acceptance_recorded_count == 0
  and .source_authority_decision_recorded_count == 0
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
  and .retention_replay_collection_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay"
  and .retention_replay_collection_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay"
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
  and .source_persistence_denial_attached_count == 7
  and .source_packet_persistence_denial_receipt_attached_count == 7
  and .source_non_send_readback_attached_count == 7
  and .source_authority_packet_attached_count == 7
  and .retention_policy_persisted_count == 0
  and .replay_index_written_count == 0
  and .expiry_enforced_count == 0
  and .garbage_collection_performed_count == 0
  and .packet_persistence_attempt_recorded_count == 0
  and .packet_persistence_denial_receipt_persisted_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .local_evidence_acceptance_authority_present_count == 0
  and .local_evidence_acceptance_allowed_count == 0
  and .local_evidence_acceptance_recorded_count == 0
  and .authority_decision_recorded_count == 0
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
  and .retention_replay_readback_ready == true
  and .retention_policy_persistence_allowed == false
  and .replay_index_write_allowed == false
  and .expiry_enforcement_allowed == false
  and .garbage_collection_allowed == false
  and .packet_persistence_attempt_recording_allowed == false
  and .packet_persistence_denial_receipt_persistence_allowed == false
  and .operator_packet_persistence_allowed == false
  and .local_evidence_acceptance_authority_allowed == false
  and .authority_decision_recording_allowed == false
  and .non_authority_receipt_persistence_allowed == false
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
  and (.blockers | index("retention_policy_persistence_disabled")) != null
  and (.blockers | index("replay_index_write_disabled")) != null
  and (.blockers | index("expiry_enforcement_disabled")) != null
  and (.blockers | index("garbage_collection_disabled")) != null
  and (.blockers | index("packet_persistence_attempt_recording_disabled")) != null
  and (.blockers | index("packet_persistence_denial_receipt_persistence_disabled")) != null
  and (.blockers | index("operator_packet_persistence_disabled")) != null
  and (.blockers | index("local_evidence_acceptance_authority_missing")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_without_persistence_"))
    and (.source_persistence_denial_entry_id | startswith("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_without_persistence_"))
    and (.source_persistence_denial_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial:"))
    and (.source_persistence_denial_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/"))
    and .source_persistence_denial_reason == "operator_packet_persistence_disabled_local_evidence_acceptance_authority_missing_local_receipt_store_write_disabled"
    and (.source_packet_persistence_denial_receipt_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-receipt:"))
    and .source_authority_packet_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"
    and .source_authority_packet_route == "operator-packet://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority"
    and (.source_authority_packet_key | startswith("local-evidence-acceptance-authority-packet:controlled-live-evidence-receipt-store:"))
    and (.source_packet_non_send_readback_id | startswith("local-evidence-acceptance-authority-packet-non-send:"))
    and (.source_packet_non_send_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/non-send/"))
    and (.source_authority_decision_request_id | startswith("local-evidence-acceptance-authority-decision-request:"))
    and (.source_non_authority_receipt_id | startswith("local-evidence-acceptance-non-authority-receipt:"))
    and (.retention_policy_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-retention-policy:"))
    and (.retention_policy_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay/retention/"))
    and (.expiry_guard_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-expiry-guard:"))
    and (.replay_key | startswith("local-evidence-acceptance-authority-packet-persistence-denial-replay-key:"))
    and (.replay_idempotency_key | startswith("local-evidence-acceptance-authority-packet-persistence-denial-replay-idempotency:"))
    and (.replay_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay/replay/"))
    and (.retention_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay/readback/"))
    and (.garbage_collection_denial_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-gc-denial:"))
    and (.supersession_guard_id | startswith("local-evidence-acceptance-authority-packet-persistence-denial-supersession-guard:"))
    and (.zero_effect_digest | startswith("sha256:local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-zero-effect:"))
    and .retention_state == "projected_not_persisted"
    and .replay_state == "projected_not_written"
    and .observed_state == "local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_projected_without_persistence"
    and .source_persistence_denial_attached == true
    and .source_packet_persistence_denial_receipt_attached == true
    and .source_non_send_readback_attached == true
    and .source_authority_packet_attached == true
    and .source_persistence_denial_projected == true
    and .source_packet_persistence_denied == true
    and .retention_policy_projected == true
    and .expiry_guard_projected == true
    and .replay_key_projected == true
    and .replay_idempotency_key_projected == true
    and .retention_readback_route_projected == true
    and .replay_readback_route_projected == true
    and .garbage_collection_denial_projected == true
    and .supersession_guard_projected == true
    and .zero_effect_digest_projected == true
    and .retention_policy_persisted == false
    and .replay_index_written == false
    and .expiry_enforced == false
    and .garbage_collection_performed == false
    and .packet_persistence_attempt_recorded == false
    and .packet_persistence_denial_receipt_persisted == false
    and .operator_packet_sent == false
    and .operator_packet_persisted == false
    and .local_evidence_acceptance_authority_present == false
    and .local_evidence_acceptance_allowed == false
    and .local_evidence_acceptance_recorded == false
    and .authority_decision_recorded == false
    and .non_authority_receipt_persisted == false
    and .evidence_acceptance_recorded == false
    and .evidence_recorded == false
    and .receipt_store_write_attempt_recorded == false
    and .receipt_store_written == false
    and .receipt_persisted == false
    and .ledger_written == false
    and .workflow_event_log_written == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .retention_policy_route == "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay/retention/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .replay_idempotency_key == "local-evidence-acceptance-authority-packet-persistence-denial-replay-idempotency:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .zero_effect_digest == "sha256:local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-zero-effect:fresh_soak_readback_missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .surface == "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence"
  and .status == "ready_blocked"
  and .local_evidence_acceptance_authority_packet_persistence_denial_readback_ready == true
  and .persistence_denial_entry_count == 7
  and .persistence_denial_projected_count == 7
  and .packet_persistence_denied_count == 7
  and .packet_persistence_attempt_recorded_count == 0
  and .packet_persisted_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .local_evidence_acceptance_allowed_count == 0
  and .authority_decision_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    local_authority_packet_persistence_denial_retention_replay --lib
)

printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-gate: PASS: local evidence acceptance authority packet persistence-denial retention/replay is read back without persistence or live execution\n'
