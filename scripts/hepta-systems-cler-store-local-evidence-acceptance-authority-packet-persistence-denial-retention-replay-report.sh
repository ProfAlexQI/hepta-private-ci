#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RETENTION_REPLAY_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local evidence acceptance authority packet persistence-denial report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing local evidence acceptance authority packet persistence-denial retention/replay Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_JSON:-}"
source_cache_input_present=false
source_report_render_count=0
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached local evidence acceptance authority packet persistence-denial report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/local-evidence-acceptance-authority-packet-persistence-denial.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render local evidence acceptance authority packet persistence-denial source report"
  source_report_render_count=1
fi
jq -e . "$source_json" >/dev/null || fail "local evidence acceptance authority packet persistence-denial source report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --argjson source_report_render_count "$source_report_render_count" \
  --arg gate "scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_RETENTION_REPLAY_2026-07-07.md" \
  --arg collection_id "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-persistence-denial-retention-replay" \
  --arg collection_route "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/persistence-denial/retention-replay" \
  '
  def hyphen_id($id): $id | gsub("_"; "-");
  def count_true($entries; $key): $entries | map(select(.[$key] == true)) | length;
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_without_persistence_" + .source_blocker_id),
    source_blocker_id,
    source_persistence_denial_entry_id:.id,
    source_persistence_denial_id:.packet_persistence_denial_id,
    source_persistence_denial_route:.packet_persistence_denial_route,
    source_persistence_denial_reason:.packet_persistence_denial_reason,
    source_packet_persistence_denial_receipt_id:.packet_persistence_denial_receipt_id,
    source_authority_packet_id:.source_authority_packet_id,
    source_authority_packet_route:.source_authority_packet_route,
    source_authority_packet_key:.source_authority_packet_key,
    source_packet_non_send_readback_id:.source_packet_non_send_readback_id,
    source_packet_non_send_readback_route:.source_packet_non_send_readback_route,
    source_authority_decision_request_id:.source_authority_decision_request_id,
    source_authority_decision_request_route:.source_authority_decision_request_route,
    source_non_authority_receipt_id:.source_non_authority_receipt_id,
    source_non_authority_receipt_route:.source_non_authority_receipt_route,
    retention_policy_id:("local-evidence-acceptance-authority-packet-persistence-denial-retention-policy:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    retention_policy_route:($collection_route + "/retention/" + hyphen_id(.source_blocker_id)),
    expiry_guard_id:("local-evidence-acceptance-authority-packet-persistence-denial-expiry-guard:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    replay_key:("local-evidence-acceptance-authority-packet-persistence-denial-replay-key:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    replay_idempotency_key:("local-evidence-acceptance-authority-packet-persistence-denial-replay-idempotency:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    replay_readback_route:($collection_route + "/replay/" + hyphen_id(.source_blocker_id)),
    retention_readback_route:($collection_route + "/readback/" + hyphen_id(.source_blocker_id)),
    garbage_collection_denial_id:("local-evidence-acceptance-authority-packet-persistence-denial-gc-denial:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    supersession_guard_id:("local-evidence-acceptance-authority-packet-persistence-denial-supersession-guard:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    zero_effect_digest:("sha256:local-evidence-acceptance-authority-packet-persistence-denial-retention-replay-zero-effect:" + .source_blocker_id),
    retention_state:"projected_not_persisted",
    replay_state:"projected_not_written",
    observed_state:"local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_projected_without_persistence",
    source_persistence_denial_attached:.packet_persistence_denial_projected,
    source_packet_persistence_denial_receipt_attached:(.packet_persistence_denial_receipt_id != null and (.packet_persistence_denial_receipt_id | length) > 0),
    source_non_send_readback_attached:.non_send_projected,
    source_authority_packet_attached:(.source_authority_packet_id == "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet"),
    source_persistence_denial_projected:.packet_persistence_denial_projected,
    source_packet_persistence_denied:.packet_persistence_denied,
    retention_policy_projected:true,
    expiry_guard_projected:true,
    replay_key_projected:true,
    replay_idempotency_key_projected:true,
    retention_readback_route_projected:true,
    replay_readback_route_projected:true,
    garbage_collection_denial_projected:true,
    supersession_guard_projected:true,
    zero_effect_digest_projected:true,
    retention_policy_persistence_allowed:false,
    retention_policy_persisted:false,
    replay_index_write_allowed:false,
    replay_index_written:false,
    expiry_enforcement_allowed:false,
    expiry_enforced:false,
    garbage_collection_allowed:false,
    garbage_collection_performed:false,
    packet_persistence_attempt_recording_allowed:false,
    packet_persistence_attempt_recorded:false,
    packet_persistence_denial_receipt_persistence_allowed:false,
    packet_persistence_denial_receipt_persisted:false,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    local_evidence_acceptance_authority_present:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recorded:false,
    authority_decision_recorded:false,
    non_authority_receipt_persisted:false,
    evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recorded:false,
    evidence_recording_allowed:false,
    evidence_recorded:false,
    receipt_store_write_attempt_recording_allowed:false,
    receipt_store_write_attempt_recorded:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    receipt_persistence_allowed:false,
    receipt_persisted:false,
    ledger_write_allowed:false,
    ledger_written:false,
    workflow_event_log_write_allowed:false,
    workflow_event_log_written:false,
    sqlite_write_allowed:false,
    sqlite_written:false,
    credential_read_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  (count_true($entries; "retention_policy_projected")) as $retention_policy_projected_count |
  (count_true($entries; "expiry_guard_projected")) as $expiry_guard_projected_count |
  (count_true($entries; "replay_key_projected")) as $replay_key_projected_count |
  (count_true($entries; "replay_idempotency_key_projected")) as $replay_idempotency_key_projected_count |
  ($entries | map(.replay_idempotency_key) | unique | length) as $replay_idempotency_key_unique_count |
  (count_true($entries; "retention_readback_route_projected")) as $retention_readback_route_projected_count |
  (count_true($entries; "replay_readback_route_projected")) as $replay_readback_route_projected_count |
  (count_true($entries; "garbage_collection_denial_projected")) as $garbage_collection_denial_projected_count |
  (count_true($entries; "supersession_guard_projected")) as $supersession_guard_projected_count |
  (count_true($entries; "zero_effect_digest_projected")) as $zero_effect_digest_projected_count |
  (count_true($entries; "source_persistence_denial_attached")) as $source_persistence_denial_attached_count |
  (count_true($entries; "source_packet_persistence_denial_receipt_attached")) as $source_packet_persistence_denial_receipt_attached_count |
  (count_true($entries; "source_non_send_readback_attached")) as $source_non_send_readback_attached_count |
  (count_true($entries; "source_authority_packet_attached")) as $source_authority_packet_attached_count |
  (count_true($entries; "retention_policy_persisted")) as $retention_policy_persisted_count |
  (count_true($entries; "replay_index_written")) as $replay_index_written_count |
  (count_true($entries; "expiry_enforced")) as $expiry_enforced_count |
  (count_true($entries; "garbage_collection_performed")) as $garbage_collection_performed_count |
  (count_true($entries; "packet_persistence_attempt_recorded")) as $packet_persistence_attempt_recorded_count |
  (count_true($entries; "packet_persistence_denial_receipt_persisted")) as $packet_persistence_denial_receipt_persisted_count |
  (count_true($entries; "operator_packet_sent")) as $operator_packet_sent_count |
  (count_true($entries; "operator_packet_persisted")) as $operator_packet_persisted_count |
  (count_true($entries; "local_evidence_acceptance_authority_present")) as $local_evidence_acceptance_authority_present_count |
  (count_true($entries; "local_evidence_acceptance_allowed")) as $local_evidence_acceptance_allowed_count |
  (count_true($entries; "local_evidence_acceptance_recorded")) as $local_evidence_acceptance_recorded_count |
  (count_true($entries; "authority_decision_recorded")) as $authority_decision_recorded_count |
  (count_true($entries; "non_authority_receipt_persisted")) as $non_authority_receipt_persisted_count |
  (count_true($entries; "evidence_acceptance_recorded")) as $evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_recorded")) as $evidence_recorded_count |
  (count_true($entries; "receipt_store_write_attempt_recorded")) as $receipt_store_write_attempt_recorded_count |
  (count_true($entries; "receipt_store_written")) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true or .non_authority_receipt_persisted == true or .packet_persistence_denial_receipt_persisted == true)) | length) as $receipt_persisted_count |
  (count_true($entries; "ledger_written")) as $ledger_written_count |
  (count_true($entries; "workflow_event_log_written")) as $workflow_event_log_written_count |
  (count_true($entries; "sqlite_written")) as $sqlite_written_count |
  (count_true($entries; "live_mutation_allowed")) as $live_mutation_allowed_count |
  ($src.local_evidence_acceptance_authority_packet_persistence_denial_readback_ready == true
    and $src.persistence_denial_entry_count == 7
    and $src.persistence_denial_projected_count == 7
    and $src.packet_persistence_denied_count == 7
    and $src.packet_persistence_allowed_count == 0
    and $src.packet_persistence_attempt_recorded_count == 0
    and $src.packet_persisted_count == 0
    and $src.operator_packet_sent_count == 0
    and $src.operator_packet_persisted_count == 0
    and $src.local_evidence_acceptance_authority_present_count == 0
    and $src.local_evidence_acceptance_allowed_count == 0
    and $src.local_evidence_acceptance_recorded_count == 0
    and $src.authority_decision_recorded_count == 0
    and $src.non_authority_receipt_persisted_count == 0
    and $src.evidence_acceptance_recorded_count == 0
    and $src.evidence_recorded_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $retention_policy_projected_count == 7
    and $expiry_guard_projected_count == 7
    and $replay_key_projected_count == 7
    and $replay_idempotency_key_projected_count == 7
    and $replay_idempotency_key_unique_count == 7
    and $retention_readback_route_projected_count == 7
    and $replay_readback_route_projected_count == 7
    and $garbage_collection_denial_projected_count == 7
    and $supersession_guard_projected_count == 7
    and $zero_effect_digest_projected_count == 7
    and $source_persistence_denial_attached_count == 7
    and $source_packet_persistence_denial_receipt_attached_count == 7
    and $source_non_send_readback_attached_count == 7
    and $source_authority_packet_attached_count == 7
    and $retention_policy_persisted_count == 0
    and $replay_index_written_count == 0
    and $expiry_enforced_count == 0
    and $garbage_collection_performed_count == 0
    and $packet_persistence_attempt_recorded_count == 0
    and $packet_persistence_denial_receipt_persisted_count == 0
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $local_evidence_acceptance_authority_present_count == 0
    and $local_evidence_acceptance_allowed_count == 0
    and $local_evidence_acceptance_recorded_count == 0
    and $authority_decision_recorded_count == 0
    and $non_authority_receipt_persisted_count == 0
    and $evidence_acceptance_recorded_count == 0
    and $evidence_recorded_count == 0
    and $receipt_store_write_attempt_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_projected_without_persistence"
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
      and .live_mutation_allowed == false))) as $ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_gate",
    schema_version:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_retention_replay_readback_without_persistence_v1",
    plugin_id:"hepta-system@hepta-local",
    source_persistence_denial_readback_ready:$src.local_evidence_acceptance_authority_packet_persistence_denial_readback_ready,
    source_persistence_denial_entry_count:$src.persistence_denial_entry_count,
    source_persistence_denial_projected_count:$src.persistence_denial_projected_count,
    source_packet_persistence_denied_count:$src.packet_persistence_denied_count,
    source_packet_persistence_allowed_count:$src.packet_persistence_allowed_count,
    source_packet_persistence_attempt_recorded_count:$src.packet_persistence_attempt_recorded_count,
    source_packet_persisted_count:$src.packet_persisted_count,
    source_operator_packet_sent_count:$src.operator_packet_sent_count,
    source_operator_packet_persisted_count:$src.operator_packet_persisted_count,
    source_local_evidence_acceptance_authority_present_count:$src.local_evidence_acceptance_authority_present_count,
    source_local_evidence_acceptance_allowed_count:$src.local_evidence_acceptance_allowed_count,
    source_local_evidence_acceptance_recorded_count:$src.local_evidence_acceptance_recorded_count,
    source_authority_decision_recorded_count:$src.authority_decision_recorded_count,
    source_non_authority_receipt_persisted_count:$src.non_authority_receipt_persisted_count,
    source_evidence_acceptance_recorded_count:$src.evidence_acceptance_recorded_count,
    source_evidence_recorded_count:$src.evidence_recorded_count,
    source_receipt_store_write_attempt_recorded_count:$src.receipt_store_write_attempt_recorded_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_receipt_persisted_count:$src.receipt_persisted_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    source_cache_mode:(if $source_cache_input_present then "provided_source_json" else "rendered_once_temp_source_json" end),
    source_cache_input_present:$source_cache_input_present,
    source_report_render_count:$source_report_render_count,
    target_source_reuse_count:1,
    lib_export_present:$lib_export_present,
    retention_replay_collection_id:$collection_id,
    retention_replay_collection_route:$collection_route,
    retention_replay_entry_count:($entries | length),
    retention_policy_projected_count:$retention_policy_projected_count,
    expiry_guard_projected_count:$expiry_guard_projected_count,
    replay_key_projected_count:$replay_key_projected_count,
    replay_idempotency_key_projected_count:$replay_idempotency_key_projected_count,
    replay_idempotency_key_unique_count:$replay_idempotency_key_unique_count,
    retention_readback_route_projected_count:$retention_readback_route_projected_count,
    replay_readback_route_projected_count:$replay_readback_route_projected_count,
    garbage_collection_denial_projected_count:$garbage_collection_denial_projected_count,
    supersession_guard_projected_count:$supersession_guard_projected_count,
    zero_effect_digest_projected_count:$zero_effect_digest_projected_count,
    source_persistence_denial_attached_count:$source_persistence_denial_attached_count,
    source_packet_persistence_denial_receipt_attached_count:$source_packet_persistence_denial_receipt_attached_count,
    source_non_send_readback_attached_count:$source_non_send_readback_attached_count,
    source_authority_packet_attached_count:$source_authority_packet_attached_count,
    retention_policy_persisted_count:$retention_policy_persisted_count,
    replay_index_written_count:$replay_index_written_count,
    expiry_enforced_count:$expiry_enforced_count,
    garbage_collection_performed_count:$garbage_collection_performed_count,
    packet_persistence_attempt_recorded_count:$packet_persistence_attempt_recorded_count,
    packet_persistence_denial_receipt_persisted_count:$packet_persistence_denial_receipt_persisted_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    local_evidence_acceptance_authority_present_count:$local_evidence_acceptance_authority_present_count,
    local_evidence_acceptance_allowed_count:$local_evidence_acceptance_allowed_count,
    local_evidence_acceptance_recorded_count:$local_evidence_acceptance_recorded_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
    evidence_acceptance_recorded_count:$evidence_acceptance_recorded_count,
    evidence_recorded_count:$evidence_recorded_count,
    receipt_store_write_attempt_recorded_count:$receipt_store_write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    retention_replay_readback_ready:$ready,
    retention_policy_persistence_allowed:false,
    replay_index_write_allowed:false,
    expiry_enforcement_allowed:false,
    garbage_collection_allowed:false,
    packet_persistence_attempt_recording_allowed:false,
    packet_persistence_denial_receipt_persistence_allowed:false,
    operator_packet_persistence_allowed:false,
    local_evidence_acceptance_authority_allowed:false,
    authority_decision_recording_allowed:false,
    non_authority_receipt_persistence_allowed:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recording_allowed:false,
    evidence_recording_allowed:false,
    receipt_store_write_attempt_recording_allowed:false,
    receipt_store_write_allowed:false,
    receipt_persistence_allowed:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "retention_policy_persistence_disabled",
      "replay_index_write_disabled",
      "expiry_enforcement_disabled",
      "garbage_collection_disabled",
      "packet_persistence_attempt_recording_disabled",
      "packet_persistence_denial_receipt_persistence_disabled",
      "operator_packet_persistence_disabled",
      "local_evidence_acceptance_authority_missing",
      "authority_decision_recording_disabled",
      "non_authority_receipt_persistence_disabled",
      "local_evidence_acceptance_disabled",
      "local_evidence_acceptance_recording_disabled",
      "evidence_acceptance_recording_disabled",
      "evidence_recording_disabled",
      "receipt_store_write_attempt_recording_disabled",
      "receipt_store_write_disabled",
      "receipt_persistence_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback",
      "keep_local_evidence_acceptance_authority_packet_persistence_denial_query_only_until_local_store_is_open"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_terminal_no_persistence_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      retention_policy_persisted:false,
      replay_index_written:false,
      expiry_enforced:false,
      garbage_collection_performed:false,
      packet_persistence_attempt_recorded:false,
      packet_persistence_denial_receipt_persisted:false,
      operator_packet_sent:false,
      operator_packet_persisted:false,
      local_evidence_acceptance_authority_accepted:false,
      authority_decision_recorded:false,
      non_authority_receipt_persisted:false,
      local_evidence_acceptance_recorded:false,
      evidence_acceptance_recorded:false,
      evidence_recorded:false,
      receipt_store_write_attempt_recorded:false,
      receipt_store_written:false,
      receipt_persisted:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      credential_read:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      replay_executed:false,
      rollback_executed:false,
      kill_switch_rehearsal_executed:false,
      kill_switch_mutated:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
