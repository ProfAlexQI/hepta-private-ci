#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-audit-index-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-audit-index-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_v1"
  and .preview_mode == "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_report_only"
  and .source_non_execution_readback_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate"
  and .source_non_execution_readback_entry_count == 7
  and .source_replay_scope_readback_count == 4
  and .source_non_execution_blocker_count == 18
  and .source_required_prior_gate_count == 2
  and .source_non_execution_readback_ready == true
  and .source_non_execution_readback_no_execution_confirmed == true
  and .source_non_execution_readback_no_authorization_confirmed == true
  and .source_non_execution_readback_ready_for_audit_index == true
  and .audit_index_entry_count == 8
  and .audit_index_blocker_count == 20
  and .required_prior_gate_count == 3
  and .audit_index_scope.index_mode == "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_report_only"
  and .audit_index_scope.index_visible == true
  and .audit_index_scope.index_recorded == false
  and .audit_index_scope.index_persisted == false
  and .audit_index_scope.index_authoritative == false
  and .audit_index_scope.index_accepted == false
  and .audit_index_scope.live_acceptance_allowed == false
  and .audit_index_scope_report_only_complete == true
  and (.audit_index_entries | map(.id) == [
    "replay_diff_plan_inventory_audit_index",
    "replay_scope_inventory_audit_index",
    "projection_diff_non_execution_audit_index",
    "redacted_payload_hash_non_execution_audit_index",
    "canary_task_result_shape_non_execution_audit_index",
    "idempotency_duplicate_suppression_non_execution_audit_index",
    "non_persistence_boundary_non_execution_audit_index",
    "live_boundary_non_execution_audit_index"
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
  and .audit_index_entries_report_only_complete == true
  and (.audit_index_blockers | map(.blocked_action) == [
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
  and (.audit_index_blockers | all(.blocked == true and .required_before_acceptance == true))
  and .audit_index_blockers_complete == true
  and .replay_diff_non_execution_readback_audit_index_preconditions_complete == true
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .non_execution_readback_visible == true
  and .non_execution_readback_executed == false
  and .non_execution_readback_recorded == false
  and .non_execution_readback_persisted == false
  and .audit_index_authorizes_readback_execution == false
  and .audit_index_authorizes_replay_execution == false
  and .audit_index_authorizes_replay_diff_recording == false
  and .audit_index_authorizes_replay_diff_persistence == false
  and .audit_index_authorizes_rollback_execution == false
  and .audit_index_authorizes_idempotency_mutation == false
  and .audit_index_authorizes_work_graph_event_persistence == false
  and .audit_index_authorizes_projection_persistence == false
  and .audit_index_authorizes_scheduler_guardrail_enforcement == false
  and .audit_index_authorizes_runtime_interception == false
  and .audit_index_authorizes_feature_flag_enablement == false
  and .audit_index_authorizes_canary_traffic == false
  and .audit_index_authorizes_operator_review_request == false
  and .audit_index_authorizes_approval_recording == false
  and .audit_index_authorizes_live_cutover == false
  and .ready_for_non_persistence_readback == true
  and .ready_for_live_execution == false
  and .source_probes.audit_index_module_present == true
  and .source_probes.non_execution_readback_gate_present == true
  and .source_probes.non_execution_readback_points_here == true
  and .source_probes.non_execution_readback_ready_present == true
  and .source_probes.non_execution_readback_no_execute_present == true
  and .source_probes.non_execution_readback_no_persist_present == true
  and .source_probes.non_execution_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate"
  and .source_probes.non_execution_readback_source_prior_readbacks_complete == true
  and .source_probes.non_execution_readback_ready_for_audit_index == true
  and .source_probes.non_execution_readback_no_execution_confirmed == true
  and .source_probes.non_execution_readback_no_authorization_confirmed == true
  and .source_probes.non_execution_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

for test_name in \
  replay_diff_non_execution_readback_audit_index_derives_from_readback \
  replay_diff_non_execution_readback_audit_index_is_visible_only \
  replay_diff_non_execution_readback_audit_index_blocks_execution_and_live_paths \
  replay_diff_non_execution_readback_audit_index_links_priors_and_side_effects; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board shadow event-store replay diff dry-run non-execution readback audit index gate passed"
