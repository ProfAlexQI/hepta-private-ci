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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback.rs
)"
audit_index_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-audit-index-gate.sh
)"
audit_index_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index.rs
)"
audit_index_ready_present="$(
  bool_for source_has "ready_for_non_persistence_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index.rs
)"
audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index.rs
)"
audit_index_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index.rs
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
    authoritative: false,
    accepted: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "live attachment denial readback audit index non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_denial_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback_audit_index",
    readback_mode: "live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix.denial_readback.audit_index.non_persistence_readback",
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
    entry("live_attachment_audit_index_surface_non_persistence_readback"; "live_attachment_denial_readback_audit_index_visible_unrecorded"; "audit_index_visible_without_record_persist_accept_or_authority"),
    entry("live_attachment_audit_index_entry_inventory_non_persistence_readback"; "live_attachment_denial_readback_audit_index_entries_visible"; "nine_audit_index_entries_visible_but_not_persisted"),
    entry("live_attachment_audit_index_blocker_inventory_non_persistence_readback"; "live_attachment_denial_readback_audit_index_blockers_visible"; "thirty_nine_blockers_visible_and_still_blocking"),
    entry("live_attachment_audit_index_prior_chain_non_persistence_readback"; "live_attachment_denial_readback_audit_index_priors_visible"; "eighteen_required_prior_gates_visible_but_not_persisted"),
    entry("live_attachment_audit_index_non_persistence_boundary_readback"; "live_attachment_denial_readback_audit_index_non_persistence_boundary"; "audit_index_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("live_attachment_audit_index_no_live_authority_readback"; "live_attachment_denial_readback_audit_index_no_live_authority"; "audit_index_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover")
  ] as $readback_entries
  | [
    blocker("audit_index_readback_record_blocked"; "record_live_attachment_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_persistence_blocked"; "persist_live_attachment_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_acceptance_blocked"; "accept_live_attachment_audit_index_non_persistence_readback"),
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
  ] as $readback_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate",
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
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate",
      source_audit_index_entry_count: 9,
      source_audit_index_blocker_count: 39,
      source_required_prior_gate_count: 18,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_gate",
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
      audit_index_readback_recorded: false,
      audit_index_readback_persisted: false,
      audit_index_readback_accepted: false,
      matrix_recording_allowed: false,
      matrix_persistence_allowed: false,
      live_attachment_allowed: false,
      live_blocking_hook_install_allowed: false,
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
      live_task_result_emission_allowed: false,
      hardening_decision_recording_allowed: false,
      hardening_decision_persistence_allowed: false,
      readback_execution_allowed: false,
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
      ready_for_terminal_no_attachment_final_closeout: true,
      ready_for_live_attachment: false,
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
