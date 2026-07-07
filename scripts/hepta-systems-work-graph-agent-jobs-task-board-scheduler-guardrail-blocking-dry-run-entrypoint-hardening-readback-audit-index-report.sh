#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-audit-index-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_CAPTURE_CACHE_DIR"' EXIT
fi

hardening_readback_report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-hardening-readback-report.sh"
)"

jq -n \
  --argjson hardening_readback_report "$hardening_readback_report" \
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
    reason: "required before scheduler/guardrail hardening readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback",
    index_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback.audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    index_accepted: false,
    live_acceptance_allowed: false
  } as $audit_index_scope
  | [
    entry("hardening_readback_scope_audit_index"; "entrypoint_hardening_readback.audit_index.scope"; "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_scope"; "hardening_readback_scope"),
    entry("hardening_readback_entry_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.readback_entry_inventory"; "hardening_contract_inventory_readback"; "hardening_readback_entries"),
    entry("hardening_entrypoint_readback_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.entrypoint_readback_inventory"; "spawn_agent_hardening_readback"; "entrypoint_readbacks"),
    entry("hardening_readback_blocker_inventory_audit_index"; "entrypoint_hardening_readback.audit_index.blocker_inventory"; "hardening_blocker_inventory_readback"; "readback_blockers"),
    entry("hardening_readback_prior_chain_audit_index"; "entrypoint_hardening_readback.audit_index.prior_chain"; "hardening_prior_chain_readback"; "required_prior_chain"),
    entry("hardening_readback_non_live_guard_audit_index"; "entrypoint_hardening_readback.audit_index.non_live_guard"; "hardening_non_live_guard_readback"; "non_live_guard_contract"),
    entry("hardening_readback_no_live_authority_audit_index"; "entrypoint_hardening_readback.audit_index.no_live_authority"; "hardening_no_live_authority_readback"; "no_live_authority"),
    entry("hardening_source_decision_trace_audit_index"; "entrypoint_hardening_readback.audit_index.source_decision_trace"; "hardening_decision_inventory_readback"; "source_decision_trace"),
    entry("hardening_live_boundary_audit_index"; "entrypoint_hardening_readback.audit_index.live_boundary"; "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"; "live_cutover_boundary")
  ] as $audit_index_entries
  | [
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
  ] as $audit_index_blockers
  | [
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
  | ($hardening_readback_report.readback_recorded == false
      and $hardening_readback_report.readback_persisted == false
      and $hardening_readback_report.readback_authoritative == false
      and $hardening_readback_report.readback_accepted == false
      and $hardening_readback_report.hardening_decision_recording_allowed == false
      and $hardening_readback_report.hardening_decision_persistence_allowed == false
      and $hardening_readback_report.work_graph_event_persistence_allowed == false
      and $hardening_readback_report.projection_persistence_allowed == false
      and ($hardening_readback_report.side_effects | to_entries | all(.value == false))) as $source_hardening_readback_no_persistence_confirmed
  | ($hardening_readback_report.hardening_readback_preconditions_complete == true
      and $hardening_readback_report.ready_for_audit_index == true
      and $hardening_readback_report.live_blocking_enforcement_allowed == false
      and $hardening_readback_report.runtime_interception_allowed == false
      and $hardening_readback_report.scheduler_admission_enforcement_allowed == false
      and $hardening_readback_report.guardrail_enforcement_allowed == false
      and $hardening_readback_report.lease_acquisition_allowed == false
      and $hardening_readback_report.work_start_allowed == false
      and $hardening_readback_report.agent_spawn_allowed == false
      and $hardening_readback_report.model_invocation_allowed == false
      and $hardening_readback_report.external_send_allowed == false
      and $hardening_readback_report.replay_execution_allowed == false
      and $hardening_readback_report.replay_diff_recording_allowed == false
      and $hardening_readback_report.replay_diff_persistence_allowed == false
      and $hardening_readback_report.rollback_execution_allowed == false
      and $hardening_readback_report.idempotency_mutation_allowed == false
      and $hardening_readback_report.config_write_allowed == false
      and $hardening_readback_report.feature_flag_mutation_allowed == false
      and $hardening_readback_report.canary_traffic_allowed == false
      and $hardening_readback_report.operator_review_request_allowed == false
      and $hardening_readback_report.approval_recording_allowed == false
      and $hardening_readback_report.live_cutover_allowed == false
      and $hardening_readback_report.ready_for_live_execution == false
      and $source_hardening_readback_no_persistence_confirmed) as $source_hardening_readback_no_live_confirmed
  | ($hardening_readback_report.gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate"
      and $hardening_readback_report.source_hardening_ready == true
      and $hardening_readback_report.source_hardening_no_live_confirmed == true
      and $hardening_readback_report.source_hardening_ready_for_readback == true
      and $hardening_readback_report.readback_scope_visible_only_complete == true
      and $hardening_readback_report.readback_entries_visible_only_complete == true
      and $hardening_readback_report.entrypoint_readbacks_visible_only_complete == true
      and $hardening_readback_report.readback_blockers_complete == true
      and $hardening_readback_report.hardening_readback_preconditions_complete == true
      and $hardening_readback_report.readback_entry_count == 7
      and $hardening_readback_report.entrypoint_readback_count == 4
      and $hardening_readback_report.readback_blocker_count == 27
      and $hardening_readback_report.required_prior_gate_count == 12
      and $source_hardening_readback_no_live_confirmed) as $source_hardening_readback_ready
  | ($source_hardening_readback_ready
      and $hardening_readback_report.ready_for_audit_index == true) as $source_hardening_readback_ready_for_audit_index
  | ($audit_index_scope.index_visible == true
      and $audit_index_scope.index_recorded == false
      and $audit_index_scope.index_persisted == false
      and $audit_index_scope.index_authoritative == false
      and $audit_index_scope.index_accepted == false
      and $audit_index_scope.live_acceptance_allowed == false) as $audit_index_scope_visible_only_complete
  | (($audit_index_entries | length) == 9
      and ($audit_index_entries | all(
        .indexed == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .authoritative == false
        and .accepted == false
        and .mutation_allowed == false
      ))) as $audit_index_entries_complete
  | (($audit_index_blockers | length) == 30
      and ($audit_index_blockers | all(
        .blocked == true
        and .required_before_acceptance == true
      ))) as $audit_index_blockers_complete
  | ($source_hardening_readback_ready
      and $source_hardening_readback_ready_for_audit_index
      and $source_hardening_readback_no_live_confirmed
      and $source_hardening_readback_no_persistence_confirmed
      and $audit_index_scope_visible_only_complete
      and $audit_index_entries_complete
      and $audit_index_blockers_complete) as $audit_index_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
      source_hardening_readback_gate: $hardening_readback_report.gate,
      source_readback_entry_count: $hardening_readback_report.readback_entry_count,
      source_entrypoint_readback_count: $hardening_readback_report.entrypoint_readback_count,
      source_readback_blocker_count: $hardening_readback_report.readback_blocker_count,
      source_required_prior_gate_count: $hardening_readback_report.required_prior_gate_count,
      source_hardening_readback_ready: $source_hardening_readback_ready,
      source_hardening_readback_no_live_confirmed: $source_hardening_readback_no_live_confirmed,
      source_hardening_readback_no_persistence_confirmed: $source_hardening_readback_no_persistence_confirmed,
      source_hardening_readback_ready_for_audit_index: $source_hardening_readback_ready_for_audit_index,
      audit_index_entry_count: ($audit_index_entries | length),
      audit_index_blocker_count: ($audit_index_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      audit_index_scope: $audit_index_scope,
      audit_index_entries: $audit_index_entries,
      audit_index_blockers: $audit_index_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
      audit_index_visible: $audit_index_scope_visible_only_complete,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      hardening_readback_visible: $source_hardening_readback_ready,
      hardening_readback_recorded: false,
      hardening_readback_persisted: false,
      hardening_readback_accepted: false,
      audit_index_scope_visible_only_complete: $audit_index_scope_visible_only_complete,
      audit_index_entries_complete: $audit_index_entries_complete,
      audit_index_blockers_complete: $audit_index_blockers_complete,
      audit_index_preconditions_complete: $audit_index_preconditions_complete,
      audit_index_authorizes_hardening_readback_recording: false,
      audit_index_authorizes_hardening_readback_persistence: false,
      audit_index_authorizes_hardening_decision_recording: false,
      audit_index_authorizes_hardening_decision_persistence: false,
      audit_index_authorizes_live_blocking_enforcement: false,
      audit_index_authorizes_runtime_interception: false,
      audit_index_authorizes_scheduler_admission_enforcement: false,
      audit_index_authorizes_guardrail_enforcement: false,
      audit_index_authorizes_work_graph_event_persistence: false,
      audit_index_authorizes_projection_persistence: false,
      audit_index_authorizes_lease_acquisition: false,
      audit_index_authorizes_work_start: false,
      audit_index_authorizes_agent_spawn: false,
      audit_index_authorizes_model_invocation: false,
      audit_index_authorizes_external_send: false,
      audit_index_authorizes_replay_execution: false,
      audit_index_authorizes_replay_diff_recording: false,
      audit_index_authorizes_replay_diff_persistence: false,
      audit_index_authorizes_rollback_execution: false,
      audit_index_authorizes_idempotency_mutation: false,
      audit_index_authorizes_config_write: false,
      audit_index_authorizes_feature_flag_mutation: false,
      audit_index_authorizes_canary_traffic: false,
      audit_index_authorizes_operator_review_request: false,
      audit_index_authorizes_approval_recording: false,
      audit_index_authorizes_live_cutover: false,
      ready_for_non_persistence_readback: $audit_index_preconditions_complete,
      ready_for_live_execution: false,
      source_probes: {
        hardening_readback_report_gate: $hardening_readback_report.gate,
        hardening_readback_preconditions_complete: $hardening_readback_report.hardening_readback_preconditions_complete,
        hardening_readback_ready_for_audit_index: $hardening_readback_report.ready_for_audit_index,
        hardening_readback_no_live_confirmed: $source_hardening_readback_no_live_confirmed,
        hardening_readback_no_persistence_confirmed: $source_hardening_readback_no_persistence_confirmed,
        hardening_readback_side_effects_all_false: ($hardening_readback_report.side_effects | to_entries | all(.value == false))
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
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
