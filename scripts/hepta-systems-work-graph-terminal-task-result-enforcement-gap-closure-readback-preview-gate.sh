#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_enforcement_gap_closure_readback_no_execution"
  and .closure_plan_count == 6
  and .enforcement_binding_count == 6
  and .readback_probe_binding_count == 6
  and .readback_plan_count == 6
  and .wrapper_binding_assertion_count == 6
  and .enforcement_binding_assertion_count == 6
  and .readback_probe_assertion_count == 6
  and .wire_field_assertion_count == 6
  and .wire_field_ref_count == 66
  and .collection_assertion_ref_count == 18
  and .drift_detector_ref_count == 30
  and .drift_detector_count == 5
  and .blocker_count == 9
  and .required_prior_gate_count == 25
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.readback_plans | all(
    .required_before_closure_application == true
    and .readback_state == "readback_assertions_defined_execution_disabled"
    and .performs_readback == false
    and .attaches_runtime_wrapper == false
    and .executes_wrapper == false
    and .persists_task_result == false
    and .enables_task_result_enforcement == false
    and .mutates_store == false
    and (.required_wire_fields | length) == 11
    and (.required_collection_assertion_ids | length) == 3
    and (.drift_detector_ids | length) == 5
  ))
  and (.readback_plans | map(select(
    .source_surface_id == "hepta_runtime_multi_agent_reducer"
    and .wrapper_id == "multi_agent_reducer_terminal_task_result_wrapper"
    and .wrapper_readback_plan_id == "readback_fixture_multi_agent_reducer_ok"
    and .expected_evidence_contract_id == "reducer_consensus_evidence"
  )) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.wrapper_binding_assertions | all(
    .expected_wrapper_state == "wrapper_contract_defined_runtime_attachment_disabled"
    and .attaches_runtime_wrapper == false
    and .executes_wrapper == false
    and (.terminal_source_blocker_ids | length) == 1
  ))
  and (.enforcement_binding_assertions | all(
    .task_result_collection_id == "taskResults"
    and .timeline_collection_id == "timelineEvents"
    and .route_blocker_id == "terminal_task_result_enforcement_disabled"
    and .expected_binding_state == "preview_binding_defined_runtime_attachment_disabled"
    and .attaches_runtime_wrapper == false
    and .persists_task_result == false
    and .enables_task_result_enforcement == false
  ))
  and (.readback_probe_assertions | all(
    .expected_probe_state == "preview_probe_binding_defined_readback_execution_disabled"
    and (.required_collection_assertion_ids | length) == 3
    and (.drift_detector_ids | length) == 5
    and .performs_readback == false
    and .persists_drift == false
    and .enables_task_result_enforcement == false
  ))
  and (.wire_field_assertions | all(
    .required_field_count == 11
    and .expected_wire_state == "task_result_wire_contract_defined_no_execution"
    and .enables_task_result_enforcement == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.drift_detectors | map(.id) == [
    "terminal_task_result_identity_drift",
    "terminal_task_result_status_drift",
    "terminal_task_result_evidence_drift",
    "terminal_task_result_readback_probe_drift",
    "terminal_task_result_redaction_drift"
  ])
  and (.drift_detectors | all(.blocks_closure_application == true and .performs_readback == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 6},
    {"id": "wrapper_runtime_attachment_disabled", "count": 6},
    {"id": "wrapper_execution_disabled", "count": 6},
    {"id": "task_result_persistence_disabled", "count": 6},
    {"id": "terminal_task_result_enforcement_disabled", "count": 6},
    {"id": "scheduler_admission_or_role_manifest_residuals_not_enforced", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 6},
    {"id": "terminal_task_result_closure_application_missing", "count": 6},
    {"id": "operator_review_required", "count": 6}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate"
  and .ready_for_terminal_task_result_enforcement_gap_closure_application_preview == true
  and .ready_for_runtime_wrapper_attachment == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_persistence == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_enforcement_gap_closure_readback.rust_module_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_readback.report_script_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure_readback.gate_script_present == true
  and .source_probes.terminal_task_result_enforcement_gap_closure.upstream_gate == true
  and .source_probes.terminal_task_result_enforcement_gap_closure.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_enforcement_gap_closure_readback --lib

echo "Hepta WorkGraph terminal TaskResult enforcement gap closure readback preview gate passed"
