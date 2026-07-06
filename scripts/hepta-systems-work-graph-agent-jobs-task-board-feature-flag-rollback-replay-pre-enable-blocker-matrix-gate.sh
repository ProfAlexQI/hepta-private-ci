#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-rollback-replay-pre-enable-blocker-matrix-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-rollback-replay-pre-enable-blocker-matrix-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_v1"
  and .preview_mode == "rollback_replay_pre_enable_blocker_matrix_no_execution_no_enablement"
  and .source_non_send_readback_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate"
  and .source_readback_entry_count == 4
  and .source_readback_blocker_count == 8
  and .rollback_replay_check_count == 6
  and .pre_enable_blocker_count == 10
  and (.rollback_replay_checks | all(
    .rollback_anchor_ref == "agent_jobs_task_board_feature_flag_canary_rollback_anchor"
    and .deterministic == true
    and .diff_required == true
    and .executed == false
    and .passed_preview == true
  ))
  and (.pre_enable_blockers | map(.blocked_action) == [
    "send_operator_packet",
    "accept_operator_packet",
    "record_operator_approval",
    "write_feature_flag_config",
    "mutate_feature_flag_state",
    "route_canary_traffic",
    "execute_replay",
    "execute_rollback",
    "enforce_scheduler_admission",
    "perform_live_cutover"
  ])
  and (.pre_enable_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 8
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate"
  and .rollback_anchor_present == true
  and .deterministic_replay_required == true
  and .replay_diff_required == true
  and .rollback_rehearsal_required == true
  and .replay_executed == false
  and .rollback_executed == false
  and .ready_for_enablement_precondition_dry_run == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.rollback_replay_module_present == true
  and .source_probes.non_send_readback_gate_present == true
  and .source_probes.non_send_readback_gate_points_here == true
  and .source_probes.non_send_readback_ready_present == true
  and .source_probes.operator_packet_unsent_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag rollback/replay pre-enable blocker matrix gate passed"
