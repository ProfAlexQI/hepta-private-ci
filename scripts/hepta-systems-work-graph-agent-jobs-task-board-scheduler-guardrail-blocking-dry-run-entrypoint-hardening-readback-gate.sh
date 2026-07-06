#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_visible_only"
  and .source_hardening_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate"
  and .source_hardened_entrypoint_count == 4
  and .source_hardening_check_count == 10
  and .source_hardening_decision_count == 4
  and .source_hardening_blocker_count == 23
  and .source_required_prior_gate_count == 11
  and .readback_entry_count == 7
  and .entrypoint_readback_count == 4
  and .readback_blocker_count == 27
  and .required_prior_gate_count == 12
  and .readback_scope.id == "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_scope"
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening"
  and .readback_scope.readback_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_visible_only"
  and .readback_scope.visible == true
  and .readback_scope.recorded == false
  and .readback_scope.persisted == false
  and .readback_scope.authoritative == false
  and .readback_scope.accepted == false
  and .readback_scope.mutation_allowed == false
  and (.readback_entries | map(.id) == [
    "hardening_contract_inventory_readback",
    "hardening_check_inventory_readback",
    "hardening_decision_inventory_readback",
    "hardening_blocker_inventory_readback",
    "hardening_prior_chain_readback",
    "hardening_non_live_guard_readback",
    "hardening_no_live_authority_readback"
  ])
  and (.readback_entries | all(
    .visible == true
    and .ready == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
  ))
  and (.entrypoint_readbacks | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoint_readbacks | all(
    .dry_run_outcome == "deny_live_allow_report_only_hardened"
    and .required_evidence_field_count == 9
    and .required_non_live_guard_count == 5
    and .readback_status == "visible_only"
    and .would_block_if_live == true
    and .report_only_allows_current_runtime == true
    and .recorded == false
    and .persisted == false
  ))
  and (.readback_blockers | map(.blocked_action) == [
    "record_hardening_readback",
    "persist_hardening_readback",
    "accept_hardening_readback",
    "record_hardening_decision",
    "persist_hardening_decision",
    "install_live_blocking_hook",
    "enable_runtime_interception",
    "enforce_scheduler_admission",
    "enable_guardrail_enforcement",
    "persist_work_graph_event",
    "persist_projection_index",
    "acquire_lane_lease",
    "start_entrypoint_work",
    "spawn_agent",
    "invoke_model",
    "send_external_message",
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate"
  and .hardening_contract_visible == true
  and .hardening_decisions_visible == true
  and .hardening_checks_visible == true
  and .hardening_blockers_visible == true
  and .readback_ready == true
  and .readback_recorded == false
  and .readback_persisted == false
  and .readback_authoritative == false
  and .readback_accepted == false
  and .hardening_decision_recording_allowed == false
  and .hardening_decision_persistence_allowed == false
  and .live_blocking_enforcement_allowed == false
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
  and .ready_for_audit_index == true
  and .ready_for_live_execution == false
  and .source_probes.readback_module_present == true
  and .source_probes.hardening_gate_present == true
  and .source_probes.hardening_points_here == true
  and .source_probes.hardening_no_live_present == true
  and .source_probes.hardening_no_interception_present == true
  and .source_probes.hardening_no_persistence_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback --lib

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint hardening readback gate passed"
