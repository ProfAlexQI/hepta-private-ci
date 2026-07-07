#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_CAPTURE_CACHE_DIR"' EXIT
fi

hardening_report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-report.sh"
)"

jq -n \
  --argjson hardening_report "$hardening_report" \
  '
  def rb_entry($id; $key; $target; $state): {
    id: $id,
    stable_readback_key: $key,
    readback_target: $target,
    observed_state: $state,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    mutation_allowed: false,
    ready: true
  };
  def entrypoint_rb($id; $entrypoint; $key): {
    id: $id,
    entrypoint_id: $entrypoint,
    deterministic_decision_key: $key,
    dry_run_outcome: "deny_live_allow_report_only_hardened",
    required_evidence_field_count: 9,
    required_non_live_guard_count: 5,
    readback_status: "visible_only",
    would_block_if_live: true,
    report_only_allows_current_runtime: true,
    recorded: false,
    persisted: false
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail entrypoint hardening readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening",
    readback_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_visible_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback",
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    mutation_allowed: false
  } as $readback_scope
  | [
    rb_entry("hardening_contract_inventory_readback"; "entrypoint_hardening.contract_inventory"; "hardened_entrypoint_contracts"; "four_entrypoint_contracts_visible_report_only"),
    rb_entry("hardening_check_inventory_readback"; "entrypoint_hardening.check_inventory"; "hardening_checks"; "ten_live_blocking_checks_visible_dry_run_only"),
    rb_entry("hardening_decision_inventory_readback"; "entrypoint_hardening.decision_inventory"; "hardening_decisions"; "four_deny_live_allow_report_only_decisions_visible"),
    rb_entry("hardening_blocker_inventory_readback"; "entrypoint_hardening.blocker_inventory"; "hardening_blockers"; "twenty_three_live_action_blockers_visible"),
    rb_entry("hardening_prior_chain_readback"; "entrypoint_hardening.required_priors"; "required_prior_chain"; "eleven_required_prior_gates_visible"),
    rb_entry("hardening_non_live_guard_readback"; "entrypoint_hardening.non_live_guards"; "non_live_guard_contract"; "enforcement_interception_persistence_remain_disabled"),
    rb_entry("hardening_no_live_authority_readback"; "entrypoint_hardening.no_live_authority"; "no_live_authority"; "readback_cannot_authorize_live_blocking_or_runtime_work")
  ] as $readback_entries
  | [
    entrypoint_rb("spawn_agent_hardening_readback"; "spawn_agent"; "scheduler_guardrail.spawn_agent.v1"),
    entrypoint_rb("spawn_agents_on_csv_hardening_readback"; "spawn_agents_on_csv"; "scheduler_guardrail.spawn_agents_on_csv.v1"),
    entrypoint_rb("task_board_claim_hardening_readback"; "task_board_claim"; "scheduler_guardrail.task_board_claim.v1"),
    entrypoint_rb("worker_task_run_hardening_readback"; "worker_task_run"; "scheduler_guardrail.worker_task_run.v1")
  ] as $entrypoint_readbacks
  | [
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
  | ($hardening_report.hardening_preconditions_complete == true
      and $hardening_report.ready_for_hardening_readback == true
      and $hardening_report.live_blocking_enforcement_enabled == false
      and $hardening_report.runtime_interception_enabled == false
      and $hardening_report.scheduler_admission_enforced == false
      and $hardening_report.guardrail_enforcement_enabled == false
      and $hardening_report.work_graph_event_persistence_enabled == false
      and $hardening_report.ready_for_live_execution == false
      and ($hardening_report.side_effects | to_entries | all(.value == false))) as $source_hardening_no_live_confirmed
  | ($hardening_report.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate"
      and $hardening_report.source_prior_readbacks_complete == true
      and $hardening_report.hardened_entrypoints_complete == true
      and $hardening_report.hardening_checks_complete == true
      and $hardening_report.hardening_decisions_complete == true
      and $hardening_report.hardening_blockers_complete == true
      and $hardening_report.hardening_preconditions_complete == true
      and $hardening_report.hardened_entrypoint_count == 4
      and $hardening_report.hardening_check_count == 10
      and $hardening_report.hardening_decision_count == 4
      and $hardening_report.hardening_blocker_count == 23
      and $hardening_report.required_prior_gate_count == 11
      and $source_hardening_no_live_confirmed) as $source_hardening_ready
  | ($source_hardening_ready
      and $hardening_report.ready_for_hardening_readback == true) as $source_hardening_ready_for_readback
  | ($source_hardening_ready
      and $hardening_report.hardened_entrypoints_complete == true) as $hardening_contract_visible
  | ($source_hardening_ready
      and $hardening_report.hardening_decisions_complete == true) as $hardening_decisions_visible
  | ($source_hardening_ready
      and $hardening_report.hardening_checks_complete == true) as $hardening_checks_visible
  | ($source_hardening_ready
      and $hardening_report.hardening_blockers_complete == true) as $hardening_blockers_visible
  | ($readback_scope.visible == true
      and $readback_scope.recorded == false
      and $readback_scope.persisted == false
      and $readback_scope.authoritative == false
      and $readback_scope.accepted == false
      and $readback_scope.mutation_allowed == false) as $readback_scope_visible_only_complete
  | (($readback_entries | length) == 7
      and ($readback_entries | all(
        .visible == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .accepted == false
        and .authoritative == false
        and .mutation_allowed == false
      ))) as $readback_entries_visible_only_complete
  | (($entrypoint_readbacks | length) == 4
      and ($entrypoint_readbacks | all(
        .dry_run_outcome == "deny_live_allow_report_only_hardened"
        and .required_evidence_field_count == 9
        and .required_non_live_guard_count == 5
        and .readback_status == "visible_only"
        and .would_block_if_live == true
        and .report_only_allows_current_runtime == true
        and .recorded == false
        and .persisted == false
      ))) as $entrypoint_readbacks_visible_only_complete
  | (($readback_blockers | length) == 27
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_hardening_ready
      and $source_hardening_ready_for_readback
      and $source_hardening_no_live_confirmed
      and $readback_scope_visible_only_complete
      and $readback_entries_visible_only_complete
      and $entrypoint_readbacks_visible_only_complete
      and $readback_blockers_complete) as $hardening_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_visible_only",
      source_hardening_gate: $hardening_report.gate,
      source_hardened_entrypoint_count: $hardening_report.hardened_entrypoint_count,
      source_hardening_check_count: $hardening_report.hardening_check_count,
      source_hardening_decision_count: $hardening_report.hardening_decision_count,
      source_hardening_blocker_count: $hardening_report.hardening_blocker_count,
      source_required_prior_gate_count: $hardening_report.required_prior_gate_count,
      source_hardening_ready: $source_hardening_ready,
      source_hardening_no_live_confirmed: $source_hardening_no_live_confirmed,
      source_hardening_ready_for_readback: $source_hardening_ready_for_readback,
      readback_entry_count: ($readback_entries | length),
      entrypoint_readback_count: ($entrypoint_readbacks | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      entrypoint_readbacks: $entrypoint_readbacks,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
      hardening_contract_visible: $hardening_contract_visible,
      hardening_decisions_visible: $hardening_decisions_visible,
      hardening_checks_visible: $hardening_checks_visible,
      hardening_blockers_visible: $hardening_blockers_visible,
      readback_scope_visible_only_complete: $readback_scope_visible_only_complete,
      readback_entries_visible_only_complete: $readback_entries_visible_only_complete,
      entrypoint_readbacks_visible_only_complete: $entrypoint_readbacks_visible_only_complete,
      readback_blockers_complete: $readback_blockers_complete,
      hardening_readback_preconditions_complete: $hardening_readback_preconditions_complete,
      readback_ready: $hardening_readback_preconditions_complete,
      readback_recorded: false,
      readback_persisted: false,
      readback_authoritative: false,
      readback_accepted: false,
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
      ready_for_audit_index: $hardening_readback_preconditions_complete,
      ready_for_live_execution: false,
      source_probes: {
        hardening_report_gate: $hardening_report.gate,
        hardening_preconditions_complete: $hardening_report.hardening_preconditions_complete,
        hardening_ready_for_readback: $hardening_report.ready_for_hardening_readback,
        hardening_source_prior_readbacks_complete: $hardening_report.source_prior_readbacks_complete,
        hardening_no_live_confirmed: $source_hardening_no_live_confirmed,
        hardening_side_effects_all_false: ($hardening_report.side_effects | to_entries | all(.value == false))
      },
      side_effects: {
        filesystem_written: false,
        readback_recorded: false,
        readback_persisted: false,
        readback_accepted: false,
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
