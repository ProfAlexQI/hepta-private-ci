#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-terminal-closeout-report.sh"
source_report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-terminal-closeout-report" \
    "$SOURCE_REPORT_SCRIPT"
)"

jq -n \
  --argjson source "$source_report" \
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
    reason: "terminal no-attachment final closeout readback terminal closeout readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
    readback_mode: "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment_final_closeout.readback.terminal_closeout.readback",
    source_closeout_visible: true,
    source_closeout_recorded: false,
    source_closeout_persisted: false,
    source_closeout_authoritative: false,
    source_closeout_accepted: false,
    readback_recorded: false,
    readback_persisted: false,
    readback_accepted: false
  } as $readback_scope
  | [
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_surface_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_visible_unrecorded"; "terminal_closeout_visible_without_record_persist_accept_or_authority"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_entry_inventory_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_entries_visible"; "eight_terminal_closeout_entries_visible_but_not_persisted"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_blocker_inventory_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_blockers_visible"; "seventy_four_blockers_visible_and_still_blocking"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_prior_chain_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_priors_visible"; "twenty_nine_required_prior_gates_visible_but_not_persisted"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_non_persistence_boundary_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_non_persistence_boundary"; "terminal_closeout_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_no_live_authority_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_no_live_authority"; "terminal_closeout_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_next_gate_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_next_gate"; "readback_is_ready_for_audit_index_only_not_live_attachment")
  ] as $readback_entries
  | (
    [
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_record_blocked"; "record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback"),
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_persistence_blocked"; "persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback"),
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_acceptance_blocked"; "accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback")
    ]
    + ($source.terminal_closeout_blockers | map(blocker(.id; .blocked_action)))
  ) as $readback_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | ($source.side_effects | to_entries | all(.value == false)) as $source_side_effects_all_false
  | ($source.terminal_closeout_preconditions_complete == true
      and $source.terminal_closeout_visible == true
      and $source.terminal_closeout_recorded == false
      and $source.terminal_closeout_persisted == false
      and $source.terminal_closeout_authoritative == false
      and $source.terminal_closeout_accepted == false
      and $source.source_readback_recorded == false
      and $source.source_readback_persisted == false
      and $source.source_readback_accepted == false
      and $source.terminal_closeout_readback_recording_allowed == false
      and $source.terminal_closeout_readback_persistence_allowed == false
      and $source.terminal_closeout_persistence_allowed == false
      and $source.work_graph_event_persistence_allowed == false
      and $source.projection_persistence_allowed == false
      and $source_side_effects_all_false) as $source_terminal_closeout_no_persistence_confirmed
  | ($source.ready_for_terminal_closeout_readback == true
      and $source.live_attachment_allowed == false
      and $source.live_blocking_hook_install_allowed == false
      and $source.runtime_interception_allowed == false
      and $source.scheduler_admission_enforcement_allowed == false
      and $source.guardrail_enforcement_allowed == false
      and $source.lease_acquisition_allowed == false
      and $source.work_start_allowed == false
      and $source.agent_spawn_allowed == false
      and $source.model_invocation_allowed == false
      and $source.external_send_allowed == false
      and $source.live_task_result_emission_allowed == false
      and $source.readback_execution_allowed == false
      and $source.replay_execution_allowed == false
      and $source.replay_diff_recording_allowed == false
      and $source.replay_diff_persistence_allowed == false
      and $source.rollback_execution_allowed == false
      and $source.idempotency_mutation_allowed == false
      and $source.config_write_allowed == false
      and $source.feature_flag_mutation_allowed == false
      and $source.canary_traffic_allowed == false
      and $source.operator_review_request_allowed == false
      and $source.approval_recording_allowed == false
      and $source.live_cutover_allowed == false
      and $source.ready_for_live_attachment == false
      and $source.ready_for_live_execution == false
      and $source_terminal_closeout_no_persistence_confirmed) as $source_terminal_closeout_no_live_confirmed
  | ($source.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_gate"
      and $source.source_non_persistence_readback_ready == true
      and $source.source_non_persistence_readback_no_persistence_confirmed == true
      and $source.source_non_persistence_readback_no_live_confirmed == true
      and $source.source_non_persistence_readback_ready_for_terminal_closeout == true
      and $source.terminal_closeout_scope_complete == true
      and $source.terminal_closeout_entries_complete == true
      and $source.terminal_closeout_blockers_complete == true
      and $source.terminal_closeout_preconditions_complete == true
      and $source.terminal_closeout_entry_count == 8
      and $source.terminal_closeout_blocker_count == 74
      and $source.required_prior_gate_count == 29
      and $source_terminal_closeout_no_live_confirmed) as $source_terminal_closeout_ready
  | ($source_terminal_closeout_ready
      and $source.ready_for_terminal_closeout_readback == true) as $source_terminal_closeout_ready_for_readback
  | ($readback_scope.source_closeout_visible == true
      and $readback_scope.source_closeout_recorded == false
      and $readback_scope.source_closeout_persisted == false
      and $readback_scope.source_closeout_authoritative == false
      and $readback_scope.source_closeout_accepted == false
      and $readback_scope.readback_recorded == false
      and $readback_scope.readback_persisted == false
      and $readback_scope.readback_accepted == false) as $readback_scope_complete
  | (($readback_entries | length) == 7
      and ($readback_entries | all(
        .visible == true
        and .recorded == false
        and .persisted == false
        and .authoritative == false
        and .accepted == false
        and .mutation_allowed == false
        and .ready == true
      ))) as $readback_entries_complete
  | (($readback_blockers | length) == 77
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_terminal_closeout_ready_for_readback
      and $readback_scope_complete
      and $readback_entries_complete
      and $readback_blockers_complete) as $terminal_closeout_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_only",
      source_terminal_closeout_gate: $source.gate,
      source_terminal_closeout_entry_count: $source.terminal_closeout_entry_count,
      source_terminal_closeout_blocker_count: $source.terminal_closeout_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      source_terminal_closeout_ready: $source_terminal_closeout_ready,
      source_terminal_closeout_no_persistence_confirmed: $source_terminal_closeout_no_persistence_confirmed,
      source_terminal_closeout_no_live_confirmed: $source_terminal_closeout_no_live_confirmed,
      source_terminal_closeout_ready_for_readback: $source_terminal_closeout_ready_for_readback,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_gate",
      readback_scope_complete: $readback_scope_complete,
      readback_entries_complete: $readback_entries_complete,
      readback_blockers_complete: $readback_blockers_complete,
      terminal_closeout_readback_preconditions_complete: $terminal_closeout_readback_preconditions_complete,
      source_terminal_closeout_visible: $source.terminal_closeout_visible,
      source_terminal_closeout_recorded: $source.terminal_closeout_recorded,
      source_terminal_closeout_persisted: $source.terminal_closeout_persisted,
      source_terminal_closeout_authoritative: $source.terminal_closeout_authoritative,
      source_terminal_closeout_accepted: $source.terminal_closeout_accepted,
      terminal_closeout_readback_visible: true,
      terminal_closeout_readback_recorded: false,
      terminal_closeout_readback_persisted: false,
      terminal_closeout_readback_authoritative: false,
      terminal_closeout_readback_accepted: false,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
      terminal_closeout_recording_allowed: false,
      terminal_closeout_persistence_allowed: false,
      terminal_closeout_acceptance_allowed: false,
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
      ready_for_terminal_closeout_readback_audit_index: $terminal_closeout_readback_preconditions_complete,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_readbacks: {
        terminal_closeout_report_gate: $source.gate,
        terminal_closeout_preconditions_complete: $source.terminal_closeout_preconditions_complete,
        terminal_closeout_ready_for_readback: $source.ready_for_terminal_closeout_readback,
        terminal_closeout_no_persistence_confirmed: $source_terminal_closeout_no_persistence_confirmed,
        terminal_closeout_no_live_confirmed: $source_terminal_closeout_no_live_confirmed,
        terminal_closeout_side_effects_all_false: $source_side_effects_all_false
      },
      side_effects: {
        filesystem_written: false,
        terminal_closeout_readback_recorded: false,
        terminal_closeout_readback_persisted: false,
        terminal_closeout_readback_accepted: false,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_accepted: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        live_attachment_enabled: false,
        live_blocking_hook_installed: false,
        runtime_interception_enabled: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        lease_acquired: false,
        work_started: false,
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
