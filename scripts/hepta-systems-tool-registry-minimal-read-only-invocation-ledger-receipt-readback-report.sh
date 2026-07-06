#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
WORKGRAPH_INVENTORY_REPORT="$ROOT/scripts/hepta-systems-workgraph-legacy-gate-recursion-inventory-readback-report.sh"
INTERNAL_INVOCATION_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh"
OPERATOR_PROTOCOL_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh"
DISPATCH_PREFLIGHT_REPORT="$ROOT/scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh"
LEDGER_APPROVAL_REPORT="$ROOT/scripts/hepta-systems-tool-invocation-ledger-approval-preflight-report.sh"
RECEIPT_PROJECTION_REPORT="$ROOT/scripts/hepta-systems-tool-invocation-receipt-projection-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$WORKGRAPH_INVENTORY_REPORT" ]] || fail "missing executable WorkGraph inventory report: $WORKGRAPH_INVENTORY_REPORT"
[[ -x "$INTERNAL_INVOCATION_REPORT" ]] || fail "missing executable internal invocation report: $INTERNAL_INVOCATION_REPORT"
[[ -x "$OPERATOR_PROTOCOL_REPORT" ]] || fail "missing executable operator protocol report: $OPERATOR_PROTOCOL_REPORT"
[[ -x "$DISPATCH_PREFLIGHT_REPORT" ]] || fail "missing executable dispatch preflight report: $DISPATCH_PREFLIGHT_REPORT"
[[ -x "$LEDGER_APPROVAL_REPORT" ]] || fail "missing executable ledger approval report: $LEDGER_APPROVAL_REPORT"
[[ -x "$RECEIPT_PROJECTION_REPORT" ]] || fail "missing executable receipt projection report: $RECEIPT_PROJECTION_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing minimal invocation ledger receipt Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing minimal invocation ledger receipt architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the minimal read-only invocation ledger receipt readback report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$WORKGRAPH_INVENTORY_REPORT" >"$tmpdir/workgraph.json" || fail "failed to render WorkGraph inventory report"
"$INTERNAL_INVOCATION_REPORT" >"$tmpdir/internal.json" || fail "failed to render internal invocation report"
"$OPERATOR_PROTOCOL_REPORT" >"$tmpdir/operator.json" || fail "failed to render operator protocol report"
"$DISPATCH_PREFLIGHT_REPORT" >"$tmpdir/dispatch.json" || fail "failed to render dispatch preflight report"
"$LEDGER_APPROVAL_REPORT" >"$tmpdir/ledger.json" || fail "failed to render ledger approval report"
"$RECEIPT_PROJECTION_REPORT" >"$tmpdir/receipt.json" || fail "failed to render receipt projection report"

jq -e . "$tmpdir/workgraph.json" >/dev/null || fail "WorkGraph inventory report did not render valid JSON"
jq -e . "$tmpdir/internal.json" >/dev/null || fail "internal invocation report did not render valid JSON"
jq -e . "$tmpdir/operator.json" >/dev/null || fail "operator protocol report did not render valid JSON"
jq -e . "$tmpdir/dispatch.json" >/dev/null || fail "dispatch preflight report did not render valid JSON"
jq -e . "$tmpdir/ledger.json" >/dev/null || fail "ledger approval report did not render valid JSON"
jq -e . "$tmpdir/receipt.json" >/dev/null || fail "receipt projection report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile workgraph "$tmpdir/workgraph.json" \
  --slurpfile internal "$tmpdir/internal.json" \
  --slurpfile operator "$tmpdir/operator.json" \
  --slurpfile dispatch "$tmpdir/dispatch.json" \
  --slurpfile ledger "$tmpdir/ledger.json" \
  --slurpfile receipt "$tmpdir/receipt.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_2026-06-30.md" \
  '
  def selected_id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
  def non_selected_id: "preview:connector:hepta-system@hepta-local:hepta_system_local_app";
  def by_id($items; $id): ($items[] | select(.candidate_tool_id == $id));
  def selected_stage($id; $stage; $route; $lookup; $status_payload; $ledger_preview; $approval; $receipt_projection): {
    entry_id:$id,
    candidate_tool_id:selected_id,
    contribution_kind:"mcp_server",
    contract_stage:$stage,
    route:$route,
    selected_for_minimal_path:true,
    preflight_only:false,
    source_bound:true,
    input_schema_validated:true,
    output_schema_validated:true,
    status_payload_materialized:$status_payload,
    registry_lookup_preview_required:$lookup,
    ledger_preview_required:$ledger_preview,
    approval_preflight_required:$approval,
    approval_packet_preview_ready:$approval,
    receipt_projection_required:$receipt_projection,
    result_receipt_projected_in_memory:$receipt_projection,
    non_acceptance_receipt_projected:$receipt_projection,
    tool_invoked:false,
    registry_lookup_executed:false,
    tool_registry_mutated:false,
    ledger_written:false,
    approval_requested:false,
    approval_accepted:false,
    approval_recorded:false,
    receipt_persisted:false,
    result_receipt_written:false,
    external_network_used:false,
    credential_read:false,
    workflow_event_log_written:false,
    sqlite_written:false,
    native_post_mutation_performed:false,
    channel_send_performed:false,
    live_execution_started:false
  };
  def non_selected_stage: {
    entry_id:"non_selected_app_connector_preflight_only",
    candidate_tool_id:non_selected_id,
    contribution_kind:"app_connector",
    contract_stage:"non_selected_preflight_only",
    route:"tool-registry://hepta-system/status/read-only/non-selected-app-preflight",
    selected_for_minimal_path:false,
    preflight_only:true,
    source_bound:true,
    input_schema_validated:true,
    output_schema_validated:true,
    status_payload_materialized:false,
    registry_lookup_preview_required:false,
    ledger_preview_required:false,
    approval_preflight_required:false,
    approval_packet_preview_ready:false,
    receipt_projection_required:false,
    result_receipt_projected_in_memory:false,
    non_acceptance_receipt_projected:false,
    tool_invoked:false,
    registry_lookup_executed:false,
    tool_registry_mutated:false,
    ledger_written:false,
    approval_requested:false,
    approval_accepted:false,
    approval_recorded:false,
    receipt_persisted:false,
    result_receipt_written:false,
    external_network_used:false,
    credential_read:false,
    workflow_event_log_written:false,
    sqlite_written:false,
    native_post_mutation_performed:false,
    channel_send_performed:false,
    live_execution_started:false
  };
  ($workgraph[0]) as $workgraph_report |
  ($internal[0]) as $internal_report |
  ($operator[0]) as $operator_report |
  ($dispatch[0]) as $dispatch_report |
  ($ledger[0]) as $ledger_report |
  ($receipt[0]) as $receipt_report |
  (by_id($dispatch_report.entries; selected_id)) as $selected_dispatch |
  (by_id($dispatch_report.entries; non_selected_id)) as $non_selected_dispatch |
  (by_id($ledger_report.entries; selected_id)) as $selected_ledger |
  (by_id($receipt_report.entries; selected_id)) as $selected_receipt |
  [
    selected_stage("selected_registry_lookup_preview"; "registry_lookup_preview"; "tool-registry://hepta-system/status/read-only/lookup-preview"; true; false; false; false; false),
    selected_stage("selected_internal_status_payload_projection"; "internal_status_payload_projection"; "internal://hepta-system/status/read-only"; false; true; false; false; false),
    selected_stage("selected_ledger_approval_preflight"; "ledger_approval_preflight"; "approval-ledger://hepta-system/status/read-only/preflight"; false; false; true; true; false),
    selected_stage("selected_result_receipt_projection"; "result_receipt_projection"; "receipt://hepta-system/status/read-only/result-projection"; false; false; false; false; true),
    non_selected_stage
  ] as $entries |
  ($entries | map(select(.selected_for_minimal_path == true)) | length) as $selected_minimal_stage_count |
  ($entries | map(select(.preflight_only == true)) | length) as $non_selected_preflight_only_count |
  ($entries | map(select(.registry_lookup_preview_required == true)) | length) as $registry_lookup_preview_required_count |
  ($entries | map(select(.status_payload_materialized == true)) | length) as $status_payload_projection_count |
  ($entries | map(select(.ledger_preview_required == true)) | length) as $ledger_preview_required_count |
  ($entries | map(select(.approval_preflight_required == true)) | length) as $approval_preflight_required_count |
  ($entries | map(select(.approval_packet_preview_ready == true)) | length) as $approval_packet_preview_count |
  ($entries | map(select(.receipt_projection_required == true)) | length) as $receipt_projection_required_count |
  ($entries | map(select(.result_receipt_projected_in_memory == true)) | length) as $result_receipt_projected_in_memory_count |
  ($workgraph_report.workgraph_legacy_gate_recursion_inventory_ready == true
    and $internal_report.internal_read_only_invocation_ready == true
    and $internal_report.candidate_count == 2
    and $internal_report.invocation_entry_count == 1
    and $internal_report.selected_candidate_tool_id == selected_id
    and $internal_report.non_selected_candidate_tool_id == non_selected_id
    and $internal_report.status_payload_materialized == true
    and $internal_report.output_schema_validated == true
    and $internal_report.receipt_projected_in_memory == true
    and $internal_report.receipt_persisted == false
    and $operator_report.approval_protocol_ready == true
    and $operator_report.approval_packet_count == 1
    and $operator_report.protocol_step_count == 3
    and $operator_report.selected_candidate_tool_id == selected_id
    and $operator_report.explicit_accept_required == true
    and $operator_report.non_acceptance_receipt_projected == true
    and $operator_report.approval_request_sent == false
    and $operator_report.approval_request_allowed == false
    and $operator_report.approval_accepted == false
    and $operator_report.approval_acceptance_allowed == false
    and $operator_report.approval_recorded == false
    and $operator_report.approval_broker_write_allowed == false
    and $operator_report.receipt_persisted == false
    and $dispatch_report.read_only_dispatch_preflight_ready == true
    and $dispatch_report.candidate_count == 2
    and $dispatch_report.dispatch_preflight_ready_count == 2
    and $dispatch_report.registry_lookup_preview_required_count == 2
    and $dispatch_report.ledger_preview_required_count == 2
    and $dispatch_report.approval_preflight_required_count == 2
    and $dispatch_report.receipt_projection_required_count == 2
    and $selected_dispatch.dispatch_preflight_ready == true
    and $selected_dispatch.tool_invocation_enabled == false
    and $selected_dispatch.ledger_write_enabled == false
    and $selected_dispatch.approval_request_enabled == false
    and $selected_dispatch.result_receipt_write_enabled == false
    and $non_selected_dispatch.dispatch_preflight_ready == true
    and $ledger_report.tool_invocation_ledger_approval_preflight_ready == true
    and $selected_ledger.ledger_preflight_ready == true
    and $selected_ledger.approval_preflight_required == true
    and $selected_ledger.ledger_write_enabled == false
    and $selected_ledger.approval_request_enabled == false
    and $receipt_report.tool_invocation_receipt_projection_ready == true
    and $selected_receipt.receipt_projection_ready == true
    and $selected_receipt.result_receipt_write_enabled == false
    and $lib_export_present == true
    and ($entries | length) == 5
    and $selected_minimal_stage_count == 4
    and $non_selected_preflight_only_count == 1
    and $registry_lookup_preview_required_count == 1
    and $status_payload_projection_count == 1
    and $ledger_preview_required_count == 1
    and $approval_preflight_required_count == 1
    and $approval_packet_preview_count == 1
    and $receipt_projection_required_count == 1
    and $result_receipt_projected_in_memory_count == 1
    and ($entries | all(.source_bound == true
      and .input_schema_validated == true
      and .output_schema_validated == true
      and .tool_invoked == false
      and .registry_lookup_executed == false
      and .tool_registry_mutated == false
      and .ledger_written == false
      and .approval_requested == false
      and .approval_accepted == false
      and .approval_recorded == false
      and .receipt_persisted == false
      and .result_receipt_written == false
      and .external_network_used == false
      and .credential_read == false
      and .workflow_event_log_written == false
      and .sqlite_written == false
      and .native_post_mutation_performed == false
      and .channel_send_performed == false
      and .live_execution_started == false))) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_gate",
    schema_version:"hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_v1",
    plugin_id:$internal_report.plugin_id,
    source_workgraph_inventory_ready:$workgraph_report.workgraph_legacy_gate_recursion_inventory_ready,
    source_internal_invocation_ready:$internal_report.internal_read_only_invocation_ready,
    source_operator_approval_protocol_ready:$operator_report.approval_protocol_ready,
    source_dispatch_preflight_ready:$dispatch_report.read_only_dispatch_preflight_ready,
    source_ledger_approval_preflight_ready:$ledger_report.tool_invocation_ledger_approval_preflight_ready,
    source_receipt_projection_ready:$receipt_report.tool_invocation_receipt_projection_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$internal_report.candidate_count,
    selected_candidate_tool_id:selected_id,
    non_selected_candidate_tool_id:non_selected_id,
    selected_minimal_path_count:1,
    selected_minimal_stage_count:$selected_minimal_stage_count,
    non_selected_preflight_only_count:$non_selected_preflight_only_count,
    registry_lookup_preview_required_count:$registry_lookup_preview_required_count,
    status_payload_projection_count:$status_payload_projection_count,
    ledger_preview_required_count:$ledger_preview_required_count,
    approval_preflight_required_count:$approval_preflight_required_count,
    approval_packet_preview_count:$approval_packet_preview_count,
    receipt_projection_required_count:$receipt_projection_required_count,
    result_receipt_projected_in_memory_count:$result_receipt_projected_in_memory_count,
    operator_protocol_step_count:$operator_report.protocol_step_count,
    explicit_accept_required:$operator_report.explicit_accept_required,
    non_acceptance_receipt_projected:$operator_report.non_acceptance_receipt_projected,
    output_schema_validated:$internal_report.output_schema_validated,
    minimal_read_only_invocation_ledger_receipt_readback_ready:$ready,
    tool_invoked:false,
    tool_invocation_switch_enabled:false,
    registry_lookup_executed:false,
    tool_registry_mutated:false,
    ledger_written:false,
    ledger_write_allowed:false,
    approval_requested:false,
    approval_request_allowed:false,
    approval_accepted:false,
    approval_acceptance_allowed:false,
    approval_recorded:false,
    receipt_persisted:false,
    result_receipt_written:false,
    external_network_allowed:false,
    credential_read_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    native_post_mutation_allowed:false,
    channel_send_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "tool_invocation_switch_disabled",
      "registry_lookup_execution_disabled",
      "tool_registry_mutation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled",
      "approval_acceptance_disabled",
      "approval_recording_disabled",
      "receipt_persistence_disabled",
      "external_network_disabled",
      "credential_read_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "native_post_mutation_disabled",
      "channel_send_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      tool_registry_mutated:false,
      registry_lookup_executed:false,
      tool_invoked:false,
      ledger_written:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      approval_broker_written:false,
      receipt_persisted:false,
      result_receipt_written:false,
      credential_read:false,
      external_network_used:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      package_or_release_written:false,
      canary_activated:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }
  '
