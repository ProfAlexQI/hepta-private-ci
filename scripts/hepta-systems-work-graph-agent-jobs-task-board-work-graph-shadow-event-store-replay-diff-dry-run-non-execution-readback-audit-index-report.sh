#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

source_has() {
  local pattern="$1"
  local path="$2"
  rg -q "$pattern" "$path"
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

audit_index_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index.rs
)"
non_execution_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-gate.sh
)"
non_execution_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback.rs
)"
non_execution_readback_ready_present="$(
  bool_for source_has "ready_for_audit_index: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback.rs
)"
non_execution_readback_no_execute_present="$(
  bool_for source_has "readback_execution_enabled: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback.rs
)"
non_execution_readback_no_persist_present="$(
  bool_for source_has "readback_persistence_enabled: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback.rs
)"

jq -n \
  --argjson audit_index_module_present "$audit_index_module_present" \
  --argjson non_execution_readback_gate_present "$non_execution_readback_gate_present" \
  --argjson non_execution_readback_points_here "$non_execution_readback_points_here" \
  --argjson non_execution_readback_ready_present "$non_execution_readback_ready_present" \
  --argjson non_execution_readback_no_execute_present "$non_execution_readback_no_execute_present" \
  --argjson non_execution_readback_no_persist_present "$non_execution_readback_no_persist_present" \
  '
  def entry($id; $key; $source; $category): {
    id: $id,
    stable_index_key: $key,
    source_readback_id: $source,
    audit_category: $category,
    indexed: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "required before replay/diff non-execution readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.work_graph.shadow_event_store.replay_diff_dry_run_non_execution_readback",
    index_mode: "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.shadow_event_store.replay_diff_dry_run_non_execution_readback.audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    index_accepted: false,
    live_acceptance_allowed: false
  } as $audit_index_scope
  | [
    entry("replay_diff_plan_inventory_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.plan_inventory"; "replay_diff_plan_inventory_non_execution_readback"; "replay_diff_plan_inventory"),
    entry("replay_scope_inventory_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.scope_inventory"; "replay_scope_inventory_non_execution_readback"; "replay_scope_inventory"),
    entry("projection_diff_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.projection_diff"; "projection_diff_non_execution_readback"; "projection_diff_no_execution"),
    entry("redacted_payload_hash_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.redacted_payload_hash"; "redacted_payload_hash_non_execution_readback"; "redacted_payload_hash_stability"),
    entry("canary_task_result_shape_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.canary_task_result_shape"; "canary_task_result_shape_non_execution_readback"; "canary_task_result_shape_diff"),
    entry("idempotency_duplicate_suppression_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.idempotency_duplicate_suppression"; "idempotency_duplicate_suppression_non_execution_readback"; "idempotency_duplicate_suppression_no_mutation"),
    entry("non_persistence_boundary_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.non_persistence_boundary"; "non_persistence_boundary_non_execution_readback"; "non_persistence_boundary"),
    entry("live_boundary_non_execution_audit_index"; "replay_diff_dry_run_non_execution_readback.audit_index.live_boundary"; "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate"; "live_cutover_boundary")
  ] as $audit_index_entries
  | [
    blocker("audit_index_record_blocked"; "record_replay_diff_non_execution_readback_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_replay_diff_non_execution_readback_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_replay_diff_non_execution_readback_audit_index"),
    blocker("readback_execution_blocked"; "execute_non_execution_readback"),
    blocker("readback_recording_blocked"; "record_non_execution_readback"),
    blocker("readback_persistence_blocked"; "persist_non_execution_readback"),
    blocker("replay_execution_blocked"; "execute_replay"),
    blocker("replay_diff_recording_blocked"; "record_replay_diff"),
    blocker("replay_diff_persistence_blocked"; "persist_replay_diff"),
    blocker("rollback_execution_blocked"; "execute_rollback"),
    blocker("idempotency_mutation_blocked"; "mutate_idempotency_index"),
    blocker("work_graph_event_persistence_blocked"; "persist_work_graph_event"),
    blocker("projection_index_persistence_blocked"; "persist_projection_index"),
    blocker("scheduler_guardrail_live_enforcement_blocked"; "enable_scheduler_guardrail_live_enforcement"),
    blocker("runtime_interception_blocked"; "enable_runtime_interception"),
    blocker("feature_flag_enablement_blocked"; "enable_feature_flag"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("approval_recording_blocked"; "record_operator_approval"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $audit_index_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_v1",
      preview_mode: "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_report_only",
      source_non_execution_readback_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
      source_non_execution_readback_entry_count: 7,
      source_replay_scope_readback_count: 4,
      source_non_execution_blocker_count: 18,
      source_required_prior_gate_count: 2,
      audit_index_entry_count: ($audit_index_entries | length),
      audit_index_blocker_count: ($audit_index_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      audit_index_scope: $audit_index_scope,
      audit_index_entries: $audit_index_entries,
      audit_index_blockers: $audit_index_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      non_execution_readback_visible: true,
      non_execution_readback_executed: false,
      non_execution_readback_recorded: false,
      non_execution_readback_persisted: false,
      audit_index_authorizes_readback_execution: false,
      audit_index_authorizes_replay_execution: false,
      audit_index_authorizes_replay_diff_recording: false,
      audit_index_authorizes_replay_diff_persistence: false,
      audit_index_authorizes_rollback_execution: false,
      audit_index_authorizes_idempotency_mutation: false,
      audit_index_authorizes_work_graph_event_persistence: false,
      audit_index_authorizes_projection_persistence: false,
      audit_index_authorizes_scheduler_guardrail_enforcement: false,
      audit_index_authorizes_runtime_interception: false,
      audit_index_authorizes_feature_flag_enablement: false,
      audit_index_authorizes_canary_traffic: false,
      audit_index_authorizes_operator_review_request: false,
      audit_index_authorizes_approval_recording: false,
      audit_index_authorizes_live_cutover: false,
      ready_for_non_persistence_readback: true,
      ready_for_live_execution: false,
      source_probes: {
        audit_index_module_present: $audit_index_module_present,
        non_execution_readback_gate_present: $non_execution_readback_gate_present,
        non_execution_readback_points_here: $non_execution_readback_points_here,
        non_execution_readback_ready_present: $non_execution_readback_ready_present,
        non_execution_readback_no_execute_present: $non_execution_readback_no_execute_present,
        non_execution_readback_no_persist_present: $non_execution_readback_no_persist_present
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        readback_executed: false,
        readback_recorded: false,
        readback_persisted: false,
        replay_executed: false,
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        shadow_event_persisted: false,
        projection_index_persisted: false,
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
