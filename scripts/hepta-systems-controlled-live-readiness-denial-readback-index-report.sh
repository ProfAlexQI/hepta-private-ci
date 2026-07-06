#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
AUDIT_REPORT="$ROOT/scripts/hepta-systems-controlled-live-readiness-audit-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_readiness_denial_readback_index.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-readiness-denial-readback-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$AUDIT_REPORT" ]] || fail "missing executable Phase 5 audit report: $AUDIT_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5a Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5a architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5a denial readback index report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_readiness_denial_readback_index_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

audit_json="${HEPTA_CONTROLLED_LIVE_AUDIT_JSON:-}"
if [[ -n "$audit_json" ]]; then
  [[ -f "$audit_json" ]] || fail "missing cached Phase 5 audit report: $audit_json"
else
  audit_json="$tmpdir/audit.json"
  "$AUDIT_REPORT" >"$audit_json" || fail "failed to render Phase 5 audit report"
fi

jq -n \
  --slurpfile audit "$audit_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-readiness-denial-readback-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_2026-06-27.md" \
  '
  def entry($id; $source_blocker_id; $layer; $query_key; $readback_route; $operator_label; $required_evidence): {
    id:$id,
    source_blocker_id:$source_blocker_id,
    layer:$layer,
    query_key:$query_key,
    readback_route:$readback_route,
    operator_label:$operator_label,
    required_evidence:$required_evidence,
    current_state:"missing",
    queryable:true,
    operator_facing:true,
    blocks_cutover:true,
    operator_recoverable:true,
    waiver_allowed:false,
    acceptance_allowed:false,
    live_mutation_allowed:false
  };
  ($audit[0]) as $audit |
  [
    entry("dirty_worktree_boundary_readback"; "dirty_worktree_boundary"; "release"; "controlled_live.blockers.dirty_worktree_boundary"; "controlled_live_readiness_denial.dirty_worktree_boundary"; "Dirty worktree boundary"; "A clean scoped worktree or explicit release boundary attestation"),
    entry("operator_live_approval_missing_readback"; "operator_live_approval_missing"; "operator"; "controlled_live.blockers.operator_live_approval_missing"; "controlled_live_readiness_denial.operator_live_approval_missing"; "Operator live approval"; "Explicit operator live approval packet with scope, payload hash, and rollback owner"),
    entry("fresh_soak_readback_missing_readback"; "fresh_soak_readback_missing"; "observability"; "controlled_live.blockers.fresh_soak_readback_missing"; "controlled_live_readiness_denial.fresh_soak_readback_missing"; "Fresh soak/readback evidence"; "Fresh soak samples and readback evidence for this exact cutover"),
    entry("credential_boundary_attestation_missing_readback"; "credential_boundary_attestation_missing"; "security"; "controlled_live.blockers.credential_boundary_attestation_missing"; "controlled_live_readiness_denial.credential_boundary_attestation_missing"; "Credential boundary attestation"; "Credential access boundary attestation without exposing secrets"),
    entry("gateway_native_telegram_post_boundary_approval_missing_readback"; "gateway_native_telegram_post_boundary_approval_missing"; "transport"; "controlled_live.blockers.gateway_native_telegram_post_boundary_approval_missing"; "controlled_live_readiness_denial.gateway_native_telegram_post_boundary_approval_missing"; "Gateway/Native/Telegram POST boundary approval"; "Explicit transport mutation boundary approval for Gateway, Native POST, and Telegram"),
    entry("rollback_rehearsal_missing_readback"; "rollback_rehearsal_missing"; "rollback"; "controlled_live.blockers.rollback_rehearsal_missing"; "controlled_live_readiness_denial.rollback_rehearsal_missing"; "Rollback rehearsal evidence"; "Rollback rehearsal evidence tied to the cutover payload and owner"),
    entry("kill_switch_rehearsal_missing_readback"; "kill_switch_rehearsal_missing"; "rollback"; "controlled_live.blockers.kill_switch_rehearsal_missing"; "controlled_live_readiness_denial.kill_switch_rehearsal_missing"; "Kill-switch rehearsal evidence"; "Kill-switch rehearsal evidence tied to the cutover payload and owner")
  ] as $entries |
  ($entries | map(select(.queryable == true)) | length) as $queryable_count |
  ($entries | map(select(.operator_facing == true)) | length) as $operator_facing_count |
  ($entries | map(select((.readback_route | length) > 0)) | length) as $readback_route_count |
  ($entries | map(select(.acceptance_allowed == true)) | length) as $accepted_denial_count |
  ($entries | map(select(.waiver_allowed == true)) | length) as $waived_blocker_count |
  ($audit.controlled_live_audit_ready == true
    and $audit.controlled_live_cutover_ready == false
    and $audit.blocker_count == 7
    and $lib_export_present == true
    and ($entries | length) == 7
    and $queryable_count == 7
    and $operator_facing_count == 7
    and $readback_route_count == 7
    and $accepted_denial_count == 0
    and $waived_blocker_count == 0
    and ($entries | all(.blocks_cutover == true and .operator_recoverable == true and .waiver_allowed == false and .acceptance_allowed == false and .live_mutation_allowed == false))) as $index_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_readiness_denial_readback_index",
    status:(if $index_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_readiness_denial_readback_index_gate",
    schema_version:"controlled_live_readiness_denial_readback_index_v1",
    plugin_id:"hepta-system@hepta-local",
    source_controlled_live_audit_ready:$audit.controlled_live_audit_ready,
    source_controlled_live_cutover_ready:$audit.controlled_live_cutover_ready,
    source_cutover_blocked:($audit.controlled_live_cutover_ready == false),
    source_blocker_count:$audit.blocker_count,
    source_blocking_precondition_count:$audit.blocking_precondition_count,
    source_audit_status:$audit.status,
    lib_export_present:$lib_export_present,
    index_entry_count:($entries | length),
    queryable_entry_count:$queryable_count,
    operator_facing_entry_count:$operator_facing_count,
    readback_route_count:$readback_route_count,
    accepted_denial_count:$accepted_denial_count,
    waived_blocker_count:$waived_blocker_count,
    readback_index_ready:$index_ready,
    controlled_live_cutover_ready:false,
    ready_for_approval_request:false,
    ready_for_approval_recording:false,
    ready_for_readback_persistence:false,
    ready_for_live_execution:false,
    entries:$entries,
    next_actions:[
      "phase5b_controlled_live_operator_packet_preview_without_approval_request",
      "keep_denials_queryable_without_acceptance_or_waiver"
    ],
    next_migration_step:"phase5b_controlled_live_operator_packet_preview_without_approval_request",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      approval_requested:false,
      approval_recorded:false,
      blocker_waived:false,
      denial_accepted:false,
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
      rollback_executed:false,
      kill_switch_mutated:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
