#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback-report.sh"

source_report="$(
  capture_json_report \
    "hepta-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback-report" \
    "$SOURCE_REPORT_SCRIPT"
)"

jq -n \
  --argjson source "$source_report" \
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
    reason: "required before final closeout readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  ($source.side_effects | to_entries | all(.value == false)) as $source_side_effects_all_false
  | ($source.final_closeout_readback_preconditions_complete == true
      and $source.final_closeout_readback_visible == true
      and $source.final_closeout_readback_recorded == false
      and $source.final_closeout_readback_persisted == false
      and $source.final_closeout_readback_authoritative == false
      and $source.final_closeout_readback_accepted == false
      and $source.source_final_closeout_visible == true
      and $source.source_final_closeout_recorded == false
      and $source.source_final_closeout_persisted == false
      and $source.source_final_closeout_authoritative == false
      and $source.source_final_closeout_accepted == false
      and $source.source_final_closeout_no_persistence_confirmed == true
      and $source.source_readbacks.final_closeout_no_persistence_confirmed == true
      and $source.source_readbacks.final_closeout_side_effects_all_false == true
      and $source.work_graph_event_persistence_allowed == false
      and $source.projection_persistence_allowed == false
      and $source_side_effects_all_false) as $source_final_closeout_readback_no_persistence_confirmed
  | ($source.ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index == true
      and $source.source_final_closeout_no_live_confirmed == true
      and $source.source_readbacks.final_closeout_no_live_confirmed == true
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
      and $source_final_closeout_readback_no_persistence_confirmed) as $source_final_closeout_readback_no_live_confirmed
  | ($source.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_gate"
      and $source.source_final_closeout_ready == true
      and $source.source_final_closeout_ready_for_readback == true
      and $source.final_closeout_readback_preconditions_complete == true
      and $source.readback_entries_complete == true
      and $source.readback_blockers_complete == true
      and $source.readback_entry_count == 6
      and $source.readback_blocker_count == 113
      and $source.required_prior_gate_count == 42
      and $source_final_closeout_readback_no_live_confirmed) as $source_final_closeout_readback_ready
  | ($source_final_closeout_readback_ready
      and $source.ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index == true) as $source_final_closeout_readback_ready_for_audit_index
  | [
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_scope_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.scope"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_surface_readback"; "final_closeout_readback_scope"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_entries_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.entries"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_entry_inventory_readback"; "readback_entries"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_source_summary_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.source_summary"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_surface_readback"; "source_final_closeout_summary"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_blocker_inventory_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.blockers"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_blocker_inventory_readback"; "blocker_inventory"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_prior_chain_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.prior_chain"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_prior_chain_readback"; "prior_chain"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_non_persistence_boundary_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence_boundary"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_boundary_readback"; "non_persistence_boundary"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_no_live_authority_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.no_live_authority"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_no_live_authority_readback"; "no_live_authority"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_branch_state_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.branch_state"; "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_surface_readback"; "terminal_branch_state"),
    entry("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_trace_evidence_audit_index"; "terminal_closeout_readback_audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.non_persistence.final_closeout.readback.audit_index.trace_evidence"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback"; "trace_evidence")
  ] as $entries
  | (
    [
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_record_blocked"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_persistence_blocked"; "persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index"),
      blocker("terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_acceptance_blocked"; "accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index")
    ]
    + ($source.readback_blockers | map(blocker(.id; .blocked_action)))
  ) as $blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_priors
  | (($entries | length) == 9
      and ($entries | all(
        .indexed == true
        and .recorded == false
        and .persisted == false
        and .authoritative == false
        and .accepted == false
        and .mutation_allowed == false
        and .ready == true
      ))) as $audit_index_entries_complete
  | (($blockers | length) == 116
      and ($blockers | all(.blocked == true and .required_before_acceptance == true))) as $audit_index_blockers_complete
  | ($source_final_closeout_readback_ready_for_audit_index
      and $audit_index_entries_complete
      and $audit_index_blockers_complete) as $audit_index_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_report_only",
      source_final_closeout_readback_gate: $source.gate,
      source_readback_entry_count: $source.readback_entry_count,
      source_readback_blocker_count: $source.readback_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      source_final_closeout_readback_ready: $source_final_closeout_readback_ready,
      source_final_closeout_readback_no_persistence_confirmed: $source_final_closeout_readback_no_persistence_confirmed,
      source_final_closeout_readback_no_live_confirmed: $source_final_closeout_readback_no_live_confirmed,
      source_final_closeout_readback_ready_for_audit_index: $source_final_closeout_readback_ready_for_audit_index,
      audit_index_entry_count: ($entries | length),
      audit_index_blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_priors | length),
      audit_index_entries_complete: $audit_index_entries_complete,
      audit_index_blockers_complete: $audit_index_blockers_complete,
      audit_index_preconditions_complete: $audit_index_preconditions_complete,
      audit_index_entries: $entries,
      audit_index_blockers: $blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      source_final_closeout_readback_visible: $source.final_closeout_readback_visible,
      source_final_closeout_readback_recorded: $source.final_closeout_readback_recorded,
      source_final_closeout_readback_persisted: $source.final_closeout_readback_persisted,
      source_final_closeout_readback_authoritative: $source.final_closeout_readback_authoritative,
      source_final_closeout_readback_accepted: $source.final_closeout_readback_accepted,
      source_final_closeout_visible: $source.source_final_closeout_visible,
      source_final_closeout_recorded: $source.source_final_closeout_recorded,
      source_final_closeout_persisted: $source.source_final_closeout_persisted,
      source_final_closeout_authoritative: $source.source_final_closeout_authoritative,
      source_final_closeout_accepted: $source.source_final_closeout_accepted,
      source_prior_audit_index_visible: $source.source_audit_index_visible,
      source_prior_audit_index_recorded: $source.source_audit_index_recorded,
      source_prior_audit_index_persisted: $source.source_audit_index_persisted,
      source_prior_audit_index_authoritative: $source.source_audit_index_authoritative,
      source_prior_audit_index_accepted: $source.source_audit_index_accepted,
      source_prior_audit_index_readback_recorded: $source.source_audit_index_readback_recorded,
      source_prior_audit_index_readback_persisted: $source.source_audit_index_readback_persisted,
      source_prior_audit_index_readback_accepted: $source.source_audit_index_readback_accepted,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
      audit_index_authorizes_final_closeout_readback_recording: false,
      audit_index_authorizes_final_closeout_readback_persistence: false,
      audit_index_authorizes_final_closeout_recording: false,
      audit_index_authorizes_final_closeout_persistence: false,
      audit_index_authorizes_prior_audit_index_recording: false,
      audit_index_authorizes_prior_audit_index_persistence: false,
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
      audit_index_authorizes_readback_replay_or_rollback: false,
      audit_index_authorizes_config_flag_or_traffic: false,
      audit_index_authorizes_operator_approval_or_live_cutover: false,
      ready_for_non_persistence_readback: $audit_index_preconditions_complete,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_readbacks: {
        final_closeout_readback_report_gate: $source.gate,
        final_closeout_readback_preconditions_complete: $source.final_closeout_readback_preconditions_complete,
        final_closeout_readback_ready_for_audit_index: $source.ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index,
        final_closeout_readback_no_persistence_confirmed: $source_final_closeout_readback_no_persistence_confirmed,
        final_closeout_readback_no_live_confirmed: $source_final_closeout_readback_no_live_confirmed,
        final_closeout_readback_side_effects_all_false: $source_side_effects_all_false
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        final_closeout_readback_recorded: false,
        final_closeout_readback_persisted: false,
        final_closeout_readback_accepted: false,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
        prior_audit_index_recorded: false,
        prior_audit_index_persisted: false,
        prior_audit_index_accepted: false,
        prior_audit_index_readback_recorded: false,
        prior_audit_index_readback_persisted: false,
        prior_audit_index_readback_accepted: false,
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
