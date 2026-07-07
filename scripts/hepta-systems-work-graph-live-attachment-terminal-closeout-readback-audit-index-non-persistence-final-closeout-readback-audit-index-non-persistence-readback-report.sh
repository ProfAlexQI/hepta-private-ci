#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-report.sh"

source_report="$(
  capture_json_report \
    "hepta-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-report" \
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
    reason: "final closeout readback audit index non-persistence readback cannot authorize this action"
  };
  ($source.side_effects | to_entries | all(.value == false)) as $source_side_effects_all_false
  | ($source.audit_index_preconditions_complete == true
      and $source.audit_index_visible == true
      and $source.audit_index_recorded == false
      and $source.audit_index_persisted == false
      and $source.audit_index_authoritative == false
      and $source.audit_index_accepted == false
      and $source.source_final_closeout_readback_visible == true
      and $source.source_final_closeout_readback_recorded == false
      and $source.source_final_closeout_readback_persisted == false
      and $source.source_final_closeout_readback_authoritative == false
      and $source.source_final_closeout_readback_accepted == false
      and $source.source_final_closeout_visible == true
      and $source.source_final_closeout_recorded == false
      and $source.source_final_closeout_persisted == false
      and $source.source_final_closeout_authoritative == false
      and $source.source_final_closeout_accepted == false
      and $source.audit_index_authorizes_final_closeout_readback_recording == false
      and $source.audit_index_authorizes_final_closeout_readback_persistence == false
      and $source.audit_index_authorizes_final_closeout_recording == false
      and $source.audit_index_authorizes_final_closeout_persistence == false
      and $source.audit_index_authorizes_work_graph_persistence == false
      and $source.audit_index_authorizes_projection_persistence == false
      and $source_side_effects_all_false) as $source_audit_index_no_persistence_confirmed
  | ($source.ready_for_non_persistence_readback == true
      and $source.audit_index_authorizes_live_attachment == false
      and $source.audit_index_authorizes_live_blocking_hook == false
      and $source.audit_index_authorizes_runtime_interception == false
      and $source.audit_index_authorizes_scheduler_admission_enforcement == false
      and $source.audit_index_authorizes_guardrail_enforcement == false
      and $source.audit_index_authorizes_lease_or_work_start == false
      and $source.audit_index_authorizes_agent_model_or_external_send == false
      and $source.audit_index_authorizes_live_task_result == false
      and $source.audit_index_authorizes_readback_replay_or_rollback == false
      and $source.audit_index_authorizes_config_flag_or_traffic == false
      and $source.audit_index_authorizes_operator_approval_or_live_cutover == false
      and $source.ready_for_live_attachment == false
      and $source.ready_for_live_execution == false
      and $source_audit_index_no_persistence_confirmed) as $source_audit_index_no_live_confirmed
  | ($source.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate"
      and $source.source_final_closeout_readback_ready == true
      and $source.source_final_closeout_readback_no_persistence_confirmed == true
      and $source.source_final_closeout_readback_no_live_confirmed == true
      and $source.source_final_closeout_readback_ready_for_audit_index == true
      and $source.audit_index_scope_complete == true
      and $source.audit_index_entries_complete == true
      and $source.audit_index_blockers_complete == true
      and $source.audit_index_preconditions_complete == true
      and $source.audit_index_entry_count == 9
      and $source.audit_index_blocker_count == 92
      and $source.required_prior_gate_count == 35
      and $source_audit_index_no_live_confirmed) as $source_audit_index_ready
  | ($source_audit_index_ready
      and $source.ready_for_non_persistence_readback == true) as $source_audit_index_ready_for_non_persistence_readback
  | {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index",
    readback_mode: "live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment_final_closeout.readback.terminal_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback",
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
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_surface_non_persistence_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.surface"; "audit_index_visible_without_record_persist_accept_or_authority"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_entry_inventory_non_persistence_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.entries"; "nine_audit_index_entries_visible_but_not_persisted"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_blocker_inventory_non_persistence_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.blockers"; "ninety_two_blockers_visible_and_still_blocking"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_prior_chain_non_persistence_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.priors"; "thirty_five_required_prior_gates_visible_but_not_persisted"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_boundary_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.boundary"; "audit_index_readback_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_no_live_authority_readback"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_readback.no_live_authority"; "audit_index_readback_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover")
  ] as $readback_entries
  | (
    [
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_record_blocked"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_persistence_blocked"; "persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_acceptance_blocked"; "accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback")
    ]
    + ($source.audit_index_blockers | map(blocker(.id; .blocked_action)))
  ) as $readback_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | ($readback_scope.audit_index_visible == true
      and $readback_scope.audit_index_recorded == false
      and $readback_scope.audit_index_persisted == false
      and $readback_scope.audit_index_authoritative == false
      and $readback_scope.audit_index_accepted == false
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
  | (($readback_blockers | length) == 95
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_audit_index_ready_for_non_persistence_readback
      and $readback_scope_complete
      and $readback_entries_complete
      and $readback_blockers_complete) as $non_persistence_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: $source.gate,
      source_audit_index_entry_count: $source.audit_index_entry_count,
      source_audit_index_blocker_count: $source.audit_index_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      source_audit_index_ready: $source_audit_index_ready,
      source_audit_index_no_persistence_confirmed: $source_audit_index_no_persistence_confirmed,
      source_audit_index_no_live_confirmed: $source_audit_index_no_live_confirmed,
      source_audit_index_ready_for_non_persistence_readback: $source_audit_index_ready_for_non_persistence_readback,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_gate",
      readback_scope_complete: $readback_scope_complete,
      readback_entries_complete: $readback_entries_complete,
      readback_blockers_complete: $readback_blockers_complete,
      non_persistence_readback_preconditions_complete: $non_persistence_readback_preconditions_complete,
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      source_final_closeout_readback_visible: $source.source_final_closeout_readback_visible,
      source_final_closeout_readback_recorded: $source.source_final_closeout_readback_recorded,
      source_final_closeout_readback_persisted: $source.source_final_closeout_readback_persisted,
      source_final_closeout_readback_authoritative: $source.source_final_closeout_readback_authoritative,
      source_final_closeout_readback_accepted: $source.source_final_closeout_readback_accepted,
      source_final_closeout_visible: $source.source_final_closeout_visible,
      source_final_closeout_recorded: $source.source_final_closeout_recorded,
      source_final_closeout_persisted: $source.source_final_closeout_persisted,
      source_final_closeout_authoritative: $source.source_final_closeout_authoritative,
      source_final_closeout_accepted: $source.source_final_closeout_accepted,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
      audit_index_readback_recorded: false,
      audit_index_readback_persisted: false,
      audit_index_readback_accepted: false,
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
      ready_for_terminal_closeout_readback_audit_index_final_closeout: $non_persistence_readback_preconditions_complete,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_readbacks: {
        audit_index_report_gate: $source.gate,
        audit_index_preconditions_complete: $source.audit_index_preconditions_complete,
        audit_index_ready_for_non_persistence_readback: $source.ready_for_non_persistence_readback,
        audit_index_no_persistence_confirmed: $source_audit_index_no_persistence_confirmed,
        audit_index_no_live_confirmed: $source_audit_index_no_live_confirmed,
        audit_index_side_effects_all_false: $source_side_effects_all_false
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
        final_closeout_readback_recorded: false,
        final_closeout_readback_persisted: false,
        final_closeout_readback_accepted: false,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
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
