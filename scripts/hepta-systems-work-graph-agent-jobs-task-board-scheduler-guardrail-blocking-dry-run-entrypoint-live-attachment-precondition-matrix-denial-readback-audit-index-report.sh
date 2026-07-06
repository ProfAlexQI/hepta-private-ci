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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index.rs
)"
denial_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-gate.sh
)"
denial_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback.rs
)"
denial_readback_ready_present="$(
  bool_for source_has "ready_for_denial_readback_audit_index: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback.rs
)"
denial_readback_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback.rs
)"
denial_readback_unpersisted_present="$(
  bool_for source_has "denial_readback_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback.rs
)"

jq -n \
  --argjson audit_index_module_present "$audit_index_module_present" \
  --argjson denial_readback_gate_present "$denial_readback_gate_present" \
  --argjson denial_readback_points_here "$denial_readback_points_here" \
  --argjson denial_readback_ready_present "$denial_readback_ready_present" \
  --argjson denial_readback_no_live_present "$denial_readback_no_live_present" \
  --argjson denial_readback_unpersisted_present "$denial_readback_unpersisted_present" \
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
    reason: "required before live attachment denial readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_denial_readback_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback",
    index_mode: "live_attachment_precondition_matrix_denial_readback_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix.denial_readback.audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    index_accepted: false,
    live_acceptance_allowed: false
  } as $scope
  | [
    entry("live_attachment_denial_readback_scope_audit_index"; "live_attachment_denial_readback.audit_index.scope"; "agent_jobs_task_board_scheduler_guardrail_live_attachment_precondition_matrix_denial_readback_scope"; "denial_readback_scope"),
    entry("live_attachment_denial_state_audit_index"; "live_attachment_denial_readback.audit_index.denial_state"; "live_attachment_matrix_denial_state_readback"; "denial_state"),
    entry("live_attachment_entrypoint_inventory_audit_index"; "live_attachment_denial_readback.audit_index.entrypoint_inventory"; "live_attachment_entrypoint_inventory_readback"; "entrypoint_inventory"),
    entry("live_attachment_precondition_check_catalog_audit_index"; "live_attachment_denial_readback.audit_index.precondition_checks"; "live_attachment_precondition_check_catalog_readback"; "precondition_check_catalog"),
    entry("live_attachment_blocker_inventory_audit_index"; "live_attachment_denial_readback.audit_index.blocker_inventory"; "live_attachment_blocker_catalog_readback"; "blocker_inventory"),
    entry("live_attachment_prior_chain_audit_index"; "live_attachment_denial_readback.audit_index.prior_chain"; "live_attachment_prior_chain_readback"; "prior_chain"),
    entry("live_attachment_non_attachment_boundary_audit_index"; "live_attachment_denial_readback.audit_index.non_attachment_boundary"; "live_attachment_non_attachment_boundary_readback"; "non_attachment_boundary"),
    entry("live_attachment_no_live_authority_audit_index"; "live_attachment_denial_readback.audit_index.no_live_authority"; "live_attachment_no_live_authority_readback"; "no_live_authority"),
    entry("live_attachment_entrypoint_surface_audit_index"; "live_attachment_denial_readback.audit_index.entrypoint_surface"; "live_attachment_precondition_matrix_entrypoint_deny_live_allow_report_only"; "entrypoint_surface_denial")
  ] as $entries
  | [
    blocker("audit_index_record_blocked"; "record_live_attachment_denial_readback_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_live_attachment_denial_readback_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_live_attachment_denial_readback_audit_index"),
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
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate",
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
    gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate",
    schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_v1",
    preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report_only",
    source_denial_readback_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate",
    source_denial_readback_entry_count: 7,
    source_entrypoint_denial_readback_count: 4,
    source_denial_readback_blocker_count: 36,
    source_required_prior_gate_count: 17,
    audit_index_entry_count: ($entries | length),
    audit_index_blocker_count: ($blockers | length),
    required_prior_gate_count: ($required_priors | length),
    audit_index_scope: $scope,
    audit_index_entries: $entries,
    audit_index_blockers: $blockers,
    required_prior_gates: $required_priors,
    recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    denial_readback_visible: true,
    denial_readback_recorded: false,
    denial_readback_persisted: false,
    denial_readback_authoritative: false,
    denial_readback_accepted: false,
    audit_index_authorizes_denial_readback_recording: false,
    audit_index_authorizes_denial_readback_persistence: false,
    audit_index_authorizes_matrix_recording: false,
    audit_index_authorizes_matrix_persistence: false,
    audit_index_authorizes_live_attachment: false,
    audit_index_authorizes_live_blocking_hook: false,
    audit_index_authorizes_runtime_interception: false,
    audit_index_authorizes_scheduler_admission_enforcement: false,
    audit_index_authorizes_guardrail_enforcement: false,
    audit_index_authorizes_work_graph_persistence: false,
    audit_index_authorizes_projection_persistence: false,
    audit_index_authorizes_lease_or_work_start: false,
    audit_index_authorizes_agent_model_or_external_send: false,
    audit_index_authorizes_live_task_result: false,
    audit_index_authorizes_replay_or_rollback: false,
    audit_index_authorizes_config_flag_or_traffic: false,
    audit_index_authorizes_operator_approval_or_live_cutover: false,
    ready_for_non_persistence_readback: true,
    ready_for_live_attachment: false,
    ready_for_live_execution: false,
    source_probes: {
      audit_index_module_present: $audit_index_module_present,
      denial_readback_gate_present: $denial_readback_gate_present,
      denial_readback_points_here: $denial_readback_points_here,
      denial_readback_ready_present: $denial_readback_ready_present,
      denial_readback_no_live_present: $denial_readback_no_live_present,
      denial_readback_unpersisted_present: $denial_readback_unpersisted_present
    },
    side_effects: {
      filesystem_written: false,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_accepted: false,
      denial_readback_recorded: false,
      denial_readback_persisted: false,
      denial_readback_accepted: false,
      matrix_recorded: false,
      matrix_persisted: false,
      matrix_accepted: false,
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
