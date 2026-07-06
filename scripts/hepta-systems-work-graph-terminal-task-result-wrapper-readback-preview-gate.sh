#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-wrapper-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-wrapper-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_wrapper_readback_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_wrapper_readback_preview_no_execution"
  and .readback_plan_count == 8
  and (.readback_plans | length) == .readback_plan_count
  and (.readback_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_task_board",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.readback_plans | all(
    .expected_task_result_collection_id == "taskResults"
    and .expected_timeline_collection_id == "timelineEvents"
    and .readback_state == "preview_contract_defined_readback_execution_disabled"
    and .performs_readback == false
    and .persists_drift == false
    and .mutates_store == false
    and .enforces_task_result == false
  ))
  and .collection_assertion_count == 5
  and (.collection_assertions | length) == .collection_assertion_count
  and (.collection_assertions | map(.collection_id) | unique == [
    "artifacts",
    "schedulerRefs",
    "taskResults",
    "timelineEvents",
    "verifierRefs"
  ])
  and (.collection_assertions | all(.blocks_wrapper_execution == true and .performs_readback == false and .mutates_store == false))
  and .drift_detector_count == 5
  and (.drift_detectors | length) == .drift_detector_count
  and (.drift_detectors | map(.id) == [
    "detect_fixture_identity_drift",
    "detect_fixture_status_drift",
    "detect_fixture_evidence_drift",
    "detect_fixture_verifier_drift",
    "detect_fixture_redaction_drift"
  ])
  and (.drift_detectors | all(.severity == "critical" and .blocks_wrapper_execution == true and .persists_drift == false))
  and (.readback_plans | all(.drift_detector_ids | length == 5))
  and .blocker_count == 4
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) == [
    "readback_execution_disabled",
    "drift_persistence_disabled",
    "wrapper_execution_disabled",
    "task_result_enforcement_disabled"
  ])
  and (.blockers | map(select(.severity == "high")) | length) == 2
  and (.blockers | all(.required_before_readback_execution == true))
  and .required_prior_gate_count == 14
  and (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate"
  and .ready_for_drift_budget_preview == true
  and .ready_for_readback_execution == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_wrapper_readback.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_readback.report_script_present == true
  and .source_probes.terminal_task_result_wrapper_readback.gate_script_present == true
  and .source_probes.terminal_task_result_wrapper_fixture.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_fixture.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_wrapper_readback --lib

echo "Hepta WorkGraph terminal TaskResult wrapper readback preview gate passed"
