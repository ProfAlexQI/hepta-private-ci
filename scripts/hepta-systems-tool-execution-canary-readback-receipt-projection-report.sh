#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-canary-cutover-plan-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_canary_readback_receipt_projection.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_CANARY_READBACK_RECEIPT_PROJECTION_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-canary-readback-receipt-projection-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution canary cutover plan report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution canary readback receipt projection Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution canary readback receipt projection architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution canary readback receipt projection report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_CANARY_READBACK_RECEIPT_PROJECTION_2026-06-21.md" \
  '
  def projection_entry($entry):
    ($entry.canary_cutover_plan_route == "canary_cutover_plan_ready"
      and $entry.canary_cutover_plan_ready == true
      and $entry.canary_start_blocked == true
      and $entry.canary_readback_receipt_required == true
      and $entry.canary_result_receipt_schema_present == true) as $canary_plan_ready |
    ($entry.canary_cutover_plan_route == "preflight_only_non_selected_candidate"
      and $entry.canary_cutover_plan_ready == true
      and $entry.preflight_only_non_selected_candidate == true) as $preflight_only |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_canary_cutover_plan_route:$entry.canary_cutover_plan_route,
      registry_guard_route:$entry.registry_guard_route,
      selected_for_status_canary:$entry.selected_for_status_canary,
      preflight_only_non_selected_candidate:$entry.preflight_only_non_selected_candidate,
      canary_readback_receipt_projection_route:(if ($canary_plan_ready and $approval_guard) then "canary_readback_receipt_projection_ready" elif ($preflight_only and $approval_guard) then "preflight_only_non_selected_candidate" elif ($canary_plan_ready or $preflight_only) then "blocked_by_registry_guard" else "blocked_by_canary_plan" end),
      canary_readback_receipt_projection_ready:(($canary_plan_ready or $preflight_only) and $approval_guard),
      canary_readback_channel_declared:true,
      canary_result_receipt_digest_required:true,
      canary_trace_correlation_required:true,
      rollback_readback_required:true,
      operator_summary_required:true,
      canary_result_receipt_write_blocked:($entry.selected_for_status_canary == true and $canary_plan_ready and $approval_guard),
      canary_cutover_switch_enabled:false,
      live_cutover_switch_enabled:false,
      canary_execution_started:false,
      canary_result_receipt_written:false,
      canary_readback_projection_written:false,
      rollback_executed:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registry_source_of_truth_enabled:false,
      tool_registration_enabled:false,
      execution_adapter_dispatch_enabled:false,
      tool_invocation_enabled:false,
      ledger_write_enabled:false,
      approval_request_enabled:false,
      operator_decision_record_write_enabled:false,
      operator_decision_receipt_write_enabled:false,
      result_receipt_write_enabled:false,
      side_effect_free:true
    };

  ($source[0]) as $source |
  ($source.entries | map(projection_entry(.))) as $entries |
  ($entries | map(select(.canary_readback_receipt_projection_ready == true)) | length) as $ready_count |
  ($entries | map(select(.canary_readback_channel_declared == true)) | length) as $channel_count |
  ($entries | map(select(.canary_result_receipt_digest_required == true)) | length) as $digest_count |
  ($entries | map(select(.canary_trace_correlation_required == true)) | length) as $trace_count |
  ($entries | map(select(.rollback_readback_required == true)) | length) as $rollback_count |
  ($entries | map(select(.operator_summary_required == true)) | length) as $summary_count |
  ($entries | map(select(.canary_result_receipt_write_blocked == true)) | length) as $write_blocked_count |
  ($entries | map(select(.selected_for_status_canary == true)) | length) as $selected_count |
  ($entries | map(select(.preflight_only_non_selected_candidate == true)) | length) as $preflight_only_count |
  ($source.tool_execution_canary_cutover_plan_ready
    and $source.tool_execution_canary_cutover_start_allowed == false
    and $source.tool_execution_canary_result_receipt_write_allowed == false
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $source.result_receipt_written == false
    and $ready_count == ($entries | length)
    and $channel_count == ($entries | length)
    and $digest_count == ($entries | length)
    and $trace_count == ($entries | length)
    and $rollback_count == ($entries | length)
    and $summary_count == ($entries | length)
    and $selected_count == 1
    and ($selected_count + $preflight_only_count) == ($entries | length)
    and $write_blocked_count == $selected_count
    and ($entries | all(if (.canary_readback_receipt_projection_route == "canary_readback_receipt_projection_ready" or .canary_readback_receipt_projection_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .canary_cutover_switch_enabled == false and .live_cutover_switch_enabled == false and .canary_execution_started == false and .canary_result_receipt_written == false and .canary_readback_projection_written == false and .rollback_executed == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .operator_decision_record_write_enabled == false and .operator_decision_receipt_write_enabled == false and .result_receipt_write_enabled == false) else true end))) as $projection_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_canary_readback_receipt_projection",
    plugin_id:$source.plugin_id,
    status:(if $projection_ready then "ready" else "blocked" end),
    source_canary_cutover_plan_surface:$source.surface,
    source_canary_cutover_plan_ready:$source.tool_execution_canary_cutover_plan_ready,
    source_canary_cutover_start_allowed:$source.tool_execution_canary_cutover_start_allowed,
    source_canary_result_receipt_write_allowed:$source.tool_execution_canary_result_receipt_write_allowed,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    canary_readback_channel_declared:true,
    canary_result_receipt_digest_required:true,
    canary_trace_correlation_required:true,
    rollback_readback_required:true,
    operator_summary_required:true,
    canary_cutover_switch_enabled:false,
    live_cutover_switch_enabled:false,
    canary_execution_started:false,
    canary_result_receipt_written:false,
    canary_readback_projection_written:false,
    rollback_executed:false,
    candidate_count:($entries | length),
    canary_readback_receipt_projection_ready_count:$ready_count,
    canary_readback_receipt_projection_blocked_count:(($entries | length) - $ready_count),
    canary_readback_channel_declared_count:$channel_count,
    canary_result_receipt_digest_required_count:$digest_count,
    canary_trace_correlation_required_count:$trace_count,
    rollback_readback_required_count:$rollback_count,
    operator_summary_required_count:$summary_count,
    canary_result_receipt_write_blocked_count:$write_blocked_count,
    selected_status_canary_count:$selected_count,
    preflight_only_non_selected_count:$preflight_only_count,
    all_canary_plan_entries_bound_to_readback_projection:($ready_count == ($entries | length) and $channel_count == ($entries | length) and $digest_count == ($entries | length) and $trace_count == ($entries | length) and $rollback_count == ($entries | length) and $summary_count == ($entries | length) and $selected_count == 1 and (($selected_count + $preflight_only_count) == ($entries | length)) and $write_blocked_count == $selected_count),
    all_canary_readback_entries_keep_no_invocation_guard:($entries | all(if (.canary_readback_receipt_projection_route == "canary_readback_receipt_projection_ready" or .canary_readback_receipt_projection_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .canary_cutover_switch_enabled == false and .live_cutover_switch_enabled == false and .canary_execution_started == false and .canary_result_receipt_written == false and .canary_readback_projection_written == false and .rollback_executed == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .operator_decision_record_write_enabled == false and .operator_decision_receipt_write_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_canary_readback_receipt_projection_ready:$projection_ready,
    tool_execution_canary_result_receipt_write_allowed:false,
    tool_execution_canary_result_acceptance_allowed:false,
    tool_execution_live_cutover_allowed:false,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    execution_adapter_dispatched:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    operator_decision_record_written:false,
    operator_decision_receipt_written:false,
    result_receipt_written:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_canary_result_acceptance_preflight_without_invocation",
    entries:$entries,
    blockers:[
      "canary_cutover_switch_disabled",
      "canary_execution_not_started",
      "canary_result_receipt_not_written",
      "canary_readback_projection_not_written",
      "rollback_not_executed",
      "canary_result_acceptance_disabled",
      "live_cutover_switch_disabled",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_canary_result_acceptance_preflight_without_invocation",
      "keep_canary_readback_projection_read_only_until_a_real_canary_has_explicit_receipts",
      "keep_canary_invocation_ledger_approval_receipt_and_live_mutation_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_canary_readback_receipt_projection.rs",
      canary_cutover_plan_report:"scripts/hepta-systems-tool-execution-canary-cutover-plan-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_decision_record_written:false,
      operator_decision_receipt_written:false,
      operator_acceptance_recorded:false,
      canary_cutover_started:false,
      canary_result_receipt_written:false,
      canary_readback_projection_written:false,
      rollback_executed:false,
      result_receipt_written:false,
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
