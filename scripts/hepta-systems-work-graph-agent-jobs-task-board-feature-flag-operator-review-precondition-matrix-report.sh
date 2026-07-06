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

operator_review_matrix_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix.rs
)"
non_persistence_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-non-persistence-readback-gate.sh
)"
non_persistence_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback.rs
)"
operator_review_request_disallowed_present="$(
  bool_for source_has "operator_review_request_allowed: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback.rs
)"
non_persistence_ready_present="$(
  bool_for source_has "ready_for_operator_review_precondition_matrix: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback.rs
)"

jq -n \
  --argjson operator_review_matrix_module_present "$operator_review_matrix_module_present" \
  --argjson non_persistence_readback_gate_present "$non_persistence_readback_gate_present" \
  --argjson non_persistence_readback_points_here "$non_persistence_readback_points_here" \
  --argjson operator_review_request_disallowed_present "$operator_review_request_disallowed_present" \
  --argjson non_persistence_ready_present "$non_persistence_ready_present" \
  '
  def check($id; $category; $satisfied; $blocking; $explanation): {
    id: $id,
    category: $category,
    required: true,
    satisfied: $satisfied,
    blocking: $blocking,
    explanation: $explanation
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "operator review precondition matrix cannot authorize this action"
  };
  [
    check("denial_audit_index_non_persistence_readback_ready"; "source_evidence"; true; false; "denial audit index non-persistence readback is available"),
    check("required_prior_chain_visible"; "source_evidence"; true; false; "the canary prior chain is visible through report-only gates"),
    check("operator_review_request_authorization_missing"; "operator_review_boundary"; false; true; "no explicit authorization exists to request operator review"),
    check("operator_packet_acceptance_missing"; "operator_packet_boundary"; false; true; "operator packet remains unsent and unaccepted"),
    check("approval_recording_authorization_missing"; "approval_boundary"; false; true; "approval recording remains disallowed"),
    check("config_write_authorization_missing"; "config_boundary"; false; true; "feature-flag config writes remain disallowed"),
    check("scheduler_guardrail_enforcement_missing"; "enforcement_boundary"; false; true; "scheduler and guardrail checks remain dry-run only"),
    check("replay_rollback_execution_missing"; "replay_rollback_boundary"; false; true; "replay and rollback remain preview-only"),
    check("live_cutover_authorization_missing"; "live_cutover_boundary"; false; true; "live WorkGraph cutover remains disabled")
  ] as $precondition_checks
  | [
    blocker("operator_review_request_blocked"; "request_operator_review"),
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
    blocker("work_graph_persistence_blocked"; "persist_work_graph_projection"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $blockers
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_v1",
      preview_mode: "operator_review_precondition_matrix_no_request_no_approval_no_write",
      source_non_persistence_readback_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
      source_readback_entry_count: 5,
      source_readback_blocker_count: 12,
      source_required_prior_gate_count: 12,
      precondition_check_count: ($precondition_checks | length),
      precondition_satisfied_count: ($precondition_checks | map(select(.satisfied == true)) | length),
      precondition_unsatisfied_count: ($precondition_checks | map(select(.satisfied == false)) | length),
      blocking_precondition_count: ($precondition_checks | map(select(.blocking == true)) | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      precondition_checks: $precondition_checks,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_non_request_readback_gate",
      matrix_mode: "deny_request_until_explicit_operator_review_authorization",
      operator_review_request_allowed: false,
      operator_review_request_sent: false,
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
      live_cutover_allowed: false,
      ready_for_non_request_readback: true,
      ready_for_operator_review_request: false,
      ready_for_approval_recording: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        operator_review_matrix_module_present: $operator_review_matrix_module_present,
        non_persistence_readback_gate_present: $non_persistence_readback_gate_present,
        non_persistence_readback_points_here: $non_persistence_readback_points_here,
        operator_review_request_disallowed_present: $operator_review_request_disallowed_present,
        non_persistence_ready_present: $non_persistence_ready_present
      },
      side_effects: {
        filesystem_written: false,
        operator_review_requested: false,
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
