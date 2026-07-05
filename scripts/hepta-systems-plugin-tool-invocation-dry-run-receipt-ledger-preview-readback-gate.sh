#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-dry-run-receipt-ledger-preview-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_DRY_RUN_RECEIPT_LEDGER_PREVIEW_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-dry-run-receipt-ledger-preview-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable plugin tool invocation dry-run receipt ledger preview report: $REPORT"
[[ -f "$DOC" ]] || fail "missing plugin tool invocation dry-run receipt ledger preview architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool invocation dry-run receipt ledger preview report"
fi

rg -q 'Hepta Systems Plugin Tool Invocation Dry Run Receipt Ledger Preview Readback' "$DOC" \
  || fail "architecture note must document the plugin tool invocation dry-run receipt ledger preview readback"
rg -q 'dry-run receipt preview ids, dry-run receipt preview digests, ledger preview ids, ledger preview digests, policy denial anchors, approval denial anchors, receipt projection anchors, stable preview receipts, and preview idempotency keys' "$DOC" \
  || fail "architecture note must document dry-run receipt, ledger preview, anchor, and idempotency projections"
rg -q 'no feature gate open, dry-run execution, dry-run receipt preview persistence, ledger preview persistence, policy decision persistence, approval preflight execution, ledger write attempt, receipt projection persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, noop result persistence, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed dry-run receipt ledger preview boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_feature_gated_dry_run_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .preview_entry_count == 2
  and .selected_read_only_status_tool_count == 1
  and .non_selected_preflight_boundary_count == 1
  and .dry_run_receipt_preview_id_projected_count == 2
  and .dry_run_receipt_preview_digest_projected_count == 2
  and .ledger_preview_id_projected_count == 2
  and .ledger_preview_digest_projected_count == 2
  and .policy_denial_anchor_projected_count == 2
  and .approval_denial_anchor_projected_count == 2
  and .receipt_projection_anchor_projected_count == 2
  and .dry_run_idempotency_anchor_projected_count == 2
  and .stable_preview_receipt_count == 2
  and .unique_preview_receipt_count == 2
  and .preview_idempotency_key_projected_count == 2
  and .stable_preview_idempotency_key_count == 2
  and .unique_preview_idempotency_key_count == 2
  and .preview_receipt_mismatch_count == 0
  and .duplicate_preview_receipt_count == 0
  and .preview_idempotency_key_mismatch_count == 0
  and .duplicate_preview_idempotency_key_count == 0
  and .feature_gate_opened_count == 0
  and .dry_run_executed_count == 0
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
  and .dry_run_receipt_ledger_preview_readback_ready == true
  and .feature_gate_open_allowed == false
  and .dry_run_execution_allowed == false
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
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .dry_run_path_selected == true and .dry_run_receipt_preview_id == "dry-run-receipt-preview:hepta-system:local-mcp:read-only-denied" and .dry_run_receipt_preview_digest == "dry-run-receipt-preview-digest:hepta-system:local-mcp:read-only-denied" and .ledger_preview_id == "ledger-preview:hepta-system:local-mcp:dry-run-read-only-denied" and .ledger_preview_digest == "ledger-preview-digest:hepta-system:local-mcp:dry-run-read-only-denied" and .policy_denial_anchor_id == "policy-denial-anchor:hepta-system:local-mcp:deny-no-invocation" and .approval_denial_anchor_id == "approval-denial-anchor:hepta-system:local-mcp:no-request" and .receipt_projection_anchor_id == "receipt-projection-anchor:hepta-system:local-mcp:no-persistence" and .dry_run_idempotency_anchor_id == "dry-run-idempotency-anchor:hepta-system:local-mcp:read-only-denied" and .first_preview_receipt_id == "dry-run-ledger-preview-receipt:hepta-system:local-mcp:read-only-denied" and .second_preview_receipt_id == "dry-run-ledger-preview-receipt:hepta-system:local-mcp:read-only-denied" and .first_preview_idempotency_key == "dry-run-ledger-preview-idempotency:hepta-system:local-mcp:read-only-denied" and .second_preview_idempotency_key == "dry-run-ledger-preview-idempotency:hepta-system:local-mcp:read-only-denied")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .dry_run_path_selected == false and .dry_run_receipt_preview_id == "dry-run-receipt-preview:hepta-system:local-app:not-selected" and .dry_run_receipt_preview_digest == "dry-run-receipt-preview-digest:hepta-system:local-app:not-selected" and .ledger_preview_id == "ledger-preview:hepta-system:local-app:not-selected" and .ledger_preview_digest == "ledger-preview-digest:hepta-system:local-app:not-selected" and .policy_denial_anchor_id == "policy-denial-anchor:hepta-system:local-app:deny-no-invocation" and .approval_denial_anchor_id == "approval-denial-anchor:hepta-system:local-app:no-request" and .receipt_projection_anchor_id == "receipt-projection-anchor:hepta-system:local-app:no-persistence" and .dry_run_idempotency_anchor_id == "dry-run-idempotency-anchor:hepta-system:local-app:not-selected" and .first_preview_receipt_id == "dry-run-ledger-preview-receipt:hepta-system:local-app:not-selected" and .second_preview_receipt_id == "dry-run-ledger-preview-receipt:hepta-system:local-app:not-selected" and .first_preview_idempotency_key == "dry-run-ledger-preview-idempotency:hepta-system:local-app:not-selected" and .second_preview_idempotency_key == "dry-run-ledger-preview-idempotency:hepta-system:local-app:not-selected")
  and (.entries | all(.dry_run_receipt_preview_id_projected == true and .dry_run_receipt_preview_digest_projected == true and .ledger_preview_id_projected == true and .ledger_preview_digest_projected == true and .policy_denial_anchor_projected == true and .approval_denial_anchor_projected == true and .receipt_projection_anchor_projected == true and .dry_run_idempotency_anchor_projected == true and .preview_idempotency_key_projected == true and .stable_preview_receipt == true and .unique_preview_receipt == true and .stable_preview_idempotency_key == true and .unique_preview_idempotency_key == true and .feature_gate_opened == false and .dry_run_executed == false and .dry_run_receipt_preview_persisted == false and .ledger_preview_persisted == false and .policy_decision_persisted == false and .approval_preflight_executed == false and .ledger_write_attempted == false and .receipt_projection_persisted == false and .tool_registered == false and .tool_registry_mutated == false and .registry_lookup_executed == false and .tool_invoked == false and .noop_result_persisted == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .plugin_installed == false and .cache_materialized == false and .cache_mutated == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("feature_gate_open_disabled")) != null
  and (.blockers | index("dry_run_execution_disabled")) != null
  and (.blockers | index("dry_run_receipt_preview_persistence_disabled")) != null
  and (.blockers | index("ledger_preview_persistence_disabled")) != null
  and (.blockers | index("policy_decision_persistence_disabled")) != null
  and (.blockers | index("approval_preflight_execution_disabled")) != null
  and (.blockers | index("ledger_write_attempt_disabled")) != null
  and (.blockers | index("receipt_projection_persistence_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("dynamic_activation_disabled")) != null
  and (.blockers | index("plugin_install_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback --lib
)

printf 'hepta-systems-plugin-tool-invocation-dry-run-receipt-ledger-preview-readback-gate: PASS: dry-run receipt and ledger preview projections are stable without feature gate open, dry-run execution, ToolRegistry registration, lookup, invocation, ledger, receipt, runtime, or live mutation\n'
