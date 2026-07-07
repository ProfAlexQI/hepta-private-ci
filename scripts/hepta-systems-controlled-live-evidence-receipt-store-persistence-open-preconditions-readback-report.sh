#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SHADOW_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-shadow-write-rehearsal-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_persistence_open_preconditions_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-persistence-open-preconditions-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SHADOW_REPORT" ]] || fail "missing executable receipt store shadow write rehearsal report: $SHADOW_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing controlled-live evidence receipt store persistence open-preconditions Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the controlled-live evidence receipt store persistence open-preconditions report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

shadow_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_SHADOW_JSON:-}"
if [[ -n "$shadow_json" ]]; then
  [[ -f "$shadow_json" ]] || fail "missing cached receipt store shadow write rehearsal report: $shadow_json"
else
  shadow_json="$tmpdir/shadow.json"
  "$SHADOW_REPORT" >"$shadow_json" || fail "failed to render receipt store shadow write rehearsal report"
fi

jq -n \
  --slurpfile shadow "$shadow_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-evidence-receipt-store-persistence-open-preconditions-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_2026-07-07.md" \
  '
  def hyphen_id($id):
    $id | gsub("_"; "-");
  def precondition_key($id):
    "controlled_live.evidence_receipt_store.persistence_open_preconditions." + $id;
  def precondition_route($id):
    "readback://controlled-live/evidence-receipt-store/persistence-open-preconditions/" + hyphen_id($id);
  def operator_approval_id($id):
    "operator-approval:controlled-live-evidence-receipt-store:" + $id;
  def evidence_acceptance_key($id):
    "controlled_live.evidence_acceptance.required." + $id;
  def store_path_write_grant_key($id):
    "controlled_live.receipt_store.write_grant.required." + $id;
  def atomic_append_plan_id($id):
    "atomic-append-plan:controlled-live-evidence-receipt-store:" + $id;
  def post_write_readback_route($id):
    "readback://controlled-live/evidence-receipt-store/post-write/" + hyphen_id($id);
  def rollback_rehearsal_route($id):
    "readback://controlled-live/evidence-receipt-store/rollback-rehearsal/" + hyphen_id($id);
  def retention_policy_id($id):
    "retention-policy:controlled-live-evidence-receipt-store:" + $id;
  ($shadow[0]) as $shadow |
  ($shadow.entries | map({
    id:("evidence_receipt_store_persistence_open_preconditions_" + .source_blocker_id),
    source_blocker_id,
    receipt_path,
    receipt_id,
    idempotency_key,
    shadow_write_route,
    persistence_precondition_key:precondition_key(.source_blocker_id),
    persistence_precondition_route:precondition_route(.source_blocker_id),
    operator_approval_id:operator_approval_id(.source_blocker_id),
    evidence_acceptance_key:evidence_acceptance_key(.source_blocker_id),
    store_path_write_grant_key:store_path_write_grant_key(.source_blocker_id),
    atomic_append_plan_id:atomic_append_plan_id(.source_blocker_id),
    post_write_readback_route:post_write_readback_route(.source_blocker_id),
    rollback_rehearsal_route:rollback_rehearsal_route(.source_blocker_id),
    retention_policy_id:retention_policy_id(.source_blocker_id),
    operator_display_order,
    operator_status,
    observed_state:"persistence_open_preconditions_listed_no_persistence",
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    precondition_state:"required_missing",
    shadow_rehearsal_confirmed:true,
    operator_approval_required:true,
    operator_approval_present:false,
    evidence_acceptance_required:true,
    evidence_acceptance_present:false,
    store_path_write_grant_required:true,
    store_path_write_grant_present:false,
    atomic_append_required:true,
    atomic_append_enabled:false,
    post_write_readback_required:true,
    post_write_readback_persisted:false,
    rollback_rehearsal_required:true,
    rollback_rehearsal_verified:false,
    retention_policy_required:true,
    retention_policy_committed:false,
    persistence_denied:true,
    ledger_denied:true,
    workflow_event_log_denied:true,
    sqlite_denied:true,
    live_denied:true,
    approval_request_allowed:false,
    approval_acceptance_allowed:false,
    evidence_recording_allowed:false,
    evidence_recorded:false,
    blocker_waiver_allowed:false,
    receipt_persistence_allowed:false,
    receipt_persisted:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.shadow_rehearsal_confirmed == true
    and .operator_approval_required == true
    and .evidence_acceptance_required == true
    and .store_path_write_grant_required == true
    and .atomic_append_required == true
    and .post_write_readback_required == true
    and .rollback_rehearsal_required == true
    and .retention_policy_required == true
    and .persistence_denied == true
    and .ledger_denied == true
    and .workflow_event_log_denied == true
    and .sqlite_denied == true
    and .live_denied == true
    and .operator_approval_present == false
    and .evidence_acceptance_present == false
    and .store_path_write_grant_present == false
    and .atomic_append_enabled == false
    and .post_write_readback_persisted == false
    and .rollback_rehearsal_verified == false
    and .retention_policy_committed == false
    and .receipt_store_write_allowed == false
    and .receipt_store_written == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .live_mutation_allowed == false)) | length) as $precondition_catalog_ready_count |
  ($entries | map(select(.operator_approval_required == true)) | length) as $operator_approval_required_count |
  ($entries | map(select(.operator_approval_present == true)) | length) as $operator_approval_present_count |
  ($entries | map(select(.evidence_acceptance_required == true)) | length) as $evidence_acceptance_required_count |
  ($entries | map(select(.evidence_acceptance_present == true)) | length) as $evidence_acceptance_present_count |
  ($entries | map(select(.store_path_write_grant_required == true)) | length) as $store_path_write_grant_required_count |
  ($entries | map(select(.store_path_write_grant_present == true)) | length) as $store_path_write_grant_present_count |
  ($entries | map(select(.atomic_append_required == true)) | length) as $atomic_append_required_count |
  ($entries | map(select(.atomic_append_enabled == true)) | length) as $atomic_append_enabled_count |
  ($entries | map(select(.post_write_readback_required == true)) | length) as $post_write_readback_required_count |
  ($entries | map(select(.post_write_readback_persisted == true)) | length) as $post_write_readback_persisted_count |
  ($entries | map(select(.rollback_rehearsal_required == true)) | length) as $rollback_rehearsal_required_count |
  ($entries | map(select(.rollback_rehearsal_verified == true)) | length) as $rollback_rehearsal_verified_count |
  ($entries | map(select(.retention_policy_required == true)) | length) as $retention_policy_required_count |
  ($entries | map(select(.retention_policy_committed == true)) | length) as $retention_policy_committed_count |
  ($entries | map(select(.persistence_denied == true)) | length) as $persistence_denial_confirmed_count |
  ($entries | map(select(.ledger_denied == true)) | length) as $ledger_denial_confirmed_count |
  ($entries | map(select(.workflow_event_log_denied == true)) | length) as $workflow_event_log_denial_confirmed_count |
  ($entries | map(select(.sqlite_denied == true)) | length) as $sqlite_denial_confirmed_count |
  ($entries | map(select(.live_denied == true)) | length) as $live_denial_confirmed_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.blocker_waiver_allowed == true)) | length) as $blocker_waived_count |
  ($shadow.in_memory_shadow_write_rehearsal_ready == true
    and $shadow.shadow_write_rehearsal_entry_count == 7
    and $shadow.shadow_write_rehearsal_ready_count == 7
    and $shadow.in_memory_shadow_receipt_rendered_count == 7
    and $shadow.receipt_store_write_allowed == false
    and $shadow.receipt_store_written == false
    and $shadow.receipt_persistence_allowed == false
    and $shadow.receipt_persisted == false
    and $shadow.ledger_write_allowed == false
    and $shadow.workflow_event_log_write_allowed == false
    and $shadow.sqlite_write_allowed == false
    and $shadow.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $precondition_catalog_ready_count == 7
    and $operator_approval_required_count == 7
    and $operator_approval_present_count == 0
    and $evidence_acceptance_required_count == 7
    and $evidence_acceptance_present_count == 0
    and $store_path_write_grant_required_count == 7
    and $store_path_write_grant_present_count == 0
    and $atomic_append_required_count == 7
    and $atomic_append_enabled_count == 0
    and $post_write_readback_required_count == 7
    and $post_write_readback_persisted_count == 0
    and $rollback_rehearsal_required_count == 7
    and $rollback_rehearsal_verified_count == 0
    and $retention_policy_required_count == 7
    and $retention_policy_committed_count == 0
    and $persistence_denial_confirmed_count == 7
    and $ledger_denial_confirmed_count == 7
    and $workflow_event_log_denial_confirmed_count == 7
    and $sqlite_denial_confirmed_count == 7
    and $live_denial_confirmed_count == 7
    and $evidence_recorded_count == 0
    and $blocker_waived_count == 0) as $persistence_open_preconditions_readback_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_persistence_open_preconditions_readback",
    status:(if $persistence_open_preconditions_readback_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_gate",
    schema_version:"controlled_live_evidence_receipt_store_persistence_open_preconditions_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    source_shadow_write_rehearsal_ready:$shadow.in_memory_shadow_write_rehearsal_ready,
    source_shadow_write_rehearsal_entry_count:$shadow.shadow_write_rehearsal_entry_count,
    source_in_memory_shadow_receipt_rendered_count:$shadow.in_memory_shadow_receipt_rendered_count,
    source_receipt_store_written:$shadow.receipt_store_written,
    source_receipt_persisted:$shadow.receipt_persisted,
    lib_export_present:$lib_export_present,
    precondition_entry_count:($entries | length),
    precondition_catalog_ready_count:$precondition_catalog_ready_count,
    operator_approval_required_count:$operator_approval_required_count,
    operator_approval_present_count:$operator_approval_present_count,
    evidence_acceptance_required_count:$evidence_acceptance_required_count,
    evidence_acceptance_present_count:$evidence_acceptance_present_count,
    store_path_write_grant_required_count:$store_path_write_grant_required_count,
    store_path_write_grant_present_count:$store_path_write_grant_present_count,
    atomic_append_required_count:$atomic_append_required_count,
    atomic_append_enabled_count:$atomic_append_enabled_count,
    post_write_readback_required_count:$post_write_readback_required_count,
    post_write_readback_persisted_count:$post_write_readback_persisted_count,
    rollback_rehearsal_required_count:$rollback_rehearsal_required_count,
    rollback_rehearsal_verified_count:$rollback_rehearsal_verified_count,
    retention_policy_required_count:$retention_policy_required_count,
    retention_policy_committed_count:$retention_policy_committed_count,
    persistence_denial_confirmed_count:$persistence_denial_confirmed_count,
    ledger_denial_confirmed_count:$ledger_denial_confirmed_count,
    workflow_event_log_denial_confirmed_count:$workflow_event_log_denial_confirmed_count,
    sqlite_denial_confirmed_count:$sqlite_denial_confirmed_count,
    live_denial_confirmed_count:$live_denial_confirmed_count,
    evidence_recorded_count:$evidence_recorded_count,
    blocker_waived_count:$blocker_waived_count,
    persistence_open_preconditions_readback_ready:$persistence_open_preconditions_readback_ready,
    persistence_open_allowed:false,
    approval_request_allowed:false,
    approval_acceptance_allowed:false,
    evidence_recording_allowed:false,
    evidence_persisted:false,
    receipt_persistence_allowed:false,
    receipt_persisted:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "operator_approval_missing",
      "evidence_acceptance_missing",
      "store_path_write_grant_missing",
      "atomic_append_not_enabled",
      "post_write_readback_missing",
      "rollback_rehearsal_missing",
      "retention_policy_not_committed",
      "receipt_persistence_disabled",
      "ledger_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance",
      "keep_persistence_disabled_until_all_open_preconditions_are_present"
    ],
    next_migration_step:"controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      evidence_recorded:false,
      evidence_persisted:false,
      receipt_persisted:false,
      receipt_store_written:false,
      blocker_waived:false,
      credential_read:false,
      packet_sent:false,
      attachment_sent:false,
      packet_persisted:false,
      attachment_persisted:false,
      readback_persisted:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
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
