#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_v1"
  and .preview_mode == "work_graph_shadow_event_store_replay_diff_dry_run_no_execute_no_persist_no_live"
  and .source_shadow_event_store_readback_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  and .source_readback_entry_count == 6
  and .source_shadow_event_join_count == 4
  and .source_non_persistence_blocker_count == 14
  and .source_append_only_shadow_path_gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .source_shadow_path_replay_diff_count == 4
  and .replay_diff_plan_count == 6
  and .replay_scope_count == 4
  and .non_execution_blocker_count == 16
  and .required_prior_gate_count == 2
  and (.replay_diff_plans | map(.id) == [
    "entrypoint_shadow_join_noop_projection_diff",
    "redacted_payload_hash_stability_diff",
    "projection_index_rebuild_dry_run_diff",
    "scheduler_admission_duplicate_suppression_diff",
    "canary_report_only_task_result_diff",
    "shadow_event_store_non_persistence_boundary_diff"
  ])
  and (.replay_diff_plans | all(
    .dry_run_ready == true
    and .replay_executed == false
    and .diff_recorded == false
    and .diff_persisted == false
    and .live_enforced == false
    and (.compared_fields | length > 0)
  ))
  and (.replay_scopes | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.replay_scopes | all(
    .dry_run_only == true
    and (.trace_id | startswith("trace-blocking-dry-run-"))
    and (.shadow_event_ref | startswith("wg-event-shadow-"))
    and (.projection_index_ref | startswith("projection_by_"))
    and (.replay_diff_ref | startswith("shadow_replay_"))
  ))
  and (.non_execution_blockers | map(.blocks) == [
    "replay_execution",
    "replay_diff_recording",
    "replay_diff_persistence",
    "projection_rebuild_execution",
    "idempotency_mutation",
    "readback_execution",
    "work_graph_event_persistence",
    "projection_index_persistence",
    "scheduler_guardrail_live_enforcement",
    "runtime_interception",
    "feature_flag_enablement",
    "canary_traffic",
    "operator_review_request",
    "approval_recording",
    "rollback_execution",
    "live_cutover"
  ])
  and (.non_execution_blockers | all(.required_before_execution == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate"
  and .deterministic_replay_plan_ready == true
  and .projection_diff_plan_ready == true
  and .duplicate_suppression_diff_ready == true
  and .redaction_hash_diff_ready == true
  and .task_result_canary_diff_ready == true
  and .replay_execution_enabled == false
  and .replay_diff_recording_enabled == false
  and .replay_diff_persistence_enabled == false
  and .rollback_execution_enabled == false
  and .shadow_event_persistence_enabled == false
  and .projection_index_persistence_enabled == false
  and .scheduler_guardrail_live_enforcement_enabled == false
  and .runtime_interception_enabled == false
  and .ready_for_non_execution_readback == true
  and .ready_for_live_execution == false
  and .source_probes.replay_diff_dry_run.rust_module_present == true
  and .source_probes.replay_diff_dry_run.report_script_present == true
  and .source_probes.replay_diff_dry_run.gate_script_present == true
  and .source_probes.shadow_event_store_readback.gate_script_present == true
  and .source_probes.append_only_event_store_shadow_path.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run --lib

echo "Hepta WorkGraph agent_jobs + task_board shadow event-store replay diff dry-run gate passed"
