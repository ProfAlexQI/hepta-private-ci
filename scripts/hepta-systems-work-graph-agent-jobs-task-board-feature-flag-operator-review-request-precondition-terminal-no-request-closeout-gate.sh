#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-terminal-no-request-closeout-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-terminal-no-request-closeout-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_v1"
  and .preview_mode == "operator_review_request_precondition_terminal_no_request_closeout_report_only"
  and .source_non_persistence_readback_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate"
  and .source_readback_entry_count == 5
  and .source_readback_blocker_count == 20
  and .source_required_prior_gate_count == 20
  and .source_non_persistence_readback_preconditions_complete == true
  and .source_non_persistence_readback_no_request_confirmed == true
  and .source_non_persistence_readback_no_authorization_confirmed == true
  and .source_non_persistence_readback_ready == true
  and .closeout_entry_count == 7
  and .closeout_blocker_count == 21
  and .required_prior_gate_count == 21
  and .closeout_scope.closeout_mode == "operator_review_request_precondition_terminal_no_request_closeout_report_only"
  and .closeout_scope.closeout_visible == true
  and .closeout_scope.closeout_recorded == false
  and .closeout_scope.closeout_persisted == false
  and .closeout_scope.closeout_authoritative == false
  and .closeout_scope.closeout_accepted == false
  and .closeout_scope.terminal_no_request == true
  and .closeout_scope.operator_review_requested == false
  and .closeout_scope_terminal_no_request_complete == true
  and (.closeout_entries | map(.id) == [
    "terminal_no_request_decision_closeout",
    "terminal_denial_readback_chain_closeout",
    "terminal_audit_index_chain_closeout",
    "terminal_no_operator_packet_closeout",
    "terminal_no_approval_config_flag_traffic_closeout",
    "terminal_no_persistence_replay_rollback_closeout",
    "terminal_no_live_cutover_closeout"
  ])
  and (.closeout_entries | all(
    .terminal == true
    and .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .operator_review_requested == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and .closeout_entries_terminal_no_request_complete == true
  and (.closeout_blockers | map(.blocked_action) == [
    "persist_terminal_no_request_closeout_readback",
    "record_terminal_no_request_closeout",
    "persist_terminal_no_request_closeout",
    "accept_terminal_no_request_closeout",
    "request_operator_review",
    "record_operator_review_request",
    "persist_operator_review_request",
    "accept_operator_review_request",
    "send_operator_packet",
    "accept_operator_packet",
    "record_operator_approval",
    "write_feature_flag_config",
    "enable_feature_flag",
    "route_canary_traffic",
    "enforce_scheduler_admission",
    "enable_guardrail_enforcement",
    "execute_replay",
    "execute_rollback",
    "persist_work_graph_projection",
    "record_work_graph_event",
    "perform_live_cutover"
  ])
  and (.closeout_blockers | all(.blocked == true))
  and .closeout_blockers_complete == true
  and .terminal_no_request_closeout_preconditions_complete == true
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_non_request_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate",
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate"
  and .terminal_closeout_visible == true
  and .terminal_closeout_recorded == false
  and .terminal_closeout_persisted == false
  and .terminal_closeout_authoritative == false
  and .terminal_closeout_accepted == false
  and .terminal_no_request == true
  and .operator_review_request_allowed == false
  and .operator_review_requested == false
  and .operator_packet_send_allowed == false
  and .operator_packet_acceptance_allowed == false
  and .approval_recording_allowed == false
  and .config_write_allowed == false
  and .feature_flag_enablement_allowed == false
  and .canary_traffic_allowed == false
  and .scheduler_enforcement_allowed == false
  and .guardrail_enforcement_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .work_graph_persistence_allowed == false
  and .live_cutover_allowed == false
  and .ready_for_terminal_no_request_closeout_readback == true
  and .ready_for_operator_review_request == false
  and .ready_for_approval_recording == false
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.terminal_closeout_module_present == true
  and .source_probes.non_persistence_readback_gate_present == true
  and .source_probes.non_persistence_readback_points_here == true
  and .source_probes.non_persistence_readback_unrequested_present == true
  and .source_probes.non_persistence_readback_ready_present == true
  and .source_probes.non_persistence_readback_unpersisted_present == true
  and .source_probes.non_persistence_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate"
  and .source_probes.non_persistence_readback_preconditions_complete == true
  and .source_probes.non_persistence_readback_ready_for_terminal_closeout == true
  and .source_probes.non_persistence_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag operator review request precondition terminal no-request closeout gate passed"
