#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-non-persistence-readback-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_CAPTURE_CACHE_DIR"' EXIT
fi

audit_index_report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-report.sh"
)"

jq -n \
  --argjson audit_index_report "$audit_index_report" \
  '
  def entry($id; $key; $state): {
    id: $id,
    stable_readback_key: $key,
    observed_state: $state,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail hardening readback audit index non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index",
    readback_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback.audit_index.non_persistence_readback",
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
    entry("hardening_audit_index_surface_non_persistence_readback"; "entrypoint_hardening_readback_audit_index_visible_unrecorded"; "audit_index_visible_without_record_persist_accept_or_authority"),
    entry("hardening_audit_index_entry_inventory_non_persistence_readback"; "entrypoint_hardening_readback_audit_index_entries_visible"; "nine_audit_index_entries_visible_but_not_persisted"),
    entry("hardening_audit_index_blocker_inventory_non_persistence_readback"; "entrypoint_hardening_readback_audit_index_blockers_visible"; "thirty_blockers_visible_and_still_blocking"),
    entry("hardening_audit_index_prior_chain_non_persistence_readback"; "entrypoint_hardening_readback_audit_index_priors_visible"; "thirteen_required_prior_gates_visible_but_not_persisted"),
    entry("hardening_audit_index_non_persistence_boundary_readback"; "entrypoint_hardening_readback_audit_index_non_persistence_boundary"; "audit_index_does_not_write_event_projection_scheduler_guardrail_or_runtime_state"),
    entry("hardening_audit_index_no_live_authority_readback"; "entrypoint_hardening_readback_audit_index_no_live_authority"; "audit_index_does_not_authorize_enforcement_interception_work_start_agent_model_external_or_live_cutover")
  ] as $readback_entries
  | [
    blocker("audit_index_readback_record_blocked"; "record_hardening_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_persistence_blocked"; "persist_hardening_audit_index_non_persistence_readback"),
    blocker("audit_index_readback_acceptance_blocked"; "accept_hardening_audit_index_non_persistence_readback"),
    blocker("audit_index_record_blocked"; "record_hardening_readback_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_hardening_readback_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_hardening_readback_audit_index"),
    blocker("readback_record_blocked"; "record_hardening_readback"),
    blocker("readback_persistence_blocked"; "persist_hardening_readback"),
    blocker("readback_acceptance_blocked"; "accept_hardening_readback"),
    blocker("hardening_decision_record_blocked"; "record_hardening_decision"),
    blocker("hardening_decision_persistence_blocked"; "persist_hardening_decision"),
    blocker("live_blocking_hook_install_blocked"; "install_live_blocking_hook"),
    blocker("runtime_interception_blocked"; "enable_runtime_interception"),
    blocker("scheduler_admission_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("guardrail_enforcement_blocked"; "enable_guardrail_enforcement"),
    blocker("work_graph_event_persistence_blocked"; "persist_work_graph_event"),
    blocker("projection_index_persistence_blocked"; "persist_projection_index"),
    blocker("lease_acquisition_blocked"; "acquire_lane_lease"),
    blocker("work_start_blocked"; "start_entrypoint_work"),
    blocker("agent_spawn_blocked"; "spawn_agent"),
    blocker("model_invocation_blocked"; "invoke_model"),
    blocker("external_send_blocked"; "send_external_message"),
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
  | ($audit_index_report.audit_index_recorded == false
      and $audit_index_report.audit_index_persisted == false
      and $audit_index_report.audit_index_authoritative == false
      and $audit_index_report.audit_index_accepted == false
      and $audit_index_report.hardening_readback_recorded == false
      and $audit_index_report.hardening_readback_persisted == false
      and $audit_index_report.hardening_readback_accepted == false
      and $audit_index_report.audit_index_authorizes_hardening_readback_recording == false
      and $audit_index_report.audit_index_authorizes_hardening_readback_persistence == false
      and $audit_index_report.audit_index_authorizes_hardening_decision_recording == false
      and $audit_index_report.audit_index_authorizes_hardening_decision_persistence == false
      and $audit_index_report.audit_index_authorizes_work_graph_event_persistence == false
      and $audit_index_report.audit_index_authorizes_projection_persistence == false
      and ($audit_index_report.side_effects | to_entries | all(.value == false))) as $source_audit_index_no_persistence_confirmed
  | ($audit_index_report.audit_index_preconditions_complete == true
      and $audit_index_report.ready_for_non_persistence_readback == true
      and $audit_index_report.audit_index_authorizes_live_blocking_enforcement == false
      and $audit_index_report.audit_index_authorizes_runtime_interception == false
      and $audit_index_report.audit_index_authorizes_scheduler_admission_enforcement == false
      and $audit_index_report.audit_index_authorizes_guardrail_enforcement == false
      and $audit_index_report.audit_index_authorizes_lease_acquisition == false
      and $audit_index_report.audit_index_authorizes_work_start == false
      and $audit_index_report.audit_index_authorizes_agent_spawn == false
      and $audit_index_report.audit_index_authorizes_model_invocation == false
      and $audit_index_report.audit_index_authorizes_external_send == false
      and $audit_index_report.audit_index_authorizes_replay_execution == false
      and $audit_index_report.audit_index_authorizes_replay_diff_recording == false
      and $audit_index_report.audit_index_authorizes_replay_diff_persistence == false
      and $audit_index_report.audit_index_authorizes_rollback_execution == false
      and $audit_index_report.audit_index_authorizes_idempotency_mutation == false
      and $audit_index_report.audit_index_authorizes_config_write == false
      and $audit_index_report.audit_index_authorizes_feature_flag_mutation == false
      and $audit_index_report.audit_index_authorizes_canary_traffic == false
      and $audit_index_report.audit_index_authorizes_operator_review_request == false
      and $audit_index_report.audit_index_authorizes_approval_recording == false
      and $audit_index_report.audit_index_authorizes_live_cutover == false
      and $audit_index_report.ready_for_live_execution == false
      and $source_audit_index_no_persistence_confirmed) as $source_audit_index_no_live_confirmed
  | ($audit_index_report.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate"
      and $audit_index_report.source_hardening_readback_ready == true
      and $audit_index_report.source_hardening_readback_no_live_confirmed == true
      and $audit_index_report.source_hardening_readback_no_persistence_confirmed == true
      and $audit_index_report.source_hardening_readback_ready_for_audit_index == true
      and $audit_index_report.audit_index_scope_visible_only_complete == true
      and $audit_index_report.audit_index_entries_complete == true
      and $audit_index_report.audit_index_blockers_complete == true
      and $audit_index_report.audit_index_preconditions_complete == true
      and $audit_index_report.audit_index_entry_count == 9
      and $audit_index_report.audit_index_blocker_count == 30
      and $audit_index_report.required_prior_gate_count == 13
      and $source_audit_index_no_live_confirmed) as $source_audit_index_ready
  | ($source_audit_index_ready
      and $audit_index_report.ready_for_non_persistence_readback == true) as $source_audit_index_ready_for_non_persistence_readback
  | ($readback_scope.audit_index_visible == true
      and $readback_scope.audit_index_recorded == false
      and $readback_scope.audit_index_persisted == false
      and $readback_scope.audit_index_authoritative == false
      and $readback_scope.audit_index_accepted == false
      and $readback_scope.readback_recorded == false
      and $readback_scope.readback_persisted == false
      and $readback_scope.readback_accepted == false) as $readback_scope_visible_only_complete
  | (($readback_entries | length) == 6
      and ($readback_entries | all(
        .visible == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .accepted == false
        and .authoritative == false
        and .mutation_allowed == false
      ))) as $readback_entries_visible_only_complete
  | (($readback_blockers | length) == 33
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_audit_index_ready
      and $source_audit_index_no_persistence_confirmed
      and $source_audit_index_no_live_confirmed
      and $source_audit_index_ready_for_non_persistence_readback
      and $readback_scope_visible_only_complete
      and $readback_entries_visible_only_complete
      and $readback_blockers_complete) as $non_persistence_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: $audit_index_report.gate,
      source_audit_index_entry_count: $audit_index_report.audit_index_entry_count,
      source_audit_index_blocker_count: $audit_index_report.audit_index_blocker_count,
      source_required_prior_gate_count: $audit_index_report.required_prior_gate_count,
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
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate",
      audit_index_visible: $source_audit_index_ready,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      hardening_readback_visible: $audit_index_report.hardening_readback_visible,
      hardening_readback_recorded: false,
      hardening_readback_persisted: false,
      hardening_readback_accepted: false,
      audit_index_readback_recorded: false,
      audit_index_readback_persisted: false,
      audit_index_readback_accepted: false,
      readback_scope_visible_only_complete: $readback_scope_visible_only_complete,
      readback_entries_visible_only_complete: $readback_entries_visible_only_complete,
      readback_blockers_complete: $readback_blockers_complete,
      non_persistence_readback_preconditions_complete: $non_persistence_readback_preconditions_complete,
      hardening_decision_recording_allowed: false,
      hardening_decision_persistence_allowed: false,
      live_blocking_enforcement_allowed: false,
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
      ready_for_terminal_no_enforcement_final_closeout: $non_persistence_readback_preconditions_complete,
      ready_for_live_execution: false,
      source_probes: {
        audit_index_report_gate: $audit_index_report.gate,
        audit_index_preconditions_complete: $audit_index_report.audit_index_preconditions_complete,
        audit_index_ready_for_non_persistence_readback: $audit_index_report.ready_for_non_persistence_readback,
        audit_index_no_persistence_confirmed: $source_audit_index_no_persistence_confirmed,
        audit_index_no_live_confirmed: $source_audit_index_no_live_confirmed,
        audit_index_side_effects_all_false: ($audit_index_report.side_effects | to_entries | all(.value == false))
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
        hardening_readback_recorded: false,
        hardening_readback_persisted: false,
        hardening_readback_accepted: false,
        hardening_decision_recorded: false,
        hardening_decision_persisted: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        live_blocking_hook_installed: false,
        runtime_interception_enabled: false,
        lease_acquired: false,
        work_started: false,
        config_written: false,
        feature_flag_mutated: false,
        canary_traffic_routed: false,
        operator_review_requested: false,
        approval_recorded: false,
        replay_executed: false,
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
