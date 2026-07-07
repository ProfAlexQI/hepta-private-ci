#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report_only"
  and .source_entrypoint_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate"
  and .source_entrypoint_binding_count == 4
  and .source_guardrail_check_count == 8
  and .source_dry_run_decision_count == 4
  and .source_entrypoint_required_prior_gate_count == 4
  and .source_entrypoint_ready == true
  and .source_entrypoint_no_live_confirmed == true
  and .source_terminal_no_execution_final_closeout_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate"
  and .source_terminal_no_execution_final_closeout_entry_count == 9
  and .source_terminal_no_execution_final_closeout_blocker_count == 26
  and .source_terminal_no_execution_final_closeout_required_prior_gate_count == 5
  and .source_terminal_no_execution_final_closeout_ready == true
  and .source_terminal_no_execution_final_closeout_no_live_confirmed == true
  and .hardened_entrypoint_count == 4
  and .hardening_check_count == 10
  and .hardening_decision_count == 4
  and .hardening_blocker_count == 23
  and .required_prior_gate_count == 11
  and .source_prior_readbacks_complete == true
  and (.hardened_entrypoints | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.hardened_entrypoints | all(
    .dry_run_outcome == "deny_live_allow_report_only_hardened"
    and (.required_evidence_fields | length == 9)
    and (.required_non_live_guards | length == 5)
    and .would_block_if_live == true
    and .report_only_allows_current_runtime == true
    and .live_blocking_enabled == false
    and .runtime_interception_enabled == false
  ))
  and .hardened_entrypoints_complete == true
  and (.hardening_checks | map(.id) == [
    "dependencies_readback_required",
    "lane_lease_snapshot_required",
    "approval_authority_snapshot_required",
    "idempotency_decision_key_required",
    "budget_timeout_snapshot_required",
    "task_result_envelope_preview_required",
    "side_effect_boundary_class_required",
    "trace_guardrail_span_required",
    "shadow_event_join_preview_required",
    "replay_diff_terminal_no_execution_closeout_required"
  ])
  and (.hardening_checks | all(
    .blocks_live_execution == true
    and .dry_run_only == true
    and (.hardening_requirement | length > 0)
  ))
  and .hardening_checks_complete == true
  and (.hardening_decisions | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.hardening_decisions | all(
    .outcome == "deny_live_allow_report_only_hardened"
    and .allow_current_runtime_to_continue == true
    and .block_live_execution == true
    and .decision_recorded == false
    and .decision_persisted == false
    and (.trace_id | length > 0)
    and (.deterministic_decision_key | length > 0)
  ))
  and .hardening_decisions_complete == true
  and (.hardening_blockers | map(.blocked_action) == [
    "install_live_blocking_hook",
    "enable_runtime_interception",
    "enforce_scheduler_admission",
    "enable_guardrail_enforcement",
    "record_hardening_decision",
    "persist_hardening_decision",
    "persist_work_graph_event",
    "persist_projection_index",
    "acquire_lane_lease",
    "start_entrypoint_work",
    "spawn_agent",
    "invoke_model",
    "send_external_message",
    "record_replay_diff",
    "execute_replay",
    "execute_rollback",
    "mutate_idempotency_index",
    "write_config",
    "mutate_feature_flag",
    "route_canary_traffic",
    "request_operator_review",
    "record_operator_approval",
    "perform_live_cutover"
  ])
  and (.hardening_blockers | all(.blocked == true))
  and .hardening_blockers_complete == true
  and .hardening_preconditions_complete == true
  and .required_prior_gates == [
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"
  and .scheduler_guardrail_entrypoint_dry_run_present == true
  and .terminal_no_execution_final_closeout_present == true
  and .deny_live_allow_report_only_hardened == true
  and .pre_entrypoint_hook_contract_hardened == true
  and .deterministic_decision_key_ready == true
  and .trace_evidence_contract_ready == true
  and .shadow_event_join_ready == true
  and .live_blocking_enforcement_enabled == false
  and .runtime_interception_enabled == false
  and .scheduler_admission_enforced == false
  and .guardrail_enforcement_enabled == false
  and .work_graph_event_persistence_enabled == false
  and .ready_for_hardening_readback == true
  and .ready_for_live_execution == false
  and .source_probes.hardening_module_present == true
  and .source_probes.entrypoint_gate_present == true
  and .source_probes.terminal_closeout_gate_present == true
  and .source_probes.terminal_closeout_points_here == true
  and .source_probes.entrypoint_no_live_present == true
  and .source_probes.entrypoint_dry_run_present == true
  and .source_probes.terminal_closeout_no_live_present == true
  and .source_probes.entrypoint_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate"
  and .source_probes.entrypoint_pre_entrypoint_hook_contract_ready == true
  and .source_probes.entrypoint_ready_for_shadow_readback == true
  and .source_probes.entrypoint_no_live_confirmed == true
  and .source_probes.entrypoint_side_effects_all_false == true
  and .source_probes.terminal_closeout_report_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate"
  and .source_probes.terminal_closeout_preconditions_complete == true
  and .source_probes.terminal_closeout_ready_for_hardening == true
  and .source_probes.terminal_closeout_no_live_confirmed == true
  and .source_probes.terminal_closeout_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

for test_name in \
  scheduler_guardrail_entrypoint_hardening_derives_from_entrypoint_and_closeout_priors \
  scheduler_guardrail_entrypoint_hardening_covers_four_entrypoints \
  scheduler_guardrail_entrypoint_hardening_blocks_live_paths \
  scheduler_guardrail_entrypoint_hardening_links_priors_and_has_no_side_effects; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint hardening gate passed"
