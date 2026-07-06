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

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-report.sh"
source_report="$("$SOURCE_REPORT_SCRIPT")"

readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback.rs
)"
readiness_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-gate.sh
)"
readiness_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness.rs
)"
readiness_ready_present="$(
  bool_for source_has "ready_for_attachability_precondition_readiness_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness.rs
)"
readiness_no_attachment_present="$(
  bool_for source_has "ready_for_live_attachment: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness.rs
)"
readiness_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness.rs
)"

jq -n \
  --argjson source "$source_report" \
  --argjson readback_module_present "$readback_module_present" \
  --argjson readiness_gate_present "$readiness_gate_present" \
  --argjson readiness_points_here "$readiness_points_here" \
  --argjson readiness_ready_present "$readiness_ready_present" \
  --argjson readiness_no_attachment_present "$readiness_no_attachment_present" \
  --argjson readiness_no_live_present "$readiness_no_live_present" \
  '
  def readback_entry($id; $key; $field; $category): {
    id: $id,
    stable_readback_key: $key,
    source_readiness_field: $field,
    readback_category: $category,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    mutation_allowed: false
  };
  def entrypoint_readback($id): {
    id: $id,
    source_entrypoint_id: $id,
    stable_readback_key: $id,
    live_attachment_candidate: true,
    live_attachment_allowed: false,
    report_only: true,
    readback_recorded: false,
    readback_persisted: false
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail live attachment attachability readiness readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_readiness_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_precondition_readiness",
    readback_mode: "live_attachment_attachability_precondition_readiness_visible_only_readback",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback",
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    mutation_allowed: false
  } as $readback_scope
  | [
    readback_entry("attachability_readiness_surface_readback"; "live_attachment.attachability_readiness.readback.surface"; "readiness_visible"; "readiness_surface"),
    readback_entry("attachability_entrypoint_inventory_readback"; "live_attachment.attachability_readiness.readback.entrypoints"; "attachability_entrypoint_count"; "entrypoint_inventory"),
    readback_entry("attachability_precondition_summary_readback"; "live_attachment.attachability_readiness.readback.preconditions"; "attachability_precondition_check_count"; "precondition_summary"),
    readback_entry("attachability_blocker_inventory_readback"; "live_attachment.attachability_readiness.readback.blockers"; "attachability_blocker_count"; "blocker_inventory"),
    readback_entry("attachability_prior_chain_readback"; "live_attachment.attachability_readiness.readback.required_priors"; "required_prior_gate_count"; "required_prior_chain"),
    readback_entry("attachability_non_persistence_boundary_readback"; "live_attachment.attachability_readiness.readback.non_persistence_boundary"; "readiness_persisted"; "non_persistence_boundary"),
    readback_entry("attachability_no_live_authority_readback"; "live_attachment.attachability_readiness.readback.no_live_authority"; "ready_for_live_execution"; "no_live_authority")
  ] as $readback_entries
  | [
    entrypoint_readback("spawn_agent"),
    entrypoint_readback("spawn_agents_on_csv"),
    entrypoint_readback("task_board_claim"),
    entrypoint_readback("worker_task_run")
  ] as $entrypoint_readbacks
  | (
    [
      blocker("readback_record_blocked"; "record_live_attachment_attachability_readback"),
      blocker("readback_persistence_blocked"; "persist_live_attachment_attachability_readback"),
      blocker("readback_acceptance_blocked"; "accept_live_attachment_attachability_readback")
    ]
    + ($source.attachability_blockers | map(blocker(.id; .blocked_action)))
  ) as $readback_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_report_only",
      source_attachability_readiness_gate: $source.gate,
      source_attachability_entrypoint_count: $source.attachability_entrypoint_count,
      source_attachability_precondition_check_count: $source.attachability_precondition_check_count,
      source_attachability_blocker_count: $source.attachability_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      readback_entry_count: ($readback_entries | length),
      entrypoint_readback_count: ($entrypoint_readbacks | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      entrypoint_readbacks: $entrypoint_readbacks,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_gate",
      source_readiness_visible: $source.readiness_visible,
      source_readiness_persisted: false,
      readback_visible: true,
      readback_recorded: false,
      readback_persisted: false,
      readback_authoritative: false,
      readback_accepted: false,
      attachability_candidates_readback_ready: true,
      attachability_preconditions_satisfied: false,
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
      ready_for_attachability_readback_audit_index: true,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_probes: {
        readback_module_present: $readback_module_present,
        readiness_gate_present: $readiness_gate_present,
        readiness_points_here: $readiness_points_here,
        readiness_ready_present: $readiness_ready_present,
        readiness_no_attachment_present: $readiness_no_attachment_present,
        readiness_no_live_present: $readiness_no_live_present
      },
      side_effects: {
        filesystem_written: false,
        readback_recorded: false,
        readback_persisted: false,
        readback_accepted: false,
        readiness_recorded: false,
        readiness_persisted: false,
        readiness_accepted: false,
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
