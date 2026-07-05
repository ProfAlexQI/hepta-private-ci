#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_INDEX_REPORT="$ROOT/scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_operator_packet_preview.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-operator-packet-preview-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_INDEX_REPORT" ]] || fail "missing executable Phase 5a readback index report: $READBACK_INDEX_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5b Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5b architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5b operator packet preview report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_operator_packet_preview_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

readback_json="${HEPTA_CONTROLLED_LIVE_DENIAL_READBACK_JSON:-}"
if [[ -n "$readback_json" ]]; then
  [[ -f "$readback_json" ]] || fail "missing cached Phase 5a readback index report: $readback_json"
else
  readback_json="$tmpdir/readback.json"
  "$READBACK_INDEX_REPORT" >"$readback_json" || fail "failed to render Phase 5a readback index report"
fi

jq -n \
  --slurpfile readback "$readback_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-operator-packet-preview-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_OPERATOR_PACKET_PREVIEW_2026-06-27.md" \
  '
  def section($id; $title; $source): {
    id:$id,
    title:$title,
    source:$source,
    preview_ready:true,
    mutation_enabled:false
  };
  ($readback[0]) as $readback |
  [
    section("scope"; "Scope"; "controlled-live readiness audit"),
    section("payload_hash"; "Payload Hash"; "operator packet preview"),
    section("rollback_owner"; "Rollback Owner"; "operator packet preview"),
    section("blocker_readbacks"; "Blocker Readbacks"; "controlled-live denial readback index"),
    section("required_evidence"; "Required Evidence"; "controlled-live denial readback index"),
    section("closed_boundary"; "Closed Boundary"; "local no-op preview")
  ] as $sections |
  ($readback.entries | map({
    source_blocker_id,
    query_key,
    readback_route,
    operator_label,
    required_evidence,
    current_state,
    included_in_packet:true,
    acceptance_allowed:false,
    waiver_allowed:false,
    live_mutation_allowed:false
  })) as $blocker_readbacks |
  ($blocker_readbacks | map(select((.required_evidence | length) > 0)) | length) as $required_evidence_count |
  ($readback.readback_index_ready == true
    and $readback.controlled_live_cutover_ready == false
    and $readback.index_entry_count == 7
    and $lib_export_present == true
    and ($sections | length) == 6
    and ($sections | all(.preview_ready == true and .mutation_enabled == false))
    and ($blocker_readbacks | length) == 7
    and $required_evidence_count == 7
    and ($blocker_readbacks | all(.included_in_packet == true and .acceptance_allowed == false and .waiver_allowed == false and .live_mutation_allowed == false))) as $packet_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_operator_packet_preview",
    status:(if $packet_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_operator_packet_preview_gate",
    schema_version:"controlled_live_operator_packet_preview_v1",
    plugin_id:"hepta-system@hepta-local",
    source_readback_index_ready:$readback.readback_index_ready,
    source_cutover_blocked:($readback.controlled_live_cutover_ready == false),
    source_blocker_count:$readback.source_blocker_count,
    source_index_entry_count:$readback.index_entry_count,
    lib_export_present:$lib_export_present,
    packet_id:"controlled-live-operator-packet-preview",
    scope_id:"hepta-system-controlled-live-read-only-chain",
    payload_hash:"sha256:controlled-live-operator-packet-preview-no-live-payload",
    rollback_owner:"operator-explicit-before-live",
    packet_section_count:($sections | length),
    blocker_readback_count:($blocker_readbacks | length),
    required_evidence_count:$required_evidence_count,
    operator_packet_preview_ready:$packet_ready,
    approval_request_ready:false,
    approval_request_sent:false,
    approval_recorded:false,
    packet_persisted:false,
    controlled_live_cutover_ready:false,
    live_execution_allowed:false,
    sections:$sections,
    blocker_readbacks:$blocker_readbacks,
    next_actions:[
      "phase5c_controlled_live_operator_packet_non_send_readback_without_approval_request",
      "keep_operator_packet_preview_unsent_and_unpersisted"
    ],
    next_migration_step:"phase5c_controlled_live_operator_packet_non_send_readback_without_approval_request",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      approval_requested:false,
      approval_recorded:false,
      packet_persisted:false,
      readback_persisted:false,
      blocker_waived:false,
      denial_accepted:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      rollback_executed:false,
      kill_switch_mutated:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
