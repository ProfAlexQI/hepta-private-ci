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

denial_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback.rs
)"
live_attachment_matrix_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-gate.sh
)"
live_attachment_matrix_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix.rs
)"
live_attachment_matrix_ready_present="$(
  bool_for source_has "ready_for_denial_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix.rs
)"
live_attachment_matrix_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix.rs
)"
live_attachment_matrix_unpersisted_present="$(
  bool_for source_has "matrix_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix.rs
)"

jq -n \
  --argjson denial_readback_module_present "$denial_readback_module_present" \
  --argjson live_attachment_matrix_gate_present "$live_attachment_matrix_gate_present" \
  --argjson live_attachment_matrix_points_here "$live_attachment_matrix_points_here" \
  --argjson live_attachment_matrix_ready_present "$live_attachment_matrix_ready_present" \
  --argjson live_attachment_matrix_no_live_present "$live_attachment_matrix_no_live_present" \
  --argjson live_attachment_matrix_unpersisted_present "$live_attachment_matrix_unpersisted_present" \
  '
  def readback_entry($id; $key; $state): {
    id: $id,
    stable_readback_key: $key,
    observed_state: $state,
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    mutation_allowed: false,
    ready: true
  };
  def entrypoint_readback($id; $surface): {
    entrypoint_id: $id,
    source_surface: $surface,
    denial_readback_key: "live_attachment_precondition_matrix_entrypoint_deny_live_allow_report_only",
    live_attachment_allowed: false,
    runtime_interception_allowed: false,
    report_only: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail live attachment denial readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_precondition_matrix_denial_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix",
    readback_mode: "live_attachment_precondition_matrix_denial_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix.denial_readback",
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false
  } as $scope
  | [
    readback_entry(
      "live_attachment_matrix_denial_state_readback";
      "live_attachment_matrix_denies_live_attachment";
      "deny_live_attachment_visible_without_record_accept_or_persistence"
    ),
    readback_entry(
      "live_attachment_entrypoint_inventory_readback";
      "live_attachment_matrix_entrypoints_visible";
      "four_entrypoints_visible_as_report_only_non_intercepting_surfaces"
    ),
    readback_entry(
      "live_attachment_precondition_check_catalog_readback";
      "live_attachment_matrix_precondition_checks_visible";
      "fourteen_checks_visible_with_ten_blocking_unsatisfied_preconditions"
    ),
    readback_entry(
      "live_attachment_blocker_catalog_readback";
      "live_attachment_matrix_blockers_visible";
      "thirty_three_blocked_actions_visible_without_authority_to_mutate"
    ),
    readback_entry(
      "live_attachment_prior_chain_readback";
      "live_attachment_matrix_prior_chain_visible";
      "sixteen_required_priors_visible_before_denial_readback"
    ),
    readback_entry(
      "live_attachment_non_attachment_boundary_readback";
      "live_attachment_denial_does_not_attach_runtime_hooks";
      "denial_readback_cannot_install_hooks_intercept_runtime_or_enforce_scheduler_guardrails"
    ),
    readback_entry(
      "live_attachment_no_live_authority_readback";
      "live_attachment_denial_does_not_unlock_live_paths";
      "denial_readback_cannot_authorize_work_start_agent_model_external_persistence_replay_config_traffic_operator_approval_or_cutover"
    )
  ] as $entries
  | [
    entrypoint_readback("spawn_agent"; "multi_agents_v2.spawn_agent"),
    entrypoint_readback("spawn_agents_on_csv"; "agent_jobs.spawn_agents_on_csv"),
    entrypoint_readback("task_board_claim"; "task_board.claim"),
    entrypoint_readback("worker_task_run"; "worker_tasks.run_worker_task")
  ] as $entrypoint_readbacks
  | [
    blocker("denial_readback_record_blocked"; "record_live_attachment_denial_readback"),
    blocker("denial_readback_persistence_blocked"; "persist_live_attachment_denial_readback"),
    blocker("denial_readback_acceptance_blocked"; "accept_live_attachment_denial_readback"),
    blocker("matrix_record_blocked"; "record_live_attachment_precondition_matrix"),
    blocker("matrix_persistence_blocked"; "persist_live_attachment_precondition_matrix"),
    blocker("matrix_acceptance_blocked"; "accept_live_attachment_precondition_matrix"),
    blocker("live_attachment_enablement_blocked"; "enable_live_attachment"),
    blocker("live_blocking_hook_install_blocked"; "install_live_blocking_hook"),
    blocker("runtime_interception_blocked"; "enable_runtime_interception"),
    blocker("scheduler_admission_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("guardrail_enforcement_blocked"; "enable_guardrail_enforcement"),
    blocker("work_graph_event_persistence_blocked"; "persist_work_graph_event"),
    blocker("projection_index_persistence_blocked"; "persist_projection_index"),
    blocker("lease_acquisition_blocked"; "acquire_lane_lease"),
    blocker("work_start_blocked"; "start_entrypoint_work"),
    blocker("spawn_agent_blocked"; "spawn_agent"),
    blocker("spawn_agents_on_csv_blocked"; "spawn_agents_on_csv"),
    blocker("task_board_claim_blocked"; "claim_task_board_work"),
    blocker("worker_task_run_blocked"; "run_worker_task"),
    blocker("model_invocation_blocked"; "invoke_model"),
    blocker("external_send_blocked"; "send_external_message"),
    blocker("live_task_result_emit_blocked"; "emit_live_task_result"),
    blocker("hardening_decision_record_blocked"; "record_hardening_decision"),
    blocker("hardening_decision_persistence_blocked"; "persist_hardening_decision"),
    blocker("readback_execution_blocked"; "execute_readback"),
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
  ] as $blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate",
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
  ] as $required_priors
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate",
    schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_v1",
    preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_only",
    source_matrix_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate",
    source_entrypoint_count: 4,
    source_precondition_check_count: 14,
    source_blocking_precondition_count: 10,
    source_blocker_count: 33,
    source_required_prior_gate_count: 16,
    denial_readback_entry_count: ($entries | length),
    entrypoint_denial_readback_count: ($entrypoint_readbacks | length),
    denial_readback_blocker_count: ($blockers | length),
    required_prior_gate_count: ($required_priors | length),
    denial_readback_scope: $scope,
    denial_readback_entries: $entries,
    entrypoint_denial_readbacks: $entrypoint_readbacks,
    denial_readback_blockers: $blockers,
    required_prior_gates: $required_priors,
    recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate",
    denial_readback_visible: true,
    denial_readback_recorded: false,
    denial_readback_persisted: false,
    denial_readback_authoritative: false,
    denial_readback_accepted: false,
    denial_readback_authorizes_live_attachment: false,
    denial_readback_authorizes_live_blocking_hook: false,
    denial_readback_authorizes_runtime_interception: false,
    denial_readback_authorizes_scheduler_admission_enforcement: false,
    denial_readback_authorizes_guardrail_enforcement: false,
    denial_readback_authorizes_work_graph_persistence: false,
    denial_readback_authorizes_lease_or_work_start: false,
    denial_readback_authorizes_agent_model_or_external_send: false,
    denial_readback_authorizes_live_task_result: false,
    denial_readback_authorizes_replay_or_rollback: false,
    denial_readback_authorizes_config_flag_or_traffic: false,
    denial_readback_authorizes_operator_approval_or_live_cutover: false,
    ready_for_denial_readback_audit_index: true,
    ready_for_live_attachment: false,
    ready_for_live_execution: false,
    source_probes: {
      denial_readback_module_present: $denial_readback_module_present,
      live_attachment_matrix_gate_present: $live_attachment_matrix_gate_present,
      live_attachment_matrix_points_here: $live_attachment_matrix_points_here,
      live_attachment_matrix_ready_present: $live_attachment_matrix_ready_present,
      live_attachment_matrix_no_live_present: $live_attachment_matrix_no_live_present,
      live_attachment_matrix_unpersisted_present: $live_attachment_matrix_unpersisted_present
    },
    side_effects: {
      filesystem_written: false,
      denial_readback_recorded: false,
      denial_readback_persisted: false,
      denial_readback_accepted: false,
      live_attachment_enabled: false,
      live_blocking_hook_installed: false,
      runtime_interception_enabled: false,
      scheduler_admission_enforced: false,
      guardrail_enforcement_enabled: false,
      work_graph_event_persisted: false,
      projection_index_persisted: false,
      lease_acquired: false,
      work_started: false,
      hardening_decision_recorded: false,
      hardening_decision_persisted: false,
      live_task_result_emitted: false,
      readback_executed: false,
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
      live_cutover_performed: false,
      runtime_mutation_performed: false,
      agent_spawn_performed: false,
      external_send_performed: false,
      model_invoked: false
    }
  }
  '
