#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_deny_only"
  and .source_final_closeout_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate"
  and .source_final_closeout_entry_count == 9
  and .source_final_closeout_blocker_count == 36
  and .source_required_prior_gate_count == 15
  and .entrypoint_count == 4
  and .precondition_check_count == 14
  and .precondition_satisfied_count == 4
  and .precondition_unsatisfied_count == 10
  and .blocking_precondition_count == 10
  and .blocker_count == 33
  and .required_prior_gate_count == 16
  and (.entrypoints | map(.id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoints | all(
    .live_attachment_candidate == true
    and .live_attachment_allowed == false
    and .report_only == true
    and .runtime_interception_allowed == false
  ))
  and (.precondition_checks | map(.id) == [
    "terminal_no_enforcement_final_closeout_ready",
    "entrypoint_scope_inventory_visible",
    "deterministic_decision_keys_visible",
    "trace_evidence_join_visible",
    "live_blocking_hook_authorization_missing",
    "runtime_interception_authorization_missing",
    "scheduler_admission_enforcement_authorization_missing",
    "guardrail_enforcement_authorization_missing",
    "work_graph_persistence_authorization_missing",
    "lease_and_work_start_authorization_missing",
    "live_task_result_acceptance_missing",
    "replay_rollback_execution_authorization_missing",
    "config_flag_traffic_authorization_missing",
    "operator_approval_live_cutover_authorization_missing"
  ])
  and (.precondition_checks | map(select(.blocking == true and .satisfied == false)) | length == 10)
  and (.precondition_checks | map(select(.blocking == false and .satisfied == true)) | length == 4)
  and (.blockers | map(.blocked_action) == [
    "record_live_attachment_precondition_matrix",
    "persist_live_attachment_precondition_matrix",
    "accept_live_attachment_precondition_matrix",
    "enable_live_attachment",
    "install_live_blocking_hook",
    "enable_runtime_interception",
    "enforce_scheduler_admission",
    "enable_guardrail_enforcement",
    "persist_work_graph_event",
    "persist_projection_index",
    "acquire_lane_lease",
    "start_entrypoint_work",
    "spawn_agent",
    "spawn_agents_on_csv",
    "claim_task_board_work",
    "run_worker_task",
    "invoke_model",
    "send_external_message",
    "emit_live_task_result",
    "record_hardening_decision",
    "persist_hardening_decision",
    "execute_readback",
    "execute_replay",
    "record_replay_diff",
    "persist_replay_diff",
    "execute_rollback",
    "mutate_idempotency_index",
    "write_config",
    "mutate_feature_flag",
    "route_canary_traffic",
    "request_operator_review",
    "record_operator_approval",
    "perform_live_cutover"
  ])
  and (.blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate"
  and .matrix_mode == "deny_live_attachment_until_explicit_scheduler_guardrail_enforcement_authorization"
  and .matrix_visible == true
  and .matrix_recorded == false
  and .matrix_persisted == false
  and .matrix_authoritative == false
  and .matrix_accepted == false
  and .live_attachment_allowed == false
  and .live_blocking_hook_install_allowed == false
  and .runtime_interception_allowed == false
  and .scheduler_admission_enforcement_allowed == false
  and .guardrail_enforcement_allowed == false
  and .work_graph_event_persistence_allowed == false
  and .projection_persistence_allowed == false
  and .lease_acquisition_allowed == false
  and .work_start_allowed == false
  and .agent_spawn_allowed == false
  and .model_invocation_allowed == false
  and .external_send_allowed == false
  and .live_task_result_emission_allowed == false
  and .hardening_decision_recording_allowed == false
  and .hardening_decision_persistence_allowed == false
  and .readback_execution_allowed == false
  and .replay_execution_allowed == false
  and .replay_diff_recording_allowed == false
  and .replay_diff_persistence_allowed == false
  and .rollback_execution_allowed == false
  and .idempotency_mutation_allowed == false
  and .config_write_allowed == false
  and .feature_flag_mutation_allowed == false
  and .canary_traffic_allowed == false
  and .operator_review_request_allowed == false
  and .approval_recording_allowed == false
  and .live_cutover_allowed == false
  and .ready_for_denial_readback == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_probes.live_attachment_matrix_module_present == true
  and .source_probes.terminal_closeout_gate_present == true
  and .source_probes.terminal_closeout_points_here == true
  and .source_probes.terminal_closeout_ready_present == true
  and .source_probes.terminal_closeout_no_live_present == true
  and .source_probes.terminal_closeout_unpersisted_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix --lib

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment precondition matrix gate passed"
