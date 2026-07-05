#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-role-manifest-enforcement-gap-closure-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"
  and .schema_version == "work_graph_role_manifest_enforcement_gap_closure_preview_v1"
  and .preview_mode == "read_only_role_manifest_gap_closure_no_enforcement"
  and .role_blocked_source_count == 4
  and .contract_adapter_count == 4
  and .manifest_required_field_count == 12
  and .capability_count == 7
  and .permission_mode_count == 5
  and .closure_plan_count == 4
  and .role_binding_count == 24
  and .capability_ref_count == 13
  and .permission_mode_ref_count == 12
  and .manifest_field_ref_count == 27
  and .closure_group_count == 5
  and .guard_count == 10
  and .blocker_count == 10
  and (.closure_plans | length) == .closure_plan_count
  and (.closure_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_agent_harness"
  ])
  and (.closure_plans | map(.role_blocker_id) == [
    "multi_agent_v2_role_manifest_not_enforced",
    "agent_jobs_role_manifest_not_enforced",
    "worker_task_role_manifest_not_enforced",
    "agent_harness_role_manifest_not_enforced"
  ])
' >/dev/null <<<"$report"

jq -e '
  (.closure_plans | all(
    (.covered_wire_fields | index("roleId"))
    and (.covered_wire_fields | index("budget"))
    and (.capability_ids | length) >= 3
    and (.tool_permission_mode_ids | length) == 3
    and (.role_binding_ids | length) == 6
    and .ready_for_readback_preview == true
    and .applies_to_runtime == false
    and .enforces_role_manifest == false
    and .changes_tool_permissions == false
    and .consumes_budget == false
    and .starts_work == false
    and .spawns_agent == false
    and .writes_store == false
  ))
  and (.closure_groups | map(.id) == [
    "role_capability_binding_closure",
    "tool_permission_binding_closure",
    "budget_lane_concurrency_binding_closure",
    "termination_output_schema_binding_closure",
    "trace_approval_readback_binding_closure"
  ])
  and (.closure_groups | all(.mutates_runtime == false and .enforces_role_manifest == false and (.closure_plan_ids | length) == 4))
' >/dev/null <<<"$report"

jq -e '
  (.guards | map(.id) == [
    "role_manifest_closure_is_preview_only",
    "role_contract_adapter_required",
    "capability_permission_binding_required",
    "output_schema_verifier_required",
    "budget_concurrency_lane_required",
    "trace_policy_required",
    "role_manifest_not_enforced",
    "tool_permissions_not_changed",
    "scheduler_admission_runtime_not_applied",
    "append_only_store_runtime_not_enabled"
  ])
  and (.guards | all(.required_before_role_manifest_enforcement == true and .satisfied_by_preview == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "role_manifest_enforcement_disabled", "count": 4},
    {"id": "role_capability_binding_not_enforced", "count": 4},
    {"id": "tool_permission_binding_not_enforced", "count": 4},
    {"id": "budget_lane_concurrency_not_enforced", "count": 4},
    {"id": "termination_output_schema_not_enforced", "count": 4},
    {"id": "role_manifest_closure_readback_missing", "count": 4},
    {"id": "scheduler_admission_runtime_application_disabled", "count": 3},
    {"id": "projection_timeline_runtime_residuals_not_promoted", "count": 4},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 4},
    {"id": "operator_review_required", "count": 4}
  ])
  and (.blockers | all(.required_before_role_manifest_enforcement == true))
  and (.blockers | all((.affected_closure_plan_ids | length) == (.affected_source_surface_ids | length)))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 36
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates | index("hepta_work_graph_role_manifest_contract_preview_gate"))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate"
  and .ready_for_role_manifest_readback_preview == true
  and .ready_for_role_manifest_application_preview == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.role_manifest_gap_closure.rust_module_present == true
  and .source_probes.role_manifest_gap_closure.report_script_present == true
  and .source_probes.role_manifest_gap_closure.gate_script_present == true
  and .source_probes.role_manifest_contract.rust_module_present == true
  and .source_probes.role_manifest_contract.gate_script_present == true
  and .source_probes.role_manifest_contract.upstream_gate == true
  and .source_probes.scheduler_admission_rerun.gate_script_present == true
  and .source_probes.scheduler_admission_rerun.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_role_manifest_enforcement_gap_closure --lib

echo "Hepta WorkGraph role manifest enforcement gap closure preview gate passed"
