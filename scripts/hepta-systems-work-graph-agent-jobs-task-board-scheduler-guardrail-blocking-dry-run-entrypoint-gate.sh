#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_v1"
  and .preview_mode == "blocking_guardrail_dry_run_before_entrypoint_no_live_enforcement"
  and .source_scheduler_gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  and .source_scheduler_entrypoint_count == 4
  and .source_scheduler_check_count == 7
  and .source_trace_guardrail_gate == "hepta_work_graph_trace_guardrail_span_report_only_gate"
  and .source_trace_span_count == 9
  and .source_blocking_guardrail_count == 6
  and .source_entrypoint_emission_gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  and .source_emission_count == 2
  and .source_final_closeout_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate"
  and .source_final_closeout_entry_count == 8
  and .entrypoint_binding_count == 4
  and .guardrail_check_count == 8
  and .dry_run_decision_count == 4
  and .required_prior_gate_count == 4
  and (.entrypoint_bindings | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoint_bindings | all(
    .dry_run_decision == "deny_live_allow_report_only"
    and .would_block_if_live == true
    and .dry_run_allows_current_runtime_to_continue == true
    and .live_blocking_enabled == false
    and (.applied_check_ids | length == 8)
    and (.required_trace_fields | length == 6)
  ))
  and (.guardrail_checks | map(.id) == [
    "dependencies_terminal_ready",
    "lane_lease_available_and_owned",
    "approval_authority_present_when_required",
    "idempotency_replay_window_clear",
    "budget_and_timeout_available",
    "task_result_contract_preview_present",
    "side_effect_boundary_locked",
    "trace_guardrail_span_present"
  ])
  and (.guardrail_checks | all(
    .blocks_live_execution == true
    and .dry_run_explanation_required == true
  ))
  and (.dry_run_decisions | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.dry_run_decisions | all(
    .outcome == "deny_live_allow_report_only"
    and .allow_current_runtime_to_continue == true
    and .block_live_execution == true
    and (.trace_id | length > 0)
  ))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  and .scheduler_admission_dry_run_present == true
  and .blocking_guardrail_dry_run_attached == true
  and .pre_entrypoint_hook_contract_ready == true
  and .live_blocking_enforcement_enabled == false
  and .runtime_interception_enabled == false
  and .work_graph_event_persistence_enabled == false
  and .ready_for_work_graph_shadow_event_store_readback == true
  and .ready_for_live_execution == false
  and .source_probes.entrypoint_module_present == true
  and .source_probes.final_closeout_gate_present == true
  and .source_probes.final_closeout_points_here == true
  and .source_probes.scheduler_dry_run_present == true
  and .source_probes.trace_guardrail_present == true
  and .source_probes.entrypoint_emission_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint --lib

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint gate passed"
