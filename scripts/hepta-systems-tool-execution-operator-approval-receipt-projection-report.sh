#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-operator-approval-packet-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_operator_approval_receipt_projection.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_OPERATOR_APPROVAL_RECEIPT_PROJECTION_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-operator-approval-receipt-projection-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution operator approval packet report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution operator approval receipt projection Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution operator approval receipt projection architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution operator approval receipt projection report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_OPERATOR_APPROVAL_RECEIPT_PROJECTION_2026-06-21.md" \
  '
  def receipt_entry($entry):
    ($entry.operator_approval_packet_route == "operator_approval_packet_ready_for_review"
      and $entry.operator_approval_packet_ready == true
      and $entry.operator_review_required == true
      and $entry.approval_request_blocked == true) as $packet_ready |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$entry.execution_adapter_kind,
      source_operator_approval_packet_route:$entry.operator_approval_packet_route,
      registry_guard_route:$entry.registry_guard_route,
      operator_approval_receipt_projection_route:(if ($packet_ready and $approval_guard) then "operator_approval_receipt_projection_ready" elif $packet_ready then "blocked_by_registry_guard" else "blocked_by_operator_approval_packet" end),
      operator_approval_receipt_projection_ready:($packet_ready and $approval_guard),
      operator_decision_receipt_required:($packet_ready and $approval_guard),
      operator_decision_readback_evidence_required:($packet_ready and $approval_guard),
      operator_decision_receipt_write_blocked:($packet_ready and $approval_guard),
      operator_decision_receipt_projection_present:true,
      operator_decision_readback_evidence_slot_present:true,
      operator_decision_record_written:false,
      operator_decision_receipt_written:false,
      operator_acceptance_present:false,
      approval_request_sent:false,
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
  ($source.entries | map(receipt_entry(.))) as $entries |
  ($entries | map(select(.operator_approval_receipt_projection_ready == true)) | length) as $ready_count |
  ($entries | map(select(.operator_decision_receipt_required == true)) | length) as $receipt_required_count |
  ($entries | map(select(.operator_decision_readback_evidence_required == true)) | length) as $readback_required_count |
  ($entries | map(select(.operator_decision_receipt_write_blocked == true)) | length) as $write_blocked_count |
  ($source.tool_execution_operator_approval_packet_ready
    and $source.tool_execution_operator_approval_request_allowed == false
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $source.result_receipt_written == false
    and $ready_count == ($entries | length)
    and $receipt_required_count == ($entries | length)
    and $readback_required_count == ($entries | length)
    and $write_blocked_count == ($entries | length)
    and ($entries | all(.operator_decision_receipt_projection_present == true and .operator_decision_readback_evidence_slot_present == true))
    and ($entries | all(if .operator_approval_receipt_projection_route == "operator_approval_receipt_projection_ready" then (.registry_guard_route == "require_approval_ledger" and .operator_decision_record_written == false and .operator_decision_receipt_written == false and .operator_acceptance_present == false and .approval_request_sent == false and .live_cutover_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end))) as $projection_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_operator_approval_receipt_projection",
    plugin_id:$source.plugin_id,
    status:(if $projection_ready then "ready" else "blocked" end),
    source_operator_approval_packet_surface:$source.surface,
    source_operator_approval_packet_ready:$source.tool_execution_operator_approval_packet_ready,
    source_approval_request_allowed:$source.tool_execution_operator_approval_request_allowed,
    source_live_cutover_allowed:$source.tool_execution_live_cutover_allowed,
    operator_decision_receipt_projection_present:true,
    operator_decision_readback_evidence_slot_present:true,
    operator_decision_record_written:false,
    operator_decision_receipt_written:false,
    operator_acceptance_present:false,
    approval_request_sent:false,
    live_cutover_switch_enabled:false,
    candidate_count:($entries | length),
    operator_approval_receipt_projection_ready_count:$ready_count,
    operator_approval_receipt_projection_blocked_count:(($entries | length) - $ready_count),
    operator_decision_receipt_required_count:$receipt_required_count,
    operator_decision_readback_evidence_required_count:$readback_required_count,
    operator_decision_receipt_write_blocked_count:$write_blocked_count,
    all_operator_packets_bound_to_receipt_projection:($ready_count == ($entries | length) and $receipt_required_count == ($entries | length) and $readback_required_count == ($entries | length) and $write_blocked_count == ($entries | length)),
    all_operator_receipt_projections_keep_approval_guard:($entries | all(if .operator_approval_receipt_projection_route == "operator_approval_receipt_projection_ready" then (.registry_guard_route == "require_approval_ledger" and .operator_decision_record_written == false and .operator_decision_receipt_written == false and .operator_acceptance_present == false and .approval_request_sent == false and .live_cutover_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .execution_adapter_dispatch_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_operator_approval_receipt_projection_ready:$projection_ready,
    tool_execution_operator_decision_write_allowed:false,
    tool_execution_live_cutover_allowed:false,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    execution_adapter_dispatched:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    operator_decision_record_written_flag:false,
    operator_decision_receipt_written_flag:false,
    result_receipt_written:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_canary_cutover_plan_without_invocation",
    entries:$entries,
    blockers:[
      "operator_decision_record_not_written",
      "operator_decision_receipt_not_written",
      "operator_acceptance_absent",
      "approval_request_not_sent",
      "live_cutover_switch_disabled",
      "router_registration_lookup_disabled",
      "registry_lookup_execution_disabled",
      "registry_source_of_truth_enablement_disabled",
      "tool_registration_disabled",
      "execution_adapter_dispatch_disabled",
      "tool_invocation_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_canary_cutover_plan_without_invocation",
      "keep_operator_approval_receipt_projection_read_only_until_decision_preflight_is_restored",
      "keep_decision_record_receipt_invocation_and_live_mutation_disabled_until_explicit_cutover"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_operator_approval_receipt_projection.rs",
      operator_approval_packet_report:"scripts/hepta-systems-tool-execution-operator-approval-packet-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_cache_mutated:false,
      plugin_installed:false,
      manifest_rewritten:false,
      manifest_schema_written:false,
      registry_source_of_truth_enabled:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registration_cutover_executed:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_decision_record_written:false,
      operator_decision_receipt_written:false,
      operator_acceptance_recorded:false,
      result_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      local_storage_created:false,
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
