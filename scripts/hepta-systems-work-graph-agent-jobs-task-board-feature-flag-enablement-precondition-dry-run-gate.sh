#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-dry-run-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-dry-run-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_v1"
  and .preview_mode == "feature_flag_enablement_precondition_dry_run_deny_no_write_no_enablement"
  and .source_rollback_replay_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate"
  and .source_rollback_replay_check_count == 6
  and .source_pre_enable_blocker_count == 10
  and .source_required_prior_gate_count == 8
  and .decision_count == 2
  and .deny_reason_count == 10
  and (.decisions | map(.flag_id) == [
    "work_graph_agent_jobs_non_blocking_canary",
    "work_graph_task_board_non_blocking_canary"
  ])
  and (.decisions | all(
    .requested_action == "enable_feature_flag_non_blocking_canary"
    and .decision == "deny"
    and (.deny_reason_ids | length) == 10
    and .canary_stage == "shadow_0ppm_report_only"
    and .requested_traffic_ppm == 1
    and .allowed_traffic_ppm == 0
    and .config_write_allowed == false
    and .feature_flag_enablement_allowed == false
    and .canary_traffic_allowed == false
    and .live_cutover_allowed == false
  ))
  and (.deny_reasons | map(.id) == [
    "operator_packet_unsent",
    "operator_packet_unaccepted",
    "approval_record_missing",
    "config_write_disabled",
    "feature_flag_state_current_off",
    "canary_traffic_zero_ppm",
    "replay_not_executed",
    "rollback_not_executed",
    "scheduler_admission_not_enforced",
    "live_cutover_disabled"
  ])
  and (.deny_reasons | all(.required == true and .satisfied == false))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 9
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate"
  and .source_rollback_replay_preconditions_complete == true
  and .source_rollback_replay_no_execution_confirmed == true
  and .source_rollback_replay_no_enablement_confirmed == true
  and .source_rollback_replay_ready == true
  and .dry_run_decisions_deny_complete == true
  and .deny_reasons_unsatisfied_complete == true
  and .enablement_precondition_dry_run_preconditions_complete == true
  and .dry_run_mode == "deny_only_precondition_explanation"
  and .dry_run_enforced == false
  and .allow_count == 0
  and .deny_count == 2
  and .config_write_allowed == false
  and .feature_flag_enablement_allowed == false
  and .canary_traffic_allowed == false
  and .live_cutover_allowed == false
  and .approval_acceptance_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .ready_for_denial_readback == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.enablement_module_present == true
  and .source_probes.rollback_replay_matrix_gate_present == true
  and .source_probes.rollback_replay_matrix_points_here == true
  and .source_probes.rollback_replay_matrix_report_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate"
  and .source_probes.rollback_replay_matrix_preconditions_complete == true
  and .source_probes.rollback_replay_matrix_ready_for_dry_run == true
  and .source_probes.rollback_replay_matrix_side_effects_all_false == true
  and .source_probes.rollback_replay_execution_false_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag enablement precondition dry-run gate passed"
