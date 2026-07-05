#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_MINIMAL_READ_ONLY_INVOCATION_LEDGER_RECEIPT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable minimal read-only invocation ledger receipt report: $REPORT"
[[ -f "$DOC" ]] || fail "missing minimal read-only invocation ledger receipt architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the minimal read-only invocation ledger receipt report"
fi

rg -q 'Hepta Systems ToolRegistry Minimal Read-Only Invocation Ledger Receipt Readback' "$DOC" \
  || fail "architecture note must document the minimal read-only invocation ledger receipt readback"
rg -q 'selected hepta-system status read-only tool path' "$DOC" \
  || fail "architecture note must document the selected hepta-system status read-only tool path"
rg -q 'no tool write, ToolRegistry registration, registry mutation, ledger write, approval request, approval acceptance, receipt persistence, workflow event-log write, SQLite write, external network, credential read, provider/model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed side-effect boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_gate"
  and .schema_version == "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_workgraph_inventory_ready == true
  and .source_internal_invocation_ready == true
  and .source_operator_approval_protocol_ready == true
  and .source_dispatch_preflight_ready == true
  and .source_ledger_approval_preflight_ready == true
  and .source_receipt_projection_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .selected_candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
  and .non_selected_candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
  and .selected_minimal_path_count == 1
  and .selected_minimal_stage_count == 4
  and .non_selected_preflight_only_count == 1
  and .registry_lookup_preview_required_count == 1
  and .status_payload_projection_count == 1
  and .ledger_preview_required_count == 1
  and .approval_preflight_required_count == 1
  and .approval_packet_preview_count == 1
  and .receipt_projection_required_count == 1
  and .result_receipt_projected_in_memory_count == 1
  and .operator_protocol_step_count == 3
  and .explicit_accept_required == true
  and .non_acceptance_receipt_projected == true
  and .output_schema_validated == true
  and .minimal_read_only_invocation_ledger_receipt_readback_ready == true
  and .tool_invoked == false
  and .tool_invocation_switch_enabled == false
  and .registry_lookup_executed == false
  and .tool_registry_mutated == false
  and .ledger_written == false
  and .ledger_write_allowed == false
  and .approval_requested == false
  and .approval_request_allowed == false
  and .approval_accepted == false
  and .approval_acceptance_allowed == false
  and .approval_recorded == false
  and .receipt_persisted == false
  and .result_receipt_written == false
  and .external_network_allowed == false
  and .credential_read_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .native_post_mutation_allowed == false
  and .channel_send_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 5
  and any(.entries[]; .entry_id == "selected_registry_lookup_preview" and .registry_lookup_preview_required == true and .selected_for_minimal_path == true)
  and any(.entries[]; .entry_id == "selected_internal_status_payload_projection" and .status_payload_materialized == true and .selected_for_minimal_path == true)
  and any(.entries[]; .entry_id == "selected_ledger_approval_preflight" and .ledger_preview_required == true and .approval_preflight_required == true and .approval_packet_preview_ready == true and .selected_for_minimal_path == true)
  and any(.entries[]; .entry_id == "selected_result_receipt_projection" and .receipt_projection_required == true and .result_receipt_projected_in_memory == true and .non_acceptance_receipt_projected == true and .selected_for_minimal_path == true)
  and any(.entries[]; .entry_id == "non_selected_app_connector_preflight_only" and .preflight_only == true and .selected_for_minimal_path == false)
  and (.entries | all(.source_bound == true and .input_schema_validated == true and .output_schema_validated == true and .tool_invoked == false and .registry_lookup_executed == false and .tool_registry_mutated == false and .ledger_written == false and .approval_requested == false and .approval_accepted == false and .approval_recorded == false and .receipt_persisted == false and .result_receipt_written == false and .external_network_used == false and .credential_read == false and .workflow_event_log_written == false and .sqlite_written == false and .native_post_mutation_performed == false and .channel_send_performed == false and .live_execution_started == false))
  and (.blockers | index("tool_invocation_switch_disabled")) != null
  and (.blockers | index("registry_lookup_execution_disabled")) != null
  and (.blockers | index("tool_registry_mutation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("approval_acceptance_disabled")) != null
  and (.blockers | index("approval_recording_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("external_network_disabled")) != null
  and (.blockers | index("credential_read_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("native_post_mutation_disabled")) != null
  and (.blockers | index("channel_send_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback --lib
)

printf 'hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-gate: PASS: minimal read-only tool path binds lookup, ledger, approval, and receipt without writes or live execution\n'
