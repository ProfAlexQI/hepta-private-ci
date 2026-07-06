#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_append_only_store_rerun_no_enforcement"
  and .source_surface_count == 12
  and .append_only_precondition_application_outcome_count == 7
  and .previous_contract_ready_surface_count == 12
  and .append_only_store_rerun_contract_ready_surface_count == 12
  and .previous_append_only_store_primary_blocked_surface_count == 6
  and .append_only_store_primary_blocked_surface_count_after == 0
  and .append_only_store_precondition_application_source_count == 12
  and .append_only_store_precondition_ready_surface_count == 12
  and .append_only_store_runtime_residual_source_count == 12
  and .rerun_ready_surface_count == 0
  and .rerun_blocked_surface_count == 12
  and .decision_delta_count == 12
  and (.decision_deltas | length) == .decision_delta_count
  and (.decision_deltas | all(.projection_contract_ready == true and .store_idempotency_guard_ready == true and .terminal_task_result_contract_ready == true and .append_only_store_precondition_ready == true))
' >/dev/null <<<"$report"

jq -e '
  (.decision_deltas | map(select(.covered_by_append_only_store_precondition_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.append_only_store_primary_gap_closed_by_precondition_application_preview == true)) | length) == 6
  and (.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_append_only_store_disabled" or .append_only_store_rerun_enforcement_decision == "deny_append_only_store_precondition_missing")) | length) == 0
  and (.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_runtime_append_only_store_enablement_disabled")) | length) == 6
  and (.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced")) | length) == 5
  and (.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_role_manifest_not_enforced")) | length) == 1
  and (.decision_deltas | map(select(.source_surface_id == "hepta_runtime_multi_agent_reducer" and .previous_enforcement_decision == "deny_append_only_store_disabled" and .append_only_store_rerun_enforcement_decision == "deny_runtime_append_only_store_enablement_disabled" and .append_only_store_rerun_state == "append_only_store_precondition_contract_ready_preview_after_application")) | length) == 1
  and (.decision_deltas | map(select(.source_surface_id == "multi_agent_v2_thread_spawn" and .append_only_store_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced")) | length) == 1
  and (.decision_deltas | map(select(.source_surface_id == "hepta_runtime_agent_harness" and .append_only_store_rerun_enforcement_decision == "deny_role_manifest_not_enforced")) | length) == 1
  and (.decision_deltas | all(.residual_route_blocker_ids | index("append_only_store_disabled_by_design") | not))
  and (.decision_deltas | all(.runtime_append_only_store_enabled == false and .scheduler_admission_enforcement_ready == false and .role_manifest_enforcement_ready == false))
' >/dev/null <<<"$report"

jq -e '
  .cleared_blocker_count == 1
  and (.cleared_blockers[0].id == "append_only_store_enablement_preconditions_missing_for_enforcement")
  and (.cleared_blockers[0].source_count_before == 6)
  and (.cleared_blockers[0].source_count_after == 0)
  and (.cleared_blockers[0].closure_gate_id == "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate")
  and (.cleared_blockers[0].cleared_source_surface_ids == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_approval_broker"
  ])
  and .residual_blocker_count == 13
  and (.residual_blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "projection_adapter_runtime_closure_application_disabled", "count": 7},
    {"id": "store_guard_runtime_application_disabled", "count": 5},
    {"id": "idempotency_index_mutation_disabled", "count": 12},
    {"id": "state_store_guard_persistence_disabled", "count": 5},
    {"id": "terminal_task_result_runtime_application_disabled", "count": 6},
    {"id": "task_result_persistence_disabled", "count": 6},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "count": 12},
    {"id": "rollback_readback_not_executed", "count": 12},
    {"id": "operator_review_required", "count": 6},
    {"id": "scheduler_admission_not_enforced", "count": 5},
    {"id": "role_manifest_not_enforced", "count": 4},
    {"id": "runtime_application_residuals_not_promoted", "count": 7}
  ])
  and (.residual_blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .enforcement_stage_count == 10
  and (.enforcement_stages | map(.id) == [
    "unified_projection_contracts",
    "projection_adapter_runtime_application",
    "store_idempotency_guard_contracts",
    "store_guard_runtime_application",
    "terminal_task_result_contracts",
    "terminal_task_result_runtime_application",
    "append_only_store_preconditions",
    "append_only_store_runtime_enablement",
    "scheduler_admission_contracts",
    "role_manifest_contracts"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false))
  and (.enforcement_stages | map(select(.id == "append_only_store_preconditions" and .observed_contract_count == 7 and .ready_contract_count_before == 0 and .ready_contract_count_after == 7)) | length) == 1
  and (.enforcement_stages | map(select(.id == "append_only_store_runtime_enablement" and .observed_contract_count == 12 and .ready_contract_count_before == 0 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "scheduler_admission_contracts" and .observed_contract_count == 5 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "role_manifest_contracts" and .observed_contract_count == 4 and .ready_contract_count_after == 0)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 31
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_scheduler_admission_controller_preview_gate"
  and .ready_for_scheduler_admission_gap_closure_preview == true
  and .ready_for_projection_enforcement == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_terminal_task_result_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_store_readiness_rerun.rust_module_present == true
  and .source_probes.append_only_store_readiness_rerun.report_script_present == true
  and .source_probes.append_only_store_readiness_rerun.gate_script_present == true
  and .source_probes.append_only_store_precondition_application.rust_module_present == true
  and .source_probes.append_only_store_precondition_application.gate_script_present == true
  and .source_probes.append_only_store_precondition_application.upstream_gate == true
  and .source_probes.terminal_task_result_readiness_rerun.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_append_only_store_rerun --lib

echo "Hepta WorkGraph unified projection enforcement readiness append-only store rerun preview gate passed"
