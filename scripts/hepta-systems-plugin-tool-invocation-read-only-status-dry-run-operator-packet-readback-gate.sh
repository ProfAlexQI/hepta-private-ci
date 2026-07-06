#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-packet-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_PACKET_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-packet-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable read-only status dry-run operator packet report: $REPORT"
[[ -f "$DOC" ]] || fail "missing read-only status dry-run operator packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the read-only status dry-run operator packet report"
fi

rg -q 'Hepta Systems Plugin Tool Invocation Read Only Status Dry Run Operator Packet Readback' "$DOC" \
  || fail "architecture note must document the read-only status dry-run operator packet readback"
rg -q 'operator packet ids, operator checklist ids, non-acceptance receipts, ledger preview links, receipt preview links, policy denial anchor links, approval denial anchor links, and operator packet idempotency keys' "$DOC" \
  || fail "architecture note must document operator packet, checklist, receipt, preview link, and idempotency projections"
rg -q 'no feature gate open, dry-run execution, operator packet send, operator packet persistence, operator checklist persistence, non-acceptance receipt persistence, operator acceptance recording, dry-run receipt preview persistence, ledger preview persistence, policy decision persistence, approval preflight execution, ledger write attempt, receipt projection persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, noop result persistence, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed operator packet boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_dry_run_receipt_ledger_preview_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .packet_entry_count == 2
  and .selected_read_only_status_tool_count == 1
  and .non_selected_preflight_boundary_count == 1
  and .operator_packet_id_projected_count == 2
  and .operator_checklist_projected_count == 2
  and .operator_checklist_item_count == 10
  and .non_acceptance_receipt_projected_count == 2
  and .ledger_preview_link_projected_count == 2
  and .receipt_preview_link_projected_count == 2
  and .policy_denial_anchor_link_projected_count == 2
  and .approval_denial_anchor_link_projected_count == 2
  and .idempotency_key_projected_count == 2
  and .stable_operator_packet_count == 2
  and .unique_operator_packet_count == 2
  and .stable_non_acceptance_receipt_count == 2
  and .unique_non_acceptance_receipt_count == 2
  and .stable_operator_packet_idempotency_key_count == 2
  and .unique_operator_packet_idempotency_key_count == 2
  and .operator_packet_mismatch_count == 0
  and .duplicate_operator_packet_count == 0
  and .non_acceptance_receipt_mismatch_count == 0
  and .duplicate_non_acceptance_receipt_count == 0
  and .operator_packet_idempotency_mismatch_count == 0
  and .duplicate_operator_packet_idempotency_key_count == 0
  and .feature_gate_opened_count == 0
  and .dry_run_executed_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .operator_checklist_persisted_count == 0
  and .non_acceptance_receipt_persisted_count == 0
  and .operator_acceptance_recorded_count == 0
  and .dry_run_receipt_preview_persisted_count == 0
  and .ledger_preview_persisted_count == 0
  and .policy_decision_persisted_count == 0
  and .approval_preflight_executed_count == 0
  and .ledger_write_attempted_count == 0
  and .receipt_projection_persisted_count == 0
  and .tool_registered_count == 0
  and .tool_registry_mutated_count == 0
  and .registry_lookup_executed_count == 0
  and .tool_invoked_count == 0
  and .noop_result_persisted_count == 0
  and .ledger_written_count == 0
  and .approval_requested_count == 0
  and .receipt_persisted_count == 0
  and .dynamic_activation_started_count == 0
  and .permission_granted_count == 0
  and .mcp_server_started_count == 0
  and .app_connector_started_count == 0
  and .plugin_installed_count == 0
  and .cache_materialized_count == 0
  and .cache_mutated_count == 0
  and .runtime_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_execution_started_count == 0
  and .operator_packet_readback_ready == true
  and .feature_gate_open_allowed == false
  and .dry_run_execution_allowed == false
  and .operator_packet_send_allowed == false
  and .operator_packet_persistence_allowed == false
  and .operator_checklist_persistence_allowed == false
  and .non_acceptance_receipt_persistence_allowed == false
  and .operator_acceptance_recording_allowed == false
  and .dry_run_receipt_preview_persistence_allowed == false
  and .ledger_preview_persistence_allowed == false
  and .policy_decision_persistence_allowed == false
  and .approval_preflight_execution_allowed == false
  and .ledger_write_allowed == false
  and .receipt_projection_persistence_allowed == false
  and .tool_registry_registration_allowed == false
  and .tool_registry_mutation_allowed == false
  and .registry_lookup_execution_allowed == false
  and .tool_invocation_allowed == false
  and .noop_result_persistence_allowed == false
  and .approval_request_allowed == false
  and .receipt_persistence_allowed == false
  and .dynamic_activation_allowed == false
  and .permission_grant_allowed == false
  and .mcp_server_start_allowed == false
  and .app_connector_start_allowed == false
  and .plugin_install_allowed == false
  and .plugin_cache_mutation_allowed == false
  and .install_cache_materialization_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .dry_run_path_selected == true and .operator_packet_id == "operator-packet:hepta-system:local-mcp:read-only-status-dry-run" and .operator_checklist_id == "operator-checklist:hepta-system:local-mcp:read-only-status-dry-run" and (.operator_checklist_items | length) == 5 and .non_acceptance_receipt_id == "operator-non-acceptance-receipt:hepta-system:local-mcp:read-only-status-dry-run" and .ledger_preview_link_id == "ledger-preview-link:hepta-system:local-mcp:dry-run-read-only-denied" and .receipt_preview_link_id == "receipt-preview-link:hepta-system:local-mcp:read-only-denied" and .policy_denial_anchor_link_id == "policy-denial-anchor-link:hepta-system:local-mcp:deny-no-invocation" and .approval_denial_anchor_link_id == "approval-denial-anchor-link:hepta-system:local-mcp:no-request" and .operator_packet_idempotency_key == "operator-packet-idempotency:hepta-system:local-mcp:read-only-status-dry-run")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .dry_run_path_selected == false and .operator_packet_id == "operator-packet:hepta-system:local-app:not-selected" and .operator_checklist_id == "operator-checklist:hepta-system:local-app:not-selected" and (.operator_checklist_items | length) == 5 and .non_acceptance_receipt_id == "operator-non-acceptance-receipt:hepta-system:local-app:not-selected" and .ledger_preview_link_id == "ledger-preview-link:hepta-system:local-app:not-selected" and .receipt_preview_link_id == "receipt-preview-link:hepta-system:local-app:not-selected" and .policy_denial_anchor_link_id == "policy-denial-anchor-link:hepta-system:local-app:deny-no-invocation" and .approval_denial_anchor_link_id == "approval-denial-anchor-link:hepta-system:local-app:no-request" and .operator_packet_idempotency_key == "operator-packet-idempotency:hepta-system:local-app:not-selected")
  and (.entries | all(.operator_packet_id_projected == true and .operator_checklist_projected == true and .non_acceptance_receipt_projected == true and .ledger_preview_link_projected == true and .receipt_preview_link_projected == true and .policy_denial_anchor_link_projected == true and .approval_denial_anchor_link_projected == true and .idempotency_key_projected == true and .stable_operator_packet == true and .unique_operator_packet == true and .stable_non_acceptance_receipt == true and .unique_non_acceptance_receipt == true and .stable_operator_packet_idempotency_key == true and .unique_operator_packet_idempotency_key == true and .feature_gate_opened == false and .dry_run_executed == false and .operator_packet_sent == false and .operator_packet_persisted == false and .operator_checklist_persisted == false and .non_acceptance_receipt_persisted == false and .operator_acceptance_recorded == false and .dry_run_receipt_preview_persisted == false and .ledger_preview_persisted == false and .policy_decision_persisted == false and .approval_preflight_executed == false and .ledger_write_attempted == false and .receipt_projection_persisted == false and .tool_registered == false and .tool_registry_mutated == false and .registry_lookup_executed == false and .tool_invoked == false and .noop_result_persisted == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .plugin_installed == false and .cache_materialized == false and .cache_mutated == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("operator_packet_send_disabled")) != null
  and (.blockers | index("operator_packet_persistence_disabled")) != null
  and (.blockers | index("operator_checklist_persistence_disabled")) != null
  and (.blockers | index("non_acceptance_receipt_persistence_disabled")) != null
  and (.blockers | index("operator_acceptance_recording_disabled")) != null
  and (.blockers | index("feature_gate_open_disabled")) != null
  and (.blockers | index("dry_run_execution_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback --lib
)

printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-packet-readback-gate: PASS: read-only status dry-run operator packets are queryable without feature gate open, dry-run execution, packet send/persistence, acceptance recording, ToolRegistry registration, lookup, invocation, ledger, receipt, runtime, or live mutation\n'
