#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-scheduler-admission-enforcement-gap-closure-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_gate"
  and .schema_version == "work_graph_scheduler_admission_enforcement_gap_closure_preview_v1"
  and .preview_mode == "read_only_scheduler_admission_gap_closure_no_enforcement"
  and .scheduler_blocked_source_count == 5
  and .controller_check_count == 7
  and .controller_decision_count == 7
  and .controller_adapter_count == 5
  and .closure_plan_count == 5
  and .admission_binding_count == 5
  and .readback_probe_binding_count == 5
  and .evidence_field_ref_count == 90
  and .closure_group_count == 4
  and .guard_count == 9
  and .blocker_count == 10
  and (.closure_plans | length) == .closure_plan_count
  and (.closure_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store"
  ])
  and (.closure_plans | all(.scheduler_blocker_id | endswith("_admission_not_enforced")))
  and (.closure_plans | all((.admission_check_ids | length) == 7 and (.admission_decision_ids | length) == 7 and (.required_evidence_fields | length) == 18 and .ready_for_readback_preview == true))
' >/dev/null <<<"$report"

jq -e '
  (.closure_plans | all(.applies_to_runtime == false and .enforces_scheduler_admission == false and .starts_work == false and .acquires_lease == false and .writes_store == false and .mutates_idempotency_index == false and .records_approval == false))
  and (.closure_groups | map(.id) == [
    "dependency_and_task_contract_admission_closure",
    "lease_budget_idempotency_admission_closure",
    "approval_and_side_effect_lock_admission_closure",
    "scheduler_source_adapter_binding_closure"
  ])
  and (.closure_groups | all(.mutates_runtime == false and .enforces_scheduler_admission == false and (.closure_plan_ids | length) == 5))
  and (.guards | map(.id) == [
    "scheduler_admission_closure_is_preview_only",
    "controller_adapter_contract_required",
    "dependency_terminal_evidence_required",
    "lane_lease_not_acquired",
    "approval_not_recorded",
    "idempotency_index_not_mutated",
    "budget_not_consumed",
    "scheduler_admission_not_enforced",
    "append_only_store_runtime_not_enabled"
  ])
  and (.guards | all(.required_before_scheduler_admission_enforcement == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "scheduler_admission_enforcement_disabled", "count": 5},
    {"id": "lane_lease_acquisition_disabled", "count": 5},
    {"id": "dependency_readback_not_executed", "count": 5},
    {"id": "approval_recording_disabled", "count": 5},
    {"id": "idempotency_index_mutation_disabled", "count": 5},
    {"id": "budget_consumption_disabled", "count": 5},
    {"id": "role_manifest_residuals_not_enforced", "count": 3},
    {"id": "projection_timeline_runtime_residuals_not_promoted", "count": 4},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 5},
    {"id": "scheduler_admission_closure_readback_missing", "count": 5}
  ])
  and (.blockers | all(.required_before_scheduler_admission_enforcement == true))
  and (.blockers | all((.affected_closure_plan_ids | length) == (.affected_source_surface_ids | length)))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 32
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate"
  and .ready_for_scheduler_admission_readback_preview == true
  and .ready_for_scheduler_admission_application_preview == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.scheduler_admission_gap_closure.rust_module_present == true
  and .source_probes.scheduler_admission_gap_closure.report_script_present == true
  and .source_probes.scheduler_admission_gap_closure.gate_script_present == true
  and .source_probes.scheduler_admission_controller.rust_module_present == true
  and .source_probes.scheduler_admission_controller.gate_script_present == true
  and .source_probes.scheduler_admission_controller.upstream_gate == true
  and .source_probes.append_only_store_rerun.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_scheduler_admission_enforcement_gap_closure --lib

echo "Hepta WorkGraph scheduler admission enforcement gap closure preview gate passed"
