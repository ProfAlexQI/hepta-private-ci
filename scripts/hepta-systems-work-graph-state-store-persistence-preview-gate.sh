#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-state-store-persistence-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-state-store-persistence-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_state_store_persistence_preview_gate"
  and .schema_version == "work_graph_state_store_persistence_preview_v1"
  and .preview_mode == "read_only_persistence_contract_preview_no_store_writes"
  and .wal_operation_count == 6
  and (.wal_operations | length) == .wal_operation_count
  and (.wal_operations | map(.id) == [
    "preview_append_node_record",
    "preview_append_edge_record",
    "preview_append_task_result_record",
    "preview_append_artifact_record",
    "preview_append_approval_record",
    "preview_append_timeline_event_record"
  ])
  and (.wal_operations | map(.record_kind) == [
    "node",
    "edge",
    "task_result",
    "artifact",
    "approval",
    "timeline_event"
  ])
  and (.wal_operations | all(.mutates_store == false and (.required_fields | index("traceId"))))
  and .checkpoint_contract_count == 4
  and (.checkpoint_contracts | length) == .checkpoint_contract_count
  and (.checkpoint_contracts | map(.id) == [
    "preview_full_graph_checkpoint",
    "preview_trace_checkpoint",
    "preview_artifact_checkpoint",
    "preview_approval_checkpoint"
  ])
  and (.checkpoint_contracts | all(.mutates_store == false and (.write_policy | startswith("disabled_until_"))))
  and (.checkpoint_contracts[] | select(.id == "preview_full_graph_checkpoint") | .included_collection_ids == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and .idempotency_guard_count == 12
  and (.idempotency_guards | length) == .idempotency_guard_count
  and .source_surface_count == 12
  and (.idempotency_guards | map(.source_surface_id) == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_thread_spawn",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_approval_broker",
    "hepta_runtime_agent_harness"
  ])
  and (.idempotency_guards | all(.required_before_persistence == true and (.key_fields | length) >= 3))
  and .readback_probe_count == 6
  and (.readback_probes | length) == .readback_probe_count
  and (.readback_probes | map(.target_collection_id) == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and (.readback_probes | all(.mutates_store == false and (.promotion_blocker | length) > 0))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "wal_records_are_append_only",
    "checkpoints_are_derived_from_wal",
    "idempotency_index_precedes_write",
    "readback_precedes_promotion",
    "payloads_are_redacted_before_persistence",
    "recovery_is_preview_only",
    "persistence_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates == [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_replay_readback_preview_gate"
  and .ready_for_replay_readback_preview == true
  and .ready_for_store_persistence == false
  and .ready_for_live_execution == false
  and .source_probes.state_store_persistence.rust_module_present == true
  and .source_probes.state_store_persistence.report_script_present == true
  and .source_probes.state_store_persistence.gate_script_present == true
  and .source_probes.adapter_projection_fixture.rust_module_present == true
  and .source_probes.adapter_projection_fixture.report_script_present == true
  and .source_probes.adapter_projection_fixture.gate_script_present == true
  and .source_probes.unified_state_store.rust_module_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_state_store_persistence_preview --lib

echo "Hepta WorkGraph state store persistence preview gate passed"
