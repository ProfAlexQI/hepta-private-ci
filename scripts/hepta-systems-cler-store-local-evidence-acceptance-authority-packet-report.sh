#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-positive-preconditions-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local evidence acceptance positive preconditions source report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing local evidence acceptance authority packet Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_POSITIVE_PRECONDITIONS_JSON:-}"
source_cache_input_present=false
source_report_render_count=0
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached local evidence acceptance positive preconditions report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/local-evidence-acceptance-positive-preconditions.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render local evidence acceptance positive preconditions source report"
  source_report_render_count=1
fi
jq -e . "$source_json" >/dev/null || fail "local evidence acceptance positive preconditions source report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --argjson source_report_render_count "$source_report_render_count" \
  --arg gate "scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_2026-07-07.md" \
  --arg packet_id "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet" \
  --arg packet_route "operator-packet://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority" \
  --arg packet_fingerprint "sha256:controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-packet-no-acceptance" \
  '
  def hyphen_id($id): $id | gsub("_"; "-");
  def count_true($entries; $key): $entries | map(select(.[$key] == true)) | length;
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_local_evidence_acceptance_authority_packet_without_acceptance_" + .source_blocker_id),
    source_blocker_id,
    source_positive_precondition_set_id:.positive_precondition_set_id,
    source_positive_precondition_key:.positive_precondition_key,
    source_positive_precondition_route:.positive_precondition_route,
    source_terminal_no_persistence_entry_id,
    source_terminal_closeout_id,
    source_terminal_closeout_route,
    source_persistence_denial_id,
    source_persistence_denial_route,
    source_denial_receipt_id,
    source_denial_receipt_route,
    source_acceptance_source_record_id,
    source_local_acceptance_authority_precondition_id:.local_acceptance_authority_precondition_id,
    source_operator_local_acceptance_approval_precondition_id:.operator_local_acceptance_approval_precondition_id,
    source_dev_evidence_acceptance_source_precondition_id:.dev_evidence_acceptance_source_precondition_id,
    source_evidence_payload_source_binding_precondition_id:.evidence_payload_source_binding_precondition_id,
    source_local_evidence_store_feature_gate_precondition_id:.local_evidence_store_feature_gate_precondition_id,
    source_local_receipt_store_feature_gate_precondition_id:.local_receipt_store_feature_gate_precondition_id,
    source_atomic_acceptance_append_precondition_id:.atomic_acceptance_append_precondition_id,
    source_post_acceptance_readback_precondition_id:.post_acceptance_readback_precondition_id,
    source_rollback_anchor_precondition_id:.rollback_anchor_precondition_id,
    source_retention_policy_commit_precondition_id:.retention_policy_commit_precondition_id,
    source_replay_idempotency_guard_precondition_id:.replay_idempotency_guard_precondition_id,
    authority_packet_id:$packet_id,
    authority_packet_route:$packet_route,
    authority_packet_payload_fingerprint:$packet_fingerprint,
    authority_packet_key:("local-evidence-acceptance-authority-packet:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    authority_decision_request_id:("local-evidence-acceptance-authority-decision-request:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    authority_decision_request_route:("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/decision-request/" + hyphen_id(.source_blocker_id)),
    non_authority_receipt_id:("local-evidence-acceptance-non-authority-receipt:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    non_authority_receipt_route:("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/non-authority-receipts/" + hyphen_id(.source_blocker_id)),
    observed_state:"local_evidence_acceptance_authority_packet_projected_without_acceptance",
    packet_projected:true,
    packet_ready:true,
    authority_packet_key_projected:true,
    authority_checklist_projected:true,
    authority_item_required_count:11,
    authority_item_present_count:0,
    source_positive_preconditions_attached:.positive_precondition_set_projected,
    source_terminal_closeout_attached,
    source_persistence_denial_attached,
    source_denial_receipt_binding_attached,
    source_acceptance_source_record_attached,
    local_acceptance_authority_required,
    local_acceptance_authority_present,
    operator_local_acceptance_approval_required,
    operator_local_acceptance_approval_present,
    dev_evidence_acceptance_source_required,
    dev_evidence_acceptance_source_present,
    evidence_payload_source_binding_required,
    evidence_payload_source_binding_present,
    local_evidence_store_feature_gate_required,
    local_evidence_store_feature_gate_enabled,
    local_receipt_store_feature_gate_required,
    local_receipt_store_feature_gate_enabled,
    atomic_acceptance_append_required,
    atomic_acceptance_append_enabled,
    post_acceptance_readback_required,
    post_acceptance_readback_persisted,
    rollback_anchor_required,
    rollback_anchor_verified,
    retention_policy_commit_required,
    retention_policy_committed,
    replay_idempotency_guard_required,
    replay_idempotency_guard_enabled,
    authority_decision_request_projected:true,
    authority_decision_recorded:false,
    non_authority_receipt_projected:true,
    non_authority_receipt_persisted:false,
    operator_packet_send_allowed:false,
    operator_packet_sent:false,
    operator_packet_persistence_allowed:false,
    operator_packet_persisted:false,
    local_evidence_acceptance_authority_allowed:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    local_evidence_acceptance_recorded:false,
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
  (count_true($entries; "packet_projected")) as $packet_projected_count |
  (count_true($entries; "packet_ready")) as $packet_ready_count |
  (count_true($entries; "authority_packet_key_projected")) as $authority_packet_key_projected_count |
  ($entries | map(.authority_packet_key) | unique | length) as $authority_packet_key_unique_count |
  (count_true($entries; "authority_checklist_projected")) as $authority_checklist_projected_count |
  ($entries | map(.authority_item_required_count) | add) as $authority_item_required_count |
  ($entries | map(.authority_item_present_count) | add) as $authority_item_present_count |
  (count_true($entries; "source_positive_preconditions_attached")) as $source_positive_preconditions_attached_count |
  (count_true($entries; "source_terminal_closeout_attached")) as $source_terminal_closeout_attached_count |
  (count_true($entries; "source_persistence_denial_attached")) as $source_persistence_denial_attached_count |
  (count_true($entries; "source_denial_receipt_binding_attached")) as $source_denial_receipt_attached_count |
  (count_true($entries; "source_acceptance_source_record_attached")) as $source_acceptance_source_record_attached_count |
  (count_true($entries; "local_acceptance_authority_required")) as $local_acceptance_authority_required_count |
  (count_true($entries; "local_acceptance_authority_present")) as $local_acceptance_authority_present_count |
  (count_true($entries; "operator_local_acceptance_approval_required")) as $operator_local_acceptance_approval_required_count |
  (count_true($entries; "operator_local_acceptance_approval_present")) as $operator_local_acceptance_approval_present_count |
  (count_true($entries; "dev_evidence_acceptance_source_required")) as $dev_evidence_acceptance_source_required_count |
  (count_true($entries; "dev_evidence_acceptance_source_present")) as $dev_evidence_acceptance_source_present_count |
  (count_true($entries; "evidence_payload_source_binding_required")) as $evidence_payload_source_binding_required_count |
  (count_true($entries; "evidence_payload_source_binding_present")) as $evidence_payload_source_binding_present_count |
  (count_true($entries; "local_evidence_store_feature_gate_required")) as $local_evidence_store_feature_gate_required_count |
  (count_true($entries; "local_evidence_store_feature_gate_enabled")) as $local_evidence_store_feature_gate_enabled_count |
  (count_true($entries; "local_receipt_store_feature_gate_required")) as $local_receipt_store_feature_gate_required_count |
  (count_true($entries; "local_receipt_store_feature_gate_enabled")) as $local_receipt_store_feature_gate_enabled_count |
  (count_true($entries; "atomic_acceptance_append_required")) as $atomic_acceptance_append_required_count |
  (count_true($entries; "atomic_acceptance_append_enabled")) as $atomic_acceptance_append_enabled_count |
  (count_true($entries; "post_acceptance_readback_required")) as $post_acceptance_readback_required_count |
  (count_true($entries; "post_acceptance_readback_persisted")) as $post_acceptance_readback_persisted_count |
  (count_true($entries; "rollback_anchor_required")) as $rollback_anchor_required_count |
  (count_true($entries; "rollback_anchor_verified")) as $rollback_anchor_verified_count |
  (count_true($entries; "retention_policy_commit_required")) as $retention_policy_commit_required_count |
  (count_true($entries; "retention_policy_committed")) as $retention_policy_committed_count |
  (count_true($entries; "replay_idempotency_guard_required")) as $replay_idempotency_guard_required_count |
  (count_true($entries; "replay_idempotency_guard_enabled")) as $replay_idempotency_guard_enabled_count |
  (count_true($entries; "authority_decision_request_projected")) as $authority_decision_request_projected_count |
  (count_true($entries; "authority_decision_recorded")) as $authority_decision_recorded_count |
  (count_true($entries; "non_authority_receipt_projected")) as $non_authority_receipt_projected_count |
  (count_true($entries; "non_authority_receipt_persisted")) as $non_authority_receipt_persisted_count |
  (count_true($entries; "operator_packet_sent")) as $operator_packet_sent_count |
  (count_true($entries; "operator_packet_persisted")) as $operator_packet_persisted_count |
  (count_true($entries; "local_evidence_acceptance_allowed")) as $local_evidence_acceptance_allowed_count |
  (count_true($entries; "local_evidence_acceptance_recorded")) as $local_evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_acceptance_recorded")) as $evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_recorded")) as $evidence_recorded_count |
  (count_true($entries; "receipt_store_write_attempt_recorded")) as $receipt_store_write_attempt_recorded_count |
  (count_true($entries; "receipt_store_written")) as $receipt_store_written_count |
  (count_true($entries; "receipt_persisted")) as $receipt_persisted_count |
  (count_true($entries; "ledger_written")) as $ledger_written_count |
  (count_true($entries; "workflow_event_log_written")) as $workflow_event_log_written_count |
  (count_true($entries; "sqlite_written")) as $sqlite_written_count |
  (count_true($entries; "live_mutation_allowed")) as $live_mutation_allowed_count |
  ($src.local_evidence_acceptance_positive_preconditions_readback_ready == true
    and $src.precondition_entry_count == 7
    and $src.positive_preconditions_missing_count == 7
    and $src.local_evidence_acceptance_allowed_count == 0
    and $src.local_evidence_acceptance_recorded_count == 0
    and $src.evidence_acceptance_recorded_count == 0
    and $src.evidence_recorded_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $packet_projected_count == 7
    and $packet_ready_count == 7
    and $authority_packet_key_projected_count == 7
    and $authority_packet_key_unique_count == 7
    and $authority_checklist_projected_count == 7
    and $authority_item_required_count == 77
    and $authority_item_present_count == 0
    and $source_positive_preconditions_attached_count == 7
    and $source_terminal_closeout_attached_count == 7
    and $source_persistence_denial_attached_count == 7
    and $source_denial_receipt_attached_count == 7
    and $source_acceptance_source_record_attached_count == 7
    and $local_acceptance_authority_required_count == 7
    and $local_acceptance_authority_present_count == 0
    and $operator_local_acceptance_approval_required_count == 7
    and $operator_local_acceptance_approval_present_count == 0
    and $dev_evidence_acceptance_source_required_count == 7
    and $dev_evidence_acceptance_source_present_count == 0
    and $evidence_payload_source_binding_required_count == 7
    and $evidence_payload_source_binding_present_count == 0
    and $local_evidence_store_feature_gate_required_count == 7
    and $local_evidence_store_feature_gate_enabled_count == 0
    and $local_receipt_store_feature_gate_required_count == 7
    and $local_receipt_store_feature_gate_enabled_count == 0
    and $atomic_acceptance_append_required_count == 7
    and $atomic_acceptance_append_enabled_count == 0
    and $post_acceptance_readback_required_count == 7
    and $post_acceptance_readback_persisted_count == 0
    and $rollback_anchor_required_count == 7
    and $rollback_anchor_verified_count == 0
    and $retention_policy_commit_required_count == 7
    and $retention_policy_committed_count == 0
    and $replay_idempotency_guard_required_count == 7
    and $replay_idempotency_guard_enabled_count == 0
    and $authority_decision_request_projected_count == 7
    and $authority_decision_recorded_count == 0
    and $non_authority_receipt_projected_count == 7
    and $non_authority_receipt_persisted_count == 0
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $local_evidence_acceptance_allowed_count == 0
    and $local_evidence_acceptance_recorded_count == 0
    and $evidence_acceptance_recorded_count == 0
    and $evidence_recorded_count == 0
    and $receipt_store_write_attempt_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0) as $ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_gate",
    schema_version:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_readback_without_acceptance_v1",
    plugin_id:"hepta-system@hepta-local",
    source_positive_preconditions_ready:$src.local_evidence_acceptance_positive_preconditions_readback_ready,
    source_precondition_entry_count:$src.precondition_entry_count,
    source_positive_preconditions_missing_count:$src.positive_preconditions_missing_count,
    source_local_evidence_acceptance_allowed_count:$src.local_evidence_acceptance_allowed_count,
    source_local_evidence_acceptance_recorded_count:$src.local_evidence_acceptance_recorded_count,
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
    authority_packet_id:$packet_id,
    authority_packet_route:$packet_route,
    authority_packet_payload_fingerprint:$packet_fingerprint,
    packet_entry_count:($entries | length),
    packet_projected_count:$packet_projected_count,
    packet_ready_count:$packet_ready_count,
    authority_packet_key_projected_count:$authority_packet_key_projected_count,
    authority_packet_key_unique_count:$authority_packet_key_unique_count,
    authority_checklist_projected_count:$authority_checklist_projected_count,
    authority_item_required_count:$authority_item_required_count,
    authority_item_present_count:$authority_item_present_count,
    source_positive_preconditions_attached_count:$source_positive_preconditions_attached_count,
    source_terminal_closeout_attached_count:$source_terminal_closeout_attached_count,
    source_persistence_denial_attached_count:$source_persistence_denial_attached_count,
    source_denial_receipt_attached_count:$source_denial_receipt_attached_count,
    source_acceptance_source_record_attached_count:$source_acceptance_source_record_attached_count,
    local_acceptance_authority_required_count:$local_acceptance_authority_required_count,
    local_acceptance_authority_present_count:$local_acceptance_authority_present_count,
    operator_local_acceptance_approval_required_count:$operator_local_acceptance_approval_required_count,
    operator_local_acceptance_approval_present_count:$operator_local_acceptance_approval_present_count,
    dev_evidence_acceptance_source_required_count:$dev_evidence_acceptance_source_required_count,
    dev_evidence_acceptance_source_present_count:$dev_evidence_acceptance_source_present_count,
    evidence_payload_source_binding_required_count:$evidence_payload_source_binding_required_count,
    evidence_payload_source_binding_present_count:$evidence_payload_source_binding_present_count,
    local_evidence_store_feature_gate_required_count:$local_evidence_store_feature_gate_required_count,
    local_evidence_store_feature_gate_enabled_count:$local_evidence_store_feature_gate_enabled_count,
    local_receipt_store_feature_gate_required_count:$local_receipt_store_feature_gate_required_count,
    local_receipt_store_feature_gate_enabled_count:$local_receipt_store_feature_gate_enabled_count,
    atomic_acceptance_append_required_count:$atomic_acceptance_append_required_count,
    atomic_acceptance_append_enabled_count:$atomic_acceptance_append_enabled_count,
    post_acceptance_readback_required_count:$post_acceptance_readback_required_count,
    post_acceptance_readback_persisted_count:$post_acceptance_readback_persisted_count,
    rollback_anchor_required_count:$rollback_anchor_required_count,
    rollback_anchor_verified_count:$rollback_anchor_verified_count,
    retention_policy_commit_required_count:$retention_policy_commit_required_count,
    retention_policy_committed_count:$retention_policy_committed_count,
    replay_idempotency_guard_required_count:$replay_idempotency_guard_required_count,
    replay_idempotency_guard_enabled_count:$replay_idempotency_guard_enabled_count,
    authority_decision_request_projected_count:$authority_decision_request_projected_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    non_authority_receipt_projected_count:$non_authority_receipt_projected_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    local_evidence_acceptance_allowed_count:$local_evidence_acceptance_allowed_count,
    local_evidence_acceptance_recorded_count:$local_evidence_acceptance_recorded_count,
    evidence_acceptance_recorded_count:$evidence_acceptance_recorded_count,
    evidence_recorded_count:$evidence_recorded_count,
    receipt_store_write_attempt_recorded_count:$receipt_store_write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    local_evidence_acceptance_authority_packet_readback_ready:$ready,
    operator_packet_send_allowed:false,
    operator_packet_sent:false,
    operator_packet_persistence_allowed:false,
    operator_packet_persisted:false,
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
      "operator_packet_send_disabled",
      "operator_packet_persistence_disabled",
      "local_evidence_acceptance_authority_missing",
      "operator_local_acceptance_approval_missing",
      "dev_evidence_acceptance_source_missing",
      "evidence_payload_source_binding_missing",
      "local_evidence_store_feature_gate_closed",
      "local_receipt_store_feature_gate_closed",
      "atomic_acceptance_append_not_enabled",
      "post_acceptance_readback_missing",
      "rollback_anchor_missing",
      "retention_policy_not_committed",
      "replay_idempotency_guard_disabled",
      "authority_decision_recording_disabled",
      "non_authority_receipt_persistence_disabled",
      "local_evidence_acceptance_disabled",
      "evidence_acceptance_recording_disabled",
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
      "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback",
      "keep_local_evidence_acceptance_authority_packet_unsent_unaccepted_unpersisted"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
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
  }
'
