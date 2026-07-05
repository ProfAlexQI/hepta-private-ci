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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index.rs
)"
hardening_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-gate.sh
)"
hardening_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback.rs
)"
hardening_readback_ready_present="$(
  bool_for source_has "ready_for_audit_index: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback.rs
)"
hardening_readback_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback.rs
)"
hardening_readback_no_persist_present="$(
  bool_for source_has "work_graph_event_persistence_allowed: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback.rs
)"

jq -n \
  --argjson audit_index_module_present "$audit_index_module_present" \
  --argjson hardening_readback_gate_present "$hardening_readback_gate_present" \
  --argjson hardening_readback_points_here "$hardening_readback_points_here" \
  --argjson hardening_readback_ready_present "$hardening_readback_ready_present" \
  --argjson hardening_readback_no_live_present "$hardening_readback_no_live_present" \
  --argjson hardening_readback_no_persist_present "$hardening_readback_no_persist_present" \
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
    reason: "required before scheduler/guardrail hardening readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback",
    index_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback.audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    index_accepted: false,
    live_acceptance_allowed: false
  } as $audit_index_scope
  | [
    entry("hardening_readback_scope_audit_index"; "entrypoint_hardening_readback.audit_index.scope"; "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_scope"; "hardening_readback_scope"),
    entry("hardening_readback_entry_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.readback_entry_inventory"; "hardening_contract_inventory_readback"; "hardening_readback_entries"),
    entry("hardening_entrypoint_readback_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.entrypoint_readback_inventory"; "spawn_agent_hardening_readback"; "entrypoint_readbacks"),
    entry("hardening_readback_blocker_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.blocker_inventory"; "hardening_blocker_inventory_readback"; "readback_blockers"),
    entry("hardening_readback_prior_chain_audit_index"; "entrypoint_hardening_readback.audit_index.prior_chain"; "hardening_prior_chain_readback"; "required_prior_chain"),
    entry("hardening_readback_non_live_guard_audit_index"; "entrypoint_hardening_readback.audit_index.non_live_guard"; "hardening_non_live_guard_readback"; "non_live_guard_contract"),
    entry("hardening_readback_no_live_authority_audit_index"; "entrypoint_hardening_readback.audit_index.no_live_authority"; "hardening_no_live_authority_readback"; "no_live_authority"),
    entry("hardening_source_decision_trace_audit_index"; "entrypoint_hardening_readback.audit_index.source_decision_trace"; "hardening_decision_inventory_readback"; "source_decision_trace"),
    entry("hardening_live_boundary_audit_index"; "entrypoint_hardening_readback.audit_index.live_boundary"; "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"; "live_cutover_boundary")
  ] as $audit_index_entries
  | [
    blocker("audit_index_record_blocked"; "record_hardening_readback_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_hardening_readback_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_hardening_readback_audit_index"),
    blocker("readback_record_blocked"; "record_hardening_readback"),
    blocker("readback_persistence_blocked"; "persist_hardening_readback"),
    blocker("readback_acceptance_blocked"; "accept_hardening_readback"),
    blocker("hardening_decision_record_blocked"; "record_hardening_decision"),
    blocker("hardening_decision_persistence_blocked"; "persist_hardening_decision"),
    blocker("live_blocking_hook_install_blocked"; "install_live_blocking_hook"),
    blocker("runtime_interception_blocked"; "enable_runtime_interception"),
    blocker("scheduler_admission_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("guardrail_enforcement_blocked"; "enable_guardrail_enforcement"),
    blocker("work_graph_event_persistence_blocked"; "persist_work_graph_event"),
    blocker("projection_index_persistence_blocked"; "persist_projection_index"),
    blocker("lease_acquisition_blocked"; "acquire_lane_lease"),
    blocker("work_start_blocked"; "start_entrypoint_work"),
    blocker("agent_spawn_blocked"; "spawn_agent"),
    blocker("model_invocation_blocked"; "invoke_model"),
    blocker("external_send_blocked"; "send_external_message"),
    blocker("replay_execution_blocked"; "execute_replay"),
    blocker("replay_diff_recording_blocked"; "record_replay_diff"),
    blocker("replay_diff_persistence_blocked"; "persist_replay_diff"),
    blocker("rollback_execution_blocked"; "execute_rollback"),
    blocker("idempotency_mutation_blocked"; "mutate_idempotency_index"),
    blocker("config_write_blocked"; "write_config"),
    blocker("feature_flag_mutation_blocked"; "mutate_feature_flag"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("approval_recording_blocked"; "record_operator_approval"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $audit_index_blockers
  | [
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
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
      source_hardening_readback_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate",
      source_readback_entry_count: 7,
      source_entrypoint_readback_count: 4,
      source_readback_blocker_count: 27,
      source_required_prior_gate_count: 12,
      audit_index_entry_count: ($audit_index_entries | length),
      audit_index_blocker_count: ($audit_index_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      audit_index_scope: $audit_index_scope,
      audit_index_entries: $audit_index_entries,
      audit_index_blockers: $audit_index_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      hardening_readback_visible: true,
      hardening_readback_recorded: false,
      hardening_readback_persisted: false,
      hardening_readback_accepted: false,
      audit_index_authorizes_hardening_readback_recording: false,
      audit_index_authorizes_hardening_readback_persistence: false,
      audit_index_authorizes_hardening_decision_recording: false,
      audit_index_authorizes_hardening_decision_persistence: false,
      audit_index_authorizes_live_blocking_enforcement: false,
      audit_index_authorizes_runtime_interception: false,
      audit_index_authorizes_scheduler_admission_enforcement: false,
      audit_index_authorizes_guardrail_enforcement: false,
      audit_index_authorizes_work_graph_event_persistence: false,
      audit_index_authorizes_projection_persistence: false,
      audit_index_authorizes_lease_acquisition: false,
      audit_index_authorizes_work_start: false,
      audit_index_authorizes_agent_spawn: false,
      audit_index_authorizes_model_invocation: false,
      audit_index_authorizes_external_send: false,
      audit_index_authorizes_replay_execution: false,
      audit_index_authorizes_replay_diff_recording: false,
      audit_index_authorizes_replay_diff_persistence: false,
      audit_index_authorizes_rollback_execution: false,
      audit_index_authorizes_idempotency_mutation: false,
      audit_index_authorizes_config_write: false,
      audit_index_authorizes_feature_flag_mutation: false,
      audit_index_authorizes_canary_traffic: false,
      audit_index_authorizes_operator_review_request: false,
      audit_index_authorizes_approval_recording: false,
      audit_index_authorizes_live_cutover: false,
      ready_for_non_persistence_readback: true,
      ready_for_live_execution: false,
      source_probes: {
        audit_index_module_present: $audit_index_module_present,
        hardening_readback_gate_present: $hardening_readback_gate_present,
        hardening_readback_points_here: $hardening_readback_points_here,
        hardening_readback_ready_present: $hardening_readback_ready_present,
        hardening_readback_no_live_present: $hardening_readback_no_live_present,
        hardening_readback_no_persist_present: $hardening_readback_no_persist_present
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        hardening_readback_recorded: false,
        hardening_readback_persisted: false,
        hardening_readback_accepted: false,
        hardening_decision_recorded: false,
        hardening_decision_persisted: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        live_blocking_hook_installed: false,
        runtime_interception_enabled: false,
        lease_acquired: false,
        work_started: false,
        config_written: false,
        feature_flag_mutated: false,
        canary_traffic_routed: false,
        operator_review_requested: false,
        approval_recorded: false,
        replay_executed: false,
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
