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

request_denial_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback.rs
)"
request_blocker_matrix_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-blocker-matrix-gate.sh
)"
request_blocker_matrix_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix.rs
)"
request_blocker_matrix_denies_present="$(
  bool_for source_has "request_decision: \"deny\"" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix.rs
)"
request_blocker_matrix_ready_present="$(
  bool_for source_has "ready_for_request_denial_readback: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix.rs
)"

jq -n \
  --argjson request_denial_readback_module_present "$request_denial_readback_module_present" \
  --argjson request_blocker_matrix_gate_present "$request_blocker_matrix_gate_present" \
  --argjson request_blocker_matrix_points_here "$request_blocker_matrix_points_here" \
  --argjson request_blocker_matrix_denies_present "$request_blocker_matrix_denies_present" \
  --argjson request_blocker_matrix_ready_present "$request_blocker_matrix_ready_present" \
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
  def blocker($id; $action; $reason): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: $reason
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition_blocker_matrix",
    readback_mode: "operator_review_request_precondition_denial_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_readback",
    denial_visible: true,
    denial_recorded: false,
    denial_persisted: false,
    denial_accepted: false,
    denial_authoritative: false,
    readback_persisted: false
  } as $request_denial_readback_scope
  | [
    entry(
      "operator_review_request_denial_decision_readback";
      "operator_review_request_precondition_decision_deny";
      "request_operator_review_deny_visible_without_record_accept_or_persistence"
    ),
    entry(
      "operator_review_request_precondition_check_catalog_readback";
      "operator_review_request_precondition_checks_visible";
      "twelve_checks_visible_with_ten_blocking_and_two_source_evidence_checks"
    ),
    entry(
      "operator_review_request_blocker_catalog_readback";
      "operator_review_request_precondition_blockers_visible";
      "seventeen_blocked_actions_visible_without_authority_to_mutate"
    ),
    entry(
      "operator_review_request_boundary_readback";
      "operator_review_request_not_requested_or_recorded";
      "operator_review_request_remains_not_requested_not_recorded_not_persisted_not_accepted"
    ),
    entry(
      "operator_review_request_live_boundary_readback";
      "operator_review_request_denial_does_not_unlock_live_paths";
      "denial_readback_cannot_authorize_packet_approval_config_enablement_traffic_or_cutover"
    )
  ] as $request_denial_readback_entries
  | [
    blocker(
      "request_denial_readback_acceptance_blocked";
      "accept_operator_review_request_denial_readback";
      "request denial readback is not an operator acceptance or approval record"
    ),
    blocker(
      "request_denial_readback_persistence_blocked";
      "persist_operator_review_request_denial_readback";
      "request denial readback remains stdout/report-only and unpersisted"
    ),
    blocker(
      "operator_review_request_blocked";
      "request_operator_review";
      "denial readback cannot request operator review"
    ),
    blocker(
      "operator_review_request_record_blocked";
      "record_operator_review_request";
      "denial readback cannot record an operator review request"
    ),
    blocker(
      "operator_review_request_acceptance_blocked";
      "accept_operator_review_request";
      "denial readback cannot accept an operator review request"
    ),
    blocker(
      "operator_packet_send_blocked";
      "send_operator_packet";
      "operator packet send remains disallowed"
    ),
    blocker(
      "operator_packet_acceptance_blocked";
      "accept_operator_packet";
      "operator packet acceptance remains missing"
    ),
    blocker(
      "approval_record_blocked";
      "record_operator_approval";
      "approval recording remains disallowed"
    ),
    blocker(
      "feature_flag_config_write_blocked";
      "write_feature_flag_config";
      "feature-flag config writes remain disabled"
    ),
    blocker(
      "feature_flag_enablement_blocked";
      "enable_feature_flag";
      "feature flags remain current off"
    ),
    blocker(
      "canary_traffic_blocked";
      "route_canary_traffic";
      "canary traffic remains 0ppm"
    ),
    blocker(
      "scheduler_enforcement_blocked";
      "enforce_scheduler_admission";
      "scheduler admission remains dry-run only"
    ),
    blocker(
      "guardrail_enforcement_blocked";
      "enable_guardrail_enforcement";
      "guardrail enforcement remains report-only"
    ),
    blocker(
      "replay_execution_blocked";
      "execute_replay";
      "replay remains unexecuted"
    ),
    blocker(
      "rollback_execution_blocked";
      "execute_rollback";
      "rollback remains unexecuted"
    ),
    blocker(
      "work_graph_projection_persistence_blocked";
      "persist_work_graph_projection";
      "WorkGraph projection persistence remains disabled"
    ),
    blocker(
      "work_graph_event_record_blocked";
      "record_work_graph_event";
      "WorkGraph event recording remains disabled"
    ),
    blocker(
      "live_cutover_blocked";
      "perform_live_cutover";
      "live cutover remains disabled"
    )
  ] as $request_denial_readback_blockers
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_v1",
      preview_mode: "operator_review_request_precondition_denial_readback_only_no_request_no_record_no_persistence",
      source_request_precondition_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate",
      source_request_decision: "deny",
      source_request_blocker_count: 17,
      source_request_precondition_check_count: 12,
      source_required_prior_gate_count: 17,
      request_denial_readback_entry_count: ($request_denial_readback_entries | length),
      request_denial_readback_blocker_count: ($request_denial_readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      request_denial_readback_scope: $request_denial_readback_scope,
      request_denial_readback_entries: $request_denial_readback_entries,
      request_denial_readback_blockers: $request_denial_readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate",
      request_denial_visible: true,
      request_denial_recorded: false,
      request_denial_persisted: false,
      request_denial_accepted: false,
      request_denial_authoritative: false,
      request_denial_readback_persisted: false,
      request_denial_authorizes_operator_review_request: false,
      request_denial_authorizes_operator_packet_send: false,
      request_denial_authorizes_approval_recording: false,
      request_denial_authorizes_config_write: false,
      request_denial_authorizes_feature_flag_enablement: false,
      request_denial_authorizes_canary_traffic: false,
      request_denial_authorizes_live_cutover: false,
      operator_review_requested: false,
      operator_review_request_recorded: false,
      operator_review_request_persisted: false,
      operator_review_request_accepted: false,
      ready_for_request_denial_audit_index: true,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        request_denial_readback_module_present: $request_denial_readback_module_present,
        request_blocker_matrix_gate_present: $request_blocker_matrix_gate_present,
        request_blocker_matrix_points_here: $request_blocker_matrix_points_here,
        request_blocker_matrix_denies_present: $request_blocker_matrix_denies_present,
        request_blocker_matrix_ready_present: $request_blocker_matrix_ready_present
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
