#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_ENABLEMENT_DRY_RUN_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-enablement-dry-run-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_ENABLEMENT_DRY_RUN_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_ENABLEMENT_DRY_RUN_CAPTURE_CACHE_DIR"' EXIT
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

enablement_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run.rs
)"
rollback_replay_matrix_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-rollback-replay-pre-enable-blocker-matrix-gate.sh
)"
rollback_replay_matrix_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix.rs
)"
rollback_replay_execution_false_present="$(
  bool_for source_has "replay_executed: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix.rs
)"

rollback_replay_matrix="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-rollback-replay-pre-enable-blocker-matrix-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-rollback-replay-pre-enable-blocker-matrix-report.sh"
)"

jq -n \
  --argjson enablement_module_present "$enablement_module_present" \
  --argjson rollback_replay_matrix_gate_present "$rollback_replay_matrix_gate_present" \
  --argjson rollback_replay_matrix_points_here "$rollback_replay_matrix_points_here" \
  --argjson rollback_replay_execution_false_present "$rollback_replay_execution_false_present" \
  --argjson rollback_replay_matrix "$rollback_replay_matrix" \
  '
  def deny_reason($id; $class; $explanation): {
    id: $id,
    blocker_class: $class,
    required: true,
    satisfied: false,
    explanation: $explanation
  };
  [
    deny_reason("operator_packet_unsent"; "operator_packet_boundary"; "operator packet is visible for readback but has not been sent"),
    deny_reason("operator_packet_unaccepted"; "operator_packet_boundary"; "operator packet has no acceptance or authoritative approval state"),
    deny_reason("approval_record_missing"; "approval_boundary"; "no operator approval record exists for feature-flag enablement"),
    deny_reason("config_write_disabled"; "config_boundary"; "feature-flag config writes remain disabled"),
    deny_reason("feature_flag_state_current_off"; "feature_flag_boundary"; "feature flag current state remains off"),
    deny_reason("canary_traffic_zero_ppm"; "traffic_boundary"; "non-blocking canary traffic remains 0ppm"),
    deny_reason("replay_not_executed"; "replay_boundary"; "replay is required but only previewed by the blocker matrix"),
    deny_reason("rollback_not_executed"; "rollback_boundary"; "rollback rehearsal is required but not executed"),
    deny_reason("scheduler_admission_not_enforced"; "scheduler_boundary"; "scheduler admission remains dry-run only"),
    deny_reason("live_cutover_disabled"; "live_cutover_boundary"; "live WorkGraph cutover remains disabled")
  ] as $deny_reasons
  | ($deny_reasons | map(.id)) as $deny_reason_ids
  | def decision($id; $flag; $surface; $entrypoint): {
      id: $id,
      flag_id: $flag,
      source_surface_id: $surface,
      entrypoint_id: $entrypoint,
      requested_action: "enable_feature_flag_non_blocking_canary",
      decision: "deny",
      explanation: "dry-run denies enablement until operator acceptance, config write authorization, replay/rollback execution, scheduler enforcement, and live cutover preconditions are explicit",
      deny_reason_ids: $deny_reason_ids,
      canary_stage: "shadow_0ppm_report_only",
      requested_traffic_ppm: 1,
      allowed_traffic_ppm: 0,
      config_write_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      live_cutover_allowed: false
    };
  [
    decision(
      "agent_jobs_feature_flag_enablement_precondition_dry_run";
      "work_graph_agent_jobs_non_blocking_canary";
      "agent_jobs_batch_workers";
      "report_agent_job_result"
    ),
    decision(
      "task_board_feature_flag_enablement_precondition_dry_run";
      "work_graph_task_board_non_blocking_canary";
      "hepta_runtime_task_board";
      "task_board_terminal_event"
    )
  ] as $decisions
  | [
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
  | ($rollback_replay_matrix.replay_executed == false
      and $rollback_replay_matrix.rollback_executed == false
      and ($rollback_replay_matrix.side_effects | to_entries | all(.value == false))) as $source_rollback_replay_no_execution_confirmed
  | ($rollback_replay_matrix.ready_for_feature_flag_config_write == false
      and $rollback_replay_matrix.ready_for_feature_flag_enablement == false
      and $rollback_replay_matrix.ready_for_canary_traffic == false
      and $rollback_replay_matrix.ready_for_live_cutover == false) as $source_rollback_replay_no_enablement_confirmed
  | ($rollback_replay_matrix.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate"
      and $rollback_replay_matrix.rollback_replay_blocker_matrix_preconditions_complete == true
      and $rollback_replay_matrix.ready_for_enablement_precondition_dry_run == true
      and $source_rollback_replay_no_execution_confirmed
      and $source_rollback_replay_no_enablement_confirmed) as $source_rollback_replay_ready
  | (($decisions | length) > 0
      and ($decisions | all(
        .decision == "deny"
        and .allowed_traffic_ppm == 0
        and .config_write_allowed == false
        and .feature_flag_enablement_allowed == false
        and .canary_traffic_allowed == false
        and .live_cutover_allowed == false
      ))) as $dry_run_decisions_deny_complete
  | (($deny_reasons | length) > 0
      and ($deny_reasons | all(.required == true and .satisfied == false))) as $deny_reasons_unsatisfied_complete
  | ($source_rollback_replay_ready
      and $dry_run_decisions_deny_complete
      and $deny_reasons_unsatisfied_complete
      and (($decisions | length) == ($decisions | map(select(.decision == "deny")) | length))) as $enablement_precondition_dry_run_preconditions_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_v1",
      preview_mode: "feature_flag_enablement_precondition_dry_run_deny_no_write_no_enablement",
      source_rollback_replay_gate: $rollback_replay_matrix.gate,
      source_rollback_replay_check_count: $rollback_replay_matrix.rollback_replay_check_count,
      source_pre_enable_blocker_count: $rollback_replay_matrix.pre_enable_blocker_count,
      source_required_prior_gate_count: $rollback_replay_matrix.required_prior_gate_count,
      decision_count: ($decisions | length),
      deny_reason_count: ($deny_reasons | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decisions: $decisions,
      deny_reasons: $deny_reasons,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate",
      source_rollback_replay_preconditions_complete: $rollback_replay_matrix.rollback_replay_blocker_matrix_preconditions_complete,
      source_rollback_replay_no_execution_confirmed: $source_rollback_replay_no_execution_confirmed,
      source_rollback_replay_no_enablement_confirmed: $source_rollback_replay_no_enablement_confirmed,
      source_rollback_replay_ready: $source_rollback_replay_ready,
      dry_run_decisions_deny_complete: $dry_run_decisions_deny_complete,
      deny_reasons_unsatisfied_complete: $deny_reasons_unsatisfied_complete,
      enablement_precondition_dry_run_preconditions_complete: $enablement_precondition_dry_run_preconditions_complete,
      dry_run_mode: "deny_only_precondition_explanation",
      dry_run_enforced: false,
      allow_count: 0,
      deny_count: ($decisions | length),
      config_write_allowed: false,
      feature_flag_enablement_allowed: false,
      canary_traffic_allowed: false,
      live_cutover_allowed: false,
      approval_acceptance_allowed: false,
      replay_execution_allowed: false,
      rollback_execution_allowed: false,
      ready_for_denial_readback: $enablement_precondition_dry_run_preconditions_complete,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        enablement_module_present: $enablement_module_present,
        rollback_replay_matrix_gate_present: $rollback_replay_matrix_gate_present,
        rollback_replay_matrix_points_here: $rollback_replay_matrix_points_here,
        rollback_replay_matrix_report_gate: $rollback_replay_matrix.gate,
        rollback_replay_matrix_preconditions_complete: $rollback_replay_matrix.rollback_replay_blocker_matrix_preconditions_complete,
        rollback_replay_matrix_ready_for_dry_run: $rollback_replay_matrix.ready_for_enablement_precondition_dry_run,
        rollback_replay_matrix_side_effects_all_false: ($rollback_replay_matrix.side_effects | to_entries | all(.value == false)),
        rollback_replay_execution_false_present: $rollback_replay_execution_false_present
      },
      side_effects: {
        filesystem_written: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
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
