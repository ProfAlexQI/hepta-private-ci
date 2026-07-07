#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-audit-index-non-persistence-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-audit-index-non-persistence-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_only"
  and .source_audit_index_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate"
  and .source_audit_index_entry_count == 9
  and .source_audit_index_blocker_count == 39
  and .source_required_prior_gate_count == 18
  and .source_audit_index_ready == true
  and .source_audit_index_no_persistence_confirmed == true
  and .source_audit_index_no_live_confirmed == true
  and .source_audit_index_ready_for_non_persistence_readback == true
  and .readback_entry_count == 6
  and .readback_blocker_count == 42
  and .required_prior_gate_count == 19
  and .readback_scope.id == "agent_jobs_task_board_scheduler_guardrail_live_attachment_denial_readback_audit_index_non_persistence_readback_scope"
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback_audit_index"
  and .readback_scope.readback_mode == "live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_only"
  and .readback_scope.audit_index_visible == true
  and .readback_scope.audit_index_recorded == false
  and .readback_scope.audit_index_persisted == false
  and .readback_scope.audit_index_authoritative == false
  and .readback_scope.audit_index_accepted == false
  and .readback_scope.readback_recorded == false
  and .readback_scope.readback_persisted == false
  and .readback_scope.readback_accepted == false
  and (.readback_entries | map(.id) == [
    "live_attachment_audit_index_surface_non_persistence_readback",
    "live_attachment_audit_index_entry_inventory_non_persistence_readback",
    "live_attachment_audit_index_blocker_inventory_non_persistence_readback",
    "live_attachment_audit_index_prior_chain_non_persistence_readback",
    "live_attachment_audit_index_non_persistence_boundary_readback",
    "live_attachment_audit_index_no_live_authority_readback"
  ])
  and (.readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.readback_blockers | map(.blocked_action) == [
    "record_live_attachment_audit_index_non_persistence_readback",
    "persist_live_attachment_audit_index_non_persistence_readback",
    "accept_live_attachment_audit_index_non_persistence_readback",
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
  and (.readback_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate",
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_gate"
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
  and .audit_index_readback_recorded == false
  and .audit_index_readback_persisted == false
  and .audit_index_readback_accepted == false
  and .readback_scope_visible_only_complete == true
  and .readback_entries_visible_only_complete == true
  and .readback_blockers_complete == true
  and .non_persistence_readback_preconditions_complete == true
  and .matrix_recording_allowed == false
  and .matrix_persistence_allowed == false
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
  and .ready_for_terminal_no_attachment_final_closeout == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.audit_index_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate"
  and .source_readbacks.audit_index_preconditions_complete == true
  and .source_readbacks.audit_index_ready_for_non_persistence_readback == true
  and .source_readbacks.audit_index_no_persistence_confirmed == true
  and .source_readbacks.audit_index_no_live_confirmed == true
  and .source_readbacks.audit_index_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  live_attachment_audit_index_non_persistence_readback_derives_from_audit_index
  live_attachment_audit_index_non_persistence_readback_stays_unpersisted
  live_attachment_audit_index_non_persistence_readback_blocks_live_paths
  live_attachment_audit_index_non_persistence_readback_has_no_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment precondition matrix denial readback audit index non-persistence readback gate passed"
