#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-blocker-matrix-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-blocker-matrix-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_v1"
  and .preview_mode == "operator_review_request_precondition_blocker_matrix_deny_only_no_request"
  and .source_non_persistence_readback_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate"
  and .source_readback_entry_count == 5
  and .source_readback_blocker_count == 16
  and .source_required_prior_gate_count == 16
  and .request_precondition_check_count == 12
  and .request_precondition_satisfied_count == 2
  and .request_precondition_unsatisfied_count == 10
  and .request_precondition_blocking_count == 10
  and .request_blocker_count == 17
  and (.request_precondition_checks | map(.id) == [
    "non_request_audit_index_non_persistence_readback_ready",
    "required_prior_chain_visible",
    "operator_review_request_authorization_missing",
    "operator_review_request_recording_authorization_missing",
    "operator_packet_send_boundary_unsatisfied",
    "operator_packet_acceptance_missing",
    "approval_recording_authorization_missing",
    "config_write_authorization_missing",
    "feature_flag_enablement_disabled",
    "canary_traffic_disallowed",
    "scheduler_guardrail_enforcement_missing",
    "replay_rollback_live_cutover_missing"
  ])
  and (.request_precondition_checks | map(select(.satisfied == true)) | length) == 2
  and (.request_precondition_checks | map(select(.blocking == true)) | length) == 10
  and (.request_blockers | map(.blocked_action) == [
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
  and (.request_blockers | all(.blocked == true))
  and .required_prior_gates == [
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
  and .required_prior_gate_count == 17
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate"
  and .source_non_persistence_readback_preconditions_complete == true
  and .source_non_persistence_readback_no_request_confirmed == true
  and .source_non_persistence_readback_no_authorization_confirmed == true
  and .source_non_persistence_readback_ready == true
  and .request_precondition_checks_complete == true
  and .request_blockers_complete == true
  and .request_precondition_blocker_matrix_preconditions_complete == true
  and .request_decision == "deny"
  and .request_target_action == "request_operator_review"
  and .operator_review_request_allowed == false
  and .operator_review_requested == false
  and .operator_review_request_recorded == false
  and .operator_review_request_persisted == false
  and .operator_review_request_accepted == false
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
  and .ready_for_request_denial_readback == true
  and .ready_for_operator_review_request == false
  and .ready_for_approval_recording == false
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.request_blocker_matrix_module_present == true
  and .source_probes.non_persistence_readback_gate_present == true
  and .source_probes.non_persistence_readback_points_here == true
  and .source_probes.non_persistence_readback_unrequested_present == true
  and .source_probes.non_persistence_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate"
  and .source_probes.non_persistence_readback_preconditions_complete == true
  and .source_probes.non_persistence_readback_ready_for_blocker_matrix == true
  and .source_probes.non_persistence_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag operator review request precondition blocker matrix gate passed"
