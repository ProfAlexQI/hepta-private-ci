#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-audit-index-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-audit-index-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report_only"
  and .source_denial_readback_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate"
  and .source_denial_readback_entry_count == 7
  and .source_entrypoint_denial_readback_count == 4
  and .source_denial_readback_blocker_count == 36
  and .source_required_prior_gate_count == 17
  and .source_denial_readback_ready == true
  and .source_denial_readback_no_persistence_confirmed == true
  and .source_denial_readback_no_live_confirmed == true
  and .source_denial_readback_ready_for_audit_index == true
  and .audit_index_entry_count == 9
  and .audit_index_blocker_count == 39
  and .required_prior_gate_count == 18
  and .audit_index_scope.id == "agent_jobs_task_board_scheduler_guardrail_live_attachment_denial_readback_audit_index_scope"
  and .audit_index_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback"
  and .audit_index_scope.index_mode == "live_attachment_precondition_matrix_denial_readback_audit_index_report_only"
  and .audit_index_scope.index_visible == true
  and .audit_index_scope.index_recorded == false
  and .audit_index_scope.index_persisted == false
  and .audit_index_scope.index_authoritative == false
  and .audit_index_scope.index_accepted == false
  and .audit_index_scope.live_acceptance_allowed == false
  and (.audit_index_entries | map(.id) == [
    "live_attachment_denial_readback_scope_audit_index",
    "live_attachment_denial_state_audit_index",
    "live_attachment_entrypoint_inventory_audit_index",
    "live_attachment_precondition_check_catalog_audit_index",
    "live_attachment_blocker_inventory_audit_index",
    "live_attachment_prior_chain_audit_index",
    "live_attachment_non_attachment_boundary_audit_index",
    "live_attachment_no_live_authority_audit_index",
    "live_attachment_entrypoint_surface_audit_index"
  ])
  and (.audit_index_entries | all(
    .indexed == true
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and .accepted == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.audit_index_blockers | map(.blocked_action) == [
    "record_live_attachment_denial_readback_audit_index",
    "persist_live_attachment_denial_readback_audit_index",
    "accept_live_attachment_denial_readback_audit_index",
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
  and (.audit_index_blockers | all(.blocked == true and .required_before_acceptance == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate",
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .denial_readback_visible == true
  and .denial_readback_recorded == false
  and .denial_readback_persisted == false
  and .denial_readback_authoritative == false
  and .denial_readback_accepted == false
  and .audit_index_scope_report_only_complete == true
  and .audit_index_entries_complete == true
  and .audit_index_blockers_complete == true
  and .audit_index_preconditions_complete == true
  and .audit_index_authorizes_denial_readback_recording == false
  and .audit_index_authorizes_denial_readback_persistence == false
  and .audit_index_authorizes_matrix_recording == false
  and .audit_index_authorizes_matrix_persistence == false
  and .audit_index_authorizes_live_attachment == false
  and .audit_index_authorizes_live_blocking_hook == false
  and .audit_index_authorizes_runtime_interception == false
  and .audit_index_authorizes_scheduler_admission_enforcement == false
  and .audit_index_authorizes_guardrail_enforcement == false
  and .audit_index_authorizes_work_graph_persistence == false
  and .audit_index_authorizes_projection_persistence == false
  and .audit_index_authorizes_lease_or_work_start == false
  and .audit_index_authorizes_agent_model_or_external_send == false
  and .audit_index_authorizes_live_task_result == false
  and .audit_index_authorizes_replay_or_rollback == false
  and .audit_index_authorizes_config_flag_or_traffic == false
  and .audit_index_authorizes_operator_approval_or_live_cutover == false
  and .ready_for_non_persistence_readback == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.denial_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate"
  and .source_readbacks.denial_readback_preconditions_complete == true
  and .source_readbacks.denial_readback_ready_for_audit_index == true
  and .source_readbacks.denial_readback_no_persistence_confirmed == true
  and .source_readbacks.denial_readback_no_live_confirmed == true
  and .source_readbacks.denial_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  live_attachment_denial_readback_audit_index_derives_from_denial_readback
  live_attachment_denial_readback_audit_index_is_visible_only
  live_attachment_denial_readback_audit_index_blocks_live_authority
  live_attachment_denial_readback_audit_index_has_no_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment precondition matrix denial readback audit index gate passed"
