#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_live_cutover_preflight.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution canary result acceptance preflight report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution live cutover preflight Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution live cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution live cutover preflight report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-live-cutover-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_PREFLIGHT_2026-06-21.md" \
  '
  def live_entry($entry):
    ($entry.canary_result_acceptance_preflight_route == "canary_result_acceptance_pending_evidence"
      and $entry.canary_result_acceptance_preflight_ready == true
      and $entry.canary_result_acceptance_pending_evidence == true
      and $entry.canary_acceptance_record_write_blocked == true
      and $entry.canary_acceptance_receipt_write_blocked == true) as $acceptance_ready |
    ($entry.canary_result_acceptance_preflight_route == "preflight_only_non_selected_candidate"
      and $entry.canary_result_acceptance_preflight_ready == true
      and $entry.preflight_only_non_selected_candidate == true) as $preflight_only |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_acceptance_preflight_route:$entry.canary_result_acceptance_preflight_route,
      registry_guard_route:$entry.registry_guard_route,
      selected_for_status_canary:$entry.selected_for_status_canary,
      preflight_only_non_selected_candidate:$entry.preflight_only_non_selected_candidate,
      live_cutover_preflight_route:(if ($acceptance_ready and $approval_guard) then "live_cutover_preflight_ready_pending_approval" elif ($preflight_only and $approval_guard) then "preflight_only_non_selected_candidate" elif ($acceptance_ready or $preflight_only) then "blocked_by_registry_guard" else "blocked_by_canary_result_acceptance_preflight" end),
      live_cutover_preflight_ready:(($acceptance_ready or $preflight_only) and $approval_guard),
      explicit_live_cutover_approval_required:$entry.selected_for_status_canary,
      explicit_live_cutover_approval_present:false,
      live_cutover_blocked:($entry.selected_for_status_canary == true and $acceptance_ready and $approval_guard),
      operator_identity_binding_present:true,
      rollback_anchor_present:true,
      kill_switch_present:true,
      observability_readback_required:true,
      live_cutover_switch_enabled:false,
      adapter_dispatch_switch_enabled:false,
      tool_invocation_execution_switch_enabled:false,
      live_cutover_started:false,
      live_cutover_acceptance_record_written:false,
      result_receipt_written:false,
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
  ($source.entries | map(live_entry(.))) as $entries |
  ($entries | map(select(.live_cutover_preflight_ready == true)) | length) as $ready_count |
  ($entries | map(select(.live_cutover_blocked == true)) | length) as $blocked_count |
  ($entries | map(select(.explicit_live_cutover_approval_required == true)) | length) as $approval_required_count |
  ($entries | map(select(.explicit_live_cutover_approval_required == true and .explicit_live_cutover_approval_present == false)) | length) as $approval_missing_count |
  ($entries | map(select(.rollback_anchor_present == true)) | length) as $rollback_count |
  ($entries | map(select(.kill_switch_present == true)) | length) as $kill_switch_count |
  ($entries | map(select(.observability_readback_required == true)) | length) as $observability_count |
  ($entries | map(select(.selected_for_status_canary == true)) | length) as $selected_count |
  ($entries | map(select(.preflight_only_non_selected_candidate == true)) | length) as $preflight_only_count |
  ($source.tool_execution_canary_result_acceptance_preflight_ready
    and $source.tool_execution_canary_result_acceptance_allowed == false
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $source.result_receipt_written == false
    and $ready_count == ($entries | length)
    and $selected_count == 1
    and ($selected_count + $preflight_only_count) == ($entries | length)
    and $blocked_count == $selected_count
    and $approval_required_count == $selected_count
    and $approval_missing_count == $selected_count
    and $rollback_count == ($entries | length)
    and $kill_switch_count == ($entries | length)
    and $observability_count == ($entries | length)
    and ($entries | all(if (.live_cutover_preflight_route == "live_cutover_preflight_ready_pending_approval" or .live_cutover_preflight_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .live_cutover_acceptance_record_written == false and .result_receipt_written == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end))) as $live_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_live_cutover_preflight",
    plugin_id:$source.plugin_id,
    status:(if $live_ready then "ready" else "blocked" end),
    source_acceptance_preflight_surface:$source.surface,
    source_acceptance_preflight_ready:$source.tool_execution_canary_result_acceptance_preflight_ready,
    source_canary_result_acceptance_allowed:$source.tool_execution_canary_result_acceptance_allowed,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    operator_identity_binding_present:true,
    explicit_live_cutover_approval_present:false,
    rollback_anchor_present:true,
    kill_switch_present:true,
    observability_readback_required:true,
    live_cutover_switch_enabled:false,
    adapter_dispatch_switch_enabled:false,
    tool_invocation_execution_switch_enabled:false,
    live_cutover_started:false,
    live_cutover_acceptance_record_written:false,
    result_receipt_written:false,
    candidate_count:($entries | length),
    live_cutover_preflight_ready_count:$ready_count,
    live_cutover_preflight_blocked_count:(($entries | length) - $ready_count),
    explicit_live_cutover_approval_required_count:$approval_required_count,
    explicit_live_cutover_approval_missing_count:$approval_missing_count,
    rollback_anchor_present_count:$rollback_count,
    kill_switch_present_count:$kill_switch_count,
    observability_readback_required_count:$observability_count,
    selected_status_canary_count:$selected_count,
    preflight_only_non_selected_count:$preflight_only_count,
    all_acceptance_preflight_entries_bound_to_live_cutover_preflight:($ready_count == ($entries | length) and $selected_count == 1 and (($selected_count + $preflight_only_count) == ($entries | length)) and $blocked_count == $selected_count and $approval_required_count == $selected_count and $approval_missing_count == $selected_count and $rollback_count == ($entries | length) and $kill_switch_count == ($entries | length) and $observability_count == ($entries | length)),
    all_live_cutover_entries_keep_no_invocation_guard:($entries | all(if (.live_cutover_preflight_route == "live_cutover_preflight_ready_pending_approval" or .live_cutover_preflight_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .explicit_live_cutover_approval_present == false and .live_cutover_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .tool_invocation_execution_switch_enabled == false and .live_cutover_started == false and .live_cutover_acceptance_record_written == false and .result_receipt_written == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_live_cutover_preflight_ready:$live_ready,
    tool_execution_live_cutover_allowed:false,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    execution_adapter_dispatched:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_live_cutover_operator_packet_without_invocation",
    entries:$entries,
    blockers:[
      "explicit_live_cutover_approval_missing",
      "canary_result_receipt_absent",
      "canary_readback_evidence_absent",
      "operator_canary_result_acceptance_absent",
      "live_cutover_switch_disabled",
      "adapter_dispatch_switch_disabled",
      "tool_invocation_execution_switch_disabled",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_live_cutover_operator_packet_without_invocation",
      "keep_live_cutover_preflight_read_only_until_explicit_operator_cutover_packet_exists",
      "keep_adapter_dispatch_tool_invocation_ledger_approval_receipt_and_mutation_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_live_cutover_preflight.rs",
      canary_result_acceptance_preflight_report:"scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-report.sh"
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
      live_cutover_started:false,
      live_cutover_acceptance_record_written:false,
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
