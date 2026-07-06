#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-shadow-adapter-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-shadow-adapter-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_shadow_adapter_readback_preview_gate"
  and .schema_version == "work_graph_shadow_adapter_readback_preview_v1"
  and .preview_mode == "read_only_shadow_adapter_readback_preview_no_adapter_execution"
  and .adapter_shadow_count == 7
  and (.adapter_shadows | length) == .adapter_shadow_count
  and (.adapter_shadows | map(.source_surface_id) == [
    "update_plan_tool",
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_approval_broker",
    "hepta_runtime_agent_harness"
  ])
  and (.adapter_shadows | all(
    .shadow_execution_enabled == false
    and .enforcement_enabled == false
    and (.expected_collection_ids | index("nodes"))
    and (.required_readback_ids | length) >= 3
  ))
  and .collection_readback_count == 6
  and (.collection_readbacks | length) == .collection_readback_count
  and (.collection_readbacks | map(.collection_id) == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and (.collection_readbacks | all(
    .blocks_activation == true
    and .mutates_store == false
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("run_id"))
    and (.required_fields | index("step_id"))
    and (.required_fields | index("checkpoint"))
    and (.required_fields | index("replay_key"))
    and (.required_fields | index("rollback_anchor"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | index("traceId"))
  ))
  and .mismatch_detector_count == 6
  and (.mismatch_detectors | length) == .mismatch_detector_count
  and (.mismatch_detectors | map(.id) == [
    "detect_shadow_node_identity_mismatch",
    "detect_shadow_edge_link_mismatch",
    "detect_shadow_task_result_contract_mismatch",
    "detect_shadow_artifact_redaction_mismatch",
    "detect_shadow_approval_scope_mismatch",
    "detect_shadow_timeline_order_mismatch"
  ])
  and (.mismatch_detectors | all(.blocks_adapter_enforcement == true and (.compared_fields | length) >= 3))
  and (.mismatch_detectors | map(select(.severity == "critical")) | length) == 5
  and .evidence_packet_count == 7
  and (.evidence_packets | length) == .evidence_packet_count
  and (.evidence_packets | map(.source_surface_id) == [
    "update_plan_tool",
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_approval_broker",
    "hepta_runtime_agent_harness"
  ])
  and (.evidence_packets | all(
    .persistence_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("run_id"))
    and (.required_fields | index("step_id"))
    and (.required_fields | index("checkpoint"))
    and (.required_fields | index("replay_key"))
    and (.required_fields | index("rollback_anchor"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | index("projectedHash"))
    and (.required_fields | index("readbackHash"))
    and (.required_fields | index("redactionState"))
  ))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "shadow_readback_requires_durable_identity_evidence",
    "shadow_readback_matches_projection_before_enforcement",
    "shadow_readback_covers_every_projected_collection",
    "mismatch_blocks_adapter_enforcement",
    "shadow_evidence_is_redacted_and_non_persistent",
    "shadow_adapter_does_not_execute_source_adapters",
    "shadow_adapter_readback_preview_has_no_side_effects"
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
  and .durable_identity_evidence.required_for_adapter_shadow_ids == [
    "shadow_update_plan_step_projection",
    "shadow_multi_agent_thread_spawn_projection",
    "shadow_agent_job_item_result_projection",
    "shadow_runtime_worker_task_artifact_projection",
    "shadow_scheduler_run_admission_projection",
    "shadow_approval_broker_human_approval_projection",
    "shadow_agent_harness_external_handoff_projection"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .recommended_next_gate == "hepta_work_graph_persistence_feature_flag_preview_gate"
  and .ready_for_persistence_feature_flag_preview == true
  and .ready_for_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.shadow_adapter_readback.rust_module_present == true
  and .source_probes.shadow_adapter_readback.report_script_present == true
  and .source_probes.shadow_adapter_readback.gate_script_present == true
  and .source_probes.activation_blocker.rust_module_present == true
  and .source_probes.activation_blocker.report_script_present == true
  and .source_probes.activation_blocker.gate_script_present == true
  and .source_probes.adapter_projection_fixture.rust_module_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_shadow_adapter_readback_preview --lib

echo "Hepta WorkGraph shadow adapter readback preview gate passed"
