#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-boundary-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable operator evidence acceptance recording boundary report: $REPORT"
[[ -f "$DOC" ]] || fail "missing operator evidence acceptance recording boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the operator evidence acceptance recording boundary report"
fi

rg -q 'Hepta Systems Plugin Tool Invocation Read Only Status Dry Run Operator Evidence Acceptance Recording Boundary Readback' "$DOC" \
  || fail "architecture note must document the operator evidence acceptance recording boundary readback"
rg -q 'evidence artifact ref links, operator identity links, acceptance record prerequisites, non-recording denial receipts, ledger persistence closure anchors, receipt persistence closure anchors, tool invocation closure anchors, runtime write closure anchors, live execution closure anchors, and acceptance-recording boundary idempotency keys' "$DOC" \
  || fail "architecture note must document the acceptance-recording boundary projections"
rg -q 'no feature gate open, dry-run execution, operator evidence packet send, operator evidence packet persistence, operator evidence recording, operator acceptance recording, acceptance record persistence, non-recording denial receipt persistence, ledger persistence, receipt persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, connector start, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed operator evidence acceptance-recording boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_gate"
  and .schema_version == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_operator_evidence_packet_readback_ready == true
  and .lib_export_present == true
  and .candidate_count == 2
  and .boundary_entry_count == 2
  and .selected_read_only_status_tool_count == 1
  and .non_selected_preflight_boundary_count == 1
  and .acceptance_recording_boundary_id_projected_count == 2
  and .evidence_artifact_ref_link_projected_count == 2
  and .operator_identity_link_projected_count == 2
  and .acceptance_record_prerequisite_projected_count == 2
  and .non_recording_denial_receipt_projected_count == 2
  and .ledger_persistence_closure_anchor_projected_count == 2
  and .receipt_persistence_closure_anchor_projected_count == 2
  and .tool_invocation_closure_anchor_projected_count == 2
  and .runtime_write_closure_anchor_projected_count == 2
  and .live_execution_closure_anchor_projected_count == 2
  and .acceptance_recording_boundary_idempotency_key_projected_count == 2
  and .stable_acceptance_recording_boundary_count == 2
  and .unique_acceptance_recording_boundary_count == 2
  and .stable_non_recording_denial_receipt_count == 2
  and .unique_non_recording_denial_receipt_count == 2
  and .stable_acceptance_recording_boundary_idempotency_key_count == 2
  and .unique_acceptance_recording_boundary_idempotency_key_count == 2
  and .acceptance_recording_boundary_mismatch_count == 0
  and .duplicate_acceptance_recording_boundary_count == 0
  and .non_recording_denial_receipt_mismatch_count == 0
  and .duplicate_non_recording_denial_receipt_count == 0
  and .acceptance_recording_boundary_idempotency_mismatch_count == 0
  and .duplicate_acceptance_recording_boundary_idempotency_key_count == 0
  and .feature_gate_opened_count == 0
  and .dry_run_executed_count == 0
  and .operator_evidence_packet_sent_count == 0
  and .operator_evidence_packet_persisted_count == 0
  and .operator_evidence_recorded_count == 0
  and .operator_acceptance_recorded_count == 0
  and .acceptance_record_persisted_count == 0
  and .non_recording_denial_receipt_persisted_count == 0
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
  and .acceptance_recording_boundary_readback_ready == true
  and .feature_gate_open_allowed == false
  and .dry_run_execution_allowed == false
  and .operator_evidence_packet_send_allowed == false
  and .operator_evidence_packet_persistence_allowed == false
  and .operator_evidence_recording_allowed == false
  and .operator_acceptance_recording_allowed == false
  and .acceptance_record_persistence_allowed == false
  and .non_recording_denial_receipt_persistence_allowed == false
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
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .dry_run_path_selected == true and .acceptance_recording_boundary_id == "operator-evidence-acceptance-recording-boundary:hepta-system:local-mcp:read-only-status-dry-run" and .evidence_artifact_ref_link_id == "operator-evidence-acceptance-artifact-link:hepta-system:local-mcp:read-only-status-dry-run:missing" and .operator_identity_link_id == "operator-evidence-acceptance-operator-identity:hepta-system:local-mcp:read-only-status-dry-run:missing" and .acceptance_record_prerequisite_id == "operator-evidence-acceptance-record-prerequisite:hepta-system:local-mcp:read-only-status-dry-run" and .non_recording_denial_receipt_id == "operator-evidence-acceptance-non-recording-denial:hepta-system:local-mcp:read-only-status-dry-run:not-recorded" and .acceptance_recording_boundary_idempotency_key == "operator-evidence-acceptance-boundary-idempotency:hepta-system:local-mcp:read-only-status-dry-run")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .dry_run_path_selected == false and .acceptance_recording_boundary_id == "operator-evidence-acceptance-recording-boundary:hepta-system:local-app:not-selected" and .evidence_artifact_ref_link_id == "operator-evidence-acceptance-artifact-link:hepta-system:local-app:not-selected:missing" and .operator_identity_link_id == "operator-evidence-acceptance-operator-identity:hepta-system:local-app:not-selected:missing" and .acceptance_record_prerequisite_id == "operator-evidence-acceptance-record-prerequisite:hepta-system:local-app:not-selected" and .non_recording_denial_receipt_id == "operator-evidence-acceptance-non-recording-denial:hepta-system:local-app:not-selected:not-recorded" and .acceptance_recording_boundary_idempotency_key == "operator-evidence-acceptance-boundary-idempotency:hepta-system:local-app:not-selected")
  and (.entries | all(.acceptance_recording_boundary_id_projected == true and .evidence_artifact_ref_link_projected == true and .operator_identity_link_projected == true and .acceptance_record_prerequisite_projected == true and .non_recording_denial_receipt_projected == true and .ledger_persistence_closure_anchor_projected == true and .receipt_persistence_closure_anchor_projected == true and .tool_invocation_closure_anchor_projected == true and .runtime_write_closure_anchor_projected == true and .live_execution_closure_anchor_projected == true and .acceptance_recording_boundary_idempotency_key_projected == true and .stable_acceptance_recording_boundary == true and .unique_acceptance_recording_boundary == true and .stable_non_recording_denial_receipt == true and .unique_non_recording_denial_receipt == true and .stable_acceptance_recording_boundary_idempotency_key == true and .unique_acceptance_recording_boundary_idempotency_key == true and .feature_gate_opened == false and .dry_run_executed == false and .operator_evidence_packet_sent == false and .operator_evidence_packet_persisted == false and .operator_evidence_recorded == false and .operator_acceptance_recorded == false and .acceptance_record_persisted == false and .non_recording_denial_receipt_persisted == false and .ledger_written == false and .receipt_persisted == false and .tool_registered == false and .registry_lookup_executed == false and .tool_invoked == false and .mcp_server_started == false and .app_connector_started == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("operator_evidence_packet_send_disabled")) != null
  and (.blockers | index("operator_evidence_packet_persistence_disabled")) != null
  and (.blockers | index("operator_evidence_recording_disabled")) != null
  and (.blockers | index("operator_acceptance_recording_disabled")) != null
  and (.blockers | index("acceptance_record_persistence_disabled")) != null
  and (.blockers | index("non_recording_denial_receipt_persistence_disabled")) != null
  and (.blockers | index("ledger_persistence_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("registry_lookup_execution_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("connector_start_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback --lib
)

printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-boundary-readback-gate: PASS: operator evidence acceptance-recording boundary is queryable without evidence recording, acceptance recording, persistence, tool invocation, runtime writes, or live mutation\n'
