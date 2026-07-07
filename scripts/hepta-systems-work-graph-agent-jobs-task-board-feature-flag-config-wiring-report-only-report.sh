#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_CONFIG_WIRING_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-config-wiring-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_CONFIG_WIRING_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_CONFIG_WIRING_CAPTURE_CACHE_DIR"' EXIT
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

feature_flag="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report.sh"
)"
canary_readback="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-canary-readback-replay-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-canary-readback-replay-report.sh"
)"
entrypoint_emission="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report.sh"
)"
trace_guardrail="$(
  capture_json_report \
    "hepta-work-graph-trace-guardrail-span-report-only-report" \
    "$ROOT/scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-report.sh"
)"
scheduler="$(
  capture_json_report \
    "hepta-work-graph-scheduler-admission-dry-run-enforcement-report" \
    "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-report.sh"
)"

jq -n \
  --argjson config_wiring_module_present "$config_wiring_module_present" \
  --argjson feature_flag_gate_present "$feature_flag_gate_present" \
  --argjson feature_flag_gate_points_here "$feature_flag_gate_points_here" \
  --argjson agent_jobs_metadata_flag_fields_present "$agent_jobs_metadata_flag_fields_present" \
  --argjson agent_jobs_metadata_safety_fields_present "$agent_jobs_metadata_safety_fields_present" \
  --argjson task_board_metadata_flag_fields_present "$task_board_metadata_flag_fields_present" \
  --argjson task_board_metadata_safety_fields_present "$task_board_metadata_safety_fields_present" \
  --argjson feature_flag "$feature_flag" \
  --argjson canary_readback "$canary_readback" \
  --argjson entrypoint_emission "$entrypoint_emission" \
  --argjson trace_guardrail "$trace_guardrail" \
  --argjson scheduler "$scheduler" \
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
  | ($feature_flag.ready_for_feature_flag_enablement == false
      and $feature_flag.ready_for_live_cutover == false
      and ($feature_flag.side_effects | to_entries | all(.value == false))) as $source_feature_flag_non_blocking_canary_no_enablement_confirmed
  | ($feature_flag.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
      and $feature_flag.feature_flag_prior_readbacks_complete == true
      and $feature_flag.feature_flag_enablement_preconditions_report_only_complete == true
      and $feature_flag.ready_for_feature_flag_config_wiring == true
      and $source_feature_flag_non_blocking_canary_no_enablement_confirmed) as $source_feature_flag_non_blocking_canary_ready
  | ($canary_readback.feature_flag_enabled == false
      and $canary_readback.ready_for_live_cutover == false
      and ($canary_readback.side_effects | to_entries | all(.value == false))) as $source_canary_readback_replay_no_live_confirmed
  | ($canary_readback.gate == "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
      and $canary_readback.canary_readback_replay_prior_readbacks_complete == true
      and $canary_readback.canary_projection_readback_replay_preview_complete == true
      and $canary_readback.ready_for_non_blocking_canary == true
      and $source_canary_readback_replay_no_live_confirmed) as $source_canary_readback_replay_ready
  | ($entrypoint_emission.ready_for_live_execution == false
      and ($entrypoint_emission.side_effects | to_entries | all(.value == false))) as $source_entrypoint_emission_no_live_confirmed
  | ($entrypoint_emission.gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
      and $entrypoint_emission.entrypoint_emission_readiness_complete == true
      and $entrypoint_emission.ready_for_canary_readback_replay_gate == true
      and $source_entrypoint_emission_no_live_confirmed) as $source_entrypoint_emission_readiness_complete
  | ($trace_guardrail.live_guardrail_enforcement_enabled == false
      and $trace_guardrail.ready_for_live_execution == false
      and ($trace_guardrail.side_effects | to_entries | all(.value == false))) as $source_trace_guardrail_no_live_blocking_confirmed
  | ($trace_guardrail.gate == "hepta_work_graph_trace_guardrail_span_report_only_gate"
      and $trace_guardrail.trace_guardrail_prior_readbacks_complete == true
      and $trace_guardrail.ready_for_agent_jobs_task_board_report_only_emission == true
      and $source_trace_guardrail_no_live_blocking_confirmed) as $source_trace_guardrail_readiness_complete
  | ($scheduler.live_blocking_enforcement_enabled == false
      and $scheduler.ready_for_live_execution == false
      and ($scheduler.side_effects | to_entries | all(.value == false))) as $source_scheduler_admission_no_live_blocking_confirmed
  | ($scheduler.gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
      and $scheduler.dry_run_enforcement_enabled == true
      and $scheduler.ready_for_append_only_event_store_shadow_path == true
      and $source_scheduler_admission_no_live_blocking_confirmed) as $source_scheduler_admission_dry_run_ready
  | ($source_feature_flag_non_blocking_canary_ready
      and $source_canary_readback_replay_ready
      and $source_entrypoint_emission_readiness_complete
      and $source_trace_guardrail_readiness_complete
      and $source_scheduler_admission_dry_run_ready) as $config_wiring_prior_readbacks_complete
  | (($config_contracts | length) > 0
      and ($config_contracts | all(
        .default_enabled == false
        and .current_enabled == false
        and .traffic_ppm == 0
        and .readback_required == true
        and .rollback_replay_required == true
        and .operator_packet_required == true
        and .config_digest_required == true
        and .config_written == false
        and .live_mutation_allowed == false
      ))) as $config_contracts_report_only_complete
  | (($config_digest_previews | length) > 0
      and ($config_digest_previews | all(
        .digest_required_before_enablement == true
        and .deterministic_input_count > 0
        and .digest_persisted == false
      ))) as $config_digest_previews_unpersisted
  | (($source_bindings | length) > 0
      and ($source_bindings | all(
        .field_present == true
        and .default_off_observed == true
        and .zero_traffic_observed == true
        and .readback_observed == true
        and .rollback_replay_observed == true
        and .live_cutover_observed == false
      ))) as $config_source_bindings_report_only_complete
  | ($config_wiring_prior_readbacks_complete
      and $config_contracts_report_only_complete
      and $config_digest_previews_unpersisted
      and $config_source_bindings_report_only_complete
      and ($safety_checks | all(.required == true))) as $config_wiring_report_only_preconditions_complete
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
      source_feature_flag_non_blocking_canary_required_prior_gate_count: $feature_flag.required_prior_gate_count,
      source_feature_flag_count: $feature_flag.feature_flag_count,
      source_feature_flag_safety_check_count: $feature_flag.safety_check_count,
      source_canary_readback_replay_required_prior_gate_count: $canary_readback.required_prior_gate_count,
      source_canary_readback_replay_entrypoint_count: $canary_readback.canary_entrypoint_count,
      source_canary_readback_replay_readback_evidence_count: $canary_readback.readback_evidence_count,
      source_canary_readback_replay_replay_diff_count: $canary_readback.replay_diff_count,
      source_entrypoint_emission_entrypoint_count: $entrypoint_emission.entrypoint_count,
      source_entrypoint_emission_emission_count: $entrypoint_emission.emission_count,
      source_trace_guardrail_span_count: $trace_guardrail.span_count,
      source_trace_guardrail_blocking_guardrail_count: $trace_guardrail.blocking_guardrail_count,
      source_scheduler_admission_entrypoint_count: $scheduler.entrypoint_count,
      source_scheduler_admission_required_prior_gate_count: ($scheduler.required_prior_gates | length),
      config_contracts: $config_contracts,
      config_digest_previews: $config_digest_previews,
      source_bindings: $source_bindings,
      safety_checks: $safety_checks,
      required_prior_gates: $required_prior_gates,
      source_feature_flag_non_blocking_canary_gate: $feature_flag.gate,
      source_canary_readback_replay_gate: $canary_readback.gate,
      source_entrypoint_emission_gate: $entrypoint_emission.gate,
      source_trace_guardrail_gate: $trace_guardrail.gate,
      source_scheduler_admission_dry_run_gate: $scheduler.gate,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
      source_feature_flag_non_blocking_canary_ready: $source_feature_flag_non_blocking_canary_ready,
      source_feature_flag_non_blocking_canary_no_enablement_confirmed: $source_feature_flag_non_blocking_canary_no_enablement_confirmed,
      source_canary_readback_replay_ready: $source_canary_readback_replay_ready,
      source_canary_readback_replay_no_live_confirmed: $source_canary_readback_replay_no_live_confirmed,
      source_entrypoint_emission_readiness_complete: $source_entrypoint_emission_readiness_complete,
      source_entrypoint_emission_no_live_confirmed: $source_entrypoint_emission_no_live_confirmed,
      source_trace_guardrail_readiness_complete: $source_trace_guardrail_readiness_complete,
      source_trace_guardrail_no_live_blocking_confirmed: $source_trace_guardrail_no_live_blocking_confirmed,
      source_scheduler_admission_dry_run_ready: $source_scheduler_admission_dry_run_ready,
      source_scheduler_admission_no_live_blocking_confirmed: $source_scheduler_admission_no_live_blocking_confirmed,
      config_wiring_prior_readbacks_complete: $config_wiring_prior_readbacks_complete,
      config_contracts_report_only_complete: $config_contracts_report_only_complete,
      config_digest_previews_unpersisted: $config_digest_previews_unpersisted,
      config_source_bindings_report_only_complete: $config_source_bindings_report_only_complete,
      config_wiring_report_only_preconditions_complete: $config_wiring_report_only_preconditions_complete,
      ready_for_operator_packet_report_only: $config_wiring_report_only_preconditions_complete,
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
        task_board_metadata_safety_fields_present: $task_board_metadata_safety_fields_present,
        feature_flag_non_blocking_canary_report_gate: $feature_flag.gate,
        feature_flag_non_blocking_canary_ready_for_config_wiring: $feature_flag.ready_for_feature_flag_config_wiring,
        feature_flag_non_blocking_canary_ready_for_enablement: $feature_flag.ready_for_feature_flag_enablement,
        feature_flag_non_blocking_canary_ready_for_live_cutover: $feature_flag.ready_for_live_cutover,
        feature_flag_non_blocking_canary_side_effects_all_false: ($feature_flag.side_effects | to_entries | all(.value == false)),
        canary_readback_replay_report_gate: $canary_readback.gate,
        canary_readback_replay_ready_for_non_blocking_canary: $canary_readback.ready_for_non_blocking_canary,
        canary_readback_replay_ready_for_live_cutover: $canary_readback.ready_for_live_cutover,
        canary_readback_replay_feature_flag_enabled: $canary_readback.feature_flag_enabled,
        canary_readback_replay_side_effects_all_false: ($canary_readback.side_effects | to_entries | all(.value == false)),
        entrypoint_emission_report_gate: $entrypoint_emission.gate,
        entrypoint_emission_readiness_complete: $entrypoint_emission.entrypoint_emission_readiness_complete,
        entrypoint_emission_side_effects_all_false: ($entrypoint_emission.side_effects | to_entries | all(.value == false)),
        trace_guardrail_report_gate: $trace_guardrail.gate,
        trace_guardrail_prior_readbacks_complete: $trace_guardrail.trace_guardrail_prior_readbacks_complete,
        trace_guardrail_side_effects_all_false: ($trace_guardrail.side_effects | to_entries | all(.value == false)),
        scheduler_admission_dry_run_report_gate: $scheduler.gate,
        scheduler_admission_dry_run_ready: $scheduler.ready_for_append_only_event_store_shadow_path,
        scheduler_admission_dry_run_side_effects_all_false: ($scheduler.side_effects | to_entries | all(.value == false))
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
