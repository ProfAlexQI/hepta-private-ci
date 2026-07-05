#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-scheduler-admission-controller-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_scheduler_admission_controller_preview_gate"
  and .schema_version == "work_graph_scheduler_admission_controller_preview_v1"
  and .preview_mode == "dry_run_admission_explain_only_no_execution"
  and .check_count == 7
  and (.checks | length) == .check_count
  and (.checks | map(.id) == [
    "dependencies_terminal_ready",
    "lane_lease_available_and_owned",
    "approval_authority_present_when_required",
    "idempotency_replay_window_clear",
    "budget_and_timeout_available",
    "task_result_contract_preview_present",
    "side_effect_boundary_locked"
  ])
  and (.checks | all(.required == true and .blocks_execution == true and (.required_evidence_fields | length) >= 2))
  and .decision_count == 7
  and (.decisions | length) == .decision_count
  and (.decisions | map(.id) == [
    "allow_preview_only",
    "deny_blocked_dependency",
    "deny_missing_lane_lease",
    "deny_missing_required_approval",
    "deny_idempotency_conflict",
    "deny_budget_or_timeout_exhausted",
    "deny_task_result_contract_missing"
  ])
  and (.decisions[] | select(.id == "allow_preview_only") | .runnable_in_preview == true and .terminal_denial == false)
  and (.decisions | map(select(.id | startswith("deny_"))) | all(.runnable_in_preview == false and .terminal_denial == true))
  and .adapter_preview_count == 5
  and (.adapter_previews | length) == .adapter_preview_count
  and (.adapter_previews | map(.source_surface_id) == [
    "hepta_runtime_scheduler_store",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers"
  ])
  and (.adapter_previews | all(.enforcement_enabled == false))
  and (.adapter_previews | all((.applied_check_ids | length) == 7))
  and .recommended_next_gate == "hepta_work_graph_observability_timeline_preview_gate"
  and .ready_for_observability_timeline_preview == true
  and .ready_for_scheduler_cutover == false
  and .ready_for_live_execution == false
  and .source_probes.scheduler_admission_controller.rust_module_present == true
  and .source_probes.scheduler_admission_controller.report_script_present == true
  and .source_probes.scheduler_admission_controller.gate_script_present == true
  and .source_probes.task_result_contract.rust_module_present == true
  and .source_probes.task_result_contract.report_script_present == true
  and .source_probes.task_result_contract.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_scheduler_admission_controller --lib

echo "Hepta WorkGraph scheduler admission controller preview gate passed"
