#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-adapter-projection-fixture-report.sh"

report="$(capture_json_report "hepta-work-graph-adapter-projection-fixture-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_adapter_projection_fixture_gate"
  and .schema_version == "work_graph_adapter_projection_fixture_v1"
  and .preview_mode == "read_only_adapter_projection_fixture_no_persistence"
  and .fixture_count == 12
  and (.fixtures | length) == .fixture_count
  and .source_surface_count == 12
  and (.fixtures | map(.source_surface_id) == [
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
  and (.fixtures | map(.node_kind) == [
    "plan_step",
    "plan_step",
    "plan_step",
    "agent_task",
    "agent_task",
    "agent_task",
    "worker_task",
    "worker_task",
    "worker_task",
    "scheduler_run",
    "human_approval",
    "external_handoff"
  ])
  and (.fixtures | all(
    (.projected_node_id | startswith("wg-node-"))
    and (.trace_id | startswith("wg-trace-preview-"))
    and (.source_record_id | length) > 0
    and (.redaction_state == "redacted_refs_only")
    and (.projected_collection_ids | index("nodes"))
    and (.projected_collection_ids | index("timelineEvents"))
    and (.projected_timeline_event_ids | length) >= 1
    and (.required_contract_gates | length) == 6
    and (.required_contract_gates | index("hepta_work_graph_unified_state_store_preview_gate"))
    and .persistence_enabled == false
    and .enforcement_enabled == false
  ))
  and (.fixtures | map(select(.source_surface_id == "hepta_runtime_approval_broker")) | length) == 1
  and (.fixtures | map(select(.source_surface_id == "hepta_runtime_approval_broker" and .projected_approval_id != null)) | length) == 1
  and (.fixtures | map(select(.projected_task_result_id != null)) | length) == 6
  and (.fixtures | map(select((.projected_artifact_ids | length) > 0)) | length) == 2
  and (.fixtures | map(select(.idempotency_key_hash != null)) | length) == 7
  and .projected_collection_count == 6
  and (.projected_collections | length) == .projected_collection_count
  and (.projected_collections | map(.id) == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and (.projected_collections | all(.required_before_persistence == true and (.fixture_ids | length) >= 1))
  and .invariant_count == 6
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "fixtures_use_deterministic_redacted_ids",
    "every_fixture_has_trace_and_node_id",
    "collection_coverage_includes_nodes_and_timeline",
    "task_result_fixtures_use_task_result_contract",
    "approval_and_external_handoff_are_preview_only",
    "fixture_gate_does_not_persist_or_enforce"
  ])
  and (.invariants | all(.required == true))
  and .recommended_next_gate == "hepta_work_graph_state_store_persistence_preview_gate"
  and .ready_for_state_store_persistence_preview == true
  and .ready_for_store_persistence == false
  and .ready_for_live_execution == false
  and .source_probes.adapter_projection_fixture.rust_module_present == true
  and .source_probes.adapter_projection_fixture.report_script_present == true
  and .source_probes.adapter_projection_fixture.gate_script_present == true
  and .source_probes.unified_state_store.rust_module_present == true
  and .source_probes.unified_state_store.report_script_present == true
  and .source_probes.unified_state_store.gate_script_present == true
  and .source_probes.approval_broker.rust_module_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_adapter_projection_fixture --lib

echo "Hepta WorkGraph adapter projection fixture gate passed"
