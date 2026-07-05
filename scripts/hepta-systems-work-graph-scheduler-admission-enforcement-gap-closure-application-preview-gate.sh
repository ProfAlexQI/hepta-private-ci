#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate"
  and .schema_version == "work_graph_scheduler_admission_enforcement_gap_closure_application_preview_v1"
  and .preview_mode == "read_only_scheduler_admission_gap_closure_application_preview_no_runtime_mutation"
  and .readback_plan_count == 5
  and .application_plan_count == 5
  and .source_outcome_count == 5
  and .scheduler_admission_contract_ready_preview_count == 5
  and .blocker_application_count == 10
  and .application_group_count == 4
  and .admission_check_ref_count == 35
  and .admission_decision_ref_count == 35
  and .evidence_field_ref_count == 90
  and .application_guard_count == 13
  and .blocker_count == 13
  and (.application_plans | length) == .application_plan_count
  and (.source_outcomes | length) == .source_outcome_count
  and (.blocker_applications | length) == .blocker_application_count
  and (.application_groups | length) == .application_group_count
  and (.application_guards | length) == .application_guard_count
  and (.blockers | length) == .blocker_count
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store"
  ])
  and (.application_plans | all(
    .application_scope == "scheduler_admission_runtime_enforcement_binding"
    and .application_state == "preview_application_defined_scheduler_admission_not_attached"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .enforces_scheduler_admission == false
    and .starts_work == false
    and .acquires_lease == false
    and .consumes_budget == false
    and .records_approval == false
    and .mutates_idempotency_index == false
    and .writes_store == false
    and .enables_append_only_store == false
    and .enforces_role_manifest == false
    and (.admission_check_ids | length) == 7
    and (.admission_decision_ids | length) == 7
    and (.required_evidence_fields | length) == 18
  ))
  and (.application_plans | map(select(
    .source_surface_id == "hepta_runtime_scheduler_store"
    and .target_node_kind == "scheduler_run"
    and .scheduler_blocker_id == "scheduler_run_admission_not_enforced"
    and .readback_probe_id == "scheduler_admission_readback_probe:hepta_runtime_scheduler_store"
  )) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.source_outcomes | all(
    .post_application_scheduler_admission_state == "scheduler_admission_contract_ready_preview_after_application"
    and .scheduler_admission_contract_ready_preview == true
    and .ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview == true
    and .ready_for_scheduler_admission_enforcement == false
    and .applies_to_runtime == false
  ))
  and (.blocker_applications | all(
    .expected_blocker_state == "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked"
    and .blocker_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .clears_runtime_blocker == false
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_groups | map({id, count: (.application_plan_ids | length)}) == [
    {"id": "dependency_and_task_contract_admission_application", "count": 5},
    {"id": "lease_budget_idempotency_admission_application", "count": 5},
    {"id": "approval_and_side_effect_lock_admission_application", "count": 5},
    {"id": "scheduler_source_adapter_binding_application", "count": 5}
  ])
  and (.application_groups | all(
    .priority == "p0"
    and .expected_scheduler_admission_ready_source_count_after_application == 5
    and .mutates_runtime == false
    and .enforces_scheduler_admission == false
    and .starts_work == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_guards | map(.id) == [
    "scheduler_admission_application_is_preview_only",
    "readback_execution_disabled",
    "scheduler_admission_enforcement_disabled",
    "lane_lease_acquisition_disabled",
    "dependency_readback_not_executed",
    "approval_recording_disabled",
    "idempotency_index_mutation_disabled",
    "budget_consumption_disabled",
    "role_manifest_residuals_not_enforced",
    "projection_timeline_runtime_residuals_not_promoted",
    "append_only_store_runtime_enablement_disabled",
    "operator_review_required",
    "enforcement_readiness_scheduler_admission_rerun_required"
  ])
  and (.application_guards | all(
    .required_before_scheduler_admission_enforcement == true
    and .satisfied_by_preview == false
  ))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "scheduler_admission_application_is_preview_only", "count": 5},
    {"id": "readback_execution_disabled", "count": 5},
    {"id": "scheduler_admission_enforcement_disabled", "count": 5},
    {"id": "lane_lease_acquisition_disabled", "count": 5},
    {"id": "dependency_readback_not_executed", "count": 5},
    {"id": "approval_recording_disabled", "count": 5},
    {"id": "idempotency_index_mutation_disabled", "count": 5},
    {"id": "budget_consumption_disabled", "count": 5},
    {"id": "role_manifest_residuals_not_enforced", "count": 3},
    {"id": "projection_timeline_runtime_residuals_not_promoted", "count": 4},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 5},
    {"id": "operator_review_required", "count": 5},
    {"id": "scheduler_admission_readiness_rerun_missing", "count": 5}
  ])
  and (.blockers | all(.required_before_scheduler_admission_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 34
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview == true
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.scheduler_admission_gap_closure_application.rust_module_present == true
  and .source_probes.scheduler_admission_gap_closure_application.report_script_present == true
  and .source_probes.scheduler_admission_gap_closure_application.gate_script_present == true
  and .source_probes.scheduler_admission_gap_closure_readback.upstream_gate == true
  and .source_probes.scheduler_admission_gap_closure_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_scheduler_admission_enforcement_gap_closure_application --lib

echo "Hepta WorkGraph scheduler admission enforcement gap closure application preview gate passed"
