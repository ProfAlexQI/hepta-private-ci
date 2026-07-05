#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_only"
  and .source_matrix_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate"
  and .source_entrypoint_count == 4
  and .source_precondition_check_count == 14
  and .source_blocking_precondition_count == 10
  and .source_blocker_count == 33
  and .source_required_prior_gate_count == 16
  and .denial_readback_entry_count == 7
  and .entrypoint_denial_readback_count == 4
  and .denial_readback_blocker_count == 36
  and .required_prior_gate_count == 17
  and .denial_readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix"
  and .denial_readback_scope.readback_mode == "live_attachment_precondition_matrix_denial_readback_only"
  and .denial_readback_scope.visible == true
  and .denial_readback_scope.recorded == false
  and .denial_readback_scope.persisted == false
  and .denial_readback_scope.authoritative == false
  and .denial_readback_scope.accepted == false
  and (.denial_readback_entries | map(.id) == [
    "live_attachment_matrix_denial_state_readback",
    "live_attachment_entrypoint_inventory_readback",
    "live_attachment_precondition_check_catalog_readback",
    "live_attachment_blocker_catalog_readback",
    "live_attachment_prior_chain_readback",
    "live_attachment_non_attachment_boundary_readback",
    "live_attachment_no_live_authority_readback"
  ])
  and (.denial_readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and .accepted == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.entrypoint_denial_readbacks | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoint_denial_readbacks | all(
    .live_attachment_allowed == false
    and .runtime_interception_allowed == false
    and .report_only == true
  ))
  and (.denial_readback_blockers | map(.blocked_action) == [
    "record_live_attachment_denial_readback",
    "persist_live_attachment_denial_readback",
    "accept_live_attachment_denial_readback",
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
  and (.denial_readback_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate",
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate"
  and .denial_readback_visible == true
  and .denial_readback_recorded == false
  and .denial_readback_persisted == false
  and .denial_readback_authoritative == false
  and .denial_readback_accepted == false
  and .denial_readback_authorizes_live_attachment == false
  and .denial_readback_authorizes_live_blocking_hook == false
  and .denial_readback_authorizes_runtime_interception == false
  and .denial_readback_authorizes_scheduler_admission_enforcement == false
  and .denial_readback_authorizes_guardrail_enforcement == false
  and .denial_readback_authorizes_work_graph_persistence == false
  and .denial_readback_authorizes_lease_or_work_start == false
  and .denial_readback_authorizes_agent_model_or_external_send == false
  and .denial_readback_authorizes_live_task_result == false
  and .denial_readback_authorizes_replay_or_rollback == false
  and .denial_readback_authorizes_config_flag_or_traffic == false
  and .denial_readback_authorizes_operator_approval_or_live_cutover == false
  and .ready_for_denial_readback_audit_index == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_probes.denial_readback_module_present == true
  and .source_probes.live_attachment_matrix_gate_present == true
  and .source_probes.live_attachment_matrix_points_here == true
  and .source_probes.live_attachment_matrix_ready_present == true
  and .source_probes.live_attachment_matrix_no_live_present == true
  and .source_probes.live_attachment_matrix_unpersisted_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback --lib

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment precondition matrix denial readback gate passed"
