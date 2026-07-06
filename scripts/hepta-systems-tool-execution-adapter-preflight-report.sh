#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-invocation-receipt-projection-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_execution_adapter_preflight.rs"
TOOL_EXECUTOR_SOURCE="$ROOT/codex-rs/tools/src/tool_executor.rs"
GATE="$ROOT/scripts/hepta-systems-tool-execution-adapter-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_ADAPTER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-adapter-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable receipt projection report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing execution adapter preflight Rust source: $RUST_SOURCE"
[[ -f "$TOOL_EXECUTOR_SOURCE" ]] || fail "missing ToolExecutor source: $TOOL_EXECUTOR_SOURCE"
[[ -f "$DOC" ]] || fail "missing execution adapter preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the execution adapter preflight report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-adapter-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_ADAPTER_PREFLIGHT_2026-06-21.md" \
  '
  def adapter_kind($kind):
    if $kind == "mcp_server" then "mcp_tool_call_adapter"
    elif $kind == "app_connector" then "app_connector_invocation_adapter"
    else "unknown_execution_adapter"
    end;

  def adapter_entry($entry):
    ($entry.receipt_projection_route == "result_receipt_projection_required" and $entry.receipt_projection_ready == true) as $projection_ready |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    (adapter_kind($entry.contribution_kind)) as $adapter_kind |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      execution_adapter_kind:$adapter_kind,
      source_receipt_projection_route:$entry.receipt_projection_route,
      registry_guard_route:$entry.registry_guard_route,
      adapter_preflight_route:(if ($projection_ready and $approval_guard and $adapter_kind != "unknown_execution_adapter") then "disabled_execution_adapter_preflight" elif ($projection_ready and $approval_guard) then "blocked_by_unknown_adapter_kind" elif $projection_ready then "blocked_by_registry_guard" else "blocked_by_receipt_projection" end),
      execution_adapter_preflight_ready:($projection_ready and $approval_guard and $adapter_kind != "unknown_execution_adapter"),
      receipt_projection_ready:$entry.receipt_projection_ready,
      result_receipt_required:$entry.result_receipt_required,
      readback_evidence_required:$entry.readback_evidence_required,
      execution_adapter_binding_present:true,
      tool_invocation_execution_switch_enabled:false,
      adapter_dispatch_switch_enabled:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registry_source_of_truth_enabled:false,
      tool_registration_enabled:false,
      tool_invocation_enabled:false,
      ledger_write_enabled:false,
      approval_request_enabled:false,
      result_receipt_write_enabled:false,
      side_effect_free:true
    };

  ($source[0]) as $source |
  ($source.entries | map(adapter_entry(.))) as $entries |
  ($entries | map(select(.execution_adapter_preflight_ready == true)) | length) as $ready_count |
  ($entries | map(select(.adapter_preflight_route == "disabled_execution_adapter_preflight")) | length) as $disabled_count |
  ($entries | map(select(.execution_adapter_kind == "mcp_tool_call_adapter")) | length) as $mcp_count |
  ($entries | map(select(.execution_adapter_kind == "app_connector_invocation_adapter")) | length) as $app_count |
  ($source.tool_invocation_receipt_projection_ready
    and $source.tool_invocation_enabled == false
    and $source.ledger_written == false
    and $source.approval_requested == false
    and $source.result_receipt_written == false
    and $ready_count == ($entries | length)
    and $disabled_count == ($entries | length)
    and $mcp_count == 1
    and $app_count == 1
    and ($entries | all(.execution_adapter_binding_present == true))
    and ($entries | all(if .adapter_preflight_route == "disabled_execution_adapter_preflight" then (.registry_guard_route == "require_approval_ledger" and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end))) as $adapter_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_adapter_preflight",
    plugin_id:$source.plugin_id,
    status:(if $adapter_ready then "ready" else "blocked" end),
    source_receipt_projection_surface:$source.surface,
    source_receipt_projection_ready:$source.tool_invocation_receipt_projection_ready,
    execution_adapter_binding_present:true,
    tool_invocation_execution_switch_enabled:false,
    adapter_dispatch_switch_enabled:false,
    candidate_count:($entries | length),
    execution_adapter_preflight_ready_count:$ready_count,
    execution_adapter_preflight_blocked_count:(($entries | length) - $ready_count),
    disabled_execution_adapter_preflight_count:$disabled_count,
    mcp_tool_call_adapter_preflight_count:$mcp_count,
    app_connector_invocation_adapter_preflight_count:$app_count,
    all_receipt_projection_entries_bound_to_execution_adapter_preflight:($ready_count == ($entries | length) and $disabled_count == ($entries | length) and $mcp_count == 1 and $app_count == 1 and ($entries | all(.execution_adapter_binding_present == true))),
    all_execution_adapter_entries_keep_approval_guard:($entries | all(if .adapter_preflight_route == "disabled_execution_adapter_preflight" then (.registry_guard_route == "require_approval_ledger" and .tool_invocation_execution_switch_enabled == false and .adapter_dispatch_switch_enabled == false and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false and .result_receipt_write_enabled == false) else true end)),
    tool_execution_adapter_preflight_ready:$adapter_ready,
    execution_adapter_preflight_allowed:$adapter_ready,
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    result_receipt_written:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_execution_cutover_preflight_without_invocation",
    entries:$entries,
    blockers:[
      "router_registration_lookup_disabled",
      "registry_lookup_execution_disabled",
      "registry_source_of_truth_enablement_disabled",
      "tool_registration_disabled",
      "tool_invocation_disabled",
      "execution_adapter_dispatch_disabled",
      "tool_invocation_ledger_write_disabled",
      "approval_broker_request_disabled",
      "result_receipt_write_disabled"
    ],
    next_actions:[
      "restore_tool_execution_cutover_preflight_without_invocation",
      "keep_execution_adapter_preflight_read_only_until_cutover_preflight_is_restored",
      "keep_registration_invocation_ledger_approval_receipts_and_live_mutation_disabled_until_explicit_cutover"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_execution_adapter_preflight.rs",
      receipt_projection_report:"scripts/hepta-systems-tool-invocation-receipt-projection-report.sh",
      tool_executor_runtime:"codex-rs/tools/src/tool_executor.rs"
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
