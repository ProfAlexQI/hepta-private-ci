#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_v1"
  and .preview_mode == "work_graph_shadow_event_store_readback_ready_no_persistence_no_live"
  and .source_scheduler_guardrail_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate"
  and .source_entrypoint_binding_count == 4
  and .source_dry_run_decision_count == 4
  and .source_scheduler_guardrail_ready == true
  and .source_scheduler_guardrail_no_live_confirmed == true
  and .source_shadow_path_gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .source_shadow_event_record_count == 8
  and .source_projection_index_count == 5
  and .source_readback_evidence_count == 5
  and .source_replay_diff_count == 4
  and .source_shadow_path_readiness_complete == true
  and .source_shadow_path_no_persistence_confirmed == true
  and .source_canary_readback_replay_gate == "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
  and .source_canary_entrypoint_count == 2
  and .source_canary_projection_index_count == 2
  and .source_canary_readback_evidence_count == 2
  and .source_canary_replay_diff_count == 2
  and .source_canary_readback_replay_ready == true
  and .source_canary_readback_replay_no_live_confirmed == true
  and .readback_entry_count == 6
  and .shadow_event_join_count == 4
  and .non_persistence_blocker_count == 14
  and .required_prior_gate_count == 3
  and (.readback_entries | map(.id) == [
    "entrypoint_dry_run_decision_shadow_event_readback",
    "redacted_payload_hash_shadow_readback",
    "projection_index_shadow_readback",
    "canary_report_only_shadow_readback",
    "replay_diff_preview_shadow_readback",
    "shadow_event_store_non_persistence_readback"
  ])
  and (.readback_entries | all(
    .status == "readback_ready_not_executed"
    and .visible == true
    and .executed == false
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and (.required_fields | length > 0)
  ))
  and (.shadow_event_joins | map(.entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.shadow_event_joins | all(
    .joined == true
    and .persisted == false
    and .live_enforced == false
    and (.dry_run_trace_id | startswith("trace-blocking-dry-run-"))
    and (.shadow_event_ref | startswith("wg-event-shadow-"))
    and .scheduler_event_ref == "wg-event-shadow-scheduler-admission-001"
  ))
  and (.shadow_event_joins | map(.shadow_event_kind) == [
    "AgentTaskSpawned",
    "TaskResultReported",
    "TaskBoardTerminalEvent",
    "ArtifactProduced"
  ])
  and (.non_persistence_blockers | map(.blocks) == [
    "event_store_enablement",
    "shadow_event_persistence",
    "projection_index_persistence",
    "readback_execution",
    "readback_recording",
    "replay_execution",
    "replay_diff_persistence",
    "scheduler_guardrail_live_enforcement",
    "runtime_interception",
    "feature_flag_enablement",
    "canary_traffic",
    "operator_review_request",
    "approval_recording",
    "live_cutover"
  ])
  and (.non_persistence_blockers | all(.required_before_enablement == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
  ]
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate"
  and .source_prior_readbacks_complete == true
  and .readback_entries_visible_only_complete == true
  and .shadow_event_joins_report_only_complete == true
  and .non_persistence_blockers_complete == true
  and .shadow_event_store_readback_ready == true
  and .entrypoint_shadow_event_join_ready == true
  and .redacted_payload_hash_join_ready == true
  and .projection_index_readback_ready == true
  and .canary_readback_join_ready == true
  and .replay_diff_readback_ready == true
  and .shadow_readback_executed == false
  and .shadow_event_persistence_enabled == false
  and .projection_index_persistence_enabled == false
  and .scheduler_guardrail_live_enforcement_enabled == false
  and .runtime_interception_enabled == false
  and .ready_for_replay_diff_dry_run == true
  and .ready_for_live_execution == false
  and .source_probes.shadow_event_store_readback.rust_module_present == true
  and .source_probes.shadow_event_store_readback.report_script_present == true
  and .source_probes.shadow_event_store_readback.gate_script_present == true
  and .source_probes.scheduler_guardrail_blocking_dry_run_entrypoint.gate_script_present == true
  and .source_probes.scheduler_guardrail_blocking_dry_run_entrypoint.report_gate == true
  and .source_probes.scheduler_guardrail_blocking_dry_run_entrypoint.ready_for_shadow_event_store_readback == true
  and .source_probes.scheduler_guardrail_blocking_dry_run_entrypoint.no_live_confirmed == true
  and .source_probes.scheduler_guardrail_blocking_dry_run_entrypoint.side_effects_all_false == true
  and .source_probes.append_only_event_store_shadow_path.gate_script_present == true
  and .source_probes.append_only_event_store_shadow_path.report_gate == true
  and .source_probes.append_only_event_store_shadow_path.readiness_complete == true
  and .source_probes.append_only_event_store_shadow_path.no_persistence_confirmed == true
  and .source_probes.append_only_event_store_shadow_path.side_effects_all_false == true
  and .source_probes.agent_jobs_task_board_canary_readback_replay.gate_script_present == true
  and .source_probes.agent_jobs_task_board_canary_readback_replay.report_gate == true
  and .source_probes.agent_jobs_task_board_canary_readback_replay.readiness_complete == true
  and .source_probes.agent_jobs_task_board_canary_readback_replay.no_live_confirmed == true
  and .source_probes.agent_jobs_task_board_canary_readback_replay.side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  shadow_event_store_readback_derives_from_scheduler_shadow_path_and_canary --lib
cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  shadow_event_store_readback_declares_entrypoint_joins --lib
cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  shadow_event_store_readback_stays_non_persistent_and_non_live --lib
cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  shadow_event_store_readback_links_required_priors_and_side_effects --lib

echo "Hepta WorkGraph agent_jobs + task_board shadow event-store readback gate passed"
