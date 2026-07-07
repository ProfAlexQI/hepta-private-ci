#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only"
  and .source_hardening_readback_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"
  and .source_readback_entry_count == 7
  and .source_entrypoint_readback_count == 4
  and .source_readback_blocker_count == 27
  and .source_required_prior_gate_count == 12
  and .source_hardening_readback_ready == true
  and .source_hardening_readback_no_live_confirmed == true
  and .source_hardening_readback_no_persistence_confirmed == true
  and .source_hardening_readback_ready_for_audit_index == true
  and .audit_index_entry_count == 9
  and .audit_index_blocker_count == 30
  and .required_prior_gate_count == 13
  and .audit_index_scope.id == "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope"
  and .audit_index_scope.index_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only"
  and .audit_index_scope.index_visible == true
  and .audit_index_scope.index_recorded == false
  and .audit_index_scope.index_persisted == false
  and .audit_index_scope.index_authoritative == false
  and .audit_index_scope.index_accepted == false
  and .audit_index_scope.live_acceptance_allowed == false
  and (.audit_index_entries | map(.id) == [
    "hardening_readback_scope_audit_index",
    "hardening_readback_entry_inventory_audit_index",
    "hardening_entrypoint_readback_inventory_audit_index",
    "hardening_readback_blocker_inventory_audit_index",
    "hardening_readback_prior_chain_audit_index",
    "hardening_readback_non_live_guard_audit_index",
    "hardening_readback_no_live_authority_audit_index",
    "hardening_source_decision_trace_audit_index",
    "hardening_live_boundary_audit_index"
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
    "record_hardening_readback_audit_index",
    "persist_hardening_readback_audit_index",
    "accept_hardening_readback_audit_index",
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
  and (.audit_index_blockers | all(.blocked == true and .required_before_acceptance == true))
  and .required_prior_gates == [
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
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .hardening_readback_visible == true
  and .hardening_readback_recorded == false
  and .hardening_readback_persisted == false
  and .hardening_readback_accepted == false
  and .audit_index_scope_visible_only_complete == true
  and .audit_index_entries_complete == true
  and .audit_index_blockers_complete == true
  and .audit_index_preconditions_complete == true
  and .audit_index_authorizes_hardening_readback_recording == false
  and .audit_index_authorizes_hardening_readback_persistence == false
  and .audit_index_authorizes_hardening_decision_recording == false
  and .audit_index_authorizes_hardening_decision_persistence == false
  and .audit_index_authorizes_live_blocking_enforcement == false
  and .audit_index_authorizes_runtime_interception == false
  and .audit_index_authorizes_scheduler_admission_enforcement == false
  and .audit_index_authorizes_guardrail_enforcement == false
  and .audit_index_authorizes_work_graph_event_persistence == false
  and .audit_index_authorizes_projection_persistence == false
  and .audit_index_authorizes_lease_acquisition == false
  and .audit_index_authorizes_work_start == false
  and .audit_index_authorizes_agent_spawn == false
  and .audit_index_authorizes_model_invocation == false
  and .audit_index_authorizes_external_send == false
  and .audit_index_authorizes_replay_execution == false
  and .audit_index_authorizes_replay_diff_recording == false
  and .audit_index_authorizes_replay_diff_persistence == false
  and .audit_index_authorizes_rollback_execution == false
  and .audit_index_authorizes_idempotency_mutation == false
  and .audit_index_authorizes_config_write == false
  and .audit_index_authorizes_feature_flag_mutation == false
  and .audit_index_authorizes_canary_traffic == false
  and .audit_index_authorizes_operator_review_request == false
  and .audit_index_authorizes_approval_recording == false
  and .audit_index_authorizes_live_cutover == false
  and .ready_for_non_persistence_readback == true
  and .ready_for_live_execution == false
  and .source_probes.hardening_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"
  and .source_probes.hardening_readback_preconditions_complete == true
  and .source_probes.hardening_readback_ready_for_audit_index == true
  and .source_probes.hardening_readback_no_live_confirmed == true
  and .source_probes.hardening_readback_no_persistence_confirmed == true
  and .source_probes.hardening_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  scheduler_guardrail_entrypoint_hardening_readback_audit_index_derives_from_readback
  scheduler_guardrail_entrypoint_hardening_readback_audit_index_is_visible_only
  scheduler_guardrail_entrypoint_hardening_readback_audit_index_blocks_live_paths
  scheduler_guardrail_entrypoint_hardening_readback_audit_index_links_priors_and_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint hardening readback audit index gate passed"
