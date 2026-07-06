#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SUMMARY_REPORT="$ROOT/scripts/hepta-systems-current-compact-capability-summary-report.sh"
SUMMARY_GATE="$ROOT/scripts/hepta-systems-current-compact-capability-summary-gate.sh"
HISTORICAL_CANONICAL_GATE="$ROOT/scripts/hepta-systems-canonical-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_WRAPPER_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-wrapper-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SUMMARY_REPORT" ]] || fail "missing executable current compact capability summary report: $SUMMARY_REPORT"
[[ -x "$SUMMARY_GATE" ]] || fail "missing executable current compact capability summary gate: $SUMMARY_GATE"
[[ -f "$DOC" ]] || fail "missing current canonical wrapper architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the current canonical wrapper report"
fi

historical_gate_created=false
historical_gate_executable=false
historical_gate_target_matches=false
historical_gate_exec_count=0

if [[ -f "$HISTORICAL_CANONICAL_GATE" ]]; then
  historical_gate_created=true
fi
if [[ -x "$HISTORICAL_CANONICAL_GATE" ]]; then
  historical_gate_executable=true
fi
if [[ -f "$HISTORICAL_CANONICAL_GATE" ]] \
  && grep -q 'TARGET="\$ROOT/scripts/hepta-systems-current-canonical-wrapper-gate.sh"' "$HISTORICAL_CANONICAL_GATE"; then
  historical_gate_target_matches=true
fi
if [[ -f "$HISTORICAL_CANONICAL_GATE" ]]; then
  historical_gate_exec_count="$(grep -c 'exec "$TARGET" "$@"' "$HISTORICAL_CANONICAL_GATE" || true)"
fi

jq -n \
  --slurpfile summary <("$SUMMARY_REPORT") \
  --arg gate "scripts/hepta-systems-current-canonical-wrapper-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_WRAPPER_2026-06-21.md" \
  --arg historical_gate_path "scripts/hepta-systems-canonical-gate.sh" \
  --arg historical_gate_target "scripts/hepta-systems-current-canonical-wrapper-gate.sh" \
  --argjson historical_gate_created "$historical_gate_created" \
  --argjson historical_gate_executable "$historical_gate_executable" \
  --argjson historical_gate_target_matches "$historical_gate_target_matches" \
  --argjson historical_gate_exec_count "$historical_gate_exec_count" \
  '
  ($summary[0]) as $summary |
  [
    {
      id:"current_compact_capability_summary_gate",
      command:"scripts/hepta-systems-current-compact-capability-summary-gate.sh",
      required:true,
      runnable_locally:true,
      invoked_by_report:false
    },
    {
      id:"restore_preflight_gate",
      command:"scripts/hepta-systems-compact-capability-matrix-restore-preflight-gate.sh",
      required:true,
      runnable_locally:true,
      invoked_by_report:false
    },
    {
      id:"canonical_attachment_index_gate",
      command:"scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh",
      required:true,
      runnable_locally:true,
      invoked_by_report:false
    }
  ] as $wrapper_plan |
  ($summary.compact_capability_summary_ready
    and $summary.execution_enabled_count == 0
    and $summary.public_ga_enabled_count == 0
    and $summary.manual_operator_live_cutover_approval_required == true
    and $summary.tool_execution_live_cutover_allowed == false
    and $summary.tool_execution_public_ga_allowed == false
    and $historical_gate_created == true
    and $historical_gate_executable == true
    and $historical_gate_target_matches == true
    and $historical_gate_exec_count == 1
    and ($wrapper_plan | all(.required == true and .runnable_locally == true and .invoked_by_report == false))
    and ($summary.side_effects | to_entries | all(.value == false))) as $wrapper_ready |
  {
    runtime:"hepta",
    surface:"current_canonical_wrapper",
    plugin_id:$summary.plugin_id,
    status:(if $wrapper_ready then "ready" else "blocked" end),
    source_compact_capability_summary_surface:$summary.surface,
    source_compact_capability_summary_ready:$summary.compact_capability_summary_ready,
    source_local_surface_count:$summary.local_surface_count,
    source_local_surface_ready_count:$summary.local_surface_ready_count,
    source_execution_enabled_count:$summary.execution_enabled_count,
    source_public_ga_enabled_count:$summary.public_ga_enabled_count,
    source_manual_operator_live_cutover_approval_required:$summary.manual_operator_live_cutover_approval_required,
    source_tool_execution_live_cutover_allowed:$summary.tool_execution_live_cutover_allowed,
    source_tool_execution_public_ga_allowed:$summary.tool_execution_public_ga_allowed,
    current_canonical_wrapper_ready:$wrapper_ready,
    wrapper_plan_step_count:($wrapper_plan | length),
    wrapper_plan:$wrapper_plan,
    historical_canonical_gate_name_claimed:$historical_gate_created,
    historical_canonical_gate_created:$historical_gate_created,
    historical_canonical_gate_executable:$historical_gate_executable,
    historical_canonical_gate_wrapper_kind:"thin_local_exec_wrapper",
    historical_canonical_gate_wrapper_path:$historical_gate_path,
    historical_canonical_gate_wrapper_target:$historical_gate_target,
    historical_canonical_gate_wrapper_target_matches:$historical_gate_target_matches,
    historical_canonical_gate_wrapper_exec_count:$historical_gate_exec_count,
    historical_canonical_gate_mutated:$historical_gate_created,
    historical_canonical_gate_mutated_by_report:false,
    canonical_gate_wrapper_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"validate_historical_canonical_gate_thin_wrapper_without_live_invocation",
    wrapper_blockers:[
      "historical_canonical_gate_thin_wrapper_validation_pending",
      "manual_operator_live_cutover_approval_required",
      "tool_execution_live_cutover_allowed_false",
      "tool_execution_public_ga_allowed_false",
      "terminal_live_gates_not_invoked",
      "live_url_not_contacted",
      "long_soak_not_started"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      current_compact_capability_summary_report:"scripts/hepta-systems-current-compact-capability-summary-report.sh",
      current_compact_capability_summary_gate:"scripts/hepta-systems-current-compact-capability-summary-gate.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
      historical_canonical_gate_mutated:false,
      canonical_gate_invoked:false,
      capability_matrix_gate_invoked:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
