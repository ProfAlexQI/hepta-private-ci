#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-enforcement-gap-closure-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_enforcement_gap_closure_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_enforcement_gap_closure_preview_no_runtime_attachment"
  and .terminal_task_result_blocker_source_count == 6
  and .wrapper_candidate_source_count == 6
  and .closure_plan_count == 6
  and .enforcement_binding_count == 6
  and .readback_probe_binding_count == 6
  and .application_group_count == 4
  and .terminal_source_blocker_ref_count == 6
  and .terminal_route_blocker_count_before == 6
  and .terminal_route_blocker_count_after_preview == 0
  and .readback_collection_assertion_ref_count == 18
  and .drift_detector_ref_count == 30
  and .application_guard_count == 9
  and .blocker_count == 9
  and .required_prior_gate_count == 24
' >/dev/null <<<"$report"

jq -e '
  (.closure_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.closure_plans | all(.readback_verified_by_preview == true))
  and (.closure_plans | all(.attaches_runtime_wrapper == false and .executes_wrapper == false and .persists_task_result == false and .enables_task_result_enforcement == false and .mutates_store == false))
  and (.closure_plans | all(.route_blocker_ids_before | index("terminal_task_result_enforcement_disabled")))
  and (.closure_plans | all((.route_blocker_ids_after_preview | index("terminal_task_result_enforcement_disabled")) == null))
  and (.closure_plans | map(select(.source_surface_id == "hepta_runtime_multi_agent_reducer" and .wrapper_id == "multi_agent_reducer_terminal_task_result_wrapper" and .readback_plan_id == "readback_fixture_multi_agent_reducer_ok" and .evidence_contract_id == "reducer_consensus_evidence")) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.enforcement_bindings | length) == 6
  and (.enforcement_bindings | all(.route_blocker_id == "terminal_task_result_enforcement_disabled" and .task_result_collection_id == "taskResults" and .timeline_collection_id == "timelineEvents"))
  and (.enforcement_bindings | all(.attaches_runtime_wrapper == false and .persists_task_result == false and .enables_task_result_enforcement == false))
  and (.readback_probe_bindings | length) == 6
  and (.readback_probe_bindings | all((.required_collection_assertion_ids | length) == 3 and (.drift_detector_ids | length) == 5))
  and (.readback_probe_bindings | all(.performs_readback == false and .persists_drift == false and .enables_task_result_enforcement == false))
' >/dev/null <<<"$report"

jq -e '
  (.application_groups | map(.id) == [
    "terminal_wrapper_contract_closure",
    "terminal_enforcement_binding_closure",
    "terminal_readback_probe_closure",
    "terminal_readiness_rerun_input_closure"
  ])
  and (.application_groups | all(.expected_contract_count_after_closure == 6 and .mutates_runtime == false and .persists_task_result == false and .enables_enforcement == false))
  and (.application_guards | map(.id) == [
    "closure_preview_only",
    "wrapper_runtime_attachment_disabled",
    "wrapper_execution_disabled",
    "task_result_persistence_disabled",
    "readback_execution_disabled",
    "terminal_task_result_enforcement_disabled",
    "scheduler_admission_or_role_manifest_residuals_not_enforced",
    "append_only_store_enablement_disabled",
    "enforcement_readiness_task_result_rerun_required"
  ])
  and (.application_guards | all(.required_before_projection_enforcement == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "terminal_task_result_closure_is_preview_only", "count": 6},
    {"id": "wrapper_runtime_attachment_disabled", "count": 6},
    {"id": "wrapper_execution_disabled", "count": 6},
    {"id": "task_result_persistence_disabled", "count": 6},
    {"id": "readback_execution_disabled", "count": 6},
    {"id": "terminal_task_result_enforcement_disabled", "count": 6},
    {"id": "scheduler_admission_or_role_manifest_residuals_not_enforced", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 6},
    {"id": "enforcement_readiness_task_result_rerun_missing", "count": 6}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate"
  and .ready_for_terminal_task_result_enforcement_gap_closure_readback_preview == true
  and .ready_for_runtime_wrapper_attachment == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_persistence == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_enforcement_gap_closure.rust_module_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure.report_script_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure.gate_script_present == true
  and .source_probes.store_guard_readiness_rerun.upstream_gate == true
  and .source_probes.terminal_task_result_wrapper.upstream_gate == true
  and .source_probes.terminal_task_result_wrapper_readback.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_enforcement_gap_closure --lib

echo "Hepta WorkGraph terminal TaskResult enforcement gap closure preview gate passed"
