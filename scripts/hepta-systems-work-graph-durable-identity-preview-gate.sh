#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-durable-identity-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_durable_identity_preview_gate"
  and .schema_version == "work_graph_durable_identity_preview_v1"
  and .preview_mode == "read_only_durable_identity_contract_preview_no_state_writes"
  and .durable_field_count == 7
  and (.durable_fields | length) == .durable_field_count
  and (.durable_fields | map(.id) == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ])
  and (.durable_fields | map(.phase) == [
    "identity",
    "identity",
    "identity",
    "checkpoint",
    "replay",
    "rollback",
    "receipt"
  ])
  and (.durable_fields | all(.mutates_state == false and (.source_fields | length) >= 3))
  and (.durable_fields[] | select(.id == "workflow_id") | .source_fields == [
    "traceId",
    "sourceThreadId",
    "sourceSurfaceId"
  ])
  and (.durable_fields[] | select(.id == "checkpoint") | .source_fields == [
    "walHeadHash",
    "checkpointHash",
    "collectionMerkleRoot"
  ])
  and (.durable_fields[] | select(.id == "replay_key") | .required_prior_gate == "hepta_work_graph_replay_readback_preview_gate")
  and (.durable_fields[] | select(.id == "rollback_anchor") | .required_prior_gate == "hepta_work_graph_replay_readback_preview_gate")
  and (.durable_fields[] | select(.id == "receipt_hash") | .required_prior_gate == "hepta_work_graph_replay_readback_preview_gate")
  and .preview_binding_count == 5
  and (.preview_bindings | length) == .preview_binding_count
  and (.preview_bindings | map(.id) == [
    "state_store_wal_to_durable_identity",
    "checkpoint_contract_to_checkpoint",
    "replay_hash_chain_to_replay_key",
    "recovery_preview_to_rollback_anchor",
    "readback_evidence_to_receipt_hash"
  ])
  and (.preview_bindings | all(.required == true and .mutates_state == false))
  and (.preview_bindings | map(.source_gate) == [
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate"
  ])
  and (.preview_bindings[] | select(.id == "state_store_wal_to_durable_identity") | .binds_fields == [
    "workflow_id",
    "run_id",
    "step_id",
    "receipt_hash"
  ])
  and (.preview_bindings[] | select(.id == "checkpoint_contract_to_checkpoint") | .source_contract_ids == [
    "preview_full_graph_checkpoint",
    "preview_trace_checkpoint"
  ])
  and (.preview_bindings[] | select(.id == "recovery_preview_to_rollback_anchor") | .source_contract_ids == [
    "preview_quarantine_checkpoint",
    "preview_rebuild_projection_indexes",
    "preview_require_operator_replay_approval"
  ])
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "durable_identity_required_before_persistence",
    "checkpoint_derived_from_wal",
    "replay_key_is_deterministic",
    "rollback_anchor_precedes_recovery",
    "receipt_hash_precedes_promotion",
    "readback_evidence_is_redacted",
    "durable_identity_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates == [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate"
  ])
  and .existing_preview_bindings.state_store_schema_version == "work_graph_state_store_persistence_preview_v1"
  and .existing_preview_bindings.replay_readback_schema_version == "work_graph_replay_readback_preview_v1"
  and .existing_preview_bindings.wal_operation_count == 6
  and .existing_preview_bindings.checkpoint_contract_count == 4
  and .existing_preview_bindings.idempotency_guard_count == 7
  and .existing_preview_bindings.readback_probe_count == 6
  and .existing_preview_bindings.replay_stage_count == 6
  and .existing_preview_bindings.readback_assertion_count == 6
  and .existing_preview_bindings.drift_detector_count == 5
  and .existing_preview_bindings.recovery_preview_count == 5
  and .recommended_next_gate == "hepta_work_graph_promotion_precondition_preview_gate"
  and .ready_for_promotion_precondition_preview == true
  and .ready_for_durable_runtime == false
  and .ready_for_replay_execution == false
  and .ready_for_rollback_execution == false
  and .ready_for_live_execution == false
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and .source_probes.state_store_persistence.rust_module_present == true
  and .source_probes.state_store_persistence.report_script_present == true
  and .source_probes.state_store_persistence.gate_script_present == true
  and .source_probes.replay_readback.rust_module_present == true
  and .source_probes.replay_readback.report_script_present == true
  and .source_probes.replay_readback.gate_script_present == true
  and .source_probes.promotion_precondition.rust_module_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_durable_identity_preview --lib

echo "Hepta WorkGraph durable identity preview gate passed"
