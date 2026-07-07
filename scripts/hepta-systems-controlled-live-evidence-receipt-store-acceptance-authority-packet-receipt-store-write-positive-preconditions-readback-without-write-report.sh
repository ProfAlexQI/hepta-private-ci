#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_WRITE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-positive-preconditions-readback-without-write-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable write-denial retention/replay report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing write-positive-preconditions Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the write-positive-preconditions report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_JSON:-}"
source_cache_input_present=false
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached write-denial retention/replay report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/write-denial-retention-replay.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render write-denial retention/replay report"
fi
jq -e . "$source_json" >/dev/null || fail "write-denial retention/replay report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --arg gate "scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-positive-preconditions-readback-without-write-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_POSITIVE_PRECONDITIONS_READBACK_WITHOUT_WRITE_2026-07-07.md" \
  --arg precondition_route "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-positive-preconditions" \
  '
  def hyphen_id($id):
    $id | gsub("_"; "-");
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_without_write_" + .source_blocker_id),
    source_blocker_id,
    source_retention_replay_entry_id:.id,
    source_receipt_store_write_denial_id,
    source_receipt_store_write_denial_route,
    source_retention_policy_id:.retention_policy_id,
    source_replay_idempotency_key:.replay_idempotency_key,
    source_zero_effect_digest:.zero_effect_digest,
    write_precondition_set_id:("receipt-store-write-positive-preconditions:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    write_precondition_route:($precondition_route + "/" + hyphen_id(.source_blocker_id)),
    acceptance_authority_precondition_id:("acceptance-authority-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    operator_write_approval_precondition_id:("operator-write-approval-required:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    evidence_acceptance_precondition_id:("evidence-acceptance-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    receipt_store_write_grant_precondition_id:("receipt-store-write-grant-required:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    write_attempt_recording_precondition_id:("write-attempt-recording-required:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    atomic_append_precondition_id:("atomic-append-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    post_write_readback_precondition_id:("post-write-readback-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    rollback_anchor_precondition_id:("rollback-anchor-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    retention_commit_precondition_id:("retention-policy-commit-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    replay_idempotency_guard_precondition_id:("replay-idempotency-guard-required:controlled-live-evidence-receipt-store-write:" + .source_blocker_id),
    operator_display_order,
    operator_status,
    observed_state:"receipt_store_write_positive_preconditions_projected_without_write",
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    source_packet_unsent,
    source_write_denial_attached,
    source_retention_replay_attached:true,
    write_precondition_set_projected:true,
    acceptance_authority_required:true,
    acceptance_authority_present:false,
    operator_write_approval_required:true,
    operator_write_approval_present:false,
    evidence_acceptance_required:true,
    evidence_acceptance_present:false,
    receipt_store_write_grant_required:true,
    receipt_store_write_grant_present:false,
    write_attempt_recording_required:true,
    write_attempt_recording_enabled:false,
    atomic_append_required:true,
    atomic_append_enabled:false,
    post_write_readback_required:true,
    post_write_readback_persisted:false,
    rollback_anchor_required:true,
    rollback_anchor_verified:false,
    retention_policy_commit_required:true,
    retention_policy_committed:false,
    replay_idempotency_guard_required:true,
    replay_idempotency_guard_enabled:false,
    write_preconditions_missing:true,
    write_attempt_recording_allowed:false,
    write_attempt_recorded:false,
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
  ($entries | map(select(.write_precondition_set_projected == true)) | length) as $write_precondition_set_projected_count |
  ($entries | map(select(.source_retention_replay_attached == true)) | length) as $source_retention_replay_attached_count |
  ($entries | map(select(.acceptance_authority_required == true)) | length) as $acceptance_authority_required_count |
  ($entries | map(select(.acceptance_authority_present == true)) | length) as $acceptance_authority_present_count |
  ($entries | map(select(.operator_write_approval_required == true)) | length) as $operator_write_approval_required_count |
  ($entries | map(select(.operator_write_approval_present == true)) | length) as $operator_write_approval_present_count |
  ($entries | map(select(.evidence_acceptance_required == true)) | length) as $evidence_acceptance_required_count |
  ($entries | map(select(.evidence_acceptance_present == true)) | length) as $evidence_acceptance_present_count |
  ($entries | map(select(.receipt_store_write_grant_required == true)) | length) as $receipt_store_write_grant_required_count |
  ($entries | map(select(.receipt_store_write_grant_present == true)) | length) as $receipt_store_write_grant_present_count |
  ($entries | map(select(.write_attempt_recording_required == true)) | length) as $write_attempt_recording_required_count |
  ($entries | map(select(.write_attempt_recording_enabled == true)) | length) as $write_attempt_recording_enabled_count |
  ($entries | map(select(.atomic_append_required == true)) | length) as $atomic_append_required_count |
  ($entries | map(select(.atomic_append_enabled == true)) | length) as $atomic_append_enabled_count |
  ($entries | map(select(.post_write_readback_required == true)) | length) as $post_write_readback_required_count |
  ($entries | map(select(.post_write_readback_persisted == true)) | length) as $post_write_readback_persisted_count |
  ($entries | map(select(.rollback_anchor_required == true)) | length) as $rollback_anchor_required_count |
  ($entries | map(select(.rollback_anchor_verified == true)) | length) as $rollback_anchor_verified_count |
  ($entries | map(select(.retention_policy_commit_required == true)) | length) as $retention_policy_commit_required_count |
  ($entries | map(select(.retention_policy_committed == true)) | length) as $retention_policy_committed_count |
  ($entries | map(select(.replay_idempotency_guard_required == true)) | length) as $replay_idempotency_guard_required_count |
  ($entries | map(select(.replay_idempotency_guard_enabled == true)) | length) as $replay_idempotency_guard_enabled_count |
  ($entries | map(select(.write_preconditions_missing == true)) | length) as $write_preconditions_missing_count |
  ($entries | map(select(.receipt_store_write_allowed == true)) | length) as $receipt_store_write_allowed_count |
  ($entries | map(select(.write_attempt_recorded == true)) | length) as $write_attempt_recorded_count |
  ($entries | map(select(.receipt_store_written == true)) | length) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.workflow_event_log_written == true)) | length) as $workflow_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_mutation_allowed == true)) | length) as $live_mutation_allowed_count |
  ($src.write_denial_retention_replay_readback_ready == true
    and $src.retention_replay_entry_count == 7
    and $src.source_write_denial_attached_count == 7
    and $src.retention_policy_persisted_count == 0
    and $src.replay_index_written_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $write_precondition_set_projected_count == 7
    and $source_retention_replay_attached_count == 7
    and $acceptance_authority_required_count == 7
    and $acceptance_authority_present_count == 0
    and $operator_write_approval_required_count == 7
    and $operator_write_approval_present_count == 0
    and $evidence_acceptance_required_count == 7
    and $evidence_acceptance_present_count == 0
    and $receipt_store_write_grant_required_count == 7
    and $receipt_store_write_grant_present_count == 0
    and $write_attempt_recording_required_count == 7
    and $write_attempt_recording_enabled_count == 0
    and $atomic_append_required_count == 7
    and $atomic_append_enabled_count == 0
    and $post_write_readback_required_count == 7
    and $post_write_readback_persisted_count == 0
    and $rollback_anchor_required_count == 7
    and $rollback_anchor_verified_count == 0
    and $retention_policy_commit_required_count == 7
    and $retention_policy_committed_count == 0
    and $replay_idempotency_guard_required_count == 7
    and $replay_idempotency_guard_enabled_count == 0
    and $write_preconditions_missing_count == 7
    and $receipt_store_write_allowed_count == 0
    and $write_attempt_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "receipt_store_write_positive_preconditions_projected_without_write"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .source_packet_unsent == true
      and .source_write_denial_attached == true
      and .source_retention_replay_attached == true
      and .write_precondition_set_projected == true
      and .acceptance_authority_required == true
      and .acceptance_authority_present == false
      and .operator_write_approval_required == true
      and .operator_write_approval_present == false
      and .evidence_acceptance_required == true
      and .evidence_acceptance_present == false
      and .receipt_store_write_grant_required == true
      and .receipt_store_write_grant_present == false
      and .write_attempt_recording_required == true
      and .write_attempt_recording_enabled == false
      and .atomic_append_required == true
      and .atomic_append_enabled == false
      and .post_write_readback_required == true
      and .post_write_readback_persisted == false
      and .rollback_anchor_required == true
      and .rollback_anchor_verified == false
      and .retention_policy_commit_required == true
      and .retention_policy_committed == false
      and .replay_idempotency_guard_required == true
      and .replay_idempotency_guard_enabled == false
      and .write_preconditions_missing == true
      and .write_attempt_recording_allowed == false
      and .write_attempt_recorded == false
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
      and .live_mutation_allowed == false))) as $ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_gate",
    schema_version:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write_v1",
    plugin_id:"hepta-system@hepta-local",
    source_retention_replay_ready:$src.write_denial_retention_replay_readback_ready,
    source_retention_replay_entry_count:$src.retention_replay_entry_count,
    source_write_denial_attached_count:$src.source_write_denial_attached_count,
    source_retention_policy_persisted_count:$src.retention_policy_persisted_count,
    source_replay_index_written_count:$src.replay_index_written_count,
    source_receipt_store_write_attempt_recorded_count:$src.receipt_store_write_attempt_recorded_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_receipt_persisted_count:$src.receipt_persisted_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    source_cache_mode:(if $source_cache_input_present then "provided_source_json" else "rendered_once_temp_source_json" end),
    source_cache_input_present:$source_cache_input_present,
    source_report_render_count:(if $source_cache_input_present then 0 else 1 end),
    target_source_reuse_count:1,
    lib_export_present:$lib_export_present,
    write_positive_preconditions_route:$precondition_route,
    precondition_entry_count:($entries | length),
    write_precondition_set_projected_count:$write_precondition_set_projected_count,
    source_retention_replay_attached_count:$source_retention_replay_attached_count,
    acceptance_authority_required_count:$acceptance_authority_required_count,
    acceptance_authority_present_count:$acceptance_authority_present_count,
    operator_write_approval_required_count:$operator_write_approval_required_count,
    operator_write_approval_present_count:$operator_write_approval_present_count,
    evidence_acceptance_required_count:$evidence_acceptance_required_count,
    evidence_acceptance_present_count:$evidence_acceptance_present_count,
    receipt_store_write_grant_required_count:$receipt_store_write_grant_required_count,
    receipt_store_write_grant_present_count:$receipt_store_write_grant_present_count,
    write_attempt_recording_required_count:$write_attempt_recording_required_count,
    write_attempt_recording_enabled_count:$write_attempt_recording_enabled_count,
    atomic_append_required_count:$atomic_append_required_count,
    atomic_append_enabled_count:$atomic_append_enabled_count,
    post_write_readback_required_count:$post_write_readback_required_count,
    post_write_readback_persisted_count:$post_write_readback_persisted_count,
    rollback_anchor_required_count:$rollback_anchor_required_count,
    rollback_anchor_verified_count:$rollback_anchor_verified_count,
    retention_policy_commit_required_count:$retention_policy_commit_required_count,
    retention_policy_committed_count:$retention_policy_committed_count,
    replay_idempotency_guard_required_count:$replay_idempotency_guard_required_count,
    replay_idempotency_guard_enabled_count:$replay_idempotency_guard_enabled_count,
    write_preconditions_missing_count:$write_preconditions_missing_count,
    receipt_store_write_allowed_count:$receipt_store_write_allowed_count,
    write_attempt_recorded_count:$write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    write_positive_preconditions_readback_ready:$ready,
    write_attempt_recording_allowed:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    receipt_persistence_allowed:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "acceptance_authority_missing",
      "operator_write_approval_missing",
      "evidence_acceptance_missing",
      "receipt_store_write_grant_missing",
      "write_attempt_recording_disabled",
      "atomic_append_not_enabled",
      "post_write_readback_missing",
      "rollback_anchor_missing",
      "retention_policy_not_committed",
      "replay_idempotency_guard_disabled",
      "receipt_store_write_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording",
      "keep_receipt_store_write_closed_until_all_write_preconditions_are_present"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_boundary_readback_without_recording",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      write_attempt_recorded:false,
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
