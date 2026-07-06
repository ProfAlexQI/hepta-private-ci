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

final_closeout_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout.rs
)"
non_persistence_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-non-persistence-readback-gate.sh
)"
non_persistence_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_ready_present="$(
  bool_for source_has "ready_for_terminal_no_enforcement_final_closeout: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_unpersisted_present="$(
  bool_for source_has "readback_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback.rs
)"

jq -n \
  --argjson final_closeout_module_present "$final_closeout_module_present" \
  --argjson non_persistence_readback_gate_present "$non_persistence_readback_gate_present" \
  --argjson non_persistence_readback_points_here "$non_persistence_readback_points_here" \
  --argjson non_persistence_readback_ready_present "$non_persistence_readback_ready_present" \
  --argjson non_persistence_readback_no_live_present "$non_persistence_readback_no_live_present" \
  --argjson non_persistence_readback_unpersisted_present "$non_persistence_readback_unpersisted_present" \
  '
  def entry($id; $key; $source; $category): {
    id: $id,
    stable_closeout_key: $key,
    source_readback_id: $source,
    closeout_category: $category,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    mutation_allowed: false,
    closed: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail entrypoint hardening terminal no-enforcement final closeout cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening.readback_audit_index_non_persistence_readback",
    closeout_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_report_only",
    stable_closeout_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.terminal_no_enforcement.final_closeout",
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    terminal: true,
    mutation_allowed: false
  } as $final_closeout_scope
  | [
    entry("hardening_no_enforcement_branch_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.branch_closed"; "hardening_audit_index_no_live_authority_readback"; "terminal_no_enforcement_branch"),
    entry("hardening_audit_index_surface_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.audit_index_surface"; "hardening_audit_index_surface_non_persistence_readback"; "audit_index_surface"),
    entry("hardening_audit_index_entry_inventory_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.audit_index_entries"; "hardening_audit_index_entry_inventory_non_persistence_readback"; "audit_index_entry_inventory"),
    entry("hardening_audit_index_blocker_inventory_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.audit_index_blockers"; "hardening_audit_index_blocker_inventory_non_persistence_readback"; "audit_index_blocker_inventory"),
    entry("hardening_audit_index_prior_chain_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.required_priors"; "hardening_audit_index_prior_chain_non_persistence_readback"; "required_prior_chain"),
    entry("hardening_non_persistence_boundary_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.non_persistence_boundary"; "hardening_audit_index_non_persistence_boundary_readback"; "non_persistence_boundary"),
    entry("hardening_no_live_authority_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.no_live_authority"; "hardening_audit_index_no_live_authority_readback"; "no_live_authority"),
    entry("hardening_entrypoint_scope_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.entrypoint_scope"; "hardening_audit_index_entry_inventory_non_persistence_readback"; "entrypoint_scope"),
    entry("hardening_scheduler_guardrail_boundary_final_closeout"; "entrypoint_hardening.terminal_no_enforcement.final_closeout.scheduler_guardrail_boundary"; "scheduler_guardrail_live_enforcement_blocked"; "scheduler_guardrail_boundary")
  ] as $final_closeout_entries
  | [
    blocker("final_closeout_record_blocked"; "record_hardening_terminal_no_enforcement_final_closeout"),
    blocker("final_closeout_persistence_blocked"; "persist_hardening_terminal_no_enforcement_final_closeout"),
    blocker("final_closeout_acceptance_blocked"; "accept_hardening_terminal_no_enforcement_final_closeout"),
    blocker("audit_index_readback_record_blocked"; "record_hardening_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_persistence_blocked"; "persist_hardening_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_acceptance_blocked"; "accept_hardening_audit_index_non_persistence_readback"),
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
  ] as $final_closeout_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
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
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_report_only",
      source_non_persistence_readback_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
      source_readback_entry_count: 6,
      source_readback_blocker_count: 33,
      source_required_prior_gate_count: 14,
      final_closeout_entry_count: ($final_closeout_entries | length),
      final_closeout_blocker_count: ($final_closeout_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      final_closeout_scope: $final_closeout_scope,
      final_closeout_entries: $final_closeout_entries,
      final_closeout_blockers: $final_closeout_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate",
      terminal_no_enforcement_branch_closed: true,
      final_closeout_visible: true,
      final_closeout_recorded: false,
      final_closeout_persisted: false,
      final_closeout_authoritative: false,
      final_closeout_accepted: false,
      source_audit_index_visible: true,
      source_audit_index_persisted: false,
      source_readback_persisted: false,
      hardening_decision_recording_allowed: false,
      hardening_decision_persistence_allowed: false,
      live_blocking_enforcement_allowed: false,
      runtime_interception_allowed: false,
      scheduler_admission_enforcement_allowed: false,
      guardrail_enforcement_allowed: false,
      work_graph_event_persistence_allowed: false,
      projection_persistence_allowed: false,
      lease_acquisition_allowed: false,
      work_start_allowed: false,
      agent_spawn_allowed: false,
      model_invocation_allowed: false,
      external_send_allowed: false,
      replay_execution_allowed: false,
      replay_diff_recording_allowed: false,
      replay_diff_persistence_allowed: false,
      rollback_execution_allowed: false,
      idempotency_mutation_allowed: false,
      config_write_allowed: false,
      feature_flag_mutation_allowed: false,
      canary_traffic_allowed: false,
      operator_review_request_allowed: false,
      approval_recording_allowed: false,
      live_cutover_allowed: false,
      ready_for_live_attachment_precondition_matrix: true,
      ready_for_live_execution: false,
      source_probes: {
        final_closeout_module_present: $final_closeout_module_present,
        non_persistence_readback_gate_present: $non_persistence_readback_gate_present,
        non_persistence_readback_points_here: $non_persistence_readback_points_here,
        non_persistence_readback_ready_present: $non_persistence_readback_ready_present,
        non_persistence_readback_no_live_present: $non_persistence_readback_no_live_present,
        non_persistence_readback_unpersisted_present: $non_persistence_readback_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
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
        replay_executed: false,
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
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
    }
  '
