#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-role-manifest-enforcement-gap-closure-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate"
  and .schema_version == "work_graph_role_manifest_enforcement_gap_closure_readback_preview_v1"
  and .preview_mode == "read_only_role_manifest_gap_closure_readback_no_execution"
  and .closure_plan_count == 4
  and .readback_plan_count == 4
  and .capability_binding_assertion_count == 4
  and .tool_permission_assertion_count == 4
  and .budget_lane_assertion_count == 4
  and .termination_output_assertion_count == 4
  and .guard_assertion_count == 10
  and .blocker_mapping_assertion_count == 10
  and .role_binding_ref_count == 24
  and .capability_ref_count == 13
  and .permission_mode_ref_count == 12
  and .manifest_field_ref_count == 27
  and .drift_detector_count == 6
  and .blocker_count == 11
  and .required_prior_gate_count == 37
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_agent_harness"
  ])
  and (.readback_plans | all(
    .required_before_closure_application == true
    and .readback_state == "readback_assertions_defined_execution_disabled"
    and (.role_binding_ids | length) == 6
    and (.capability_ids | length) >= 3
    and (.tool_permission_mode_ids | length) == 3
    and .performs_readback == false
    and .enforces_role_manifest == false
    and .changes_tool_permissions == false
    and .consumes_budget == false
    and .mutates_lane_binding == false
    and .starts_work == false
    and .spawns_agent == false
    and .writes_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.capability_binding_assertions | all(
    .expected_binding_state == "role_capability_binding_defined_enforcement_disabled"
    and (.capability_ids | length) >= 3
    and .enforces_role_manifest == false
    and .changes_tool_permissions == false
  ))
  and (.tool_permission_assertions | all(
    .expected_permission_state == "role_tool_permission_defined_no_permission_change"
    and (.tool_permission_mode_ids | length) == 3
    and .changes_tool_permissions == false
    and .mutates_runtime == false
  ))
  and (.budget_lane_assertions | all(
    .expected_budget_lane_state == "role_budget_lane_binding_defined_no_budget_or_lane_mutation"
    and .consumes_budget == false
    and .mutates_lane_binding == false
  ))
  and (.termination_output_assertions | all(
    .expected_terminal_contract_state == "role_terminal_contract_defined_no_work_or_agent_spawn"
    and .starts_work == false
    and .spawns_agent == false
  ))
  and (.termination_output_assertions | map(select(.source_surface_id == "agent_jobs_batch_workers" and .output_schema_ref_declared == true and .verifier_ref_declared == true)) | length) == 1
  and (.guard_assertions | all(
    .expected_guard_state == "guard_declared_satisfied_by_runtime_false"
    and .required_before_role_manifest_enforcement == true
    and .satisfied_by_readback_preview == false
    and .mutates_runtime == false
  ))
  and (.blocker_mapping_assertions | all(
    .expected_blocker_state == "blocks_role_manifest_until_readback_and_application_preview"
    and .required_before_role_manifest_enforcement == true
    and .performs_readback == false
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.drift_detectors | map(.id) == [
    "role_manifest_source_coverage_drift",
    "role_manifest_capability_binding_drift",
    "role_manifest_tool_permission_binding_drift",
    "role_manifest_budget_lane_binding_drift",
    "role_manifest_termination_output_binding_drift",
    "role_manifest_no_mutation_blocker_mapping_drift"
  ])
  and (.drift_detectors | all(.blocks_closure_application == true and .performs_readback == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 4},
    {"id": "role_manifest_enforcement_disabled", "count": 4},
    {"id": "role_capability_binding_not_enforced", "count": 4},
    {"id": "tool_permission_binding_not_enforced", "count": 4},
    {"id": "budget_lane_concurrency_not_enforced", "count": 4},
    {"id": "termination_output_schema_not_enforced", "count": 4},
    {"id": "scheduler_admission_runtime_application_disabled", "count": 3},
    {"id": "projection_timeline_runtime_residuals_not_promoted", "count": 4},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 4},
    {"id": "operator_review_required", "count": 4},
    {"id": "role_manifest_closure_application_missing", "count": 4}
  ])
  and (.blockers | all(.required_before_role_manifest_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_gate"
  and .ready_for_role_manifest_closure_application_preview == true
  and .ready_for_readback_execution == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.role_manifest_gap_closure_readback.rust_module_present == true
  and .source_probes.role_manifest_gap_closure_readback.report_script_present == true
  and .source_probes.role_manifest_gap_closure_readback.gate_script_present == true
  and .source_probes.role_manifest_gap_closure.upstream_gate == true
  and .source_probes.role_manifest_gap_closure.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_role_manifest_enforcement_gap_closure_readback --lib

echo "Hepta WorkGraph role manifest enforcement gap closure readback preview gate passed"
