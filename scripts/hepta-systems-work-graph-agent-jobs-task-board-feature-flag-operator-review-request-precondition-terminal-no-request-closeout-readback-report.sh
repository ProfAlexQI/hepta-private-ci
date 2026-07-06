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

terminal_closeout_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback.rs
)"
terminal_closeout_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-terminal-no-request-closeout-gate.sh
)"
terminal_closeout_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout.rs
)"
terminal_closeout_unrequested_present="$(
  bool_for source_has "operator_review_requested: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout.rs
)"
terminal_closeout_ready_present="$(
  bool_for source_has "ready_for_terminal_no_request_closeout_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout.rs
)"
terminal_closeout_unpersisted_present="$(
  bool_for source_has "work_graph_persistence_allowed: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout.rs
)"

jq -n \
  --argjson terminal_closeout_readback_module_present "$terminal_closeout_readback_module_present" \
  --argjson terminal_closeout_gate_present "$terminal_closeout_gate_present" \
  --argjson terminal_closeout_points_here "$terminal_closeout_points_here" \
  --argjson terminal_closeout_unrequested_present "$terminal_closeout_unrequested_present" \
  --argjson terminal_closeout_ready_present "$terminal_closeout_ready_present" \
  --argjson terminal_closeout_unpersisted_present "$terminal_closeout_unpersisted_present" \
  '
  def entry($id; $observed): {
    id: $id,
    stable_readback_key: $id,
    observed_state: $observed,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    operator_review_requested: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "terminal no-request closeout readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout",
    readback_mode: "operator_review_request_precondition_terminal_no_request_closeout_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback",
    closeout_visible: true,
    closeout_recorded: false,
    closeout_persisted: false,
    closeout_authoritative: false,
    closeout_accepted: false,
    readback_visible: true,
    readback_recorded: false,
    readback_persisted: false,
    readback_authoritative: false,
    readback_accepted: false,
    terminal_no_request: true,
    operator_review_requested: false
  } as $readback_scope
  | [
    entry("terminal_no_request_closeout_decision_readback"; "terminal_no_request_decision_visible"),
    entry("terminal_no_request_closeout_blocker_chain_readback"; "terminal_closeout_blocker_chain_visible"),
    entry("terminal_no_request_closeout_prior_chain_readback"; "terminal_closeout_required_priors_visible"),
    entry("terminal_no_request_closeout_no_request_boundary_readback"; "operator_review_request_still_absent"),
    entry("terminal_no_request_closeout_no_live_boundary_readback"; "live_cutover_still_absent")
  ] as $readback_entries
  | [
    blocker("terminal_closeout_readback_record_blocked"; "record_terminal_no_request_closeout_readback"),
    blocker("terminal_closeout_readback_persistence_blocked"; "persist_terminal_no_request_closeout_readback"),
    blocker("terminal_closeout_record_blocked"; "record_terminal_no_request_closeout"),
    blocker("terminal_closeout_persistence_blocked"; "persist_terminal_no_request_closeout"),
    blocker("terminal_closeout_acceptance_blocked"; "accept_terminal_no_request_closeout"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("operator_review_request_record_blocked"; "record_operator_review_request"),
    blocker("operator_review_request_persistence_blocked"; "persist_operator_review_request"),
    blocker("operator_review_request_acceptance_blocked"; "accept_operator_review_request"),
    blocker("operator_packet_send_blocked"; "send_operator_packet"),
    blocker("operator_packet_acceptance_blocked"; "accept_operator_packet"),
    blocker("approval_record_blocked"; "record_operator_approval"),
    blocker("feature_flag_config_write_blocked"; "write_feature_flag_config"),
    blocker("feature_flag_enablement_blocked"; "enable_feature_flag"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"),
    blocker("scheduler_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("guardrail_enforcement_blocked"; "enable_guardrail_enforcement"),
    blocker("replay_execution_blocked"; "execute_replay"),
    blocker("rollback_execution_blocked"; "execute_rollback"),
    blocker("work_graph_projection_persistence_blocked"; "persist_work_graph_projection"),
    blocker("work_graph_event_record_blocked"; "record_work_graph_event"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $readback_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_non_request_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_v1",
      preview_mode: "operator_review_request_precondition_terminal_no_request_closeout_readback_only",
      source_terminal_closeout_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate",
      source_closeout_entry_count: 7,
      source_closeout_blocker_count: 21,
      source_required_prior_gate_count: 21,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_gate",
      terminal_closeout_visible: true,
      terminal_closeout_recorded: false,
      terminal_closeout_persisted: false,
      terminal_closeout_authoritative: false,
      terminal_closeout_accepted: false,
      readback_visible: true,
      readback_recorded: false,
      readback_persisted: false,
      readback_authoritative: false,
      readback_accepted: false,
      terminal_no_request: true,
      operator_review_request_allowed: false,
      operator_review_requested: false,
      operator_packet_send_allowed: false,
      operator_packet_acceptance_allowed: false,
      approval_recording_allowed: false,
      config_write_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      scheduler_enforcement_allowed: false,
      guardrail_enforcement_allowed: false,
      replay_execution_allowed: false,
      rollback_execution_allowed: false,
      work_graph_persistence_allowed: false,
      live_cutover_allowed: false,
      ready_for_terminal_no_request_closeout_readback_audit_index: true,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        terminal_closeout_readback_module_present: $terminal_closeout_readback_module_present,
        terminal_closeout_gate_present: $terminal_closeout_gate_present,
        terminal_closeout_points_here: $terminal_closeout_points_here,
        terminal_closeout_unrequested_present: $terminal_closeout_unrequested_present,
        terminal_closeout_ready_present: $terminal_closeout_ready_present,
        terminal_closeout_unpersisted_present: $terminal_closeout_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
        operator_review_requested: false,
        operator_review_request_recorded: false,
        operator_review_request_persisted: false,
        operator_review_request_accepted: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_accepted: false,
        terminal_closeout_readback_recorded: false,
        terminal_closeout_readback_persisted: false,
        terminal_closeout_readback_accepted: false,
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
