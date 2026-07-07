#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-terminal-no-execution-final-closeout-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-terminal-no-execution-final-closeout-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_v1"
  and .preview_mode == "work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report_only"
  and .source_non_persistence_readback_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate"
  and .source_readback_entry_count == 6
  and .source_readback_blocker_count == 23
  and .source_required_prior_gate_count == 4
  and .source_non_persistence_readback_ready == true
  and .source_non_persistence_readback_no_persistence_confirmed == true
  and .source_non_persistence_readback_no_authorization_confirmed == true
  and .source_non_persistence_readback_ready_for_terminal_closeout == true
  and .final_closeout_entry_count == 9
  and .final_closeout_blocker_count == 26
  and .required_prior_gate_count == 5
  and .final_closeout_scope.source_surface_id == "work_graph_agent_jobs_task_board.work_graph.shadow_event_store.replay_diff_dry_run.non_execution_readback_audit_index_non_persistence_readback"
  and .final_closeout_scope.closeout_mode == "work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report_only"
  and .final_closeout_scope.visible == true
  and .final_closeout_scope.recorded == false
  and .final_closeout_scope.persisted == false
  and .final_closeout_scope.authoritative == false
  and .final_closeout_scope.accepted == false
  and .final_closeout_scope.terminal == true
  and .final_closeout_scope.mutation_allowed == false
  and .final_closeout_scope_visible_only_complete == true
  and (.final_closeout_entries | map(.id) == [
    "replay_diff_no_execution_branch_final_closeout",
    "replay_diff_audit_index_surface_final_closeout",
    "replay_diff_audit_index_entry_inventory_final_closeout",
    "replay_diff_audit_index_blocker_inventory_final_closeout",
    "replay_diff_audit_index_prior_chain_final_closeout",
    "replay_diff_non_persistence_boundary_final_closeout",
    "replay_diff_no_live_authority_final_closeout",
    "replay_diff_entrypoint_scope_final_closeout",
    "replay_diff_scheduler_guardrail_boundary_final_closeout"
  ])
  and (.final_closeout_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
    and .closed == true
  ))
  and .final_closeout_entries_complete == true
  and (.final_closeout_blockers | map(.blocked_action) == [
    "record_replay_diff_terminal_no_execution_final_closeout",
    "persist_replay_diff_terminal_no_execution_final_closeout",
    "accept_replay_diff_terminal_no_execution_final_closeout",
    "record_replay_diff_audit_index_non_persistence_readback",
    "persist_replay_diff_audit_index_non_persistence_readback",
    "accept_replay_diff_audit_index_non_persistence_readback",
    "record_replay_diff_non_execution_readback_audit_index",
    "persist_replay_diff_non_execution_readback_audit_index",
    "accept_replay_diff_non_execution_readback_audit_index",
    "execute_non_execution_readback",
    "record_non_execution_readback",
    "persist_non_execution_readback",
    "execute_replay",
    "record_replay_diff",
    "persist_replay_diff",
    "execute_rollback",
    "mutate_idempotency_index",
    "persist_work_graph_event",
    "persist_projection_index",
    "enable_scheduler_guardrail_live_enforcement",
    "enable_runtime_interception",
    "enable_feature_flag",
    "route_canary_traffic",
    "request_operator_review",
    "record_operator_approval",
    "perform_live_cutover"
  ])
  and (.final_closeout_blockers | all(.blocked == true))
  and .final_closeout_blockers_complete == true
  and .terminal_no_execution_final_closeout_preconditions_complete == true
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate"
  and .terminal_no_execution_branch_closed == true
  and .final_closeout_visible == true
  and .final_closeout_recorded == false
  and .final_closeout_persisted == false
  and .final_closeout_authoritative == false
  and .final_closeout_accepted == false
  and .source_audit_index_visible == true
  and .source_audit_index_persisted == false
  and .source_readback_persisted == false
  and .readback_execution_allowed == false
  and .replay_execution_allowed == false
  and .replay_diff_recording_allowed == false
  and .replay_diff_persistence_allowed == false
  and .rollback_execution_allowed == false
  and .idempotency_mutation_allowed == false
  and .work_graph_event_persistence_allowed == false
  and .projection_persistence_allowed == false
  and .scheduler_guardrail_enforcement_allowed == false
  and .runtime_interception_allowed == false
  and .feature_flag_enablement_allowed == false
  and .canary_traffic_allowed == false
  and .operator_review_request_allowed == false
  and .approval_recording_allowed == false
  and .live_cutover_allowed == false
  and .ready_for_scheduler_guardrail_blocking_dry_run_entrypoint_hardening == true
  and .ready_for_live_execution == false
  and .source_probes.final_closeout_module_present == true
  and .source_probes.non_persistence_readback_gate_present == true
  and .source_probes.non_persistence_readback_points_here == true
  and .source_probes.non_persistence_readback_ready_present == true
  and .source_probes.non_persistence_readback_no_live_present == true
  and .source_probes.non_persistence_readback_unpersisted_present == true
  and .source_probes.non_persistence_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate"
  and .source_probes.non_persistence_readback_preconditions_complete == true
  and .source_probes.non_persistence_readback_ready_for_terminal_closeout == true
  and .source_probes.non_persistence_readback_no_persistence_confirmed == true
  and .source_probes.non_persistence_readback_no_authorization_confirmed == true
  and .source_probes.non_persistence_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

for test_name in \
  replay_diff_terminal_no_execution_final_closeout_derives_from_non_persistence_readback \
  replay_diff_terminal_no_execution_final_closeout_is_visible_only \
  replay_diff_terminal_no_execution_final_closeout_blocks_execution_and_live_paths \
  replay_diff_terminal_no_execution_final_closeout_links_priors_and_side_effects; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board shadow event-store replay/diff dry-run terminal no-execution final closeout gate passed"
