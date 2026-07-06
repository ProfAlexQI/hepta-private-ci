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

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-ai-np-final-closeout-report.sh"
SOURCE_MODULE="codex-rs/hepta-runtime/src/wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout.rs"

source_report="$("$SOURCE_REPORT_SCRIPT")"

final_closeout_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback.rs
)"
final_closeout_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-ai-np-final-closeout-gate.sh
)"
final_closeout_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_gate" \
    "$SOURCE_MODULE"
)"
final_closeout_ready_present="$(
  bool_for source_has "ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback:" "$SOURCE_MODULE"
)"
final_closeout_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" "$SOURCE_MODULE"
)"
final_closeout_unpersisted_present="$(
  bool_for source_has "final_closeout_persisted: false" "$SOURCE_MODULE"
)"

jq -n \
  --argjson source "$source_report" \
  --argjson final_closeout_readback_module_present "$final_closeout_readback_module_present" \
  --argjson final_closeout_gate_present "$final_closeout_gate_present" \
  --argjson final_closeout_points_here "$final_closeout_points_here" \
  --argjson final_closeout_ready_present "$final_closeout_ready_present" \
  --argjson final_closeout_no_live_present "$final_closeout_no_live_present" \
  --argjson final_closeout_unpersisted_present "$final_closeout_unpersisted_present" \
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
    reason: "final closeout readback cannot authorize this action"
  };
  [
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_surface"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.surface"; "final_closeout_visible_unrecorded_unpersisted_unaccepted"),
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_entries"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.entries"; "eight_final_closeout_entries_visible_and_closed"),
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_blockers"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.blockers"; "one_hundred_thirty_four_final_closeout_blockers_visible_and_still_blocking"),
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_priors"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.priors"; "forty_nine_required_prior_gates_visible_but_not_persisted"),
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_boundary"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.boundary"; "final_closeout_readback_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_no_live_authority"; "final_closeout_readback.audit_index.non_persistence.final_closeout.readback.no_live_authority"; "final_closeout_readback_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover")
  ] as $readback_entries
  | (
    [
      blocker("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_record_blocked"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback"),
      blocker("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_persistence_blocked"; "persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback"),
      blocker("final_closeout_readback_audit_index_non_persistence_final_closeout_readback_acceptance_blocked"; "accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback")
    ]
    + ($source.final_closeout_blockers | map(blocker(.id; .blocked_action)))
  ) as $readback_blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_v1",
      source_final_closeout_gate: $source.gate,
      source_final_closeout_entry_count: $source.final_closeout_entry_count,
      source_final_closeout_blocker_count: $source.final_closeout_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate",
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
      source_audit_index_visible: $source.source_audit_index_visible,
      source_audit_index_recorded: $source.source_audit_index_recorded,
      source_audit_index_persisted: $source.source_audit_index_persisted,
      source_audit_index_authoritative: $source.source_audit_index_authoritative,
      source_audit_index_accepted: $source.source_audit_index_accepted,
      source_audit_index_readback_recorded: $source.source_audit_index_readback_recorded,
      source_audit_index_readback_persisted: $source.source_audit_index_readback_persisted,
      source_audit_index_readback_accepted: $source.source_audit_index_readback_accepted,
      source_prior_final_closeout_readback_visible: $source.source_final_closeout_readback_visible,
      source_prior_final_closeout_readback_recorded: $source.source_final_closeout_readback_recorded,
      source_prior_final_closeout_readback_persisted: $source.source_final_closeout_readback_persisted,
      source_prior_final_closeout_readback_authoritative: $source.source_final_closeout_readback_authoritative,
      source_prior_final_closeout_readback_accepted: $source.source_final_closeout_readback_accepted,
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
      ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index: true,
      ready_for_live_attachment: false,
      ready_for_live_execution: false,
      source_probes: {
        final_closeout_readback_module_present: $final_closeout_readback_module_present,
        final_closeout_gate_present: $final_closeout_gate_present,
        final_closeout_points_here: $final_closeout_points_here,
        final_closeout_ready_present: $final_closeout_ready_present,
        final_closeout_no_live_present: $final_closeout_no_live_present,
        final_closeout_unpersisted_present: $final_closeout_unpersisted_present
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
        prior_final_closeout_readback_recorded: false,
        prior_final_closeout_readback_persisted: false,
        prior_final_closeout_readback_accepted: false,
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
