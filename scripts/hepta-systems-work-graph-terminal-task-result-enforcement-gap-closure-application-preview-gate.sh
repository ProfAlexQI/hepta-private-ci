#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_enforcement_gap_closure_application_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_enforcement_gap_closure_application_preview_no_runtime_mutation"
  and .readback_plan_count == 6
  and .application_plan_count == 6
  and .source_outcome_count == 6
  and .source_terminal_task_result_contract_ready_preview_count == 6
  and .application_group_count == 4
  and .wire_field_ref_count == 66
  and .collection_assertion_ref_count == 18
  and .drift_detector_ref_count == 30
  and .terminal_source_blocker_ref_count == 6
  and .application_guard_count == 9
  and .blocker_count == 9
  and (.application_plans | length) == .application_plan_count
  and (.source_outcomes | length) == .source_outcome_count
  and (.application_groups | length) == .application_group_count
  and (.application_guards | length) == .application_guard_count
  and (.blockers | length) == .blocker_count
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.application_plans | all(
    .application_scope == "terminal_task_result_runtime_enforcement_binding"
    and .application_state == "preview_application_defined_terminal_task_result_enforcement_not_attached"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .attaches_runtime_wrapper == false
    and .executes_wrapper == false
    and .persists_task_result == false
    and .enables_task_result_enforcement == false
    and .enables_append_only_store == false
    and .enforces_projection == false
    and .mutates_store == false
    and (.required_wire_fields | length) == 11
    and (.required_collection_assertion_ids | length) == 3
    and (.drift_detector_ids | length) == 5
    and (.terminal_source_blocker_ids | length) == 1
  ))
  and (.application_plans | map(select(
    .source_surface_id == "hepta_runtime_scheduler_store"
    and .wrapper_id == "scheduler_run_terminal_task_result_wrapper"
    and .terminal_source_kind == "scheduler_run"
    and .expected_evidence_contract_id == "scheduler_admission_decision_evidence"
  )) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.source_outcomes | all(
    .post_application_terminal_task_result_state == "terminal_task_result_contract_ready_preview_after_application"
    and .terminal_task_result_contract_ready_preview == true
    and .ready_for_enforcement_readiness_terminal_task_result_rerun == true
    and .ready_for_task_result_enforcement == false
    and .ready_for_projection_enforcement == false
    and .applies_to_runtime == false
  ))
  and (.application_groups | map({id, count: (.application_plan_ids | length)}) == [
    {"id": "multi_agent_terminal_task_result_application", "count": 2},
    {"id": "batch_agent_jobs_terminal_task_result_application", "count": 1},
    {"id": "runtime_scheduler_terminal_task_result_application", "count": 2},
    {"id": "agent_harness_terminal_task_result_application", "count": 1}
  ])
  and (.application_groups | all(
    .priority == "p0"
    and .mutates_runtime == false
    and .persists_task_result == false
    and .enables_task_result_enforcement == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_guards | map(.id) == [
    "runtime_wrapper_attachment_disabled",
    "wrapper_execution_disabled",
    "task_result_persistence_disabled",
    "terminal_task_result_enforcement_disabled",
    "append_only_store_enablement_disabled",
    "scheduler_admission_enforcement_disabled",
    "role_manifest_enforcement_disabled",
    "operator_review_required",
    "enforcement_readiness_task_result_rerun_required"
  ])
  and (.application_guards | all(.required_before_projection_enforcement == true and .satisfied_by_preview == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "terminal_task_result_application_is_preview_only", "count": 6},
    {"id": "wrapper_runtime_attachment_disabled", "count": 6},
    {"id": "wrapper_execution_disabled", "count": 6},
    {"id": "task_result_persistence_disabled", "count": 6},
    {"id": "terminal_task_result_enforcement_disabled", "count": 6},
    {"id": "scheduler_admission_or_role_manifest_residuals_not_enforced", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 6},
    {"id": "enforcement_readiness_task_result_rerun_missing", "count": 6},
    {"id": "operator_review_required", "count": 6}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 26
  and (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview == true
  and .ready_for_runtime_wrapper_attachment == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_persistence == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_enforcement_gap_closure_application.rust_module_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_application.report_script_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_application.gate_script_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_readback.upstream_gate == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_enforcement_gap_closure_application --lib

echo "Hepta WorkGraph terminal TaskResult enforcement gap closure application preview gate passed"
