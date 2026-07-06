#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-preflight-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_live_cutover_operator_packet.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-operator-packet-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_OPERATOR_PACKET_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-operator-packet-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution live cutover preflight report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution live cutover operator packet Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution live cutover operator packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution live cutover operator packet report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-live-cutover-operator-packet-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_OPERATOR_PACKET_2026-06-21.md" \
  '
  def packet_entry($entry):
    ($entry.live_cutover_preflight_route == "live_cutover_preflight_ready_pending_approval"
      and $entry.live_cutover_preflight_ready == true
      and $entry.live_cutover_blocked == true
      and $entry.explicit_live_cutover_approval_required == true
      and $entry.explicit_live_cutover_approval_present == false) as $preflight_ready |
    ($entry.live_cutover_preflight_route == "preflight_only_non_selected_candidate"
      and $entry.live_cutover_preflight_ready == true
      and $entry.preflight_only_non_selected_candidate == true) as $preflight_only |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_live_cutover_preflight_route:$entry.live_cutover_preflight_route,
      registry_guard_route:$entry.registry_guard_route,
      selected_for_status_canary:$entry.selected_for_status_canary,
      preflight_only_non_selected_candidate:$entry.preflight_only_non_selected_candidate,
      live_cutover_operator_packet_route:(if ($preflight_ready and $approval_guard) then "live_cutover_operator_packet_ready_for_review" elif ($preflight_only and $approval_guard) then "preflight_only_non_selected_candidate" elif ($preflight_ready or $preflight_only) then "blocked_by_registry_guard" else "blocked_by_live_cutover_preflight" end),
      live_cutover_operator_packet_ready:(($preflight_ready or $preflight_only) and $approval_guard),
      operator_review_required:($entry.selected_for_status_canary == true and $preflight_ready and $approval_guard),
      remaining_blocker_readback_required:($entry.selected_for_status_canary == true and $preflight_ready and $approval_guard),
      approval_request_blocked:($entry.selected_for_status_canary == true and $preflight_ready and $approval_guard),
      explicit_live_cutover_approval_required:($entry.selected_for_status_canary == true and $preflight_ready and $approval_guard),
      explicit_live_cutover_approval_present:$entry.explicit_live_cutover_approval_present,
      operator_packet_template_present:true,
      operator_session_binding_present:true,
      operator_packet_rendered:false,
      approval_request_sent:false,
      operator_cutover_approval_record_written:false,
      operator_cutover_acceptance_present:false,
      live_cutover_switch_enabled:false,
      adapter_dispatch_switch_enabled:false,
      tool_invocation_execution_switch_enabled:false,
      live_cutover_started:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registry_source_of_truth_enabled:false,
      tool_registration_enabled:false,
      execution_adapter_dispatch_enabled:false,
      tool_invocation_enabled:false,
      ledger_write_enabled:false,
      approval_request_enabled:false,
      result_receipt_write_enabled:false,
      side_effect_free:true
    };

  ($source[0]) as $source |
  ($source.entries | map(packet_entry(.))) as $entries |
  ($entries | map(select(.live_cutover_operator_packet_ready == true)) | length) as $ready_count |
  ($entries | map(select(.operator_review_required == true)) | length) as $review_count |
  ($entries | map(select(.remaining_blocker_readback_required == true)) | length) as $readback_count |
  ($entries | map(select(.approval_request_blocked == true)) | length) as $request_blocked_count |
  ($entries | map(select(.selected_for_status_canary == true)) | length) as $selected_count |
  ($entries | map(select(.preflight_only_non_selected_candidate == true)) | length) as $preflight_only_count |
  ($source.tool_execution_live_cutover_preflight_ready
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $ready_count == ($entries | length)
    and $selected_count == 1
    and ($selected_count + $preflight_only_count) == ($entries | length)
    and $review_count == $selected_count
    and $readback_count == $selected_count
    and $request_blocked_count == $selected_count
    and ($entries | all(.operator_packet_template_present == true and .operator_session_binding_present == true))
    and ($entries | all(if (.live_cutover_operator_packet_route == "live_cutover_operator_packet_ready_for_review" or .live_cutover_operator_packet_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .operator_packet_rendered == false and .approval_request_sent == false and .operator_cutover_approval_record_written == false and .operator_cutover_acceptance_present == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end))) as $packet_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_live_cutover_operator_packet",
    plugin_id:$source.plugin_id,
    status:(if $packet_ready then "ready" else "blocked" end),
    source_live_cutover_preflight_surface:$source.surface,
    source_live_cutover_preflight_ready:$source.tool_execution_live_cutover_preflight_ready,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    operator_packet_template_present:true,
    operator_session_binding_present:true,
    operator_packet_rendered:false,
    approval_request_sent:false,
    operator_cutover_approval_record_written:false,
    operator_cutover_acceptance_present:false,
    live_cutover_switch_enabled:false,
    candidate_count:($entries | length),
    live_cutover_operator_packet_ready_count:$ready_count,
    live_cutover_operator_packet_blocked_count:(($entries | length) - $ready_count),
    operator_review_required_count:$review_count,
    remaining_blocker_readback_required_count:$readback_count,
    approval_request_blocked_count:$request_blocked_count,
    selected_status_canary_count:$selected_count,
    preflight_only_non_selected_count:$preflight_only_count,
    all_live_cutover_preflight_entries_bound_to_operator_packet:($ready_count == ($entries | length) and $selected_count == 1 and (($selected_count + $preflight_only_count) == ($entries | length)) and $review_count == $selected_count and $readback_count == $selected_count and $request_blocked_count == $selected_count),
    all_live_cutover_operator_packets_keep_no_invocation_guard:($entries | all(if (.live_cutover_operator_packet_route == "live_cutover_operator_packet_ready_for_review" or .live_cutover_operator_packet_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .operator_packet_rendered == false and .approval_request_sent == false and .operator_cutover_approval_record_written == false and .operator_cutover_acceptance_present == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_live_cutover_operator_packet_ready:$packet_ready,
    tool_execution_live_cutover_approval_request_allowed:false,
    tool_execution_live_cutover_allowed:false,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    execution_adapter_dispatched:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    result_receipt_written:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_live_cutover_operator_receipt_projection_without_invocation",
    entries:$entries,
    blockers:[
      "operator_packet_not_rendered",
      "approval_request_not_sent",
      "operator_cutover_approval_record_not_written",
      "operator_cutover_acceptance_absent",
      "explicit_live_cutover_approval_missing",
      "live_cutover_switch_disabled",
      "adapter_dispatch_switch_disabled",
      "tool_invocation_execution_switch_disabled",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_live_cutover_operator_receipt_projection_without_invocation",
      "keep_live_cutover_operator_packet_read_only_until_explicit_approval_request_path_is_restored",
      "keep_adapter_dispatch_tool_invocation_ledger_approval_receipt_and_mutation_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_live_cutover_operator_packet.rs",
      live_cutover_preflight_report:"scripts/hepta-systems-tool-execution-live-cutover-preflight-report.sh"
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
      operator_packet_rendered:false,
      operator_cutover_approval_record_written:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
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
