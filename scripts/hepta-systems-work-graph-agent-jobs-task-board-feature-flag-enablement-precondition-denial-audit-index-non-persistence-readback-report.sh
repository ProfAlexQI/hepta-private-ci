#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-denial-audit-index-non-persistence-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_CAPTURE_CACHE_DIR"' EXIT
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
denial_audit_index_unpersisted_present="$(
  bool_for source_has "audit_index_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index.rs
)"

denial_audit_index="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-report.sh"
)"

jq -n \
  --argjson non_persistence_readback_module_present "$non_persistence_readback_module_present" \
  --argjson denial_audit_index_gate_present "$denial_audit_index_gate_present" \
  --argjson denial_audit_index_points_here "$denial_audit_index_points_here" \
  --argjson denial_audit_index_unpersisted_present "$denial_audit_index_unpersisted_present" \
  --argjson denial_audit_index "$denial_audit_index" \
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
  | ($denial_audit_index.audit_index_visible == true
      and $denial_audit_index.audit_index_recorded == false
      and $denial_audit_index.audit_index_persisted == false
      and $denial_audit_index.audit_index_authoritative == false
      and $denial_audit_index.audit_index_acceptance_allowed == false
      and ($denial_audit_index.side_effects | to_entries | all(.value == false))) as $source_denial_audit_index_no_record_persist_accept_confirmed
  | ($denial_audit_index.audit_index_authorizes_config_write == false
      and $denial_audit_index.audit_index_authorizes_feature_flag_enablement == false
      and $denial_audit_index.audit_index_authorizes_canary_traffic == false
      and $denial_audit_index.audit_index_authorizes_scheduler_enforcement == false
      and $denial_audit_index.audit_index_authorizes_replay_execution == false
      and $denial_audit_index.audit_index_authorizes_rollback_execution == false
      and $denial_audit_index.audit_index_authorizes_live_cutover == false
      and $denial_audit_index.ready_for_feature_flag_config_write == false
      and $denial_audit_index.ready_for_feature_flag_enablement == false
      and $denial_audit_index.ready_for_canary_traffic == false
      and $denial_audit_index.ready_for_live_cutover == false) as $source_denial_audit_index_no_authorization_confirmed
  | ($denial_audit_index.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate"
      and $denial_audit_index.denial_audit_index_preconditions_complete == true
      and $denial_audit_index.ready_for_non_persistence_readback == true
      and $source_denial_audit_index_no_record_persist_accept_confirmed
      and $source_denial_audit_index_no_authorization_confirmed) as $source_denial_audit_index_ready
  | ($readback_scope.audit_index_visible == true
      and $readback_scope.audit_index_recorded == false
      and $readback_scope.audit_index_persisted == false
      and $readback_scope.audit_index_authoritative == false
      and $readback_scope.audit_index_accepted == false
      and $readback_scope.readback_persisted == false) as $readback_scope_non_persistent_complete
  | (($readback_entries | length) > 0
      and ($readback_entries | all(
        .visible == true
        and .ready == true
        and .recorded == false
        and .persisted == false
        and .accepted == false
        and .authoritative == false
        and .mutation_allowed == false
      ))) as $readback_entries_non_persistent_complete
  | (($readback_blockers | length) > 0
      and ($readback_blockers | all(.blocked == true))) as $readback_blockers_complete
  | ($source_denial_audit_index_ready
      and $readback_scope_non_persistent_complete
      and $readback_entries_non_persistent_complete
      and $readback_blockers_complete) as $non_persistence_readback_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_v1",
      preview_mode: "denial_audit_index_non_persistence_readback_no_record_no_acceptance_no_write",
      source_denial_audit_index_gate: $denial_audit_index.gate,
      source_audit_index_entry_count: $denial_audit_index.audit_index_entry_count,
      source_audit_index_blocker_count: $denial_audit_index.audit_index_blocker_count,
      source_required_prior_gate_count: $denial_audit_index.required_prior_gate_count,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate",
      source_denial_audit_index_preconditions_complete: $denial_audit_index.denial_audit_index_preconditions_complete,
      source_denial_audit_index_no_record_persist_accept_confirmed: $source_denial_audit_index_no_record_persist_accept_confirmed,
      source_denial_audit_index_no_authorization_confirmed: $source_denial_audit_index_no_authorization_confirmed,
      source_denial_audit_index_ready: $source_denial_audit_index_ready,
      readback_scope_non_persistent_complete: $readback_scope_non_persistent_complete,
      readback_entries_non_persistent_complete: $readback_entries_non_persistent_complete,
      readback_blockers_complete: $readback_blockers_complete,
      non_persistence_readback_preconditions_complete: $non_persistence_readback_preconditions_complete,
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
      ready_for_operator_review_precondition_matrix: $non_persistence_readback_preconditions_complete,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        non_persistence_readback_module_present: $non_persistence_readback_module_present,
        denial_audit_index_gate_present: $denial_audit_index_gate_present,
        denial_audit_index_points_here: $denial_audit_index_points_here,
        denial_audit_index_report_gate: $denial_audit_index.gate,
        denial_audit_index_preconditions_complete: $denial_audit_index.denial_audit_index_preconditions_complete,
        denial_audit_index_ready_for_non_persistence_readback: $denial_audit_index.ready_for_non_persistence_readback,
        denial_audit_index_side_effects_all_false: ($denial_audit_index.side_effects | to_entries | all(.value == false)),
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
