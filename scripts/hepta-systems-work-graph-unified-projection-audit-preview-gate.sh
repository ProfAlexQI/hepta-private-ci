#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-audit-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-unified-projection-audit-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_unified_projection_audit_preview_gate"
  and .schema_version == "work_graph_unified_projection_audit_preview_v1"
  and .preview_mode == "read_only_cross_surface_projection_audit_no_persistence"
  and .source_surface_count == 12
  and (.source_surfaces | length) == .source_surface_count
  and .source_category_count == 6
  and .projected_node_kind_count == 6
  and .projected_collection_count == 6
  and (.source_surfaces | map(.source_surface_id) == [
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
  and (.source_surfaces | map(.source_category) | unique == [
    "batch_agent_jobs",
    "external_handoff",
    "multi_agent",
    "operator_control",
    "planning",
    "runtime_scheduler"
  ])
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 7
  and (.required_prior_gates == [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate"
  ])
  and .coverage_gap_count == 5
  and (.coverage_gaps | map(.id) == [
    "planning_identity_is_split_between_update_plan_and_plan_mode",
    "mailbox_wait_lacks_structured_task_result_join",
    "task_board_has_admission_shape_without_unified_store_projection",
    "batch_and_worker_results_are_not_enforced_task_results",
    "role_manifest_and_scheduler_admission_remain_preview_only"
  ])
  and (.coverage_gaps | map(select(.severity == "high")) | length) == 4
  and (.source_surfaces | all(.has_adapter_fixture == true))
  and (.source_surfaces | map(select(.source_surface_id == "plan_mode_proposed_plan_blocks" and .coverage_state == "partial_projection_preview")) | length) == 1
  and (.source_surfaces | map(select(.source_surface_id == "multi_agent_v2_mailbox_wait" and .coverage_state == "timeline_only_preview")) | length) == 1
  and (.source_surfaces | map(select(.source_surface_id == "hepta_runtime_task_board" and (.blocker_ids | index("unified_store_projection_missing")))) | length) == 1
  and (.source_surfaces | map(select(.source_surface_id == "multi_agent_v2_thread_spawn" and .has_task_result_projection == true and .has_role_manifest_projection == true)) | length) == 1
  and (.source_surfaces | map(select(.requires_terminal_task_result == true)) | length) == 6
  and (.source_surfaces | all(.next_projection_step | length > 0))
' >/dev/null <<<"$report"

jq -e '
  .next_cut_count == 4
  and (.next_cuts | map(.priority) == ["P0", "P1", "P2", "P3"])
  and (.next_cuts | all(.must_remain_side_effect_free == true))
  and .recommended_next_gate == "hepta_work_graph_state_store_persistence_preview_gate"
  and .ready_for_state_store_persistence_preview == true
  and .ready_for_store_persistence == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.unified_projection_audit.rust_module_present == true
  and .source_probes.unified_projection_audit.report_script_present == true
  and .source_probes.unified_projection_audit.gate_script_present == true
  and .source_probes.adapter_projection_fixture.rust_module_present == true
  and (.source_probes.source_surfaces | to_entries | all(.value == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_audit --lib

echo "Hepta WorkGraph unified projection audit preview gate passed"
