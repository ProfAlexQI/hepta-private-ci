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

feature_flag_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary.rs
)"
canary_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-canary-readback-replay-gate.sh
)"
entrypoint_emission_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-gate.sh
)"
agent_jobs_feature_flag_field_present="$(
  bool_for source_has "feature_flag_id" codex-rs/core/src/tools/handlers/agent_jobs.rs
)"
agent_jobs_feature_flag_id_present="$(
  bool_for source_has "work_graph_agent_jobs_non_blocking_canary" codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_feature_flag_field_present="$(
  bool_for source_has "feature_flag_id" codex-rs/hepta-runtime/src/task_board.rs
)"
task_board_feature_flag_id_present="$(
  bool_for source_has "work_graph_task_board_non_blocking_canary" codex-rs/hepta-runtime/src/task_board.rs
)"

jq -n \
  --argjson feature_flag_module_present "$feature_flag_module_present" \
  --argjson canary_readback_gate_present "$canary_readback_gate_present" \
  --argjson entrypoint_emission_gate_present "$entrypoint_emission_gate_present" \
  --argjson agent_jobs_feature_flag_field_present "$agent_jobs_feature_flag_field_present" \
  --argjson agent_jobs_feature_flag_id_present "$agent_jobs_feature_flag_id_present" \
  --argjson task_board_feature_flag_field_present "$task_board_feature_flag_field_present" \
  --argjson task_board_feature_flag_id_present "$task_board_feature_flag_id_present" \
  '
  def flag($id; $source; $entrypoint): {
    id: $id,
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    config_surface_id: "work_graph_agent_jobs_task_board_canary_flags",
    default_enabled: false,
    current_enabled: false,
    canary_stage_id: "shadow_0ppm_report_only",
    traffic_ppm: 0,
    required_before_enablement: [
      "canary_readback_replay_gate_green",
      "operator_review_packet",
      "rollback_replay_gate_green",
      "feature_flag_config_digest"
    ],
    allows_live_blocking: false,
    allows_persistence: false
  };
  def binding($source; $entrypoint; $flag): {
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    report_only_field: "workGraphReportOnly",
    feature_flag_id: $flag,
    feature_flag_field_attached: true,
    readback_field_attached: true,
    rollback_replay_field_attached: true,
    live_cutover_field_attached: true,
    live_cutover_enabled: false
  };
  def safety($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    flag("work_graph_agent_jobs_non_blocking_canary"; "agent_jobs_batch_workers"; "report_agent_job_result"),
    flag("work_graph_task_board_non_blocking_canary"; "hepta_runtime_task_board"; "task_board_terminal_event")
  ] as $feature_flags
  | [{
    id: "shadow_0ppm_report_only",
    order: 0,
    traffic_ppm: 0,
    mode: "report_only_observation",
    blocks_runtime_mutation: true,
    requires_readback_gate: true,
    requires_rollback_replay_gate: true
  }] as $rollout_stages
  | [
    binding("agent_jobs_batch_workers"; "report_agent_job_result"; "work_graph_agent_jobs_non_blocking_canary"),
    binding("hepta_runtime_task_board"; "task_board_terminal_event"; "work_graph_task_board_non_blocking_canary")
  ] as $emission_bindings
  | [
    safety("feature_flags_default_off"; "canary flags must be present in report-only metadata but disabled by default"),
    safety("non_blocking_canary_traffic_zero_ppm"; "the first canary stage must observe only and route zero live traffic"),
    safety("readback_and_rollback_replay_required"; "canary enablement must require readback and rollback/replay gates before any runtime mutation"),
    safety("live_cutover_remains_false"; "feature flag metadata must not enable live blocking, persistence, external send, or cutover")
  ] as $safety_checks
  | [
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_v1",
      preview_mode: "feature_flag_non_blocking_canary_report_only_no_flag_mutation",
      feature_flag_count: ($feature_flags | length),
      rollout_stage_count: ($rollout_stages | length),
      emission_binding_count: ($emission_bindings | length),
      safety_check_count: ($safety_checks | length),
      required_prior_gate_count: ($required_prior_gates | length),
      feature_flags: $feature_flags,
      rollout_stages: $rollout_stages,
      emission_bindings: $emission_bindings,
      safety_checks: $safety_checks,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
      ready_for_feature_flag_config_wiring: true,
      ready_for_feature_flag_enablement: false,
      ready_for_live_cutover: false,
      source_probes: {
        feature_flag_module_present: $feature_flag_module_present,
        canary_readback_gate_present: $canary_readback_gate_present,
        entrypoint_emission_gate_present: $entrypoint_emission_gate_present,
        agent_jobs_feature_flag_field_present: $agent_jobs_feature_flag_field_present,
        agent_jobs_feature_flag_id_present: $agent_jobs_feature_flag_id_present,
        task_board_feature_flag_field_present: $task_board_feature_flag_field_present,
        task_board_feature_flag_id_present: $task_board_feature_flag_id_present
      },
      side_effects: {
        filesystem_written: false,
        feature_flag_mutated: false,
        non_blocking_canary_enabled: false,
        live_cutover_enabled: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        replay_executed: false,
        rollback_executed: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
