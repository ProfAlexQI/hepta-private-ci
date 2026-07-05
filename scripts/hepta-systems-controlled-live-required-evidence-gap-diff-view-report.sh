#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
GAP_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_diff_view.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-diff-view-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$GAP_SUMMARY_REPORT" ]] || fail "missing executable Phase 5f gap summary report: $GAP_SUMMARY_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5g Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5g architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5g required evidence gap diff view report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_required_evidence_gap_diff_view_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

gap_summary_json="${HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_JSON:-}"
if [[ -n "$gap_summary_json" ]]; then
  [[ -f "$gap_summary_json" ]] || fail "missing cached Phase 5f gap summary report: $gap_summary_json"
else
  gap_summary_json="$tmpdir/gap-summary.json"
  "$GAP_SUMMARY_REPORT" >"$gap_summary_json" || fail "failed to render Phase 5f gap summary report"
fi

jq -n \
  --slurpfile gap_summary "$gap_summary_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_2026-06-27.md" \
  '
  def diff_view_key($id):
    "controlled_live.required_evidence.gap.diff_view." + $id;
  def comparison_anchor($id):
    "gap-summary-owner-risk:" + ($id | gsub("_"; "-"));
  ($gap_summary[0]) as $gap_summary |
  ($gap_summary.entries | map({
    id,
    source_blocker_id,
    gap_key,
    diff_view_key:diff_view_key(.source_blocker_id),
    comparison_anchor:comparison_anchor(.source_blocker_id),
    owner,
    risk_bucket,
    previous_state:"missing",
    current_state:.evidence_state,
    state_delta:"unchanged_missing",
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
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select((.diff_view_key | length) > 0 and (.comparison_anchor | length) > 0)) | length) as $stable_diff_key_count |
  ($entries | map(select(.comparable == true)) | length) as $comparable_entry_count |
  ($entries | map(select(.previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing")) | length) as $unchanged_missing_count |
  ($entries | map(.owner) | unique | length) as $owner_count |
  ($entries | map(.risk_bucket) | unique | length) as $risk_bucket_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.blocker_waiver_allowed == true)) | length) as $blocker_waived_count |
  ($gap_summary.gap_summary_ready == true
    and $gap_summary.gap_entry_count == 7
    and $gap_summary.missing_evidence_count == 7
    and ($entries | length) == 7
    and $stable_diff_key_count == 7
    and $comparable_entry_count == 7
    and $unchanged_missing_count == 7
    and $owner_count == 7
    and $risk_bucket_count == 3
    and $evidence_recorded_count == 0
    and $blocker_waived_count == 0
    and $lib_export_present == true
    and ($entries | all(.operator_visible == true
      and .queryable == true
      and .comparable == true
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .approval_acceptance_allowed == false
      and .persistence_allowed == false
      and .live_mutation_allowed == false))) as $diff_view_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_required_evidence_gap_diff_view",
    status:(if $diff_view_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_required_evidence_gap_diff_view_gate",
    schema_version:"controlled_live_required_evidence_gap_diff_view_v1",
    plugin_id:"hepta-system@hepta-local",
    source_gap_summary_ready:$gap_summary.gap_summary_ready,
    source_gap_entry_count:$gap_summary.gap_entry_count,
    source_missing_evidence_count:$gap_summary.missing_evidence_count,
    lib_export_present:$lib_export_present,
    diff_entry_count:($entries | length),
    stable_diff_key_count:$stable_diff_key_count,
    comparable_entry_count:$comparable_entry_count,
    unchanged_missing_count:$unchanged_missing_count,
    owner_count:$owner_count,
    risk_bucket_count:$risk_bucket_count,
    evidence_recorded_count:$evidence_recorded_count,
    diff_view_ready:$diff_view_ready,
    approval_acceptance_ready:false,
    approval_accepted:false,
    blocker_waived_count:$blocker_waived_count,
    credential_read_allowed:false,
    evidence_recording_allowed:false,
    evidence_persisted:false,
    controlled_live_cutover_ready:false,
    live_execution_allowed:false,
    entries:$entries,
    next_actions:[
      "phase5h_controlled_live_required_evidence_gap_operator_readback_without_acceptance",
      "keep_gap_diff_view_operator_facing_without_acceptance"
    ],
    next_migration_step:"phase5h_controlled_live_required_evidence_gap_operator_readback_without_acceptance",
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
