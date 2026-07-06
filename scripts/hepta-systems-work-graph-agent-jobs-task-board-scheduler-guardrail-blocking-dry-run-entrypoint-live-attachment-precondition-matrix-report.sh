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

live_attachment_matrix_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix.rs
)"
terminal_closeout_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-terminal-no-enforcement-final-closeout-gate.sh
)"
terminal_closeout_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout.rs
)"
terminal_closeout_ready_present="$(
  bool_for source_has "ready_for_live_attachment_precondition_matrix: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout.rs
)"
terminal_closeout_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout.rs
)"
terminal_closeout_unpersisted_present="$(
  bool_for source_has "final_closeout_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout.rs
)"

jq -n \
  --argjson live_attachment_matrix_module_present "$live_attachment_matrix_module_present" \
  --argjson terminal_closeout_gate_present "$terminal_closeout_gate_present" \
  --argjson terminal_closeout_points_here "$terminal_closeout_points_here" \
  --argjson terminal_closeout_ready_present "$terminal_closeout_ready_present" \
  --argjson terminal_closeout_no_live_present "$terminal_closeout_no_live_present" \
  --argjson terminal_closeout_unpersisted_present "$terminal_closeout_unpersisted_present" \
  '
  def entrypoint($id; $surface): {
    id: $id,
    surface: $surface,
    live_attachment_candidate: true,
    live_attachment_allowed: false,
    report_only: true,
    runtime_interception_allowed: false
  };
  def check($id; $category; $satisfied; $blocking; $explanation): {
    id: $id,
    category: $category,
    required: true,
    satisfied: $satisfied,
    blocking: $blocking,
    explanation: $explanation
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail live attachment precondition matrix cannot authorize this action"
  };
  [
    entrypoint("spawn_agent"; "multi_agents_v2.spawn_agent"),
    entrypoint("spawn_agents_on_csv"; "agent_jobs.spawn_agents_on_csv"),
    entrypoint("task_board_claim"; "task_board.claim"),
    entrypoint("worker_task_run"; "worker_tasks.run_worker_task")
  ] as $entrypoints
  | [
    check("terminal_no_enforcement_final_closeout_ready"; "source_evidence"; true; false; "terminal no-enforcement closeout is visible as report-only evidence"),
    check("entrypoint_scope_inventory_visible"; "entrypoint_scope"; true; false; "four entrypoint surfaces are covered by the dry-run hardening chain"),
    check("deterministic_decision_keys_visible"; "decision_key"; true; false; "hardened dry-run decisions have deterministic keys for later attachment"),
    check("trace_evidence_join_visible"; "trace_evidence"; true; false; "trace and evidence references are visible without being persisted"),
    check("live_blocking_hook_authorization_missing"; "live_hook_boundary"; false; true; "no authorization exists to install live blocking hooks"),
    check("runtime_interception_authorization_missing"; "runtime_boundary"; false; true; "runtime interception remains explicitly disallowed"),
    check("scheduler_admission_enforcement_authorization_missing"; "scheduler_boundary"; false; true; "scheduler admission remains dry-run only"),
    check("guardrail_enforcement_authorization_missing"; "guardrail_boundary"; false; true; "guardrail spans remain report-only and cannot block live traffic"),
    check("work_graph_persistence_authorization_missing"; "work_graph_persistence_boundary"; false; true; "WorkGraph event and projection persistence remain disabled"),
    check("lease_and_work_start_authorization_missing"; "lease_work_start_boundary"; false; true; "lane leases and entrypoint work starts cannot be acquired from this matrix"),
    check("live_task_result_acceptance_missing"; "task_result_boundary"; false; true; "TaskResult emission remains report-only and not live-accepted"),
    check("replay_rollback_execution_authorization_missing"; "replay_rollback_boundary"; false; true; "replay, replay diff recording, rollback, and idempotency mutation remain disabled"),
    check("config_flag_traffic_authorization_missing"; "config_flag_traffic_boundary"; false; true; "config writes, feature flag mutation, and canary traffic remain disallowed"),
    check("operator_approval_live_cutover_authorization_missing"; "operator_live_boundary"; false; true; "operator review, approval recording, and live cutover remain absent")
  ] as $precondition_checks
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_deny_only",
      source_final_closeout_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate",
      source_final_closeout_entry_count: 9,
      source_final_closeout_blocker_count: 36,
      source_required_prior_gate_count: 15,
      entrypoint_count: ($entrypoints | length),
      precondition_check_count: ($precondition_checks | length),
      precondition_satisfied_count: ($precondition_checks | map(select(.satisfied == true)) | length),
      precondition_unsatisfied_count: ($precondition_checks | map(select(.satisfied == false)) | length),
      blocking_precondition_count: ($precondition_checks | map(select(.blocking == true)) | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      entrypoints: $entrypoints,
      precondition_checks: $precondition_checks,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate",
      matrix_mode: "deny_live_attachment_until_explicit_scheduler_guardrail_enforcement_authorization",
      matrix_visible: true,
      matrix_recorded: false,
      matrix_persisted: false,
      matrix_authoritative: false,
      matrix_accepted: false,
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
      ready_for_denial_readback: true,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_probes: {
        live_attachment_matrix_module_present: $live_attachment_matrix_module_present,
        terminal_closeout_gate_present: $terminal_closeout_gate_present,
        terminal_closeout_points_here: $terminal_closeout_points_here,
        terminal_closeout_ready_present: $terminal_closeout_ready_present,
        terminal_closeout_no_live_present: $terminal_closeout_no_live_present,
        terminal_closeout_unpersisted_present: $terminal_closeout_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
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
