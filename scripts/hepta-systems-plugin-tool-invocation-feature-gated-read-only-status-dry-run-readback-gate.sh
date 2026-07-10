#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable plugin tool invocation feature-gated dry-run report: $REPORT"
[[ -f "$DOC" ]] || fail "missing plugin tool invocation feature-gated dry-run architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the plugin tool invocation feature-gated dry-run report"
fi

rg -q 'Hepta Systems Plugin Tool Invocation Feature Gated Read Only Status Dry Run Readback' "$DOC" \
  || fail "architecture note must document the plugin tool invocation feature-gated dry-run readback"
rg -q 'feature gate ids, closed feature gates, selected read-only status dry-run payloads, dry-run result projections, policy denials, receipt projections, stable dry-run receipts, and idempotency keys' "$DOC" \
  || fail "architecture note must document feature gate, dry-run, policy, receipt, and idempotency projections"
rg -q 'registration-denial query hit, shadow lookup projection, internal status payload projection, structured result projection, approval/ledger/receipt projection, and local append-only-store projection' "$DOC" \
  || fail "architecture note must document the read-only status dry-run path proof"
rg -q 'no feature gate open, dry-run execution, dry-run payload persistence, dry-run result persistence, policy decision persistence, approval preflight execution, ledger write attempt, receipt projection persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, noop result persistence, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed feature-gated dry-run boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_policy_approval_ledger_boundary_ready == true
  and .source_registration_denial_query_api_ready == true
  and .source_tool_registry_shadow_lookup_ready == true
  and .source_internal_read_only_invocation_ready == true
  and .source_minimal_ledger_receipt_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .dry_run_entry_count == 2
  and .selected_read_only_status_tool_count == 1
  and .non_selected_preflight_boundary_count == 1
  and .registration_denial_query_hit_count == 2
  and .shadow_lookup_projection_attached_count == 2
  and .internal_status_payload_projection_attached_count == 1
  and .internal_call_dry_run_projected_count == 1
  and .structured_result_projection_attached_count == 1
  and .approval_ledger_receipt_projection_attached_count == 1
  and .local_append_only_store_projection_attached_count == 1
  and .selected_dry_run_path_proof_count == 1
  and .feature_gate_id_projected_count == 2
  and .feature_gate_closed_count == 2
  and .dry_run_payload_projected_count == 1
  and .dry_run_payload_digest_projected_count == 1
  and .dry_run_result_projection_count == 1
  and .policy_denial_projected_count == 2
  and .receipt_projection_count == 2
  and .stable_dry_run_receipt_count == 2
  and .unique_dry_run_receipt_count == 2
  and .idempotency_key_projected_count == 2
  and .stable_idempotency_key_count == 2
  and .unique_idempotency_key_count == 2
  and .dry_run_receipt_mismatch_count == 0
  and .duplicate_dry_run_receipt_count == 0
  and .idempotency_key_mismatch_count == 0
  and .duplicate_idempotency_key_count == 0
  and .feature_gate_opened_count == 0
  and .dry_run_executed_count == 0
  and .dry_run_payload_persisted_count == 0
  and .dry_run_result_persisted_count == 0
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
  and .feature_gated_read_only_status_dry_run_readback_ready == true
  and .feature_gated_read_only_status_dry_run_path_proof_ready == true
  and .feature_gate_open_allowed == false
  and .dry_run_execution_allowed == false
  and .dry_run_payload_persistence_allowed == false
  and .dry_run_result_persistence_allowed == false
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
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .dry_run_path_selected == true and .dry_run_selection_reason == "selected_mcp_status_read_only_path" and .feature_gate_id == "feature-gate:hepta-system:local-mcp:status-dry-run" and .feature_gate_state == "closed" and (.source_registration_denial_id | startswith("registration-denial:hepta-system:")) and (.source_shadow_lookup_result_id | startswith("shadow-lookup-result:hepta-system:")) and .source_internal_status_request_id == "hepta-system.status.internal-read-only.v1" and .source_status_payload_fingerprint == "hepta-system-status.internal-read-only.v1.e2e4.fixture9.live0" and .source_minimal_receipt_stage_id == "selected_result_receipt_projection" and .registration_denial_query_hit == true and .shadow_lookup_projection_attached == true and .internal_status_payload_projection_attached == true and .internal_call_dry_run_projected == true and .structured_result_projection_attached == true and .approval_ledger_receipt_projection_attached == true and .local_append_only_store_projection_attached == true and .selected_dry_run_path_proof == true and .dry_run_request_id == "dry-run-request:hepta-system:local-mcp:status-read-only" and .dry_run_payload_id == "dry-run-payload:hepta-system:local-mcp:status-read-only-v0" and .dry_run_payload_digest == "dry-run-payload-digest:hepta-system:local-mcp:status-read-only-v0" and .dry_run_result_projection_id == "dry-run-result-projection:hepta-system:local-mcp:status-read-only-v0" and .receipt_projection_id == "dry-run-receipt-projection:hepta-system:local-mcp:read-only-denied" and .first_dry_run_receipt_id == "dry-run-receipt:hepta-system:local-mcp:read-only-denied" and .second_dry_run_receipt_id == "dry-run-receipt:hepta-system:local-mcp:read-only-denied" and .first_dry_run_idempotency_key == "dry-run-idempotency:hepta-system:local-mcp:read-only-denied" and .second_dry_run_idempotency_key == "dry-run-idempotency:hepta-system:local-mcp:read-only-denied")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .dry_run_path_selected == false and .dry_run_selection_reason == "non_selected_app_connector_preflight_boundary" and .feature_gate_id == "feature-gate:hepta-system:local-app:status-dry-run" and .feature_gate_state == "closed" and (.source_registration_denial_id | startswith("registration-denial:hepta-system:")) and (.source_shadow_lookup_result_id | startswith("shadow-lookup-result:hepta-system:")) and .source_internal_status_request_id == "hepta-system.status.internal-read-only.non-selected-app.v1" and .source_status_payload_fingerprint == "not-selected.preflight-only.no-payload" and .source_minimal_receipt_stage_id == "none_preflight_only" and .registration_denial_query_hit == true and .shadow_lookup_projection_attached == true and .internal_status_payload_projection_attached == false and .internal_call_dry_run_projected == false and .selected_dry_run_path_proof == false and .dry_run_request_id == "dry-run-request:hepta-system:local-app:not-selected" and .dry_run_payload_id == "dry-run-payload:hepta-system:local-app:not-selected" and .dry_run_payload_digest == "dry-run-payload-digest:hepta-system:local-app:not-selected" and .dry_run_result_projection_id == "dry-run-result-projection:hepta-system:local-app:not-selected" and .receipt_projection_id == "dry-run-receipt-projection:hepta-system:local-app:not-selected" and .first_dry_run_receipt_id == "dry-run-receipt:hepta-system:local-app:not-selected" and .second_dry_run_receipt_id == "dry-run-receipt:hepta-system:local-app:not-selected" and .first_dry_run_idempotency_key == "dry-run-idempotency:hepta-system:local-app:not-selected" and .second_dry_run_idempotency_key == "dry-run-idempotency:hepta-system:local-app:not-selected")
  and (.entries | all(.feature_gate_id_projected == true and .feature_gate_closed == true and .registration_denial_query_hit == true and .shadow_lookup_projection_attached == true and .policy_denial_projected == true and .receipt_projection_projected == true and .dry_run_receipt_projected == true and .stable_dry_run_receipt == true and .unique_dry_run_receipt == true and .idempotency_key_projected == true and .stable_idempotency_key == true and .unique_idempotency_key == true and .feature_gate_opened == false and .dry_run_executed == false and .dry_run_payload_persisted == false and .dry_run_result_persisted == false and .policy_decision_persisted == false and .approval_preflight_executed == false and .ledger_write_attempted == false and .receipt_projection_persisted == false and .tool_registered == false and .tool_registry_mutated == false and .registry_lookup_executed == false and .tool_invoked == false and .noop_result_persisted == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .dynamic_activation_started == false and .permission_granted == false and .mcp_server_started == false and .app_connector_started == false and .plugin_installed == false and .cache_materialized == false and .cache_mutated == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("feature_gate_open_disabled")) != null
  and (.blockers | index("dry_run_execution_disabled")) != null
  and (.blockers | index("dry_run_payload_persistence_disabled")) != null
  and (.blockers | index("dry_run_result_persistence_disabled")) != null
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
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback --lib
)

printf 'hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-gate: PASS: feature-gated read-only status dry-run projections are stable without feature gate open, dry-run execution, ToolRegistry registration, lookup, invocation, ledger, receipt, runtime, or live mutation\n'
