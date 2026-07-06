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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback.rs
)"
audit_index_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-denial-readback-audit-index-gate.sh
)"
audit_index_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index.rs
)"
audit_index_unrequested_present="$(
  bool_for source_has "operator_review_requested: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index.rs
)"
audit_index_ready_present="$(
  bool_for source_has "ready_for_non_persistence_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index.rs
)"
audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index.rs
)"

jq -n \
  --argjson non_persistence_readback_module_present "$non_persistence_readback_module_present" \
  --argjson audit_index_gate_present "$audit_index_gate_present" \
  --argjson audit_index_points_here "$audit_index_points_here" \
  --argjson audit_index_unrequested_present "$audit_index_unrequested_present" \
  --argjson audit_index_ready_present "$audit_index_ready_present" \
  --argjson audit_index_unpersisted_present "$audit_index_unpersisted_present" \
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
    operator_review_requested: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "operator review request denial audit index non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition_denial_readback_audit_index",
    readback_mode: "operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_readback.audit_index.non_persistence_readback",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    operator_review_requested: false,
    readback_persisted: false
  } as $readback_scope
  | [
    entry("operator_review_request_denial_audit_index_surface_readback"; "operator_review_request_denial_audit_index_visible_unrecorded"; "audit_index_visible_without_request_record_persist_accept_or_authority"),
    entry("operator_review_request_denial_audit_index_prior_chain_readback"; "operator_review_request_denial_audit_index_required_priors_visible"; "nineteen_required_prior_gates_visible_but_not_persisted"),
    entry("operator_review_request_denial_audit_index_blocker_readback"; "operator_review_request_denial_audit_index_blockers_visible"; "nineteen_blockers_visible_and_still_blocking"),
    entry("operator_review_request_denial_audit_index_non_persistence_boundary_readback"; "operator_review_request_denial_audit_index_non_persistence_boundary"; "audit_index_does_not_write_work_graph_projection_config_or_approval_state"),
    entry("operator_review_request_denial_audit_index_no_request_boundary_readback"; "operator_review_request_denial_audit_index_no_request_boundary"; "audit_index_does_not_request_operator_review_or_acceptance")
  ] as $readback_entries
  | [
    blocker("audit_index_readback_persistence_blocked"; "persist_operator_review_request_denial_audit_index_readback"),
    blocker("audit_index_record_blocked"; "record_operator_review_request_denial_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_operator_review_request_denial_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_operator_review_request_denial_audit_index"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("operator_review_request_record_blocked"; "record_operator_review_request"),
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate",
      source_audit_index_entry_count: 6,
      source_audit_index_blocker_count: 19,
      source_required_prior_gate_count: 19,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      operator_review_request_allowed: false,
      operator_review_requested: false,
      operator_packet_send_allowed: false,
      operator_packet_acceptance_allowed: false,
      approval_recording_allowed: false,
      readback_persisted: false,
      config_write_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      scheduler_enforcement_allowed: false,
      guardrail_enforcement_allowed: false,
      replay_execution_allowed: false,
      rollback_execution_allowed: false,
      work_graph_persistence_allowed: false,
      live_cutover_allowed: false,
      ready_for_terminal_no_request_closeout: true,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        non_persistence_readback_module_present: $non_persistence_readback_module_present,
        audit_index_gate_present: $audit_index_gate_present,
        audit_index_points_here: $audit_index_points_here,
        audit_index_unrequested_present: $audit_index_unrequested_present,
        audit_index_ready_present: $audit_index_ready_present,
        audit_index_unpersisted_present: $audit_index_unpersisted_present
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
