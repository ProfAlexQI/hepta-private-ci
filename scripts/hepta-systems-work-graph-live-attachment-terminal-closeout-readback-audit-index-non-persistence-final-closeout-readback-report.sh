#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-report.sh"
source_report="$(
  capture_json_report \
    "hepta-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-report" \
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
    reason: "terminal closeout readback audit index non-persistence final closeout readback cannot authorize this action"
  };
  ($source.side_effects | to_entries | all(.value == false)) as $source_side_effects_all_false
  | ($source.final_closeout_preconditions_complete == true
      and $source.final_closeout_visible == true
      and $source.final_closeout_recorded == false
      and $source.final_closeout_persisted == false
      and $source.final_closeout_authoritative == false
      and $source.final_closeout_accepted == false
      and $source.source_audit_index_visible == true
      and $source.source_audit_index_recorded == false
      and $source.source_audit_index_persisted == false
      and $source.source_audit_index_authoritative == false
      and $source.source_audit_index_accepted == false
      and $source.source_readback_recorded == false
      and $source.source_readback_persisted == false
      and $source.source_readback_accepted == false
      and $source.work_graph_event_persistence_allowed == false
      and $source.projection_persistence_allowed == false
      and $source_side_effects_all_false) as $source_final_closeout_no_persistence_confirmed
  | ($source.ready_for_terminal_closeout_readback_audit_index_final_closeout_readback == true
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
      and $source_final_closeout_no_persistence_confirmed) as $source_final_closeout_no_live_confirmed
  | ($source.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_gate"
      and $source.source_non_persistence_readback_ready == true
      and $source.source_non_persistence_readback_no_persistence_confirmed == true
      and $source.source_non_persistence_readback_no_live_confirmed == true
      and $source.source_non_persistence_readback_ready_for_final_closeout == true
      and $source.final_closeout_scope_complete == true
      and $source.final_closeout_entries_complete == true
      and $source.final_closeout_blockers_complete == true
      and $source.final_closeout_preconditions_complete == true
      and $source.final_closeout_entry_count == 8
      and $source.final_closeout_blocker_count == 86
      and $source.required_prior_gate_count == 33
      and $source_final_closeout_no_live_confirmed) as $source_final_closeout_ready
  | ($source_final_closeout_ready
      and $source.ready_for_terminal_closeout_readback_audit_index_final_closeout_readback == true) as $source_final_closeout_ready_for_readback
  | {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout",
    readback_mode: "live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment_final_closeout.readback.terminal_closeout.readback.audit_index.non_persistence.final_closeout.readback",
    source_final_closeout_visible: true,
    source_final_closeout_recorded: false,
    source_final_closeout_persisted: false,
    source_final_closeout_accepted: false,
    readback_recorded: false,
    readback_persisted: false,
    readback_accepted: false
  } as $readback_scope
  | [
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_surface_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.surface"; "final_closeout_visible_unrecorded_unpersisted_unaccepted"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_entry_inventory_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.entries"; "eight_final_closeout_entries_visible_and_closed"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_blocker_inventory_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.blockers"; "eighty_six_final_closeout_blockers_visible_and_still_blocking"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_prior_chain_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.priors"; "thirty_three_required_prior_gates_visible_but_not_persisted"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_boundary_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.boundary"; "final_closeout_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_no_live_authority_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.no_live_authority"; "final_closeout_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover")
  ] as $readback_entries
  | (
    [
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_record_blocked"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_persistence_blocked"; "persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_acceptance_blocked"; "accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback")
    ]
    + ($source.final_closeout_blockers | map(blocker(.id; .blocked_action)))
  ) as $readback_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | ($readback_scope.source_final_closeout_visible == true
      and $readback_scope.source_final_closeout_recorded == false
      and $readback_scope.source_final_closeout_persisted == false
      and $readback_scope.source_final_closeout_accepted == false
      and $readback_scope.readback_recorded == false
      and $readback_scope.readback_persisted == false
      and $readback_scope.readback_accepted == false) as $readback_scope_complete
  | (($readback_entries | length) == 6
      and ($readback_entries | all(
        .visible == true
        and .recorded == false
        and .persisted == false
        and .authoritative == false
        and .accepted == false
        and .mutation_allowed == false
        and .ready == true
      ))) as $readback_entries_complete
  | (($readback_blockers | length) == 89
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_final_closeout_ready_for_readback
      and $readback_scope_complete
      and $readback_entries_complete
      and $readback_blockers_complete) as $final_closeout_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_only",
      source_final_closeout_gate: $source.gate,
      source_final_closeout_entry_count: $source.final_closeout_entry_count,
      source_final_closeout_blocker_count: $source.final_closeout_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      source_final_closeout_ready: $source_final_closeout_ready,
      source_final_closeout_no_persistence_confirmed: $source_final_closeout_no_persistence_confirmed,
      source_final_closeout_no_live_confirmed: $source_final_closeout_no_live_confirmed,
      source_final_closeout_ready_for_readback: $source_final_closeout_ready_for_readback,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate",
      readback_scope_complete: $readback_scope_complete,
      readback_entries_complete: $readback_entries_complete,
      readback_blockers_complete: $readback_blockers_complete,
      final_closeout_readback_preconditions_complete: $final_closeout_readback_preconditions_complete,
      source_final_closeout_visible: $source.final_closeout_visible,
      source_final_closeout_recorded: $source.final_closeout_recorded,
      source_final_closeout_persisted: $source.final_closeout_persisted,
      source_final_closeout_authoritative: $source.final_closeout_authoritative,
      source_final_closeout_accepted: $source.final_closeout_accepted,
      final_closeout_readback_visible: true,
      final_closeout_readback_recorded: false,
      final_closeout_readback_persisted: false,
      final_closeout_readback_authoritative: false,
      final_closeout_readback_accepted: false,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
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
      ready_for_terminal_closeout_readback_audit_index_final_closeout_readback_audit_index: $final_closeout_readback_preconditions_complete,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_readbacks: {
        final_closeout_report_gate: $source.gate,
        final_closeout_preconditions_complete: $source.final_closeout_preconditions_complete,
        final_closeout_ready_for_readback: $source.ready_for_terminal_closeout_readback_audit_index_final_closeout_readback,
        final_closeout_no_persistence_confirmed: $source_final_closeout_no_persistence_confirmed,
        final_closeout_no_live_confirmed: $source_final_closeout_no_live_confirmed,
        final_closeout_side_effects_all_false: $source_side_effects_all_false
      },
      side_effects: {
        filesystem_written: false,
        final_closeout_readback_recorded: false,
        final_closeout_readback_persisted: false,
        final_closeout_readback_accepted: false,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
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
