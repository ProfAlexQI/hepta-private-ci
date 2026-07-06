#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
EVIDENCE_PLAN_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_required_evidence_readback_index.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-readback-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$EVIDENCE_PLAN_REPORT" ]] || fail "missing executable Phase 5d evidence collection plan report: $EVIDENCE_PLAN_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5e Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5e architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5e required evidence readback index report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_required_evidence_readback_index_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

plan_json="${HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_JSON:-}"
if [[ -n "$plan_json" ]]; then
  [[ -f "$plan_json" ]] || fail "missing cached Phase 5d evidence collection plan report: $plan_json"
else
  plan_json="$tmpdir/plan.json"
  "$EVIDENCE_PLAN_REPORT" >"$plan_json" || fail "failed to render Phase 5d evidence collection plan report"
fi

jq -n \
  --slurpfile plan "$plan_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-required-evidence-readback-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_2026-06-27.md" \
  '
  def query_key($id):
    "controlled_live.required_evidence." + $id;
  def readback_route($id):
    "controlled_live_required_evidence.readback." + $id;
  def diff_key($id):
    "controlled_live.required_evidence.diff." + $id;
  def fingerprint($id):
    "required-evidence:fingerprint:" + ($id | gsub("_"; "-"));
  ($plan[0]) as $plan |
  ($plan.entries | map({
    id,
    source_blocker_id,
    layer,
    query_key:query_key(.source_blocker_id),
    readback_route:readback_route(.source_blocker_id),
    diff_key:diff_key(.source_blocker_id),
    fingerprint:fingerprint(.source_blocker_id),
    operator_label,
    required_evidence,
    evidence_state:.current_state,
    queryable:true,
    operator_visible:true,
    diffable:true,
    evidence_recorded:false,
    evidence_recording_allowed:false,
    credential_read_allowed:false,
    approval_acceptance_allowed:false,
    blocker_waiver_allowed:false,
    persistence_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.queryable == true)) | length) as $queryable_entry_count |
  ($entries | map(select(.operator_visible == true)) | length) as $operator_visible_entry_count |
  ($entries | map(select(.diffable == true)) | length) as $diffable_entry_count |
  ($entries | map(select((.fingerprint | length) > 0)) | length) as $fingerprint_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.blocker_waiver_allowed == true)) | length) as $blocker_waived_count |
  ($plan.evidence_collection_plan_ready == true
    and $plan.plan_entry_count == 7
    and ($entries | length) == 7
    and $queryable_entry_count == 7
    and $operator_visible_entry_count == 7
    and $diffable_entry_count == 7
    and $fingerprint_count == 7
    and $evidence_recorded_count == 0
    and $blocker_waived_count == 0
    and $lib_export_present == true
    and ($entries | all(.evidence_state == "missing"
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .approval_acceptance_allowed == false
      and .persistence_allowed == false
      and .live_mutation_allowed == false))) as $index_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_required_evidence_readback_index",
    status:(if $index_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_required_evidence_readback_index_gate",
    schema_version:"controlled_live_required_evidence_readback_index_v1",
    plugin_id:"hepta-system@hepta-local",
    source_evidence_collection_plan_ready:$plan.evidence_collection_plan_ready,
    source_plan_entry_count:$plan.plan_entry_count,
    lib_export_present:$lib_export_present,
    index_entry_count:($entries | length),
    queryable_entry_count:$queryable_entry_count,
    operator_visible_entry_count:$operator_visible_entry_count,
    diffable_entry_count:$diffable_entry_count,
    fingerprint_count:$fingerprint_count,
    evidence_recorded_count:$evidence_recorded_count,
    readback_index_ready:$index_ready,
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
      "phase5f_controlled_live_required_evidence_gap_summary_without_acceptance",
      "keep_required_evidence_readback_index_queryable_without_recording"
    ],
    next_migration_step:"phase5f_controlled_live_required_evidence_gap_summary_without_acceptance",
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
