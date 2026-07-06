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

non_persistence_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback.rs
)"
denial_audit_index_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-gate.sh
)"
denial_audit_index_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index.rs
)"
denial_audit_index_ready_present="$(
  bool_for source_has "ready_for_non_persistence_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index.rs
)"
denial_audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index.rs
)"

jq -n \
  --argjson non_persistence_readback_module_present "$non_persistence_readback_module_present" \
  --argjson denial_audit_index_gate_present "$denial_audit_index_gate_present" \
  --argjson denial_audit_index_points_here "$denial_audit_index_points_here" \
  --argjson denial_audit_index_ready_present "$denial_audit_index_ready_present" \
  --argjson denial_audit_index_unpersisted_present "$denial_audit_index_unpersisted_present" \
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
    reason: "non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.enablement_precondition_denial_audit_index",
    readback_mode: "denial_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.enablement_precondition.denial_audit_index.non_persistence_readback",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    readback_persisted: false
  } as $readback_scope
  | [
    entry("audit_index_surface_readback"; "denial_audit_index_visible_unrecorded_unpersisted"; "audit_index_visible_without_record_persist_accept_or_authority"),
    entry("audit_index_prior_chain_readback"; "denial_audit_index_required_priors_visible"; "eleven_required_prior_gates_visible_but_not_persisted"),
    entry("audit_index_blocker_readback"; "denial_audit_index_blockers_visible"; "eleven_blockers_visible_and_still_blocking"),
    entry("audit_index_non_persistence_boundary_readback"; "denial_audit_index_non_persistence_boundary"; "audit_index_does_not_write_work_graph_projection_or_config_state"),
    entry("audit_index_no_acceptance_boundary_readback"; "denial_audit_index_no_acceptance_boundary"; "audit_index_does_not_request_operator_review_or_acceptance")
  ] as $readback_entries
  | [
    blocker("audit_index_record_blocked"; "record_denial_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_denial_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_denial_audit_index"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("approval_record_blocked"; "record_operator_approval"),
    blocker("feature_flag_config_write_blocked"; "write_feature_flag_config"),
    blocker("feature_flag_enablement_blocked"; "enable_feature_flag"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"),
    blocker("scheduler_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("replay_execution_blocked"; "execute_replay"),
    blocker("rollback_execution_blocked"; "execute_rollback"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $readback_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_v1",
      preview_mode: "denial_audit_index_non_persistence_readback_no_record_no_acceptance_no_write",
      source_denial_audit_index_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate",
      source_audit_index_entry_count: 6,
      source_audit_index_blocker_count: 11,
      source_required_prior_gate_count: 11,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      readback_persisted: false,
      operator_review_request_allowed: false,
      approval_recorded: false,
      config_write_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      scheduler_enforcement_allowed: false,
      replay_execution_allowed: false,
      rollback_execution_allowed: false,
      live_cutover_allowed: false,
      ready_for_operator_review_precondition_matrix: true,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        non_persistence_readback_module_present: $non_persistence_readback_module_present,
        denial_audit_index_gate_present: $denial_audit_index_gate_present,
        denial_audit_index_points_here: $denial_audit_index_points_here,
        denial_audit_index_ready_present: $denial_audit_index_ready_present,
        denial_audit_index_unpersisted_present: $denial_audit_index_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        readback_persisted: false,
        config_written: false,
        feature_flag_mutated: false,
        non_blocking_canary_enabled: false,
        canary_traffic_routed: false,
        live_cutover_enabled: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        config_digest_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        replay_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
