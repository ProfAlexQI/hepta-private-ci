#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-open-preconditions-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_OPEN_PRECONDITIONS_READBACK_2026-07-01.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-open-preconditions-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable persistence open preconditions report: $REPORT"
[[ -f "$DOC" ]] || fail "missing persistence open preconditions architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the persistence open preconditions report"
fi

rg -q 'Hepta Systems Plugin Tool Invocation Read Only Status Dry Run Operator Evidence Acceptance Recording Persistence Open Preconditions Readback' "$DOC" \
  || fail "architecture note must document the persistence open preconditions readback"
rg -q 'persistence open precondition set, source denial receipt, source denial receipt digest, source idempotency key, evidence artifact, operator identity, operator acceptance, operator evidence record store binding, acceptance record schema, acceptance record store binding, idempotency index, ledger store binding, receipt store binding, runtime event-log store binding, rollback anchor, kill-switch, retention policy, readback query, controlled-live evidence, and feature gate' "$DOC" \
  || fail "architecture note must document the query-only persistence open precondition set"
rg -q 'no feature gate open, dry-run execution, operator evidence packet send, operator evidence packet persistence, operator evidence recording, operator acceptance recording, acceptance record persistence, persistence open denial receipt persistence, persistence denial receipt persistence, non-recording denial receipt persistence, idempotency index write, ledger persistence, receipt persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, connector start, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed persistence open boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_persistence_denial_receipt_readback_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .precondition_entry_count == 2
  and .selected_read_only_status_tool_count == 1
  and .non_selected_preflight_boundary_count == 1
  and .persistence_open_precondition_set_projected_count == 2
  and .source_persistence_denial_receipt_linked_count == 2
  and .source_persistence_denial_receipt_digest_linked_count == 2
  and .source_persistence_idempotency_key_linked_count == 2
  and .evidence_artifact_presence_precondition_projected_count == 2
  and .operator_identity_precondition_projected_count == 2
  and .operator_acceptance_precondition_projected_count == 2
  and .operator_evidence_record_store_binding_precondition_projected_count == 2
  and .acceptance_record_schema_precondition_projected_count == 2
  and .acceptance_record_store_binding_precondition_projected_count == 2
  and .acceptance_record_idempotency_index_precondition_projected_count == 2
  and .ledger_store_binding_precondition_projected_count == 2
  and .receipt_store_binding_precondition_projected_count == 2
  and .runtime_event_log_store_binding_precondition_projected_count == 2
  and .rollback_anchor_precondition_projected_count == 2
  and .kill_switch_precondition_projected_count == 2
  and .retention_policy_precondition_projected_count == 2
  and .readback_query_precondition_projected_count == 2
  and .controlled_live_evidence_precondition_projected_count == 2
  and .feature_gate_precondition_projected_count == 2
  and .persistence_open_precondition_item_count == 32
  and .stable_persistence_open_precondition_set_count == 2
  and .unique_persistence_open_precondition_set_count == 2
  and .stable_persistence_open_denial_receipt_count == 2
  and .unique_persistence_open_denial_receipt_count == 2
  and .stable_persistence_open_idempotency_key_count == 2
  and .unique_persistence_open_idempotency_key_count == 2
  and .persistence_open_precondition_set_mismatch_count == 0
  and .duplicate_persistence_open_precondition_set_count == 0
  and .persistence_open_denial_receipt_mismatch_count == 0
  and .duplicate_persistence_open_denial_receipt_count == 0
  and .persistence_open_idempotency_mismatch_count == 0
  and .duplicate_persistence_open_idempotency_key_count == 0
  and .feature_gate_opened_count == 0
  and .dry_run_executed_count == 0
  and .operator_evidence_packet_sent_count == 0
  and .operator_evidence_packet_persisted_count == 0
  and .operator_evidence_recorded_count == 0
  and .operator_acceptance_recorded_count == 0
  and .acceptance_record_persisted_count == 0
  and .persistence_open_denial_receipt_persisted_count == 0
  and .persistence_denial_receipt_persisted_count == 0
  and .non_recording_denial_receipt_persisted_count == 0
  and .idempotency_index_written_count == 0
  and .ledger_written_count == 0
  and .receipt_persisted_count == 0
  and .tool_registered_count == 0
  and .registry_lookup_executed_count == 0
  and .tool_invoked_count == 0
  and .mcp_server_started_count == 0
  and .app_connector_started_count == 0
  and .runtime_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_execution_started_count == 0
  and .persistence_open_preconditions_readback_ready == true
  and .feature_gate_open_allowed == false
  and .dry_run_execution_allowed == false
  and .operator_evidence_packet_send_allowed == false
  and .operator_evidence_packet_persistence_allowed == false
  and .operator_evidence_recording_allowed == false
  and .operator_acceptance_recording_allowed == false
  and .acceptance_record_persistence_allowed == false
  and .persistence_open_denial_receipt_persistence_allowed == false
  and .persistence_denial_receipt_persistence_allowed == false
  and .non_recording_denial_receipt_persistence_allowed == false
  and .idempotency_index_write_allowed == false
  and .ledger_persistence_allowed == false
  and .receipt_persistence_allowed == false
  and .tool_registry_registration_allowed == false
  and .registry_lookup_execution_allowed == false
  and .tool_invocation_allowed == false
  and .connector_start_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .dry_run_path_selected == true and .persistence_open_precondition_set_id == "operator-evidence-acceptance-recording-persistence-open-preconditions:hepta-system:local-mcp:read-only-status-dry-run" and .persistence_open_idempotency_key == "operator-evidence-acceptance-recording-persistence-open-idempotency:hepta-system:local-mcp:read-only-status-dry-run")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .dry_run_path_selected == false and .persistence_open_precondition_set_id == "operator-evidence-acceptance-recording-persistence-open-preconditions:hepta-system:local-app:not-selected" and .persistence_open_idempotency_key == "operator-evidence-acceptance-recording-persistence-open-idempotency:hepta-system:local-app:not-selected")
  and (.entries | all(.persistence_open_precondition_set_projected == true and .source_persistence_denial_receipt_linked == true and .source_persistence_denial_receipt_digest_linked == true and .source_persistence_idempotency_key_linked == true and .evidence_artifact_presence_precondition_projected == true and .operator_identity_precondition_projected == true and .operator_acceptance_precondition_projected == true and .operator_evidence_record_store_binding_precondition_projected == true and .acceptance_record_schema_precondition_projected == true and .acceptance_record_store_binding_precondition_projected == true and .acceptance_record_idempotency_index_precondition_projected == true and .ledger_store_binding_precondition_projected == true and .receipt_store_binding_precondition_projected == true and .runtime_event_log_store_binding_precondition_projected == true and .rollback_anchor_precondition_projected == true and .kill_switch_precondition_projected == true and .retention_policy_precondition_projected == true and .readback_query_precondition_projected == true and .controlled_live_evidence_precondition_projected == true and .feature_gate_precondition_projected == true and .stable_persistence_open_precondition_set == true and .unique_persistence_open_precondition_set == true and .stable_persistence_open_denial_receipt == true and .unique_persistence_open_denial_receipt == true and .stable_persistence_open_idempotency_key == true and .unique_persistence_open_idempotency_key == true and .feature_gate_opened == false and .dry_run_executed == false and .operator_evidence_packet_sent == false and .operator_evidence_packet_persisted == false and .operator_evidence_recorded == false and .operator_acceptance_recorded == false and .acceptance_record_persisted == false and .persistence_open_denial_receipt_persisted == false and .persistence_denial_receipt_persisted == false and .non_recording_denial_receipt_persisted == false and .idempotency_index_written == false and .ledger_written == false and .receipt_persisted == false and .tool_registered == false and .registry_lookup_executed == false and .tool_invoked == false and .mcp_server_started == false and .app_connector_started == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("feature_gate_closed")) != null
  and (.blockers | index("operator_evidence_artifact_absent")) != null
  and (.blockers | index("operator_acceptance_unrecorded")) != null
  and (.blockers | index("acceptance_record_store_binding_absent")) != null
  and (.blockers | index("acceptance_record_idempotency_index_absent")) != null
  and (.blockers | index("ledger_store_binding_absent")) != null
  and (.blockers | index("receipt_store_binding_absent")) != null
  and (.blockers | index("runtime_event_log_store_binding_absent")) != null
  and (.blockers | index("rollback_anchor_absent")) != null
  and (.blockers | index("kill_switch_unrehearsed")) != null
  and (.blockers | index("controlled_live_evidence_absent")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback --lib
)

printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-open-preconditions-readback-gate: PASS: persistence-open preconditions are queryable without acceptance-record writes, idempotency writes, tool invocation, runtime writes, or live mutation\n'
