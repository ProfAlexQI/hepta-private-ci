#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-canary-dry-run-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-canary-dry-run-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_canary_dry_run_preview_gate"
  and .schema_version == "work_graph_persistence_canary_dry_run_preview_v1"
  and .preview_mode == "read_only_persistence_canary_dry_run_preview_no_canary_execution"
  and .lane_guard_count == 5
  and (.lane_guards | length) == .lane_guard_count
  and (.lane_guards | map(.id) == [
    "hepta_backend_lane_lock_required",
    "cargo_target_dir_isolated",
    "no_cross_lane_runtime_write",
    "no_external_delivery_lane",
    "no_operator_approval_recording_lane"
  ])
  and (.lane_guards | all(
    .lane_id == "hepta-backend"
    and .blocks_cross_lane_execution == true
    and .live_execution_allowed == false
  ))
  and .dry_run_scenario_count == 6
  and (.dry_run_scenarios | length) == .dry_run_scenario_count
  and (.dry_run_scenarios | map(.source_feature_flag_id) == [
    "work_graph_store_persistence_flag",
    "work_graph_wal_append_flag",
    "work_graph_checkpoint_write_flag",
    "work_graph_readback_receipt_persistence_flag",
    "work_graph_idempotency_index_write_flag",
    "work_graph_replay_execution_feature_flag"
  ])
  and (.dry_run_scenarios | all(
    .traffic_ppm == 0
    and .writes_allowed == false
    and .promotion_allowed == false
    and (.expected_evidence_ids | length) >= 3
    and (.expected_evidence_ids | index("durable_identity_evidence_packet"))
  ))
  and .traffic_guard_count == 5
  and (.traffic_guards | length) == .traffic_guard_count
  and (.traffic_guards | map(.id) == [
    "disabled_stage_traffic_guard",
    "local_dry_run_traffic_guard",
    "shadow_write_fixture_traffic_guard",
    "shadow_readback_compare_traffic_guard",
    "canary_lane_dry_run_traffic_guard"
  ])
  and (.traffic_guards | all(.max_traffic_ppm == 0 and .blocks_live_traffic == true))
  and .write_guard_count == 6
  and (.write_guards | length) == .write_guard_count
  and (.write_guards | map(.target_collection_id) == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and (.write_guards | all(.allowed_write_mode == "none" and .blocks_live_writes == true and .mutates_store == false))
  and .rollback_receipt_count == 6
  and (.rollback_receipts | length) == .rollback_receipt_count
  and (.rollback_receipts | map(.id) == [
    "operator_kill_switch_receipt",
    "wal_checksum_mismatch_receipt",
    "shadow_readback_drift_receipt",
    "idempotency_collision_receipt",
    "disk_budget_exceeded_receipt",
    "operator_approval_expired_receipt"
  ])
  and (.rollback_receipts | all(
    .persistence_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("run_id"))
    and (.required_fields | index("step_id"))
    and (.required_fields | index("checkpoint"))
    and (.required_fields | index("replay_key"))
    and (.required_fields | index("rollback_anchor"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | index("redactionState"))
    and (.required_fields | index("evidenceHash"))
  ))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "canary_dry_run_requires_durable_identity_evidence",
    "canary_dry_run_requires_feature_flags_default_off",
    "canary_dry_run_is_lane_scoped",
    "canary_dry_run_has_zero_live_traffic",
    "canary_dry_run_has_zero_live_writes",
    "rollback_receipts_are_redacted_and_non_persistent",
    "persistence_canary_dry_run_preview_has_no_side_effects"
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
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_promotion_precondition_preview_gate",
    "hepta_work_graph_activation_enforcement_blocker_preview_gate",
    "hepta_work_graph_shadow_adapter_readback_preview_gate",
    "hepta_work_graph_persistence_feature_flag_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ]
  and .durable_identity_evidence.required_for_dry_run_scenario_ids == [
    "canary_store_persistence_dry_run",
    "canary_wal_append_dry_run",
    "canary_checkpoint_write_dry_run",
    "canary_readback_receipt_dry_run",
    "canary_idempotency_index_dry_run",
    "canary_replay_execution_dry_run"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .recommended_next_gate == "hepta_work_graph_persistence_canary_readback_receipt_preview_gate"
  and .ready_for_canary_readback_receipt_preview == true
  and .ready_for_canary_execution == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_canary_dry_run.rust_module_present == true
  and .source_probes.persistence_canary_dry_run.report_script_present == true
  and .source_probes.persistence_canary_dry_run.gate_script_present == true
  and .source_probes.persistence_feature_flag.gate_script_present == true
  and .source_probes.shadow_adapter_readback.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_canary_dry_run_preview --lib

echo "Hepta WorkGraph persistence canary dry-run preview gate passed"
