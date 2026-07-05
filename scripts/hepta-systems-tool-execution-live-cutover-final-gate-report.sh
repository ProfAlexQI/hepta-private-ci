#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_live_cutover_final_gate.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_FINAL_GATE_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-final-gate-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution live cutover receipt rollback packet report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution live cutover final gate Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution live cutover final gate architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution live cutover final gate report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_FINAL_GATE_2026-06-21.md" \
  '
  def final_entry($entry):
    ($entry.live_cutover_receipt_rollback_packet_route == "live_cutover_receipt_rollback_packet_ready"
      and $entry.live_cutover_receipt_rollback_packet_ready == true
      and $entry.live_cutover_start_blocked == true
      and $entry.rollback_execution_blocked == true
      and $entry.result_receipt_write_blocked == true) as $packet_ready |
    ($entry.live_cutover_receipt_rollback_packet_route == "preflight_only_non_selected_candidate"
      and $entry.live_cutover_receipt_rollback_packet_ready == true
      and $entry.preflight_only_non_selected_candidate == true) as $preflight_only |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_live_cutover_receipt_rollback_packet_route:$entry.live_cutover_receipt_rollback_packet_route,
      registry_guard_route:$entry.registry_guard_route,
      selected_for_status_canary:$entry.selected_for_status_canary,
      preflight_only_non_selected_candidate:$entry.preflight_only_non_selected_candidate,
      live_cutover_final_gate_route:(if ($packet_ready and $approval_guard) then "live_cutover_final_gate_ready_blocked" elif ($preflight_only and $approval_guard) then "preflight_only_non_selected_candidate" elif ($packet_ready or $preflight_only) then "blocked_by_registry_guard" else "blocked_by_receipt_rollback_packet" end),
      live_cutover_final_gate_ready:(($packet_ready or $preflight_only) and $approval_guard),
      final_gate_policy_present:true,
      final_cutover_ticket_present:true,
      final_operator_readback_required:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      explicit_live_cutover_approval_required:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      explicit_live_cutover_approval_present:false,
      live_cutover_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      approval_request_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      operator_acceptance_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      execution_switch_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      adapter_dispatch_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      tool_invocation_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      ledger_write_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      rollback_execution_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      result_receipt_write_blocked:($entry.selected_for_status_canary == true and $packet_ready and $approval_guard),
      approval_request_sent:false,
      operator_cutover_decision_receipt_written:false,
      operator_cutover_readback_evidence_written:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_switch_enabled:false,
      adapter_dispatch_switch_enabled:false,
      tool_invocation_execution_switch_enabled:false,
      live_cutover_started:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      result_receipt_written:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registry_source_of_truth_enabled:false,
      tool_registration_enabled:false,
      execution_adapter_dispatch_enabled:false,
      tool_invocation_enabled:false,
      ledger_write_enabled:false,
      approval_request_enabled:false,
      side_effect_free:true
    };

  ($source[0]) as $source |
  ($source.entries | map(final_entry(.))) as $entries |
  ($entries | map(select(.live_cutover_final_gate_ready == true)) | length) as $ready_count |
  ($entries | map(select(.explicit_live_cutover_approval_required == true)) | length) as $approval_required_count |
  ($entries | map(select(.explicit_live_cutover_approval_required == true and .explicit_live_cutover_approval_present == false)) | length) as $approval_missing_count |
  ($entries | map(select(.final_operator_readback_required == true)) | length) as $readback_count |
  ($entries | map(select(.live_cutover_blocked == true)) | length) as $live_blocked_count |
  ($entries | map(select(.approval_request_blocked == true)) | length) as $approval_request_blocked_count |
  ($entries | map(select(.operator_acceptance_blocked == true)) | length) as $acceptance_blocked_count |
  ($entries | map(select(.execution_switch_blocked == true)) | length) as $execution_switch_blocked_count |
  ($entries | map(select(.rollback_execution_blocked == true)) | length) as $rollback_blocked_count |
  ($entries | map(select(.result_receipt_write_blocked == true)) | length) as $receipt_write_blocked_count |
  ($entries | map(select(.selected_for_status_canary == true)) | length) as $selected_count |
  ($entries | map(select(.preflight_only_non_selected_candidate == true)) | length) as $preflight_only_count |
  ($source.tool_execution_live_cutover_receipt_rollback_packet_ready
    and $source.tool_execution_live_cutover_start_allowed == false
    and $source.tool_execution_live_cutover_rollback_allowed == false
    and $source.tool_execution_live_cutover_result_receipt_write_allowed == false
    and $source.tool_execution_live_cutover_allowed == false
    and $ready_count == ($entries | length)
    and $selected_count == 1
    and ($selected_count + $preflight_only_count) == ($entries | length)
    and $approval_required_count == $selected_count
    and $approval_missing_count == $selected_count
    and $readback_count == $selected_count
    and $live_blocked_count == $selected_count
    and $approval_request_blocked_count == $selected_count
    and $acceptance_blocked_count == $selected_count
    and $execution_switch_blocked_count == $selected_count
    and $rollback_blocked_count == $selected_count
    and $receipt_write_blocked_count == $selected_count
    and ($entries | all(if (.live_cutover_final_gate_route == "live_cutover_final_gate_ready_blocked" or .live_cutover_final_gate_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .approval_request_sent == false and .operator_cutover_decision_receipt_written == false and .operator_cutover_readback_evidence_written == false and .operator_cutover_acceptance_recorded == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .rollback_executed == false and .rollback_receipt_written == false and .result_receipt_written == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false) else true end))) as $final_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_live_cutover_final_gate",
    plugin_id:$source.plugin_id,
    status:(if $final_ready then "ready" else "blocked" end),
    source_live_cutover_receipt_rollback_packet_surface:$source.surface,
    source_live_cutover_receipt_rollback_packet_ready:$source.tool_execution_live_cutover_receipt_rollback_packet_ready,
    source_live_cutover_start_allowed:$source.tool_execution_live_cutover_start_allowed,
    source_live_cutover_rollback_allowed:$source.tool_execution_live_cutover_rollback_allowed,
    source_live_cutover_result_receipt_write_allowed:$source.tool_execution_live_cutover_result_receipt_write_allowed,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    final_gate_policy_present:true,
    final_cutover_ticket_present:true,
    final_operator_readback_required:true,
    explicit_live_cutover_approval_present:false,
    approval_request_sent:false,
    operator_cutover_decision_receipt_written:false,
    operator_cutover_readback_evidence_written:false,
    operator_cutover_acceptance_recorded:false,
    live_cutover_switch_enabled:false,
    adapter_dispatch_switch_enabled:false,
    tool_invocation_execution_switch_enabled:false,
    live_cutover_started:false,
    rollback_executed:false,
    rollback_receipt_written:false,
    result_receipt_written:false,
    candidate_count:($entries | length),
    live_cutover_final_gate_ready_count:$ready_count,
    live_cutover_final_gate_blocked_count:(($entries | length) - $ready_count),
    explicit_live_cutover_approval_required_count:$approval_required_count,
    explicit_live_cutover_approval_missing_count:$approval_missing_count,
    final_operator_readback_required_count:$readback_count,
    live_cutover_blocked_count:$live_blocked_count,
    approval_request_blocked_count:$approval_request_blocked_count,
    operator_acceptance_blocked_count:$acceptance_blocked_count,
    execution_switch_blocked_count:$execution_switch_blocked_count,
    rollback_execution_blocked_count:$rollback_blocked_count,
    result_receipt_write_blocked_count:$receipt_write_blocked_count,
    selected_status_canary_count:$selected_count,
    preflight_only_non_selected_count:$preflight_only_count,
    all_receipt_rollback_packets_bound_to_final_gate:($ready_count == ($entries | length) and $selected_count == 1 and (($selected_count + $preflight_only_count) == ($entries | length)) and $approval_required_count == $selected_count and $approval_missing_count == $selected_count and $readback_count == $selected_count and $live_blocked_count == $selected_count and $approval_request_blocked_count == $selected_count and $acceptance_blocked_count == $selected_count and $execution_switch_blocked_count == $selected_count and $rollback_blocked_count == $selected_count and $receipt_write_blocked_count == $selected_count),
    all_live_cutover_final_gate_entries_keep_no_invocation_guard:($entries | all(if (.live_cutover_final_gate_route == "live_cutover_final_gate_ready_blocked" or .live_cutover_final_gate_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .approval_request_sent == false and .operator_cutover_decision_receipt_written == false and .operator_cutover_readback_evidence_written == false and .operator_cutover_acceptance_recorded == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .rollback_executed == false and .rollback_receipt_written == false and .result_receipt_written == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false) else true end)),
    tool_execution_live_cutover_final_gate_ready:$final_ready,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    execution_adapter_dispatched:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    live_mutation_ready:false,
    next_migration_step:"manual_operator_live_cutover_approval_required",
    entries:$entries,
    blockers:[
      "explicit_live_cutover_approval_missing",
      "approval_request_not_sent",
      "operator_cutover_acceptance_absent",
      "live_cutover_switch_disabled",
      "adapter_dispatch_switch_disabled",
      "tool_invocation_execution_switch_disabled",
      "live_cutover_blocked",
      "rollback_execution_blocked",
      "result_receipt_write_blocked",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled"
    ],
    next_actions:[
      "manual_operator_live_cutover_approval_required",
      "keep_live_cutover_final_gate_read_only_until_explicit_live_cutover_approval",
      "keep_start_rollback_result_receipt_dispatch_invocation_ledger_approval_and_mutation_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_live_cutover_final_gate.rs",
      live_cutover_receipt_rollback_packet_report:"scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-report.sh"
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
      operator_cutover_decision_receipt_written:false,
      operator_cutover_readback_evidence_written:false,
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
