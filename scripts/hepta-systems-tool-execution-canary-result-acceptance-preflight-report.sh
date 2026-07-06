#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_canary_result_acceptance_preflight.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_CANARY_RESULT_ACCEPTANCE_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-canary-result-acceptance-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution canary readback receipt projection report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution canary result acceptance preflight Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution canary result acceptance preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution canary result acceptance preflight report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_CANARY_RESULT_ACCEPTANCE_PREFLIGHT_2026-06-21.md" \
  '
  def acceptance_entry($entry):
    ($entry.canary_readback_receipt_projection_route == "canary_readback_receipt_projection_ready"
      and $entry.canary_readback_receipt_projection_ready == true
      and $entry.canary_result_receipt_write_blocked == true) as $projection_ready |
    ($entry.canary_readback_receipt_projection_route == "preflight_only_non_selected_candidate"
      and $entry.canary_readback_receipt_projection_ready == true
      and $entry.preflight_only_non_selected_candidate == true) as $preflight_only |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_readback_projection_route:$entry.canary_readback_receipt_projection_route,
      registry_guard_route:$entry.registry_guard_route,
      selected_for_status_canary:$entry.selected_for_status_canary,
      preflight_only_non_selected_candidate:$entry.preflight_only_non_selected_candidate,
      canary_result_acceptance_preflight_route:(if ($projection_ready and $approval_guard) then "canary_result_acceptance_pending_evidence" elif ($preflight_only and $approval_guard) then "preflight_only_non_selected_candidate" elif ($projection_ready or $preflight_only) then "blocked_by_registry_guard" else "blocked_by_readback_projection" end),
      canary_result_acceptance_preflight_ready:(($projection_ready or $preflight_only) and $approval_guard),
      canary_result_acceptance_pending_evidence:($projection_ready and $approval_guard),
      canary_result_receipt_required:$entry.selected_for_status_canary,
      canary_readback_evidence_required:$entry.selected_for_status_canary,
      canary_acceptance_record_write_blocked:($entry.selected_for_status_canary == true and $projection_ready and $approval_guard),
      canary_acceptance_receipt_write_blocked:($entry.selected_for_status_canary == true and $projection_ready and $approval_guard),
      canary_result_acceptance_policy_present:true,
      operator_identity_binding_present:true,
      canary_result_receipt_present:false,
      canary_readback_evidence_present:false,
      operator_canary_result_acceptance_present:false,
      canary_acceptance_record_written:false,
      canary_acceptance_receipt_written:false,
      live_cutover_switch_enabled:false,
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
  ($source.entries | map(acceptance_entry(.))) as $entries |
  ($entries | map(select(.canary_result_acceptance_preflight_ready == true)) | length) as $ready_count |
  ($entries | map(select(.canary_result_acceptance_pending_evidence == true)) | length) as $pending_count |
  ($entries | map(select(.canary_result_receipt_required == true)) | length) as $receipt_required_count |
  ($entries | map(select(.canary_readback_evidence_required == true)) | length) as $readback_required_count |
  ($entries | map(select(.canary_acceptance_record_write_blocked == true)) | length) as $record_write_blocked_count |
  ($entries | map(select(.canary_acceptance_receipt_write_blocked == true)) | length) as $receipt_write_blocked_count |
  ($entries | map(select(.selected_for_status_canary == true)) | length) as $selected_count |
  ($entries | map(select(.preflight_only_non_selected_candidate == true)) | length) as $preflight_only_count |
  ($source.tool_execution_canary_readback_receipt_projection_ready
    and $source.tool_execution_canary_result_receipt_write_allowed == false
    and $source.tool_execution_canary_result_acceptance_allowed == false
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $source.result_receipt_written == false
    and $ready_count == ($entries | length)
    and $selected_count == 1
    and ($selected_count + $preflight_only_count) == ($entries | length)
    and $pending_count == $selected_count
    and $receipt_required_count == $selected_count
    and $readback_required_count == $selected_count
    and $record_write_blocked_count == $selected_count
    and $receipt_write_blocked_count == $selected_count
    and ($entries | all(if (.canary_result_acceptance_preflight_route == "canary_result_acceptance_pending_evidence" or .canary_result_acceptance_preflight_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .canary_result_receipt_present == false and .canary_readback_evidence_present == false and .operator_canary_result_acceptance_present == false and .canary_acceptance_record_written == false and .canary_acceptance_receipt_written == false and .live_cutover_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end))) as $acceptance_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_canary_result_acceptance_preflight",
    plugin_id:$source.plugin_id,
    status:(if $acceptance_ready then "ready" else "blocked" end),
    source_readback_projection_surface:$source.surface,
    source_readback_projection_ready:$source.tool_execution_canary_readback_receipt_projection_ready,
    source_canary_result_receipt_write_allowed:$source.tool_execution_canary_result_receipt_write_allowed,
    source_canary_result_acceptance_allowed:$source.tool_execution_canary_result_acceptance_allowed,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    canary_result_acceptance_policy_present:true,
    operator_identity_binding_present:true,
    canary_result_receipt_present:false,
    canary_readback_evidence_present:false,
    operator_canary_result_acceptance_present:false,
    canary_acceptance_record_written:false,
    canary_acceptance_receipt_written:false,
    live_cutover_switch_enabled:false,
    candidate_count:($entries | length),
    canary_result_acceptance_preflight_ready_count:$ready_count,
    canary_result_acceptance_preflight_blocked_count:(($entries | length) - $ready_count),
    canary_result_acceptance_pending_evidence_count:$pending_count,
    canary_result_receipt_required_count:$receipt_required_count,
    canary_readback_evidence_required_count:$readback_required_count,
    canary_acceptance_record_write_blocked_count:$record_write_blocked_count,
    canary_acceptance_receipt_write_blocked_count:$receipt_write_blocked_count,
    selected_status_canary_count:$selected_count,
    preflight_only_non_selected_count:$preflight_only_count,
    all_readback_projections_bound_to_acceptance_preflight:($ready_count == ($entries | length) and $selected_count == 1 and (($selected_count + $preflight_only_count) == ($entries | length)) and $pending_count == $selected_count and $receipt_required_count == $selected_count and $readback_required_count == $selected_count and $record_write_blocked_count == $selected_count and $receipt_write_blocked_count == $selected_count),
    all_acceptance_preflight_entries_keep_no_invocation_guard:($entries | all(if (.canary_result_acceptance_preflight_route == "canary_result_acceptance_pending_evidence" or .canary_result_acceptance_preflight_route == "preflight_only_non_selected_candidate") then (.registry_guard_route == "require_approval_ledger" and .canary_result_receipt_present == false and .canary_readback_evidence_present == false and .operator_canary_result_acceptance_present == false and .canary_acceptance_record_written == false and .canary_acceptance_receipt_written == false and .live_cutover_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_canary_result_acceptance_preflight_ready:$acceptance_ready,
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
    result_receipt_written:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_live_cutover_preflight_without_invocation",
    entries:$entries,
    blockers:[
      "canary_result_receipt_absent",
      "canary_readback_evidence_absent",
      "operator_canary_result_acceptance_absent",
      "canary_acceptance_record_not_written",
      "canary_acceptance_receipt_not_written",
      "live_cutover_switch_disabled",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_live_cutover_preflight_without_invocation",
      "keep_canary_result_acceptance_preflight_read_only_until_receipts_and_operator_acceptance_are_explicit",
      "keep_live_cutover_invocation_ledger_approval_receipt_and_mutation_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_canary_result_acceptance_preflight.rs",
      canary_readback_receipt_projection_report:"scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-report.sh"
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
      canary_result_acceptance_recorded:false,
      canary_acceptance_receipt_written:false,
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
