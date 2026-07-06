#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
GAP_DIFF_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_operator_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-operator-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$GAP_DIFF_REPORT" ]] || fail "missing executable Phase 5g gap diff view report: $GAP_DIFF_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5h Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5h architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5h required evidence gap operator readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_required_evidence_gap_operator_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

gap_diff_json="${HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_JSON:-}"
if [[ -n "$gap_diff_json" ]]; then
  [[ -f "$gap_diff_json" ]] || fail "missing cached Phase 5g gap diff view report: $gap_diff_json"
else
  gap_diff_json="$tmpdir/gap-diff.json"
  "$GAP_DIFF_REPORT" >"$gap_diff_json" || fail "failed to render Phase 5g gap diff view report"
fi

jq -n \
  --slurpfile gap_diff "$gap_diff_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_2026-06-27.md" \
  '
  def operator_readback_key($id):
    "controlled_live.required_evidence.gap.operator_readback." + $id;
  def operator_readback_route($id):
    "readback://controlled-live/required-evidence/gap/operator/" + ($id | gsub("_"; "-"));
  def operator_display_order($id):
    if $id == "dirty_worktree_boundary" then 1
    elif $id == "operator_live_approval_missing" then 2
    elif $id == "fresh_soak_readback_missing" then 3
    elif $id == "credential_boundary_attestation_missing" then 4
    elif $id == "gateway_native_telegram_post_boundary_approval_missing" then 5
    elif $id == "rollback_rehearsal_missing" then 6
    elif $id == "kill_switch_rehearsal_missing" then 7
    else 0
    end;
  ($gap_diff[0]) as $gap_diff |
  ($gap_diff.entries | map({
    id,
    source_blocker_id,
    operator_readback_key:operator_readback_key(.source_blocker_id),
    operator_readback_route:operator_readback_route(.source_blocker_id),
    operator_display_order:operator_display_order(.source_blocker_id),
    operator_status:"blocked_missing_evidence",
    operator_action:"collect_required_evidence_before_live_cutover",
    gap_key,
    diff_view_key,
    comparison_anchor,
    owner,
    risk_bucket,
    previous_state,
    current_state,
    state_delta,
    cutover_risk,
    query_key,
    readback_route,
    diff_key,
    fingerprint,
    operator_label,
    required_evidence,
    operator_visible:true,
    queryable:true,
    comparable:true,
    evidence_recorded:false,
    evidence_recording_allowed:false,
    credential_read_allowed:false,
    approval_acceptance_allowed:false,
    blocker_waiver_allowed:false,
    persistence_allowed:false,
    readback_persistence_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.operator_visible == true)) | length) as $operator_visible_entry_count |
  ($entries | map(select((.operator_readback_key | length) > 0 and (.operator_readback_route | length) > 0 and .operator_display_order > 0)) | length) as $stable_readback_key_count |
  ($entries | map(select(.previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing")) | length) as $unchanged_missing_count |
  ($entries | map(.owner) | unique | length) as $owner_count |
  ($entries | map(.risk_bucket) | unique | length) as $risk_bucket_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.blocker_waiver_allowed == true)) | length) as $blocker_waived_count |
  ($gap_diff.diff_view_ready == true
    and $gap_diff.diff_entry_count == 7
    and $gap_diff.unchanged_missing_count == 7
    and ($entries | length) == 7
    and $operator_visible_entry_count == 7
    and $stable_readback_key_count == 7
    and $unchanged_missing_count == 7
    and $owner_count == 7
    and $risk_bucket_count == 3
    and $evidence_recorded_count == 0
    and $blocker_waived_count == 0
    and $lib_export_present == true
    and ($entries | all(.operator_visible == true
      and .queryable == true
      and .comparable == true
      and .operator_status == "blocked_missing_evidence"
      and .operator_action == "collect_required_evidence_before_live_cutover"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .approval_acceptance_allowed == false
      and .persistence_allowed == false
      and .readback_persistence_allowed == false
      and .live_mutation_allowed == false))) as $operator_readback_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_required_evidence_gap_operator_readback",
    status:(if $operator_readback_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_required_evidence_gap_operator_readback_gate",
    schema_version:"controlled_live_required_evidence_gap_operator_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    source_diff_view_ready:$gap_diff.diff_view_ready,
    source_diff_entry_count:$gap_diff.diff_entry_count,
    source_unchanged_missing_count:$gap_diff.unchanged_missing_count,
    lib_export_present:$lib_export_present,
    operator_readback_entry_count:($entries | length),
    operator_visible_entry_count:$operator_visible_entry_count,
    stable_readback_key_count:$stable_readback_key_count,
    unchanged_missing_count:$unchanged_missing_count,
    owner_count:$owner_count,
    risk_bucket_count:$risk_bucket_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_readback_ready:$operator_readback_ready,
    approval_acceptance_ready:false,
    approval_accepted:false,
    blocker_waived_count:$blocker_waived_count,
    credential_read_allowed:false,
    evidence_recording_allowed:false,
    evidence_persisted:false,
    readback_persisted:false,
    controlled_live_cutover_ready:false,
    live_execution_allowed:false,
    entries:$entries,
    next_actions:[
      "phase5i_controlled_live_required_evidence_gap_operator_packet_attachment_without_acceptance",
      "keep_operator_readback_visible_without_acceptance"
    ],
    next_migration_step:"phase5i_controlled_live_required_evidence_gap_operator_packet_attachment_without_acceptance",
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
      blocker_waived:false,
      credential_read:false,
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
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
