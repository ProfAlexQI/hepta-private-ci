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

non_persistence_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback.rs
)"
audit_index_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-non-execution-readback-audit-index-gate.sh
)"
audit_index_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index.rs
)"
audit_index_ready_present="$(
  bool_for source_has "ready_for_non_persistence_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index.rs
)"
audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index.rs
)"
audit_index_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index.rs
)"

jq -n \
  --argjson non_persistence_readback_module_present "$non_persistence_readback_module_present" \
  --argjson audit_index_gate_present "$audit_index_gate_present" \
  --argjson audit_index_points_here "$audit_index_points_here" \
  --argjson audit_index_ready_present "$audit_index_ready_present" \
  --argjson audit_index_unpersisted_present "$audit_index_unpersisted_present" \
  --argjson audit_index_no_live_present "$audit_index_no_live_present" \
  '
  def entry($id; $key; $state): {
    id: $id,
    stable_readback_key: $key,
    observed_state: $state,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "replay/diff non-execution readback audit index non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.work_graph.shadow_event_store.replay_diff_dry_run_non_execution_readback_audit_index",
    readback_mode: "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.shadow_event_store.replay_diff_dry_run_non_execution_readback.audit_index.non_persistence_readback",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    readback_recorded: false,
    readback_persisted: false,
    readback_accepted: false
  } as $readback_scope
  | [
    entry("replay_diff_audit_index_surface_non_persistence_readback"; "replay_diff_non_execution_readback_audit_index_visible_unrecorded"; "audit_index_visible_without_record_persist_accept_or_authority"),
    entry("replay_diff_audit_index_entry_inventory_non_persistence_readback"; "replay_diff_non_execution_readback_audit_index_entries_visible"; "eight_audit_index_entries_visible_but_not_persisted"),
    entry("replay_diff_audit_index_blocker_inventory_non_persistence_readback"; "replay_diff_non_execution_readback_audit_index_blockers_visible"; "twenty_blockers_visible_and_still_blocking"),
    entry("replay_diff_audit_index_prior_chain_non_persistence_readback"; "replay_diff_non_execution_readback_audit_index_priors_visible"; "four_required_prior_gates_visible_but_not_persisted"),
    entry("replay_diff_audit_index_non_persistence_boundary_readback"; "replay_diff_non_execution_readback_audit_index_non_persistence_boundary"; "audit_index_does_not_write_event_projection_replay_diff_or_idempotency_state"),
    entry("replay_diff_audit_index_no_live_authority_readback"; "replay_diff_non_execution_readback_audit_index_no_live_authority"; "audit_index_does_not_authorize_execution_enforcement_interception_or_live_cutover")
  ] as $readback_entries
  | [
    blocker("audit_index_readback_record_blocked"; "record_replay_diff_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_persistence_blocked"; "persist_replay_diff_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_acceptance_blocked"; "accept_replay_diff_audit_index_non_persistence_readback"),
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
  ] as $readback_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
      source_audit_index_entry_count: 8,
      source_audit_index_blocker_count: 20,
      source_required_prior_gate_count: 3,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      non_execution_readback_visible: true,
      non_execution_readback_executed: false,
      non_execution_readback_recorded: false,
      non_execution_readback_persisted: false,
      audit_index_readback_recorded: false,
      audit_index_readback_persisted: false,
      audit_index_readback_accepted: false,
      readback_execution_allowed: false,
      replay_execution_allowed: false,
      replay_diff_recording_allowed: false,
      replay_diff_persistence_allowed: false,
      rollback_execution_allowed: false,
      idempotency_mutation_allowed: false,
      work_graph_event_persistence_allowed: false,
      projection_persistence_allowed: false,
      scheduler_guardrail_enforcement_allowed: false,
      runtime_interception_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      operator_review_request_allowed: false,
      approval_recording_allowed: false,
      live_cutover_allowed: false,
      ready_for_terminal_no_execution_final_closeout: true,
      ready_for_live_execution: false,
      source_probes: {
        non_persistence_readback_module_present: $non_persistence_readback_module_present,
        audit_index_gate_present: $audit_index_gate_present,
        audit_index_points_here: $audit_index_points_here,
        audit_index_ready_present: $audit_index_ready_present,
        audit_index_unpersisted_present: $audit_index_unpersisted_present,
        audit_index_no_live_present: $audit_index_no_live_present
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
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
