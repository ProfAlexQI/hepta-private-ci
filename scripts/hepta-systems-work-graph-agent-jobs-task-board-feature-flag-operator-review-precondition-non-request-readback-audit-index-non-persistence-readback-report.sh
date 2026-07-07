#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_OPERATOR_REVIEW_NON_REQUEST_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-operator-review-non-request-audit-index-non-persistence-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_OPERATOR_REVIEW_NON_REQUEST_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_OPERATOR_REVIEW_NON_REQUEST_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR"' EXIT
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

non_persistence_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback.rs
)"
audit_index_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-precondition-non-request-readback-audit-index-gate.sh
)"
audit_index_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index.rs
)"
audit_index_unrequested_present="$(
  bool_for source_has "operator_review_requested: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index.rs
)"
audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index.rs
)"

audit_index="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-review-precondition-non-request-readback-audit-index-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-precondition-non-request-readback-audit-index-report.sh"
)"

jq -n \
  --argjson non_persistence_readback_module_present "$non_persistence_readback_module_present" \
  --argjson audit_index_gate_present "$audit_index_gate_present" \
  --argjson audit_index_points_here "$audit_index_points_here" \
  --argjson audit_index_unrequested_present "$audit_index_unrequested_present" \
  --argjson audit_index_unpersisted_present "$audit_index_unpersisted_present" \
  --argjson audit_index "$audit_index" \
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
    reason: "operator review non-request audit index non-persistence readback cannot authorize this action"
  };
  {
    id: "agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_precondition_non_request_readback_audit_index",
    readback_mode: "operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_precondition.non_request_readback.audit_index.non_persistence_readback",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    operator_review_requested: false,
    readback_persisted: false
  } as $readback_scope
  | [
    entry("operator_review_non_request_audit_index_surface_readback"; "operator_review_non_request_audit_index_visible_unrecorded"; "audit_index_visible_without_request_record_persist_accept_or_authority"),
    entry("operator_review_non_request_audit_index_prior_chain_readback"; "operator_review_non_request_audit_index_required_priors_visible"; "fifteen_required_prior_gates_visible_but_not_persisted"),
    entry("operator_review_non_request_audit_index_blocker_readback"; "operator_review_non_request_audit_index_blockers_visible"; "fifteen_blockers_visible_and_still_blocking"),
    entry("operator_review_non_request_audit_index_non_persistence_boundary_readback"; "operator_review_non_request_audit_index_non_persistence_boundary"; "audit_index_does_not_write_work_graph_projection_config_or_approval_state"),
    entry("operator_review_non_request_audit_index_no_request_boundary_readback"; "operator_review_non_request_audit_index_no_request_boundary"; "audit_index_does_not_request_operator_review_or_acceptance")
  ] as $readback_entries
  | [
    blocker("audit_index_readback_persistence_blocked"; "persist_operator_review_non_request_audit_index_readback"),
    blocker("audit_index_record_blocked"; "record_operator_review_non_request_audit_index"),
    blocker("audit_index_persistence_blocked"; "persist_operator_review_non_request_audit_index"),
    blocker("audit_index_acceptance_blocked"; "accept_operator_review_non_request_audit_index"),
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
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $readback_blockers
  | [
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
  | ($audit_index.audit_index_visible == true
      and $audit_index.audit_index_recorded == false
      and $audit_index.audit_index_persisted == false
      and $audit_index.audit_index_authoritative == false
      and $audit_index.audit_index_accepted == false
      and $audit_index.operator_review_request_allowed == false
      and $audit_index.operator_review_requested == false
      and ($audit_index.side_effects | to_entries | all(.value == false))) as $source_audit_index_no_record_persist_request_confirmed
  | ($audit_index.operator_review_request_allowed == false
      and $audit_index.operator_review_requested == false
      and $audit_index.operator_packet_send_allowed == false
      and $audit_index.operator_packet_acceptance_allowed == false
      and $audit_index.approval_recording_allowed == false
      and $audit_index.audit_index_authorizes_operator_review_request == false
      and $audit_index.audit_index_authorizes_config_write == false
      and $audit_index.audit_index_authorizes_feature_flag_enablement == false
      and $audit_index.audit_index_authorizes_canary_traffic == false
      and $audit_index.audit_index_authorizes_scheduler_enforcement == false
      and $audit_index.audit_index_authorizes_guardrail_enforcement == false
      and $audit_index.audit_index_authorizes_replay_execution == false
      and $audit_index.audit_index_authorizes_rollback_execution == false
      and $audit_index.audit_index_authorizes_live_cutover == false
      and $audit_index.ready_for_operator_review_request == false
      and $audit_index.ready_for_approval_recording == false
      and $audit_index.ready_for_feature_flag_config_write == false
      and $audit_index.ready_for_feature_flag_enablement == false
      and $audit_index.ready_for_canary_traffic == false
      and $audit_index.ready_for_live_cutover == false) as $source_audit_index_no_authorization_confirmed
  | ($audit_index.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_gate"
      and $audit_index.non_request_audit_index_preconditions_complete == true
      and $audit_index.ready_for_non_persistence_readback == true
      and $source_audit_index_no_record_persist_request_confirmed
      and $source_audit_index_no_authorization_confirmed) as $source_audit_index_ready
  | ($readback_scope.audit_index_visible == true
      and $readback_scope.audit_index_recorded == false
      and $readback_scope.audit_index_persisted == false
      and $readback_scope.audit_index_authoritative == false
      and $readback_scope.audit_index_accepted == false
      and $readback_scope.operator_review_requested == false
      and $readback_scope.readback_persisted == false) as $readback_scope_non_persistent_complete
  | (($readback_entries | length) > 0
      and ($readback_entries | all(
        .visible == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .accepted == false
        and .authoritative == false
        and .operator_review_requested == false
        and .mutation_allowed == false
      ))) as $readback_entries_non_persistent_complete
  | (($readback_blockers | length) > 0
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_audit_index_ready
      and $readback_scope_non_persistent_complete
      and $readback_entries_non_persistent_complete
      and $readback_blockers_complete) as $non_persistence_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_v1",
      preview_mode: "operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_only",
      source_audit_index_gate: $audit_index.gate,
      source_audit_index_entry_count: $audit_index.audit_index_entry_count,
      source_audit_index_blocker_count: $audit_index.audit_index_blocker_count,
      source_required_prior_gate_count: $audit_index.required_prior_gate_count,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate",
      source_audit_index_preconditions_complete: $audit_index.non_request_audit_index_preconditions_complete,
      source_audit_index_no_record_persist_request_confirmed: $source_audit_index_no_record_persist_request_confirmed,
      source_audit_index_no_authorization_confirmed: $source_audit_index_no_authorization_confirmed,
      source_audit_index_ready: $source_audit_index_ready,
      readback_scope_non_persistent_complete: $readback_scope_non_persistent_complete,
      readback_entries_non_persistent_complete: $readback_entries_non_persistent_complete,
      readback_blockers_complete: $readback_blockers_complete,
      non_persistence_readback_preconditions_complete: $non_persistence_readback_preconditions_complete,
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
      live_cutover_allowed: false,
      ready_for_operator_review_request_precondition_blocker_matrix: $non_persistence_readback_preconditions_complete,
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
        audit_index_report_gate: $audit_index.gate,
        audit_index_preconditions_complete: $audit_index.non_request_audit_index_preconditions_complete,
        audit_index_ready_for_non_persistence_readback: $audit_index.ready_for_non_persistence_readback,
        audit_index_side_effects_all_false: ($audit_index.side_effects | to_entries | all(.value == false)),
        audit_index_unpersisted_present: $audit_index_unpersisted_present
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
