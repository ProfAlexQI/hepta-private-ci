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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-gate.sh
)"
replay_diff_dry_run_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-gate.sh
)"
shadow_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-readback-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson replay_diff_dry_run_gate_present "$replay_diff_dry_run_gate_present" \
  --argjson shadow_readback_gate_present "$shadow_readback_gate_present" \
  '
  def readback_entry($id; $source; $target; $fields; $evidence): {
    id: $id,
    source_plan_id: $source,
    readback_target: $target,
    required_fields: $fields,
    evidence_ref: $evidence,
    status: "non_execution_readback_ready_not_executed",
    visible: true,
    executed: false,
    recorded: false,
    persisted: false,
    authoritative: false
  };
  def scope_readback($id; $entrypoint; $scope; $trace; $event_ref): {
    id: $id,
    entrypoint_id: $entrypoint,
    replay_scope_ref: $scope,
    trace_id: $trace,
    shadow_event_ref: $event_ref,
    readback_status: "scope_readback_ready_not_executed",
    dry_run_only: true,
    replay_executed: false,
    diff_recorded: false,
    persisted: false
  };
  def blocker($id; $blocks): {
    id: $id,
    blocks: $blocks,
    reason: "required before non-execution readback can be recorded, accepted, enforced, or cut live",
    required_before_audit_acceptance: true
  };
  [
    readback_entry(
      "replay_diff_plan_inventory_non_execution_readback";
      "entrypoint_shadow_join_noop_projection_diff";
      "replay_diff_plan_inventory";
      ["replayDiffPlanId", "dryRunReady", "replayExecuted", "diffPersisted"];
      "evidence:replay-diff-plan-inventory-non-execution-readback"
    ),
    readback_entry(
      "replay_scope_inventory_non_execution_readback";
      "spawn_agent_replay_diff_scope";
      "replay_scope_inventory";
      ["entrypointId", "traceId", "shadowEventRef", "dryRunOnly"];
      "evidence:replay-scope-inventory-non-execution-readback"
    ),
    readback_entry(
      "projection_diff_non_execution_readback";
      "projection_index_rebuild_dry_run_diff";
      "projection_diff_no_execution";
      ["projectionIndexRef", "expectedResult", "replayExecuted"];
      "evidence:projection-diff-non-execution-readback"
    ),
    readback_entry(
      "redacted_payload_hash_non_execution_readback";
      "redacted_payload_hash_stability_diff";
      "redacted_payload_hash_no_execution";
      ["redactedPayloadRef", "payloadHash", "diffRecorded"];
      "evidence:redacted-payload-hash-non-execution-readback"
    ),
    readback_entry(
      "canary_task_result_shape_non_execution_readback";
      "canary_report_only_task_result_diff";
      "canary_task_result_shape_no_execution";
      ["workGraphReportOnly", "taskId", "diffPersisted"];
      "evidence:canary-task-result-shape-non-execution-readback"
    ),
    readback_entry(
      "idempotency_duplicate_suppression_non_execution_readback";
      "scheduler_admission_duplicate_suppression_diff";
      "idempotency_duplicate_suppression_no_mutation";
      ["deterministicEventId", "traceId", "idempotencyMutated"];
      "evidence:idempotency-duplicate-suppression-non-execution-readback"
    ),
    readback_entry(
      "non_persistence_boundary_non_execution_readback";
      "shadow_event_store_non_persistence_boundary_diff";
      "non_persistence_boundary_no_live";
      ["shadowPersisted", "eventStoreEnabled", "liveCutoverEnabled"];
      "evidence:non-persistence-boundary-non-execution-readback"
    )
  ] as $non_execution_readback_entries
  | [
    scope_readback(
      "spawn_agent_replay_scope_non_execution_readback";
      "spawn_agent";
      "spawn_agent_replay_diff_scope";
      "trace-blocking-dry-run-spawn-agent-001";
      "wg-event-shadow-spawn-001"
    ),
    scope_readback(
      "spawn_agents_on_csv_replay_scope_non_execution_readback";
      "spawn_agents_on_csv";
      "spawn_agents_on_csv_replay_diff_scope";
      "trace-blocking-dry-run-agent-jobs-001";
      "wg-event-shadow-agent-job-result-001"
    ),
    scope_readback(
      "task_board_claim_replay_scope_non_execution_readback";
      "task_board_claim";
      "task_board_claim_replay_diff_scope";
      "trace-blocking-dry-run-task-board-001";
      "wg-event-shadow-task-board-terminal-001"
    ),
    scope_readback(
      "worker_task_run_replay_scope_non_execution_readback";
      "worker_task_run";
      "worker_task_run_replay_diff_scope";
      "trace-blocking-dry-run-worker-task-001";
      "wg-event-shadow-worker-artifact-001"
    )
  ] as $replay_scope_readbacks
  | [
    blocker("readback_execution_blocked"; "readback_execution"),
    blocker("readback_recording_blocked"; "readback_recording"),
    blocker("readback_persistence_blocked"; "readback_persistence"),
    blocker("replay_execution_blocked"; "replay_execution"),
    blocker("replay_diff_recording_blocked"; "replay_diff_recording"),
    blocker("replay_diff_persistence_blocked"; "replay_diff_persistence"),
    blocker("rollback_execution_blocked"; "rollback_execution"),
    blocker("idempotency_mutation_blocked"; "idempotency_mutation"),
    blocker("work_graph_event_persistence_blocked"; "work_graph_event_persistence"),
    blocker("projection_index_persistence_blocked"; "projection_index_persistence"),
    blocker("scheduler_guardrail_live_enforcement_blocked"; "scheduler_guardrail_live_enforcement"),
    blocker("runtime_interception_blocked"; "runtime_interception"),
    blocker("feature_flag_enablement_blocked"; "feature_flag_enablement"),
    blocker("canary_traffic_blocked"; "canary_traffic"),
    blocker("operator_review_request_blocked"; "operator_review_request"),
    blocker("approval_recording_blocked"; "approval_recording"),
    blocker("audit_index_acceptance_blocked"; "audit_index_acceptance"),
    blocker("live_cutover_blocked"; "live_cutover")
  ] as $non_execution_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_v1",
      preview_mode: "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_no_execute_no_persist_no_live",
      source_replay_diff_dry_run_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
      source_replay_diff_plan_count: 6,
      source_replay_scope_count: 4,
      source_non_execution_blocker_count: 16,
      source_shadow_event_store_readback_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
      source_shadow_readback_entry_count: 6,
      source_shadow_event_join_count: 4,
      non_execution_readback_entry_count: ($non_execution_readback_entries | length),
      replay_scope_readback_count: ($replay_scope_readbacks | length),
      non_execution_blocker_count: ($non_execution_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      non_execution_readback_entries: $non_execution_readback_entries,
      replay_scope_readbacks: $replay_scope_readbacks,
      non_execution_blockers: $non_execution_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
      dry_run_non_execution_readback_ready: true,
      replay_diff_plan_readback_ready: true,
      replay_scope_readback_ready: true,
      side_effect_boundary_readback_ready: true,
      replay_execution_confirmed_absent: true,
      replay_diff_recording_confirmed_absent: true,
      replay_diff_persistence_confirmed_absent: true,
      rollback_execution_confirmed_absent: true,
      idempotency_mutation_confirmed_absent: true,
      readback_execution_enabled: false,
      readback_recording_enabled: false,
      readback_persistence_enabled: false,
      replay_execution_enabled: false,
      replay_diff_persistence_enabled: false,
      shadow_event_persistence_enabled: false,
      scheduler_guardrail_live_enforcement_enabled: false,
      runtime_interception_enabled: false,
      ready_for_audit_index: true,
      ready_for_live_execution: false,
      source_probes: {
        non_execution_readback: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        replay_diff_dry_run: {
          gate_script_present: $replay_diff_dry_run_gate_present
        },
        shadow_event_store_readback: {
          gate_script_present: $shadow_readback_gate_present
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
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
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
