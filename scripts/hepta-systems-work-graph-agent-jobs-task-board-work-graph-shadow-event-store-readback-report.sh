#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-readback-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-readback-gate.sh
)"
scheduler_guardrail_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-gate.sh
)"
shadow_path_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-gate.sh
)"
canary_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-canary-readback-replay-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson scheduler_guardrail_gate_present "$scheduler_guardrail_gate_present" \
  --argjson shadow_path_gate_present "$shadow_path_gate_present" \
  --argjson canary_readback_gate_present "$canary_readback_gate_present" \
  '
  def readback_entry($id; $source; $target; $fields; $evidence): {
    id: $id,
    source_ref: $source,
    readback_target: $target,
    required_fields: $fields,
    evidence_ref: $evidence,
    status: "readback_ready_not_executed",
    visible: true,
    executed: false,
    recorded: false,
    persisted: false,
    authoritative: false
  };
  def join($id; $entrypoint; $surface; $trace; $kind; $event_ref; $scheduler_ref; $projection; $readback; $diff): {
    id: $id,
    entrypoint_id: $entrypoint,
    source_surface_id: $surface,
    dry_run_trace_id: $trace,
    shadow_event_kind: $kind,
    shadow_event_ref: $event_ref,
    scheduler_event_ref: $scheduler_ref,
    projection_index_ref: $projection,
    readback_evidence_ref: $readback,
    replay_diff_ref: $diff,
    joined: true,
    persisted: false,
    live_enforced: false
  };
  def blocker($id; $blocks): {
    id: $id,
    blocks: $blocks,
    reason: "required before shadow event-store readback can become persisted or live",
    required_before_enablement: true
  };
  [
    readback_entry(
      "entrypoint_dry_run_decision_shadow_event_readback";
      "scheduler_guardrail_blocking_dry_run_entrypoint";
      "entrypoint_dry_run_decision_join";
      ["entrypointId", "dryRunDecision", "traceId", "deterministicEventId"];
      "evidence:entrypoint-dry-run-shadow-event-readback"
    ),
    readback_entry(
      "redacted_payload_hash_shadow_readback";
      "append_only_event_store_shadow_path";
      "redacted_payload_hash";
      ["eventId", "redactedPayloadRef", "payloadHash"];
      "evidence:redacted-payload-hash-shadow-readback"
    ),
    readback_entry(
      "projection_index_shadow_readback";
      "append_only_event_store_shadow_path";
      "projection_index";
      ["projectionIndexKey", "collectionId", "deterministicOrder"];
      "evidence:projection-index-shadow-readback"
    ),
    readback_entry(
      "canary_report_only_shadow_readback";
      "agent_jobs_task_board_canary_readback_replay";
      "canary_report_only_join";
      ["workGraphReportOnly", "taskId", "traceId"];
      "evidence:canary-report-only-shadow-readback"
    ),
    readback_entry(
      "replay_diff_preview_shadow_readback";
      "append_only_event_store_shadow_path";
      "replay_diff_preview";
      ["replayDiffRef", "payloadHash", "expectedDiff"];
      "evidence:replay-diff-preview-shadow-readback"
    ),
    readback_entry(
      "shadow_event_store_non_persistence_readback";
      "shadow_event_store_boundary";
      "non_persistence_boundary";
      ["shadowPersisted", "eventStoreEnabled", "liveCutoverEnabled"];
      "evidence:shadow-event-store-non-persistence-readback"
    )
  ] as $readback_entries
  | [
    join(
      "spawn_agent_shadow_event_store_readback_join";
      "spawn_agent";
      "multi_agent_v2_thread_spawn";
      "trace-blocking-dry-run-spawn-agent-001";
      "AgentTaskSpawned";
      "wg-event-shadow-spawn-001";
      "wg-event-shadow-scheduler-admission-001";
      "projection_by_trace_id";
      "shadow_readback_scheduler_admission_join";
      "shadow_replay_noop_projection_diff"
    ),
    join(
      "agent_jobs_csv_shadow_event_store_readback_join";
      "spawn_agents_on_csv";
      "agent_jobs_batch_workers";
      "trace-blocking-dry-run-agent-jobs-001";
      "TaskResultReported";
      "wg-event-shadow-agent-job-result-001";
      "wg-event-shadow-scheduler-admission-001";
      "projection_by_task_id";
      "shadow_readback_terminal_task_result_join";
      "shadow_replay_duplicate_event_suppression_diff"
    ),
    join(
      "task_board_claim_shadow_event_store_readback_join";
      "task_board_claim";
      "hepta_runtime_task_board";
      "trace-blocking-dry-run-task-board-001";
      "TaskBoardTerminalEvent";
      "wg-event-shadow-task-board-terminal-001";
      "wg-event-shadow-scheduler-admission-001";
      "projection_by_task_id";
      "shadow_readback_scheduler_admission_join";
      "shadow_replay_projection_index_rebuild_diff"
    ),
    join(
      "worker_task_run_shadow_event_store_readback_join";
      "worker_task_run";
      "hepta_runtime_worker_tasks";
      "trace-blocking-dry-run-worker-task-001";
      "ArtifactProduced";
      "wg-event-shadow-worker-artifact-001";
      "wg-event-shadow-scheduler-admission-001";
      "projection_by_source_surface";
      "shadow_readback_payload_hash_check";
      "shadow_replay_redaction_hash_stability_diff"
    )
  ] as $shadow_event_joins
  | [
    blocker("shadow_event_store_enablement_blocked"; "event_store_enablement"),
    blocker("shadow_event_persistence_blocked"; "shadow_event_persistence"),
    blocker("projection_index_persistence_blocked"; "projection_index_persistence"),
    blocker("readback_execution_blocked"; "readback_execution"),
    blocker("readback_recording_blocked"; "readback_recording"),
    blocker("replay_execution_blocked"; "replay_execution"),
    blocker("replay_diff_persistence_blocked"; "replay_diff_persistence"),
    blocker("scheduler_guardrail_live_enforcement_blocked"; "scheduler_guardrail_live_enforcement"),
    blocker("runtime_interception_blocked"; "runtime_interception"),
    blocker("feature_flag_enablement_blocked"; "feature_flag_enablement"),
    blocker("canary_traffic_blocked"; "canary_traffic"),
    blocker("operator_review_request_blocked"; "operator_review_request"),
    blocker("approval_recording_blocked"; "approval_recording"),
    blocker("live_cutover_blocked"; "live_cutover")
  ] as $non_persistence_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_v1",
      preview_mode: "work_graph_shadow_event_store_readback_ready_no_persistence_no_live",
      source_scheduler_guardrail_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
      source_entrypoint_binding_count: 4,
      source_dry_run_decision_count: 4,
      source_shadow_path_gate: "hepta_work_graph_append_only_event_store_shadow_path_gate",
      source_shadow_event_record_count: 8,
      source_projection_index_count: 5,
      source_readback_evidence_count: 5,
      source_replay_diff_count: 4,
      source_canary_readback_replay_gate: "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
      source_canary_entrypoint_count: 2,
      source_canary_projection_index_count: 2,
      source_canary_readback_evidence_count: 2,
      source_canary_replay_diff_count: 2,
      readback_entry_count: ($readback_entries | length),
      shadow_event_join_count: ($shadow_event_joins | length),
      non_persistence_blocker_count: ($non_persistence_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_entries: $readback_entries,
      shadow_event_joins: $shadow_event_joins,
      non_persistence_blockers: $non_persistence_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
      shadow_event_store_readback_ready: true,
      entrypoint_shadow_event_join_ready: true,
      redacted_payload_hash_join_ready: true,
      projection_index_readback_ready: true,
      canary_readback_join_ready: true,
      replay_diff_readback_ready: true,
      shadow_readback_executed: false,
      shadow_event_persistence_enabled: false,
      projection_index_persistence_enabled: false,
      scheduler_guardrail_live_enforcement_enabled: false,
      runtime_interception_enabled: false,
      ready_for_replay_diff_dry_run: true,
      ready_for_live_execution: false,
      source_probes: {
        shadow_event_store_readback: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        scheduler_guardrail_blocking_dry_run_entrypoint: {
          gate_script_present: $scheduler_guardrail_gate_present
        },
        append_only_event_store_shadow_path: {
          gate_script_present: $shadow_path_gate_present
        },
        agent_jobs_task_board_canary_readback_replay: {
          gate_script_present: $canary_readback_gate_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        shadow_event_persisted: false,
        projection_index_persisted: false,
        readback_executed: false,
        readback_recorded: false,
        readback_persisted: false,
        replay_executed: false,
        replay_diff_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        runtime_interception_enabled: false,
        config_written: false,
        feature_flag_mutated: false,
        canary_traffic_routed: false,
        operator_review_requested: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
