#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_OPERATOR_REVIEW_REQUEST_TERMINAL_CLOSEOUT_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-operator-review-request-terminal-closeout-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_OPERATOR_REVIEW_REQUEST_TERMINAL_CLOSEOUT_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_OPERATOR_REVIEW_REQUEST_TERMINAL_CLOSEOUT_CAPTURE_CACHE_DIR"' EXIT
fi

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

terminal_closeout_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout.rs
)"
non_persistence_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-non-persistence-readback-gate.sh
)"
non_persistence_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_unrequested_present="$(
  bool_for source_has "operator_review_requested: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_ready_present="$(
  bool_for source_has "ready_for_terminal_no_request_closeout" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback.rs
)"
non_persistence_readback_unpersisted_present="$(
  bool_for source_has "work_graph_persistence_allowed: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback.rs
)"

non_persistence_readback="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-non-persistence-readback-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-non-persistence-readback-report.sh"
)"

jq -n \
  --argjson terminal_closeout_module_present "$terminal_closeout_module_present" \
  --argjson non_persistence_readback_gate_present "$non_persistence_readback_gate_present" \
  --argjson non_persistence_readback_points_here "$non_persistence_readback_points_here" \
  --argjson non_persistence_readback_unrequested_present "$non_persistence_readback_unrequested_present" \
  --argjson non_persistence_readback_ready_present "$non_persistence_readback_ready_present" \
  --argjson non_persistence_readback_unpersisted_present "$non_persistence_readback_unpersisted_present" \
  --argjson non_persistence_readback "$non_persistence_readback" \
  '
  def entry($id; $category): {
    id: $id,
    stable_closeout_key: $id,
    closeout_category: $category,
    terminal: true,
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
    reason: "terminal no-request closeout cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_audit_index_non_persistence_readback",
    closeout_mode: "operator_review_request_precondition_terminal_no_request_closeout_report_only",
    stable_closeout_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout",
    closeout_visible: true,
    closeout_recorded: false,
    closeout_persisted: false,
    closeout_authoritative: false,
    closeout_accepted: false,
    terminal_no_request: true,
    operator_review_requested: false
  } as $closeout_scope
  | [
    entry("terminal_no_request_decision_closeout"; "no_request_decision"),
    entry("terminal_denial_readback_chain_closeout"; "denial_readback_chain"),
    entry("terminal_audit_index_chain_closeout"; "audit_index_chain"),
    entry("terminal_no_operator_packet_closeout"; "operator_packet_boundary"),
    entry("terminal_no_approval_config_flag_traffic_closeout"; "approval_config_flag_traffic_boundary"),
    entry("terminal_no_persistence_replay_rollback_closeout"; "persistence_replay_rollback_boundary"),
    entry("terminal_no_live_cutover_closeout"; "live_cutover_boundary")
  ] as $closeout_entries
  | [
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
  ] as $closeout_blockers
  | [
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
  | ($non_persistence_readback.audit_index_visible == true
      and $non_persistence_readback.audit_index_recorded == false
      and $non_persistence_readback.audit_index_persisted == false
      and $non_persistence_readback.audit_index_authoritative == false
      and $non_persistence_readback.audit_index_accepted == false
      and $non_persistence_readback.readback_persisted == false
      and $non_persistence_readback.operator_review_request_allowed == false
      and $non_persistence_readback.operator_review_requested == false
      and $non_persistence_readback.operator_packet_send_allowed == false
      and $non_persistence_readback.operator_packet_acceptance_allowed == false
      and $non_persistence_readback.approval_recording_allowed == false
      and $non_persistence_readback.ready_for_terminal_no_request_closeout == true
      and ($non_persistence_readback.side_effects | to_entries | all(.value == false))) as $source_non_persistence_readback_no_request_confirmed
  | ($non_persistence_readback.operator_review_request_allowed == false
      and $non_persistence_readback.operator_review_requested == false
      and $non_persistence_readback.operator_packet_send_allowed == false
      and $non_persistence_readback.operator_packet_acceptance_allowed == false
      and $non_persistence_readback.approval_recording_allowed == false
      and $non_persistence_readback.config_write_allowed == false
      and $non_persistence_readback.feature_flag_enablement_allowed == false
      and $non_persistence_readback.canary_traffic_allowed == false
      and $non_persistence_readback.scheduler_enforcement_allowed == false
      and $non_persistence_readback.guardrail_enforcement_allowed == false
      and $non_persistence_readback.replay_execution_allowed == false
      and $non_persistence_readback.rollback_execution_allowed == false
      and $non_persistence_readback.work_graph_persistence_allowed == false
      and $non_persistence_readback.live_cutover_allowed == false
      and $non_persistence_readback.ready_for_operator_review_request == false
      and $non_persistence_readback.ready_for_approval_recording == false
      and $non_persistence_readback.ready_for_feature_flag_config_write == false
      and $non_persistence_readback.ready_for_feature_flag_enablement == false
      and $non_persistence_readback.ready_for_canary_traffic == false
      and $non_persistence_readback.ready_for_live_cutover == false) as $source_non_persistence_readback_no_authorization_confirmed
  | ($non_persistence_readback.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate"
      and $non_persistence_readback.non_persistence_readback_preconditions_complete == true
      and $non_persistence_readback.ready_for_terminal_no_request_closeout == true
      and $source_non_persistence_readback_no_request_confirmed
      and $source_non_persistence_readback_no_authorization_confirmed) as $source_non_persistence_readback_ready
  | ($closeout_scope.closeout_visible == true
      and $closeout_scope.terminal_no_request == true
      and $closeout_scope.closeout_recorded == false
      and $closeout_scope.closeout_persisted == false
      and $closeout_scope.closeout_authoritative == false
      and $closeout_scope.closeout_accepted == false
      and $closeout_scope.operator_review_requested == false) as $closeout_scope_terminal_no_request_complete
  | (($closeout_entries | length) > 0
      and ($closeout_entries | all(
        .terminal == true
        and .visible == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .accepted == false
        and .authoritative == false
        and .operator_review_requested == false
        and .mutation_allowed == false
      ))) as $closeout_entries_terminal_no_request_complete
  | (($closeout_blockers | length) > 0
      and ($closeout_blockers | all(.blocked == true))) as $closeout_blockers_complete
  | ($source_non_persistence_readback_ready
      and $closeout_scope_terminal_no_request_complete
      and $closeout_entries_terminal_no_request_complete
      and $closeout_blockers_complete) as $terminal_no_request_closeout_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_v1",
      preview_mode: "operator_review_request_precondition_terminal_no_request_closeout_report_only",
      source_non_persistence_readback_gate: $non_persistence_readback.gate,
      source_readback_entry_count: $non_persistence_readback.readback_entry_count,
      source_readback_blocker_count: $non_persistence_readback.readback_blocker_count,
      source_required_prior_gate_count: $non_persistence_readback.required_prior_gate_count,
      source_non_persistence_readback_preconditions_complete: $non_persistence_readback.non_persistence_readback_preconditions_complete,
      source_non_persistence_readback_no_request_confirmed: $source_non_persistence_readback_no_request_confirmed,
      source_non_persistence_readback_no_authorization_confirmed: $source_non_persistence_readback_no_authorization_confirmed,
      source_non_persistence_readback_ready: $source_non_persistence_readback_ready,
      closeout_entry_count: ($closeout_entries | length),
      closeout_blocker_count: ($closeout_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      closeout_scope: $closeout_scope,
      closeout_entries: $closeout_entries,
      closeout_blockers: $closeout_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate",
      closeout_scope_terminal_no_request_complete: $closeout_scope_terminal_no_request_complete,
      closeout_entries_terminal_no_request_complete: $closeout_entries_terminal_no_request_complete,
      closeout_blockers_complete: $closeout_blockers_complete,
      terminal_no_request_closeout_preconditions_complete: $terminal_no_request_closeout_preconditions_complete,
      terminal_closeout_visible: true,
      terminal_closeout_recorded: false,
      terminal_closeout_persisted: false,
      terminal_closeout_authoritative: false,
      terminal_closeout_accepted: false,
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
      ready_for_terminal_no_request_closeout_readback: $terminal_no_request_closeout_preconditions_complete,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        terminal_closeout_module_present: $terminal_closeout_module_present,
        non_persistence_readback_gate_present: $non_persistence_readback_gate_present,
        non_persistence_readback_points_here: $non_persistence_readback_points_here,
        non_persistence_readback_unrequested_present: $non_persistence_readback_unrequested_present,
        non_persistence_readback_ready_present: $non_persistence_readback_ready_present,
        non_persistence_readback_unpersisted_present: $non_persistence_readback_unpersisted_present,
        non_persistence_readback_report_gate: $non_persistence_readback.gate,
        non_persistence_readback_preconditions_complete: $non_persistence_readback.non_persistence_readback_preconditions_complete,
        non_persistence_readback_ready_for_terminal_closeout: $non_persistence_readback.ready_for_terminal_no_request_closeout,
        non_persistence_readback_side_effects_all_false: ($non_persistence_readback.side_effects | to_entries | all(.value == false))
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
