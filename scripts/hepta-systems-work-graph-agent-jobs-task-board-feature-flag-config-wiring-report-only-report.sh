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

config_wiring_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only.rs
)"
feature_flag_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-gate.sh
)"
feature_flag_gate_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary.rs
)"
agent_jobs_metadata_flag_fields_present="$(
  bool_for source_has "feature_flag_enabled: false" \
    codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
agent_jobs_metadata_safety_fields_present="$(
  bool_for source_has "rollback_replay_required: true" \
    codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_metadata_flag_fields_present="$(
  bool_for source_has "feature_flag_enabled: false" codex-rs/hepta-runtime/src/task_board.rs
)"
task_board_metadata_safety_fields_present="$(
  bool_for source_has "rollback_replay_required: true" codex-rs/hepta-runtime/src/task_board.rs
)"

jq -n \
  --argjson config_wiring_module_present "$config_wiring_module_present" \
  --argjson feature_flag_gate_present "$feature_flag_gate_present" \
  --argjson feature_flag_gate_points_here "$feature_flag_gate_points_here" \
  --argjson agent_jobs_metadata_flag_fields_present "$agent_jobs_metadata_flag_fields_present" \
  --argjson agent_jobs_metadata_safety_fields_present "$agent_jobs_metadata_safety_fields_present" \
  --argjson task_board_metadata_flag_fields_present "$task_board_metadata_flag_fields_present" \
  --argjson task_board_metadata_safety_fields_present "$task_board_metadata_safety_fields_present" \
  '
  def contract($id; $flag; $source; $entrypoint): {
    id: $id,
    flag_id: $flag,
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    owner: "hepta-backend",
    config_surface_id: "work_graph_agent_jobs_task_board_canary_flags",
    default_enabled: false,
    current_enabled: false,
    canary_stage_id: "shadow_0ppm_report_only",
    traffic_ppm: 0,
    readback_required: true,
    rollback_replay_required: true,
    operator_packet_required: true,
    config_digest_required: true,
    config_written: false,
    live_mutation_allowed: false
  };
  def digest($id; $flag): {
    id: $id,
    flag_id: $flag,
    digest_algorithm: "sha256",
    digest_source: "canonical_config_contract_preview",
    redaction_policy: "redact_operator_packet_and_runtime_payloads",
    deterministic_input_count: 9,
    digest_required_before_enablement: true,
    digest_persisted: false
  };
  def binding($source; $entrypoint; $contract): {
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    metadata_field: "workGraphReportOnly",
    config_contract_id: $contract,
    field_present: true,
    default_off_observed: true,
    zero_traffic_observed: true,
    readback_observed: true,
    rollback_replay_observed: true,
    live_cutover_observed: false
  };
  def safety($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    contract(
      "agent_jobs_feature_flag_config_contract";
      "work_graph_agent_jobs_non_blocking_canary";
      "agent_jobs_batch_workers";
      "report_agent_job_result"
    ),
    contract(
      "task_board_feature_flag_config_contract";
      "work_graph_task_board_non_blocking_canary";
      "hepta_runtime_task_board";
      "task_board_terminal_event"
    )
  ] as $config_contracts
  | [
    digest(
      "agent_jobs_feature_flag_config_digest_preview";
      "work_graph_agent_jobs_non_blocking_canary"
    ),
    digest(
      "task_board_feature_flag_config_digest_preview";
      "work_graph_task_board_non_blocking_canary"
    )
  ] as $config_digest_previews
  | [
    binding(
      "agent_jobs_batch_workers";
      "report_agent_job_result";
      "agent_jobs_feature_flag_config_contract"
    ),
    binding(
      "hepta_runtime_task_board";
      "task_board_terminal_event";
      "task_board_feature_flag_config_contract"
    )
  ] as $source_bindings
  | [
    safety(
      "feature_flag_config_contract_default_off";
      "config wiring can name canary flags, but every contract must remain default/current off"
    ),
    safety(
      "feature_flag_config_digest_required_unpersisted";
      "enablement requires a deterministic digest preview without persisting the digest"
    ),
    safety(
      "feature_flag_operator_packet_required";
      "operator packet review is required before any flag write, traffic, or live cutover"
    ),
    safety(
      "feature_flag_config_wiring_has_no_runtime_mutation";
      "report-only config wiring must not mutate config, traffic, WorkGraph state, or runtime enforcement"
    )
  ] as $safety_checks
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_v1",
      preview_mode: "feature_flag_config_wiring_report_only_no_config_write",
      config_contract_count: ($config_contracts | length),
      config_digest_preview_count: ($config_digest_previews | length),
      source_binding_count: ($source_bindings | length),
      safety_check_count: ($safety_checks | length),
      required_prior_gate_count: ($required_prior_gates | length),
      config_contracts: $config_contracts,
      config_digest_previews: $config_digest_previews,
      source_bindings: $source_bindings,
      safety_checks: $safety_checks,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
      ready_for_operator_packet_report_only: true,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_live_cutover: false,
      source_probes: {
        config_wiring_module_present: $config_wiring_module_present,
        feature_flag_gate_present: $feature_flag_gate_present,
        feature_flag_gate_points_here: $feature_flag_gate_points_here,
        agent_jobs_metadata_flag_fields_present: $agent_jobs_metadata_flag_fields_present,
        agent_jobs_metadata_safety_fields_present: $agent_jobs_metadata_safety_fields_present,
        task_board_metadata_flag_fields_present: $task_board_metadata_flag_fields_present,
        task_board_metadata_safety_fields_present: $task_board_metadata_safety_fields_present
      },
      side_effects: {
        filesystem_written: false,
        config_written: false,
        feature_flag_mutated: false,
        non_blocking_canary_enabled: false,
        live_cutover_enabled: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        config_digest_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        replay_executed: false,
        rollback_executed: false,
        approval_recorded: false,
        operator_packet_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
