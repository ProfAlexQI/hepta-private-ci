#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-non-persistence-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-non-persistence-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_v1"
  and .preview_mode == "operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_only"
  and .source_audit_index_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate"
  and .source_audit_index_entry_count == 6
  and .source_audit_index_blocker_count == 19
  and .source_required_prior_gate_count == 19
  and .source_audit_index_preconditions_complete == true
  and .source_audit_index_no_record_persist_request_confirmed == true
  and .source_audit_index_no_authorization_confirmed == true
  and .source_audit_index_ready == true
  and .readback_entry_count == 5
  and .readback_blocker_count == 20
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition_denial_readback_audit_index"
  and .readback_scope.readback_mode == "operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_only"
  and .readback_scope.audit_index_visible == true
  and .readback_scope.audit_index_recorded == false
  and .readback_scope.audit_index_persisted == false
  and .readback_scope.audit_index_authoritative == false
  and .readback_scope.audit_index_accepted == false
  and .readback_scope.operator_review_requested == false
  and .readback_scope.readback_persisted == false
  and .readback_scope_non_persistent_complete == true
  and (.readback_entries | map(.id) == [
    "operator_review_request_denial_audit_index_surface_readback",
    "operator_review_request_denial_audit_index_prior_chain_readback",
    "operator_review_request_denial_audit_index_blocker_readback",
    "operator_review_request_denial_audit_index_non_persistence_boundary_readback",
    "operator_review_request_denial_audit_index_no_request_boundary_readback"
  ])
  and (.readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .operator_review_requested == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and .readback_entries_non_persistent_complete == true
  and (.readback_blockers | map(.blocked_action) == [
    "persist_operator_review_request_denial_audit_index_readback",
    "record_operator_review_request_denial_audit_index",
    "persist_operator_review_request_denial_audit_index",
    "accept_operator_review_request_denial_audit_index",
    "request_operator_review",
    "record_operator_review_request",
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
  and (.readback_blockers | all(.blocked == true))
  and .readback_blockers_complete == true
  and .non_persistence_readback_preconditions_complete == true
  and .required_prior_gates == [
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
  and .required_prior_gate_count == 20
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .operator_review_request_allowed == false
  and .operator_review_requested == false
  and .operator_packet_send_allowed == false
  and .operator_packet_acceptance_allowed == false
  and .approval_recording_allowed == false
  and .readback_persisted == false
  and .config_write_allowed == false
  and .feature_flag_enablement_allowed == false
  and .canary_traffic_allowed == false
  and .scheduler_enforcement_allowed == false
  and .guardrail_enforcement_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .work_graph_persistence_allowed == false
  and .live_cutover_allowed == false
  and .ready_for_terminal_no_request_closeout == true
  and .ready_for_operator_review_request == false
  and .ready_for_approval_recording == false
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.non_persistence_readback_module_present == true
  and .source_probes.audit_index_gate_present == true
  and .source_probes.audit_index_points_here == true
  and .source_probes.audit_index_unrequested_present == true
  and .source_probes.audit_index_ready_present == true
  and .source_probes.audit_index_unpersisted_present == true
  and .source_probes.audit_index_report_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate"
  and .source_probes.audit_index_preconditions_complete == true
  and .source_probes.audit_index_ready_for_non_persistence_readback == true
  and .source_probes.audit_index_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag operator review request precondition denial readback audit index non-persistence readback gate passed"
