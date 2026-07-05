#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-audit-index-non-persistence-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-audit-index-non-persistence-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_v1"
  and .preview_mode == "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_only"
  and .source_audit_index_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate"
  and .source_audit_index_entry_count == 8
  and .source_audit_index_blocker_count == 20
  and .source_required_prior_gate_count == 3
  and .readback_entry_count == 6
  and .readback_blocker_count == 23
  and .required_prior_gate_count == 4
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.work_graph.shadow_event_store.replay_diff_dry_run_non_execution_readback_audit_index"
  and .readback_scope.readback_mode == "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_only"
  and .readback_scope.audit_index_visible == true
  and .readback_scope.audit_index_recorded == false
  and .readback_scope.audit_index_persisted == false
  and .readback_scope.audit_index_authoritative == false
  and .readback_scope.audit_index_accepted == false
  and .readback_scope.readback_recorded == false
  and .readback_scope.readback_persisted == false
  and .readback_scope.readback_accepted == false
  and (.readback_entries | map(.id) == [
    "replay_diff_audit_index_surface_non_persistence_readback",
    "replay_diff_audit_index_entry_inventory_non_persistence_readback",
    "replay_diff_audit_index_blocker_inventory_non_persistence_readback",
    "replay_diff_audit_index_prior_chain_non_persistence_readback",
    "replay_diff_audit_index_non_persistence_boundary_readback",
    "replay_diff_audit_index_no_live_authority_readback"
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
  and (.readback_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .non_execution_readback_visible == true
  and .non_execution_readback_executed == false
  and .non_execution_readback_recorded == false
  and .non_execution_readback_persisted == false
  and .audit_index_readback_recorded == false
  and .audit_index_readback_persisted == false
  and .audit_index_readback_accepted == false
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
  and .ready_for_terminal_no_execution_final_closeout == true
  and .ready_for_live_execution == false
  and .source_probes.non_persistence_readback_module_present == true
  and .source_probes.audit_index_gate_present == true
  and .source_probes.audit_index_points_here == true
  and .source_probes.audit_index_ready_present == true
  and .source_probes.audit_index_unpersisted_present == true
  and .source_probes.audit_index_no_live_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback --lib

echo "Hepta WorkGraph agent_jobs + task_board shadow event-store replay diff dry-run non-execution readback audit index non-persistence readback gate passed"
