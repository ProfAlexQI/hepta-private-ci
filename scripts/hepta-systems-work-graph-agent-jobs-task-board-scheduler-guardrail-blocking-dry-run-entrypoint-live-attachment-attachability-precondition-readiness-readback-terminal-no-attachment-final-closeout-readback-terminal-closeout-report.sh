#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-audit-index-non-persistence-readback-report.sh"
source_report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-audit-index-non-persistence-readback-report" \
    "$SOURCE_REPORT_SCRIPT"
)"

jq -n \
  --argjson source "$source_report" \
  '
  def entry($id; $key; $source_id; $category): {
    id: $id,
    stable_closeout_key: $key,
    source_readback_id: $source_id,
    closeout_category: $category,
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
    reason: "terminal no-attachment final closeout readback terminal closeout cannot authorize this action",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
    closeout_mode: "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_report_only",
    stable_closeout_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment_final_closeout.readback.terminal_closeout",
    closeout_visible: true,
    closeout_recorded: false,
    closeout_persisted: false,
    closeout_authoritative: false,
    closeout_accepted: false,
    live_acceptance_allowed: false
  } as $terminal_closeout_scope
  | [
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_branch_closed"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.branch_closed"; "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback"; "branch_state"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_source_readback"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.source_non_persistence_readback"; "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback"; "source_readback"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_inventory"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.inventory"; "terminal_no_attachment_final_closeout_readback_audit_index_entry_inventory_non_persistence_readback"; "inventory"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_blockers"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.blockers"; "terminal_no_attachment_final_closeout_readback_audit_index_blocker_inventory_non_persistence_readback"; "blockers"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_priors"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.priors"; "terminal_no_attachment_final_closeout_readback_audit_index_prior_chain_non_persistence_readback"; "prior_chain"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_non_persistence_boundary"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.non_persistence_boundary"; "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_boundary_readback"; "non_persistence"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_no_live_authority"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.no_live_authority"; "terminal_no_attachment_final_closeout_readback_audit_index_no_live_authority_readback"; "no_live_authority"),
    entry("terminal_no_attachment_final_closeout_readback_terminal_closeout_entrypoint_surface"; "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.entrypoint_surface"; "terminal_no_attachment_final_closeout_readback_audit_index_surface_non_persistence_readback"; "entrypoint_surface")
  ] as $terminal_closeout_entries
  | (
    [
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_record_blocked"; "record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout"),
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_persistence_blocked"; "persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout"),
      blocker("terminal_no_attachment_final_closeout_readback_terminal_closeout_acceptance_blocked"; "accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout")
    ]
    + ($source.readback_blockers | map(blocker(.id; .blocked_action)))
  ) as $terminal_closeout_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | ($source.side_effects | to_entries | all(.value == false)) as $source_side_effects_all_false
  | ($source.non_persistence_readback_preconditions_complete == true
      and $source.audit_index_visible == true
      and $source.audit_index_recorded == false
      and $source.audit_index_persisted == false
      and $source.audit_index_authoritative == false
      and $source.audit_index_accepted == false
      and $source.audit_index_readback_recorded == false
      and $source.audit_index_readback_persisted == false
      and $source.audit_index_readback_accepted == false
      and $source.terminal_closeout_recording_allowed == false
      and $source.terminal_closeout_persistence_allowed == false
      and $source.work_graph_event_persistence_allowed == false
      and $source.projection_persistence_allowed == false
      and $source_side_effects_all_false) as $source_non_persistence_readback_no_persistence_confirmed
  | ($source.ready_for_terminal_no_attachment_final_closeout_readback_terminal_closeout == true
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
      and $source.hardening_decision_recording_allowed == false
      and $source.hardening_decision_persistence_allowed == false
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
      and $source_non_persistence_readback_no_persistence_confirmed) as $source_non_persistence_readback_no_live_confirmed
  | ($source.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback_gate"
      and $source.source_audit_index_ready == true
      and $source.source_audit_index_no_persistence_confirmed == true
      and $source.source_audit_index_no_live_confirmed == true
      and $source.source_audit_index_ready_for_non_persistence_readback == true
      and $source.non_persistence_readback_preconditions_complete == true
      and $source.readback_entry_count == 6
      and $source.readback_blocker_count == 71
      and $source.required_prior_gate_count == 28
      and $source_non_persistence_readback_no_live_confirmed) as $source_non_persistence_readback_ready
  | ($source_non_persistence_readback_ready
      and $source.ready_for_terminal_no_attachment_final_closeout_readback_terminal_closeout == true) as $source_non_persistence_readback_ready_for_terminal_closeout
  | ($terminal_closeout_scope.closeout_visible == true
      and $terminal_closeout_scope.closeout_recorded == false
      and $terminal_closeout_scope.closeout_persisted == false
      and $terminal_closeout_scope.closeout_authoritative == false
      and $terminal_closeout_scope.closeout_accepted == false
      and $terminal_closeout_scope.live_acceptance_allowed == false) as $terminal_closeout_scope_complete
  | (($terminal_closeout_entries | length) == 8
      and ($terminal_closeout_entries | all(
        .visible == true
        and .recorded == false
        and .persisted == false
        and .authoritative == false
        and .accepted == false
        and .mutation_allowed == false
        and .ready == true
      ))) as $terminal_closeout_entries_complete
  | (($terminal_closeout_blockers | length) == 74
      and ($terminal_closeout_blockers | all(
        .blocked == true
        and .required_before_acceptance == true
      ))) as $terminal_closeout_blockers_complete
  | ($source_non_persistence_readback_ready_for_terminal_closeout
      and $terminal_closeout_scope_complete
      and $terminal_closeout_entries_complete
      and $terminal_closeout_blockers_complete) as $terminal_closeout_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_report_only",
      source_non_persistence_readback_gate: $source.gate,
      source_readback_entry_count: $source.readback_entry_count,
      source_readback_blocker_count: $source.readback_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      source_non_persistence_readback_ready: $source_non_persistence_readback_ready,
      source_non_persistence_readback_no_persistence_confirmed: $source_non_persistence_readback_no_persistence_confirmed,
      source_non_persistence_readback_no_live_confirmed: $source_non_persistence_readback_no_live_confirmed,
      source_non_persistence_readback_ready_for_terminal_closeout: $source_non_persistence_readback_ready_for_terminal_closeout,
      terminal_closeout_entry_count: ($terminal_closeout_entries | length),
      terminal_closeout_blocker_count: ($terminal_closeout_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      terminal_closeout_scope: $terminal_closeout_scope,
      terminal_closeout_entries: $terminal_closeout_entries,
      terminal_closeout_blockers: $terminal_closeout_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate",
      terminal_closeout_scope_complete: $terminal_closeout_scope_complete,
      terminal_closeout_entries_complete: $terminal_closeout_entries_complete,
      terminal_closeout_blockers_complete: $terminal_closeout_blockers_complete,
      terminal_closeout_preconditions_complete: $terminal_closeout_preconditions_complete,
      terminal_closeout_visible: true,
      terminal_closeout_recorded: false,
      terminal_closeout_persisted: false,
      terminal_closeout_authoritative: false,
      terminal_closeout_accepted: false,
      source_audit_index_visible: $source.audit_index_visible,
      source_audit_index_recorded: $source.audit_index_recorded,
      source_audit_index_persisted: $source.audit_index_persisted,
      source_audit_index_authoritative: $source.audit_index_authoritative,
      source_audit_index_accepted: $source.audit_index_accepted,
      source_readback_recorded: $source.audit_index_readback_recorded,
      source_readback_persisted: $source.audit_index_readback_persisted,
      source_readback_accepted: $source.audit_index_readback_accepted,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
      terminal_closeout_readback_recording_allowed: false,
      terminal_closeout_readback_persistence_allowed: false,
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
      ready_for_terminal_closeout_readback: $terminal_closeout_preconditions_complete,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_readbacks: {
        non_persistence_readback_report_gate: $source.gate,
        non_persistence_readback_preconditions_complete: $source.non_persistence_readback_preconditions_complete,
        non_persistence_readback_ready_for_terminal_closeout: $source.ready_for_terminal_no_attachment_final_closeout_readback_terminal_closeout,
        non_persistence_readback_no_persistence_confirmed: $source_non_persistence_readback_no_persistence_confirmed,
        non_persistence_readback_no_live_confirmed: $source_non_persistence_readback_no_live_confirmed,
        non_persistence_readback_side_effects_all_false: $source_side_effects_all_false
      },
      side_effects: {
        filesystem_written: false,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_accepted: false,
        terminal_closeout_readback_recorded: false,
        terminal_closeout_readback_persisted: false,
        terminal_closeout_readback_accepted: false,
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
