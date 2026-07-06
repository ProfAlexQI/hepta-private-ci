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

final_closeout_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout.rs
)"
non_persistence_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-terminal-no-request-closeout-readback-audit-index-non-persistence-readback-gate.sh
)"
non_persistence_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_unrequested_present="$(
  bool_for source_has "operator_review_requested: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_no_live_present="$(
  bool_for source_has "ready_for_live_cutover: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_unpersisted_present="$(
  bool_for source_has "readback_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback.rs
)"

jq -n \
  --argjson final_closeout_module_present "$final_closeout_module_present" \
  --argjson non_persistence_readback_gate_present "$non_persistence_readback_gate_present" \
  --argjson non_persistence_readback_points_here "$non_persistence_readback_points_here" \
  --argjson non_persistence_readback_unrequested_present "$non_persistence_readback_unrequested_present" \
  --argjson non_persistence_readback_no_live_present "$non_persistence_readback_no_live_present" \
  --argjson non_persistence_readback_unpersisted_present "$non_persistence_readback_unpersisted_present" \
  '
  def entry($id; $key; $source; $category): {
    id: $id,
    stable_closeout_key: $key,
    source_readback_id: $source,
    closeout_category: $category,
    visible: true,
    recorded: false,
    persisted: false,
    accepted: false,
    authoritative: false,
    operator_review_requested: false,
    mutation_allowed: false,
    closed: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "terminal no-request final closeout cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback_audit_index_non_persistence_readback",
    closeout_mode: "operator_review_request_precondition_terminal_no_request_final_closeout_report_only",
    stable_closeout_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request.final_closeout",
    visible: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    terminal: true,
    operator_review_requested: false,
    mutation_allowed: false
  } as $final_closeout_scope
  | [
    entry("terminal_no_request_branch_final_closeout"; "terminal_no_request.final_closeout.branch_closed"; "terminal_no_request_closeout_readback_audit_index_non_persistence_readback"; "terminal_no_request_branch"),
    entry("terminal_no_request_prior_chain_final_closeout"; "terminal_no_request.final_closeout.required_priors_closed"; "terminal_no_request_closeout_readback_audit_index_prior_chain_readback"; "required_prior_chain"),
    entry("terminal_no_request_blocker_chain_final_closeout"; "terminal_no_request.final_closeout.blockers_closed_visible_only"; "terminal_no_request_closeout_readback_audit_index_blocker_readback"; "blocker_chain"),
    entry("terminal_no_request_operator_review_boundary_final_closeout"; "terminal_no_request.final_closeout.operator_review_request_boundary"; "terminal_no_request_closeout_readback_audit_index_no_request_boundary_readback"; "operator_review_request_boundary"),
    entry("terminal_no_request_operator_packet_boundary_final_closeout"; "terminal_no_request.final_closeout.operator_packet_boundary"; "operator_packet_send_blocked"; "operator_packet_boundary"),
    entry("terminal_no_request_approval_boundary_final_closeout"; "terminal_no_request.final_closeout.approval_boundary"; "approval_record_blocked"; "approval_boundary"),
    entry("terminal_no_request_config_flag_traffic_boundary_final_closeout"; "terminal_no_request.final_closeout.config_flag_traffic_boundary"; "feature_flag_config_write_blocked"; "config_flag_traffic_boundary"),
    entry("terminal_no_request_live_boundary_final_closeout"; "terminal_no_request.final_closeout.live_boundary"; "live_cutover_blocked"; "live_cutover_boundary")
  ] as $final_closeout_entries
  | [
    blocker("final_closeout_record_blocked"; "record_terminal_no_request_final_closeout"),
    blocker("final_closeout_persistence_blocked"; "persist_terminal_no_request_final_closeout"),
    blocker("final_closeout_acceptance_blocked"; "accept_terminal_no_request_final_closeout"),
    blocker("audit_index_readback_persistence_blocked"; "persist_terminal_no_request_closeout_readback_audit_index_readback"),
    blocker("audit_index_record_blocked"; "record_terminal_no_request_closeout_readback_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_terminal_no_request_closeout_readback_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_terminal_no_request_closeout_readback_audit_index"),
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
  ] as $final_closeout_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate",
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_v1",
      preview_mode: "operator_review_request_precondition_terminal_no_request_final_closeout_report_only",
      source_non_persistence_readback_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback_gate",
      source_readback_entry_count: 5,
      source_readback_blocker_count: 26,
      source_required_prior_gate_count: 24,
      final_closeout_entry_count: ($final_closeout_entries | length),
      final_closeout_blocker_count: ($final_closeout_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      final_closeout_scope: $final_closeout_scope,
      final_closeout_entries: $final_closeout_entries,
      final_closeout_blockers: $final_closeout_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
      terminal_no_request_branch_closed: true,
      final_closeout_visible: true,
      final_closeout_recorded: false,
      final_closeout_persisted: false,
      final_closeout_authoritative: false,
      final_closeout_accepted: false,
      source_audit_index_visible: true,
      source_audit_index_persisted: false,
      source_readback_persisted: false,
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
      ready_for_scheduler_guardrail_blocking_dry_run_entrypoint: true,
      ready_for_work_graph_shadow_event_store_readback: true,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        final_closeout_module_present: $final_closeout_module_present,
        non_persistence_readback_gate_present: $non_persistence_readback_gate_present,
        non_persistence_readback_points_here: $non_persistence_readback_points_here,
        non_persistence_readback_unrequested_present: $non_persistence_readback_unrequested_present,
        non_persistence_readback_no_live_present: $non_persistence_readback_no_live_present,
        non_persistence_readback_unpersisted_present: $non_persistence_readback_unpersisted_present
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
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_accepted: false,
        terminal_closeout_readback_recorded: false,
        terminal_closeout_readback_persisted: false,
        terminal_closeout_readback_accepted: false,
        non_persistence_readback_recorded: false,
        non_persistence_readback_persisted: false,
        non_persistence_readback_accepted: false,
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
    }
  '
